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
    fn Actor_ProcessTalkRequest(actor: *mut Actor,
                                globalCtx: *mut GlobalContext) -> u32_0;
    #[no_mangle]
    fn func_8002F2CC(actor: *mut Actor, globalCtx: *mut GlobalContext,
                     arg2: f32_0) -> s32;
    #[no_mangle]
    fn func_8002F2F4(actor: *mut Actor, globalCtx: *mut GlobalContext) -> s32;
    #[no_mangle]
    fn Actor_TextboxIsClosing(actor: *mut Actor,
                              globalCtx: *mut GlobalContext) -> u32_0;
    #[no_mangle]
    fn Actor_HasParent(actor: *mut Actor, globalCtx: *mut GlobalContext)
     -> u32_0;
    #[no_mangle]
    fn func_8002F434(actor: *mut Actor, globalCtx: *mut GlobalContext,
                     getItemId: s32, xzRange: f32_0, yRange: f32_0) -> s32;
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
    fn Actor_ChangeCategory(globalCtx: *mut GlobalContext,
                            actorCtx: *mut ActorContext, actor: *mut Actor,
                            actorCategory: u8_0);
    #[no_mangle]
    fn Rand_ZeroFloat(f: f32_0) -> f32_0;
    #[no_mangle]
    fn Rand_CenteredFloat(f: f32_0) -> f32_0;
    #[no_mangle]
    fn Camera_GetInputDirYaw(camera: *mut Camera) -> s16;
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
    fn CollisionCheck_SetOC(globalCtx: *mut GlobalContext,
                            colChkCtx: *mut CollisionCheckContext,
                            collider: *mut Collider) -> s32;
    #[no_mangle]
    fn func_80064520(globalCtx: *mut GlobalContext,
                     csCtx: *mut CutsceneContext);
    #[no_mangle]
    fn func_80064534(globalCtx: *mut GlobalContext,
                     csCtx: *mut CutsceneContext);
    #[no_mangle]
    fn Environment_EnableUnderwaterLights(globalCtx: *mut GlobalContext,
                                          waterLightsIndex: s32);
    #[no_mangle]
    fn Math_CosS(angle: s16) -> f32_0;
    #[no_mangle]
    fn Math_SinS(angle: s16) -> f32_0;
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
    fn Math_ApproachZeroF(pValue: *mut f32_0, fraction: f32_0, step: f32_0);
    #[no_mangle]
    fn Math_ApproachS(pValue: *mut s16, target: s16, scale: s16, step: s16);
    #[no_mangle]
    fn func_80078884(sfxId: u16_0);
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
    fn Interface_ChangeAlpha(alphaType: u16_0);
    #[no_mangle]
    fn Rupees_ChangeBy(rupeeChange: s16);
    #[no_mangle]
    fn Gfx_CallSetupDL(gfx: *mut Gfx, i: u32_0) -> *mut Gfx;
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
    fn SkelAnime_InitFlex(globalCtx: *mut GlobalContext,
                          skelAnime: *mut SkelAnime,
                          skeletonHeaderSeg: *mut FlexSkeletonHeader,
                          animation: *mut AnimationHeader,
                          jointTable: *mut Vec3s, morphTable: *mut Vec3s,
                          limbCount: s32) -> s32;
    #[no_mangle]
    fn SkelAnime_Update(skelAnime: *mut SkelAnime) -> s32;
    #[no_mangle]
    fn Animation_MorphToLoop(skelAnime: *mut SkelAnime,
                             animation: *mut AnimationHeader,
                             morphFrames: f32_0);
    #[no_mangle]
    fn SkelAnime_Free(skelAnime: *mut SkelAnime,
                      globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn SkinMatrix_Vec3fMtxFMultXYZW(mf: *mut MtxF, src: *mut Vec3f,
                                    xyzDest: *mut Vec3f, wDest: *mut f32_0);
    #[no_mangle]
    fn func_800A9F6C(_: f32_0, _: u8_0, _: u8_0, _: u8_0);
    #[no_mangle]
    fn func_800AA148() -> u32_0;
    #[no_mangle]
    fn ShrinkWindow_SetVal(value: s32);
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
    fn Math_Atan2S(x: f32_0, y: f32_0) -> s16;
    #[no_mangle]
    fn Math_Atan2F(x: f32_0, y: f32_0) -> f32_0;
    #[no_mangle]
    fn Matrix_Push();
    #[no_mangle]
    fn Matrix_Pop();
    #[no_mangle]
    fn Matrix_Get(dest: *mut MtxF);
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
    fn func_800F436C(pos: *mut Vec3f, sfxId: u16_0, arg2: f32_0);
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
    fn Message_ShouldAdvance(globalCtx: *mut GlobalContext) -> u8_0;
    #[no_mangle]
    fn Message_CloseTextbox(_: *mut GlobalContext);
    #[no_mangle]
    fn Message_StartTextbox(globalCtx: *mut GlobalContext, textId: u16_0,
                            actor: *mut Actor);
    #[no_mangle]
    fn Message_ContinueTextbox(globalCtx: *mut GlobalContext, textId: u16_0);
    #[no_mangle]
    fn Message_GetState(msgCtx: *mut MessageContext) -> u8_0;
    #[no_mangle]
    static mut gSegments: [u32_0; 16];
    #[no_mangle]
    static mut gGameInfo: *mut GameInfo;
    #[no_mangle]
    static mut gSaveContext: SaveContext;
    #[no_mangle]
    static mut gTimeIncrement: u16_0;
    #[no_mangle]
    static mut gFishingFishAnim: AnimationHeader;
    #[no_mangle]
    static mut gFishingFishSkel: FlexSkeletonHeader;
    #[no_mangle]
    static mut gFishingStreamSplashDL: [Gfx; 0];
    #[no_mangle]
    static mut gFishingBubbleMaterialDL: [Gfx; 0];
    #[no_mangle]
    static mut gFishingBubbleModelDL: [Gfx; 0];
    #[no_mangle]
    static mut gFishingDustSplashMaterialDL: [Gfx; 0];
    #[no_mangle]
    static mut gFishingDustSplashModelDL: [Gfx; 0];
    #[no_mangle]
    static mut gFishingLineModelDL: [Gfx; 0];
    #[no_mangle]
    static mut gFishingRainDropModelDL: [Gfx; 0];
    #[no_mangle]
    static mut gFishingRainSplashMaterialDL: [Gfx; 0];
    #[no_mangle]
    static mut gFishingRainSplashModelDL: [Gfx; 0];
    #[no_mangle]
    static mut gFishingOwnerAnim: AnimationHeader;
    #[no_mangle]
    static mut gFishingOwnerHairDL: [Gfx; 0];
    #[no_mangle]
    static mut gFishingOwnerHatDL: [Gfx; 0];
    #[no_mangle]
    static mut gFishingOwnerSkel: FlexSkeletonHeader;
    #[no_mangle]
    static mut gFishingRippleMaterialDL: [Gfx; 0];
    #[no_mangle]
    static mut gFishingRippleModelDL: [Gfx; 0];
    #[no_mangle]
    static mut gFishingWaterDustMaterialDL: [Gfx; 0];
    #[no_mangle]
    static mut gFishingWaterDustModelDL: [Gfx; 0];
    #[no_mangle]
    static mut gFishingOwnerEyeOpenTex: [u64_0; 0];
    #[no_mangle]
    static mut gFishingOwnerEyeHalfTex: [u64_0; 0];
    #[no_mangle]
    static mut gFishingOwnerEyeClosedTex: [u64_0; 0];
    #[no_mangle]
    static mut gFishingSinkingLureSegmentMaterialDL: [Gfx; 0];
    #[no_mangle]
    static mut gFishingSinkingLureSegmentModelDL: [Gfx; 0];
    #[no_mangle]
    static mut gFishingGroupFishMaterialDL: [Gfx; 0];
    #[no_mangle]
    static mut gFishingGroupFishModelDL: [Gfx; 0];
    #[no_mangle]
    static mut gFishingLoachAnim: AnimationHeader;
    #[no_mangle]
    static mut gFishingLoachSkel: FlexSkeletonHeader;
    #[no_mangle]
    static mut gFishingRodSegmentStripTex: [u64_0; 0];
    #[no_mangle]
    static mut gFishingRodSegmentBlackTex: [u64_0; 0];
    #[no_mangle]
    static mut gFishingRodSegmentWhiteTex: [u64_0; 0];
    #[no_mangle]
    static mut gFishingRodMaterialDL: [Gfx; 0];
    #[no_mangle]
    static mut gFishingRodSegmentDL: [Gfx; 0];
    #[no_mangle]
    static mut gFishingLureHookDL: [Gfx; 0];
    #[no_mangle]
    static mut gFishingLureFloatDL: [Gfx; 0];
    #[no_mangle]
    static mut gFishingLilyPadMaterialDL: [Gfx; 0];
    #[no_mangle]
    static mut gFishingLilyPadModelDL: [Gfx; 0];
    #[no_mangle]
    static mut gFishingRockMaterialDL: [Gfx; 0];
    #[no_mangle]
    static mut gFishingRockModelDL: [Gfx; 0];
    #[no_mangle]
    static mut gFishingWoodPostMaterialDL: [Gfx; 0];
    #[no_mangle]
    static mut gFishingWoodPostModelDL: [Gfx; 0];
    #[no_mangle]
    static mut gFishingReedMaterialDL: [Gfx; 0];
    #[no_mangle]
    static mut gFishingReedModelDL: [Gfx; 0];
    #[no_mangle]
    static mut gFishingAquariumBottomDL: [Gfx; 0];
    #[no_mangle]
    static mut gFishingAquariumContainerDL: [Gfx; 0];
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
pub type C2RustUnnamed_2 = libc::c_uint;
pub const HS_DAMPE_RACE: C2RustUnnamed_2 = 6;
pub const HS_UNK_05: C2RustUnnamed_2 = 5;
pub const HS_MARATHON: C2RustUnnamed_2 = 4;
pub const HS_HORSE_RACE: C2RustUnnamed_2 = 3;
pub const HS_FISHING: C2RustUnnamed_2 = 2;
pub const HS_POE_POINTS: C2RustUnnamed_2 = 1;
pub const HS_HBA: C2RustUnnamed_2 = 0;
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
    pub c2rust_unnamed: C2RustUnnamed_3,
    pub rgba: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
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
    pub c2rust_unnamed: C2RustUnnamed_4,
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
pub union C2RustUnnamed_4 {
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
    pub c2rust_unnamed: C2RustUnnamed_5,
    pub normal: Vec3s,
    pub dist: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub vtxData: [u16_0; 3],
    pub c2rust_unnamed: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
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
    pub sides: [C2RustUnnamed_7; 2],
    pub id: s16,
    pub pos: Vec3s,
    pub rotY: s16,
    pub params: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
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
    pub c2rust_unnamed: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub single: C2RustUnnamed_10,
    pub multi: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
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
pub struct C2RustUnnamed_10 {
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
    pub restrictions: C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
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
    pub c2rust_unnamed: C2RustUnnamed_12,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
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
    pub c2rust_unnamed: C2RustUnnamed_13,
    pub startPos: Vec3i,
    pub endPos: Vec3i,
    pub normal: Vec3i,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_13 {
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
    pub flags: C2RustUnnamed_14,
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
pub struct C2RustUnnamed_14 {
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
pub const GI_MAX: C2RustUnnamed_21 = 126;
pub const GI_TEXT_0: C2RustUnnamed_21 = 125;
pub const GI_ICE_TRAP: C2RustUnnamed_21 = 124;
pub const GI_BULLET_BAG_50: C2RustUnnamed_21 = 123;
pub const GI_NUT_UPGRADE_40: C2RustUnnamed_21 = 122;
pub const GI_NUT_UPGRADE_30: C2RustUnnamed_21 = 121;
pub const GI_STICK_UPGRADE_30: C2RustUnnamed_21 = 120;
pub const GI_STICK_UPGRADE_20: C2RustUnnamed_21 = 119;
pub const GI_HEART_PIECE_WIN: C2RustUnnamed_21 = 118;
pub const GI_RUPEE_PURPLE_LOSE: C2RustUnnamed_21 = 117;
pub const GI_RUPEE_RED_LOSE: C2RustUnnamed_21 = 116;
pub const GI_RUPEE_BLUE_LOSE: C2RustUnnamed_21 = 115;
pub const GI_RUPEE_GREEN_LOSE: C2RustUnnamed_21 = 114;
pub const GI_DOOR_KEY: C2RustUnnamed_21 = 113;
pub const GI_BIG_POE: C2RustUnnamed_21 = 112;
pub const GI_POE: C2RustUnnamed_21 = 111;
pub const GI_BLUE_FIRE: C2RustUnnamed_21 = 110;
pub const GI_BUGS: C2RustUnnamed_21 = 109;
pub const GI_FISH: C2RustUnnamed_21 = 108;
pub const GI_BOMBCHUS_20: C2RustUnnamed_21 = 107;
pub const GI_BOMBCHUS_5: C2RustUnnamed_21 = 106;
pub const GI_SEEDS_30: C2RustUnnamed_21 = 105;
pub const GI_BOMBS_30: C2RustUnnamed_21 = 104;
pub const GI_BOMBS_20: C2RustUnnamed_21 = 103;
pub const GI_BOMBS_10: C2RustUnnamed_21 = 102;
pub const GI_BOMBS_1: C2RustUnnamed_21 = 101;
pub const GI_NUTS_10: C2RustUnnamed_21 = 100;
pub const GI_NUTS_5_2: C2RustUnnamed_21 = 99;
pub const GI_STICKS_10: C2RustUnnamed_21 = 98;
pub const GI_STICKS_5: C2RustUnnamed_21 = 97;
pub const GI_BULLET_BAG_40: C2RustUnnamed_21 = 96;
pub const GI_BULLET_BAG_30: C2RustUnnamed_21 = 95;
pub const GI_NAYRUS_LOVE: C2RustUnnamed_21 = 94;
pub const GI_FARORES_WIND: C2RustUnnamed_21 = 93;
pub const GI_DINS_FIRE: C2RustUnnamed_21 = 92;
pub const GI_SKULL_TOKEN: C2RustUnnamed_21 = 91;
pub const GI_ARROW_LIGHT: C2RustUnnamed_21 = 90;
pub const GI_ARROW_ICE: C2RustUnnamed_21 = 89;
pub const GI_ARROW_FIRE: C2RustUnnamed_21 = 88;
pub const GI_SWORD_BGS: C2RustUnnamed_21 = 87;
pub const GI_RUPEE_GOLD: C2RustUnnamed_21 = 86;
pub const GI_RUPEE_PURPLE: C2RustUnnamed_21 = 85;
pub const GI_BRACELET: C2RustUnnamed_21 = 84;
pub const GI_MASK_GERUDO: C2RustUnnamed_21 = 83;
pub const GI_MASK_ZORA: C2RustUnnamed_21 = 82;
pub const GI_MASK_GORON: C2RustUnnamed_21 = 81;
pub const GI_MILK: C2RustUnnamed_21 = 80;
pub const GI_HEART_CONTAINER_2: C2RustUnnamed_21 = 79;
pub const GI_RUPEE_RED: C2RustUnnamed_21 = 78;
pub const GI_RUPEE_BLUE: C2RustUnnamed_21 = 77;
pub const GI_RUPEE_GREEN: C2RustUnnamed_21 = 76;
pub const GI_ARROWS_LARGE: C2RustUnnamed_21 = 75;
pub const GI_ARROWS_MEDIUM: C2RustUnnamed_21 = 74;
pub const GI_ARROWS_SMALL: C2RustUnnamed_21 = 73;
pub const GI_HEART: C2RustUnnamed_21 = 72;
pub const GI_WEIRD_EGG: C2RustUnnamed_21 = 71;
pub const GI_WALLET_GIANT: C2RustUnnamed_21 = 70;
pub const GI_WALLET_ADULT: C2RustUnnamed_21 = 69;
pub const GI_MAGIC_LARGE: C2RustUnnamed_21 = 68;
pub const GI_MAGIC_SMALL: C2RustUnnamed_21 = 67;
pub const GI_KEY_SMALL: C2RustUnnamed_21 = 66;
pub const GI_MAP: C2RustUnnamed_21 = 65;
pub const GI_COMPASS: C2RustUnnamed_21 = 64;
pub const GI_KEY_BOSS: C2RustUnnamed_21 = 63;
pub const GI_HEART_PIECE: C2RustUnnamed_21 = 62;
pub const GI_HEART_CONTAINER: C2RustUnnamed_21 = 61;
pub const GI_SEEDS_5: C2RustUnnamed_21 = 60;
pub const GI_OCARINA_FAIRY: C2RustUnnamed_21 = 59;
pub const GI_GERUDO_CARD: C2RustUnnamed_21 = 58;
pub const GI_STONE_OF_AGONY: C2RustUnnamed_21 = 57;
pub const GI_SCALE_GOLD: C2RustUnnamed_21 = 56;
pub const GI_SCALE_SILVER: C2RustUnnamed_21 = 55;
pub const GI_GAUNTLETS_GOLD: C2RustUnnamed_21 = 54;
pub const GI_GAUNTLETS_SILVER: C2RustUnnamed_21 = 53;
pub const GI_BOMB_BAG_40: C2RustUnnamed_21 = 52;
pub const GI_BOMB_BAG_30: C2RustUnnamed_21 = 51;
pub const GI_BOMB_BAG_20: C2RustUnnamed_21 = 50;
pub const GI_QUIVER_50: C2RustUnnamed_21 = 49;
pub const GI_QUIVER_40: C2RustUnnamed_21 = 48;
pub const GI_BOOTS_HOVER: C2RustUnnamed_21 = 47;
pub const GI_BOOTS_IRON: C2RustUnnamed_21 = 46;
pub const GI_TUNIC_ZORA: C2RustUnnamed_21 = 45;
pub const GI_TUNIC_GORON: C2RustUnnamed_21 = 44;
pub const GI_SHIELD_MIRROR: C2RustUnnamed_21 = 43;
pub const GI_SHIELD_HYLIAN: C2RustUnnamed_21 = 42;
pub const GI_SHIELD_DEKU: C2RustUnnamed_21 = 41;
pub const GI_SWORD_KNIFE: C2RustUnnamed_21 = 40;
pub const GI_SWORD_KOKIRI: C2RustUnnamed_21 = 39;
pub const GI_CLAIM_CHECK: C2RustUnnamed_21 = 38;
pub const GI_EYEDROPS: C2RustUnnamed_21 = 37;
pub const GI_FROG: C2RustUnnamed_21 = 36;
pub const GI_PRESCRIPTION: C2RustUnnamed_21 = 35;
pub const GI_SWORD_BROKEN: C2RustUnnamed_21 = 34;
pub const GI_SAW: C2RustUnnamed_21 = 33;
pub const GI_ODD_POTION: C2RustUnnamed_21 = 32;
pub const GI_ODD_MUSHROOM: C2RustUnnamed_21 = 31;
pub const GI_POCKET_CUCCO: C2RustUnnamed_21 = 30;
pub const GI_POCKET_EGG: C2RustUnnamed_21 = 29;
pub const GI_MASK_TRUTH: C2RustUnnamed_21 = 28;
pub const GI_MASK_BUNNY: C2RustUnnamed_21 = 27;
pub const GI_MASK_KEATON: C2RustUnnamed_21 = 26;
pub const GI_CHICKEN: C2RustUnnamed_21 = 25;
pub const GI_MASK_SPOOKY: C2RustUnnamed_21 = 24;
pub const GI_MASK_SKULL: C2RustUnnamed_21 = 23;
pub const GI_BEAN: C2RustUnnamed_21 = 22;
pub const GI_LETTER_RUTO: C2RustUnnamed_21 = 21;
pub const GI_MILK_BOTTLE: C2RustUnnamed_21 = 20;
pub const GI_FAIRY: C2RustUnnamed_21 = 19;
pub const GI_POTION_BLUE: C2RustUnnamed_21 = 18;
pub const GI_POTION_GREEN: C2RustUnnamed_21 = 17;
pub const GI_POTION_RED: C2RustUnnamed_21 = 16;
pub const GI_BOTTLE: C2RustUnnamed_21 = 15;
pub const GI_COJIRO: C2RustUnnamed_21 = 14;
pub const GI_HAMMER: C2RustUnnamed_21 = 13;
pub const GI_OCARINA_OOT: C2RustUnnamed_21 = 12;
pub const GI_LETTER_ZELDA: C2RustUnnamed_21 = 11;
pub const GI_LENS: C2RustUnnamed_21 = 10;
pub const GI_LONGSHOT: C2RustUnnamed_21 = 9;
pub const GI_HOOKSHOT: C2RustUnnamed_21 = 8;
pub const GI_STICKS_1: C2RustUnnamed_21 = 7;
pub const GI_BOOMERANG: C2RustUnnamed_21 = 6;
pub const GI_SLINGSHOT: C2RustUnnamed_21 = 5;
pub const GI_BOW: C2RustUnnamed_21 = 4;
pub const GI_BOMBCHUS_10: C2RustUnnamed_21 = 3;
pub const GI_NUTS_5: C2RustUnnamed_21 = 2;
pub const GI_BOMBS_5: C2RustUnnamed_21 = 1;
pub const GI_NONE: C2RustUnnamed_21 = 0;
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
pub type C2RustUnnamed_24 = libc::c_uint;
pub const TEXT_STATE_AWAITING_NEXT: C2RustUnnamed_24 = 10;
pub const TEXT_STATE_9: C2RustUnnamed_24 = 9;
pub const TEXT_STATE_8: C2RustUnnamed_24 = 8;
pub const TEXT_STATE_SONG_DEMO_DONE: C2RustUnnamed_24 = 7;
pub const TEXT_STATE_DONE: C2RustUnnamed_24 = 6;
pub const TEXT_STATE_EVENT: C2RustUnnamed_24 = 5;
pub const TEXT_STATE_CHOICE: C2RustUnnamed_24 = 4;
pub const TEXT_STATE_DONE_FADING: C2RustUnnamed_24 = 3;
pub const TEXT_STATE_CLOSING: C2RustUnnamed_24 = 2;
pub const TEXT_STATE_DONE_HAS_NEXT: C2RustUnnamed_24 = 1;
pub const TEXT_STATE_NONE: C2RustUnnamed_24 = 0;
pub type C2RustUnnamed_25 = libc::c_uint;
pub const MTXMODE_APPLY: C2RustUnnamed_25 = 1;
pub const MTXMODE_NEW: C2RustUnnamed_25 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fishing {
    pub actor: Actor,
    pub unk_14C: [libc::c_char; 4],
    pub unk_150: u8_0,
    pub unk_151: u8_0,
    pub unk_152: u8_0,
    pub unk_154: s16,
    pub unk_156: u8_0,
    pub unk_157: u8_0,
    pub unk_158: s16,
    pub unk_15A: s16,
    pub unk_15C: s16,
    pub unk_15E: s16,
    pub unk_160: s16,
    pub unk_162: s16,
    pub unk_164: s16,
    pub unk_166: s16,
    pub unk_168: s16,
    pub unk_16A: s16,
    pub unk_16C: s16,
    pub unk_16E: s16,
    pub unk_170: s16,
    pub unk_172: s16,
    pub unk_174: s16,
    pub unk_176: s16,
    pub unk_178: s16,
    pub unk_17A: [s16; 4],
    pub unk_184: f32_0,
    pub unk_188: f32_0,
    pub unk_18C: f32_0,
    pub unk_190: f32_0,
    pub unk_194: f32_0,
    pub unk_198: f32_0,
    pub unk_19C: f32_0,
    pub unk_1A0: s16,
    pub unk_1A2: s16,
    pub unk_1A4: s16,
    pub unk_1A8: f32_0,
    pub unk_1AC: f32_0,
    pub unk_1B0: f32_0,
    pub unk_1B4: Vec3f,
    pub fishMouthPos: Vec3f,
    pub unk_1CC: [s16; 3],
    pub unk_1D2: u8_0,
    pub unk_1D3: u8_0,
    pub unk_1D4: u8_0,
    pub unk_1D5: u8_0,
    pub skelAnime: SkelAnime,
    pub lightNode: *mut LightNode,
    pub lightInfo: LightInfo,
    pub collider: ColliderJntSph,
    pub colliderElements: [ColliderJntSphElement; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FishingFishInit {
    pub unk_00: u8_0,
    pub pos: Vec3s,
    pub unk_08: u8_0,
    pub unk_0C: f32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FishingProp {
    pub pos: Vec3f,
    pub rotX: f32_0,
    pub rotY: f32_0,
    pub reedAngle: f32_0,
    pub projectedPos: Vec3f,
    pub scale: f32_0,
    pub lilyPadAngle: s16,
    pub lilyPadOffset: f32_0,
    pub type_0: u8_0,
    pub timer: s16,
    pub shouldDraw: u8_0,
    pub drawDistance: f32_0,
}
pub const FS_PROP_NONE: C2RustUnnamed_27 = 0;
pub const FS_PROP_LILY_PAD: C2RustUnnamed_27 = 2;
pub const FS_PROP_WOOD_POST: C2RustUnnamed_27 = 4;
pub const FS_PROP_REED: C2RustUnnamed_27 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FishingPropInit {
    pub type_0: u8_0,
    pub pos: Vec3s,
}
pub const FS_PROP_INIT_STOP: C2RustUnnamed_27 = 35;
pub const FS_PROP_ROCK: C2RustUnnamed_27 = 3;
pub const FS_GROUP_FISH_NONE: C2RustUnnamed_28 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FishingGroupFish {
    pub type_0: u8_0,
    pub timer: s16,
    pub pos: Vec3f,
    pub unk_10: Vec3f,
    pub projectedPos: Vec3f,
    pub unk_28: f32_0,
    pub unk_2C: f32_0,
    pub unk_30: f32_0,
    pub unk_34: f32_0,
    pub unk_38: f32_0,
    pub unk_3C: s16,
    pub unk_3E: s16,
    pub unk_40: s16,
    pub unk_42: s16,
    pub shouldDraw: u8_0,
}
pub const FS_GROUP_FISH_NORMAL: C2RustUnnamed_28 = 1;
pub const FS_EFF_NONE: C2RustUnnamed_26 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FishingEffect {
    pub pos: Vec3f,
    pub vel: Vec3f,
    pub accel: Vec3f,
    pub type_0: u8_0,
    pub timer: u8_0,
    pub unk_26: [libc::c_char; 4],
    pub alpha: s16,
    pub unk_2C: s16,
    pub unk_2E: s16,
    pub unk_30: f32_0,
    pub unk_34: f32_0,
    pub unk_38: f32_0,
    pub unk_3C: f32_0,
}
pub const FS_EFF_OWNER_HAT: C2RustUnnamed_26 = 6;
pub const FS_EFF_RAIN_SPLASH: C2RustUnnamed_26 = 8;
pub const FS_EFF_RAIN_RIPPLE: C2RustUnnamed_26 = 7;
pub const FS_EFF_RAIN_DROP: C2RustUnnamed_26 = 5;
pub const FS_EFF_BUBBLE: C2RustUnnamed_26 = 4;
pub const FS_EFF_WATER_DUST: C2RustUnnamed_26 = 3;
pub const FS_EFF_DUST_SPLASH: C2RustUnnamed_26 = 2;
pub const FS_EFF_RIPPLE: C2RustUnnamed_26 = 1;
pub type C2RustUnnamed_26 = libc::c_uint;
pub type C2RustUnnamed_27 = libc::c_uint;
pub type C2RustUnnamed_28 = libc::c_uint;
#[no_mangle]
pub static mut Fishing_InitVars: ActorInit =
    unsafe {
        {
            let mut init =
                ActorInit{id: ACTOR_FISHING as libc::c_int as s16,
                          category: ACTORCAT_NPC as libc::c_int as u8_0,
                          flags:
                              ((1 as libc::c_int) << 4 as libc::c_int) as
                                  u32_0,
                          objectId: OBJECT_FISH as libc::c_int as s16,
                          instanceSize:
                              ::std::mem::size_of::<Fishing>() as
                                  libc::c_ulong,
                          init:
                              ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                      *mut Actor,
                                                                                  _:
                                                                                      *mut GlobalContext)
                                                                 -> ()>,
                                                      ActorFunc>(Some(Fishing_Init
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
                                                      ActorFunc>(Some(Fishing_Destroy
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
                                                      ActorFunc>(Some(Fishing_UpdateFish
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
                                                      ActorFunc>(Some(Fishing_DrawFish
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
static mut D_80B7A650: f32_0 = 0.0f32;
static mut D_80B7A654: u8_0 = 0 as libc::c_int as u8_0;
static mut D_80B7A658: f32_0 = 0.0f32;
static mut D_80B7A65C: Vec3f =
    { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
static mut D_80B7A668: f32_0 = 0.0f32;
static mut sSinkingLureLocation: u8_0 = 0 as libc::c_int as u8_0;
static mut D_80B7A670: f32_0 = 0.0f32;
static mut D_80B7A674: u8_0 = 1 as libc::c_int as u8_0;
static mut D_80B7A678: u16_0 = 0 as libc::c_int as u16_0;
static mut D_80B7A67C: u8_0 = 0 as libc::c_int as u8_0;
static mut D_80B7A680: s32 = 0 as libc::c_int;
static mut D_80B7A684: s16 = 0 as libc::c_int as s16;
static mut D_80B7A688: u8_0 = 0 as libc::c_int as u8_0;
static mut D_80B7A68C: u8_0 = 0 as libc::c_int as u8_0;
static mut D_80B7A690: u8_0 = 0 as libc::c_int as u8_0;
static mut D_80B7A694: s16 = 0 as libc::c_int as s16;
static mut sFishMouthOffset: Vec3f =
    { let mut init = Vec3f{x: 500.0f32, y: 500.0f32, z: 0.0f32,}; init };
static mut D_80B7A6A4: u8_0 = 0 as libc::c_int as u8_0;
static mut D_80B7A6A8: f32_0 = 0.0f32;
static mut D_80B7A6AC: f32_0 = 0.0f32;
static mut D_80B7A6B0: f32_0 = 0.0f32;
static mut D_80B7A6B4: f32_0 = 0.0f32;
static mut D_80B7A6B8: f32_0 = 0.0f32;
static mut D_80B7A6BC: f32_0 = 0.0f32;
static mut D_80B7A6C0: f32_0 = 0.0f32;
static mut D_80B7A6C4: s16 = 0 as libc::c_int as s16;
static mut D_80B7A6C8: s16 = 0 as libc::c_int as s16;
static mut D_80B7A6CC: u8_0 = 0 as libc::c_int as u8_0;
static mut D_80B7A6D0: u8_0 = 0 as libc::c_int as u8_0;
static mut D_80B7A6D4: u8_0 = 0 as libc::c_int as u8_0;
static mut sJntSphElementsInit: [ColliderJntSphElementInit; 12] =
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
     },
     {
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
     },
     {
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
     },
     {
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
     },
     {
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
     },
     {
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
     },
     {
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
     },
     {
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
     },
     {
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
     },
     {
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
     },
     {
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
     },
     {
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
static mut sJntSphInit: ColliderJntSphInit =
    unsafe {
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
                                                                     4 as
                                                                         libc::c_int)
                                                                    as u8_0,
                                                            acFlags:
                                                                ((1 as
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
                                                                COLSHAPE_JNTSPH
                                                                    as
                                                                    libc::c_int
                                                                    as u8_0,};
                                           init
                                       },
                                   count: 12 as libc::c_int,
                                   elements:
                                       sJntSphElementsInit.as_ptr() as
                                           *mut _,};
            init
        }
    };
static mut D_80B7A898: f32_0 = 0.0f32;
static mut sZeroVec: Vec3f =
    { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
static mut sFishingMain: *mut Fishing = 0 as *const Fishing as *mut Fishing;
static mut D_80B7E074: u8_0 = 0;
static mut sLinkAge: u8_0 = 0;
static mut D_80B7E076: u8_0 = 0;
static mut D_80B7E077: u8_0 = 0;
static mut D_80B7E078: f32_0 = 0.;
static mut D_80B7E07C: u8_0 = 0;
static mut D_80B7E07D: u8_0 = 0;
static mut D_80B7E07E: u8_0 = 0;
static mut D_80B7E080: s16 = 0;
static mut D_80B7E082: u8_0 = 0;
static mut D_80B7E084: u16_0 = 0;
static mut D_80B7E086: u16_0 = 0;
static mut D_80B7E088: s8 = 0;
static mut sOwnerHeadPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
static mut sEffOwnerHatRot: Vec3s = Vec3s{x: 0, y: 0, z: 0,};
static mut D_80B7E0A2: u8_0 = 0;
static mut D_80B7E0A4: s16 = 0;
static mut D_80B7E0A6: s16 = 0;
static mut sFishingHookedFish: *mut Fishing =
    0 as *const Fishing as *mut Fishing;
static mut D_80B7E0AC: s16 = 0;
static mut D_80B7E0AE: s16 = 0;
static mut D_80B7E0B0: s16 = 0;
static mut D_80B7E0B2: s16 = 0;
static mut D_80B7E0B4: s16 = 0;
static mut D_80B7E0B6: u8_0 = 0;
static mut sLurePos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
static mut D_80B7E0C8: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
static mut sLureRot: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
static mut D_80B7E0E8: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
static mut D_80B7E0F8: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
static mut D_80B7E104: f32_0 = 0.;
static mut D_80B7E108: f32_0 = 0.;
static mut D_80B7E10C: f32_0 = 0.;
static mut D_80B7E110: f32_0 = 0.;
static mut D_80B7E114: s8 = 0;
static mut D_80B7E116: s16 = 0;
static mut D_80B7E118: u8_0 = 0;
static mut D_80B7E11C: f32_0 = 0.;
static mut D_80B7E120: u8_0 = 0;
static mut D_80B7E122: s16 = 0;
static mut D_80B7E124: u8_0 = 0;
static mut D_80B7E128: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
static mut D_80B7E134: f32_0 = 0.;
static mut D_80B7E138: f32_0 = 0.;
static mut D_80B7E13C: s16 = 0;
static mut D_80B7E140: f32_0 = 0.;
static mut D_80B7E144: f32_0 = 0.;
static mut D_80B7E148: f32_0 = 0.;
static mut D_80B7E14C: f32_0 = 0.;
static mut D_80B7E150: s16 = 0;
static mut D_80B7E154: f32_0 = 0.;
static mut sRodTipPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
static mut sReelLinePos: [Vec3f; 200] = [Vec3f{x: 0., y: 0., z: 0.,}; 200];
static mut sReelLineRot: [Vec3f; 200] = [Vec3f{x: 0., y: 0., z: 0.,}; 200];
static mut sReelLineUnk: [Vec3f; 200] = [Vec3f{x: 0., y: 0., z: 0.,}; 200];
static mut sLureHookRefPos: [Vec3f; 2] = [Vec3f{x: 0., y: 0., z: 0.,}; 2];
static mut sLureHookRotY: [f32_0; 2] = [0.; 2];
static mut D_80B7FDA8: u8_0 = 0;
static mut sSinkingLurePos: [Vec3f; 20] = [Vec3f{x: 0., y: 0., z: 0.,}; 20];
static mut D_80B7FEA0: s16 = 0;
static mut sProjectedW: f32_0 = 0.;
static mut sCameraEye: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
static mut sCameraAt: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
static mut sCameraId: s16 = 0;
static mut D_80B7FEC8: f32_0 = 0.;
static mut D_80B7FECC: f32_0 = 0.;
static mut D_80B7FED0: f32_0 = 0.;
static mut sSinkingLureBasePos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
static mut D_80B7FEE4: f32_0 = 0.;
static mut sRandSeed0: s32 = 0;
static mut sRandSeed1: s32 = 0;
static mut sRandSeed2: s32 = 0;
static mut sPondProps: [FishingProp; 140] =
    [FishingProp{pos: Vec3f{x: 0., y: 0., z: 0.,},
                 rotX: 0.,
                 rotY: 0.,
                 reedAngle: 0.,
                 projectedPos: Vec3f{x: 0., y: 0., z: 0.,},
                 scale: 0.,
                 lilyPadAngle: 0,
                 lilyPadOffset: 0.,
                 type_0: 0,
                 timer: 0,
                 shouldDraw: 0,
                 drawDistance: 0.,}; 140];
static mut sGroupFishes: [FishingGroupFish; 60] =
    [FishingGroupFish{type_0: 0,
                      timer: 0,
                      pos: Vec3f{x: 0., y: 0., z: 0.,},
                      unk_10: Vec3f{x: 0., y: 0., z: 0.,},
                      projectedPos: Vec3f{x: 0., y: 0., z: 0.,},
                      unk_28: 0.,
                      unk_2C: 0.,
                      unk_30: 0.,
                      unk_34: 0.,
                      unk_38: 0.,
                      unk_3C: 0,
                      unk_3E: 0,
                      unk_40: 0,
                      unk_42: 0,
                      shouldDraw: 0,}; 60];
static mut sFishGroupAngle1: f32_0 = 0.;
static mut sFishGroupAngle2: f32_0 = 0.;
static mut sFishGroupAngle3: f32_0 = 0.;
static mut sFishingEffects: [FishingEffect; 130] =
    [FishingEffect{pos: Vec3f{x: 0., y: 0., z: 0.,},
                   vel: Vec3f{x: 0., y: 0., z: 0.,},
                   accel: Vec3f{x: 0., y: 0., z: 0.,},
                   type_0: 0,
                   timer: 0,
                   unk_26: [0; 4],
                   alpha: 0,
                   unk_2C: 0,
                   unk_2E: 0,
                   unk_30: 0.,
                   unk_34: 0.,
                   unk_38: 0.,
                   unk_3C: 0.,}; 130];
static mut sStreamSoundProjectedPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
#[no_mangle]
pub unsafe extern "C" fn Fishing_SetColliderElement(mut index: s32,
                                                    mut collider:
                                                        *mut ColliderJntSph,
                                                    mut pos: *mut Vec3f,
                                                    mut scale: f32_0) {
    (*(*collider).elements.offset(index as isize)).dim.worldSphere.center.x =
        (*pos).x as s16;
    (*(*collider).elements.offset(index as isize)).dim.worldSphere.center.y =
        (*pos).y as s16;
    (*(*collider).elements.offset(index as isize)).dim.worldSphere.center.z =
        (*pos).z as s16;
    (*(*collider).elements.offset(index as isize)).dim.worldSphere.radius =
        ((*(*collider).elements.offset(index as isize)).dim.modelSphere.radius
             as libc::c_int as libc::c_float *
             (*(*collider).elements.offset(index as isize)).dim.scale * scale
             * 1.6f32) as s16;
}
#[no_mangle]
pub unsafe extern "C" fn Fishing_SeedRand(mut seed0: s32, mut seed1: s32,
                                          mut seed2: s32) {
    sRandSeed0 = seed0;
    sRandSeed1 = seed1;
    sRandSeed2 = seed2;
}
#[no_mangle]
pub unsafe extern "C" fn Fishing_RandZeroOne() -> f32_0 {
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
pub unsafe extern "C" fn Fishing_SmoothStepToS(mut pValue: *mut s16,
                                               mut target: s16,
                                               mut scale: s16, mut step: s16)
 -> s16 {
    let mut stepSize: s16 = 0;
    let mut diff: s16 = 0;
    diff = (target as libc::c_int - *pValue as libc::c_int) as s16;
    stepSize = (diff as libc::c_int / scale as libc::c_int) as s16;
    if stepSize as libc::c_int > step as libc::c_int { stepSize = step }
    if (stepSize as libc::c_int) < -(step as libc::c_int) {
        stepSize = -(step as libc::c_int) as s16
    }
    *pValue = (*pValue as libc::c_int + stepSize as libc::c_int) as s16;
    return stepSize;
}
#[no_mangle]
pub unsafe extern "C" fn Fishing_SpawnRipple(mut projectedPos: *mut Vec3f,
                                             mut effect: *mut FishingEffect,
                                             mut pos: *mut Vec3f,
                                             mut arg3: f32_0, mut arg4: f32_0,
                                             mut arg5: s16,
                                             mut countLimit: s16) {
    let mut i: s16 = 0;
    if !projectedPos.is_null() &&
           ((*projectedPos).z > 500.0f32 || (*projectedPos).z < 0.0f32) {
        return
    }
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < countLimit as libc::c_int {
        if (*effect).type_0 as libc::c_int == FS_EFF_NONE as libc::c_int {
            (*effect).type_0 = FS_EFF_RIPPLE as libc::c_int as u8_0;
            (*effect).pos = *pos;
            (*effect).vel = sZeroVec;
            (*effect).accel = sZeroVec;
            (*effect).unk_30 = arg3 * 0.0025f32;
            (*effect).unk_34 = arg4 * 0.0025f32;
            if arg3 > 300.0f32 {
                (*effect).alpha = 0 as libc::c_int as s16;
                (*effect).unk_2E = arg5;
                (*effect).unk_2C = 0 as libc::c_int as s16;
                (*effect).unk_38 =
                    ((*effect).unk_34 - (*effect).unk_30) * 0.05f32
            } else {
                (*effect).alpha = arg5;
                (*effect).unk_2C = 1 as libc::c_int as s16;
                (*effect).unk_38 =
                    ((*effect).unk_34 - (*effect).unk_30) * 0.1f32
            }
            break ;
        } else { effect = effect.offset(1); i += 1 }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Fishing_SpawnDustSplash(mut projectedPos: *mut Vec3f,
                                                 mut effect:
                                                     *mut FishingEffect,
                                                 mut pos: *mut Vec3f,
                                                 mut vel: *mut Vec3f,
                                                 mut scale: f32_0) {
    let mut i: s16 = 0;
    let mut accel: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: -1.0f32, z: 0.0f32,}; init };
    if !projectedPos.is_null() &&
           ((*projectedPos).z > 500.0f32 || (*projectedPos).z < 0.0f32) {
        return
    }
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < 100 as libc::c_int {
        if (*effect).type_0 as libc::c_int == FS_EFF_NONE as libc::c_int ||
               (*effect).type_0 as libc::c_int ==
                   FS_EFF_RAIN_DROP as libc::c_int ||
               (*effect).type_0 as libc::c_int ==
                   FS_EFF_RAIN_RIPPLE as libc::c_int ||
               (*effect).type_0 as libc::c_int ==
                   FS_EFF_RAIN_SPLASH as libc::c_int {
            (*effect).type_0 = FS_EFF_DUST_SPLASH as libc::c_int as u8_0;
            (*effect).pos = *pos;
            (*effect).vel = *vel;
            (*effect).accel = accel;
            (*effect).alpha =
                (100 as libc::c_int +
                     Rand_ZeroFloat(100.0f32) as s16 as libc::c_int) as s16;
            (*effect).unk_30 = scale;
            break ;
        } else { effect = effect.offset(1); i += 1 }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Fishing_SpawnWaterDust(mut projectedPos: *mut Vec3f,
                                                mut effect:
                                                    *mut FishingEffect,
                                                mut pos: *mut Vec3f,
                                                mut scale: f32_0) {
    let mut i: s16 = 0;
    let mut accel: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.05f32, z: 0.0f32,}; init };
    if !projectedPos.is_null() &&
           ((*projectedPos).z > 500.0f32 || (*projectedPos).z < 0.0f32) {
        return
    }
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < 90 as libc::c_int {
        if (*effect).type_0 as libc::c_int == FS_EFF_NONE as libc::c_int {
            (*effect).type_0 = FS_EFF_WATER_DUST as libc::c_int as u8_0;
            (*effect).pos = *pos;
            (*effect).vel = sZeroVec;
            (*effect).accel = accel;
            (*effect).alpha = 255 as libc::c_int as s16;
            (*effect).timer = Rand_ZeroFloat(100.0f32) as s16 as u8_0;
            (*effect).unk_30 = scale;
            (*effect).unk_34 = 2.0f32 * scale;
            break ;
        } else { effect = effect.offset(1); i += 1 }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Fishing_SpawnBubble(mut projectedPos: *mut Vec3f,
                                             mut effect: *mut FishingEffect,
                                             mut pos: *mut Vec3f,
                                             mut scale: f32_0,
                                             mut arg4: u8_0) {
    let mut i: s16 = 0;
    let mut vel: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 1.0f32, z: 0.0f32,}; init };
    if !projectedPos.is_null() &&
           ((*projectedPos).z > 500.0f32 || (*projectedPos).z < 0.0f32) {
        return
    }
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < 90 as libc::c_int {
        if (*effect).type_0 as libc::c_int == FS_EFF_NONE as libc::c_int {
            (*effect).type_0 = FS_EFF_BUBBLE as libc::c_int as u8_0;
            (*effect).pos = *pos;
            (*effect).vel = vel;
            (*effect).accel = sZeroVec;
            (*effect).timer = Rand_ZeroFloat(100.0f32) as s16 as u8_0;
            (*effect).unk_30 = scale;
            (*effect).unk_2C = arg4 as s16;
            break ;
        } else { effect = effect.offset(1); i += 1 }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Fishing_SpawnRainDrop(mut effect: *mut FishingEffect,
                                               mut pos: *mut Vec3f,
                                               mut rot: *mut Vec3f) {
    let mut i: s16 = 0;
    let mut velSrc: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    velSrc.x = 0.0f32;
    velSrc.y = 0.0f32;
    velSrc.z = 300.0f32;
    effect = effect.offset(30 as libc::c_int as isize);
    i = 30 as libc::c_int as s16;
    while (i as libc::c_int) < 130 as libc::c_int {
        if (*effect).type_0 as libc::c_int == FS_EFF_NONE as libc::c_int {
            (*effect).type_0 = FS_EFF_RAIN_DROP as libc::c_int as u8_0;
            (*effect).pos = *pos;
            (*effect).accel = sZeroVec;
            (*effect).unk_34 = (*rot).x;
            (*effect).unk_38 = (*rot).y;
            (*effect).unk_3C = (*rot).z;
            Matrix_RotateY((*rot).y, MTXMODE_NEW as libc::c_int as u8_0);
            Matrix_RotateX((*rot).x, MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_MultVec3f(&mut velSrc, &mut (*effect).vel);
            break ;
        } else { effect = effect.offset(1); i += 1 }
    };
}
static mut sPondPropInits: [FishingPropInit; 141] =
    [{
         let mut init =
             FishingPropInit{type_0: FS_PROP_ROCK as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 529 as libc::c_int as s16,
                                               y: -(53 as libc::c_int) as s16,
                                               z:
                                                   -(498 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_ROCK as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 461 as libc::c_int as s16,
                                               y: -(66 as libc::c_int) as s16,
                                               z:
                                                   -(480 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_ROCK as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 398 as libc::c_int as s16,
                                               y: -(73 as libc::c_int) as s16,
                                               z:
                                                   -(474 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_ROCK as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(226 as libc::c_int) as
                                                       s16,
                                               y: -(52 as libc::c_int) as s16,
                                               z:
                                                   -(691 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_ROCK as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(300 as libc::c_int) as
                                                       s16,
                                               y: -(41 as libc::c_int) as s16,
                                               z:
                                                   -(710 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_ROCK as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(333 as libc::c_int) as
                                                       s16,
                                               y: -(50 as libc::c_int) as s16,
                                               z:
                                                   -(643 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_ROCK as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(387 as libc::c_int) as
                                                       s16,
                                               y: -(46 as libc::c_int) as s16,
                                               z:
                                                   -(632 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_ROCK as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(484 as libc::c_int) as
                                                       s16,
                                               y: -(43 as libc::c_int) as s16,
                                               z:
                                                   -(596 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_ROCK as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(409 as libc::c_int) as
                                                       s16,
                                               y: -(57 as libc::c_int) as s16,
                                               z:
                                                   -(560 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_WOOD_POST as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 444 as libc::c_int as s16,
                                               y: -(87 as libc::c_int) as s16,
                                               z:
                                                   -(322 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_WOOD_POST as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 447 as libc::c_int as s16,
                                               y: -(91 as libc::c_int) as s16,
                                               z:
                                                   -(274 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_WOOD_POST as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 395 as libc::c_int as s16,
                                               y:
                                                   -(109 as libc::c_int) as
                                                       s16,
                                               z:
                                                   -(189 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_REED as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 617 as libc::c_int as s16,
                                               y: -(29 as libc::c_int) as s16,
                                               z: 646 as libc::c_int as s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_REED as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 698 as libc::c_int as s16,
                                               y: -(26 as libc::c_int) as s16,
                                               z: 584 as libc::c_int as s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_REED as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 711 as libc::c_int as s16,
                                               y: -(29 as libc::c_int) as s16,
                                               z: 501 as libc::c_int as s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_REED as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 757 as libc::c_int as s16,
                                               y: -(28 as libc::c_int) as s16,
                                               z: 457 as libc::c_int as s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_REED as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 812 as libc::c_int as s16,
                                               y: -(29 as libc::c_int) as s16,
                                               z: 341 as libc::c_int as s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_REED as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 856 as libc::c_int as s16,
                                               y: -(30 as libc::c_int) as s16,
                                               z: 235 as libc::c_int as s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_REED as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 847 as libc::c_int as s16,
                                               y: -(31 as libc::c_int) as s16,
                                               z: 83 as libc::c_int as s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_REED as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 900 as libc::c_int as s16,
                                               y: -(26 as libc::c_int) as s16,
                                               z: 119 as libc::c_int as s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 861 as libc::c_int as s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z: 137 as libc::c_int as s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 836 as libc::c_int as s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z: 150 as libc::c_int as s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 829 as libc::c_int as s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z: 200 as libc::c_int as s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 788 as libc::c_int as s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z: 232 as libc::c_int as s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 803 as libc::c_int as s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z: 319 as libc::c_int as s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 756 as libc::c_int as s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z: 348 as libc::c_int as s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 731 as libc::c_int as s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z: 377 as libc::c_int as s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 700 as libc::c_int as s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z: 392 as libc::c_int as s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 706 as libc::c_int as s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z: 351 as libc::c_int as s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 677 as libc::c_int as s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z: 286 as libc::c_int as s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 691 as libc::c_int as s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z: 250 as libc::c_int as s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 744 as libc::c_int as s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z: 290 as libc::c_int as s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 766 as libc::c_int as s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z: 201 as libc::c_int as s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 781 as libc::c_int as s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z: 128 as libc::c_int as s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 817 as libc::c_int as s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z: 46 as libc::c_int as s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 857 as libc::c_int as s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(50 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 724 as libc::c_int as s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z: 110 as libc::c_int as s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 723 as libc::c_int as s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z: 145 as libc::c_int as s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 728 as libc::c_int as s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z: 202 as libc::c_int as s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 721 as libc::c_int as s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z: 237 as libc::c_int as s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 698 as libc::c_int as s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z: 312 as libc::c_int as s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 660 as libc::c_int as s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z: 349 as libc::c_int as s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 662 as libc::c_int as s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z: 388 as libc::c_int as s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 667 as libc::c_int as s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z: 432 as libc::c_int as s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 732 as libc::c_int as s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z: 429 as libc::c_int as s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 606 as libc::c_int as s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z: 366 as libc::c_int as s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 604 as libc::c_int as s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z: 286 as libc::c_int as s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 620 as libc::c_int as s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z: 217 as libc::c_int as s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 663 as libc::c_int as s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z: 159 as libc::c_int as s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 682 as libc::c_int as s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z: 73 as libc::c_int as s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 777 as libc::c_int as s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z: 83 as libc::c_int as s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 766 as libc::c_int as s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z: 158 as libc::c_int as s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_REED as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 1073 as libc::c_int as s16,
                                               y: 0 as libc::c_int as s16,
                                               z:
                                                   -(876 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_REED as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 970 as libc::c_int as s16,
                                               y: 0 as libc::c_int as s16,
                                               z:
                                                   -(853 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_REED as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 896 as libc::c_int as s16,
                                               y: 0 as libc::c_int as s16,
                                               z:
                                                   -(886 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_REED as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 646 as libc::c_int as s16,
                                               y: -(27 as libc::c_int) as s16,
                                               z:
                                                   -(651 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_REED as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 597 as libc::c_int as s16,
                                               y: -(29 as libc::c_int) as s16,
                                               z:
                                                   -(657 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_REED as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 547 as libc::c_int as s16,
                                               y: -(32 as libc::c_int) as s16,
                                               z:
                                                   -(651 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_REED as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 690 as libc::c_int as s16,
                                               y: -(29 as libc::c_int) as s16,
                                               z:
                                                   -(546 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_REED as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 720 as libc::c_int as s16,
                                               y: -(29 as libc::c_int) as s16,
                                               z:
                                                   -(490 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_REED as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(756 as libc::c_int) as
                                                       s16,
                                               y: -(30 as libc::c_int) as s16,
                                               z:
                                                   -(409 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_REED as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(688 as libc::c_int) as
                                                       s16,
                                               y: -(34 as libc::c_int) as s16,
                                               z:
                                                   -(458 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_REED as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(613 as libc::c_int) as
                                                       s16,
                                               y: -(34 as libc::c_int) as s16,
                                               z:
                                                   -(581 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(593 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(479 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(602 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(421 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(664 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(371 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(708 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(316 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(718 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(237 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_REED as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(807 as libc::c_int) as
                                                       s16,
                                               y: -(36 as libc::c_int) as s16,
                                               z:
                                                   -(183 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_REED as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(856 as libc::c_int) as
                                                       s16,
                                               y: -(29 as libc::c_int) as s16,
                                               z:
                                                   -(259 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(814 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(317 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(759 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(384 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(718 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(441 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(474 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(567 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(519 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(517 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(539 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(487 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(575 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(442 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(594 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(525 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(669 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(514 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(653 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(456 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_REED as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(663 as libc::c_int) as
                                                       s16,
                                               y: -(28 as libc::c_int) as s16,
                                               z:
                                                   -(606 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_REED as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(708 as libc::c_int) as
                                                       s16,
                                               y: -(26 as libc::c_int) as s16,
                                               z:
                                                   -(567 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_REED as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(739 as libc::c_int) as
                                                       s16,
                                               y: -(27 as libc::c_int) as s16,
                                               z:
                                                   -(506 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_REED as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(752 as libc::c_int) as
                                                       s16,
                                               y: -(28 as libc::c_int) as s16,
                                               z:
                                                   -(464 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_REED as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(709 as libc::c_int) as
                                                       s16,
                                               y: -(29 as libc::c_int) as s16,
                                               z:
                                                   -(513 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(544 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(436 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(559 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(397 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(616 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(353 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(712 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(368 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(678 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(403 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(664 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(273 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(630 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(276 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(579 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(311 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(588 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(351 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(555 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(534 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(547 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(567 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(592 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(571 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(541 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(610 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(476 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(629 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(439 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(598 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(412 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(550 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(411 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(606 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(370 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(634 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(352 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(662 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(413 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(641 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(488 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(666 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(578 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(656 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(560 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(640 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(531 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(654 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(451 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(669 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(439 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(699 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(482 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(719 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(524 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(720 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(569 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(714 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_REED as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(520 as libc::c_int) as
                                                       s16,
                                               y: -(27 as libc::c_int) as s16,
                                               z:
                                                   -(727 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_REED as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(572 as libc::c_int) as
                                                       s16,
                                               y: -(28 as libc::c_int) as s16,
                                               z:
                                                   -(686 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_REED as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(588 as libc::c_int) as
                                                       s16,
                                               y: -(32 as libc::c_int) as s16,
                                               z:
                                                   -(631 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_REED as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(622 as libc::c_int) as
                                                       s16,
                                               y: -(34 as libc::c_int) as s16,
                                               z:
                                                   -(571 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_REED as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(628 as libc::c_int) as
                                                       s16,
                                               y: -(36 as libc::c_int) as s16,
                                               z:
                                                   -(510 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_REED as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(655 as libc::c_int) as
                                                       s16,
                                               y: -(36 as libc::c_int) as s16,
                                               z:
                                                   -(466 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_REED as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(655 as libc::c_int) as
                                                       s16,
                                               y: -(41 as libc::c_int) as s16,
                                               z:
                                                   -(393 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_REED as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(661 as libc::c_int) as
                                                       s16,
                                               y: -(47 as libc::c_int) as s16,
                                               z:
                                                   -(328 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_REED as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(723 as libc::c_int) as
                                                       s16,
                                               y: -(40 as libc::c_int) as s16,
                                               z:
                                                   -(287 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_REED as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(756 as libc::c_int) as
                                                       s16,
                                               y: -(33 as libc::c_int) as s16,
                                               z:
                                                   -(349 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_REED as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(755 as libc::c_int) as
                                                       s16,
                                               y: -(43 as libc::c_int) as s16,
                                               z:
                                                   -(210 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(770 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(281 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(750 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(313 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(736 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(341 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(620 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(418 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(601 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(371 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(635 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(383 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(627 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(311 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(665 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(327 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(524 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(537 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(514 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(579 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(512 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(623 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(576 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(582 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(600 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(608 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(657 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(531 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_LILY_PAD as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(641 as libc::c_int) as
                                                       s16,
                                               y: -(22 as libc::c_int) as s16,
                                               z:
                                                   -(547 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             FishingPropInit{type_0: FS_PROP_INIT_STOP as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 0 as libc::c_int as s16,
                                               y: 0,
                                               z: 0,};
                                     init
                                 },};
         init
     }];
#[no_mangle]
pub unsafe extern "C" fn Fishing_InitPondProps(mut this: *mut Fishing,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    let mut prop: *mut FishingProp =
        &mut *sPondProps.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut FishingProp;
    let mut colliderPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut i: s16 = 0;
    Fishing_SeedRand(1 as libc::c_int, 29100 as libc::c_int,
                     9786 as libc::c_int);
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < 140 as libc::c_int {
        if sPondPropInits[i as usize].type_0 as libc::c_int ==
               FS_PROP_INIT_STOP as libc::c_int {
            break ;
        }
        (*prop).type_0 = sPondPropInits[i as usize].type_0;
        (*prop).pos.x = sPondPropInits[i as usize].pos.x as f32_0;
        (*prop).pos.y = sPondPropInits[i as usize].pos.y as f32_0;
        (*prop).pos.z = sPondPropInits[i as usize].pos.z as f32_0;
        (*prop).rotX = 0.0f32;
        (*prop).reedAngle = 0.0f32;
        (*prop).timer = Rand_ZeroFloat(100.0f32) as s16;
        (*prop).drawDistance = 800.0f32;
        if (*prop).type_0 as libc::c_int == FS_PROP_REED as libc::c_int {
            (*prop).scale = Fishing_RandZeroOne() * 0.25f32 + 0.75f32;
            (*prop).reedAngle =
                Rand_ZeroFloat(2 as libc::c_int as libc::c_float *
                                   3.14159265358979323846f32);
            if sLinkAge as libc::c_int == 1 as libc::c_int {
                (*prop).scale *= 0.6f32
            }
            (*prop).drawDistance = 1200.0f32
        } else if (*prop).type_0 as libc::c_int ==
                      FS_PROP_WOOD_POST as libc::c_int {
            (*prop).scale = 0.08f32;
            (*prop).drawDistance = 1200.0f32;
            colliderPos = (*prop).pos;
            colliderPos.y += 50.0f32;
            Fishing_SetColliderElement(i as s32,
                                       &mut (*sFishingMain).collider,
                                       &mut colliderPos,
                                       (*prop).scale * 3.5f32);
        } else if (*prop).type_0 as libc::c_int ==
                      FS_PROP_LILY_PAD as libc::c_int {
            (*prop).scale = Fishing_RandZeroOne() * 0.3f32 + 0.5f32;
            (*prop).rotY =
                Rand_ZeroFloat(2 as libc::c_int as libc::c_float *
                                   3.14159265358979323846f32);
            if sLinkAge as libc::c_int == 1 as libc::c_int {
                if i as libc::c_int % 4 as libc::c_int != 0 as libc::c_int {
                    (*prop).scale *= 0.6f32
                } else {
                    (*prop).type_0 = FS_PROP_NONE as libc::c_int as u8_0
                }
            }
        } else {
            (*prop).scale = Fishing_RandZeroOne() * 0.1f32 + 0.3f32;
            (*prop).rotY =
                Rand_ZeroFloat(2 as libc::c_int as libc::c_float *
                                   3.14159265358979323846f32);
            (*prop).drawDistance = 1000.0f32;
            Fishing_SetColliderElement(i as s32,
                                       &mut (*sFishingMain).collider,
                                       &mut (*prop).pos, (*prop).scale);
        }
        prop = prop.offset(1);
        i += 1
    };
}
static mut sFishInits: [FishingFishInit; 17] =
    [{
         let mut init =
             FishingFishInit{unk_00: 0 as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 666 as libc::c_int as s16,
                                               y: -(45 as libc::c_int) as s16,
                                               z: 354 as libc::c_int as s16,};
                                     init
                                 },
                             unk_08: 38 as libc::c_int as u8_0,
                             unk_0C: 0.1f32,};
         init
     },
     {
         let mut init =
             FishingFishInit{unk_00: 0 as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 681 as libc::c_int as s16,
                                               y: -(45 as libc::c_int) as s16,
                                               z: 240 as libc::c_int as s16,};
                                     init
                                 },
                             unk_08: 36 as libc::c_int as u8_0,
                             unk_0C: 0.1f32,};
         init
     },
     {
         let mut init =
             FishingFishInit{unk_00: 0 as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 670 as libc::c_int as s16,
                                               y: -(45 as libc::c_int) as s16,
                                               z: 90 as libc::c_int as s16,};
                                     init
                                 },
                             unk_08: 41 as libc::c_int as u8_0,
                             unk_0C: 0.05f32,};
         init
     },
     {
         let mut init =
             FishingFishInit{unk_00: 0 as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 615 as libc::c_int as s16,
                                               y: -(45 as libc::c_int) as s16,
                                               z:
                                                   -(450 as libc::c_int) as
                                                       s16,};
                                     init
                                 },
                             unk_08: 35 as libc::c_int as u8_0,
                             unk_0C: 0.2f32,};
         init
     },
     {
         let mut init =
             FishingFishInit{unk_00: 0 as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 500 as libc::c_int as s16,
                                               y: -(45 as libc::c_int) as s16,
                                               z:
                                                   -(420 as libc::c_int) as
                                                       s16,};
                                     init
                                 },
                             unk_08: 39 as libc::c_int as u8_0,
                             unk_0C: 0.1f32,};
         init
     },
     {
         let mut init =
             FishingFishInit{unk_00: 0 as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 420 as libc::c_int as s16,
                                               y: -(45 as libc::c_int) as s16,
                                               z:
                                                   -(550 as libc::c_int) as
                                                       s16,};
                                     init
                                 },
                             unk_08: 44 as libc::c_int as u8_0,
                             unk_0C: 0.05f32,};
         init
     },
     {
         let mut init =
             FishingFishInit{unk_00: 0 as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(264 as libc::c_int) as
                                                       s16,
                                               y: -(45 as libc::c_int) as s16,
                                               z:
                                                   -(640 as libc::c_int) as
                                                       s16,};
                                     init
                                 },
                             unk_08: 40 as libc::c_int as u8_0,
                             unk_0C: 0.1f32,};
         init
     },
     {
         let mut init =
             FishingFishInit{unk_00: 0 as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(470 as libc::c_int) as
                                                       s16,
                                               y: -(45 as libc::c_int) as s16,
                                               z:
                                                   -(540 as libc::c_int) as
                                                       s16,};
                                     init
                                 },
                             unk_08: 34 as libc::c_int as u8_0,
                             unk_0C: 0.2f32,};
         init
     },
     {
         let mut init =
             FishingFishInit{unk_00: 0 as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(557 as libc::c_int) as
                                                       s16,
                                               y: -(45 as libc::c_int) as s16,
                                               z:
                                                   -(430 as libc::c_int) as
                                                       s16,};
                                     init
                                 },
                             unk_08: 54 as libc::c_int as u8_0,
                             unk_0C: 0.01f32,};
         init
     },
     {
         let mut init =
             FishingFishInit{unk_00: 0 as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(260 as libc::c_int) as
                                                       s16,
                                               y: -(60 as libc::c_int) as s16,
                                               z:
                                                   -(330 as libc::c_int) as
                                                       s16,};
                                     init
                                 },
                             unk_08: 47 as libc::c_int as u8_0,
                             unk_0C: 0.05f32,};
         init
     },
     {
         let mut init =
             FishingFishInit{unk_00: 0 as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(500 as libc::c_int) as
                                                       s16,
                                               y: -(60 as libc::c_int) as s16,
                                               z: 330 as libc::c_int as s16,};
                                     init
                                 },
                             unk_08: 42 as libc::c_int as u8_0,
                             unk_0C: 0.06f32,};
         init
     },
     {
         let mut init =
             FishingFishInit{unk_00: 0 as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 428 as libc::c_int as s16,
                                               y: -(40 as libc::c_int) as s16,
                                               z:
                                                   -(283 as libc::c_int) as
                                                       s16,};
                                     init
                                 },
                             unk_08: 33 as libc::c_int as u8_0,
                             unk_0C: 0.2f32,};
         init
     },
     {
         let mut init =
             FishingFishInit{unk_00: 0 as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 409 as libc::c_int as s16,
                                               y: -(70 as libc::c_int) as s16,
                                               z:
                                                   -(230 as libc::c_int) as
                                                       s16,};
                                     init
                                 },
                             unk_08: 57 as libc::c_int as u8_0,
                             unk_0C: 0.0f32,};
         init
     },
     {
         let mut init =
             FishingFishInit{unk_00: 0 as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 450 as libc::c_int as s16,
                                               y: -(67 as libc::c_int) as s16,
                                               z:
                                                   -(300 as libc::c_int) as
                                                       s16,};
                                     init
                                 },
                             unk_08: 63 as libc::c_int as u8_0,
                             unk_0C: 0.0f32,};
         init
     },
     {
         let mut init =
             FishingFishInit{unk_00: 0 as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(136 as libc::c_int) as
                                                       s16,
                                               y: -(65 as libc::c_int) as s16,
                                               z:
                                                   -(196 as libc::c_int) as
                                                       s16,};
                                     init
                                 },
                             unk_08: 71 as libc::c_int as u8_0,
                             unk_0C: 0.0f32,};
         init
     },
     {
         let mut init =
             FishingFishInit{unk_00: 1 as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(561 as libc::c_int) as
                                                       s16,
                                               y: -(35 as libc::c_int) as s16,
                                               z:
                                                   -(547 as libc::c_int) as
                                                       s16,};
                                     init
                                 },
                             unk_08: 45 as libc::c_int as u8_0,
                             unk_0C: 0.0f32,};
         init
     },
     {
         let mut init =
             FishingFishInit{unk_00: 1 as libc::c_int as u8_0,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 667 as libc::c_int as s16,
                                               y: -(35 as libc::c_int) as s16,
                                               z: 317 as libc::c_int as s16,};
                                     init
                                 },
                             unk_08: 43 as libc::c_int as u8_0,
                             unk_0C: 0.0f32,};
         init
     }];
// Initialized in run_static_initializers
static mut sInitChain: [InitChainEntry; 2] =
    [InitChainEntry{cont_type_0_offset_value: [0; 4],}; 2];
#[no_mangle]
pub unsafe extern "C" fn Fishing_Init(mut thisx: *mut Actor,
                                      mut globalCtx2: *mut GlobalContext) {
    let mut globalCtx: *mut GlobalContext = globalCtx2;
    let mut this: *mut Fishing = thisx as *mut Fishing;
    let mut fishCount: u16_0 = 0;
    let mut i: s16 = 0;
    Actor_ProcessInitChain(thisx, sInitChain.as_mut_ptr());
    ActorShape_Init(&mut (*thisx).shape, 0.0f32, None, 0.0f32);
    if (*gGameInfo).data[(13 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 5 as libc::c_int) as usize]
           as libc::c_int != 0 as libc::c_int {
        sLinkAge = 1 as libc::c_int as u8_0
    } else { sLinkAge = gSaveContext.linkAge as u8_0 }
    if ((*thisx).params as libc::c_int) < 100 as libc::c_int {
        D_80B7E074 = 0 as libc::c_int as u8_0;
        sFishingMain = this;
        Collider_InitJntSph(globalCtx, &mut (*sFishingMain).collider);
        Collider_SetJntSph(globalCtx, &mut (*sFishingMain).collider, thisx,
                           &mut sJntSphInit,
                           (*sFishingMain).colliderElements.as_mut_ptr());
        (*thisx).params = 1 as libc::c_int as s16;
        SkelAnime_InitFlex(globalCtx, &mut (*this).skelAnime,
                           &mut gFishingOwnerSkel, &mut gFishingOwnerAnim,
                           0 as *mut Vec3s, 0 as *mut Vec3s,
                           0 as libc::c_int);
        Animation_MorphToLoop(&mut (*this).skelAnime, &mut gFishingOwnerAnim,
                              0.0f32);
        (*thisx).update =
            Some(Fishing_UpdateOwner as
                     unsafe extern "C" fn(_: *mut Actor,
                                          _: *mut GlobalContext) -> ());
        (*thisx).draw =
            Some(Fishing_DrawOwner as
                     unsafe extern "C" fn(_: *mut Actor,
                                          _: *mut GlobalContext) -> ());
        (*thisx).shape.rot.y = -(0x6000 as libc::c_int) as s16;
        (*thisx).world.pos.x = 160.0f32;
        (*thisx).world.pos.y = -2.0f32;
        (*thisx).world.pos.z = 1208.0f32;
        Actor_SetScale(thisx, 0.011f32);
        (*thisx).focus.pos = (*thisx).world.pos;
        (*thisx).focus.pos.y += 75.0f32;
        (*thisx).flags |=
            ((1 as libc::c_int) << 0 as libc::c_int |
                 (1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint;
        if sLinkAge as libc::c_int != 1 as libc::c_int {
            if gSaveContext.highScores[HS_FISHING as libc::c_int as usize] &
                   0x1000 as libc::c_int != 0 {
                D_80B7A688 = 0 as libc::c_int as u8_0
            } else { D_80B7A688 = 1 as libc::c_int as u8_0 }
        } else { D_80B7A688 = 2 as libc::c_int as u8_0 }
        D_80B7A684 = 20 as libc::c_int as s16;
        (*globalCtx).specialEffects =
            sFishingEffects.as_mut_ptr() as *mut libc::c_void;
        gTimeIncrement = 1 as libc::c_int as u16_0;
        D_80B7E0AC = 0 as libc::c_int as s16;
        D_80B7E0A6 = 10 as libc::c_int as s16;
        Audio_QueueSeqCmd(((0x1 as libc::c_int) << 28 as libc::c_int |
                               (SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                                   24 as libc::c_int | 0x100ff as libc::c_int)
                              as u32_0);
        if sLinkAge as libc::c_int == 1 as libc::c_int {
            if gSaveContext.highScores[HS_FISHING as libc::c_int as usize] &
                   0x7f as libc::c_int != 0 as libc::c_int {
                D_80B7E078 =
                    (gSaveContext.highScores[HS_FISHING as libc::c_int as
                                                 usize] & 0x7f as libc::c_int)
                        as f32_0
            } else { D_80B7E078 = 40.0f32 }
        } else if gSaveContext.highScores[HS_FISHING as libc::c_int as usize]
                      & 0x7f000000 as libc::c_int != 0 as libc::c_int {
            D_80B7E078 =
                ((gSaveContext.highScores[HS_FISHING as libc::c_int as usize]
                      & 0x7f000000 as libc::c_int) >> 0x18 as libc::c_int) as
                    f32_0
        } else { D_80B7E078 = 45.0f32 }
        D_80B7E07D =
            ((gSaveContext.highScores[HS_FISHING as libc::c_int as usize] &
                  0xff0000 as libc::c_int) >> 0x10 as libc::c_int) as u8_0;
        if D_80B7E07D as libc::c_int & 7 as libc::c_int == 7 as libc::c_int {
            (*globalCtx).roomCtx.unk_74[0 as libc::c_int as usize] =
                90 as libc::c_int as s16;
            D_80B7E076 = 1 as libc::c_int as u8_0
        } else {
            (*globalCtx).roomCtx.unk_74[0 as libc::c_int as usize] =
                40 as libc::c_int as s16;
            D_80B7E076 = 0 as libc::c_int as u8_0
        }
        if D_80B7E07D as libc::c_int & 7 as libc::c_int == 6 as libc::c_int ||
               (*gGameInfo).data[(13 as libc::c_int * 6 as libc::c_int *
                                      16 as libc::c_int + 3 as libc::c_int) as
                                     usize] as libc::c_int != 0 as libc::c_int
           {
            D_80B7E077 = 100 as libc::c_int as u8_0;
            if (*gGameInfo).data[(13 as libc::c_int * 6 as libc::c_int *
                                      16 as libc::c_int + 3 as libc::c_int) as
                                     usize] as libc::c_int != 0 as libc::c_int
               {
                (*gGameInfo).data[(13 as libc::c_int * 6 as libc::c_int *
                                       16 as libc::c_int + 3 as libc::c_int)
                                      as usize] = 0 as libc::c_int as s16;
                gSaveContext.highScores[HS_FISHING as libc::c_int as usize] =
                    (gSaveContext.highScores[HS_FISHING as libc::c_int as
                                                 usize] as libc::c_uint &
                         0xff00ffff as libc::c_uint) as s32;
                gSaveContext.highScores[HS_FISHING as libc::c_int as usize] |=
                    0x60000 as libc::c_int
            }
        } else { D_80B7E077 = 0 as libc::c_int as u8_0 }
        i = 0 as libc::c_int as s16;
        while (i as libc::c_int) < 130 as libc::c_int {
            sFishingEffects[i as usize].type_0 =
                FS_EFF_NONE as libc::c_int as u8_0;
            i += 1
        }
        i = 0 as libc::c_int as s16;
        while (i as libc::c_int) < 140 as libc::c_int {
            sPondProps[i as usize].type_0 =
                FS_PROP_NONE as libc::c_int as u8_0;
            i += 1
        }
        sFishGroupAngle1 = 0.7f32;
        sFishGroupAngle2 = 2.3f32;
        sFishGroupAngle3 = 4.6f32;
        i = 0 as libc::c_int as s16;
        while (i as libc::c_int) < 60 as libc::c_int {
            let mut fish: *mut FishingGroupFish =
                &mut *sGroupFishes.as_mut_ptr().offset(i as isize) as
                    *mut FishingGroupFish;
            (*fish).type_0 = FS_GROUP_FISH_NORMAL as libc::c_int as u8_0;
            if i as libc::c_int <= 20 as libc::c_int {
                (*fish).pos.x = sinf(sFishGroupAngle1) * 720.0f32;
                (*fish).unk_10.x = (*fish).pos.x;
                (*fish).pos.z = cosf(sFishGroupAngle1) * 720.0f32;
                (*fish).unk_10.z = (*fish).pos.z
            } else if i as libc::c_int <= 40 as libc::c_int {
                (*fish).pos.x = sinf(sFishGroupAngle2) * 720.0f32;
                (*fish).unk_10.x = (*fish).pos.x;
                (*fish).pos.z = cosf(sFishGroupAngle2) * 720.0f32;
                (*fish).unk_10.z = (*fish).pos.z
            } else {
                (*fish).pos.x = sinf(sFishGroupAngle3) * 720.0f32;
                (*fish).unk_10.x = (*fish).pos.x;
                (*fish).pos.z = cosf(sFishGroupAngle3) * 720.0f32;
                (*fish).unk_10.z = (*fish).pos.z
            }
            (*fish).pos.y = -35.0f32;
            (*fish).unk_10.y = (*fish).pos.y;
            (*fish).timer = Rand_ZeroFloat(100.0f32) as s16;
            (*fish).unk_3C = 0 as libc::c_int as s16;
            (*fish).unk_3E = 0 as libc::c_int as s16;
            (*fish).unk_40 = 0 as libc::c_int as s16;
            if sLinkAge as libc::c_int != 1 as libc::c_int {
                if i as libc::c_int >= 15 as libc::c_int &&
                       (i as libc::c_int) < 20 as libc::c_int ||
                       i as libc::c_int >= 35 as libc::c_int &&
                           (i as libc::c_int) < 40 as libc::c_int ||
                       i as libc::c_int >= 55 as libc::c_int &&
                           (i as libc::c_int) < 60 as libc::c_int {
                    (*fish).type_0 = FS_GROUP_FISH_NONE as libc::c_int as u8_0
                }
            }
            i += 1
        }
        Fishing_InitPondProps(this, globalCtx);
        Actor_SpawnAsChild(&mut (*globalCtx).actorCtx, thisx, globalCtx,
                           ACTOR_EN_KANBAN as libc::c_int as s16, 53.0f32,
                           -17.0f32, 982.0f32, 0 as libc::c_int as s16,
                           0 as libc::c_int as s16, 0 as libc::c_int as s16,
                           0x300 as libc::c_int as s16);
        Actor_Spawn(&mut (*globalCtx).actorCtx, globalCtx,
                    ACTOR_FISHING as libc::c_int as s16, 0.0f32, 0.0f32,
                    0.0f32, 0 as libc::c_int as s16, 0 as libc::c_int as s16,
                    0 as libc::c_int as s16, 200 as libc::c_int as s16);
        if (*gGameInfo).data[(13 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 1 as libc::c_int) as
                                 usize] as libc::c_int == 1 as libc::c_int ||
               D_80B7E07D as libc::c_int & 3 as libc::c_int ==
                   3 as libc::c_int {
            if sLinkAge as libc::c_int != 1 as libc::c_int {
                fishCount = 16 as libc::c_int as u16_0
            } else { fishCount = 17 as libc::c_int as u16_0 }
        } else { fishCount = 15 as libc::c_int as u16_0 }
        i = 0 as libc::c_int as s16;
        while (i as libc::c_int) < fishCount as libc::c_int {
            Actor_Spawn(&mut (*globalCtx).actorCtx, globalCtx,
                        ACTOR_FISHING as libc::c_int as s16,
                        sFishInits[i as usize].pos.x as f32_0,
                        sFishInits[i as usize].pos.y as f32_0,
                        sFishInits[i as usize].pos.z as f32_0,
                        0 as libc::c_int as s16,
                        Rand_ZeroFloat(0x10000 as libc::c_int as f32_0) as
                            s16, 0 as libc::c_int as s16,
                        (100 as libc::c_int + i as libc::c_int) as s16);
            i += 1
        }
    } else {
        if ((*thisx).params as libc::c_int) < 115 as libc::c_int ||
               (*thisx).params as libc::c_int == 200 as libc::c_int {
            SkelAnime_InitFlex(globalCtx, &mut (*this).skelAnime,
                               &mut gFishingFishSkel, &mut gFishingFishAnim,
                               0 as *mut Vec3s, 0 as *mut Vec3s,
                               0 as libc::c_int);
            Animation_MorphToLoop(&mut (*this).skelAnime,
                                  &mut gFishingFishAnim, 0.0f32);
        } else {
            SkelAnime_InitFlex(globalCtx, &mut (*this).skelAnime,
                               &mut gFishingLoachSkel, &mut gFishingLoachAnim,
                               0 as *mut Vec3s, 0 as *mut Vec3s,
                               0 as libc::c_int);
            Animation_MorphToLoop(&mut (*this).skelAnime,
                                  &mut gFishingLoachAnim, 0.0f32);
        }
        SkelAnime_Update(&mut (*this).skelAnime);
        if (*thisx).params as libc::c_int == 200 as libc::c_int {
            (*this).unk_158 = 100 as libc::c_int as s16;
            Actor_ChangeCategory(globalCtx, &mut (*globalCtx).actorCtx, thisx,
                                 ACTORCAT_PROP as libc::c_int as u8_0);
            (*thisx).targetMode = 0 as libc::c_int as s8;
            (*thisx).flags |=
                ((1 as libc::c_int) << 0 as libc::c_int |
                     (1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint;
            (*this).lightNode =
                LightContext_InsertLight(globalCtx,
                                         &mut (*globalCtx).lightCtx,
                                         &mut (*this).lightInfo)
        } else {
            (*this).unk_158 = 10 as libc::c_int as s16;
            (*this).unk_15A = 10 as libc::c_int as s16;
            (*this).unk_150 =
                sFishInits[((*thisx).params as libc::c_int -
                                100 as libc::c_int) as usize].unk_00;
            (*this).unk_1A8 =
                sFishInits[((*thisx).params as libc::c_int -
                                100 as libc::c_int) as usize].unk_0C;
            (*this).unk_1AC =
                sFishInits[((*thisx).params as libc::c_int -
                                100 as libc::c_int) as usize].unk_08 as f32_0;
            (*this).unk_1AC += Rand_ZeroFloat(4.99999f32);
            if (*this).unk_1AC >= 65.0f32 && Rand_ZeroOne() < 0.05f32 {
                (*this).unk_1AC += Rand_ZeroFloat(7.99999f32)
            }
            if (*gGameInfo).data[(13 as libc::c_int * 6 as libc::c_int *
                                      16 as libc::c_int + 6 as libc::c_int) as
                                     usize] as libc::c_int != 0 as libc::c_int
               {
                (*this).unk_1AC =
                    (*gGameInfo).data[(13 as libc::c_int * 6 as libc::c_int *
                                           16 as libc::c_int +
                                           6 as libc::c_int) as usize] as
                        libc::c_int as libc::c_float + 80.0f32
            }
            if sLinkAge as libc::c_int == 1 as libc::c_int {
                (*this).unk_1AC *= 0.73f32
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Fishing_Destroy(mut thisx: *mut Actor,
                                         mut globalCtx2: *mut GlobalContext) {
    let mut globalCtx: *mut GlobalContext = globalCtx2;
    let mut this: *mut Fishing = thisx as *mut Fishing;
    SkelAnime_Free(&mut (*this).skelAnime, globalCtx);
    if (*thisx).params as libc::c_int == 200 as libc::c_int {
        LightContext_RemoveLight(globalCtx, &mut (*globalCtx).lightCtx,
                                 (*this).lightNode);
    } else if (*thisx).params as libc::c_int == 1 as libc::c_int {
        Collider_DestroyJntSph(globalCtx, &mut (*this).collider);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Fishing_UpdateEffects(mut effect: *mut FishingEffect,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    let mut rippleY: f32_0 = 0.;
    let mut i: s16 = 0;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < 130 as libc::c_int {
        if (*effect).type_0 != 0 {
            (*effect).timer = (*effect).timer.wrapping_add(1);
            (*effect).pos.x += (*effect).vel.x;
            (*effect).pos.y += (*effect).vel.y;
            (*effect).pos.z += (*effect).vel.z;
            (*effect).vel.y += (*effect).accel.y;
            if (*effect).type_0 as libc::c_int == FS_EFF_RIPPLE as libc::c_int
               {
                Math_ApproachF(&mut (*effect).unk_30, (*effect).unk_34,
                               0.2f32, (*effect).unk_38);
                if (*effect).unk_2C as libc::c_int == 0 as libc::c_int {
                    (*effect).alpha =
                        ((*effect).alpha as libc::c_int + 20 as libc::c_int)
                            as s16;
                    if (*effect).alpha as libc::c_int >=
                           (*effect).unk_2E as libc::c_int {
                        (*effect).alpha = (*effect).unk_2E;
                        (*effect).unk_2C += 1
                    }
                } else {
                    (*effect).alpha =
                        ((*effect).alpha as libc::c_int - 8 as libc::c_int) as
                            s16;
                    if (*effect).alpha as libc::c_int <= 0 as libc::c_int {
                        (*effect).type_0 = FS_EFF_NONE as libc::c_int as u8_0
                    }
                }
            } else if (*effect).type_0 as libc::c_int ==
                          FS_EFF_WATER_DUST as libc::c_int {
                Math_ApproachF(&mut (*effect).unk_30, (*effect).unk_34,
                               0.1f32, 0.1f32);
                (*effect).alpha =
                    ((*effect).alpha as libc::c_int - 10 as libc::c_int) as
                        s16;
                if (*effect).pos.y >
                       (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface
                           as libc::c_int as libc::c_float - 5.0f32 {
                    (*effect).accel.y = 0.0f32;
                    (*effect).vel.y = 0.0f32;
                    (*effect).alpha =
                        ((*effect).alpha as libc::c_int - 5 as libc::c_int) as
                            s16
                }
                if (*effect).alpha as libc::c_int <= 0 as libc::c_int {
                    (*effect).type_0 = FS_EFF_NONE as libc::c_int as u8_0
                }
            } else if (*effect).type_0 as libc::c_int ==
                          FS_EFF_BUBBLE as libc::c_int {
                if (*effect).unk_2C as libc::c_int == 0 as libc::c_int {
                    rippleY =
                        (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface
                            as f32_0
                } else { rippleY = 69.0f32 }
                if (*effect).pos.y >= rippleY {
                    (*effect).type_0 = FS_EFF_NONE as libc::c_int as u8_0;
                    if Rand_ZeroOne() < 0.3f32 {
                        let mut pos: Vec3f = (*effect).pos;
                        pos.y = rippleY;
                        Fishing_SpawnRipple(0 as *mut Vec3f,
                                            (*globalCtx).specialEffects as
                                                *mut FishingEffect, &mut pos,
                                            20.0f32, 60.0f32,
                                            150 as libc::c_int as s16,
                                            90 as libc::c_int as s16);
                    }
                }
            } else if (*effect).type_0 as libc::c_int ==
                          FS_EFF_DUST_SPLASH as libc::c_int {
                if (*effect).vel.y < -20.0f32 {
                    (*effect).vel.y = -20.0f32;
                    (*effect).accel.y = 0.0f32
                }
                if (*effect).pos.y <=
                       (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface
                           as libc::c_int as libc::c_float {
                    (*effect).type_0 = FS_EFF_NONE as libc::c_int as u8_0;
                    if Rand_ZeroOne() < 0.5f32 {
                        let mut pos_0: Vec3f = (*effect).pos;
                        pos_0.y =
                            (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface
                                as f32_0;
                        Fishing_SpawnRipple(0 as *mut Vec3f,
                                            (*globalCtx).specialEffects as
                                                *mut FishingEffect,
                                            &mut pos_0, 40.0f32, 110.0f32,
                                            150 as libc::c_int as s16,
                                            90 as libc::c_int as s16);
                    }
                }
            } else if (*effect).type_0 as libc::c_int ==
                          FS_EFF_RAIN_DROP as libc::c_int {
                if (*effect).pos.y <
                       (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface
                           as libc::c_int as libc::c_float {
                    let mut sqDistXZ: f32_0 =
                        (*effect).pos.x * (*effect).pos.x +
                            (*effect).pos.z * (*effect).pos.z;
                    if sqDistXZ > 920.0f32 * 920.0f32 {
                        (*effect).pos.y =
                            (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface
                                as libc::c_int as libc::c_float +
                                (sqrtf(sqDistXZ) - 920.0f32) * 0.11f32;
                        (*effect).timer =
                            ((*gGameInfo).data[(13 as libc::c_int *
                                                    6 as libc::c_int *
                                                    16 as libc::c_int +
                                                    17 as libc::c_int) as
                                                   usize] as libc::c_int +
                                 2 as libc::c_int) as u8_0;
                        (*effect).type_0 =
                            FS_EFF_RAIN_SPLASH as libc::c_int as u8_0;
                        (*effect).unk_30 =
                            ((*gGameInfo).data[(13 as libc::c_int *
                                                    6 as libc::c_int *
                                                    16 as libc::c_int +
                                                    18 as libc::c_int) as
                                                   usize] as libc::c_int +
                                 30 as libc::c_int) as libc::c_float *
                                0.001f32
                    } else {
                        (*effect).pos.y =
                            (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface
                                as libc::c_int as libc::c_float + 3.0f32;
                        (*effect).timer = 0 as libc::c_int as u8_0;
                        if Rand_ZeroOne() < 0.75f32 {
                            (*effect).type_0 =
                                FS_EFF_RAIN_RIPPLE as libc::c_int as u8_0;
                            (*effect).vel = sZeroVec;
                            (*effect).unk_30 =
                                ((*gGameInfo).data[(13 as libc::c_int *
                                                        6 as libc::c_int *
                                                        16 as libc::c_int +
                                                        18 as libc::c_int) as
                                                       usize] as libc::c_int +
                                     30 as libc::c_int) as libc::c_float *
                                    0.001f32
                        } else {
                            (*effect).type_0 =
                                FS_EFF_NONE as libc::c_int as u8_0
                        }
                    }
                    (*effect).vel = sZeroVec
                }
            } else if (*effect).type_0 as libc::c_int >=
                          FS_EFF_RAIN_RIPPLE as libc::c_int {
                (*effect).unk_30 +=
                    ((*gGameInfo).data[(13 as libc::c_int * 6 as libc::c_int *
                                            16 as libc::c_int +
                                            18 as libc::c_int) as usize] as
                         libc::c_int + 30 as libc::c_int) as libc::c_float *
                        0.001f32;
                if (*effect).timer as libc::c_int >= 6 as libc::c_int {
                    (*effect).type_0 = FS_EFF_NONE as libc::c_int as u8_0
                }
            } else if (*effect).type_0 as libc::c_int ==
                          FS_EFF_OWNER_HAT as libc::c_int {
                let mut sqDistXZ_0: f32_0 = 0.;
                let mut bottomY: f32_0 = 0.;
                (*effect).unk_30 = 0.010000001f32;
                Math_ApproachS(&mut sEffOwnerHatRot.y,
                               0 as libc::c_int as s16,
                               20 as libc::c_int as s16,
                               100 as libc::c_int as s16);
                Math_ApproachS(&mut sEffOwnerHatRot.x,
                               0 as libc::c_int as s16,
                               20 as libc::c_int as s16,
                               100 as libc::c_int as s16);
                Math_ApproachS(&mut sEffOwnerHatRot.z,
                               -(0x4000 as libc::c_int) as s16,
                               20 as libc::c_int as s16,
                               100 as libc::c_int as s16);
                sqDistXZ_0 =
                    (*effect).pos.x * (*effect).pos.x +
                        (*effect).pos.z * (*effect).pos.z;
                bottomY =
                    (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface as
                        libc::c_int as libc::c_float +
                        (sqrtf(sqDistXZ_0) - 920.0f32) * 0.147f32;
                if (*effect).pos.y > bottomY - 10.0f32 {
                    (*effect).pos.y -= 0.1f32
                }
                if (*effect).timer as libc::c_int % 16 as libc::c_int ==
                       0 as libc::c_int {
                    let mut pos_1: Vec3f = (*effect).pos;
                    pos_1.y =
                        (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface
                            as f32_0;
                    Fishing_SpawnRipple(0 as *mut Vec3f,
                                        (*globalCtx).specialEffects as
                                            *mut FishingEffect, &mut pos_1,
                                        30.0f32, 300.0f32,
                                        150 as libc::c_int as s16,
                                        90 as libc::c_int as s16);
                }
                if (*effect).unk_2C as libc::c_int >= 0 as libc::c_int {
                    (*effect).unk_2C += 1
                }
                if (*effect).unk_2C as libc::c_int == 30 as libc::c_int {
                    Message_StartTextbox(globalCtx,
                                         0x40b3 as libc::c_int as u16_0,
                                         0 as *mut Actor);
                }
                if (*effect).unk_2C as libc::c_int >= 100 as libc::c_int &&
                       Message_GetState(&mut (*globalCtx).msgCtx) as
                           libc::c_int == TEXT_STATE_EVENT as libc::c_int {
                    if Message_ShouldAdvance(globalCtx) as libc::c_int != 0 ||
                           Message_GetState(&mut (*globalCtx).msgCtx) as
                               libc::c_int == TEXT_STATE_NONE as libc::c_int {
                        Message_CloseTextbox(globalCtx);
                        Rupees_ChangeBy(-(50 as libc::c_int) as s16);
                        (*effect).unk_2C = -(1 as libc::c_int) as s16
                    }
                }
            }
        }
        effect = effect.offset(1);
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn Fishing_DrawEffects(mut effect: *mut FishingEffect,
                                             mut globalCtx:
                                                 *mut GlobalContext) {
    let mut flag: u8_0 = 0 as libc::c_int as u8_0;
    let mut rotY: f32_0 = 0.;
    let mut i: s16 = 0;
    let mut pad: s32 = 0;
    let mut firstEffect: *mut FishingEffect = effect;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_fishing.c\x00" as *const u8 as *const libc::c_char,
                    2271 as libc::c_int);
    Matrix_Push();
    let fresh0 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g: *mut Gfx = fresh0;
    (*_g).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g).words.w1 = 0 as libc::c_int as libc::c_uint;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < 100 as libc::c_int {
        if (*effect).type_0 as libc::c_int == FS_EFF_RIPPLE as libc::c_int {
            if flag as libc::c_int == 0 as libc::c_int {
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
                             (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                (*_g_0).words.w1 =
                    gFishingRippleMaterialDL.as_mut_ptr() as libc::c_uint;
                let fresh2 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_1: *mut Gfx = fresh2;
                (*_g_1).words.w0 =
                    (0xfb as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int;
                (*_g_1).words.w1 =
                    (155 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (155 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            16 as libc::c_int |
                        (155 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            8 as libc::c_int |
                        (0 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                flag = flag.wrapping_add(1)
            }
            let fresh3 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_2: *mut Gfx = fresh3;
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
                    ((*effect).alpha as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            Matrix_Translate((*effect).pos.x, (*effect).pos.y,
                             (*effect).pos.z,
                             MTXMODE_NEW as libc::c_int as u8_0);
            Matrix_Scale((*effect).unk_30, 1.0f32, (*effect).unk_30,
                         MTXMODE_APPLY as libc::c_int as u8_0);
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
                              b"../z_fishing.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              2305 as libc::c_int) as libc::c_uint;
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
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_4).words.w1 =
                gFishingRippleModelDL.as_mut_ptr() as libc::c_uint
        }
        effect = effect.offset(1);
        i += 1
    }
    effect = firstEffect;
    flag = 0 as libc::c_int as u8_0;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < 100 as libc::c_int {
        if (*effect).type_0 as libc::c_int ==
               FS_EFF_DUST_SPLASH as libc::c_int {
            if flag as libc::c_int == 0 as libc::c_int {
                let fresh6 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_5: *mut Gfx = fresh6;
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
                    gFishingDustSplashMaterialDL.as_mut_ptr() as libc::c_uint;
                let fresh7 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_6: *mut Gfx = fresh7;
                (*_g_6).words.w0 =
                    (0xfb as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int;
                (*_g_6).words.w1 =
                    (200 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (200 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            16 as libc::c_int |
                        (200 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            8 as libc::c_int |
                        (0 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                flag = flag.wrapping_add(1)
            }
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
                (180 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (180 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (180 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    ((*effect).alpha as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            Matrix_Translate((*effect).pos.x, (*effect).pos.y,
                             (*effect).pos.z,
                             MTXMODE_NEW as libc::c_int as u8_0);
            func_800D1FD4(&mut (*globalCtx).billboardMtxF);
            Matrix_Scale((*effect).unk_30, (*effect).unk_30, 1.0f32,
                         MTXMODE_APPLY as libc::c_int as u8_0);
            let fresh9 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_8: *mut Gfx = fresh9;
            (*_g_8).words.w0 =
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
            (*_g_8).words.w1 =
                Matrix_NewMtx((*globalCtx).state.gfxCtx,
                              b"../z_fishing.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              2346 as libc::c_int) as libc::c_uint;
            let fresh10 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_9: *mut Gfx = fresh10;
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
                gFishingDustSplashModelDL.as_mut_ptr() as libc::c_uint
        }
        effect = effect.offset(1);
        i += 1
    }
    effect = firstEffect;
    flag = 0 as libc::c_int as u8_0;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < 100 as libc::c_int {
        if (*effect).type_0 as libc::c_int == FS_EFF_WATER_DUST as libc::c_int
           {
            if flag as libc::c_int == 0 as libc::c_int {
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
                (*_g_10).words.w1 =
                    gFishingWaterDustMaterialDL.as_mut_ptr() as libc::c_uint;
                let fresh12 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_11: *mut Gfx = fresh12;
                (*_g_11).words.w0 =
                    (0xfb as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int;
                (*_g_11).words.w1 =
                    (40 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (90 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            16 as libc::c_int |
                        (80 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            8 as libc::c_int |
                        (128 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                flag = flag.wrapping_add(1)
            }
            let fresh13 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_12: *mut Gfx = fresh13;
            (*_g_12).words.w0 =
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
            (*_g_12).words.w1 =
                (40 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (90 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (80 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    ((*effect).alpha as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh14 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_13: *mut Gfx = fresh14;
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
                Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                                 ((*effect).timer as libc::c_int +
                                      i as libc::c_int * 3 as libc::c_int) as
                                     u32_0,
                                 (((*effect).timer as libc::c_int +
                                       i as libc::c_int * 3 as libc::c_int) *
                                      5 as libc::c_int) as u32_0,
                                 32 as libc::c_int, 64 as libc::c_int,
                                 1 as libc::c_int, 0 as libc::c_int as u32_0,
                                 0 as libc::c_int as u32_0, 32 as libc::c_int,
                                 32 as libc::c_int) as libc::c_uint;
            Matrix_Translate((*effect).pos.x, (*effect).pos.y,
                             (*effect).pos.z,
                             MTXMODE_NEW as libc::c_int as u8_0);
            func_800D1FD4(&mut (*globalCtx).billboardMtxF);
            Matrix_Scale((*effect).unk_30, (*effect).unk_30, 1.0f32,
                         MTXMODE_APPLY as libc::c_int as u8_0);
            let fresh15 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_14: *mut Gfx = fresh15;
            (*_g_14).words.w0 =
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
            (*_g_14).words.w1 =
                Matrix_NewMtx((*globalCtx).state.gfxCtx,
                              b"../z_fishing.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              2394 as libc::c_int) as libc::c_uint;
            let fresh16 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_15: *mut Gfx = fresh16;
            (*_g_15).words.w0 =
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
            (*_g_15).words.w1 =
                gFishingWaterDustModelDL.as_mut_ptr() as libc::c_uint
        }
        effect = effect.offset(1);
        i += 1
    }
    effect = firstEffect;
    flag = 0 as libc::c_int as u8_0;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < 100 as libc::c_int {
        if (*effect).type_0 as libc::c_int == FS_EFF_BUBBLE as libc::c_int {
            if flag as libc::c_int == 0 as libc::c_int {
                let fresh17 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_16: *mut Gfx = fresh17;
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
                (*_g_16).words.w1 =
                    gFishingBubbleMaterialDL.as_mut_ptr() as libc::c_uint;
                let fresh18 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_17: *mut Gfx = fresh18;
                (*_g_17).words.w0 =
                    (0xfb as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int;
                (*_g_17).words.w1 =
                    (150 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (150 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            16 as libc::c_int |
                        (150 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            8 as libc::c_int |
                        (0 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                let fresh19 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_18: *mut Gfx = fresh19;
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
                        (255 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                flag = flag.wrapping_add(1)
            }
            Matrix_Translate((*effect).pos.x, (*effect).pos.y,
                             (*effect).pos.z,
                             MTXMODE_NEW as libc::c_int as u8_0);
            func_800D1FD4(&mut (*globalCtx).billboardMtxF);
            Matrix_Scale((*effect).unk_30, (*effect).unk_30, 1.0f32,
                         MTXMODE_APPLY as libc::c_int as u8_0);
            let fresh20 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_19: *mut Gfx = fresh20;
            (*_g_19).words.w0 =
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
            (*_g_19).words.w1 =
                Matrix_NewMtx((*globalCtx).state.gfxCtx,
                              b"../z_fishing.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              2423 as libc::c_int) as libc::c_uint;
            let fresh21 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_20: *mut Gfx = fresh21;
            (*_g_20).words.w0 =
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
            (*_g_20).words.w1 =
                gFishingBubbleModelDL.as_mut_ptr() as libc::c_uint
        }
        effect = effect.offset(1);
        i += 1
    }
    effect = firstEffect.offset(30 as libc::c_int as isize);
    flag = 0 as libc::c_int as u8_0;
    i = 30 as libc::c_int as s16;
    while (i as libc::c_int) < 130 as libc::c_int {
        if (*effect).type_0 as libc::c_int == FS_EFF_RAIN_DROP as libc::c_int
           {
            if flag as libc::c_int == 0 as libc::c_int {
                (*__gfxCtx).polyXlu.p =
                    Gfx_CallSetupDL((*__gfxCtx).polyXlu.p,
                                    0x14 as libc::c_int as u32_0);
                let fresh22 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_21: *mut Gfx = fresh22;
                (*_g_21).words.w0 =
                    (0xfc as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (((31 as libc::c_int as u32_0 &
                               (((0x1 as libc::c_int) << 4 as libc::c_int) -
                                    1 as libc::c_int) as libc::c_uint) <<
                              20 as libc::c_int |
                              (31 as libc::c_int as u32_0 &
                                   (((0x1 as libc::c_int) << 5 as libc::c_int)
                                        - 1 as libc::c_int) as libc::c_uint)
                                  << 15 as libc::c_int |
                              (7 as libc::c_int as u32_0 &
                                   (((0x1 as libc::c_int) << 3 as libc::c_int)
                                        - 1 as libc::c_int) as libc::c_uint)
                                  << 12 as libc::c_int |
                              (7 as libc::c_int as u32_0 &
                                   (((0x1 as libc::c_int) << 3 as libc::c_int)
                                        - 1 as libc::c_int) as libc::c_uint)
                                  << 9 as libc::c_int |
                              ((31 as libc::c_int as u32_0 &
                                    (((0x1 as libc::c_int) <<
                                          4 as libc::c_int) -
                                         1 as libc::c_int) as libc::c_uint) <<
                                   5 as libc::c_int |
                                   (31 as libc::c_int as u32_0 &
                                        (((0x1 as libc::c_int) <<
                                              5 as libc::c_int) -
                                             1 as libc::c_int) as
                                            libc::c_uint) <<
                                       0 as libc::c_int)) &
                             (((0x1 as libc::c_int) << 24 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                (*_g_21).words.w1 =
                    (31 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        28 as libc::c_int |
                        (3 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 3 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            15 as libc::c_int |
                        (7 as libc::c_int as u32_0 &
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
                             24 as libc::c_int |
                             (7 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 3 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 21 as libc::c_int |
                             (7 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 3 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 18 as libc::c_int |
                             (3 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 3 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 6 as libc::c_int |
                             (7 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 3 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 3 as libc::c_int |
                             (3 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 3 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int);
                let fresh23 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_22: *mut Gfx = fresh23;
                (*_g_22).words.w0 =
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
                (*_g_22).words.w1 =
                    (150 as libc::c_int as u32_0 &
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
                        (30 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                flag = flag.wrapping_add(1)
            }
            Matrix_Translate((*effect).pos.x, (*effect).pos.y,
                             (*effect).pos.z,
                             MTXMODE_NEW as libc::c_int as u8_0);
            Matrix_RotateY((*effect).unk_38,
                           MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_RotateX((*effect).unk_34,
                           MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_RotateZ((*effect).unk_3C,
                           MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_Scale(0.002f32, 1.0f32, 0.1f32,
                         MTXMODE_APPLY as libc::c_int as u8_0);
            let fresh24 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_23: *mut Gfx = fresh24;
            (*_g_23).words.w0 =
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
            (*_g_23).words.w1 =
                Matrix_NewMtx((*globalCtx).state.gfxCtx,
                              b"../z_fishing.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              2467 as libc::c_int) as libc::c_uint;
            let fresh25 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_24: *mut Gfx = fresh25;
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
                gFishingRainDropModelDL.as_mut_ptr() as libc::c_uint
        }
        effect = effect.offset(1);
        i += 1
    }
    func_80093D84((*globalCtx).state.gfxCtx);
    effect = firstEffect.offset(30 as libc::c_int as isize);
    flag = 0 as libc::c_int as u8_0;
    i = 30 as libc::c_int as s16;
    while (i as libc::c_int) < 130 as libc::c_int {
        if (*effect).type_0 as libc::c_int ==
               FS_EFF_RAIN_RIPPLE as libc::c_int {
            if flag as libc::c_int == 0 as libc::c_int {
                let fresh26 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_25: *mut Gfx = fresh26;
                (*_g_25).words.w0 =
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
                (*_g_25).words.w1 =
                    gFishingRippleMaterialDL.as_mut_ptr() as libc::c_uint;
                let fresh27 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_26: *mut Gfx = fresh27;
                (*_g_26).words.w0 =
                    (0xfb as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int;
                (*_g_26).words.w1 =
                    (155 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (155 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            16 as libc::c_int |
                        (155 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            8 as libc::c_int |
                        (0 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                let fresh28 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_27: *mut Gfx = fresh28;
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
                        (130 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                flag = flag.wrapping_add(1)
            }
            Matrix_Translate((*effect).pos.x, (*effect).pos.y,
                             (*effect).pos.z,
                             MTXMODE_NEW as libc::c_int as u8_0);
            Matrix_Scale((*effect).unk_30, 1.0f32, (*effect).unk_30,
                         MTXMODE_APPLY as libc::c_int as u8_0);
            let fresh29 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_28: *mut Gfx = fresh29;
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
                Matrix_NewMtx((*globalCtx).state.gfxCtx,
                              b"../z_fishing.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              2504 as libc::c_int) as libc::c_uint;
            let fresh30 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_29: *mut Gfx = fresh30;
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
                gFishingRippleModelDL.as_mut_ptr() as libc::c_uint
        }
        effect = effect.offset(1);
        i += 1
    }
    effect = firstEffect.offset(30 as libc::c_int as isize);
    flag = 0 as libc::c_int as u8_0;
    i = 30 as libc::c_int as s16;
    while (i as libc::c_int) < 130 as libc::c_int {
        if (*effect).type_0 as libc::c_int ==
               FS_EFF_RAIN_SPLASH as libc::c_int {
            if flag as libc::c_int == 0 as libc::c_int {
                let fresh31 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_30: *mut Gfx = fresh31;
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
                    gFishingRainSplashMaterialDL.as_mut_ptr() as libc::c_uint;
                let fresh32 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_31: *mut Gfx = fresh32;
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
                        (((*gGameInfo).data[(13 as libc::c_int *
                                                 6 as libc::c_int *
                                                 16 as libc::c_int +
                                                 19 as libc::c_int) as usize]
                              as libc::c_int + 80 as libc::c_int) as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                flag = flag.wrapping_add(1)
            }
            if Rand_ZeroOne() < 0.5f32 {
                rotY = 0.0f32
            } else { rotY = 3.14159265358979323846f32 }
            Matrix_Translate((*effect).pos.x, (*effect).pos.y,
                             (*effect).pos.z,
                             MTXMODE_NEW as libc::c_int as u8_0);
            func_800D1FD4(&mut (*globalCtx).billboardMtxF);
            Matrix_RotateY(rotY, MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_Scale((*effect).unk_30, (*effect).unk_30, 1.0f32,
                         MTXMODE_APPLY as libc::c_int as u8_0);
            let fresh33 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_32: *mut Gfx = fresh33;
            (*_g_32).words.w0 =
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
            (*_g_32).words.w1 =
                Matrix_NewMtx((*globalCtx).state.gfxCtx,
                              b"../z_fishing.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              2541 as libc::c_int) as libc::c_uint;
            let fresh34 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_33: *mut Gfx = fresh34;
            (*_g_33).words.w0 =
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
            (*_g_33).words.w1 =
                gFishingRainSplashModelDL.as_mut_ptr() as libc::c_uint
        }
        effect = effect.offset(1);
        i += 1
    }
    effect = firstEffect;
    if (*effect).type_0 as libc::c_int == FS_EFF_OWNER_HAT as libc::c_int {
        Matrix_Translate((*effect).pos.x, (*effect).pos.y, (*effect).pos.z,
                         MTXMODE_NEW as libc::c_int as u8_0);
        Matrix_RotateY(sEffOwnerHatRot.y as libc::c_int as libc::c_float *
                           3.14159265358979323846f32 /
                           32768 as libc::c_int as libc::c_float,
                       MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_RotateX(sEffOwnerHatRot.x as libc::c_int as libc::c_float *
                           3.14159265358979323846f32 /
                           32768 as libc::c_int as libc::c_float,
                       MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_RotateZ(sEffOwnerHatRot.z as libc::c_int as libc::c_float *
                           3.14159265358979323846f32 /
                           32768 as libc::c_int as libc::c_float,
                       MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_Scale((*effect).unk_30, (*effect).unk_30, (*effect).unk_30,
                     MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_Translate(-1250.0f32, 0.0f32, 0.0f32,
                         MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_RotateX(3.14159265358979323846f32 /
                           2 as libc::c_int as libc::c_float,
                       MTXMODE_APPLY as libc::c_int as u8_0);
        let fresh35 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_34: *mut Gfx = fresh35;
        (*_g_34).words.w0 =
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
        (*_g_34).words.w1 =
            Matrix_NewMtx((*globalCtx).state.gfxCtx,
                          b"../z_fishing.c\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                          2560 as libc::c_int) as libc::c_uint;
        let fresh36 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_35: *mut Gfx = fresh36;
        (*_g_35).words.w0 =
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
        (*_g_35).words.w1 = gFishingOwnerHatDL.as_mut_ptr() as libc::c_uint
    }
    Matrix_Pop();
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_fishing.c\x00" as *const u8 as
                         *const libc::c_char, 2565 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Fishing_DrawStreamSplash(mut globalCtx:
                                                      *mut GlobalContext) {
    let mut pad: s32 = 0;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_fishing.c\x00" as *const u8 as *const libc::c_char,
                    2572 as libc::c_int);
    let fresh37 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g: *mut Gfx = fresh37;
    (*_g).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((0x9 as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g).words.w1 =
        Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                         (*globalCtx).gameplayFrames.wrapping_mul(1 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint),
                         (*globalCtx).gameplayFrames.wrapping_mul(8 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint),
                         32 as libc::c_int, 64 as libc::c_int,
                         1 as libc::c_int,
                         (*globalCtx).gameplayFrames.wrapping_mul(2 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint).wrapping_neg(),
                         0 as libc::c_int as u32_0, 16 as libc::c_int,
                         16 as libc::c_int) as libc::c_uint;
    let fresh38 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_0: *mut Gfx = fresh38;
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
            (50 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    Matrix_Translate(670.0f32, -24.0f32, -600.0f32,
                     MTXMODE_NEW as libc::c_int as u8_0);
    Matrix_Scale(0.02f32, 1.0f32, 0.02f32,
                 MTXMODE_APPLY as libc::c_int as u8_0);
    let fresh39 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_1: *mut Gfx = fresh39;
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
                      b"../z_fishing.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      2598 as libc::c_int) as libc::c_uint;
    let fresh40 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_2: *mut Gfx = fresh40;
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
        gSegments[((gFishingStreamSplashDL.as_mut_ptr() as u32_0) <<
                       4 as libc::c_int >> 28 as libc::c_int) as
                      usize].wrapping_add(gFishingStreamSplashDL.as_mut_ptr()
                                              as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as libc::c_uint;
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_fishing.c\x00" as *const u8 as
                         *const libc::c_char, 2613 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn func_80B6C2EC(mut vec: *mut Vec3f) -> s32 {
    if (*vec).x >= 110.0f32 && (*vec).x <= 150.0f32 && (*vec).z <= 1400.0f32
           && (*vec).z >= 1160.0f32 ||
           (*vec).x >= 110.0f32 && (*vec).x <= 210.0f32 &&
               (*vec).z <= 1200.0f32 && (*vec).z >= 1160.0f32 {
        if (*vec).y <= 42.0f32 { return 1 as libc::c_int }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Fishing_UpdateLine(mut globalCtx: *mut GlobalContext,
                                            mut basePos: *mut Vec3f,
                                            mut pos: *mut Vec3f,
                                            mut rot: *mut Vec3f,
                                            mut unk: *mut Vec3f) {
    let mut i: s16 = 0;
    let mut k: s16 = 0;
    let mut dx: f32_0 = 0.;
    let mut dy: f32_0 = 0.;
    let mut dz: f32_0 = 0.;
    let mut rx: f32_0 = 0.;
    let mut ry: f32_0 = 0.;
    let mut dist: f32_0 = 0.;
    let mut spD8: f32_0 = 0.;
    let mut temp_s2: s16 = 0;
    let mut pad: s32 = 0;
    let mut temp_f20: f32_0 = 0.;
    let mut posSrc: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
    let mut posStep: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut phi_f18: f32_0 = 0.;
    let mut spA4: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp98: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp94: f32_0 = 0.;
    let mut sp90: f32_0 = 0.;
    let mut sp8C: f32_0 = 0.;
    let mut sqDistXZ: f32_0 = 0.;
    let mut temp_f18: f32_0 = 0.;
    let mut phi_f12: f32_0 = 0.;
    let mut phi_f2: f32_0 = 0.;
    if D_80B7A6A4 as libc::c_int != 0 as libc::c_int {
        spA4 = *basePos;
        sp98 = *pos.offset((200 as libc::c_int - 1 as libc::c_int) as isize);
        sp94 = sp98.x - spA4.x;
        sp90 = sp98.y - spA4.y;
        sp8C = sp98.z - spA4.z;
        phi_f18 = sqrtf(sp94 * sp94 + sp90 * sp90 + sp8C * sp8C) * 0.97f32;
        if phi_f18 > 1000.0f32 { phi_f18 = 1000.0f32 }
        D_80B7E144 = 200.0f32 - phi_f18 * 200.0f32 * 0.001f32
    }
    temp_s2 = D_80B7E144 as s16;
    posSrc.z = 5.0f32;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < 200 as libc::c_int {
        if i as libc::c_int <= temp_s2 as libc::c_int {
            *pos.offset(i as isize) = *basePos
        } else if D_80B7A6A4 as libc::c_int != 0 as libc::c_int {
            temp_f20 =
                (i as libc::c_int - temp_s2 as libc::c_int) as f32_0 /
                    (200 as libc::c_int - temp_s2 as libc::c_int +
                         1 as libc::c_int) as f32_0;
            Math_ApproachF(&mut (*pos.offset(i as isize)).x,
                           sp94 * temp_f20 + spA4.x, 1.0f32, 20.0f32);
            Math_ApproachF(&mut (*pos.offset(i as isize)).y,
                           sp90 * temp_f20 + spA4.y, 1.0f32, 20.0f32);
            Math_ApproachF(&mut (*pos.offset(i as isize)).z,
                           sp8C * temp_f20 + spA4.z, 1.0f32, 20.0f32);
        }
        i += 1
    }
    i = (temp_s2 as libc::c_int + 1 as libc::c_int) as s16;
    k = 0 as libc::c_int as s16;
    while (i as libc::c_int) < 200 as libc::c_int {
        temp_f18 = 2.0f32 * D_80B7E148;
        dx =
            (*pos.offset(i as libc::c_int as isize)).x -
                (*pos.offset(i as libc::c_int as
                                 isize).offset(-(1 as libc::c_int as
                                                     isize))).x;
        spD8 = (*pos.offset(i as libc::c_int as isize)).y;
        sqDistXZ =
            (*pos.offset(i as libc::c_int as isize)).x *
                (*pos.offset(i as libc::c_int as isize)).x +
                (*pos.offset(i as libc::c_int as isize)).z *
                    (*pos.offset(i as libc::c_int as isize)).z;
        if sqDistXZ > 920.0f32 * 920.0f32 {
            phi_f12 =
                (sqrtf(sqDistXZ) - 920.0f32) * 0.11f32 +
                    (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface as
                        libc::c_int as libc::c_float
        } else {
            phi_f12 =
                (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface as
                    f32_0
        }
        if D_80B7E0B6 as libc::c_int == 2 as libc::c_int {
            if spD8 < phi_f12 {
                phi_f12 =
                    (sqrtf(sqDistXZ) - 920.0f32) * 0.147f32 +
                        (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface
                            as libc::c_int as libc::c_float;
                if spD8 > phi_f12 {
                    phi_f2 = (spD8 - phi_f12) * 0.05f32;
                    if phi_f2 > 0.29999998f32 { phi_f2 = 0.29999998f32 }
                    if i as libc::c_int >= 100 as libc::c_int {
                        phi_f2 *=
                            (i as libc::c_int - 100 as libc::c_int) as
                                libc::c_float * 0.02f32;
                        spD8 -= phi_f2
                    }
                }
            } else { spD8 -= temp_f18 }
        } else if i as libc::c_int > 200 as libc::c_int - 10 as libc::c_int {
            if spD8 > phi_f12 {
                phi_f2 = (spD8 - phi_f12) * 0.2f32;
                if phi_f2 > temp_f18 { phi_f2 = temp_f18 }
                spD8 -= phi_f2
            }
        } else if spD8 > phi_f12 { spD8 -= temp_f18 }
        if func_80B6C2EC(&mut *pos.offset(i as isize)) != 0 { spD8 = 42.0f32 }
        dy =
            spD8 -
                (*pos.offset(i as libc::c_int as
                                 isize).offset(-(1 as libc::c_int as
                                                     isize))).y;
        dz =
            (*pos.offset(i as libc::c_int as isize)).z -
                (*pos.offset(i as libc::c_int as
                                 isize).offset(-(1 as libc::c_int as
                                                     isize))).z;
        ry = Math_Atan2F(dz, dx);
        dist = sqrtf(dx * dx + dz * dz);
        rx = -Math_Atan2F(dist, dy);
        (*rot.offset(i as libc::c_int as
                         isize).offset(-(1 as libc::c_int as isize))).y = ry;
        (*rot.offset(i as libc::c_int as
                         isize).offset(-(1 as libc::c_int as isize))).x = rx;
        Matrix_RotateY(ry, MTXMODE_NEW as libc::c_int as u8_0);
        Matrix_RotateX(rx, MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_MultVec3f(&mut posSrc, &mut posStep);
        (*pos.offset(i as libc::c_int as isize)).x =
            (*pos.offset(i as libc::c_int as
                             isize).offset(-(1 as libc::c_int as isize))).x +
                posStep.x;
        (*pos.offset(i as libc::c_int as isize)).y =
            (*pos.offset(i as libc::c_int as
                             isize).offset(-(1 as libc::c_int as isize))).y +
                posStep.y;
        (*pos.offset(i as libc::c_int as isize)).z =
            (*pos.offset(i as libc::c_int as
                             isize).offset(-(1 as libc::c_int as isize))).z +
                posStep.z;
        i += 1;
        k += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn Fishing_UpdateLinePos(mut pos: *mut Vec3f) {
    let mut i: s16 = 0;
    let mut dx: f32_0 = 0.;
    let mut dy: f32_0 = 0.;
    let mut dz: f32_0 = 0.;
    let mut rx: f32_0 = 0.;
    let mut ry: f32_0 = 0.;
    let mut dist: f32_0 = 0.;
    let mut posSrc: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
    let mut posStep: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut min: s16 = D_80B7E144 as s16;
    posSrc.z = 5.0f32;
    i = (200 as libc::c_int - 2 as libc::c_int) as s16;
    while i as libc::c_int > min as libc::c_int {
        dx =
            (*pos.offset(i as libc::c_int as isize)).x -
                (*pos.offset(i as libc::c_int as
                                 isize).offset(1 as libc::c_int as isize)).x;
        dy =
            (*pos.offset(i as libc::c_int as isize)).y -
                (*pos.offset(i as libc::c_int as
                                 isize).offset(1 as libc::c_int as isize)).y;
        dz =
            (*pos.offset(i as libc::c_int as isize)).z -
                (*pos.offset(i as libc::c_int as
                                 isize).offset(1 as libc::c_int as isize)).z;
        ry = Math_Atan2F(dz, dx);
        dist = sqrtf(dx * dx + dz * dz);
        rx = -Math_Atan2F(dist, dy);
        Matrix_RotateY(ry, MTXMODE_NEW as libc::c_int as u8_0);
        Matrix_RotateX(rx, MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_MultVec3f(&mut posSrc, &mut posStep);
        (*pos.offset(i as libc::c_int as isize)).x =
            (*pos.offset(i as libc::c_int as
                             isize).offset(1 as libc::c_int as isize)).x +
                posStep.x;
        (*pos.offset(i as libc::c_int as isize)).y =
            (*pos.offset(i as libc::c_int as
                             isize).offset(1 as libc::c_int as isize)).y +
                posStep.y;
        (*pos.offset(i as libc::c_int as isize)).z =
            (*pos.offset(i as libc::c_int as
                             isize).offset(1 as libc::c_int as isize)).z +
                posStep.z;
        i -= 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn Fishing_DrawLureHook(mut globalCtx:
                                                  *mut GlobalContext,
                                              mut pos: *mut Vec3f,
                                              mut refPos: *mut Vec3f,
                                              mut hookIndex: u8_0) {
    let mut dx: f32_0 = 0.;
    let mut dy: f32_0 = 0.;
    let mut dz: f32_0 = 0.;
    let mut rx: f32_0 = 0.;
    let mut ry: f32_0 = 0.;
    let mut dist: f32_0 = 0.;
    let mut offsetY: f32_0 = 0.;
    let mut posSrc: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 1.0f32,}; init };
    let mut posStep: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_fishing.c\x00" as *const u8 as *const libc::c_char,
                    2963 as libc::c_int);
    Matrix_Push();
    if D_80B7A694 as libc::c_int == 3 as libc::c_int &&
           ((*pos).y >
                (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface as
                    libc::c_int as libc::c_float ||
                D_80B7A68C as libc::c_int != 0 as libc::c_int &&
                    hookIndex as libc::c_int != 0) {
        offsetY = 0.0f32
    } else if (*pos).y <
                  (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface as
                      libc::c_int as libc::c_float {
        offsetY = -1.0f32
    } else { offsetY = -3.0f32 }
    dx = (*refPos).x - (*pos).x;
    dy = (*refPos).y - (*pos).y + offsetY;
    dz = (*refPos).z - (*pos).z;
    ry = Math_Atan2F(dz, dx);
    dist = sqrtf(dx * dx + dz * dz);
    rx = -Math_Atan2F(dist, dy);
    Matrix_RotateY(ry, MTXMODE_NEW as libc::c_int as u8_0);
    Matrix_RotateX(rx, MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_MultVec3f(&mut posSrc, &mut posStep);
    (*refPos).x = (*pos).x + posStep.x;
    (*refPos).y = (*pos).y + posStep.y;
    (*refPos).z = (*pos).z + posStep.z;
    Matrix_Translate((*pos).x, (*pos).y, (*pos).z,
                     MTXMODE_NEW as libc::c_int as u8_0);
    if (*player).actor.speedXZ == 0.0f32 && D_80B7E138 == 0.0f32 {
        Math_ApproachF(&mut *sLureHookRotY.as_mut_ptr().offset(hookIndex as
                                                                   isize), ry,
                       0.1f32, 0.3f32);
    } else { sLureHookRotY[hookIndex as usize] = ry }
    Matrix_RotateY(sLureHookRotY[hookIndex as usize],
                   MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_RotateX(rx, MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_Scale(0.0039999997f32, 0.0039999997f32, 0.005f32,
                 MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_RotateY(3.14159265358979323846f32,
                   MTXMODE_APPLY as libc::c_int as u8_0);
    let fresh41 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g: *mut Gfx = fresh41;
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
    (*_g).words.w1 =
        Matrix_NewMtx((*globalCtx).state.gfxCtx,
                      b"../z_fishing.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      3029 as libc::c_int) as libc::c_uint;
    let fresh42 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_0: *mut Gfx = fresh42;
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
    (*_g_0).words.w1 = gFishingLureHookDL.as_mut_ptr() as libc::c_uint;
    Matrix_RotateZ(3.14159265358979323846f32 /
                       2 as libc::c_int as libc::c_float,
                   MTXMODE_APPLY as libc::c_int as u8_0);
    let fresh43 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
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
                      b"../z_fishing.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      3034 as libc::c_int) as libc::c_uint;
    let fresh44 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
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
    (*_g_2).words.w1 = gFishingLureHookDL.as_mut_ptr() as libc::c_uint;
    if hookIndex as libc::c_int == 1 as libc::c_int &&
           D_80B7A68C as libc::c_int != 0 as libc::c_int {
        Matrix_Scale(2.0f32, 2.0f32, 2.0f32,
                     MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_Translate(250.0f32, 0.0f32, -1400.0f32,
                         MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_Push();
        if D_80B7A690 as libc::c_int != 0 as libc::c_int {
            let mut effect: *mut FishingEffect =
                (*globalCtx).specialEffects as *mut FishingEffect;
            let mut mf: MtxF = MtxF{mf: [[0.; 4]; 4],};
            Matrix_MultVec3f(&mut sZeroVec, &mut (*effect).pos);
            Matrix_Get(&mut mf);
            Matrix_MtxFToYXZRotS(&mut mf, &mut sEffOwnerHatRot,
                                 0 as libc::c_int);
            D_80B7A690 = 0 as libc::c_int as u8_0;
            D_80B7A68C = 0 as libc::c_int as u8_0;
            (*effect).type_0 = FS_EFF_OWNER_HAT as libc::c_int as u8_0;
            (*effect).unk_2C = 0 as libc::c_int as s16;
            (*effect).vel = sZeroVec;
            (*effect).accel = sZeroVec
        }
        Matrix_Pop();
        Matrix_Translate(-1250.0f32, 0.0f32, 0.0f32,
                         MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_RotateX(3.14159265358979323846f32 /
                           2 as libc::c_int as libc::c_float,
                       MTXMODE_APPLY as libc::c_int as u8_0);
        let fresh45 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_3: *mut Gfx = fresh45;
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
                          b"../z_fishing.c\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                          3085 as libc::c_int) as libc::c_uint;
        let fresh46 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_4: *mut Gfx = fresh46;
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
        (*_g_4).words.w1 = gFishingOwnerHatDL.as_mut_ptr() as libc::c_uint
    }
    Matrix_Pop();
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_fishing.c\x00" as *const u8 as
                         *const libc::c_char, 3098 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Fishing_UpdateSinkingLure(mut globalCtx:
                                                       *mut GlobalContext) {
    let mut i: s16 = 0;
    let mut dx: f32_0 = 0.;
    let mut dy: f32_0 = 0.;
    let mut dz: f32_0 = 0.;
    let mut rx: f32_0 = 0.;
    let mut ry: f32_0 = 0.;
    let mut dist: f32_0 = 0.;
    let mut offsetY: f32_0 = 0.;
    let mut posSrc: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
    let mut posStep: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp94: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp88: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut offsetX: f32_0 = 0.;
    let mut offsetZ: f32_0 = 0.;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    posSrc.z = 0.85f32;
    sSinkingLurePos[0 as libc::c_int as usize] = sLurePos;
    if D_80B7A6D4 as libc::c_int != 0 as libc::c_int {
        offsetY = -1.0f32
    } else if sLurePos.y <
                  (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface as
                      libc::c_int as libc::c_float {
        offsetY = 0.5f32
    } else { offsetY = -5.0f32 }
    if D_80B7A694 as libc::c_int == 5 as libc::c_int {
        Matrix_RotateY((*player).actor.shape.rot.y as libc::c_int as
                           libc::c_float *
                           (3.14159265358979323846f32 /
                                32768 as libc::c_int as libc::c_float),
                       MTXMODE_NEW as libc::c_int as u8_0);
        sp94.x = 5.0f32;
        sp94.y = 0.0f32;
        sp94.z = 3.0f32;
        Matrix_MultVec3f(&mut sp94, &mut sp88);
    }
    i = 1 as libc::c_int as s16;
    while (i as libc::c_int) < 20 as libc::c_int {
        let mut pos: *mut Vec3f = sSinkingLurePos.as_mut_ptr();
        if (i as libc::c_int) < 10 as libc::c_int &&
               D_80B7A694 as libc::c_int == 5 as libc::c_int {
            offsetX =
                (10 as libc::c_int - i as libc::c_int) as libc::c_float *
                    sp88.x * 0.1f32;
            offsetZ =
                (10 as libc::c_int - i as libc::c_int) as libc::c_float *
                    sp88.z * 0.1f32
        } else { offsetZ = 0.0f32; offsetX = offsetZ }
        dx =
            (*pos.offset(i as libc::c_int as isize)).x -
                (*pos.offset(i as libc::c_int as
                                 isize).offset(-(1 as libc::c_int as
                                                     isize))).x + offsetX;
        dy =
            (*pos.offset(i as libc::c_int as isize)).y -
                (*pos.offset(i as libc::c_int as
                                 isize).offset(-(1 as libc::c_int as
                                                     isize))).y + offsetY;
        dz =
            (*pos.offset(i as libc::c_int as isize)).z -
                (*pos.offset(i as libc::c_int as
                                 isize).offset(-(1 as libc::c_int as
                                                     isize))).z + offsetZ;
        ry = Math_Atan2F(dz, dx);
        dist = sqrtf(dx * dx + dz * dz);
        rx = -Math_Atan2F(dist, dy);
        Matrix_RotateY(ry, MTXMODE_NEW as libc::c_int as u8_0);
        Matrix_RotateX(rx, MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_MultVec3f(&mut posSrc, &mut posStep);
        (*pos.offset(i as libc::c_int as isize)).x =
            (*pos.offset(i as libc::c_int as
                             isize).offset(-(1 as libc::c_int as isize))).x +
                posStep.x;
        (*pos.offset(i as libc::c_int as isize)).y =
            (*pos.offset(i as libc::c_int as
                             isize).offset(-(1 as libc::c_int as isize))).y +
                posStep.y;
        (*pos.offset(i as libc::c_int as isize)).z =
            (*pos.offset(i as libc::c_int as
                             isize).offset(-(1 as libc::c_int as isize))).z +
                posStep.z;
        i += 1
    };
}
static mut sSinkingLureSizes: [f32_0; 20] =
    [1.0f32, 1.5f32, 1.8f32, 2.0f32, 1.8f32, 1.6f32, 1.4f32, 1.2f32, 1.0f32,
     1.0f32, 0.9f32, 0.85f32, 0.8f32, 0.7f32, 0.8f32, 1.0f32, 1.2f32, 1.1f32,
     1.0f32, 0.8f32];
#[no_mangle]
pub unsafe extern "C" fn Fishing_DrawSinkingLure(mut globalCtx:
                                                     *mut GlobalContext) {
    let mut i: s16 = 0;
    let mut scale: f32_0 = 0.;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_fishing.c\x00" as *const u8 as *const libc::c_char,
                    3209 as libc::c_int);
    Fishing_UpdateSinkingLure(globalCtx);
    if sLurePos.y <
           (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface as
               libc::c_int as libc::c_float {
        func_80093D18((*globalCtx).state.gfxCtx);
        let fresh47 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g: *mut Gfx = fresh47;
        (*_g).words.w0 =
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
        (*_g).words.w1 =
            gFishingSinkingLureSegmentMaterialDL.as_mut_ptr() as libc::c_uint;
        i = (20 as libc::c_int - 1 as libc::c_int) as s16;
        while i as libc::c_int >= 0 as libc::c_int {
            if (i as libc::c_int + D_80B7FEA0 as libc::c_int) <
                   20 as libc::c_int {
                Matrix_Translate(sSinkingLurePos[i as usize].x,
                                 sSinkingLurePos[i as usize].y,
                                 sSinkingLurePos[i as usize].z,
                                 MTXMODE_NEW as libc::c_int as u8_0);
                scale =
                    sSinkingLureSizes[(i as libc::c_int +
                                           D_80B7FEA0 as libc::c_int) as
                                          usize] * 0.04f32;
                Matrix_Scale(scale, scale, scale,
                             MTXMODE_APPLY as libc::c_int as u8_0);
                func_800D1FD4(&mut (*globalCtx).billboardMtxF);
                let fresh48 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_0: *mut Gfx = fresh48;
                (*_g_0).words.w0 =
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
                (*_g_0).words.w1 =
                    Matrix_NewMtx((*globalCtx).state.gfxCtx,
                                  b"../z_fishing.c\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char, 3239 as libc::c_int)
                        as libc::c_uint;
                let fresh49 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_1: *mut Gfx = fresh49;
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
                    gFishingSinkingLureSegmentModelDL.as_mut_ptr() as
                        libc::c_uint
            }
            i -= 1
        }
    } else {
        func_80093D84((*globalCtx).state.gfxCtx);
        let fresh50 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_2: *mut Gfx = fresh50;
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
            gFishingSinkingLureSegmentMaterialDL.as_mut_ptr() as libc::c_uint;
        i = (20 as libc::c_int - 1 as libc::c_int) as s16;
        while i as libc::c_int >= 0 as libc::c_int {
            if (i as libc::c_int + D_80B7FEA0 as libc::c_int) <
                   20 as libc::c_int {
                Matrix_Translate(sSinkingLurePos[i as usize].x,
                                 sSinkingLurePos[i as usize].y,
                                 sSinkingLurePos[i as usize].z,
                                 MTXMODE_NEW as libc::c_int as u8_0);
                scale =
                    sSinkingLureSizes[(i as libc::c_int +
                                           D_80B7FEA0 as libc::c_int) as
                                          usize] * 0.04f32;
                Matrix_Scale(scale, scale, scale,
                             MTXMODE_APPLY as libc::c_int as u8_0);
                func_800D1FD4(&mut (*globalCtx).billboardMtxF);
                let fresh51 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_3: *mut Gfx = fresh51;
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
                               0 as libc::c_int) ^ 0x1 as libc::c_int) as
                             u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                (*_g_3).words.w1 =
                    Matrix_NewMtx((*globalCtx).state.gfxCtx,
                                  b"../z_fishing.c\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char, 3265 as libc::c_int)
                        as libc::c_uint;
                let fresh52 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_4: *mut Gfx = fresh52;
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
                    gFishingSinkingLureSegmentModelDL.as_mut_ptr() as
                        libc::c_uint
            }
            i -= 1
        }
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_fishing.c\x00" as *const u8 as
                         *const libc::c_char, 3271 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Fishing_DrawLureAndLine(mut globalCtx:
                                                     *mut GlobalContext,
                                                 mut linePos: *mut Vec3f,
                                                 mut lineRot: *mut Vec3f) {
    let mut posSrc: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut posStep: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut hookPos: [Vec3f; 2] = [Vec3f{x: 0., y: 0., z: 0.,}; 2];
    let mut i: s16 = 0;
    let mut spB4: s16 = D_80B7E144 as s16;
    let mut pad: s32 = 0;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_fishing.c\x00" as *const u8 as *const libc::c_char,
                    3287 as libc::c_int);
    func_80093D18((*globalCtx).state.gfxCtx);
    Matrix_Push();
    if D_80B7A6D4 as libc::c_int != 0 as libc::c_int {
        let mut posTemp: Vec3f = sLurePos;
        sLurePos = sSinkingLureBasePos;
        Fishing_DrawSinkingLure(globalCtx);
        sLurePos = posTemp
    }
    if D_80B7A694 as libc::c_int == 4 as libc::c_int ||
           D_80B7A694 as libc::c_int == 5 as libc::c_int {
        sLurePos = (*sFishingHookedFish).fishMouthPos;
        if D_80B7A694 as libc::c_int == 5 as libc::c_int &&
               D_80B7E0B6 as libc::c_int == 2 as libc::c_int {
            Matrix_RotateY((*player).actor.shape.rot.y as libc::c_int as
                               libc::c_float *
                               (3.14159265358979323846f32 /
                                    32768 as libc::c_int as libc::c_float),
                           MTXMODE_NEW as libc::c_int as u8_0);
            posSrc.x = 2.0f32;
            posSrc.y = 0.0f32;
            posSrc.z = 0.0f32;
            Matrix_MultVec3f(&mut posSrc, &mut posStep);
            sLurePos.x += posStep.x;
            sLurePos.z += posStep.z
        }
    } else if D_80B7A694 as libc::c_int == 0 as libc::c_int {
        sLurePos =
            sReelLinePos[(200 as libc::c_int - 1 as libc::c_int) as usize];
        sLureRot.x =
            sReelLineRot[(200 as libc::c_int - 2 as libc::c_int) as usize].x +
                3.14159265358979323846f32;
        if (*player).actor.speedXZ == 0.0f32 &&
               D_80B7E0B0 as libc::c_int == 0 as libc::c_int {
            Math_ApproachF(&mut sLureRot.y,
                           sReelLineRot[(200 as libc::c_int -
                                             2 as libc::c_int) as usize].y,
                           0.1f32, 0.2f32);
        } else {
            sLureRot.y =
                sReelLineRot[(200 as libc::c_int - 2 as libc::c_int) as
                                 usize].y
        }
    }
    if D_80B7E0B6 as libc::c_int != 2 as libc::c_int {
        Matrix_Translate(sLurePos.x, sLurePos.y, sLurePos.z,
                         MTXMODE_NEW as libc::c_int as u8_0);
        Matrix_RotateY(sLureRot.y + D_80B7E104,
                       MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_RotateX(sLureRot.x, MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_Scale(0.0039999997f32, 0.0039999997f32, 0.0039999997f32,
                     MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_Translate(0.0f32, 0.0f32, D_80B7E108,
                         MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_RotateZ(3.14159265358979323846f32 /
                           2 as libc::c_int as libc::c_float,
                       MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_RotateY(3.14159265358979323846f32 /
                           2 as libc::c_int as libc::c_float,
                       MTXMODE_APPLY as libc::c_int as u8_0);
        func_80093D18((*globalCtx).state.gfxCtx);
        let fresh53 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g: *mut Gfx = fresh53;
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
                          b"../z_fishing.c\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                          3369 as libc::c_int) as libc::c_uint;
        let fresh54 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_0: *mut Gfx = fresh54;
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
        (*_g_0).words.w1 = gFishingLureFloatDL.as_mut_ptr() as libc::c_uint;
        posSrc.x = -850.0f32;
        posSrc.y = 0.0f32;
        posSrc.z = 0.0f32;
        Matrix_MultVec3f(&mut posSrc, &mut D_80B7E0C8);
        posSrc.x = 500.0f32;
        posSrc.z = -300.0f32;
        Matrix_MultVec3f(&mut posSrc,
                         &mut *hookPos.as_mut_ptr().offset(0 as libc::c_int as
                                                               isize));
        Fishing_DrawLureHook(globalCtx,
                             &mut *hookPos.as_mut_ptr().offset(0 as
                                                                   libc::c_int
                                                                   as isize),
                             &mut *sLureHookRefPos.as_mut_ptr().offset(0 as
                                                                           libc::c_int
                                                                           as
                                                                           isize),
                             0 as libc::c_int as u8_0);
        posSrc.x = 2100.0f32;
        posSrc.z = -50.0f32;
        Matrix_MultVec3f(&mut posSrc,
                         &mut *hookPos.as_mut_ptr().offset(1 as libc::c_int as
                                                               isize));
        Fishing_DrawLureHook(globalCtx,
                             &mut *hookPos.as_mut_ptr().offset(1 as
                                                                   libc::c_int
                                                                   as isize),
                             &mut *sLureHookRefPos.as_mut_ptr().offset(1 as
                                                                           libc::c_int
                                                                           as
                                                                           isize),
                             1 as libc::c_int as u8_0);
    }
    (*__gfxCtx).polyXlu.p =
        Gfx_CallSetupDL((*__gfxCtx).polyXlu.p, 0x14 as libc::c_int as u32_0);
    let fresh55 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_1: *mut Gfx = fresh55;
    (*_g_1).words.w0 =
        (0xfc as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (((31 as libc::c_int as u32_0 &
                   (((0x1 as libc::c_int) << 4 as libc::c_int) -
                        1 as libc::c_int) as libc::c_uint) <<
                  20 as libc::c_int |
                  (31 as libc::c_int as u32_0 &
                       (((0x1 as libc::c_int) << 5 as libc::c_int) -
                            1 as libc::c_int) as libc::c_uint) <<
                      15 as libc::c_int |
                  (7 as libc::c_int as u32_0 &
                       (((0x1 as libc::c_int) << 3 as libc::c_int) -
                            1 as libc::c_int) as libc::c_uint) <<
                      12 as libc::c_int |
                  (7 as libc::c_int as u32_0 &
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
    (*_g_1).words.w1 =
        (31 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 28 as libc::c_int |
            (3 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 15 as libc::c_int
            |
            (7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            (3 as libc::c_int as u32_0 &
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
                 (3 as libc::c_int as u32_0 &
                      (((0x1 as libc::c_int) << 3 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     6 as libc::c_int |
                 (7 as libc::c_int as u32_0 &
                      (((0x1 as libc::c_int) << 3 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     3 as libc::c_int |
                 (3 as libc::c_int as u32_0 &
                      (((0x1 as libc::c_int) << 3 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     0 as libc::c_int);
    let fresh56 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_2: *mut Gfx = fresh56;
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
            (55 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    if D_80B7A694 as libc::c_int == 4 as libc::c_int &&
           (D_80B7E124 as libc::c_int != 0 as libc::c_int ||
                D_80B7E0B6 as libc::c_int != 2 as libc::c_int) {
        let mut rx: f32_0 = 0.;
        let mut ry: f32_0 = 0.;
        let mut dist: f32_0 = 0.;
        let mut dx: f32_0 = 0.;
        let mut dy: f32_0 = 0.;
        let mut dz: f32_0 = 0.;
        dx = sLurePos.x - sRodTipPos.x;
        dy = sLurePos.y - sRodTipPos.y;
        dz = sLurePos.z - sRodTipPos.z;
        ry = Math_FAtan2F(dx, dz);
        dist = sqrtf(dx * dx + dz * dz);
        rx = -Math_FAtan2F(dy, dist);
        dist = sqrtf(dx * dx + dy * dy + dz * dz) * 0.001f32;
        Matrix_Translate(sRodTipPos.x, sRodTipPos.y, sRodTipPos.z,
                         MTXMODE_NEW as libc::c_int as u8_0);
        Matrix_RotateY(ry, MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_RotateX(rx, MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_Scale(D_80B7E14C, 1.0f32, dist,
                     MTXMODE_APPLY as libc::c_int as u8_0);
        let fresh57 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_3: *mut Gfx = fresh57;
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
                          b"../z_fishing.c\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                          3444 as libc::c_int) as libc::c_uint;
        let fresh58 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_4: *mut Gfx = fresh58;
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
        (*_g_4).words.w1 = gFishingLineModelDL.as_mut_ptr() as libc::c_uint
    } else {
        i = spB4;
        while (i as libc::c_int) < 200 as libc::c_int - 1 as libc::c_int {
            if i as libc::c_int == 200 as libc::c_int - 3 as libc::c_int &&
                   D_80B7E0B6 as libc::c_int == 0 as libc::c_int &&
                   D_80B7A694 as libc::c_int == 3 as libc::c_int {
                let mut rx_0: f32_0 = 0.;
                let mut ry_0: f32_0 = 0.;
                let mut dist_0: f32_0 = 0.;
                let mut dx_0: f32_0 = 0.;
                let mut dy_0: f32_0 = 0.;
                let mut dz_0: f32_0 = 0.;
                dx_0 =
                    D_80B7E0C8.x -
                        (*linePos.offset(i as libc::c_int as isize)).x;
                dy_0 =
                    D_80B7E0C8.y -
                        (*linePos.offset(i as libc::c_int as isize)).y;
                dz_0 =
                    D_80B7E0C8.z -
                        (*linePos.offset(i as libc::c_int as isize)).z;
                ry_0 = Math_FAtan2F(dx_0, dz_0);
                dist_0 = sqrtf(dx_0 * dx_0 + dz_0 * dz_0);
                rx_0 = -Math_FAtan2F(dy_0, dist_0);
                dist_0 =
                    sqrtf(dx_0 * dx_0 + dy_0 * dy_0 + dz_0 * dz_0) * 0.001f32;
                Matrix_Translate((*linePos.offset(i as libc::c_int as
                                                      isize)).x,
                                 (*linePos.offset(i as libc::c_int as
                                                      isize)).y,
                                 (*linePos.offset(i as libc::c_int as
                                                      isize)).z,
                                 MTXMODE_NEW as libc::c_int as u8_0);
                Matrix_RotateY(ry_0, MTXMODE_APPLY as libc::c_int as u8_0);
                Matrix_RotateX(rx_0, MTXMODE_APPLY as libc::c_int as u8_0);
                Matrix_Scale(D_80B7E14C, 1.0f32, dist_0,
                             MTXMODE_APPLY as libc::c_int as u8_0);
                let fresh59 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_5: *mut Gfx = fresh59;
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
                               0 as libc::c_int) ^ 0x1 as libc::c_int) as
                             u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                (*_g_5).words.w1 =
                    Matrix_NewMtx((*globalCtx).state.gfxCtx,
                                  b"../z_fishing.c\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char, 3475 as libc::c_int)
                        as libc::c_uint;
                let fresh60 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_6: *mut Gfx = fresh60;
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
                    gFishingLineModelDL.as_mut_ptr() as libc::c_uint;
                break ;
            } else {
                Matrix_Translate((*linePos.offset(i as libc::c_int as
                                                      isize)).x,
                                 (*linePos.offset(i as libc::c_int as
                                                      isize)).y,
                                 (*linePos.offset(i as libc::c_int as
                                                      isize)).z,
                                 MTXMODE_NEW as libc::c_int as u8_0);
                Matrix_RotateY((*lineRot.offset(i as libc::c_int as isize)).y,
                               MTXMODE_APPLY as libc::c_int as u8_0);
                Matrix_RotateX((*lineRot.offset(i as libc::c_int as isize)).x,
                               MTXMODE_APPLY as libc::c_int as u8_0);
                Matrix_Scale(D_80B7E14C, 1.0f32, 0.005f32,
                             MTXMODE_APPLY as libc::c_int as u8_0);
                let fresh61 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_7: *mut Gfx = fresh61;
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
                                  b"../z_fishing.c\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char, 3492 as libc::c_int)
                        as libc::c_uint;
                let fresh62 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_8: *mut Gfx = fresh62;
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
                    gFishingLineModelDL.as_mut_ptr() as libc::c_uint;
                i += 1
            }
        }
    }
    Matrix_Pop();
    func_80093D84((*globalCtx).state.gfxCtx);
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_fishing.c\x00" as *const u8 as
                         *const libc::c_char, 3500 as libc::c_int);
}
static mut sRodScales: [f32_0; 22] =
    [1.0f32, 1.0f32, 1.0f32, 0.9625f32, 0.925f32, 0.8875f32, 0.85f32,
     0.8125f32, 0.775f32, 0.73749995f32, 0.7f32, 0.6625f32, 0.625f32,
     0.5875f32, 0.54999995f32, 0.5125f32, 0.47499996f32, 0.4375f32,
     0.39999998f32, 0.36249995f32, 0.325f32, 0.28749996f32];
static mut sRodBendRatios: [f32_0; 22] =
    [0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.06f32, 0.12f32, 0.18f32,
     0.24f32, 0.30f32, 0.36f32, 0.42f32, 0.48f32, 0.54f32, 0.60f32, 0.60f32,
     0.5142f32, 0.4285f32, 0.3428f32, 0.2571f32, 0.1714f32, 0.0857f32];
static mut sRodTipOffset: Vec3f =
    { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
#[no_mangle]
pub unsafe extern "C" fn Fishing_DrawRod(mut globalCtx: *mut GlobalContext) {
    let mut i: s16 = 0;
    let mut spC8: f32_0 = 0.;
    let mut spC4: f32_0 = 0.;
    let mut spC0: f32_0 = 0.;
    let mut input: *mut Input =
        &mut *(*globalCtx).state.input.as_mut_ptr().offset(0 as libc::c_int as
                                                               isize) as
            *mut Input;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut pad: s32 = 0;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_fishing.c\x00" as *const u8 as *const libc::c_char,
                    3600 as libc::c_int);
    if D_80B7FDA8 as libc::c_int != 0 as libc::c_int {
        D_80B7FDA8 = D_80B7FDA8.wrapping_sub(1);
        Math_ApproachF(&mut D_80B7A6C0, 35.0f32, 1.0f32, 100.0f32);
        Math_ApproachF(&mut D_80B7A6BC, -0.8f32, 1.0f32, 0.4f32);
        Math_ApproachS(&mut (*player).actor.shape.rot.x,
                       -(4000 as libc::c_int) as s16, 2 as libc::c_int as s16,
                       15000 as libc::c_int as s16);
    } else {
        let mut target: s16 = 0 as libc::c_int as s16;
        if D_80B7A694 as libc::c_int == 4 as libc::c_int &&
               D_80B7E124 as libc::c_int != 0 as libc::c_int {
            target =
                (Math_SinS((D_80B7E0AE as libc::c_int * 25600 as libc::c_int)
                               as s16) * 1500.0f32) as s16
        } else {
            Math_ApproachZeroF(&mut D_80B7A6C0, 0.1f32, 10.0f32);
            Math_ApproachZeroF(&mut D_80B7A6BC, 1.0f32, 0.05f32);
        }
        Math_ApproachS(&mut (*player).actor.shape.rot.x, target,
                       5 as libc::c_int as s16, 1000 as libc::c_int as s16);
    }
    if D_80B7A694 as libc::c_int == 3 as libc::c_int ||
           D_80B7A694 as libc::c_int == 4 as libc::c_int {
        if (*input).rel.stick_x as libc::c_int == 0 as libc::c_int &&
               D_80B7A6C4 as libc::c_int != 0 as libc::c_int {
            D_80B7A6B0 = 0.0f32
        }
        if (*input).rel.stick_y as libc::c_int == 0 as libc::c_int &&
               D_80B7A6C8 as libc::c_int != 0 as libc::c_int {
            D_80B7A6B4 = 0.0f32
        }
        spC8 = (*player).unk_85C;
        Math_SmoothStepToF(&mut (*player).unk_85C,
                           (*input).rel.stick_y as libc::c_int as
                               libc::c_float * 0.02f32, 0.3f32, 5.0f32,
                           0.0f32);
        spC8 = (*player).unk_85C - spC8;
        spC4 = (*player).unk_858;
        Math_SmoothStepToF(&mut (*player).unk_858,
                           (*input).rel.stick_x as libc::c_int as
                               libc::c_float * 0.02f32, 0.3f32, 5.0f32,
                           0.0f32);
        spC4 = (*player).unk_858 - spC4;
        if (*player).unk_858 > 1.0f32 { (*player).unk_858 = 1.0f32 }
        if (*player).unk_85C > 1.0f32 { (*player).unk_85C = 1.0f32 }
        if (*player).unk_858 < -1.0f32 { (*player).unk_858 = -1.0f32 }
        if (*player).unk_85C < -1.0f32 { (*player).unk_85C = -1.0f32 }
        Math_ApproachF(&mut D_80B7A6A8, spC4 * 70.0f32 * -0.01f32, 1.0f32,
                       D_80B7A6B0);
        Math_ApproachF(&mut D_80B7A6B0, 1.0f32, 1.0f32, 0.1f32);
        Math_ApproachF(&mut D_80B7A6AC, spC8 * 70.0f32 * 0.01f32, 1.0f32,
                       D_80B7A6B4);
        Math_ApproachF(&mut D_80B7A6B4, 1.0f32, 1.0f32, 0.1f32);
        Math_ApproachZeroF(&mut D_80B7A6B8, 1.0f32, 0.05f32);
    } else {
        Math_ApproachZeroF(&mut (*player).unk_85C, 1.0f32, 0.1f32);
        Math_ApproachZeroF(&mut (*player).unk_858, 1.0f32, 0.1f32);
        Math_ApproachF(&mut D_80B7A6AC,
                       Math_SinS((D_80B7E0AE as libc::c_int *
                                      3000 as libc::c_int) as s16) * 0.025f32
                           + -0.03f32, 1.0f32, 0.05f32);
        Math_ApproachZeroF(&mut D_80B7A6A8, 1.0f32, 0.05f32);
        if D_80B7E0B4 as libc::c_int >= 19 as libc::c_int &&
               D_80B7E0B4 as libc::c_int <= 24 as libc::c_int {
            Math_ApproachF(&mut D_80B7A6B8, 0.8f32, 1.0f32, 0.2f32);
        } else { Math_ApproachF(&mut D_80B7A6B8, 0.0f32, 1.0f32, 0.4f32); }
    }
    func_80093D18((*globalCtx).state.gfxCtx);
    let fresh63 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g: *mut Gfx = fresh63;
    (*_g).words.w0 =
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
    (*_g).words.w1 = gFishingRodMaterialDL.as_mut_ptr() as libc::c_uint;
    let fresh64 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_0: *mut Gfx = fresh64;
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
            (155 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    Matrix_Mult(&mut (*player).mf_9E0, MTXMODE_NEW as libc::c_int as u8_0);
    if sLinkAge as libc::c_int != 1 as libc::c_int {
        Matrix_Translate(0.0f32, 400.0f32, 0.0f32,
                         MTXMODE_APPLY as libc::c_int as u8_0);
    } else {
        Matrix_Translate(0.0f32, 230.0f32, 0.0f32,
                         MTXMODE_APPLY as libc::c_int as u8_0);
    }
    if D_80B7A694 as libc::c_int == 5 as libc::c_int {
        Matrix_RotateY(0.56f32 * 3.14159265358979323846f32,
                       MTXMODE_APPLY as libc::c_int as u8_0);
    } else {
        Matrix_RotateY(0.41f32 * 3.14159265358979323846f32,
                       MTXMODE_APPLY as libc::c_int as u8_0);
    }
    Matrix_RotateX(-3.14159265358979323846f32 / 5.0000003f32,
                   MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_RotateZ((*player).unk_858 * 0.5f32 +
                       3.0f32 * 3.14159265358979323846f32 / 20.0f32,
                   MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_RotateX((D_80B7A6C0 + 20.0f32) * 0.01f32 *
                       3.14159265358979323846f32,
                   MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_Scale(0.70000005f32, 0.70000005f32, 0.70000005f32,
                 MTXMODE_APPLY as libc::c_int as u8_0);
    spC0 =
        D_80B7A6BC * (((*player).unk_85C - 1.0f32) * -0.25f32 + 0.5f32) +
            (D_80B7A6AC + D_80B7A6B8);
    Matrix_Translate(0.0f32, 0.0f32, -1300.0f32,
                     MTXMODE_APPLY as libc::c_int as u8_0);
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < 22 as libc::c_int {
        Matrix_RotateY(sRodBendRatios[i as usize] * D_80B7A6A8 * 0.5f32,
                       MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_RotateX(sRodBendRatios[i as usize] * spC0 * 0.5f32,
                       MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_Push();
        Matrix_Scale(sRodScales[i as usize], sRodScales[i as usize], 0.52f32,
                     MTXMODE_APPLY as libc::c_int as u8_0);
        let fresh65 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_1: *mut Gfx = fresh65;
        (*_g_1).words.w0 =
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
        (*_g_1).words.w1 =
            Matrix_NewMtx((*globalCtx).state.gfxCtx,
                          b"../z_fishing.c\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                          3809 as libc::c_int) as libc::c_uint;
        if (i as libc::c_int) < 5 as libc::c_int {
            let fresh66 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_2: *mut Gfx = fresh66;
            (*_g_2).words.w0 =
                (0xfd as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 3 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        21 as libc::c_int |
                    (2 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 2 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        19 as libc::c_int |
                    ((1 as libc::c_int - 1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_2).words.w1 =
                gFishingRodSegmentBlackTex.as_mut_ptr() as libc::c_uint;
            let fresh67 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_3: *mut Gfx = fresh67;
            (*_g_3).words.w0 =
                (0xf5 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 3 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        21 as libc::c_int |
                    (2 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 2 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        19 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 9 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        9 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 9 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_3).words.w1 =
                (7 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        20 as libc::c_int |
                    ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 2 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        18 as libc::c_int |
                    (3 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        14 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        10 as libc::c_int |
                    ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 2 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    (4 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        4 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh68 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_4: *mut Gfx = fresh68;
            (*_g_4).words.w0 =
                (0xe6 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_4).words.w1 = 0 as libc::c_int as libc::c_uint;
            let fresh69 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_5: *mut Gfx = fresh69;
            (*_g_5).words.w0 =
                (0xf3 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        12 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_5).words.w1 =
                (7 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((if ((16 as libc::c_int * 8 as libc::c_int +
                               0 as libc::c_int >> 0 as libc::c_int) -
                              1 as libc::c_int) < 2047 as libc::c_int {
                          (16 as libc::c_int * 8 as libc::c_int +
                               0 as libc::c_int >> 0 as libc::c_int) -
                              1 as libc::c_int
                      } else { 2047 as libc::c_int }) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        12 as libc::c_int |
                    (((((1 as libc::c_int) << 11 as libc::c_int) +
                           (if 1 as libc::c_int >
                                   16 as libc::c_int * 2 as libc::c_int /
                                       8 as libc::c_int {
                                1 as libc::c_int
                            } else {
                                (16 as libc::c_int * 2 as libc::c_int) /
                                    8 as libc::c_int
                            }) - 1 as libc::c_int) /
                          (if 1 as libc::c_int >
                                  16 as libc::c_int * 2 as libc::c_int /
                                      8 as libc::c_int {
                               1 as libc::c_int
                           } else {
                               (16 as libc::c_int * 2 as libc::c_int) /
                                   8 as libc::c_int
                           })) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh70 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_6: *mut Gfx = fresh70;
            (*_g_6).words.w0 =
                (0xe7 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_6).words.w1 = 0 as libc::c_int as libc::c_uint;
            let fresh71 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_7: *mut Gfx = fresh71;
            (*_g_7).words.w0 =
                (0xf5 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 3 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        21 as libc::c_int |
                    (2 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 2 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        19 as libc::c_int |
                    ((16 as libc::c_int * 2 as libc::c_int + 7 as libc::c_int
                          >> 3 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 9 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        9 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 9 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_7).words.w1 =
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        20 as libc::c_int |
                    ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 2 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        18 as libc::c_int |
                    (3 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        14 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        10 as libc::c_int |
                    ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 2 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    (4 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        4 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh72 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_8: *mut Gfx = fresh72;
            (*_g_8).words.w0 =
                (0xf2 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        12 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_8).words.w1 =
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (((16 as libc::c_int - 1 as libc::c_int) <<
                          2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        12 as libc::c_int |
                    (((8 as libc::c_int - 1 as libc::c_int) <<
                          2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int
        } else if (i as libc::c_int) < 8 as libc::c_int ||
                      i as libc::c_int % 2 as libc::c_int == 0 as libc::c_int
         {
            let fresh73 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_9: *mut Gfx = fresh73;
            (*_g_9).words.w0 =
                (0xfd as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 3 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        21 as libc::c_int |
                    (2 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 2 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        19 as libc::c_int |
                    ((1 as libc::c_int - 1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_9).words.w1 =
                gFishingRodSegmentWhiteTex.as_mut_ptr() as libc::c_uint;
            let fresh74 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_10: *mut Gfx = fresh74;
            (*_g_10).words.w0 =
                (0xf5 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 3 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        21 as libc::c_int |
                    (2 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 2 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        19 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 9 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        9 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 9 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_10).words.w1 =
                (7 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        20 as libc::c_int |
                    ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 2 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        18 as libc::c_int |
                    (3 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        14 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        10 as libc::c_int |
                    ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 2 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    (4 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        4 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh75 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_11: *mut Gfx = fresh75;
            (*_g_11).words.w0 =
                (0xe6 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_11).words.w1 = 0 as libc::c_int as libc::c_uint;
            let fresh76 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_12: *mut Gfx = fresh76;
            (*_g_12).words.w0 =
                (0xf3 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        12 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_12).words.w1 =
                (7 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((if ((16 as libc::c_int * 8 as libc::c_int +
                               0 as libc::c_int >> 0 as libc::c_int) -
                              1 as libc::c_int) < 2047 as libc::c_int {
                          (16 as libc::c_int * 8 as libc::c_int +
                               0 as libc::c_int >> 0 as libc::c_int) -
                              1 as libc::c_int
                      } else { 2047 as libc::c_int }) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        12 as libc::c_int |
                    (((((1 as libc::c_int) << 11 as libc::c_int) +
                           (if 1 as libc::c_int >
                                   16 as libc::c_int * 2 as libc::c_int /
                                       8 as libc::c_int {
                                1 as libc::c_int
                            } else {
                                (16 as libc::c_int * 2 as libc::c_int) /
                                    8 as libc::c_int
                            }) - 1 as libc::c_int) /
                          (if 1 as libc::c_int >
                                  16 as libc::c_int * 2 as libc::c_int /
                                      8 as libc::c_int {
                               1 as libc::c_int
                           } else {
                               (16 as libc::c_int * 2 as libc::c_int) /
                                   8 as libc::c_int
                           })) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh77 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_13: *mut Gfx = fresh77;
            (*_g_13).words.w0 =
                (0xe7 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_13).words.w1 = 0 as libc::c_int as libc::c_uint;
            let fresh78 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_14: *mut Gfx = fresh78;
            (*_g_14).words.w0 =
                (0xf5 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 3 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        21 as libc::c_int |
                    (2 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 2 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        19 as libc::c_int |
                    ((16 as libc::c_int * 2 as libc::c_int + 7 as libc::c_int
                          >> 3 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 9 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        9 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 9 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_14).words.w1 =
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        20 as libc::c_int |
                    ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 2 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        18 as libc::c_int |
                    (3 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        14 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        10 as libc::c_int |
                    ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 2 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    (4 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        4 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh79 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_15: *mut Gfx = fresh79;
            (*_g_15).words.w0 =
                (0xf2 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        12 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_15).words.w1 =
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (((16 as libc::c_int - 1 as libc::c_int) <<
                          2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        12 as libc::c_int |
                    (((8 as libc::c_int - 1 as libc::c_int) <<
                          2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int
        } else {
            let fresh80 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_16: *mut Gfx = fresh80;
            (*_g_16).words.w0 =
                (0xfd as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 3 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        21 as libc::c_int |
                    (2 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 2 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        19 as libc::c_int |
                    ((1 as libc::c_int - 1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_16).words.w1 =
                gFishingRodSegmentStripTex.as_mut_ptr() as libc::c_uint;
            let fresh81 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_17: *mut Gfx = fresh81;
            (*_g_17).words.w0 =
                (0xf5 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 3 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        21 as libc::c_int |
                    (2 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 2 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        19 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 9 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        9 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 9 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_17).words.w1 =
                (7 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        20 as libc::c_int |
                    ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 2 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        18 as libc::c_int |
                    (3 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        14 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        10 as libc::c_int |
                    ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 2 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    (4 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        4 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh82 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_18: *mut Gfx = fresh82;
            (*_g_18).words.w0 =
                (0xe6 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_18).words.w1 = 0 as libc::c_int as libc::c_uint;
            let fresh83 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_19: *mut Gfx = fresh83;
            (*_g_19).words.w0 =
                (0xf3 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        12 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_19).words.w1 =
                (7 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((if ((16 as libc::c_int * 8 as libc::c_int +
                               0 as libc::c_int >> 0 as libc::c_int) -
                              1 as libc::c_int) < 2047 as libc::c_int {
                          (16 as libc::c_int * 8 as libc::c_int +
                               0 as libc::c_int >> 0 as libc::c_int) -
                              1 as libc::c_int
                      } else { 2047 as libc::c_int }) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        12 as libc::c_int |
                    (((((1 as libc::c_int) << 11 as libc::c_int) +
                           (if 1 as libc::c_int >
                                   16 as libc::c_int * 2 as libc::c_int /
                                       8 as libc::c_int {
                                1 as libc::c_int
                            } else {
                                (16 as libc::c_int * 2 as libc::c_int) /
                                    8 as libc::c_int
                            }) - 1 as libc::c_int) /
                          (if 1 as libc::c_int >
                                  16 as libc::c_int * 2 as libc::c_int /
                                      8 as libc::c_int {
                               1 as libc::c_int
                           } else {
                               (16 as libc::c_int * 2 as libc::c_int) /
                                   8 as libc::c_int
                           })) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh84 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_20: *mut Gfx = fresh84;
            (*_g_20).words.w0 =
                (0xe7 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_20).words.w1 = 0 as libc::c_int as libc::c_uint;
            let fresh85 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_21: *mut Gfx = fresh85;
            (*_g_21).words.w0 =
                (0xf5 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 3 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        21 as libc::c_int |
                    (2 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 2 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        19 as libc::c_int |
                    ((16 as libc::c_int * 2 as libc::c_int + 7 as libc::c_int
                          >> 3 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 9 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        9 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 9 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_21).words.w1 =
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        20 as libc::c_int |
                    ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 2 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        18 as libc::c_int |
                    (3 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        14 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        10 as libc::c_int |
                    ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 2 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    (4 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        4 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh86 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_22: *mut Gfx = fresh86;
            (*_g_22).words.w0 =
                (0xf2 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        12 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_22).words.w1 =
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (((16 as libc::c_int - 1 as libc::c_int) <<
                          2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        12 as libc::c_int |
                    (((8 as libc::c_int - 1 as libc::c_int) <<
                          2 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int
        }
        let fresh87 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_23: *mut Gfx = fresh87;
        (*_g_23).words.w0 =
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
        (*_g_23).words.w1 = gFishingRodSegmentDL.as_mut_ptr() as libc::c_uint;
        Matrix_Pop();
        Matrix_Translate(0.0f32, 0.0f32, 500.0f32,
                         MTXMODE_APPLY as libc::c_int as u8_0);
        if i as libc::c_int == 21 as libc::c_int {
            Matrix_MultVec3f(&mut sRodTipOffset, &mut sRodTipPos);
        }
        i += 1
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_fishing.c\x00" as *const u8 as
                         *const libc::c_char, 3838 as libc::c_int);
}
static mut D_80B7AF94: Vec3f =
    { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
#[no_mangle]
pub unsafe extern "C" fn Fishing_UpdateLure(mut this: *mut Fishing,
                                            mut globalCtx:
                                                *mut GlobalContext) {
    let mut spE4: f32_0 = 0.;
    let mut spE0: f32_0 = 0.;
    let mut phi_v0: s16 = 0;
    let mut spDC: s16 = 0;
    let mut spD8: f32_0 = 0.;
    let mut spD4: f32_0 = 0.;
    let mut spD0: f32_0 = 0.;
    let mut phi_f16: f32_0 = 0.;
    let mut spC8: f32_0 = 0.;
    let mut i: s16 = 0;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut zeroVec: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
    let mut spA8: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp9C: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp90: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut input: *mut Input =
        &mut *(*globalCtx).state.input.as_mut_ptr().offset(0 as libc::c_int as
                                                               isize) as
            *mut Input;
    let mut sp80: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp7C: f32_0 = 0.;
    let mut sp78: f32_0 = 0.;
    let mut phi_f0: f32_0 = 0.;
    let mut sp70: f32_0 = 0.;
    let mut sp64: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp58: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut pad: s32 = 0;
    D_80B7E0AE += 1;
    if D_80B7E0B0 as libc::c_int != 0 as libc::c_int { D_80B7E0B0 -= 1 }
    if D_80B7E0B2 as libc::c_int != 0 as libc::c_int { D_80B7E0B2 -= 1 }
    if D_80B7E0B4 as libc::c_int != 0 as libc::c_int { D_80B7E0B4 -= 1 }
    if D_80B7E122 as libc::c_int != 0 as libc::c_int { D_80B7E122 -= 1 }
    if D_80B7E150 as libc::c_int != 0 as libc::c_int { D_80B7E150 -= 1 }
    if D_80B7A6A4 as libc::c_int != 0 as libc::c_int {
        D_80B7A6A4 = D_80B7A6A4.wrapping_sub(1)
    }
    if D_80B7E0A4 as libc::c_int != 0 as libc::c_int { D_80B7E0A4 -= 1 }
    if D_80B7E114 as libc::c_int != 0 as libc::c_int { D_80B7E114 -= 1 }
    if D_80B7E0AC as libc::c_int == 1 as libc::c_int {
        D_80B7E0AC = 2 as libc::c_int as s16;
        D_80B7E084 = 0 as libc::c_int as u16_0;
        D_80B7E082 = 0 as libc::c_int as u8_0;
        D_80B7E0B6 = 0 as libc::c_int as u8_0;
        if sLinkAge as libc::c_int == 1 as libc::c_int &&
               gSaveContext.highScores[HS_FISHING as libc::c_int as usize] &
                   0x400 as libc::c_int != 0 ||
               sLinkAge as libc::c_int != 1 as libc::c_int &&
                   gSaveContext.highScores[HS_FISHING as libc::c_int as usize]
                       & 0x800 as libc::c_int != 0 {
            sSinkingLureLocation =
                (Rand_ZeroFloat(3.999f32) as u8_0 as libc::c_int +
                     1 as libc::c_int) as u8_0
        }
        D_80B7E148 = 520.0f32;
        D_80B7E144 = 195.0f32;
        D_80B7E150 = 0 as libc::c_int as s16;
        D_80B7E114 = D_80B7E150 as s8;
        D_80B7E120 = D_80B7E114 as u8_0;
        D_80B7E0B4 = D_80B7E120 as s16;
        D_80B7E0B2 = D_80B7E0B4;
        D_80B7E0B0 = D_80B7E0B2;
        D_80B7E0AE = D_80B7E0B0;
        D_80B7E0B6 = D_80B7E0AE as u8_0;
        D_80B7A694 = D_80B7E0B6 as s16;
        D_80B7E108 = 0.0f32;
        D_80B7E154 = D_80B7E108;
        D_80B7E104 = D_80B7E154;
        D_80B7E128 = zeroVec;
        i = 0 as libc::c_int as s16;
        while (i as libc::c_int) < 200 as libc::c_int {
            sReelLinePos[i as usize] = zeroVec;
            sReelLineRot[i as usize] = zeroVec;
            sReelLineUnk[i as usize] = zeroVec;
            i += 1
        }
    }
    SkinMatrix_Vec3fMtxFMultXYZW(&mut (*globalCtx).viewProjectionMtxF,
                                 &mut sLurePos, &mut D_80B7AF94,
                                 &mut sProjectedW);
    if D_80B7A694 as libc::c_int == 0 as libc::c_int {
        Math_ApproachF(&mut D_80B7E108, -800.0f32, 1.0f32, 20.0f32);
    } else { Math_ApproachF(&mut D_80B7E108, 300.0f32, 1.0f32, 20.0f32); }
    match D_80B7A694 as libc::c_int {
        0 => {
            D_80B7FEA0 = 0 as libc::c_int as s16;
            if (*gGameInfo).data[(13 as libc::c_int * 6 as libc::c_int *
                                      16 as libc::c_int + 14 as libc::c_int)
                                     as usize] as libc::c_int !=
                   0 as libc::c_int {
                (*gGameInfo).data[(13 as libc::c_int * 6 as libc::c_int *
                                       16 as libc::c_int + 14 as libc::c_int)
                                      as usize] = 0 as libc::c_int as s16;
                D_80B7E0B6 =
                    (2 as libc::c_int - D_80B7E0B6 as libc::c_int) as u8_0;
                if D_80B7E0B6 as libc::c_int != 0 as libc::c_int {
                    D_80B7E082 = 0 as libc::c_int as u8_0
                }
            }
            Math_ApproachF(&mut D_80B7E144, 195.0f32, 1.0f32, 1.0f32);
            if (*player).stateFlags1 &
                   0x8000000 as libc::c_int as libc::c_uint != 0 {
                D_80B7E0B4 = 0 as libc::c_int as s16;
                (*player).unk_860 = 0 as libc::c_int as s16
            }
            if D_80B7E0B4 as libc::c_int == 0 as libc::c_int {
                if D_80B7E0B0 as libc::c_int == 0 as libc::c_int &&
                       (*player).unk_860 as libc::c_int == 1 as libc::c_int {
                    D_80B7E0B4 = 37 as libc::c_int as s16;
                    Message_CloseTextbox(globalCtx);
                }
            } else {
                sLureRot.x =
                    sReelLineRot[(200 as libc::c_int - 2 as libc::c_int) as
                                     usize].x + 3.14159265358979323846f32;
                sLureRot.y =
                    sReelLineRot[(200 as libc::c_int - 2 as libc::c_int) as
                                     usize].y;
                if D_80B7E0B4 as libc::c_int == 18 as libc::c_int {
                    D_80B7A694 = 1 as libc::c_int as s16;
                    sLurePos = sRodTipPos;
                    Matrix_RotateY((*player).actor.shape.rot.y as libc::c_int
                                       as libc::c_float / 32768.0f32 *
                                       3.14159265358979323846f32,
                                   MTXMODE_NEW as libc::c_int as u8_0);
                    sp90.x = 0.0f32;
                    sp90.y = 0.0f32;
                    sp90.z = 25.0f32;
                    Matrix_MultVec3f(&mut sp90, &mut D_80B7E0E8);
                    D_80B7E0E8.y = 15.0f32;
                    D_80B7E0F8.z = 0.0f32;
                    D_80B7E0F8.x = D_80B7E0F8.z;
                    D_80B7E0F8.y = -1.0f32;
                    D_80B7E148 = 0.0f32;
                    D_80B7E0B2 = 5 as libc::c_int as s16;
                    D_80B7E11C = 0.5f32;
                    D_80B7E118 = Rand_ZeroFloat(1.9f32) as u8_0;
                    sFishMouthOffset.y = 500.0f32;
                    func_80078914(&mut D_80B7AF94,
                                  0x1818 as libc::c_int as u16_0);
                }
            }
        }
        1 => {
            spE0 = sLurePos.y;
            sLurePos.x += D_80B7E0E8.x;
            sLurePos.y += D_80B7E0E8.y;
            sLurePos.z += D_80B7E0E8.z;
            D_80B7E0E8.x += D_80B7E0F8.x;
            D_80B7E0E8.y += D_80B7E0F8.y;
            D_80B7E0E8.z += D_80B7E0F8.z;
            if !((*input).cur.button as libc::c_int |
                     !(0x8000 as libc::c_int)) == 0 as libc::c_int ||
                   D_80B7A68C as libc::c_int != 0 as libc::c_int {
                D_80B7E0E8.x *= 0.9f32;
                D_80B7E0E8.z *= 0.9f32;
                if D_80B7A68C as libc::c_int == 0 as libc::c_int {
                    func_80078884((0x183e as libc::c_int -
                                       0x800 as libc::c_int) as u16_0);
                }
            }
            spD8 = sLurePos.x - sRodTipPos.x;
            spD4 = sLurePos.y - sRodTipPos.y;
            spD0 = sLurePos.z - sRodTipPos.z;
            if D_80B7E0B2 as libc::c_int != 0 as libc::c_int {
                sLureRot.x =
                    sReelLineRot[(200 as libc::c_int - 2 as libc::c_int) as
                                     usize].x + 3.14159265358979323846f32;
                sLureRot.y =
                    sReelLineRot[(200 as libc::c_int - 2 as libc::c_int) as
                                     usize].y
            } else {
                sLureRot.x = 0.0f32;
                sLureRot.y =
                    Math_Atan2F(spD0, spD8) + 3.14159265358979323846f32
            }
            phi_f16 = sqrtf(spD8 * spD8 + spD4 * spD4 + spD0 * spD0);
            if phi_f16 > 1000.0f32 { phi_f16 = 1000.0f32 }
            D_80B7E144 = 200.0f32 - phi_f16 * 200.0f32 * 0.001f32;
            spC8 = sLurePos.x * sLurePos.x + sLurePos.z * sLurePos.z;
            if spC8 > 920.0f32 * 920.0f32 {
                if (*gGameInfo).data[(13 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          56 as libc::c_int) as usize] as
                       libc::c_int != 0 as libc::c_int ||
                       sLurePos.y > 160.0f32 || sLurePos.x < 80.0f32 ||
                       sLurePos.x > 180.0f32 || sLurePos.z > 1350.0f32 ||
                       sLurePos.z < 1100.0f32 || sLurePos.y < 45.0f32 {
                    sp80 = (*this).actor.world.pos;
                    (*this).actor.world.pos = sLurePos;
                    (*this).actor.prevPos = (*this).actor.world.pos;
                    Actor_UpdateBgCheckInfo(globalCtx, &mut (*this).actor,
                                            15.0f32, 30.0f32, 30.0f32,
                                            0x43 as libc::c_int);
                    (*this).actor.world.pos = sp80;
                    if (*this).actor.bgCheckFlags as libc::c_int &
                           0x10 as libc::c_int != 0 {
                        D_80B7E0E8.y = -0.5f32
                    }
                    if (*this).actor.bgCheckFlags as libc::c_int &
                           8 as libc::c_int != 0 {
                        if D_80B7E0E8.y > 0.0f32 { D_80B7E0E8.y = 0.0f32 }
                        D_80B7E0E8.z = 0.0f32;
                        D_80B7E0E8.x = D_80B7E0E8.z
                    }
                } else if func_80B6C2EC(&mut sLurePos) != 0 as libc::c_int {
                    D_80B7A694 = 3 as libc::c_int as s16;
                    D_80B7E154 = 0.0f32
                }
                spE4 =
                    (sqrtf(spC8) - 920.0f32) * 0.11f32 +
                        (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface
                            as libc::c_int as libc::c_float;
                if sLurePos.y <= spE4 {
                    sLurePos.y = spE4;
                    D_80B7E0E8.z = 0.0f32;
                    D_80B7E0E8.y = D_80B7E0E8.z;
                    D_80B7E0E8.x = D_80B7E0E8.y;
                    D_80B7A694 = 3 as libc::c_int as s16;
                    D_80B7E154 = 0.0f64 as f32_0
                } else {
                    Math_ApproachF(&mut D_80B7E148, 0.0f32, 1.0f32, 0.05f32);
                    func_80078914(&mut D_80B7AF94,
                                  (0x38a4 as libc::c_int -
                                       0x800 as libc::c_int) as u16_0);
                }
            } else {
                spE4 =
                    (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface as
                        f32_0;
                if sLurePos.y <= spE4 {
                    D_80B7A694 = 2 as libc::c_int as s16;
                    D_80B7E154 = 0.0f32;
                    D_80B7E0E8.z = 0.0f32;
                    D_80B7E0E8.x = D_80B7E0E8.z;
                    if D_80B7E0B6 as libc::c_int == 2 as libc::c_int {
                        D_80B7E0A2 = 0 as libc::c_int as u8_0
                    } else { D_80B7E0A2 = 10 as libc::c_int as u8_0 }
                    if sLurePos.y <= spE4 && spE4 < spE0 &&
                           spE4 ==
                               (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface
                                   as libc::c_int as libc::c_float {
                        D_80B7E114 = 10 as libc::c_int as s8;
                        func_80078914(&mut D_80B7AF94,
                                      0x2817 as libc::c_int as u16_0);
                        D_80B7E0F8.y = 0.0f32;
                        D_80B7E0E8.y *= 0.2f32;
                        i = 0 as libc::c_int as s16;
                        while (i as libc::c_int) < 50 as libc::c_int {
                            sp7C = Rand_ZeroFloat(1.5f32) + 0.5f32;
                            sp78 = Rand_ZeroFloat(6.28f32);
                            sp9C.x = sinf(sp78) * sp7C;
                            sp9C.z = cosf(sp78) * sp7C;
                            sp9C.y = Rand_ZeroFloat(3.0f32) + 3.0f32;
                            spA8 = sLurePos;
                            spA8.x += sp9C.x * 3.0f32;
                            spA8.y =
                                (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface
                                    as f32_0;
                            spA8.z += sp9C.z * 3.0f32;
                            Fishing_SpawnDustSplash(0 as *mut Vec3f,
                                                    (*globalCtx).specialEffects
                                                        as *mut FishingEffect,
                                                    &mut spA8, &mut sp9C,
                                                    Rand_ZeroFloat(0.02f32) +
                                                        0.025f32);
                            i += 1
                        }
                        spA8 = sLurePos;
                        spA8.y =
                            (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface
                                as f32_0;
                        Fishing_SpawnRipple(0 as *mut Vec3f,
                                            (*globalCtx).specialEffects as
                                                *mut FishingEffect, &mut spA8,
                                            100.0f32, 800.0f32,
                                            150 as libc::c_int as s16,
                                            90 as libc::c_int as s16);
                    }
                } else {
                    Math_ApproachZeroF(&mut D_80B7E148, 1.0f32, 0.05f32);
                    func_80078914(&mut D_80B7AF94,
                                  (0x38a4 as libc::c_int -
                                       0x800 as libc::c_int) as u16_0);
                }
            }
            sReelLinePos[(200 as libc::c_int - 1 as libc::c_int) as usize].x =
                sLurePos.x;
            sReelLinePos[(200 as libc::c_int - 1 as libc::c_int) as usize].y =
                sLurePos.y;
            sReelLinePos[(200 as libc::c_int - 1 as libc::c_int) as usize].z =
                sLurePos.z;
            D_80B7E140 = 1.0f32;
            D_80B7E10C = 0.5f32
        }
        2 => {
            if sLurePos.y <=
                   (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface as
                       libc::c_int as libc::c_float {
                sLurePos.y += D_80B7E0E8.y;
                Math_ApproachZeroF(&mut D_80B7E0E8.y, 1.0f32, 1.0f32);
                if D_80B7E0B6 as libc::c_int != 2 as libc::c_int {
                    Math_ApproachF(&mut sLurePos.y,
                                   (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface
                                       as f32_0, 0.5f32, 1.0f32);
                }
            }
            Math_ApproachF(&mut D_80B7E148, 2.0f32, 1.0f32, 0.1f32);
            if D_80B7E0A2 as libc::c_int == 0 as libc::c_int {
                D_80B7A694 = 3 as libc::c_int as s16
            } else { D_80B7E0A2 = D_80B7E0A2.wrapping_sub(1) }
        }
        3 => {
            D_80B7FEA0 = 0 as libc::c_int as s16;
            if D_80B7A68C as libc::c_int != 0 as libc::c_int &&
                   sLurePos.x * sLurePos.x + sLurePos.z * sLurePos.z <
                       500.0f32 * 500.0f32 {
                D_80B7A690 = 1 as libc::c_int as u8_0
            }
            (*player).unk_860 = 2 as libc::c_int as s16;
            if D_80B7E138 < 3.0f32 {
                spD0 =
                    D_80B7E10C *
                        Math_SinS((D_80B7E0AE as libc::c_int *
                                       0x1060 as libc::c_int) as s16);
                Math_ApproachF(&mut sLureRot.x,
                               -3.14159265358979323846f32 / 6.0f32 + spD0,
                               0.3f32, D_80B7E110);
                Math_ApproachF(&mut D_80B7E110, 0.5f32, 1.0f32, 0.02f32);
                Math_ApproachZeroF(&mut D_80B7E10C, 1.0f32, 0.02f32);
            } else { D_80B7E110 = 0.0f32 }
            spDC = 0x4000 as libc::c_int as s16;
            spE4 =
                (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface as
                    f32_0;
            spC8 = sLurePos.x * sLurePos.x + sLurePos.z * sLurePos.z;
            if spC8 < 920.0f32 * 920.0f32 {
                if sLurePos.y <= spE4 + 4.0f32 {
                    sp70 = 0.0f32;
                    if D_80B7E150 as libc::c_int == 0 as libc::c_int {
                        if fabsf((*input).rel.stick_x as f32_0) > 30.0f32 {
                            sp70 =
                                fabsf(((*input).rel.stick_x as libc::c_int -
                                           D_80B7A6C4 as libc::c_int) as
                                          libc::c_float * (1.0f32 / 60.0f32))
                        } else if fabsf((*input).rel.stick_y as f32_0) >
                                      30.0f32 {
                            sp70 =
                                fabsf(((*input).rel.stick_y as libc::c_int -
                                           D_80B7A6C8 as libc::c_int) as
                                          libc::c_float * (1.0f32 / 60.0f32))
                        }
                    }
                    if sp70 > 1.0f32 { sp70 = 1.0f32 }
                    if !((*input).press.button as libc::c_int |
                             !(0x4000 as libc::c_int)) == 0 as libc::c_int {
                        sp70 = 0.5f32
                    }
                    if D_80B7A68C as libc::c_int != 0 as libc::c_int {
                        if sp70 > 0.3f32 { sp70 = 0.3f32 }
                    }
                    if sp70 > 0.2f32 && D_80B7E138 < 4.0f32 {
                        D_80B7E150 = 5 as libc::c_int as s16;
                        if sp70 > 0.8f32 {
                            D_80B7E120 = 2 as libc::c_int as u8_0
                        } else { D_80B7E120 = 1 as libc::c_int as u8_0 }
                        sp90.x = (*player).actor.world.pos.x - sLurePos.x;
                        sp90.z = (*player).actor.world.pos.z - sLurePos.z;
                        sp90.y = Math_Atan2F(sp90.z, sp90.x);
                        D_80B7E134 = sp70 * D_80B7E140 + sp90.y;
                        D_80B7E140 = D_80B7E140 * -1.0f32;
                        D_80B7E138 = fabsf(sp70) * 6.0f32;
                        sLureRot.x = 0.0f32;
                        D_80B7E10C = 0.5f32;
                        D_80B7E144 +=
                            fabsf(sp70) *
                                (7.5f32 +
                                     (*gGameInfo).data[(13 as libc::c_int *
                                                            6 as libc::c_int *
                                                            16 as libc::c_int
                                                            +
                                                            25 as libc::c_int)
                                                           as usize] as
                                         libc::c_int as libc::c_float *
                                         0.1f32);
                        func_800F436C(&mut D_80B7AF94,
                                      0x28c3 as libc::c_int as u16_0,
                                      sp70 * 1.999f32 * 0.25f32 + 0.75f32);
                        if D_80B7E0B6 as libc::c_int == 2 as libc::c_int {
                            D_80B7E128.y = 5.0f32 * sp70;
                            sReelLinePos[(200 as libc::c_int -
                                              1 as libc::c_int) as usize].y +=
                                D_80B7E128.y;
                            sLurePos.y += D_80B7E128.y
                        }
                    } else if !((*input).cur.button as libc::c_int |
                                    !(0x8000 as libc::c_int)) ==
                                  0 as libc::c_int {
                        spDC = 0x500 as libc::c_int as s16;
                        D_80B7E134 =
                            sReelLineRot[(200 as libc::c_int -
                                              2 as libc::c_int) as usize].y +
                                3.14159265358979323846f32;
                        sLureRot.x = 0.0f32;
                        D_80B7E10C = 0.5f32;
                        if D_80B7E0B6 as libc::c_int == 2 as libc::c_int {
                            D_80B7E128.y = 0.2f32;
                            sReelLinePos[(200 as libc::c_int -
                                              1 as libc::c_int) as usize].y +=
                                D_80B7E128.y;
                            sLurePos.y += D_80B7E128.y
                        }
                    }
                } else if D_80B7E144 > 150.0f32 {
                    sLureRot.x =
                        sReelLineRot[(200 as libc::c_int - 2 as libc::c_int)
                                         as usize].x +
                            3.14159265358979323846f32;
                    D_80B7E134 =
                        sReelLineRot[(200 as libc::c_int - 2 as libc::c_int)
                                         as usize].y +
                            3.14159265358979323846f32;
                    D_80B7E144 += 2.0f32
                }
            } else {
                spE4 =
                    (sqrtf(spC8) - 920.0f32) * 0.11f32 +
                        (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface
                            as libc::c_int as libc::c_float;
                if sLurePos.y <= spE4 {
                    sLurePos.y = spE4;
                    spDC = 0x500 as libc::c_int as s16;
                    D_80B7E134 =
                        sReelLineRot[(200 as libc::c_int - 2 as libc::c_int)
                                         as usize].y +
                            3.14159265358979323846f32;
                    sLureRot.x = 0.0f32;
                    if !((*input).press.button as libc::c_int |
                             !(0x4000 as libc::c_int)) == 0 as libc::c_int {
                        D_80B7E144 += 6.0f32;
                        func_80078914(&mut D_80B7AF94,
                                      0x801 as libc::c_int as u16_0);
                    }
                } else if D_80B7E144 > 150.0f32 {
                    sLureRot.x =
                        sReelLineRot[(200 as libc::c_int - 2 as libc::c_int)
                                         as usize].x +
                            3.14159265358979323846f32;
                    D_80B7E134 =
                        sReelLineRot[(200 as libc::c_int - 2 as libc::c_int)
                                         as usize].y +
                            3.14159265358979323846f32;
                    D_80B7E144 += 2.0f32
                }
            }
            Math_ApproachZeroF(&mut D_80B7E138, 1.0f32, 0.3f32);
            Math_ApproachS(&mut D_80B7E13C,
                           (D_80B7E134 * 32768.0f32 /
                                3.14159265358979323846f32) as s16,
                           3 as libc::c_int as s16, spDC);
            sLureRot.y =
                D_80B7E13C as libc::c_int as libc::c_float / 32768.0f32 *
                    3.14159265358979323846f32;
            sp90.x = 0.0f32;
            sp90.y = 0.0f32;
            sp90.z = D_80B7E138;
            Matrix_RotateY(sLureRot.y, MTXMODE_NEW as libc::c_int as u8_0);
            if D_80B7E0B6 as libc::c_int == 2 as libc::c_int {
                Matrix_MultVec3f(&mut sp90, &mut sp64);
                D_80B7E128.x = sp64.x;
                D_80B7E128.z = sp64.z;
                phi_f0 = 10.0f32
            } else {
                Matrix_MultVec3f(&mut sp90, &mut D_80B7E128);
                phi_f0 = 0.0f32
            }
            D_80B7E104 = 0.0f32;
            if D_80B7E0B6 as libc::c_int == 1 as libc::c_int &&
                   !((*input).cur.button as libc::c_int |
                         !(0x8000 as libc::c_int)) == 0 as libc::c_int {
                D_80B7E128.y = -2.0f32;
                if D_80B7E0AE as libc::c_int & 1 as libc::c_int !=
                       0 as libc::c_int {
                    D_80B7E104 = 0.5f32
                } else { D_80B7E104 = -0.5f32 }
            } else if sReelLinePos[(200 as libc::c_int - 1 as libc::c_int) as
                                       usize].y <
                          (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface
                              as libc::c_int as libc::c_float + phi_f0 {
                if D_80B7E0B6 as libc::c_int == 2 as libc::c_int {
                    sp58 = (*this).actor.world.pos;
                    (*this).actor.world.pos = sLurePos;
                    (*this).actor.prevPos = (*this).actor.world.pos;
                    Actor_UpdateBgCheckInfo(globalCtx, &mut (*this).actor,
                                            15.0f32, 30.0f32, 30.0f32,
                                            0x44 as libc::c_int);
                    (*this).actor.world.pos = sp58;
                    D_80B7E128.y += -0.5f32;
                    if D_80B7E128.y < -1.0f32 { D_80B7E128.y = -1.0f32 }
                    if sLurePos.y < (*this).actor.floorHeight + 5.0f32 {
                        sLurePos.y = (*this).actor.floorHeight + 5.0f32;
                        sReelLinePos[(200 as libc::c_int - 1 as libc::c_int)
                                         as usize].y = sLurePos.y;
                        D_80B7E128.y = 0.0f32
                    } else { D_80B7E120 = 1 as libc::c_int as u8_0 }
                } else {
                    D_80B7E128.y =
                        fabsf(sReelLinePos[(200 as libc::c_int -
                                                1 as libc::c_int) as usize].y
                                  -
                                  (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface
                                      as libc::c_int as libc::c_float) *
                            0.2f32;
                    if D_80B7E128.y > 1.5f32 { D_80B7E128.y = 1.5f32 }
                }
            }
            sReelLinePos[(200 as libc::c_int - 1 as libc::c_int) as usize].x
                += D_80B7E128.x;
            sReelLinePos[(200 as libc::c_int - 1 as libc::c_int) as usize].y
                += D_80B7E128.y;
            sReelLinePos[(200 as libc::c_int - 1 as libc::c_int) as usize].z
                += D_80B7E128.z;
            if sReelLinePos[(200 as libc::c_int - 1 as libc::c_int) as
                                usize].y > spE4 + 6.0f32 {
                sReelLinePos[(200 as libc::c_int - 1 as libc::c_int) as
                                 usize].y -= 5.0f32
            }
            D_80B7E0F8.y = 0.0f32;
            D_80B7E0E8.z = D_80B7E0F8.y;
            D_80B7E0E8.y = D_80B7E0E8.z;
            D_80B7E0E8.x = D_80B7E0E8.y;
            if !((*input).cur.button as libc::c_int |
                     !(0x8000 as libc::c_int)) == 0 as libc::c_int {
                if !((*input).cur.button as libc::c_int |
                         !(0x10 as libc::c_int)) == 0 as libc::c_int {
                    D_80B7E144 += 1.5f32;
                    func_80078884((0x183e as libc::c_int -
                                       0x800 as libc::c_int) as u16_0);
                    Math_ApproachF(&mut D_80B7E154, 1000.0f32, 1.0f32,
                                   2.0f32);
                } else {
                    D_80B7E144 += D_80B7E11C;
                    func_80078884((0x183d as libc::c_int -
                                       0x800 as libc::c_int) as u16_0);
                    Math_ApproachF(&mut D_80B7E154, 1000.0f32, 1.0f32,
                                   0.2f32);
                }
                if sReelLinePos[(200 as libc::c_int - 1 as libc::c_int) as
                                    usize].y >
                       (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface
                           as libc::c_int as libc::c_float + 4.0f32 {
                    Math_ApproachF(&mut D_80B7E148, 3.0f32, 1.0f32, 0.2f32);
                } else {
                    Math_ApproachF(&mut D_80B7E148, 1.0f32, 1.0f32, 0.2f32);
                }
            } else {
                Math_ApproachF(&mut D_80B7E148, 2.0f32, 1.0f32, 0.2f32);
            }
            Math_ApproachF(&mut sLurePos.x,
                           sReelLinePos[(200 as libc::c_int -
                                             1 as libc::c_int) as usize].x,
                           1.0f32, D_80B7E154);
            Math_ApproachF(&mut sLurePos.y,
                           sReelLinePos[(200 as libc::c_int -
                                             1 as libc::c_int) as usize].y,
                           1.0f32, D_80B7E154);
            Math_ApproachF(&mut sLurePos.z,
                           sReelLinePos[(200 as libc::c_int -
                                             1 as libc::c_int) as usize].z,
                           1.0f32, D_80B7E154);
            if D_80B7E138 > 1.0f32 {
                Math_ApproachF(&mut D_80B7E154, 1000.0f32, 1.0f32, 1.0f32);
            }
            Math_ApproachF(&mut D_80B7E154, 1000.0f32, 1.0f32, 0.1f32);
            if D_80B7E144 >= 195.0f32 {
                D_80B7E144 = 195.0f32;
                D_80B7A694 = 0 as libc::c_int as s16;
                D_80B7E148 = 520.0f32;
                D_80B7A6CC = 3 as libc::c_int as u8_0
            }
            if sLurePos.y <=
                   (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface as
                       libc::c_int as libc::c_float + 4.0f32 &&
                   sLurePos.y >=
                       (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface
                           as libc::c_int as libc::c_float - 4.0f32 {
                phi_v0 = 63 as libc::c_int as s16;
                if !((*input).cur.button as libc::c_int |
                         !(0x8000 as libc::c_int)) == 0 as libc::c_int ||
                       D_80B7E138 > 1.0f32 {
                    phi_v0 = 1 as libc::c_int as s16
                }
                if D_80B7E0AE as libc::c_int & phi_v0 as libc::c_int ==
                       0 as libc::c_int {
                    spA8 = sLurePos;
                    spA8.y =
                        (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface
                            as f32_0;
                    Fishing_SpawnRipple(0 as *mut Vec3f,
                                        (*globalCtx).specialEffects as
                                            *mut FishingEffect, &mut spA8,
                                        30.0f32, 300.0f32,
                                        150 as libc::c_int as s16,
                                        90 as libc::c_int as s16);
                }
            }
        }
        4 => {
            if (*this).unk_157 as libc::c_int != 0 as libc::c_int {
                (*this).unk_157 = (*this).unk_157.wrapping_sub(1);
                D_80B7E144 += D_80B7E11C
            }
            if !((*input).cur.button as libc::c_int |
                     !(0x8000 as libc::c_int)) == 0 as libc::c_int {
                if sLurePos.x * sLurePos.x + sLurePos.z * sLurePos.z >
                       920.0f32 * 920.0f32 {
                    D_80B7E144 +=
                        1.0f32 +
                            (*gGameInfo).data[(13 as libc::c_int *
                                                   6 as libc::c_int *
                                                   16 as libc::c_int +
                                                   65 as libc::c_int) as
                                                  usize] as libc::c_int as
                                libc::c_float * 0.1f32
                } else { D_80B7E144 += D_80B7E11C }
                func_80078884((0x183d as libc::c_int - 0x800 as libc::c_int)
                                  as u16_0);
            }
            if D_80B7E0AE as libc::c_int & 0x1f as libc::c_int ==
                   0 as libc::c_int {
                if D_80B7E124 as libc::c_int != 0 as libc::c_int ||
                       D_80B7E0B6 as libc::c_int != 2 as libc::c_int {
                    D_80B7A6A4 = 5 as libc::c_int as u8_0
                }
            }
            Math_ApproachF(&mut D_80B7E148, 0.0f32, 1.0f32, 0.2f32);
        }
        5 => {
            D_80B7E14C = 0.0005000001f32;
            sReelLinePos[(200 as libc::c_int - 1 as libc::c_int) as usize].x =
                sLurePos.x;
            sReelLinePos[(200 as libc::c_int - 1 as libc::c_int) as usize].y =
                sLurePos.y;
            sReelLinePos[(200 as libc::c_int - 1 as libc::c_int) as usize].z =
                sLurePos.z;
            D_80B7E148 = 2.0f32
        }
        _ => { }
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_80B70A2C(mut this: *mut Fishing,
                                       mut globalCtx: *mut GlobalContext,
                                       mut ignorePosCheck: u8_0) -> s32 {
    let mut i: s16 = 0;
    let mut count: s16 = 0;
    let mut scale: f32_0 = 0.;
    let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut vel: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut speedXZ: f32_0 = 0.;
    let mut angle: f32_0 = 0.;
    if (*this).actor.world.pos.y <
           (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface as
               libc::c_int as libc::c_float - 10.0f32 && ignorePosCheck == 0 {
        return 0 as libc::c_int
    }
    // Necessary to match
    ((*this).unk_1AC) != 0.;
    if (*this).unk_1AC >= 40.0f32 {
        count = 40 as libc::c_int as s16;
        scale = 1.2f32
    } else { count = 30 as libc::c_int as s16; scale = 1.0f32 }
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < count as libc::c_int {
        speedXZ = (Rand_ZeroFloat(1.5f32) + 0.5f32) * scale;
        angle = Rand_ZeroFloat(6.28f32);
        vel.x = sinf(angle) * speedXZ;
        vel.z = cosf(angle) * speedXZ;
        vel.y = (Rand_ZeroFloat(3.0f32) + 3.0f32) * scale;
        pos = (*this).actor.world.pos;
        pos.x += vel.x * 3.0f32;
        pos.y =
            (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface as f32_0;
        pos.z += vel.z * 3.0f32;
        Fishing_SpawnDustSplash(&mut (*this).actor.projectedPos,
                                (*globalCtx).specialEffects as
                                    *mut FishingEffect, &mut pos, &mut vel,
                                (Rand_ZeroFloat(0.02f32) + 0.025f32) * scale);
        i += 1
    }
    pos = (*this).actor.world.pos;
    pos.y = (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface as f32_0;
    Fishing_SpawnRipple(&mut (*this).actor.projectedPos,
                        (*globalCtx).specialEffects as *mut FishingEffect,
                        &mut pos, 100.0f32, 800.0f32,
                        150 as libc::c_int as s16, 90 as libc::c_int as s16);
    (*this).unk_151 = 30 as libc::c_int as u8_0;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_80B70CF0(mut this: *mut Fishing,
                                       mut globalCtx: *mut GlobalContext) {
    let mut count: s16 = 0;
    let mut i: s16 = 0;
    let mut scale: f32_0 = 0.;
    let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut vel: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut speedXZ: f32_0 = 0.;
    let mut angle: f32_0 = 0.;
    // Necessary to match
    ((*this).unk_1AC) != 0.;
    if (*this).unk_1AC >= 45.0f32 {
        count = 30 as libc::c_int as s16;
        scale = 0.5f32
    } else { count = 20 as libc::c_int as s16; scale = 0.3f32 }
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < count as libc::c_int {
        speedXZ = (Rand_ZeroFloat(1.5f32) + 0.5f32) * scale;
        angle = Rand_ZeroFloat(6.28f32);
        vel.x = sinf(angle) * speedXZ;
        vel.z = cosf(angle) * speedXZ;
        vel.y = Rand_ZeroFloat(2.0f32) + 2.0f32;
        pos = (*this).actor.world.pos;
        pos.x += vel.x * 3.0f32;
        pos.y += vel.y * 3.0f32;
        pos.z += vel.z * 3.0f32;
        Fishing_SpawnDustSplash(&mut (*this).actor.projectedPos,
                                (*globalCtx).specialEffects as
                                    *mut FishingEffect, &mut pos, &mut vel,
                                (Rand_ZeroFloat(0.02f32) + 0.025f32) * scale);
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_80B70ED4(mut this: *mut Fishing,
                                       mut input: *mut Input) {
    let mut sp34: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp28: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp24: f32_0 = 0.;
    sp34.x = sLurePos.x - (*this).actor.world.pos.x;
    sp34.y = sLurePos.y - (*this).actor.world.pos.y;
    sp34.z = sLurePos.z - (*this).actor.world.pos.z;
    sp24 = sp34.x * sp34.x + sp34.y * sp34.y + sp34.z * sp34.z;
    if D_80B7A694 as libc::c_int == 3 as libc::c_int &&
           (*this).unk_1A2 as libc::c_int == 0 as libc::c_int &&
           D_80B7A68C as libc::c_int == 0 as libc::c_int {
        Matrix_RotateY(-((*this).actor.shape.rot.y as libc::c_int) as
                           libc::c_float / 32768.0f32 *
                           3.14159265358979323846f32,
                       MTXMODE_NEW as libc::c_int as u8_0);
        Matrix_MultVec3f(&mut sp34, &mut sp28);
        if sp28.z > 0.0f32 || (*this).unk_1AC < 40.0f32 {
            if (*this).unk_158 as libc::c_int == 7 as libc::c_int &&
                   sp24 < 200.0f32 * 200.0f32 {
                (*this).unk_158 = 4 as libc::c_int as s16;
                (*this).unk_1B4 = sLurePos;
                (*this).unk_1B0 = 28672.0f32;
                (*this).unk_188 = 5.0f32
            } else {
                if (!((*input).cur.button as libc::c_int |
                          !(0x8000 as libc::c_int)) == 0 as libc::c_int ||
                        D_80B7E138 > 1.0f32) && sp24 < 120.0f32 * 120.0f32 {
                    (*this).unk_158 = 2 as libc::c_int as s16;
                    (*this).unk_15E = 0 as libc::c_int as s16;
                    (*this).unk_17A[0 as libc::c_int as usize] =
                        0 as libc::c_int as s16;
                    (*this).unk_17A[2 as libc::c_int as usize] =
                        (Rand_ZeroFloat(100.0f32) as s16 as libc::c_int +
                             100 as libc::c_int) as s16;
                    (*this).unk_1A8 =
                        sFishInits[((*this).actor.params as libc::c_int -
                                        100 as libc::c_int) as usize].unk_0C;
                    (*this).unk_1B0 = 0.0f32
                }
                if (*this).unk_17A[1 as libc::c_int as usize] as libc::c_int
                       == 0 as libc::c_int && sp24 < 70.0f32 * 70.0f32 {
                    (*this).unk_158 = 2 as libc::c_int as s16;
                    (*this).unk_15E = 0 as libc::c_int as s16;
                    (*this).unk_17A[0 as libc::c_int as usize] =
                        0 as libc::c_int as s16;
                    (*this).unk_17A[2 as libc::c_int as usize] =
                        (Rand_ZeroFloat(100.0f32) as s16 as libc::c_int +
                             100 as libc::c_int) as s16;
                    (*this).unk_1A8 =
                        sFishInits[((*this).actor.params as libc::c_int -
                                        100 as libc::c_int) as usize].unk_0C;
                    (*this).unk_1B0 = 0.0f32
                }
            }
        }
    } else if D_80B7A694 as libc::c_int == 4 as libc::c_int &&
                  D_80B7E124 as libc::c_int != 0 as libc::c_int &&
                  sp24 < 100.0f32 * 100.0f32 &&
                  (*this).unk_158 as libc::c_int >= 10 as libc::c_int {
        (*this).unk_15A = 0 as libc::c_int as s16;
        (*this).unk_158 = 1 as libc::c_int as s16;
        (*this).unk_1A4 = 1000 as libc::c_int as s16;
        (*this).unk_1A2 = 100 as libc::c_int as s16;
        (*this).unk_17A[1 as libc::c_int as usize] = 50 as libc::c_int as s16
    }
    if D_80B7E0B6 as libc::c_int != 2 as libc::c_int &&
           D_80B7E114 as libc::c_int != 0 as libc::c_int &&
           (*this).unk_1AC > 60.0f32 && sp24 < 30.0f32 * 30.0f32 &&
           (*this).unk_158 as libc::c_int >= 10 as libc::c_int {
        (*this).unk_15A = 0 as libc::c_int as s16;
        (*this).unk_158 = 1 as libc::c_int as s16;
        (*this).unk_1A4 = 1000 as libc::c_int as s16;
        (*this).unk_1A2 = 100 as libc::c_int as s16;
        (*this).unk_17A[1 as libc::c_int as usize] = 50 as libc::c_int as s16
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_80B71278(mut this: *mut Fishing,
                                       mut arg1: u8_0) {
    let mut sfxId: s16 = 0;
    let mut temp: u8_0 = 0;
    if (*this).unk_150 as libc::c_int == 0 as libc::c_int {
        temp = (*this).unk_1AC as u8_0
    } else { temp = (2.0f32 * (*this).unk_1AC) as u8_0 }
    if arg1 as libc::c_int == 0 as libc::c_int {
        if temp as libc::c_int >= 50 as libc::c_int {
            sfxId = 0x2889 as libc::c_int as s16
        } else if temp as libc::c_int >= 40 as libc::c_int {
            sfxId = 0x2817 as libc::c_int as s16
        } else { sfxId = 0x2817 as libc::c_int as s16 }
    } else if temp as libc::c_int >= 50 as libc::c_int {
        sfxId = 0x288a as libc::c_int as s16
    } else if temp as libc::c_int >= 40 as libc::c_int {
        sfxId = 0x2808 as libc::c_int as s16
    } else { sfxId = 0x2808 as libc::c_int as s16 }
    Audio_PlayActorSound2(&mut (*this).actor, sfxId as u16_0);
}
#[no_mangle]
pub unsafe extern "C" fn Fishing_HandleAquariumDialog(mut this: *mut Fishing,
                                                      mut globalCtx:
                                                          *mut GlobalContext) {
    if sLinkAge as libc::c_int == 1 as libc::c_int {
        if gSaveContext.highScores[HS_FISHING as libc::c_int as usize] &
               0x7f as libc::c_int != 0 as libc::c_int {
            if gSaveContext.highScores[HS_FISHING as libc::c_int as usize] &
                   0x80 as libc::c_int != 0 {
                (*this).actor.textId = 0x40b1 as libc::c_int as u16_0
            } else { (*this).actor.textId = 0x4089 as libc::c_int as u16_0 }
        } else { (*this).actor.textId = 0x40ae as libc::c_int as u16_0 }
    } else if gSaveContext.highScores[HS_FISHING as libc::c_int as usize] &
                  0x7f000000 as libc::c_int != 0 as libc::c_int {
        if gSaveContext.highScores[HS_FISHING as libc::c_int as usize] as
               libc::c_uint & 0x80000000 as libc::c_uint != 0 {
            (*this).actor.textId = 0x40b1 as libc::c_int as u16_0
        } else { (*this).actor.textId = 0x4089 as libc::c_int as u16_0 }
    } else { (*this).actor.textId = 0x40ae as libc::c_int as u16_0 }
    if (*this).unk_1D3 as libc::c_int == 0 as libc::c_int {
        if (*this).unk_1D4 as libc::c_int == 0 as libc::c_int {
            (*this).actor.flags |=
                ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
            if Actor_ProcessTalkRequest(&mut (*this).actor, globalCtx) != 0 {
                D_80B7A678 = D_80B7E078 as u16_0;
                (*this).unk_1D3 = 1 as libc::c_int as u8_0
            } else { func_8002F2F4(&mut (*this).actor, globalCtx); }
        } else {
            (*this).unk_1D4 = (*this).unk_1D4.wrapping_sub(1);
            (*this).actor.flags &=
                !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
        }
    } else if Actor_TextboxIsClosing(&mut (*this).actor, globalCtx) != 0 {
        (*this).unk_1D3 = 0 as libc::c_int as u8_0;
        (*this).unk_1D4 = 20 as libc::c_int as u8_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn Fishing_UpdateFish(mut thisx: *mut Actor,
                                            mut globalCtx2:
                                                *mut GlobalContext) {
    let mut i: s16 = 0;
    let mut sp134: s16 = 10 as libc::c_int as s16;
    let mut sp130: f32_0 = 0.;
    let mut sp12C: f32_0 = 0.;
    let mut sp128: f32_0 = 0.;
    let mut sp124: f32_0 = 0.;
    let mut multiplier: f32_0 = 0.;
    let mut sp11C: f32_0 = 0.;
    let mut sp118: f32_0 = 0.;
    let mut sp10C: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp100: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut spFE: s16 = 0;
    let mut spFC: s16 = 0;
    let mut spFA: s16 = 0;
    let mut phi_v0: s16 = 0;
    let mut spF6: s16 = 0;
    let mut spF4: s16 = 0;
    let mut spF2: s16 = 0;
    let mut spF0: s16 = 0;
    let mut spEE: s16 = 0;
    let mut this: *mut Fishing = thisx as *mut Fishing;
    let mut globalCtx: *mut GlobalContext = globalCtx2;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut input: *mut Input =
        &mut *(*globalCtx).state.input.as_mut_ptr().offset(0 as libc::c_int as
                                                               isize) as
            *mut Input;
    let mut spD8: f32_0 = 0.;
    let mut phi_f0: f32_0 = 0.;
    let mut phi_f2: f32_0 = 0.;
    let mut spC4: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut spB8: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut phi_v0_2: u8_0 = 0;
    let mut temp_f0: f32_0 = 0.;
    let mut temp: f32_0 = 0.;
    let mut pad: s32 = 0;
    let mut spA4: f32_0 = 0.;
    let mut spA2: u16_0 = 0;
    let mut phi_a1: u8_0 = 0;
    (*this).actor.uncullZoneForward = 700.0f32;
    (*this).actor.uncullZoneScale = 50.0f32;
    if (*this).unk_150 as libc::c_int == 0 as libc::c_int {
        sp118 = (*player).actor.speedXZ * 0.15f32 + 0.25f32
    } else { sp118 = (*player).actor.speedXZ * 0.3f32 + 0.25f32 }
    if D_80B7E0B0 as libc::c_int != 0 as libc::c_int ||
           sCameraId as libc::c_int != 0 as libc::c_int ||
           (*player).actor.world.pos.z > 1150.0f32 &&
               (*this).unk_158 as libc::c_int != 100 as libc::c_int {
        (*this).actor.flags &=
            !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
    } else {
        (*this).actor.flags |=
            ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
        if D_80B7A694 as libc::c_int != 0 as libc::c_int {
            if D_80B7E0B2 as libc::c_int == 0 as libc::c_int {
                (*this).actor.focus.pos = sLurePos
            } else if D_80B7E0B2 as libc::c_int == 1 as libc::c_int {
                D_80B7A6CC = 1 as libc::c_int as u8_0;
                D_80B7FED0 = 0.0f32;
                D_80B7E088 = 2 as libc::c_int as s8
            }
        }
        (*this).actor.focus.pos = (*this).actor.world.pos
    }
    (*this).unk_15C += 1;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < 4 as libc::c_int {
        if (*this).unk_17A[i as usize] as libc::c_int != 0 as libc::c_int {
            (*this).unk_17A[i as usize] -= 1
        }
        i += 1
    }
    if (*this).unk_1A4 as libc::c_int != 0 as libc::c_int {
        (*this).unk_1A4 -= 1
    }
    if (*this).unk_1A2 as libc::c_int != 0 as libc::c_int {
        (*this).unk_1A2 -= 1
    }
    if (*this).unk_1A0 as libc::c_int != 0 as libc::c_int {
        (*this).unk_1A0 -= 1
    }
    if (*this).unk_151 as libc::c_int != 0 as libc::c_int {
        (*this).unk_151 = (*this).unk_151.wrapping_sub(1)
    }
    Math_ApproachF(&mut (*this).unk_198, (*this).unk_190, 1.0f32, 0.2f32);
    if (*this).unk_158 as libc::c_int == 6 as libc::c_int {
        Math_ApproachF(&mut (*this).unk_19C, (*this).unk_194, 0.2f32,
                       200.0f32);
    } else {
        phi_f0 = 1.0f32;
        phi_f2 = 1.0f32;
        if (*this).actor.world.pos.y >
               (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface as
                   libc::c_int as libc::c_float {
            phi_f0 =
                (*gGameInfo).data[(13 as libc::c_int * 6 as libc::c_int *
                                       16 as libc::c_int + 64 as libc::c_int)
                                      as usize] as libc::c_int as
                    libc::c_float * 0.1f32 + 1.5f32;
            phi_f2 = 3.0f32
        }
        Math_ApproachF(&mut (*this).unk_19C, (*this).unk_194 * phi_f0, 1.0f32,
                       500.0f32 * phi_f2);
    }
    Math_ApproachS(&mut (*this).unk_170, 0 as libc::c_int as s16,
                   5 as libc::c_int as s16, 0x1f4 as libc::c_int as s16);
    if (*this).unk_150 as libc::c_int == 0 as libc::c_int {
        Actor_SetScale(&mut (*this).actor,
                       (*this).unk_1AC * 15.0f32 * 0.00001f32);
        (*this).unk_18C += (*this).unk_198;
        temp = cosf((*this).unk_18C);
        (*this).unk_16C =
            ((*this).unk_16E as libc::c_int +
                 (temp * (*this).unk_19C) as s16 as libc::c_int) as s16;
        temp = cosf((*this).unk_18C + -1.2f32);
        (*this).unk_176 =
            ((*this).unk_16E as libc::c_int +
                 (temp * (*this).unk_19C * 1.6f32) as s16 as libc::c_int) as
                s16
    } else {
        Actor_SetScale(&mut (*this).actor,
                       (*this).unk_1AC * 65.0f32 * 0.000001f32);
        (*this).actor.scale.x = (*this).actor.scale.z * 1.1f32;
        (*this).actor.scale.y = (*this).actor.scale.z * 1.1f32;
        (*this).unk_18C += (*this).unk_198 * 0.8f32;
        i = 0 as libc::c_int as s16;
        while (i as libc::c_int) < 3 as libc::c_int {
            temp =
                cosf((*this).unk_18C +
                         i as libc::c_int as libc::c_float * 2.1f32);
            (*this).unk_1CC[i as usize] =
                ((*this).unk_16E as libc::c_int +
                     (temp * (*this).unk_19C * 2.0f32) as s16 as libc::c_int)
                    as s16;
            i += 1
        }
        temp = cosf((*this).unk_18C + 0.4f32);
        (*this).unk_16C = ((*this).unk_19C * temp * 2.0f32 * 0.6f32) as s16
    }
    sp130 = (*this).unk_1B4.x - (*this).actor.world.pos.x;
    sp12C = (*this).unk_1B4.y - (*this).actor.world.pos.y;
    sp128 = (*this).unk_1B4.z - (*this).actor.world.pos.z;
    spFC = Math_Atan2S(sp128, sp130);
    sp124 = sqrtf(sp130 * sp130 + sp128 * sp128);
    spFE = Math_Atan2S(sp124, sp12C);
    sp124 = sqrtf(sp130 * sp130 + sp128 * sp128 + sp12C * sp12C);
    if (*this).unk_1A0 as libc::c_int != 0 as libc::c_int &&
           (*this).unk_158 as libc::c_int != 2 as libc::c_int &&
           (*this).unk_158 as libc::c_int != 3 as libc::c_int &&
           (*this).unk_158 as libc::c_int != 4 as libc::c_int {
        if (*this).unk_15C as libc::c_int & 0x40 as libc::c_int !=
               0 as libc::c_int {
            spFC = (spFC as libc::c_int + 0x4000 as libc::c_int) as s16
        } else { spFC = (spFC as libc::c_int - 0x4000 as libc::c_int) as s16 }
        if (*this).unk_15C as libc::c_int + 0x20 as libc::c_int &
               0x40 as libc::c_int != 0 as libc::c_int {
            spFE = (spFE as libc::c_int + 0x2000 as libc::c_int) as s16
        } else { spFE = (spFE as libc::c_int - 0x2000 as libc::c_int) as s16 }
    }
    let mut current_block_906: u64;
    match (*this).unk_158 as libc::c_int {
        100 => {
            Fishing_HandleAquariumDialog(this, globalCtx);
            (*this).actor.uncullZoneForward = 500.0f32;
            (*this).actor.uncullZoneScale = 300.0f32;
            Lights_PointNoGlowSetInfo(&mut (*this).lightInfo,
                                      (*this).actor.world.pos.x as s16,
                                      ((*this).actor.world.pos.y as s16 as
                                           libc::c_int as libc::c_float +
                                           20.0f32) as s16,
                                      ((*this).actor.world.pos.z as s16 as
                                           libc::c_int as libc::c_float -
                                           50.0f32) as s16,
                                      255 as libc::c_int as u8_0,
                                      255 as libc::c_int as u8_0,
                                      255 as libc::c_int as u8_0,
                                      255 as libc::c_int as s16);
            (*this).unk_1AC = D_80B7E078;
            sp100.y =
                Math_SinS((*globalCtx).gameplayFrames.wrapping_mul(300 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint)
                              as s16) * 1 as libc::c_int as libc::c_float;
            sp100.z =
                Math_SinS((*globalCtx).gameplayFrames.wrapping_mul(230 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint)
                              as s16) * 2 as libc::c_int as libc::c_float;
            (*this).actor.world.pos.x = 130.0f32;
            (*this).actor.world.pos.y = 55.0f32 + sp100.y;
            (*this).actor.world.pos.z = 1300.0f32 + sp100.z;
            (*this).actor.shape.rot.y = -(0x8000 as libc::c_int) as s16;
            if (*this).actor.projectedPos.z < 200.0f32 &&
                   (*this).actor.projectedPos.z > 0.0f32 {
                spC4.x = Rand_CenteredFloat(5.0f32) + 130.0f32;
                spC4.y = 40.0f32;
                spC4.z = Rand_CenteredFloat(5.0f32) + 1280.0f32;
                Fishing_SpawnBubble(0 as *mut Vec3f,
                                    (*globalCtx).specialEffects as
                                        *mut FishingEffect, &mut spC4,
                                    Rand_ZeroFloat(0.02f32) + 0.03f32,
                                    1 as libc::c_int as u8_0);
            }
            Math_ApproachS(&mut (*this).unk_172,
                           (Math_SinS(((*this).unk_15C as libc::c_int *
                                           0x800 as libc::c_int) as s16) *
                                2500.0f32 + 2500.0f32) as s16,
                           2 as libc::c_int as s16,
                           0x7d0 as libc::c_int as s16);
            Math_ApproachS(&mut (*this).unk_174,
                           (Math_SinS(((*this).unk_15C as libc::c_int *
                                           0xa00 as libc::c_int) as s16) *
                                1500.0f32) as s16, 2 as libc::c_int as s16,
                           0x7d0 as libc::c_int as s16);
            (*this).unk_190 = 0.3f32;
            (*this).unk_194 = 1000.0f32 / 3.0f32;
            return
        }
        10 => {
            (*this).unk_1B4 = (*this).actor.home.pos;
            Math_ApproachF(&mut (*this).actor.speedXZ, 2.0f32, 1.0f32,
                           0.5f32);
            Math_ApproachF(&mut (*this).unk_1B0, 4096.0f32, 1.0f32, 256.0f32);
            if sp124 < 40.0f32 {
                (*this).unk_158 = 11 as libc::c_int as s16;
                (*this).unk_190 = 0.4f32;
                (*this).unk_194 = 500.0f32
            }
            func_80B70ED4(this, input);
            if (*this).actor.xzDistToPlayer < 250.0f32 * sp118 {
                (*this).unk_158 = 0 as libc::c_int as s16;
                (*this).unk_15A = (*this).unk_158;
                (*this).unk_1A4 = 1000 as libc::c_int as s16;
                (*this).unk_1A2 = 200 as libc::c_int as s16;
                (*this).unk_17A[1 as libc::c_int as usize] =
                    50 as libc::c_int as s16
            }
            current_block_906 = 7001999846415694221;
        }
        11 => {
            (*this).unk_1B4 = (*this).actor.home.pos;
            Math_ApproachF(&mut (*this).actor.speedXZ, 0.0f32, 1.0f32,
                           0.05f32);
            Math_ApproachF(&mut (*this).unk_1B0, 0.0f32, 1.0f32, 256.0f32);
            if sp124 >= 40.0f32 {
                (*this).unk_158 = 10 as libc::c_int as s16;
                (*this).unk_190 = 1.0f32;
                (*this).unk_194 = 2000.0f32
            }
            func_80B70ED4(this, input);
            if (*this).actor.xzDistToPlayer < 250.0f32 * sp118 {
                (*this).unk_158 = 0 as libc::c_int as s16;
                (*this).unk_15A = (*this).unk_158;
                (*this).unk_1A4 = 1000 as libc::c_int as s16;
                (*this).unk_1A2 = 200 as libc::c_int as s16;
                (*this).unk_17A[1 as libc::c_int as usize] =
                    50 as libc::c_int as s16
            }
            if Message_GetState(&mut (*globalCtx).msgCtx) as libc::c_int ==
                   TEXT_STATE_NONE as libc::c_int {
                if gSaveContext.dayTime as libc::c_int >=
                       0xc000 as libc::c_int &&
                       gSaveContext.dayTime as libc::c_int <=
                           0xc01b as libc::c_int {
                    (*this).unk_158 = 7 as libc::c_int as s16;
                    (*this).unk_17A[3 as libc::c_int as usize] =
                        (Rand_ZeroFloat(150.0f32) as s16 as libc::c_int +
                             200 as libc::c_int) as s16
                }
                if gSaveContext.dayTime as libc::c_int >=
                       0x3aaa as libc::c_int &&
                       gSaveContext.dayTime as libc::c_int <=
                           0x3ac5 as libc::c_int {
                    (*this).unk_158 = 7 as libc::c_int as s16;
                    (*this).unk_17A[3 as libc::c_int as usize] =
                        (Rand_ZeroFloat(150.0f32) as s16 as libc::c_int +
                             200 as libc::c_int) as s16
                }
            }
            if (*gGameInfo).data[(13 as libc::c_int * 6 as libc::c_int *
                                      16 as libc::c_int + 15 as libc::c_int)
                                     as usize] as libc::c_int !=
                   0 as libc::c_int {
                (*gGameInfo).data[(13 as libc::c_int * 6 as libc::c_int *
                                       16 as libc::c_int + 15 as libc::c_int)
                                      as usize] = 0 as libc::c_int as s16;
                (*this).unk_158 = 7 as libc::c_int as s16;
                (*this).unk_17A[3 as libc::c_int as usize] =
                    (Rand_ZeroFloat(150.0f32) as s16 as libc::c_int +
                         2000 as libc::c_int) as s16
            }
            current_block_906 = 7001999846415694221;
        }
        0 => {
            Math_ApproachF(&mut (*this).actor.speedXZ, 1.0f32, 1.0f32,
                           0.05f32);
            Math_ApproachF(&mut (*this).unk_1B0, 0.0f32, 1.0f32, 256.0f32);
            if (*this).unk_17A[0 as libc::c_int as usize] as libc::c_int ==
                   0 as libc::c_int {
                if (*this).unk_1A4 as libc::c_int == 0 as libc::c_int {
                    (*this).unk_15A = 10 as libc::c_int as s16;
                    (*this).unk_158 = (*this).unk_15A
                } else {
                    (*this).unk_158 = 1 as libc::c_int as s16;
                    (*this).unk_17A[0 as libc::c_int as usize] =
                        (Rand_ZeroFloat(30.0f32) as s16 as libc::c_int +
                             10 as libc::c_int) as s16;
                    (*this).unk_1B4.x = Rand_CenteredFloat(300.0f32);
                    (*this).unk_1B4.y =
                        (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface
                            as libc::c_int as libc::c_float - 50.0f32 -
                            Rand_ZeroFloat(50.0f32);
                    (*this).unk_1B4.z = Rand_CenteredFloat(300.0f32);
                    (*this).unk_190 = 1.0f32;
                    (*this).unk_194 = 2000.0f32
                }
            }
            if D_80B7E0B6 as libc::c_int == 2 as libc::c_int {
                func_80B70ED4(this, input);
            } else {
                (*this).actor.flags &=
                    !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
            }
            current_block_906 = 7001999846415694221;
        }
        1 => {
            if (*this).unk_150 as libc::c_int == 1 as libc::c_int {
                (*this).unk_158 = -(1 as libc::c_int) as s16;
                (*this).unk_1A4 = 20000 as libc::c_int as s16;
                (*this).unk_1A2 = 20000 as libc::c_int as s16;
                (*this).unk_1B4.x = 0.0f32;
                (*this).unk_1B4.y = -140.0f32;
                (*this).unk_1B4.z = 0.0f32
            } else {
                Math_ApproachF(&mut (*this).unk_1B0, 4096.0f32, 1.0f32,
                               256.0f32);
                if (*this).actor.xzDistToPlayer < 250.0f32 * sp118 ||
                       (*this).unk_17A[1 as libc::c_int as usize] as
                           libc::c_int != 0 as libc::c_int {
                    Math_ApproachF(&mut (*this).unk_1B0, 8192.0f32, 1.0f32,
                                   768.0f32);
                    Math_ApproachF(&mut (*this).actor.speedXZ, 4.2f32, 1.0f32,
                                   0.75f64 as f32_0);
                    (*this).unk_190 = 1.2f32;
                    (*this).unk_194 = 4000.0f32;
                    (*this).unk_17A[0 as libc::c_int as usize] =
                        20 as libc::c_int as s16
                } else {
                    (*this).unk_190 = 1.0f32;
                    (*this).unk_194 = 2000.0f32;
                    Math_ApproachF(&mut (*this).actor.speedXZ, 1.5f32, 1.0f32,
                                   0.1f32);
                }
                if (*this).unk_17A[0 as libc::c_int as usize] as libc::c_int
                       == 0 as libc::c_int || sp124 < 50.0f32 {
                    (*this).unk_158 = 0 as libc::c_int as s16;
                    (*this).unk_17A[0 as libc::c_int as usize] =
                        (Rand_ZeroFloat(30.0f32) as s16 as libc::c_int +
                             3 as libc::c_int) as s16;
                    (*this).unk_190 = 1.0f32;
                    (*this).unk_194 = 500.0f32
                }
                if D_80B7E0B6 as libc::c_int == 2 as libc::c_int {
                    func_80B70ED4(this, input);
                } else {
                    (*this).actor.flags &=
                        !((1 as libc::c_int) << 0 as libc::c_int) as
                            libc::c_uint
                }
            }
            current_block_906 = 7001999846415694221;
        }
        -1 => {
            Math_ApproachS(&mut (*this).unk_166, 0 as libc::c_int as s16,
                           0x14 as libc::c_int as s16,
                           0x20 as libc::c_int as s16);
            if (*this).actor.xzDistToPlayer < 250.0f32 * sp118 ||
                   (*this).unk_17A[1 as libc::c_int as usize] as libc::c_int
                       != 0 as libc::c_int {
                Math_ApproachF(&mut (*this).actor.speedXZ, 3.0f32, 1.0f32,
                               0.75f64 as f32_0);
                (*this).unk_190 = 1.0f32;
                (*this).unk_17A[0 as libc::c_int as usize] =
                    20 as libc::c_int as s16;
                (*this).unk_194 = 4000.0f32;
                Math_ApproachF(&mut (*this).unk_1B0, 4096.0f32, 1.0f32,
                               256.0f32);
                if (*globalCtx).gameplayFrames.wrapping_rem(32 as libc::c_int
                                                                as
                                                                libc::c_uint)
                       == 0 as libc::c_int as libc::c_uint {
                    (*this).unk_1B4.x = Rand_CenteredFloat(600.0f32);
                    (*this).unk_1B4.z = Rand_CenteredFloat(600.0f32);
                    (*this).unk_1B4.y = -120.0f32
                }
            } else if sp124 > 50.0f32 {
                (*this).unk_190 = 0.8f32;
                (*this).unk_194 = 1500.0f32;
                Math_ApproachF(&mut (*this).actor.speedXZ, 1.0f32, 1.0f32,
                               0.1f32);
                Math_ApproachF(&mut (*this).unk_1B0, 2048.0f32, 1.0f32,
                               128.0f32);
            } else {
                (*this).unk_190 = 0.4f32;
                (*this).unk_194 = 500.0f32;
                Math_ApproachZeroF(&mut (*this).actor.speedXZ, 1.0f32,
                                   0.02f32);
                Math_ApproachF(&mut (*this).unk_1B0, 0.0f32, 1.0f32,
                               256.0f32);
            }
            if (*this).unk_1A4 as libc::c_int == 0 as libc::c_int {
                (*this).unk_158 = 10 as libc::c_int as s16;
                (*this).unk_15A = 10 as libc::c_int as s16
            } else if (*gGameInfo).data[(13 as libc::c_int * 6 as libc::c_int
                                             * 16 as libc::c_int +
                                             2 as libc::c_int) as usize] as
                          libc::c_int != 0 as libc::c_int ||
                          (*this).unk_1A4 as libc::c_int &
                              0x7ff as libc::c_int == 0 as libc::c_int &&
                              ((*this).unk_1A4 as libc::c_int) <
                                  15000 as libc::c_int {
                (*gGameInfo).data[(13 as libc::c_int * 6 as libc::c_int *
                                       16 as libc::c_int + 2 as libc::c_int)
                                      as usize] = 0 as libc::c_int as s16;
                (*this).unk_158 = -(2 as libc::c_int) as s16;
                (*this).actor.shape.rot.x = 0 as libc::c_int as s16;
                (*this).actor.world.rot.x = (*this).actor.shape.rot.x;
                (*this).unk_1B4.y =
                    (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface as
                        libc::c_int as libc::c_float + 10.0f32;
                (*this).unk_1B4.x = Rand_ZeroFloat(50.0f32);
                (*this).unk_1B4.z = Rand_ZeroFloat(50.0f32)
            }
            (*this).actor.flags &=
                !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
            current_block_906 = 7001999846415694221;
        }
        -2 => {
            if (*this).actor.xzDistToPlayer < 250.0f32 * sp118 ||
                   (*this).unk_17A[1 as libc::c_int as usize] as libc::c_int
                       != 0 as libc::c_int {
                (*this).unk_158 = -(1 as libc::c_int) as s16;
                (*this).unk_1B4.y = -120.0f32
            } else {
                (*this).unk_190 = 0.6f32;
                (*this).unk_194 = 1000.0f32;
                Math_ApproachS(&mut (*this).unk_166,
                               -(0x1000 as libc::c_int) as s16,
                               0x14 as libc::c_int as s16,
                               0x100 as libc::c_int as s16);
                if (*this).actor.world.pos.y <
                       (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface
                           as libc::c_int as libc::c_float - 20.0f32 {
                    Math_ApproachF(&mut (*this).actor.speedXZ, 0.5f32, 1.0f32,
                                   0.1f32);
                    current_block_906 = 6660696667549920226;
                } else {
                    Math_ApproachZeroF(&mut (*this).actor.speedXZ, 1.0f32,
                                       0.01f32);
                    if (*this).actor.speedXZ == 0.0f32 ||
                           (*this).actor.world.pos.y >
                               (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface
                                   as libc::c_int as libc::c_float - 5.0f32 {
                        (*this).unk_1B4.x = Rand_ZeroFloat(300.0f32);
                        (*this).unk_1B4.z = Rand_ZeroFloat(300.0f32);
                        (*this).unk_1B4.y =
                            (*this).actor.floorHeight + 10.0f32;
                        (*this).unk_158 = -(25 as libc::c_int) as s16;
                        (*this).unk_1B0 = 0.0f32;
                        spB8 = (*this).fishMouthPos;
                        spB8.y =
                            (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface
                                as f32_0;
                        Fishing_SpawnRipple(&mut (*this).actor.projectedPos,
                                            (*globalCtx).specialEffects as
                                                *mut FishingEffect, &mut spB8,
                                            10.0f32, 300.0f32,
                                            150 as libc::c_int as s16,
                                            90 as libc::c_int as s16);
                        Fishing_SpawnRipple(&mut (*this).actor.projectedPos,
                                            (*globalCtx).specialEffects as
                                                *mut FishingEffect, &mut spB8,
                                            30.0f32, 400.0f32,
                                            150 as libc::c_int as s16,
                                            90 as libc::c_int as s16);
                        Audio_PlayActorSound2(&mut (*this).actor,
                                              0x836 as libc::c_int as u16_0);
                        current_block_906 = 7001999846415694221;
                    } else { current_block_906 = 6660696667549920226; }
                }
                match current_block_906 {
                    7001999846415694221 => { }
                    _ => {
                        Math_ApproachF(&mut (*this).unk_1B0, 2048.0f32,
                                       1.0f32, 128.0f32);
                        (*this).actor.flags &=
                            !((1 as libc::c_int) << 0 as libc::c_int) as
                                libc::c_uint
                    }
                }
            }
            current_block_906 = 7001999846415694221;
        }
        -25 => {
            if (*this).actor.xzDistToPlayer < 250.0f32 * sp118 ||
                   (*this).unk_17A[1 as libc::c_int as usize] as libc::c_int
                       != 0 as libc::c_int {
                (*this).unk_158 = -(1 as libc::c_int) as s16;
                (*this).unk_1B4.y = -120.0f32
            } else {
                Math_ApproachS(&mut (*this).unk_166,
                               0x1000 as libc::c_int as s16,
                               0x14 as libc::c_int as s16,
                               0x6a as libc::c_int as s16);
                if sp124 > 40.0f32 {
                    (*this).unk_190 = 0.7f32;
                    (*this).unk_194 = 1200.0f32;
                    Math_ApproachF(&mut (*this).actor.speedXZ, 0.5f32, 1.0f32,
                                   0.01f32);
                    Math_ApproachF(&mut (*this).unk_1B0, 2048.0f32, 1.0f32,
                                   128.0f32);
                } else { (*this).unk_158 = -(1 as libc::c_int) as s16 }
            }
            current_block_906 = 7001999846415694221;
        }
        2 => {
            if (*this).actor.params as libc::c_int + D_80B7E118 as libc::c_int
                   & 1 as libc::c_int != 0 as libc::c_int {
                sp10C.x = 10.0f32
            } else { sp10C.x = -10.0f32 }
            sp10C.y = 0.0f32;
            sp10C.z = 0.0f32;
            Matrix_RotateY(sLureRot.y, MTXMODE_NEW as libc::c_int as u8_0);
            Matrix_MultVec3f(&mut sp10C, &mut sp100);
            (*this).unk_1B4.x = sLurePos.x + sp100.x;
            (*this).unk_1B4.z = sLurePos.z + sp100.z;
            if D_80B7E0B6 as libc::c_int == 2 as libc::c_int {
                (*this).unk_1B4.y = sLurePos.y
            } else if (*this).unk_150 as libc::c_int == 0 as libc::c_int {
                (*this).unk_1B4.y = sLurePos.y - 15.0f32
            } else { (*this).unk_1B4.y = sLurePos.y - 5.0f32 }
            if (*this).unk_1B4.y <= (*this).actor.floorHeight {
                (*this).unk_1B4.y = (*this).actor.floorHeight + 3.0f32
            }
            if D_80B7E0B6 as libc::c_int != 2 as libc::c_int &&
                   (*this).unk_1B4.y < (*this).actor.world.pos.y {
                Math_ApproachF(&mut (*this).actor.world.pos.y,
                               (*this).unk_1B4.y, 0.1f32,
                               ((*this).actor.world.pos.y - (*this).unk_1B4.y)
                                   * 0.1f32);
            }
            Math_ApproachF(&mut (*this).unk_1B0, 8192.0f32, 1.0f32,
                           ((*gGameInfo).data[(13 as libc::c_int *
                                                   6 as libc::c_int *
                                                   16 as libc::c_int +
                                                   16 as libc::c_int) as
                                                  usize] as libc::c_int *
                                128 as libc::c_int) as libc::c_float +
                               384.0f32);
            if !((*input).press.button as libc::c_int |
                     !(0x8000 as libc::c_int)) == 0 as libc::c_int {
                (*this).unk_1A8 += 0.005f32
            }
            if D_80B7E120 as libc::c_int != 0 as libc::c_int {
                if D_80B7E120 as libc::c_int == 1 as libc::c_int {
                    (*this).unk_1A8 += 0.01f32
                } else { (*this).unk_1A8 += 0.05f32 }
                D_80B7E120 = 0 as libc::c_int as u8_0
            }
            if !((*input).press.button as libc::c_int |
                     !(0x4000 as libc::c_int)) == 0 as libc::c_int {
                (*this).unk_1A8 += 0.008f32
            }
            if sp124 < (*this).unk_1AC * 0.5f32 + 20.0f32 {
                if (*this).unk_15E as libc::c_int == 0 as libc::c_int {
                    (*this).unk_190 = 1.0f32;
                    (*this).unk_194 = 500.0f32;
                    (*this).unk_17A[0 as libc::c_int as usize] =
                        (Rand_ZeroFloat(10.0f32) as s16 as libc::c_int +
                             2 as libc::c_int) as s16
                }
                Math_ApproachF(&mut (*this).actor.speedXZ, -0.2f32, 1.0f32,
                               0.1f32);
                (*this).unk_15E = 1 as libc::c_int as s16
            } else {
                if (*this).unk_15E as libc::c_int != 0 as libc::c_int {
                    (*this).unk_190 = 1.0f32;
                    (*this).unk_1B0 = 0.0f32;
                    (*this).unk_194 = 3000.0f32
                }
                Math_ApproachF(&mut (*this).actor.speedXZ, 3.0f32, 1.0f32,
                               0.15f32);
                (*this).unk_15E = 0 as libc::c_int as s16
            }
            if (*this).unk_1AC >= 60.0f32 {
                multiplier = 0.3f32
            } else if (*this).unk_1AC >= 45.0f32 {
                multiplier = 0.6f32
            } else { multiplier = 1.0f32 }
            if gSaveContext.dayTime as libc::c_int >= 0xb555 as libc::c_int &&
                   gSaveContext.dayTime as libc::c_int <=
                       0xcaaa as libc::c_int {
                multiplier *= 1.75f32
            } else if gSaveContext.dayTime as libc::c_int >=
                          0x3555 as libc::c_int &&
                          gSaveContext.dayTime as libc::c_int <=
                              0x4aaa as libc::c_int {
                multiplier *= 1.5f32
            } else if D_80B7E076 as libc::c_int != 0 as libc::c_int {
                multiplier *= 1.5f32
            } else if D_80B7A650 as u8_0 as libc::c_int != 0 as libc::c_int {
                multiplier *= 3.0f32
            }
            sp11C = 0.03f32 * multiplier;
            if D_80B7E0B6 as libc::c_int == 2 as libc::c_int {
                sp11C *= 5.0f32
            }
            if ((*this).unk_17A[0 as libc::c_int as usize] as libc::c_int ==
                    1 as libc::c_int || Rand_ZeroOne() < sp11C) &&
                   (Rand_ZeroOne() < (*this).unk_1A8 * multiplier ||
                        (*this).unk_150 as libc::c_int + 1 as libc::c_int ==
                            (*gGameInfo).data[(13 as libc::c_int *
                                                   6 as libc::c_int *
                                                   16 as libc::c_int +
                                                   69 as libc::c_int) as
                                                  usize] as libc::c_int) {
                if (*this).unk_150 as libc::c_int == 0 as libc::c_int {
                    (*this).unk_158 = 3 as libc::c_int as s16;
                    (*this).unk_190 = 1.2f32;
                    (*this).unk_194 = 5000.0f32;
                    (*this).unk_17A[0 as libc::c_int as usize] =
                        Rand_ZeroFloat(10.0f32) as s16
                } else {
                    (*this).unk_158 = -(3 as libc::c_int) as s16;
                    (*this).unk_190 = 1.0f32;
                    (*this).unk_194 = 3000.0f32;
                    (*this).unk_17A[0 as libc::c_int as usize] =
                        40 as libc::c_int as s16
                }
                if D_80B7E0B6 as libc::c_int == 2 as libc::c_int {
                    (*this).unk_188 = Rand_ZeroFloat(1.5f32) + 3.0f32
                } else { (*this).unk_188 = Rand_ZeroFloat(1.5f32) + 4.5f32 }
            }
            if D_80B7A694 as libc::c_int != 3 as libc::c_int ||
                   (*this).unk_17A[2 as libc::c_int as usize] as libc::c_int
                       == 0 as libc::c_int ||
                   sqrtf((*this).actor.world.pos.x * (*this).actor.world.pos.x
                             +
                             (*this).actor.world.pos.z *
                                 (*this).actor.world.pos.z) > 800.0f32 {
                (*this).unk_158 = (*this).unk_15A;
                (*this).unk_17A[1 as libc::c_int as usize] =
                    (Rand_ZeroFloat(30.0f32) as s16 as libc::c_int +
                         50 as libc::c_int) as s16;
                (*this).unk_17A[0 as libc::c_int as usize] =
                    (Rand_ZeroFloat(10.0f32) as s16 as libc::c_int +
                         5 as libc::c_int) as s16;
                (*this).unk_190 = 1.0f32;
                (*this).unk_1B0 = 0.0f32;
                (*this).unk_194 = 2000.0f32
            }
            if (*this).actor.xzDistToPlayer < 100.0f32 * sp118 {
                (*this).unk_158 = 0 as libc::c_int as s16;
                (*this).unk_15A = (*this).unk_158;
                (*this).unk_1A4 = 1000 as libc::c_int as s16;
                (*this).unk_1A2 = 200 as libc::c_int as s16;
                (*this).unk_17A[1 as libc::c_int as usize] =
                    50 as libc::c_int as s16
            }
            current_block_906 = 7001999846415694221;
        }
        3 => {
            (*this).unk_151 = 6 as libc::c_int as u8_0;
            sp134 = 2 as libc::c_int as s16;
            if (*player).actor.world.pos.x as s16 as libc::c_int +
                   D_80B7E118 as libc::c_int & 1 as libc::c_int !=
                   0 as libc::c_int {
                sp10C.x = 30.0f32
            } else { sp10C.x = -30.0f32 }
            sp10C.y = 0.0f32;
            sp10C.z = 30.0f32;
            Matrix_RotateY(sLureRot.y, MTXMODE_NEW as libc::c_int as u8_0);
            Matrix_MultVec3f(&mut sp10C, &mut sp100);
            (*this).unk_1B4.x = sLurePos.x + sp100.x;
            (*this).unk_1B4.z = sLurePos.z + sp100.z;
            (*this).unk_1B4.y = sLurePos.y - 10.0f32;
            (*this).unk_1B0 = 4096.0f32;
            Math_ApproachF(&mut (*this).actor.speedXZ,
                           (*this).unk_188 * 0.8f32, 1.0f32, 1.0f32);
            if D_80B7A694 as libc::c_int != 3 as libc::c_int ||
                   sLurePos.y >
                       (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface
                           as libc::c_int as libc::c_float + 5.0f32 ||
                   sqrtf(sLurePos.x * sLurePos.x + sLurePos.z * sLurePos.z) >
                       800.0f32 {
                (*this).unk_158 = (*this).unk_15A;
                (*this).unk_17A[0 as libc::c_int as usize] =
                    0 as libc::c_int as s16;
                (*this).unk_190 = 1.0f32;
                (*this).unk_194 = 2000.0f32
            } else if (*this).unk_17A[0 as libc::c_int as usize] as
                          libc::c_int == 0 as libc::c_int || sp124 < 30.0f32 {
                (*this).unk_158 = 4 as libc::c_int as s16;
                (*this).unk_1B4 = sLurePos;
                (*this).unk_1B0 = 16384.0f32;
                (*this).unk_190 = 1.2f32;
                (*this).unk_194 = 5000.0f32;
                (*this).unk_17A[0 as libc::c_int as usize] =
                    20 as libc::c_int as s16
            }
            current_block_906 = 7001999846415694221;
        }
        4 => {
            Math_ApproachF(&mut (*this).unk_1B0, 16384.0f32, 1.0f32,
                           4096.0f32);
            Math_ApproachS(&mut (*this).unk_170, 0x4e20 as libc::c_int as s16,
                           4 as libc::c_int as s16,
                           0x1388 as libc::c_int as s16);
            (*this).unk_151 = 50 as libc::c_int as u8_0;
            sp134 = 2 as libc::c_int as s16;
            (*this).unk_1B4 = sLurePos;
            Math_ApproachF(&mut (*this).actor.speedXZ, (*this).unk_188,
                           1.0f32, 1.0f32);
            if D_80B7A694 as libc::c_int != 3 as libc::c_int ||
                   (*this).unk_17A[0 as libc::c_int as usize] as libc::c_int
                       == 0 as libc::c_int ||
                   sLurePos.y >
                       (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface
                           as libc::c_int as libc::c_float + 5.0f32 ||
                   sqrtf(sLurePos.x * sLurePos.x + sLurePos.z * sLurePos.z) >
                       800.0f32 {
                (*this).unk_17A[0 as libc::c_int as usize] =
                    0 as libc::c_int as s16;
                (*this).unk_158 = (*this).unk_15A;
                (*this).unk_190 = 1.0f32;
                (*this).unk_194 = 2000.0f32
            } else if sp124 < 10.0f32 {
                if func_80B70A2C(this, globalCtx, 0 as libc::c_int as u8_0) !=
                       0 {
                    func_80B71278(this, 0 as libc::c_int as u8_0);
                }
                (*this).unk_158 = 5 as libc::c_int as s16;
                (*this).unk_190 = 1.2f32;
                (*this).unk_194 = 5000.0f32;
                (*this).unk_17A[1 as libc::c_int as usize] =
                    150 as libc::c_int as s16;
                (*this).unk_17A[0 as libc::c_int as usize] =
                    0 as libc::c_int as s16;
                (*this).unk_17A[2 as libc::c_int as usize] =
                    0 as libc::c_int as s16;
                (*this).unk_17A[3 as libc::c_int as usize] =
                    120 as libc::c_int as s16;
                D_80B7A694 = 4 as libc::c_int as s16;
                sFishingHookedFish = this;
                sFishMouthOffset.y = 500.0f32 - Rand_ZeroFloat(400.0f32);
                if D_80B7E0B6 as libc::c_int == 2 as libc::c_int {
                    if (*this).unk_1AC > 70.0f32 {
                        phi_v0 =
                            (Rand_ZeroFloat(20.0f32) as s16 as libc::c_int +
                                 10 as libc::c_int) as s16
                    } else if (*this).unk_1AC > 60.0f32 {
                        phi_v0 =
                            (Rand_ZeroFloat(30.0f32) as s16 as libc::c_int +
                                 20 as libc::c_int) as s16
                    } else if (*this).unk_1AC > 50.0f32 {
                        phi_v0 =
                            (Rand_ZeroFloat(30.0f32) as s16 as libc::c_int +
                                 30 as libc::c_int) as s16
                    } else {
                        phi_v0 =
                            (Rand_ZeroFloat(40.0f32) as s16 as libc::c_int +
                                 40 as libc::c_int) as s16
                    }
                    D_80B7E122 = phi_v0;
                    D_80B7E0A4 = phi_v0;
                    func_800A9F6C(0.0f32, 60 as libc::c_int as u8_0,
                                  (phi_v0 as libc::c_int * 3 as libc::c_int)
                                      as u8_0, 10 as libc::c_int as u8_0);
                } else {
                    if (*this).unk_1AC > 70.0f32 {
                        phi_v0 =
                            (Rand_ZeroFloat(5.0f32) as s16 as libc::c_int +
                                 10 as libc::c_int) as s16
                    } else if (*this).unk_1AC > 60.0f32 {
                        phi_v0 =
                            (Rand_ZeroFloat(5.0f32) as s16 as libc::c_int +
                                 15 as libc::c_int) as s16
                    } else if (*this).unk_1AC > 50.0f32 {
                        phi_v0 =
                            (Rand_ZeroFloat(5.0f32) as s16 as libc::c_int +
                                 17 as libc::c_int) as s16
                    } else {
                        phi_v0 =
                            (Rand_ZeroFloat(5.0f32) as s16 as libc::c_int +
                                 25 as libc::c_int) as s16
                    }
                    D_80B7E122 = phi_v0;
                    D_80B7E0A4 = phi_v0;
                    func_800A9F6C(0.0f32, 180 as libc::c_int as u8_0,
                                  (phi_v0 as libc::c_int * 3 as libc::c_int)
                                      as u8_0, 10 as libc::c_int as u8_0);
                }
                D_80B7E124 = 0 as libc::c_int as u8_0;
                D_80B7E116 = 100 as libc::c_int as s16;
                D_80B7E080 = 0 as libc::c_int as s16
            }
            current_block_906 = 7001999846415694221;
        }
        -3 => {
            (*this).unk_151 = 50 as libc::c_int as u8_0;
            (*this).unk_1B4 = sLurePos;
            Math_ApproachF(&mut (*this).actor.speedXZ, 2.0f32, 1.0f32,
                           1.0f32);
            if D_80B7A694 as libc::c_int != 3 as libc::c_int ||
                   (*this).unk_17A[0 as libc::c_int as usize] as libc::c_int
                       == 0 as libc::c_int ||
                   sLurePos.y >
                       (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface
                           as libc::c_int as libc::c_float + 5.0f32 ||
                   sqrtf(sLurePos.x * sLurePos.x + sLurePos.z * sLurePos.z) >
                       800.0f32 {
                (*this).unk_17A[0 as libc::c_int as usize] =
                    0 as libc::c_int as s16;
                (*this).unk_190 = 1.0f32;
                (*this).unk_158 = (*this).unk_15A;
                (*this).unk_194 = 2000.0f32
            } else if sp124 < 10.0f32 {
                if sLurePos.y >
                       (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface
                           as libc::c_int as libc::c_float - 10.0f32 {
                    Audio_PlayActorSound2(&mut (*this).actor,
                                          0x288a as libc::c_int as u16_0);
                    func_80078884(0x836 as libc::c_int as u16_0);
                }
                func_80B70A2C(this, globalCtx, 0 as libc::c_int as u8_0);
                (*this).unk_158 = 5 as libc::c_int as s16;
                (*this).unk_190 = 1.2f32;
                (*this).unk_194 = 5000.0f32;
                (*this).unk_17A[1 as libc::c_int as usize] =
                    150 as libc::c_int as s16;
                (*this).unk_17A[0 as libc::c_int as usize] =
                    0 as libc::c_int as s16;
                (*this).unk_17A[2 as libc::c_int as usize] =
                    0 as libc::c_int as s16;
                (*this).unk_17A[3 as libc::c_int as usize] =
                    120 as libc::c_int as s16;
                D_80B7A694 = 4 as libc::c_int as s16;
                sFishingHookedFish = this;
                if D_80B7E0B6 as libc::c_int == 2 as libc::c_int {
                    D_80B7E122 = 30 as libc::c_int as s16;
                    D_80B7E0A4 = 100 as libc::c_int as s16;
                    func_800A9F6C(0.0f32, 60 as libc::c_int as u8_0,
                                  90 as libc::c_int as u8_0,
                                  10 as libc::c_int as u8_0);
                } else {
                    D_80B7E122 = 30 as libc::c_int as s16;
                    D_80B7E0A4 = 40 as libc::c_int as s16;
                    func_800A9F6C(0.0f32, 180 as libc::c_int as u8_0,
                                  90 as libc::c_int as u8_0,
                                  10 as libc::c_int as u8_0);
                }
                D_80B7E124 = 0 as libc::c_int as u8_0;
                D_80B7E116 = 100 as libc::c_int as s16;
                D_80B7E080 = 0 as libc::c_int as s16
            }
            current_block_906 = 7001999846415694221;
        }
        5 => {
            (*this).actor.uncullZoneForward = 1200.0f32;
            (*this).actor.uncullZoneScale = 200.0f32;
            D_80B7E080 += 1;
            osSyncPrintf(b"HIT FISH %dcm\n\x00" as *const u8 as
                             *const libc::c_char,
                         (*this).unk_1AC as u8_0 as libc::c_int);
            Math_ApproachS(&mut (*this).unk_170, 0x2af8 as libc::c_int as s16,
                           4 as libc::c_int as s16,
                           0xbb8 as libc::c_int as s16);
            sFishingHookedFish = this;
            Math_ApproachS(&mut (*player).actor.shape.rot.y,
                           ((*this).actor.yawTowardsPlayer as libc::c_int +
                                0x8000 as libc::c_int) as s16,
                           5 as libc::c_int as s16,
                           0x500 as libc::c_int as s16);
            if D_80B7E124 as libc::c_int == 0 as libc::c_int {
                if (D_80B7FEA0 as libc::c_int) < 20 as libc::c_int &&
                       D_80B7E0AE as libc::c_int & 3 as libc::c_int ==
                           0 as libc::c_int {
                    D_80B7FEA0 += 1
                }
            }
            if D_80B7E122 as libc::c_int != 0 as libc::c_int &&
                   D_80B7E124 as libc::c_int == 0 as libc::c_int {
                if ((*input).rel.stick_y as libc::c_int) <
                       -(50 as libc::c_int) &&
                       D_80B7A6C8 as libc::c_int > -(40 as libc::c_int) ||
                       !((*input).press.button as libc::c_int |
                             !(0x8000 as libc::c_int)) == 0 as libc::c_int {
                    if ((*input).rel.stick_y as libc::c_int) <
                           -(50 as libc::c_int) {
                        temp_f0 =
                            40.0f32 -
                                ((*this).unk_1AC - 30.0f32) * 1.333333f32;
                        if temp_f0 > 0.0f32 {
                            (*this).unk_152 = temp_f0 as u8_0;
                            (*this).unk_154 =
                                ((*this).actor.yawTowardsPlayer as libc::c_int
                                     -
                                     (*this).actor.shape.rot.y as libc::c_int)
                                    as s16;
                            (*this).unk_156 = 1 as libc::c_int as u8_0
                        }
                    }
                    (*this).unk_198 = 1.7f32;
                    (*this).unk_19C = 7000.0f32;
                    D_80B7E124 = 1 as libc::c_int as u8_0;
                    Audio_QueueSeqCmd(((SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                                           24 as libc::c_int |
                                           0x1a as libc::c_int |
                                           0x800 as libc::c_int) as u32_0);
                    D_80B7E0A6 = 0 as libc::c_int as s16;
                    if (*this).unk_150 as libc::c_int == 1 as libc::c_int {
                        spA4 = (*this).unk_1AC * 3.0f32 + 120.0f32
                    } else { spA4 = 2.0f32 * (*this).unk_1AC + 120.0f32 }
                    if spA4 > 255.0f32 { spA4 = 255.0f32 }
                    func_800A9F6C(0.0f32, spA4 as u8_0,
                                  120 as libc::c_int as u8_0,
                                  5 as libc::c_int as u8_0);
                    D_80B7E0A4 = 40 as libc::c_int as s16;
                    D_80B7FDA8 = 10 as libc::c_int as u8_0;
                    func_80078884(0x184a as libc::c_int as u16_0);
                }
            }
            if (*this).actor.world.pos.y <
                   (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface as
                       libc::c_int as libc::c_float {
                if (*this).unk_17A[1 as libc::c_int as usize] as libc::c_int >
                       30 as libc::c_int {
                    phi_v0_2 = 7 as libc::c_int as u8_0
                } else { phi_v0_2 = 0xf as libc::c_int as u8_0 }
                if (*this).unk_15C as libc::c_int & phi_v0_2 as libc::c_int ==
                       0 as libc::c_int && Rand_ZeroOne() < 0.75f32 &&
                       D_80B7E0A4 as libc::c_int == 0 as libc::c_int {
                    if (*this).unk_1AC >= 70.0f32 {
                        spA4 = 255.0f32
                    } else if (*this).unk_1AC >= 60.0f32 {
                        spA4 = 230.0f32
                    } else if (*this).unk_1AC >= 50.0f32 {
                        spA4 = 200.0f32
                    } else if (*this).unk_1AC >= 40.0f32 {
                        spA4 = 170.0f32
                    } else { spA4 = 140.0f32 }
                    if phi_v0_2 as libc::c_int == 0xf as libc::c_int {
                        spA4 *= 3.0f32 / 4.0f32
                    }
                    func_800A9F6C(0.0f32, spA4 as u8_0,
                                  (Rand_ZeroFloat(5.0f32) as s16 as
                                       libc::c_int + 10 as libc::c_int) as
                                      u8_0, 5 as libc::c_int as u8_0);
                }
                if (*this).unk_17A[1 as libc::c_int as usize] as libc::c_int >
                       30 as libc::c_int {
                    if (*this).unk_17A[0 as libc::c_int as usize] as
                           libc::c_int == 0 as libc::c_int {
                        sp10C.x = 0.0f32;
                        sp10C.y = 0.0f32;
                        sp10C.z = 200.0f32;
                        spA2 = 0 as libc::c_int as u16_0;
                        while (spA2 as libc::c_int) < 100 as libc::c_int {
                            Matrix_RotateY(Rand_CenteredFloat(3.0f32 *
                                                                  3.14159265358979323846f32
                                                                  / 4.0f32) +
                                               ((*this).actor.yawTowardsPlayer
                                                    as libc::c_int +
                                                    0x8000 as libc::c_int) as
                                                   libc::c_float / 32768.0f32
                                                   *
                                                   3.14159265358979323846f32,
                                           MTXMODE_NEW as libc::c_int as
                                               u8_0);
                            Matrix_MultVec3f(&mut sp10C, &mut sp100);
                            (*this).unk_1B4.x =
                                (*this).actor.world.pos.x + sp100.x;
                            (*this).unk_1B4.z =
                                (*this).actor.world.pos.z + sp100.z;
                            if (*this).unk_1B4.x * (*this).unk_1B4.x +
                                   (*this).unk_1B4.z * (*this).unk_1B4.z <
                                   750.0f32 * 750.0f32 {
                                break ;
                            }
                            spA2 = spA2.wrapping_add(1)
                        }
                        if Rand_ZeroOne() < 0.1f32 &&
                               (*this).unk_17A[3 as libc::c_int as usize] as
                                   libc::c_int == 0 as libc::c_int {
                            if (*this).unk_1AC >= 60.0f32 {
                                phi_a1 = 255 as libc::c_int as u8_0
                            } else if (*this).unk_1AC >= 50.0f32 {
                                phi_a1 = 200 as libc::c_int as u8_0
                            } else { phi_a1 = 180 as libc::c_int as u8_0 }
                            func_800A9F6C(0.0f32, phi_a1,
                                          90 as libc::c_int as u8_0,
                                          2 as libc::c_int as u8_0);
                            (*this).unk_17A[0 as libc::c_int as usize] =
                                20 as libc::c_int as s16;
                            (*this).unk_17A[1 as libc::c_int as usize] =
                                100 as libc::c_int as s16;
                            (*this).unk_17A[2 as libc::c_int as usize] =
                                20 as libc::c_int as s16;
                            (*this).unk_17A[3 as libc::c_int as usize] =
                                100 as libc::c_int as s16;
                            (*this).unk_1B4.y = 300.0f32;
                            D_80B7E0A4 = 0x28 as libc::c_int as s16;
                            D_80B7E116 =
                                (Rand_ZeroFloat(30.0f32) as s16 as libc::c_int
                                     + 20 as libc::c_int) as s16
                        } else {
                            (*this).unk_17A[0 as libc::c_int as usize] =
                                (Rand_ZeroFloat(10.0f32) as s16 as libc::c_int
                                     + 3 as libc::c_int) as s16;
                            (*this).unk_17A[2 as libc::c_int as usize] =
                                0 as libc::c_int as s16;
                            (*this).unk_1B4.y =
                                -70.0f32 - Rand_ZeroFloat(150.0f32)
                        }
                    }
                    if (*this).unk_17A[2 as libc::c_int as usize] as
                           libc::c_int != 0 as libc::c_int {
                        D_80B7E11C = 0.0f32;
                        (*this).unk_190 = 1.6f32;
                        (*this).unk_194 = 6000.0f32;
                        Math_ApproachF(&mut (*this).actor.speedXZ, 7.5f32,
                                       1.0f32, 1.0f32);
                        Math_ApproachS(&mut (*this).unk_170,
                                       0x4e20 as libc::c_int as s16,
                                       2 as libc::c_int as s16,
                                       0xfa0 as libc::c_int as s16);
                    } else {
                        if D_80B7E124 as libc::c_int == 0 as libc::c_int &&
                               D_80B7E0B6 as libc::c_int == 2 as libc::c_int {
                            (*this).unk_190 = 1.0f32;
                            (*this).unk_194 = 2000.0f32;
                            Math_ApproachF(&mut (*this).actor.speedXZ, 3.0f32,
                                           1.0f32, 0.2f32);
                        } else {
                            (*this).unk_190 = 1.4f32;
                            (*this).unk_194 = 5000.0f32;
                            Math_ApproachF(&mut (*this).actor.speedXZ, 5.0f32,
                                           1.0f32, 0.5f32);
                        }
                        if (*this).unk_150 as libc::c_int == 0 as libc::c_int
                           {
                            D_80B7E11C = 1.0f32 - (*this).unk_1AC * 0.00899f32
                        } else {
                            D_80B7E11C =
                                1.0f32 - (*this).unk_1AC * 0.00899f32 * 1.4f32
                        }
                    }
                } else {
                    if (*this).unk_17A[1 as libc::c_int as usize] as
                           libc::c_int & 0xf as libc::c_int ==
                           0 as libc::c_int &&
                           !((*input).cur.button as libc::c_int |
                                 !(0x8000 as libc::c_int)) == 0 as libc::c_int
                           &&
                           (!((*this).unk_1AC >= 60.0f32) ||
                                D_80B7E080 as libc::c_int >=
                                    2000 as libc::c_int) {
                        (*this).unk_152 =
                            (Rand_ZeroFloat(30.0f32) as s16 as libc::c_int +
                                 15 as libc::c_int) as u8_0;
                        (*this).unk_154 =
                            ((*this).actor.yawTowardsPlayer as libc::c_int -
                                 (*this).actor.shape.rot.y as libc::c_int) as
                                s16
                    }
                    (*this).unk_190 = 1.0f32;
                    (*this).unk_194 = 4500.0f32;
                    if (*this).unk_150 as libc::c_int == 0 as libc::c_int {
                        D_80B7E11C = 1.3f32 - (*this).unk_1AC * 0.00899f32
                    } else {
                        D_80B7E11C =
                            1.3f32 - (*this).unk_1AC * 0.00899f32 * 1.4f32
                    }
                    Math_ApproachF(&mut (*this).actor.speedXZ, 2.0f32, 1.0f32,
                                   0.5f32);
                    if (*this).unk_17A[1 as libc::c_int as usize] as
                           libc::c_int == 0 as libc::c_int {
                        (*this).unk_152 = 0 as libc::c_int as u8_0;
                        if (D_80B7E080 as libc::c_int) < 2000 as libc::c_int {
                            (*this).unk_17A[1 as libc::c_int as usize] =
                                (Rand_ZeroFloat(50.0f32) as s16 as libc::c_int
                                     + 50 as libc::c_int) as s16
                        } else if (D_80B7E080 as libc::c_int) <
                                      3000 as libc::c_int {
                            (*this).unk_17A[1 as libc::c_int as usize] =
                                (Rand_ZeroFloat(20.0f32) as s16 as libc::c_int
                                     + 30 as libc::c_int) as s16
                        } else {
                            (*this).unk_17A[1 as libc::c_int as usize] =
                                (Rand_ZeroFloat(10.0f32) as s16 as libc::c_int
                                     + 25 as libc::c_int) as s16
                        }
                    }
                }
            }
            if D_80B7E074 as libc::c_int != 0 as libc::c_int {
                D_80B7E11C = 0.0f32
            }
            if D_80B7E124 as libc::c_int != 0 ||
                   D_80B7E0B6 as libc::c_int != 2 as libc::c_int {
                if (*this).actor.speedXZ < 3.0f32 {
                    if D_80B7E0AE as libc::c_int & 8 as libc::c_int !=
                           0 as libc::c_int {
                        sp100.x = -0.8f32
                    } else { sp100.x = -0.75f32 }
                } else if D_80B7E0AE as libc::c_int & 4 as libc::c_int !=
                              0 as libc::c_int {
                    sp100.x = -0.9f32
                } else { sp100.x = -0.85f32 }
                Math_ApproachF(&mut D_80B7A6C0, 35.0f32, 0.1f32, 3.5f32);
                Math_ApproachF(&mut D_80B7A6BC, sp100.x, 0.3f32, 0.1f32);
            }
            sReelLinePos[(200 as libc::c_int - 1 as libc::c_int) as usize] =
                (*this).fishMouthPos;
            sp10C.x =
                sReelLinePos[(200 as libc::c_int - 1 as libc::c_int) as
                                 usize].x -
                    sReelLinePos[(200 as libc::c_int - 2 as libc::c_int) as
                                     usize].x;
            sp10C.y =
                sReelLinePos[(200 as libc::c_int - 1 as libc::c_int) as
                                 usize].y -
                    sReelLinePos[(200 as libc::c_int - 2 as libc::c_int) as
                                     usize].y;
            sp10C.z =
                sReelLinePos[(200 as libc::c_int - 1 as libc::c_int) as
                                 usize].z -
                    sReelLinePos[(200 as libc::c_int - 2 as libc::c_int) as
                                     usize].z;
            if sp10C.x * sp10C.x + sp10C.y * sp10C.y + sp10C.z * sp10C.z >
                   20.0f32 * 20.0f32 {
                Math_ApproachF(&mut (*this).actor.world.pos.x,
                               sReelLinePos[(200 as libc::c_int -
                                                 2 as libc::c_int) as
                                                usize].x, 0.2f32,
                               2.0f32 * ((*this).actor.speedXZ * 1.5f32));
                Math_ApproachF(&mut (*this).actor.world.pos.y,
                               sReelLinePos[(200 as libc::c_int -
                                                 2 as libc::c_int) as
                                                usize].y, 0.2f32,
                               2.0f32 * ((*this).actor.speedXZ * 1.5f32) *
                                   5.0f32 * 0.1f32);
                Math_ApproachF(&mut (*this).actor.world.pos.z,
                               sReelLinePos[(200 as libc::c_int -
                                                 2 as libc::c_int) as
                                                usize].z, 0.2f32,
                               2.0f32 * ((*this).actor.speedXZ * 1.5f32));
            }
            if !((*input).cur.button as libc::c_int |
                     !(0x8000 as libc::c_int)) == 0 as libc::c_int ||
                   ((*input).rel.stick_y as libc::c_int) <
                       -(30 as libc::c_int) {
                if (D_80B7E116 as libc::c_int) < 100 as libc::c_int {
                    D_80B7E116 += 1
                }
            } else if D_80B7E116 as libc::c_int != 0 as libc::c_int {
                D_80B7E116 -= 1
            }
            if (D_80B7A694 as libc::c_int) < 3 as libc::c_int ||
                   D_80B7E074 as libc::c_int != 0 as libc::c_int &&
                       D_80B7E080 as libc::c_int > 50 as libc::c_int ||
                   D_80B7E080 as libc::c_int >= 6000 as libc::c_int ||
                   D_80B7E122 as libc::c_int == 0 as libc::c_int &&
                       D_80B7E124 as libc::c_int == 0 as libc::c_int ||
                   D_80B7E116 as libc::c_int == 0 as libc::c_int ||
                   D_80B7E0AE as libc::c_int & 0x7f as libc::c_int ==
                       0 as libc::c_int && Rand_ZeroOne() < 0.05f32 &&
                       D_80B7E0B6 as libc::c_int != 2 as libc::c_int &&
                       (*gGameInfo).data[(13 as libc::c_int * 6 as libc::c_int
                                              * 16 as libc::c_int +
                                              69 as libc::c_int) as usize] as
                           libc::c_int == 0 as libc::c_int {
                D_80B7A67C = 20 as libc::c_int as u8_0;
                if D_80B7E122 as libc::c_int == 0 as libc::c_int &&
                       D_80B7E124 as libc::c_int == 0 as libc::c_int {
                    D_80B7E086 = 0x4081 as libc::c_int as u16_0;
                    if sLinkAge as libc::c_int == 1 as libc::c_int &&
                           gSaveContext.highScores[HS_FISHING as libc::c_int
                                                       as usize] &
                               0x400 as libc::c_int != 0 ||
                           sLinkAge as libc::c_int != 1 as libc::c_int &&
                               gSaveContext.highScores[HS_FISHING as
                                                           libc::c_int as
                                                           usize] &
                                   0x800 as libc::c_int != 0 {
                        D_80B7A67C = 0 as libc::c_int as u8_0
                    }
                } else {
                    D_80B7E086 = 0x4082 as libc::c_int as u16_0;
                    func_800A9F6C(0.0f32, 1 as libc::c_int as u8_0,
                                  3 as libc::c_int as u8_0,
                                  1 as libc::c_int as u8_0);
                    Audio_QueueSeqCmd(((0x1 as libc::c_int) <<
                                           28 as libc::c_int |
                                           (SEQ_PLAYER_BGM_MAIN as
                                                libc::c_int) <<
                                               24 as libc::c_int |
                                           0xa00ff as libc::c_int) as u32_0);
                }
                (*this).unk_15A = 0 as libc::c_int as s16;
                (*this).unk_158 = (*this).unk_15A;
                (*this).unk_1A4 = 10000 as libc::c_int as s16;
                (*this).unk_1A2 = 500 as libc::c_int as s16;
                (*this).unk_17A[1 as libc::c_int as usize] =
                    50 as libc::c_int as s16;
                (*this).unk_17A[0 as libc::c_int as usize] =
                    0 as libc::c_int as s16;
                (*this).unk_190 = 1.0f32;
                (*this).unk_194 = 3000.0f32;
                if D_80B7A694 as libc::c_int == 4 as libc::c_int {
                    D_80B7A694 = 3 as libc::c_int as s16
                }
                D_80B7E0A6 = 50 as libc::c_int as s16;
                D_80B7E11C = 0.5f32;
                (*this).unk_152 = 0 as libc::c_int as u8_0;
                current_block_906 = 7001999846415694221;
            } else if (*this).actor.xzDistToPlayer <
                          (*gGameInfo).data[(13 as libc::c_int *
                                                 6 as libc::c_int *
                                                 16 as libc::c_int +
                                                 59 as libc::c_int) as usize]
                              as libc::c_int as libc::c_float + 50.0f32 {
                (*this).unk_158 = 6 as libc::c_int as s16;
                (*this).unk_17A[0 as libc::c_int as usize] =
                    100 as libc::c_int as s16;
                (*player).unk_860 = 3 as libc::c_int as s16;
                func_800A9F6C(0.0f32, 1 as libc::c_int as u8_0,
                              3 as libc::c_int as u8_0,
                              1 as libc::c_int as u8_0);
                D_80B7E084 = D_80B7E084.wrapping_add(1);
                func_80064520(globalCtx, &mut (*globalCtx).csCtx);
                D_80B7A6CC = 100 as libc::c_int as u8_0;
                D_80B7FEC8 = 45.0f32;
                D_80B7A694 = 5 as libc::c_int as s16;
                (*this).unk_190 = 1.0f32;
                (*this).unk_194 = 500.0f32;
                (*this).unk_19C = 5000.0f32;
                if (*this).actor.world.pos.y <=
                       (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface
                           as libc::c_int as libc::c_float {
                    func_80B71278(this, 1 as libc::c_int as u8_0);
                    func_80B70A2C(this, globalCtx, 1 as libc::c_int as u8_0);
                }
                current_block_906 = 12032741234595264021;
            } else { current_block_906 = 7001999846415694221; }
        }
        6 => { current_block_906 = 12032741234595264021; }
        7 => {
            (*this).unk_151 = 50 as libc::c_int as u8_0;
            sp134 = 5 as libc::c_int as s16;
            (*this).unk_1B0 = 12288.0f32;
            if ((*this).actor.params as libc::c_int) < 104 as libc::c_int {
                (*this).unk_1B4 =
                    sGroupFishes[((*this).actor.params as libc::c_int -
                                      100 as libc::c_int) as usize].pos;
                D_80B7A898 = 1 as libc::c_int as f32_0
            } else if ((*this).actor.params as libc::c_int) <
                          108 as libc::c_int {
                (*this).unk_1B4 =
                    sGroupFishes[((*this).actor.params as libc::c_int -
                                      100 as libc::c_int + 16 as libc::c_int)
                                     as usize].pos;
                D_80B7A898 = 2 as libc::c_int as f32_0
            } else {
                (*this).unk_1B4 =
                    sGroupFishes[((*this).actor.params as libc::c_int -
                                      100 as libc::c_int + 32 as libc::c_int)
                                     as usize].pos;
                D_80B7A898 = 3 as libc::c_int as f32_0
            }
            Math_ApproachF(&mut (*this).actor.speedXZ, 5.0f32, 1.0f32,
                           1.0f32);
            if sp124 < 20.0f32 {
                Math_ApproachS(&mut (*this).unk_170,
                               0x4e20 as libc::c_int as s16,
                               2 as libc::c_int as s16,
                               0xfa0 as libc::c_int as s16);
                if (*this).unk_17A[2 as libc::c_int as usize] as libc::c_int
                       == 0 as libc::c_int &&
                       func_80B70A2C(this, globalCtx,
                                     0 as libc::c_int as u8_0) != 0 {
                    func_80B71278(this, Rand_ZeroFloat(1.99f32) as u8_0);
                    (*this).unk_17A[2 as libc::c_int as usize] =
                        (Rand_ZeroFloat(20.0f32) as s16 as libc::c_int +
                             20 as libc::c_int) as s16
                }
            }
            if (*this).unk_17A[3 as libc::c_int as usize] as libc::c_int ==
                   0 as libc::c_int {
                (*this).unk_158 = 10 as libc::c_int as s16;
                (*this).unk_15A = 10 as libc::c_int as s16
            } else {
                func_80B70ED4(this, input);
                if (*this).actor.xzDistToPlayer < 100.0f32 * sp118 {
                    (*this).unk_158 = 0 as libc::c_int as s16;
                    (*this).unk_15A = (*this).unk_158;
                    (*this).unk_1A4 = 500 as libc::c_int as s16;
                    (*this).unk_1A2 = 200 as libc::c_int as s16;
                    (*this).unk_17A[1 as libc::c_int as usize] =
                        50 as libc::c_int as s16
                }
            }
            current_block_906 = 7001999846415694221;
        }
        _ => { current_block_906 = 7001999846415694221; }
    }
    match current_block_906 {
        12032741234595264021 => {
            Math_ApproachS(&mut (*this).unk_170, 0x2af8 as libc::c_int as s16,
                           2 as libc::c_int as s16,
                           0xfa0 as libc::c_int as s16);
            Math_ApproachF(&mut D_80B7FEC8, 15.0f32, 0.05f32, 0.75f32);
            sp10C.x = D_80B7FEC8;
            if sLinkAge as libc::c_int != 1 as libc::c_int {
                sp10C.y = 30.0f32;
                sp10C.z = 55.0f32
            } else { sp10C.y = 10.0f32; sp10C.z = 50.0f32 }
            Matrix_RotateY((*player).actor.shape.rot.y as libc::c_int as
                               libc::c_float / 32768.0f32 *
                               3.14159265358979323846f32,
                           MTXMODE_NEW as libc::c_int as u8_0);
            Matrix_MultVec3f(&mut sp10C, &mut sCameraEye);
            sCameraEye.x += (*player).actor.world.pos.x;
            sCameraEye.y += (*player).actor.world.pos.y;
            sCameraEye.z += (*player).actor.world.pos.z;
            sCameraAt = (*player).actor.world.pos;
            if sLinkAge as libc::c_int != 1 as libc::c_int {
                sCameraAt.y += 40.0f32
            } else { sCameraAt.y += 25.0f32 }
            if (*this).unk_17A[0 as libc::c_int as usize] as libc::c_int ==
                   90 as libc::c_int {
                Audio_QueueSeqCmd(((SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                                       24 as libc::c_int | 0x24 as libc::c_int
                                       | 0x900 as libc::c_int) as u32_0);
                D_80B7A67C = 40 as libc::c_int as u8_0;
                if (*this).unk_150 as libc::c_int == 0 as libc::c_int {
                    D_80B7A678 = (*this).unk_1AC as u16_0;
                    if D_80B7A678 as libc::c_int >= 75 as libc::c_int {
                        D_80B7E086 = 0x409f as libc::c_int as u16_0
                    } else if D_80B7A678 as libc::c_int >= 50 as libc::c_int {
                        D_80B7E086 = 0x4091 as libc::c_int as u16_0
                    } else { D_80B7E086 = 0x4083 as libc::c_int as u16_0 }
                } else {
                    D_80B7A678 = (2.0f32 * (*this).unk_1AC) as u16_0;
                    D_80B7E086 = 0x4099 as libc::c_int as u16_0
                }
                (*this).unk_1D5 = 0 as libc::c_int as u8_0
            }
            (*this).unk_160 = -(0x4000 as libc::c_int) as s16;
            (*this).actor.shape.rot.y =
                ((*player).actor.shape.rot.y as libc::c_int +
                     0x5000 as libc::c_int) as s16;
            (*this).unk_16E = 0 as libc::c_int as s16;
            (*this).unk_164 = (*this).unk_16E;
            (*this).unk_162 = (*this).unk_164;
            (*this).actor.shape.rot.z = (*this).unk_162;
            (*this).actor.shape.rot.x = (*this).actor.shape.rot.z;
            sp10C.x = 4.0f32;
            sp10C.y = -10.0f32;
            sp10C.z = 5.0f32;
            Matrix_MultVec3f(&mut sp10C, &mut sp100);
            Math_ApproachF(&mut (*this).actor.world.pos.x,
                           (*player).bodyPartsPos[15 as libc::c_int as
                                                      usize].x + sp100.x,
                           1.0f32, 6.0f32);
            Math_ApproachF(&mut (*this).actor.world.pos.y,
                           (*player).bodyPartsPos[15 as libc::c_int as
                                                      usize].y + sp100.y,
                           1.0f32, 6.0f32);
            Math_ApproachF(&mut (*this).actor.world.pos.z,
                           (*player).bodyPartsPos[15 as libc::c_int as
                                                      usize].z + sp100.z,
                           1.0f32, 6.0f32);
            D_80B7E144 = 188.0f32;
            if (*this).unk_17A[0 as libc::c_int as usize] as libc::c_int <=
                   50 as libc::c_int {
                match (*this).unk_1D5 as libc::c_int {
                    0 => {
                        if Message_GetState(&mut (*globalCtx).msgCtx) as
                               libc::c_int == TEXT_STATE_CHOICE as libc::c_int
                               ||
                               Message_GetState(&mut (*globalCtx).msgCtx) as
                                   libc::c_int ==
                                   TEXT_STATE_NONE as libc::c_int {
                            if Message_ShouldAdvance(globalCtx) != 0 {
                                Message_CloseTextbox(globalCtx);
                                if (*globalCtx).msgCtx.choiceIndex as
                                       libc::c_int == 0 as libc::c_int {
                                    if D_80B7A670 == 0.0f32 {
                                        D_80B7A670 = (*this).unk_1AC;
                                        D_80B7E07C = (*this).unk_150;
                                        D_80B7E07E = D_80B7E0B6;
                                        Actor_Kill(&mut (*this).actor);
                                    } else if (*this).unk_150 as libc::c_int
                                                  == 0 as libc::c_int &&
                                                  D_80B7E07C as libc::c_int ==
                                                      0 as libc::c_int &&
                                                  ((*this).unk_1AC as s16 as
                                                       libc::c_int) <
                                                      D_80B7A670 as s16 as
                                                          libc::c_int {
                                        (*this).unk_1D5 =
                                            1 as libc::c_int as u8_0;
                                        (*this).unk_17A[0 as libc::c_int as
                                                            usize] =
                                            0x3c as libc::c_int as s16;
                                        Message_StartTextbox(globalCtx,
                                                             0x4098 as
                                                                 libc::c_int
                                                                 as u16_0,
                                                             0 as *mut Actor);
                                    } else {
                                        let mut temp1: f32_0 = D_80B7A670;
                                        let mut temp2: s16 =
                                            D_80B7E07C as s16;
                                        D_80B7A670 = (*this).unk_1AC;
                                        D_80B7E07C = (*this).unk_150;
                                        D_80B7E07E = D_80B7E0B6;
                                        (*this).unk_1AC = temp1;
                                        (*this).unk_150 = temp2 as u8_0
                                    }
                                }
                                if (*this).unk_1D5 as libc::c_int ==
                                       0 as libc::c_int {
                                    D_80B7A694 = 0 as libc::c_int as s16
                                }
                            }
                        }
                    }
                    1 => {
                        if Message_GetState(&mut (*globalCtx).msgCtx) as
                               libc::c_int == TEXT_STATE_CHOICE as libc::c_int
                               ||
                               Message_GetState(&mut (*globalCtx).msgCtx) as
                                   libc::c_int ==
                                   TEXT_STATE_NONE as libc::c_int {
                            if Message_ShouldAdvance(globalCtx) != 0 {
                                Message_CloseTextbox(globalCtx);
                                if (*globalCtx).msgCtx.choiceIndex as
                                       libc::c_int != 0 as libc::c_int {
                                    let mut temp1_0: f32_0 = D_80B7A670;
                                    let mut temp2_0: s16 = D_80B7E07C as s16;
                                    D_80B7A670 = (*this).unk_1AC;
                                    D_80B7E07E = D_80B7E0B6;
                                    (*this).unk_1AC = temp1_0;
                                    (*this).unk_150 = temp2_0 as u8_0
                                }
                                D_80B7A694 = 0 as libc::c_int as s16
                            }
                        }
                    }
                    _ => { }
                }
            }
            if D_80B7A694 as libc::c_int == 0 as libc::c_int {
                if (*this).actor.update.is_some() {
                    (*this).unk_15A = 0 as libc::c_int as s16;
                    (*this).unk_158 = (*this).unk_15A;
                    (*this).unk_1A4 = 10000 as libc::c_int as s16;
                    (*this).unk_1A2 = 500 as libc::c_int as s16;
                    (*this).unk_17A[1 as libc::c_int as usize] =
                        50 as libc::c_int as s16;
                    (*this).unk_17A[0 as libc::c_int as usize] =
                        0 as libc::c_int as s16;
                    (*this).unk_190 = 1.0f32;
                    (*this).unk_194 = 2000.0f32;
                    SkelAnime_Free(&mut (*this).skelAnime, globalCtx);
                    if (*this).unk_150 as libc::c_int == 0 as libc::c_int {
                        SkelAnime_InitFlex(globalCtx, &mut (*this).skelAnime,
                                           &mut gFishingFishSkel,
                                           &mut gFishingFishAnim,
                                           0 as *mut Vec3s, 0 as *mut Vec3s,
                                           0 as libc::c_int);
                        Animation_MorphToLoop(&mut (*this).skelAnime,
                                              &mut gFishingFishAnim, 0.0f32);
                    } else {
                        SkelAnime_InitFlex(globalCtx, &mut (*this).skelAnime,
                                           &mut gFishingLoachSkel,
                                           &mut gFishingLoachAnim,
                                           0 as *mut Vec3s, 0 as *mut Vec3s,
                                           0 as libc::c_int);
                        Animation_MorphToLoop(&mut (*this).skelAnime,
                                              &mut gFishingLoachAnim, 0.0f32);
                    }
                }
                D_80B7E148 = 520.0f32;
                D_80B7E144 = 195.0f32;
                Audio_QueueSeqCmd(((0x1 as libc::c_int) << 28 as libc::c_int |
                                       (SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                                           24 as libc::c_int |
                                       0xa00ff as libc::c_int) as u32_0);
                D_80B7E0A6 = 20 as libc::c_int as s16;
                D_80B7A6CC = 3 as libc::c_int as u8_0
            }
        }
        _ => { }
    }
    Math_ApproachS(&mut (*this).unk_172,
                   (Math_SinS(((*this).unk_15C as libc::c_int *
                                   0x1000 as libc::c_int) as s16) * 5000.0f32
                        + 5000.0f32) as s16, 2 as libc::c_int as s16,
                   0x7d0 as libc::c_int as s16);
    if (*this).unk_158 as libc::c_int != 6 as libc::c_int {
        if (*this).actor.world.pos.y >
               (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface as
                   libc::c_int as libc::c_float {
            (*this).unk_190 = 1.5f32;
            (*this).unk_194 = 5000.0f32;
            Math_ApproachS(&mut (*this).unk_16E, 0 as libc::c_int as s16,
                           5 as libc::c_int as s16,
                           0x7d0 as libc::c_int as s16);
            spFA = 3 as libc::c_int as s16;
            spF0 = spFA;
            spF4 = spF0;
            spEE = 0x2000 as libc::c_int as s16;
            spF2 = spEE;
            (*this).unk_17A[2 as libc::c_int as usize] =
                0 as libc::c_int as s16;
            (*this).unk_184 -= 1.0f32
        } else {
            Math_ApproachZeroF(&mut (*this).unk_184, 1.0f32, 2.0f32);
            if (*this).unk_158 as libc::c_int != -(1 as libc::c_int) &&
                   (*this).unk_158 as libc::c_int != -(2 as libc::c_int) &&
                   (*this).unk_158 as libc::c_int != -(25 as libc::c_int) {
                (*this).unk_166 = 0 as libc::c_int as s16
            }
            (*this).unk_16A = 0 as libc::c_int as s16;
            (*this).unk_168 = (*this).unk_16A;
            spFA = 4 as libc::c_int as s16;
            spF0 = spFA;
            spF4 = spF0;
            spEE = 0x2000 as libc::c_int as s16;
            spF2 = spEE;
            spF6 =
                (Fishing_SmoothStepToS(&mut (*this).actor.world.rot.y, spFC,
                                       sp134, (*this).unk_1B0 as s16) as
                     libc::c_int as libc::c_float * 3.0f32) as s16;
            Math_ApproachS(&mut (*this).actor.world.rot.x, spFE, sp134,
                           ((*this).unk_1B0 * 0.5f32) as s16);
            if spF6 as libc::c_int > 0x1f40 as libc::c_int {
                spF6 = 0x1f40 as libc::c_int as s16
            } else if (spF6 as libc::c_int) < -(0x1f40 as libc::c_int) {
                spF6 = -(0x1f40 as libc::c_int) as s16
            }
            if (*this).actor.speedXZ >= 3.2f32 {
                Math_ApproachS(&mut (*this).unk_16E, spF6,
                               2 as libc::c_int as s16,
                               0x4e20 as libc::c_int as s16);
            } else {
                Math_ApproachS(&mut (*this).unk_16E, spF6,
                               3 as libc::c_int as s16,
                               0xbb8 as libc::c_int as s16);
            }
            func_8002D908(&mut (*this).actor);
        }
        func_8002D7EC(&mut (*this).actor);
        (*this).actor.world.pos.y += (*this).unk_184 * 1.5f32;
        if (*this).unk_152 as libc::c_int != 0 as libc::c_int {
            (*this).unk_168 = (*this).unk_154;
            (*this).unk_152 = (*this).unk_152.wrapping_sub(1);
            if (*this).unk_156 as libc::c_int != 0 as libc::c_int {
                spF0 = 5 as libc::c_int as s16;
                spEE = 0x4000 as libc::c_int as s16
            } else {
                spF0 = 10 as libc::c_int as s16;
                spEE = 0x800 as libc::c_int as s16
            }
            (*this).unk_166 =
                (-(0x500 as libc::c_int) -
                     (*this).actor.shape.rot.x as libc::c_int) as s16;
            spF4 = 5 as libc::c_int as s16;
            spF2 = 0x4000 as libc::c_int as s16
        } else { (*this).unk_156 = 0 as libc::c_int as u8_0 }
        Math_ApproachS(&mut (*this).unk_160, (*this).unk_166, spF4, spF2);
        Math_ApproachS(&mut (*this).unk_162, (*this).unk_168, spF0, spEE);
        Math_ApproachS(&mut (*this).unk_164, (*this).unk_16A, spFA,
                       0x2000 as libc::c_int as s16);
        if (*this).actor.speedXZ <= 0.5f32 {
            Math_ApproachS(&mut (*this).actor.shape.rot.x,
                           0 as libc::c_int as s16, 10 as libc::c_int as s16,
                           (*this).unk_178);
            Math_ApproachS(&mut (*this).unk_178, 0x500 as libc::c_int as s16,
                           1 as libc::c_int as s16,
                           0x20 as libc::c_int as s16);
        } else {
            Math_ApproachS(&mut (*this).actor.shape.rot.x,
                           -((*this).actor.world.rot.x as libc::c_int) as s16,
                           10 as libc::c_int as s16,
                           0x1000 as libc::c_int as s16);
            (*this).unk_178 = 0 as libc::c_int as s16
        }
        (*this).actor.shape.rot.y = (*this).actor.world.rot.y;
        if (*this).unk_158 as libc::c_int != -(1 as libc::c_int) &&
               (*this).unk_158 as libc::c_int != -(2 as libc::c_int) &&
               (*this).unk_158 as libc::c_int != -(25 as libc::c_int) {
            if (*this).actor.world.pos.y >
                   (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface as
                       libc::c_int as libc::c_float &&
                   (*this).actor.prevPos.y <=
                       (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface
                           as libc::c_int as libc::c_float {
                func_80B70A2C(this, globalCtx, 1 as libc::c_int as u8_0);
                func_80B71278(this, 1 as libc::c_int as u8_0);
                (*this).unk_184 = (*this).actor.velocity.y;
                (*this).actor.velocity.y = 0.0f32;
                (*this).unk_16A = Rand_CenteredFloat(32768.0f32) as s16
            } else if (*this).actor.world.pos.y <
                          (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface
                              as libc::c_int as libc::c_float &&
                          (*this).actor.prevPos.y >=
                              (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface
                                  as libc::c_int as libc::c_float {
                if (*this).unk_184 < -5.0f32 { (*this).unk_184 = -5.0f32 }
                (*this).actor.world.rot.x = -(0xfa0 as libc::c_int) as s16;
                func_80B70A2C(this, globalCtx, 1 as libc::c_int as u8_0);
                (*this).unk_1D2 = 20 as libc::c_int as u8_0;
                func_80B71278(this, 0 as libc::c_int as u8_0);
            }
        }
        if (*this).actor.world.pos.y <
               (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface as
                   libc::c_int as libc::c_float &&
               (*this).actor.world.pos.y >
                   (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface as
                       libc::c_int as libc::c_float - 10.0f32 &&
               (*this).unk_15C as libc::c_int & 1 as libc::c_int ==
                   0 as libc::c_int && (*this).actor.speedXZ > 0.0f32 {
            let mut pos: Vec3f = (*this).actor.world.pos;
            pos.y =
                (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface as
                    f32_0;
            Fishing_SpawnRipple(&mut (*this).actor.projectedPos,
                                (*globalCtx).specialEffects as
                                    *mut FishingEffect, &mut pos, 80.0f32,
                                500.0f32, 150 as libc::c_int as s16,
                                90 as libc::c_int as s16);
        }
        if (*this).actor.speedXZ > 0.0f32 ||
               (*this).unk_158 as libc::c_int == 5 as libc::c_int {
            let mut velocityY: f32_0 = (*this).actor.velocity.y;
            spD8 = (*this).unk_1AC * 0.1f32;
            (*this).actor.world.pos.y -= spD8;
            (*this).actor.prevPos.y -= spD8;
            (*this).actor.velocity.y = -1.0f32;
            Actor_UpdateBgCheckInfo(globalCtx, &mut (*this).actor, 30.0f32,
                                    30.0f32, 100.0f32, 0x45 as libc::c_int);
            (*this).actor.world.pos.y += spD8;
            (*this).actor.prevPos.y += spD8;
            (*this).actor.velocity.y = velocityY;
            if (*this).actor.bgCheckFlags as libc::c_int & 8 as libc::c_int !=
                   0 {
                (*this).unk_1A0 = 20 as libc::c_int as s16
            }
            if (*this).actor.bgCheckFlags as libc::c_int & 1 as libc::c_int !=
                   0 {
                if (*this).actor.world.pos.y >
                       (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface
                           as libc::c_int as libc::c_float {
                    (*this).unk_184 = Rand_ZeroFloat(3.0f32) + 3.0f32;
                    (*this).actor.velocity.x =
                        (*this).actor.world.pos.x * -0.003f32;
                    (*this).actor.velocity.z =
                        (*this).actor.world.pos.z * -0.003f32;
                    Audio_PlayActorSound2(&mut (*this).actor,
                                          0x2869 as libc::c_int as u16_0);
                    func_80B70CF0(this, globalCtx);
                    if Rand_ZeroOne() < 0.5f32 {
                        (*this).unk_16A = 0x4000 as libc::c_int as s16
                    } else {
                        (*this).unk_16A = -(0x4000 as libc::c_int) as s16
                    }
                    if Rand_ZeroOne() < 0.5f32 {
                        (*this).unk_166 = 0 as libc::c_int as s16
                    } else {
                        (*this).unk_166 =
                            (Rand_CenteredFloat(32.0f32) as s16 as libc::c_int
                                 + 0x8000 as libc::c_int) as s16
                    }
                    (*this).unk_168 = Rand_CenteredFloat(16384.0f32) as s16;
                    (*this).unk_190 = 1.0f32;
                    (*this).unk_194 = 5000.0f32;
                    (*this).unk_19C = 5000.0f32
                } else {
                    (*this).unk_184 = 0.0f32;
                    if (*this).unk_158 as libc::c_int == 5 as libc::c_int &&
                           (*this).unk_15C as libc::c_int & 1 as libc::c_int
                               == 0 as libc::c_int {
                        let mut pos_0: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                        pos_0.x =
                            Rand_CenteredFloat(10.0f32) +
                                (*this).actor.world.pos.x;
                        pos_0.z =
                            Rand_CenteredFloat(10.0f32) +
                                (*this).actor.world.pos.z;
                        pos_0.y = (*this).actor.floorHeight + 5.0f32;
                        Fishing_SpawnWaterDust(&mut (*this).actor.projectedPos,
                                               (*globalCtx).specialEffects as
                                                   *mut FishingEffect,
                                               &mut pos_0,
                                               (*this).unk_1AC * 0.005f32 +
                                                   0.15f32);
                    }
                }
            }
        }
    }
    if (*this).unk_1D2 as libc::c_int != 0 as libc::c_int {
        let mut i_0: s16 = 0;
        let mut pos_1: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
        let mut range: f32_0 = (*this).unk_1AC * 0.075f32 + 10.0f32;
        (*this).unk_1D2 = (*this).unk_1D2.wrapping_sub(1);
        i_0 = 0 as libc::c_int as s16;
        while (i_0 as libc::c_int) < 2 as libc::c_int {
            pos_1.x = Rand_CenteredFloat(range) + (*this).actor.world.pos.x;
            pos_1.y = Rand_CenteredFloat(range) + (*this).actor.world.pos.y;
            pos_1.z = Rand_CenteredFloat(range) + (*this).actor.world.pos.z;
            Fishing_SpawnBubble(&mut (*this).actor.projectedPos,
                                (*globalCtx).specialEffects as
                                    *mut FishingEffect, &mut pos_1,
                                Rand_ZeroFloat(0.035f32) + 0.04f32,
                                0 as libc::c_int as u8_0);
            i_0 += 1
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Fishing_FishOverrideLimbDraw(mut globalCtx:
                                                          *mut GlobalContext,
                                                      mut limbIndex: s32,
                                                      mut dList:
                                                          *mut *mut Gfx,
                                                      mut pos: *mut Vec3f,
                                                      mut rot: *mut Vec3s,
                                                      mut thisx:
                                                          *mut libc::c_void)
 -> s32 {
    let mut this: *mut Fishing = thisx as *mut Fishing;
    if limbIndex == 0xd as libc::c_int {
        (*rot).z =
            ((*rot).z as libc::c_int -
                 ((*this).unk_170 as libc::c_int - 11000 as libc::c_int)) as
                s16
    } else if limbIndex == 2 as libc::c_int || limbIndex == 3 as libc::c_int {
        (*rot).y =
            ((*rot).y as libc::c_int + (*this).unk_16C as libc::c_int) as s16
    } else if limbIndex == 4 as libc::c_int {
        (*rot).y =
            ((*rot).y as libc::c_int + (*this).unk_176 as libc::c_int) as s16
    } else if limbIndex == 0xe as libc::c_int {
        (*rot).y =
            ((*rot).y as libc::c_int - (*this).unk_172 as libc::c_int) as s16
    } else if limbIndex == 0xf as libc::c_int {
        (*rot).y =
            ((*rot).y as libc::c_int + (*this).unk_172 as libc::c_int) as s16
    } else if limbIndex == 8 as libc::c_int {
        (*rot).y =
            ((*rot).y as libc::c_int + (*this).unk_174 as libc::c_int) as s16
    } else if limbIndex == 9 as libc::c_int {
        (*rot).y =
            ((*rot).y as libc::c_int - (*this).unk_174 as libc::c_int) as s16
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Fishing_FishPostLimbDraw(mut globalCtx:
                                                      *mut GlobalContext,
                                                  mut limbIndex: s32,
                                                  mut dList: *mut *mut Gfx,
                                                  mut rot: *mut Vec3s,
                                                  mut thisx:
                                                      *mut libc::c_void) {
    let mut this: *mut Fishing = thisx as *mut Fishing;
    if limbIndex == 0xd as libc::c_int {
        Matrix_MultVec3f(&mut sFishMouthOffset, &mut (*this).fishMouthPos);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Fishing_LoachOverrideLimbDraw(mut globalCtx:
                                                           *mut GlobalContext,
                                                       mut limbIndex: s32,
                                                       mut dList:
                                                           *mut *mut Gfx,
                                                       mut pos: *mut Vec3f,
                                                       mut rot: *mut Vec3s,
                                                       mut thisx:
                                                           *mut libc::c_void)
 -> s32 {
    let mut this: *mut Fishing = thisx as *mut Fishing;
    if limbIndex == 3 as libc::c_int {
        (*rot).y =
            ((*rot).y as libc::c_int +
                 (*this).unk_1CC[0 as libc::c_int as usize] as libc::c_int) as
                s16
    } else if limbIndex == 4 as libc::c_int {
        (*rot).y =
            ((*rot).y as libc::c_int +
                 (*this).unk_1CC[1 as libc::c_int as usize] as libc::c_int) as
                s16
    } else if limbIndex == 5 as libc::c_int {
        (*rot).y =
            ((*rot).y as libc::c_int +
                 (*this).unk_1CC[2 as libc::c_int as usize] as libc::c_int) as
                s16
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Fishing_LoachPostLimbDraw(mut globalCtx:
                                                       *mut GlobalContext,
                                                   mut limbIndex: s32,
                                                   mut dList: *mut *mut Gfx,
                                                   mut rot: *mut Vec3s,
                                                   mut thisx:
                                                       *mut libc::c_void) {
    static mut sLoachMouthOffset: Vec3f =
        { let mut init = Vec3f{x: 500.0f32, y: 500.0f32, z: 0.0f32,}; init };
    let mut this: *mut Fishing = thisx as *mut Fishing;
    if limbIndex == 0xb as libc::c_int {
        Matrix_MultVec3f(&mut sLoachMouthOffset, &mut (*this).fishMouthPos);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Fishing_DrawFish(mut thisx: *mut Actor,
                                          mut globalCtx: *mut GlobalContext) {
    let mut this: *mut Fishing = thisx as *mut Fishing;
    func_80093D18((*globalCtx).state.gfxCtx);
    Matrix_Translate((*this).actor.world.pos.x, (*this).actor.world.pos.y,
                     (*this).actor.world.pos.z,
                     MTXMODE_NEW as libc::c_int as u8_0);
    Matrix_RotateY(((*this).unk_162 as libc::c_int +
                        (*this).actor.shape.rot.y as libc::c_int) as
                       libc::c_float / 32768.0f32 * 3.14159265358979323846f32,
                   MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_RotateX(((*this).unk_160 as libc::c_int +
                        (*this).actor.shape.rot.x as libc::c_int) as
                       libc::c_float / 32768.0f32 * 3.14159265358979323846f32,
                   MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_RotateZ(((*this).unk_164 as libc::c_int +
                        (*this).actor.shape.rot.z as libc::c_int) as
                       libc::c_float / 32768.0f32 * 3.14159265358979323846f32,
                   MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_Scale((*this).actor.scale.x, (*this).actor.scale.y,
                 (*this).actor.scale.z, MTXMODE_APPLY as libc::c_int as u8_0);
    if (*this).unk_150 as libc::c_int == 0 as libc::c_int {
        Matrix_RotateY((*this).unk_16C as libc::c_int as libc::c_float *
                           (3.14159265358979323846f32 /
                                32768 as libc::c_int as libc::c_float) -
                           3.14159265358979323846f32 /
                               2 as libc::c_int as libc::c_float,
                       MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_Translate(0.0f32, 0.0f32,
                         (*this).unk_16C as libc::c_int as libc::c_float *
                             10.0f32 * 0.01f32,
                         MTXMODE_APPLY as libc::c_int as u8_0);
        SkelAnime_DrawFlexOpa(globalCtx, (*this).skelAnime.skeleton,
                              (*this).skelAnime.jointTable,
                              (*this).skelAnime.dListCount as s32,
                              Some(Fishing_FishOverrideLimbDraw as
                                       unsafe extern "C" fn(_:
                                                                *mut GlobalContext,
                                                            _: s32,
                                                            _: *mut *mut Gfx,
                                                            _: *mut Vec3f,
                                                            _: *mut Vec3s,
                                                            _:
                                                                *mut libc::c_void)
                                           -> s32),
                              Some(Fishing_FishPostLimbDraw as
                                       unsafe extern "C" fn(_:
                                                                *mut GlobalContext,
                                                            _: s32,
                                                            _: *mut *mut Gfx,
                                                            _: *mut Vec3s,
                                                            _:
                                                                *mut libc::c_void)
                                           -> ()), this as *mut libc::c_void);
    } else {
        Matrix_Translate(0.0f32, 0.0f32, 3000.0f32,
                         MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_RotateY((*this).unk_16C as libc::c_int as libc::c_float *
                           (3.14159265358979323846f32 /
                                32768 as libc::c_int as libc::c_float),
                       MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_Translate(0.0f32, 0.0f32, -3000.0f32,
                         MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_RotateY(-(3.14159265358979323846f32 /
                             2 as libc::c_int as libc::c_float),
                       MTXMODE_APPLY as libc::c_int as u8_0);
        SkelAnime_DrawFlexOpa(globalCtx, (*this).skelAnime.skeleton,
                              (*this).skelAnime.jointTable,
                              (*this).skelAnime.dListCount as s32,
                              Some(Fishing_LoachOverrideLimbDraw as
                                       unsafe extern "C" fn(_:
                                                                *mut GlobalContext,
                                                            _: s32,
                                                            _: *mut *mut Gfx,
                                                            _: *mut Vec3f,
                                                            _: *mut Vec3s,
                                                            _:
                                                                *mut libc::c_void)
                                           -> s32),
                              Some(Fishing_LoachPostLimbDraw as
                                       unsafe extern "C" fn(_:
                                                                *mut GlobalContext,
                                                            _: s32,
                                                            _: *mut *mut Gfx,
                                                            _: *mut Vec3s,
                                                            _:
                                                                *mut libc::c_void)
                                           -> ()), this as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Fishing_HandleReedContact(mut prop: *mut FishingProp,
                                                   mut entityPos:
                                                       *mut Vec3f) {
    let mut dx: f32_0 = (*prop).pos.x - (*entityPos).x;
    let mut dz: f32_0 = (*prop).pos.z - (*entityPos).z;
    let mut distXZ: f32_0 = sqrtf(dx * dx + dz * dz);
    if distXZ <= 20.0f32 {
        (*prop).rotY = Math_Atan2F(dz, dx);
        Math_ApproachF(&mut (*prop).rotX, (20.0f32 - distXZ) * 0.03f32,
                       0.2f32, 0.2f32);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Fishing_HandleLilyPadContact(mut prop:
                                                          *mut FishingProp,
                                                      mut entityPos:
                                                          *mut Vec3f,
                                                      mut fishTimer: u8_0) {
    let mut dx: f32_0 = (*prop).pos.x - (*entityPos).x;
    let mut dz: f32_0 = (*prop).pos.z - (*entityPos).z;
    let mut distXZ: f32_0 = sqrtf(dx * dx + dz * dz);
    if distXZ <= 40.0f32 {
        Math_ApproachS(&mut (*prop).lilyPadAngle, Math_Atan2S(dz, dx),
                       10 as libc::c_int as s16, 0x300 as libc::c_int as s16);
    }
    if fishTimer as libc::c_int != 0 && distXZ <= 60.0f32 {
        let mut heightTarget: f32_0 = 1.0f32;
        if fishTimer as libc::c_int >= 21 as libc::c_int {
            heightTarget = 1.5f32
        }
        Math_ApproachF(&mut (*prop).lilyPadOffset, heightTarget, 0.1f32,
                       0.2f32);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Fishing_UpdatePondProps(mut globalCtx:
                                                     *mut GlobalContext) {
    let mut prop: *mut FishingProp =
        &mut *sPondProps.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut FishingProp;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut actor: *mut Actor = 0 as *mut Actor;
    let mut i: s16 = 0;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < 140 as libc::c_int {
        if (*prop).type_0 as libc::c_int != FS_PROP_NONE as libc::c_int {
            (*prop).shouldDraw = 0 as libc::c_int as u8_0;
            (*prop).timer += 1;
            SkinMatrix_Vec3fMtxFMultXYZW(&mut (*globalCtx).viewProjectionMtxF,
                                         &mut (*prop).pos,
                                         &mut (*prop).projectedPos,
                                         &mut sProjectedW);
            if (*prop).projectedPos.z < (*prop).drawDistance &&
                   fabsf((*prop).projectedPos.x) <
                       100.0f32 + (*prop).projectedPos.z {
                (*prop).shouldDraw = 1 as libc::c_int as u8_0
            }
            if (*prop).projectedPos.z < 500.0f32 &&
                   fabsf((*prop).projectedPos.x) <
                       100.0f32 + (*prop).projectedPos.z {
                if (*prop).type_0 as libc::c_int ==
                       FS_PROP_REED as libc::c_int {
                    Fishing_HandleReedContact(prop,
                                              &mut (*player).actor.world.pos);
                    actor =
                        (*globalCtx).actorCtx.actorLists[ACTORCAT_NPC as
                                                             libc::c_int as
                                                             usize].head;
                    while !actor.is_null() {
                        if !((*actor).id as libc::c_int ==
                                 ACTOR_FISHING as libc::c_int &&
                                 (*actor).params as libc::c_int >=
                                     100 as libc::c_int) {
                            actor = (*actor).next
                        } else {
                            Fishing_HandleReedContact(prop,
                                                      &mut (*actor).world.pos);
                            actor = (*actor).next
                        }
                    }
                    Math_ApproachZeroF(&mut (*prop).rotX, 0.05f32, 0.05f32);
                } else if (*prop).type_0 as libc::c_int ==
                              FS_PROP_LILY_PAD as libc::c_int {
                    Fishing_HandleLilyPadContact(prop,
                                                 &mut (*player).actor.world.pos,
                                                 0 as libc::c_int as u8_0);
                    actor =
                        (*globalCtx).actorCtx.actorLists[ACTORCAT_NPC as
                                                             libc::c_int as
                                                             usize].head;
                    while !actor.is_null() {
                        if !((*actor).id as libc::c_int ==
                                 ACTOR_FISHING as libc::c_int &&
                                 (*actor).params as libc::c_int >=
                                     100 as libc::c_int) {
                            actor = (*actor).next
                        } else {
                            Fishing_HandleLilyPadContact(prop,
                                                         &mut (*actor).world.pos,
                                                         (*(actor as
                                                                *mut Fishing)).unk_151);
                            actor = (*actor).next
                        }
                    }
                    Math_ApproachS(&mut (*prop).lilyPadAngle,
                                   0 as libc::c_int as s16,
                                   20 as libc::c_int as s16,
                                   80 as libc::c_int as s16);
                    (*prop).pos.y =
                        Math_SinS(((*prop).timer as libc::c_int *
                                       0x1000 as libc::c_int) as s16) *
                            (*prop).lilyPadOffset +
                            ((*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface
                                 as libc::c_int as libc::c_float + 2.0f32);
                    Math_ApproachZeroF(&mut (*prop).lilyPadOffset, 0.1f32,
                                       0.02f32);
                }
            }
        }
        prop = prop.offset(1);
        i += 1
    }
    if sCameraId as libc::c_int == 0 as libc::c_int {
        CollisionCheck_SetOC(globalCtx, &mut (*globalCtx).colChkCtx,
                             &mut (*sFishingMain).collider.base);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Fishing_DrawPondProps(mut globalCtx:
                                                   *mut GlobalContext) {
    let mut flag: u8_0 = 0 as libc::c_int as u8_0;
    let mut prop: *mut FishingProp =
        &mut *sPondProps.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut FishingProp;
    let mut i: s16 = 0;
    let mut pad: s32 = 0;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_fishing.c\x00" as *const u8 as *const libc::c_char,
                    7704 as libc::c_int);
    Matrix_Push();
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < 140 as libc::c_int {
        if (*prop).type_0 as libc::c_int == FS_PROP_REED as libc::c_int {
            if flag as libc::c_int == 0 as libc::c_int {
                let fresh88 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g: *mut Gfx = fresh88;
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
                    gFishingReedMaterialDL.as_mut_ptr() as libc::c_uint;
                flag = flag.wrapping_add(1)
            }
            if (*prop).shouldDraw != 0 {
                Matrix_Translate((*prop).pos.x, (*prop).pos.y, (*prop).pos.z,
                                 MTXMODE_NEW as libc::c_int as u8_0);
                Matrix_Scale((*prop).scale, (*prop).scale, (*prop).scale,
                             MTXMODE_APPLY as libc::c_int as u8_0);
                Matrix_RotateY((*prop).rotY,
                               MTXMODE_APPLY as libc::c_int as u8_0);
                Matrix_RotateX((*prop).rotX,
                               MTXMODE_APPLY as libc::c_int as u8_0);
                Matrix_RotateY((*prop).reedAngle,
                               MTXMODE_APPLY as libc::c_int as u8_0);
                let fresh89 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_0: *mut Gfx = fresh89;
                (*_g_0).words.w0 =
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
                (*_g_0).words.w1 =
                    Matrix_NewMtx((*globalCtx).state.gfxCtx,
                                  b"../z_fishing.c\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char, 7726 as libc::c_int)
                        as libc::c_uint;
                let fresh90 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_1: *mut Gfx = fresh90;
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
                    gFishingReedModelDL.as_mut_ptr() as libc::c_uint
            }
        }
        prop = prop.offset(1);
        i += 1
    }
    prop =
        &mut *sPondProps.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut FishingProp;
    flag = 0 as libc::c_int as u8_0;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < 140 as libc::c_int {
        if (*prop).type_0 as libc::c_int == FS_PROP_WOOD_POST as libc::c_int {
            if flag as libc::c_int == 0 as libc::c_int {
                let fresh91 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_2: *mut Gfx = fresh91;
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
                    gFishingWoodPostMaterialDL.as_mut_ptr() as libc::c_uint;
                flag = flag.wrapping_add(1)
            }
            if (*prop).shouldDraw != 0 {
                Matrix_Translate((*prop).pos.x, (*prop).pos.y, (*prop).pos.z,
                                 MTXMODE_NEW as libc::c_int as u8_0);
                Matrix_Scale((*prop).scale, (*prop).scale, (*prop).scale,
                             MTXMODE_APPLY as libc::c_int as u8_0);
                let fresh92 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_3: *mut Gfx = fresh92;
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
                               0 as libc::c_int) ^ 0x1 as libc::c_int) as
                             u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                (*_g_3).words.w1 =
                    Matrix_NewMtx((*globalCtx).state.gfxCtx,
                                  b"../z_fishing.c\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char, 7748 as libc::c_int)
                        as libc::c_uint;
                let fresh93 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_4: *mut Gfx = fresh93;
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
                    gFishingWoodPostModelDL.as_mut_ptr() as libc::c_uint
            }
        }
        prop = prop.offset(1);
        i += 1
    }
    prop =
        &mut *sPondProps.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut FishingProp;
    flag = 0 as libc::c_int as u8_0;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < 140 as libc::c_int {
        if (*prop).type_0 as libc::c_int == FS_PROP_LILY_PAD as libc::c_int {
            if flag as libc::c_int == 0 as libc::c_int {
                let fresh94 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_5: *mut Gfx = fresh94;
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
                    gFishingLilyPadMaterialDL.as_mut_ptr() as libc::c_uint;
                flag = flag.wrapping_add(1)
            }
            if (*prop).shouldDraw != 0 {
                Matrix_Translate((*prop).pos.x, (*prop).pos.y, (*prop).pos.z,
                                 MTXMODE_NEW as libc::c_int as u8_0);
                Matrix_Scale((*prop).scale, 1.0f32, (*prop).scale,
                             MTXMODE_APPLY as libc::c_int as u8_0);
                Matrix_RotateY((*prop).lilyPadAngle as libc::c_int as
                                   libc::c_float *
                                   (3.14159265358979323846f32 /
                                        32768 as libc::c_int as
                                            libc::c_float),
                               MTXMODE_APPLY as libc::c_int as u8_0);
                Matrix_Translate(0.0f32, 0.0f32, 20.0f32,
                                 MTXMODE_APPLY as libc::c_int as u8_0);
                Matrix_RotateY((*prop).rotY,
                               MTXMODE_APPLY as libc::c_int as u8_0);
                let fresh95 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_6: *mut Gfx = fresh95;
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
                                  b"../z_fishing.c\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char, 7774 as libc::c_int)
                        as libc::c_uint;
                let fresh96 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_7: *mut Gfx = fresh96;
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
                    gFishingLilyPadModelDL.as_mut_ptr() as libc::c_uint
            }
        }
        prop = prop.offset(1);
        i += 1
    }
    prop =
        &mut *sPondProps.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut FishingProp;
    flag = 0 as libc::c_int as u8_0;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < 140 as libc::c_int {
        if (*prop).type_0 as libc::c_int == FS_PROP_ROCK as libc::c_int {
            if flag as libc::c_int == 0 as libc::c_int {
                let fresh97 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_8: *mut Gfx = fresh97;
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
                    gFishingRockMaterialDL.as_mut_ptr() as libc::c_uint;
                flag = flag.wrapping_add(1)
            }
            if (*prop).shouldDraw != 0 {
                Matrix_Translate((*prop).pos.x, (*prop).pos.y, (*prop).pos.z,
                                 MTXMODE_NEW as libc::c_int as u8_0);
                Matrix_Scale((*prop).scale, (*prop).scale, (*prop).scale,
                             MTXMODE_APPLY as libc::c_int as u8_0);
                Matrix_RotateY((*prop).rotY,
                               MTXMODE_APPLY as libc::c_int as u8_0);
                let fresh98 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_9: *mut Gfx = fresh98;
                (*_g_9).words.w0 =
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
                (*_g_9).words.w1 =
                    Matrix_NewMtx((*globalCtx).state.gfxCtx,
                                  b"../z_fishing.c\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char, 7798 as libc::c_int)
                        as libc::c_uint;
                let fresh99 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_10: *mut Gfx = fresh99;
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
                (*_g_10).words.w1 =
                    gFishingRockModelDL.as_mut_ptr() as libc::c_uint
            }
        }
        prop = prop.offset(1);
        i += 1
    }
    Matrix_Pop();
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_fishing.c\x00" as *const u8 as
                         *const libc::c_char, 7805 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Fishing_UpdateGroupFishes(mut globalCtx:
                                                       *mut GlobalContext) {
    let mut groupContactFlags: s16 = 0 as libc::c_int as s16;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut fish: *mut FishingGroupFish =
        &mut *sGroupFishes.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut FishingGroupFish;
    let mut dy: f32_0 = 0.;
    let mut dx: f32_0 = 0.;
    let mut dist: f32_0 = 0.;
    let mut dz: f32_0 = 0.;
    let mut offset: f32_0 = 0.;
    let mut groupIndex: s16 = 0;
    let mut groupFlag: s16 = 0;
    let mut spD8: f32_0 = 0.;
    let mut spD6: s16 = 0;
    let mut spD4: s16 = 0;
    let mut target: s16 = 0;
    let mut i: s16 = 0;
    let mut basePos: [Vec3f; 3] = [Vec3f{x: 0., y: 0., z: 0.,}; 3];
    let mut ripplePos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut refPos: *mut Vec3f = 0 as *mut Vec3f;
    let mut temp1: f32_0 = 0.;
    let mut temp2: f32_0 = 0.;
    if D_80B7E114 as libc::c_int != 0 as libc::c_int ||
           D_80B7A694 as libc::c_int == 4 as libc::c_int {
        refPos = &mut sLurePos
    } else { refPos = &mut (*player).actor.world.pos }
    basePos[0 as libc::c_int as usize].x = sinf(sFishGroupAngle1) * 720.0f32;
    basePos[0 as libc::c_int as usize].y = -35.0f32;
    basePos[0 as libc::c_int as usize].z = cosf(sFishGroupAngle1) * 720.0f32;
    temp1 = (*refPos).x - basePos[0 as libc::c_int as usize].x;
    temp2 = (*refPos).z - basePos[0 as libc::c_int as usize].z;
    if temp1 * temp1 + temp2 * temp2 < 50.0f32 * 50.0f32 {
        sFishGroupAngle1 += 0.3f32;
        groupContactFlags =
            (groupContactFlags as libc::c_int | 1 as libc::c_int) as s16
    } else if D_80B7A898 != 0.0f32 {
        sFishGroupAngle1 += 0.05f32;
        basePos[0 as libc::c_int as usize].y =
            (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface as
                libc::c_int as libc::c_float - 5.0f32
    } else {
        Math_ApproachF(&mut sFishGroupAngle1, 0.7f32, 1.0f32, 0.001f32);
    }
    basePos[1 as libc::c_int as usize].x = sinf(sFishGroupAngle2) * 720.0f32;
    basePos[1 as libc::c_int as usize].y = -35.0f32;
    basePos[1 as libc::c_int as usize].z = cosf(sFishGroupAngle2) * 720.0f32;
    temp1 = (*refPos).x - basePos[1 as libc::c_int as usize].x;
    temp2 = (*refPos).z - basePos[1 as libc::c_int as usize].z;
    if temp1 * temp1 + temp2 * temp2 < 50.0f32 * 50.0f32 {
        sFishGroupAngle2 -= 0.3f32;
        groupContactFlags =
            (groupContactFlags as libc::c_int | 2 as libc::c_int) as s16
    } else if D_80B7A898 != 0.0f32 {
        sFishGroupAngle2 -= 0.05f32;
        basePos[1 as libc::c_int as usize].y =
            (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface as
                libc::c_int as libc::c_float - 5.0f32
    } else {
        Math_ApproachF(&mut sFishGroupAngle2, 2.3f32, 1.0f32, 0.001f32);
    }
    basePos[2 as libc::c_int as usize].x = sinf(sFishGroupAngle3) * 720.0f32;
    basePos[2 as libc::c_int as usize].y = -35.0f32;
    basePos[2 as libc::c_int as usize].z = cosf(sFishGroupAngle3) * 720.0f32;
    temp1 = (*refPos).x - basePos[2 as libc::c_int as usize].x;
    temp2 = (*refPos).z - basePos[2 as libc::c_int as usize].z;
    if temp1 * temp1 + temp2 * temp2 < 50.0f32 * 50.0f32 {
        sFishGroupAngle3 -= 0.3f32;
        groupContactFlags =
            (groupContactFlags as libc::c_int | 4 as libc::c_int) as s16
    } else if D_80B7A898 != 0.0f32 {
        sFishGroupAngle3 -= 0.05f32;
        basePos[2 as libc::c_int as usize].y =
            (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface as
                libc::c_int as libc::c_float - 5.0f32
    } else {
        Math_ApproachF(&mut sFishGroupAngle3, 4.6f32, 1.0f32, 0.001f32);
    }
    if sLinkAge as libc::c_int == 1 as libc::c_int {
        spD8 = 0.8f32
    } else { spD8 = 1.0f32 }
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < 60 as libc::c_int {
        if (*fish).type_0 as libc::c_int != FS_GROUP_FISH_NONE as libc::c_int
           {
            (*fish).timer += 1;
            SkinMatrix_Vec3fMtxFMultXYZW(&mut (*globalCtx).viewProjectionMtxF,
                                         &mut (*fish).pos,
                                         &mut (*fish).projectedPos,
                                         &mut sProjectedW);
            if (*fish).projectedPos.z < 400.0f32 &&
                   fabsf((*fish).projectedPos.x) <
                       100.0f32 + (*fish).projectedPos.z {
                (*fish).shouldDraw = 1 as libc::c_int as u8_0
            } else { (*fish).shouldDraw = 0 as libc::c_int as u8_0 }
            if i as libc::c_int <= 20 as libc::c_int {
                groupIndex = 0 as libc::c_int as s16;
                groupFlag = 1 as libc::c_int as s16
            } else if i as libc::c_int <= 40 as libc::c_int {
                groupIndex = 1 as libc::c_int as s16;
                groupFlag = 2 as libc::c_int as s16
            } else {
                groupIndex = 2 as libc::c_int as s16;
                groupFlag = 4 as libc::c_int as s16
            }
            dx = (*fish).unk_10.x - (*fish).pos.x;
            dy = (*fish).unk_10.y - (*fish).pos.y;
            dz = (*fish).unk_10.z - (*fish).pos.z;
            spD4 = Math_Atan2S(dz, dx);
            dist = sqrtf(dx * dx + dz * dz);
            spD6 = Math_Atan2S(dist, dy);
            if dist < 10.0f32 ||
                   (*fish).timer as libc::c_int % 32 as libc::c_int ==
                       0 as libc::c_int && Rand_ZeroOne() > 0.5f32 {
                (*fish).unk_10.y =
                    basePos[groupIndex as usize].y +
                        Rand_CenteredFloat(10.0f32);
                if D_80B7A898 != 0.0f32 {
                    (*fish).unk_10.x =
                        basePos[groupIndex as usize].x +
                            Rand_CenteredFloat(200.0f32);
                    (*fish).unk_10.z =
                        basePos[groupIndex as usize].z +
                            Rand_CenteredFloat(200.0f32)
                } else {
                    (*fish).unk_10.x =
                        basePos[groupIndex as usize].x +
                            Rand_CenteredFloat(100.0f32);
                    (*fish).unk_10.z =
                        basePos[groupIndex as usize].z +
                            Rand_CenteredFloat(100.0f32)
                }
                ripplePos = (*fish).pos;
                ripplePos.y =
                    (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface as
                        f32_0;
                Fishing_SpawnRipple(&mut (*fish).projectedPos,
                                    (*globalCtx).specialEffects as
                                        *mut FishingEffect, &mut ripplePos,
                                    20.0f32,
                                    Rand_ZeroFloat(50.0f32) + 100.0f32,
                                    150 as libc::c_int as s16,
                                    90 as libc::c_int as s16);
                if (*fish).unk_28 < 1.5f32 { (*fish).unk_28 = 1.5f32 }
                (*fish).unk_34 = 1.5f32;
                (*fish).unk_38 = 1.0f32
            }
            target =
                (Fishing_SmoothStepToS(&mut (*fish).unk_3E, spD4,
                                       5 as libc::c_int as s16,
                                       0x4000 as libc::c_int as s16) as
                     libc::c_int as libc::c_float * 3.0f32) as s16;
            if target as libc::c_int > 0x1f40 as libc::c_int {
                target = 0x1f40 as libc::c_int as s16
            } else if (target as libc::c_int) < -(0x1f40 as libc::c_int) {
                target = -(0x1f40 as libc::c_int) as s16
            }
            Math_ApproachS(&mut (*fish).unk_42, target,
                           3 as libc::c_int as s16,
                           0x1388 as libc::c_int as s16);
            offset =
                (*fish).unk_42 as libc::c_int as libc::c_float * -0.0001f32;
            Math_ApproachS(&mut (*fish).unk_3C, spD6, 5 as libc::c_int as s16,
                           0x4000 as libc::c_int as s16);
            if groupContactFlags as libc::c_int & groupFlag as libc::c_int !=
                   0 {
                (*fish).unk_38 = 1.0f32;
                (*fish).unk_28 = 6.0f32;
                (*fish).unk_34 = 2.0f32
            }
            if D_80B7A898 != 0.0f32 {
                (*fish).unk_38 = 1.0f32;
                (*fish).unk_28 = 4.0f32;
                (*fish).unk_34 = 2.0f32
            }
            Math_ApproachF(&mut (*fish).unk_28, 0.75f32, 1.0f32, 0.05f32);
            temp1 = (*fish).unk_28 * spD8;
            temp2 = Math_CosS((*fish).unk_3C) * temp1;
            (*fish).pos.x += temp2 * Math_SinS((*fish).unk_3E);
            (*fish).pos.y += temp1 * Math_SinS((*fish).unk_3C);
            (*fish).pos.z += temp2 * Math_CosS((*fish).unk_3E);
            if (*fish).shouldDraw != 0 {
                Math_ApproachF(&mut (*fish).unk_34, 1.0f32, 1.0f32, 0.1f32);
                Math_ApproachF(&mut (*fish).unk_38, 0.4f32, 1.0f32, 0.04f32);
                (*fish).unk_30 += (*fish).unk_34;
                (*fish).unk_2C =
                    cosf((*fish).unk_30) * (*fish).unk_38 + offset
            }
        }
        fish = fish.offset(1);
        i += 1
    }
    D_80B7A898 = 0.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn Fishing_DrawGroupFishes(mut globalCtx:
                                                     *mut GlobalContext) {
    let mut flag: u8_0 = 0 as libc::c_int as u8_0;
    let mut fish: *mut FishingGroupFish =
        &mut *sGroupFishes.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut FishingGroupFish;
    let mut scale: f32_0 = 0.;
    let mut i: s16 = 0;
    let mut pad: s32 = 0;
    if sLinkAge as libc::c_int == 1 as libc::c_int {
        scale = 0.003325f32
    } else { scale = 0.00475f32 }
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_fishing.c\x00" as *const u8 as *const libc::c_char,
                    8048 as libc::c_int);
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < 60 as libc::c_int {
        if (*fish).type_0 as libc::c_int != FS_GROUP_FISH_NONE as libc::c_int
           {
            if flag as libc::c_int == 0 as libc::c_int {
                let fresh100 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g: *mut Gfx = fresh100;
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
                    gFishingGroupFishMaterialDL.as_mut_ptr() as libc::c_uint;
                let fresh101 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_0: *mut Gfx = fresh101;
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
                    (155 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (155 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            16 as libc::c_int |
                        (155 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            8 as libc::c_int |
                        (255 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                flag = flag.wrapping_add(1)
            }
            if (*fish).shouldDraw != 0 {
                Matrix_Translate((*fish).pos.x, (*fish).pos.y, (*fish).pos.z,
                                 MTXMODE_NEW as libc::c_int as u8_0);
                Matrix_RotateY((*fish).unk_3E as f32_0 *
                                   3.14159265358979323846f32 / 32768.0f32,
                               MTXMODE_APPLY as libc::c_int as u8_0);
                Matrix_RotateX(-((*fish).unk_3C as f32_0) *
                                   3.14159265358979323846f32 / 32768.0f32,
                               MTXMODE_APPLY as libc::c_int as u8_0);
                Matrix_Scale((*fish).unk_2C * scale, scale, scale,
                             MTXMODE_APPLY as libc::c_int as u8_0);
                let fresh102 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_1: *mut Gfx = fresh102;
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
                               0 as libc::c_int) ^ 0x1 as libc::c_int) as
                             u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                (*_g_1).words.w1 =
                    Matrix_NewMtx((*globalCtx).state.gfxCtx,
                                  b"../z_fishing.c\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char, 8093 as libc::c_int)
                        as libc::c_uint;
                let fresh103 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_2: *mut Gfx = fresh103;
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
                    gFishingGroupFishModelDL.as_mut_ptr() as libc::c_uint
            }
        }
        fish = fish.offset(1);
        i += 1
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_fishing.c\x00" as *const u8 as
                         *const libc::c_char, 8099 as libc::c_int);
}
static mut D_80B7AFB8: [u16_0; 6] =
    [0x4096 as libc::c_int as u16_0, 0x408d as libc::c_int as u16_0,
     0x408e as libc::c_int as u16_0, 0x408f as libc::c_int as u16_0,
     0x4094 as libc::c_int as u16_0, 0x4095 as libc::c_int as u16_0];
#[no_mangle]
pub unsafe extern "C" fn Fishing_HandleOwnerDialog(mut this: *mut Fishing,
                                                   mut globalCtx:
                                                       *mut GlobalContext) {
    let mut current_block_241: u64;
    match (*this).unk_15C as libc::c_int {
        0 => {
            if D_80B7E0AC as libc::c_int == 0 as libc::c_int {
                if sLinkAge as libc::c_int != 1 as libc::c_int {
                    if gSaveContext.highScores[HS_FISHING as libc::c_int as
                                                   usize] &
                           0x100 as libc::c_int != 0 &&
                           gSaveContext.highScores[HS_FISHING as libc::c_int
                                                       as usize] &
                               0x200 as libc::c_int == 0 {
                        (*this).actor.textId = 0x4093 as libc::c_int as u16_0
                    } else {
                        (*this).actor.textId = 0x407b as libc::c_int as u16_0
                    }
                } else {
                    (*this).actor.textId = 0x407b as libc::c_int as u16_0
                }
            } else if D_80B7A68C as libc::c_int == 0 as libc::c_int {
                (*this).actor.textId = 0x4084 as libc::c_int as u16_0
            } else { (*this).actor.textId = 0x4097 as libc::c_int as u16_0 }
            if Actor_ProcessTalkRequest(&mut (*this).actor, globalCtx) != 0 {
                if D_80B7E0AC as libc::c_int == 0 as libc::c_int {
                    (*this).unk_15C = 1 as libc::c_int as s16;
                    if sLinkAge as libc::c_int != 1 as libc::c_int {
                        gSaveContext.highScores[HS_FISHING as libc::c_int as
                                                    usize] |=
                            0x200 as libc::c_int
                    } else {
                        gSaveContext.highScores[HS_FISHING as libc::c_int as
                                                    usize] |=
                            0x100 as libc::c_int
                    }
                } else { (*this).unk_15C = 10 as libc::c_int as s16 }
            } else { func_8002F2CC(&mut (*this).actor, globalCtx, 100.0f32); }
        }
        1 => {
            if Message_GetState(&mut (*globalCtx).msgCtx) as libc::c_int ==
                   TEXT_STATE_CHOICE as libc::c_int &&
                   Message_ShouldAdvance(globalCtx) as libc::c_int != 0 {
                Message_CloseTextbox(globalCtx);
                match (*globalCtx).msgCtx.choiceIndex as libc::c_int {
                    0 => {
                        if gSaveContext.rupees as libc::c_int >=
                               20 as libc::c_int {
                            Rupees_ChangeBy(-(20 as libc::c_int) as s16);
                            if func_800AA148() ==
                                   0 as libc::c_int as libc::c_uint {
                                (*this).actor.textId =
                                    0x407c as libc::c_int as u16_0
                            } else {
                                (*this).actor.textId =
                                    0x407d as libc::c_int as u16_0
                            }
                            Message_ContinueTextbox(globalCtx,
                                                    (*this).actor.textId);
                            (*this).unk_15C = 2 as libc::c_int as s16
                        } else {
                            Message_ContinueTextbox(globalCtx,
                                                    0x407e as libc::c_int as
                                                        u16_0);
                            (*this).unk_15C = 3 as libc::c_int as s16
                        }
                    }
                    1 => {
                        Message_ContinueTextbox(globalCtx,
                                                0x2d as libc::c_int as u16_0);
                        (*this).unk_15C = 3 as libc::c_int as s16
                    }
                    _ => { }
                }
            }
        }
        2 => {
            if Message_GetState(&mut (*globalCtx).msgCtx) as libc::c_int ==
                   TEXT_STATE_EVENT as libc::c_int &&
                   Message_ShouldAdvance(globalCtx) as libc::c_int != 0 {
                Message_CloseTextbox(globalCtx);
                Message_ContinueTextbox(globalCtx,
                                        0x407f as libc::c_int as u16_0);
                (*this).unk_15C = 4 as libc::c_int as s16
            }
        }
        3 => {
            if Message_GetState(&mut (*globalCtx).msgCtx) as libc::c_int ==
                   TEXT_STATE_EVENT as libc::c_int &&
                   Message_ShouldAdvance(globalCtx) as libc::c_int != 0 {
                Message_CloseTextbox(globalCtx);
                (*this).unk_15C = 0 as libc::c_int as s16
            }
            if Message_GetState(&mut (*globalCtx).msgCtx) as libc::c_int ==
                   TEXT_STATE_DONE as libc::c_int {
                (*this).unk_15C = 0 as libc::c_int as s16
            }
        }
        4 => {
            if Message_GetState(&mut (*globalCtx).msgCtx) as libc::c_int ==
                   TEXT_STATE_CHOICE as libc::c_int &&
                   Message_ShouldAdvance(globalCtx) as libc::c_int != 0 {
                Message_CloseTextbox(globalCtx);
                match (*globalCtx).msgCtx.choiceIndex as libc::c_int {
                    0 => {
                        D_80B7A678 = D_80B7E078 as u16_0;
                        Message_ContinueTextbox(globalCtx,
                                                0x4080 as libc::c_int as
                                                    u16_0);
                        (*this).unk_15C = 5 as libc::c_int as s16
                    }
                    1 => {
                        Message_ContinueTextbox(globalCtx,
                                                0x407f as libc::c_int as
                                                    u16_0);
                    }
                    _ => { }
                }
            }
        }
        5 => {
            if Message_GetState(&mut (*globalCtx).msgCtx) as libc::c_int ==
                   TEXT_STATE_EVENT as libc::c_int &&
                   Message_ShouldAdvance(globalCtx) as libc::c_int != 0 {
                Message_CloseTextbox(globalCtx);
                (*globalCtx).interfaceCtx.unk_260 = 1 as libc::c_int as u8_0;
                (*globalCtx).startPlayerFishing.expect("non-null function pointer")(globalCtx);
                D_80B7E0AC = 1 as libc::c_int as s16;
                D_80B7A684 = 20 as libc::c_int as s16;
                (*this).unk_15C = 0 as libc::c_int as s16;
                if (gSaveContext.highScores[HS_FISHING as libc::c_int as
                                                usize] &
                        0xff0000 as libc::c_int) < 0xff0000 as libc::c_int {
                    gSaveContext.highScores[HS_FISHING as libc::c_int as
                                                usize] +=
                        0x10000 as libc::c_int
                }
            }
        }
        10 => {
            if D_80B7A68C as libc::c_int != 0 as libc::c_int {
                if Message_GetState(&mut (*globalCtx).msgCtx) as libc::c_int
                       == TEXT_STATE_CHOICE as libc::c_int &&
                       Message_ShouldAdvance(globalCtx) as libc::c_int != 0 {
                    Message_CloseTextbox(globalCtx);
                    match (*globalCtx).msgCtx.choiceIndex as libc::c_int {
                        0 => {
                            Message_ContinueTextbox(globalCtx,
                                                    0x40b2 as libc::c_int as
                                                        u16_0);
                            D_80B7A688 = 1 as libc::c_int as u8_0;
                            D_80B7A68C = 0 as libc::c_int as u8_0;
                            (*this).unk_15C = 20 as libc::c_int as s16
                        }
                        1 => { (*this).unk_15C = 0 as libc::c_int as s16 }
                        _ => { }
                    }
                }
            } else if Message_GetState(&mut (*globalCtx).msgCtx) as
                          libc::c_int == TEXT_STATE_CHOICE as libc::c_int &&
                          Message_ShouldAdvance(globalCtx) as libc::c_int != 0
             {
                Message_CloseTextbox(globalCtx);
                match (*globalCtx).msgCtx.choiceIndex as libc::c_int {
                    0 => {
                        if D_80B7A670 == 0.0f32 {
                            (*this).actor.textId =
                                0x408c as libc::c_int as u16_0;
                            (*this).unk_15C = 20 as libc::c_int as s16
                        } else if D_80B7E07C as libc::c_int ==
                                      0 as libc::c_int {
                            D_80B7A678 = D_80B7A670 as u16_0;
                            if (D_80B7E078 as s16 as libc::c_int) <
                                   D_80B7A670 as s16 as libc::c_int {
                                if D_80B7E07E as libc::c_int ==
                                       2 as libc::c_int {
                                    (*this).actor.textId =
                                        0x40b0 as libc::c_int as u16_0
                                } else {
                                    (*this).actor.textId =
                                        0x4086 as libc::c_int as u16_0
                                }
                                (*this).unk_15C = 11 as libc::c_int as s16
                            } else {
                                (*this).actor.textId =
                                    0x408b as libc::c_int as u16_0;
                                (*this).unk_15C = 20 as libc::c_int as s16
                            }
                        } else {
                            (*this).actor.textId =
                                0x409b as libc::c_int as u16_0;
                            (*this).unk_15C = 11 as libc::c_int as s16
                        }
                        Message_ContinueTextbox(globalCtx,
                                                (*this).actor.textId);
                    }
                    1 => {
                        if D_80B7A680 > 36000 as libc::c_int {
                            D_80B7A680 = 30000 as libc::c_int;
                            Message_ContinueTextbox(globalCtx,
                                                    0x4088 as libc::c_int as
                                                        u16_0);
                        } else {
                            if D_80B7E076 as libc::c_int == 0 as libc::c_int {
                                if D_80B7E082 as libc::c_int ==
                                       0 as libc::c_int {
                                    D_80B7E082 = D_80B7E082.wrapping_add(1)
                                }
                            }
                            if D_80B7E0B6 as libc::c_int == 2 as libc::c_int
                                   &&
                                   D_80B7AFB8[D_80B7E082 as usize] as
                                       libc::c_int == 0x408d as libc::c_int {
                                Message_ContinueTextbox(globalCtx,
                                                        0x40af as libc::c_int
                                                            as u16_0);
                            } else {
                                Message_ContinueTextbox(globalCtx,
                                                        D_80B7AFB8[D_80B7E082
                                                                       as
                                                                       usize]);
                            }
                            D_80B7E082 = D_80B7E082.wrapping_add(1);
                            if sLinkAge as libc::c_int != 1 as libc::c_int {
                                if D_80B7E082 as libc::c_int >=
                                       6 as libc::c_int {
                                    D_80B7E082 = 0 as libc::c_int as u8_0
                                }
                            } else if D_80B7E082 as libc::c_int >=
                                          4 as libc::c_int {
                                D_80B7E082 = 0 as libc::c_int as u8_0
                            }
                        }
                        (*this).unk_15C = 0 as libc::c_int as s16
                    }
                    2 => {
                        if D_80B7E084 as libc::c_int == 0 as libc::c_int {
                            Message_ContinueTextbox(globalCtx,
                                                    0x4085 as libc::c_int as
                                                        u16_0);
                        } else if sLinkAge as libc::c_int == 1 as libc::c_int
                         {
                            Message_ContinueTextbox(globalCtx,
                                                    0x4092 as libc::c_int as
                                                        u16_0);
                        }
                        (*this).unk_15C = 22 as libc::c_int as s16
                    }
                    _ => { }
                }
            }
        }
        11 => {
            if (Message_GetState(&mut (*globalCtx).msgCtx) as libc::c_int ==
                    TEXT_STATE_EVENT as libc::c_int ||
                    Message_GetState(&mut (*globalCtx).msgCtx) as libc::c_int
                        == TEXT_STATE_NONE as libc::c_int) &&
                   Message_ShouldAdvance(globalCtx) as libc::c_int != 0 {
                let mut getItemId: s32 = 0;
                Message_CloseTextbox(globalCtx);
                if D_80B7E07C as libc::c_int == 0 as libc::c_int {
                    D_80B7E078 = D_80B7A670;
                    D_80B7A670 = 0.0f32;
                    if sLinkAge as libc::c_int == 1 as libc::c_int {
                        let mut temp: f32_0 = 0.;
                        gSaveContext.highScores[HS_FISHING as libc::c_int as
                                                    usize] =
                            (gSaveContext.highScores[HS_FISHING as libc::c_int
                                                         as usize] as
                                 libc::c_uint & 0xffffff00 as libc::c_uint) as
                                s32;
                        gSaveContext.highScores[HS_FISHING as libc::c_int as
                                                    usize] |=
                            D_80B7E078 as s32 & 0x7f as libc::c_int;
                        temp =
                            ((gSaveContext.highScores[HS_FISHING as
                                                          libc::c_int as
                                                          usize] &
                                  0x7f000000 as libc::c_int) >>
                                 0x18 as libc::c_int) as f32_0;
                        if temp < D_80B7E078 {
                            gSaveContext.highScores[HS_FISHING as libc::c_int
                                                        as usize] &=
                                0xffffff as libc::c_int;
                            gSaveContext.highScores[HS_FISHING as libc::c_int
                                                        as usize] |=
                                (D_80B7E078 as s32 & 0x7f as libc::c_int) <<
                                    0x18 as libc::c_int;
                            if D_80B7E07E as libc::c_int == 2 as libc::c_int {
                                gSaveContext.highScores[HS_FISHING as
                                                            libc::c_int as
                                                            usize] =
                                    (gSaveContext.highScores[HS_FISHING as
                                                                 libc::c_int
                                                                 as usize] as
                                         libc::c_uint |
                                         0x80000000 as libc::c_uint) as s32
                            }
                        }
                        if D_80B7E07E as libc::c_int == 2 as libc::c_int {
                            gSaveContext.highScores[HS_FISHING as libc::c_int
                                                        as usize] |=
                                0x80 as libc::c_int;
                            (*this).unk_15C = 0 as libc::c_int as s16;
                            current_block_241 = 7986280648684494582;
                        } else { current_block_241 = 3151994457458062110; }
                    } else {
                        gSaveContext.highScores[HS_FISHING as libc::c_int as
                                                    usize] &=
                            0xffffff as libc::c_int;
                        gSaveContext.highScores[HS_FISHING as libc::c_int as
                                                    usize] |=
                            (D_80B7E078 as s32 & 0x7f as libc::c_int) <<
                                0x18 as libc::c_int;
                        if D_80B7E07E as libc::c_int == 2 as libc::c_int {
                            gSaveContext.highScores[HS_FISHING as libc::c_int
                                                        as usize] =
                                (gSaveContext.highScores[HS_FISHING as
                                                             libc::c_int as
                                                             usize] as
                                     libc::c_uint |
                                     0x80000000 as libc::c_uint) as s32;
                            (*this).unk_15C = 0 as libc::c_int as s16;
                            current_block_241 = 7986280648684494582;
                        } else { current_block_241 = 3151994457458062110; }
                    }
                    match current_block_241 {
                        7986280648684494582 => { }
                        _ => {
                            if D_80B7E078 >= 60.0f32 {
                                getItemId = GI_RUPEE_PURPLE as libc::c_int
                            } else if D_80B7E078 >= 50.0f32 {
                                getItemId = GI_RUPEE_RED as libc::c_int
                            } else if D_80B7E078 >= 40.0f32 {
                                getItemId = GI_RUPEE_BLUE as libc::c_int
                            } else {
                                getItemId = GI_RUPEE_GREEN as libc::c_int
                            }
                            if sLinkAge as libc::c_int == 1 as libc::c_int {
                                if D_80B7E078 >= 50.0f32 &&
                                       gSaveContext.highScores[HS_FISHING as
                                                                   libc::c_int
                                                                   as usize] &
                                           0x400 as libc::c_int == 0 {
                                    gSaveContext.highScores[HS_FISHING as
                                                                libc::c_int as
                                                                usize] |=
                                        0x400 as libc::c_int;
                                    getItemId = GI_HEART_PIECE as libc::c_int;
                                    sSinkingLureLocation =
                                        (Rand_ZeroFloat(3.999f32) as u8_0 as
                                             libc::c_int + 1 as libc::c_int)
                                            as u8_0
                                }
                            } else if D_80B7E078 >= 60.0f32 &&
                                          gSaveContext.highScores[HS_FISHING
                                                                      as
                                                                      libc::c_int
                                                                      as
                                                                      usize] &
                                              0x800 as libc::c_int == 0 {
                                gSaveContext.highScores[HS_FISHING as
                                                            libc::c_int as
                                                            usize] |=
                                    0x800 as libc::c_int;
                                getItemId = GI_SCALE_GOLD as libc::c_int;
                                sSinkingLureLocation =
                                    (Rand_ZeroFloat(3.999f32) as u8_0 as
                                         libc::c_int + 1 as libc::c_int) as
                                        u8_0
                            }
                            current_block_241 = 13526015532137226550;
                        }
                    }
                } else {
                    getItemId = GI_RUPEE_PURPLE as libc::c_int;
                    D_80B7A670 = 0.0f32;
                    current_block_241 = 13526015532137226550;
                }
                match current_block_241 {
                    7986280648684494582 => { }
                    _ => {
                        (*this).actor.parent = 0 as *mut Actor;
                        func_8002F434(&mut (*this).actor, globalCtx,
                                      getItemId, 2000.0f32, 1000.0f32);
                        (*this).unk_15C = 23 as libc::c_int as s16
                    }
                }
            }
        }
        20 => {
            if Message_GetState(&mut (*globalCtx).msgCtx) as libc::c_int ==
                   TEXT_STATE_EVENT as libc::c_int &&
                   Message_ShouldAdvance(globalCtx) as libc::c_int != 0 {
                Message_CloseTextbox(globalCtx);
                (*this).unk_15C = 0 as libc::c_int as s16
            }
        }
        21 => {
            if Message_GetState(&mut (*globalCtx).msgCtx) as libc::c_int ==
                   TEXT_STATE_CHOICE as libc::c_int &&
                   Message_ShouldAdvance(globalCtx) as libc::c_int != 0 {
                Message_CloseTextbox(globalCtx);
                match (*globalCtx).msgCtx.choiceIndex as libc::c_int {
                    0 => { (*this).unk_15C = 0 as libc::c_int as s16 }
                    1 => {
                        if D_80B7E084 as libc::c_int == 0 as libc::c_int {
                            Message_ContinueTextbox(globalCtx,
                                                    0x4085 as libc::c_int as
                                                        u16_0);
                        } else if sLinkAge as libc::c_int == 1 as libc::c_int
                         {
                            Message_ContinueTextbox(globalCtx,
                                                    0x4092 as libc::c_int as
                                                        u16_0);
                        }
                        (*this).unk_15C = 22 as libc::c_int as s16
                    }
                    _ => { }
                }
            }
        }
        22 => {
            !globalCtx.is_null();
            if Message_GetState(&mut (*globalCtx).msgCtx) as libc::c_int ==
                   TEXT_STATE_NONE as libc::c_int {
                (*this).unk_15C = 0 as libc::c_int as s16;
                if D_80B7A68C as libc::c_int != 0 as libc::c_int {
                    D_80B7A688 = 1 as libc::c_int as u8_0;
                    D_80B7A68C = 0 as libc::c_int as u8_0
                }
                D_80B7E0AC = 0 as libc::c_int as s16;
                (*globalCtx).interfaceCtx.unk_260 = 0 as libc::c_int as u8_0
            }
        }
        23 => {
            D_80B7A674 = 0 as libc::c_int as u8_0;
            if Actor_HasParent(&mut (*this).actor, globalCtx) != 0 {
                (*this).unk_15C = 24 as libc::c_int as s16
            } else {
                func_8002F434(&mut (*this).actor, globalCtx,
                              GI_SCALE_GOLD as libc::c_int, 2000.0f32,
                              1000.0f32);
            }
        }
        24 => {
            D_80B7A674 = 0 as libc::c_int as u8_0;
            if Message_GetState(&mut (*globalCtx).msgCtx) as libc::c_int ==
                   TEXT_STATE_DONE as libc::c_int &&
                   Message_ShouldAdvance(globalCtx) as libc::c_int != 0 {
                if D_80B7E07C as libc::c_int == 0 as libc::c_int {
                    (*this).unk_15C = 0 as libc::c_int as s16
                } else {
                    Message_StartTextbox(globalCtx,
                                         0x409c as libc::c_int as u16_0,
                                         0 as *mut Actor);
                    (*this).unk_15C = 20 as libc::c_int as s16
                }
            }
        }
        _ => { }
    };
}
static mut D_80B7AFC4: [s16; 5] =
    [0 as libc::c_int as s16, 1 as libc::c_int as s16,
     2 as libc::c_int as s16, 2 as libc::c_int as s16,
     1 as libc::c_int as s16];
static mut sStreamSoundPos: Vec3f =
    { let mut init = Vec3f{x: 670.0f32, y: 0.0f32, z: -600.0f32,}; init };
static mut sSinkingLureLocationPos: [Vec3s; 4] =
    [{
         let mut init =
             Vec3s{x: -(364 as libc::c_int) as s16,
                   y: -(30 as libc::c_int) as s16,
                   z: -(269 as libc::c_int) as s16,};
         init
     },
     {
         let mut init =
             Vec3s{x: 1129 as libc::c_int as s16,
                   y: 3 as libc::c_int as s16,
                   z: -(855 as libc::c_int) as s16,};
         init
     },
     {
         let mut init =
             Vec3s{x: -(480 as libc::c_int) as s16,
                   y: 0 as libc::c_int as s16,
                   z: -(1055 as libc::c_int) as s16,};
         init
     },
     {
         let mut init =
             Vec3s{x: 553 as libc::c_int as s16,
                   y: -(48 as libc::c_int) as s16,
                   z: -(508 as libc::c_int) as s16,};
         init
     }];
#[no_mangle]
pub unsafe extern "C" fn Fishing_UpdateOwner(mut thisx: *mut Actor,
                                             mut globalCtx2:
                                                 *mut GlobalContext) {
    let mut globalCtx: *mut GlobalContext = globalCtx2;
    let mut this: *mut Fishing = thisx as *mut Fishing;
    let mut sp114: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp108: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut spFC: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut headRotTarget: s16 = 0;
    let mut playerShadowAlpha: s16 = 0;
    let mut target: f32_0 = 0.;
    let mut camAtFraction: f32_0 = 0.;
    let mut lureDistXZ: f32_0 = 0.;
    let mut pad: s32 = 0;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut input: *mut Input =
        &mut *(*globalCtx).state.input.as_mut_ptr().offset(0 as libc::c_int as
                                                               isize) as
            *mut Input;
    playerShadowAlpha = (*player).actor.shape.shadowAlpha as s16;
    if (*player).actor.world.pos.x * (*player).actor.world.pos.x +
           (*player).actor.world.pos.z * (*player).actor.world.pos.z <
           920.0f32 * 920.0f32 {
        Math_ApproachS(&mut playerShadowAlpha, 0 as libc::c_int as s16,
                       1 as libc::c_int as s16, 40 as libc::c_int as s16);
    } else {
        Math_ApproachS(&mut playerShadowAlpha, 200 as libc::c_int as s16,
                       1 as libc::c_int as s16, 40 as libc::c_int as s16);
    }
    (*player).actor.shape.shadowAlpha = playerShadowAlpha as u8_0;
    SkelAnime_Update(&mut (*this).skelAnime);
    if D_80B7A684 as libc::c_int != 0 as libc::c_int ||
           Message_GetState(&mut (*globalCtx).msgCtx) as libc::c_int !=
               TEXT_STATE_NONE as libc::c_int {
        (*this).actor.flags &=
            !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
    } else {
        (*this).actor.flags |=
            ((1 as libc::c_int) << 0 as libc::c_int |
                 (1 as libc::c_int) << 5 as libc::c_int) as libc::c_uint
    }
    if (*this).actor.xzDistToPlayer < 120.0f32 ||
           Message_GetState(&mut (*globalCtx).msgCtx) as libc::c_int !=
               TEXT_STATE_NONE as libc::c_int {
        headRotTarget =
            ((*this).actor.shape.rot.y as libc::c_int -
                 (*this).actor.yawTowardsPlayer as libc::c_int) as s16
    } else { headRotTarget = 0 as libc::c_int as s16 }
    if headRotTarget as libc::c_int > 0x2710 as libc::c_int {
        headRotTarget = 0x2710 as libc::c_int as s16
    } else if (headRotTarget as libc::c_int) < -(0x2710 as libc::c_int) {
        headRotTarget = -(0x2710 as libc::c_int) as s16
    }
    Math_ApproachS(&mut (*this).unk_164, headRotTarget,
                   3 as libc::c_int as s16, 0x1388 as libc::c_int as s16);
    if (*globalCtx).gameplayFrames.wrapping_rem(32 as libc::c_int as
                                                    libc::c_uint) ==
           0 as libc::c_int as libc::c_uint && Rand_ZeroOne() < 0.3f32 {
        (*this).unk_162 = 4 as libc::c_int as s16
    }
    (*this).unk_160 = D_80B7AFC4[(*this).unk_162 as usize];
    if (*this).unk_162 as libc::c_int != 0 as libc::c_int {
        (*this).unk_162 -= 1
    }
    if D_80B7A684 as libc::c_int != 0 as libc::c_int { D_80B7A684 -= 1 }
    if D_80B7A68C as libc::c_int == 0 as libc::c_int &&
           D_80B7E0B6 as libc::c_int != 2 as libc::c_int &&
           D_80B7A694 as libc::c_int > 0 as libc::c_int &&
           D_80B7A688 as libc::c_int == 1 as libc::c_int &&
           D_80B7A684 as libc::c_int == 0 as libc::c_int {
        let mut dx: f32_0 = sOwnerHeadPos.x - sLurePos.x;
        let mut dy: f32_0 = sOwnerHeadPos.y - sLurePos.y;
        let mut dz: f32_0 = sOwnerHeadPos.z - sLurePos.z;
        if sqrtf(dx * dx + dy * dy + dz * dz) < 25.0f32 ||
               (*gGameInfo).data[(13 as libc::c_int * 6 as libc::c_int *
                                      16 as libc::c_int + 77 as libc::c_int)
                                     as usize] as libc::c_int >
                   0 as libc::c_int {
            (*gGameInfo).data[(13 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 77 as libc::c_int) as
                                  usize] = 0 as libc::c_int as s16;
            D_80B7A688 = 0 as libc::c_int as u8_0;
            D_80B7A68C = 1 as libc::c_int as u8_0;
            Message_StartTextbox(globalCtx, 0x4087 as libc::c_int as u16_0,
                                 0 as *mut Actor);
        }
    }
    if D_80B7A688 as libc::c_int == 0 as libc::c_int {
        gSaveContext.highScores[HS_FISHING as libc::c_int as usize] |=
            0x1000 as libc::c_int
    } else if D_80B7A688 as libc::c_int == 1 as libc::c_int {
        gSaveContext.highScores[HS_FISHING as libc::c_int as usize] &=
            !(0x1000 as libc::c_int)
    }
    if ((*gGameInfo).data[(13 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 77 as libc::c_int) as
                              usize] as libc::c_int) < 0 as libc::c_int {
        (*gGameInfo).data[(13 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 77 as libc::c_int) as
                              usize] = 0 as libc::c_int as s16;
        D_80B7A690 = 1 as libc::c_int as u8_0
    }
    if D_80B7A67C as libc::c_int != 0 as libc::c_int {
        D_80B7A67C = D_80B7A67C.wrapping_sub(1);
        if D_80B7A67C as libc::c_int == 0 as libc::c_int {
            Message_StartTextbox(globalCtx, D_80B7E086, 0 as *mut Actor);
        }
    }
    Fishing_HandleOwnerDialog(this, globalCtx);
    D_80B7E14C = 0.0015f32;
    D_80B7A680 += 1;
    if D_80B7E0AC as libc::c_int != 0 as libc::c_int &&
           D_80B7A674 as libc::c_int != 0 {
        Fishing_UpdateLure(this, globalCtx);
    }
    Fishing_UpdateEffects((*globalCtx).specialEffects as *mut FishingEffect,
                          globalCtx);
    Fishing_UpdatePondProps(globalCtx);
    Fishing_UpdateGroupFishes(globalCtx);
    if D_80B7E0AC as libc::c_int != 0 as libc::c_int &&
           D_80B7A6CC as libc::c_int == 0 as libc::c_int &&
           (*player).actor.world.pos.z > 1360.0f32 &&
           fabsf((*player).actor.world.pos.x) < 25.0f32 {
        (*player).actor.world.pos.z = 1360.0f32;
        (*player).actor.speedXZ = 0.0f32;
        if D_80B7A6D0 as libc::c_int == 0 as libc::c_int {
            D_80B7A6CC = 10 as libc::c_int as u8_0
        }
    }
    if sSinkingLureLocation as libc::c_int != 0 as libc::c_int &&
           fabsf((*player).actor.world.pos.x -
                     sSinkingLureLocationPos[(sSinkingLureLocation as
                                                  libc::c_int -
                                                  1 as libc::c_int) as
                                                 usize].x as libc::c_int as
                         libc::c_float) < 25.0f32 &&
           fabsf((*player).actor.world.pos.y -
                     sSinkingLureLocationPos[(sSinkingLureLocation as
                                                  libc::c_int -
                                                  1 as libc::c_int) as
                                                 usize].y as libc::c_int as
                         libc::c_float) < 10.0f32 &&
           fabsf((*player).actor.world.pos.z -
                     sSinkingLureLocationPos[(sSinkingLureLocation as
                                                  libc::c_int -
                                                  1 as libc::c_int) as
                                                 usize].z as libc::c_int as
                         libc::c_float) < 25.0f32 {
        sSinkingLureLocation = 0 as libc::c_int as u8_0;
        D_80B7A6CC = 20 as libc::c_int as u8_0;
        func_800A9F6C(0.0f32, 150 as libc::c_int as u8_0,
                      10 as libc::c_int as u8_0, 10 as libc::c_int as u8_0);
        func_80078884(0x4807 as libc::c_int as u16_0);
        Audio_QueueSeqCmd(((0x1 as libc::c_int) << 28 as libc::c_int |
                               (SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                                   24 as libc::c_int |
                               0x1400ff as libc::c_int) as u32_0);
    }
    if (*gGameInfo).data[(13 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 0 as libc::c_int) as usize]
           as libc::c_int != 0 as libc::c_int {
        (*gGameInfo).data[(13 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 0 as libc::c_int) as usize]
            = 0 as libc::c_int as s16;
        D_80B7E0B6 = 0 as libc::c_int as u8_0;
        D_80B7A6CC = 20 as libc::c_int as u8_0;
        func_800A9F6C(0.0f32, 150 as libc::c_int as u8_0,
                      10 as libc::c_int as u8_0, 10 as libc::c_int as u8_0);
        func_80078884(0x4807 as libc::c_int as u16_0);
        Audio_QueueSeqCmd(((0x1 as libc::c_int) << 28 as libc::c_int |
                               (SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                                   24 as libc::c_int |
                               0x1400ff as libc::c_int) as u32_0);
    }
    if D_80B7A6D0 as libc::c_int != 0 as libc::c_int {
        D_80B7A6D0 = D_80B7A6D0.wrapping_sub(1)
    }
    let mut current_block_311: u64;
    match D_80B7A6CC as libc::c_int {
        1 => {
            let mut camera: *mut Camera = 0 as *mut Camera;
            sCameraId = Gameplay_CreateSubCamera(globalCtx);
            Gameplay_ChangeCameraStatus(globalCtx, 0 as libc::c_int as s16,
                                        1 as libc::c_int as s16);
            Gameplay_ChangeCameraStatus(globalCtx, sCameraId,
                                        7 as libc::c_int as s16);
            camera = Gameplay_GetCamera(globalCtx, 0 as libc::c_int as s16);
            sCameraEye.x = (*camera).eye.x;
            sCameraEye.y = (*camera).eye.y;
            sCameraEye.z = (*camera).eye.z;
            sCameraAt.x = (*camera).at.x;
            sCameraAt.y = (*camera).at.y;
            sCameraAt.z = (*camera).at.z;
            D_80B7A6CC = 2 as libc::c_int as u8_0;
            Interface_ChangeAlpha(12 as libc::c_int as u16_0);
            D_80B7FECC = 0.0f32;
            current_block_311 = 14684468078014671776;
            // fallthrough
        }
        2 => { current_block_311 = 14684468078014671776; }
        3 => {
            let mut camera_0: *mut Camera =
                Gameplay_GetCamera(globalCtx, 0 as libc::c_int as s16);
            (*camera_0).eye = sCameraEye;
            (*camera_0).eyeNext = sCameraEye;
            (*camera_0).at = sCameraAt;
            func_800C08AC(globalCtx, sCameraId, 0 as libc::c_int as s16);
            func_80064534(globalCtx, &mut (*globalCtx).csCtx);
            D_80B7A6CC = 0 as libc::c_int as u8_0;
            sCameraId = 0 as libc::c_int as s16;
            Environment_EnableUnderwaterLights(globalCtx, 0 as libc::c_int);
            (*globalCtx).envCtx.adjFogNear = 0 as libc::c_int as s16;
            (*player).unk_860 = -(5 as libc::c_int) as s16;
            D_80B7E0B0 = 5 as libc::c_int as s16;
            current_block_311 = 10708630372277746932;
        }
        10 => {
            let mut camera_1: *mut Camera = 0 as *mut Camera;
            func_80064520(globalCtx, &mut (*globalCtx).csCtx);
            sCameraId = Gameplay_CreateSubCamera(globalCtx);
            Gameplay_ChangeCameraStatus(globalCtx, 0 as libc::c_int as s16,
                                        1 as libc::c_int as s16);
            Gameplay_ChangeCameraStatus(globalCtx, sCameraId,
                                        7 as libc::c_int as s16);
            func_8002DF54(globalCtx, &mut (*this).actor,
                          5 as libc::c_int as u8_0);
            camera_1 = Gameplay_GetCamera(globalCtx, 0 as libc::c_int as s16);
            sCameraEye.x = (*camera_1).eye.x;
            sCameraEye.y = (*camera_1).eye.y;
            sCameraEye.z = (*camera_1).eye.z;
            sCameraAt.x = (*camera_1).at.x;
            sCameraAt.y = (*camera_1).at.y;
            sCameraAt.z = (*camera_1).at.z;
            Message_StartTextbox(globalCtx, 0x409e as libc::c_int as u16_0,
                                 0 as *mut Actor);
            D_80B7A6CC = 11 as libc::c_int as u8_0;
            func_800A9F6C(0.0f32, 150 as libc::c_int as u8_0,
                          10 as libc::c_int as u8_0,
                          10 as libc::c_int as u8_0);
            current_block_311 = 16621553065303054045;
            // fallthrough
        }
        11 => { current_block_311 = 16621553065303054045; }
        20 => {
            let mut camera_3: *mut Camera = 0 as *mut Camera;
            func_80064520(globalCtx, &mut (*globalCtx).csCtx);
            sCameraId = Gameplay_CreateSubCamera(globalCtx);
            Gameplay_ChangeCameraStatus(globalCtx, 0 as libc::c_int as s16,
                                        1 as libc::c_int as s16);
            Gameplay_ChangeCameraStatus(globalCtx, sCameraId,
                                        7 as libc::c_int as s16);
            func_8002DF54(globalCtx, &mut (*this).actor,
                          5 as libc::c_int as u8_0);
            camera_3 = Gameplay_GetCamera(globalCtx, 0 as libc::c_int as s16);
            sCameraEye.x = (*camera_3).eye.x;
            sCameraEye.y = (*camera_3).eye.y;
            sCameraEye.z = (*camera_3).eye.z;
            sCameraAt.x = (*camera_3).at.x;
            sCameraAt.y = (*camera_3).at.y;
            sCameraAt.z = (*camera_3).at.z;
            Message_StartTextbox(globalCtx, 0x409a as libc::c_int as u16_0,
                                 0 as *mut Actor);
            D_80B7A6CC = 21 as libc::c_int as u8_0;
            D_80B7FEC8 = 45.0f32;
            D_80B7A6D0 = 10 as libc::c_int as u8_0;
            current_block_311 = 16750139345838862497;
            // fallthrough
        }
        21 => { current_block_311 = 16750139345838862497; }
        22 => {
            if D_80B7A6D0 as libc::c_int == 30 as libc::c_int {
                Audio_QueueSeqCmd(((SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                                       24 as libc::c_int | 0x22 as libc::c_int
                                       | 0x900 as libc::c_int) as u32_0);
            }
            D_80B7A6D4 = 1 as libc::c_int as u8_0;
            Math_ApproachF(&mut D_80B7FEE4, 71.0f32, 0.5f32, 3.0f32);
            Matrix_RotateY((*player).actor.shape.rot.y as libc::c_int as
                               libc::c_float / 32768.0f32 *
                               3.14159265358979323846f32,
                           MTXMODE_NEW as libc::c_int as u8_0);
            sp114.x =
                Math_SinS((*globalCtx).gameplayFrames.wrapping_mul(0x1000 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint)
                              as s16);
            sp114.y = D_80B7FEE4;
            sp114.z = -5.0f32;
            if sLinkAge as libc::c_int == 1 as libc::c_int {
                sp114.y -= 20.0f32
            }
            Matrix_MultVec3f(&mut sp114, &mut sp108);
            sSinkingLureBasePos.x = (*player).actor.world.pos.x + sp108.x;
            sSinkingLureBasePos.y = (*player).actor.world.pos.y + sp108.y;
            sSinkingLureBasePos.z = (*player).actor.world.pos.z + sp108.z;
            Math_ApproachF(&mut D_80B7FEC8, 15.0f32, 0.1f32, 0.75f32);
            sp114.x = D_80B7FEC8 - 15.0f32;
            if sLinkAge as libc::c_int != 1 as libc::c_int {
                sp114.y = 60.0f32;
                sp114.z = -30.0f32
            } else { sp114.y = 40.0f32; sp114.z = -35.0f32 }
            Matrix_MultVec3f(&mut sp114, &mut sCameraEye);
            sCameraEye.x += (*player).actor.world.pos.x;
            sCameraEye.y += (*player).actor.world.pos.y;
            sCameraEye.z += (*player).actor.world.pos.z;
            sCameraAt = (*player).actor.world.pos;
            if sLinkAge as libc::c_int != 1 as libc::c_int {
                sCameraAt.y += 62.0f32
            } else { sCameraAt.y += 40.0f32 }
            if D_80B7A6D0 as libc::c_int == 0 as libc::c_int {
                if Message_GetState(&mut (*globalCtx).msgCtx) as libc::c_int
                       == TEXT_STATE_CHOICE as libc::c_int ||
                       Message_GetState(&mut (*globalCtx).msgCtx) as
                           libc::c_int == TEXT_STATE_NONE as libc::c_int {
                    if Message_ShouldAdvance(globalCtx) != 0 {
                        let mut camera_4: *mut Camera =
                            Gameplay_GetCamera(globalCtx,
                                               0 as libc::c_int as s16);
                        Message_CloseTextbox(globalCtx);
                        if (*globalCtx).msgCtx.choiceIndex as libc::c_int ==
                               0 as libc::c_int {
                            D_80B7E0B6 = 2 as libc::c_int as u8_0;
                            D_80B7E082 = 0 as libc::c_int as u8_0
                        }
                        (*camera_4).eye = sCameraEye;
                        (*camera_4).eyeNext = sCameraEye;
                        (*camera_4).at = sCameraAt;
                        func_800C08AC(globalCtx, sCameraId,
                                      0 as libc::c_int as s16);
                        func_80064534(globalCtx, &mut (*globalCtx).csCtx);
                        func_8002DF54(globalCtx, &mut (*this).actor,
                                      7 as libc::c_int as u8_0);
                        D_80B7A6CC = 0 as libc::c_int as u8_0;
                        sCameraId = 0 as libc::c_int as s16;
                        (*player).unk_860 = -(5 as libc::c_int) as s16;
                        D_80B7E0B0 = 5 as libc::c_int as s16;
                        D_80B7A6D4 = 0 as libc::c_int as u8_0;
                        D_80B7E0A6 = 20 as libc::c_int as s16;
                        Environment_EnableUnderwaterLights(globalCtx,
                                                           0 as libc::c_int);
                        (*globalCtx).envCtx.adjFogNear =
                            0 as libc::c_int as s16
                    }
                }
            }
            current_block_311 = 10708630372277746932;
        }
        0 | 100 | _ => { current_block_311 = 10708630372277746932; }
    }
    match current_block_311 {
        16621553065303054045 => {
            (*player).actor.world.pos.z = 1360.0f32;
            (*player).actor.speedXZ = 0.0f32;
            if Message_GetState(&mut (*globalCtx).msgCtx) as libc::c_int ==
                   TEXT_STATE_NONE as libc::c_int {
                let mut camera_2: *mut Camera =
                    Gameplay_GetCamera(globalCtx, 0 as libc::c_int as s16);
                (*camera_2).eye = sCameraEye;
                (*camera_2).eyeNext = sCameraEye;
                (*camera_2).at = sCameraAt;
                func_800C08AC(globalCtx, sCameraId, 0 as libc::c_int as s16);
                func_80064534(globalCtx, &mut (*globalCtx).csCtx);
                func_8002DF54(globalCtx, &mut (*this).actor,
                              7 as libc::c_int as u8_0);
                D_80B7A6CC = 0 as libc::c_int as u8_0;
                sCameraId = 0 as libc::c_int as s16;
                D_80B7A6D0 = 30 as libc::c_int as u8_0;
                Environment_EnableUnderwaterLights(globalCtx,
                                                   0 as libc::c_int);
                (*globalCtx).envCtx.adjFogNear = 0 as libc::c_int as s16
            }
        }
        14684468078014671776 => {
            ShrinkWindow_SetVal(0x1b as libc::c_int);
            spFC.x = sLurePos.x - (*player).actor.world.pos.x;
            spFC.z = sLurePos.z - (*player).actor.world.pos.z;
            lureDistXZ = sqrtf(spFC.x * spFC.x + spFC.z * spFC.z);
            Matrix_RotateY(Math_Atan2F(spFC.z, spFC.x),
                           MTXMODE_NEW as libc::c_int as u8_0);
            sp114.x = 0.0f32;
            sp114.y = 0.0f32;
            sp114.z = 100.0f32;
            Matrix_MultVec3f(&mut sp114, &mut spFC);
            if D_80B7A694 as libc::c_int == 1 as libc::c_int {
                camAtFraction = 0.2f32
            } else { camAtFraction = 0.1f32 }
            Math_ApproachF(&mut sCameraAt.x, sLurePos.x, camAtFraction,
                           fabsf(spFC.x) * D_80B7FECC);
            Math_ApproachF(&mut sCameraAt.y, sLurePos.y, camAtFraction,
                           50.0f32 * D_80B7FECC);
            Math_ApproachF(&mut sCameraAt.z, sLurePos.z, camAtFraction,
                           fabsf(spFC.z) * D_80B7FECC);
            sp114.x = 0.0f32 - D_80B7FED0;
            if sLinkAge as libc::c_int != 1 as libc::c_int {
                sp114.y = 80.0f32
            } else { sp114.y = 55.0f32 }
            sp114.z = -80.0f32;
            Matrix_MultVec3f(&mut sp114, &mut sp108);
            sp108.x += (*player).actor.world.pos.x;
            sp108.y += (*player).actor.world.pos.y;
            sp108.z += (*player).actor.world.pos.z;
            Math_ApproachF(&mut D_80B7FED0, 30.0f32, 0.1f32, 0.4f32);
            if !((*input).press.button as libc::c_int |
                     !(0x2000 as libc::c_int)) == 0 as libc::c_int {
                if D_80B7E088 as libc::c_int >= 0 as libc::c_int &&
                       D_80B7E122 as libc::c_int == 0 as libc::c_int {
                    D_80B7E088 += 1;
                    if D_80B7E088 as libc::c_int >= 4 as libc::c_int {
                        D_80B7E088 = 0 as libc::c_int as s8
                    }
                    if D_80B7E088 as libc::c_int == 0 as libc::c_int ||
                           D_80B7E088 as libc::c_int == 3 as libc::c_int {
                        func_80078884(0x4814 as libc::c_int as u16_0);
                    } else { func_80078884(0x4813 as libc::c_int as u16_0); }
                }
            }
            if D_80B7A694 as libc::c_int >= 3 as libc::c_int {
                if lureDistXZ < 110.0f32 {
                    D_80B7E088 = -(1 as libc::c_int) as s8
                } else if lureDistXZ > 300.0f32 &&
                              (D_80B7E088 as libc::c_int) < 0 as libc::c_int {
                    D_80B7E088 = 0 as libc::c_int as s8
                }
            }
            if D_80B7E088 as libc::c_int > 0 as libc::c_int {
                let mut dist: f32_0 = 0.;
                let mut offset: f32_0 = 0.;
                let mut factor: f32_0 = 0.;
                dist = sqrtf(spFC.x * spFC.x + spFC.z * spFC.z) * 0.001f32;
                if dist > 1.0f32 { dist = 1.0f32 }
                if D_80B7E088 as libc::c_int == 2 as libc::c_int {
                    offset = 0.3f32
                } else { offset = 0.1f32 }
                factor = 0.4f32 + offset + dist * 0.4f32;
                sp108.x += (sLurePos.x - sp108.x) * factor;
                sp108.y += (sLurePos.y - sp108.y) * factor + 20.0f32;
                sp108.z += (sLurePos.z - sp108.z) * factor;
                D_80B7E14C = 0.0005000001f32
            }
            sp114.x = 0.0f32;
            sp114.y = 0.0f32;
            sp114.z = 100.0f32;
            Matrix_MultVec3f(&mut sp114, &mut spFC);
            Math_ApproachF(&mut sCameraEye.x, sp108.x, 0.3f32,
                           fabsf(spFC.x) * D_80B7FECC);
            Math_ApproachF(&mut sCameraEye.y, sp108.y, 0.3f32,
                           20.0f32 * D_80B7FECC);
            Math_ApproachF(&mut sCameraEye.z, sp108.z, 0.3f32,
                           fabsf(spFC.z) * D_80B7FECC);
        }
        16750139345838862497 => {
            if D_80B7A6D0 as libc::c_int == 0 as libc::c_int &&
                   Message_ShouldAdvance(globalCtx) as libc::c_int != 0 {
                D_80B7A6CC = 22 as libc::c_int as u8_0;
                D_80B7A6D0 = 40 as libc::c_int as u8_0;
                func_8002DF54(globalCtx, &mut (*this).actor,
                              0x1c as libc::c_int as u8_0);
                D_80B7FEE4 = 0.0f32
            }
        }
        _ => { }
    }
    if sCameraId as libc::c_int != 0 as libc::c_int {
        Gameplay_CameraSetAtEye(globalCtx, sCameraId, &mut sCameraAt,
                                &mut sCameraEye);
        Math_ApproachF(&mut D_80B7FECC, 1.0f32, 1.0f32, 0.02f32);
        if sCameraEye.y <=
               (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface as
                   libc::c_int as libc::c_float + 1.0f32 {
            Environment_EnableUnderwaterLights(globalCtx, 1 as libc::c_int);
            if D_80B7E076 as libc::c_int != 0 as libc::c_int {
                (*globalCtx).envCtx.adjFogNear = -(0xb2 as libc::c_int) as s16
            } else {
                (*globalCtx).envCtx.adjFogNear = -(0x2e as libc::c_int) as s16
            }
        } else {
            Environment_EnableUnderwaterLights(globalCtx, 0 as libc::c_int);
            (*globalCtx).envCtx.adjFogNear = 0 as libc::c_int as s16
        }
    }
    if (*player).actor.floorHeight <
           (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface as
               libc::c_int as libc::c_float - 3.0f32 &&
           (*player).actor.world.pos.y < (*player).actor.floorHeight + 3.0f32
           && (*player).actor.speedXZ > 1.0f32 &&
           (*globalCtx).gameplayFrames.wrapping_rem(2 as libc::c_int as
                                                        libc::c_uint) ==
               0 as libc::c_int as libc::c_uint {
        let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
        pos.x = Rand_CenteredFloat(20.0f32) + (*player).actor.world.pos.x;
        pos.z = Rand_CenteredFloat(20.0f32) + (*player).actor.world.pos.z;
        pos.y = (*player).actor.floorHeight + 5.0f32;
        Fishing_SpawnWaterDust(0 as *mut Vec3f,
                               (*globalCtx).specialEffects as
                                   *mut FishingEffect, &mut pos, 0.5f32);
    }
    if (*player).actor.floorHeight <
           (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface as
               libc::c_int as libc::c_float &&
           (*player).actor.floorHeight >
               (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface as
                   libc::c_int as libc::c_float - 10.0f32 &&
           (*player).actor.speedXZ >= 4.0f32 &&
           (*globalCtx).gameplayFrames.wrapping_rem(4 as libc::c_int as
                                                        libc::c_uint) ==
               0 as libc::c_int as libc::c_uint {
        let mut i: s16 = 0;
        i = 0 as libc::c_int as s16;
        while (i as libc::c_int) < 10 as libc::c_int {
            let mut pos_0: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
            let mut vel: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
            let mut speedXZ: f32_0 = 0.;
            let mut angle: f32_0 = 0.;
            speedXZ = Rand_ZeroFloat(1.5f32) + 1.5f32;
            angle = Rand_ZeroFloat(6.28f32);
            vel.x = sinf(angle) * speedXZ;
            vel.z = cosf(angle) * speedXZ;
            vel.y = Rand_ZeroFloat(3.0f32) + 2.0f32;
            pos_0 = (*player).actor.world.pos;
            pos_0.x += 2.0f32 * vel.x;
            pos_0.y =
                (*(*(*globalCtx).colCtx.colHeader).waterBoxes).ySurface as
                    f32_0;
            pos_0.z += 2.0f32 * vel.z;
            Fishing_SpawnDustSplash(0 as *mut Vec3f,
                                    (*globalCtx).specialEffects as
                                        *mut FishingEffect, &mut pos_0,
                                    &mut vel,
                                    Rand_ZeroFloat(0.01f32) + 0.020000001f32);
            i += 1
        }
    }
    if (*gGameInfo).data[(16 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 15 as libc::c_int) as usize]
           as libc::c_int != 0 as libc::c_int {
        if D_80B7A654 as libc::c_int !=
               (*gGameInfo).data[(16 as libc::c_int * 6 as libc::c_int *
                                      16 as libc::c_int + 15 as libc::c_int)
                                     as usize] as libc::c_int -
                   1 as libc::c_int {
            if D_80B7A654 as libc::c_int == 0 as libc::c_int {
                (*globalCtx).envCtx.gloomySkyMode = 1 as libc::c_int as u8_0
            } else {
                (*globalCtx).envCtx.gloomySkyMode = 2 as libc::c_int as u8_0
            }
        }
        D_80B7A654 =
            ((*gGameInfo).data[(16 as libc::c_int * 6 as libc::c_int *
                                    16 as libc::c_int + 15 as libc::c_int) as
                                   usize] as libc::c_int - 1 as libc::c_int)
                as u8_0
    }
    if (*gGameInfo).data[(16 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 14 as libc::c_int) as usize]
           as libc::c_int == 1 as libc::c_int {
        (*globalCtx).envCtx.gloomySkyMode = 1 as libc::c_int as u8_0
    }
    if (*gGameInfo).data[(16 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 14 as libc::c_int) as usize]
           as libc::c_int == -(1 as libc::c_int) {
        (*globalCtx).envCtx.gloomySkyMode = 2 as libc::c_int as u8_0
    }
    (*gGameInfo).data[(16 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 14 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    osSyncPrintf(b"\x1b[32m\x00" as *const u8 as *const libc::c_char);
    osSyncPrintf(b"zelda_time %x\n\x00" as *const u8 as *const libc::c_char,
                 gSaveContext.dayTime as libc::c_int);
    osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
    if D_80B7E077 as libc::c_int >= 2 as libc::c_int {
        D_80B7E077 = D_80B7E077.wrapping_sub(1)
    }
    if D_80B7E077 as libc::c_int == 1 as libc::c_int &&
           Message_GetState(&mut (*globalCtx).msgCtx) as libc::c_int ==
               TEXT_STATE_NONE as libc::c_int &&
           D_80B7A680 & 0xfff as libc::c_int == 0xfff as libc::c_int {
        D_80B7E077 = 200 as libc::c_int as u8_0;
        if Rand_ZeroOne() < 0.5f32 {
            D_80B7A654 =
                (Rand_ZeroFloat(10.0f32) as u8_0 as libc::c_int +
                     5 as libc::c_int) as u8_0;
            (*globalCtx).envCtx.gloomySkyMode = 1 as libc::c_int as u8_0
        } else {
            D_80B7A654 = 0 as libc::c_int as u8_0;
            (*globalCtx).envCtx.gloomySkyMode = 2 as libc::c_int as u8_0
        }
    }
    Math_ApproachF(&mut D_80B7A650, D_80B7A654 as f32_0, 1.0f32, 0.05f32);
    if D_80B7A650 > 0.0f32 {
        target = D_80B7A650 * 0.03f32 + 0.8f32;
        if target > 1.2f32 { target = 1.2f32 }
        Math_ApproachF(&mut D_80B7A668, target, 1.0f32, 0.01f32);
    }
    target = (10.0f32 - D_80B7A650) * 150.1f32;
    if target < 0.0f32 { target = 0.0f32 }
    Math_ApproachF(&mut D_80B7A65C.z, target, 1.0f32, 5.0f32);
    if D_80B7A65C.z < 1500.0f32 {
        func_800F436C(&mut D_80B7A65C,
                      (0x28da as libc::c_int - 0x800 as libc::c_int) as u16_0,
                      D_80B7A668);
    }
    if D_80B7A654 as libc::c_int != 0 as libc::c_int {
        Math_ApproachF(&mut D_80B7A658, -200.0f32, 1.0f32, 2.0f32);
    } else { Math_ApproachZeroF(&mut D_80B7A658, 1.0f32, 2.0f32); }
    (*globalCtx).envCtx.adjLight1Color[2 as libc::c_int as usize] =
        D_80B7A658 as s16;
    (*globalCtx).envCtx.adjLight1Color[1 as libc::c_int as usize] =
        (*globalCtx).envCtx.adjLight1Color[2 as libc::c_int as usize];
    (*globalCtx).envCtx.adjLight1Color[0 as libc::c_int as usize] =
        (*globalCtx).envCtx.adjLight1Color[1 as libc::c_int as usize];
    if D_80B7A650 as u8_0 as libc::c_int > 0 as libc::c_int {
        let mut pad_0: s32 = 0;
        let mut camera_5: *mut Camera =
            Gameplay_GetCamera(globalCtx, 0 as libc::c_int as s16);
        let mut i_0: s16 = 0;
        let mut pad1: s32 = 0;
        let mut pos_1: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
        let mut rot: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
        let mut projectedPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
        let mut pad2: s32 = 0;
        rot.x = 3.14159265358979323846f32 / 2.0f32 + 0.1f32;
        rot.y = 1.0f32;
        rot.z =
            Camera_GetInputDirYaw(camera_5) as libc::c_int as libc::c_float *
                -(3.14159265358979323846f32 /
                      32768 as libc::c_int as libc::c_float) + rot.y;
        i_0 = 0 as libc::c_int as s16;
        while (i_0 as libc::c_int) < D_80B7A650 as u8_0 as libc::c_int {
            pos_1.x = Rand_CenteredFloat(700.0f32) + (*globalCtx).view.eye.x;
            pos_1.y = Rand_ZeroFloat(100.0f32) + 150.0f32 - 170.0f32;
            pos_1.z = Rand_CenteredFloat(700.0f32) + (*globalCtx).view.eye.z;
            if pos_1.z < 1160.0f32 {
                SkinMatrix_Vec3fMtxFMultXYZW(&mut (*globalCtx).viewProjectionMtxF,
                                             &mut pos_1, &mut projectedPos,
                                             &mut sProjectedW);
                if projectedPos.z < 0.0f32 {
                    i_0 -= 1
                } else {
                    Fishing_SpawnRainDrop((*globalCtx).specialEffects as
                                              *mut FishingEffect, &mut pos_1,
                                          &mut rot);
                }
            }
            i_0 += 1
        }
    }
    SkinMatrix_Vec3fMtxFMultXYZW(&mut (*globalCtx).viewProjectionMtxF,
                                 &mut sStreamSoundPos,
                                 &mut sStreamSoundProjectedPos,
                                 &mut sProjectedW);
    func_80078914(&mut sStreamSoundProjectedPos,
                  (0x2828 as libc::c_int - 0x800 as libc::c_int) as u16_0);
    gSaveContext.minigameScore =
        (D_80B7A678 as f32_0 * D_80B7A678 as f32_0 * 0.0036f32 + 0.5f32) as
            u16_0;
    if (*gGameInfo).data[(25 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 26 as libc::c_int) as usize]
           as libc::c_int != 0 as libc::c_int {
        (*gGameInfo).data[(25 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 26 as libc::c_int) as
                              usize] = 0 as libc::c_int as s16;
        Message_StartTextbox(globalCtx,
                             (0x407b as libc::c_int +
                                  (*gGameInfo).data[(25 as libc::c_int *
                                                         6 as libc::c_int *
                                                         16 as libc::c_int +
                                                         27 as libc::c_int) as
                                                        usize] as libc::c_int)
                                 as u16_0, 0 as *mut Actor);
    }
    osSyncPrintf(b"HI_SCORE = %x\n\x00" as *const u8 as *const libc::c_char,
                 gSaveContext.highScores[HS_FISHING as libc::c_int as usize]);
}
#[no_mangle]
pub unsafe extern "C" fn Fishing_OwnerOverrideLimbDraw(mut globalCtx:
                                                           *mut GlobalContext,
                                                       mut limbIndex: s32,
                                                       mut dList:
                                                           *mut *mut Gfx,
                                                       mut pos: *mut Vec3f,
                                                       mut rot: *mut Vec3s,
                                                       mut thisx:
                                                           *mut libc::c_void)
 -> s32 {
    let mut this: *mut Fishing = thisx as *mut Fishing;
    if limbIndex == 8 as libc::c_int {
        // Head
        (*rot).x =
            ((*rot).x as libc::c_int - (*this).unk_164 as libc::c_int) as s16
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Fishing_OwnerPostLimbDraw(mut globalCtx:
                                                       *mut GlobalContext,
                                                   mut limbIndex: s32,
                                                   mut dList: *mut *mut Gfx,
                                                   mut rot: *mut Vec3s,
                                                   mut thisx:
                                                       *mut libc::c_void) {
    if limbIndex == 8 as libc::c_int {
        // Head
        let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
        let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
        __gfxCtx = (*globalCtx).state.gfxCtx;
        Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                        b"../z_fishing.c\x00" as *const u8 as
                            *const libc::c_char, 9134 as libc::c_int);
        Matrix_MultVec3f(&mut sZeroVec, &mut sOwnerHeadPos);
        if D_80B7A688 as libc::c_int == 1 as libc::c_int {
            let fresh104 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g: *mut Gfx = fresh104;
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
                gSegments[((gFishingOwnerHatDL.as_mut_ptr() as u32_0) <<
                               4 as libc::c_int >> 28 as libc::c_int) as
                              usize].wrapping_add(gFishingOwnerHatDL.as_mut_ptr()
                                                      as u32_0 &
                                                      0xffffff as libc::c_int
                                                          as
                                                          libc::c_uint).wrapping_add(0x80000000
                                                                                         as
                                                                                         libc::c_uint)
                    as *mut libc::c_void as libc::c_uint
        } else if D_80B7A688 as libc::c_int == 2 as libc::c_int {
            let fresh105 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_0: *mut Gfx = fresh105;
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
                gSegments[((gFishingOwnerHairDL.as_mut_ptr() as u32_0) <<
                               4 as libc::c_int >> 28 as libc::c_int) as
                              usize].wrapping_add(gFishingOwnerHairDL.as_mut_ptr()
                                                      as u32_0 &
                                                      0xffffff as libc::c_int
                                                          as
                                                          libc::c_uint).wrapping_add(0x80000000
                                                                                         as
                                                                                         libc::c_uint)
                    as *mut libc::c_void as libc::c_uint
        }
        Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                         b"../z_fishing.c\x00" as *const u8 as
                             *const libc::c_char, 9142 as libc::c_int);
    };
}
static mut sFishingOwnerEyeTexs: [*mut libc::c_void; 3] =
    unsafe {
        [gFishingOwnerEyeOpenTex.as_ptr() as *mut _ as *mut libc::c_void,
         gFishingOwnerEyeHalfTex.as_ptr() as *mut _ as *mut libc::c_void,
         gFishingOwnerEyeClosedTex.as_ptr() as *mut _ as *mut libc::c_void]
    };
#[no_mangle]
pub unsafe extern "C" fn Fishing_DrawOwner(mut thisx: *mut Actor,
                                           mut globalCtx:
                                               *mut GlobalContext) {
    let mut pad: s32 = 0;
    let mut this: *mut Fishing = thisx as *mut Fishing;
    let mut input: *mut Input =
        &mut *(*globalCtx).state.input.as_mut_ptr().offset(0 as libc::c_int as
                                                               isize) as
            *mut Input;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_fishing.c\x00" as *const u8 as *const libc::c_char,
                    9156 as libc::c_int);
    func_80093D18((*globalCtx).state.gfxCtx);
    func_80093D84((*globalCtx).state.gfxCtx);
    if (*this).actor.projectedPos.z < 1500.0f32 &&
           fabsf((*this).actor.projectedPos.x) <
               100.0f32 + (*this).actor.projectedPos.z {
        let fresh106 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g: *mut Gfx = fresh106;
        (*_g).words.w0 =
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
        (*_g).words.w1 =
            gSegments[((sFishingOwnerEyeTexs[(*this).unk_160 as usize] as
                            u32_0) << 4 as libc::c_int >> 28 as libc::c_int)
                          as
                          usize].wrapping_add(sFishingOwnerEyeTexs[(*this).unk_160
                                                                       as
                                                                       usize]
                                                  as u32_0 &
                                                  0xffffff as libc::c_int as
                                                      libc::c_uint).wrapping_add(0x80000000
                                                                                     as
                                                                                     libc::c_uint)
                as *mut libc::c_void as libc::c_uint;
        SkelAnime_DrawFlexOpa(globalCtx, (*this).skelAnime.skeleton,
                              (*this).skelAnime.jointTable,
                              (*this).skelAnime.dListCount as s32,
                              Some(Fishing_OwnerOverrideLimbDraw as
                                       unsafe extern "C" fn(_:
                                                                *mut GlobalContext,
                                                            _: s32,
                                                            _: *mut *mut Gfx,
                                                            _: *mut Vec3f,
                                                            _: *mut Vec3s,
                                                            _:
                                                                *mut libc::c_void)
                                           -> s32),
                              Some(Fishing_OwnerPostLimbDraw as
                                       unsafe extern "C" fn(_:
                                                                *mut GlobalContext,
                                                            _: s32,
                                                            _: *mut *mut Gfx,
                                                            _: *mut Vec3s,
                                                            _:
                                                                *mut libc::c_void)
                                           -> ()), this as *mut libc::c_void);
    }
    Fishing_DrawPondProps(globalCtx);
    Fishing_DrawEffects((*globalCtx).specialEffects as *mut FishingEffect,
                        globalCtx);
    Fishing_DrawGroupFishes(globalCtx);
    Fishing_DrawStreamSplash(globalCtx);
    if D_80B7E0A6 as libc::c_int != 0 as libc::c_int {
        D_80B7E0A6 -= 1;
        if D_80B7E0A6 as libc::c_int == 0 as libc::c_int {
            if sLinkAge as libc::c_int != 1 as libc::c_int {
                Audio_QueueSeqCmd(((SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                                       24 as libc::c_int |
                                       0x19 as libc::c_int) as u32_0);
            } else {
                Audio_QueueSeqCmd(((SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                                       24 as libc::c_int |
                                       0x27 as libc::c_int) as u32_0);
            }
            if sLinkAge as libc::c_int != 1 as libc::c_int {
                Audio_QueueSeqCmd(((SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                                       24 as libc::c_int |
                                       0x19 as libc::c_int) as u32_0);
            } else {
                Audio_QueueSeqCmd(((SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                                       24 as libc::c_int |
                                       0x27 as libc::c_int) as u32_0);
            }
        }
    }
    if D_80B7E0AC as libc::c_int != 0 as libc::c_int &&
           D_80B7A674 as libc::c_int != 0 {
        Fishing_DrawRod(globalCtx);
        Fishing_UpdateLinePos(sReelLinePos.as_mut_ptr());
        Fishing_UpdateLine(globalCtx, &mut sRodTipPos,
                           sReelLinePos.as_mut_ptr(),
                           sReelLineRot.as_mut_ptr(),
                           sReelLineUnk.as_mut_ptr());
        Fishing_DrawLureAndLine(globalCtx, sReelLinePos.as_mut_ptr(),
                                sReelLineRot.as_mut_ptr());
        D_80B7A6C4 = (*input).rel.stick_x as s16;
        D_80B7A6C8 = (*input).rel.stick_y as s16
    }
    D_80B7A674 = 1 as libc::c_int as u8_0;
    Matrix_Translate(130.0f32, 40.0f32, 1300.0f32,
                     MTXMODE_NEW as libc::c_int as u8_0);
    Matrix_Scale(0.08f32, 0.12f32, 0.14f32,
                 MTXMODE_APPLY as libc::c_int as u8_0);
    let fresh107 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_0: *mut Gfx = fresh107;
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
                      b"../z_fishing.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      9297 as libc::c_int) as libc::c_uint;
    let fresh108 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_1: *mut Gfx = fresh108;
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
                      b"../z_fishing.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      9298 as libc::c_int) as libc::c_uint;
    let fresh109 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_2: *mut Gfx = fresh109;
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
    (*_g_2).words.w1 = gFishingAquariumBottomDL.as_mut_ptr() as libc::c_uint;
    let fresh110 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_3: *mut Gfx = fresh110;
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
    (*_g_3).words.w1 =
        gFishingAquariumContainerDL.as_mut_ptr() as libc::c_uint;
    if D_80B7E0AC as libc::c_int != 0 as libc::c_int &&
           D_80B7E0B6 as libc::c_int == 2 as libc::c_int {
        Fishing_DrawSinkingLure(globalCtx);
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_fishing.c\x00" as *const u8 as
                         *const libc::c_char, 9305 as libc::c_int);
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
