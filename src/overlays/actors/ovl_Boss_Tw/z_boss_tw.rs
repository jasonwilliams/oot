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
    fn ActorShape_Init(shape: *mut ActorShape, yOffset: f32_0,
                       shadowDraw: ActorShadowFunc, shadowScale: f32_0);
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
    fn func_8002D7EC(actor: *mut Actor);
    #[no_mangle]
    fn func_8002D908(actor: *mut Actor);
    #[no_mangle]
    fn func_8002DF54(globalCtx: *mut GlobalContext, actor: *mut Actor,
                     arg2: u8_0) -> s32;
    #[no_mangle]
    fn Actor_UpdateBgCheckInfo(globalCtx: *mut GlobalContext,
                               actor: *mut Actor, wallCheckHeight: f32_0,
                               wallCheckRadius: f32_0,
                               ceilingCheckHeight: f32_0, flags: s32);
    #[no_mangle]
    fn func_8002F6D4(globalCtx: *mut GlobalContext, actor: *mut Actor,
                     arg2: f32_0, arg3: s16, arg4: f32_0, arg5: u32_0);
    #[no_mangle]
    fn func_8002F7DC(actor: *mut Actor, sfxId: u16_0);
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
    fn Collider_UpdateCylinder(actor: *mut Actor,
                               collider: *mut ColliderCylinder);
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
    fn Rand_S16Offset(base: s16, range: s16) -> s16;
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
    fn func_80078884(sfxId: u16_0);
    #[no_mangle]
    fn func_80078914(arg0: *mut Vec3f, sfxId: u16_0);
    #[no_mangle]
    fn Player_HasMirrorShieldEquipped(globalCtx: *mut GlobalContext) -> s32;
    #[no_mangle]
    fn Gfx_SetFog(gfx: *mut Gfx, r: s32, g: s32, b: s32, a: s32, near: s32,
                  far: s32) -> *mut Gfx;
    #[no_mangle]
    fn Gfx_SetFog2(gfx: *mut Gfx, r: s32, g: s32, b: s32, a: s32, near: s32,
                   far: s32) -> *mut Gfx;
    #[no_mangle]
    fn func_80093D18(gfxCtx: *mut GraphicsContext);
    #[no_mangle]
    fn func_80093D84(gfxCtx: *mut GraphicsContext);
    #[no_mangle]
    fn func_80094044(gfxCtx: *mut GraphicsContext);
    #[no_mangle]
    fn Gfx_TexScroll(gfxCtx: *mut GraphicsContext, x: u32_0, y: u32_0,
                     width: s32, height: s32) -> *mut Gfx;
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
    fn SkelAnime_Free(skelAnime: *mut SkelAnime,
                      globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn SkinMatrix_Vec3fMtxFMultXYZW(mf: *mut MtxF, src: *mut Vec3f,
                                    xyzDest: *mut Vec3f, wDest: *mut f32_0);
    #[no_mangle]
    fn func_800AA000(_: f32_0, _: u8_0, _: u8_0, _: u8_0);
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
    fn Matrix_MultVec3f(src: *mut Vec3f, dest: *mut Vec3f);
    #[no_mangle]
    fn func_800D1FD4(mf: *mut MtxF);
    #[no_mangle]
    fn Matrix_MtxFToYXZRotS(mf: *mut MtxF, rotDest: *mut Vec3s, flag: s32);
    #[no_mangle]
    fn Audio_PlaySoundGeneral(sfxId: u16_0, pos: *mut Vec3f, token: u8_0,
                              freqScale: *mut f32_0, a4: *mut f32_0,
                              reverbAdd: *mut s8);
    #[no_mangle]
    fn Audio_QueueSeqCmd(bgmID: u32_0);
    #[no_mangle]
    fn Math_FAtan2F(y: f32_0, x: f32_0) -> f32_0;
    #[no_mangle]
    fn Rand_ZeroOne() -> f32_0;
    #[no_mangle]
    fn sinf(_: f32_0) -> f32_0;
    #[no_mangle]
    fn cosf(_: f32_0) -> f32_0;
    #[no_mangle]
    fn Message_StartTextbox(globalCtx: *mut GlobalContext, textId: u16_0,
                            actor: *mut Actor);
    #[no_mangle]
    static mut gSegments: [u32_0; 16];
    #[no_mangle]
    static mut gSaveContext: SaveContext;
    #[no_mangle]
    static mut D_801333E8: s8;
    #[no_mangle]
    static mut D_801333E0: f32_0;
    #[no_mangle]
    static mut gCircleShadowDL: [Gfx; 0];
    #[no_mangle]
    static mut object_tw_Anim_0004A4: AnimationHeader;
    #[no_mangle]
    static mut object_tw_Anim_000AAC: AnimationHeader;
    #[no_mangle]
    static mut object_tw_Anim_0012A4: AnimationHeader;
    #[no_mangle]
    static mut object_tw_Anim_0017E0: AnimationHeader;
    #[no_mangle]
    static mut object_tw_Anim_001D10: AnimationHeader;
    #[no_mangle]
    static mut object_tw_Anim_00230C: AnimationHeader;
    #[no_mangle]
    static mut object_tw_Anim_003614: AnimationHeader;
    #[no_mangle]
    static mut object_tw_Anim_003E34: AnimationHeader;
    #[no_mangle]
    static mut object_tw_Anim_004548: AnimationHeader;
    #[no_mangle]
    static mut object_tw_Anim_00578C: AnimationHeader;
    #[no_mangle]
    static mut object_tw_Anim_006530: AnimationHeader;
    #[no_mangle]
    static mut object_tw_Anim_006F28: AnimationHeader;
    #[no_mangle]
    static mut object_tw_Skel_0070E0: FlexSkeletonHeader;
    #[no_mangle]
    static mut object_tw_Anim_007688: AnimationHeader;
    #[no_mangle]
    static mut object_tw_Anim_007CA8: AnimationHeader;
    #[no_mangle]
    static mut object_tw_Anim_0088C8: AnimationHeader;
    #[no_mangle]
    static mut object_tw_Anim_009398: AnimationHeader;
    #[no_mangle]
    static mut object_tw_Tex_00A438: [u64_0; 0];
    #[no_mangle]
    static mut object_tw_Tex_00B238: [u64_0; 0];
    #[no_mangle]
    static mut object_tw_Tex_00B638: [u64_0; 0];
    #[no_mangle]
    static mut object_tw_DL_012B38: [Gfx; 0];
    #[no_mangle]
    static mut object_tw_DL_012CE0: [Gfx; 0];
    #[no_mangle]
    static mut object_tw_DL_013310: [Gfx; 0];
    #[no_mangle]
    static mut object_tw_DL_0134B8: [Gfx; 0];
    #[no_mangle]
    static mut object_tw_DL_013AE8: [Gfx; 0];
    #[no_mangle]
    static mut object_tw_DL_013D68: [Gfx; 0];
    #[no_mangle]
    static mut object_tw_DL_013E98: [Gfx; 0];
    #[no_mangle]
    static mut object_tw_DL_013F98: [Gfx; 0];
    #[no_mangle]
    static mut object_tw_DL_014070: [Gfx; 0];
    #[no_mangle]
    static mut object_tw_DL_014158: [Gfx; 0];
    #[no_mangle]
    static mut object_tw_DL_018FC0: [Gfx; 0];
    #[no_mangle]
    static mut object_tw_DL_019938: [Gfx; 0];
    #[no_mangle]
    static mut object_tw_DL_019D40: [Gfx; 0];
    #[no_mangle]
    static mut object_tw_DL_01A430: [Gfx; 0];
    #[no_mangle]
    static mut object_tw_DL_01A528: [Gfx; 0];
    #[no_mangle]
    static mut object_tw_DL_01A5A8: [Gfx; 0];
    #[no_mangle]
    static mut object_tw_DL_01A790: [Gfx; 0];
    #[no_mangle]
    static mut object_tw_DL_01A998: [Gfx; 0];
    #[no_mangle]
    static mut object_tw_DL_01AA50: [Gfx; 0];
    #[no_mangle]
    static mut object_tw_DL_01AB00: [Gfx; 0];
    #[no_mangle]
    static mut object_tw_DL_01BC00: [Gfx; 0];
    #[no_mangle]
    static mut object_tw_DL_01C1C0: [Gfx; 0];
    #[no_mangle]
    static mut object_tw_DL_01CEE0: [Gfx; 0];
    #[no_mangle]
    static mut object_tw_DL_01DBE8: [Gfx; 0];
    #[no_mangle]
    static mut object_tw_DL_01DDF0: [Gfx; 0];
    #[no_mangle]
    static mut object_tw_DL_01E020: [Gfx; 0];
    #[no_mangle]
    static mut object_tw_DL_01E0E0: [Gfx; 0];
    #[no_mangle]
    static mut object_tw_DL_01E2C0: [Gfx; 0];
    #[no_mangle]
    static mut object_tw_DL_01E3A0: [Gfx; 0];
    #[no_mangle]
    static mut object_tw_DL_01E9F0: [Gfx; 0];
    #[no_mangle]
    static mut object_tw_DL_01EC68: [Gfx; 0];
    #[no_mangle]
    static mut object_tw_DL_01EEB0: [Gfx; 0];
    #[no_mangle]
    static mut object_tw_DL_01F238: [Gfx; 0];
    #[no_mangle]
    static mut object_tw_DL_01F390: [Gfx; 0];
    #[no_mangle]
    static mut object_tw_DL_01F608: [Gfx; 0];
    #[no_mangle]
    static mut object_tw_Skel_01F888: FlexSkeletonHeader;
    #[no_mangle]
    static mut object_tw_Anim_0216DC: AnimationHeader;
    #[no_mangle]
    static mut object_tw_Anim_022700: AnimationHeader;
    #[no_mangle]
    static mut object_tw_Anim_023750: AnimationHeader;
    #[no_mangle]
    static mut object_tw_Anim_024374: AnimationHeader;
    #[no_mangle]
    static mut object_tw_Anim_0244B4: AnimationHeader;
    #[no_mangle]
    static mut object_tw_Tex_02A070: [u64_0; 0];
    #[no_mangle]
    static mut object_tw_Tex_02A470: [u64_0; 0];
    #[no_mangle]
    static mut object_tw_Tex_02A9B0: [u64_0; 0];
    #[no_mangle]
    static mut object_tw_DL_02D890: [Gfx; 0];
    #[no_mangle]
    static mut object_tw_DL_02D940: [Gfx; 0];
    #[no_mangle]
    static mut object_tw_Blob_02E170: [u8_0; 0];
    #[no_mangle]
    static mut object_tw_Skel_032020: FlexSkeletonHeader;
    #[no_mangle]
    static mut object_tw_Anim_032BF8: AnimationHeader;
    #[no_mangle]
    static mut object_tw_Anim_0338F0: AnimationHeader;
    #[no_mangle]
    static mut object_tw_Anim_0343B4: AnimationHeader;
    #[no_mangle]
    static mut object_tw_Anim_035030: AnimationHeader;
    #[no_mangle]
    static mut object_tw_Anim_035988: AnimationHeader;
    #[no_mangle]
    static mut object_tw_Anim_036FBC: AnimationHeader;
    #[no_mangle]
    static mut object_tw_Anim_038E2C: AnimationHeader;
    #[no_mangle]
    static mut object_tw_Anim_03A2D0: AnimationHeader;
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
pub const OBJECT_ID_MAX: C2RustUnnamed_19 = 402;
pub const OBJECT_ZL4: C2RustUnnamed_19 = 401;
pub const OBJECT_TIMEBLOCK: C2RustUnnamed_19 = 400;
pub const OBJECT_OUKE_HAKA: C2RustUnnamed_19 = 399;
pub const OBJECT_DOOR_KILLER: C2RustUnnamed_19 = 398;
pub const OBJECT_GI_SWORD_1: C2RustUnnamed_19 = 397;
pub const OBJECT_COB: C2RustUnnamed_19 = 396;
pub const OBJECT_COW: C2RustUnnamed_19 = 395;
pub const OBJECT_BWALL: C2RustUnnamed_19 = 394;
pub const OBJECT_PS: C2RustUnnamed_19 = 393;
pub const OBJECT_GS: C2RustUnnamed_19 = 392;
pub const OBJECT_HAKA_DOOR: C2RustUnnamed_19 = 391;
pub const OBJECT_GEFF: C2RustUnnamed_19 = 390;
pub const OBJECT_GJ: C2RustUnnamed_19 = 389;
pub const OBJECT_SKB: C2RustUnnamed_19 = 388;
pub const OBJECT_WF: C2RustUnnamed_19 = 387;
pub const OBJECT_MU: C2RustUnnamed_19 = 386;
pub const OBJECT_SPOT01_MATOYAB: C2RustUnnamed_19 = 385;
pub const OBJECT_SPOT01_MATOYA: C2RustUnnamed_19 = 384;
pub const OBJECT_GI_RUPY: C2RustUnnamed_19 = 383;
pub const OBJECT_GANON_ANIME3: C2RustUnnamed_19 = 382;
pub const OBJECT_GANON_ANIME2: C2RustUnnamed_19 = 381;
pub const OBJECT_GANON_ANIME1: C2RustUnnamed_19 = 380;
pub const OBJECT_GI_DEKUPOUCH: C2RustUnnamed_19 = 379;
pub const OBJECT_EFC_DOUGHNUT: C2RustUnnamed_19 = 378;
pub const OBJECT_DEMO_KEKKAI: C2RustUnnamed_19 = 377;
pub const OBJECT_BOWL: C2RustUnnamed_19 = 376;
pub const OBJECT_GI_SOUL: C2RustUnnamed_19 = 375;
pub const OBJECT_GI_GHOST: C2RustUnnamed_19 = 374;
pub const OBJECT_GI_BUTTERFLY: C2RustUnnamed_19 = 373;
pub const OBJECT_GI_INSECT: C2RustUnnamed_19 = 372;
pub const OBJECT_GI_FIRE: C2RustUnnamed_19 = 371;
pub const OBJECT_DNK: C2RustUnnamed_19 = 370;
pub const OBJECT_DNS: C2RustUnnamed_19 = 369;
pub const OBJECT_KIBAKO2: C2RustUnnamed_19 = 368;
pub const OBJECT_SPOT11_OBJ: C2RustUnnamed_19 = 367;
pub const OBJECT_UNSET_16E: C2RustUnnamed_19 = 366;
pub const OBJECT_JYA_DOOR: C2RustUnnamed_19 = 365;
pub const OBJECT_JYA_IRON: C2RustUnnamed_19 = 364;
pub const OBJECT_DOG: C2RustUnnamed_19 = 363;
pub const OBJECT_GR: C2RustUnnamed_19 = 362;
pub const OBJECT_GELDB: C2RustUnnamed_19 = 361;
pub const OBJECT_SHOPNUTS: C2RustUnnamed_19 = 360;
pub const OBJECT_GLA: C2RustUnnamed_19 = 359;
pub const OBJECT_SPOT00_BREAK: C2RustUnnamed_19 = 358;
pub const OBJECT_RS: C2RustUnnamed_19 = 357;
pub const OBJECT_HINTNUTS: C2RustUnnamed_19 = 356;
pub const OBJECT_BOMBIWA: C2RustUnnamed_19 = 355;
pub const OBJECT_SPOT12_OBJ: C2RustUnnamed_19 = 354;
pub const OBJECT_SPOT05_OBJECTS: C2RustUnnamed_19 = 353;
pub const OBJECT_BG: C2RustUnnamed_19 = 352;
pub const OBJECT_BIGOKUTA: C2RustUnnamed_19 = 351;
pub const OBJECT_SSH: C2RustUnnamed_19 = 350;
pub const OBJECT_GI_GODDESS: C2RustUnnamed_19 = 349;
pub const OBJECT_GI_SUTARU: C2RustUnnamed_19 = 348;
pub const OBJECT_FISH: C2RustUnnamed_19 = 347;
pub const OBJECT_EC: C2RustUnnamed_19 = 346;
pub const OBJECT_DS2: C2RustUnnamed_19 = 345;
pub const OBJECT_GI_M_ARROW: C2RustUnnamed_19 = 344;
pub const OBJECT_GI_HOVERBOOTS: C2RustUnnamed_19 = 343;
pub const OBJECT_ZG: C2RustUnnamed_19 = 342;
pub const OBJECT_TS: C2RustUnnamed_19 = 341;
pub const OBJECT_KA: C2RustUnnamed_19 = 340;
pub const OBJECT_GANON2: C2RustUnnamed_19 = 339;
pub const OBJECT_GI_GERUDOMASK: C2RustUnnamed_19 = 338;
pub const OBJECT_GI_ZORAMASK: C2RustUnnamed_19 = 337;
pub const OBJECT_GI_GOLONMASK: C2RustUnnamed_19 = 336;
pub const OBJECT_ZL2_ANIME2: C2RustUnnamed_19 = 335;
pub const OBJECT_ZL2_ANIME1: C2RustUnnamed_19 = 334;
pub const OBJECT_EFC_ERUPC: C2RustUnnamed_19 = 333;
pub const OBJECT_GT: C2RustUnnamed_19 = 332;
pub const OBJECT_DOOR_GERUDO: C2RustUnnamed_19 = 331;
pub const OBJECT_MAG: C2RustUnnamed_19 = 330;
pub const OBJECT_GI_FROG: C2RustUnnamed_19 = 329;
pub const OBJECT_GI_SOLDOUT: C2RustUnnamed_19 = 328;
pub const OBJECT_GI_BRACELET: C2RustUnnamed_19 = 327;
pub const OBJECT_GI_PRESCRIPTION: C2RustUnnamed_19 = 326;
pub const OBJECT_CS: C2RustUnnamed_19 = 325;
pub const OBJECT_JS: C2RustUnnamed_19 = 324;
pub const OBJECT_GI_BROKENSWORD: C2RustUnnamed_19 = 323;
pub const OBJECT_GI_TICKETSTONE: C2RustUnnamed_19 = 322;
pub const OBJECT_GI_MUSHROOM: C2RustUnnamed_19 = 321;
pub const OBJECT_GI_POWDER: C2RustUnnamed_19 = 320;
pub const OBJECT_GI_EYE_LOTION: C2RustUnnamed_19 = 319;
pub const OBJECT_OS: C2RustUnnamed_19 = 318;
pub const OBJECT_FA: C2RustUnnamed_19 = 317;
pub const OBJECT_MM: C2RustUnnamed_19 = 316;
pub const OBJECT_STREAM: C2RustUnnamed_19 = 315;
pub const OBJECT_SIOFUKI: C2RustUnnamed_19 = 314;
pub const OBJECT_GANON_OBJECTS: C2RustUnnamed_19 = 313;
pub const OBJECT_GI_TRUTH_MASK: C2RustUnnamed_19 = 312;
pub const OBJECT_GI_RABIT_MASK: C2RustUnnamed_19 = 311;
pub const OBJECT_GI_SKJ_MASK: C2RustUnnamed_19 = 310;
pub const OBJECT_GI_REDEAD_MASK: C2RustUnnamed_19 = 309;
pub const OBJECT_GI_KI_TAN_MASK: C2RustUnnamed_19 = 308;
pub const OBJECT_FU: C2RustUnnamed_19 = 307;
pub const OBJECT_MK: C2RustUnnamed_19 = 306;
pub const OBJECT_OWL: C2RustUnnamed_19 = 305;
pub const OBJECT_GJYO_OBJECTS: C2RustUnnamed_19 = 304;
pub const OBJECT_KANBAN: C2RustUnnamed_19 = 303;
pub const OBJECT_GI_COIN: C2RustUnnamed_19 = 302;
pub const OBJECT_GI_GLOVES: C2RustUnnamed_19 = 301;
pub const OBJECT_TSUBO: C2RustUnnamed_19 = 300;
pub const OBJECT_KUSA: C2RustUnnamed_19 = 299;
pub const OBJECT_LIGHTSWITCH: C2RustUnnamed_19 = 298;
pub const OBJECT_INGATE: C2RustUnnamed_19 = 297;
pub const OBJECT_HS: C2RustUnnamed_19 = 296;
pub const OBJECT_MS: C2RustUnnamed_19 = 295;
pub const OBJECT_GM: C2RustUnnamed_19 = 294;
pub const OBJECT_BLKOBJ: C2RustUnnamed_19 = 293;
pub const OBJECT_NWC: C2RustUnnamed_19 = 292;
pub const OBJECT_UNSET_123: C2RustUnnamed_19 = 291;
pub const OBJECT_DAIKU: C2RustUnnamed_19 = 290;
pub const OBJECT_TORYO: C2RustUnnamed_19 = 289;
pub const OBJECT_UNSET_120: C2RustUnnamed_19 = 288;
pub const OBJECT_GOROIWA: C2RustUnnamed_19 = 287;
pub const OBJECT_MAMENOKI: C2RustUnnamed_19 = 286;
pub const OBJECT_D_LIFT: C2RustUnnamed_19 = 285;
pub const OBJECT_D_HSBLOCK: C2RustUnnamed_19 = 284;
pub const OBJECT_D_ELEVATOR: C2RustUnnamed_19 = 283;
pub const OBJECT_GND_MAGIC: C2RustUnnamed_19 = 282;
pub const OBJECT_GI_SEED: C2RustUnnamed_19 = 281;
pub const OBJECT_GI_BOOTS_2: C2RustUnnamed_19 = 280;
pub const OBJECT_YABUSAME_POINT: C2RustUnnamed_19 = 279;
pub const OBJECT_GE1: C2RustUnnamed_19 = 278;
pub const OBJECT_BOB: C2RustUnnamed_19 = 277;
pub const OBJECT_FZ: C2RustUnnamed_19 = 276;
pub const OBJECT_SPOT07_OBJECT: C2RustUnnamed_19 = 275;
pub const OBJECT_SPOT03_OBJECT: C2RustUnnamed_19 = 274;
pub const OBJECT_BOJ: C2RustUnnamed_19 = 273;
pub const OBJECT_ANE: C2RustUnnamed_19 = 272;
pub const OBJECT_DS: C2RustUnnamed_19 = 271;
pub const OBJECT_GI_OCARINA_0: C2RustUnnamed_19 = 270;
pub const OBJECT_BBA: C2RustUnnamed_19 = 269;
pub const OBJECT_BJI: C2RustUnnamed_19 = 268;
pub const OBJECT_GI_BOTTLE_LETTER: C2RustUnnamed_19 = 267;
pub const OBJECT_SKJ: C2RustUnnamed_19 = 266;
pub const OBJECT_GI_NIWATORI: C2RustUnnamed_19 = 265;
pub const OBJECT_CNE: C2RustUnnamed_19 = 264;
pub const OBJECT_AHG: C2RustUnnamed_19 = 263;
pub const OBJECT_IK: C2RustUnnamed_19 = 262;
pub const OBJECT_AOB: C2RustUnnamed_19 = 261;
pub const OBJECT_MASTERZOORA: C2RustUnnamed_19 = 260;
pub const OBJECT_MASTERGOLON: C2RustUnnamed_19 = 259;
pub const OBJECT_MASTERKOKIRIHEAD: C2RustUnnamed_19 = 258;
pub const OBJECT_MASTERKOKIRI: C2RustUnnamed_19 = 257;
pub const OBJECT_UMAJUMP: C2RustUnnamed_19 = 256;
pub const OBJECT_KZ: C2RustUnnamed_19 = 255;
pub const OBJECT_ZO: C2RustUnnamed_19 = 254;
pub const OBJECT_KW1: C2RustUnnamed_19 = 253;
pub const OBJECT_KM1: C2RustUnnamed_19 = 252;
pub const OBJECT_MD: C2RustUnnamed_19 = 251;
pub const OBJECT_MD_UNUSED: C2RustUnnamed_19 = 250;
pub const OBJECT_SPOT01_OBJECTS: C2RustUnnamed_19 = 249;
pub const OBJECT_GI_LONGSWORD: C2RustUnnamed_19 = 248;
pub const OBJECT_GI_GRASS: C2RustUnnamed_19 = 247;
pub const OBJECT_GI_HAMMER: C2RustUnnamed_19 = 246;
pub const OBJECT_GI_SAW: C2RustUnnamed_19 = 245;
pub const OBJECT_GI_FISH: C2RustUnnamed_19 = 244;
pub const OBJECT_GI_BEAN: C2RustUnnamed_19 = 243;
pub const OBJECT_GI_CLOTHES: C2RustUnnamed_19 = 242;
pub const OBJECT_JYA_OBJ: C2RustUnnamed_19 = 241;
pub const OBJECT_SPOT15_OBJ: C2RustUnnamed_19 = 240;
pub const OBJECT_GI_LETTER: C2RustUnnamed_19 = 239;
pub const OBJECT_GI_SHIELD_3: C2RustUnnamed_19 = 238;
pub const OBJECT_DEMO_6K: C2RustUnnamed_19 = 237;
pub const OBJECT_ANI: C2RustUnnamed_19 = 236;
pub const OBJECT_GI_LIQUID: C2RustUnnamed_19 = 235;
pub const OBJECT_GI_GLASSES: C2RustUnnamed_19 = 234;
pub const OBJECT_GI_BOW: C2RustUnnamed_19 = 233;
pub const OBJECT_GI_BOOMERANG: C2RustUnnamed_19 = 232;
pub const OBJECT_GI_PACHINKO: C2RustUnnamed_19 = 231;
pub const OBJECT_FR: C2RustUnnamed_19 = 230;
pub const OBJECT_NY: C2RustUnnamed_19 = 229;
pub const OBJECT_UNSET_E4: C2RustUnnamed_19 = 228;
pub const OBJECT_NY_UNUSED: C2RustUnnamed_19 = 227;
pub const OBJECT_SST: C2RustUnnamed_19 = 226;
pub const OBJECT_GANON: C2RustUnnamed_19 = 225;
pub const OBJECT_MA1: C2RustUnnamed_19 = 224;
pub const OBJECT_GI_MILK: C2RustUnnamed_19 = 223;
pub const OBJECT_GI_OCARINA: C2RustUnnamed_19 = 222;
pub const OBJECT_GI_HOOKSHOT: C2RustUnnamed_19 = 221;
pub const OBJECT_GI_SHIELD_2: C2RustUnnamed_19 = 220;
pub const OBJECT_GI_SCALE: C2RustUnnamed_19 = 219;
pub const OBJECT_GI_EGG: C2RustUnnamed_19 = 218;
pub const OBJECT_GI_BOMB_2: C2RustUnnamed_19 = 217;
pub const OBJECT_GI_ARROW: C2RustUnnamed_19 = 216;
pub const OBJECT_GI_GERUDO: C2RustUnnamed_19 = 215;
pub const OBJECT_ANUBICE: C2RustUnnamed_19 = 214;
pub const OBJECT_BXA: C2RustUnnamed_19 = 213;
pub const OBJECT_RR: C2RustUnnamed_19 = 212;
pub const OBJECT_TW: C2RustUnnamed_19 = 211;
pub const OBJECT_HNI: C2RustUnnamed_19 = 210;
pub const OBJECT_GI_PURSE: C2RustUnnamed_19 = 209;
pub const OBJECT_MA2: C2RustUnnamed_19 = 208;
pub const OBJECT_OF1S: C2RustUnnamed_19 = 207;
pub const OBJECT_GI_BOMB_1: C2RustUnnamed_19 = 206;
pub const OBJECT_GI_MAGICPOT: C2RustUnnamed_19 = 205;
pub const OBJECT_DEKUJR: C2RustUnnamed_19 = 204;
pub const OBJECT_GI_SHIELD_1: C2RustUnnamed_19 = 203;
pub const OBJECT_RU2: C2RustUnnamed_19 = 202;
pub const OBJECT_OF1D_MAP: C2RustUnnamed_19 = 201;
pub const OBJECT_GI_MAP: C2RustUnnamed_19 = 200;
pub const OBJECT_GI_STICK: C2RustUnnamed_19 = 199;
pub const OBJECT_GI_BOTTLE: C2RustUnnamed_19 = 198;
pub const OBJECT_OS_ANIME: C2RustUnnamed_19 = 197;
pub const OBJECT_OE4S: C2RustUnnamed_19 = 196;
pub const OBJECT_OE1S: C2RustUnnamed_19 = 195;
pub const OBJECT_SPOT16_OBJ: C2RustUnnamed_19 = 194;
pub const OBJECT_TR: C2RustUnnamed_19 = 193;
pub const OBJECT_IN: C2RustUnnamed_19 = 192;
pub const OBJECT_GI_BOMBPOUCH: C2RustUnnamed_19 = 191;
pub const OBJECT_GI_ARROWCASE: C2RustUnnamed_19 = 190;
pub const OBJECT_GI_HEARTS: C2RustUnnamed_19 = 189;
pub const OBJECT_SA: C2RustUnnamed_19 = 188;
pub const OBJECT_GI_NUTS: C2RustUnnamed_19 = 187;
pub const OBJECT_GI_MEDAL: C2RustUnnamed_19 = 186;
pub const OBJECT_GI_BOSSKEY: C2RustUnnamed_19 = 185;
pub const OBJECT_GI_COMPASS: C2RustUnnamed_19 = 184;
pub const OBJECT_GI_HEART: C2RustUnnamed_19 = 183;
pub const OBJECT_GI_MELODY: C2RustUnnamed_19 = 182;
pub const OBJECT_SB: C2RustUnnamed_19 = 181;
pub const OBJECT_MO: C2RustUnnamed_19 = 180;
pub const OBJECT_NB: C2RustUnnamed_19 = 179;
pub const OBJECT_SHOP_DUNGEN: C2RustUnnamed_19 = 178;
pub const OBJECT_SPOT17_OBJ: C2RustUnnamed_19 = 177;
pub const OBJECT_BDOOR: C2RustUnnamed_19 = 176;
pub const OBJECT_SPOT18_OBJ: C2RustUnnamed_19 = 175;
pub const OBJECT_SPOT09_OBJ: C2RustUnnamed_19 = 174;
pub const OBJECT_GI_JEWEL: C2RustUnnamed_19 = 173;
pub const OBJECT_BROB: C2RustUnnamed_19 = 172;
pub const OBJECT_MIR_RAY: C2RustUnnamed_19 = 171;
pub const OBJECT_GI_KEY: C2RustUnnamed_19 = 170;
pub const OBJECT_DEMO_TRE_LGT: C2RustUnnamed_19 = 169;
pub const OBJECT_EFC_TW: C2RustUnnamed_19 = 168;
pub const OBJECT_RL: C2RustUnnamed_19 = 167;
pub const OBJECT_DH: C2RustUnnamed_19 = 166;
pub const OBJECT_FD2: C2RustUnnamed_19 = 165;
pub const OBJECT_SYOKUDAI: C2RustUnnamed_19 = 164;
pub const OBJECT_RU1: C2RustUnnamed_19 = 163;
pub const OBJECT_HAKA: C2RustUnnamed_19 = 162;
pub const OBJECT_SPOT02_OBJECTS: C2RustUnnamed_19 = 161;
pub const OBJECT_HORSE_LINK_CHILD: C2RustUnnamed_19 = 160;
pub const OBJECT_MEDAL: C2RustUnnamed_19 = 159;
pub const OBJECT_FW: C2RustUnnamed_19 = 158;
pub const OBJECT_DU: C2RustUnnamed_19 = 157;
pub const OBJECT_FD: C2RustUnnamed_19 = 156;
pub const OBJECT_GNDD: C2RustUnnamed_19 = 155;
pub const OBJECT_HEAVY_OBJECT: C2RustUnnamed_19 = 154;
pub const OBJECT_PO_SISTERS: C2RustUnnamed_19 = 153;
pub const OBJECT_RD: C2RustUnnamed_19 = 152;
pub const OBJECT_SD: C2RustUnnamed_19 = 151;
pub const OBJECT_BDAN_OBJECTS: C2RustUnnamed_19 = 150;
pub const OBJECT_TRIFORCE_SPOT: C2RustUnnamed_19 = 149;
pub const OBJECT_LIGHT_RING: C2RustUnnamed_19 = 148;
pub const OBJECT_GOD_LGT: C2RustUnnamed_19 = 147;
pub const OBJECT_EFC_STAR_FIELD: C2RustUnnamed_19 = 146;
pub const OBJECT_EFC_LGT_SHOWER: C2RustUnnamed_19 = 145;
pub const OBJECT_EFC_FLASH: C2RustUnnamed_19 = 144;
pub const OBJECT_EFC_FIRE_BALL: C2RustUnnamed_19 = 143;
pub const OBJECT_EFC_CRYSTAL_LIGHT: C2RustUnnamed_19 = 142;
pub const OBJECT_HAKACH_OBJECTS: C2RustUnnamed_19 = 141;
pub const OBJECT_BV: C2RustUnnamed_19 = 140;
pub const OBJECT_VM: C2RustUnnamed_19 = 139;
pub const OBJECT_XC: C2RustUnnamed_19 = 138;
pub const OBJECT_TK: C2RustUnnamed_19 = 137;
pub const OBJECT_TA: C2RustUnnamed_19 = 136;
pub const OBJECT_IM: C2RustUnnamed_19 = 135;
pub const OBJECT_VASE: C2RustUnnamed_19 = 134;
pub const OBJECT_TRAP: C2RustUnnamed_19 = 133;
pub const OBJECT_UNSET_84: C2RustUnnamed_19 = 132;
pub const OBJECT_UNSET_83: C2RustUnnamed_19 = 131;
pub const OBJECT_PU_BOX: C2RustUnnamed_19 = 130;
pub const OBJECT_LIGHTBOX: C2RustUnnamed_19 = 129;
pub const OBJECT_UNSET_80: C2RustUnnamed_19 = 128;
pub const OBJECT_UNSET_7F: C2RustUnnamed_19 = 127;
pub const OBJECT_UNSET_7E: C2RustUnnamed_19 = 126;
pub const OBJECT_UNSET_7D: C2RustUnnamed_19 = 125;
pub const OBJECT_WOOD02: C2RustUnnamed_19 = 124;
pub const OBJECT_UNSET_7B: C2RustUnnamed_19 = 123;
pub const OBJECT_UNSET_7A: C2RustUnnamed_19 = 122;
pub const OBJECT_UNSET_79: C2RustUnnamed_19 = 121;
pub const OBJECT_UNSET_78: C2RustUnnamed_19 = 120;
pub const OBJECT_BIRD: C2RustUnnamed_19 = 119;
pub const OBJECT_HATA: C2RustUnnamed_19 = 118;
pub const OBJECT_WARP2: C2RustUnnamed_19 = 117;
pub const OBJECT_SPOT08_OBJ: C2RustUnnamed_19 = 116;
pub const OBJECT_MORI_TEX: C2RustUnnamed_19 = 115;
pub const OBJECT_MORI_OBJECTS: C2RustUnnamed_19 = 114;
pub const OBJECT_MORI_HINERI2A: C2RustUnnamed_19 = 113;
pub const OBJECT_MORI_HINERI2: C2RustUnnamed_19 = 112;
pub const OBJECT_MORI_HINERI1A: C2RustUnnamed_19 = 111;
pub const OBJECT_PO_COMPOSER: C2RustUnnamed_19 = 110;
pub const OBJECT_PO_FIELD: C2RustUnnamed_19 = 109;
pub const OBJECT_RELAY_OBJECTS: C2RustUnnamed_19 = 108;
pub const OBJECT_ICE_OBJECTS: C2RustUnnamed_19 = 107;
pub const OBJECT_SPOT06_OBJECTS: C2RustUnnamed_19 = 106;
pub const OBJECT_HAKA_OBJECTS: C2RustUnnamed_19 = 105;
pub const OBJECT_MJIN_OKA: C2RustUnnamed_19 = 104;
pub const OBJECT_MJIN_WIND: C2RustUnnamed_19 = 103;
pub const OBJECT_MJIN_SOUL: C2RustUnnamed_19 = 102;
pub const OBJECT_MJIN_ICE: C2RustUnnamed_19 = 101;
pub const OBJECT_MJIN_FLAME: C2RustUnnamed_19 = 100;
pub const OBJECT_MJIN_DARK: C2RustUnnamed_19 = 99;
pub const OBJECT_MJIN_FLASH: C2RustUnnamed_19 = 98;
pub const OBJECT_MJIN: C2RustUnnamed_19 = 97;
pub const OBJECT_ZL2: C2RustUnnamed_19 = 96;
pub const OBJECT_YUKABYUN: C2RustUnnamed_19 = 95;
pub const OBJECT_TOKI_OBJECTS: C2RustUnnamed_19 = 94;
pub const OBJECT_BB: C2RustUnnamed_19 = 93;
pub const OBJECT_MORI_HINERI1: C2RustUnnamed_19 = 92;
pub const OBJECT_OSSAN: C2RustUnnamed_19 = 91;
pub const OBJECT_FHG: C2RustUnnamed_19 = 90;
pub const OBJECT_MIZU_OBJECTS: C2RustUnnamed_19 = 89;
pub const OBJECT_OA11: C2RustUnnamed_19 = 88;
pub const OBJECT_OA10: C2RustUnnamed_19 = 87;
pub const OBJECT_VALI: C2RustUnnamed_19 = 86;
pub const OBJECT_OE12: C2RustUnnamed_19 = 85;
pub const OBJECT_OE11: C2RustUnnamed_19 = 84;
pub const OBJECT_OE10: C2RustUnnamed_19 = 83;
pub const OBJECT_OE9: C2RustUnnamed_19 = 82;
pub const OBJECT_OE8: C2RustUnnamed_19 = 81;
pub const OBJECT_OE7: C2RustUnnamed_19 = 80;
pub const OBJECT_OE6: C2RustUnnamed_19 = 79;
pub const OBJECT_OE5: C2RustUnnamed_19 = 78;
pub const OBJECT_MENKURI_OBJECTS: C2RustUnnamed_19 = 77;
pub const OBJECT_OE4: C2RustUnnamed_19 = 76;
pub const OBJECT_OE3: C2RustUnnamed_19 = 75;
pub const OBJECT_DEKUNUTS: C2RustUnnamed_19 = 74;
pub const OBJECT_B_HEART: C2RustUnnamed_19 = 73;
pub const OBJECT_WARP1: C2RustUnnamed_19 = 72;
pub const OBJECT_OPENING_DEMO1: C2RustUnnamed_19 = 71;
pub const OBJECT_HORSE_ZELDA: C2RustUnnamed_19 = 70;
pub const OBJECT_OB4: C2RustUnnamed_19 = 69;
pub const OBJECT_OB3: C2RustUnnamed_19 = 68;
pub const OBJECT_OB2: C2RustUnnamed_19 = 67;
pub const OBJECT_OA9: C2RustUnnamed_19 = 66;
pub const OBJECT_OA8: C2RustUnnamed_19 = 65;
pub const OBJECT_JJ: C2RustUnnamed_19 = 64;
pub const OBJECT_OA7: C2RustUnnamed_19 = 63;
pub const OBJECT_OA6: C2RustUnnamed_19 = 62;
pub const OBJECT_OA5: C2RustUnnamed_19 = 61;
pub const OBJECT_OA4: C2RustUnnamed_19 = 60;
pub const OBJECT_OA3: C2RustUnnamed_19 = 59;
pub const OBJECT_UNSET_3A: C2RustUnnamed_19 = 58;
pub const OBJECT_DEKUBABA: C2RustUnnamed_19 = 57;
pub const OBJECT_AM: C2RustUnnamed_19 = 56;
pub const OBJECT_GND: C2RustUnnamed_19 = 55;
pub const OBJECT_YDAN_OBJECTS: C2RustUnnamed_19 = 54;
pub const OBJECT_OE2: C2RustUnnamed_19 = 53;
pub const OBJECT_OE_ANIME: C2RustUnnamed_19 = 52;
pub const OBJECT_OE1: C2RustUnnamed_19 = 51;
pub const OBJECT_SK2: C2RustUnnamed_19 = 50;
pub const OBJECT_BOMBF: C2RustUnnamed_19 = 49;
pub const OBJECT_MB: C2RustUnnamed_19 = 48;
pub const OBJECT_SPOT00_OBJECTS: C2RustUnnamed_19 = 47;
pub const OBJECT_OA2: C2RustUnnamed_19 = 46;
pub const OBJECT_HORSE_GANON: C2RustUnnamed_19 = 45;
pub const OBJECT_HIDAN_OBJECTS: C2RustUnnamed_19 = 44;
pub const OBJECT_DDAN_OBJECTS: C2RustUnnamed_19 = 43;
pub const OBJECT_SPOT04_OBJECTS: C2RustUnnamed_19 = 42;
pub const OBJECT_O_ANIME: C2RustUnnamed_19 = 41;
pub const OBJECT_OB1: C2RustUnnamed_19 = 40;
pub const OBJECT_HORSE_NORMAL: C2RustUnnamed_19 = 39;
pub const OBJECT_EI: C2RustUnnamed_19 = 38;
pub const OBJECT_BW: C2RustUnnamed_19 = 37;
pub const OBJECT_ST: C2RustUnnamed_19 = 36;
pub const OBJECT_OA1: C2RustUnnamed_19 = 35;
pub const OBJECT_TP: C2RustUnnamed_19 = 34;
pub const OBJECT_BL: C2RustUnnamed_19 = 33;
pub const OBJECT_TORCH2: C2RustUnnamed_19 = 32;
pub const OBJECT_DODOJR: C2RustUnnamed_19 = 31;
pub const OBJECT_GOL: C2RustUnnamed_19 = 30;
pub const OBJECT_ZL1: C2RustUnnamed_19 = 29;
pub const OBJECT_GOMA: C2RustUnnamed_19 = 28;
pub const OBJECT_ZF: C2RustUnnamed_19 = 27;
pub const OBJECT_HORSE: C2RustUnnamed_19 = 26;
pub const OBJECT_KINGDODONGO: C2RustUnnamed_19 = 25;
pub const OBJECT_PEEHAT: C2RustUnnamed_19 = 24;
pub const OBJECT_REEBA: C2RustUnnamed_19 = 23;
pub const OBJECT_TITE: C2RustUnnamed_19 = 22;
pub const OBJECT_LINK_CHILD: C2RustUnnamed_19 = 21;
pub const OBJECT_LINK_BOY: C2RustUnnamed_19 = 20;
pub const OBJECT_NIW: C2RustUnnamed_19 = 19;
pub const OBJECT_BUBBLE: C2RustUnnamed_19 = 18;
pub const OBJECT_UNSET_11: C2RustUnnamed_19 = 17;
pub const OBJECT_UNSET_10: C2RustUnnamed_19 = 16;
pub const OBJECT_FIRE: C2RustUnnamed_19 = 15;
pub const OBJECT_BOX: C2RustUnnamed_19 = 14;
pub const OBJECT_FIREFLY: C2RustUnnamed_19 = 13;
pub const OBJECT_DODONGO: C2RustUnnamed_19 = 12;
pub const OBJECT_WALLMASTER: C2RustUnnamed_19 = 11;
pub const OBJECT_DY_OBJ: C2RustUnnamed_19 = 10;
pub const OBJECT_POH: C2RustUnnamed_19 = 9;
pub const OBJECT_CROW: C2RustUnnamed_19 = 8;
pub const OBJECT_OKUTA: C2RustUnnamed_19 = 7;
pub const OBJECT_HUMAN: C2RustUnnamed_19 = 6;
pub const OBJECT_UNSET_5: C2RustUnnamed_19 = 5;
pub const OBJECT_UNSET_4: C2RustUnnamed_19 = 4;
pub const OBJECT_GAMEPLAY_DANGEON_KEEP: C2RustUnnamed_19 = 3;
pub const OBJECT_GAMEPLAY_FIELD_KEEP: C2RustUnnamed_19 = 2;
pub const OBJECT_GAMEPLAY_KEEP: C2RustUnnamed_19 = 1;
pub const OBJECT_INVALID: C2RustUnnamed_19 = 0;
pub type C2RustUnnamed_20 = libc::c_uint;
pub const SEQ_PLAYER_BGM_SUB: C2RustUnnamed_20 = 3;
pub const SEQ_PLAYER_SFX: C2RustUnnamed_20 = 2;
pub const SEQ_PLAYER_FANFARE: C2RustUnnamed_20 = 1;
pub const SEQ_PLAYER_BGM_MAIN: C2RustUnnamed_20 = 0;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct InitChainEntry {
    #[bitfield(name = "cont", ty = "u32_0", bits = "0..=0")]
    #[bitfield(name = "type_0", ty = "u32_0", bits = "1..=4")]
    #[bitfield(name = "offset", ty = "u32_0", bits = "5..=15")]
    #[bitfield(name = "value", ty = "s32", bits = "16..=31")]
    pub cont_type_0_offset_value: [u8; 4],
}
pub type C2RustUnnamed_21 = libc::c_uint;
pub const ICHAINTYPE_VEC3S: C2RustUnnamed_21 = 10;
pub const ICHAINTYPE_VEC3F_DIV1000: C2RustUnnamed_21 = 9;
pub const ICHAINTYPE_VEC3F: C2RustUnnamed_21 = 8;
pub const ICHAINTYPE_F32_DIV1000: C2RustUnnamed_21 = 7;
pub const ICHAINTYPE_F32: C2RustUnnamed_21 = 6;
pub const ICHAINTYPE_S32: C2RustUnnamed_21 = 5;
pub const ICHAINTYPE_U32: C2RustUnnamed_21 = 4;
pub const ICHAINTYPE_S16: C2RustUnnamed_21 = 3;
pub const ICHAINTYPE_U16: C2RustUnnamed_21 = 2;
pub const ICHAINTYPE_S8: C2RustUnnamed_21 = 1;
pub const ICHAINTYPE_U8: C2RustUnnamed_21 = 0;
pub type C2RustUnnamed_22 = libc::c_uint;
pub const MTXMODE_APPLY: C2RustUnnamed_22 = 1;
pub const MTXMODE_NEW: C2RustUnnamed_22 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BossTw {
    pub actor: Actor,
    pub actionFunc: BossTwActionFunc,
    pub work: [s16; 13],
    pub unused_170: [libc::c_char; 14],
    pub timers: [s16; 5],
    pub workf: [f32_0; 20],
    pub fogR: f32_0,
    pub fogG: f32_0,
    pub fogB: f32_0,
    pub fogNear: f32_0,
    pub fogFar: f32_0,
    pub blastTailPos: [Vec3f; 50],
    pub csState1: s16,
    pub crownPos: Vec3f,
    pub scepterFlamePos: [Vec3f; 5],
    pub beamOrigin: Vec3f,
    pub leftScepterPos: Vec3f,
    pub rightScepterPos: Vec3f,
    pub targetPos: Vec3f,
    pub groundBlastPos2: Vec3f,
    pub rotateSpeed: f32_0,
    pub eyeTexIdx: s16,
    pub leftEyeTexIdx: s16,
    pub scepterAlpha: f32_0,
    pub flameAlpha: f32_0,
    pub spawnPortalAlpha: f32_0,
    pub unk_4DC: f32_0,
    pub spawnPortalScale: f32_0,
    pub updateRate1: f32_0,
    pub flameRotation: f32_0,
    pub portalRotation: f32_0,
    pub updateRate2: f32_0,
    pub twinrovaStun: u8_0,
    pub beamScale: f32_0,
    pub beamShootState: s16,
    pub groundBlastPos: Vec3f,
    pub beamReflectionOrigin: Vec3f,
    pub beamPitch: f32_0,
    pub beamYaw: f32_0,
    pub beamRoll: f32_0,
    pub magicDir: Vec3s,
    pub beamDist: f32_0,
    pub unk_530: Vec3f,
    pub beamReflectionPitch: f32_0,
    pub beamReflectionYaw: f32_0,
    pub unused_544: f32_0,
    pub beamReflectionDist: f32_0,
    pub unk_54C: Vec3f,
    pub unk_558: Vec3f,
    pub visible: u8_0,
    pub blastActive: u8_0,
    pub blastType: s16,
    pub skelAnime: SkelAnime,
    pub collider: ColliderCylinder,
    pub unk_5F8: u8_0,
    pub unk_5F9: u8_0,
    pub csState2: s16,
    pub subCamId: s16,
    pub csSfxTimer: s16,
    pub subCamEye: Vec3f,
    pub subCamAt: Vec3f,
    pub unused_618: [libc::c_char; 12],
    pub subCamEye2: Vec3f,
    pub subCamAt2: Vec3f,
    pub unused_63C: [libc::c_char; 24],
    pub subCamEyeStep: Vec3f,
    pub subCamAtStep: Vec3f,
    pub subCamEyeTarget: Vec3f,
    pub unused_678: [libc::c_char; 12],
    pub subCamAtTarget: Vec3f,
    pub unused_690: [libc::c_char; 12],
    pub subCamUpdateRate: f32_0,
    pub subCamDistStep: f32_0,
    pub subCamDist: f32_0,
    pub unused_6A8: [libc::c_char; 4],
    pub subCamYaw: f32_0,
    pub subCamYawStep: f32_0,
}
pub type BossTwActionFunc
    =
    Option<unsafe extern "C" fn(_: *mut BossTw, _: *mut GlobalContext) -> ()>;
pub type C2RustUnnamed_23 = libc::c_uint;
pub const WORK_MAX: C2RustUnnamed_23 = 13;
pub const BURN_TMR: C2RustUnnamed_23 = 12;
pub const PLAYED_CHRG_SFX: C2RustUnnamed_23 = 11;
pub const YAW_TGT: C2RustUnnamed_23 = 10;
pub const TW_BLINK_IDX: C2RustUnnamed_23 = 9;
pub const UNK_S8: C2RustUnnamed_23 = 8;
pub const CAN_SHOOT: C2RustUnnamed_23 = 7;
pub const FOG_TIMER: C2RustUnnamed_23 = 6;
pub const INVINC_TIMER: C2RustUnnamed_23 = 5;
pub const BLINK_IDX: C2RustUnnamed_23 = 4;
pub const TAIL_IDX: C2RustUnnamed_23 = 3;
pub const TW_PLLR_IDX: C2RustUnnamed_23 = 2;
pub const CS_TIMER_2: C2RustUnnamed_23 = 1;
pub const CS_TIMER_1: C2RustUnnamed_23 = 0;
pub type C2RustUnnamed_24 = libc::c_uint;
pub const FWORK_MAX: C2RustUnnamed_24 = 20;
pub const UNK_F19: C2RustUnnamed_24 = 19;
pub const UNK_F18: C2RustUnnamed_24 = 18;
pub const UNK_F17: C2RustUnnamed_24 = 17;
pub const UNK_F16: C2RustUnnamed_24 = 16;
pub const UNK_F15: C2RustUnnamed_24 = 15;
pub const UNK_F14: C2RustUnnamed_24 = 14;
pub const KM_GD_CRTR_SCL: C2RustUnnamed_24 = 13;
pub const UNK_F13: C2RustUnnamed_24 = 13;
pub const KM_GD_FLM_SCL: C2RustUnnamed_24 = 12;
pub const UNK_F12: C2RustUnnamed_24 = 12;
pub const KM_GRND_CRTR_A: C2RustUnnamed_24 = 11;
pub const UNK_F11: C2RustUnnamed_24 = 11;
pub const KM_GD_SMOKE_A: C2RustUnnamed_24 = 10;
pub const TAIL_ALPHA: C2RustUnnamed_24 = 10;
pub const UNK_F10: C2RustUnnamed_24 = 10;
pub const KM_GD_FLM_A: C2RustUnnamed_24 = 9;
pub const UNK_F9: C2RustUnnamed_24 = 9;
pub const ANIM_SW_TGT: C2RustUnnamed_24 = 8;
pub const INNR_CRWN_TX_Y2: C2RustUnnamed_24 = 7;
pub const INNR_CRWN_TX_Y1: C2RustUnnamed_24 = 6;
pub const OUTR_CRWN_TX_Y2: C2RustUnnamed_24 = 5;
pub const OUTR_CRWN_TX_Y1: C2RustUnnamed_24 = 4;
pub const INNR_CRWN_TX_X2: C2RustUnnamed_24 = 3;
pub const INNR_CRWN_TX_X1: C2RustUnnamed_24 = 2;
pub const OUTR_CRWN_TX_X2: C2RustUnnamed_24 = 1;
pub const OUTR_CRWN_TX_X1: C2RustUnnamed_24 = 0;
pub type C2RustUnnamed_25 = libc::c_int;
pub const WARP_RED: C2RustUnnamed_25 = 10;
pub const WARP_GREEN: C2RustUnnamed_25 = 9;
pub const WARP_ORANGE: C2RustUnnamed_25 = 8;
pub const WARP_UNK_7: C2RustUnnamed_25 = 7;
pub const WARP_DESTINATION: C2RustUnnamed_25 = 6;
pub const WARP_BLUE_RUTO: C2RustUnnamed_25 = 5;
pub const WARP_YELLOW: C2RustUnnamed_25 = 4;
pub const WARP_PURPLE_CRYSTAL: C2RustUnnamed_25 = 3;
pub const WARP_SAGES: C2RustUnnamed_25 = 2;
pub const WARP_CLEAR_FLAG: C2RustUnnamed_25 = 1;
pub const WARP_DUNGEON_CHILD: C2RustUnnamed_25 = 0;
pub const WARP_DUNGEON_ADULT: C2RustUnnamed_25 = -1;
pub const WARP_BLUE_CRYSTAL: C2RustUnnamed_25 = -2;
pub type C2RustUnnamed_26 = libc::c_uint;
pub const TWEFF_SHLD_HIT: C2RustUnnamed_26 = 10;
pub const TWEFF_SHLD_DEFL: C2RustUnnamed_26 = 9;
pub const TWEFF_SHLD_BLST: C2RustUnnamed_26 = 8;
pub const TWEFF_MERGEFLAME: C2RustUnnamed_26 = 7;
pub const TWEFF_FLAME: C2RustUnnamed_26 = 6;
pub const TWEFF_PLYR_FRZ: C2RustUnnamed_26 = 5;
pub const TWEFF_RING: C2RustUnnamed_26 = 4;
pub const TWEFF_3: C2RustUnnamed_26 = 3;
pub const TWEFF_2: C2RustUnnamed_26 = 2;
pub const TWEFF_DOT: C2RustUnnamed_26 = 1;
pub const TWEFF_NONE: C2RustUnnamed_26 = 0;
pub type C2RustUnnamed_27 = libc::c_uint;
pub const EFF_WORK_MAX: C2RustUnnamed_27 = 2;
pub const EFF_UNKS1: C2RustUnnamed_27 = 1;
pub const EFF_ARGS: C2RustUnnamed_27 = 0;
pub type C2RustUnnamed_28 = libc::c_uint;
pub const EFF_FWORK_MAX: C2RustUnnamed_28 = 4;
pub const EFF_YAW: C2RustUnnamed_28 = 3;
pub const EFF_ROLL: C2RustUnnamed_28 = 2;
pub const EFF_DIST: C2RustUnnamed_28 = 1;
pub const EFF_SCALE: C2RustUnnamed_28 = 0;
pub type C2RustUnnamed_29 = libc::c_uint;
pub const TW_DEATHBALL_KOUME: C2RustUnnamed_29 = 105;
pub const TW_DEATHBALL_KOTAKE: C2RustUnnamed_29 = 104;
pub const TW_ICE_BLAST_GROUND: C2RustUnnamed_29 = 103;
pub const TW_ICE_BLAST: C2RustUnnamed_29 = 102;
pub const TW_FIRE_BLAST_GROUND: C2RustUnnamed_29 = 101;
pub const TW_FIRE_BLAST: C2RustUnnamed_29 = 100;
pub const TW_TWINROVA: C2RustUnnamed_29 = 2;
pub const TW_KOUME: C2RustUnnamed_29 = 1;
pub const TW_KOTAKE: C2RustUnnamed_29 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BossTwEffect {
    pub type_0: u8_0,
    pub frame: u8_0,
    pub pos: Vec3f,
    pub curSpeed: Vec3f,
    pub accel: Vec3f,
    pub color: Color_RGB8,
    pub alpha: s16,
    pub work: [s16; 2],
    pub workf: [f32_0; 4],
    pub target: *mut Actor,
}
#[no_mangle]
pub static mut Boss_Tw_InitVars: ActorInit =
    unsafe {
        {
            let mut init =
                ActorInit{id: ACTOR_BOSS_TW as libc::c_int as s16,
                          category: ACTORCAT_BOSS as libc::c_int as u8_0,
                          flags:
                              ((1 as libc::c_int) << 0 as libc::c_int |
                                   (1 as libc::c_int) << 2 as libc::c_int |
                                   (1 as libc::c_int) << 4 as libc::c_int |
                                   (1 as libc::c_int) << 5 as libc::c_int) as
                                  u32_0,
                          objectId: OBJECT_TW as libc::c_int as s16,
                          instanceSize:
                              ::std::mem::size_of::<BossTw>() as
                                  libc::c_ulong,
                          init:
                              ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                      *mut Actor,
                                                                                  _:
                                                                                      *mut GlobalContext)
                                                                 -> ()>,
                                                      ActorFunc>(Some(BossTw_Init
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
                                                      ActorFunc>(Some(BossTw_Destroy
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
                                                      ActorFunc>(Some(BossTw_Update
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
                                                      ActorFunc>(Some(BossTw_Draw
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
static mut D_8094A7D0: Vec3f =
    { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 1000.0f32,}; init };
static mut sZeroVector: Vec3f =
    { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
static mut sCylinderInitBlasts: ColliderCylinderInit =
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
                                                                                               0x100000
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
                                                            25 as libc::c_int
                                                                as s16,
                                                        height:
                                                            35 as libc::c_int
                                                                as s16,
                                                        yShift:
                                                            -(17 as
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
static mut sCylinderInitKoumeKotake: ColliderCylinderInit =
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
                                                                                            0x20
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
                                                                                               0xffcdfffe
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
                                                            45 as libc::c_int
                                                                as s16,
                                                        height:
                                                            120 as libc::c_int
                                                                as s16,
                                                        yShift:
                                                            -(30 as
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
static mut sCylinderInitTwinrova: ColliderCylinderInit =
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
                                                                                            0x20
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
                                                                                               0xffcdfffe
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
                                                            45 as libc::c_int
                                                                as s16,
                                                        height:
                                                            120 as libc::c_int
                                                                as s16,
                                                        yShift:
                                                            -(30 as
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
static mut sTwinrovaPillarPos: [Vec3f; 4] =
    [{ let mut init = Vec3f{x: 580.0f32, y: 380.0f32, z: 0.0f32,}; init },
     { let mut init = Vec3f{x: 0.0f32, y: 380.0f32, z: 580.0f32,}; init },
     { let mut init = Vec3f{x: -580.0f32, y: 380.0f32, z: 0.0f32,}; init },
     { let mut init = Vec3f{x: 0.0f32, y: 380.0f32, z: -580.0f32,}; init }];
static mut sTwInitalized: u8_0 = 0 as libc::c_int as u8_0;
// Initialized in run_static_initializers
static mut sInitChain: [InitChainEntry; 3] =
    [InitChainEntry{cont_type_0_offset_value: [0; 4],}; 3];
static mut sEnvType: s8 = 0;
static mut sGroundBlastType: u8_0 = 0;
static mut sKotakePtr: *mut BossTw = 0 as *const BossTw as *mut BossTw;
static mut sKoumePtr: *mut BossTw = 0 as *const BossTw as *mut BossTw;
static mut sTwinrovaPtr: *mut BossTw = 0 as *const BossTw as *mut BossTw;
static mut sShieldFireCharge: u8_0 = 0;
static mut sShieldIceCharge: u8_0 = 0;
static mut D_8094C854: f32_0 = 0.;
static mut D_8094C858: f32_0 = 0.;
static mut sTwinrovaBlastType: u8_0 = 0;
static mut sFixedBlastType: u8_0 = 0;
static mut sFixedBlatSeq: u8_0 = 0;
static mut sFreezeState: u8_0 = 0;
static mut sShieldHitPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
static mut sShieldHitYaw: s16 = 0;
static mut sBeamDivertTimer: u8_0 = 0;
static mut D_8094C86F: u8_0 = 0;
static mut D_8094C870: u8_0 = 0;
static mut D_8094C872: s16 = 0;
static mut D_8094C874: s16 = 0;
static mut D_8094C876: s16 = 0;
static mut D_8094C878: u8_0 = 0;
static mut D_8094C87A: s16 = 0;
static mut D_8094C87C: s16 = 0;
static mut D_8094C87E: u8_0 = 0;
static mut sTwEffects: [BossTwEffect; 150] =
    [BossTwEffect{type_0: 0,
                  frame: 0,
                  pos: Vec3f{x: 0., y: 0., z: 0.,},
                  curSpeed: Vec3f{x: 0., y: 0., z: 0.,},
                  accel: Vec3f{x: 0., y: 0., z: 0.,},
                  color: Color_RGB8{r: 0, g: 0, b: 0,},
                  alpha: 0,
                  work: [0; 2],
                  workf: [0.; 4],
                  target: 0 as *const Actor as *mut Actor,}; 150];
#[no_mangle]
pub unsafe extern "C" fn BossTw_AddDotEffect(mut globalCtx:
                                                 *mut GlobalContext,
                                             mut initalPos: *mut Vec3f,
                                             mut initalSpeed: *mut Vec3f,
                                             mut accel: *mut Vec3f,
                                             mut scale: f32_0, mut args: s16,
                                             mut countLimit: s16) {
    let mut i: s16 = 0;
    let mut eff: *mut BossTwEffect = 0 as *mut BossTwEffect;
    i = 0 as libc::c_int as s16;
    eff = (*globalCtx).specialEffects as *mut BossTwEffect;
    while (i as libc::c_int) < countLimit as libc::c_int {
        if (*eff).type_0 as libc::c_int == TWEFF_NONE as libc::c_int {
            (*eff).type_0 = TWEFF_DOT as libc::c_int as u8_0;
            (*eff).pos = *initalPos;
            (*eff).curSpeed = *initalSpeed;
            (*eff).accel = *accel;
            (*eff).workf[EFF_SCALE as libc::c_int as usize] =
                scale / 1000.0f32;
            (*eff).alpha = 255 as libc::c_int as s16;
            (*eff).frame = Rand_ZeroFloat(10.0f32) as s16 as u8_0;
            (*eff).work[EFF_ARGS as libc::c_int as usize] = args;
            break ;
        } else { i += 1; eff = eff.offset(1) }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_AddDmgCloud(mut globalCtx: *mut GlobalContext,
                                            mut type_0: s16,
                                            mut initialPos: *mut Vec3f,
                                            mut initalSpeed: *mut Vec3f,
                                            mut accel: *mut Vec3f,
                                            mut scale: f32_0, mut alpha: s16,
                                            mut args: s16,
                                            mut countLimit: s16) {
    let mut i: s16 = 0;
    let mut eff: *mut BossTwEffect = 0 as *mut BossTwEffect;
    i = 0 as libc::c_int as s16;
    eff = (*globalCtx).specialEffects as *mut BossTwEffect;
    while (i as libc::c_int) < countLimit as libc::c_int {
        if (*eff).type_0 as libc::c_int == TWEFF_NONE as libc::c_int {
            (*eff).type_0 = type_0 as u8_0;
            (*eff).pos = *initialPos;
            (*eff).curSpeed = *initalSpeed;
            (*eff).accel = *accel;
            (*eff).workf[EFF_SCALE as libc::c_int as usize] =
                scale / 1000.0f32;
            (*eff).work[EFF_ARGS as libc::c_int as usize] = args;
            (*eff).alpha = alpha;
            (*eff).frame = Rand_ZeroFloat(100.0f32) as s16 as u8_0;
            break ;
        } else { i += 1; eff = eff.offset(1) }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_AddRingEffect(mut globalCtx:
                                                  *mut GlobalContext,
                                              mut initalPos: *mut Vec3f,
                                              mut scale: f32_0,
                                              mut arg3: f32_0, mut alpha: s16,
                                              mut args: s16, mut arg6: s16,
                                              mut arg7: s16) {
    let mut i: s16 = 0;
    let mut eff: *mut BossTwEffect = 0 as *mut BossTwEffect;
    i = 0 as libc::c_int as s16;
    eff = (*globalCtx).specialEffects as *mut BossTwEffect;
    while (i as libc::c_int) < arg7 as libc::c_int {
        if (*eff).type_0 as libc::c_int == TWEFF_NONE as libc::c_int {
            (*eff).type_0 = TWEFF_RING as libc::c_int as u8_0;
            (*eff).pos = *initalPos;
            (*eff).curSpeed = sZeroVector;
            (*eff).accel = sZeroVector;
            (*eff).workf[EFF_SCALE as libc::c_int as usize] =
                scale * 0.0025f32;
            (*eff).workf[EFF_DIST as libc::c_int as usize] = arg3 * 0.0025f32;
            (*eff).work[EFF_ARGS as libc::c_int as usize] = args;
            (*eff).work[EFF_UNKS1 as libc::c_int as usize] = arg6;
            (*eff).alpha = alpha;
            (*eff).workf[EFF_ROLL as libc::c_int as usize] =
                Rand_ZeroFloat(3.14159265358979323846f32);
            (*eff).frame = 0 as libc::c_int as u8_0;
            break ;
        } else { i += 1; eff = eff.offset(1) }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_AddPlayerFreezeEffect(mut globalCtx:
                                                          *mut GlobalContext,
                                                      mut target:
                                                          *mut Actor) {
    let mut eff: *mut BossTwEffect = 0 as *mut BossTwEffect;
    let mut i: s16 = 0;
    eff = (*globalCtx).specialEffects as *mut BossTwEffect;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[BossTwEffect; 150]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<BossTwEffect>()
                                                   as libc::c_ulong) as s32 {
        if (*eff).type_0 as libc::c_int == TWEFF_NONE as libc::c_int {
            (*eff).type_0 = TWEFF_PLYR_FRZ as libc::c_int as u8_0;
            (*eff).curSpeed = sZeroVector;
            (*eff).accel = sZeroVector;
            (*eff).frame = 0 as libc::c_int as u8_0;
            (*eff).target = target;
            (*eff).workf[EFF_DIST as libc::c_int as usize] = 0.0f32;
            (*eff).workf[EFF_SCALE as libc::c_int as usize] = 0.0f32;
            (*eff).workf[EFF_ROLL as libc::c_int as usize] = 0.0f32;
            if target.is_null() {
                (*eff).work[EFF_ARGS as libc::c_int as usize] =
                    100 as libc::c_int as s16
            } else {
                (*eff).work[EFF_ARGS as libc::c_int as usize] =
                    20 as libc::c_int as s16
            }
            break ;
        } else { i += 1; eff = eff.offset(1) }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_AddFlameEffect(mut globalCtx:
                                                   *mut GlobalContext,
                                               mut initalPos: *mut Vec3f,
                                               mut initalSpeed: *mut Vec3f,
                                               mut accel: *mut Vec3f,
                                               mut scale: f32_0,
                                               mut args: s16) {
    let mut i: s16 = 0;
    let mut eff: *mut BossTwEffect = 0 as *mut BossTwEffect;
    i = 0 as libc::c_int as s16;
    eff = (*globalCtx).specialEffects as *mut BossTwEffect;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[BossTwEffect; 150]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<BossTwEffect>()
                                                   as libc::c_ulong) as s32 {
        if (*eff).type_0 as libc::c_int == TWEFF_NONE as libc::c_int {
            (*eff).type_0 = TWEFF_FLAME as libc::c_int as u8_0;
            (*eff).pos = *initalPos;
            (*eff).curSpeed = *initalSpeed;
            (*eff).accel = *accel;
            (*eff).workf[EFF_SCALE as libc::c_int as usize] =
                scale / 1000.0f32;
            (*eff).work[EFF_ARGS as libc::c_int as usize] = args;
            (*eff).work[EFF_UNKS1 as libc::c_int as usize] =
                0 as libc::c_int as s16;
            (*eff).alpha = 0 as libc::c_int as s16;
            (*eff).frame = Rand_ZeroFloat(1000.0f32) as s16 as u8_0;
            break ;
        } else { i += 1; eff = eff.offset(1) }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_AddMergeFlameEffect(mut globalCtx:
                                                        *mut GlobalContext,
                                                    mut initialPos:
                                                        *mut Vec3f,
                                                    mut scale: f32_0,
                                                    mut dist: f32_0,
                                                    mut args: s16) {
    let mut i: s16 = 0;
    let mut eff: *mut BossTwEffect = 0 as *mut BossTwEffect;
    i = 0 as libc::c_int as s16;
    eff = (*globalCtx).specialEffects as *mut BossTwEffect;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[BossTwEffect; 150]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<BossTwEffect>()
                                                   as libc::c_ulong) as s32 {
        if (*eff).type_0 as libc::c_int == TWEFF_NONE as libc::c_int {
            (*eff).type_0 = TWEFF_MERGEFLAME as libc::c_int as u8_0;
            (*eff).pos = *initialPos;
            (*eff).curSpeed = sZeroVector;
            (*eff).accel = sZeroVector;
            (*eff).workf[EFF_SCALE as libc::c_int as usize] =
                scale / 1000.0f32;
            (*eff).work[EFF_ARGS as libc::c_int as usize] = args;
            (*eff).work[EFF_UNKS1 as libc::c_int as usize] =
                0 as libc::c_int as s16;
            (*eff).workf[EFF_DIST as libc::c_int as usize] = dist;
            (*eff).workf[EFF_ROLL as libc::c_int as usize] =
                Rand_ZeroFloat(2.0f32 * 3.14159265358979323846f32);
            (*eff).alpha = 0 as libc::c_int as s16;
            (*eff).frame = Rand_ZeroFloat(1000.0f32) as s16 as u8_0;
            break ;
        } else { i += 1; eff = eff.offset(1) }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_AddShieldBlastEffect(mut globalCtx:
                                                         *mut GlobalContext,
                                                     mut initalPos:
                                                         *mut Vec3f,
                                                     mut initalSpeed:
                                                         *mut Vec3f,
                                                     mut accel: *mut Vec3f,
                                                     mut scale: f32_0,
                                                     mut arg5: f32_0,
                                                     mut alpha: s16,
                                                     mut args: s16) {
    let mut i: s16 = 0;
    let mut eff: *mut BossTwEffect = 0 as *mut BossTwEffect;
    i = 0 as libc::c_int as s16;
    eff = (*globalCtx).specialEffects as *mut BossTwEffect;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[BossTwEffect; 150]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<BossTwEffect>()
                                                   as libc::c_ulong) as s32 {
        if (*eff).type_0 as libc::c_int == TWEFF_NONE as libc::c_int {
            (*eff).type_0 = TWEFF_SHLD_BLST as libc::c_int as u8_0;
            (*eff).pos = *initalPos;
            (*eff).curSpeed = *initalSpeed;
            (*eff).accel = *accel;
            (*eff).workf[EFF_SCALE as libc::c_int as usize] =
                scale / 1000.0f32;
            (*eff).workf[EFF_DIST as libc::c_int as usize] = arg5 / 1000.0f32;
            (*eff).work[EFF_ARGS as libc::c_int as usize] = args;
            (*eff).work[EFF_UNKS1 as libc::c_int as usize] =
                0 as libc::c_int as s16;
            (*eff).alpha = alpha;
            (*eff).frame = Rand_ZeroFloat(1000.0f32) as s16 as u8_0;
            break ;
        } else { i += 1; eff = eff.offset(1) }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_AddShieldDeflectEffect(mut globalCtx:
                                                           *mut GlobalContext,
                                                       mut arg1: f32_0,
                                                       mut arg2: s16) {
    let mut i: s16 = 0;
    let mut j: s16 = 0;
    let mut eff: *mut BossTwEffect = 0 as *mut BossTwEffect;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    sShieldHitPos = (*player).bodyPartsPos[15 as libc::c_int as usize];
    sShieldHitYaw = (*player).actor.shape.rot.y;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < 8 as libc::c_int {
        eff = (*globalCtx).specialEffects as *mut BossTwEffect;
        j = 0 as libc::c_int as s16;
        while (j as libc::c_int) <
                  (::std::mem::size_of::<[BossTwEffect; 150]>() as
                       libc::c_ulong).wrapping_div(::std::mem::size_of::<BossTwEffect>()
                                                       as libc::c_ulong) as
                      s32 {
            if (*eff).type_0 as libc::c_int == TWEFF_NONE as libc::c_int {
                (*eff).type_0 = TWEFF_SHLD_DEFL as libc::c_int as u8_0;
                (*eff).pos = sShieldHitPos;
                (*eff).curSpeed = sZeroVector;
                (*eff).accel = sZeroVector;
                (*eff).workf[EFF_ROLL as libc::c_int as usize] =
                    i as libc::c_int as libc::c_float *
                        (3.14159265358979323846f32 / 4.0f32);
                (*eff).workf[EFF_YAW as libc::c_int as usize] =
                    3.14159265358979323846f32 / 2.0f32;
                (*eff).workf[EFF_DIST as libc::c_int as usize] = 0.0f32;
                (*eff).workf[EFF_SCALE as libc::c_int as usize] =
                    arg1 / 1000.0f32;
                (*eff).work[EFF_ARGS as libc::c_int as usize] = arg2;
                (*eff).work[EFF_UNKS1 as libc::c_int as usize] =
                    0 as libc::c_int as s16;
                (*eff).alpha = 255 as libc::c_int as s16;
                (*eff).frame = Rand_ZeroFloat(1000.0f32) as s16 as u8_0;
                break ;
            } else { j += 1; eff = eff.offset(1) }
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_AddShieldHitEffect(mut globalCtx:
                                                       *mut GlobalContext,
                                                   mut arg1: f32_0,
                                                   mut arg2: s16) {
    let mut i: s16 = 0;
    let mut j: s16 = 0;
    let mut eff: *mut BossTwEffect = 0 as *mut BossTwEffect;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    sShieldHitPos = (*player).bodyPartsPos[15 as libc::c_int as usize];
    sShieldHitYaw = (*player).actor.shape.rot.y;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < 8 as libc::c_int {
        eff = (*globalCtx).specialEffects as *mut BossTwEffect;
        j = 0 as libc::c_int as s16;
        while (j as libc::c_int) <
                  (::std::mem::size_of::<[BossTwEffect; 150]>() as
                       libc::c_ulong).wrapping_div(::std::mem::size_of::<BossTwEffect>()
                                                       as libc::c_ulong) as
                      s32 {
            if (*eff).type_0 as libc::c_int == TWEFF_NONE as libc::c_int {
                (*eff).type_0 = TWEFF_SHLD_HIT as libc::c_int as u8_0;
                (*eff).pos = sShieldHitPos;
                (*eff).curSpeed = sZeroVector;
                (*eff).accel = sZeroVector;
                (*eff).workf[EFF_ROLL as libc::c_int as usize] =
                    i as libc::c_int as libc::c_float *
                        (3.14159265358979323846f32 / 4.0f32);
                (*eff).workf[EFF_YAW as libc::c_int as usize] =
                    3.14159265358979323846f32 / 2.0f32;
                (*eff).workf[EFF_DIST as libc::c_int as usize] = 0.0f32;
                (*eff).workf[EFF_SCALE as libc::c_int as usize] =
                    arg1 / 1000.0f32;
                (*eff).work[EFF_ARGS as libc::c_int as usize] = arg2;
                (*eff).work[EFF_UNKS1 as libc::c_int as usize] =
                    0 as libc::c_int as s16;
                (*eff).alpha = 255 as libc::c_int as s16;
                (*eff).frame = Rand_ZeroFloat(1000.0f32) as s16 as u8_0;
                break ;
            } else { j += 1; eff = eff.offset(1) }
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_Init(mut thisx: *mut Actor,
                                     mut globalCtx2: *mut GlobalContext) {
    let mut globalCtx: *mut GlobalContext = globalCtx2;
    let mut this: *mut BossTw = thisx as *mut BossTw;
    let mut i: s16 = 0;
    Actor_ProcessInitChain(&mut (*this).actor, sInitChain.as_mut_ptr());
    ActorShape_Init(&mut (*this).actor.shape, 0.0f32, None, 0.0f32);
    if (*this).actor.params as libc::c_int >= TW_FIRE_BLAST as libc::c_int {
        // Blasts
        Actor_SetScale(&mut (*this).actor, 0.01f32);
        (*this).actor.update =
            Some(BossTw_BlastUpdate as
                     unsafe extern "C" fn(_: *mut Actor,
                                          _: *mut GlobalContext) -> ());
        (*this).actor.draw =
            Some(BossTw_BlastDraw as
                     unsafe extern "C" fn(_: *mut Actor,
                                          _: *mut GlobalContext) -> ());
        (*this).actor.flags &=
            !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
        Collider_InitCylinder(globalCtx, &mut (*this).collider);
        Collider_SetCylinder(globalCtx, &mut (*this).collider,
                             &mut (*this).actor, &mut sCylinderInitBlasts);
        if (*this).actor.params as libc::c_int == TW_FIRE_BLAST as libc::c_int
               ||
               (*this).actor.params as libc::c_int ==
                   TW_FIRE_BLAST_GROUND as libc::c_int {
            (*this).actionFunc =
                Some(BossTw_BlastFire as
                         unsafe extern "C" fn(_: *mut BossTw,
                                              _: *mut GlobalContext) -> ());
            (*this).collider.info.toucher.effect = 1 as libc::c_int as u8_0
        } else if (*this).actor.params as libc::c_int ==
                      TW_ICE_BLAST as libc::c_int ||
                      (*this).actor.params as libc::c_int ==
                          TW_ICE_BLAST_GROUND as libc::c_int {
            (*this).actionFunc =
                Some(BossTw_BlastIce as
                         unsafe extern "C" fn(_: *mut BossTw,
                                              _: *mut GlobalContext) -> ())
        } else if (*this).actor.params as libc::c_int >=
                      TW_DEATHBALL_KOTAKE as libc::c_int {
            (*this).actionFunc =
                Some(BossTw_DeathBall as
                         unsafe extern "C" fn(_: *mut BossTw,
                                              _: *mut GlobalContext) -> ());
            (*this).actor.draw =
                Some(BossTw_DrawDeathBall as
                         unsafe extern "C" fn(_: *mut Actor,
                                              _: *mut GlobalContext) -> ());
            (*this).workf[TAIL_ALPHA as libc::c_int as usize] = 128.0f32;
            if (*thisx).params as libc::c_int ==
                   TW_DEATHBALL_KOTAKE as libc::c_int {
                (*thisx).world.rot.y =
                    ((*sTwinrovaPtr).actor.world.rot.y as libc::c_int +
                         0x4000 as libc::c_int) as s16
            } else {
                (*thisx).world.rot.y =
                    ((*sTwinrovaPtr).actor.world.rot.y as libc::c_int -
                         0x4000 as libc::c_int) as s16
            }
        }
        (*this).timers[1 as libc::c_int as usize] = 150 as libc::c_int as s16;
        return
    }
    Actor_SetScale(&mut (*this).actor,
                   (2.5f64 * 0.01f32 as libc::c_double) as f32_0);
    (*this).actor.colChkInfo.mass = 255 as libc::c_int as u8_0;
    (*this).actor.colChkInfo.health = 0 as libc::c_int as u8_0;
    Collider_InitCylinder(globalCtx, &mut (*this).collider);
    if sTwInitalized == 0 {
        sTwInitalized = 1 as libc::c_int as u8_0;
        (*globalCtx).envCtx.unk_BF = 1 as libc::c_int as u8_0;
        (*globalCtx).envCtx.unk_BE = 1 as libc::c_int as u8_0;
        (*globalCtx).envCtx.unk_BD = 1 as libc::c_int as u8_0;
        (*globalCtx).envCtx.unk_D8 = 0.0f32;
        sShieldIceCharge = 0 as libc::c_int as u8_0;
        sShieldFireCharge = sShieldIceCharge;
        sFixedBlatSeq = sShieldFireCharge;
        sTwinrovaBlastType = sFixedBlatSeq;
        sFreezeState = sTwinrovaBlastType;
        sGroundBlastType = sFreezeState;
        sEnvType = sGroundBlastType as s8;
        sBeamDivertTimer = sEnvType as u8_0;
        D_8094C872 = sBeamDivertTimer as s16;
        D_8094C86F = D_8094C872 as u8_0;
        D_8094C870 = D_8094C86F;
        D_8094C87E = D_8094C870;
        D_8094C87C = D_8094C87E as s16;
        D_8094C87A = D_8094C87C;
        D_8094C878 = D_8094C87A as u8_0;
        D_8094C876 = D_8094C878 as s16;
        D_8094C874 = D_8094C876;
        D_8094C854 = 0.0f32;
        D_8094C858 = D_8094C854;
        sFixedBlastType = Rand_ZeroFloat(1.99f32) as u8_0;
        (*globalCtx).specialEffects =
            sTwEffects.as_mut_ptr() as *mut libc::c_void;
        i = 0 as libc::c_int as s16;
        while (i as libc::c_int) <
                  (::std::mem::size_of::<[BossTwEffect; 150]>() as
                       libc::c_ulong).wrapping_div(::std::mem::size_of::<BossTwEffect>()
                                                       as libc::c_ulong) as
                      s32 {
            sTwEffects[i as usize].type_0 = TWEFF_NONE as libc::c_int as u8_0;
            i += 1
        }
    }
    if (*this).actor.params as libc::c_int == TW_KOTAKE as libc::c_int {
        Collider_SetCylinder(globalCtx, &mut (*this).collider,
                             &mut (*this).actor,
                             &mut sCylinderInitKoumeKotake);
        (*this).actor.naviEnemyId = 0x33 as libc::c_int as u8_0;
        SkelAnime_InitFlex(globalCtx, &mut (*this).skelAnime,
                           &mut object_tw_Skel_0070E0,
                           &mut object_tw_Anim_006F28, 0 as *mut Vec3s,
                           0 as *mut Vec3s, 0 as libc::c_int);
        if gSaveContext.eventChkInf[7 as libc::c_int as usize] as libc::c_int
               & 0x20 as libc::c_int != 0 {
            // began twinrova battle
            BossTw_SetupFlyTo(this, globalCtx);
            (*this).actor.world.pos.x = -600.0f32;
            (*this).actor.world.pos.y = 400.0f32;
            (*this).actor.world.pos.z = 0.0f32;
            Audio_QueueSeqCmd(((SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                                   24 as libc::c_int | 0x1b as libc::c_int) as
                                  u32_0);
        } else { BossTw_SetupCSWait(this, globalCtx); }
        Animation_MorphToLoop(&mut (*this).skelAnime,
                              &mut object_tw_Anim_006F28, -3.0f32);
        (*this).visible = 1 as libc::c_int as u8_0
    } else if (*this).actor.params as libc::c_int == TW_KOUME as libc::c_int {
        Collider_SetCylinder(globalCtx, &mut (*this).collider,
                             &mut (*this).actor,
                             &mut sCylinderInitKoumeKotake);
        (*this).actor.naviEnemyId = 0x32 as libc::c_int as u8_0;
        SkelAnime_InitFlex(globalCtx, &mut (*this).skelAnime,
                           &mut object_tw_Skel_01F888,
                           &mut object_tw_Anim_006F28, 0 as *mut Vec3s,
                           0 as *mut Vec3s, 0 as libc::c_int);
        if gSaveContext.eventChkInf[7 as libc::c_int as usize] as libc::c_int
               & 0x20 as libc::c_int != 0 {
            // began twinrova battle
            BossTw_SetupFlyTo(this, globalCtx);
            (*this).actor.world.pos.x = 600.0f32;
            (*this).actor.world.pos.y = 400.0f32;
            (*this).actor.world.pos.z = 0.0f32
        } else { BossTw_SetupCSWait(this, globalCtx); }
        Animation_MorphToLoop(&mut (*this).skelAnime,
                              &mut object_tw_Anim_006F28, -3.0f32);
        (*this).visible = 1 as libc::c_int as u8_0
    } else {
        // Twinrova
        Collider_SetCylinder(globalCtx, &mut (*this).collider,
                             &mut (*this).actor, &mut sCylinderInitTwinrova);
        (*this).actor.naviEnemyId = 0x5b as libc::c_int as u8_0;
        (*this).actor.colChkInfo.health = 24 as libc::c_int as u8_0;
        (*this).actor.update =
            Some(BossTw_TwinrovaUpdate as
                     unsafe extern "C" fn(_: *mut Actor,
                                          _: *mut GlobalContext) -> ());
        (*this).actor.draw =
            Some(BossTw_TwinrovaDraw as
                     unsafe extern "C" fn(_: *mut Actor,
                                          _: *mut GlobalContext) -> ());
        SkelAnime_InitFlex(globalCtx, &mut (*this).skelAnime,
                           &mut object_tw_Skel_032020,
                           &mut object_tw_Anim_0244B4, 0 as *mut Vec3s,
                           0 as *mut Vec3s, 0 as libc::c_int);
        Animation_MorphToLoop(&mut (*this).skelAnime,
                              &mut object_tw_Anim_0244B4, -3.0f32);
        if gSaveContext.eventChkInf[7 as libc::c_int as usize] as libc::c_int
               & 0x20 as libc::c_int != 0 {
            // began twinrova battle
            BossTw_SetupWait(this, globalCtx);
        } else {
            BossTw_TwinrovaSetupIntroCS(this, globalCtx);
            (*this).actor.world.pos.x = 0.0f32;
            (*this).actor.world.pos.y = 1000.0f32;
            (*this).actor.world.pos.z = 0.0f32
        }
        (*this).actor.params = TW_TWINROVA as libc::c_int as s16;
        sTwinrovaPtr = this;
        if Flags_GetClear(globalCtx, (*globalCtx).roomCtx.curRoom.num as s32)
               != 0 {
            // twinrova has been defeated.
            Actor_Kill(&mut (*this).actor);
            Actor_SpawnAsChild(&mut (*globalCtx).actorCtx, &mut (*this).actor,
                               globalCtx,
                               ACTOR_DOOR_WARP1 as libc::c_int as s16,
                               600.0f32, 230.0f32, 0.0f32,
                               0 as libc::c_int as s16,
                               0 as libc::c_int as s16,
                               0 as libc::c_int as s16,
                               WARP_DUNGEON_ADULT as libc::c_int as s16);
            Actor_Spawn(&mut (*globalCtx).actorCtx, globalCtx,
                        ACTOR_ITEM_B_HEART as libc::c_int as s16, -600.0f32,
                        230.0f32, 0.0f32, 0 as libc::c_int as s16,
                        0 as libc::c_int as s16, 0 as libc::c_int as s16,
                        0 as libc::c_int as s16);
        } else {
            sKotakePtr =
                Actor_SpawnAsChild(&mut (*globalCtx).actorCtx,
                                   &mut (*this).actor, globalCtx,
                                   ACTOR_BOSS_TW as libc::c_int as s16,
                                   (*this).actor.world.pos.x,
                                   (*this).actor.world.pos.y,
                                   (*this).actor.world.pos.z,
                                   0 as libc::c_int as s16,
                                   0 as libc::c_int as s16,
                                   0 as libc::c_int as s16,
                                   TW_KOTAKE as libc::c_int as s16) as
                    *mut BossTw;
            sKoumePtr =
                Actor_SpawnAsChild(&mut (*globalCtx).actorCtx,
                                   &mut (*this).actor, globalCtx,
                                   ACTOR_BOSS_TW as libc::c_int as s16,
                                   (*this).actor.world.pos.x,
                                   (*this).actor.world.pos.y,
                                   (*this).actor.world.pos.z,
                                   0 as libc::c_int as s16,
                                   0 as libc::c_int as s16,
                                   0 as libc::c_int as s16,
                                   TW_KOUME as libc::c_int as s16) as
                    *mut BossTw;
            (*sKotakePtr).actor.parent = &mut (*sKoumePtr).actor;
            (*sKoumePtr).actor.parent = &mut (*sKotakePtr).actor
        }
    }
    (*this).fogR =
        (*globalCtx).lightCtx.fogColor[0 as libc::c_int as usize] as f32_0;
    (*this).fogG =
        (*globalCtx).lightCtx.fogColor[1 as libc::c_int as usize] as f32_0;
    (*this).fogB =
        (*globalCtx).lightCtx.fogColor[2 as libc::c_int as usize] as f32_0;
    (*this).fogNear = (*globalCtx).lightCtx.fogNear as f32_0;
    (*this).fogFar = 1000.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_Destroy(mut thisx: *mut Actor,
                                        mut globalCtx: *mut GlobalContext) {
    let mut this: *mut BossTw = thisx as *mut BossTw;
    Collider_DestroyCylinder(globalCtx, &mut (*this).collider);
    if ((*thisx).params as libc::c_int) < TW_FIRE_BLAST as libc::c_int {
        SkelAnime_Free(&mut (*this).skelAnime, globalCtx);
    }
    if (*thisx).params as libc::c_int == TW_TWINROVA as libc::c_int {
        sTwInitalized = 0 as libc::c_int as u8_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_SetupTurnToPlayer(mut this: *mut BossTw,
                                                  mut globalCtx:
                                                      *mut GlobalContext) {
    let mut otherTw: *mut BossTw = (*this).actor.parent as *mut BossTw;
    (*this).actionFunc =
        Some(BossTw_TurnToPlayer as
                 unsafe extern "C" fn(_: *mut BossTw, _: *mut GlobalContext)
                     -> ());
    if !otherTw.is_null() &&
           (*otherTw).actionFunc ==
               Some(BossTw_ShootBeam as
                        unsafe extern "C" fn(_: *mut BossTw,
                                             _: *mut GlobalContext) -> ()) {
        (*this).timers[0 as libc::c_int as usize] = 40 as libc::c_int as s16
    } else {
        (*this).timers[0 as libc::c_int as usize] = 60 as libc::c_int as s16
    }
    (*this).rotateSpeed = 0.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_TurnToPlayer(mut this: *mut BossTw,
                                             mut globalCtx:
                                                 *mut GlobalContext) {
    let mut otherTw: *mut BossTw = (*this).actor.parent as *mut BossTw;
    SkelAnime_Update(&mut (*this).skelAnime);
    Math_ApproachF(&mut (*this).actor.speedXZ, 0.0f32, 1.0f32, 1.0f32);
    Math_ApproachS(&mut (*this).actor.shape.rot.y,
                   (*this).actor.yawTowardsPlayer, 5 as libc::c_int as s16,
                   (*this).rotateSpeed as s16);
    Math_ApproachS(&mut (*this).actor.shape.rot.x, 0 as libc::c_int as s16,
                   5 as libc::c_int as s16, (*this).rotateSpeed as s16);
    Math_ApproachF(&mut (*this).rotateSpeed, 4096.0f32, 1.0f32, 200.0f32);
    func_8002D908(&mut (*this).actor);
    func_8002D7EC(&mut (*this).actor);
    if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
           0 as libc::c_int {
        if (*otherTw).actionFunc !=
               Some(BossTw_ShootBeam as
                        unsafe extern "C" fn(_: *mut BossTw,
                                             _: *mut GlobalContext) -> ()) &&
               (*this).work[CAN_SHOOT as libc::c_int as usize] as libc::c_int
                   != 0 {
            (*this).work[CAN_SHOOT as libc::c_int as usize] =
                0 as libc::c_int as s16;
            BossTw_SetupShootBeam(this, globalCtx);
            (*this).actor.speedXZ = 0.0f32
        } else { BossTw_SetupFlyTo(this, globalCtx); }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_SetupFlyTo(mut this: *mut BossTw,
                                           mut globalCtx:
                                               *mut GlobalContext) {
    static mut sPillarPositions: [Vec3f; 4] =
        [{ let mut init = Vec3f{x: 600.0f32, y: 400.0f32, z: 0.0f32,}; init },
         { let mut init = Vec3f{x: 0.0f32, y: 400.0f32, z: 600.0f32,}; init },
         {
             let mut init = Vec3f{x: -600.0f32, y: 400.0f32, z: 0.0f32,};
             init
         },
         {
             let mut init = Vec3f{x: 0.0f32, y: 400.0f32, z: -600.0f32,};
             init
         }];
    let mut otherTw: *mut BossTw = (*this).actor.parent as *mut BossTw;
    (*this).unk_5F8 = 1 as libc::c_int as u8_0;
    (*this).actor.flags |=
        ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
    (*this).actionFunc =
        Some(BossTw_FlyTo as
                 unsafe extern "C" fn(_: *mut BossTw, _: *mut GlobalContext)
                     -> ());
    (*this).rotateSpeed = 0.0f32;
    Animation_MorphToLoop(&mut (*this).skelAnime, &mut object_tw_Anim_006F28,
                          -10.0f32);
    if Rand_ZeroOne() < 0.5f32 &&
           (!otherTw.is_null() &&
                (*otherTw).actionFunc ==
                    Some(BossTw_ShootBeam as
                             unsafe extern "C" fn(_: *mut BossTw,
                                                  _: *mut GlobalContext)
                                 -> ())) {
        // Other Sister is shooting a beam, go near them.
        (*this).targetPos.x =
            (*otherTw).actor.world.pos.x + Rand_CenteredFloat(200.0f32);
        (*this).targetPos.y = Rand_ZeroFloat(200.0f32) + 340.0f32;
        (*this).targetPos.z =
            (*otherTw).actor.world.pos.z + Rand_CenteredFloat(200.0f32);
        (*this).timers[0 as libc::c_int as usize] =
            (Rand_ZeroFloat(50.0f32) as s16 as libc::c_int +
                 50 as libc::c_int) as s16
    } else if Rand_ZeroOne() < 0.5f32 {
        // Fly to a random spot.
        (*this).targetPos.x = Rand_CenteredFloat(800.0f32);
        (*this).targetPos.y = Rand_ZeroFloat(200.0f32) + 340.0f32;
        (*this).targetPos.z = Rand_CenteredFloat(800.0f32);
        (*this).timers[0 as libc::c_int as usize] =
            (Rand_ZeroFloat(50.0f32) as s16 as libc::c_int +
                 50 as libc::c_int) as s16
    } else {
        // fly to a random pillar.
        let mut idx: s16 =
            Rand_ZeroFloat((::std::mem::size_of::<[Vec3f; 4]>() as
                                libc::c_ulong).wrapping_div(::std::mem::size_of::<Vec3f>()
                                                                as
                                                                libc::c_ulong)
                               as s32 as libc::c_float - 0.01f32) as s16;
        (*this).targetPos = sPillarPositions[idx as usize];
        (*this).timers[0 as libc::c_int as usize] = 200 as libc::c_int as s16;
        (*this).work[CAN_SHOOT as libc::c_int as usize] =
            1 as libc::c_int as s16
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_FlyTo(mut this: *mut BossTw,
                                      mut globalCtx: *mut GlobalContext) {
    let mut xDiff: f32_0 = 0.;
    let mut yDiff: f32_0 = 0.;
    let mut zDiff: f32_0 = 0.;
    let mut pitchTarget: f32_0 = 0.;
    let mut yawTarget: f32_0 = 0.;
    let mut xzDist: f32_0 = 0.;
    Audio_PlayActorSound2(&mut (*this).actor,
                          (0x391f as libc::c_int - 0x800 as libc::c_int) as
                              u16_0);
    Math_ApproachF(&mut (*this).scepterAlpha, 0.0f32, 1.0f32, 10.0f32);
    SkelAnime_Update(&mut (*this).skelAnime);
    xDiff = (*this).targetPos.x - (*this).actor.world.pos.x;
    yDiff = (*this).targetPos.y - (*this).actor.world.pos.y;
    zDiff = (*this).targetPos.z - (*this).actor.world.pos.z;
    yawTarget =
        (Math_FAtan2F(xDiff, zDiff) *
             (32768.0f32 / 3.14159265358979323846f32)) as s16 as f32_0;
    xzDist = sqrtf(xDiff * xDiff + zDiff * zDiff);
    pitchTarget =
        (Math_FAtan2F(yDiff, xzDist) *
             (32768.0f32 / 3.14159265358979323846f32)) as s16 as f32_0;
    Math_ApproachS(&mut (*this).actor.world.rot.x, pitchTarget as s16,
                   0xa as libc::c_int as s16, (*this).rotateSpeed as s16);
    Math_ApproachS(&mut (*this).actor.world.rot.y, yawTarget as s16,
                   0xa as libc::c_int as s16, (*this).rotateSpeed as s16);
    Math_ApproachS(&mut (*this).actor.shape.rot.y, yawTarget as s16,
                   0xa as libc::c_int as s16, (*this).rotateSpeed as s16);
    Math_ApproachS(&mut (*this).actor.shape.rot.x, pitchTarget as s16,
                   0xa as libc::c_int as s16, (*this).rotateSpeed as s16);
    Math_ApproachF(&mut (*this).rotateSpeed, 4096.0f32, 1.0f32, 100.0f32);
    Math_ApproachF(&mut (*this).actor.speedXZ, 10.0f32, 1.0f32, 1.0f32);
    func_8002D908(&mut (*this).actor);
    func_8002D7EC(&mut (*this).actor);
    if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
           0 as libc::c_int || xzDist < 70.0f32 {
        BossTw_SetupTurnToPlayer(this, globalCtx);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_SetupShootBeam(mut this: *mut BossTw,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    (*this).actionFunc =
        Some(BossTw_ShootBeam as
                 unsafe extern "C" fn(_: *mut BossTw, _: *mut GlobalContext)
                     -> ());
    Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                              &mut object_tw_Anim_007688, -5.0f32);
    (*this).workf[ANIM_SW_TGT as libc::c_int as usize] =
        Animation_GetLastFrame(&mut object_tw_Anim_007688 as
                                   *mut AnimationHeader as *mut libc::c_void)
            as f32_0;
    (*this).timers[1 as libc::c_int as usize] = 70 as libc::c_int as s16;
    (*this).targetPos = (*player).actor.world.pos;
    (*this).csState1 = 0 as libc::c_int as s16;
    (*this).beamDist = 0.0f32;
    (*this).beamReflectionDist = 0.0f32;
    (*this).beamShootState = -(1 as libc::c_int) as s16;
    (*this).beamScale = 0.01f32;
    (*this).beamReflectionOrigin = (*this).beamOrigin;
    (*this).flameAlpha = 0.0f32;
    (*this).spawnPortalAlpha = 0.0f32;
    (*this).spawnPortalScale = 2000.0f32;
    (*this).updateRate1 = 0.0f32;
    (*this).portalRotation = 0.0f32;
    (*this).updateRate2 = 0.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_SpawnGroundBlast(mut this: *mut BossTw,
                                                 mut globalCtx:
                                                     *mut GlobalContext,
                                                 mut blastType: s16) {
    let mut groundBlast: *mut BossTw = 0 as *mut BossTw;
    let mut i: s16 = 0;
    let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut velocity: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut accel: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[BossTwEffect; 150]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<BossTwEffect>()
                                                   as libc::c_ulong) as s32 {
        velocity.x = Rand_CenteredFloat(20.0f32);
        velocity.y = Rand_ZeroFloat(10.0f32);
        velocity.z = Rand_CenteredFloat(20.0f32);
        accel.y = 0.2f32;
        accel.x = Rand_CenteredFloat(0.25f32);
        accel.z = Rand_CenteredFloat(0.25f32);
        pos = (*this).groundBlastPos;
        BossTw_AddDotEffect(globalCtx, &mut pos, &mut velocity, &mut accel,
                            (Rand_ZeroFloat(2.0f32) as s16 as libc::c_int +
                                 8 as libc::c_int) as f32_0, blastType,
                            75 as libc::c_int as s16);
        i += 1
    }
    if blastType as libc::c_int == 1 as libc::c_int {
        sGroundBlastType = 1 as libc::c_int as u8_0;
        groundBlast =
            Actor_SpawnAsChild(&mut (*globalCtx).actorCtx, &mut (*this).actor,
                               globalCtx, ACTOR_BOSS_TW as libc::c_int as s16,
                               (*this).groundBlastPos.x,
                               (*this).groundBlastPos.y,
                               (*this).groundBlastPos.z,
                               0 as libc::c_int as s16,
                               0 as libc::c_int as s16,
                               0 as libc::c_int as s16,
                               TW_FIRE_BLAST_GROUND as libc::c_int as s16) as
                *mut BossTw;
        if !groundBlast.is_null() {
            if (*sTwinrovaPtr).actionFunc ==
                   Some(BossTw_Wait as
                            unsafe extern "C" fn(_: *mut BossTw,
                                                 _: *mut GlobalContext) -> ())
               {
                (*groundBlast).timers[0 as libc::c_int as usize] =
                    100 as libc::c_int as s16
            } else {
                (*groundBlast).timers[0 as libc::c_int as usize] =
                    50 as libc::c_int as s16
            }
            (*sKoumePtr).workf[KM_GRND_CRTR_A as libc::c_int as usize] =
                255.0f32;
            (*sKoumePtr).workf[KM_GD_SMOKE_A as libc::c_int as usize] =
                (*sKoumePtr).workf[KM_GRND_CRTR_A as libc::c_int as usize];
            (*sKoumePtr).workf[KM_GD_FLM_A as libc::c_int as usize] =
                (*sKoumePtr).workf[KM_GD_SMOKE_A as libc::c_int as usize];
            (*sKoumePtr).workf[KM_GD_FLM_SCL as libc::c_int as usize] =
                1.0f32;
            (*sKoumePtr).workf[KM_GD_CRTR_SCL as libc::c_int as usize] =
                0.005f32;
            (*sKoumePtr).groundBlastPos2 = (*groundBlast).actor.world.pos;
            sEnvType = 4 as libc::c_int as s8
        }
    } else {
        sGroundBlastType = 2 as libc::c_int as u8_0;
        groundBlast =
            Actor_SpawnAsChild(&mut (*globalCtx).actorCtx, &mut (*this).actor,
                               globalCtx, ACTOR_BOSS_TW as libc::c_int as s16,
                               (*this).groundBlastPos.x,
                               (*this).groundBlastPos.y,
                               (*this).groundBlastPos.z,
                               0 as libc::c_int as s16,
                               0 as libc::c_int as s16,
                               0 as libc::c_int as s16,
                               TW_ICE_BLAST_GROUND as libc::c_int as s16) as
                *mut BossTw;
        if !groundBlast.is_null() {
            if (*sTwinrovaPtr).actionFunc ==
                   Some(BossTw_Wait as
                            unsafe extern "C" fn(_: *mut BossTw,
                                                 _: *mut GlobalContext) -> ())
               {
                (*groundBlast).timers[0 as libc::c_int as usize] =
                    100 as libc::c_int as s16
            } else {
                (*groundBlast).timers[0 as libc::c_int as usize] =
                    50 as libc::c_int as s16
            }
            (*sKotakePtr).workf[UNK_F11 as libc::c_int as usize] = 50.0f32;
            (*sKotakePtr).workf[UNK_F9 as libc::c_int as usize] = 250.0f32;
            (*sKotakePtr).workf[UNK_F12 as libc::c_int as usize] = 0.005f32;
            (*sKotakePtr).workf[UNK_F14 as libc::c_int as usize] = 1.0f32;
            (*sKotakePtr).workf[UNK_F16 as libc::c_int as usize] = 70.0f32;
            (*sKotakePtr).groundBlastPos2 = (*groundBlast).actor.world.pos;
            sEnvType = 3 as libc::c_int as s8
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_BeamHitPlayerCheck(mut this: *mut BossTw,
                                                   mut globalCtx:
                                                       *mut GlobalContext)
 -> s32 {
    let mut offset: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut beamDistFromPlayer: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut i: s16 = 0;
    offset.x = (*player).actor.world.pos.x - (*this).beamOrigin.x;
    offset.y = (*player).actor.world.pos.y - (*this).beamOrigin.y;
    offset.z = (*player).actor.world.pos.z - (*this).beamOrigin.z;
    Matrix_RotateX(-(*this).beamPitch, MTXMODE_NEW as libc::c_int as u8_0);
    Matrix_RotateY(-(*this).beamYaw, MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_MultVec3f(&mut offset, &mut beamDistFromPlayer);
    if fabsf(beamDistFromPlayer.x) < 20.0f32 &&
           fabsf(beamDistFromPlayer.y) < 50.0f32 &&
           beamDistFromPlayer.z > 100.0f32 &&
           beamDistFromPlayer.z <= (*this).beamDist {
        if (*sTwinrovaPtr).timers[2 as libc::c_int as usize] as libc::c_int ==
               0 as libc::c_int {
            (*sTwinrovaPtr).timers[2 as libc::c_int as usize] =
                150 as libc::c_int as s16;
            (*this).beamDist =
                sqrtf(offset.x * offset.x + offset.y * offset.y +
                          offset.z * offset.z);
            func_8002F6D4(globalCtx, &mut (*this).actor, 3.0f32,
                          (*this).actor.shape.rot.y, 0.0f32,
                          0x20 as libc::c_int as u32_0);
            if (*this).actor.params as libc::c_int == 0 as libc::c_int {
                if sFreezeState as libc::c_int == 0 as libc::c_int {
                    sFreezeState = 1 as libc::c_int as u8_0
                }
            } else if (*player).isBurning == 0 {
                i = 0 as libc::c_int as s16;
                while (i as libc::c_int) <
                          (::std::mem::size_of::<[u8_0; 18]>() as
                               libc::c_ulong).wrapping_div(::std::mem::size_of::<u8_0>()
                                                               as
                                                               libc::c_ulong)
                              as s32 {
                    (*player).flameTimers[i as usize] =
                        Rand_S16Offset(0 as libc::c_int as s16,
                                       200 as libc::c_int as s16) as u8_0;
                    i += 1
                }
                (*player).isBurning = 1 as libc::c_int as u8_0;
                func_8002F7DC(&mut (*player).actor,
                              ((*(*player).ageProperties).unk_92 as
                                   libc::c_int + 0x681e as libc::c_int) as
                                  u16_0);
            }
        }
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/* *
 * Checks if the beam shot by `this` will be reflected
 * returns 0 if the beam will not be reflected,
 * returns 1 if the beam will be reflected,
 * and returns 2 if the beam will be diverted backwards
 */
#[no_mangle]
pub unsafe extern "C" fn BossTw_CheckBeamReflection(mut this: *mut BossTw,
                                                    mut globalCtx:
                                                        *mut GlobalContext)
 -> s32 {
    let mut offset: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut vec: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    if (*player).stateFlags1 & 0x400000 as libc::c_int as libc::c_uint != 0 &&
           (((*player).actor.shape.rot.y as libc::c_int -
                 (*this).actor.shape.rot.y as libc::c_int +
                 0x8000 as libc::c_int) as s16 as libc::c_int) <
               0x2000 as libc::c_int &&
           ((*player).actor.shape.rot.y as libc::c_int -
                (*this).actor.shape.rot.y as libc::c_int +
                0x8000 as libc::c_int) as s16 as libc::c_int >
               -(0x2000 as libc::c_int) {
        // player is shielding and facing angles are less than 45 degrees in either direction
        offset.x = 0.0f32;
        offset.y = 0.0f32;
        offset.z = 10.0f32;
        // set beam check point to 10 units in front of link.
        Matrix_RotateY((*player).actor.shape.rot.y as libc::c_int as
                           libc::c_float / 32768.0f32 *
                           3.14159265358979323846f32,
                       MTXMODE_NEW as libc::c_int as u8_0);
        Matrix_MultVec3f(&mut offset, &mut vec);
        // calculates a vector where the origin is at the beams origin,
        // and the positive z axis is pointing in the direction the beam
        // is shooting
        offset.x = (*player).actor.world.pos.x + vec.x - (*this).beamOrigin.x;
        offset.y = (*player).actor.world.pos.y + vec.y - (*this).beamOrigin.y;
        offset.z = (*player).actor.world.pos.z + vec.z - (*this).beamOrigin.z;
        Matrix_RotateX(-(*this).beamPitch,
                       MTXMODE_NEW as libc::c_int as u8_0);
        Matrix_RotateY(-(*this).beamYaw,
                       MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_MultVec3f(&mut offset, &mut vec);
        if fabsf(vec.x) < 30.0f32 && fabsf(vec.y) < 70.0f32 &&
               vec.z > 100.0f32 && vec.z <= (*this).beamDist {
            // if the beam's origin is within 30 x units, 70 y units, is farther than 100 units
            // and the distance from the beams origin to 10 units in front of link is less than the beams
            // current distance (the distance of the beam is equal to or longer than the distance to 10 units
            // in front of link)
            if Player_HasMirrorShieldEquipped(globalCtx) != 0 {
                // player has mirror shield equipped
                (*this).beamDist =
                    sqrtf(offset.x * offset.x + offset.y * offset.y +
                              offset.z * offset.z);
                return 1 as libc::c_int
            }
            if sBeamDivertTimer as libc::c_int > 10 as libc::c_int {
                return 0 as libc::c_int
            }
            if sBeamDivertTimer as libc::c_int == 0 as libc::c_int {
                // beam hit the shield, normal shield equipped,
                // divert the beam backwards from link's Y rotation
                BossTw_AddShieldDeflectEffect(globalCtx, 10.0f32,
                                              (*this).actor.params);
                (*globalCtx).envCtx.unk_D8 = 1.0f32;
                (*this).timers[0 as libc::c_int as usize] =
                    10 as libc::c_int as s16;
                func_80078884(0x1838 as libc::c_int as u16_0);
            }
            sBeamDivertTimer = sBeamDivertTimer.wrapping_add(1);
            (*this).beamDist =
                sqrtf(offset.x * offset.x + offset.y * offset.y +
                          offset.z * offset.z);
            return 2 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_BeamReflHitCheck(mut this: *mut BossTw,
                                                 mut pos: *mut Vec3f) -> s32 {
    let mut offset: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut beamDistFromTarget: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    offset.x = (*pos).x - (*this).beamReflectionOrigin.x;
    offset.y = (*pos).y - (*this).beamReflectionOrigin.y;
    offset.z = (*pos).z - (*this).beamReflectionOrigin.z;
    Matrix_RotateX(-(*this).beamReflectionPitch,
                   MTXMODE_NEW as libc::c_int as u8_0);
    Matrix_RotateY(-(*this).beamReflectionYaw,
                   MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_MultVec3f(&mut offset, &mut beamDistFromTarget);
    if fabsf(beamDistFromTarget.x) < 50.0f32 &&
           fabsf(beamDistFromTarget.y) < 50.0f32 &&
           beamDistFromTarget.z > 100.0f32 &&
           beamDistFromTarget.z <= (*this).beamReflectionDist {
        (*this).beamReflectionDist =
            sqrtf(offset.x * offset.x + offset.y * offset.y +
                      offset.z * offset.z) * 1.1f32;
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_GetFloorY(mut pos: *mut Vec3f) -> f32_0 {
    let mut posRotated: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    if fabsf((*pos).x) < 350.0f32 && fabsf((*pos).z) < 350.0f32 &&
           (*pos).y < 240.0f32 {
        if (*pos).y > 200.0f32 { return 240.0f32 }
        return 35.0f32
    }
    if fabsf((*pos).x) < 110.0f32 &&
           (fabsf((*pos).z - 600.0f32) < 110.0f32 ||
                fabsf((*pos).z + 600.0f32) < 110.0f32) && (*pos).y < 230.0f32
       {
        if (*pos).y > 190.0f32 { return 230.0f32 }
        return 35.0f32
    }
    if fabsf((*pos).z) < 110.0f32 &&
           (fabsf((*pos).x - 600.0f32) < 110.0f32 ||
                fabsf((*pos).x + 600.0f32) < 110.0f32) && (*pos).y < 230.0f32
       {
        if (*pos).y > 190.0f32 { return 230.0f32 }
        return 35.0f32
    }
    if (*pos).y < -20.0f32 { return 0.0f32 }
    if fabsf((*pos).x) > 1140.0f32 || fabsf((*pos).z) > 1140.0f32 {
        return 35.0f32
    }
    Matrix_Push();
    Matrix_RotateY(45.0f32 * (3.14159265358979323846f32 / 180.0f32),
                   MTXMODE_NEW as libc::c_int as u8_0);
    Matrix_MultVec3f(pos, &mut posRotated);
    Matrix_Pop();
    if fabsf(posRotated.x) > 920.0f32 || fabsf(posRotated.z) > 920.0f32 {
        return 35.0f32
    }
    return -100.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_ShootBeam(mut this: *mut BossTw,
                                          mut globalCtx: *mut GlobalContext) {
    let mut i: s16 = 0;
    let mut xDiff: f32_0 = 0.;
    let mut yDiff: f32_0 = 0.;
    let mut zDiff: f32_0 = 0.;
    let mut floorY: f32_0 = 0.;
    let mut sp130: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp128: Vec3s = Vec3s{x: 0, y: 0, z: 0,};
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut otherTw: *mut BossTw = (*this).actor.parent as *mut BossTw;
    let mut input: *mut Input =
        &mut *(*globalCtx).state.input.as_mut_ptr().offset(0 as libc::c_int as
                                                               isize) as
            *mut Input;
    Math_ApproachF(&mut (*this).actor.world.pos.y, 400.0f32, 0.05f32,
                   (*this).actor.speedXZ);
    Math_ApproachF(&mut (*this).actor.speedXZ, 5.0f32, 1.0f32, 0.25f32);
    SkelAnime_Update(&mut (*this).skelAnime);
    (*this).beamRoll += -0.3f32;
    if (*this).timers[1 as libc::c_int as usize] as libc::c_int !=
           0 as libc::c_int {
        Math_ApproachS(&mut (*this).actor.shape.rot.y,
                       (*this).actor.yawTowardsPlayer,
                       5 as libc::c_int as s16, (*this).rotateSpeed as s16);
        if (*player).stateFlags1 & 0x400000 as libc::c_int as libc::c_uint !=
               0 &&
               (((*player).actor.shape.rot.y as libc::c_int -
                     (*this).actor.shape.rot.y as libc::c_int +
                     0x8000 as libc::c_int) as s16 as libc::c_int) <
                   0x2000 as libc::c_int &&
               ((*player).actor.shape.rot.y as libc::c_int -
                    (*this).actor.shape.rot.y as libc::c_int +
                    0x8000 as libc::c_int) as s16 as libc::c_int >
                   -(0x2000 as libc::c_int) {
            Math_ApproachF(&mut (*this).targetPos.x,
                           (*player).bodyPartsPos[15 as libc::c_int as
                                                      usize].x, 1.0f32,
                           400.0f32);
            Math_ApproachF(&mut (*this).targetPos.y,
                           (*player).bodyPartsPos[15 as libc::c_int as
                                                      usize].y, 1.0f32,
                           400.0f32);
            Math_ApproachF(&mut (*this).targetPos.z,
                           (*player).bodyPartsPos[15 as libc::c_int as
                                                      usize].z, 1.0f32,
                           400.0f32);
        } else {
            Math_ApproachF(&mut (*this).targetPos.x,
                           (*player).actor.world.pos.x, 1.0f32, 400.0f32);
            Math_ApproachF(&mut (*this).targetPos.y,
                           (*player).actor.world.pos.y + 30.0f32, 1.0f32,
                           400.0f32);
            Math_ApproachF(&mut (*this).targetPos.z,
                           (*player).actor.world.pos.z, 1.0f32, 400.0f32);
        }
        (*this).timers[0 as libc::c_int as usize] = 70 as libc::c_int as s16;
        (*this).groundBlastPos.z = 0.0f32;
        (*this).groundBlastPos.y = (*this).groundBlastPos.z;
        (*this).groundBlastPos.x = (*this).groundBlastPos.y;
        (*this).portalRotation += (*this).updateRate2 * 0.0025f32;
        Math_ApproachF(&mut (*this).spawnPortalAlpha, 255.0f32, 1.0f32,
                       10.0f32);
        Math_ApproachF(&mut (*this).updateRate2, 50.0f32, 1.0f32, 2.0f32);
        if ((*this).timers[1 as libc::c_int as usize] as libc::c_int) <
               50 as libc::c_int {
            if ((*this).timers[1 as libc::c_int as usize] as libc::c_int) <
                   10 as libc::c_int {
                if (*this).timers[1 as libc::c_int as usize] as libc::c_int ==
                       9 as libc::c_int {
                    (*globalCtx).envCtx.unk_D8 = 0.5f32;
                    (*globalCtx).envCtx.unk_BD =
                        (3 as libc::c_int -
                             (*this).actor.params as libc::c_int) as u8_0;
                    Audio_PlayActorSound2(&mut (*this).actor,
                                          0x3913 as libc::c_int as u16_0);
                }
                if (*this).timers[1 as libc::c_int as usize] as libc::c_int ==
                       5 as libc::c_int {
                    (*this).scepterAlpha = 255 as libc::c_int as f32_0
                }
                if (*this).timers[1 as libc::c_int as usize] as libc::c_int >
                       4 as libc::c_int {
                    let mut j: s16 = 0;
                    j = 0 as libc::c_int as s16;
                    while (j as libc::c_int) < 2 as libc::c_int {
                        i = 0 as libc::c_int as s16;
                        while (i as libc::c_int) <
                                  (::std::mem::size_of::<[Vec3f; 5]>() as
                                       libc::c_ulong).wrapping_div(::std::mem::size_of::<Vec3f>()
                                                                       as
                                                                       libc::c_ulong)
                                      as s32 {
                            let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                            let mut velocity: Vec3f =
                                Vec3f{x: 0., y: 0., z: 0.,};
                            let mut accel: Vec3f =
                                Vec3f{x: 0., y: 0., z: 0.,};
                            pos.x = (*this).scepterFlamePos[i as usize].x;
                            pos.y = (*this).scepterFlamePos[i as usize].y;
                            pos.z = (*this).scepterFlamePos[i as usize].z;
                            velocity.x = Rand_CenteredFloat(10.0f32);
                            velocity.y = Rand_CenteredFloat(10.0f32);
                            velocity.z = Rand_CenteredFloat(10.0f32);
                            accel.x = 0.0f32;
                            accel.y = 0.0f32;
                            accel.z = 0.0f32;
                            BossTw_AddFlameEffect(globalCtx, &mut pos,
                                                  &mut velocity, &mut accel,
                                                  Rand_ZeroFloat(10.0f32) +
                                                      25.0f32,
                                                  (*this).actor.params);
                            i += 1
                        }
                        j += 1
                    }
                }
            }
            if ((*this).timers[1 as libc::c_int as usize] as libc::c_int) <
                   20 as libc::c_int {
                Math_ApproachF(&mut (*this).flameAlpha,
                               0 as libc::c_int as f32_0, 1.0f32, 20.0f32);
                Math_ApproachF(&mut (*this).spawnPortalAlpha,
                               0 as libc::c_int as f32_0, 1.0f32, 30.0f32);
            } else {
                Math_ApproachF(&mut (*this).flameAlpha, 255.0f32, 1.0f32,
                               10.0f32);
                if (*this).actor.params as libc::c_int == 1 as libc::c_int {
                    Audio_PlayActorSound2(&mut (*this).actor,
                                          (0x390e as libc::c_int -
                                               0x800 as libc::c_int) as
                                              u16_0);
                } else {
                    Audio_PlayActorSound2(&mut (*this).actor,
                                          (0x3912 as libc::c_int -
                                               0x800 as libc::c_int) as
                                              u16_0);
                }
            }
            (*this).flameRotation += (*this).updateRate1 * 0.0025f32;
            Math_ApproachF(&mut (*this).spawnPortalScale, 0.0f32, 0.1f32,
                           (*this).updateRate1);
            Math_ApproachF(&mut (*this).updateRate1, 50.0f32, 1.0f32, 2.0f32);
        }
        if Animation_OnFrame(&mut (*this).skelAnime,
                             (*this).workf[ANIM_SW_TGT as libc::c_int as
                                               usize]) != 0 {
            Animation_MorphToLoop(&mut (*this).skelAnime,
                                  &mut object_tw_Anim_009398, 0.0f32);
            (*this).workf[ANIM_SW_TGT as libc::c_int as usize] = 10000.0f32
        }
        if (*this).timers[1 as libc::c_int as usize] as libc::c_int ==
               1 as libc::c_int {
            Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                                      &mut object_tw_Anim_003614, 0.0f32);
            (*this).workf[ANIM_SW_TGT as libc::c_int as usize] =
                Animation_GetLastFrame(&mut object_tw_Anim_003614 as
                                           *mut AnimationHeader as
                                           *mut libc::c_void) as f32_0;
            (*this).unk_4DC = 0.0f32;
            (*this).spawnPortalAlpha = 0.0f32;
            (*this).flameAlpha = 0.0f32;
            sBeamDivertTimer = 0 as libc::c_int as u8_0
        }
    } else {
        if Animation_OnFrame(&mut (*this).skelAnime,
                             (*this).workf[ANIM_SW_TGT as libc::c_int as
                                               usize]) != 0 {
            Animation_MorphToLoop(&mut (*this).skelAnime,
                                  &mut object_tw_Anim_003E34, 0.0f32);
            (*this).workf[ANIM_SW_TGT as libc::c_int as usize] = 10000.0f32
        }
        if Animation_OnFrame(&mut (*this).skelAnime,
                             (*this).workf[ANIM_SW_TGT as libc::c_int as
                                               usize] - 5.0f32) != 0 {
            (*this).beamShootState = 0 as libc::c_int as s16;
            sEnvType =
                ((*this).actor.params as libc::c_int + 1 as libc::c_int) as s8
        }
        if Animation_OnFrame(&mut (*this).skelAnime,
                             (*this).workf[ANIM_SW_TGT as libc::c_int as
                                               usize] - 13.0f32) != 0 {
            Audio_PlayActorSound2(&mut (*this).actor,
                                  0x3923 as libc::c_int as u16_0);
            Audio_PlayActorSound2(&mut (*this).actor,
                                  0x39b3 as libc::c_int as u16_0);
        }
        xDiff = (*this).targetPos.x - (*this).beamOrigin.x;
        yDiff = (*this).targetPos.y - (*this).beamOrigin.y;
        zDiff = (*this).targetPos.z - (*this).beamOrigin.z;
        (*this).beamYaw = Math_FAtan2F(xDiff, zDiff);
        (*this).beamPitch =
            -Math_FAtan2F(yDiff, sqrtf(xDiff * xDiff + zDiff * zDiff));
        match (*this).beamShootState as libc::c_int {
            0 => {
                if (*this).timers[0 as libc::c_int as usize] as libc::c_int !=
                       0 as libc::c_int {
                    let mut beamReflection: s32 =
                        BossTw_CheckBeamReflection(this, globalCtx);
                    if beamReflection == 1 as libc::c_int {
                        let mut pos_0: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                        let mut velocity_0: Vec3f =
                            Vec3f{x: 0., y: 0., z: 0.,};
                        let mut accel_0: Vec3f =
                            {
                                let mut init =
                                    Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,};
                                init
                            };
                        i = 0 as libc::c_int as s16;
                        while (i as libc::c_int) <
                                  (::std::mem::size_of::<[BossTwEffect; 150]>()
                                       as
                                       libc::c_ulong).wrapping_div(::std::mem::size_of::<BossTwEffect>()
                                                                       as
                                                                       libc::c_ulong)
                                      as s32 {
                            velocity_0.x = Rand_CenteredFloat(15.0f32);
                            velocity_0.y = Rand_CenteredFloat(15.0f32);
                            velocity_0.z = Rand_CenteredFloat(15.0f32);
                            pos_0 =
                                (*player).bodyPartsPos[15 as libc::c_int as
                                                           usize];
                            BossTw_AddDotEffect(globalCtx, &mut pos_0,
                                                &mut velocity_0, &mut accel_0,
                                                (Rand_ZeroFloat(2.0f32) as s16
                                                     as libc::c_int +
                                                     5 as libc::c_int) as
                                                    f32_0,
                                                (*this).actor.params,
                                                150 as libc::c_int as s16);
                            i += 1
                        }
                        (*this).beamShootState = 1 as libc::c_int as s16;
                        func_80078914(&mut (*player).actor.projectedPos,
                                      0x180c as libc::c_int as u16_0);
                        Matrix_MtxFToYXZRotS(&mut (*player).shieldMf,
                                             &mut sp128, 0 as libc::c_int);
                        sp128.y =
                            (sp128.y as libc::c_int + 0x8000 as libc::c_int)
                                as s16;
                        sp128.x = -(sp128.x as libc::c_int) as s16;
                        (*this).magicDir.x = sp128.x;
                        (*this).magicDir.y = sp128.y;
                        (*this).groundBlastPos.x = 0.0f32;
                        (*this).groundBlastPos.y = 0.0f32;
                        (*this).groundBlastPos.z = 0.0f32;
                        (*globalCtx).envCtx.unk_D8 = 1.0f32;
                        func_800AA000(0.0f32, 0x64 as libc::c_int as u8_0,
                                      5 as libc::c_int as u8_0,
                                      4 as libc::c_int as u8_0);
                    } else if beamReflection == 0 as libc::c_int {
                        BossTw_BeamHitPlayerCheck(this, globalCtx);
                        if (*this).csState1 as libc::c_int == 0 as libc::c_int
                           {
                            Math_ApproachF(&mut (*this).beamDist,
                                           2.0f32 *
                                               sqrtf(xDiff * xDiff +
                                                         yDiff * yDiff +
                                                         zDiff * zDiff),
                                           1.0f32, 40.0f32);
                        }
                    }
                }
                SkinMatrix_Vec3fMtxFMultXYZW(&mut (*globalCtx).viewProjectionMtxF,
                                             &mut (*this).beamReflectionOrigin,
                                             &mut (*this).unk_54C,
                                             &mut (*this).actor.projectedW);
                if (*this).actor.params as libc::c_int == 1 as libc::c_int {
                    Audio_PlaySoundGeneral((0x3922 as libc::c_int -
                                                0x800 as libc::c_int) as
                                               u16_0, &mut (*this).unk_54C,
                                           4 as libc::c_int as u8_0,
                                           &mut D_801333E0, &mut D_801333E0,
                                           &mut D_801333E8);
                } else {
                    Audio_PlaySoundGeneral((0x3911 as libc::c_int -
                                                0x800 as libc::c_int) as
                                               u16_0, &mut (*this).unk_54C,
                                           4 as libc::c_int as u8_0,
                                           &mut D_801333E0, &mut D_801333E0,
                                           &mut D_801333E8);
                }
            }
            1 => {
                if !((*input).cur.button as libc::c_int |
                         !(0x10 as libc::c_int)) == 0 as libc::c_int {
                    let mut player_0: *mut Player =
                        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as
                                                             libc::c_int as
                                                             usize].head as
                            *mut Player;
                    (*this).beamDist =
                        sqrtf(xDiff * xDiff + yDiff * yDiff + zDiff * zDiff);
                    Math_ApproachF(&mut (*this).beamReflectionDist, 2000.0f32,
                                   1.0f32, 40.0f32);
                    Math_ApproachF(&mut (*this).targetPos.x,
                                   (*player_0).bodyPartsPos[15 as libc::c_int
                                                                as usize].x,
                                   1.0f32, 400.0f32);
                    Math_ApproachF(&mut (*this).targetPos.y,
                                   (*player_0).bodyPartsPos[15 as libc::c_int
                                                                as usize].y,
                                   1.0f32, 400.0f32);
                    Math_ApproachF(&mut (*this).targetPos.z,
                                   (*player_0).bodyPartsPos[15 as libc::c_int
                                                                as usize].z,
                                   1.0f32, 400.0f32);
                    if (*this).work[CS_TIMER_1 as libc::c_int as usize] as
                           libc::c_int % 4 as libc::c_int == 0 as libc::c_int
                       {
                        BossTw_AddRingEffect(globalCtx,
                                             &mut *(*player_0).bodyPartsPos.as_mut_ptr().offset(15
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    isize),
                                             0.5f32, 3.0f32,
                                             0xff as libc::c_int as s16,
                                             (*this).actor.params,
                                             1 as libc::c_int as s16,
                                             150 as libc::c_int as s16);
                    }
                } else {
                    (*this).beamShootState = 0 as libc::c_int as s16;
                    (*this).beamReflectionDist = 0.0f32
                }
                SkinMatrix_Vec3fMtxFMultXYZW(&mut (*globalCtx).viewProjectionMtxF,
                                             &mut (*this).unk_530,
                                             &mut (*this).unk_558,
                                             &mut (*this).actor.projectedW);
                if (*this).actor.params as libc::c_int == 1 as libc::c_int {
                    Audio_PlaySoundGeneral((0x3922 as libc::c_int -
                                                0x800 as libc::c_int) as
                                               u16_0, &mut (*this).unk_558,
                                           4 as libc::c_uint as u8_0,
                                           &mut D_801333E0, &mut D_801333E0,
                                           &mut D_801333E8);
                    Audio_PlaySoundGeneral((0x3917 as libc::c_int -
                                                0x800 as libc::c_int) as
                                               u16_0, &mut (*this).unk_558,
                                           4 as libc::c_int as u8_0,
                                           &mut D_801333E0, &mut D_801333E0,
                                           &mut D_801333E8);
                } else {
                    Audio_PlaySoundGeneral((0x3911 as libc::c_int -
                                                0x800 as libc::c_int) as
                                               u16_0, &mut (*this).unk_558,
                                           4 as libc::c_int as u8_0,
                                           &mut D_801333E0, &mut D_801333E0,
                                           &mut D_801333E8);
                    Audio_PlaySoundGeneral((0x3918 as libc::c_int -
                                                0x800 as libc::c_int) as
                                               u16_0, &mut (*this).unk_558,
                                           4 as libc::c_int as u8_0,
                                           &mut D_801333E0, &mut D_801333E0,
                                           &mut D_801333E8);
                }
            }
            -1 | _ => { }
        }
        if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
               0 as libc::c_int &&
               (sEnvType as libc::c_int == 1 as libc::c_int ||
                    sEnvType as libc::c_int == 2 as libc::c_int) {
            sEnvType = 0 as libc::c_int as s8
        }
        if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
               0 as libc::c_int {
            Math_ApproachF(&mut (*this).beamScale, 0.0f32, 1.0f32, 0.0005f32);
            if (*this).beamScale == 0.0f32 {
                BossTw_SetupFinishBeamShoot(this, globalCtx);
                (*this).beamReflectionDist = 0.0f32;
                (*this).beamDist = 0.0f32
            }
        }
    }
    Matrix_Translate((*this).beamOrigin.x, (*this).beamOrigin.y,
                     (*this).beamOrigin.z,
                     MTXMODE_NEW as libc::c_int as u8_0);
    Matrix_RotateY((*this).beamYaw, MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_RotateX((*this).beamPitch, MTXMODE_APPLY as libc::c_int as u8_0);
    sp130.x = 0.0f32;
    sp130.y = 0.0f32;
    sp130.z = (*this).beamDist + -5.0f32;
    Matrix_MultVec3f(&mut sp130, &mut (*this).beamReflectionOrigin);
    if (*this).csState1 as libc::c_int == 0 as libc::c_int &&
           (*this).beamShootState as libc::c_int == 0 as libc::c_int &&
           (*this).timers[0 as libc::c_int as usize] as libc::c_int !=
               0 as libc::c_int {
        (*this).groundBlastPos.y =
            BossTw_GetFloorY(&mut (*this).beamReflectionOrigin);
        if (*this).groundBlastPos.y >= 0.0f32 {
            (*this).csState1 = 1 as libc::c_int as s16;
            (*this).groundBlastPos.x = (*this).beamReflectionOrigin.x;
            (*this).groundBlastPos.z = (*this).beamReflectionOrigin.z;
            BossTw_SpawnGroundBlast(this, globalCtx, (*this).actor.params);
            (*this).timers[0 as libc::c_int as usize] =
                20 as libc::c_int as s16
        }
    }
    if (*this).beamShootState as libc::c_int == 1 as libc::c_int {
        if (*this).csState1 as libc::c_int == 0 as libc::c_int {
            Matrix_MtxFToYXZRotS(&mut (*player).shieldMf, &mut sp128,
                                 0 as libc::c_int);
            sp128.y = (sp128.y as libc::c_int + 0x8000 as libc::c_int) as s16;
            sp128.x = -(sp128.x as libc::c_int) as s16;
            Math_ApproachS(&mut (*this).magicDir.x, sp128.x,
                           5 as libc::c_int as s16,
                           0x2000 as libc::c_int as s16);
            Math_ApproachS(&mut (*this).magicDir.y, sp128.y,
                           5 as libc::c_int as s16,
                           0x2000 as libc::c_int as s16);
            (*this).beamReflectionPitch =
                (*this).magicDir.x as libc::c_int as libc::c_float /
                    32768.0f32 * 3.14159265358979323846f32;
            (*this).beamReflectionYaw =
                (*this).magicDir.y as libc::c_int as libc::c_float /
                    32768.0f32 * 3.14159265358979323846f32
        }
        Matrix_Translate((*this).beamReflectionOrigin.x,
                         (*this).beamReflectionOrigin.y,
                         (*this).beamReflectionOrigin.z,
                         MTXMODE_NEW as libc::c_int as u8_0);
        Matrix_RotateY((*this).beamReflectionYaw,
                       MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_RotateX((*this).beamReflectionPitch,
                       MTXMODE_APPLY as libc::c_int as u8_0);
        sp130.x = 0.0f32;
        sp130.y = 0.0f32;
        sp130.z = (*this).beamReflectionDist + -170.0f32;
        Matrix_MultVec3f(&mut sp130, &mut (*this).unk_530);
        if (*this).csState1 as libc::c_int == 0 as libc::c_int {
            sp130.z = 0.0f32;
            i = 0 as libc::c_int as s16;
            while (i as libc::c_int) < 200 as libc::c_int {
                let mut spBC: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                Matrix_MultVec3f(&mut sp130, &mut spBC);
                floorY = BossTw_GetFloorY(&mut spBC);
                (*this).groundBlastPos.y = floorY;
                if floorY >= 0.0f32 {
                    if (*this).groundBlastPos.y != 35.0f32 &&
                           0.0f32 < (*this).beamReflectionPitch &&
                           (*this).timers[0 as libc::c_int as usize] as
                               libc::c_int != 0 as libc::c_int {
                        (*this).csState1 = 1 as libc::c_int as s16;
                        (*this).groundBlastPos.x = spBC.x;
                        (*this).groundBlastPos.z = spBC.z;
                        BossTw_SpawnGroundBlast(this, globalCtx,
                                                (*this).actor.params);
                        (*this).timers[0 as libc::c_int as usize] =
                            20 as libc::c_int as s16
                    } else {
                        i = 0 as libc::c_int as s16;
                        while (i as libc::c_int) < 5 as libc::c_int {
                            let mut velocity_1: Vec3f =
                                Vec3f{x: 0., y: 0., z: 0.,};
                            let mut accel_1: Vec3f =
                                Vec3f{x: 0., y: 0., z: 0.,};
                            velocity_1.x = Rand_CenteredFloat(20.0f32);
                            velocity_1.y = Rand_CenteredFloat(20.0f32);
                            velocity_1.z = Rand_CenteredFloat(20.0f32);
                            accel_1.x = 0.0f32;
                            accel_1.y = 0.0f32;
                            accel_1.z = 0.0f32;
                            BossTw_AddFlameEffect(globalCtx,
                                                  &mut (*this).unk_530,
                                                  &mut velocity_1,
                                                  &mut accel_1,
                                                  Rand_ZeroFloat(10.0f32) +
                                                      25.0f32,
                                                  (*this).actor.params);
                            i += 1
                        }
                        (*this).beamReflectionDist = sp130.z;
                        Math_ApproachF(&mut (*globalCtx).envCtx.unk_D8,
                                       0.8f32, 1.0f32, 0.2f32);
                    }
                    break ;
                } else {
                    sp130.z += 20.0f32;
                    if (*this).beamReflectionDist < sp130.z { break ; }
                    i += 1
                }
            }
        }
        if BossTw_BeamReflHitCheck(this, &mut (*this).actor.world.pos) != 0 &&
               (*this).work[CS_TIMER_1 as libc::c_int as usize] as libc::c_int
                   % 4 as libc::c_int == 0 as libc::c_int {
            BossTw_AddRingEffect(globalCtx, &mut (*this).unk_530, 0.5f32,
                                 3.0f32, 255 as libc::c_int as s16,
                                 (*this).actor.params,
                                 1 as libc::c_int as s16,
                                 150 as libc::c_int as s16);
        }
        if BossTw_BeamReflHitCheck(this, &mut (*otherTw).actor.world.pos) != 0
               &&
               (*otherTw).actionFunc !=
                   Some(BossTw_HitByBeam as
                            unsafe extern "C" fn(_: *mut BossTw,
                                                 _: *mut GlobalContext) -> ())
           {
            i = 0 as libc::c_int as s16;
            while (i as libc::c_int) < 50 as libc::c_int {
                let mut pos_1: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                let mut velocity_2: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                let mut accel_2: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                pos_1.x =
                    (*otherTw).actor.world.pos.x +
                        Rand_CenteredFloat(50.0f32);
                pos_1.y =
                    (*otherTw).actor.world.pos.y +
                        Rand_CenteredFloat(50.0f32);
                pos_1.z =
                    (*otherTw).actor.world.pos.z +
                        Rand_CenteredFloat(50.0f32);
                velocity_2.x = Rand_CenteredFloat(20.0f32);
                velocity_2.y = Rand_CenteredFloat(20.0f32);
                velocity_2.z = Rand_CenteredFloat(20.0f32);
                accel_2.x = 0.0f32;
                accel_2.y = 0.0f32;
                accel_2.z = 0.0f32;
                BossTw_AddFlameEffect(globalCtx, &mut pos_1, &mut velocity_2,
                                      &mut accel_2,
                                      Rand_ZeroFloat(10.0f32) + 25.0f32,
                                      (*this).actor.params);
                i += 1
            }
            BossTw_SetupHitByBeam(otherTw, globalCtx);
            Audio_PlayActorSound2(&mut (*otherTw).actor,
                                  0x3916 as libc::c_int as u16_0);
            (*globalCtx).envCtx.unk_D8 = 1.0f32;
            (*otherTw).actor.colChkInfo.health =
                (*otherTw).actor.colChkInfo.health.wrapping_add(1)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_SetupFinishBeamShoot(mut this: *mut BossTw,
                                                     mut globalCtx:
                                                         *mut GlobalContext) {
    (*this).actionFunc =
        Some(BossTw_FinishBeamShoot as
                 unsafe extern "C" fn(_: *mut BossTw, _: *mut GlobalContext)
                     -> ());
    Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                              &mut object_tw_Anim_004548, 0.0f32);
    (*this).workf[ANIM_SW_TGT as libc::c_int as usize] =
        Animation_GetLastFrame(&mut object_tw_Anim_004548 as
                                   *mut AnimationHeader as *mut libc::c_void)
            as f32_0;
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_FinishBeamShoot(mut this: *mut BossTw,
                                                mut globalCtx:
                                                    *mut GlobalContext) {
    SkelAnime_Update(&mut (*this).skelAnime);
    Math_ApproachF(&mut (*this).scepterAlpha, 0.0f32, 1.0f32, 10.0f32);
    if Animation_OnFrame(&mut (*this).skelAnime,
                         (*this).workf[ANIM_SW_TGT as libc::c_int as usize])
           != 0 {
        if (*sTwinrovaPtr).timers[2 as libc::c_int as usize] as libc::c_int ==
               0 as libc::c_int {
            BossTw_SetupFlyTo(this, globalCtx);
        } else { BossTw_SetupLaugh(this, globalCtx); }
        (*this).scepterAlpha = 0.0f32
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_SetupHitByBeam(mut this: *mut BossTw,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    (*this).actionFunc =
        Some(BossTw_HitByBeam as
                 unsafe extern "C" fn(_: *mut BossTw, _: *mut GlobalContext)
                     -> ());
    Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                              &mut object_tw_Anim_00578C, 0.0f32);
    (*this).timers[0 as libc::c_int as usize] = 53 as libc::c_int as s16;
    (*this).actor.speedXZ = 0.0f32;
    if (*this).actor.params as libc::c_int == 0 as libc::c_int {
        (*this).work[FOG_TIMER as libc::c_int as usize] =
            20 as libc::c_int as s16
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_HitByBeam(mut this: *mut BossTw,
                                          mut globalCtx: *mut GlobalContext) {
    SkelAnime_Update(&mut (*this).skelAnime);
    if (*this).work[CS_TIMER_1 as libc::c_int as usize] as libc::c_int %
           4 as libc::c_int == 0 as libc::c_int {
        let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
        let mut velocity: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
        let mut accel: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
        pos.x = (*this).actor.world.pos.x + Rand_CenteredFloat(80.0f32);
        pos.y = (*this).actor.world.pos.y + Rand_CenteredFloat(80.0f32);
        pos.z = (*this).actor.world.pos.z + Rand_CenteredFloat(80.0f32);
        velocity.x = 0.0f32;
        velocity.y = 0.0f32;
        velocity.z = 0.0f32;
        accel.x = 0.0f32;
        accel.y = 0.1f32;
        accel.z = 0.0f32;
        BossTw_AddDmgCloud(globalCtx,
                           ((*this).actor.params as libc::c_int +
                                2 as libc::c_int) as s16, &mut pos,
                           &mut velocity, &mut accel,
                           Rand_ZeroFloat(10.0f32) + 15.0f32,
                           0 as libc::c_int as s16, 0 as libc::c_int as s16,
                           150 as libc::c_int as s16);
    }
    if (*this).actor.params as libc::c_int == 1 as libc::c_int {
        Math_ApproachF(&mut (*this).fogR, 255.0f32, 1.0f32, 30.0f32);
        Math_ApproachF(&mut (*this).fogG, 255.0f32, 1.0f32, 30.0f32);
        Math_ApproachF(&mut (*this).fogB, 255.0f32, 1.0f32, 30.0f32);
        Math_ApproachF(&mut (*this).fogNear, 900.0f32, 1.0f32, 30.0f32);
        Math_ApproachF(&mut (*this).fogFar, 1099.0f32, 1.0f32, 30.0f32);
    }
    Math_ApproachF(&mut (*this).actor.world.pos.y,
                   Math_SinS(((*this).work[CS_TIMER_1 as libc::c_int as usize]
                                  as libc::c_int * 1500 as libc::c_int) as
                                 s16) * 20.0f32 + 350.0f32 + 50.0f32, 0.1f32,
                   (*this).actor.speedXZ);
    Math_ApproachF(&mut (*this).actor.speedXZ, 5.0f32, 1.0f32, 1.0f32);
    (*this).actor.world.pos.y -= 50.0f32;
    Actor_UpdateBgCheckInfo(globalCtx, &mut (*this).actor, 50.0f32, 50.0f32,
                            100.0f32, 4 as libc::c_int);
    (*this).actor.world.pos.y += 50.0f32;
    if (*this).actor.bgCheckFlags as libc::c_int & 1 as libc::c_int != 0 {
        (*this).actor.speedXZ = 0.0f32
    }
    if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
           1 as libc::c_int {
        Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                                  &mut object_tw_Anim_006530, 0.0f32);
        (*this).workf[ANIM_SW_TGT as libc::c_int as usize] =
            Animation_GetLastFrame(&mut object_tw_Anim_006530 as
                                       *mut AnimationHeader as
                                       *mut libc::c_void) as f32_0
    }
    if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
           0 as libc::c_int &&
           Animation_OnFrame(&mut (*this).skelAnime,
                             (*this).workf[ANIM_SW_TGT as libc::c_int as
                                               usize]) != 0 {
        BossTw_SetupFlyTo(this, globalCtx);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_SetupLaugh(mut this: *mut BossTw,
                                           mut globalCtx:
                                               *mut GlobalContext) {
    (*this).actionFunc =
        Some(BossTw_Laugh as
                 unsafe extern "C" fn(_: *mut BossTw, _: *mut GlobalContext)
                     -> ());
    Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                              &mut object_tw_Anim_0088C8, 0.0f32);
    (*this).workf[ANIM_SW_TGT as libc::c_int as usize] =
        Animation_GetLastFrame(&mut object_tw_Anim_0088C8 as
                                   *mut AnimationHeader as *mut libc::c_void)
            as f32_0;
    (*this).actor.speedXZ = 0.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_Laugh(mut this: *mut BossTw,
                                      mut globalCtx: *mut GlobalContext) {
    SkelAnime_Update(&mut (*this).skelAnime);
    if Animation_OnFrame(&mut (*this).skelAnime, 10.0f32) != 0 {
        if (*this).actor.params as libc::c_int == TW_KOUME as libc::c_int {
            Audio_PlayActorSound2(&mut (*this).actor,
                                  0x39b0 as libc::c_int as u16_0);
        } else {
            Audio_PlayActorSound2(&mut (*this).actor,
                                  0x39b1 as libc::c_int as u16_0);
        }
    }
    if Animation_OnFrame(&mut (*this).skelAnime,
                         (*this).workf[ANIM_SW_TGT as libc::c_int as usize])
           != 0 {
        BossTw_SetupFlyTo(this, globalCtx);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_SetupSpin(mut this: *mut BossTw,
                                          mut globalCtx: *mut GlobalContext) {
    (*this).actionFunc =
        Some(BossTw_Spin as
                 unsafe extern "C" fn(_: *mut BossTw, _: *mut GlobalContext)
                     -> ());
    Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                              &mut object_tw_Anim_007CA8, -3.0f32);
    (*this).workf[ANIM_SW_TGT as libc::c_int as usize] =
        Animation_GetLastFrame(&mut object_tw_Anim_007CA8 as
                                   *mut AnimationHeader as *mut libc::c_void)
            as f32_0;
    (*this).actor.speedXZ = 0.0f32;
    SkelAnime_Update(&mut (*this).skelAnime);
    (*this).timers[0 as libc::c_int as usize] = 20 as libc::c_int as s16;
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_Spin(mut this: *mut BossTw,
                                     mut globalCtx: *mut GlobalContext) {
    if (*this).timers[0 as libc::c_int as usize] as libc::c_int !=
           0 as libc::c_int {
        (*this).collider.base.colType = COLTYPE_METAL as libc::c_int as u8_0;
        (*this).actor.shape.rot.y =
            ((*this).actor.shape.rot.y as libc::c_int - 0x3000 as libc::c_int)
                as s16;
        if (*this).timers[0 as libc::c_int as usize] as libc::c_int %
               4 as libc::c_int == 0 as libc::c_int {
            Audio_PlayActorSound2(&mut (*this).actor,
                                  0x3921 as libc::c_int as u16_0);
        }
    } else {
        SkelAnime_Update(&mut (*this).skelAnime);
        Math_ApproachS(&mut (*this).actor.shape.rot.y,
                       (*this).actor.world.rot.y, 3 as libc::c_int as s16,
                       0x2000 as libc::c_int as s16);
        if Animation_OnFrame(&mut (*this).skelAnime,
                             (*this).workf[ANIM_SW_TGT as libc::c_int as
                                               usize]) != 0 {
            BossTw_SetupFlyTo(this, globalCtx);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_SetupMergeCS(mut this: *mut BossTw,
                                             mut globalCtx:
                                                 *mut GlobalContext) {
    (*this).actionFunc =
        Some(BossTw_MergeCS as
                 unsafe extern "C" fn(_: *mut BossTw, _: *mut GlobalContext)
                     -> ());
    (*this).rotateSpeed = 0.0f32;
    (*this).actor.speedXZ = 0.0f32;
    Animation_MorphToLoop(&mut (*this).skelAnime, &mut object_tw_Anim_006F28,
                          -10.0f32);
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_MergeCS(mut this: *mut BossTw,
                                        mut globalCtx: *mut GlobalContext) {
    Math_ApproachF(&mut (*this).scepterAlpha, 0.0f32, 1.0f32, 10.0f32);
    SkelAnime_Update(&mut (*this).skelAnime);
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_SetupWait(mut this: *mut BossTw,
                                          mut globalCtx: *mut GlobalContext) {
    (*this).actionFunc =
        Some(BossTw_Wait as
                 unsafe extern "C" fn(_: *mut BossTw, _: *mut GlobalContext)
                     -> ());
    (*this).visible = 0 as libc::c_int as u8_0;
    (*this).actor.world.pos.y = -2000.0f32;
    (*this).actor.flags &=
        !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_Wait(mut this: *mut BossTw,
                                     mut globalCtx: *mut GlobalContext) {
    if (*this).actor.params as libc::c_int == TW_TWINROVA as libc::c_int &&
           (*sKoumePtr).actionFunc ==
               Some(BossTw_FlyTo as
                        unsafe extern "C" fn(_: *mut BossTw,
                                             _: *mut GlobalContext) -> ()) &&
           (*sKotakePtr).actionFunc ==
               Some(BossTw_FlyTo as
                        unsafe extern "C" fn(_: *mut BossTw,
                                             _: *mut GlobalContext) -> ()) &&
           (*sKoumePtr).actor.colChkInfo.health as libc::c_int +
               (*sKotakePtr).actor.colChkInfo.health as libc::c_int >=
               4 as libc::c_int {
        BossTw_TwinrovaSetupMergeCS(this, globalCtx);
        BossTw_SetupMergeCS(sKotakePtr, globalCtx);
        BossTw_SetupMergeCS(sKoumePtr, globalCtx);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_TwinrovaSetupMergeCS(mut this: *mut BossTw,
                                                     mut globalCtx:
                                                         *mut GlobalContext) {
    (*this).actionFunc =
        Some(BossTw_TwinrovaMergeCS as
                 unsafe extern "C" fn(_: *mut BossTw, _: *mut GlobalContext)
                     -> ());
    (*this).csState2 = 0 as libc::c_int as s16;
    (*this).csState1 = 0 as libc::c_int as s16;
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_TwinrovaMergeCS(mut this: *mut BossTw,
                                                mut globalCtx:
                                                    *mut GlobalContext) {
    let mut i: s16 = 0;
    let mut spB0: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut spA4: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut current_block_47: u64;
    match (*this).csState2 as libc::c_int {
        0 => {
            (*this).csState2 = 1 as libc::c_int as s16;
            func_80064520(globalCtx, &mut (*globalCtx).csCtx);
            func_8002DF54(globalCtx, &mut (*this).actor,
                          0x39 as libc::c_int as u8_0);
            (*this).subCamId = Gameplay_CreateSubCamera(globalCtx);
            Gameplay_ChangeCameraStatus(globalCtx, 0 as libc::c_int as s16,
                                        1 as libc::c_int as s16);
            Gameplay_ChangeCameraStatus(globalCtx, (*this).subCamId,
                                        7 as libc::c_int as s16);
            (*this).subCamDist = 800.0f32;
            (*this).subCamYaw = 3.14159265358979323846f32;
            (*sKoumePtr).actor.world.rot.x = 0 as libc::c_int as s16;
            (*sKoumePtr).actor.shape.rot.x = 0 as libc::c_int as s16;
            (*sKotakePtr).actor.world.rot.x = 0 as libc::c_int as s16;
            (*sKotakePtr).actor.shape.rot.x = 0 as libc::c_int as s16;
            (*this).workf[UNK_F9 as libc::c_int as usize] = 0.0f32;
            (*this).workf[UNK_F10 as libc::c_int as usize] = 0.0f32;
            (*this).workf[UNK_F11 as libc::c_int as usize] = 600.0f32;
            Audio_QueueSeqCmd(((0x1 as libc::c_int) << 28 as libc::c_int |
                                   (SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                                       24 as libc::c_int |
                                   0xc800ff as libc::c_int) as u32_0);
            (*this).work[CS_TIMER_2 as libc::c_int as usize] =
                0 as libc::c_int as s16;
            current_block_47 = 13528689403053953748;
        }
        1 => { current_block_47 = 13528689403053953748; }
        2 => {
            spB0.x = 0.0f32;
            spB0.y = 0.0f32;
            spB0.z = (*this).subCamDist;
            Matrix_RotateY((*this).subCamYaw,
                           MTXMODE_NEW as libc::c_int as u8_0);
            Matrix_MultVec3f(&mut spB0, &mut spA4);
            (*this).subCamEye.x = spA4.x;
            (*this).subCamEye.z = spA4.z;
            Math_ApproachF(&mut (*this).subCamEye.y, 420.0f32, 0.1f32,
                           (*this).subCamUpdateRate * 20.0f32);
            Math_ApproachF(&mut (*this).subCamAt.y, 470.0f32, 0.1f32,
                           (*this).subCamUpdateRate * 6.0f32);
            Math_ApproachF(&mut (*this).subCamYaw, 0.3f32, 0.02f32, 0.03f32);
            Math_ApproachF(&mut (*this).subCamDist, 60.0f32, 0.1f32,
                           (*this).subCamUpdateRate * 32.0f32);
            Math_ApproachF(&mut (*this).subCamUpdateRate,
                           1 as libc::c_int as f32_0,
                           1 as libc::c_int as f32_0, 0.1f32);
            current_block_47 = 1622411330066726685;
        }
        _ => { current_block_47 = 1622411330066726685; }
    }
    match current_block_47 {
        13528689403053953748 =>
        // fallthrough
        {
            if (*this).work[CS_TIMER_2 as libc::c_int as usize] as libc::c_int
                   == 20 as libc::c_int {
                Message_StartTextbox(globalCtx,
                                     0x6059 as libc::c_int as u16_0,
                                     0 as *mut Actor);
            }
            if (*this).work[CS_TIMER_2 as libc::c_int as usize] as libc::c_int
                   == 80 as libc::c_int {
                Message_StartTextbox(globalCtx,
                                     0x605a as libc::c_int as u16_0,
                                     0 as *mut Actor);
            }
            (*this).subCamAt.x = 0.0f32;
            (*this).subCamAt.y = 440.0f32;
            (*this).subCamAt.z = 0.0f32;
            spB0.x = 0.0f32;
            spB0.y = 0.0f32;
            spB0.z = (*this).subCamDist;
            Matrix_RotateY((*this).subCamYaw,
                           MTXMODE_NEW as libc::c_int as u8_0);
            Matrix_MultVec3f(&mut spB0, &mut spA4);
            (*this).subCamEye.x = spA4.x;
            (*this).subCamEye.y = 300.0f32;
            (*this).subCamEye.z = spA4.z;
            Math_ApproachF(&mut (*this).subCamYaw, 0.3f32, 0.02f32, 0.03f32);
            Math_ApproachF(&mut (*this).subCamDist, 200.0f32, 0.1f32, 5.0f32);
        }
        _ => { }
    }
    if (*this).subCamId as libc::c_int != 0 as libc::c_int {
        if (*this).unk_5F9 as libc::c_int == 0 as libc::c_int {
            Gameplay_CameraSetAtEye(globalCtx, (*this).subCamId,
                                    &mut (*this).subCamAt,
                                    &mut (*this).subCamEye);
        } else {
            Gameplay_CameraSetAtEye(globalCtx, (*this).subCamId,
                                    &mut (*this).subCamAt2,
                                    &mut (*this).subCamEye2);
        }
    }
    let mut current_block_177: u64;
    match (*this).csState1 as libc::c_int {
        0 => {
            Audio_PlayActorSound2(&mut (*sKotakePtr).actor,
                                  (0x391f as libc::c_int -
                                       0x800 as libc::c_int) as u16_0);
            Audio_PlayActorSound2(&mut (*sKoumePtr).actor,
                                  (0x391f as libc::c_int -
                                       0x800 as libc::c_int) as u16_0);
            spB0.x = (*this).workf[UNK_F11 as libc::c_int as usize];
            spB0.y = 400.0f32;
            spB0.z = 0.0f32;
            Matrix_RotateY((*this).workf[UNK_F9 as libc::c_int as usize],
                           MTXMODE_NEW as libc::c_int as u8_0);
            Matrix_MultVec3f(&mut spB0, &mut spA4);
            (*sKoumePtr).actor.world.pos.x = spA4.x;
            (*sKoumePtr).actor.world.pos.y = spA4.y;
            (*sKoumePtr).actor.world.pos.z = spA4.z;
            (*sKoumePtr).actor.shape.rot.y =
                ((*this).workf[UNK_F9 as libc::c_int as usize] /
                     3.14159265358979323846f32 * 32768.0f32) as s16;
            (*sKotakePtr).actor.world.pos.x = -spA4.x;
            (*sKotakePtr).actor.world.pos.y = spA4.y;
            (*sKotakePtr).actor.world.pos.z = -spA4.z;
            (*sKotakePtr).actor.shape.rot.y =
                ((*this).workf[UNK_F9 as libc::c_int as usize] /
                     3.14159265358979323846f32 * 32768.0f32 + 32768.0f32) as
                    s16;
            Math_ApproachF(&mut *(*this).workf.as_mut_ptr().offset(UNK_F11 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                           0.0f32, 0.1f32, 7.0f32);
            (*this).workf[UNK_F9 as libc::c_int as usize] -=
                (*this).workf[UNK_F10 as libc::c_int as usize];
            Math_ApproachF(&mut *(*this).workf.as_mut_ptr().offset(UNK_F10 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                           0.5f32, 1 as libc::c_int as f32_0,
                           0.0039999997f32);
            if (*this).workf[UNK_F11 as libc::c_int as usize] < 10.0f32 {
                if (*this).work[PLAYED_CHRG_SFX as libc::c_int as usize] == 0
                   {
                    Audio_PlayActorSound2(&mut (*sKoumePtr).actor,
                                          0x3910 as libc::c_int as u16_0);
                    (*this).work[PLAYED_CHRG_SFX as libc::c_int as usize] =
                        1 as libc::c_int as s16
                }
                Math_ApproachF(&mut (*sKoumePtr).actor.scale.x,
                               0.005000001f32, 1 as libc::c_int as f32_0,
                               0.0003750001f32);
                i = 0 as libc::c_int as s16;
                while (i as libc::c_int) < 4 as libc::c_int {
                    let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                    let mut yOffset: f32_0 = 0.;
                    let mut xScale: f32_0 = 0.;
                    xScale = (*sKoumePtr).actor.scale.x * 3000.0f32;
                    yOffset = Rand_CenteredFloat(xScale * 2.0f32);
                    pos.x = 3000.0f32;
                    pos.y = 400.0f32 + yOffset;
                    pos.z = 0.0f32;
                    BossTw_AddMergeFlameEffect(globalCtx, &mut pos,
                                               Rand_ZeroFloat(5.0f32) +
                                                   10.0f32,
                                               sqrtf(xScale * xScale -
                                                         yOffset * yOffset),
                                               Rand_ZeroFloat(1.99f32) as
                                                   s16);
                    i += 1
                }
                if (*sKoumePtr).actor.scale.x <= 0.0051f32 {
                    let mut pos_0: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                    let mut velocity: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                    let mut accel: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                    (*this).actor.world.pos.y = 400.0f32;
                    i = 0 as libc::c_int as s16;
                    while (i as libc::c_int) < 50 as libc::c_int {
                        pos_0 = (*this).actor.world.pos;
                        velocity.x = Rand_CenteredFloat(20.0f32);
                        velocity.y = Rand_CenteredFloat(20.0f32);
                        velocity.z = Rand_CenteredFloat(20.0f32);
                        pos_0.x += velocity.x;
                        pos_0.y += velocity.y;
                        pos_0.z += velocity.z;
                        accel.x = 0.0f32;
                        accel.y = accel.x;
                        accel.z = accel.y;
                        BossTw_AddFlameEffect(globalCtx, &mut pos_0,
                                              &mut velocity, &mut accel,
                                              Rand_ZeroFloat(10.0f32) +
                                                  25.0f32,
                                              (velocity.x < 0.0f32) as
                                                  libc::c_int as s16);
                        i += 1
                    }
                    (*this).csState1 = 1 as libc::c_int as s16;
                    (*this).visible = 1 as libc::c_int as u8_0;
                    (*this).actor.flags |=
                        ((1 as libc::c_int) << 0 as libc::c_int) as
                            libc::c_uint;
                    (*this).actor.shape.rot.y = 0 as libc::c_int as s16;
                    BossTw_SetupWait(sKotakePtr, globalCtx);
                    BossTw_SetupWait(sKoumePtr, globalCtx);
                    Actor_SetScale(&mut (*this).actor, 0.0f32);
                    Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                                              &mut object_tw_Anim_038E2C,
                                              0.0f32);
                    (*this).workf[ANIM_SW_TGT as libc::c_int as usize] =
                        Animation_GetLastFrame(&mut object_tw_Anim_038E2C as
                                                   *mut AnimationHeader as
                                                   *mut libc::c_void) as
                            f32_0;
                    (*this).timers[0 as libc::c_int as usize] =
                        50 as libc::c_int as s16;
                    func_8002DF54(globalCtx, &mut (*this).actor,
                                  2 as libc::c_int as u8_0);
                    Audio_PlayActorSound2(&mut (*this).actor,
                                          0x390d as libc::c_int as u16_0);
                    Audio_QueueSeqCmd(((SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                                           24 as libc::c_int |
                                           0x1b as libc::c_int) as u32_0);
                }
            }
            (*sKoumePtr).actor.scale.z = (*sKoumePtr).actor.scale.x;
            (*sKoumePtr).actor.scale.y = (*sKoumePtr).actor.scale.z;
            (*sKotakePtr).actor.scale.z = (*sKoumePtr).actor.scale.y;
            (*sKotakePtr).actor.scale.y = (*sKotakePtr).actor.scale.z;
            (*sKotakePtr).actor.scale.x = (*sKotakePtr).actor.scale.y;
            current_block_177 = 3640574680223420878;
        }
        1 => {
            if Animation_OnFrame(&mut (*this).skelAnime,
                                 (*this).workf[ANIM_SW_TGT as libc::c_int as
                                                   usize]) != 0 {
                Animation_MorphToLoop(&mut (*this).skelAnime,
                                      &mut object_tw_Anim_032BF8, -15.0f32);
            }
            sEnvType = -(1 as libc::c_int) as s8;
            (*globalCtx).envCtx.unk_BD = 4 as libc::c_int as u8_0;
            Math_ApproachF(&mut (*globalCtx).envCtx.unk_D8,
                           1 as libc::c_int as f32_0,
                           1 as libc::c_int as f32_0, 0.1f32);
            current_block_177 = 729329823820679814;
        }
        2 => { current_block_177 = 729329823820679814; }
        _ => { current_block_177 = 3640574680223420878; }
    }
    match current_block_177 {
        729329823820679814 =>
        // fallthrough
        {
            SkelAnime_Update(&mut (*this).skelAnime);
            Math_ApproachF(&mut (*this).actor.scale.x, 0.0069999993f32,
                           1 as libc::c_int as f32_0, 0.0006999999f32);
            (*this).actor.scale.z = (*this).actor.scale.x;
            (*this).actor.scale.y = (*this).actor.scale.z;
            if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
                   1 as libc::c_int {
                (*this).csState2 = 2 as libc::c_int as s16;
                (*this).subCamUpdateRate = 0.0f32;
                (*this).timers[1 as libc::c_int as usize] =
                    65 as libc::c_int as s16;
                (*this).timers[2 as libc::c_int as usize] =
                    90 as libc::c_int as s16;
                (*this).timers[3 as libc::c_int as usize] =
                    50 as libc::c_int as s16;
                (*player).actor.world.pos.x = 0.0f32;
                (*player).actor.world.pos.y = 240.0f32;
                (*player).actor.world.pos.z = 270.0f32;
                (*player).actor.shape.rot.y = -(0x8000 as libc::c_int) as s16;
                (*player).actor.world.rot.y = (*player).actor.shape.rot.y;
                (*this).subCamEye2.x = 0.0f32;
                (*this).subCamEye2.y = 290.0f32;
                (*this).subCamEye2.z = 222.0f32;
                (*this).subCamAt2.x = (*player).actor.world.pos.x;
                (*this).subCamAt2.y = (*player).actor.world.pos.y + 54.0f32;
                (*this).subCamAt2.z = (*player).actor.world.pos.z
            }
            if (*this).timers[3 as libc::c_int as usize] as libc::c_int ==
                   19 as libc::c_int {
                func_8002DF54(globalCtx, &mut (*this).actor,
                              5 as libc::c_int as u8_0);
            }
            if (*this).timers[3 as libc::c_int as usize] as libc::c_int ==
                   16 as libc::c_int {
                func_8002F7DC(&mut (*player).actor,
                              ((*(*player).ageProperties).unk_92 as
                                   libc::c_int + 0x6816 as libc::c_int) as
                                  u16_0);
            }
            if (*this).timers[3 as libc::c_int as usize] as libc::c_int !=
                   0 as libc::c_int &&
                   ((*this).timers[3 as libc::c_int as usize] as libc::c_int)
                       < 20 as libc::c_int {
                (*this).unk_5F9 = 1 as libc::c_int as u8_0;
                Math_ApproachF(&mut (*this).subCamEye2.z, 242.0f32, 0.2f32,
                               100.0f32);
            } else { (*this).unk_5F9 = 0 as libc::c_int as u8_0 }
            if (*this).timers[1 as libc::c_int as usize] as libc::c_int ==
                   8 as libc::c_int {
                (*this).work[TW_BLINK_IDX as libc::c_int as usize] =
                    8 as libc::c_int as s16;
                func_80078884(0x39bb as libc::c_int as u16_0);
            }
            if (*this).timers[2 as libc::c_int as usize] as libc::c_int ==
                   4 as libc::c_int {
                sEnvType = 0 as libc::c_int as s8;
                (*globalCtx).envCtx.unk_BE = 5 as libc::c_int as u8_0
            }
            if (*this).timers[2 as libc::c_int as usize] as libc::c_int ==
                   1 as libc::c_int {
                let mut cam: *mut Camera =
                    Gameplay_GetCamera(globalCtx, 0 as libc::c_int as s16);
                (*cam).eye = (*this).subCamEye;
                (*cam).eyeNext = (*this).subCamEye;
                (*cam).at = (*this).subCamAt;
                func_800C08AC(globalCtx, (*this).subCamId,
                              0 as libc::c_int as s16);
                (*this).subCamId = 0 as libc::c_int as s16;
                (*this).csState2 = (*this).subCamId;
                func_80064534(globalCtx, &mut (*globalCtx).csCtx);
                func_8002DF54(globalCtx, &mut (*this).actor,
                              7 as libc::c_int as u8_0);
                (*this).work[TW_PLLR_IDX as libc::c_int as usize] =
                    0 as libc::c_int as s16;
                (*this).targetPos =
                    sTwinrovaPillarPos[0 as libc::c_int as usize];
                BossTw_TwinrovaSetupFly(this, globalCtx);
            }
        }
        _ => { }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_SetupDeathCS(mut this: *mut BossTw,
                                             mut globalCtx:
                                                 *mut GlobalContext) {
    (*this).actionFunc =
        Some(BossTw_DeathCS as
                 unsafe extern "C" fn(_: *mut BossTw, _: *mut GlobalContext)
                     -> ());
    Animation_MorphToLoop(&mut (*this).skelAnime, &mut object_tw_Anim_0004A4,
                          -3.0f32);
    (*this).unk_5F8 = 0 as libc::c_int as u8_0;
    (*this).work[CS_TIMER_2 as libc::c_int as usize] =
        Rand_ZeroFloat(20.0f32) as s16;
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_DeathCS(mut this: *mut BossTw,
                                        mut globalCtx: *mut GlobalContext) {
    if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
           0 as libc::c_int {
        SkelAnime_Update(&mut (*this).skelAnime);
    }
    Math_ApproachS(&mut (*this).actor.shape.rot.y,
                   (*this).work[YAW_TGT as libc::c_int as usize],
                   5 as libc::c_int as s16, (*this).rotateSpeed as s16);
    Math_ApproachF(&mut (*this).rotateSpeed, 20480.0f32, 1.0f32, 1000.0f32);
    if (*sTwinrovaPtr).work[CS_TIMER_2 as libc::c_int as usize] as libc::c_int
           > 140 as libc::c_int {
        Math_ApproachF(&mut (*this).fogR, 100.0f32, 1.0f32, 15.0f32);
        Math_ApproachF(&mut (*this).fogG, 255.0f32, 1.0f32, 15.0f32);
        Math_ApproachF(&mut (*this).fogB, 255.0f32, 1.0f32, 15.0f32);
        Math_ApproachF(&mut (*this).fogNear, 850.0f32, 1.0f32, 15.0f32);
        Math_ApproachF(&mut (*this).fogFar, 1099.0f32, 1.0f32, 15.0f32);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_SetupCSWait(mut this: *mut BossTw,
                                            mut globalCtx:
                                                *mut GlobalContext) {
    (*this).actionFunc =
        Some(BossTw_CSWait as
                 unsafe extern "C" fn(_: *mut BossTw, _: *mut GlobalContext)
                     -> ());
    (*this).visible = 0 as libc::c_int as u8_0;
    (*this).actor.world.pos.y = -2000.0f32;
    (*this).actor.flags &=
        !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
}
/* *
 * Do nothing while waiting for the inital cutscene to start
 */
#[no_mangle]
pub unsafe extern "C" fn BossTw_CSWait(mut this: *mut BossTw,
                                       mut globalCtx: *mut GlobalContext) {
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_TwinrovaSetupIntroCS(mut this: *mut BossTw,
                                                     mut globalCtx:
                                                         *mut GlobalContext) {
    (*this).actionFunc =
        Some(BossTw_TwinrovaIntroCS as
                 unsafe extern "C" fn(_: *mut BossTw, _: *mut GlobalContext)
                     -> ());
    (*this).visible = 0 as libc::c_int as u8_0;
    (*this).actor.world.pos.y = -2000.0f32;
    (*this).actor.flags &=
        !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_TwinrovaIntroCS(mut this: *mut BossTw,
                                                mut globalCtx:
                                                    *mut GlobalContext) {
    let mut updateCam: u8_0 = 0 as libc::c_int as u8_0;
    let mut i: s16 = 0;
    let mut sp90: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp84: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    if (*this).csSfxTimer as libc::c_int > 220 as libc::c_int &&
           ((*this).csSfxTimer as libc::c_int) < 630 as libc::c_int {
        func_80078884((0x3920 as libc::c_int - 0x800 as libc::c_int) as
                          u16_0);
    }
    if (*this).csSfxTimer as libc::c_int == 180 as libc::c_int {
        func_80078914(&mut D_8094A7D0, 0x39b0 as libc::c_int as u16_0);
        func_80078914(&mut D_8094A7D0, 0x39b1 as libc::c_int as u16_0);
        Audio_QueueSeqCmd(((SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                               24 as libc::c_int | 0x61 as libc::c_int) as
                              u32_0);
    }
    (*this).csSfxTimer += 1;
    match (*this).csState2 as libc::c_int {
        0 => {
            (*this).csSfxTimer = 0 as libc::c_int as s16;
            if (*player).actor.world.pos.x * (*player).actor.world.pos.x +
                   (*player).actor.world.pos.z * (*player).actor.world.pos.z <
                   150.0f32 * 150.0f32 {
                (*player).actor.world.pos.z = 0.0f32;
                (*player).actor.world.pos.x = (*player).actor.world.pos.z;
                (*this).csState2 = 1 as libc::c_int as s16;
                func_80064520(globalCtx, &mut (*globalCtx).csCtx);
                func_8002DF54(globalCtx, &mut (*this).actor,
                              0x39 as libc::c_int as u8_0);
                (*this).subCamId = Gameplay_CreateSubCamera(globalCtx);
                Gameplay_ChangeCameraStatus(globalCtx,
                                            0 as libc::c_int as s16,
                                            1 as libc::c_int as s16);
                Gameplay_ChangeCameraStatus(globalCtx, (*this).subCamId,
                                            7 as libc::c_int as s16);
                (*this).subCamEye.x = 0.0f32;
                (*this).subCamEye.y = 350 as libc::c_int as f32_0;
                (*this).subCamEye.z = 200 as libc::c_int as f32_0;
                (*this).subCamEyeTarget.x = 450 as libc::c_int as f32_0;
                (*this).subCamEyeTarget.y = 900 as libc::c_int as f32_0;
                (*this).subCamAt.x = 0 as libc::c_int as f32_0;
                (*this).subCamAt.y = 270 as libc::c_int as f32_0;
                (*this).subCamAt.z = 0 as libc::c_int as f32_0;
                (*this).subCamAtTarget.x = 0 as libc::c_int as f32_0;
                (*this).subCamAtTarget.y = 240 as libc::c_int as f32_0;
                (*this).subCamAtTarget.z = 140 as libc::c_int as f32_0;
                (*this).subCamEyeTarget.z = 530 as libc::c_int as f32_0;
                (*this).subCamEyeStep.x =
                    fabsf((*this).subCamEyeTarget.x - (*this).subCamEye.x);
                (*this).subCamEyeStep.y =
                    fabsf((*this).subCamEyeTarget.y - (*this).subCamEye.y);
                (*this).subCamEyeStep.z =
                    fabsf((*this).subCamEyeTarget.z - (*this).subCamEye.z);
                (*this).subCamAtStep.x =
                    fabsf((*this).subCamAtTarget.x - (*this).subCamAt.x);
                (*this).subCamAtStep.y =
                    fabsf((*this).subCamAtTarget.y - (*this).subCamAt.y);
                (*this).subCamAtStep.z =
                    fabsf((*this).subCamAtTarget.z - (*this).subCamAt.z);
                (*this).subCamDistStep = 0.05f32;
                (*this).work[CS_TIMER_1 as libc::c_int as usize] =
                    0 as libc::c_int as s16
            }
        }
        1 => {
            updateCam = 1 as libc::c_int as u8_0;
            if (*this).work[CS_TIMER_1 as libc::c_int as usize] as libc::c_int
                   == 30 as libc::c_int {
                Message_StartTextbox(globalCtx,
                                     0x6048 as libc::c_int as u16_0,
                                     0 as *mut Actor);
            }
            Math_ApproachF(&mut (*this).subCamUpdateRate, 0.01f32, 1.0f32,
                           0.0001f32);
            if (*this).work[CS_TIMER_1 as libc::c_int as usize] as libc::c_int
                   > 100 as libc::c_int {
                (*globalCtx).envCtx.unk_BD = 0 as libc::c_int as u8_0;
                Math_ApproachF(&mut (*globalCtx).envCtx.unk_D8, 1.0f32,
                               1.0f32, 0.03f32);
            }
            if (*this).work[CS_TIMER_1 as libc::c_int as usize] as libc::c_int
                   == 180 as libc::c_int {
                func_80078884(0x390c as libc::c_int as u16_0);
            }
            if (*this).work[CS_TIMER_1 as libc::c_int as usize] as libc::c_int
                   > 180 as libc::c_int {
                (*this).spawnPortalScale = 0.05f32;
                Math_ApproachF(&mut (*this).spawnPortalAlpha, 255.0f32,
                               1.0f32, 5.0f32);
                if (*this).work[CS_TIMER_1 as libc::c_int as usize] as
                       libc::c_int >= 236 as libc::c_int {
                    (*this).csState2 = 2 as libc::c_int as s16;
                    (*sKoumePtr).visible = 1 as libc::c_int as u8_0;
                    Animation_MorphToLoop(&mut (*sKoumePtr).skelAnime,
                                          &mut object_tw_Anim_0004A4, 0.0f32);
                    (*sKoumePtr).actor.world.pos.x = 0.0f32;
                    (*sKoumePtr).actor.world.pos.y = 80.0f32;
                    (*sKoumePtr).actor.world.pos.z = 600.0f32;
                    (*sKoumePtr).actor.world.rot.y =
                        -(0x8000 as libc::c_int) as s16;
                    (*sKoumePtr).actor.shape.rot.y =
                        (*sKoumePtr).actor.world.rot.y;
                    (*this).subCamEye.x = -(30 as libc::c_int) as f32_0;
                    (*this).subCamEye.y = 260 as libc::c_int as f32_0;
                    (*this).subCamEye.z = 470 as libc::c_int as f32_0;
                    (*this).subCamAt.x = 0.0f32;
                    (*this).subCamAt.y = 270 as libc::c_int as f32_0;
                    (*this).subCamAt.z = 600.0f32;
                    (*this).work[CS_TIMER_1 as libc::c_int as usize] =
                        0 as libc::c_int as s16;
                    Actor_SetScale(&mut (*sKoumePtr).actor, 0.014999999f32);
                }
            }
        }
        2 => {
            SkelAnime_Update(&mut (*sKoumePtr).skelAnime);
            Math_ApproachF(&mut (*sKoumePtr).actor.world.pos.y, 240.0f32,
                           0.05f32, 5.0f32);
            (*this).subCamEye.x -= 0.2f32;
            (*this).subCamEye.z += 0.2f32;
            if (*this).work[CS_TIMER_1 as libc::c_int as usize] as libc::c_int
                   > 50 as libc::c_int {
                (*this).csState2 = 3 as libc::c_int as s16;
                (*this).subCamEyeTarget.x = -(30 as libc::c_int) as f32_0;
                (*this).subCamEyeTarget.y = 260 as libc::c_int as f32_0;
                (*this).subCamEyeTarget.z = 530 as libc::c_int as f32_0;
                (*this).subCamAtTarget.x = 0.0f32;
                (*this).subCamAtTarget.y = 265 as libc::c_int as f32_0;
                (*this).subCamAtTarget.z = 580 as libc::c_int as f32_0;
                (*this).subCamEyeStep.x =
                    fabsf((*this).subCamEyeTarget.x - (*this).subCamEye.x);
                (*this).subCamEyeStep.y =
                    fabsf((*this).subCamEyeTarget.y - (*this).subCamEye.y);
                (*this).subCamEyeStep.z =
                    fabsf((*this).subCamEyeTarget.z - (*this).subCamEye.z);
                (*this).subCamAtStep.x =
                    fabsf((*this).subCamAtTarget.x - (*this).subCamAt.x);
                (*this).subCamAtStep.y =
                    fabsf((*this).subCamAtTarget.y - (*this).subCamAt.y);
                (*this).subCamAtStep.z =
                    fabsf((*this).subCamAtTarget.z - (*this).subCamAt.z);
                (*this).subCamUpdateRate = 0 as libc::c_int as f32_0;
                (*this).subCamDistStep = 0.1f32;
                (*this).work[CS_TIMER_1 as libc::c_int as usize] =
                    0 as libc::c_int as s16
            }
        }
        3 => {
            SkelAnime_Update(&mut (*sKoumePtr).skelAnime);
            updateCam = 1 as libc::c_int as u8_0;
            Math_ApproachF(&mut (*sKoumePtr).actor.world.pos.y, 240.0f32,
                           0.05f32, 5.0f32);
            Math_ApproachF(&mut (*this).subCamUpdateRate, 1.0f32, 1.0f32,
                           0.02f32);
            if (*this).work[CS_TIMER_1 as libc::c_int as usize] as libc::c_int
                   == 30 as libc::c_int {
                Message_StartTextbox(globalCtx,
                                     0x6049 as libc::c_int as u16_0,
                                     0 as *mut Actor);
            }
            if (*this).work[CS_TIMER_1 as libc::c_int as usize] as libc::c_int
                   > 80 as libc::c_int {
                (*this).csState2 = 4 as libc::c_int as s16;
                (*this).actor.speedXZ = 0 as libc::c_int as f32_0;
                (*this).subCamEyeTarget.x = -80.0f32;
                (*this).subCamEyeTarget.y = 260.0f32;
                (*this).subCamEyeTarget.z = 430.0f32;
                (*this).subCamAtTarget.x = (*sKoumePtr).actor.world.pos.x;
                (*this).subCamAtTarget.y =
                    (*sKoumePtr).actor.world.pos.y + 20.0f32;
                (*this).subCamAtTarget.z = (*sKoumePtr).actor.world.pos.z;
                (*this).subCamEyeStep.x =
                    fabsf((*this).subCamEyeTarget.x - (*this).subCamEye.x);
                (*this).subCamEyeStep.y =
                    fabsf((*this).subCamEyeTarget.y - (*this).subCamEye.y);
                (*this).subCamEyeStep.z =
                    fabsf((*this).subCamEyeTarget.z - (*this).subCamEye.z);
                (*this).subCamAtStep.x =
                    fabsf((*this).subCamAtTarget.x - (*this).subCamAt.x);
                (*this).subCamAtStep.y =
                    fabsf((*this).subCamAtTarget.y - (*this).subCamAt.y);
                (*this).subCamAtStep.z =
                    fabsf((*this).subCamAtTarget.z - (*this).subCamAt.z);
                (*this).subCamUpdateRate = 0.0f32;
                (*this).subCamDistStep = 0.05f32;
                Animation_MorphToPlayOnce(&mut (*sKoumePtr).skelAnime,
                                          &mut object_tw_Anim_000AAC, 0.0f32);
                (*this).workf[ANIM_SW_TGT as libc::c_int as usize] =
                    Animation_GetLastFrame(&mut object_tw_Anim_000AAC as
                                               *mut AnimationHeader as
                                               *mut libc::c_void) as f32_0;
                (*this).work[CS_TIMER_1 as libc::c_int as usize] =
                    0 as libc::c_int as s16
            }
        }
        4 => {
            updateCam = 1 as libc::c_int as u8_0;
            SkelAnime_Update(&mut (*sKoumePtr).skelAnime);
            (*this).subCamAtTarget.y =
                20.0f32 + (*sKoumePtr).actor.world.pos.y;
            Math_ApproachF(&mut (*sKoumePtr).actor.world.pos.y,
                           350 as libc::c_int as f32_0, 0.1f32,
                           (*this).actor.speedXZ);
            Math_ApproachF(&mut (*this).actor.speedXZ, 9.0f32, 1.0f32,
                           0.9f32);
            Math_ApproachF(&mut (*this).subCamUpdateRate, 1.0f32, 1.0f32,
                           0.02f32);
            if (*this).work[CS_TIMER_1 as libc::c_int as usize] as libc::c_int
                   >= 30 as libc::c_int {
                if ((*this).work[CS_TIMER_1 as libc::c_int as usize] as
                        libc::c_int) < 45 as libc::c_int {
                    (*globalCtx).envCtx.unk_BE = 0 as libc::c_int as u8_0;
                    (*globalCtx).envCtx.unk_BD = 2 as libc::c_int as u8_0;
                    (*globalCtx).envCtx.unk_D8 = 1.0f32
                } else {
                    Math_ApproachZeroF(&mut (*globalCtx).envCtx.unk_D8,
                                       1.0f32, 0.1f32);
                }
                if (*this).work[CS_TIMER_1 as libc::c_int as usize] as
                       libc::c_int == 30 as libc::c_int {
                    i = 0 as libc::c_int as s16;
                    while (i as libc::c_int) < 50 as libc::c_int {
                        let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                        let mut velocity: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                        pos.x =
                            (*sKoumePtr).actor.world.pos.x +
                                Rand_CenteredFloat(50.0f32);
                        pos.y =
                            (*sKoumePtr).actor.world.pos.y +
                                Rand_CenteredFloat(50.0f32);
                        pos.z =
                            (*sKoumePtr).actor.world.pos.z +
                                Rand_CenteredFloat(50.0f32);
                        velocity.x = Rand_CenteredFloat(20.0f32);
                        velocity.y = Rand_CenteredFloat(20.0f32);
                        velocity.z = Rand_CenteredFloat(20.0f32);
                        BossTw_AddFlameEffect(globalCtx, &mut pos,
                                              &mut velocity, &mut sZeroVector,
                                              Rand_ZeroFloat(10.0f32) +
                                                  25.0f32,
                                              1 as libc::c_int as s16);
                        i += 1
                    }
                    Audio_PlayActorSound2(&mut (*sKoumePtr).actor,
                                          0x390d as libc::c_int as u16_0);
                    (*globalCtx).envCtx.unk_D8 = 0 as libc::c_int as f32_0
                }
                if (*this).work[CS_TIMER_1 as libc::c_int as usize] as
                       libc::c_int >= 35 as libc::c_int {
                    if ((*this).work[CS_TIMER_1 as libc::c_int as usize] as
                            libc::c_int) < 50 as libc::c_int {
                        Math_ApproachF(&mut (*sKoumePtr).actor.scale.x,
                                       Math_SinS(((*this).work[CS_TIMER_1 as
                                                                   libc::c_int
                                                                   as usize]
                                                      as libc::c_int *
                                                      0x4200 as libc::c_int)
                                                     as s16) * 20.0f32 /
                                           10000.0f32 + 0.024999999f32,
                                       1.0f32, 0.005f32);
                    } else {
                        if (*this).work[CS_TIMER_1 as libc::c_int as usize] as
                               libc::c_int == 50 as libc::c_int {
                            Animation_MorphToPlayOnce(&mut (*sKoumePtr).skelAnime,
                                                      &mut object_tw_Anim_0088C8,
                                                      -(5 as libc::c_int) as
                                                          f32_0);
                            (*this).workf[ANIM_SW_TGT as libc::c_int as usize]
                                =
                                Animation_GetLastFrame(&mut object_tw_Anim_0088C8
                                                           as
                                                           *mut AnimationHeader
                                                           as
                                                           *mut libc::c_void)
                                    as f32_0
                        }
                        if (*this).work[CS_TIMER_1 as libc::c_int as usize] as
                               libc::c_int == 60 as libc::c_int {
                            Audio_PlayActorSound2(&mut (*sKoumePtr).actor,
                                                  0x39b0 as libc::c_int as
                                                      u16_0);
                        }
                        if Animation_OnFrame(&mut (*sKoumePtr).skelAnime,
                                             (*this).workf[ANIM_SW_TGT as
                                                               libc::c_int as
                                                               usize]) != 0 {
                            Animation_MorphToLoop(&mut (*sKoumePtr).skelAnime,
                                                  &mut object_tw_Anim_006F28,
                                                  0.0f32);
                            (*this).workf[ANIM_SW_TGT as libc::c_int as usize]
                                = 1000.0f32
                        }
                        Math_ApproachF(&mut (*sKoumePtr).actor.scale.x,
                                       0.024999999f32, 0.1f32, 0.005f32);
                    }
                    Actor_SetScale(&mut (*sKoumePtr).actor,
                                   (*sKoumePtr).actor.scale.x);
                    (*sKoumePtr).actor.shape.rot.y =
                        -(0x8000 as libc::c_int) as s16;
                    (*sKoumePtr).unk_5F8 = 1 as libc::c_int as u8_0;
                    if (*this).work[CS_TIMER_1 as libc::c_int as usize] as
                           libc::c_int == 0x64 as libc::c_int {
                        (*this).csState2 = 10 as libc::c_int as s16;
                        (*this).work[CS_TIMER_1 as libc::c_int as usize] =
                            0 as libc::c_int as s16;
                        (*this).subCamYawStep = 0.0f32;
                        (*sKotakePtr).visible = 1 as libc::c_int as u8_0;
                        Animation_MorphToLoop(&mut (*sKotakePtr).skelAnime,
                                              &mut object_tw_Anim_0004A4,
                                              0.0f32);
                        (*sKotakePtr).actor.world.pos.x = 0.0f32;
                        (*sKotakePtr).actor.world.pos.y = 80.0f32;
                        (*sKotakePtr).actor.world.pos.z = -600.0f32;
                        (*sKotakePtr).actor.world.rot.y =
                            0 as libc::c_int as s16;
                        (*sKotakePtr).actor.shape.rot.y =
                            (*sKotakePtr).actor.world.rot.y;
                        (*this).work[CS_TIMER_1 as libc::c_int as usize] =
                            0 as libc::c_int as s16;
                        (*this).subCamEye.x = -30.0f32;
                        (*this).subCamEye.y = 260.0f32;
                        (*this).subCamEye.z = -470.0f32;
                        (*this).subCamAt.x = 0 as libc::c_int as f32_0;
                        (*this).subCamAt.y = 270.0f32;
                        (*this).subCamAt.z = -600.0f32;
                        Actor_SetScale(&mut (*sKotakePtr).actor,
                                       0.014999999f32);
                    }
                } else {
                    (*sKoumePtr).actor.shape.rot.y =
                        ((*sKoumePtr).actor.shape.rot.y as libc::c_int +
                             (*this).subCamYawStep as s16 as libc::c_int) as
                            s16
                }
            } else {
                if (*this).work[CS_TIMER_1 as libc::c_int as usize] as
                       libc::c_int % 8 as libc::c_int == 0 as libc::c_int {
                    Audio_PlayActorSound2(&mut (*sKoumePtr).actor,
                                          0x3921 as libc::c_int as u16_0);
                }
                (*sKoumePtr).actor.shape.rot.y =
                    ((*sKoumePtr).actor.shape.rot.y as libc::c_int +
                         (*this).subCamYawStep as s16 as libc::c_int) as s16;
                Math_ApproachF(&mut (*this).subCamYawStep, 12288.0f32, 1.0f32,
                               384.0f32);
                if Animation_OnFrame(&mut (*sKoumePtr).skelAnime,
                                     (*this).workf[ANIM_SW_TGT as libc::c_int
                                                       as usize]) != 0 {
                    Animation_MorphToLoop(&mut (*sKoumePtr).skelAnime,
                                          &mut object_tw_Anim_006F28, 0.0f32);
                    (*this).workf[ANIM_SW_TGT as libc::c_int as usize] =
                        1000.0f32
                }
            }
        }
        10 => {
            SkelAnime_Update(&mut (*sKotakePtr).skelAnime);
            Math_ApproachF(&mut (*sKotakePtr).actor.world.pos.y, 240.0f32,
                           0.05f32, 5.0f32);
            (*this).subCamEye.x -= 0.2f32;
            (*this).subCamEye.z -= 0.2f32;
            if (*this).work[CS_TIMER_1 as libc::c_int as usize] as libc::c_int
                   >= 0x33 as libc::c_int {
                (*this).csState2 = 11 as libc::c_int as s16;
                (*this).subCamEyeTarget.x = -(30 as libc::c_int) as f32_0;
                (*this).subCamEyeTarget.y = 260 as libc::c_int as f32_0;
                (*this).subCamEyeTarget.z = -(530 as libc::c_int) as f32_0;
                (*this).subCamAtTarget.x = 0 as libc::c_int as f32_0;
                (*this).subCamAtTarget.y = 265 as libc::c_int as f32_0;
                (*this).subCamAtTarget.z = -(580 as libc::c_int) as f32_0;
                (*this).subCamEyeStep.x =
                    fabsf((*this).subCamEyeTarget.x - (*this).subCamEye.x);
                (*this).subCamEyeStep.y =
                    fabsf((*this).subCamEyeTarget.y - (*this).subCamEye.y);
                (*this).subCamEyeStep.z =
                    fabsf((*this).subCamEyeTarget.z - (*this).subCamEye.z);
                (*this).subCamAtStep.x =
                    fabsf((*this).subCamAtTarget.x - (*this).subCamAt.x);
                (*this).subCamAtStep.y =
                    fabsf((*this).subCamAtTarget.y - (*this).subCamAt.y);
                (*this).subCamAtStep.z =
                    fabsf((*this).subCamAtTarget.z - (*this).subCamAt.z);
                (*this).subCamUpdateRate = 0 as libc::c_int as f32_0;
                (*this).subCamDistStep = 0.1f32;
                (*this).work[CS_TIMER_1 as libc::c_int as usize] =
                    0 as libc::c_int as s16
            }
        }
        11 => {
            SkelAnime_Update(&mut (*sKotakePtr).skelAnime);
            updateCam = 1 as libc::c_int as u8_0;
            Math_ApproachF(&mut (*sKotakePtr).actor.world.pos.y, 240.0f32,
                           0.05f32, 5.0f32);
            Math_ApproachF(&mut (*this).subCamUpdateRate, 1.0f32, 1.0f32,
                           0.02f32);
            if (*this).work[CS_TIMER_1 as libc::c_int as usize] as libc::c_int
                   == 30 as libc::c_int {
                Message_StartTextbox(globalCtx,
                                     0x604a as libc::c_int as u16_0,
                                     0 as *mut Actor);
            }
            if (*this).work[CS_TIMER_1 as libc::c_int as usize] as libc::c_int
                   > 80 as libc::c_int {
                (*this).csState2 = 12 as libc::c_int as s16;
                (*this).actor.speedXZ = 0 as libc::c_int as f32_0;
                (*this).subCamEyeTarget.y = 260.0f32;
                (*this).subCamEyeTarget.x = -80.0f32;
                (*this).subCamEyeTarget.z = -430.0f32;
                (*this).subCamAtTarget.x = (*sKotakePtr).actor.world.pos.x;
                (*this).subCamAtTarget.y =
                    (*sKotakePtr).actor.world.pos.y + 20.0f32;
                (*this).subCamAtTarget.z = (*sKotakePtr).actor.world.pos.z;
                (*this).subCamEyeStep.x =
                    fabsf((*this).subCamEyeTarget.x - (*this).subCamEye.x);
                (*this).subCamEyeStep.y =
                    fabsf((*this).subCamEyeTarget.y - (*this).subCamEye.y);
                (*this).subCamEyeStep.z =
                    fabsf((*this).subCamEyeTarget.z - (*this).subCamEye.z);
                (*this).subCamAtStep.x =
                    fabsf((*this).subCamAtTarget.x - (*this).subCamAt.x);
                (*this).subCamAtStep.y =
                    fabsf((*this).subCamAtTarget.y - (*this).subCamAt.y);
                (*this).subCamAtStep.z =
                    fabsf((*this).subCamAtTarget.z - (*this).subCamAt.z);
                (*this).subCamUpdateRate = 0 as libc::c_int as f32_0;
                (*this).subCamDistStep = 0.05f32;
                Animation_MorphToPlayOnce(&mut (*sKotakePtr).skelAnime,
                                          &mut object_tw_Anim_000AAC,
                                          0 as libc::c_int as f32_0);
                (*this).workf[ANIM_SW_TGT as libc::c_int as usize] =
                    Animation_GetLastFrame(&mut object_tw_Anim_000AAC as
                                               *mut AnimationHeader as
                                               *mut libc::c_void) as f32_0;
                (*this).work[CS_TIMER_1 as libc::c_int as usize] =
                    0 as libc::c_int as s16
            }
        }
        12 => {
            updateCam = 1 as libc::c_int as u8_0;
            SkelAnime_Update(&mut (*sKotakePtr).skelAnime);
            (*this).subCamAtTarget.y =
                (*sKotakePtr).actor.world.pos.y + 20.0f32;
            Math_ApproachF(&mut (*sKotakePtr).actor.world.pos.y,
                           350 as libc::c_int as f32_0, 0.1f32,
                           (*this).actor.speedXZ);
            Math_ApproachF(&mut (*this).actor.speedXZ, 9.0f32, 1.0f32,
                           0.9f32);
            Math_ApproachF(&mut (*this).subCamUpdateRate, 1.0f32, 1.0f32,
                           0.02f32);
            if (*this).work[CS_TIMER_1 as libc::c_int as usize] as libc::c_int
                   >= 30 as libc::c_int {
                if ((*this).work[CS_TIMER_1 as libc::c_int as usize] as
                        libc::c_int) < 45 as libc::c_int {
                    (*globalCtx).envCtx.unk_BD = 3 as libc::c_int as u8_0;
                    (*globalCtx).envCtx.unk_D8 = 1.0f32
                } else {
                    Math_ApproachZeroF(&mut (*globalCtx).envCtx.unk_D8,
                                       1.0f32, 0.1f32);
                }
                if (*this).work[CS_TIMER_1 as libc::c_int as usize] as
                       libc::c_int == 30 as libc::c_int {
                    i = 0 as libc::c_int as s16;
                    while (i as libc::c_int) < 50 as libc::c_int {
                        let mut pos_0: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                        let mut velocity_0: Vec3f =
                            Vec3f{x: 0., y: 0., z: 0.,};
                        pos_0.x =
                            (*sKotakePtr).actor.world.pos.x +
                                Rand_CenteredFloat(50.0f32);
                        pos_0.y =
                            (*sKotakePtr).actor.world.pos.y +
                                Rand_CenteredFloat(50.0f32);
                        pos_0.z =
                            (*sKotakePtr).actor.world.pos.z +
                                Rand_CenteredFloat(50.0f32);
                        velocity_0.x = Rand_CenteredFloat(20.0f32);
                        velocity_0.y = Rand_CenteredFloat(20.0f32);
                        velocity_0.z = Rand_CenteredFloat(20.0f32);
                        BossTw_AddFlameEffect(globalCtx, &mut pos_0,
                                              &mut velocity_0,
                                              &mut sZeroVector,
                                              Rand_ZeroFloat(10.0f32) +
                                                  25.0f32,
                                              0 as libc::c_int as s16);
                        i += 1
                    }
                    Audio_PlayActorSound2(&mut (*sKotakePtr).actor,
                                          0x390d as libc::c_int as u16_0);
                    (*globalCtx).envCtx.unk_D8 = 0.0f32
                }
                if (*this).work[CS_TIMER_1 as libc::c_int as usize] as
                       libc::c_int >= 35 as libc::c_int {
                    if ((*this).work[CS_TIMER_1 as libc::c_int as usize] as
                            libc::c_int) < 50 as libc::c_int {
                        Math_ApproachF(&mut (*sKotakePtr).actor.scale.x,
                                       Math_SinS(((*this).work[CS_TIMER_1 as
                                                                   libc::c_int
                                                                   as usize]
                                                      as libc::c_int *
                                                      0x4200 as libc::c_int)
                                                     as s16) * 20.0f32 /
                                           10000.0f32 + 0.024999999f32,
                                       1.0f32, 0.005f32);
                    } else {
                        if (*this).work[CS_TIMER_1 as libc::c_int as usize] as
                               libc::c_int == 50 as libc::c_int {
                            Animation_MorphToPlayOnce(&mut (*sKotakePtr).skelAnime,
                                                      &mut object_tw_Anim_0088C8,
                                                      -5.0f32);
                            (*this).workf[ANIM_SW_TGT as libc::c_int as usize]
                                =
                                Animation_GetLastFrame(&mut object_tw_Anim_0088C8
                                                           as
                                                           *mut AnimationHeader
                                                           as
                                                           *mut libc::c_void)
                                    as f32_0
                        }
                        if (*this).work[CS_TIMER_1 as libc::c_int as usize] as
                               libc::c_int == 60 as libc::c_int {
                            Audio_PlayActorSound2(&mut (*sKotakePtr).actor,
                                                  0x39b1 as libc::c_int as
                                                      u16_0);
                        }
                        if Animation_OnFrame(&mut (*sKotakePtr).skelAnime,
                                             (*this).workf[ANIM_SW_TGT as
                                                               libc::c_int as
                                                               usize]) != 0 {
                            Animation_MorphToLoop(&mut (*sKotakePtr).skelAnime,
                                                  &mut object_tw_Anim_006F28,
                                                  0.0f32);
                            (*this).workf[ANIM_SW_TGT as libc::c_int as usize]
                                = 1000.0f32
                        }
                        Math_ApproachF(&mut (*sKotakePtr).actor.scale.x,
                                       0.024999999f32, 0.1f32, 0.005f32);
                    }
                    Actor_SetScale(&mut (*sKotakePtr).actor,
                                   (*sKotakePtr).actor.scale.x);
                    (*sKotakePtr).actor.shape.rot.y = 0 as libc::c_int as s16;
                    (*sKotakePtr).unk_5F8 = 1 as libc::c_int as u8_0;
                    if (*this).work[CS_TIMER_1 as libc::c_int as usize] as
                           libc::c_int == 100 as libc::c_int {
                        (*this).csState2 = 20 as libc::c_int as s16;
                        (*this).work[CS_TIMER_1 as libc::c_int as usize] =
                            0 as libc::c_int as s16;
                        (*this).workf[UNK_F11 as libc::c_int as usize] =
                            600.0f32;
                        (*this).subCamEye.x = 800.0f32;
                        (*this).subCamEye.y = 300.0f32;
                        (*this).subCamEye.z = 0 as libc::c_int as f32_0;
                        (*this).subCamAt.x = 0.0f32;
                        (*this).subCamAt.y = 400.0f32;
                        (*this).subCamAt.z = 0 as libc::c_int as f32_0;
                        (*this).workf[UNK_F9 as libc::c_int as usize] =
                            -3.14159265358979323846f32 / 2.0f32;
                        (*this).workf[UNK_F10 as libc::c_int as usize] =
                            0.0f32;
                        (*this).subCamEyeStep.x = 0.0f32;
                        (*this).spawnPortalAlpha = 0.0f32
                    }
                } else {
                    (*sKotakePtr).actor.shape.rot.y =
                        ((*sKotakePtr).actor.shape.rot.y as libc::c_int +
                             (*this).subCamYawStep as s16 as libc::c_int) as
                            s16
                }
            } else {
                if (*this).work[CS_TIMER_1 as libc::c_int as usize] as
                       libc::c_int % 8 as libc::c_int == 0 as libc::c_int {
                    Audio_PlayActorSound2(&mut (*sKotakePtr).actor,
                                          0x3921 as libc::c_int as u16_0);
                }
                (*sKotakePtr).actor.shape.rot.y =
                    ((*sKotakePtr).actor.shape.rot.y as libc::c_int +
                         (*this).subCamYawStep as s16 as libc::c_int) as s16;
                Math_ApproachF(&mut (*this).subCamYawStep, 12288.0f32, 1.0f32,
                               384.0f32);
                if Animation_OnFrame(&mut (*sKotakePtr).skelAnime,
                                     (*this).workf[ANIM_SW_TGT as libc::c_int
                                                       as usize]) != 0 {
                    Animation_MorphToLoop(&mut (*sKotakePtr).skelAnime,
                                          &mut object_tw_Anim_006F28, 0.0f32);
                    (*this).workf[ANIM_SW_TGT as libc::c_int as usize] =
                        1000.0f32
                }
            }
        }
        20 => {
            if (*this).work[CS_TIMER_1 as libc::c_int as usize] as libc::c_int
                   > 20 as libc::c_int &&
                   ((*this).work[CS_TIMER_1 as libc::c_int as usize] as
                        libc::c_int) < 120 as libc::c_int {
                (*globalCtx).envCtx.unk_BD = 1 as libc::c_int as u8_0;
                Math_ApproachF(&mut (*globalCtx).envCtx.unk_D8, 1.0f32,
                               1.0f32, 0.015f32);
            }
            if (*this).work[CS_TIMER_1 as libc::c_int as usize] as libc::c_int
                   == 90 as libc::c_int {
                Audio_QueueSeqCmd(((0x1 as libc::c_int) << 28 as libc::c_int |
                                       (SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                                           24 as libc::c_int |
                                       0x5a00ff as libc::c_int) as u32_0);
            }
            if (*this).work[CS_TIMER_1 as libc::c_int as usize] as libc::c_int
                   == 120 as libc::c_int {
                sEnvType = 0 as libc::c_int as s8;
                (*globalCtx).envCtx.unk_BE = 1 as libc::c_int as u8_0;
                (*globalCtx).envCtx.unk_BD = 1 as libc::c_int as u8_0;
                (*globalCtx).envCtx.unk_D8 = 0.0f32;
                TitleCard_InitBossName(globalCtx,
                                       &mut (*globalCtx).actorCtx.titleCtx,
                                       gSegments[((object_tw_Blob_02E170.as_mut_ptr()
                                                       as u32_0) <<
                                                      4 as libc::c_int >>
                                                      28 as libc::c_int) as
                                                     usize].wrapping_add(object_tw_Blob_02E170.as_mut_ptr()
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
                gSaveContext.eventChkInf[7 as libc::c_int as usize] =
                    (gSaveContext.eventChkInf[7 as libc::c_int as usize] as
                         libc::c_int | 0x20 as libc::c_int) as u16_0;
                Audio_QueueSeqCmd(((SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                                       24 as libc::c_int |
                                       0x1b as libc::c_int) as u32_0);
            }
            if (*this).work[CS_TIMER_1 as libc::c_int as usize] as libc::c_int
                   >= 160 as libc::c_int {
                if (*this).work[CS_TIMER_1 as libc::c_int as usize] as
                       libc::c_int == 160 as libc::c_int {
                    (*this).subCamEyeStep.x = 0.0f32
                }
                Math_ApproachF(&mut (*this).subCamEye.x, 0.0f32, 0.05f32,
                               (*this).subCamEyeStep.x * 0.5f32);
                Math_ApproachF(&mut (*this).subCamEye.z, 1000.0f32, 0.05f32,
                               (*this).subCamEyeStep.x);
                Math_ApproachF(&mut (*this).subCamEyeStep.x, 40.0f32, 1.0f32,
                               1 as libc::c_int as f32_0);
            } else {
                Math_ApproachF(&mut (*this).subCamEye.x, 300.0f32, 0.05f32,
                               (*this).subCamEyeStep.x);
                Math_ApproachF(&mut (*this).subCamEyeStep.x, 5.0f32, 1.0f32,
                               0.5f32);
            }
            if ((*this).work[CS_TIMER_1 as libc::c_int as usize] as
                    libc::c_int) < 200 as libc::c_int {
                Audio_PlayActorSound2(&mut (*sKoumePtr).actor,
                                      (0x391f as libc::c_int -
                                           0x800 as libc::c_int) as u16_0);
                Audio_PlayActorSound2(&mut (*sKotakePtr).actor,
                                      (0x391f as libc::c_int -
                                           0x800 as libc::c_int) as u16_0);
                sp90.x = (*this).workf[UNK_F11 as libc::c_int as usize];
                sp90.y = 400.0f32;
                sp90.z = 0.0f32;
                Matrix_RotateY((*this).workf[UNK_F9 as libc::c_int as usize],
                               MTXMODE_NEW as libc::c_int as u8_0);
                Matrix_MultVec3f(&mut sp90, &mut sp84);
                (*sKoumePtr).actor.world.pos.x = sp84.x;
                (*sKoumePtr).actor.world.pos.y = sp84.y;
                (*sKoumePtr).actor.world.pos.z = sp84.z;
                (*sKoumePtr).actor.shape.rot.y =
                    ((*this).workf[UNK_F9 as libc::c_int as usize] /
                         3.14159265358979323846f32 * 32768.0f32) as s16;
                (*sKoumePtr).actor.world.rot.y =
                    (*sKoumePtr).actor.shape.rot.y;
                (*sKotakePtr).actor.world.pos.x = -sp84.x;
                (*sKotakePtr).actor.world.pos.y = sp84.y;
                (*sKotakePtr).actor.world.pos.z = -sp84.z;
                (*sKotakePtr).actor.world.rot.y =
                    ((*this).workf[UNK_F9 as libc::c_int as usize] /
                         3.14159265358979323846f32 * 32768.0f32 + 32768.0f32)
                        as s16;
                (*sKotakePtr).actor.shape.rot.y =
                    (*sKotakePtr).actor.world.rot.y;
                Math_ApproachF(&mut *(*this).workf.as_mut_ptr().offset(UNK_F11
                                                                           as
                                                                           libc::c_int
                                                                           as
                                                                           isize),
                               80.0f32, 0.1f32, 5.0f32);
                (*this).workf[UNK_F9 as libc::c_int as usize] -=
                    (*this).workf[UNK_F10 as libc::c_int as usize];
                Math_ApproachF(&mut *(*this).workf.as_mut_ptr().offset(UNK_F10
                                                                           as
                                                                           libc::c_int
                                                                           as
                                                                           isize),
                               0.19999999f32, 1.0f32, 0.0019999994f32);
            }
            if (*this).work[CS_TIMER_1 as libc::c_int as usize] as libc::c_int
                   == 200 as libc::c_int {
                (*sKoumePtr).actionFunc =
                    Some(BossTw_FlyTo as
                             unsafe extern "C" fn(_: *mut BossTw,
                                                  _: *mut GlobalContext)
                                 -> ());
                (*sKotakePtr).actionFunc =
                    Some(BossTw_FlyTo as
                             unsafe extern "C" fn(_: *mut BossTw,
                                                  _: *mut GlobalContext)
                                 -> ());
                (*sKoumePtr).targetPos.x = 600.0f32;
                (*sKoumePtr).targetPos.y = 400.0f32;
                (*sKoumePtr).targetPos.z = 0.0f32;
                (*sKoumePtr).timers[0 as libc::c_int as usize] =
                    100 as libc::c_int as s16;
                (*sKotakePtr).targetPos.x = -600.0f32;
                (*sKotakePtr).targetPos.y = 400.0f32;
                (*sKotakePtr).targetPos.z = 0.0f32;
                (*sKotakePtr).timers[0 as libc::c_int as usize] =
                    100 as libc::c_int as s16
            }
            if (*this).work[CS_TIMER_1 as libc::c_int as usize] as libc::c_int
                   == 260 as libc::c_int {
                let mut cam: *mut Camera =
                    Gameplay_GetCamera(globalCtx, 0 as libc::c_int as s16);
                (*cam).eye = (*this).subCamEye;
                (*cam).eyeNext = (*this).subCamEye;
                (*cam).at = (*this).subCamAt;
                func_800C08AC(globalCtx, (*this).subCamId,
                              0 as libc::c_int as s16);
                (*this).subCamId = 0 as libc::c_int as s16;
                (*this).csState2 = (*this).subCamId;
                func_80064534(globalCtx, &mut (*globalCtx).csCtx);
                func_8002DF54(globalCtx, &mut (*this).actor,
                              7 as libc::c_int as u8_0);
                BossTw_SetupWait(this, globalCtx);
            }
        }
        _ => { }
    }
    if (*this).subCamId as libc::c_int != 0 as libc::c_int {
        if updateCam != 0 {
            Math_ApproachF(&mut (*this).subCamEye.x,
                           (*this).subCamEyeTarget.x, (*this).subCamDistStep,
                           (*this).subCamEyeStep.x *
                               (*this).subCamUpdateRate);
            Math_ApproachF(&mut (*this).subCamEye.y,
                           (*this).subCamEyeTarget.y, (*this).subCamDistStep,
                           (*this).subCamEyeStep.y *
                               (*this).subCamUpdateRate);
            Math_ApproachF(&mut (*this).subCamEye.z,
                           (*this).subCamEyeTarget.z, (*this).subCamDistStep,
                           (*this).subCamEyeStep.z *
                               (*this).subCamUpdateRate);
            Math_ApproachF(&mut (*this).subCamAt.x, (*this).subCamAtTarget.x,
                           (*this).subCamDistStep,
                           (*this).subCamAtStep.x * (*this).subCamUpdateRate);
            Math_ApproachF(&mut (*this).subCamAt.y, (*this).subCamAtTarget.y,
                           (*this).subCamDistStep,
                           (*this).subCamAtStep.y * (*this).subCamUpdateRate);
            Math_ApproachF(&mut (*this).subCamAt.z, (*this).subCamAtTarget.z,
                           (*this).subCamDistStep,
                           (*this).subCamAtStep.z * (*this).subCamUpdateRate);
        }
        Gameplay_CameraSetAtEye(globalCtx, (*this).subCamId,
                                &mut (*this).subCamAt,
                                &mut (*this).subCamEye);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_DeathBall(mut this: *mut BossTw,
                                          mut globalCtx: *mut GlobalContext) {
    let mut xDiff: f32_0 = 0.;
    let mut yDiff: f32_0 = 0.;
    let mut zDiff: f32_0 = 0.;
    let mut pad: s32 = 0;
    let mut i: s16 = 0;
    let mut yaw: s16 = 0;
    if (*this).work[CS_TIMER_1 as libc::c_int as usize] as libc::c_int %
           16 as libc::c_int == 0 as libc::c_int {
        Audio_PlayActorSound2(&mut (*this).actor,
                              0x391e as libc::c_int as u16_0);
    }
    if ((*sTwinrovaPtr).csState2 as libc::c_int) < 2 as libc::c_int {
        if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
               0 as libc::c_int {
            (*this).timers[0 as libc::c_int as usize] =
                20 as libc::c_int as s16;
            (*this).targetPos.x =
                Rand_CenteredFloat(100.0f32) +
                    (*sTwinrovaPtr).actor.world.pos.x;
            (*this).targetPos.y = Rand_CenteredFloat(50.0f32) + 400.0f32;
            (*this).targetPos.z =
                Rand_CenteredFloat(100.0f32) +
                    (*sTwinrovaPtr).actor.world.pos.z
        }
        (*this).timers[1 as libc::c_int as usize] = 10 as libc::c_int as s16;
        (*this).rotateSpeed = 8192.0f32;
        (*this).actor.speedXZ = 5.0f32
    } else {
        if (*this).timers[1 as libc::c_int as usize] as libc::c_int ==
               9 as libc::c_int {
            (*this).targetPos.y = 413.0f32;
            (*this).actor.world.pos.z = 0.0f32;
            (*this).actor.world.pos.x = 0.0f32;
            i = 0 as libc::c_int as s16;
            while (i as libc::c_int) <
                      (::std::mem::size_of::<[Vec3f; 50]>() as
                           libc::c_ulong).wrapping_div(::std::mem::size_of::<Vec3f>()
                                                           as libc::c_ulong)
                          as s32 {
                (*this).blastTailPos[i as usize] = (*this).actor.world.pos;
                i += 1
            }
        }
        if (*this).actor.params as libc::c_int == 0x69 as libc::c_int {
            (*this).targetPos.x = (*sKoumePtr).actor.world.pos.x;
            (*this).targetPos.z = (*sKoumePtr).actor.world.pos.z
        } else {
            (*this).targetPos.x = (*sKotakePtr).actor.world.pos.x;
            (*this).targetPos.z = (*sKotakePtr).actor.world.pos.z
        }
        Math_ApproachF(&mut (*this).targetPos.y, 263.0f32, 1.0f32, 2.0f32);
        if (*this).targetPos.y == 263.0f32 {
            Math_ApproachF(&mut (*this).actor.speedXZ, 0.0f32, 1.0f32,
                           0.2f32);
            if (*sTwinrovaPtr).csState2 as libc::c_int == 3 as libc::c_int {
                Actor_Kill(&mut (*this).actor);
            }
        }
    }
    xDiff = (*this).targetPos.x - (*this).actor.world.pos.x;
    yDiff = (*this).targetPos.y - (*this).actor.world.pos.y;
    zDiff = (*this).targetPos.z - (*this).actor.world.pos.z;
    yaw =
        (Math_FAtan2F(xDiff, zDiff) *
             (32768 as libc::c_int as libc::c_float /
                  3.14159265358979323846f32)) as s16;
    Math_ApproachS(&mut (*this).actor.world.rot.x,
                   (Math_FAtan2F(yDiff, sqrtf(xDiff * xDiff + zDiff * zDiff))
                        *
                        (32768 as libc::c_int as libc::c_float /
                             3.14159265358979323846f32)) as s16,
                   5 as libc::c_int as s16, (*this).rotateSpeed as s16);
    Math_ApproachS(&mut (*this).actor.world.rot.y, yaw,
                   5 as libc::c_int as s16, (*this).rotateSpeed as s16);
    func_8002D908(&mut (*this).actor);
    func_8002D7EC(&mut (*this).actor);
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_TwinrovaSetupDeathCS(mut this: *mut BossTw,
                                                     mut globalCtx:
                                                         *mut GlobalContext) {
    (*this).actionFunc =
        Some(BossTw_TwinrovaDeathCS as
                 unsafe extern "C" fn(_: *mut BossTw, _: *mut GlobalContext)
                     -> ());
    Animation_MorphToLoop(&mut (*this).skelAnime, &mut object_tw_Anim_024374,
                          -3.0f32);
    (*this).actor.world.rot.y = (*this).actor.shape.rot.y;
    (*this).actor.flags &=
        !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
    (*this).csState1 = 0 as libc::c_int as s16;
    (*this).csState2 = (*this).csState1;
    (*this).work[CS_TIMER_2 as libc::c_int as usize] =
        0 as libc::c_int as s16;
    (*this).work[CS_TIMER_1 as libc::c_int as usize] =
        (*this).work[CS_TIMER_2 as libc::c_int as usize];
    (*this).work[INVINC_TIMER as libc::c_int as usize] =
        10000 as libc::c_int as s16;
    BossTw_SetupDeathCS(sKoumePtr, globalCtx);
    BossTw_SetupDeathCS(sKotakePtr, globalCtx);
    (*sKotakePtr).timers[0 as libc::c_int as usize] = 8 as libc::c_int as s16;
    (*this).workf[UNK_F19 as libc::c_int as usize] = 1.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_DeathCSMsgSfx(mut this: *mut BossTw,
                                              mut globalCtx:
                                                  *mut GlobalContext) {
    let mut pad: s32 = 0;
    let mut pad2: s32 = 0;
    let mut pad3: s32 = 0;
    let mut msgId2: s16 = 0;
    let mut msgId1: s16 = 0;
    let mut kotakeAnim: u8_0 = 0;
    let mut koumeAnim: u8_0 = 0;
    let mut sp35: u8_0 = 0;
    msgId2 = 0 as libc::c_int as s16;
    msgId1 = 0 as libc::c_int as s16;
    kotakeAnim = 0 as libc::c_int as u8_0;
    koumeAnim = 0 as libc::c_int as u8_0;
    sp35 = 0 as libc::c_int as u8_0;
    if (*this).work[CS_TIMER_2 as libc::c_int as usize] as libc::c_int ==
           80 as libc::c_int {
        koumeAnim = 1 as libc::c_int as u8_0
    }
    if (*this).work[CS_TIMER_2 as libc::c_int as usize] as libc::c_int ==
           80 as libc::c_int {
        msgId2 = 0x604b as libc::c_int as s16;
        sp35 = 50 as libc::c_int as u8_0
    }
    if (*this).work[CS_TIMER_2 as libc::c_int as usize] as libc::c_int ==
           140 as libc::c_int {
        koumeAnim = 2 as libc::c_int as u8_0;
        kotakeAnim = koumeAnim
    }
    if (*this).work[CS_TIMER_2 as libc::c_int as usize] as libc::c_int ==
           170 as libc::c_int {
        kotakeAnim = 3 as libc::c_int as u8_0;
        (*sKotakePtr).work[YAW_TGT as libc::c_int as usize] =
            -(0x4000 as libc::c_int) as s16;
        (*sKotakePtr).rotateSpeed = 0.0f32;
        Audio_PlayActorSound2(&mut (*sKotakePtr).actor,
                              0x39b4 as libc::c_int as u16_0);
        msgId2 = 0x604c as libc::c_int as s16
    }
    if (*this).work[CS_TIMER_2 as libc::c_int as usize] as libc::c_int ==
           210 as libc::c_int {
        D_8094C874 = 30 as libc::c_int as s16
    }
    if (*this).work[CS_TIMER_2 as libc::c_int as usize] as libc::c_int ==
           270 as libc::c_int {
        koumeAnim = 3 as libc::c_int as u8_0;
        (*sKoumePtr).work[YAW_TGT as libc::c_int as usize] =
            0x4000 as libc::c_int as s16;
        (*sKoumePtr).rotateSpeed = 0.0f32;
        Audio_PlayActorSound2(&mut (*sKoumePtr).actor,
                              0x39b4 as libc::c_int as u16_0);
    }
    if (*this).work[CS_TIMER_2 as libc::c_int as usize] as libc::c_int ==
           290 as libc::c_int {
        msgId2 = 0x604d as libc::c_int as s16;
        sp35 = 35 as libc::c_int as u8_0
    }
    if (*this).work[CS_TIMER_2 as libc::c_int as usize] as libc::c_int ==
           350 as libc::c_int {
        kotakeAnim = 2 as libc::c_int as u8_0;
        koumeAnim = kotakeAnim;
        (*sKotakePtr).work[YAW_TGT as libc::c_int as usize] =
            0 as libc::c_int as s16;
        (*sKoumePtr).work[YAW_TGT as libc::c_int as usize] =
            (*sKotakePtr).work[YAW_TGT as libc::c_int as usize];
        (*sKotakePtr).rotateSpeed = 0.0f32;
        (*sKoumePtr).rotateSpeed = (*sKotakePtr).rotateSpeed
    }
    if (*this).work[CS_TIMER_2 as libc::c_int as usize] as libc::c_int ==
           380 as libc::c_int {
        kotakeAnim = 3 as libc::c_int as u8_0;
        koumeAnim = kotakeAnim
    }
    if (*this).work[CS_TIMER_2 as libc::c_int as usize] as libc::c_int ==
           400 as libc::c_int {
        kotakeAnim = 2 as libc::c_int as u8_0;
        koumeAnim = kotakeAnim
    }
    if (*this).work[CS_TIMER_2 as libc::c_int as usize] as libc::c_int ==
           430 as libc::c_int {
        koumeAnim = 4 as libc::c_int as u8_0;
        D_8094C874 = 435 as libc::c_int as s16;
        D_8094C878 = 1 as libc::c_int as u8_0
    }
    if (*this).work[CS_TIMER_2 as libc::c_int as usize] as libc::c_int >
           440 as libc::c_int &&
           ((*this).work[CS_TIMER_2 as libc::c_int as usize] as libc::c_int) <
               860 as libc::c_int {
        func_80078884((0x39e7 as libc::c_int - 0x800 as libc::c_int) as
                          u16_0);
    }
    if (*this).work[CS_TIMER_2 as libc::c_int as usize] as libc::c_int ==
           430 as libc::c_int {
        msgId2 = 0x604e as libc::c_int as s16
    }
    if (*this).work[CS_TIMER_2 as libc::c_int as usize] as libc::c_int ==
           480 as libc::c_int {
        kotakeAnim = 4 as libc::c_int as u8_0;
        (*sKotakePtr).work[YAW_TGT as libc::c_int as usize] =
            -(0x4000 as libc::c_int) as s16
    }
    if (*this).work[CS_TIMER_2 as libc::c_int as usize] as libc::c_int ==
           500 as libc::c_int {
        koumeAnim = 2 as libc::c_int as u8_0
    }
    if (*this).work[CS_TIMER_2 as libc::c_int as usize] as libc::c_int ==
           480 as libc::c_int {
        msgId1 = 0x604f as libc::c_int as s16
    }
    if (*this).work[CS_TIMER_2 as libc::c_int as usize] as libc::c_int ==
           530 as libc::c_int {
        koumeAnim = 4 as libc::c_int as u8_0;
        (*sKoumePtr).work[YAW_TGT as libc::c_int as usize] =
            0x4000 as libc::c_int as s16;
        D_8094C87A = 335 as libc::c_int as s16;
        D_8094C87E = 1 as libc::c_int as u8_0
    }
    if (*this).work[CS_TIMER_2 as libc::c_int as usize] as libc::c_int ==
           530 as libc::c_int {
        msgId2 = 0x6050 as libc::c_int as s16
    }
    if (*this).work[CS_TIMER_2 as libc::c_int as usize] as libc::c_int ==
           580 as libc::c_int {
        msgId1 = 0x6051 as libc::c_int as s16
    }
    if (*this).work[CS_TIMER_2 as libc::c_int as usize] as libc::c_int ==
           620 as libc::c_int {
        msgId2 = 0x6052 as libc::c_int as s16
    }
    if (*this).work[CS_TIMER_2 as libc::c_int as usize] as libc::c_int ==
           660 as libc::c_int {
        msgId1 = 0x6053 as libc::c_int as s16
    }
    if (*this).work[CS_TIMER_2 as libc::c_int as usize] as libc::c_int ==
           700 as libc::c_int {
        msgId2 = 0x6054 as libc::c_int as s16
    }
    if (*this).work[CS_TIMER_2 as libc::c_int as usize] as libc::c_int ==
           740 as libc::c_int {
        msgId1 = 0x6055 as libc::c_int as s16
    }
    if (*this).work[CS_TIMER_2 as libc::c_int as usize] as libc::c_int ==
           780 as libc::c_int {
        msgId2 = 0x6056 as libc::c_int as s16
    }
    if (*this).work[CS_TIMER_2 as libc::c_int as usize] as libc::c_int ==
           820 as libc::c_int {
        msgId1 = 0x6057 as libc::c_int as s16;
        Audio_QueueSeqCmd(((0x1 as libc::c_int) << 28 as libc::c_int |
                               (SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                                   24 as libc::c_int |
                               0x5000ff as libc::c_int) as u32_0);
    }
    if (*this).work[CS_TIMER_2 as libc::c_int as usize] as libc::c_int ==
           860 as libc::c_int {
        kotakeAnim = 3 as libc::c_int as u8_0;
        koumeAnim = kotakeAnim
    }
    if (*this).work[CS_TIMER_2 as libc::c_int as usize] as libc::c_int ==
           900 as libc::c_int {
        Audio_PlayActorSound2(&mut (*sKoumePtr).actor,
                              0x39b5 as libc::c_int as u16_0);
        Audio_PlayActorSound2(&mut (*sKotakePtr).actor,
                              0x39b5 as libc::c_int as u16_0);
    }
    if (*this).work[CS_TIMER_2 as libc::c_int as usize] as libc::c_int ==
           930 as libc::c_int {
        msgId2 = 0x6058 as libc::c_int as s16
    }
    if msgId2 as libc::c_int != 0 as libc::c_int {
        Message_StartTextbox(globalCtx, msgId2 as u16_0, 0 as *mut Actor);
        if sp35 != 0 {
            D_8094C876 = 10 as libc::c_int as s16;
            D_8094C874 = sp35 as s16;
            D_8094C878 = 0 as libc::c_int as u8_0
        }
    }
    if msgId1 as libc::c_int != 0 as libc::c_int {
        Message_StartTextbox(globalCtx, msgId1 as u16_0, 0 as *mut Actor);
    }
    match kotakeAnim as libc::c_int {
        1 => {
            Animation_MorphToLoop(&mut (*sKotakePtr).skelAnime,
                                  &mut object_tw_Anim_00230C, -5.0f32);
        }
        2 => {
            Animation_MorphToLoop(&mut (*sKotakePtr).skelAnime,
                                  &mut object_tw_Anim_001D10, -5.0f32);
        }
        3 => {
            Animation_MorphToLoop(&mut (*sKotakePtr).skelAnime,
                                  &mut object_tw_Anim_0017E0, -5.0f32);
        }
        4 => {
            Animation_MorphToLoop(&mut (*sKotakePtr).skelAnime,
                                  &mut object_tw_Anim_0012A4, -5.0f32);
        }
        _ => { }
    }
    match koumeAnim as libc::c_int {
        1 => {
            Animation_MorphToLoop(&mut (*sKoumePtr).skelAnime,
                                  &mut object_tw_Anim_00230C, -5.0f32);
        }
        2 => {
            Animation_MorphToLoop(&mut (*sKoumePtr).skelAnime,
                                  &mut object_tw_Anim_001D10, -5.0f32);
        }
        3 => {
            Animation_MorphToLoop(&mut (*sKoumePtr).skelAnime,
                                  &mut object_tw_Anim_0017E0, -5.0f32);
        }
        4 => {
            Animation_MorphToLoop(&mut (*sKoumePtr).skelAnime,
                                  &mut object_tw_Anim_0012A4, -5.0f32);
        }
        _ => { }
    }
    if (*this).work[CS_TIMER_2 as libc::c_int as usize] as libc::c_int >=
           120 as libc::c_int &&
           ((*this).work[CS_TIMER_2 as libc::c_int as usize] as libc::c_int) <
               500 as libc::c_int {
        Math_ApproachF(&mut *(*this).workf.as_mut_ptr().offset(UNK_F18 as
                                                                   libc::c_int
                                                                   as isize),
                       255.0f32, 0.1f32, 5.0f32);
    }
    if (*this).work[CS_TIMER_2 as libc::c_int as usize] as libc::c_int >=
           150 as libc::c_int {
        Math_ApproachF(&mut *(*sKoumePtr).workf.as_mut_ptr().offset(UNK_F17 as
                                                                        libc::c_int
                                                                        as
                                                                        isize),
                       Math_SinS(((*this).work[CS_TIMER_1 as libc::c_int as
                                                   usize] as libc::c_int *
                                      2000 as libc::c_int) as s16) * 0.05f32 +
                           0.4f32, 0.1f32, 0.01f32);
        Math_ApproachF(&mut *(*sKotakePtr).workf.as_mut_ptr().offset(UNK_F17
                                                                         as
                                                                         libc::c_int
                                                                         as
                                                                         isize),
                       Math_CosS(((*this).work[CS_TIMER_1 as libc::c_int as
                                                   usize] as libc::c_int *
                                      1700 as libc::c_int) as s16) * 0.05f32 +
                           0.4f32, 0.1f32, 0.01f32);
        if (*this).work[CS_TIMER_2 as libc::c_int as usize] as libc::c_int >=
               880 as libc::c_int {
            Math_ApproachF(&mut (*sKotakePtr).actor.world.pos.y, 2000.0f32,
                           1.0f32, (*this).actor.speedXZ);
            Math_ApproachF(&mut (*sKoumePtr).actor.world.pos.y, 2000.0f32,
                           1.0f32, (*this).actor.speedXZ);
            Math_ApproachF(&mut (*this).actor.speedXZ, 10.0f32, 1.0f32,
                           0.25f32);
            if (*this).work[CS_TIMER_2 as libc::c_int as usize] as libc::c_int
                   >= 930 as libc::c_int {
                Math_ApproachF(&mut *(*this).workf.as_mut_ptr().offset(UNK_F19
                                                                           as
                                                                           libc::c_int
                                                                           as
                                                                           isize),
                               5.0f32, 1.0f32, 0.05f32);
                Math_ApproachF(&mut *(*this).workf.as_mut_ptr().offset(UNK_F18
                                                                           as
                                                                           libc::c_int
                                                                           as
                                                                           isize),
                               0.0f32, 1.0f32, 3.0f32);
            }
            Audio_PlayActorSound2(&mut (*this).actor,
                                  (0x2886 as libc::c_int -
                                       0x800 as libc::c_int) as u16_0);
        } else {
            let mut yTarget: f32_0 =
                Math_CosS(((*this).work[CS_TIMER_2 as libc::c_int as usize] as
                               libc::c_int * 1700 as libc::c_int) as s16) *
                    4.0f32;
            Math_ApproachF(&mut (*sKotakePtr).actor.world.pos.y,
                           20.0f32 + (263.0f32 + yTarget), 0.1f32,
                           (*this).actor.speedXZ);
            yTarget =
                Math_SinS(((*this).work[CS_TIMER_2 as libc::c_int as usize] as
                               libc::c_int * 1500 as libc::c_int) as s16) *
                    4.0f32;
            Math_ApproachF(&mut (*sKoumePtr).actor.world.pos.y,
                           20.0f32 + (263.0f32 + yTarget), 0.1f32,
                           (*this).actor.speedXZ);
            Math_ApproachF(&mut (*this).actor.speedXZ, 1.0f32, 1.0f32,
                           0.05f32);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_TwinrovaDeathCS(mut this: *mut BossTw,
                                                mut globalCtx:
                                                    *mut GlobalContext) {
    let mut i: s16 = 0;
    let mut spD0: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut mainCam: *mut Camera =
        Gameplay_GetCamera(globalCtx, 0 as libc::c_int as s16);
    SkelAnime_Update(&mut (*this).skelAnime);
    (*this).work[UNK_S8 as libc::c_int as usize] =
        ((*this).work[UNK_S8 as libc::c_int as usize] as libc::c_int +
             20 as libc::c_int) as s16;
    if (*this).work[UNK_S8 as libc::c_int as usize] as libc::c_int >
           255 as libc::c_int {
        (*this).work[UNK_S8 as libc::c_int as usize] =
            255 as libc::c_int as s16
    }
    Math_ApproachF(&mut *(*this).workf.as_mut_ptr().offset(UNK_F12 as
                                                               libc::c_int as
                                                               isize), 0.0f32,
                   1.0f32, 0.05f32);
    (*this).unk_5F8 = 1 as libc::c_int as u8_0;
    match (*this).csState1 as libc::c_int {
        0 => {
            if (*this).work[CS_TIMER_1 as libc::c_int as usize] as libc::c_int
                   == 15 as libc::c_int {
                Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                                          &mut object_tw_Anim_0216DC,
                                          -3.0f32);
            }
            if (*this).work[CS_TIMER_1 as libc::c_int as usize] as libc::c_int
                   >= 15 as libc::c_int {
                Math_ApproachF(&mut (*this).actor.world.pos.y, 400.0f32,
                               0.05f32, 10.0f32);
            }
            if (*this).work[CS_TIMER_1 as libc::c_int as usize] as libc::c_int
                   >= 55 as libc::c_int {
                if (*this).work[CS_TIMER_1 as libc::c_int as usize] as
                       libc::c_int == 55 as libc::c_int {
                    (*globalCtx).envCtx.unk_D8 = 0 as libc::c_int as f32_0
                }
                sEnvType = -(1 as libc::c_int) as s8;
                (*globalCtx).envCtx.unk_BE = 5 as libc::c_int as u8_0;
                (*globalCtx).envCtx.unk_BD = 0 as libc::c_int as u8_0;
                Math_ApproachF(&mut (*globalCtx).envCtx.unk_D8, 1.0f32,
                               1.0f32, 0.015f32);
                Math_ApproachF(&mut (*this).actor.scale.x, 0.00024999998f32,
                               0.1f32, 0.00005f32);
                (*this).actor.shape.rot.y =
                    ((*this).actor.shape.rot.y as libc::c_int +
                         (*this).actor.speedXZ as s16 as libc::c_int) as s16;
                (*this).workf[UNK_F13 as libc::c_int as usize] +=
                    (*this).actor.speedXZ;
                if (*this).workf[UNK_F13 as libc::c_int as usize] > 65536.0f32
                   {
                    (*this).workf[UNK_F13 as libc::c_int as usize] -=
                        65536.0f32;
                    Audio_PlayActorSound2(&mut (*this).actor,
                                          0x3921 as libc::c_int as u16_0);
                }
                Math_ApproachF(&mut (*this).actor.speedXZ, 12288.0f32, 1.0f32,
                               256.0f32);
                if (*this).work[CS_TIMER_1 as libc::c_int as usize] as
                       libc::c_int == 135 as libc::c_int {
                    let mut spBC: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                    let mut spB0: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                    let mut spA4: Vec3f =
                        {
                            let mut init =
                                Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,};
                            init
                        };
                    func_80078884(0x390d as libc::c_int as u16_0);
                    i = 0 as libc::c_int as s16;
                    while (i as libc::c_int) < 100 as libc::c_int {
                        spB0.x = Rand_CenteredFloat(5.0f32);
                        spB0.y = Rand_CenteredFloat(5.0f32);
                        spB0.z = Rand_CenteredFloat(5.0f32);
                        spBC = (*this).actor.world.pos;
                        spBC.x += spB0.x;
                        spBC.y += spB0.y;
                        spBC.z += spB0.z;
                        BossTw_AddFlameEffect(globalCtx, &mut spBC, &mut spB0,
                                              &mut spA4,
                                              Rand_ZeroFloat(2.0f32) +
                                                  5 as libc::c_int as
                                                      libc::c_float,
                                              Rand_ZeroFloat(1.99f32) as s16);
                        i += 1
                    }
                    (*this).csState1 = 1 as libc::c_int as s16;
                    (*this).visible = 0 as libc::c_int as u8_0;
                    (*this).actor.scale.x = 0.0f32;
                    Actor_SpawnAsChild(&mut (*globalCtx).actorCtx,
                                       &mut (*this).actor, globalCtx,
                                       ACTOR_BOSS_TW as libc::c_int as s16,
                                       (*this).actor.world.pos.x,
                                       (*this).actor.world.pos.y,
                                       (*this).actor.world.pos.z,
                                       0 as libc::c_int as s16,
                                       0 as libc::c_int as s16,
                                       0 as libc::c_int as s16,
                                       TW_DEATHBALL_KOUME as libc::c_int as
                                           s16);
                    Actor_SpawnAsChild(&mut (*globalCtx).actorCtx,
                                       &mut (*this).actor, globalCtx,
                                       ACTOR_BOSS_TW as libc::c_int as s16,
                                       (*this).actor.world.pos.x,
                                       (*this).actor.world.pos.y,
                                       (*this).actor.world.pos.z,
                                       0 as libc::c_int as s16,
                                       0 as libc::c_int as s16,
                                       0 as libc::c_int as s16,
                                       TW_DEATHBALL_KOTAKE as libc::c_int as
                                           s16);
                    (*this).actor.flags &=
                        !((1 as libc::c_int) << 0 as libc::c_int) as
                            libc::c_uint
                }
            }
            Actor_SetScale(&mut (*this).actor, (*this).actor.scale.x);
        }
        1 | _ => { }
    }
    match (*this).csState2 as libc::c_int {
        0 => {
            (*this).csState2 = 1 as libc::c_int as s16;
            func_80064520(globalCtx, &mut (*globalCtx).csCtx);
            func_8002DF54(globalCtx, &mut (*this).actor,
                          8 as libc::c_int as u8_0);
            (*this).subCamId = Gameplay_CreateSubCamera(globalCtx);
            Gameplay_ChangeCameraStatus(globalCtx, 0 as libc::c_int as s16,
                                        1 as libc::c_int as s16);
            Gameplay_ChangeCameraStatus(globalCtx, (*this).subCamId,
                                        7 as libc::c_int as s16);
            (*this).subCamEye = (*mainCam).eye;
            (*this).subCamAt = (*mainCam).at;
            Audio_QueueSeqCmd(((0x1 as libc::c_int) << 28 as libc::c_int |
                                   (SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                                       24 as libc::c_int |
                                   0x100ff as libc::c_int) as u32_0);
        }
        1 => {
            spD0.x = Math_SinS((*this).actor.world.rot.y) * 200.0f32;
            spD0.z = Math_CosS((*this).actor.world.rot.y) * 200.0f32;
            Math_ApproachF(&mut (*this).subCamEye.x,
                           spD0.x + (*this).actor.world.pos.x, 0.1f32,
                           50.0f32);
            Math_ApproachF(&mut (*this).subCamEye.y, 300.0f32, 0.1f32,
                           50.0f32);
            Math_ApproachF(&mut (*this).subCamEye.z,
                           spD0.z + (*this).actor.world.pos.z, 0.1f32,
                           50.0f32);
            Math_ApproachF(&mut (*this).subCamAt.x, (*this).actor.world.pos.x,
                           0.1f32, 50.0f32);
            Math_ApproachF(&mut (*this).subCamAt.y, (*this).actor.world.pos.y,
                           0.1f32, 50.0f32);
            Math_ApproachF(&mut (*this).subCamAt.z, (*this).actor.world.pos.z,
                           0.1f32, 50.0f32);
            if (*this).work[CS_TIMER_1 as libc::c_int as usize] as libc::c_int
                   == 170 as libc::c_int {
                (*this).csState2 = 2 as libc::c_int as s16;
                (*this).work[CS_TIMER_2 as libc::c_int as usize] =
                    0 as libc::c_int as s16;
                (*this).subCamEye.z = 170.0f32;
                (*this).subCamDist = 170.0f32;
                (*this).subCamEye.x = 0.0f32;
                (*this).subCamAt.x = 0.0f32;
                (*this).subCamAt.z = 0.0f32;
                (*this).subCamEye.y = 260.0f32;
                (*player).actor.shape.rot.y = -(0x8000 as libc::c_int) as s16;
                (*player).actor.world.pos.x = -40.0f32;
                (*player).actor.world.pos.y = 240.0f32;
                (*player).actor.world.pos.z = 90.0f32;
                (*sKoumePtr).actor.world.pos.x = -37.0f32;
                (*sKotakePtr).actor.world.pos.x = 37.0f32;
                (*sKotakePtr).actor.world.pos.y = 263.0f32;
                (*sKoumePtr).actor.world.pos.y =
                    (*sKotakePtr).actor.world.pos.y;
                (*this).subCamAt.y = (*sKoumePtr).actor.world.pos.y + 17.0f32;
                (*sKotakePtr).actor.world.pos.z = 0.0f32;
                (*sKoumePtr).actor.world.pos.z =
                    (*sKotakePtr).actor.world.pos.z;
                (*sKotakePtr).actor.shape.rot.y = 0 as libc::c_int as s16;
                (*sKoumePtr).actor.shape.rot.y =
                    (*sKotakePtr).actor.shape.rot.y;
                (*sKotakePtr).actor.shape.rot.x =
                    (*sKoumePtr).actor.shape.rot.y;
                (*sKoumePtr).actor.shape.rot.x =
                    (*sKotakePtr).actor.shape.rot.x;
                (*sKotakePtr).work[YAW_TGT as libc::c_int as usize] =
                    (*sKoumePtr).actor.shape.rot.x;
                (*sKoumePtr).work[YAW_TGT as libc::c_int as usize] =
                    (*sKotakePtr).work[YAW_TGT as libc::c_int as usize];
                func_8002DF54(globalCtx, &mut (*sKoumePtr).actor,
                              1 as libc::c_int as u8_0);
                (*sKoumePtr).actor.flags |=
                    ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
            }
        }
        2 => {
            if (*this).work[CS_TIMER_2 as libc::c_int as usize] as libc::c_int
                   == 100 as libc::c_int {
                let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                let mut velocity: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                let mut accel: Vec3f =
                    {
                        let mut init =
                            Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,};
                        init
                    };
                let mut zero: s32 = 0 as libc::c_int;
                i = 0 as libc::c_int as s16;
                while (i as libc::c_int) < 50 as libc::c_int {
                    velocity.x = Rand_CenteredFloat(3.0f32);
                    velocity.y = Rand_CenteredFloat(3.0f32);
                    velocity.z = Rand_CenteredFloat(3.0f32);
                    pos = (*sKoumePtr).actor.world.pos;
                    pos.x += velocity.x * 2.0f32;
                    pos.y += velocity.y * 2.0f32;
                    pos.z += velocity.z * 2.0f32;
                    BossTw_AddFlameEffect(globalCtx, &mut pos, &mut velocity,
                                          &mut accel,
                                          Rand_ZeroFloat(2.0f32) +
                                              5 as libc::c_int as
                                                  libc::c_float,
                                          1 as libc::c_int as s16);
                    // fake code needed to match, tricks the compiler into allocating more stack
                    if zero != 0 {
                        accel.x =
                            (accel.x as libc::c_double * 2.0f64) as f32_0
                    } // fixes regalloc, may be fake
                    velocity.x = Rand_CenteredFloat(3.0f32);
                    velocity.y = Rand_CenteredFloat(3.0f32);
                    velocity.z = Rand_CenteredFloat(3.0f32);
                    pos = (*sKotakePtr).actor.world.pos;
                    pos.x += velocity.x * 2.0f32;
                    pos.y += velocity.y * 2.0f32;
                    pos.z += velocity.z * 2.0f32;
                    BossTw_AddFlameEffect(globalCtx, &mut pos, &mut velocity,
                                          &mut accel,
                                          Rand_ZeroFloat(2.0f32) +
                                              5 as libc::c_int as
                                                  libc::c_float,
                                          0 as libc::c_int as s16);
                    i += 1
                }
                Actor_SetScale(&mut (*sKoumePtr).actor, 0.0f32);
                Actor_SetScale(&mut (*sKotakePtr).actor, 0.0f32);
                (*sKoumePtr).visible = 1 as libc::c_int as u8_0;
                (*sKotakePtr).visible = 1 as libc::c_int as u8_0;
                func_80078884(0x390d as libc::c_int as u16_0);
                Audio_QueueSeqCmd(((SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                                       24 as libc::c_int |
                                       0x61 as libc::c_int) as u32_0);
                (*this).csState2 = 3 as libc::c_int as s16;
                (*this).work[CS_TIMER_2 as libc::c_int as usize] =
                    0 as libc::c_int as s16;
                (*this).subCamDistStep = 0.0f32;
                (*this).actor.speedXZ = (*this).subCamDistStep;
                (*this).subCamYawStep = (*this).actor.speedXZ;
                (*this).subCamYaw = (*this).subCamYawStep
            }
        }
        3 => {
            BossTw_DeathCSMsgSfx(this, globalCtx);
            if ((*this).work[CS_TIMER_2 as libc::c_int as usize] as
                    libc::c_int) < 150 as libc::c_int {
                (*globalCtx).envCtx.unk_BE = 1 as libc::c_int as u8_0;
                (*globalCtx).envCtx.unk_BD = 0 as libc::c_int as u8_0;
                Math_ApproachZeroF(&mut (*globalCtx).envCtx.unk_D8, 1.0f32,
                                   0.1f32);
            } else {
                (*globalCtx).envCtx.unk_BE = 1 as libc::c_int as u8_0;
                (*globalCtx).envCtx.unk_BD = 6 as libc::c_int as u8_0;
                Math_ApproachF(&mut (*globalCtx).envCtx.unk_D8,
                               Math_SinS(((*this).work[CS_TIMER_2 as
                                                           libc::c_int as
                                                           usize] as
                                              libc::c_int *
                                              4096 as libc::c_int) as s16) /
                                   4.0f32 + 0.75f32, 1.0f32, 0.1f32);
            }
            Math_ApproachF(&mut (*this).subCamAt.y,
                           (*sKoumePtr).actor.world.pos.y + 17.0f32, 0.05f32,
                           10.0f32);
            if (*this).work[CS_TIMER_2 as libc::c_int as usize] as libc::c_int
                   >= 50 as libc::c_int {
                Math_ApproachF(&mut (*this).subCamDist, 110.0f32, 0.05f32,
                               (*this).subCamDistStep);
                Math_ApproachF(&mut (*this).subCamDistStep, 1.0f32, 1.0f32,
                               0.025f32);
                (*this).subCamEye.x =
                    (*this).subCamDist * sinf((*this).subCamYaw);
                (*this).subCamEye.z =
                    (*this).subCamDist * cosf((*this).subCamYaw);
                if (*this).work[CS_TIMER_2 as libc::c_int as usize] as
                       libc::c_int >= 151 as libc::c_int {
                    (*this).subCamYaw += (*this).subCamYawStep;
                    if (*this).work[CS_TIMER_2 as libc::c_int as usize] as
                           libc::c_int >= 800 as libc::c_int {
                        Math_ApproachF(&mut (*this).subCamYawStep, 0.0f32,
                                       1.0f32, 0.0001f32);
                    } else {
                        Math_ApproachF(&mut (*this).subCamYawStep, 0.015f32,
                                       1.0f32, 0.0001f32);
                    }
                }
            }
            Math_ApproachF(&mut (*sKoumePtr).actor.scale.x, 0.009999999f32,
                           0.1f32, 0.001f32);
            Actor_SetScale(&mut (*sKoumePtr).actor,
                           (*sKoumePtr).actor.scale.x);
            Actor_SetScale(&mut (*sKotakePtr).actor,
                           (*sKoumePtr).actor.scale.x);
            if (*this).work[CS_TIMER_2 as libc::c_int as usize] as libc::c_int
                   >= 1020 as libc::c_int {
                mainCam =
                    Gameplay_GetCamera(globalCtx, 0 as libc::c_int as s16);
                (*mainCam).eye = (*this).subCamEye;
                (*mainCam).eyeNext = (*this).subCamEye;
                (*mainCam).at = (*this).subCamAt;
                func_800C08AC(globalCtx, (*this).subCamId,
                              0 as libc::c_int as s16);
                (*this).csState2 = 4 as libc::c_int as s16;
                (*this).subCamId = 0 as libc::c_int as s16;
                func_80064534(globalCtx, &mut (*globalCtx).csCtx);
                func_8002DF54(globalCtx, &mut (*this).actor,
                              7 as libc::c_int as u8_0);
                Audio_QueueSeqCmd(((SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                                       24 as libc::c_int |
                                       0x21 as libc::c_int) as u32_0);
                Actor_SpawnAsChild(&mut (*globalCtx).actorCtx,
                                   &mut (*this).actor, globalCtx,
                                   ACTOR_DOOR_WARP1 as libc::c_int as s16,
                                   600.0f32, 230.0f32, 0.0f32,
                                   0 as libc::c_int as s16,
                                   0 as libc::c_int as s16,
                                   0 as libc::c_int as s16,
                                   WARP_DUNGEON_ADULT as libc::c_int as s16);
                Actor_Spawn(&mut (*globalCtx).actorCtx, globalCtx,
                            ACTOR_ITEM_B_HEART as libc::c_int as s16,
                            -600.0f32, 230.0f32, 0.0f32,
                            0 as libc::c_int as s16, 0 as libc::c_int as s16,
                            0 as libc::c_int as s16, 0 as libc::c_int as s16);
                (*this).actor.world.pos.y = -2000.0f32;
                (*this).workf[UNK_F18 as libc::c_int as usize] = 0.0f32;
                (*sKotakePtr).visible = 0 as libc::c_int as u8_0;
                (*sKoumePtr).visible = (*sKotakePtr).visible;
                !(&mut (*this).subCamEye as *mut Vec3f).is_null();
                Flags_SetClear(globalCtx,
                               (*globalCtx).roomCtx.curRoom.num as s32);
            }
        }
        4 => { sEnvType = 0 as libc::c_int as s8 }
        _ => { }
    }
    if (*this).subCamId != 0 {
        Gameplay_CameraSetAtEye(globalCtx, (*this).subCamId,
                                &mut (*this).subCamAt,
                                &mut (*this).subCamEye);
    };
}
static mut D_8094A900: [s16; 5] =
    [0 as libc::c_int as s16, 1 as libc::c_int as s16,
     2 as libc::c_int as s16, 2 as libc::c_int as s16,
     1 as libc::c_int as s16];
static mut D_8094A90C: [s16; 9] =
    [0 as libc::c_int as s16, 1 as libc::c_int as s16,
     2 as libc::c_int as s16, 2 as libc::c_int as s16,
     2 as libc::c_int as s16, 2 as libc::c_int as s16,
     2 as libc::c_int as s16, 2 as libc::c_int as s16,
     1 as libc::c_int as s16];
#[no_mangle]
pub unsafe extern "C" fn BossTw_Update(mut thisx: *mut Actor,
                                       mut globalCtx: *mut GlobalContext) {
    let mut this: *mut BossTw = thisx as *mut BossTw;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut i: s16 = 0;
    let mut pad: s32 = 0;
    (*this).collider.base.colType = COLTYPE_HIT3 as libc::c_int as u8_0;
    Math_ApproachF(&mut (*this).fogR,
                   (*globalCtx).lightCtx.fogColor[0 as libc::c_int as usize]
                       as f32_0, 1.0f32, 10.0f32);
    Math_ApproachF(&mut (*this).fogG,
                   (*globalCtx).lightCtx.fogColor[1 as libc::c_int as usize]
                       as f32_0, 1.0f32, 10.0f32);
    Math_ApproachF(&mut (*this).fogB,
                   (*globalCtx).lightCtx.fogColor[2 as libc::c_int as usize]
                       as f32_0, 1.0f32, 10.0f32);
    Math_ApproachF(&mut (*this).fogNear,
                   (*globalCtx).lightCtx.fogNear as f32_0, 1.0f32, 10.0f32);
    Math_ApproachF(&mut (*this).fogFar, 1000.0f32, 1.0f32, 10.0f32);
    (*this).work[CS_TIMER_1 as libc::c_int as usize] += 1;
    (*this).work[CS_TIMER_2 as libc::c_int as usize] += 1;
    (*this).work[TAIL_IDX as libc::c_int as usize] += 1;
    if (*this).work[TAIL_IDX as libc::c_int as usize] as libc::c_int >=
           (::std::mem::size_of::<[Vec3f; 50]>() as
                libc::c_ulong).wrapping_div(::std::mem::size_of::<Vec3f>() as
                                                libc::c_ulong) as s32 {
        (*this).work[TAIL_IDX as libc::c_int as usize] =
            0 as libc::c_int as s16
    }
    (*this).blastTailPos[(*this).work[TAIL_IDX as libc::c_int as usize] as
                             usize] = (*this).actor.world.pos;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < 5 as libc::c_int {
        if (*this).timers[i as usize] as libc::c_int != 0 as libc::c_int {
            (*this).timers[i as usize] -= 1
        }
        i += 1
    }
    if (*this).work[INVINC_TIMER as libc::c_int as usize] as libc::c_int !=
           0 as libc::c_int {
        (*this).work[INVINC_TIMER as libc::c_int as usize] -= 1
    }
    if (*this).work[FOG_TIMER as libc::c_int as usize] as libc::c_int !=
           0 as libc::c_int {
        (*this).work[FOG_TIMER as libc::c_int as usize] -= 1
    }
    if (*this).actionFunc ==
           Some(BossTw_FlyTo as
                    unsafe extern "C" fn(_: *mut BossTw,
                                         _: *mut GlobalContext) -> ()) ||
           (*this).actionFunc ==
               Some(BossTw_Spin as
                        unsafe extern "C" fn(_: *mut BossTw,
                                             _: *mut GlobalContext) -> ()) ||
           (*this).actionFunc ==
               Some(BossTw_TurnToPlayer as
                        unsafe extern "C" fn(_: *mut BossTw,
                                             _: *mut GlobalContext) -> ()) {
        if (((*player).actor.shape.rot.y as libc::c_int -
                 (*this).actor.yawTowardsPlayer as libc::c_int +
                 0x8000 as libc::c_int) as s16 as libc::c_int) <
               0x1000 as libc::c_int &&
               ((*player).actor.shape.rot.y as libc::c_int -
                    (*this).actor.yawTowardsPlayer as libc::c_int +
                    0x8000 as libc::c_int) as s16 as libc::c_int >
                   -(0x1000 as libc::c_int) &&
               (*player).unk_A73 as libc::c_int != 0 {
            BossTw_SetupSpin(this, globalCtx);
        }
    }
    (*this).actionFunc.expect("non-null function pointer")(this, globalCtx);
    if (*this).actionFunc !=
           Some(BossTw_Wait as
                    unsafe extern "C" fn(_: *mut BossTw,
                                         _: *mut GlobalContext) -> ()) {
        (*this).collider.dim.radius = 45 as libc::c_int as s16;
        if (*this).actionFunc ==
               Some(BossTw_Spin as
                        unsafe extern "C" fn(_: *mut BossTw,
                                             _: *mut GlobalContext) -> ()) {
            (*this).collider.dim.radius =
                ((*this).collider.dim.radius as libc::c_int *
                     2 as libc::c_int) as s16
        }
        (*this).collider.dim.height = 120 as libc::c_int as s16;
        (*this).collider.dim.yShift = -(30 as libc::c_int) as s16;
        if (*this).work[INVINC_TIMER as libc::c_int as usize] as libc::c_int
               == 0 as libc::c_int {
            if (*this).collider.base.acFlags as libc::c_int &
                   (1 as libc::c_int) << 1 as libc::c_int != 0 {
                (*this).collider.base.acFlags =
                    ((*this).collider.base.acFlags as libc::c_int &
                         !((1 as libc::c_int) << 1 as libc::c_int)) as u8_0
            }
            Collider_UpdateCylinder(&mut (*this).actor,
                                    &mut (*this).collider);
            CollisionCheck_SetAC(globalCtx, &mut (*globalCtx).colChkCtx,
                                 &mut (*this).collider.base);
            CollisionCheck_SetAT(globalCtx, &mut (*globalCtx).colChkCtx,
                                 &mut (*this).collider.base);
        }
        if (*this).actor.params as libc::c_int == 0 as libc::c_int {
            (*this).workf[OUTR_CRWN_TX_X2 as libc::c_int as usize] += 1.0f32;
            (*this).workf[OUTR_CRWN_TX_Y2 as libc::c_int as usize] -= 7.0f32;
            (*this).workf[INNR_CRWN_TX_Y1 as libc::c_int as usize] += 1.0f32
        } else {
            (*this).workf[OUTR_CRWN_TX_X2 as libc::c_int as usize] += 0.0f32;
            (*this).workf[INNR_CRWN_TX_X2 as libc::c_int as usize] += 0.0f32;
            (*this).workf[OUTR_CRWN_TX_Y2 as libc::c_int as usize] +=
                -15.0f32;
            (*this).workf[INNR_CRWN_TX_Y2 as libc::c_int as usize] += -10.0f32
        }
        if (*this).work[CS_TIMER_2 as libc::c_int as usize] as libc::c_int %
               32 as libc::c_int == 0 as libc::c_int &&
               Rand_ZeroOne() < 0.3f32 {
            (*this).work[BLINK_IDX as libc::c_int as usize] =
                4 as libc::c_int as s16
        }
        (*this).eyeTexIdx =
            D_8094A900[(*this).work[BLINK_IDX as libc::c_int as usize] as
                           usize];
        if (*this).work[BLINK_IDX as libc::c_int as usize] as libc::c_int !=
               0 as libc::c_int {
            (*this).work[BLINK_IDX as libc::c_int as usize] -= 1
        }
        if (*this).actionFunc !=
               Some(BossTw_MergeCS as
                        unsafe extern "C" fn(_: *mut BossTw,
                                             _: *mut GlobalContext) -> ()) &&
               (*this).unk_5F8 as libc::c_int != 0 as libc::c_int {
            let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
            let mut velocity: Vec3f =
                {
                    let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,};
                    init
                };
            let mut accel: Vec3f =
                {
                    let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,};
                    init
                };
            if (*this).scepterAlpha > 0.0f32 {
                i = 0 as libc::c_int as s16;
                while i as libc::c_int <= 0 as libc::c_int {
                    pos = (*this).scepterFlamePos[0 as libc::c_int as usize];
                    pos.x += Rand_CenteredFloat(70.0f32);
                    pos.y += Rand_CenteredFloat(70.0f32);
                    pos.z += Rand_CenteredFloat(70.0f32);
                    accel.y = 0.4f32;
                    accel.x = Rand_CenteredFloat(0.5f32);
                    accel.z = Rand_CenteredFloat(0.5f32);
                    BossTw_AddDotEffect(globalCtx, &mut pos, &mut velocity,
                                        &mut accel,
                                        (Rand_ZeroFloat(2.0f32) as s16 as
                                             libc::c_int + 8 as libc::c_int)
                                            as f32_0, (*this).actor.params,
                                        37 as libc::c_int as s16);
                    i += 1
                }
            }
            i = 0 as libc::c_int as s16;
            while i as libc::c_int <= 0 as libc::c_int {
                pos = (*this).crownPos;
                pos.x += Rand_CenteredFloat(70.0f32);
                pos.y += Rand_CenteredFloat(70.0f32);
                pos.z += Rand_CenteredFloat(70.0f32);
                accel.y = 0.4f32;
                accel.x = Rand_CenteredFloat(0.5f32);
                accel.z = Rand_CenteredFloat(0.5f32);
                BossTw_AddDotEffect(globalCtx, &mut pos, &mut velocity,
                                    &mut accel,
                                    (Rand_ZeroFloat(2.0f32) as s16 as
                                         libc::c_int + 8 as libc::c_int) as
                                        f32_0, (*this).actor.params,
                                    37 as libc::c_int as s16);
                i += 1
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_TwinrovaUpdate(mut thisx: *mut Actor,
                                               mut globalCtx2:
                                                   *mut GlobalContext) {
    let mut i: s16 = 0;
    let mut globalCtx: *mut GlobalContext = globalCtx2;
    let mut this: *mut BossTw = thisx as *mut BossTw;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    (*this).actor.flags &=
        !((1 as libc::c_int) << 10 as libc::c_int) as libc::c_uint;
    (*this).unk_5F8 = 0 as libc::c_int as u8_0;
    (*this).collider.base.colType = COLTYPE_HIT3 as libc::c_int as u8_0;
    Math_ApproachF(&mut (*this).fogR,
                   (*globalCtx).lightCtx.fogColor[0 as libc::c_int as usize]
                       as f32_0, 1.0f32, 10.0f32);
    Math_ApproachF(&mut (*this).fogG,
                   (*globalCtx).lightCtx.fogColor[1 as libc::c_int as usize]
                       as f32_0, 1.0f32, 10.0f32);
    Math_ApproachF(&mut (*this).fogB,
                   (*globalCtx).lightCtx.fogColor[2 as libc::c_int as usize]
                       as f32_0, 1.0f32, 10.0f32);
    Math_ApproachF(&mut (*this).fogNear,
                   (*globalCtx).lightCtx.fogNear as f32_0, 1.0f32, 10.0f32);
    Math_ApproachF(&mut (*this).fogFar, 1000.0f32, 1.0f32, 10.0f32);
    (*this).work[CS_TIMER_1 as libc::c_int as usize] += 1;
    (*this).work[CS_TIMER_2 as libc::c_int as usize] += 1;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < 5 as libc::c_int {
        if (*this).timers[i as usize] as libc::c_int != 0 as libc::c_int {
            (*this).timers[i as usize] -= 1
        }
        i += 1
    }
    if (*this).work[INVINC_TIMER as libc::c_int as usize] as libc::c_int !=
           0 as libc::c_int {
        (*this).work[INVINC_TIMER as libc::c_int as usize] -= 1
    }
    if (*this).work[FOG_TIMER as libc::c_int as usize] as libc::c_int !=
           0 as libc::c_int {
        (*this).work[FOG_TIMER as libc::c_int as usize] -= 1
    }
    (*this).actionFunc.expect("non-null function pointer")(this, globalCtx);
    if (*this).actionFunc !=
           Some(BossTw_TwinrovaShootBlast as
                    unsafe extern "C" fn(_: *mut BossTw,
                                         _: *mut GlobalContext) -> ()) &&
           (*this).actionFunc !=
               Some(BossTw_TwinrovaChargeBlast as
                        unsafe extern "C" fn(_: *mut BossTw,
                                             _: *mut GlobalContext) -> ()) &&
           (*this).visible as libc::c_int != 0 &&
           (*this).unk_5F8 as libc::c_int == 0 as libc::c_int &&
           (((*player).actor.shape.rot.y as libc::c_int -
                 (*this).actor.yawTowardsPlayer as libc::c_int +
                 0x8000 as libc::c_int) as s16 as libc::c_int) <
               0x1000 as libc::c_int &&
           ((*player).actor.shape.rot.y as libc::c_int -
                (*this).actor.yawTowardsPlayer as libc::c_int +
                0x8000 as libc::c_int) as s16 as libc::c_int >
               -(0x1000 as libc::c_int) &&
           (*player).unk_A73 as libc::c_int != 0 as libc::c_int {
        BossTw_TwinrovaSetupSpin(this, globalCtx);
    }
    (*this).eyeTexIdx =
        D_8094A900[(*this).work[BLINK_IDX as libc::c_int as usize] as usize];
    if (*this).work[BLINK_IDX as libc::c_int as usize] as libc::c_int !=
           0 as libc::c_int {
        (*this).work[BLINK_IDX as libc::c_int as usize] -= 1
    }
    if (*this).work[CS_TIMER_2 as libc::c_int as usize] as libc::c_int %
           32 as libc::c_int == 0 as libc::c_int {
        if (*this).actionFunc !=
               Some(BossTw_TwinrovaMergeCS as
                        unsafe extern "C" fn(_: *mut BossTw,
                                             _: *mut GlobalContext) -> ()) {
            if Rand_ZeroOne() < 0.3f32 {
                (*this).work[BLINK_IDX as libc::c_int as usize] =
                    4 as libc::c_int as s16
            }
        }
    }
    if (*this).actionFunc ==
           Some(BossTw_TwinrovaMergeCS as
                    unsafe extern "C" fn(_: *mut BossTw,
                                         _: *mut GlobalContext) -> ()) {
        (*this).leftEyeTexIdx =
            D_8094A90C[(*this).work[TW_BLINK_IDX as libc::c_int as usize] as
                           usize];
        if (*this).work[TW_BLINK_IDX as libc::c_int as usize] as libc::c_int
               != 0 as libc::c_int {
            (*this).work[TW_BLINK_IDX as libc::c_int as usize] -= 1
        }
    } else {
        if (*this).actionFunc ==
               Some(BossTw_TwinrovaStun as
                        unsafe extern "C" fn(_: *mut BossTw,
                                             _: *mut GlobalContext) -> ()) {
            (*this).eyeTexIdx = 1 as libc::c_int as s16
        }
        if (*this).actionFunc ==
               Some(BossTw_TwinrovaDeathCS as
                        unsafe extern "C" fn(_: *mut BossTw,
                                             _: *mut GlobalContext) -> ()) {
            (*this).eyeTexIdx = 2 as libc::c_int as s16
        }
        (*this).leftEyeTexIdx = (*this).eyeTexIdx
    }
    if (*this).visible as libc::c_int != 0 &&
           (*this).unk_5F8 as libc::c_int == 0 as libc::c_int {
        let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
        let mut velocity: Vec3f =
            { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
        let mut accel: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
        if (*this).work[UNK_S8 as libc::c_int as usize] as libc::c_int !=
               0 as libc::c_int {
            (*this).work[UNK_S8 as libc::c_int as usize] =
                ((*this).work[UNK_S8 as libc::c_int as usize] as libc::c_int -
                     20 as libc::c_int) as s16;
            if ((*this).work[UNK_S8 as libc::c_int as usize] as libc::c_int) <
                   0 as libc::c_int {
                (*this).work[UNK_S8 as libc::c_int as usize] =
                    0 as libc::c_int as s16
            }
        }
        Math_ApproachF(&mut *(*this).workf.as_mut_ptr().offset(UNK_F12 as
                                                                   libc::c_int
                                                                   as isize),
                       1.0f32, 1.0f32, 0.05f32);
        accel.y = 0.4f32;
        i = 0 as libc::c_int as s16;
        while (i as libc::c_int) < 2 as libc::c_int {
            pos = (*this).leftScepterPos;
            pos.x += Rand_CenteredFloat(30.0f32);
            pos.y += Rand_CenteredFloat(30.0f32);
            pos.z += Rand_CenteredFloat(30.0f32);
            accel.x = Rand_CenteredFloat(0.5f32);
            accel.z = Rand_CenteredFloat(0.5f32);
            BossTw_AddDotEffect(globalCtx, &mut pos, &mut velocity,
                                &mut accel,
                                (Rand_ZeroFloat(2.0f32) as s16 as libc::c_int
                                     + 7 as libc::c_int) as f32_0,
                                0 as libc::c_int as s16,
                                75 as libc::c_int as s16);
            i += 1
        }
        i = 0 as libc::c_int as s16;
        while (i as libc::c_int) < 2 as libc::c_int {
            pos = (*this).rightScepterPos;
            pos.x += Rand_CenteredFloat(30.0f32);
            pos.y += Rand_CenteredFloat(30.0f32);
            pos.z += Rand_CenteredFloat(30.0f32);
            accel.x = Rand_CenteredFloat(0.5f32);
            accel.z = Rand_CenteredFloat(0.5f32);
            BossTw_AddDotEffect(globalCtx, &mut pos, &mut velocity,
                                &mut accel,
                                (Rand_ZeroFloat(2.0f32) as s16 as libc::c_int
                                     + 7 as libc::c_int) as f32_0,
                                1 as libc::c_int as s16,
                                75 as libc::c_int as s16);
            i += 1
        }
    }
    (*this).collider.dim.radius = 35 as libc::c_int as s16;
    if (*this).actionFunc ==
           Some(BossTw_TwinrovaSpin as
                    unsafe extern "C" fn(_: *mut BossTw,
                                         _: *mut GlobalContext) -> ()) {
        (*this).collider.dim.radius =
            ((*this).collider.dim.radius as libc::c_int * 2 as libc::c_int) as
                s16
    }
    (*this).collider.dim.height = 150 as libc::c_int as s16;
    (*this).collider.dim.yShift = -(60 as libc::c_int) as s16;
    Collider_UpdateCylinder(&mut (*this).actor, &mut (*this).collider);
    if (*this).work[INVINC_TIMER as libc::c_int as usize] as libc::c_int ==
           0 as libc::c_int {
        if (*this).actionFunc !=
               Some(BossTw_TwinrovaStun as
                        unsafe extern "C" fn(_: *mut BossTw,
                                             _: *mut GlobalContext) -> ()) {
            if (*this).twinrovaStun as libc::c_int != 0 as libc::c_int {
                (*this).twinrovaStun = 0 as libc::c_int as u8_0;
                (*this).work[FOG_TIMER as libc::c_int as usize] =
                    10 as libc::c_int as s16;
                BossTw_TwinrovaDamage(this, globalCtx,
                                      0 as libc::c_int as u8_0);
                Audio_PlayActorSound2(&mut (*this).actor,
                                      0x391a as libc::c_int as u16_0);
            } else if (*this).collider.base.acFlags as libc::c_int &
                          (1 as libc::c_int) << 1 as libc::c_int != 0 {
                let mut info: *mut ColliderInfo =
                    (*this).collider.info.acHitInfo;
                (*this).collider.base.acFlags =
                    ((*this).collider.base.acFlags as libc::c_int &
                         !((1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
                ((*info).toucher.dmgFlags &
                     ((1 as libc::c_int) << 0x2 as libc::c_int |
                          ((1 as libc::c_int) << 0x5 as libc::c_int |
                               (1 as libc::c_int) << 0xb as libc::c_int |
                               (1 as libc::c_int) << 0xc as libc::c_int |
                               (1 as libc::c_int) << 0xd as libc::c_int |
                               (1 as libc::c_int) << 0xe as libc::c_int |
                               (1 as libc::c_int) << 0xf as libc::c_int |
                               (1 as libc::c_int) << 0x10 as libc::c_int)) as
                         libc::c_uint) != 0;
            }
        } else if (*this).collider.base.acFlags as libc::c_int &
                      (1 as libc::c_int) << 1 as libc::c_int != 0 {
            let mut damage: u8_0 = 0;
            let mut swordDamage: u8_0 = 0;
            let mut info_0: *mut ColliderInfo =
                (*this).collider.info.acHitInfo;
            (*this).collider.base.acFlags =
                ((*this).collider.base.acFlags as libc::c_int &
                     !((1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
            swordDamage = 0 as libc::c_int as u8_0;
            damage =
                CollisionCheck_GetSwordDamage((*info_0).toucher.dmgFlags as
                                                  s32);
            if damage as libc::c_int == 0 as libc::c_int {
                damage = 2 as libc::c_int as u8_0
            } else { swordDamage = 1 as libc::c_int as u8_0 }
            if (*info_0).toucher.dmgFlags &
                   ((1 as libc::c_int) << 0x7 as libc::c_int) as libc::c_uint
                   == 0 {
                if ((*this).actor.colChkInfo.health as s8 as libc::c_int) <
                       3 as libc::c_int && swordDamage == 0 {
                    damage = 0 as libc::c_int as u8_0
                }
                BossTw_TwinrovaDamage(this, globalCtx, damage);
            }
        }
    }
    CollisionCheck_SetAC(globalCtx, &mut (*globalCtx).colChkCtx,
                         &mut (*this).collider.base);
    osSyncPrintf(b"OooooooooooooooooooooooooooooooooCC\n\x00" as *const u8 as
                     *const libc::c_char);
    CollisionCheck_SetOC(globalCtx, &mut (*globalCtx).colChkCtx,
                         &mut (*this).collider.base);
    (*globalCtx).envCtx.unk_DC = 2 as libc::c_int as u8_0;
    match sEnvType as libc::c_int {
        0 => {
            Math_ApproachZeroF(&mut (*globalCtx).envCtx.unk_D8, 1.0f32,
                               0.02f32);
        }
        1 => {
            (*globalCtx).envCtx.unk_BD = 3 as libc::c_int as u8_0;
            Math_ApproachF(&mut (*globalCtx).envCtx.unk_D8, 0.5f32, 1.0f32,
                           0.05f32);
        }
        2 => {
            (*globalCtx).envCtx.unk_BD = 2 as libc::c_int as u8_0;
            Math_ApproachF(&mut (*globalCtx).envCtx.unk_D8,
                           Math_SinS(((*this).work[CS_TIMER_1 as libc::c_int
                                                       as usize] as
                                          libc::c_int * 0x3000 as libc::c_int)
                                         as s16) * 0.03f32 + 0.5f32, 1.0f32,
                           0.05f32);
        }
        3 => {
            (*globalCtx).envCtx.unk_BD = 3 as libc::c_int as u8_0;
            Math_ApproachF(&mut (*globalCtx).envCtx.unk_D8, 1.0f32, 1.0f32,
                           0.1f32);
        }
        4 => {
            (*globalCtx).envCtx.unk_BD = 2 as libc::c_int as u8_0;
            Math_ApproachF(&mut (*globalCtx).envCtx.unk_D8,
                           Math_SinS(((*this).work[CS_TIMER_1 as libc::c_int
                                                       as usize] as
                                          libc::c_int * 0x3e00 as libc::c_int)
                                         as s16) * 0.05f32 + 0.95f32, 1.0f32,
                           0.1f32);
        }
        5 => {
            (*globalCtx).envCtx.unk_BD = 0 as libc::c_int as u8_0;
            Math_ApproachF(&mut (*globalCtx).envCtx.unk_D8, 1.0f32, 1.0f32,
                           0.05f32);
        }
        -1 | _ => { }
    }
    BossTw_UpdateEffects(globalCtx);
    if sFreezeState as libc::c_int == 1 as libc::c_int {
        sFreezeState = 2 as libc::c_int as u8_0;
        BossTw_AddPlayerFreezeEffect(globalCtx, 0 as *mut Actor);
        func_80078914(&mut (*player).actor.projectedPos,
                      0x6806 as libc::c_int as u16_0);
        func_80078914(&mut (*player).actor.projectedPos,
                      0x86e as libc::c_int as u16_0);
        if sShieldFireCharge as libc::c_int != 0 as libc::c_int {
            sShieldFireCharge = 4 as libc::c_int as u8_0
        }
    }
    if (*player).isBurning as libc::c_int != 0 &&
           sShieldIceCharge as libc::c_int != 0 as libc::c_int {
        sShieldIceCharge = 4 as libc::c_int as u8_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_OverrideLimbDraw(mut globalCtx:
                                                     *mut GlobalContext,
                                                 mut limbIndex: s32,
                                                 mut dList: *mut *mut Gfx,
                                                 mut pos: *mut Vec3f,
                                                 mut rot: *mut Vec3s,
                                                 mut thisx: *mut libc::c_void)
 -> s32 {
    let mut this: *mut BossTw = thisx as *mut BossTw;
    if limbIndex == 21 as libc::c_int {
        if (*this).unk_5F8 as libc::c_int == 0 as libc::c_int {
            if (*this).actor.params as libc::c_int == 0 as libc::c_int {
                *dList = object_tw_DL_012CE0.as_mut_ptr()
            } else { *dList = object_tw_DL_0134B8.as_mut_ptr() }
        }
    }
    if limbIndex == 14 as libc::c_int {
        if (*this).actionFunc ==
               Some(BossTw_DeathCS as
                        unsafe extern "C" fn(_: *mut BossTw,
                                             _: *mut GlobalContext) -> ()) {
            *dList = 0 as *mut Gfx
        } else if (*this).scepterAlpha == 0.0f32 {
            if (*this).actor.params as libc::c_int == 0 as libc::c_int {
                *dList = object_tw_DL_012B38.as_mut_ptr()
            } else { *dList = object_tw_DL_013310.as_mut_ptr() }
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_PostLimbDraw(mut globalCtx:
                                                 *mut GlobalContext,
                                             mut limbIndex: s32,
                                             mut dList: *mut *mut Gfx,
                                             mut rot: *mut Vec3s,
                                             mut thisx: *mut libc::c_void) {
    static mut D_8094A944: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
    static mut D_8094A950: Vec3f =
        {
            let mut init = Vec3f{x: 0.0f32, y: 2000.0f32, z: -2000.0f32,};
            init
        };
    static mut D_8094A95C: [Vec3f; 5] =
        [{
             let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: -10000.0f32,};
             init
         },
         { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: -8000.0f32,}; init },
         { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: -9000.0f32,}; init },
         {
             let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: -11000.0f32,};
             init
         },
         {
             let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: -12000.0f32,};
             init
         }];
    let mut this: *mut BossTw = thisx as *mut BossTw;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_boss_tw.c\x00" as *const u8 as *const libc::c_char,
                    6168 as libc::c_int);
    match limbIndex {
        21 => {
            Matrix_MultVec3f(&mut D_8094A944, &mut (*this).actor.focus.pos);
            Matrix_MultVec3f(&mut D_8094A950, &mut (*this).crownPos);
            if (*this).unk_5F8 as libc::c_int != 0 as libc::c_int {
                let fresh0 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g: *mut Gfx = fresh0;
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
                        (((0x2 as libc::c_int | 0 as libc::c_int |
                               0 as libc::c_int) ^ 0x1 as libc::c_int) as
                             u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                (*_g).words.w1 =
                    Matrix_NewMtx((*globalCtx).state.gfxCtx,
                                  b"../z_boss_tw.c\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char, 6190 as libc::c_int)
                        as libc::c_uint;
                if (*this).actor.params as libc::c_int == 0 as libc::c_int {
                    let fresh1 = (*__gfxCtx).polyXlu.p;
                    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                    let mut _g_0: *mut Gfx = fresh1;
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
                                 (((0x1 as libc::c_int) << 16 as libc::c_int)
                                      - 1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int;
                    (*_g_0).words.w1 =
                        gSegments[((object_tw_DL_013AE8.as_mut_ptr() as u32_0)
                                       << 4 as libc::c_int >>
                                       28 as libc::c_int) as
                                      usize].wrapping_add(object_tw_DL_013AE8.as_mut_ptr()
                                                              as u32_0 &
                                                              0xffffff as
                                                                  libc::c_int
                                                                  as
                                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                                 as
                                                                                                 libc::c_uint)
                            as *mut libc::c_void as libc::c_uint
                } else {
                    let fresh2 = (*__gfxCtx).polyXlu.p;
                    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                    let mut _g_1: *mut Gfx = fresh2;
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
                                 (((0x1 as libc::c_int) << 16 as libc::c_int)
                                      - 1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int;
                    (*_g_1).words.w1 =
                        gSegments[((object_tw_DL_013D68.as_mut_ptr() as u32_0)
                                       << 4 as libc::c_int >>
                                       28 as libc::c_int) as
                                      usize].wrapping_add(object_tw_DL_013D68.as_mut_ptr()
                                                              as u32_0 &
                                                              0xffffff as
                                                                  libc::c_int
                                                                  as
                                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                                 as
                                                                                                 libc::c_uint)
                            as *mut libc::c_void as libc::c_uint
                }
            }
        }
        14 => {
            Matrix_MultVec3f(&mut *D_8094A95C.as_mut_ptr().offset(0 as
                                                                      libc::c_int
                                                                      as
                                                                      isize),
                             &mut *(*this).scepterFlamePos.as_mut_ptr().offset(0
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize));
            Matrix_MultVec3f(&mut *D_8094A95C.as_mut_ptr().offset(1 as
                                                                      libc::c_int
                                                                      as
                                                                      isize),
                             &mut *(*this).scepterFlamePos.as_mut_ptr().offset(1
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize));
            Matrix_MultVec3f(&mut *D_8094A95C.as_mut_ptr().offset(2 as
                                                                      libc::c_int
                                                                      as
                                                                      isize),
                             &mut *(*this).scepterFlamePos.as_mut_ptr().offset(2
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize));
            Matrix_MultVec3f(&mut *D_8094A95C.as_mut_ptr().offset(3 as
                                                                      libc::c_int
                                                                      as
                                                                      isize),
                             &mut *(*this).scepterFlamePos.as_mut_ptr().offset(3
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize));
            Matrix_MultVec3f(&mut *D_8094A95C.as_mut_ptr().offset(4 as
                                                                      libc::c_int
                                                                      as
                                                                      isize),
                             &mut *(*this).scepterFlamePos.as_mut_ptr().offset(4
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize));
            if (*this).scepterAlpha > 0.0f32 {
                let fresh3 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_2: *mut Gfx = fresh3;
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
                        (((0x2 as libc::c_int | 0 as libc::c_int |
                               0 as libc::c_int) ^ 0x1 as libc::c_int) as
                             u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                (*_g_2).words.w1 =
                    Matrix_NewMtx((*globalCtx).state.gfxCtx,
                                  b"../z_boss_tw.c\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char, 6221 as libc::c_int)
                        as libc::c_uint;
                if (*this).actor.params as libc::c_int == 0 as libc::c_int {
                    let fresh4 = (*__gfxCtx).polyXlu.p;
                    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                    let mut _g_3: *mut Gfx = fresh4;
                    (*_g_3).words.w0 =
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
                    (*_g_3).words.w1 =
                        (255 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            24 as libc::c_int |
                            (225 as libc::c_int as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                16 as libc::c_int |
                            (255 as libc::c_int as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                8 as libc::c_int |
                            ((*this).scepterAlpha as s16 as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int;
                    let fresh5 = (*__gfxCtx).polyXlu.p;
                    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                    let mut _g_4: *mut Gfx = fresh5;
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
                                 (((0x1 as libc::c_int) << 16 as libc::c_int)
                                      - 1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int;
                    (*_g_4).words.w1 =
                        gSegments[((object_tw_DL_013E98.as_mut_ptr() as u32_0)
                                       << 4 as libc::c_int >>
                                       28 as libc::c_int) as
                                      usize].wrapping_add(object_tw_DL_013E98.as_mut_ptr()
                                                              as u32_0 &
                                                              0xffffff as
                                                                  libc::c_int
                                                                  as
                                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                                 as
                                                                                                 libc::c_uint)
                            as *mut libc::c_void as libc::c_uint;
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
                            (0 as libc::c_int as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int;
                    (*_g_5).words.w1 =
                        (195 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            24 as libc::c_int |
                            (225 as libc::c_int as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                16 as libc::c_int |
                            (235 as libc::c_int as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                8 as libc::c_int |
                            ((*this).scepterAlpha as s16 as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int;
                    let fresh7 = (*__gfxCtx).polyXlu.p;
                    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                    let mut _g_6: *mut Gfx = fresh7;
                    (*_g_6).words.w0 =
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
                    (*_g_6).words.w1 =
                        gSegments[((object_tw_DL_013F98.as_mut_ptr() as u32_0)
                                       << 4 as libc::c_int >>
                                       28 as libc::c_int) as
                                      usize].wrapping_add(object_tw_DL_013F98.as_mut_ptr()
                                                              as u32_0 &
                                                              0xffffff as
                                                                  libc::c_int
                                                                  as
                                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                                 as
                                                                                                 libc::c_uint)
                            as *mut libc::c_void as libc::c_uint
                } else {
                    let fresh8 = (*__gfxCtx).polyXlu.p;
                    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                    let mut _g_7: *mut Gfx = fresh8;
                    (*_g_7).words.w0 =
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
                    (*_g_7).words.w1 =
                        (100 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            24 as libc::c_int |
                            (20 as libc::c_int as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                16 as libc::c_int |
                            (0 as libc::c_int as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                8 as libc::c_int |
                            ((*this).scepterAlpha as s16 as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int;
                    let fresh9 = (*__gfxCtx).polyXlu.p;
                    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                    let mut _g_8: *mut Gfx = fresh9;
                    (*_g_8).words.w0 =
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
                    (*_g_8).words.w1 =
                        gSegments[((object_tw_DL_014070.as_mut_ptr() as u32_0)
                                       << 4 as libc::c_int >>
                                       28 as libc::c_int) as
                                      usize].wrapping_add(object_tw_DL_014070.as_mut_ptr()
                                                              as u32_0 &
                                                              0xffffff as
                                                                  libc::c_int
                                                                  as
                                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                                 as
                                                                                                 libc::c_uint)
                            as *mut libc::c_void as libc::c_uint;
                    let fresh10 = (*__gfxCtx).polyXlu.p;
                    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                    let mut _g_9: *mut Gfx = fresh10;
                    (*_g_9).words.w0 =
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
                    (*_g_9).words.w1 =
                        (255 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            24 as libc::c_int |
                            (70 as libc::c_int as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                16 as libc::c_int |
                            (0 as libc::c_int as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                8 as libc::c_int |
                            ((*this).scepterAlpha as s16 as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int;
                    let fresh11 = (*__gfxCtx).polyXlu.p;
                    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
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
                                 (((0x1 as libc::c_int) << 16 as libc::c_int)
                                      - 1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int;
                    (*_g_10).words.w1 =
                        gSegments[((object_tw_DL_014158.as_mut_ptr() as u32_0)
                                       << 4 as libc::c_int >>
                                       28 as libc::c_int) as
                                      usize].wrapping_add(object_tw_DL_014158.as_mut_ptr()
                                                              as u32_0 &
                                                              0xffffff as
                                                                  libc::c_int
                                                                  as
                                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                                 as
                                                                                                 libc::c_uint)
                            as *mut libc::c_void as libc::c_uint
                }
            }
        }
        _ => { }
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_boss_tw.c\x00" as *const u8 as
                         *const libc::c_char, 6236 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn func_80941BC0(mut this: *mut BossTw,
                                       mut globalCtx: *mut GlobalContext) {
    let mut pad: s32 = 0;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_boss_tw.c\x00" as *const u8 as *const libc::c_char,
                    6341 as libc::c_int);
    Matrix_Push();
    func_80093D84((*globalCtx).state.gfxCtx);
    Matrix_Translate((*this).groundBlastPos2.x, (*this).groundBlastPos2.y,
                     (*this).groundBlastPos2.z,
                     MTXMODE_NEW as libc::c_int as u8_0);
    Matrix_Scale((*this).workf[UNK_F12 as libc::c_int as usize],
                 (*this).workf[UNK_F12 as libc::c_int as usize],
                 (*this).workf[UNK_F12 as libc::c_int as usize],
                 MTXMODE_APPLY as libc::c_int as u8_0);
    let fresh12 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g: *mut Gfx = fresh12;
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
            (((0x2 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int) ^
                  0x1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g).words.w1 =
        Matrix_NewMtx((*globalCtx).state.gfxCtx,
                      b"../z_boss_tw.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      6358 as libc::c_int) as libc::c_uint;
    let fresh13 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_0: *mut Gfx = fresh13;
    (*_g_0).words.w0 =
        (0xfa as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_0).words.w1 =
        (255 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            ((*this).workf[UNK_F11 as libc::c_int as usize] as s16 as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh14 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_1: *mut Gfx = fresh14;
    (*_g_1).words.w0 =
        (0xfb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_1).words.w1 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (40 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (30 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (80 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh15 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_2: *mut Gfx = fresh15;
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
    (*_g_2).words.w1 =
        gSegments[((object_tw_DL_01BC00.as_mut_ptr() as u32_0) <<
                       4 as libc::c_int >> 28 as libc::c_int) as
                      usize].wrapping_add(object_tw_DL_01BC00.as_mut_ptr() as
                                              u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as libc::c_uint;
    let fresh16 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_3: *mut Gfx = fresh16;
    (*_g_3).words.w0 =
        (0xfa as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_3).words.w1 =
        (215 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (215 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (215 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (((*this).workf[UNK_F11 as libc::c_int as usize] as s16 as
                  libc::c_int as libc::c_float *
                  (*this).workf[UNK_F14 as libc::c_int as usize]) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh17 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_4: *mut Gfx = fresh17;
    (*_g_4).words.w0 =
        (0xfb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_4).words.w1 =
        (255 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (128 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh18 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_5: *mut Gfx = fresh18;
    (*_g_5).words.w0 =
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
    (*_g_5).words.w1 =
        Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                         0 as libc::c_int as u32_0, 0 as libc::c_int as u32_0,
                         0x20 as libc::c_int, 0x40 as libc::c_int,
                         1 as libc::c_int,
                         (*this).workf[UNK_F16 as libc::c_int as usize] as
                             u32_0 & 0x3f as libc::c_int as libc::c_uint,
                         ((*this).work[CS_TIMER_2 as libc::c_int as usize] as
                              libc::c_int * 4 as libc::c_int &
                              0x3f as libc::c_int) as u32_0,
                         0x10 as libc::c_int, 0x10 as libc::c_int) as
            libc::c_uint;
    Matrix_Push();
    Matrix_RotateY((*this).workf[UNK_F15 as libc::c_int as usize],
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
            (((0x2 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int) ^
                  0x1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_6).words.w1 =
        Matrix_NewMtx((*globalCtx).state.gfxCtx,
                      b"../z_boss_tw.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      6423 as libc::c_int) as libc::c_uint;
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
    (*_g_7).words.w1 =
        gSegments[((object_tw_DL_01C1C0.as_mut_ptr() as u32_0) <<
                       4 as libc::c_int >> 28 as libc::c_int) as
                      usize].wrapping_add(object_tw_DL_01C1C0.as_mut_ptr() as
                                              u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as libc::c_uint;
    Matrix_Pop();
    let fresh21 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_8: *mut Gfx = fresh21;
    (*_g_8).words.w0 =
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
            (((0x2 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int) ^
                  0x1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_8).words.w1 =
        Matrix_NewMtx((*globalCtx).state.gfxCtx,
                      b"../z_boss_tw.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      6427 as libc::c_int) as libc::c_uint;
    let fresh22 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_9: *mut Gfx = fresh22;
    (*_g_9).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((0xd as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_9).words.w1 =
        Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                         ((*this).work[CS_TIMER_2 as libc::c_int as usize] as
                              libc::c_int & 0x7f as libc::c_int) as u32_0,
                         ((*this).work[CS_TIMER_2 as libc::c_int as usize] as
                              libc::c_int * 8 as libc::c_int &
                              0xff as libc::c_int) as u32_0,
                         0x20 as libc::c_int, 0x40 as libc::c_int,
                         1 as libc::c_int,
                         (-((*this).work[CS_TIMER_2 as libc::c_int as usize]
                                as libc::c_int) * 2 as libc::c_int &
                              0x3f as libc::c_int) as u32_0,
                         0 as libc::c_int as u32_0, 0x10 as libc::c_int,
                         0x10 as libc::c_int) as libc::c_uint;
    let fresh23 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_10: *mut Gfx = fresh23;
    (*_g_10).words.w0 =
        (0xfa as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_10).words.w1 =
        (195 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (225 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (235 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            ((*this).workf[UNK_F9 as libc::c_int as usize] as s16 as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh24 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_11: *mut Gfx = fresh24;
    (*_g_11).words.w0 =
        (0xfb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_11).words.w1 =
        (255 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (128 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh25 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_12: *mut Gfx = fresh25;
    (*_g_12).words.w0 =
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
    (*_g_12).words.w1 =
        (0x10 as libc::c_int | 0x40 as libc::c_int | 0x300 as libc::c_int |
             0xc00 as libc::c_int | 0x4000 as libc::c_int |
             (3 as libc::c_int) << 30 as libc::c_int |
             (2 as libc::c_int) << 26 as libc::c_int |
             (0 as libc::c_int) << 22 as libc::c_int |
             (0 as libc::c_int) << 18 as libc::c_int |
             (0x10 as libc::c_int | 0x40 as libc::c_int | 0x300 as libc::c_int
                  | 0x4000 as libc::c_int | 0xc00 as libc::c_int |
                  (0 as libc::c_int) << 28 as libc::c_int |
                  (0 as libc::c_int) << 24 as libc::c_int |
                  (1 as libc::c_int) << 20 as libc::c_int |
                  (0 as libc::c_int) << 16 as libc::c_int)) as libc::c_uint;
    let fresh26 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_13: *mut Gfx = fresh26;
    (*_g_13).words.w0 =
        (0xd9 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (!(0 as libc::c_int as u32_0) &
                 (((0x1 as libc::c_int) << 24 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_13).words.w1 =
        (0x400 as libc::c_int | 0x10000 as libc::c_int) as u32_0;
    let fresh27 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_14: *mut Gfx = fresh27;
    (*_g_14).words.w0 =
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
    (*_g_14).words.w1 =
        gSegments[((object_tw_DL_01A790.as_mut_ptr() as u32_0) <<
                       4 as libc::c_int >> 28 as libc::c_int) as
                      usize].wrapping_add(object_tw_DL_01A790.as_mut_ptr() as
                                              u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as libc::c_uint;
    Matrix_Pop();
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_boss_tw.c\x00" as *const u8 as
                         *const libc::c_char, 6461 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn func_80942180(mut this: *mut BossTw,
                                       mut globalCtx: *mut GlobalContext) {
    let mut pad: s32 = 0;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_boss_tw.c\x00" as *const u8 as *const libc::c_char,
                    6468 as libc::c_int);
    Matrix_Push();
    func_80093D84((*globalCtx).state.gfxCtx);
    Matrix_Translate((*this).groundBlastPos2.x, (*this).groundBlastPos2.y,
                     (*this).groundBlastPos2.z,
                     MTXMODE_NEW as libc::c_int as u8_0);
    Matrix_Scale((*this).workf[KM_GD_CRTR_SCL as libc::c_int as usize],
                 (*this).workf[KM_GD_CRTR_SCL as libc::c_int as usize],
                 (*this).workf[KM_GD_CRTR_SCL as libc::c_int as usize],
                 MTXMODE_APPLY as libc::c_int as u8_0);
    let fresh28 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g: *mut Gfx = fresh28;
    (*_g).words.w0 =
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
    (*_g).words.w1 =
        Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                         (-((*this).work[CS_TIMER_1 as libc::c_int as usize]
                                as libc::c_int) & 0x7f as libc::c_int) as
                             u32_0, 0 as libc::c_int as u32_0,
                         0x20 as libc::c_int, 0x20 as libc::c_int,
                         1 as libc::c_int,
                         ((*this).work[CS_TIMER_1 as libc::c_int as usize] as
                              libc::c_int * 2 as libc::c_int &
                              0x7f as libc::c_int) as u32_0,
                         0 as libc::c_int as u32_0, 0x20 as libc::c_int,
                         0x20 as libc::c_int) as libc::c_uint;
    let fresh29 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_0: *mut Gfx = fresh29;
    (*_g_0).words.w0 =
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
            (((0x2 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int) ^
                  0x1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_0).words.w1 =
        Matrix_NewMtx((*globalCtx).state.gfxCtx,
                      b"../z_boss_tw.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      6497 as libc::c_int) as libc::c_uint;
    let fresh30 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_1: *mut Gfx = fresh30;
    (*_g_1).words.w0 =
        (0xfa as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_1).words.w1 =
        (100 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (40 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            ((*this).workf[KM_GRND_CRTR_A as libc::c_int as usize] as s16 as
                 u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh31 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_2: *mut Gfx = fresh31;
    (*_g_2).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_2).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh32 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_3: *mut Gfx = fresh32;
    (*_g_3).words.w0 =
        (0xfb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_3).words.w1 =
        (255 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (245 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (128 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh33 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_4: *mut Gfx = fresh33;
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
    (*_g_4).words.w1 =
        gSegments[((object_tw_DL_019D40.as_mut_ptr() as u32_0) <<
                       4 as libc::c_int >> 28 as libc::c_int) as
                      usize].wrapping_add(object_tw_DL_019D40.as_mut_ptr() as
                                              u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as libc::c_uint;
    func_800D1FD4(&mut (*globalCtx).billboardMtxF);
    let fresh34 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_5: *mut Gfx = fresh34;
    (*_g_5).words.w0 =
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
            (((0x2 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int) ^
                  0x1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_5).words.w1 =
        Matrix_NewMtx((*globalCtx).state.gfxCtx,
                      b"../z_boss_tw.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      6514 as libc::c_int) as libc::c_uint;
    let fresh35 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_6: *mut Gfx = fresh35;
    (*_g_6).words.w0 =
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
    (*_g_6).words.w1 =
        Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                         ((*this).work[CS_TIMER_1 as libc::c_int as usize] as
                              libc::c_int & 0x7f as libc::c_int) as u32_0,
                         (-((*this).work[CS_TIMER_1 as libc::c_int as usize]
                                as libc::c_int) * 6 as libc::c_int &
                              0xff as libc::c_int) as u32_0,
                         0x20 as libc::c_int, 0x40 as libc::c_int,
                         1 as libc::c_int,
                         ((*this).work[CS_TIMER_1 as libc::c_int as usize] as
                              libc::c_int * 2 as libc::c_int &
                              0x7f as libc::c_int) as u32_0,
                         (-((*this).work[CS_TIMER_1 as libc::c_int as usize]
                                as libc::c_int) * 6 as libc::c_int &
                              0xff as libc::c_int) as u32_0,
                         0x20 as libc::c_int, 0x40 as libc::c_int) as
            libc::c_uint;
    let fresh36 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_7: *mut Gfx = fresh36;
    (*_g_7).words.w0 =
        (0xfa as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_7).words.w1 =
        (80 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            ((*this).workf[KM_GD_SMOKE_A as libc::c_int as usize] as s16 as
                 u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh37 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_8: *mut Gfx = fresh37;
    (*_g_8).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_8).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh38 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_9: *mut Gfx = fresh38;
    (*_g_9).words.w0 =
        (0xfb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_9).words.w1 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (100 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh39 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_10: *mut Gfx = fresh39;
    (*_g_10).words.w0 =
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
    (*_g_10).words.w1 =
        gSegments[((object_tw_DL_018FC0.as_mut_ptr() as u32_0) <<
                       4 as libc::c_int >> 28 as libc::c_int) as
                      usize].wrapping_add(object_tw_DL_018FC0.as_mut_ptr() as
                                              u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as libc::c_uint;
    let fresh40 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_11: *mut Gfx = fresh40;
    (*_g_11).words.w0 =
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
    (*_g_11).words.w1 =
        Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                         (-((*this).work[CS_TIMER_1 as libc::c_int as usize]
                                as libc::c_int) * 3 as libc::c_int &
                              0x7f as libc::c_int) as u32_0,
                         0 as libc::c_int as u32_0, 0x20 as libc::c_int,
                         0x20 as libc::c_int, 1 as libc::c_int,
                         0 as libc::c_int as u32_0,
                         (-((*this).work[CS_TIMER_1 as libc::c_int as usize]
                                as libc::c_int) * 10 as libc::c_int &
                              0xff as libc::c_int) as u32_0,
                         0x20 as libc::c_int, 0x40 as libc::c_int) as
            libc::c_uint;
    let fresh41 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_12: *mut Gfx = fresh41;
    (*_g_12).words.w0 =
        (0xfa as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_12).words.w1 =
        (100 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (50 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (((*this).workf[KM_GD_FLM_A as libc::c_int as usize] * 0.7f32) as
                 s16 as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh42 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_13: *mut Gfx = fresh42;
    (*_g_13).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_13).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh43 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_14: *mut Gfx = fresh43;
    (*_g_14).words.w0 =
        (0xfb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_14).words.w1 =
        (200 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (235 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (240 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (128 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    Matrix_Scale((*this).workf[KM_GD_FLM_SCL as libc::c_int as usize],
                 (*this).workf[KM_GD_FLM_SCL as libc::c_int as usize],
                 (*this).workf[KM_GD_FLM_SCL as libc::c_int as usize],
                 MTXMODE_APPLY as libc::c_int as u8_0);
    let fresh44 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_15: *mut Gfx = fresh44;
    (*_g_15).words.w0 =
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
            (((0x2 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int) ^
                  0x1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_15).words.w1 =
        Matrix_NewMtx((*globalCtx).state.gfxCtx,
                      b"../z_boss_tw.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      6575 as libc::c_int) as libc::c_uint;
    let fresh45 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_16: *mut Gfx = fresh45;
    (*_g_16).words.w0 =
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
    (*_g_16).words.w1 =
        gSegments[((object_tw_DL_019938.as_mut_ptr() as u32_0) <<
                       4 as libc::c_int >> 28 as libc::c_int) as
                      usize].wrapping_add(object_tw_DL_019938.as_mut_ptr() as
                                              u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as libc::c_uint;
    Matrix_Pop();
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_boss_tw.c\x00" as *const u8 as
                         *const libc::c_char, 6579 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn func_809426F0(mut this: *mut BossTw,
                                       mut globalCtx: *mut GlobalContext) {
    let mut pad: s32 = 0;
    let mut i: s16 = 0;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_boss_tw.c\x00" as *const u8 as *const libc::c_char,
                    6587 as libc::c_int);
    let fresh46 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g: *mut Gfx = fresh46;
    (*_g).words.w0 =
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
    (*_g).words.w1 =
        Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                         0 as libc::c_int as u32_0,
                         (-((*this).work[CS_TIMER_2 as libc::c_int as usize]
                                as libc::c_int) * 15 as libc::c_int) as u8_0
                             as u32_0, 0x20 as libc::c_int,
                         0x40 as libc::c_int, 1 as libc::c_int,
                         0 as libc::c_int as u32_0, 0 as libc::c_int as u32_0,
                         0x40 as libc::c_int, 0x40 as libc::c_int) as
            libc::c_uint;
    Matrix_Push();
    Matrix_Translate(0.0f32, 0.0f32, 5000.0f32,
                     MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_Scale((*this).spawnPortalScale / 2000.0f32,
                 (*this).spawnPortalScale / 2000.0f32,
                 (*this).spawnPortalScale / 2000.0f32,
                 MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_RotateZ((*this).portalRotation,
                   MTXMODE_APPLY as libc::c_int as u8_0);
    let fresh47 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_0: *mut Gfx = fresh47;
    (*_g_0).words.w0 =
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
            (((0x2 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int) ^
                  0x1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_0).words.w1 =
        Matrix_NewMtx((*globalCtx).state.gfxCtx,
                      b"../z_boss_tw.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      6614 as libc::c_int) as libc::c_uint;
    if (*this).actor.params as libc::c_int == 0 as libc::c_int {
        let fresh48 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_1: *mut Gfx = fresh48;
        (*_g_1).words.w0 =
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
        (*_g_1).words.w1 =
            (135 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (175 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                (165 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                ((*this).spawnPortalAlpha as s16 as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh49 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_2: *mut Gfx = fresh49;
        (*_g_2).words.w0 =
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
        (*_g_2).words.w1 =
            gSegments[((object_tw_DL_01CEE0.as_mut_ptr() as u32_0) <<
                           4 as libc::c_int >> 28 as libc::c_int) as
                          usize].wrapping_add(object_tw_DL_01CEE0.as_mut_ptr()
                                                  as u32_0 &
                                                  0xffffff as libc::c_int as
                                                      libc::c_uint).wrapping_add(0x80000000
                                                                                     as
                                                                                     libc::c_uint)
                as *mut libc::c_void as libc::c_uint
    } else {
        let fresh50 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_3: *mut Gfx = fresh50;
        (*_g_3).words.w0 =
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
        (*_g_3).words.w1 =
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (255 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                ((*this).spawnPortalAlpha as s16 as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh51 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_4: *mut Gfx = fresh51;
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
        (*_g_4).words.w1 =
            gSegments[((object_tw_DL_01DBE8.as_mut_ptr() as u32_0) <<
                           4 as libc::c_int >> 28 as libc::c_int) as
                          usize].wrapping_add(object_tw_DL_01DBE8.as_mut_ptr()
                                                  as u32_0 &
                                                  0xffffff as libc::c_int as
                                                      libc::c_uint).wrapping_add(0x80000000
                                                                                     as
                                                                                     libc::c_uint)
                as *mut libc::c_void as libc::c_uint
    }
    Matrix_Pop();
    if (*this).actor.params as libc::c_int == 0 as libc::c_int {
        let fresh52 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_5: *mut Gfx = fresh52;
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
            (195 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (225 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                (235 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                ((*this).flameAlpha as s16 as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh53 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_6: *mut Gfx = fresh53;
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
        (*_g_6).words.w1 =
            gSegments[((object_tw_DL_01A998.as_mut_ptr() as u32_0) <<
                           4 as libc::c_int >> 28 as libc::c_int) as
                          usize].wrapping_add(object_tw_DL_01A998.as_mut_ptr()
                                                  as u32_0 &
                                                  0xffffff as libc::c_int as
                                                      libc::c_uint).wrapping_add(0x80000000
                                                                                     as
                                                                                     libc::c_uint)
                as *mut libc::c_void as libc::c_uint
    } else {
        let fresh54 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_7: *mut Gfx = fresh54;
        (*_g_7).words.w0 =
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
        (*_g_7).words.w1 =
            (200 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (20 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                ((*this).flameAlpha as s16 as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh55 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_8: *mut Gfx = fresh55;
        (*_g_8).words.w0 =
            (0xfb as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_8).words.w1 =
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (215 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                (255 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (128 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int
    }
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < 8 as libc::c_int {
        Matrix_Push();
        Matrix_Translate(0.0f32, 0.0f32, 5000.0f32,
                         MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_RotateZ(i as libc::c_int as libc::c_float *
                           3.14159265358979323846f32 * 2.0f32 * 0.125f32 +
                           (*this).flameRotation,
                       MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_Translate(0.0f32, (*this).spawnPortalScale * 1.5f32, 0.0f32,
                         MTXMODE_APPLY as libc::c_int as u8_0);
        let fresh56 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_9: *mut Gfx = fresh56;
        (*_g_9).words.w0 =
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
        (*_g_9).words.w1 =
            Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                             ((*this).work[CS_TIMER_2 as libc::c_int as usize]
                                  as libc::c_int * 3 as libc::c_int +
                                  i as libc::c_int * 10 as libc::c_int &
                                  0x7f as libc::c_int) as u32_0,
                             (-((*this).work[CS_TIMER_2 as libc::c_int as
                                                 usize] as libc::c_int) *
                                  15 as libc::c_int +
                                  i as libc::c_int * 50 as libc::c_int) as
                                 u8_0 as u32_0, 0x20 as libc::c_int,
                             0x40 as libc::c_int, 1 as libc::c_int,
                             0 as libc::c_int as u32_0,
                             0 as libc::c_int as u32_0, 0x20 as libc::c_int,
                             0x20 as libc::c_int) as libc::c_uint;
        Matrix_Scale(0.4f32, 0.4f32, 0.4f32,
                     MTXMODE_APPLY as libc::c_int as u8_0);
        func_800D1FD4(&mut (*globalCtx).billboardMtxF);
        let fresh57 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_10: *mut Gfx = fresh57;
        (*_g_10).words.w0 =
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
                (((0x2 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int) ^
                      0x1 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_10).words.w1 =
            Matrix_NewMtx((*globalCtx).state.gfxCtx,
                          b"../z_boss_tw.c\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                          6751 as libc::c_int) as libc::c_uint;
        let fresh58 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_11: *mut Gfx = fresh58;
        (*_g_11).words.w0 =
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
        (*_g_11).words.w1 =
            gSegments[((object_tw_DL_01A430.as_mut_ptr() as u32_0) <<
                           4 as libc::c_int >> 28 as libc::c_int) as
                          usize].wrapping_add(object_tw_DL_01A430.as_mut_ptr()
                                                  as u32_0 &
                                                  0xffffff as libc::c_int as
                                                      libc::c_uint).wrapping_add(0x80000000
                                                                                     as
                                                                                     libc::c_uint)
                as *mut libc::c_void as libc::c_uint;
        Matrix_Pop();
        i += 1
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_boss_tw.c\x00" as *const u8 as
                         *const libc::c_char, 6756 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn func_80942C70(mut thisx: *mut Actor,
                                       mut globalCtx: *mut GlobalContext) {
    let mut this: *mut BossTw = thisx as *mut BossTw;
    let mut alpha: s16 = 0;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_boss_tw.c\x00" as *const u8 as *const libc::c_char,
                    6765 as libc::c_int);
    if (*this).beamDist != 0.0f32 {
        Matrix_Push();
        let fresh59 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g: *mut Gfx = fresh59;
        (*_g).words.w0 =
            (0xdb as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0x6 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                ((0xc as libc::c_int * 4 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g).words.w1 =
            Gfx_TexScroll((*globalCtx).state.gfxCtx,
                          0 as libc::c_int as u32_0,
                          ((*this).work[CS_TIMER_1 as libc::c_int as usize] as
                               libc::c_int * -(0xf as libc::c_int)) as u8_0 as
                              u32_0, 0x20 as libc::c_int, 0x40 as libc::c_int)
                as libc::c_uint;
        alpha = ((*this).beamScale * 100.0f32 * 255.0f32) as s16;
        if (*this).actor.params as libc::c_int == 1 as libc::c_int {
            let fresh60 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_0: *mut Gfx = fresh60;
            (*_g_0).words.w0 =
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
            (*_g_0).words.w1 =
                (255 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (255 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (60 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    (alpha as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh61 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_1: *mut Gfx = fresh61;
            (*_g_1).words.w0 =
                (0xfb as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_1).words.w1 =
                (255 as libc::c_int as u32_0 &
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
                    (128 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int
        } else {
            let fresh62 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_2: *mut Gfx = fresh62;
            (*_g_2).words.w0 =
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
            (*_g_2).words.w1 =
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
                    (alpha as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh63 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_3: *mut Gfx = fresh63;
            (*_g_3).words.w0 =
                (0xfb as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_3).words.w1 =
                (100 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (100 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (255 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    (128 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int
        }
        Matrix_Translate((*this).beamOrigin.x, (*this).beamOrigin.y,
                         (*this).beamOrigin.z,
                         MTXMODE_NEW as libc::c_int as u8_0);
        Matrix_RotateY((*this).beamYaw, MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_RotateX((*this).beamPitch,
                       MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_RotateZ((*this).beamRoll,
                       MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_Scale((*this).beamScale, (*this).beamScale,
                     (*this).beamDist * 0.01f32 * 98.0f32 / 20000.0f32,
                     MTXMODE_APPLY as libc::c_int as u8_0);
        let fresh64 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_4: *mut Gfx = fresh64;
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
                (((0x2 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int) ^
                      0x1 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_4).words.w1 =
            Matrix_NewMtx((*globalCtx).state.gfxCtx,
                          b"../z_boss_tw.c\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                          6846 as libc::c_int) as libc::c_uint;
        let fresh65 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_5: *mut Gfx = fresh65;
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
        (*_g_5).words.w1 =
            gSegments[((object_tw_DL_01DDF0.as_mut_ptr() as u32_0) <<
                           4 as libc::c_int >> 28 as libc::c_int) as
                          usize].wrapping_add(object_tw_DL_01DDF0.as_mut_ptr()
                                                  as u32_0 &
                                                  0xffffff as libc::c_int as
                                                      libc::c_uint).wrapping_add(0x80000000
                                                                                     as
                                                                                     libc::c_uint)
                as *mut libc::c_void as libc::c_uint;
        if (*this).beamReflectionDist > 10.0f32 {
            Matrix_Translate((*this).beamReflectionOrigin.x,
                             (*this).beamReflectionOrigin.y,
                             (*this).beamReflectionOrigin.z,
                             MTXMODE_NEW as libc::c_int as u8_0);
            Matrix_RotateY((*this).beamReflectionYaw,
                           MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_RotateX((*this).beamReflectionPitch,
                           MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_RotateZ((*this).beamRoll,
                           MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_Scale((*this).beamScale, (*this).beamScale,
                         (*this).beamReflectionDist * 0.01f32 * 100.0f32 /
                             20000.0f32,
                         MTXMODE_APPLY as libc::c_int as u8_0);
            let fresh66 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_6: *mut Gfx = fresh66;
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
                    (((0x2 as libc::c_int | 0 as libc::c_int |
                           0 as libc::c_int) ^ 0x1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_6).words.w1 =
                Matrix_NewMtx((*globalCtx).state.gfxCtx,
                              b"../z_boss_tw.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              6870 as libc::c_int) as libc::c_uint;
            let fresh67 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_7: *mut Gfx = fresh67;
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
            (*_g_7).words.w1 =
                gSegments[((object_tw_DL_01DDF0.as_mut_ptr() as u32_0) <<
                               4 as libc::c_int >> 28 as libc::c_int) as
                              usize].wrapping_add(object_tw_DL_01DDF0.as_mut_ptr()
                                                      as u32_0 &
                                                      0xffffff as libc::c_int
                                                          as
                                                          libc::c_uint).wrapping_add(0x80000000
                                                                                         as
                                                                                         libc::c_uint)
                    as *mut libc::c_void as libc::c_uint
        }
        Matrix_Pop();
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_boss_tw.c\x00" as *const u8 as
                         *const libc::c_char, 6878 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn func_80943028(mut thisx: *mut Actor,
                                       mut globalCtx: *mut GlobalContext) {
    let mut this: *mut BossTw = thisx as *mut BossTw;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_boss_tw.c\x00" as *const u8 as *const libc::c_char,
                    6885 as libc::c_int);
    Matrix_Push();
    Matrix_Translate((*this).actor.world.pos.x,
                     (*this).actor.world.pos.y + 57.0f32,
                     (*this).actor.world.pos.z,
                     MTXMODE_NEW as libc::c_int as u8_0);
    Matrix_Scale((*this).workf[UNK_F17 as libc::c_int as usize],
                 (*this).workf[UNK_F17 as libc::c_int as usize],
                 (*this).workf[UNK_F17 as libc::c_int as usize],
                 MTXMODE_APPLY as libc::c_int as u8_0);
    let fresh68 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g: *mut Gfx = fresh68;
    (*_g).words.w0 =
        (0xfa as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (0 as libc::c_int as u32_0 &
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
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh69 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_0: *mut Gfx = fresh69;
    (*_g_0).words.w0 =
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
    (*_g_0).words.w1 =
        Matrix_NewMtx((*globalCtx).state.gfxCtx,
                      b"../z_boss_tw.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      6908 as libc::c_int) as libc::c_uint;
    let fresh70 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_1: *mut Gfx = fresh70;
    (*_g_1).words.w0 =
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
    (*_g_1).words.w1 =
        gSegments[((object_tw_DL_01F608.as_mut_ptr() as u32_0) <<
                       4 as libc::c_int >> 28 as libc::c_int) as
                      usize].wrapping_add(object_tw_DL_01F608.as_mut_ptr() as
                                              u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as libc::c_uint;
    func_80094044((*globalCtx).state.gfxCtx);
    let fresh71 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_2: *mut Gfx = fresh71;
    (*_g_2).words.w0 =
        (0xfa as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_2).words.w1 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (200 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    Matrix_Translate((*this).actor.world.pos.x, 240.0f32,
                     (*this).actor.world.pos.z,
                     MTXMODE_NEW as libc::c_int as u8_0);
    Matrix_Scale((*this).actor.scale.x * 4000.0f32 / 100.0f32, 1.0f32,
                 (*this).actor.scale.x * 4000.0f32 / 100.0f32,
                 MTXMODE_APPLY as libc::c_int as u8_0);
    let fresh72 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_3: *mut Gfx = fresh72;
    (*_g_3).words.w0 =
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
    (*_g_3).words.w1 =
        Matrix_NewMtx((*globalCtx).state.gfxCtx,
                      b"../z_boss_tw.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      6926 as libc::c_int) as libc::c_uint;
    let fresh73 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_4: *mut Gfx = fresh73;
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
    (*_g_4).words.w1 =
        gSegments[((gCircleShadowDL.as_mut_ptr() as u32_0) << 4 as libc::c_int
                       >> 28 as libc::c_int) as
                      usize].wrapping_add(gCircleShadowDL.as_mut_ptr() as
                                              u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as libc::c_uint;
    Matrix_Pop();
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_boss_tw.c\x00" as *const u8 as
                         *const libc::c_char, 6933 as libc::c_int);
}
static mut sEyeTextures: [*mut libc::c_void; 3] =
    unsafe {
        [object_tw_Tex_00A438.as_ptr() as *mut _ as *mut libc::c_void,
         object_tw_Tex_00B238.as_ptr() as *mut _ as *mut libc::c_void,
         object_tw_Tex_00B638.as_ptr() as *mut _ as *mut libc::c_void]
    };
#[no_mangle]
pub unsafe extern "C" fn BossTw_Draw(mut thisx: *mut Actor,
                                     mut globalCtx2: *mut GlobalContext) {
    static mut D_8094A9A4: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 200.0f32, z: 2000.0f32,}; init };
    let mut globalCtx: *mut GlobalContext = globalCtx2;
    let mut this: *mut BossTw = thisx as *mut BossTw;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_boss_tw.c\x00" as *const u8 as *const libc::c_char,
                    6947 as libc::c_int);
    if (*this).visible != 0 {
        let fresh74 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g: *mut Gfx = fresh74;
        (*_g).words.w0 =
            (0xdb as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0x6 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                ((10 as libc::c_int * 4 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g).words.w1 =
            gSegments[((sEyeTextures[(*this).eyeTexIdx as usize] as u32_0) <<
                           4 as libc::c_int >> 28 as libc::c_int) as
                          usize].wrapping_add(sEyeTextures[(*this).eyeTexIdx
                                                               as usize] as
                                                  u32_0 &
                                                  0xffffff as libc::c_int as
                                                      libc::c_uint).wrapping_add(0x80000000
                                                                                     as
                                                                                     libc::c_uint)
                as *mut libc::c_void as libc::c_uint;
        let fresh75 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_0: *mut Gfx = fresh75;
        (*_g_0).words.w0 =
            (0xdb as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0x6 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                ((10 as libc::c_int * 4 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_0).words.w1 =
            gSegments[((sEyeTextures[(*this).eyeTexIdx as usize] as u32_0) <<
                           4 as libc::c_int >> 28 as libc::c_int) as
                          usize].wrapping_add(sEyeTextures[(*this).eyeTexIdx
                                                               as usize] as
                                                  u32_0 &
                                                  0xffffff as libc::c_int as
                                                      libc::c_uint).wrapping_add(0x80000000
                                                                                     as
                                                                                     libc::c_uint)
                as *mut libc::c_void as libc::c_uint;
        let fresh76 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_1: *mut Gfx = fresh76;
        (*_g_1).words.w0 =
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
        (*_g_1).words.w1 =
            Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                             ((*this).workf[OUTR_CRWN_TX_X1 as libc::c_int as
                                                usize] as s16 as libc::c_int &
                                  0x7f as libc::c_int) as u32_0,
                             ((*this).workf[OUTR_CRWN_TX_Y1 as libc::c_int as
                                                usize] as s16 as libc::c_int &
                                  0x7f as libc::c_int) as u32_0,
                             0x20 as libc::c_int, 0x20 as libc::c_int,
                             1 as libc::c_int,
                             ((*this).workf[OUTR_CRWN_TX_X2 as libc::c_int as
                                                usize] as s16 as libc::c_int &
                                  0x7f as libc::c_int) as u32_0,
                             ((*this).workf[OUTR_CRWN_TX_Y2 as libc::c_int as
                                                usize] as s16 as libc::c_int &
                                  0xff as libc::c_int) as u32_0,
                             0x20 as libc::c_int, 0x40 as libc::c_int) as
                libc::c_uint;
        if (*this).actor.params as libc::c_int == TW_KOTAKE as libc::c_int {
            let fresh77 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_2: *mut Gfx = fresh77;
            (*_g_2).words.w0 =
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
            (*_g_2).words.w1 =
                Gfx_TexScroll((*globalCtx).state.gfxCtx,
                              ((*this).workf[INNR_CRWN_TX_X1 as libc::c_int as
                                                 usize] as s16 as libc::c_int
                                   & 0x7f as libc::c_int) as u32_0,
                              ((*this).workf[INNR_CRWN_TX_Y1 as libc::c_int as
                                                 usize] as s16 as libc::c_int
                                   & 0xff as libc::c_int) as u32_0,
                              0x20 as libc::c_int, 0x40 as libc::c_int) as
                    libc::c_uint
        } else {
            let fresh78 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_3: *mut Gfx = fresh78;
            (*_g_3).words.w0 =
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
            (*_g_3).words.w1 =
                Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                                 ((*this).workf[INNR_CRWN_TX_X1 as libc::c_int
                                                    as usize] as s16 as
                                      libc::c_int & 0x7f as libc::c_int) as
                                     u32_0,
                                 ((*this).workf[INNR_CRWN_TX_Y1 as libc::c_int
                                                    as usize] as s16 as
                                      libc::c_int & 0x7f as libc::c_int) as
                                     u32_0, 0x20 as libc::c_int,
                                 0x20 as libc::c_int, 1 as libc::c_int,
                                 ((*this).workf[INNR_CRWN_TX_X2 as libc::c_int
                                                    as usize] as s16 as
                                      libc::c_int & 0x7f as libc::c_int) as
                                     u32_0,
                                 ((*this).workf[INNR_CRWN_TX_Y2 as libc::c_int
                                                    as usize] as s16 as
                                      libc::c_int & 0xff as libc::c_int) as
                                     u32_0, 0x20 as libc::c_int,
                                 0x40 as libc::c_int) as libc::c_uint
        }
        func_80093D18((*globalCtx).state.gfxCtx);
        func_80093D84((*globalCtx).state.gfxCtx);
        if (*this).work[FOG_TIMER as libc::c_int as usize] as libc::c_int &
               2 as libc::c_int != 0 {
            (*__gfxCtx).polyOpa.p =
                Gfx_SetFog((*__gfxCtx).polyOpa.p, 255 as libc::c_int,
                           50 as libc::c_int, 0 as libc::c_int,
                           0 as libc::c_int, 900 as libc::c_int,
                           1099 as libc::c_int)
        } else {
            (*__gfxCtx).polyOpa.p =
                Gfx_SetFog((*__gfxCtx).polyOpa.p,
                           (*this).fogR as u32_0 as s32,
                           (*this).fogG as u32_0 as s32,
                           (*this).fogB as u32_0 as s32, 0 as libc::c_int,
                           (*this).fogNear as s32, (*this).fogFar as s32)
        }
        Matrix_Push();
        SkelAnime_DrawFlexOpa(globalCtx, (*this).skelAnime.skeleton,
                              (*this).skelAnime.jointTable,
                              (*this).skelAnime.dListCount as s32,
                              Some(BossTw_OverrideLimbDraw as
                                       unsafe extern "C" fn(_:
                                                                *mut GlobalContext,
                                                            _: s32,
                                                            _: *mut *mut Gfx,
                                                            _: *mut Vec3f,
                                                            _: *mut Vec3s,
                                                            _:
                                                                *mut libc::c_void)
                                           -> s32),
                              Some(BossTw_PostLimbDraw as
                                       unsafe extern "C" fn(_:
                                                                *mut GlobalContext,
                                                            _: s32,
                                                            _: *mut *mut Gfx,
                                                            _: *mut Vec3s,
                                                            _:
                                                                *mut libc::c_void)
                                           -> ()), this as *mut libc::c_void);
        Matrix_Pop();
        (*__gfxCtx).polyOpa.p =
            Gameplay_SetFog(globalCtx, (*__gfxCtx).polyOpa.p)
    }
    if (*this).actor.params as libc::c_int == TW_KOTAKE as libc::c_int {
        if (*this).workf[UNK_F9 as libc::c_int as usize] > 0.0f32 {
            if (*this).workf[UNK_F11 as libc::c_int as usize] > 0.0f32 {
                let mut diff: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                diff.x =
                    (*this).groundBlastPos2.x - (*player).actor.world.pos.x;
                diff.y =
                    (*this).groundBlastPos2.y - (*player).actor.world.pos.y;
                diff.z =
                    (*this).groundBlastPos2.z - (*player).actor.world.pos.z;
                if fabsf(diff.y) < 10.0f32 &&
                       (*player).actor.bgCheckFlags as libc::c_int &
                           1 as libc::c_int != 0 &&
                       sqrtf(diff.x * diff.x + diff.z * diff.z) <
                           (*this).workf[UNK_F12 as libc::c_int as usize] *
                               4600.0f32 &&
                       sFreezeState as libc::c_int == 0 as libc::c_int &&
                       (*this).workf[UNK_F11 as libc::c_int as usize] >
                           200.0f32 {
                    sFreezeState = 1 as libc::c_int as u8_0;
                    (*sTwinrovaPtr).timers[2 as libc::c_int as usize] =
                        100 as libc::c_int as s16
                }
            }
            func_80941BC0(this, globalCtx);
        }
    } else { func_80942180(this, globalCtx); }
    if (*this).visible != 0 {
        if (*this).actionFunc ==
               Some(BossTw_DeathCS as
                        unsafe extern "C" fn(_: *mut BossTw,
                                             _: *mut GlobalContext) -> ()) {
            func_80943028(&mut (*this).actor, globalCtx);
        } else {
            func_809426F0(this, globalCtx);
            Matrix_MultVec3f(&mut D_8094A9A4, &mut (*this).beamOrigin);
            func_80942C70(&mut (*this).actor, globalCtx);
        }
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_boss_tw.c\x00" as *const u8 as
                         *const libc::c_char, 7123 as libc::c_int);
}
#[no_mangle]
pub static mut D_8094A9B0: [*mut libc::c_void; 3] =
    unsafe {
        [object_tw_Tex_02A9B0.as_ptr() as *mut _ as *mut libc::c_void,
         object_tw_Tex_02A070.as_ptr() as *mut _ as *mut libc::c_void,
         object_tw_Tex_02A470.as_ptr() as *mut _ as *mut libc::c_void]
    };
#[no_mangle]
pub unsafe extern "C" fn BossTw_TwinrovaOverrideLimbDraw(mut globalCtx:
                                                             *mut GlobalContext,
                                                         mut limbIndex: s32,
                                                         mut dList:
                                                             *mut *mut Gfx,
                                                         mut pos: *mut Vec3f,
                                                         mut rot: *mut Vec3s,
                                                         mut thisx:
                                                             *mut libc::c_void)
 -> s32 {
    let mut this: *mut BossTw = thisx as *mut BossTw;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_boss_tw.c\x00" as *const u8 as *const libc::c_char,
                    7139 as libc::c_int);
    match limbIndex {
        21 => {
            let fresh79 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g: *mut Gfx = fresh79;
            (*_g).words.w0 =
                (0xdb as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0x6 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    ((0xc as libc::c_int * 4 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g).words.w1 =
                Gfx_TexScroll((*globalCtx).state.gfxCtx,
                              0 as libc::c_int as u32_0,
                              (*this).work[CS_TIMER_1 as libc::c_int as usize]
                                  as f32_0 as s16 as u32_0, 8 as libc::c_int,
                              8 as libc::c_int) as libc::c_uint;
            let fresh80 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_0: *mut Gfx = fresh80;
            (*_g_0).words.w0 =
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
            (*_g_0).words.w1 =
                gSegments[((D_8094A9B0[(*this).eyeTexIdx as usize] as u32_0)
                               << 4 as libc::c_int >> 28 as libc::c_int) as
                              usize].wrapping_add(D_8094A9B0[(*this).eyeTexIdx
                                                                 as usize] as
                                                      u32_0 &
                                                      0xffffff as libc::c_int
                                                          as
                                                          libc::c_uint).wrapping_add(0x80000000
                                                                                         as
                                                                                         libc::c_uint)
                    as *mut libc::c_void as libc::c_uint;
            let fresh81 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_1: *mut Gfx = fresh81;
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
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_1).words.w1 =
                gSegments[((D_8094A9B0[(*this).leftEyeTexIdx as usize] as
                                u32_0) << 4 as libc::c_int >>
                               28 as libc::c_int) as
                              usize].wrapping_add(D_8094A9B0[(*this).leftEyeTexIdx
                                                                 as usize] as
                                                      u32_0 &
                                                      0xffffff as libc::c_int
                                                          as
                                                          libc::c_uint).wrapping_add(0x80000000
                                                                                         as
                                                                                         libc::c_uint)
                    as *mut libc::c_void as libc::c_uint;
            let fresh82 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_2: *mut Gfx = fresh82;
            (*_g_2).words.w0 =
                (0xfb as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_2).words.w1 =
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
                    ((*this).work[UNK_S8 as libc::c_int as usize] as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int
        }
        17 | 41 => {
            *dList = 0 as *mut Gfx;
            let fresh83 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_3: *mut Gfx = fresh83;
            (*_g_3).words.w0 =
                (0xdb as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0x6 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    ((0xa as libc::c_int * 4 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_3).words.w1 =
                Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                                 0 as libc::c_int as u32_0,
                                 0 as libc::c_int as u32_0,
                                 0x20 as libc::c_int, 0x20 as libc::c_int,
                                 1 as libc::c_int, 0 as libc::c_int as u32_0,
                                 (-((*this).work[CS_TIMER_1 as libc::c_int as
                                                     usize] as libc::c_int) *
                                      0xf as libc::c_int) as u32_0,
                                 0x20 as libc::c_int, 0x40 as libc::c_int) as
                    libc::c_uint
        }
        18 | 42 => {
            *dList = 0 as *mut Gfx;
            let fresh84 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_4: *mut Gfx = fresh84;
            (*_g_4).words.w0 =
                (0xdb as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0x6 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    ((0xb as libc::c_int * 4 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_4).words.w1 =
                Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                                 0 as libc::c_int as u32_0,
                                 0 as libc::c_int as u32_0,
                                 0x20 as libc::c_int, 0x20 as libc::c_int,
                                 1 as libc::c_int, 0 as libc::c_int as u32_0,
                                 (-((*this).work[CS_TIMER_1 as libc::c_int as
                                                     usize] as libc::c_int) *
                                      0xa as libc::c_int) as u32_0,
                                 0x20 as libc::c_int, 0x40 as libc::c_int) as
                    libc::c_uint
        }
        16 | 32 => {
            *dList = 0 as *mut Gfx;
            let fresh85 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_5: *mut Gfx = fresh85;
            (*_g_5).words.w0 =
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
            (*_g_5).words.w1 =
                Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                                 0 as libc::c_int as u32_0,
                                 0 as libc::c_int as u32_0,
                                 0x20 as libc::c_int, 0x20 as libc::c_int,
                                 1 as libc::c_int,
                                 (*this).work[CS_TIMER_1 as libc::c_int as
                                                  usize] as u32_0,
                                 (-((*this).work[CS_TIMER_1 as libc::c_int as
                                                     usize] as libc::c_int) *
                                      7 as libc::c_int) as u32_0,
                                 0x20 as libc::c_int, 0x40 as libc::c_int) as
                    libc::c_uint
        }
        15 | 31 => {
            *dList = 0 as *mut Gfx;
            let fresh86 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_6: *mut Gfx = fresh86;
            (*_g_6).words.w0 =
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
            (*_g_6).words.w1 =
                Gfx_TexScroll((*globalCtx).state.gfxCtx,
                              0 as libc::c_int as u32_0,
                              (*this).work[CS_TIMER_1 as libc::c_int as usize]
                                  as u32_0, 0x20 as libc::c_int,
                              0x40 as libc::c_int) as libc::c_uint
        }
        19 => {
            if (*this).unk_5F8 as libc::c_int != 0 as libc::c_int {
                *dList = object_tw_DL_02D940.as_mut_ptr()
            }
        }
        20 => {
            if (*this).unk_5F8 as libc::c_int != 0 as libc::c_int {
                *dList = object_tw_DL_02D890.as_mut_ptr()
            }
        }
        _ => { }
    }
    if (*this).unk_5F8 as libc::c_int != 0 as libc::c_int &&
           (limbIndex == 34 as libc::c_int || limbIndex == 40 as libc::c_int)
       {
        *dList = 0 as *mut Gfx
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_boss_tw.c\x00" as *const u8 as
                         *const libc::c_char, 7251 as libc::c_int);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_TwinrovaPostLimbDraw(mut globalCtx:
                                                         *mut GlobalContext,
                                                     mut limbIndex: s32,
                                                     mut dList: *mut *mut Gfx,
                                                     mut rot: *mut Vec3s,
                                                     mut thisx:
                                                         *mut libc::c_void) {
    static mut D_8094A9BC: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
    static mut D_8094A9C8: Vec3f =
        {
            let mut init = Vec3f{x: 0.0f32, y: 2000.0f32, z: -2000.0f32,};
            init
        };
    static mut D_8094A9D4: Vec3f =
        { let mut init = Vec3f{x: 13000.0f32, y: 0.0f32, z: 0.0f32,}; init };
    static mut D_8094A9E0: Vec3f =
        { let mut init = Vec3f{x: 13000.0f32, y: 0.0f32, z: 0.0f32,}; init };
    let mut this: *mut BossTw = thisx as *mut BossTw;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_boss_tw.c\x00" as *const u8 as *const libc::c_char,
                    7262 as libc::c_int);
    match limbIndex {
        34 => {
            Matrix_MultVec3f(&mut D_8094A9D4, &mut (*this).leftScepterPos);
        }
        40 => {
            Matrix_MultVec3f(&mut D_8094A9E0, &mut (*this).rightScepterPos);
        }
        21 => {
            Matrix_MultVec3f(&mut D_8094A9BC, &mut (*this).actor.focus.pos);
            Matrix_MultVec3f(&mut D_8094A9C8, &mut (*this).crownPos);
        }
        15 | 16 | 17 | 18 | 31 | 32 | 41 | 42 => {
            Matrix_Push();
            Matrix_Scale((*this).workf[UNK_F12 as libc::c_int as usize],
                         (*this).workf[UNK_F12 as libc::c_int as usize],
                         (*this).workf[UNK_F12 as libc::c_int as usize],
                         MTXMODE_APPLY as libc::c_int as u8_0);
            let fresh87 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g: *mut Gfx = fresh87;
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
                    (((0x2 as libc::c_int | 0 as libc::c_int |
                           0 as libc::c_int) ^ 0x1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g).words.w1 =
                Matrix_NewMtx((*globalCtx).state.gfxCtx,
                              b"../z_boss_tw.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              7295 as libc::c_int) as libc::c_uint;
            Matrix_Pop();
            let fresh88 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_0: *mut Gfx = fresh88;
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
            (*_g_0).words.w1 = *dList as libc::c_uint
        }
        _ => { }
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_boss_tw.c\x00" as *const u8 as
                         *const libc::c_char, 7301 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_ShieldChargeDraw(mut this: *mut BossTw,
                                                 mut globalCtx:
                                                     *mut GlobalContext) {
    let mut pad: s32 = 0;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut temp_t0: s16 = 0;
    let mut temp_a0: s16 = 0;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_boss_tw.c\x00" as *const u8 as *const libc::c_char,
                    7311 as libc::c_int);
    Matrix_Push();
    temp_t0 =
        (sShieldFireCharge as libc::c_int | sShieldIceCharge as libc::c_int)
            as s16;
    if temp_t0 as libc::c_int == 1 as libc::c_int {
        func_80078884((0x1832 as libc::c_int & !(0x800 as libc::c_int)) as
                          u16_0);
    } else if temp_t0 as libc::c_int == 2 as libc::c_int {
        func_80078884((0x1833 as libc::c_int & !(0x800 as libc::c_int)) as
                          u16_0);
    } else if temp_t0 as libc::c_int == 3 as libc::c_int {
        func_80078884((0x1834 as libc::c_int & !(0x800 as libc::c_int)) as
                          u16_0);
    }
    if temp_t0 as libc::c_int != 0 as libc::c_int &&
           (temp_t0 as libc::c_int) < 4 as libc::c_int {
        Math_ApproachF(&mut D_8094C854, 255.0f32, 1.0f32, 20.0f32);
        if temp_t0 as libc::c_int == 3 as libc::c_int {
            temp_t0 = (temp_t0 as libc::c_int * 3 as libc::c_int) as s16
        }
    } else if temp_t0 as libc::c_int == 0 as libc::c_int {
        D_8094C854 = 0.0f32
    } else {
        Math_ApproachF(&mut D_8094C854, 0.0f32, 1.0f32, 10.0f32);
        if D_8094C854 == 0.0f32 {
            sShieldIceCharge = 0 as libc::c_int as u8_0;
            sShieldFireCharge = 0 as libc::c_int as u8_0
        }
        temp_t0 = 1 as libc::c_int as s16
    }
    if Player_HasMirrorShieldEquipped(globalCtx) != 0 {
        if temp_t0 as libc::c_int != 0 as libc::c_int {
            Matrix_Mult(&mut (*player).shieldMf,
                        MTXMODE_NEW as libc::c_int as u8_0);
            Matrix_RotateX(3.14159265358979323846f32 / 2.0f32,
                           MTXMODE_APPLY as libc::c_int as u8_0);
            let fresh89 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g: *mut Gfx = fresh89;
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
                    (((0x2 as libc::c_int | 0 as libc::c_int |
                           0 as libc::c_int) ^ 0x1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g).words.w1 =
                Matrix_NewMtx((*globalCtx).state.gfxCtx,
                              b"../z_boss_tw.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              7362 as libc::c_int) as libc::c_uint;
            temp_a0 =
                (Math_SinS(((*this).work[CS_TIMER_1 as libc::c_int as usize]
                                as libc::c_int * 2730 as libc::c_int *
                                temp_t0 as libc::c_int) as s16) * D_8094C854 *
                     0.5f32 + D_8094C854 * 0.5f32) as s16;
            if sShieldFireCharge as libc::c_int != 0 as libc::c_int {
                let fresh90 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_0: *mut Gfx = fresh90;
                (*_g_0).words.w0 =
                    (0xfb as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int;
                (*_g_0).words.w1 =
                    (255 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (245 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            16 as libc::c_int |
                        (255 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            8 as libc::c_int |
                        (temp_a0 as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                let fresh91 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_1: *mut Gfx = fresh91;
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
                    gSegments[((object_tw_DL_01E0E0.as_mut_ptr() as u32_0) <<
                                   4 as libc::c_int >> 28 as libc::c_int) as
                                  usize].wrapping_add(object_tw_DL_01E0E0.as_mut_ptr()
                                                          as u32_0 &
                                                          0xffffff as
                                                              libc::c_int as
                                                              libc::c_uint).wrapping_add(0x80000000
                                                                                             as
                                                                                             libc::c_uint)
                        as *mut libc::c_void as libc::c_uint;
                let fresh92 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_2: *mut Gfx = fresh92;
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
                    Gfx_TwoTexScroll((*globalCtx).state.gfxCtx,
                                     0 as libc::c_int,
                                     ((*this).work[CS_TIMER_1 as libc::c_int
                                                       as usize] as
                                          libc::c_int * 2 as libc::c_int *
                                          temp_t0 as libc::c_int) as u32_0,
                                     0 as libc::c_int as u32_0,
                                     0x20 as libc::c_int, 0x20 as libc::c_int,
                                     1 as libc::c_int,
                                     (-((*this).work[CS_TIMER_1 as libc::c_int
                                                         as usize] as
                                            libc::c_int) * 2 as libc::c_int *
                                          temp_t0 as libc::c_int) as u32_0,
                                     0 as libc::c_int as u32_0,
                                     0x20 as libc::c_int, 0x20 as libc::c_int)
                        as libc::c_uint;
                let fresh93 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_3: *mut Gfx = fresh93;
                (*_g_3).words.w0 =
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
                (*_g_3).words.w1 =
                    (100 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (20 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            16 as libc::c_int |
                        (0 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            8 as libc::c_int |
                        (D_8094C854 as s16 as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                let fresh94 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_4: *mut Gfx = fresh94;
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
                    gSegments[((object_tw_DL_01E020.as_mut_ptr() as u32_0) <<
                                   4 as libc::c_int >> 28 as libc::c_int) as
                                  usize].wrapping_add(object_tw_DL_01E020.as_mut_ptr()
                                                          as u32_0 &
                                                          0xffffff as
                                                              libc::c_int as
                                                              libc::c_uint).wrapping_add(0x80000000
                                                                                             as
                                                                                             libc::c_uint)
                        as *mut libc::c_void as libc::c_uint
            } else {
                let fresh95 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_5: *mut Gfx = fresh95;
                (*_g_5).words.w0 =
                    (0xfb as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int;
                (*_g_5).words.w1 =
                    (225 as libc::c_int as u32_0 &
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
                        (temp_a0 as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                let fresh96 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_6: *mut Gfx = fresh96;
                (*_g_6).words.w0 =
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
                (*_g_6).words.w1 =
                    gSegments[((object_tw_DL_01E3A0.as_mut_ptr() as u32_0) <<
                                   4 as libc::c_int >> 28 as libc::c_int) as
                                  usize].wrapping_add(object_tw_DL_01E3A0.as_mut_ptr()
                                                          as u32_0 &
                                                          0xffffff as
                                                              libc::c_int as
                                                              libc::c_uint).wrapping_add(0x80000000
                                                                                             as
                                                                                             libc::c_uint)
                        as *mut libc::c_void as libc::c_uint;
                let fresh97 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_7: *mut Gfx = fresh97;
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
                                     (-((*this).work[CS_TIMER_1 as libc::c_int
                                                         as usize] as
                                            libc::c_int) * 5 as libc::c_int *
                                          temp_t0 as libc::c_int) as u32_0,
                                     0x20 as libc::c_int, 0x40 as libc::c_int,
                                     1 as libc::c_int,
                                     ((*this).work[CS_TIMER_1 as libc::c_int
                                                       as usize] as
                                          libc::c_int * 4 as libc::c_int *
                                          temp_t0 as libc::c_int) as u32_0,
                                     0 as libc::c_int as u32_0,
                                     0x20 as libc::c_int, 0x20 as libc::c_int)
                        as libc::c_uint;
                let fresh98 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_8: *mut Gfx = fresh98;
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
                    (175 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (205 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            16 as libc::c_int |
                        (195 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            8 as libc::c_int |
                        (D_8094C854 as s16 as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                let fresh99 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_9: *mut Gfx = fresh99;
                (*_g_9).words.w0 =
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
                (*_g_9).words.w1 =
                    gSegments[((object_tw_DL_01E2C0.as_mut_ptr() as u32_0) <<
                                   4 as libc::c_int >> 28 as libc::c_int) as
                                  usize].wrapping_add(object_tw_DL_01E2C0.as_mut_ptr()
                                                          as u32_0 &
                                                          0xffffff as
                                                              libc::c_int as
                                                              libc::c_uint).wrapping_add(0x80000000
                                                                                             as
                                                                                             libc::c_uint)
                        as *mut libc::c_void as libc::c_uint
            }
        }
    }
    if D_8094C86F as libc::c_int != 0 as libc::c_int {
        let mut step: f32_0 =
            if D_8094C872 as libc::c_int > 0 as libc::c_int {
                100.0f32
            } else { 60.0f32 };
        D_8094C86F = D_8094C86F.wrapping_sub(1);
        Math_ApproachF(&mut D_8094C858, 255.0f32, 1.0f32, step);
    } else {
        let mut step_0: f32_0 =
            if D_8094C872 as libc::c_int > 0 as libc::c_int {
                40.0f32
            } else { 20.0f32 };
        Math_ApproachF(&mut D_8094C858, 0.0f32, 1.0f32, step_0);
    }
    if Player_HasMirrorShieldEquipped(globalCtx) != 0 && D_8094C858 > 0.0f32 {
        let mut scale: f32_0 =
            if D_8094C872 as libc::c_int > 0 as libc::c_int {
                1.3f32
            } else { 1.0f32 };
        Matrix_Mult(&mut (*player).shieldMf,
                    MTXMODE_NEW as libc::c_int as u8_0);
        Matrix_RotateX(3.14159265358979323846f32 / 2.0f32,
                       MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_Scale(scale, scale, scale,
                     MTXMODE_APPLY as libc::c_int as u8_0);
        let fresh100 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_10: *mut Gfx = fresh100;
        (*_g_10).words.w0 =
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
                (((0x2 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int) ^
                      0x1 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_10).words.w1 =
            Matrix_NewMtx((*globalCtx).state.gfxCtx,
                          b"../z_boss_tw.c\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                          7486 as libc::c_int) as libc::c_uint;
        if sShieldFireCharge as libc::c_int != 0 as libc::c_int {
            let fresh101 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_11: *mut Gfx = fresh101;
            (*_g_11).words.w0 =
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
            (*_g_11).words.w1 =
                (255 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (220 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (20 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    (D_8094C858 as s16 as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh102 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_12: *mut Gfx = fresh102;
            (*_g_12).words.w0 =
                (0xfb as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_12).words.w1 =
                (255 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (20 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    (110 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int
        } else {
            let fresh103 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_13: *mut Gfx = fresh103;
            (*_g_13).words.w0 =
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
            (*_g_13).words.w1 =
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
                    (D_8094C858 as s16 as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh104 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_14: *mut Gfx = fresh104;
            (*_g_14).words.w0 =
                (0xfb as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_14).words.w1 =
                (185 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (225 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (205 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    (150 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int
        }
        let fresh105 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_15: *mut Gfx = fresh105;
        (*_g_15).words.w0 =
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
        (*_g_15).words.w1 =
            Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                             0 as libc::c_int as u32_0,
                             ((*this).work[CS_TIMER_1 as libc::c_int as usize]
                                  as libc::c_int * D_8094C872 as libc::c_int)
                                 as u32_0, 0x20 as libc::c_int,
                             0x40 as libc::c_int, 1 as libc::c_int,
                             0 as libc::c_int as u32_0,
                             ((*this).work[CS_TIMER_1 as libc::c_int as usize]
                                  as libc::c_int * D_8094C872 as libc::c_int)
                                 as u32_0, 0x20 as libc::c_int,
                             0x20 as libc::c_int) as libc::c_uint;
        let fresh106 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_16: *mut Gfx = fresh106;
        (*_g_16).words.w0 =
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
        (*_g_16).words.w1 =
            gSegments[((object_tw_DL_01E9F0.as_mut_ptr() as u32_0) <<
                           4 as libc::c_int >> 28 as libc::c_int) as
                          usize].wrapping_add(object_tw_DL_01E9F0.as_mut_ptr()
                                                  as u32_0 &
                                                  0xffffff as libc::c_int as
                                                      libc::c_uint).wrapping_add(0x80000000
                                                                                     as
                                                                                     libc::c_uint)
                as *mut libc::c_void as libc::c_uint
    }
    Matrix_Pop();
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_boss_tw.c\x00" as *const u8 as
                         *const libc::c_char, 7531 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_SpawnPortalDraw(mut this: *mut BossTw,
                                                mut globalCtx:
                                                    *mut GlobalContext) {
    let mut pad: s32 = 0;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_boss_tw.c\x00" as *const u8 as *const libc::c_char,
                    7546 as libc::c_int);
    func_80093D84((*globalCtx).state.gfxCtx);
    let fresh107 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g: *mut Gfx = fresh107;
    (*_g).words.w0 =
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
    (*_g).words.w1 =
        Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                         0 as libc::c_int as u32_0,
                         (-((*this).work[CS_TIMER_1 as libc::c_int as usize]
                                as libc::c_int) * 15 as libc::c_int) as u32_0,
                         0x20 as libc::c_int, 0x40 as libc::c_int,
                         1 as libc::c_int, 0 as libc::c_int as u32_0,
                         0 as libc::c_int as u32_0, 0x40 as libc::c_int,
                         0x40 as libc::c_int) as libc::c_uint;
    Matrix_Push();
    Matrix_Translate(0.0f32, 232.0f32, -600.0f32,
                     MTXMODE_NEW as libc::c_int as u8_0);
    Matrix_Scale((*this).spawnPortalScale, (*this).spawnPortalScale,
                 (*this).spawnPortalScale,
                 MTXMODE_APPLY as libc::c_int as u8_0);
    let fresh108 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_0: *mut Gfx = fresh108;
    (*_g_0).words.w0 =
        (0xfa as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_0).words.w1 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            ((*this).spawnPortalAlpha as s16 as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh109 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_1: *mut Gfx = fresh109;
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
            (((0x2 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int) ^
                  0x1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_1).words.w1 =
        Matrix_NewMtx((*globalCtx).state.gfxCtx,
                      b"../z_boss_tw.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      7582 as libc::c_int) as libc::c_uint;
    let fresh110 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_2: *mut Gfx = fresh110;
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
    (*_g_2).words.w1 =
        gSegments[((object_tw_DL_01EC68.as_mut_ptr() as u32_0) <<
                       4 as libc::c_int >> 28 as libc::c_int) as
                      usize].wrapping_add(object_tw_DL_01EC68.as_mut_ptr() as
                                              u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as libc::c_uint;
    let fresh111 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_3: *mut Gfx = fresh111;
    (*_g_3).words.w0 =
        (0xfa as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_3).words.w1 =
        (135 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (175 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (165 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            ((*this).spawnPortalAlpha as s16 as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    Matrix_Translate(0.0f32, 2.0f32, 0.0f32,
                     MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_RotateX(3.14159265358979323846f32 / 2.0f32,
                   MTXMODE_APPLY as libc::c_int as u8_0);
    let fresh112 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_4: *mut Gfx = fresh112;
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
            (((0x2 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int) ^
                  0x1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_4).words.w1 =
        Matrix_NewMtx((*globalCtx).state.gfxCtx,
                      b"../z_boss_tw.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      7596 as libc::c_int) as libc::c_uint;
    let fresh113 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_5: *mut Gfx = fresh113;
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
    (*_g_5).words.w1 =
        gSegments[((object_tw_DL_01CEE0.as_mut_ptr() as u32_0) <<
                       4 as libc::c_int >> 28 as libc::c_int) as
                      usize].wrapping_add(object_tw_DL_01CEE0.as_mut_ptr() as
                                              u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as libc::c_uint;
    Matrix_Translate(0.0f32, 232.0f32, 600.0f32,
                     MTXMODE_NEW as libc::c_int as u8_0);
    Matrix_Scale((*this).spawnPortalScale, (*this).spawnPortalScale,
                 (*this).spawnPortalScale,
                 MTXMODE_APPLY as libc::c_int as u8_0);
    let fresh114 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_6: *mut Gfx = fresh114;
    (*_g_6).words.w0 =
        (0xfa as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_6).words.w1 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            ((*this).spawnPortalAlpha as s16 as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh115 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_7: *mut Gfx = fresh115;
    (*_g_7).words.w0 =
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
            (((0x2 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int) ^
                  0x1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_7).words.w1 =
        Matrix_NewMtx((*globalCtx).state.gfxCtx,
                      b"../z_boss_tw.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      7617 as libc::c_int) as libc::c_uint;
    let fresh116 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_8: *mut Gfx = fresh116;
    (*_g_8).words.w0 =
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
    (*_g_8).words.w1 =
        gSegments[((object_tw_DL_01EC68.as_mut_ptr() as u32_0) <<
                       4 as libc::c_int >> 28 as libc::c_int) as
                      usize].wrapping_add(object_tw_DL_01EC68.as_mut_ptr() as
                                              u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as libc::c_uint;
    let fresh117 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_9: *mut Gfx = fresh117;
    (*_g_9).words.w0 =
        (0xfa as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_9).words.w1 =
        (255 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            ((*this).spawnPortalAlpha as s16 as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    Matrix_Translate(0.0f32, 2.0f32, 0.0f32,
                     MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_RotateX(3.14159265358979323846f32 / 2.0f32,
                   MTXMODE_APPLY as libc::c_int as u8_0);
    let fresh118 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_10: *mut Gfx = fresh118;
    (*_g_10).words.w0 =
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
            (((0x2 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int) ^
                  0x1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_10).words.w1 =
        Matrix_NewMtx((*globalCtx).state.gfxCtx,
                      b"../z_boss_tw.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      7631 as libc::c_int) as libc::c_uint;
    let fresh119 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_11: *mut Gfx = fresh119;
    (*_g_11).words.w0 =
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
    (*_g_11).words.w1 =
        gSegments[((object_tw_DL_01DBE8.as_mut_ptr() as u32_0) <<
                       4 as libc::c_int >> 28 as libc::c_int) as
                      usize].wrapping_add(object_tw_DL_01DBE8.as_mut_ptr() as
                                              u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as libc::c_uint;
    Matrix_Pop();
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_boss_tw.c\x00" as *const u8 as
                         *const libc::c_char, 7635 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn func_80944C50(mut this: *mut BossTw,
                                       mut globalCtx: *mut GlobalContext) {
    let mut pad: s32 = 0;
    let mut scale: f32_0 = 0.;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_boss_tw.c\x00" as *const u8 as *const libc::c_char,
                    7645 as libc::c_int);
    Matrix_Push();
    Matrix_Translate(0.0f32, 750.0f32, 0.0f32,
                     MTXMODE_NEW as libc::c_int as u8_0);
    Matrix_Scale(0.35f32, 0.35f32, 0.35f32,
                 MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_Push();
    Matrix_Scale((*this).workf[UNK_F19 as libc::c_int as usize],
                 (*this).workf[UNK_F19 as libc::c_int as usize],
                 (*this).workf[UNK_F19 as libc::c_int as usize],
                 MTXMODE_APPLY as libc::c_int as u8_0);
    let fresh120 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g: *mut Gfx = fresh120;
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
            (((0x2 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int) ^
                  0x1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g).words.w1 =
        Matrix_NewMtx((*globalCtx).state.gfxCtx,
                      b"../z_boss_tw.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      7671 as libc::c_int) as libc::c_uint;
    let fresh121 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_0: *mut Gfx = fresh121;
    (*_g_0).words.w0 =
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
    (*_g_0).words.w1 =
        gSegments[((object_tw_DL_01F390.as_mut_ptr() as u32_0) <<
                       4 as libc::c_int >> 28 as libc::c_int) as
                      usize].wrapping_add(object_tw_DL_01F390.as_mut_ptr() as
                                              u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as libc::c_uint;
    Matrix_Pop();
    let fresh122 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_1: *mut Gfx = fresh122;
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
                         (-((*sKoumePtr).work[CS_TIMER_1 as libc::c_int as
                                                  usize] as libc::c_int) *
                              2 as libc::c_int) as u32_0,
                         0 as libc::c_int as u32_0, 0x20 as libc::c_int,
                         0x20 as libc::c_int, 1 as libc::c_int,
                         (-((*sKoumePtr).work[CS_TIMER_1 as libc::c_int as
                                                  usize] as libc::c_int) *
                              2 as libc::c_int) as u32_0,
                         0 as libc::c_int as u32_0, 0x20 as libc::c_int,
                         0x40 as libc::c_int) as libc::c_uint;
    let fresh123 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_2: *mut Gfx = fresh123;
    (*_g_2).words.w0 =
        (0xfa as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_2).words.w1 =
        (255 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (((*this).workf[UNK_F18 as libc::c_int as usize] as s16 as
                  libc::c_int / 2 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh124 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_3: *mut Gfx = fresh124;
    (*_g_3).words.w0 =
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
            (((0x2 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int) ^
                  0x1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_3).words.w1 =
        Matrix_NewMtx((*globalCtx).state.gfxCtx,
                      b"../z_boss_tw.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      7694 as libc::c_int) as libc::c_uint;
    let fresh125 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_4: *mut Gfx = fresh125;
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
    (*_g_4).words.w1 =
        gSegments[((object_tw_DL_01F238.as_mut_ptr() as u32_0) <<
                       4 as libc::c_int >> 28 as libc::c_int) as
                      usize].wrapping_add(object_tw_DL_01F238.as_mut_ptr() as
                                              u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as libc::c_uint;
    let fresh126 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_5: *mut Gfx = fresh126;
    (*_g_5).words.w0 =
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
    (*_g_5).words.w1 =
        Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                         (-((*sKoumePtr).work[CS_TIMER_1 as libc::c_int as
                                                  usize] as libc::c_int) *
                              5 as libc::c_int) as u32_0,
                         (-((*sKoumePtr).work[CS_TIMER_1 as libc::c_int as
                                                  usize] as libc::c_int) *
                              2 as libc::c_int) as u32_0, 0x20 as libc::c_int,
                         0x40 as libc::c_int, 1 as libc::c_int,
                         0 as libc::c_int as u32_0,
                         (-((*sKoumePtr).work[CS_TIMER_1 as libc::c_int as
                                                  usize] as libc::c_int) *
                              2 as libc::c_int) as u32_0, 0x10 as libc::c_int,
                         0x10 as libc::c_int) as libc::c_uint;
    let fresh127 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_6: *mut Gfx = fresh127;
    (*_g_6).words.w0 =
        (0xfa as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_6).words.w1 =
        (255 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (((*this).workf[UNK_F18 as libc::c_int as usize] * 0.3f32) as s16
                 as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    scale = (*this).workf[UNK_F18 as libc::c_int as usize] / 150.0f32;
    scale = if scale > 1.0f32 { 1.0f32 } else { scale };
    Matrix_Scale(scale, 1.0f32, scale, MTXMODE_APPLY as libc::c_int as u8_0);
    let fresh128 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_7: *mut Gfx = fresh128;
    (*_g_7).words.w0 =
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
            (((0x2 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int) ^
                  0x1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_7).words.w1 =
        Matrix_NewMtx((*globalCtx).state.gfxCtx,
                      b"../z_boss_tw.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      7728 as libc::c_int) as libc::c_uint;
    let fresh129 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_8: *mut Gfx = fresh129;
    (*_g_8).words.w0 =
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
    (*_g_8).words.w1 =
        gSegments[((object_tw_DL_01EEB0.as_mut_ptr() as u32_0) <<
                       4 as libc::c_int >> 28 as libc::c_int) as
                      usize].wrapping_add(object_tw_DL_01EEB0.as_mut_ptr() as
                                              u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as libc::c_uint;
    Matrix_Pop();
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_boss_tw.c\x00" as *const u8 as
                         *const libc::c_char, 7732 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_TwinrovaDraw(mut thisx: *mut Actor,
                                             mut globalCtx2:
                                                 *mut GlobalContext) {
    static mut D_8094A9EC: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 200.0f32, z: 2000.0f32,}; init };
    let mut globalCtx: *mut GlobalContext = globalCtx2;
    let mut this: *mut BossTw = thisx as *mut BossTw;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_boss_tw.c\x00" as *const u8 as *const libc::c_char,
                    7748 as libc::c_int);
    if (*this).visible != 0 {
        func_80093D18((*globalCtx).state.gfxCtx);
        func_80093D84((*globalCtx).state.gfxCtx);
        (*__gfxCtx).polyOpa.p =
            if (*this).work[FOG_TIMER as libc::c_int as usize] as libc::c_int
                   & 2 as libc::c_int != 0 {
                Gfx_SetFog2((*__gfxCtx).polyOpa.p, 255 as libc::c_int,
                            50 as libc::c_int, 0 as libc::c_int,
                            0 as libc::c_int, 900 as libc::c_int,
                            1099 as libc::c_int)
            } else {
                Gfx_SetFog2((*__gfxCtx).polyOpa.p,
                            (*this).fogR as u32_0 as s32,
                            (*this).fogG as u32_0 as s32,
                            (*this).fogB as u32_0 as s32, 0 as libc::c_int,
                            (*this).fogNear as s32, (*this).fogFar as s32)
            };
        Matrix_Push();
        SkelAnime_DrawFlexOpa(globalCtx, (*this).skelAnime.skeleton,
                              (*this).skelAnime.jointTable,
                              (*this).skelAnime.dListCount as s32,
                              Some(BossTw_TwinrovaOverrideLimbDraw as
                                       unsafe extern "C" fn(_:
                                                                *mut GlobalContext,
                                                            _: s32,
                                                            _: *mut *mut Gfx,
                                                            _: *mut Vec3f,
                                                            _: *mut Vec3s,
                                                            _:
                                                                *mut libc::c_void)
                                           -> s32),
                              Some(BossTw_TwinrovaPostLimbDraw as
                                       unsafe extern "C" fn(_:
                                                                *mut GlobalContext,
                                                            _: s32,
                                                            _: *mut *mut Gfx,
                                                            _: *mut Vec3s,
                                                            _:
                                                                *mut libc::c_void)
                                           -> ()),
                              thisx as *mut libc::c_void);
        Matrix_Pop();
        Matrix_MultVec3f(&mut D_8094A9EC, &mut (*this).beamOrigin);
        (*__gfxCtx).polyOpa.p =
            Gfx_SetFog2((*__gfxCtx).polyOpa.p,
                        (*globalCtx).lightCtx.fogColor[0 as libc::c_int as
                                                           usize] as s32,
                        (*globalCtx).lightCtx.fogColor[1 as libc::c_int as
                                                           usize] as s32,
                        (*globalCtx).lightCtx.fogColor[2 as libc::c_int as
                                                           usize] as s32,
                        0 as libc::c_int,
                        (*globalCtx).lightCtx.fogNear as s32,
                        1000 as libc::c_int)
    }
    BossTw_DrawEffects(globalCtx);
    BossTw_ShieldChargeDraw(this, globalCtx);
    if (*this).spawnPortalAlpha > 0.0f32 {
        BossTw_SpawnPortalDraw(this, globalCtx);
    }
    if (*this).workf[UNK_F18 as libc::c_int as usize] > 0.0f32 {
        func_80944C50(this, globalCtx);
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_boss_tw.c\x00" as *const u8 as
                         *const libc::c_char, 7804 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_BlastFire(mut this: *mut BossTw,
                                          mut globalCtx: *mut GlobalContext) {
    let mut i: s16 = 0;
    let mut xDiff: f32_0 = 0.;
    let mut yDiff: f32_0 = 0.;
    let mut zDiff: f32_0 = 0.;
    let mut distXZ: f32_0 = 0.;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut player2: *mut Player = player;
    match (*this).actor.params as libc::c_int {
        100 => {
            let mut current_block_89: u64;
            match (*this).csState1 as libc::c_int {
                0 => {
                    Actor_SetScale(&mut (*this).actor, 0.03f32);
                    (*this).csState1 = 1 as libc::c_int as s16;
                    xDiff =
                        (*player).actor.world.pos.x -
                            (*this).actor.world.pos.x;
                    yDiff =
                        (*player).actor.world.pos.y + 30.0f32 -
                            (*this).actor.world.pos.y;
                    zDiff =
                        (*player).actor.world.pos.z -
                            (*this).actor.world.pos.z;
                    // yaw
                    (*this).actor.world.rot.y =
                        (Math_FAtan2F(xDiff, zDiff) *
                             (32768 as libc::c_int as libc::c_float /
                                  3.14159265358979323846f32)) as s16;
                    // pitch
                    distXZ = sqrtf(xDiff * xDiff + zDiff * zDiff);
                    (*this).actor.world.rot.x =
                        (Math_FAtan2F(yDiff, distXZ) *
                             (32768 as libc::c_int as libc::c_float /
                                  3.14159265358979323846f32)) as s16;
                    (*this).actor.speedXZ = 20.0f32;
                    i = 0 as libc::c_int as s16;
                    while (i as libc::c_int) < 50 as libc::c_int {
                        (*this).blastTailPos[i as usize] =
                            (*this).actor.world.pos;
                        i += 1
                    }
                    (*this).workf[TAIL_ALPHA as libc::c_int as usize] =
                        255.0f32;
                    current_block_89 = 16941840430239256862;
                }
                1 | 10 => { current_block_89 = 16941840430239256862; }
                2 => {
                    Math_ApproachF(&mut *(*this).workf.as_mut_ptr().offset(TAIL_ALPHA
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                   0.0f32, 1.0f32, 15.0f32);
                    if (*this).timers[0 as libc::c_int as usize] as
                           libc::c_int == 0 as libc::c_int {
                        Actor_Kill(&mut (*this).actor);
                    }
                    current_block_89 = 6471821049853688503;
                }
                _ => { current_block_89 = 6471821049853688503; }
            }
            match current_block_89 {
                16941840430239256862 =>
                // fallthrough
                {
                    (*this).blastActive = 1 as libc::c_int as u8_0;
                    if (*this).timers[0 as libc::c_int as usize] as
                           libc::c_int == 0 as libc::c_int {
                        func_8002D908(&mut (*this).actor);
                        func_8002D7EC(&mut (*this).actor);
                        Audio_PlayActorSound2(&mut (*this).actor,
                                              (0x3922 as libc::c_int &
                                                   !(0x800 as libc::c_int)) as
                                                  u16_0);
                    } else {
                        let mut velocity: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                        let mut velDir: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                        let mut blastDir: Vec3s = Vec3s{x: 0, y: 0, z: 0,};
                        let mut alpha: s16 = 0;
                        (*this).actor.world.pos =
                            (*player2).bodyPartsPos[15 as libc::c_int as
                                                        usize];
                        (*this).actor.world.pos.y = -2000.0f32;
                        Matrix_MtxFToYXZRotS(&mut (*player2).shieldMf,
                                             &mut blastDir, 0 as libc::c_int);
                        blastDir.x = -(blastDir.x as libc::c_int) as s16;
                        blastDir.y =
                            (blastDir.y as libc::c_int +
                                 0x8000 as libc::c_int) as s16;
                        Math_ApproachS(&mut (*this).magicDir.x, blastDir.x,
                                       0xa as libc::c_int as s16,
                                       0x800 as libc::c_int as s16);
                        Math_ApproachS(&mut (*this).magicDir.y, blastDir.y,
                                       0xa as libc::c_int as s16,
                                       0x800 as libc::c_int as s16);
                        if (*this).timers[0 as libc::c_int as usize] as
                               libc::c_int == 50 as libc::c_int {
                            D_8094C86F = 10 as libc::c_int as u8_0;
                            D_8094C872 = 7 as libc::c_int as s16;
                            (*globalCtx).envCtx.unk_D8 = 1.0f32
                        }
                        if (*this).timers[0 as libc::c_int as usize] as
                               libc::c_int <= 50 as libc::c_int {
                            Audio_PlayActorSound2(&mut (*this).actor,
                                                  (0x3922 as libc::c_int &
                                                       !(0x800 as
                                                             libc::c_int)) as
                                                      u16_0);
                            Audio_PlayActorSound2(&mut (*this).actor,
                                                  (0x3917 as libc::c_int &
                                                       !(0x800 as
                                                             libc::c_int)) as
                                                      u16_0);
                            Matrix_RotateY((*this).magicDir.y as libc::c_int
                                               as libc::c_float / 32678.0f32 *
                                               3.14159265358979323846f32,
                                           MTXMODE_NEW as libc::c_int as
                                               u8_0);
                            Matrix_RotateX((*this).magicDir.x as libc::c_int
                                               as libc::c_float / 32678.0f32 *
                                               3.14159265358979323846f32,
                                           MTXMODE_APPLY as libc::c_int as
                                               u8_0);
                            velDir.x = 0.0f32;
                            velDir.y = 0.0f32;
                            velDir.z = 50.0f32;
                            Matrix_MultVec3f(&mut velDir, &mut velocity);
                            alpha =
                                ((*this).timers[0 as libc::c_int as usize] as
                                     libc::c_int * 10 as libc::c_int) as s16;
                            alpha =
                                if alpha as libc::c_int > 255 as libc::c_int {
                                    255 as libc::c_int
                                } else { alpha as libc::c_int } as s16;
                            BossTw_AddShieldBlastEffect(globalCtx,
                                                        &mut *(*player2).bodyPartsPos.as_mut_ptr().offset(15
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              isize),
                                                        &mut velocity,
                                                        &mut sZeroVector,
                                                        10.0f32, 80.0f32,
                                                        alpha,
                                                        1 as libc::c_int as
                                                            s16);
                        }
                        if (*this).timers[0 as libc::c_int as usize] as
                               libc::c_int == 1 as libc::c_int {
                            sEnvType = 0 as libc::c_int as s8;
                            sShieldFireCharge =
                                sShieldFireCharge.wrapping_add(1);
                            Actor_Kill(&mut (*this).actor);
                        }
                        return
                    }
                    (*this).groundBlastPos.y =
                        BossTw_GetFloorY(&mut (*this).actor.world.pos);
                    if (*this).groundBlastPos.y >= 0.0f32 {
                        if (*this).groundBlastPos.y != 35.0f32 {
                            (*this).groundBlastPos.x =
                                (*this).actor.world.pos.x;
                            (*this).groundBlastPos.z =
                                (*this).actor.world.pos.z;
                            BossTw_SpawnGroundBlast(this, globalCtx,
                                                    1 as libc::c_int as s16);
                        } else {
                            let mut velocity_0: Vec3f =
                                Vec3f{x: 0., y: 0., z: 0.,};
                            let mut accel: Vec3f =
                                Vec3f{x: 0., y: 0., z: 0.,};
                            i = 0 as libc::c_int as s16;
                            while (i as libc::c_int) < 50 as libc::c_int {
                                velocity_0.x = Rand_CenteredFloat(20.0f32);
                                velocity_0.y = Rand_CenteredFloat(20.0f32);
                                velocity_0.z = Rand_CenteredFloat(20.0f32);
                                accel.x = 0.0f32;
                                accel.y = 0.0f32;
                                accel.z = 0.0f32;
                                BossTw_AddFlameEffect(globalCtx,
                                                      &mut (*this).actor.world.pos,
                                                      &mut velocity_0,
                                                      &mut accel,
                                                      Rand_ZeroFloat(10.0f32)
                                                          + 25.0f32,
                                                      (*this).blastType);
                                i += 1
                            }
                            (*globalCtx).envCtx.unk_D8 = 0.5f32
                        }
                        (*this).csState1 = 2 as libc::c_int as s16;
                        (*this).timers[0 as libc::c_int as usize] =
                            20 as libc::c_int as s16
                    } else {
                        let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                        let mut velocity_1: Vec3f =
                            {
                                let mut init =
                                    Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,};
                                init
                            };
                        let mut accel_0: Vec3f =
                            {
                                let mut init =
                                    Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,};
                                init
                            };
                        i = 0 as libc::c_int as s16;
                        while (i as libc::c_int) < 10 as libc::c_int {
                            pos =
                                (*this).blastTailPos[Rand_ZeroFloat(29.9f32)
                                                         as s16 as usize];
                            pos.x += Rand_CenteredFloat(40.0f32);
                            pos.y += Rand_CenteredFloat(40.0f32);
                            pos.z += Rand_CenteredFloat(40.0f32);
                            accel_0.y = 0.4f32;
                            accel_0.x = Rand_CenteredFloat(0.5f32);
                            accel_0.z = Rand_CenteredFloat(0.5f32);
                            BossTw_AddDotEffect(globalCtx, &mut pos,
                                                &mut velocity_1, &mut accel_0,
                                                (Rand_ZeroFloat(2.0f32) as s16
                                                     as libc::c_int +
                                                     8 as libc::c_int) as
                                                    f32_0,
                                                1 as libc::c_int as s16,
                                                75 as libc::c_int as s16);
                            i += 1
                        }
                    }
                }
                _ => { }
            }
        }
        101 => {
            if (*this).timers[0 as libc::c_int as usize] as libc::c_int !=
                   0 as libc::c_int {
                if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
                       1 as libc::c_int {
                    sEnvType = 0 as libc::c_int as s8
                }
                if sGroundBlastType as libc::c_int == 2 as libc::c_int {
                    (*this).timers[0 as libc::c_int as usize] =
                        0 as libc::c_int as s16
                }
                Audio_PlayActorSound2(&mut (*this).actor,
                                      (0x390f as libc::c_int -
                                           0x800 as libc::c_int) as u16_0);
                xDiff =
                    (*sKoumePtr).groundBlastPos2.x -
                        (*player).actor.world.pos.x;
                yDiff =
                    (*sKoumePtr).groundBlastPos2.y -
                        (*player).actor.world.pos.y;
                zDiff =
                    (*sKoumePtr).groundBlastPos2.z -
                        (*player).actor.world.pos.z;
                if (*player).isBurning == 0 &&
                       (*player).actor.bgCheckFlags as libc::c_int &
                           1 as libc::c_int != 0 && fabsf(yDiff) < 10.0f32 &&
                       sqrtf(xDiff * xDiff + zDiff * zDiff) <
                           (*sKoumePtr).workf[UNK_F13 as libc::c_int as usize]
                               * 4550.0f32 {
                    let mut j: s16 = 0;
                    j = 0 as libc::c_int as s16;
                    while (j as libc::c_int) < 18 as libc::c_int {
                        (*player).flameTimers[j as usize] =
                            Rand_S16Offset(0 as libc::c_int as s16,
                                           200 as libc::c_int as s16) as u8_0;
                        j += 1
                    }
                    (*player).isBurning = 1 as libc::c_int as u8_0;
                    if (*this).work[BURN_TMR as libc::c_int as usize] as
                           libc::c_int == 0 as libc::c_int {
                        func_8002F7DC(&mut (*player).actor,
                                      ((*(*player).ageProperties).unk_92 as
                                           libc::c_int +
                                           0x681e as libc::c_int) as u16_0);
                        (*this).work[BURN_TMR as libc::c_int as usize] =
                            40 as libc::c_int as s16
                    }
                    (*sTwinrovaPtr).timers[2 as libc::c_int as usize] =
                        100 as libc::c_int as s16
                }
                Math_ApproachF(&mut *(*sKoumePtr).workf.as_mut_ptr().offset(UNK_F13
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize),
                               0.04f32, 0.1f32, 0.002f32);
            } else {
                let mut sp4C: f32_0 =
                    if sGroundBlastType as libc::c_int == 2 as libc::c_int {
                        3.0f32
                    } else { 1.0f32 };
                Math_ApproachF(&mut *(*sKoumePtr).workf.as_mut_ptr().offset(UNK_F9
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize),
                               0.0f32, 1.0f32, 10.0f32 * sp4C);
                Math_ApproachF(&mut *(*sKoumePtr).workf.as_mut_ptr().offset(UNK_F12
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize),
                               0.0f32, 1.0f32, 0.03f32 * sp4C);
                Math_ApproachF(&mut *(*sKoumePtr).workf.as_mut_ptr().offset(TAIL_ALPHA
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize),
                               0.0f32, 1.0f32, 3.0f32 * sp4C);
                Math_ApproachF(&mut *(*sKoumePtr).workf.as_mut_ptr().offset(UNK_F11
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize),
                               0.0f32, 1.0f32, 6.0f32 * sp4C);
                if (*sKoumePtr).workf[TAIL_ALPHA as libc::c_int as usize] <=
                       0.0f32 {
                    Actor_Kill(&mut (*this).actor);
                }
            }
        }
        _ => { }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_BlastIce(mut this: *mut BossTw,
                                         mut globalCtx: *mut GlobalContext) {
    let mut i: s16 = 0;
    let mut xDiff: f32_0 = 0.;
    let mut yDiff: f32_0 = 0.;
    let mut zDiff: f32_0 = 0.;
    let mut xzDist: f32_0 = 0.;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut player2: *mut Player = player;
    match (*this).actor.params as libc::c_int {
        102 => {
            let mut current_block_86: u64;
            match (*this).csState1 as libc::c_int {
                0 => {
                    Actor_SetScale(&mut (*this).actor, 0.03f32);
                    (*this).csState1 = 1 as libc::c_int as s16;
                    xDiff =
                        (*player).actor.world.pos.x -
                            (*this).actor.world.pos.x;
                    yDiff =
                        (*player).actor.world.pos.y + 30.0f32 -
                            (*this).actor.world.pos.y;
                    zDiff =
                        (*player).actor.world.pos.z -
                            (*this).actor.world.pos.z;
                    (*this).actor.world.rot.y =
                        (Math_FAtan2F(xDiff, zDiff) *
                             (32768 as libc::c_int as libc::c_float /
                                  3.14159265358979323846f32)) as s16;
                    xzDist = sqrtf(xDiff * xDiff + zDiff * zDiff);
                    (*this).actor.world.rot.x =
                        (Math_FAtan2F(yDiff, xzDist) *
                             (32768 as libc::c_int as libc::c_float /
                                  3.14159265358979323846f32)) as s16;
                    (*this).actor.speedXZ = 20.0f32;
                    i = 0 as libc::c_int as s16;
                    while (i as libc::c_int) < 50 as libc::c_int {
                        (*this).blastTailPos[i as usize] =
                            (*this).actor.world.pos;
                        i += 1
                    }
                    (*this).workf[TAIL_ALPHA as libc::c_int as usize] =
                        255.0f32;
                    current_block_86 = 12808238446928043155;
                }
                1 | 10 => { current_block_86 = 12808238446928043155; }
                2 => {
                    Math_ApproachF(&mut *(*this).workf.as_mut_ptr().offset(TAIL_ALPHA
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                   0.0f32, 1.0f32, 15.0f32);
                    if (*this).timers[0 as libc::c_int as usize] as
                           libc::c_int == 0 as libc::c_int {
                        Actor_Kill(&mut (*this).actor);
                    }
                    current_block_86 = 16037123508100270995;
                }
                _ => { current_block_86 = 16037123508100270995; }
            }
            match current_block_86 {
                12808238446928043155 =>
                // fallthrough
                {
                    (*this).blastActive = 1 as libc::c_int as u8_0;
                    if (*this).timers[0 as libc::c_int as usize] as
                           libc::c_int == 0 as libc::c_int {
                        func_8002D908(&mut (*this).actor);
                        func_8002D7EC(&mut (*this).actor);
                        Audio_PlayActorSound2(&mut (*this).actor,
                                              (0x3911 as libc::c_int -
                                                   0x800 as libc::c_int) as
                                                  u16_0);
                        (*this).groundBlastPos.y =
                            BossTw_GetFloorY(&mut (*this).actor.world.pos);
                        if (*this).groundBlastPos.y >= 0.0f32 {
                            if (*this).groundBlastPos.y != 35.0f32 {
                                (*this).groundBlastPos.x =
                                    (*this).actor.world.pos.x;
                                (*this).groundBlastPos.z =
                                    (*this).actor.world.pos.z;
                                BossTw_SpawnGroundBlast(this, globalCtx,
                                                        0 as libc::c_int as
                                                            s16);
                            } else {
                                i = 0 as libc::c_int as s16;
                                while (i as libc::c_int) < 50 as libc::c_int {
                                    let mut velocity_0: Vec3f =
                                        Vec3f{x: 0., y: 0., z: 0.,};
                                    let mut accel: Vec3f =
                                        Vec3f{x: 0., y: 0., z: 0.,};
                                    velocity_0.x =
                                        Rand_CenteredFloat(20.0f32);
                                    velocity_0.y =
                                        Rand_CenteredFloat(20.0f32);
                                    velocity_0.z =
                                        Rand_CenteredFloat(20.0f32);
                                    accel.x = 0.0f32;
                                    accel.y = 0.0f32;
                                    accel.z = 0.0f32;
                                    BossTw_AddFlameEffect(globalCtx,
                                                          &mut (*this).actor.world.pos,
                                                          &mut velocity_0,
                                                          &mut accel,
                                                          Rand_ZeroFloat(10.0f32)
                                                              + 25.0f32,
                                                          (*this).blastType);
                                    i += 1
                                }
                                (*globalCtx).envCtx.unk_D8 = 0.5f32
                            }
                            (*this).csState1 = 2 as libc::c_int as s16;
                            (*this).timers[0 as libc::c_int as usize] =
                                20 as libc::c_int as s16
                        } else {
                            let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                            let mut velocity_1: Vec3f =
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                };
                            let mut accel_0: Vec3f =
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                };
                            i = 0 as libc::c_int as s16;
                            while (i as libc::c_int) < 10 as libc::c_int {
                                pos =
                                    (*this).blastTailPos[Rand_ZeroFloat(29.9f32)
                                                             as s16 as usize];
                                pos.x += Rand_CenteredFloat(40.0f32);
                                pos.y += Rand_CenteredFloat(40.0f32);
                                pos.z += Rand_CenteredFloat(40.0f32);
                                accel_0.y = 0.4f32;
                                accel_0.x = Rand_CenteredFloat(0.5f32);
                                accel_0.z = Rand_CenteredFloat(0.5f32);
                                BossTw_AddDotEffect(globalCtx, &mut pos,
                                                    &mut velocity_1,
                                                    &mut accel_0,
                                                    (Rand_ZeroFloat(2.0f32) as
                                                         s16 as libc::c_int +
                                                         8 as libc::c_int) as
                                                        f32_0,
                                                    0 as libc::c_int as s16,
                                                    75 as libc::c_int as s16);
                                i += 1
                            }
                        }
                    } else {
                        let mut velocity: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                        let mut spF4: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                        let mut reflDir: Vec3s = Vec3s{x: 0, y: 0, z: 0,};
                        let mut alpha: s16 = 0;
                        (*this).actor.world.pos =
                            (*player2).bodyPartsPos[15 as libc::c_int as
                                                        usize];
                        (*this).actor.world.pos.y = -2000.0f32;
                        Matrix_MtxFToYXZRotS(&mut (*player2).shieldMf,
                                             &mut reflDir, 0 as libc::c_int);
                        reflDir.x = -(reflDir.x as libc::c_int) as s16;
                        reflDir.y =
                            (reflDir.y as libc::c_int + 0x8000 as libc::c_int)
                                as s16;
                        Math_ApproachS(&mut (*this).magicDir.x, reflDir.x,
                                       0xa as libc::c_int as s16,
                                       0x800 as libc::c_int as s16);
                        Math_ApproachS(&mut (*this).magicDir.y, reflDir.y,
                                       0xa as libc::c_int as s16,
                                       0x800 as libc::c_int as s16);
                        if (*this).timers[0 as libc::c_int as usize] as
                               libc::c_int == 50 as libc::c_int {
                            D_8094C86F = 10 as libc::c_int as u8_0;
                            D_8094C872 = 7 as libc::c_int as s16;
                            (*globalCtx).envCtx.unk_D8 = 1.0f32
                        }
                        if (*this).timers[0 as libc::c_int as usize] as
                               libc::c_int <= 50 as libc::c_int {
                            Audio_PlayActorSound2(&mut (*this).actor,
                                                  (0x3911 as libc::c_int -
                                                       0x800 as libc::c_int)
                                                      as u16_0);
                            Audio_PlayActorSound2(&mut (*this).actor,
                                                  (0x3918 as libc::c_int -
                                                       0x800 as libc::c_int)
                                                      as u16_0);
                            Matrix_RotateY((*this).magicDir.y as libc::c_int
                                               as libc::c_float / 32678.0f32 *
                                               3.14159265358979323846f32,
                                           MTXMODE_NEW as libc::c_int as
                                               u8_0);
                            Matrix_RotateX((*this).magicDir.x as libc::c_int
                                               as libc::c_float / 32678.0f32 *
                                               3.14159265358979323846f32,
                                           MTXMODE_APPLY as libc::c_int as
                                               u8_0);
                            spF4.x = 0.0f32;
                            spF4.y = 0.0f32;
                            spF4.z = 50.0f32;
                            Matrix_MultVec3f(&mut spF4, &mut velocity);
                            alpha =
                                ((*this).timers[0 as libc::c_int as usize] as
                                     libc::c_int * 10 as libc::c_int) as s16;
                            alpha =
                                if alpha as libc::c_int > 255 as libc::c_int {
                                    255 as libc::c_int
                                } else { alpha as libc::c_int } as s16;
                            BossTw_AddShieldBlastEffect(globalCtx,
                                                        &mut *(*player2).bodyPartsPos.as_mut_ptr().offset(15
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              isize),
                                                        &mut velocity,
                                                        &mut sZeroVector,
                                                        10.0f32, 80.0f32,
                                                        alpha,
                                                        0 as libc::c_int as
                                                            s16);
                        }
                        if (*this).timers[0 as libc::c_int as usize] as
                               libc::c_int == 1 as libc::c_int {
                            sEnvType = 0 as libc::c_int as s8;
                            sShieldIceCharge =
                                sShieldIceCharge.wrapping_add(1);
                            Actor_Kill(&mut (*this).actor);
                        }
                    }
                }
                _ => { }
            }
        }
        103 => {
            if (*this).timers[0 as libc::c_int as usize] as libc::c_int !=
                   0 as libc::c_int {
                if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
                       1 as libc::c_int {
                    sEnvType = 0 as libc::c_int as s8
                }
                if sGroundBlastType as libc::c_int == 1 as libc::c_int {
                    (*this).timers[0 as libc::c_int as usize] =
                        0 as libc::c_int as s16
                }
                Audio_PlayActorSound2(&mut (*this).actor,
                                      (0x28b2 as libc::c_int -
                                           0x800 as libc::c_int) as u16_0);
                if (*this).timers[0 as libc::c_int as usize] as libc::c_int >
                       (if (*sTwinrovaPtr).actionFunc ==
                               Some(BossTw_Wait as
                                        unsafe extern "C" fn(_: *mut BossTw,
                                                             _:
                                                                 *mut GlobalContext)
                                            -> ()) {
                            70 as libc::c_int
                        } else { 20 as libc::c_int }) {
                    let mut pad: s32 = 0;
                    let mut pos_0: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                    let mut velocity_2: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                    let mut accel_1: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                    pos_0.x =
                        (*sKotakePtr).groundBlastPos2.x +
                            Rand_CenteredFloat(320.0f32);
                    pos_0.z =
                        (*sKotakePtr).groundBlastPos2.z +
                            Rand_CenteredFloat(320.0f32);
                    pos_0.y = (*sKotakePtr).groundBlastPos2.y;
                    velocity_2.x = 0.0f32;
                    velocity_2.y = 0.0f32;
                    velocity_2.z = 0.0f32;
                    accel_1.x = 0.0f32;
                    accel_1.y = 0.13f32;
                    accel_1.z = 0.0f32;
                    BossTw_AddDmgCloud(globalCtx, 3 as libc::c_int as s16,
                                       &mut pos_0, &mut velocity_2,
                                       &mut accel_1,
                                       Rand_ZeroFloat(5.0f32) + 20.0f32,
                                       0 as libc::c_int as s16,
                                       0 as libc::c_int as s16,
                                       80 as libc::c_int as s16);
                    velocity_2.x = Rand_CenteredFloat(10.0f32);
                    velocity_2.z = Rand_CenteredFloat(10.0f32);
                    velocity_2.y = Rand_ZeroFloat(3.0f32) + 3.0f32;
                    pos_0.x =
                        (*sKotakePtr).groundBlastPos2.x +
                            velocity_2.x * 0.5f32;
                    pos_0.z =
                        (*sKotakePtr).groundBlastPos2.z +
                            velocity_2.z * 0.5f32;
                    BossTw_AddDmgCloud(globalCtx, 3 as libc::c_int as s16,
                                       &mut pos_0, &mut velocity_2,
                                       &mut accel_1,
                                       Rand_ZeroFloat(5.0f32) + 15.0f32,
                                       255 as libc::c_int as s16,
                                       2 as libc::c_int as s16,
                                       130 as libc::c_int as s16);
                }
                Math_ApproachF(&mut *(*sKotakePtr).workf.as_mut_ptr().offset(UNK_F9
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 isize),
                               80.0f32, 1.0f32, 3.0f32);
                Math_ApproachF(&mut *(*sKotakePtr).workf.as_mut_ptr().offset(UNK_F11
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 isize),
                               255.0f32, 1.0f32, 10.0f32);
                Math_ApproachF(&mut *(*sKotakePtr).workf.as_mut_ptr().offset(UNK_F12
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 isize),
                               0.04f32, 0.1f32, 0.002f32);
                Math_ApproachF(&mut *(*sKotakePtr).workf.as_mut_ptr().offset(UNK_F16
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 isize),
                               70.0f32, 1.0f32, 5.0f32);
                if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
                       70 as libc::c_int ||
                       (*this).timers[0 as libc::c_int as usize] as
                           libc::c_int == 30 as libc::c_int {
                    (*sKotakePtr).workf[UNK_F16 as libc::c_int as usize] =
                        10.0f32
                }
                if (*this).timers[0 as libc::c_int as usize] as libc::c_int %
                       4 as libc::c_int == 0 as libc::c_int {
                    (*sKotakePtr).workf[UNK_F15 as libc::c_int as usize] =
                        2.0f32 *
                            Rand_ZeroFloat(9.9f32) as s16 as libc::c_int as
                                libc::c_float * 3.14159265358979323846f32 /
                            10.0f32
                }
            } else {
                let mut sp80: f32_0 = 0.;
                if sGroundBlastType as libc::c_int == 1 as libc::c_int {
                    if (*sKotakePtr).workf[UNK_F11 as libc::c_int as usize] >
                           1.0f32 {
                        i = 0 as libc::c_int as s16;
                        while (i as libc::c_int) < 3 as libc::c_int {
                            let mut pos_1: Vec3f =
                                Vec3f{x: 0., y: 0., z: 0.,};
                            let mut velocity_3: Vec3f =
                                Vec3f{x: 0., y: 0., z: 0.,};
                            let mut accel_2: Vec3f =
                                Vec3f{x: 0., y: 0., z: 0.,};
                            pos_1.x =
                                Rand_CenteredFloat(280.0f32) +
                                    (*sKotakePtr).groundBlastPos2.x;
                            pos_1.z =
                                Rand_CenteredFloat(280.0f32) +
                                    (*sKotakePtr).groundBlastPos2.z;
                            pos_1.y =
                                (*sKotakePtr).groundBlastPos2.y + 30.0f32;
                            velocity_3.x = 0.0f32;
                            velocity_3.y = 0.0f32;
                            velocity_3.z = 0.0f32;
                            accel_2.x = 0.0f32;
                            accel_2.y = 0.13f32;
                            accel_2.z = 0.0f32;
                            BossTw_AddDmgCloud(globalCtx,
                                               3 as libc::c_int as s16,
                                               &mut pos_1, &mut velocity_3,
                                               &mut accel_2,
                                               Rand_ZeroFloat(5.0f32) +
                                                   20 as libc::c_int as
                                                       libc::c_float,
                                               0 as libc::c_int as s16,
                                               0 as libc::c_int as s16,
                                               80 as libc::c_int as s16);
                            i += 1
                        }
                    }
                    sp80 = 3.0f32
                } else { sp80 = 1.0f32 }
                Math_ApproachF(&mut *(*sKotakePtr).workf.as_mut_ptr().offset(UNK_F14
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 isize),
                               0.0f32, 1.0f32, 0.2f32 * sp80);
                Math_ApproachF(&mut *(*sKotakePtr).workf.as_mut_ptr().offset(UNK_F11
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 isize),
                               0.0f32, 1.0f32, 5.0f32 * sp80);
                Math_ApproachF(&mut *(*sKotakePtr).workf.as_mut_ptr().offset(UNK_F9
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 isize),
                               0.0f32, 1.0f32, sp80);
                if (*sKotakePtr).workf[UNK_F9 as libc::c_int as usize] <=
                       0.0f32 {
                    Actor_Kill(&mut (*this).actor);
                }
            }
        }
        _ => { }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_BlastShieldCheck(mut this: *mut BossTw,
                                                 mut globalCtx:
                                                     *mut GlobalContext)
 -> s32 {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut ret: s32 = 0 as libc::c_int;
    let mut info: *mut ColliderInfo = 0 as *mut ColliderInfo;
    if (*this).csState1 as libc::c_int == 1 as libc::c_int {
        if (*this).collider.base.acFlags as libc::c_int &
               (1 as libc::c_int) << 1 as libc::c_int != 0 {
            (*this).collider.base.acFlags =
                ((*this).collider.base.acFlags as libc::c_int &
                     !((1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
            (*this).collider.base.atFlags =
                ((*this).collider.base.atFlags as libc::c_int &
                     !((1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
            info = (*this).collider.info.acHitInfo;
            if (*info).toucher.dmgFlags &
                   ((1 as libc::c_int) << 0x14 as libc::c_int) as libc::c_uint
                   != 0 {
                (*this).work[INVINC_TIMER as libc::c_int as usize] =
                    7 as libc::c_int as s16;
                (*globalCtx).envCtx.unk_D8 = 1.0f32;
                func_800AA000(0.0f32, 100 as libc::c_int as u8_0,
                              5 as libc::c_int as u8_0,
                              4 as libc::c_int as u8_0);
                if Player_HasMirrorShieldEquipped(globalCtx) != 0 {
                    if (*this).blastType as libc::c_int == 1 as libc::c_int {
                        if sShieldIceCharge as libc::c_int != 0 as libc::c_int
                           {
                            sShieldIceCharge = 0 as libc::c_int as u8_0;
                            BossTw_AddShieldDeflectEffect(globalCtx, 10.0f32,
                                                          1 as libc::c_int as
                                                              s16);
                        } else {
                            BossTw_AddShieldHitEffect(globalCtx, 10.0f32,
                                                      1 as libc::c_int as
                                                          s16);
                            sShieldFireCharge =
                                sShieldFireCharge.wrapping_add(1);
                            D_8094C86F =
                                (sShieldFireCharge as libc::c_int *
                                     2 as libc::c_int + 8 as libc::c_int) as
                                    u8_0;
                            D_8094C872 = -(7 as libc::c_int) as s16
                        }
                    } else if sShieldFireCharge as libc::c_int !=
                                  0 as libc::c_int {
                        sShieldFireCharge = 0 as libc::c_int as u8_0;
                        BossTw_AddShieldDeflectEffect(globalCtx, 10.0f32,
                                                      0 as libc::c_int as
                                                          s16);
                    } else {
                        BossTw_AddShieldHitEffect(globalCtx, 10.0f32,
                                                  0 as libc::c_int as s16);
                        sShieldIceCharge = sShieldIceCharge.wrapping_add(1);
                        D_8094C86F =
                            (sShieldIceCharge as libc::c_int *
                                 2 as libc::c_int + 8 as libc::c_int) as u8_0;
                        D_8094C872 = -(7 as libc::c_int) as s16
                    }
                    if sShieldIceCharge as libc::c_int >= 3 as libc::c_int ||
                           sShieldFireCharge as libc::c_int >=
                               3 as libc::c_int {
                        (*this).timers[0 as libc::c_int as usize] =
                            80 as libc::c_int as s16;
                        (*this).csState1 = 10 as libc::c_int as s16;
                        Matrix_MtxFToYXZRotS(&mut (*player).shieldMf,
                                             &mut (*this).magicDir,
                                             0 as libc::c_int);
                        (*this).magicDir.y =
                            ((*this).magicDir.y as libc::c_int +
                                 0x8000 as libc::c_int) as s16;
                        (*this).magicDir.x =
                            -((*this).magicDir.x as libc::c_int) as s16;
                        D_8094C86F = 8 as libc::c_int as u8_0
                    } else {
                        (*this).csState1 = 2 as libc::c_int as s16;
                        (*this).timers[0 as libc::c_int as usize] =
                            20 as libc::c_int as s16;
                        sEnvType = 0 as libc::c_int as s8
                    }
                } else {
                    BossTw_AddShieldDeflectEffect(globalCtx, 10.0f32,
                                                  (*this).blastType);
                    (*this).csState1 = 2 as libc::c_int as s16;
                    (*this).timers[0 as libc::c_int as usize] =
                        20 as libc::c_int as s16;
                    sEnvType = 0 as libc::c_int as s8;
                    sShieldIceCharge = 0 as libc::c_int as u8_0;
                    sShieldFireCharge = 0 as libc::c_int as u8_0;
                    func_80078884(0x1838 as libc::c_int as u16_0);
                }
                ret = 1 as libc::c_int
            }
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_BlastUpdate(mut thisx: *mut Actor,
                                            mut globalCtx:
                                                *mut GlobalContext) {
    let mut this: *mut BossTw = thisx as *mut BossTw;
    let mut collider: *mut ColliderCylinder = 0 as *mut ColliderCylinder;
    let mut i: s16 = 0;
    (*this).work[CS_TIMER_1 as libc::c_int as usize] += 1;
    (*this).work[CS_TIMER_2 as libc::c_int as usize] += 1;
    (*this).work[TAIL_IDX as libc::c_int as usize] += 1;
    if (*this).work[TAIL_IDX as libc::c_int as usize] as libc::c_int >
           29 as libc::c_int {
        (*this).work[TAIL_IDX as libc::c_int as usize] =
            0 as libc::c_int as s16
    }
    (*this).blastTailPos[(*this).work[TAIL_IDX as libc::c_int as usize] as
                             usize] = (*this).actor.world.pos;
    (*this).actionFunc.expect("non-null function pointer")(this, globalCtx);
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < 5 as libc::c_int {
        if (*this).timers[i as usize] as libc::c_int != 0 as libc::c_int {
            (*this).timers[i as usize] -= 1
        }
        i += 1
    }
    if (*this).work[INVINC_TIMER as libc::c_int as usize] as libc::c_int !=
           0 as libc::c_int {
        (*this).work[INVINC_TIMER as libc::c_int as usize] -= 1
    }
    if (*this).work[BURN_TMR as libc::c_int as usize] as libc::c_int !=
           0 as libc::c_int {
        (*this).work[BURN_TMR as libc::c_int as usize] -= 1
    }
    (*this).actor.focus.pos = (*this).actor.world.pos;
    collider = &mut (*this).collider;
    Collider_UpdateCylinder(&mut (*this).actor, collider);
    if (*this).blastActive as libc::c_int != 0 &&
           (*this).work[INVINC_TIMER as libc::c_int as usize] as libc::c_int
               == 0 as libc::c_int &&
           BossTw_BlastShieldCheck(this, globalCtx) == 0 {
        CollisionCheck_SetAC(globalCtx, &mut (*globalCtx).colChkCtx,
                             &mut (*collider).base);
        CollisionCheck_SetAT(globalCtx, &mut (*globalCtx).colChkCtx,
                             &mut (*collider).base);
    }
    (*this).blastActive = 0 as libc::c_int as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_BlastDraw(mut thisx: *mut Actor,
                                          mut globalCtx2:
                                              *mut GlobalContext) {
    let mut globalCtx: *mut GlobalContext = globalCtx2;
    let mut this: *mut BossTw = thisx as *mut BossTw;
    let mut scaleFactor: f32_0 = 0.;
    let mut tailIdx: s16 = 0;
    let mut i: s16 = 0;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_boss_tw.c\x00" as *const u8 as *const libc::c_char,
                    8818 as libc::c_int);
    func_80093D84((*globalCtx).state.gfxCtx);
    match (*this).actor.params as libc::c_int {
        100 => {
            let fresh130 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g: *mut Gfx = fresh130;
            (*_g).words.w0 =
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
            (*_g).words.w1 =
                (200 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (20 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    ((*this).workf[TAIL_ALPHA as libc::c_int as usize] as s8
                         as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh131 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_0: *mut Gfx = fresh131;
            (*_g_0).words.w0 =
                (0xfb as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_0).words.w1 =
                (255 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (215 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (255 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    (128 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            i = 9 as libc::c_int as s16;
            while i as libc::c_int >= 0 as libc::c_int {
                let fresh132 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_1: *mut Gfx = fresh132;
                (*_g_1).words.w0 =
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
                (*_g_1).words.w1 =
                    Gfx_TwoTexScroll((*globalCtx).state.gfxCtx,
                                     0 as libc::c_int,
                                     ((*this).work[CS_TIMER_1 as libc::c_int
                                                       as usize] as
                                          libc::c_int * 3 as libc::c_int +
                                          i as libc::c_int * 10 as libc::c_int
                                          & 0x7f as libc::c_int) as u32_0,
                                     (-((*this).work[CS_TIMER_1 as libc::c_int
                                                         as usize] as
                                            libc::c_int) * 15 as libc::c_int +
                                          i as libc::c_int * 50 as libc::c_int
                                          & 0xff as libc::c_int) as u32_0,
                                     0x20 as libc::c_int, 0x40 as libc::c_int,
                                     1 as libc::c_int,
                                     0 as libc::c_int as u32_0,
                                     0 as libc::c_int as u32_0,
                                     0x20 as libc::c_int, 0x20 as libc::c_int)
                        as libc::c_uint;
                tailIdx =
                    (((*this).work[TAIL_IDX as libc::c_int as usize] as
                          libc::c_int - i as libc::c_int + 30 as libc::c_int)
                         % 30 as libc::c_int) as s16;
                Matrix_Translate((*this).blastTailPos[tailIdx as usize].x,
                                 (*this).blastTailPos[tailIdx as usize].y,
                                 (*this).blastTailPos[tailIdx as usize].z,
                                 MTXMODE_NEW as libc::c_int as u8_0);
                scaleFactor =
                    1.0f32 - i as libc::c_int as libc::c_float * 0.09f32;
                Matrix_Scale((*this).actor.scale.x * scaleFactor,
                             (*this).actor.scale.y * scaleFactor,
                             (*this).actor.scale.z * scaleFactor,
                             MTXMODE_APPLY as libc::c_int as u8_0);
                func_800D1FD4(&mut (*globalCtx).billboardMtxF);
                let fresh133 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_2: *mut Gfx = fresh133;
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
                               0 as libc::c_int) ^ 0x1 as libc::c_int) as
                             u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                (*_g_2).words.w1 =
                    Matrix_NewMtx((*globalCtx).state.gfxCtx,
                                  b"../z_boss_tw.c\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char, 8865 as libc::c_int)
                        as libc::c_uint;
                let fresh134 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_3: *mut Gfx = fresh134;
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
                    gSegments[((object_tw_DL_01A430.as_mut_ptr() as u32_0) <<
                                   4 as libc::c_int >> 28 as libc::c_int) as
                                  usize].wrapping_add(object_tw_DL_01A430.as_mut_ptr()
                                                          as u32_0 &
                                                          0xffffff as
                                                              libc::c_int as
                                                              libc::c_uint).wrapping_add(0x80000000
                                                                                             as
                                                                                             libc::c_uint)
                        as *mut libc::c_void as libc::c_uint;
                i -= 1
            }
        }
        102 => {
            let fresh135 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_4: *mut Gfx = fresh135;
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
                (195 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (225 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (235 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    ((*this).workf[TAIL_ALPHA as libc::c_int as usize] as s8
                         as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh136 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_5: *mut Gfx = fresh136;
            (*_g_5).words.w0 =
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
            (*_g_5).words.w1 =
                gSegments[((object_tw_DL_01A998.as_mut_ptr() as u32_0) <<
                               4 as libc::c_int >> 28 as libc::c_int) as
                              usize].wrapping_add(object_tw_DL_01A998.as_mut_ptr()
                                                      as u32_0 &
                                                      0xffffff as libc::c_int
                                                          as
                                                          libc::c_uint).wrapping_add(0x80000000
                                                                                         as
                                                                                         libc::c_uint)
                    as *mut libc::c_void as libc::c_uint;
            i = 9 as libc::c_int as s16;
            while i as libc::c_int >= 0 as libc::c_int {
                let fresh137 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_6: *mut Gfx = fresh137;
                (*_g_6).words.w0 =
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
                (*_g_6).words.w1 =
                    Gfx_TwoTexScroll((*globalCtx).state.gfxCtx,
                                     0 as libc::c_int,
                                     ((*this).work[CS_TIMER_1 as libc::c_int
                                                       as usize] as
                                          libc::c_int * 3 as libc::c_int +
                                          i as libc::c_int *
                                              0xa as libc::c_int &
                                          0x7f as libc::c_int) as u32_0,
                                     (-((*this).work[CS_TIMER_1 as libc::c_int
                                                         as usize] as
                                            libc::c_int) * 0xf as libc::c_int
                                          +
                                          i as libc::c_int *
                                              50 as libc::c_int) as u8_0 as
                                         u32_0, 0x20 as libc::c_int,
                                     0x40 as libc::c_int, 1 as libc::c_int,
                                     0 as libc::c_int as u32_0,
                                     0 as libc::c_int as u32_0,
                                     0x20 as libc::c_int, 0x20 as libc::c_int)
                        as libc::c_uint;
                tailIdx =
                    (((*this).work[TAIL_IDX as libc::c_int as usize] as
                          libc::c_int - i as libc::c_int + 30 as libc::c_int)
                         % 30 as libc::c_int) as s16;
                Matrix_Translate((*this).blastTailPos[tailIdx as usize].x,
                                 (*this).blastTailPos[tailIdx as usize].y,
                                 (*this).blastTailPos[tailIdx as usize].z,
                                 MTXMODE_NEW as libc::c_int as u8_0);
                scaleFactor =
                    1.0f32 - i as libc::c_int as libc::c_float * 0.09f32;
                Matrix_Scale((*this).actor.scale.x * scaleFactor,
                             (*this).actor.scale.y * scaleFactor,
                             (*this).actor.scale.z * scaleFactor,
                             MTXMODE_APPLY as libc::c_int as u8_0);
                func_800D1FD4(&mut (*globalCtx).billboardMtxF);
                let fresh138 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_7: *mut Gfx = fresh138;
                (*_g_7).words.w0 =
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
                (*_g_7).words.w1 =
                    Matrix_NewMtx((*globalCtx).state.gfxCtx,
                                  b"../z_boss_tw.c\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char, 9004 as libc::c_int)
                        as libc::c_uint;
                let fresh139 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_8: *mut Gfx = fresh139;
                (*_g_8).words.w0 =
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
                (*_g_8).words.w1 =
                    gSegments[((object_tw_DL_01AB00.as_mut_ptr() as u32_0) <<
                                   4 as libc::c_int >> 28 as libc::c_int) as
                                  usize].wrapping_add(object_tw_DL_01AB00.as_mut_ptr()
                                                          as u32_0 &
                                                          0xffffff as
                                                              libc::c_int as
                                                              libc::c_uint).wrapping_add(0x80000000
                                                                                             as
                                                                                             libc::c_uint)
                        as *mut libc::c_void as libc::c_uint;
                i -= 1
            }
        }
        101 | 103 | _ => { }
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_boss_tw.c\x00" as *const u8 as
                         *const libc::c_char, 9013 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_DrawDeathBall(mut thisx: *mut Actor,
                                              mut globalCtx2:
                                                  *mut GlobalContext) {
    let mut globalCtx: *mut GlobalContext = globalCtx2;
    let mut this: *mut BossTw = thisx as *mut BossTw;
    let mut scaleFactor: f32_0 = 0.;
    let mut tailIdx: s16 = 0;
    let mut i: s16 = 0;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_boss_tw.c\x00" as *const u8 as *const libc::c_char,
                    9028 as libc::c_int);
    func_80093D84((*globalCtx).state.gfxCtx);
    if (*this).actor.params as libc::c_int ==
           TW_DEATHBALL_KOUME as libc::c_int {
        let fresh140 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g: *mut Gfx = fresh140;
        (*_g).words.w0 =
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
        (*_g).words.w1 =
            (200 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (20 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                ((*this).workf[TAIL_ALPHA as libc::c_int as usize] as s8 as
                     u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh141 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_0: *mut Gfx = fresh141;
        (*_g_0).words.w0 =
            (0xfb as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_0).words.w1 =
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (215 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                (255 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (128 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        i = 9 as libc::c_int as s16;
        while i as libc::c_int >= 0 as libc::c_int {
            let fresh142 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_1: *mut Gfx = fresh142;
            (*_g_1).words.w0 =
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
            (*_g_1).words.w1 =
                Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                                 ((*this).work[CS_TIMER_1 as libc::c_int as
                                                   usize] as libc::c_int *
                                      3 as libc::c_int +
                                      i as libc::c_int * 0xa as libc::c_int &
                                      0x7f as libc::c_int) as u32_0,
                                 (-((*this).work[CS_TIMER_1 as libc::c_int as
                                                     usize] as libc::c_int) *
                                      0xf as libc::c_int +
                                      i as libc::c_int * 50 as libc::c_int) as
                                     u8_0 as u32_0, 0x20 as libc::c_int,
                                 0x40 as libc::c_int, 1 as libc::c_int,
                                 0 as libc::c_int as u32_0,
                                 0 as libc::c_int as u32_0,
                                 0x20 as libc::c_int, 0x20 as libc::c_int) as
                    libc::c_uint;
            tailIdx =
                (((*this).work[TAIL_IDX as libc::c_int as usize] as
                      libc::c_int - i as libc::c_int + 30 as libc::c_int) %
                     30 as libc::c_int) as s16;
            Matrix_Translate((*this).blastTailPos[tailIdx as usize].x,
                             (*this).blastTailPos[tailIdx as usize].y,
                             (*this).blastTailPos[tailIdx as usize].z,
                             MTXMODE_NEW as libc::c_int as u8_0);
            scaleFactor =
                1.0f32 - i as libc::c_int as libc::c_float * 0.09f32;
            Matrix_Scale((*this).actor.scale.x * scaleFactor,
                         (*this).actor.scale.y * scaleFactor,
                         (*this).actor.scale.z * scaleFactor,
                         MTXMODE_APPLY as libc::c_int as u8_0);
            func_800D1FD4(&mut (*globalCtx).billboardMtxF);
            let fresh143 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_2: *mut Gfx = fresh143;
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
                Matrix_NewMtx((*globalCtx).state.gfxCtx,
                              b"../z_boss_tw.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              9071 as libc::c_int) as libc::c_uint;
            let fresh144 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_3: *mut Gfx = fresh144;
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
                gSegments[((object_tw_DL_01A430.as_mut_ptr() as u32_0) <<
                               4 as libc::c_int >> 28 as libc::c_int) as
                              usize].wrapping_add(object_tw_DL_01A430.as_mut_ptr()
                                                      as u32_0 &
                                                      0xffffff as libc::c_int
                                                          as
                                                          libc::c_uint).wrapping_add(0x80000000
                                                                                         as
                                                                                         libc::c_uint)
                    as *mut libc::c_void as libc::c_uint;
            i -= 1
        }
    } else {
        let fresh145 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_4: *mut Gfx = fresh145;
        (*_g_4).words.w0 =
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
        (*_g_4).words.w1 =
            (195 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (225 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                (235 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                ((*this).workf[TAIL_ALPHA as libc::c_int as usize] as s8 as
                     u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh146 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_5: *mut Gfx = fresh146;
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
        (*_g_5).words.w1 =
            gSegments[((object_tw_DL_01A998.as_mut_ptr() as u32_0) <<
                           4 as libc::c_int >> 28 as libc::c_int) as
                          usize].wrapping_add(object_tw_DL_01A998.as_mut_ptr()
                                                  as u32_0 &
                                                  0xffffff as libc::c_int as
                                                      libc::c_uint).wrapping_add(0x80000000
                                                                                     as
                                                                                     libc::c_uint)
                as *mut libc::c_void as libc::c_uint;
        i = 9 as libc::c_int as s16;
        while i as libc::c_int >= 0 as libc::c_int {
            let fresh147 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_6: *mut Gfx = fresh147;
            (*_g_6).words.w0 =
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
            (*_g_6).words.w1 =
                Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                                 ((*this).work[CS_TIMER_1 as libc::c_int as
                                                   usize] as libc::c_int *
                                      3 as libc::c_int +
                                      i as libc::c_int * 0xa as libc::c_int &
                                      0x7f as libc::c_int) as u32_0,
                                 (-((*this).work[CS_TIMER_1 as libc::c_int as
                                                     usize] as libc::c_int) *
                                      0xf as libc::c_int +
                                      i as libc::c_int * 50 as libc::c_int) as
                                     u8_0 as u32_0, 0x20 as libc::c_int,
                                 0x40 as libc::c_int, 1 as libc::c_int,
                                 0 as libc::c_int as u32_0,
                                 0 as libc::c_int as u32_0,
                                 0x20 as libc::c_int, 0x20 as libc::c_int) as
                    libc::c_uint;
            tailIdx =
                (((*this).work[TAIL_IDX as libc::c_int as usize] as
                      libc::c_int - i as libc::c_int + 30 as libc::c_int) %
                     30 as libc::c_int) as s16;
            Matrix_Translate((*this).blastTailPos[tailIdx as usize].x,
                             (*this).blastTailPos[tailIdx as usize].y,
                             (*this).blastTailPos[tailIdx as usize].z,
                             MTXMODE_NEW as libc::c_int as u8_0);
            scaleFactor =
                1.0f32 - i as libc::c_int as libc::c_float * 0.09f32;
            Matrix_Scale((*this).actor.scale.x * scaleFactor,
                         (*this).actor.scale.y * scaleFactor,
                         (*this).actor.scale.z * scaleFactor,
                         MTXMODE_APPLY as libc::c_int as u8_0);
            func_800D1FD4(&mut (*globalCtx).billboardMtxF);
            let fresh148 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_7: *mut Gfx = fresh148;
            (*_g_7).words.w0 =
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
            (*_g_7).words.w1 =
                Matrix_NewMtx((*globalCtx).state.gfxCtx,
                              b"../z_boss_tw.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              9107 as libc::c_int) as libc::c_uint;
            let fresh149 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_8: *mut Gfx = fresh149;
            (*_g_8).words.w0 =
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
            (*_g_8).words.w1 =
                gSegments[((object_tw_DL_01AB00.as_mut_ptr() as u32_0) <<
                               4 as libc::c_int >> 28 as libc::c_int) as
                              usize].wrapping_add(object_tw_DL_01AB00.as_mut_ptr()
                                                      as u32_0 &
                                                      0xffffff as libc::c_int
                                                          as
                                                          libc::c_uint).wrapping_add(0x80000000
                                                                                         as
                                                                                         libc::c_uint)
                    as *mut libc::c_void as libc::c_uint;
            i -= 1
        }
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_boss_tw.c\x00" as *const u8 as
                         *const libc::c_char, 9111 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_UpdateEffects(mut globalCtx:
                                                  *mut GlobalContext) {
    static mut sDotColors: [Color_RGB8; 8] =
        [{
             let mut init =
                 Color_RGB8{r: 255 as libc::c_int as u8_0,
                            g: 128 as libc::c_int as u8_0,
                            b: 0 as libc::c_int as u8_0,};
             init
         },
         {
             let mut init =
                 Color_RGB8{r: 255 as libc::c_int as u8_0,
                            g: 0 as libc::c_int as u8_0,
                            b: 0 as libc::c_int as u8_0,};
             init
         },
         {
             let mut init =
                 Color_RGB8{r: 255 as libc::c_int as u8_0,
                            g: 255 as libc::c_int as u8_0,
                            b: 0 as libc::c_int as u8_0,};
             init
         },
         {
             let mut init =
                 Color_RGB8{r: 255 as libc::c_int as u8_0,
                            g: 0 as libc::c_int as u8_0,
                            b: 0 as libc::c_int as u8_0,};
             init
         },
         {
             let mut init =
                 Color_RGB8{r: 100 as libc::c_int as u8_0,
                            g: 100 as libc::c_int as u8_0,
                            b: 100 as libc::c_int as u8_0,};
             init
         },
         {
             let mut init =
                 Color_RGB8{r: 255 as libc::c_int as u8_0,
                            g: 255 as libc::c_int as u8_0,
                            b: 255 as libc::c_int as u8_0,};
             init
         },
         {
             let mut init =
                 Color_RGB8{r: 150 as libc::c_int as u8_0,
                            g: 150 as libc::c_int as u8_0,
                            b: 150 as libc::c_int as u8_0,};
             init
         },
         {
             let mut init =
                 Color_RGB8{r: 255 as libc::c_int as u8_0,
                            g: 255 as libc::c_int as u8_0,
                            b: 255 as libc::c_int as u8_0,};
             init
         }];
    let mut sp11C: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut eff: *mut BossTwEffect =
        (*globalCtx).specialEffects as *mut BossTwEffect;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut sp113: u8_0 = 0 as libc::c_int as u8_0;
    let mut i: s16 = 0;
    let mut j: s16 = 0;
    let mut colorIdx: s16 = 0;
    let mut off: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut spF4: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut spE8: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut spDC: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut spD0: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut phi_f22: f32_0 = 0.;
    let mut spC0: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut spB4: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut spA8: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut spA6: s16 = 0;
    let mut phi_f0: f32_0 = 0.;
    let mut unk44: *mut Actor = 0 as *mut Actor;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[BossTwEffect; 150]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<BossTwEffect>()
                                                   as libc::c_ulong) as s32 {
        if (*eff).type_0 as libc::c_int != 0 as libc::c_int {
            (*eff).pos.x += (*eff).curSpeed.x;
            (*eff).pos.y += (*eff).curSpeed.y;
            (*eff).pos.z += (*eff).curSpeed.z;
            (*eff).frame = (*eff).frame.wrapping_add(1);
            (*eff).curSpeed.x += (*eff).accel.x;
            (*eff).curSpeed.y += (*eff).accel.y;
            (*eff).curSpeed.z += (*eff).accel.z;
            if (*eff).type_0 as libc::c_int == 1 as libc::c_int {
                colorIdx =
                    ((*eff).frame as libc::c_int % 4 as libc::c_int) as s16;
                if (*eff).work[EFF_ARGS as libc::c_int as usize] as
                       libc::c_int == 0 as libc::c_int {
                    colorIdx =
                        (colorIdx as libc::c_int + 4 as libc::c_int) as s16
                }
                (*eff).color.r = sDotColors[colorIdx as usize].r;
                (*eff).color.g = sDotColors[colorIdx as usize].g;
                (*eff).color.b = sDotColors[colorIdx as usize].b;
                (*eff).alpha =
                    ((*eff).alpha as libc::c_int - 20 as libc::c_int) as s16;
                if (*eff).alpha as libc::c_int <= 0 as libc::c_int {
                    (*eff).alpha = 0 as libc::c_int as s16;
                    (*eff).type_0 = TWEFF_NONE as libc::c_int as u8_0
                }
            } else if (*eff).type_0 as libc::c_int == 3 as libc::c_int ||
                          (*eff).type_0 as libc::c_int == 2 as libc::c_int {
                if (*eff).work[EFF_ARGS as libc::c_int as usize] as
                       libc::c_int == 2 as libc::c_int {
                    (*eff).alpha =
                        ((*eff).alpha as libc::c_int - 20 as libc::c_int) as
                            s16;
                    if (*eff).alpha as libc::c_int <= 0 as libc::c_int {
                        (*eff).alpha = 0 as libc::c_int as s16;
                        (*eff).type_0 = TWEFF_NONE as libc::c_int as u8_0
                    }
                } else if (*eff).work[EFF_ARGS as libc::c_int as usize] as
                              libc::c_int == 0 as libc::c_int {
                    (*eff).alpha =
                        ((*eff).alpha as libc::c_int + 10 as libc::c_int) as
                            s16;
                    if (*eff).alpha as libc::c_int >= 100 as libc::c_int {
                        (*eff).work[EFF_ARGS as libc::c_int as usize] += 1
                    }
                } else {
                    (*eff).alpha =
                        ((*eff).alpha as libc::c_int - 3 as libc::c_int) as
                            s16;
                    if (*eff).alpha as libc::c_int <= 0 as libc::c_int {
                        (*eff).alpha = 0 as libc::c_int as s16;
                        (*eff).type_0 = TWEFF_NONE as libc::c_int as u8_0
                    }
                }
            } else if (*eff).type_0 as libc::c_int ==
                          TWEFF_FLAME as libc::c_int {
                if (*eff).work[EFF_UNKS1 as libc::c_int as usize] as
                       libc::c_int != 0 as libc::c_int {
                    (*eff).alpha =
                        ((*eff).alpha as libc::c_int -
                             (i as libc::c_int & 7 as libc::c_int) -
                             0xd as libc::c_int) as s16;
                    if (*eff).alpha as libc::c_int <= 0 as libc::c_int {
                        (*eff).alpha = 0 as libc::c_int as s16;
                        (*eff).type_0 = TWEFF_NONE as libc::c_int as u8_0
                    }
                } else {
                    (*eff).alpha =
                        ((*eff).alpha as libc::c_int + 300 as libc::c_int) as
                            s16;
                    if (*eff).alpha as libc::c_int >= 255 as libc::c_int {
                        (*eff).alpha = 255 as libc::c_int as s16;
                        (*eff).work[EFF_UNKS1 as libc::c_int as usize] += 1
                    }
                }
            } else if (*eff).type_0 as libc::c_int ==
                          TWEFF_SHLD_BLST as libc::c_int {
                D_8094C870 = 1 as libc::c_int as u8_0;
                (*eff).work[EFF_UNKS1 as libc::c_int as usize] += 1;
                if (*eff).work[EFF_UNKS1 as libc::c_int as usize] as
                       libc::c_int > 30 as libc::c_int {
                    (*eff).alpha =
                        ((*eff).alpha as libc::c_int - 10 as libc::c_int) as
                            s16;
                    if (*eff).alpha as libc::c_int <= 0 as libc::c_int {
                        (*eff).alpha = 0 as libc::c_int as s16;
                        (*eff).type_0 = TWEFF_NONE as libc::c_int as u8_0
                    }
                }
                Math_ApproachF(&mut *(*eff).workf.as_mut_ptr().offset(EFF_SCALE
                                                                          as
                                                                          libc::c_int
                                                                          as
                                                                          isize),
                               (*eff).workf[EFF_DIST as libc::c_int as usize],
                               0.1f32, 0.003f32);
                off.x = (*sTwinrovaPtr).actor.world.pos.x - (*eff).pos.x;
                off.y =
                    ((*sTwinrovaPtr).actor.world.pos.y - (*eff).pos.y) *
                        0.5f32;
                off.z = (*sTwinrovaPtr).actor.world.pos.z - (*eff).pos.z;
                if (*sTwinrovaPtr).actionFunc !=
                       Some(BossTw_TwinrovaStun as
                                unsafe extern "C" fn(_: *mut BossTw,
                                                     _: *mut GlobalContext)
                                    -> ()) {
                    if off.x * off.x + off.y * off.y + off.z * off.z <
                           60.0f32 * 60.0f32 {
                        j = 0 as libc::c_int as s16;
                        while (j as libc::c_int) < 50 as libc::c_int {
                            spF4.x =
                                (*sTwinrovaPtr).actor.world.pos.x +
                                    Rand_CenteredFloat(35.0f32);
                            spF4.y =
                                (*sTwinrovaPtr).actor.world.pos.y +
                                    Rand_CenteredFloat(70.0f32);
                            spF4.z =
                                (*sTwinrovaPtr).actor.world.pos.z +
                                    Rand_CenteredFloat(35.0f32);
                            spE8.x = Rand_CenteredFloat(20.0f32);
                            spE8.y = Rand_CenteredFloat(20.0f32);
                            spE8.z = Rand_CenteredFloat(20.0f32);
                            spDC.x = 0.0f32;
                            spDC.y = 0.0f32;
                            spDC.z = 0.0f32;
                            BossTw_AddFlameEffect(globalCtx, &mut spF4,
                                                  &mut spE8, &mut spDC,
                                                  Rand_ZeroFloat(10.0f32) +
                                                      25.0f32,
                                                  (*eff).work[EFF_ARGS as
                                                                  libc::c_int
                                                                  as usize]);
                            j += 1
                        }
                        (*sTwinrovaPtr).twinrovaStun =
                            1 as libc::c_int as u8_0;
                        (*globalCtx).envCtx.unk_D8 = 1.0f32;
                        (*eff).type_0 = TWEFF_NONE as libc::c_int as u8_0
                    }
                }
            } else if (*eff).type_0 as libc::c_int ==
                          TWEFF_MERGEFLAME as libc::c_int {
                sp11C.x = 0.0f32;
                sp11C.y = (*eff).pos.y;
                sp11C.z = (*eff).workf[EFF_DIST as libc::c_int as usize];
                Matrix_RotateY((*sTwinrovaPtr).workf[UNK_F9 as libc::c_int as
                                                         usize] +
                                   (*eff).workf[EFF_ROLL as libc::c_int as
                                                    usize],
                               MTXMODE_NEW as libc::c_int as u8_0);
                Matrix_MultVec3f(&mut sp11C, &mut (*eff).pos);
                if (*eff).work[EFF_UNKS1 as libc::c_int as usize] as
                       libc::c_int != 0 as libc::c_int {
                    (*eff).alpha =
                        ((*eff).alpha as libc::c_int - 60 as libc::c_int) as
                            s16;
                    if (*eff).alpha as libc::c_int <= 0 as libc::c_int {
                        (*eff).alpha = 0 as libc::c_int as s16;
                        (*eff).type_0 = TWEFF_NONE as libc::c_int as u8_0
                    }
                } else {
                    (*eff).alpha =
                        ((*eff).alpha as libc::c_int + 60 as libc::c_int) as
                            s16;
                    if (*eff).alpha as libc::c_int >= 255 as libc::c_int {
                        (*eff).alpha = 255 as libc::c_int as s16;
                        (*eff).work[EFF_UNKS1 as libc::c_int as usize] += 1
                    }
                }
            } else if (*eff).type_0 as libc::c_int ==
                          TWEFF_SHLD_DEFL as libc::c_int {
                (*eff).work[EFF_UNKS1 as libc::c_int as usize] += 1;
                sp11C.x = 0.0f32;
                sp11C.y = 0.0f32;
                sp11C.z = -(*eff).workf[EFF_DIST as libc::c_int as usize];
                Matrix_RotateY(sShieldHitYaw as libc::c_int as libc::c_float /
                                   32768.0f32 * 3.14159265358979323846f32,
                               MTXMODE_NEW as libc::c_int as u8_0);
                Matrix_RotateX(-0.2f32, MTXMODE_APPLY as libc::c_int as u8_0);
                Matrix_RotateZ((*eff).workf[EFF_ROLL as libc::c_int as usize],
                               MTXMODE_APPLY as libc::c_int as u8_0);
                Matrix_RotateY((*eff).workf[EFF_YAW as libc::c_int as usize],
                               MTXMODE_APPLY as libc::c_int as u8_0);
                Matrix_MultVec3f(&mut sp11C, &mut (*eff).pos);
                (*eff).pos.x += sShieldHitPos.x;
                (*eff).pos.y += sShieldHitPos.y;
                (*eff).pos.z += sShieldHitPos.z;
                if ((*eff).work[EFF_UNKS1 as libc::c_int as usize] as
                        libc::c_int) < 10 as libc::c_int {
                    Math_ApproachF(&mut *(*eff).workf.as_mut_ptr().offset(EFF_DIST
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              isize),
                                   50.0f32, 0.5f32, 100.0f32);
                } else {
                    Math_ApproachF(&mut *(*eff).workf.as_mut_ptr().offset(EFF_YAW
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              isize),
                                   0.0f32, 0.5f32, 10.0f32);
                    Math_ApproachF(&mut *(*eff).workf.as_mut_ptr().offset(EFF_DIST
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              isize),
                                   1000.0f32, 1.0f32, 10.0f32);
                    if (*eff).work[EFF_UNKS1 as libc::c_int as usize] as
                           libc::c_int >= 0x10 as libc::c_int {
                        if (*eff).work[EFF_UNKS1 as libc::c_int as usize] as
                               libc::c_int == 16 as libc::c_int &&
                               sp113 as libc::c_int == 0 as libc::c_int {
                            sp113 = 1 as libc::c_int as u8_0;
                            spD0 = (*eff).pos;
                            if (*eff).pos.y > 40.0f32 {
                                spD0.y = 220.0f32
                            } else { spD0.y = -50.0f32 }
                            phi_f0 = BossTw_GetFloorY(&mut spD0);
                            (*sTwinrovaPtr).groundBlastPos.y = phi_f0;
                            if phi_f0 >= 0.0f32 {
                                if (*sTwinrovaPtr).groundBlastPos.y != 35.0f32
                                   {
                                    (*sTwinrovaPtr).groundBlastPos.x =
                                        (*eff).pos.x;
                                    (*sTwinrovaPtr).groundBlastPos.z =
                                        (*eff).pos.z;
                                    BossTw_SpawnGroundBlast(sTwinrovaPtr,
                                                            globalCtx,
                                                            (*eff).work[EFF_ARGS
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            usize]);
                                }
                            }
                        }
                        (*eff).alpha =
                            ((*eff).alpha as libc::c_int - 300 as libc::c_int)
                                as s16;
                        if (*eff).alpha as libc::c_int <= 0 as libc::c_int {
                            (*eff).alpha = 0 as libc::c_int as s16;
                            (*eff).type_0 = TWEFF_NONE as libc::c_int as u8_0
                        }
                    }
                }
                BossTw_AddFlameEffect(globalCtx, &mut (*eff).pos,
                                      &mut sZeroVector, &mut sZeroVector,
                                      10 as libc::c_int as f32_0,
                                      (*eff).work[EFF_ARGS as libc::c_int as
                                                      usize]);
            } else if (*eff).type_0 as libc::c_int ==
                          TWEFF_SHLD_HIT as libc::c_int {
                (*eff).work[EFF_UNKS1 as libc::c_int as usize] += 1;
                sp11C.x = 0.0f32;
                sp11C.y = 0.0f32;
                sp11C.z = -(*eff).workf[EFF_DIST as libc::c_int as usize];
                Matrix_RotateY(sShieldHitYaw as libc::c_int as libc::c_float /
                                   32768.0f32 * 3.14159265358979323846f32,
                               MTXMODE_NEW as libc::c_int as u8_0);
                Matrix_RotateX(-0.2f32, MTXMODE_APPLY as libc::c_int as u8_0);
                Matrix_RotateZ((*eff).workf[EFF_ROLL as libc::c_int as usize],
                               MTXMODE_APPLY as libc::c_int as u8_0);
                Matrix_RotateY((*eff).workf[EFF_YAW as libc::c_int as usize],
                               MTXMODE_APPLY as libc::c_int as u8_0);
                Matrix_MultVec3f(&mut sp11C, &mut (*eff).pos);
                (*eff).pos.x += sShieldHitPos.x;
                (*eff).pos.y += sShieldHitPos.y;
                (*eff).pos.z += sShieldHitPos.z;
                if ((*eff).work[EFF_UNKS1 as libc::c_int as usize] as
                        libc::c_int) < 5 as libc::c_int {
                    Math_ApproachF(&mut *(*eff).workf.as_mut_ptr().offset(EFF_DIST
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              isize),
                                   40.0f32, 0.5f32, 100.0f32);
                } else {
                    Math_ApproachF(&mut *(*eff).workf.as_mut_ptr().offset(EFF_DIST
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              isize),
                                   0.0f32, 0.2f32, 5.0f32);
                    if (*eff).work[EFF_UNKS1 as libc::c_int as usize] as
                           libc::c_int >= 11 as libc::c_int {
                        (*eff).alpha =
                            ((*eff).alpha as libc::c_int - 30 as libc::c_int)
                                as s16;
                        if (*eff).alpha as libc::c_int <= 0 as libc::c_int {
                            (*eff).alpha = 0 as libc::c_int as s16;
                            (*eff).type_0 = TWEFF_NONE as libc::c_int as u8_0
                        }
                    }
                }
                BossTw_AddFlameEffect(globalCtx, &mut (*eff).pos,
                                      &mut sZeroVector, &mut sZeroVector,
                                      10 as libc::c_int as f32_0,
                                      (*eff).work[EFF_ARGS as libc::c_int as
                                                      usize]);
            } else if (*eff).type_0 as libc::c_int == 4 as libc::c_int {
                if (*eff).work[EFF_UNKS1 as libc::c_int as usize] as
                       libc::c_int == 0 as libc::c_int {
                    Math_ApproachF(&mut *(*eff).workf.as_mut_ptr().offset(EFF_SCALE
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              isize),
                                   (*eff).workf[EFF_DIST as libc::c_int as
                                                    usize], 0.05f32, 1.0f32);
                    if (*eff).frame as libc::c_int >= 16 as libc::c_int {
                        (*eff).alpha =
                            ((*eff).alpha as libc::c_int - 10 as libc::c_int)
                                as s16;
                        if (*eff).alpha as libc::c_int <= 0 as libc::c_int {
                            (*eff).alpha = 0 as libc::c_int as s16;
                            (*eff).type_0 = TWEFF_NONE as libc::c_int as u8_0
                        }
                    }
                } else {
                    Math_ApproachF(&mut *(*eff).workf.as_mut_ptr().offset(EFF_SCALE
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              isize),
                                   (*eff).workf[EFF_DIST as libc::c_int as
                                                    usize], 0.1f32, 2.0f32);
                    (*eff).alpha =
                        ((*eff).alpha as libc::c_int - 15 as libc::c_int) as
                            s16;
                    if (*eff).alpha as libc::c_int <= 0 as libc::c_int {
                        (*eff).alpha = 0 as libc::c_int as s16;
                        (*eff).type_0 = TWEFF_NONE as libc::c_int as u8_0
                    }
                }
            } else if (*eff).type_0 as libc::c_int ==
                          TWEFF_PLYR_FRZ as libc::c_int {
                if ((*eff).work[EFF_ARGS as libc::c_int as usize] as
                        libc::c_int) < (*eff).frame as libc::c_int {
                    phi_f0 = 1.0f32;
                    if !(*eff).target.is_null() ||
                           sGroundBlastType as libc::c_int == 1 as libc::c_int
                       {
                        phi_f0 *= 3.0f32
                    }
                    Math_ApproachF(&mut *(*eff).workf.as_mut_ptr().offset(EFF_SCALE
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              isize),
                                   0.0f32, 1.0f32, 0.0005f32 * phi_f0);
                    if (*eff).workf[EFF_SCALE as libc::c_int as usize] ==
                           0.0f32 {
                        (*eff).type_0 = TWEFF_NONE as libc::c_int as u8_0;
                        if (*eff).target.is_null() {
                            (*player).stateFlags2 &=
                                !(0x8000 as libc::c_int) as libc::c_uint;
                            sFreezeState = 0 as libc::c_int as u8_0
                        }
                    }
                } else {
                    if sGroundBlastType as libc::c_int == 1 as libc::c_int {
                        (*eff).frame = 100 as libc::c_int as u8_0
                    }
                    Math_ApproachF(&mut *(*eff).workf.as_mut_ptr().offset(EFF_DIST
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              isize),
                                   0.8f32, 0.2f32, 0.04f32);
                    if (*eff).target.is_null() {
                        Math_ApproachF(&mut *(*eff).workf.as_mut_ptr().offset(EFF_SCALE
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize),
                                       0.012f32, 1.0f32, 0.002f32);
                        (*eff).workf[EFF_ROLL as libc::c_int as usize] +=
                            (*eff).workf[EFF_DIST as libc::c_int as usize];
                        if (*eff).workf[EFF_ROLL as libc::c_int as usize] >=
                               0.8f32 {
                            (*eff).workf[EFF_ROLL as libc::c_int as usize] -=
                                0.8f32;
                            (*player).stateFlags2 |=
                                0x8000 as libc::c_int as libc::c_uint
                        } else {
                            (*player).stateFlags2 &=
                                !(0x8000 as libc::c_int) as libc::c_uint
                        }
                        if (*sKotakePtr).workf[UNK_F11 as libc::c_int as
                                                   usize] > 10.0f32 &&
                               (*sKotakePtr).workf[UNK_F11 as libc::c_int as
                                                       usize] < 200.0f32 {
                            (*eff).frame = 100 as libc::c_int as u8_0
                        }
                        if (*globalCtx).gameplayFrames &
                               1 as libc::c_int as libc::c_uint == 0 {
                            (*globalCtx).damagePlayer.expect("non-null function pointer")(globalCtx,
                                                                                          -(1
                                                                                                as
                                                                                                libc::c_int));
                        }
                    } else {
                        Math_ApproachF(&mut *(*eff).workf.as_mut_ptr().offset(EFF_SCALE
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize),
                                       0.042f32, 1.0f32, 0.002f32);
                    }
                    if (*eff).workf[EFF_DIST as libc::c_int as usize] > 0.4f32
                           &&
                           (*eff).frame as libc::c_int & 7 as libc::c_int ==
                               0 as libc::c_int {
                        spA6 = Rand_ZeroFloat(17.9f32) as s16;
                        if (*eff).target.is_null() {
                            spC0.x =
                                (*player).bodyPartsPos[spA6 as usize].x +
                                    Rand_CenteredFloat(5.0f32);
                            spC0.y =
                                (*player).bodyPartsPos[spA6 as usize].y +
                                    Rand_CenteredFloat(5.0f32);
                            spC0.z =
                                (*player).bodyPartsPos[spA6 as usize].z +
                                    Rand_CenteredFloat(5.0f32);
                            phi_f22 = 10.0f32
                        } else {
                            unk44 = (*eff).target;
                            spC0.x =
                                (*unk44).world.pos.x +
                                    Rand_CenteredFloat(40.0f32);
                            spC0.y =
                                (*unk44).world.pos.y +
                                    Rand_CenteredFloat(40.0f32);
                            spC0.z =
                                (*unk44).world.pos.z +
                                    Rand_CenteredFloat(40.0f32);
                            phi_f22 = 20.0f32
                        }
                        spB4.x = 0.0f32;
                        spB4.y = 0.0f32;
                        spB4.z = 0.0f32;
                        spA8.x = 0.0f32;
                        spA8.y = 0.1f32;
                        spA8.z = 0.0f32;
                        BossTw_AddDmgCloud(globalCtx, 3 as libc::c_int as s16,
                                           &mut spC0, &mut spB4, &mut spA8,
                                           phi_f22 +
                                               Rand_ZeroFloat(phi_f22 *
                                                                  0.5f32),
                                           0 as libc::c_int as s16,
                                           0 as libc::c_int as s16,
                                           150 as libc::c_int as s16);
                    }
                }
            }
        }
        eff = eff.offset(1);
        i += 1
    };
}
static mut sRandSeed0: s32 = 0;
static mut sRandSeed1: s32 = 0;
static mut sRandSeed2: s32 = 0;
#[no_mangle]
pub unsafe extern "C" fn BossTw_InitRand(mut seed0: s32, mut seed1: s32,
                                         mut seed2: s32) {
    sRandSeed0 = seed0;
    sRandSeed1 = seed1;
    sRandSeed2 = seed2;
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_RandZeroOne() -> f32_0 {
    let mut rand: f32_0 = 0.;
    // Wichmann-Hill algorithm
    sRandSeed0 = sRandSeed0 * 171 as libc::c_int % 30269 as libc::c_int;
    sRandSeed1 = sRandSeed1 * 172 as libc::c_int % 30307 as libc::c_int;
    sRandSeed2 = sRandSeed2 * 170 as libc::c_int % 30323 as libc::c_int;
    rand =
        sRandSeed0 as libc::c_float / 30269.0f32 +
            sRandSeed1 as libc::c_float / 30307.0f32 +
            sRandSeed2 as libc::c_float / 30323.0f32;
    while rand >= 1.0f32 { rand -= 1.0f32 }
    return fabsf(rand);
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_DrawEffects(mut globalCtx:
                                                *mut GlobalContext) {
    let mut sp18F: u8_0 = 0 as libc::c_int as u8_0;
    let mut i: s16 = 0;
    let mut j: s16 = 0;
    let mut pad: s32 = 0;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut phi_s4: s16 = 0;
    let mut currentEffect: *mut BossTwEffect =
        (*globalCtx).specialEffects as *mut BossTwEffect;
    let mut effectHead: *mut BossTwEffect = 0 as *mut BossTwEffect;
    let mut gfxCtx: *mut GraphicsContext = (*globalCtx).state.gfxCtx;
    effectHead = currentEffect;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), gfxCtx,
                    b"../z_boss_tw.c\x00" as *const u8 as *const libc::c_char,
                    9592 as libc::c_int);
    func_80093D84((*globalCtx).state.gfxCtx);
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[BossTwEffect; 150]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<BossTwEffect>()
                                                   as libc::c_ulong) as s32 {
        if (*currentEffect).type_0 as libc::c_int == 1 as libc::c_int {
            if sp18F as libc::c_int == 0 as libc::c_int {
                let fresh150 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g: *mut Gfx = fresh150;
                (*_g).words.w0 =
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
                (*_g).words.w1 =
                    object_tw_DL_01A528.as_mut_ptr() as libc::c_uint;
                sp18F = sp18F.wrapping_add(1)
            }
            let fresh151 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_0: *mut Gfx = fresh151;
            (*_g_0).words.w0 =
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
            (*_g_0).words.w1 =
                ((*currentEffect).color.r as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((*currentEffect).color.g as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    ((*currentEffect).color.b as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    ((*currentEffect).alpha as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            Matrix_Translate((*currentEffect).pos.x, (*currentEffect).pos.y,
                             (*currentEffect).pos.z,
                             MTXMODE_NEW as libc::c_int as u8_0);
            func_800D1FD4(&mut (*globalCtx).billboardMtxF);
            Matrix_Scale((*currentEffect).workf[EFF_SCALE as libc::c_int as
                                                    usize],
                         (*currentEffect).workf[EFF_SCALE as libc::c_int as
                                                    usize], 1.0f32,
                         MTXMODE_APPLY as libc::c_int as u8_0);
            let fresh152 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_1: *mut Gfx = fresh152;
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
                Matrix_NewMtx(gfxCtx,
                              b"../z_boss_tw.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              9617 as libc::c_int) as libc::c_uint;
            let fresh153 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_2: *mut Gfx = fresh153;
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
            (*_g_2).words.w1 =
                object_tw_DL_01A5A8.as_mut_ptr() as libc::c_uint
        }
        currentEffect = currentEffect.offset(1);
        i += 1
    }
    sp18F = 0 as libc::c_int as u8_0;
    currentEffect = effectHead;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[BossTwEffect; 150]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<BossTwEffect>()
                                                   as libc::c_ulong) as s32 {
        if (*currentEffect).type_0 as libc::c_int == 3 as libc::c_int {
            if sp18F as libc::c_int == 0 as libc::c_int {
                let fresh154 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_3: *mut Gfx = fresh154;
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
                    gSegments[((object_tw_DL_01A998.as_mut_ptr() as u32_0) <<
                                   4 as libc::c_int >> 28 as libc::c_int) as
                                  usize].wrapping_add(object_tw_DL_01A998.as_mut_ptr()
                                                          as u32_0 &
                                                          0xffffff as
                                                              libc::c_int as
                                                              libc::c_uint).wrapping_add(0x80000000
                                                                                             as
                                                                                             libc::c_uint)
                        as *mut libc::c_void as libc::c_uint;
                sp18F = sp18F.wrapping_add(1)
            }
            let fresh155 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_4: *mut Gfx = fresh155;
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
                (195 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (225 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (235 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    ((*currentEffect).alpha as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh156 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_5: *mut Gfx = fresh156;
            (*_g_5).words.w0 =
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
            (*_g_5).words.w1 =
                Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                                 ((*currentEffect).frame as libc::c_int *
                                      3 as libc::c_int & 0x7f as libc::c_int)
                                     as u32_0,
                                 ((*currentEffect).frame as libc::c_int *
                                      15 as libc::c_int & 0xff as libc::c_int)
                                     as u32_0, 0x20 as libc::c_int,
                                 0x40 as libc::c_int, 1 as libc::c_int,
                                 0 as libc::c_int as u32_0,
                                 0 as libc::c_int as u32_0,
                                 0x20 as libc::c_int, 0x20 as libc::c_int) as
                    libc::c_uint;
            Matrix_Translate((*currentEffect).pos.x, (*currentEffect).pos.y,
                             (*currentEffect).pos.z,
                             MTXMODE_NEW as libc::c_int as u8_0);
            func_800D1FD4(&mut (*globalCtx).billboardMtxF);
            Matrix_Scale((*currentEffect).workf[EFF_SCALE as libc::c_int as
                                                    usize],
                         (*currentEffect).workf[EFF_SCALE as libc::c_int as
                                                    usize], 1.0f32,
                         MTXMODE_APPLY as libc::c_int as u8_0);
            let fresh157 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_6: *mut Gfx = fresh157;
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
                           0 as libc::c_int) ^ 0x1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_6).words.w1 =
                Matrix_NewMtx(gfxCtx,
                              b"../z_boss_tw.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              9660 as libc::c_int) as libc::c_uint;
            let fresh158 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_7: *mut Gfx = fresh158;
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
            (*_g_7).words.w1 =
                gSegments[((object_tw_DL_01AB00.as_mut_ptr() as u32_0) <<
                               4 as libc::c_int >> 28 as libc::c_int) as
                              usize].wrapping_add(object_tw_DL_01AB00.as_mut_ptr()
                                                      as u32_0 &
                                                      0xffffff as libc::c_int
                                                          as
                                                          libc::c_uint).wrapping_add(0x80000000
                                                                                         as
                                                                                         libc::c_uint)
                    as *mut libc::c_void as libc::c_uint
        }
        currentEffect = currentEffect.offset(1);
        i += 1
    }
    sp18F = 0 as libc::c_int as u8_0;
    currentEffect = effectHead;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[BossTwEffect; 150]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<BossTwEffect>()
                                                   as libc::c_ulong) as s32 {
        if (*currentEffect).type_0 as libc::c_int == 2 as libc::c_int {
            if sp18F as libc::c_int == 0 as libc::c_int {
                let fresh159 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_8: *mut Gfx = fresh159;
                (*_g_8).words.w0 =
                    (0xe7 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int;
                (*_g_8).words.w1 = 0 as libc::c_int as libc::c_uint;
                let fresh160 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_9: *mut Gfx = fresh160;
                (*_g_9).words.w0 =
                    (0xfb as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int;
                (*_g_9).words.w1 =
                    (255 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (215 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            16 as libc::c_int |
                        (255 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            8 as libc::c_int |
                        (128 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                sp18F = sp18F.wrapping_add(1)
            }
            let fresh161 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_10: *mut Gfx = fresh161;
            (*_g_10).words.w0 =
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
            (*_g_10).words.w1 =
                (200 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (20 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    ((*currentEffect).alpha as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh162 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_11: *mut Gfx = fresh162;
            (*_g_11).words.w0 =
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
            (*_g_11).words.w1 =
                Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                                 ((*currentEffect).frame as libc::c_int *
                                      3 as libc::c_int & 0x7f as libc::c_int)
                                     as u32_0,
                                 ((*currentEffect).frame as libc::c_int *
                                      15 as libc::c_int & 0xff as libc::c_int)
                                     as u32_0, 0x20 as libc::c_int,
                                 0x40 as libc::c_int, 1 as libc::c_int,
                                 0 as libc::c_int as u32_0,
                                 0 as libc::c_int as u32_0,
                                 0x20 as libc::c_int, 0x20 as libc::c_int) as
                    libc::c_uint;
            Matrix_Translate((*currentEffect).pos.x, (*currentEffect).pos.y,
                             (*currentEffect).pos.z,
                             MTXMODE_NEW as libc::c_int as u8_0);
            func_800D1FD4(&mut (*globalCtx).billboardMtxF);
            Matrix_Scale((*currentEffect).workf[EFF_SCALE as libc::c_int as
                                                    usize],
                         (*currentEffect).workf[EFF_SCALE as libc::c_int as
                                                    usize], 1.0f32,
                         MTXMODE_APPLY as libc::c_int as u8_0);
            let fresh163 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_12: *mut Gfx = fresh163;
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
                           0 as libc::c_int) ^ 0x1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_12).words.w1 =
                Matrix_NewMtx(gfxCtx,
                              b"../z_boss_tw.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              9709 as libc::c_int) as libc::c_uint;
            let fresh164 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_13: *mut Gfx = fresh164;
            (*_g_13).words.w0 =
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
            (*_g_13).words.w1 =
                gSegments[((object_tw_DL_01A430.as_mut_ptr() as u32_0) <<
                               4 as libc::c_int >> 28 as libc::c_int) as
                              usize].wrapping_add(object_tw_DL_01A430.as_mut_ptr()
                                                      as u32_0 &
                                                      0xffffff as libc::c_int
                                                          as
                                                          libc::c_uint).wrapping_add(0x80000000
                                                                                         as
                                                                                         libc::c_uint)
                    as *mut libc::c_void as libc::c_uint
        }
        currentEffect = currentEffect.offset(1);
        i += 1
    }
    sp18F = 0 as libc::c_int as u8_0;
    currentEffect = effectHead;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[BossTwEffect; 150]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<BossTwEffect>()
                                                   as libc::c_ulong) as s32 {
        if (*currentEffect).type_0 as libc::c_int == 4 as libc::c_int {
            if sp18F as libc::c_int == 0 as libc::c_int {
                sp18F = sp18F.wrapping_add(1)
            }
            let fresh165 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_14: *mut Gfx = fresh165;
            (*_g_14).words.w0 =
                (0xdb as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0x6 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    ((0xd as libc::c_int * 4 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_14).words.w1 =
                Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                                 ((*currentEffect).frame as libc::c_int &
                                      0x7f as libc::c_int) as u32_0,
                                 ((*currentEffect).frame as libc::c_int *
                                      8 as libc::c_int & 0xff as libc::c_int)
                                     as u32_0, 0x20 as libc::c_int,
                                 0x40 as libc::c_int, 1 as libc::c_int,
                                 ((*currentEffect).frame as libc::c_int *
                                      -(2 as libc::c_int) &
                                      0x7f as libc::c_int) as u32_0,
                                 0 as libc::c_int as u32_0,
                                 0x10 as libc::c_int, 0x10 as libc::c_int) as
                    libc::c_uint;
            if (*currentEffect).work[EFF_ARGS as libc::c_int as usize] as
                   libc::c_int == 1 as libc::c_int {
                let fresh166 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_15: *mut Gfx = fresh166;
                (*_g_15).words.w0 =
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
                (*_g_15).words.w1 =
                    (255 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (65 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            16 as libc::c_int |
                        (0 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            8 as libc::c_int |
                        ((*currentEffect).alpha as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                let fresh167 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_16: *mut Gfx = fresh167;
                (*_g_16).words.w0 =
                    (0xe7 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int;
                (*_g_16).words.w1 = 0 as libc::c_int as libc::c_uint;
                let fresh168 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_17: *mut Gfx = fresh168;
                (*_g_17).words.w0 =
                    (0xfb as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int;
                (*_g_17).words.w1 =
                    (255 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (255 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            16 as libc::c_int |
                        (0 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            8 as libc::c_int |
                        (128 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int
            } else {
                let fresh169 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_18: *mut Gfx = fresh169;
                (*_g_18).words.w0 =
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
                (*_g_18).words.w1 =
                    (195 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (225 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            16 as libc::c_int |
                        (235 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            8 as libc::c_int |
                        ((*currentEffect).alpha as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                let fresh170 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_19: *mut Gfx = fresh170;
                (*_g_19).words.w0 =
                    (0xfb as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int;
                (*_g_19).words.w1 =
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
                        (128 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int
            }
            Matrix_Translate((*currentEffect).pos.x, (*currentEffect).pos.y,
                             (*currentEffect).pos.z,
                             MTXMODE_NEW as libc::c_int as u8_0);
            func_800D1FD4(&mut (*globalCtx).billboardMtxF);
            if (*currentEffect).work[EFF_UNKS1 as libc::c_int as usize] as
                   libc::c_int == 0 as libc::c_int {
                Matrix_Translate(0.0f32, 0.0f32, 60.0f32,
                                 MTXMODE_APPLY as libc::c_int as u8_0);
            } else {
                Matrix_Translate(0.0f32, 0.0f32, 0.0f32,
                                 MTXMODE_APPLY as libc::c_int as u8_0);
            }
            Matrix_RotateZ((*currentEffect).workf[EFF_ROLL as libc::c_int as
                                                      usize],
                           MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_RotateX(3.14159265358979323846f32 / 2.0f32,
                           MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_Scale((*currentEffect).workf[EFF_SCALE as libc::c_int as
                                                    usize], 1.0f32,
                         (*currentEffect).workf[EFF_SCALE as libc::c_int as
                                                    usize],
                         MTXMODE_APPLY as libc::c_int as u8_0);
            let fresh171 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_20: *mut Gfx = fresh171;
            (*_g_20).words.w0 =
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
            (*_g_20).words.w1 =
                Matrix_NewMtx(gfxCtx,
                              b"../z_boss_tw.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              9775 as libc::c_int) as libc::c_uint;
            let fresh172 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_21: *mut Gfx = fresh172;
            (*_g_21).words.w0 =
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
            (*_g_21).words.w1 =
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
            let fresh173 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_22: *mut Gfx = fresh173;
            (*_g_22).words.w0 =
                (0xd9 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (!((0x400 as libc::c_int | 0x10000 as libc::c_int) as
                           u32_0) &
                         (((0x1 as libc::c_int) << 24 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_22).words.w1 = 0 as libc::c_int as u32_0;
            let fresh174 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_23: *mut Gfx = fresh174;
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
                gSegments[((object_tw_DL_01A790.as_mut_ptr() as u32_0) <<
                               4 as libc::c_int >> 28 as libc::c_int) as
                              usize].wrapping_add(object_tw_DL_01A790.as_mut_ptr()
                                                      as u32_0 &
                                                      0xffffff as libc::c_int
                                                          as
                                                          libc::c_uint).wrapping_add(0x80000000
                                                                                         as
                                                                                         libc::c_uint)
                    as *mut libc::c_void as libc::c_uint
        }
        currentEffect = currentEffect.offset(1);
        i += 1
    }
    sp18F = 0 as libc::c_int as u8_0;
    currentEffect = effectHead;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[BossTwEffect; 150]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<BossTwEffect>()
                                                   as libc::c_ulong) as s32 {
        let mut actor: *mut Actor = 0 as *mut Actor;
        let mut off: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
        if (*currentEffect).type_0 as libc::c_int ==
               TWEFF_PLYR_FRZ as libc::c_int {
            if sp18F as libc::c_int == 0 as libc::c_int {
                let fresh175 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_24: *mut Gfx = fresh175;
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
                    gSegments[((object_tw_DL_01AA50.as_mut_ptr() as u32_0) <<
                                   4 as libc::c_int >> 28 as libc::c_int) as
                                  usize].wrapping_add(object_tw_DL_01AA50.as_mut_ptr()
                                                          as u32_0 &
                                                          0xffffff as
                                                              libc::c_int as
                                                              libc::c_uint).wrapping_add(0x80000000
                                                                                             as
                                                                                             libc::c_uint)
                        as *mut libc::c_void as libc::c_uint;
                let fresh176 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_25: *mut Gfx = fresh176;
                (*_g_25).words.w0 =
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
                (*_g_25).words.w1 =
                    (195 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (225 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            16 as libc::c_int |
                        (235 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            8 as libc::c_int |
                        (255 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                let fresh177 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_26: *mut Gfx = fresh177;
                (*_g_26).words.w0 =
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
                (*_g_26).words.w1 =
                    Gfx_TwoTexScroll((*globalCtx).state.gfxCtx,
                                     0 as libc::c_int,
                                     0 as libc::c_int as u32_0,
                                     0 as libc::c_int as u32_0,
                                     0x20 as libc::c_int, 0x40 as libc::c_int,
                                     1 as libc::c_int,
                                     0 as libc::c_int as u32_0,
                                     0 as libc::c_int as u32_0,
                                     0x20 as libc::c_int, 0x20 as libc::c_int)
                        as libc::c_uint;
                sp18F = sp18F.wrapping_add(1);
                BossTw_InitRand(1 as libc::c_int, 0x71ac as libc::c_int,
                                0x263a as libc::c_int);
            }
            actor = (*currentEffect).target;
            phi_s4 =
                if actor.is_null() {
                    70 as libc::c_int
                } else { 20 as libc::c_int } as s16;
            j = 0 as libc::c_int as s16;
            while (j as libc::c_int) < phi_s4 as libc::c_int {
                off.x = (BossTw_RandZeroOne() - 0.5f32) * 30.0f32;
                off.y =
                    (*currentEffect).workf[EFF_DIST as libc::c_int as usize] *
                        j as libc::c_int as libc::c_float;
                off.z = (BossTw_RandZeroOne() - 0.5f32) * 30.0f32;
                if !actor.is_null() {
                    Matrix_Translate((*actor).world.pos.x + off.x,
                                     (*actor).world.pos.y + off.y,
                                     (*actor).world.pos.z + off.z,
                                     MTXMODE_NEW as libc::c_int as u8_0);
                } else {
                    Matrix_Translate((*player).actor.world.pos.x + off.x,
                                     (*player).actor.world.pos.y + off.y,
                                     (*player).actor.world.pos.z + off.z,
                                     MTXMODE_NEW as libc::c_int as u8_0);
                }
                Matrix_Scale((*currentEffect).workf[EFF_SCALE as libc::c_int
                                                        as usize],
                             (*currentEffect).workf[EFF_SCALE as libc::c_int
                                                        as usize],
                             (*currentEffect).workf[EFF_SCALE as libc::c_int
                                                        as usize],
                             MTXMODE_APPLY as libc::c_int as u8_0);
                Matrix_RotateY(BossTw_RandZeroOne() *
                                   3.14159265358979323846f32,
                               MTXMODE_APPLY as libc::c_int as u8_0);
                Matrix_RotateX((BossTw_RandZeroOne() - 0.5f32) *
                                   3.14159265358979323846f32 * 0.5f32,
                               MTXMODE_APPLY as libc::c_int as u8_0);
                let fresh178 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_27: *mut Gfx = fresh178;
                (*_g_27).words.w0 =
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
                (*_g_27).words.w1 =
                    Matrix_NewMtx(gfxCtx,
                                  b"../z_boss_tw.c\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char, 9855 as libc::c_int)
                        as libc::c_uint;
                let fresh179 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_28: *mut Gfx = fresh179;
                (*_g_28).words.w0 =
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
                (*_g_28).words.w1 =
                    gSegments[((object_tw_DL_01AB00.as_mut_ptr() as u32_0) <<
                                   4 as libc::c_int >> 28 as libc::c_int) as
                                  usize].wrapping_add(object_tw_DL_01AB00.as_mut_ptr()
                                                          as u32_0 &
                                                          0xffffff as
                                                              libc::c_int as
                                                              libc::c_uint).wrapping_add(0x80000000
                                                                                             as
                                                                                             libc::c_uint)
                        as *mut libc::c_void as libc::c_uint;
                j += 1
            }
        }
        currentEffect = currentEffect.offset(1);
        i += 1
    }
    sp18F = 0 as libc::c_int as u8_0;
    currentEffect = effectHead;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[BossTwEffect; 150]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<BossTwEffect>()
                                                   as libc::c_ulong) as s32 {
        if (*currentEffect).type_0 as libc::c_int >= 6 as libc::c_int {
            if (*currentEffect).work[EFF_ARGS as libc::c_int as usize] as
                   libc::c_int == 0 as libc::c_int {
                let fresh180 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_29: *mut Gfx = fresh180;
                (*_g_29).words.w0 =
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
                (*_g_29).words.w1 =
                    (195 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (225 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            16 as libc::c_int |
                        (235 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            8 as libc::c_int |
                        ((*currentEffect).alpha as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                let fresh181 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_30: *mut Gfx = fresh181;
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
                    gSegments[((object_tw_DL_01A998.as_mut_ptr() as u32_0) <<
                                   4 as libc::c_int >> 28 as libc::c_int) as
                                  usize].wrapping_add(object_tw_DL_01A998.as_mut_ptr()
                                                          as u32_0 &
                                                          0xffffff as
                                                              libc::c_int as
                                                              libc::c_uint).wrapping_add(0x80000000
                                                                                             as
                                                                                             libc::c_uint)
                        as *mut libc::c_void as libc::c_uint
            } else {
                let fresh182 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_31: *mut Gfx = fresh182;
                (*_g_31).words.w0 =
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
                (*_g_31).words.w1 =
                    (200 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (20 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            16 as libc::c_int |
                        (0 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            8 as libc::c_int |
                        ((*currentEffect).alpha as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                let fresh183 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_32: *mut Gfx = fresh183;
                (*_g_32).words.w0 =
                    (0xe7 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int;
                (*_g_32).words.w1 = 0 as libc::c_int as libc::c_uint;
                let fresh184 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_33: *mut Gfx = fresh184;
                (*_g_33).words.w0 =
                    (0xfb as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int;
                (*_g_33).words.w1 =
                    (255 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (215 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            16 as libc::c_int |
                        (255 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            8 as libc::c_int |
                        (128 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int
            }
            let fresh185 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_34: *mut Gfx = fresh185;
            (*_g_34).words.w0 =
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
            (*_g_34).words.w1 =
                Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                                 ((*currentEffect).frame as libc::c_int *
                                      3 as libc::c_int & 0x7f as libc::c_int)
                                     as u32_0,
                                 (-((*currentEffect).frame as libc::c_int) *
                                      15 as libc::c_int & 0xff as libc::c_int)
                                     as u32_0, 0x20 as libc::c_int,
                                 0x40 as libc::c_int, 1 as libc::c_int,
                                 0 as libc::c_int as u32_0,
                                 0 as libc::c_int as u32_0,
                                 0x20 as libc::c_int, 0x20 as libc::c_int) as
                    libc::c_uint;
            Matrix_Translate((*currentEffect).pos.x, (*currentEffect).pos.y,
                             (*currentEffect).pos.z,
                             MTXMODE_NEW as libc::c_int as u8_0);
            func_800D1FD4(&mut (*globalCtx).billboardMtxF);
            Matrix_Scale((*currentEffect).workf[EFF_SCALE as libc::c_int as
                                                    usize],
                         (*currentEffect).workf[EFF_SCALE as libc::c_int as
                                                    usize], 1.0f32,
                         MTXMODE_APPLY as libc::c_int as u8_0);
            let fresh186 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_35: *mut Gfx = fresh186;
            (*_g_35).words.w0 =
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
            (*_g_35).words.w1 =
                Matrix_NewMtx(gfxCtx,
                              b"../z_boss_tw.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              9911 as libc::c_int) as libc::c_uint;
            if (*currentEffect).work[EFF_ARGS as libc::c_int as usize] as
                   libc::c_int == 0 as libc::c_int {
                let fresh187 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_36: *mut Gfx = fresh187;
                (*_g_36).words.w0 =
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
                (*_g_36).words.w1 =
                    gSegments[((object_tw_DL_01AB00.as_mut_ptr() as u32_0) <<
                                   4 as libc::c_int >> 28 as libc::c_int) as
                                  usize].wrapping_add(object_tw_DL_01AB00.as_mut_ptr()
                                                          as u32_0 &
                                                          0xffffff as
                                                              libc::c_int as
                                                              libc::c_uint).wrapping_add(0x80000000
                                                                                             as
                                                                                             libc::c_uint)
                        as *mut libc::c_void as libc::c_uint
            } else {
                let fresh188 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_37: *mut Gfx = fresh188;
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
                    gSegments[((object_tw_DL_01A430.as_mut_ptr() as u32_0) <<
                                   4 as libc::c_int >> 28 as libc::c_int) as
                                  usize].wrapping_add(object_tw_DL_01A430.as_mut_ptr()
                                                          as u32_0 &
                                                          0xffffff as
                                                              libc::c_int as
                                                              libc::c_uint).wrapping_add(0x80000000
                                                                                             as
                                                                                             libc::c_uint)
                        as *mut libc::c_void as libc::c_uint
            }
        }
        currentEffect = currentEffect.offset(1);
        i += 1
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), gfxCtx,
                     b"../z_boss_tw.c\x00" as *const u8 as
                         *const libc::c_char, 9920 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_TwinrovaSetupArriveAtTarget(mut this:
                                                                *mut BossTw,
                                                            mut globalCtx:
                                                                *mut GlobalContext) {
    (*this).actionFunc =
        Some(BossTw_TwinrovaArriveAtTarget as
                 unsafe extern "C" fn(_: *mut BossTw, _: *mut GlobalContext)
                     -> ());
    Animation_MorphToLoop(&mut (*this).skelAnime, &mut object_tw_Anim_032BF8,
                          -3.0f32);
    (*this).work[CS_TIMER_1 as libc::c_int as usize] =
        Rand_ZeroFloat(100.0f32) as s16;
    (*this).timers[1 as libc::c_int as usize] = 25 as libc::c_int as s16;
    (*this).rotateSpeed = 0.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_TwinrovaArriveAtTarget(mut this: *mut BossTw,
                                                       mut globalCtx:
                                                           *mut GlobalContext) {
    SkelAnime_Update(&mut (*this).skelAnime);
    Math_ApproachF(&mut (*this).actor.world.pos.x, (*this).targetPos.x,
                   0.1f32, fabsf((*this).actor.velocity.x) * 1.5f32);
    Math_ApproachF(&mut (*this).actor.world.pos.y, (*this).targetPos.y,
                   0.1f32, fabsf((*this).actor.velocity.y) * 1.5f32);
    Math_ApproachF(&mut (*this).targetPos.y, 380.0f32, 1.0f32, 2.0f32);
    Math_ApproachF(&mut (*this).actor.world.pos.z, (*this).targetPos.z,
                   0.1f32, fabsf((*this).actor.velocity.z) * 1.5f32);
    if (*this).timers[1 as libc::c_int as usize] as libc::c_int ==
           1 as libc::c_int {
        BossTw_TwinrovaSetupChargeBlast(this, globalCtx);
    }
    Math_ApproachS(&mut (*this).actor.shape.rot.y,
                   (*this).actor.yawTowardsPlayer, 5 as libc::c_int as s16,
                   (*this).rotateSpeed as s16);
    Math_ApproachF(&mut (*this).rotateSpeed, 4096.0f32, 1.0f32, 350.0f32);
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_TwinrovaSetupChargeBlast(mut this:
                                                             *mut BossTw,
                                                         mut globalCtx:
                                                             *mut GlobalContext) {
    (*this).actionFunc =
        Some(BossTw_TwinrovaChargeBlast as
                 unsafe extern "C" fn(_: *mut BossTw, _: *mut GlobalContext)
                     -> ());
    Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                              &mut object_tw_Anim_036FBC, -5.0f32);
    (*this).workf[ANIM_SW_TGT as libc::c_int as usize] =
        Animation_GetLastFrame(&mut object_tw_Anim_036FBC as
                                   *mut AnimationHeader as *mut libc::c_void)
            as f32_0;
    (*this).csState1 = 0 as libc::c_int as s16;
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_TwinrovaChargeBlast(mut this: *mut BossTw,
                                                    mut globalCtx:
                                                        *mut GlobalContext) {
    SkelAnime_Update(&mut (*this).skelAnime);
    Math_ApproachF(&mut (*this).actor.world.pos.x, (*this).targetPos.x,
                   0.03f32, fabsf((*this).actor.velocity.x) * 1.5f32);
    Math_ApproachF(&mut (*this).actor.world.pos.y, (*this).targetPos.y,
                   0.03f32, fabsf((*this).actor.velocity.y) * 1.5f32);
    Math_ApproachF(&mut (*this).actor.world.pos.z, (*this).targetPos.z,
                   0.03f32, fabsf((*this).actor.velocity.z) * 1.5f32);
    Math_ApproachS(&mut (*this).actor.shape.rot.y,
                   (*this).actor.yawTowardsPlayer, 5 as libc::c_int as s16,
                   0x1000 as libc::c_int as s16);
    if Animation_OnFrame(&mut (*this).skelAnime,
                         (*this).workf[ANIM_SW_TGT as libc::c_int as usize])
           != 0 {
        if ((*this).actor.colChkInfo.health as s8 as libc::c_int) <
               10 as libc::c_int {
            sTwinrovaBlastType = Rand_ZeroFloat(1.99f32) as u8_0
        } else {
            sFixedBlatSeq = sFixedBlatSeq.wrapping_add(1);
            if sFixedBlatSeq as libc::c_int >= 4 as libc::c_int {
                sFixedBlatSeq = 1 as libc::c_int as u8_0;
                sFixedBlastType =
                    (sFixedBlastType == 0) as libc::c_int as u8_0
            }
            sTwinrovaBlastType = sFixedBlastType
        }
        BossTw_TwinrovaSetupShootBlast(this, globalCtx);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_TwinrovaSetupShootBlast(mut this: *mut BossTw,
                                                        mut globalCtx:
                                                            *mut GlobalContext) {
    (*this).actionFunc =
        Some(BossTw_TwinrovaShootBlast as
                 unsafe extern "C" fn(_: *mut BossTw, _: *mut GlobalContext)
                     -> ());
    if sTwinrovaBlastType as libc::c_int == 0 as libc::c_int {
        Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                                  &mut object_tw_Anim_022700, 0.0f32);
    } else {
        Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                                  &mut object_tw_Anim_023750, 0.0f32);
    }
    (*this).workf[ANIM_SW_TGT as libc::c_int as usize] =
        Animation_GetLastFrame(&mut object_tw_Anim_023750 as
                                   *mut AnimationHeader as *mut libc::c_void)
            as f32_0;
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_TwinrovaShootBlast(mut this: *mut BossTw,
                                                   mut globalCtx:
                                                       *mut GlobalContext) {
    let mut twMagic: *mut BossTw = 0 as *mut BossTw;
    let mut magicSpawnPos: *mut Vec3f = 0 as *mut Vec3f;
    let mut magicParams: s32 = 0;
    let mut i: s16 = 0;
    SkelAnime_Update(&mut (*this).skelAnime);
    if Animation_OnFrame(&mut (*this).skelAnime, 8.0f32) != 0 {
        Audio_PlayActorSound2(&mut (*this).actor,
                              0x3923 as libc::c_int as u16_0);
        Audio_PlayActorSound2(&mut (*this).actor,
                              0x39b8 as libc::c_int as u16_0);
    }
    if Animation_OnFrame(&mut (*this).skelAnime, 12.0f32) != 0 {
        if sTwinrovaBlastType as libc::c_int != 0 as libc::c_int {
            magicParams = TW_FIRE_BLAST as libc::c_int;
            magicSpawnPos = &mut (*this).rightScepterPos
        } else {
            magicParams = TW_ICE_BLAST as libc::c_int;
            magicSpawnPos = &mut (*this).leftScepterPos
        }
        twMagic =
            Actor_SpawnAsChild(&mut (*globalCtx).actorCtx, &mut (*this).actor,
                               globalCtx, ACTOR_BOSS_TW as libc::c_int as s16,
                               (*magicSpawnPos).x, (*magicSpawnPos).y,
                               (*magicSpawnPos).z, 0 as libc::c_int as s16,
                               0 as libc::c_int as s16,
                               0 as libc::c_int as s16, magicParams as s16) as
                *mut BossTw;
        if !twMagic.is_null() {
            (*twMagic).blastType =
                if magicParams == TW_ICE_BLAST as libc::c_int {
                    0 as libc::c_int
                } else { 1 as libc::c_int } as s16
        }
        sEnvType =
            ((*twMagic).blastType as libc::c_int + 1 as libc::c_int) as s8;
        let mut velocity: Vec3f =
            { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
        let mut accel: Vec3f =
            { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
        i = 0 as libc::c_int as s16;
        while (i as libc::c_int) < 100 as libc::c_int {
            velocity.x = Rand_CenteredFloat(30.0f32);
            velocity.y = Rand_CenteredFloat(30.0f32);
            velocity.z = Rand_CenteredFloat(30.0f32);
            BossTw_AddDotEffect(globalCtx, magicSpawnPos, &mut velocity,
                                &mut accel,
                                (Rand_ZeroFloat(2.0f32) as s16 as libc::c_int
                                     + 11 as libc::c_int) as f32_0,
                                (*twMagic).blastType,
                                75 as libc::c_int as s16);
            i += 1
        }
    }
    if Animation_OnFrame(&mut (*this).skelAnime,
                         (*this).workf[ANIM_SW_TGT as libc::c_int as usize])
           != 0 {
        BossTw_TwinrovaSetupDoneBlastShoot(this, globalCtx);
    }
    Math_ApproachS(&mut (*this).actor.shape.rot.y,
                   (*this).actor.yawTowardsPlayer, 5 as libc::c_int as s16,
                   0x1000 as libc::c_int as s16);
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_TwinrovaSetupDoneBlastShoot(mut this:
                                                                *mut BossTw,
                                                            mut globalCtx:
                                                                *mut GlobalContext) {
    (*this).actionFunc =
        Some(BossTw_TwinrovaDoneBlastShoot as
                 unsafe extern "C" fn(_: *mut BossTw, _: *mut GlobalContext)
                     -> ());
    Animation_MorphToLoop(&mut (*this).skelAnime, &mut object_tw_Anim_032BF8,
                          -10.0f32);
    (*this).timers[1 as libc::c_int as usize] = 60 as libc::c_int as s16;
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_TwinrovaDoneBlastShoot(mut this: *mut BossTw,
                                                       mut globalCtx:
                                                           *mut GlobalContext) {
    SkelAnime_Update(&mut (*this).skelAnime);
    if (*this).timers[1 as libc::c_int as usize] as libc::c_int ==
           0 as libc::c_int && D_8094C870 as libc::c_int == 0 as libc::c_int {
        if (*sTwinrovaPtr).timers[2 as libc::c_int as usize] as libc::c_int ==
               0 as libc::c_int {
            BossTw_TwinrovaSetupFly(this, globalCtx);
        } else { BossTw_TwinrovaSetupLaugh(this, globalCtx); }
    }
    D_8094C870 = 0 as libc::c_int as u8_0;
    Math_ApproachS(&mut (*this).actor.shape.rot.y,
                   (*this).actor.yawTowardsPlayer, 5 as libc::c_int as s16,
                   0x1000 as libc::c_int as s16);
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_TwinrovaDamage(mut this: *mut BossTw,
                                               mut globalCtx:
                                                   *mut GlobalContext,
                                               mut damage: u8_0) {
    if (*this).actionFunc !=
           Some(BossTw_TwinrovaStun as
                    unsafe extern "C" fn(_: *mut BossTw,
                                         _: *mut GlobalContext) -> ()) {
        Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                                  &mut object_tw_Anim_0338F0, -15.0f32);
        (*this).timers[0 as libc::c_int as usize] = 150 as libc::c_int as s16;
        (*this).timers[1 as libc::c_int as usize] = 20 as libc::c_int as s16;
        (*this).csState1 = 0 as libc::c_int as s16;
        (*this).actor.velocity.y = 0.0f32
    } else {
        (*this).work[FOG_TIMER as libc::c_int as usize] =
            10 as libc::c_int as s16;
        (*this).work[INVINC_TIMER as libc::c_int as usize] =
            20 as libc::c_int as s16;
        Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                                  &mut object_tw_Anim_024374, -3.0f32);
        (*this).workf[ANIM_SW_TGT as libc::c_int as usize] =
            Animation_GetLastFrame(&mut object_tw_Anim_024374 as
                                       *mut AnimationHeader as
                                       *mut libc::c_void) as f32_0;
        (*this).csState1 = 1 as libc::c_int as s16;
        (*this).actor.colChkInfo.health =
            ((*this).actor.colChkInfo.health as libc::c_int -
                 damage as libc::c_int) as u8_0;
        if ((*this).actor.colChkInfo.health as s8 as libc::c_int) <
               0 as libc::c_int {
            (*this).actor.colChkInfo.health = 0 as libc::c_int as u8_0
        }
        if (*this).actor.colChkInfo.health as s8 as libc::c_int <=
               0 as libc::c_int {
            BossTw_TwinrovaSetupDeathCS(this, globalCtx);
            Enemy_StartFinishingBlow(globalCtx, &mut (*this).actor);
            Audio_PlayActorSound2(&mut (*this).actor,
                                  0x391b as libc::c_int as u16_0);
            return
        }
        Audio_PlayActorSound2(&mut (*this).actor,
                              0x39b7 as libc::c_int as u16_0);
        Audio_PlayActorSound2(&mut (*this).actor,
                              0x3914 as libc::c_int as u16_0);
    }
    (*this).actionFunc =
        Some(BossTw_TwinrovaStun as
                 unsafe extern "C" fn(_: *mut BossTw, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_TwinrovaStun(mut this: *mut BossTw,
                                             mut globalCtx:
                                                 *mut GlobalContext) {
    let mut cloudType: s16 = 0;
    (*this).unk_5F8 = 1 as libc::c_int as u8_0;
    (*this).actor.flags |=
        ((1 as libc::c_int) << 10 as libc::c_int) as libc::c_uint;
    cloudType =
        if sTwinrovaBlastType as libc::c_int == 0 as libc::c_int {
            3 as libc::c_int
        } else { 2 as libc::c_int } as s16;
    if (*this).work[CS_TIMER_1 as libc::c_int as usize] as libc::c_int %
           8 as libc::c_int == 0 as libc::c_int {
        let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
        let mut velocity: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
        let mut accel: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
        pos.x = (*this).actor.world.pos.x + Rand_CenteredFloat(20.0f32);
        pos.y =
            (*this).actor.world.pos.y + Rand_CenteredFloat(40.0f32) +
                20 as libc::c_int as libc::c_float;
        pos.z = (*this).actor.world.pos.z + Rand_CenteredFloat(20.0f32);
        velocity.x = 0.0f32;
        velocity.y = 0.0f32;
        velocity.z = 0.0f32;
        accel.x = 0.0f32;
        accel.y = 0.1f32;
        accel.z = 0.0f32;
        BossTw_AddDmgCloud(globalCtx, cloudType, &mut pos, &mut velocity,
                           &mut accel, Rand_ZeroFloat(5.0f32) + 10.0f32,
                           0 as libc::c_int as s16, 0 as libc::c_int as s16,
                           150 as libc::c_int as s16);
    }
    SkelAnime_Update(&mut (*this).skelAnime);
    (*this).work[UNK_S8 as libc::c_int as usize] =
        ((*this).work[UNK_S8 as libc::c_int as usize] as libc::c_int +
             20 as libc::c_int) as s16;
    if (*this).work[UNK_S8 as libc::c_int as usize] as libc::c_int >
           255 as libc::c_int {
        (*this).work[UNK_S8 as libc::c_int as usize] =
            255 as libc::c_int as s16
    }
    Math_ApproachF(&mut *(*this).workf.as_mut_ptr().offset(UNK_F12 as
                                                               libc::c_int as
                                                               isize), 0.0f32,
                   1.0f32, 0.05f32);
    (*this).actor.world.pos.y += (*this).actor.velocity.y;
    Math_ApproachF(&mut (*this).actor.velocity.y, -5.0f32, 1.0f32, 0.5f32);
    (*this).actor.world.pos.y -= 30.0f32;
    Actor_UpdateBgCheckInfo(globalCtx, &mut (*this).actor, 50.0f32, 50.0f32,
                            100.0f32, 4 as libc::c_int);
    (*this).actor.world.pos.y += 30.0f32;
    if (*this).csState1 as libc::c_int == 0 as libc::c_int {
        if (*this).timers[1 as libc::c_int as usize] as libc::c_int ==
               0 as libc::c_int {
            (*this).csState1 = 1 as libc::c_int as s16;
            (*this).workf[ANIM_SW_TGT as libc::c_int as usize] =
                Animation_GetLastFrame(&mut object_tw_Anim_0343B4 as
                                           *mut AnimationHeader as
                                           *mut libc::c_void) as f32_0;
            Animation_Change(&mut (*this).skelAnime,
                             &mut object_tw_Anim_0343B4, 1.0f32, 0.0f32,
                             (*this).workf[ANIM_SW_TGT as libc::c_int as
                                               usize],
                             3 as libc::c_int as u8_0, 0.0f32);
        }
    } else if Animation_OnFrame(&mut (*this).skelAnime,
                                (*this).workf[ANIM_SW_TGT as libc::c_int as
                                                  usize]) != 0 {
        (*this).workf[ANIM_SW_TGT as libc::c_int as usize] = 1000.0f32;
        Animation_MorphToLoop(&mut (*this).skelAnime,
                              &mut object_tw_Anim_035030, 0.0f32);
    }
    if (*this).actor.bgCheckFlags as libc::c_int & 1 as libc::c_int != 0 {
        (*this).actor.velocity.y = 0.0f32
    }
    if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
           0 as libc::c_int {
        BossTw_TwinrovaSetupGetUp(this, globalCtx);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_TwinrovaSetupGetUp(mut this: *mut BossTw,
                                                   mut globalCtx:
                                                       *mut GlobalContext) {
    Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                              &mut object_tw_Anim_035988, 0.0f32);
    (*this).workf[ANIM_SW_TGT as libc::c_int as usize] =
        Animation_GetLastFrame(&mut object_tw_Anim_035988 as
                                   *mut AnimationHeader as *mut libc::c_void)
            as f32_0;
    (*this).actionFunc =
        Some(BossTw_TwinrovaGetUp as
                 unsafe extern "C" fn(_: *mut BossTw, _: *mut GlobalContext)
                     -> ());
    (*this).timers[0 as libc::c_int as usize] = 50 as libc::c_int as s16;
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_TwinrovaGetUp(mut this: *mut BossTw,
                                              mut globalCtx:
                                                  *mut GlobalContext) {
    SkelAnime_Update(&mut (*this).skelAnime);
    Math_ApproachF(&mut (*this).actor.world.pos.y, (*this).targetPos.y,
                   0.05f32, 5.0f32);
    if Animation_OnFrame(&mut (*this).skelAnime,
                         (*this).workf[ANIM_SW_TGT as libc::c_int as usize])
           != 0 {
        (*this).workf[ANIM_SW_TGT as libc::c_int as usize] = 1000.0f32;
        Animation_MorphToLoop(&mut (*this).skelAnime,
                              &mut object_tw_Anim_032BF8, 0.0f32);
    }
    if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
           0 as libc::c_int {
        BossTw_TwinrovaSetupFly(this, globalCtx);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_TwinrovaSetupFly(mut this: *mut BossTw,
                                                 mut globalCtx:
                                                     *mut GlobalContext) {
    let mut xDiff: f32_0 = 0.;
    let mut zDiff: f32_0 = 0.;
    let mut yDiff: f32_0 = 0.;
    let mut xzDist: f32_0 = 0.;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    loop  {
        (*this).work[TW_PLLR_IDX as libc::c_int as usize] =
            ((*this).work[TW_PLLR_IDX as libc::c_int as usize] as libc::c_int
                 +
                 (Rand_ZeroFloat(2.99f32) as s16 as libc::c_int +
                      1 as libc::c_int) as s16 as libc::c_int) as s16;
        (*this).work[TW_PLLR_IDX as libc::c_int as usize] =
            ((*this).work[TW_PLLR_IDX as libc::c_int as usize] as libc::c_int
                 % 4 as libc::c_int) as s16;
        (*this).targetPos =
            sTwinrovaPillarPos[(*this).work[TW_PLLR_IDX as libc::c_int as
                                                usize] as usize];
        xDiff = (*this).targetPos.x - (*player).actor.world.pos.x;
        zDiff = (*this).targetPos.z - (*player).actor.world.pos.z;
        xzDist = xDiff * xDiff + zDiff * zDiff;
        if xzDist > 300.0f32 * 300.0f32 { break ; }
    }
    (*this).targetPos.y = 480.0f32;
    xDiff = (*this).targetPos.x - (*this).actor.world.pos.x;
    yDiff = (*this).targetPos.y - (*this).actor.world.pos.y;
    zDiff = (*this).targetPos.z - (*this).actor.world.pos.z;
    (*this).actionFunc =
        Some(BossTw_TwinrovaFly as
                 unsafe extern "C" fn(_: *mut BossTw, _: *mut GlobalContext)
                     -> ());
    (*this).rotateSpeed = 0.0f32;
    (*this).actor.speedXZ = 0.0f32;
    (*this).actor.world.rot.y =
        (Math_FAtan2F(xDiff, zDiff) *
             (32768 as libc::c_int as libc::c_float /
                  3.14159265358979323846f32)) as s16;
    xzDist = sqrtf(xDiff * xDiff + zDiff * zDiff);
    (*this).actor.world.rot.x =
        (Math_FAtan2F(yDiff, xzDist) *
             (32768 as libc::c_int as libc::c_float /
                  3.14159265358979323846f32)) as s16;
    Animation_MorphToLoop(&mut (*this).skelAnime, &mut object_tw_Anim_032BF8,
                          -10.0f32);
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_TwinrovaFly(mut this: *mut BossTw,
                                            mut globalCtx:
                                                *mut GlobalContext) {
    let mut xDiff: f32_0 = 0.;
    let mut yDiff: f32_0 = 0.;
    let mut zDiff: f32_0 = 0.;
    let mut pad: s32 = 0;
    let mut yaw: f32_0 = 0.;
    let mut xzDist: f32_0 = 0.;
    Audio_PlayActorSound2(&mut (*this).actor,
                          (0x391f as libc::c_int - 0x800 as libc::c_int) as
                              u16_0);
    SkelAnime_Update(&mut (*this).skelAnime);
    xDiff = (*this).targetPos.x - (*this).actor.world.pos.x;
    yDiff = (*this).targetPos.y - (*this).actor.world.pos.y;
    zDiff = (*this).targetPos.z - (*this).actor.world.pos.z;
    // Convert from radians to degrees, then degrees to binary angle
    yaw =
        (Math_FAtan2F(xDiff, zDiff) *
             (180.0f32 / 3.14159265358979323846f32 * (65536.0f32 / 360.0f32)))
            as s16 as f32_0;
    xzDist = sqrtf(xDiff * xDiff + zDiff * zDiff);
    Math_ApproachS(&mut (*this).actor.world.rot.x,
                   (Math_FAtan2F(yDiff, xzDist) *
                        (180.0f32 / 3.14159265358979323846f32 *
                             (65536.0f32 / 360.0f32))) as s16 as f32_0 as s16,
                   0xa as libc::c_int as s16, (*this).rotateSpeed as s16);
    Math_ApproachS(&mut (*this).actor.world.rot.y, yaw as s16,
                   0xa as libc::c_int as s16, (*this).rotateSpeed as s16);
    Math_ApproachS(&mut (*this).actor.shape.rot.y, yaw as s16,
                   0xa as libc::c_int as s16, (*this).rotateSpeed as s16);
    Math_ApproachF(&mut (*this).rotateSpeed, 2000.0f32, 1.0f32, 100.0f32);
    Math_ApproachF(&mut (*this).actor.speedXZ, 30.0f32, 1.0f32, 2.0f32);
    func_8002D908(&mut (*this).actor);
    Math_ApproachF(&mut (*this).actor.world.pos.x, (*this).targetPos.x,
                   0.1f32, fabsf((*this).actor.velocity.x) * 1.5f32);
    Math_ApproachF(&mut (*this).actor.world.pos.y, (*this).targetPos.y,
                   0.1f32, fabsf((*this).actor.velocity.y) * 1.5f32);
    Math_ApproachF(&mut (*this).targetPos.y, 380.0f32, 1.0f32, 2.0f32);
    Math_ApproachF(&mut (*this).actor.world.pos.z, (*this).targetPos.z,
                   0.1f32, fabsf((*this).actor.velocity.z) * 1.5f32);
    if xzDist < 200.0f32 {
        BossTw_TwinrovaSetupArriveAtTarget(this, globalCtx);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_TwinrovaSetupSpin(mut this: *mut BossTw,
                                                  mut globalCtx:
                                                      *mut GlobalContext) {
    (*this).actionFunc =
        Some(BossTw_TwinrovaSpin as
                 unsafe extern "C" fn(_: *mut BossTw, _: *mut GlobalContext)
                     -> ());
    Animation_MorphToLoop(&mut (*this).skelAnime, &mut object_tw_Anim_032BF8,
                          0.0f32);
    (*this).timers[0 as libc::c_int as usize] = 20 as libc::c_int as s16;
    (*this).actor.speedXZ = 0.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_TwinrovaSpin(mut this: *mut BossTw,
                                             mut globalCtx:
                                                 *mut GlobalContext) {
    SkelAnime_Update(&mut (*this).skelAnime);
    if (*this).timers[0 as libc::c_int as usize] as libc::c_int !=
           0 as libc::c_int {
        (*this).collider.base.colType = COLTYPE_METAL as libc::c_int as u8_0;
        (*this).actor.shape.rot.y =
            ((*this).actor.shape.rot.y as libc::c_int - 0x3000 as libc::c_int)
                as s16;
        if (*this).timers[0 as libc::c_int as usize] as libc::c_int %
               4 as libc::c_int == 0 as libc::c_int {
            Audio_PlayActorSound2(&mut (*this).actor,
                                  0x3921 as libc::c_int as u16_0);
        }
    } else { BossTw_TwinrovaSetupFly(this, globalCtx); };
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_TwinrovaSetupLaugh(mut this: *mut BossTw,
                                                   mut globalCtx:
                                                       *mut GlobalContext) {
    (*this).actionFunc =
        Some(BossTw_TwinrovaLaugh as
                 unsafe extern "C" fn(_: *mut BossTw, _: *mut GlobalContext)
                     -> ());
    Animation_MorphToPlayOnce(&mut (*this).skelAnime,
                              &mut object_tw_Anim_03A2D0, 0.0f32);
    (*this).workf[ANIM_SW_TGT as libc::c_int as usize] =
        Animation_GetLastFrame(&mut object_tw_Anim_03A2D0 as
                                   *mut AnimationHeader as *mut libc::c_void)
            as f32_0;
    (*this).actor.speedXZ = 0.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn BossTw_TwinrovaLaugh(mut this: *mut BossTw,
                                              mut globalCtx:
                                                  *mut GlobalContext) {
    SkelAnime_Update(&mut (*this).skelAnime);
    if Animation_OnFrame(&mut (*this).skelAnime, 10.0f32) != 0 {
        Audio_PlayActorSound2(&mut (*this).actor,
                              0x39b9 as libc::c_int as u16_0);
    }
    if Animation_OnFrame(&mut (*this).skelAnime,
                         (*this).workf[ANIM_SW_TGT as libc::c_int as usize])
           != 0 {
        BossTw_TwinrovaSetupFly(this, globalCtx);
    };
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
