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
    fn DmaMgr_SendRequest1(ram0: *mut libc::c_void, vrom: u32_0, size: u32_0,
                           file: *const libc::c_char, line: s32) -> s32;
    #[no_mangle]
    fn __assert(exp: *const libc::c_char, file: *const libc::c_char,
                line: s32);
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn bzero(__s: *mut libc::c_void, __n: u32_0);
    #[no_mangle]
    fn func_80026400(globalCtx: *mut GlobalContext, color: *mut Color_RGBA8,
                     arg2: s16, arg3: s16);
    #[no_mangle]
    fn func_80026608(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn func_80026860(globalCtx: *mut GlobalContext, color: *mut Color_RGBA8,
                     arg2: s16, arg3: s16);
    #[no_mangle]
    fn func_80026A6C(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn Effect_GetGlobalCtx() -> *mut GlobalContext;
    #[no_mangle]
    fn Effect_DrawAll(gfxCtx: *mut GraphicsContext);
    #[no_mangle]
    fn EffectSs_DrawAll(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn func_8002857C(globalCtx: *mut GlobalContext, pos: *mut Vec3f,
                     velocity: *mut Vec3f, accel: *mut Vec3f);
    #[no_mangle]
    fn func_8002865C(globalCtx: *mut GlobalContext, pos: *mut Vec3f,
                     velocity: *mut Vec3f, accel: *mut Vec3f, scale: s16,
                     scaleStep: s16);
    #[no_mangle]
    fn func_800286CC(globalCtx: *mut GlobalContext, pos: *mut Vec3f,
                     velocity: *mut Vec3f, accel: *mut Vec3f, scale: s16,
                     scaleStep: s16);
    #[no_mangle]
    fn EffectSsKiraKira_SpawnSmall(globalCtx: *mut GlobalContext,
                                   pos: *mut Vec3f, velocity: *mut Vec3f,
                                   accel: *mut Vec3f,
                                   primColor: *mut Color_RGBA8,
                                   envColor: *mut Color_RGBA8);
    #[no_mangle]
    fn EffectSsKiraKira_SpawnDispersed(globalCtx: *mut GlobalContext,
                                       pos: *mut Vec3f, velocity: *mut Vec3f,
                                       accel: *mut Vec3f,
                                       primColor: *mut Color_RGBA8,
                                       envColor: *mut Color_RGBA8, scale: s16,
                                       life: s32);
    #[no_mangle]
    fn EffectSsGRipple_Spawn(globalCtx: *mut GlobalContext, pos: *mut Vec3f,
                             radius: s16, radiusMax: s16, life: s16);
    #[no_mangle]
    fn ActorOverlayTable_Init();
    #[no_mangle]
    fn ActorOverlayTable_Cleanup();
    #[no_mangle]
    fn func_80038A28(poly: *mut CollisionPoly, tx: f32_0, ty: f32_0,
                     tz: f32_0, dest: *mut MtxF);
    #[no_mangle]
    fn BgCheck_EntityRaycastFloor2(globalCtx: *mut GlobalContext,
                                   colCtx: *mut CollisionContext,
                                   outPoly: *mut *mut CollisionPoly,
                                   pos: *mut Vec3f) -> f32_0;
    #[no_mangle]
    fn BgCheck_EntityRaycastFloor5(globalCtx: *mut GlobalContext,
                                   colCtx: *mut CollisionContext,
                                   outPoly: *mut *mut CollisionPoly,
                                   bgId: *mut s32, actor: *mut Actor,
                                   pos: *mut Vec3f) -> f32_0;
    #[no_mangle]
    fn BgCheck_EntitySphVsWall3(colCtx: *mut CollisionContext,
                                posResult: *mut Vec3f, posNext: *mut Vec3f,
                                posPrev: *mut Vec3f, radius: f32_0,
                                outPoly: *mut *mut CollisionPoly,
                                outBgId: *mut s32, actor: *mut Actor,
                                checkHeight: f32_0) -> s32;
    #[no_mangle]
    fn BgCheck_EntitySphVsWall4(colCtx: *mut CollisionContext,
                                posResult: *mut Vec3f, posNext: *mut Vec3f,
                                posPrev: *mut Vec3f, radius: f32_0,
                                outPoly: *mut *mut CollisionPoly,
                                outBgId: *mut s32, actor: *mut Actor,
                                checkHeight: f32_0) -> s32;
    #[no_mangle]
    fn BgCheck_EntityCheckCeiling(colCtx: *mut CollisionContext,
                                  arg1: *mut f32_0, arg2: *mut Vec3f,
                                  arg3: f32_0,
                                  outPoly: *mut *mut CollisionPoly,
                                  outBgId: *mut s32, actor: *mut Actor)
     -> s32;
    #[no_mangle]
    fn BgCheck_CameraLineTest1(colCtx: *mut CollisionContext,
                               posA: *mut Vec3f, posB: *mut Vec3f,
                               posResult: *mut Vec3f,
                               outPoly: *mut *mut CollisionPoly, chkWall: s32,
                               chkFloor: s32, chkCeil: s32, chkOneFace: s32,
                               bgId: *mut s32) -> s32;
    #[no_mangle]
    fn func_8003F8EC(globalCtx: *mut GlobalContext,
                     dyna: *mut DynaCollisionContext, actor: *mut Actor);
    #[no_mangle]
    fn DynaPoly_Setup(globalCtx: *mut GlobalContext,
                      dyna: *mut DynaCollisionContext);
    #[no_mangle]
    fn DynaPoly_UpdateBgActorTransforms(globalCtx: *mut GlobalContext,
                                        dyna: *mut DynaCollisionContext);
    #[no_mangle]
    fn func_80041D4C(colCtx: *mut CollisionContext, poly: *mut CollisionPoly,
                     bgId: s32) -> u32_0;
    #[no_mangle]
    fn SurfaceType_GetSfx(colCtx: *mut CollisionContext,
                          poly: *mut CollisionPoly, bgId: s32) -> u16_0;
    #[no_mangle]
    fn SurfaceType_IsIgnoredByProjectiles(colCtx: *mut CollisionContext,
                                          poly: *mut CollisionPoly, bgId: s32)
     -> s32;
    #[no_mangle]
    fn WaterBox_GetSurface1(globalCtx: *mut GlobalContext,
                            colCtx: *mut CollisionContext, x: f32_0, z: f32_0,
                            ySurface: *mut f32_0,
                            outWaterBox: *mut *mut WaterBox) -> s32;
    #[no_mangle]
    fn func_80043334(colCtx: *mut CollisionContext, actor: *mut Actor,
                     bgId: s32);
    #[no_mangle]
    fn func_800433A4(colCtx: *mut CollisionContext, bgId: s32,
                     actor: *mut Actor) -> s32;
    #[no_mangle]
    fn Camera_ChangeMode(camera: *mut Camera, mode: s16) -> s32;
    #[no_mangle]
    fn Camera_ChangeSetting(camera: *mut Camera, setting: s16) -> s32;
    #[no_mangle]
    fn CollisionCheck_ClearContext(globalCtx: *mut GlobalContext,
                                   colChkCtx: *mut CollisionCheckContext);
    #[no_mangle]
    fn CollisionCheck_DrawCollision(globalCtx: *mut GlobalContext,
                                    colChkCtx: *mut CollisionCheckContext);
    #[no_mangle]
    fn CollisionCheck_BlueBlood(globalCtx: *mut GlobalContext,
                                collider: *mut Collider, v: *mut Vec3f);
    #[no_mangle]
    fn CollisionCheck_InitInfo(info: *mut CollisionCheckInfo);
    #[no_mangle]
    fn CollisionCheck_ResetDamage(info: *mut CollisionCheckInfo);
    #[no_mangle]
    fn CollisionCheck_SpawnShieldParticlesMetal(globalCtx: *mut GlobalContext,
                                                v: *mut Vec3f);
    #[no_mangle]
    fn CollisionCheck_CylSideVsLineSeg(radius: f32_0, height: f32_0,
                                       offset: f32_0, actorPos: *mut Vec3f,
                                       itemPos: *mut Vec3f,
                                       itemProjPos: *mut Vec3f,
                                       out1: *mut Vec3f, out2: *mut Vec3f)
     -> s32;
    #[no_mangle]
    fn Audio_PlaySoundAtPosition(globalCtx: *mut GlobalContext,
                                 pos: *mut Vec3f, duration: s32,
                                 sfxId: u16_0);
    #[no_mangle]
    fn func_8006DC68(globalCtx: *mut GlobalContext, player: *mut Player);
    #[no_mangle]
    fn Lib_MemSet(dest: *mut u8_0, size: size_t, val: u8_0);
    #[no_mangle]
    fn Math_CosS(angle: s16) -> f32_0;
    #[no_mangle]
    fn Math_SinS(angle: s16) -> f32_0;
    #[no_mangle]
    fn Math_StepToS(pValue: *mut s16, target: s16, step: s16) -> s32;
    #[no_mangle]
    fn Math_StepToF(pValue: *mut f32_0, target: f32_0, step: f32_0) -> s32;
    #[no_mangle]
    fn Rand_S16Offset(base: s16, range: s16) -> s16;
    #[no_mangle]
    fn Math_Vec3f_Copy(dest: *mut Vec3f, src: *mut Vec3f);
    #[no_mangle]
    fn Math_Vec3f_DistXYZ(a: *mut Vec3f, b: *mut Vec3f) -> f32_0;
    #[no_mangle]
    fn Math_Vec3f_DistXYZAndStoreDiff(a: *mut Vec3f, b: *mut Vec3f,
                                      dest: *mut Vec3f) -> f32_0;
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
    fn func_80078884(sfxId: u16_0);
    #[no_mangle]
    fn func_800788CC(sfxId: u16_0);
    #[no_mangle]
    fn func_80078914(arg0: *mut Vec3f, sfxId: u16_0);
    #[no_mangle]
    fn Lights_PointNoGlowSetInfo(info: *mut LightInfo, x: s16, y: s16, z: s16,
                                 r: u8_0, g: u8_0, b: u8_0, radius: s16);
    #[no_mangle]
    fn Lights_Draw(lights: *mut Lights, gfxCtx: *mut GraphicsContext);
    #[no_mangle]
    fn Lights_BindAll(lights: *mut Lights, listHead: *mut LightNode,
                      vec: *mut Vec3f);
    #[no_mangle]
    fn LightContext_NewLights(lightCtx: *mut LightContext,
                              gfxCtx: *mut GraphicsContext) -> *mut Lights;
    #[no_mangle]
    fn LightContext_InsertLight(globalCtx: *mut GlobalContext,
                                lightCtx: *mut LightContext,
                                info: *mut LightInfo) -> *mut LightNode;
    #[no_mangle]
    fn LightContext_RemoveLight(globalCtx: *mut GlobalContext,
                                lightCtx: *mut LightContext,
                                node: *mut LightNode);
    #[no_mangle]
    fn Lights_DrawGlow(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn ZeldaArena_MallocDebug(size: u32_0, file: *const libc::c_char,
                              line: s32) -> *mut libc::c_void;
    #[no_mangle]
    fn ZeldaArena_MallocRDebug(size: u32_0, file: *const libc::c_char,
                               line: s32) -> *mut libc::c_void;
    #[no_mangle]
    fn ZeldaArena_FreeDebug(ptr: *mut libc::c_void, file: *const libc::c_char,
                            line: s32);
    #[no_mangle]
    fn Rupees_ChangeBy(rupeeChange: s16);
    #[no_mangle]
    fn func_800876C8(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn Player_InCsMode(globalCtx: *mut GlobalContext) -> s32;
    #[no_mangle]
    fn func_8008EDF0(player: *mut Player);
    #[no_mangle]
    fn Player_GetExplosiveHeld(player: *mut Player) -> s32;
    #[no_mangle]
    fn Quake_SetSpeed(idx: s16, value: s16) -> u32_0;
    #[no_mangle]
    fn Quake_SetCountdown(idx: s16, value: s16) -> u32_0;
    #[no_mangle]
    fn Quake_SetQuakeValues(idx: s16, y: s16, x: s16, zoom: s16, rotZ: s16)
     -> u32_0;
    #[no_mangle]
    fn Quake_Add(cam: *mut Camera, callbackIdx: u32_0) -> s16;
    #[no_mangle]
    fn Gfx_CallSetupDL(gfx: *mut Gfx, i: u32_0) -> *mut Gfx;
    #[no_mangle]
    fn func_80093808(gfx: *mut Gfx) -> *mut Gfx;
    #[no_mangle]
    fn func_80093D18(gfxCtx: *mut GraphicsContext);
    #[no_mangle]
    fn func_80093D84(gfxCtx: *mut GraphicsContext);
    #[no_mangle]
    fn Object_GetIndex(objectCtx: *mut ObjectContext, objectId: s16) -> s32;
    #[no_mangle]
    fn Object_IsLoaded(objectCtx: *mut ObjectContext, bankIndex: s32) -> s32;
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
    fn Animation_Change(skelAnime: *mut SkelAnime,
                        animation: *mut AnimationHeader, playSpeed: f32_0,
                        startFrame: f32_0, endFrame: f32_0, mode: u8_0,
                        morphFrames: f32_0);
    #[no_mangle]
    fn SkelAnime_UpdateTranslation(skelAnime: *mut SkelAnime, pos: *mut Vec3f,
                                   angle: s16);
    #[no_mangle]
    fn SkinMatrix_Vec3fMtxFMultXYZW(mf: *mut MtxF, src: *mut Vec3f,
                                    xyzDest: *mut Vec3f, wDest: *mut f32_0);
    #[no_mangle]
    fn func_800AA000(_: f32_0, _: u8_0, _: u8_0, _: u8_0);
    #[no_mangle]
    fn func_800ABE74(eyeX: f32_0, eyeY: f32_0, eyeZ: f32_0) -> s32;
    #[no_mangle]
    fn func_800BFCB8(globalCtx: *mut GlobalContext, mf: *mut MtxF,
                     vec: *mut Vec3f) -> f32_0;
    #[no_mangle]
    fn Gameplay_GetActiveCamId(globalCtx: *mut GlobalContext) -> s16;
    #[no_mangle]
    fn Gameplay_GetCamera(globalCtx: *mut GlobalContext, camId: s16)
     -> *mut Camera;
    #[no_mangle]
    fn Gameplay_SaveSceneFlags(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn func_800C0CB8(globalCtx: *mut GlobalContext) -> s32;
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
    fn Math_SinF(angle: f32_0) -> f32_0;
    #[no_mangle]
    fn Math_CosF(angle: f32_0) -> f32_0;
    #[no_mangle]
    fn Math_Atan2S(x: f32_0, y: f32_0) -> s16;
    #[no_mangle]
    fn Matrix_Push();
    #[no_mangle]
    fn Matrix_Pop();
    #[no_mangle]
    fn Matrix_Get(dest: *mut MtxF);
    #[no_mangle]
    fn Matrix_Put(src: *mut MtxF);
    #[no_mangle]
    fn Matrix_Mult(mf: *mut MtxF, mode: u8_0);
    #[no_mangle]
    fn Matrix_Translate(x: f32_0, y: f32_0, z: f32_0, mode: u8_0);
    #[no_mangle]
    fn Matrix_Scale(x: f32_0, y: f32_0, z: f32_0, mode: u8_0);
    #[no_mangle]
    fn Matrix_RotateY(y: f32_0, mode: u8_0);
    #[no_mangle]
    fn Matrix_RotateZ(z: f32_0, mode: u8_0);
    #[no_mangle]
    fn func_800D1694(x: f32_0, y: f32_0, z: f32_0, vec: *mut Vec3s);
    #[no_mangle]
    fn Matrix_NewMtx(gfxCtx: *mut GraphicsContext, file: *mut libc::c_char,
                     line: s32) -> *mut Mtx;
    #[no_mangle]
    fn Matrix_MultVec3f(src: *mut Vec3f, dest: *mut Vec3f);
    #[no_mangle]
    fn Matrix_MtxFCopy(dest: *mut MtxF, src: *mut MtxF);
    #[no_mangle]
    fn Matrix_MtxFToYXZRotS(mf: *mut MtxF, rotDest: *mut Vec3s, flag: s32);
    #[no_mangle]
    fn Fault_AddClient(_: *mut FaultClient, _: *mut libc::c_void,
                       _: *mut libc::c_void, _: *mut libc::c_void);
    #[no_mangle]
    fn Fault_RemoveClient(_: *mut FaultClient);
    #[no_mangle]
    fn FaultDrawer_SetCursor(_: s32, _: s32);
    #[no_mangle]
    fn FaultDrawer_Printf(_: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Math_FAtan2F(y: f32_0, x: f32_0) -> f32_0;
    #[no_mangle]
    static mut gGameInfo: *mut GameInfo;
    #[no_mangle]
    static mut gSegments: [u32_0; 16];
    #[no_mangle]
    static mut gSaveContext: SaveContext;
    #[no_mangle]
    fn guLookAtHilite(m: *mut Mtx, l: *mut LookAt, h: *mut Hilite,
                      xEye: f32_0, yEye: f32_0, zEye: f32_0, xAt: f32_0,
                      yAt: f32_0, zAt: f32_0, xUp: f32_0, yUp: f32_0,
                      zUp: f32_0, xl1: f32_0, yl1: f32_0, zl1: f32_0,
                      xl2: f32_0, yl2: f32_0, zl2: f32_0, hiliteWidth: s32,
                      hiliteHeight: s32);
    #[no_mangle]
    fn Message_GetState(msgCtx: *mut MessageContext) -> u8_0;
    #[no_mangle]
    static mut D_801333E8: s8;
    #[no_mangle]
    static mut D_801333E0: f32_0;
    #[no_mangle]
    fn Audio_PlaySoundGeneral(sfxId: u16_0, pos: *mut Vec3f, token: u8_0,
                              freqScale: *mut f32_0, a4: *mut f32_0,
                              reverbAdd: *mut s8);
    #[no_mangle]
    fn Overlay_Load(vRomStart: u32_0, vRomEnd: u32_0,
                    vRamStart: *mut libc::c_void, vRamEnd: *mut libc::c_void,
                    allocatedVRamAddress: *mut libc::c_void) -> s32;
    #[no_mangle]
    static mut gActorOverlayTable: [ActorOverlay; 471];
    #[no_mangle]
    static mut gMtxFClear: MtxF;
    #[no_mangle]
    fn Audio_StopSfxByPos(_: *mut Vec3f);
    #[no_mangle]
    static mut gMaxActorId: s32;
    #[no_mangle]
    fn Rand_ZeroOne() -> f32_0;
    #[no_mangle]
    static mut D_801333D4: Vec3f;
    #[no_mangle]
    fn func_800F4C58(pos: *mut Vec3f, sfxId: u16_0, _: u8_0);
    #[no_mangle]
    fn cosf(_: f32_0) -> f32_0;
    #[no_mangle]
    fn sinf(_: f32_0) -> f32_0;
    #[no_mangle]
    static mut gDbgCamEnabled: s32;
    #[no_mangle]
    fn Message_ContinueTextbox(globalCtx: *mut GlobalContext, textId: u16_0);
    #[no_mangle]
    fn Message_ShouldAdvance(globalCtx: *mut GlobalContext) -> u8_0;
    #[no_mangle]
    static mut gZTargetArrowDL: [Gfx; 0];
    #[no_mangle]
    static mut gEffFlash1DL: [Gfx; 0];
    #[no_mangle]
    static mut gLensOfTruthMaskTex: [u64_0; 0];
    #[no_mangle]
    static mut gFootShadowDL: [Gfx; 0];
    #[no_mangle]
    static mut gCircleShadowDL: [Gfx; 0];
    #[no_mangle]
    static mut gHorseShadowDL: [Gfx; 0];
    #[no_mangle]
    static mut gZTargetLockOnTriangleDL: [Gfx; 0];
    #[no_mangle]
    static mut gDoorLockDL: [Gfx; 0];
    #[no_mangle]
    static mut gDoorChainsDL: [Gfx; 0];
    #[no_mangle]
    static mut object_bdoor_DL_001400: [Gfx; 0];
    #[no_mangle]
    static mut object_bdoor_DL_001530: [Gfx; 0];
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
pub struct LookAt {
    pub l: [Light; 2],
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
pub type C2RustUnnamed_2 = libc::c_uint;
pub const RESPAWN_MODE_TOP: C2RustUnnamed_2 = 2;
pub const RESPAWN_MODE_RETURN: C2RustUnnamed_2 = 1;
pub const RESPAWN_MODE_DOWN: C2RustUnnamed_2 = 0;
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
pub struct ColliderJntSphElementDim {
    pub modelSphere: Sphere16,
    pub worldSphere: Sphere16,
    pub scale: f32_0,
    pub limb: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderJntSphElement {
    pub info: ColliderInfo,
    pub dim: ColliderJntSphElementDim,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColliderJntSph {
    pub base: Collider,
    pub count: s32,
    pub elements: *mut ColliderJntSphElement,
}
pub type callback1_800343CC
    =
    Option<unsafe extern "C" fn(_: *mut GlobalContext, _: *mut Actor)
               -> u16_0>;
pub type callback2_800343CC
    =
    Option<unsafe extern "C" fn(_: *mut GlobalContext, _: *mut Actor) -> s16>;
pub type C2RustUnnamed_15 = libc::c_uint;
pub const ALLOCTYPE_PERMANENT: C2RustUnnamed_15 = 2;
pub const ALLOCTYPE_ABSOLUTE: C2RustUnnamed_15 = 1;
pub const ALLOCTYPE_NORMAL: C2RustUnnamed_15 = 0;
pub type C2RustUnnamed_16 = libc::c_uint;
pub const FOOT_RIGHT: C2RustUnnamed_16 = 1;
pub const FOOT_LEFT: C2RustUnnamed_16 = 0;
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
pub const CS_STATE_UNSKIPPABLE_EXEC: C2RustUnnamed_19 = 4;
pub const CS_STATE_UNSKIPPABLE_INIT: C2RustUnnamed_19 = 3;
pub const CS_STATE_SKIPPABLE_EXEC: C2RustUnnamed_19 = 2;
pub const CS_STATE_SKIPPABLE_INIT: C2RustUnnamed_19 = 1;
pub const CS_STATE_IDLE: C2RustUnnamed_19 = 0;
pub type C2RustUnnamed_20 = libc::c_uint;
pub const CAM_SET_MAX: C2RustUnnamed_20 = 66;
pub const CAM_SET_NORMAL4: C2RustUnnamed_20 = 65;
pub const CAM_SET_PIVOT_FROM_SIDE: C2RustUnnamed_20 = 64;
pub const CAM_SET_DIRECTED_YAW: C2RustUnnamed_20 = 63;
pub const CAM_SET_DUNGEON2: C2RustUnnamed_20 = 62;
pub const CAM_SET_JABU_TENTACLE: C2RustUnnamed_20 = 61;
pub const CAM_SET_CS_C: C2RustUnnamed_20 = 60;
pub const CAM_SET_FISHING: C2RustUnnamed_20 = 59;
pub const CAM_SET_NORMAL2: C2RustUnnamed_20 = 58;
pub const CAM_SET_PIVOT_VERTICAL: C2RustUnnamed_20 = 57;
pub const CAM_SET_TURN_AROUND: C2RustUnnamed_20 = 56;
pub const CAM_SET_FIRE_BIRDS_EYE: C2RustUnnamed_20 = 55;
pub const CAM_SET_MEADOW_UNUSED: C2RustUnnamed_20 = 54;
pub const CAM_SET_MEADOW_BIRDS_EYE: C2RustUnnamed_20 = 53;
pub const CAM_SET_BIG_OCTO: C2RustUnnamed_20 = 52;
pub const CAM_SET_FOREST_DEFEAT_POE: C2RustUnnamed_20 = 51;
pub const CAM_SET_FOREST_UNUSED: C2RustUnnamed_20 = 50;
pub const CAM_SET_FIRE_STAIRCASE: C2RustUnnamed_20 = 49;
pub const CAM_SET_FIRE_PLATFORM: C2RustUnnamed_20 = 48;
pub const CAM_SET_SCENE_TRANSITION: C2RustUnnamed_20 = 47;
pub const CAM_SET_SCENE_UNUSED: C2RustUnnamed_20 = 46;
pub const CAM_SET_BEAN_LOST_WOODS: C2RustUnnamed_20 = 45;
pub const CAM_SET_BEAN_GENERIC: C2RustUnnamed_20 = 44;
pub const CAM_SET_CS_ATTENTION: C2RustUnnamed_20 = 43;
pub const CAM_SET_CS_3: C2RustUnnamed_20 = 42;
pub const CAM_SET_ITEM_UNUSED: C2RustUnnamed_20 = 41;
pub const CAM_SET_SLOW_CHEST_CS: C2RustUnnamed_20 = 40;
pub const CAM_SET_FOREST_BIRDS_EYE: C2RustUnnamed_20 = 39;
pub const CAM_SET_CS_TWISTED_HALLWAY: C2RustUnnamed_20 = 38;
pub const CAM_SET_CS_0: C2RustUnnamed_20 = 37;
pub const CAM_SET_PIVOT_WATER_SURFACE: C2RustUnnamed_20 = 36;
pub const CAM_SET_PIVOT_CORNER: C2RustUnnamed_20 = 35;
pub const CAM_SET_FREE2: C2RustUnnamed_20 = 34;
pub const CAM_SET_FREE0: C2RustUnnamed_20 = 33;
pub const CAM_SET_START1: C2RustUnnamed_20 = 32;
pub const CAM_SET_START0: C2RustUnnamed_20 = 31;
pub const CAM_SET_CRAWLSPACE: C2RustUnnamed_20 = 30;
pub const CAM_SET_DOORC: C2RustUnnamed_20 = 29;
pub const CAM_SET_DOOR0: C2RustUnnamed_20 = 28;
pub const CAM_SET_PREREND_SIDE_SCROLL: C2RustUnnamed_20 = 27;
pub const CAM_SET_PREREND_PIVET: C2RustUnnamed_20 = 26;
pub const CAM_SET_PREREND_FIXED: C2RustUnnamed_20 = 25;
pub const CAM_SET_PIVOT_IN_FRONT: C2RustUnnamed_20 = 24;
pub const CAM_SET_PIVOT_SHOP_BROWSING: C2RustUnnamed_20 = 23;
pub const CAM_SET_PIVOT_CRAWLSPACE: C2RustUnnamed_20 = 22;
pub const CAM_SET_CHU_BOWLING: C2RustUnnamed_20 = 21;
pub const CAM_SET_MARKET_BALCONY: C2RustUnnamed_20 = 20;
pub const CAM_SET_TOWER_UNUSED: C2RustUnnamed_20 = 19;
pub const CAM_SET_TOWER_CLIMB: C2RustUnnamed_20 = 18;
pub const CAM_SET_BOSS_GANON: C2RustUnnamed_20 = 17;
pub const CAM_SET_BOSS_GANONDORF: C2RustUnnamed_20 = 16;
pub const CAM_SET_BOSS_TWINROVA_FLOOR: C2RustUnnamed_20 = 15;
pub const CAM_SET_BOSS_TWINROVA_PLATFORM: C2RustUnnamed_20 = 14;
pub const CAM_SET_BOSS_MORPHA: C2RustUnnamed_20 = 13;
pub const CAM_SET_BOSS_BONGO: C2RustUnnamed_20 = 12;
pub const CAM_SET_BOSS_VOLVAGIA: C2RustUnnamed_20 = 11;
pub const CAM_SET_BOSS_PHANTOM_GANON: C2RustUnnamed_20 = 10;
pub const CAM_SET_BOSS_BARINADE: C2RustUnnamed_20 = 9;
pub const CAM_SET_BOSS_DODONGO: C2RustUnnamed_20 = 8;
pub const CAM_SET_BOSS_GOHMA: C2RustUnnamed_20 = 7;
pub const CAM_SET_HORSE: C2RustUnnamed_20 = 6;
pub const CAM_SET_NORMAL3: C2RustUnnamed_20 = 5;
pub const CAM_SET_DUNGEON1: C2RustUnnamed_20 = 4;
pub const CAM_SET_DUNGEON0: C2RustUnnamed_20 = 3;
pub const CAM_SET_NORMAL1: C2RustUnnamed_20 = 2;
pub const CAM_SET_NORMAL0: C2RustUnnamed_20 = 1;
pub const CAM_SET_NONE: C2RustUnnamed_20 = 0;
pub type C2RustUnnamed_21 = libc::c_uint;
pub const SCENE_ID_MAX: C2RustUnnamed_21 = 110;
pub const SCENE_TESTROOM: C2RustUnnamed_21 = 109;
pub const SCENE_SASATEST: C2RustUnnamed_21 = 108;
pub const SCENE_HAIRAL_NIWA2: C2RustUnnamed_21 = 107;
pub const SCENE_SUTARU: C2RustUnnamed_21 = 106;
pub const SCENE_SYOTES2: C2RustUnnamed_21 = 105;
pub const SCENE_SYOTES: C2RustUnnamed_21 = 104;
pub const SCENE_DEPTH_TEST: C2RustUnnamed_21 = 103;
pub const SCENE_BESITU: C2RustUnnamed_21 = 102;
pub const SCENE_TEST01: C2RustUnnamed_21 = 101;
pub const SCENE_GANON_TOU: C2RustUnnamed_21 = 100;
pub const SCENE_SPOT20: C2RustUnnamed_21 = 99;
pub const SCENE_SPOT18: C2RustUnnamed_21 = 98;
pub const SCENE_SPOT17: C2RustUnnamed_21 = 97;
pub const SCENE_SPOT16: C2RustUnnamed_21 = 96;
pub const SCENE_SPOT15: C2RustUnnamed_21 = 95;
pub const SCENE_SPOT13: C2RustUnnamed_21 = 94;
pub const SCENE_SPOT12: C2RustUnnamed_21 = 93;
pub const SCENE_SPOT11: C2RustUnnamed_21 = 92;
pub const SCENE_SPOT10: C2RustUnnamed_21 = 91;
pub const SCENE_SPOT09: C2RustUnnamed_21 = 90;
pub const SCENE_SPOT08: C2RustUnnamed_21 = 89;
pub const SCENE_SPOT07: C2RustUnnamed_21 = 88;
pub const SCENE_SPOT06: C2RustUnnamed_21 = 87;
pub const SCENE_SPOT05: C2RustUnnamed_21 = 86;
pub const SCENE_SPOT04: C2RustUnnamed_21 = 85;
pub const SCENE_SPOT03: C2RustUnnamed_21 = 84;
pub const SCENE_SPOT02: C2RustUnnamed_21 = 83;
pub const SCENE_SPOT01: C2RustUnnamed_21 = 82;
pub const SCENE_SPOT00: C2RustUnnamed_21 = 81;
pub const SCENE_KINSUTA: C2RustUnnamed_21 = 80;
pub const SCENE_GANON_DEMO: C2RustUnnamed_21 = 79;
pub const SCENE_MAHOUYA: C2RustUnnamed_21 = 78;
pub const SCENE_MIHARIGOYA: C2RustUnnamed_21 = 77;
pub const SCENE_SOUKO: C2RustUnnamed_21 = 76;
pub const SCENE_BOWLING: C2RustUnnamed_21 = 75;
pub const SCENE_NAKANIWA: C2RustUnnamed_21 = 74;
pub const SCENE_TURIBORI: C2RustUnnamed_21 = 73;
pub const SCENE_HAKASITARELAY: C2RustUnnamed_21 = 72;
pub const SCENE_HIRAL_DEMO: C2RustUnnamed_21 = 71;
pub const SCENE_HAIRAL_NIWA_N: C2RustUnnamed_21 = 70;
pub const SCENE_HAIRAL_NIWA: C2RustUnnamed_21 = 69;
pub const SCENE_KENJYANOMA: C2RustUnnamed_21 = 68;
pub const SCENE_TOKINOMA: C2RustUnnamed_21 = 67;
pub const SCENE_SYATEKIJYOU: C2RustUnnamed_21 = 66;
pub const SCENE_HAKAANA_OUKE: C2RustUnnamed_21 = 65;
pub const SCENE_HAKAANA2: C2RustUnnamed_21 = 64;
pub const SCENE_HAKAANA: C2RustUnnamed_21 = 63;
pub const SCENE_KAKUSIANA: C2RustUnnamed_21 = 62;
pub const SCENE_YOUSEI_IZUMI_YOKO: C2RustUnnamed_21 = 61;
pub const SCENE_YOUSEI_IZUMI_TATE: C2RustUnnamed_21 = 60;
pub const SCENE_DAIYOUSEI_IZUMI: C2RustUnnamed_21 = 59;
pub const SCENE_HUT: C2RustUnnamed_21 = 58;
pub const SCENE_TENT: C2RustUnnamed_21 = 57;
pub const SCENE_HYLIA_LABO: C2RustUnnamed_21 = 56;
pub const SCENE_LABO: C2RustUnnamed_21 = 55;
pub const SCENE_MALON_STABLE: C2RustUnnamed_21 = 54;
pub const SCENE_IMPA: C2RustUnnamed_21 = 53;
pub const SCENE_LINK_HOME: C2RustUnnamed_21 = 52;
pub const SCENE_FACE_SHOP: C2RustUnnamed_21 = 51;
pub const SCENE_NIGHT_SHOP: C2RustUnnamed_21 = 50;
pub const SCENE_ALLEY_SHOP: C2RustUnnamed_21 = 49;
pub const SCENE_DRAG: C2RustUnnamed_21 = 48;
pub const SCENE_ZOORA: C2RustUnnamed_21 = 47;
pub const SCENE_GOLON: C2RustUnnamed_21 = 46;
pub const SCENE_KOKIRI_SHOP: C2RustUnnamed_21 = 45;
pub const SCENE_SHOP1: C2RustUnnamed_21 = 44;
pub const SCENE_KAKARIKO3: C2RustUnnamed_21 = 43;
pub const SCENE_KAKARIKO: C2RustUnnamed_21 = 42;
pub const SCENE_KOKIRI_HOME5: C2RustUnnamed_21 = 41;
pub const SCENE_KOKIRI_HOME4: C2RustUnnamed_21 = 40;
pub const SCENE_KOKIRI_HOME3: C2RustUnnamed_21 = 39;
pub const SCENE_KOKIRI_HOME: C2RustUnnamed_21 = 38;
pub const SCENE_SHRINE_R: C2RustUnnamed_21 = 37;
pub const SCENE_SHRINE_N: C2RustUnnamed_21 = 36;
pub const SCENE_SHRINE: C2RustUnnamed_21 = 35;
pub const SCENE_MARKET_RUINS: C2RustUnnamed_21 = 34;
pub const SCENE_MARKET_NIGHT: C2RustUnnamed_21 = 33;
pub const SCENE_MARKET_DAY: C2RustUnnamed_21 = 32;
pub const SCENE_MARKET_ALLEY_N: C2RustUnnamed_21 = 31;
pub const SCENE_MARKET_ALLEY: C2RustUnnamed_21 = 30;
pub const SCENE_ENRUI: C2RustUnnamed_21 = 29;
pub const SCENE_ENTRA_N: C2RustUnnamed_21 = 28;
pub const SCENE_ENTRA: C2RustUnnamed_21 = 27;
pub const SCENE_GANON_FINAL: C2RustUnnamed_21 = 26;
pub const SCENE_GANON_BOSS: C2RustUnnamed_21 = 25;
pub const SCENE_HAKADAN_BS: C2RustUnnamed_21 = 24;
pub const SCENE_JYASINBOSS: C2RustUnnamed_21 = 23;
pub const SCENE_MIZUSIN_BS: C2RustUnnamed_21 = 22;
pub const SCENE_FIRE_BS: C2RustUnnamed_21 = 21;
pub const SCENE_MORIBOSSROOM: C2RustUnnamed_21 = 20;
pub const SCENE_BDAN_BOSS: C2RustUnnamed_21 = 19;
pub const SCENE_DDAN_BOSS: C2RustUnnamed_21 = 18;
pub const SCENE_YDAN_BOSS: C2RustUnnamed_21 = 17;
pub const SCENE_TAKARAYA: C2RustUnnamed_21 = 16;
pub const SCENE_GANONTIKA_SONOGO: C2RustUnnamed_21 = 15;
pub const SCENE_GANON_SONOGO: C2RustUnnamed_21 = 14;
pub const SCENE_GANONTIKA: C2RustUnnamed_21 = 13;
pub const SCENE_GERUDOWAY: C2RustUnnamed_21 = 12;
pub const SCENE_MEN: C2RustUnnamed_21 = 11;
pub const SCENE_GANON: C2RustUnnamed_21 = 10;
pub const SCENE_ICE_DOUKUTO: C2RustUnnamed_21 = 9;
pub const SCENE_HAKADANCH: C2RustUnnamed_21 = 8;
pub const SCENE_HAKADAN: C2RustUnnamed_21 = 7;
pub const SCENE_JYASINZOU: C2RustUnnamed_21 = 6;
pub const SCENE_MIZUSIN: C2RustUnnamed_21 = 5;
pub const SCENE_HIDAN: C2RustUnnamed_21 = 4;
pub const SCENE_BMORI1: C2RustUnnamed_21 = 3;
pub const SCENE_BDAN: C2RustUnnamed_21 = 2;
pub const SCENE_DDAN: C2RustUnnamed_21 = 1;
pub const SCENE_YDAN: C2RustUnnamed_21 = 0;
pub type C2RustUnnamed_22 = libc::c_uint;
pub const GI_MAX: C2RustUnnamed_22 = 126;
pub const GI_TEXT_0: C2RustUnnamed_22 = 125;
pub const GI_ICE_TRAP: C2RustUnnamed_22 = 124;
pub const GI_BULLET_BAG_50: C2RustUnnamed_22 = 123;
pub const GI_NUT_UPGRADE_40: C2RustUnnamed_22 = 122;
pub const GI_NUT_UPGRADE_30: C2RustUnnamed_22 = 121;
pub const GI_STICK_UPGRADE_30: C2RustUnnamed_22 = 120;
pub const GI_STICK_UPGRADE_20: C2RustUnnamed_22 = 119;
pub const GI_HEART_PIECE_WIN: C2RustUnnamed_22 = 118;
pub const GI_RUPEE_PURPLE_LOSE: C2RustUnnamed_22 = 117;
pub const GI_RUPEE_RED_LOSE: C2RustUnnamed_22 = 116;
pub const GI_RUPEE_BLUE_LOSE: C2RustUnnamed_22 = 115;
pub const GI_RUPEE_GREEN_LOSE: C2RustUnnamed_22 = 114;
pub const GI_DOOR_KEY: C2RustUnnamed_22 = 113;
pub const GI_BIG_POE: C2RustUnnamed_22 = 112;
pub const GI_POE: C2RustUnnamed_22 = 111;
pub const GI_BLUE_FIRE: C2RustUnnamed_22 = 110;
pub const GI_BUGS: C2RustUnnamed_22 = 109;
pub const GI_FISH: C2RustUnnamed_22 = 108;
pub const GI_BOMBCHUS_20: C2RustUnnamed_22 = 107;
pub const GI_BOMBCHUS_5: C2RustUnnamed_22 = 106;
pub const GI_SEEDS_30: C2RustUnnamed_22 = 105;
pub const GI_BOMBS_30: C2RustUnnamed_22 = 104;
pub const GI_BOMBS_20: C2RustUnnamed_22 = 103;
pub const GI_BOMBS_10: C2RustUnnamed_22 = 102;
pub const GI_BOMBS_1: C2RustUnnamed_22 = 101;
pub const GI_NUTS_10: C2RustUnnamed_22 = 100;
pub const GI_NUTS_5_2: C2RustUnnamed_22 = 99;
pub const GI_STICKS_10: C2RustUnnamed_22 = 98;
pub const GI_STICKS_5: C2RustUnnamed_22 = 97;
pub const GI_BULLET_BAG_40: C2RustUnnamed_22 = 96;
pub const GI_BULLET_BAG_30: C2RustUnnamed_22 = 95;
pub const GI_NAYRUS_LOVE: C2RustUnnamed_22 = 94;
pub const GI_FARORES_WIND: C2RustUnnamed_22 = 93;
pub const GI_DINS_FIRE: C2RustUnnamed_22 = 92;
pub const GI_SKULL_TOKEN: C2RustUnnamed_22 = 91;
pub const GI_ARROW_LIGHT: C2RustUnnamed_22 = 90;
pub const GI_ARROW_ICE: C2RustUnnamed_22 = 89;
pub const GI_ARROW_FIRE: C2RustUnnamed_22 = 88;
pub const GI_SWORD_BGS: C2RustUnnamed_22 = 87;
pub const GI_RUPEE_GOLD: C2RustUnnamed_22 = 86;
pub const GI_RUPEE_PURPLE: C2RustUnnamed_22 = 85;
pub const GI_BRACELET: C2RustUnnamed_22 = 84;
pub const GI_MASK_GERUDO: C2RustUnnamed_22 = 83;
pub const GI_MASK_ZORA: C2RustUnnamed_22 = 82;
pub const GI_MASK_GORON: C2RustUnnamed_22 = 81;
pub const GI_MILK: C2RustUnnamed_22 = 80;
pub const GI_HEART_CONTAINER_2: C2RustUnnamed_22 = 79;
pub const GI_RUPEE_RED: C2RustUnnamed_22 = 78;
pub const GI_RUPEE_BLUE: C2RustUnnamed_22 = 77;
pub const GI_RUPEE_GREEN: C2RustUnnamed_22 = 76;
pub const GI_ARROWS_LARGE: C2RustUnnamed_22 = 75;
pub const GI_ARROWS_MEDIUM: C2RustUnnamed_22 = 74;
pub const GI_ARROWS_SMALL: C2RustUnnamed_22 = 73;
pub const GI_HEART: C2RustUnnamed_22 = 72;
pub const GI_WEIRD_EGG: C2RustUnnamed_22 = 71;
pub const GI_WALLET_GIANT: C2RustUnnamed_22 = 70;
pub const GI_WALLET_ADULT: C2RustUnnamed_22 = 69;
pub const GI_MAGIC_LARGE: C2RustUnnamed_22 = 68;
pub const GI_MAGIC_SMALL: C2RustUnnamed_22 = 67;
pub const GI_KEY_SMALL: C2RustUnnamed_22 = 66;
pub const GI_MAP: C2RustUnnamed_22 = 65;
pub const GI_COMPASS: C2RustUnnamed_22 = 64;
pub const GI_KEY_BOSS: C2RustUnnamed_22 = 63;
pub const GI_HEART_PIECE: C2RustUnnamed_22 = 62;
pub const GI_HEART_CONTAINER: C2RustUnnamed_22 = 61;
pub const GI_SEEDS_5: C2RustUnnamed_22 = 60;
pub const GI_OCARINA_FAIRY: C2RustUnnamed_22 = 59;
pub const GI_GERUDO_CARD: C2RustUnnamed_22 = 58;
pub const GI_STONE_OF_AGONY: C2RustUnnamed_22 = 57;
pub const GI_SCALE_GOLD: C2RustUnnamed_22 = 56;
pub const GI_SCALE_SILVER: C2RustUnnamed_22 = 55;
pub const GI_GAUNTLETS_GOLD: C2RustUnnamed_22 = 54;
pub const GI_GAUNTLETS_SILVER: C2RustUnnamed_22 = 53;
pub const GI_BOMB_BAG_40: C2RustUnnamed_22 = 52;
pub const GI_BOMB_BAG_30: C2RustUnnamed_22 = 51;
pub const GI_BOMB_BAG_20: C2RustUnnamed_22 = 50;
pub const GI_QUIVER_50: C2RustUnnamed_22 = 49;
pub const GI_QUIVER_40: C2RustUnnamed_22 = 48;
pub const GI_BOOTS_HOVER: C2RustUnnamed_22 = 47;
pub const GI_BOOTS_IRON: C2RustUnnamed_22 = 46;
pub const GI_TUNIC_ZORA: C2RustUnnamed_22 = 45;
pub const GI_TUNIC_GORON: C2RustUnnamed_22 = 44;
pub const GI_SHIELD_MIRROR: C2RustUnnamed_22 = 43;
pub const GI_SHIELD_HYLIAN: C2RustUnnamed_22 = 42;
pub const GI_SHIELD_DEKU: C2RustUnnamed_22 = 41;
pub const GI_SWORD_KNIFE: C2RustUnnamed_22 = 40;
pub const GI_SWORD_KOKIRI: C2RustUnnamed_22 = 39;
pub const GI_CLAIM_CHECK: C2RustUnnamed_22 = 38;
pub const GI_EYEDROPS: C2RustUnnamed_22 = 37;
pub const GI_FROG: C2RustUnnamed_22 = 36;
pub const GI_PRESCRIPTION: C2RustUnnamed_22 = 35;
pub const GI_SWORD_BROKEN: C2RustUnnamed_22 = 34;
pub const GI_SAW: C2RustUnnamed_22 = 33;
pub const GI_ODD_POTION: C2RustUnnamed_22 = 32;
pub const GI_ODD_MUSHROOM: C2RustUnnamed_22 = 31;
pub const GI_POCKET_CUCCO: C2RustUnnamed_22 = 30;
pub const GI_POCKET_EGG: C2RustUnnamed_22 = 29;
pub const GI_MASK_TRUTH: C2RustUnnamed_22 = 28;
pub const GI_MASK_BUNNY: C2RustUnnamed_22 = 27;
pub const GI_MASK_KEATON: C2RustUnnamed_22 = 26;
pub const GI_CHICKEN: C2RustUnnamed_22 = 25;
pub const GI_MASK_SPOOKY: C2RustUnnamed_22 = 24;
pub const GI_MASK_SKULL: C2RustUnnamed_22 = 23;
pub const GI_BEAN: C2RustUnnamed_22 = 22;
pub const GI_LETTER_RUTO: C2RustUnnamed_22 = 21;
pub const GI_MILK_BOTTLE: C2RustUnnamed_22 = 20;
pub const GI_FAIRY: C2RustUnnamed_22 = 19;
pub const GI_POTION_BLUE: C2RustUnnamed_22 = 18;
pub const GI_POTION_GREEN: C2RustUnnamed_22 = 17;
pub const GI_POTION_RED: C2RustUnnamed_22 = 16;
pub const GI_BOTTLE: C2RustUnnamed_22 = 15;
pub const GI_COJIRO: C2RustUnnamed_22 = 14;
pub const GI_HAMMER: C2RustUnnamed_22 = 13;
pub const GI_OCARINA_OOT: C2RustUnnamed_22 = 12;
pub const GI_LETTER_ZELDA: C2RustUnnamed_22 = 11;
pub const GI_LENS: C2RustUnnamed_22 = 10;
pub const GI_LONGSHOT: C2RustUnnamed_22 = 9;
pub const GI_HOOKSHOT: C2RustUnnamed_22 = 8;
pub const GI_STICKS_1: C2RustUnnamed_22 = 7;
pub const GI_BOOMERANG: C2RustUnnamed_22 = 6;
pub const GI_SLINGSHOT: C2RustUnnamed_22 = 5;
pub const GI_BOW: C2RustUnnamed_22 = 4;
pub const GI_BOMBCHUS_10: C2RustUnnamed_22 = 3;
pub const GI_NUTS_5: C2RustUnnamed_22 = 2;
pub const GI_BOMBS_5: C2RustUnnamed_22 = 1;
pub const GI_NONE: C2RustUnnamed_22 = 0;
pub type C2RustUnnamed_23 = libc::c_uint;
pub const EXCH_ITEM_MAX: C2RustUnnamed_23 = 30;
pub const EXCH_ITEM_LETTER_RUTO: C2RustUnnamed_23 = 29;
pub const EXCH_ITEM_BIG_POE: C2RustUnnamed_23 = 28;
pub const EXCH_ITEM_POE: C2RustUnnamed_23 = 27;
pub const EXCH_ITEM_BUG: C2RustUnnamed_23 = 26;
pub const EXCH_ITEM_BLUE_FIRE: C2RustUnnamed_23 = 25;
pub const EXCH_ITEM_FISH: C2RustUnnamed_23 = 24;
pub const EXCH_ITEM_MASK_GERUDO: C2RustUnnamed_23 = 23;
pub const EXCH_ITEM_MASK_ZORA: C2RustUnnamed_23 = 22;
pub const EXCH_ITEM_MASK_GORON: C2RustUnnamed_23 = 21;
pub const EXCH_ITEM_MASK_TRUTH: C2RustUnnamed_23 = 20;
pub const EXCH_ITEM_MASK_BUNNY: C2RustUnnamed_23 = 19;
pub const EXCH_ITEM_MASK_KEATON: C2RustUnnamed_23 = 18;
pub const EXCH_ITEM_MASK_SPOOKY: C2RustUnnamed_23 = 17;
pub const EXCH_ITEM_MASK_SKULL: C2RustUnnamed_23 = 16;
pub const EXCH_ITEM_CLAIM_CHECK: C2RustUnnamed_23 = 15;
pub const EXCH_ITEM_EYEDROPS: C2RustUnnamed_23 = 14;
pub const EXCH_ITEM_FROG: C2RustUnnamed_23 = 13;
pub const EXCH_ITEM_PRESCRIPTION: C2RustUnnamed_23 = 12;
pub const EXCH_ITEM_SWORD_BROKEN: C2RustUnnamed_23 = 11;
pub const EXCH_ITEM_SAW: C2RustUnnamed_23 = 10;
pub const EXCH_ITEM_ODD_POTION: C2RustUnnamed_23 = 9;
pub const EXCH_ITEM_ODD_MUSHROOM: C2RustUnnamed_23 = 8;
pub const EXCH_ITEM_COJIRO: C2RustUnnamed_23 = 7;
pub const EXCH_ITEM_POCKET_CUCCO: C2RustUnnamed_23 = 6;
pub const EXCH_ITEM_POCKET_EGG: C2RustUnnamed_23 = 5;
pub const EXCH_ITEM_BEAN: C2RustUnnamed_23 = 4;
pub const EXCH_ITEM_CHICKEN: C2RustUnnamed_23 = 3;
pub const EXCH_ITEM_WEIRD_EGG: C2RustUnnamed_23 = 2;
pub const EXCH_ITEM_LETTER_ZELDA: C2RustUnnamed_23 = 1;
pub const EXCH_ITEM_NONE: C2RustUnnamed_23 = 0;
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
pub const MSGMODE_PAUSED: C2RustUnnamed_24 = 55;
pub const MSGMODE_TEXT_CLOSING: C2RustUnnamed_24 = 54;
pub const MSGMODE_TEXT_DONE: C2RustUnnamed_24 = 53;
pub const MSGMODE_TEXT_AWAIT_NEXT: C2RustUnnamed_24 = 52;
pub const MSGMODE_FROGS_WAITING: C2RustUnnamed_24 = 51;
pub const MSGMODE_FROGS_PLAYING: C2RustUnnamed_24 = 50;
pub const MSGMODE_FROGS_START: C2RustUnnamed_24 = 49;
pub const MSGMODE_MEMORY_GAME_START_NEXT_ROUND: C2RustUnnamed_24 = 48;
pub const MSGMODE_MEMORY_GAME_ROUND_SUCCESS: C2RustUnnamed_24 = 47;
pub const MSGMODE_MEMORY_GAME_PLAYER_PLAYING: C2RustUnnamed_24 = 46;
pub const MSGMODE_MEMORY_GAME_RIGHT_SKULLKID_WAIT: C2RustUnnamed_24 = 45;
pub const MSGMODE_MEMORY_GAME_RIGHT_SKULLKID_PLAYING: C2RustUnnamed_24 = 44;
pub const MSGMODE_MEMORY_GAME_LEFT_SKULLKID_WAIT: C2RustUnnamed_24 = 43;
pub const MSGMODE_MEMORY_GAME_LEFT_SKULLKID_PLAYING: C2RustUnnamed_24 = 42;
pub const MSGMODE_MEMORY_GAME_START: C2RustUnnamed_24 = 41;
pub const MSGMODE_SCARECROW_PLAYBACK: C2RustUnnamed_24 = 40;
pub const MSGMODE_SCARECROW_RECORDING_DONE: C2RustUnnamed_24 = 39;
pub const MSGMODE_SCARECROW_RECORDING_FAILED: C2RustUnnamed_24 = 38;
pub const MSGMODE_SCARECROW_RECORDING_ONGOING: C2RustUnnamed_24 = 37;
pub const MSGMODE_SCARECROW_RECORDING_START: C2RustUnnamed_24 = 36;
pub const MSGMODE_SCARECROW_LONG_PLAYBACK: C2RustUnnamed_24 = 35;
pub const MSGMODE_SCARECROW_LONG_RECORDING_ONGOING: C2RustUnnamed_24 = 34;
pub const MSGMODE_SCARECROW_LONG_RECORDING_START: C2RustUnnamed_24 = 33;
pub const MSGMODE_UNK_20: C2RustUnnamed_24 = 32;
pub const MSGMODE_OCARINA_AWAIT_INPUT: C2RustUnnamed_24 = 31;
pub const MSGMODE_SONG_PLAYBACK_NOTES_DROP: C2RustUnnamed_24 = 30;
pub const MSGMODE_SONG_PLAYBACK_FAIL: C2RustUnnamed_24 = 29;
pub const MSGMODE_SONG_PLAYBACK_SUCCESS: C2RustUnnamed_24 = 28;
pub const MSGMODE_SONG_PLAYBACK: C2RustUnnamed_24 = 27;
pub const MSGMODE_SONG_DEMONSTRATION_DONE: C2RustUnnamed_24 = 26;
pub const MSGMODE_SONG_DEMONSTRATION: C2RustUnnamed_24 = 25;
pub const MSGMODE_SONG_DEMONSTRATION_SELECT_INSTRUMENT: C2RustUnnamed_24 = 24;
pub const MSGMODE_SONG_PLAYED_ACT: C2RustUnnamed_24 = 23;
pub const MSGMODE_SONG_PLAYED_ACT_BEGIN: C2RustUnnamed_24 = 22;
pub const MSGMODE_DISPLAY_SONG_PLAYED_TEXT: C2RustUnnamed_24 = 21;
pub const MSGMODE_DISPLAY_SONG_PLAYED_TEXT_BEGIN: C2RustUnnamed_24 = 20;
pub const MSGMODE_DISPLAY_SONG_PLAYED: C2RustUnnamed_24 = 19;
pub const MSGMODE_SETUP_DISPLAY_SONG_PLAYED: C2RustUnnamed_24 = 18;
pub const MSGMODE_SONG_PLAYED: C2RustUnnamed_24 = 17;
pub const MSGMODE_OCARINA_NOTES_DROP: C2RustUnnamed_24 = 16;
pub const MSGMODE_OCARINA_FAIL_NO_TEXT: C2RustUnnamed_24 = 15;
pub const MSGMODE_OCARINA_FAIL: C2RustUnnamed_24 = 14;
pub const MSGMODE_OCARINA_CORRECT_PLAYBACK: C2RustUnnamed_24 = 13;
pub const MSGMODE_OCARINA_PLAYING: C2RustUnnamed_24 = 12;
pub const MSGMODE_SONG_PLAYBACK_STARTING: C2RustUnnamed_24 = 11;
pub const MSGMODE_SONG_DEMONSTRATION_STARTING: C2RustUnnamed_24 = 10;
pub const MSGMODE_OCARINA_STARTING: C2RustUnnamed_24 = 9;
pub const MSGMODE_TEXT_DELAYED_BREAK: C2RustUnnamed_24 = 8;
pub const MSGMODE_TEXT_AWAIT_INPUT: C2RustUnnamed_24 = 7;
pub const MSGMODE_TEXT_DISPLAYING: C2RustUnnamed_24 = 6;
pub const MSGMODE_TEXT_CONTINUING: C2RustUnnamed_24 = 5;
pub const MSGMODE_TEXT_NEXT_MSG: C2RustUnnamed_24 = 4;
pub const MSGMODE_TEXT_STARTING: C2RustUnnamed_24 = 3;
pub const MSGMODE_TEXT_BOX_GROWING: C2RustUnnamed_24 = 2;
pub const MSGMODE_TEXT_START: C2RustUnnamed_24 = 1;
pub const MSGMODE_NONE: C2RustUnnamed_24 = 0;
pub type C2RustUnnamed_25 = libc::c_uint;
pub const TEXT_STATE_AWAITING_NEXT: C2RustUnnamed_25 = 10;
pub const TEXT_STATE_9: C2RustUnnamed_25 = 9;
pub const TEXT_STATE_8: C2RustUnnamed_25 = 8;
pub const TEXT_STATE_SONG_DEMO_DONE: C2RustUnnamed_25 = 7;
pub const TEXT_STATE_DONE: C2RustUnnamed_25 = 6;
pub const TEXT_STATE_EVENT: C2RustUnnamed_25 = 5;
pub const TEXT_STATE_CHOICE: C2RustUnnamed_25 = 4;
pub const TEXT_STATE_DONE_FADING: C2RustUnnamed_25 = 3;
pub const TEXT_STATE_CLOSING: C2RustUnnamed_25 = 2;
pub const TEXT_STATE_DONE_HAS_NEXT: C2RustUnnamed_25 = 1;
pub const TEXT_STATE_NONE: C2RustUnnamed_25 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct struct_80034EC0_Entry {
    pub animation: *mut AnimationHeader,
    pub playbackSpeed: f32_0,
    pub startFrame: f32_0,
    pub frameCount: f32_0,
    pub mode: u8_0,
    pub transitionRate: f32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct struct_80034A14_arg1 {
    pub unk_00: s16,
    pub unk_02: s16,
    pub unk_04: s16,
    pub unk_06: s16,
    pub unk_08: Vec3s,
    pub unk_0E: Vec3s,
    pub unk_14: f32_0,
    pub unk_18: Vec3f,
    pub unk_24: s16,
}
pub type C2RustUnnamed_26 = libc::c_uint;
pub const MTXMODE_APPLY: C2RustUnnamed_26 = 1;
pub const MTXMODE_NEW: C2RustUnnamed_26 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FaultClient {
    pub next: *mut FaultClient,
    pub callback: u32_0,
    pub param1: u32_0,
    pub param2: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NaviColor {
    pub inner: Color_RGBA8,
    pub outer: Color_RGBA8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ArmsHook {
    pub actor: Actor,
    pub collider: ColliderQuad,
    pub hookInfo: WeaponInfo,
    pub unk_1E8: Vec3f,
    pub unk_1F4: Vec3f,
    pub grabbed: *mut Actor,
    pub grabbedDistDiff: Vec3f,
    pub timer: s16,
    pub actionFunc: ArmsHookActionFunc,
}
pub type ArmsHookActionFunc
    =
    Option<unsafe extern "C" fn(_: *mut ArmsHook, _: *mut GlobalContext)
               -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TargetRangeParams {
    pub rangeSq: f32_0,
    pub leashScale: f32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EnPart {
    pub actor: Actor,
    pub action: u8_0,
    pub timer: s16,
    pub displayList: *mut Gfx,
    pub rotZ: f32_0,
    pub rotZSpeed: f32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DoorLockInfo {
    pub chainAngle: f32_0,
    pub chainLength: f32_0,
    pub yShift: f32_0,
    pub chainsScale: f32_0,
    pub chainsRotZInit: f32_0,
    pub chainDL: *mut Gfx,
    pub lockDL: *mut Gfx,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct struct_80116130 {
    pub sub_00: struct_80116130_0,
    pub unk_10: f32_0,
    pub unk_14: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct struct_80116130_0 {
    pub unk_00: s16,
    pub unk_02: s16,
    pub unk_04: s16,
    pub unk_06: s16,
    pub unk_08: s16,
    pub unk_0A: s16,
    pub unk_0C: u8_0,
}
static mut sCurCeilingPoly: *mut CollisionPoly =
    0 as *const CollisionPoly as *mut CollisionPoly;
static mut sCurCeilingBgId: s32 = 0;
#[no_mangle]
pub unsafe extern "C" fn ActorShape_Init(mut shape: *mut ActorShape,
                                         mut yOffset: f32_0,
                                         mut shadowDraw: ActorShadowFunc,
                                         mut shadowScale: f32_0) {
    (*shape).yOffset = yOffset; // One for each foot
    (*shape).shadowDraw = shadowDraw;
    (*shape).shadowScale = shadowScale;
    (*shape).shadowAlpha = 255 as libc::c_int as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn ActorShadow_Draw(mut actor: *mut Actor,
                                          mut lights: *mut Lights,
                                          mut globalCtx: *mut GlobalContext,
                                          mut dlist: *mut Gfx,
                                          mut color: *mut Color_RGBA8) {
    let mut temp1: f32_0 = 0.;
    let mut temp2: f32_0 = 0.;
    let mut sp60: MtxF = MtxF{mf: [[0.; 4]; 4],};
    if !(*actor).floorPoly.is_null() {
        temp1 = (*actor).world.pos.y - (*actor).floorHeight;
        if temp1 >= -50.0f32 && temp1 < 500.0f32 {
            let mut __gfxCtx: *mut GraphicsContext =
                0 as *mut GraphicsContext;
            let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
            __gfxCtx = (*globalCtx).state.gfxCtx;
            Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                            b"../z_actor.c\x00" as *const u8 as
                                *const libc::c_char, 1553 as libc::c_int);
            (*__gfxCtx).polyOpa.p =
                Gfx_CallSetupDL((*__gfxCtx).polyOpa.p,
                                0x2c as libc::c_int as u32_0);
            let fresh0 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g: *mut Gfx = fresh0;
            (*_g).words.w0 =
                (0xfc as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (((31 as libc::c_int as u32_0 &
                           (((0x1 as libc::c_int) << 4 as libc::c_int) -
                                1 as libc::c_int) as libc::c_uint) <<
                          20 as libc::c_int |
                          (31 as libc::c_int as u32_0 &
                               (((0x1 as libc::c_int) << 5 as libc::c_int) -
                                    1 as libc::c_int) as libc::c_uint) <<
                              15 as libc::c_int |
                          (1 as libc::c_int as u32_0 &
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
                                    (((0x1 as libc::c_int) <<
                                          5 as libc::c_int) -
                                         1 as libc::c_int) as libc::c_uint) <<
                                   0 as libc::c_int)) &
                         (((0x1 as libc::c_int) << 24 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g).words.w1 =
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
                    (7 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 3 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        9 as libc::c_int |
                    ((31 as libc::c_int as u32_0 &
                          (((0x1 as libc::c_int) << 4 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         24 as libc::c_int |
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
            temp1 =
                if temp1 < 0.0f32 {
                    0.0f32
                } else if temp1 > 150.0f32 { 150.0f32 } else { temp1 };
            temp2 =
                1.0f32 -
                    temp1 * (1.0f32 / 350 as libc::c_int as libc::c_float);
            if !color.is_null() {
                let fresh1 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_0: *mut Gfx = fresh1;
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
                    ((*color).r as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        ((*color).g as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            16 as libc::c_int |
                        ((*color).b as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            8 as libc::c_int |
                        (((*actor).shape.shadowAlpha as libc::c_int as
                              libc::c_float * temp2) as u32_0 &
                             0xff as libc::c_int as libc::c_uint &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int
            } else {
                let fresh2 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_1: *mut Gfx = fresh2;
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
                        (((*actor).shape.shadowAlpha as libc::c_int as
                              libc::c_float * temp2) as u32_0 &
                             0xff as libc::c_int as libc::c_uint &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int
            }
            func_80038A28((*actor).floorPoly, (*actor).world.pos.x,
                          (*actor).floorHeight, (*actor).world.pos.z,
                          &mut sp60);
            Matrix_Put(&mut sp60);
            if dlist != gCircleShadowDL.as_mut_ptr() {
                Matrix_RotateY((*actor).shape.rot.y as libc::c_int as
                                   libc::c_float *
                                   (3.14159265358979323846f32 /
                                        0x8000 as libc::c_int as
                                            libc::c_float),
                               MTXMODE_APPLY as libc::c_int as u8_0);
            }
            temp2 =
                (1.0f32 -
                     temp1 * (1.0f32 / 350 as libc::c_int as libc::c_float)) *
                    (*actor).shape.shadowScale;
            Matrix_Scale((*actor).scale.x * temp2, 1.0f32,
                         (*actor).scale.z * temp2,
                         MTXMODE_APPLY as libc::c_int as u8_0);
            let fresh3 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
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
                    (((0 as libc::c_int | 0x2 as libc::c_int) ^
                          0x1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_2).words.w1 =
                Matrix_NewMtx((*globalCtx).state.gfxCtx,
                              b"../z_actor.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              1588 as libc::c_int) as libc::c_uint;
            let fresh4 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_3: *mut Gfx = fresh4;
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
            (*_g_3).words.w1 = dlist as libc::c_uint;
            Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                             b"../z_actor.c\x00" as *const u8 as
                                 *const libc::c_char, 1594 as libc::c_int);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ActorShadow_DrawCircle(mut actor: *mut Actor,
                                                mut lights: *mut Lights,
                                                mut globalCtx:
                                                    *mut GlobalContext) {
    ActorShadow_Draw(actor, lights, globalCtx, gCircleShadowDL.as_mut_ptr(),
                     0 as *mut Color_RGBA8);
}
#[no_mangle]
pub unsafe extern "C" fn ActorShadow_DrawWhiteCircle(mut actor: *mut Actor,
                                                     mut lights: *mut Lights,
                                                     mut globalCtx:
                                                         *mut GlobalContext) {
    static mut white: Color_RGBA8 =
        {
            let mut init =
                Color_RGBA8{r: 255 as libc::c_int as u8_0,
                            g: 255 as libc::c_int as u8_0,
                            b: 255 as libc::c_int as u8_0,
                            a: 255 as libc::c_int as u8_0,};
            init
        };
    ActorShadow_Draw(actor, lights, globalCtx, gCircleShadowDL.as_mut_ptr(),
                     &mut white);
}
#[no_mangle]
pub unsafe extern "C" fn ActorShadow_DrawHorse(mut actor: *mut Actor,
                                               mut lights: *mut Lights,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    ActorShadow_Draw(actor, lights, globalCtx, gHorseShadowDL.as_mut_ptr(),
                     0 as *mut Color_RGBA8);
}
#[no_mangle]
pub unsafe extern "C" fn ActorShadow_DrawFoot(mut globalCtx:
                                                  *mut GlobalContext,
                                              mut light: *mut Light,
                                              mut arg2: *mut MtxF,
                                              mut arg3: s32, mut arg4: f32_0,
                                              mut arg5: f32_0,
                                              mut arg6: f32_0) {
    let mut pad1: s32 = 0;
    let mut sp58: f32_0 = 0.;
    let mut pad2: [s32; 2] = [0; 2];
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_actor.c\x00" as *const u8 as *const libc::c_char,
                    1661 as libc::c_int);
    let fresh5 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g: *mut Gfx = fresh5;
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
            (((if arg3 as libc::c_float * 0.00005f32 > 1.0f32 {
                   1.0f32
               } else { (arg3 as libc::c_float) * 0.00005f32 }) * arg4) as
                 u32_0 & 0xff as libc::c_int as libc::c_uint &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    sp58 =
        Math_FAtan2F((*light).l.dir[0 as libc::c_int as usize] as f32_0,
                     (*light).l.dir[2 as libc::c_int as usize] as f32_0);
    arg6 *=
        4.5f32 -
            (*light).l.dir[1 as libc::c_int as usize] as libc::c_int as
                libc::c_float * 0.035f32;
    arg6 = if arg6 < 1.0f32 { 1.0f32 } else { arg6 };
    Matrix_Put(arg2);
    Matrix_RotateY(sp58, MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_Scale(arg5, 1.0f32, arg5 * arg6,
                 MTXMODE_APPLY as libc::c_int as u8_0);
    let fresh6 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_0: *mut Gfx = fresh6;
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
            (((0 as libc::c_int | 0x2 as libc::c_int) ^ 0x1 as libc::c_int) as
                 u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_0).words.w1 =
        Matrix_NewMtx((*globalCtx).state.gfxCtx,
                      b"../z_actor.c\x00" as *const u8 as *const libc::c_char
                          as *mut libc::c_char, 1687 as libc::c_int) as
            libc::c_uint;
    let fresh7 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_1: *mut Gfx = fresh7;
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
    (*_g_1).words.w1 = gFootShadowDL.as_mut_ptr() as libc::c_uint;
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_actor.c\x00" as *const u8 as *const libc::c_char,
                     1693 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn ActorShadow_DrawFeet(mut actor: *mut Actor,
                                              mut lights: *mut Lights,
                                              mut globalCtx:
                                                  *mut GlobalContext) {
    let mut distToFloor: f32_0 = (*actor).world.pos.y - (*actor).floorHeight;
    if distToFloor > 20.0f32 {
        let mut shadowScale: f32_0 = (*actor).shape.shadowScale;
        let mut shadowAlpha: u8_0 = (*actor).shape.shadowAlpha;
        let mut alphaRatio: f32_0 = 0.;
        (*actor).shape.shadowScale *= 0.3f32;
        alphaRatio = (distToFloor - 20.0f32) * 0.02f32;
        (*actor).shape.shadowAlpha =
            ((*actor).shape.shadowAlpha as f32_0 *
                 (if alphaRatio > 1.0f32 { 1.0f32 } else { alphaRatio })) as
                u8_0;
        ActorShadow_DrawCircle(actor, lights, globalCtx);
        (*actor).shape.shadowScale = shadowScale;
        (*actor).shape.shadowAlpha = shadowAlpha
    }
    if distToFloor < 200.0f32 {
        let mut floorMtx: MtxF = MtxF{mf: [[0.; 4]; 4],};
        let mut floorHeight: [f32_0; 2] = [0.; 2];
        let mut distToFloor_0: f32_0 = 0.;
        let mut shadowAlpha_0: f32_0 = 0.;
        let mut shadowScaleX: f32_0 = 0.;
        let mut shadowScaleZ: f32_0 = 0.;
        let mut lightPtr: *mut Light = 0 as *mut Light;
        let mut lightNum: s32 = 0;
        let mut lightNumMax: s32 = 0;
        let mut i: s32 = 0;
        let mut j: s32 = 0;
        let mut numLights: s32 =
            (*lights).numLights as libc::c_int - 2 as libc::c_int;
        let mut firstLightPtr: *mut Light =
            &mut *(*lights).l.l.as_mut_ptr().offset(0 as libc::c_int as isize)
                as *mut Light;
        let mut feetPosPtr: *mut Vec3f = (*actor).shape.feetPos.as_mut_ptr();
        let mut floorHeightPtr: *mut f32_0 = floorHeight.as_mut_ptr();
        let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
        let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
        __gfxCtx = (*globalCtx).state.gfxCtx;
        Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                        b"../z_actor.c\x00" as *const u8 as
                            *const libc::c_char, 1741 as libc::c_int);
        (*__gfxCtx).polyOpa.p =
            Gfx_CallSetupDL((*__gfxCtx).polyOpa.p,
                            0x2c as libc::c_int as u32_0);
        (*actor).shape.feetFloorFlags = 0 as libc::c_int as u8_0;
        i = 0 as libc::c_int;
        while i < 2 as libc::c_int {
            (*feetPosPtr).y += 50.0f32;
            *floorHeightPtr =
                func_800BFCB8(globalCtx, &mut floorMtx, feetPosPtr);
            (*feetPosPtr).y -= 50.0f32;
            (*actor).shape.feetFloorFlags =
                (((*actor).shape.feetFloorFlags as libc::c_int) <<
                     1 as libc::c_int) as u8_0;
            distToFloor_0 = (*feetPosPtr).y - *floorHeightPtr;
            if -1.0f32 <= distToFloor_0 && distToFloor_0 < 500.0f32 {
                if distToFloor_0 <= 0.0f32 {
                    (*actor).shape.feetFloorFlags =
                        (*actor).shape.feetFloorFlags.wrapping_add(1)
                }
                distToFloor_0 =
                    if distToFloor_0 > 30.0f32 {
                        30.0f32
                    } else { distToFloor_0 };
                shadowAlpha_0 =
                    (*actor).shape.shadowAlpha as f32_0 *
                        (1.0f32 - distToFloor_0 * (1.0f32 / 30.0f32));
                distToFloor_0 =
                    if distToFloor_0 > 30.0f32 {
                        30.0f32
                    } else { distToFloor_0 };
                shadowScaleZ =
                    1.0f32 - distToFloor_0 * (1.0f32 / (30.0f32 + 40.0f32));
                shadowScaleX =
                    shadowScaleZ * (*actor).shape.shadowScale *
                        (*actor).scale.x;
                lightNumMax = 0 as libc::c_int;
                lightPtr = firstLightPtr;
                j = 0 as libc::c_int;
                while j < numLights {
                    if (*lightPtr).l.dir[1 as libc::c_int as usize] as
                           libc::c_int > 0 as libc::c_int {
                        lightNum =
                            ((*lightPtr).l.col[0 as libc::c_int as usize] as
                                 libc::c_int +
                                 (*lightPtr).l.col[1 as libc::c_int as usize]
                                     as libc::c_int +
                                 (*lightPtr).l.col[2 as libc::c_int as usize]
                                     as libc::c_int) *
                                (if (*lightPtr).l.dir[1 as libc::c_int as
                                                          usize] as
                                        libc::c_int >= 0 as libc::c_int {
                                     (*lightPtr).l.dir[1 as libc::c_int as
                                                           usize] as
                                         libc::c_int
                                 } else {
                                     -((*lightPtr).l.dir[1 as libc::c_int as
                                                             usize] as
                                           libc::c_int)
                                 });
                        if lightNum > 0 as libc::c_int {
                            lightNumMax += lightNum;
                            ActorShadow_DrawFoot(globalCtx, lightPtr,
                                                 &mut floorMtx, lightNum,
                                                 shadowAlpha_0, shadowScaleX,
                                                 shadowScaleZ);
                        }
                    }
                    lightPtr = lightPtr.offset(1);
                    j += 1
                }
                j = 0 as libc::c_int;
                while j < 2 as libc::c_int {
                    if (*lightPtr).l.dir[1 as libc::c_int as usize] as
                           libc::c_int > 0 as libc::c_int {
                        lightNum =
                            ((*lightPtr).l.col[0 as libc::c_int as usize] as
                                 libc::c_int +
                                 (*lightPtr).l.col[1 as libc::c_int as usize]
                                     as libc::c_int +
                                 (*lightPtr).l.col[2 as libc::c_int as usize]
                                     as libc::c_int) *
                                (if (*lightPtr).l.dir[1 as libc::c_int as
                                                          usize] as
                                        libc::c_int >= 0 as libc::c_int {
                                     (*lightPtr).l.dir[1 as libc::c_int as
                                                           usize] as
                                         libc::c_int
                                 } else {
                                     -((*lightPtr).l.dir[1 as libc::c_int as
                                                             usize] as
                                           libc::c_int)
                                 }) - lightNumMax * 8 as libc::c_int;
                        if lightNum > 0 as libc::c_int {
                            ActorShadow_DrawFoot(globalCtx, lightPtr,
                                                 &mut floorMtx, lightNum,
                                                 shadowAlpha_0, shadowScaleX,
                                                 shadowScaleZ);
                        }
                    }
                    lightPtr = lightPtr.offset(1);
                    j += 1
                }
            }
            feetPosPtr = feetPosPtr.offset(1);
            floorHeightPtr = floorHeightPtr.offset(1);
            i += 1
        }
        if (*actor).bgCheckFlags as libc::c_int & 1 as libc::c_int == 0 {
            (*actor).shape.feetFloorFlags = 0 as libc::c_int as u8_0
        } else if (*actor).shape.feetFloorFlags as libc::c_int ==
                      3 as libc::c_int {
            let mut footDistY: f32_0 =
                (*actor).shape.feetPos[FOOT_LEFT as libc::c_int as usize].y -
                    (*actor).shape.feetPos[FOOT_RIGHT as libc::c_int as
                                               usize].y;
            if floorHeight[0 as libc::c_int as usize] + footDistY <
                   floorHeight[1 as libc::c_int as usize] - footDistY {
                (*actor).shape.feetFloorFlags = 2 as libc::c_int as u8_0
            } else {
                (*actor).shape.feetFloorFlags = 1 as libc::c_int as u8_0
            }
        }
        Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                         b"../z_actor.c\x00" as *const u8 as
                             *const libc::c_char, 1831 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Actor_SetFeetPos(mut actor: *mut Actor,
                                          mut limbIndex: s32,
                                          mut leftFootIndex: s32,
                                          mut leftFootPos: *mut Vec3f,
                                          mut rightFootIndex: s32,
                                          mut rightFootPos: *mut Vec3f) {
    if limbIndex == leftFootIndex {
        Matrix_MultVec3f(leftFootPos,
                         &mut *(*actor).shape.feetPos.as_mut_ptr().offset(FOOT_LEFT
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              isize));
    } else if limbIndex == rightFootIndex {
        Matrix_MultVec3f(rightFootPos,
                         &mut *(*actor).shape.feetPos.as_mut_ptr().offset(FOOT_RIGHT
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              isize));
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_8002BE04(mut globalCtx: *mut GlobalContext,
                                       mut arg1: *mut Vec3f,
                                       mut arg2: *mut Vec3f,
                                       mut arg3: *mut f32_0) {
    SkinMatrix_Vec3fMtxFMultXYZW(&mut (*globalCtx).viewProjectionMtxF, arg1,
                                 arg2, arg3);
    *arg3 = if *arg3 < 1.0f32 { 1.0f32 } else { (1.0f32) / *arg3 };
}
// size = 0x8
#[no_mangle]
pub static mut sNaviColorList: [NaviColor; 13] =
    [{
         let mut init =
             NaviColor{inner:
                           {
                               let mut init =
                                   Color_RGBA8{r: 0 as libc::c_int as u8_0,
                                               g: 255 as libc::c_int as u8_0,
                                               b: 0 as libc::c_int as u8_0,
                                               a:
                                                   255 as libc::c_int as
                                                       u8_0,};
                               init
                           },
                       outer:
                           {
                               let mut init =
                                   Color_RGBA8{r: 0 as libc::c_int as u8_0,
                                               g: 255 as libc::c_int as u8_0,
                                               b: 0 as libc::c_int as u8_0,
                                               a: 0 as libc::c_int as u8_0,};
                               init
                           },};
         init
     },
     {
         let mut init =
             NaviColor{inner:
                           {
                               let mut init =
                                   Color_RGBA8{r: 0 as libc::c_int as u8_0,
                                               g: 255 as libc::c_int as u8_0,
                                               b: 0 as libc::c_int as u8_0,
                                               a:
                                                   255 as libc::c_int as
                                                       u8_0,};
                               init
                           },
                       outer:
                           {
                               let mut init =
                                   Color_RGBA8{r: 0 as libc::c_int as u8_0,
                                               g: 255 as libc::c_int as u8_0,
                                               b: 0 as libc::c_int as u8_0,
                                               a: 0 as libc::c_int as u8_0,};
                               init
                           },};
         init
     },
     {
         let mut init =
             NaviColor{inner:
                           {
                               let mut init =
                                   Color_RGBA8{r: 255 as libc::c_int as u8_0,
                                               g: 255 as libc::c_int as u8_0,
                                               b: 255 as libc::c_int as u8_0,
                                               a:
                                                   255 as libc::c_int as
                                                       u8_0,};
                               init
                           },
                       outer:
                           {
                               let mut init =
                                   Color_RGBA8{r: 0 as libc::c_int as u8_0,
                                               g: 0 as libc::c_int as u8_0,
                                               b: 255 as libc::c_int as u8_0,
                                               a: 0 as libc::c_int as u8_0,};
                               init
                           },};
         init
     },
     {
         let mut init =
             NaviColor{inner:
                           {
                               let mut init =
                                   Color_RGBA8{r: 0 as libc::c_int as u8_0,
                                               g: 255 as libc::c_int as u8_0,
                                               b: 0 as libc::c_int as u8_0,
                                               a:
                                                   255 as libc::c_int as
                                                       u8_0,};
                               init
                           },
                       outer:
                           {
                               let mut init =
                                   Color_RGBA8{r: 0 as libc::c_int as u8_0,
                                               g: 255 as libc::c_int as u8_0,
                                               b: 0 as libc::c_int as u8_0,
                                               a: 0 as libc::c_int as u8_0,};
                               init
                           },};
         init
     },
     {
         let mut init =
             NaviColor{inner:
                           {
                               let mut init =
                                   Color_RGBA8{r: 150 as libc::c_int as u8_0,
                                               g: 150 as libc::c_int as u8_0,
                                               b: 255 as libc::c_int as u8_0,
                                               a:
                                                   255 as libc::c_int as
                                                       u8_0,};
                               init
                           },
                       outer:
                           {
                               let mut init =
                                   Color_RGBA8{r: 150 as libc::c_int as u8_0,
                                               g: 150 as libc::c_int as u8_0,
                                               b: 255 as libc::c_int as u8_0,
                                               a: 0 as libc::c_int as u8_0,};
                               init
                           },};
         init
     },
     {
         let mut init =
             NaviColor{inner:
                           {
                               let mut init =
                                   Color_RGBA8{r: 255 as libc::c_int as u8_0,
                                               g: 255 as libc::c_int as u8_0,
                                               b: 0 as libc::c_int as u8_0,
                                               a:
                                                   255 as libc::c_int as
                                                       u8_0,};
                               init
                           },
                       outer:
                           {
                               let mut init =
                                   Color_RGBA8{r: 200 as libc::c_int as u8_0,
                                               g: 155 as libc::c_int as u8_0,
                                               b: 0 as libc::c_int as u8_0,
                                               a: 0 as libc::c_int as u8_0,};
                               init
                           },};
         init
     },
     {
         let mut init =
             NaviColor{inner:
                           {
                               let mut init =
                                   Color_RGBA8{r: 0 as libc::c_int as u8_0,
                                               g: 255 as libc::c_int as u8_0,
                                               b: 0 as libc::c_int as u8_0,
                                               a:
                                                   255 as libc::c_int as
                                                       u8_0,};
                               init
                           },
                       outer:
                           {
                               let mut init =
                                   Color_RGBA8{r: 0 as libc::c_int as u8_0,
                                               g: 255 as libc::c_int as u8_0,
                                               b: 0 as libc::c_int as u8_0,
                                               a: 0 as libc::c_int as u8_0,};
                               init
                           },};
         init
     },
     {
         let mut init =
             NaviColor{inner:
                           {
                               let mut init =
                                   Color_RGBA8{r: 0 as libc::c_int as u8_0,
                                               g: 255 as libc::c_int as u8_0,
                                               b: 0 as libc::c_int as u8_0,
                                               a:
                                                   255 as libc::c_int as
                                                       u8_0,};
                               init
                           },
                       outer:
                           {
                               let mut init =
                                   Color_RGBA8{r: 0 as libc::c_int as u8_0,
                                               g: 255 as libc::c_int as u8_0,
                                               b: 0 as libc::c_int as u8_0,
                                               a: 0 as libc::c_int as u8_0,};
                               init
                           },};
         init
     },
     {
         let mut init =
             NaviColor{inner:
                           {
                               let mut init =
                                   Color_RGBA8{r: 0 as libc::c_int as u8_0,
                                               g: 255 as libc::c_int as u8_0,
                                               b: 0 as libc::c_int as u8_0,
                                               a:
                                                   255 as libc::c_int as
                                                       u8_0,};
                               init
                           },
                       outer:
                           {
                               let mut init =
                                   Color_RGBA8{r: 0 as libc::c_int as u8_0,
                                               g: 255 as libc::c_int as u8_0,
                                               b: 0 as libc::c_int as u8_0,
                                               a: 0 as libc::c_int as u8_0,};
                               init
                           },};
         init
     },
     {
         let mut init =
             NaviColor{inner:
                           {
                               let mut init =
                                   Color_RGBA8{r: 255 as libc::c_int as u8_0,
                                               g: 255 as libc::c_int as u8_0,
                                               b: 0 as libc::c_int as u8_0,
                                               a:
                                                   255 as libc::c_int as
                                                       u8_0,};
                               init
                           },
                       outer:
                           {
                               let mut init =
                                   Color_RGBA8{r: 200 as libc::c_int as u8_0,
                                               g: 155 as libc::c_int as u8_0,
                                               b: 0 as libc::c_int as u8_0,
                                               a: 0 as libc::c_int as u8_0,};
                               init
                           },};
         init
     },
     {
         let mut init =
             NaviColor{inner:
                           {
                               let mut init =
                                   Color_RGBA8{r: 0 as libc::c_int as u8_0,
                                               g: 255 as libc::c_int as u8_0,
                                               b: 0 as libc::c_int as u8_0,
                                               a:
                                                   255 as libc::c_int as
                                                       u8_0,};
                               init
                           },
                       outer:
                           {
                               let mut init =
                                   Color_RGBA8{r: 0 as libc::c_int as u8_0,
                                               g: 255 as libc::c_int as u8_0,
                                               b: 0 as libc::c_int as u8_0,
                                               a: 0 as libc::c_int as u8_0,};
                               init
                           },};
         init
     },
     {
         let mut init =
             NaviColor{inner:
                           {
                               let mut init =
                                   Color_RGBA8{r: 0 as libc::c_int as u8_0,
                                               g: 255 as libc::c_int as u8_0,
                                               b: 0 as libc::c_int as u8_0,
                                               a:
                                                   255 as libc::c_int as
                                                       u8_0,};
                               init
                           },
                       outer:
                           {
                               let mut init =
                                   Color_RGBA8{r: 0 as libc::c_int as u8_0,
                                               g: 255 as libc::c_int as u8_0,
                                               b: 0 as libc::c_int as u8_0,
                                               a: 0 as libc::c_int as u8_0,};
                               init
                           },};
         init
     },
     {
         let mut init =
             NaviColor{inner:
                           {
                               let mut init =
                                   Color_RGBA8{r: 0 as libc::c_int as u8_0,
                                               g: 255 as libc::c_int as u8_0,
                                               b: 0 as libc::c_int as u8_0,
                                               a:
                                                   255 as libc::c_int as
                                                       u8_0,};
                               init
                           },
                       outer:
                           {
                               let mut init =
                                   Color_RGBA8{r: 0 as libc::c_int as u8_0,
                                               g: 255 as libc::c_int as u8_0,
                                               b: 0 as libc::c_int as u8_0,
                                               a: 0 as libc::c_int as u8_0,};
                               init
                           },};
         init
     }];
// unused
#[no_mangle]
pub static mut D_80115FF0: [Gfx; 1] =
    [Gfx{words:
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
pub unsafe extern "C" fn func_8002BE64(mut targetCtx: *mut TargetContext,
                                       mut index: s32, mut arg2: f32_0,
                                       mut arg3: f32_0, mut arg4: f32_0) {
    (*targetCtx).arr_50[index as usize].pos.x = arg2;
    (*targetCtx).arr_50[index as usize].pos.y = arg3;
    (*targetCtx).arr_50[index as usize].pos.z = arg4;
    (*targetCtx).arr_50[index as usize].unk_0C = (*targetCtx).unk_44;
}
#[no_mangle]
pub unsafe extern "C" fn func_8002BE98(mut targetCtx: *mut TargetContext,
                                       mut actorCategory: s32,
                                       mut globalCtx: *mut GlobalContext) {
    let mut entry: *mut TargetContextEntry = 0 as *mut TargetContextEntry;
    let mut naviColor: *mut NaviColor = 0 as *mut NaviColor;
    let mut i: s32 = 0;
    Math_Vec3f_Copy(&mut (*targetCtx).targetCenterPos,
                    &mut (*globalCtx).view.eye);
    (*targetCtx).unk_44 = 500.0f32;
    (*targetCtx).unk_48 = 0x100 as libc::c_int as s16;
    naviColor =
        &mut *sNaviColorList.as_mut_ptr().offset(actorCategory as isize) as
            *mut NaviColor;
    entry =
        &mut *(*targetCtx).arr_50.as_mut_ptr().offset(0 as libc::c_int as
                                                          isize) as
            *mut TargetContextEntry;
    i = 0 as libc::c_int;
    while i <
              (::std::mem::size_of::<[TargetContextEntry; 3]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<TargetContextEntry>()
                                                   as libc::c_ulong) as s32 {
        func_8002BE64(targetCtx, i, 0.0f32, 0.0f32, 0.0f32);
        (*entry).color.r = (*naviColor).inner.r;
        (*entry).color.g = (*naviColor).inner.g;
        (*entry).color.b = (*naviColor).inner.b;
        entry = entry.offset(1);
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_8002BF60(mut targetCtx: *mut TargetContext,
                                       mut actor: *mut Actor,
                                       mut actorCategory: s32,
                                       mut globalCtx: *mut GlobalContext) {
    let mut naviColor: *mut NaviColor =
        &mut *sNaviColorList.as_mut_ptr().offset(actorCategory as isize) as
            *mut NaviColor;
    (*targetCtx).naviRefPos.x = (*actor).focus.pos.x;
    (*targetCtx).naviRefPos.y =
        (*actor).focus.pos.y + (*actor).targetArrowOffset * (*actor).scale.y;
    (*targetCtx).naviRefPos.z = (*actor).focus.pos.z;
    (*targetCtx).naviInner.r = (*naviColor).inner.r as f32_0;
    (*targetCtx).naviInner.g = (*naviColor).inner.g as f32_0;
    (*targetCtx).naviInner.b = (*naviColor).inner.b as f32_0;
    (*targetCtx).naviInner.a = (*naviColor).inner.a as f32_0;
    (*targetCtx).naviOuter.r = (*naviColor).outer.r as f32_0;
    (*targetCtx).naviOuter.g = (*naviColor).outer.g as f32_0;
    (*targetCtx).naviOuter.b = (*naviColor).outer.b as f32_0;
    (*targetCtx).naviOuter.a = (*naviColor).outer.a as f32_0;
}
#[no_mangle]
pub unsafe extern "C" fn func_8002C0C0(mut targetCtx: *mut TargetContext,
                                       mut actor: *mut Actor,
                                       mut globalCtx: *mut GlobalContext) {
    (*targetCtx).arrowPointedActor = 0 as *mut Actor;
    (*targetCtx).targetedActor = 0 as *mut Actor;
    (*targetCtx).unk_40 = 0.0f32;
    (*targetCtx).unk_8C = 0 as *mut Actor;
    (*targetCtx).bgmEnemy = 0 as *mut Actor;
    (*targetCtx).unk_4B = 0 as libc::c_int as u8_0;
    (*targetCtx).unk_4C = 0 as libc::c_int as s8;
    func_8002BF60(targetCtx, actor, (*actor).category as s32, globalCtx);
    func_8002BE98(targetCtx, (*actor).category as s32, globalCtx);
}
#[no_mangle]
pub unsafe extern "C" fn func_8002C124(mut targetCtx: *mut TargetContext,
                                       mut globalCtx: *mut GlobalContext) {
    let mut actor: *mut Actor = (*targetCtx).targetedActor;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_actor.c\x00" as *const u8 as *const libc::c_char,
                    2029 as libc::c_int);
    if (*targetCtx).unk_48 as libc::c_int != 0 as libc::c_int {
        let mut entry: *mut TargetContextEntry = 0 as *mut TargetContextEntry;
        let mut player: *mut Player = 0 as *mut Player;
        let mut spCE: s16 = 0;
        let mut temp1: f32_0 = 0.;
        let mut spBC: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
        let mut spB8: s32 = 0;
        let mut spB4: f32_0 = 0.;
        let mut spB0: s32 = 0;
        let mut spAC: s32 = 0;
        let mut var1: f32_0 = 0.;
        let mut var2: f32_0 = 0.;
        let mut i: s32 = 0;
        player =
            (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                                 usize].head as *mut Player;
        spCE = 0xff as libc::c_int as s16;
        var1 = 1.0f32;
        if (*targetCtx).unk_4B as libc::c_int != 0 as libc::c_int {
            spB8 = 1 as libc::c_int
        } else { spB8 = 3 as libc::c_int }
        if !actor.is_null() {
            Math_Vec3f_Copy(&mut (*targetCtx).targetCenterPos,
                            &mut (*actor).focus.pos);
            var1 = (500.0f32 - (*targetCtx).unk_44) / 420.0f32
        } else {
            (*targetCtx).unk_48 =
                ((*targetCtx).unk_48 as libc::c_int - 120 as libc::c_int) as
                    s16;
            if ((*targetCtx).unk_48 as libc::c_int) < 0 as libc::c_int {
                (*targetCtx).unk_48 = 0 as libc::c_int as s16
            }
            spCE = (*targetCtx).unk_48
        }
        func_8002BE04(globalCtx, &mut (*targetCtx).targetCenterPos, &mut spBC,
                      &mut spB4);
        spBC.x = 160 as libc::c_int as libc::c_float * (spBC.x * spB4) * var1;
        spBC.x =
            if spBC.x < -320.0f32 {
                -320.0f32
            } else if spBC.x > 320.0f32 { 320.0f32 } else { spBC.x };
        spBC.y = 120 as libc::c_int as libc::c_float * (spBC.y * spB4) * var1;
        spBC.y =
            if spBC.y < -240.0f32 {
                -240.0f32
            } else if spBC.y > 240.0f32 { 240.0f32 } else { spBC.y };
        spBC.z = spBC.z * var1;
        (*targetCtx).unk_4C -= 1;
        if ((*targetCtx).unk_4C as libc::c_int) < 0 as libc::c_int {
            (*targetCtx).unk_4C = 2 as libc::c_int as s8
        }
        func_8002BE64(targetCtx, (*targetCtx).unk_4C as s32, spBC.x, spBC.y,
                      spBC.z);
        if (*player).stateFlags1 & 0x40 as libc::c_int as libc::c_uint == 0 ||
               actor != (*player).unk_664 {
            (*__gfxCtx).overlay.p =
                Gfx_CallSetupDL((*__gfxCtx).overlay.p,
                                0x39 as libc::c_int as u32_0);
            spB0 = 0 as libc::c_int;
            spAC = (*targetCtx).unk_4C as s32;
            while spB0 < spB8 {
                entry =
                    &mut *(*targetCtx).arr_50.as_mut_ptr().offset(spAC as
                                                                      isize)
                        as *mut TargetContextEntry;
                if (*entry).unk_0C < 500.0f32 {
                    if (*entry).unk_0C <= 120.0f32 {
                        var2 = 0.15f32
                    } else {
                        var2 =
                            ((*entry).unk_0C - 120.0f32) * 0.001f32 + 0.15f32
                    }
                    Matrix_Translate((*entry).pos.x, (*entry).pos.y, 0.0f32,
                                     MTXMODE_NEW as libc::c_int as u8_0);
                    Matrix_Scale(var2, 0.15f32, 1.0f32,
                                 MTXMODE_APPLY as libc::c_int as u8_0);
                    let fresh8 = (*__gfxCtx).overlay.p;
                    (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
                    let mut _g: *mut Gfx = fresh8;
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
                        ((*entry).color.r as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            24 as libc::c_int |
                            ((*entry).color.g as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                16 as libc::c_int |
                            ((*entry).color.b as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                8 as libc::c_int |
                            (spCE as u8_0 as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int;
                    Matrix_RotateZ(((*targetCtx).unk_4B as libc::c_int &
                                        0x7f as libc::c_int) as libc::c_float
                                       *
                                       (3.14159265358979323846f32 /
                                            64 as libc::c_int as
                                                libc::c_float),
                                   MTXMODE_APPLY as libc::c_int as u8_0);
                    i = 0 as libc::c_int;
                    while i < 4 as libc::c_int {
                        Matrix_RotateZ(3.14159265358979323846f32 /
                                           2 as libc::c_int as libc::c_float,
                                       MTXMODE_APPLY as libc::c_int as u8_0);
                        Matrix_Push();
                        Matrix_Translate((*entry).unk_0C, (*entry).unk_0C,
                                         0.0f32,
                                         MTXMODE_APPLY as libc::c_int as
                                             u8_0);
                        let fresh9 = (*__gfxCtx).overlay.p;
                        (*__gfxCtx).overlay.p =
                            (*__gfxCtx).overlay.p.offset(1);
                        let mut _g_0: *mut Gfx = fresh9;
                        (*_g_0).words.w0 =
                            (0xda as libc::c_int as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
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
                                          1 as libc::c_int) as libc::c_uint)
                                    << 19 as libc::c_int |
                                ((0 as libc::c_int / 8 as libc::c_int) as
                                     u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 8 as libc::c_int |
                                (((0 as libc::c_int | 0x2 as libc::c_int) ^
                                      0x1 as libc::c_int) as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 0 as libc::c_int;
                        (*_g_0).words.w1 =
                            Matrix_NewMtx((*globalCtx).state.gfxCtx,
                                          b"../z_actor.c\x00" as *const u8 as
                                              *const libc::c_char as
                                              *mut libc::c_char,
                                          2116 as libc::c_int) as
                                libc::c_uint;
                        let fresh10 = (*__gfxCtx).overlay.p;
                        (*__gfxCtx).overlay.p =
                            (*__gfxCtx).overlay.p.offset(1);
                        let mut _g_1: *mut Gfx = fresh10;
                        (*_g_1).words.w0 =
                            (0xde as libc::c_int as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                24 as libc::c_int |
                                (0 as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 16 as libc::c_int |
                                (0 as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           16 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 0 as libc::c_int;
                        (*_g_1).words.w1 =
                            gZTargetLockOnTriangleDL.as_mut_ptr() as
                                libc::c_uint;
                        Matrix_Pop();
                        i += 1
                    }
                }
                spCE =
                    (spCE as libc::c_int -
                         0xff as libc::c_int / 3 as libc::c_int) as s16;
                if (spCE as libc::c_int) < 0 as libc::c_int {
                    spCE = 0 as libc::c_int as s16
                }
                spB0 += 1;
                spAC = (spAC + 1 as libc::c_int) % 3 as libc::c_int
            }
        }
    }
    actor = (*targetCtx).unk_94;
    if !actor.is_null() &&
           (*actor).flags &
               ((1 as libc::c_int) << 27 as libc::c_int) as libc::c_uint == 0
       {
        let mut naviColor: *mut NaviColor =
            &mut *sNaviColorList.as_mut_ptr().offset((*actor).category as
                                                         isize) as
                *mut NaviColor;
        (*__gfxCtx).polyXlu.p =
            Gfx_CallSetupDL((*__gfxCtx).polyXlu.p,
                            0x7 as libc::c_int as u32_0);
        Matrix_Translate((*actor).focus.pos.x,
                         (*actor).focus.pos.y +
                             (*actor).targetArrowOffset * (*actor).scale.y +
                             17.0f32, (*actor).focus.pos.z,
                         MTXMODE_NEW as libc::c_int as u8_0);
        Matrix_RotateY((*globalCtx).gameplayFrames.wrapping_mul(3000 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint)
                           as u16_0 as f32_0 *
                           (3.14159265358979323846f32 /
                                0x8000 as libc::c_int as libc::c_float),
                       MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_Scale(((*gGameInfo).data[(17 as libc::c_int * 6 as libc::c_int
                                             * 16 as libc::c_int +
                                             27 as libc::c_int) as usize] as
                          libc::c_int + 35 as libc::c_int) as libc::c_float /
                         1000.0f32,
                     ((*gGameInfo).data[(17 as libc::c_int * 6 as libc::c_int
                                             * 16 as libc::c_int +
                                             28 as libc::c_int) as usize] as
                          libc::c_int + 60 as libc::c_int) as libc::c_float /
                         1000.0f32,
                     ((*gGameInfo).data[(17 as libc::c_int * 6 as libc::c_int
                                             * 16 as libc::c_int +
                                             29 as libc::c_int) as usize] as
                          libc::c_int + 50 as libc::c_int) as libc::c_float /
                         1000.0f32, MTXMODE_APPLY as libc::c_int as u8_0);
        let fresh11 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_2: *mut Gfx = fresh11;
        (*_g_2).words.w0 =
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
        (*_g_2).words.w1 =
            ((*naviColor).inner.r as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((*naviColor).inner.g as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                ((*naviColor).inner.b as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (255 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh12 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_3: *mut Gfx = fresh12;
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
                (((0 as libc::c_int | 0x2 as libc::c_int) ^
                      0x1 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_3).words.w1 =
            Matrix_NewMtx((*globalCtx).state.gfxCtx,
                          b"../z_actor.c\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                          2153 as libc::c_int) as libc::c_uint;
        let fresh13 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_4: *mut Gfx = fresh13;
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
        (*_g_4).words.w1 = gZTargetArrowDL.as_mut_ptr() as libc::c_uint
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_actor.c\x00" as *const u8 as *const libc::c_char,
                     2158 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn func_8002C7BC(mut targetCtx: *mut TargetContext,
                                       mut player: *mut Player,
                                       mut actorArg: *mut Actor,
                                       mut globalCtx: *mut GlobalContext) {
    let mut pad: s32 = 0;
    let mut unkActor: *mut Actor = 0 as *mut Actor;
    let mut actorCategory: s32 = 0;
    let mut sp50: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp4C: f32_0 = 0.;
    let mut temp1: f32_0 = 0.;
    let mut temp2: f32_0 = 0.;
    let mut temp3: f32_0 = 0.;
    let mut temp4: f32_0 = 0.;
    let mut temp5: f32_0 = 0.;
    let mut temp6: f32_0 = 0.;
    let mut lockOnSfxId: s32 = 0;
    unkActor = 0 as *mut Actor;
    if !(*player).unk_664.is_null() &&
           (*player).unk_84B[(*player).unk_846 as usize] as libc::c_int ==
               2 as libc::c_int {
        (*targetCtx).unk_94 = 0 as *mut Actor
    } else {
        func_80032AF0(globalCtx, &mut (*globalCtx).actorCtx, &mut unkActor,
                      player);
        (*targetCtx).unk_94 = unkActor
    }
    if !(*targetCtx).unk_8C.is_null() {
        unkActor = (*targetCtx).unk_8C;
        (*targetCtx).unk_8C = 0 as *mut Actor
    } else if !actorArg.is_null() { unkActor = actorArg }
    if !unkActor.is_null() {
        actorCategory = (*unkActor).category as s32
    } else { actorCategory = (*player).actor.category as s32 }
    if unkActor != (*targetCtx).arrowPointedActor ||
           actorCategory != (*targetCtx).activeCategory as libc::c_int {
        (*targetCtx).arrowPointedActor = unkActor;
        (*targetCtx).activeCategory = actorCategory as u8_0;
        (*targetCtx).unk_40 = 1.0f32
    }
    if unkActor.is_null() { unkActor = &mut (*player).actor }
    if Math_StepToF(&mut (*targetCtx).unk_40, 0.0f32, 0.25f32) ==
           0 as libc::c_int {
        temp1 = 0.25f32 / (*targetCtx).unk_40;
        temp2 = (*unkActor).world.pos.x - (*targetCtx).naviRefPos.x;
        temp3 =
            (*unkActor).world.pos.y +
                (*unkActor).targetArrowOffset * (*unkActor).scale.y -
                (*targetCtx).naviRefPos.y;
        temp4 = (*unkActor).world.pos.z - (*targetCtx).naviRefPos.z;
        (*targetCtx).naviRefPos.x += temp2 * temp1;
        (*targetCtx).naviRefPos.y += temp3 * temp1;
        (*targetCtx).naviRefPos.z += temp4 * temp1
    } else { func_8002BF60(targetCtx, unkActor, actorCategory, globalCtx); }
    if !actorArg.is_null() &&
           (*targetCtx).unk_4B as libc::c_int == 0 as libc::c_int {
        func_8002BE04(globalCtx, &mut (*actorArg).focus.pos, &mut sp50,
                      &mut sp4C);
        if sp50.z <= 0.0f32 || 1.0f32 <= fabsf(sp50.x * sp4C) ||
               1.0f32 <= fabsf(sp50.y * sp4C) {
            actorArg = 0 as *mut Actor
        }
    }
    if !actorArg.is_null() {
        if actorArg != (*targetCtx).targetedActor {
            func_8002BE98(targetCtx, (*actorArg).category as s32, globalCtx);
            (*targetCtx).targetedActor = actorArg;
            if (*actorArg).id as libc::c_int == ACTOR_EN_BOOM as libc::c_int {
                (*targetCtx).unk_48 = 0 as libc::c_int as s16
            }
            lockOnSfxId =
                if (*actorArg).flags &
                       ((1 as libc::c_int) << 0 as libc::c_int |
                            (1 as libc::c_int) << 2 as libc::c_int) as
                           libc::c_uint ==
                       ((1 as libc::c_int) << 0 as libc::c_int |
                            (1 as libc::c_int) << 2 as libc::c_int) as
                           libc::c_uint {
                    0x4830 as libc::c_int
                } else { 0x4810 as libc::c_int };
            func_80078884(lockOnSfxId as u16_0);
        }
        (*targetCtx).targetCenterPos.x = (*actorArg).world.pos.x;
        (*targetCtx).targetCenterPos.y =
            (*actorArg).world.pos.y -
                (*actorArg).shape.yOffset * (*actorArg).scale.y;
        (*targetCtx).targetCenterPos.z = (*actorArg).world.pos.z;
        if (*targetCtx).unk_4B as libc::c_int == 0 as libc::c_int {
            temp5 = (500.0f32 - (*targetCtx).unk_44) * 3.0f32;
            temp6 =
                if temp5 < 30.0f32 {
                    30.0f32
                } else if 100.0f32 < temp5 { 100.0f32 } else { temp5 };
            if Math_StepToF(&mut (*targetCtx).unk_44, 80.0f32, temp6) !=
                   0 as libc::c_int {
                (*targetCtx).unk_4B = (*targetCtx).unk_4B.wrapping_add(1)
            }
        } else {
            (*targetCtx).unk_4B =
                ((*targetCtx).unk_4B as libc::c_int + 3 as libc::c_int |
                     0x80 as libc::c_int) as u8_0;
            (*targetCtx).unk_44 = 120.0f32
        }
    } else {
        (*targetCtx).targetedActor = 0 as *mut Actor;
        Math_StepToF(&mut (*targetCtx).unk_44, 500.0f32, 80.0f32);
    };
}
/* *
 * Tests if current scene switch flag is set.
 */
#[no_mangle]
pub unsafe extern "C" fn Flags_GetSwitch(mut globalCtx: *mut GlobalContext,
                                         mut flag: s32) -> s32 {
    if flag < 0x20 as libc::c_int {
        return ((*globalCtx).actorCtx.flags.swch &
                    ((1 as libc::c_int) << flag) as libc::c_uint) as s32
    } else {
        return ((*globalCtx).actorCtx.flags.tempSwch &
                    ((1 as libc::c_int) << flag - 0x20 as libc::c_int) as
                        libc::c_uint) as s32
    };
}
/* *
 * Sets current scene switch flag.
 */
#[no_mangle]
pub unsafe extern "C" fn Flags_SetSwitch(mut globalCtx: *mut GlobalContext,
                                         mut flag: s32) {
    if flag < 0x20 as libc::c_int {
        (*globalCtx).actorCtx.flags.swch |=
            ((1 as libc::c_int) << flag) as libc::c_uint
    } else {
        (*globalCtx).actorCtx.flags.tempSwch |=
            ((1 as libc::c_int) << flag - 0x20 as libc::c_int) as libc::c_uint
    };
}
/* *
 * Unsets current scene switch flag.
 */
#[no_mangle]
pub unsafe extern "C" fn Flags_UnsetSwitch(mut globalCtx: *mut GlobalContext,
                                           mut flag: s32) {
    if flag < 0x20 as libc::c_int {
        (*globalCtx).actorCtx.flags.swch &=
            !((1 as libc::c_int) << flag) as libc::c_uint
    } else {
        (*globalCtx).actorCtx.flags.tempSwch &=
            !((1 as libc::c_int) << flag - 0x20 as libc::c_int) as
                libc::c_uint
    };
}
/* *
 * Tests if current scene unknown flag is set.
 */
#[no_mangle]
pub unsafe extern "C" fn Flags_GetUnknown(mut globalCtx: *mut GlobalContext,
                                          mut flag: s32) -> s32 {
    if flag < 0x20 as libc::c_int {
        return ((*globalCtx).actorCtx.flags.unk0 &
                    ((1 as libc::c_int) << flag) as libc::c_uint) as s32
    } else {
        return ((*globalCtx).actorCtx.flags.unk1 &
                    ((1 as libc::c_int) << flag - 0x20 as libc::c_int) as
                        libc::c_uint) as s32
    };
}
/* *
 * Sets current scene unknown flag.
 */
#[no_mangle]
pub unsafe extern "C" fn Flags_SetUnknown(mut globalCtx: *mut GlobalContext,
                                          mut flag: s32) {
    if flag < 0x20 as libc::c_int {
        (*globalCtx).actorCtx.flags.unk0 |=
            ((1 as libc::c_int) << flag) as libc::c_uint
    } else {
        (*globalCtx).actorCtx.flags.unk1 |=
            ((1 as libc::c_int) << flag - 0x20 as libc::c_int) as libc::c_uint
    };
}
/* *
 * Unsets current scene unknown flag.
 */
#[no_mangle]
pub unsafe extern "C" fn Flags_UnsetUnknown(mut globalCtx: *mut GlobalContext,
                                            mut flag: s32) {
    if flag < 0x20 as libc::c_int {
        (*globalCtx).actorCtx.flags.unk0 &=
            !((1 as libc::c_int) << flag) as libc::c_uint
    } else {
        (*globalCtx).actorCtx.flags.unk1 &=
            !((1 as libc::c_int) << flag - 0x20 as libc::c_int) as
                libc::c_uint
    };
}
/* *
 * Tests if current scene chest flag is set.
 */
#[no_mangle]
pub unsafe extern "C" fn Flags_GetTreasure(mut globalCtx: *mut GlobalContext,
                                           mut flag: s32) -> s32 {
    return ((*globalCtx).actorCtx.flags.chest &
                ((1 as libc::c_int) << flag) as libc::c_uint) as s32;
}
/* *
 * Sets current scene chest flag.
 */
#[no_mangle]
pub unsafe extern "C" fn Flags_SetTreasure(mut globalCtx: *mut GlobalContext,
                                           mut flag: s32) {
    (*globalCtx).actorCtx.flags.chest |=
        ((1 as libc::c_int) << flag) as libc::c_uint;
}
/* *
 * Tests if current scene clear flag is set.
 */
#[no_mangle]
pub unsafe extern "C" fn Flags_GetClear(mut globalCtx: *mut GlobalContext,
                                        mut flag: s32) -> s32 {
    return ((*globalCtx).actorCtx.flags.clear &
                ((1 as libc::c_int) << flag) as libc::c_uint) as s32;
}
/* *
 * Sets current scene clear flag.
 */
#[no_mangle]
pub unsafe extern "C" fn Flags_SetClear(mut globalCtx: *mut GlobalContext,
                                        mut flag: s32) {
    (*globalCtx).actorCtx.flags.clear |=
        ((1 as libc::c_int) << flag) as libc::c_uint;
}
/* *
 * Unsets current scene clear flag.
 */
#[no_mangle]
pub unsafe extern "C" fn Flags_UnsetClear(mut globalCtx: *mut GlobalContext,
                                          mut flag: s32) {
    (*globalCtx).actorCtx.flags.clear &=
        !((1 as libc::c_int) << flag) as libc::c_uint;
}
/* *
 * Tests if current scene temp clear flag is set.
 */
#[no_mangle]
pub unsafe extern "C" fn Flags_GetTempClear(mut globalCtx: *mut GlobalContext,
                                            mut flag: s32) -> s32 {
    return ((*globalCtx).actorCtx.flags.tempClear &
                ((1 as libc::c_int) << flag) as libc::c_uint) as s32;
}
/* *
 * Sets current scene temp clear flag.
 */
#[no_mangle]
pub unsafe extern "C" fn Flags_SetTempClear(mut globalCtx: *mut GlobalContext,
                                            mut flag: s32) {
    (*globalCtx).actorCtx.flags.tempClear |=
        ((1 as libc::c_int) << flag) as libc::c_uint;
}
/* *
 * Unsets current scene temp clear flag.
 */
#[no_mangle]
pub unsafe extern "C" fn Flags_UnsetTempClear(mut globalCtx:
                                                  *mut GlobalContext,
                                              mut flag: s32) {
    (*globalCtx).actorCtx.flags.tempClear &=
        !((1 as libc::c_int) << flag) as libc::c_uint;
}
/* *
 * Tests if current scene collectible flag is set.
 */
#[no_mangle]
pub unsafe extern "C" fn Flags_GetCollectible(mut globalCtx:
                                                  *mut GlobalContext,
                                              mut flag: s32) -> s32 {
    if flag < 0x20 as libc::c_int {
        return ((*globalCtx).actorCtx.flags.collect &
                    ((1 as libc::c_int) << flag) as libc::c_uint) as s32
    } else {
        return ((*globalCtx).actorCtx.flags.tempCollect &
                    ((1 as libc::c_int) << flag - 0x20 as libc::c_int) as
                        libc::c_uint) as s32
    };
}
/* *
 * Sets current scene collectible flag.
 */
#[no_mangle]
pub unsafe extern "C" fn Flags_SetCollectible(mut globalCtx:
                                                  *mut GlobalContext,
                                              mut flag: s32) {
    if flag != 0 as libc::c_int {
        if flag < 0x20 as libc::c_int {
            (*globalCtx).actorCtx.flags.collect |=
                ((1 as libc::c_int) << flag) as libc::c_uint
        } else {
            (*globalCtx).actorCtx.flags.tempCollect |=
                ((1 as libc::c_int) << flag - 0x20 as libc::c_int) as
                    libc::c_uint
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_8002CDE4(mut globalCtx: *mut GlobalContext,
                                       mut titleCtx: *mut TitleCardContext) {
    (*titleCtx).alpha = 0 as libc::c_int as s16;
    (*titleCtx).intensity = (*titleCtx).alpha;
    (*titleCtx).delayTimer = (*titleCtx).intensity as u8_0;
    (*titleCtx).durationTimer = (*titleCtx).delayTimer;
}
#[no_mangle]
pub unsafe extern "C" fn TitleCard_InitBossName(mut globalCtx:
                                                    *mut GlobalContext,
                                                mut titleCtx:
                                                    *mut TitleCardContext,
                                                mut texture:
                                                    *mut libc::c_void,
                                                mut x: s16, mut y: s16,
                                                mut width: u8_0,
                                                mut height: u8_0) {
    (*titleCtx).texture = texture;
    (*titleCtx).x = x;
    (*titleCtx).y = y;
    (*titleCtx).width = width;
    (*titleCtx).height = height;
    (*titleCtx).durationTimer = 80 as libc::c_int as u8_0;
    (*titleCtx).delayTimer = 0 as libc::c_int as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn TitleCard_InitPlaceName(mut globalCtx:
                                                     *mut GlobalContext,
                                                 mut titleCtx:
                                                     *mut TitleCardContext,
                                                 mut texture:
                                                     *mut libc::c_void,
                                                 mut x: s32, mut y: s32,
                                                 mut width: s32,
                                                 mut height: s32,
                                                 mut delay: s32) {
    let mut loadedScene: *mut SceneTableEntry = (*globalCtx).loadedScene;
    let mut size: u32_0 =
        (*loadedScene).titleFile.vromEnd.wrapping_sub((*loadedScene).titleFile.vromStart);
    if size != 0 as libc::c_int as libc::c_uint &&
           size <= 0x3000 as libc::c_int as libc::c_uint {
        DmaMgr_SendRequest1(texture, (*loadedScene).titleFile.vromStart, size,
                            b"../z_actor.c\x00" as *const u8 as
                                *const libc::c_char, 2765 as libc::c_int);
    }
    (*titleCtx).texture = texture;
    (*titleCtx).x = x as s16;
    (*titleCtx).y = y as s16;
    (*titleCtx).width = width as u8_0;
    (*titleCtx).height = height as u8_0;
    (*titleCtx).durationTimer = 80 as libc::c_int as u8_0;
    (*titleCtx).delayTimer = delay as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn TitleCard_Update(mut globalCtx: *mut GlobalContext,
                                          mut titleCtx:
                                              *mut TitleCardContext) {
    if (if (*titleCtx).delayTimer as libc::c_int == 0 as libc::c_int {
            0 as libc::c_int
        } else {
            (*titleCtx).delayTimer = (*titleCtx).delayTimer.wrapping_sub(1);
            (*titleCtx).delayTimer as libc::c_int
        }) == 0 as libc::c_int {
        if (if (*titleCtx).durationTimer as libc::c_int == 0 as libc::c_int {
                0 as libc::c_int
            } else {
                (*titleCtx).durationTimer =
                    (*titleCtx).durationTimer.wrapping_sub(1);
                (*titleCtx).durationTimer as libc::c_int
            }) == 0 as libc::c_int {
            Math_StepToS(&mut (*titleCtx).alpha, 0 as libc::c_int as s16,
                         30 as libc::c_int as s16);
            Math_StepToS(&mut (*titleCtx).intensity, 0 as libc::c_int as s16,
                         70 as libc::c_int as s16);
        } else {
            Math_StepToS(&mut (*titleCtx).alpha, 255 as libc::c_int as s16,
                         10 as libc::c_int as s16);
            Math_StepToS(&mut (*titleCtx).intensity,
                         255 as libc::c_int as s16, 20 as libc::c_int as s16);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn TitleCard_Draw(mut globalCtx: *mut GlobalContext,
                                        mut titleCtx: *mut TitleCardContext) {
    let mut spCC: s32 = 0;
    let mut spC8: s32 = 0;
    let mut unk1: s32 = 0;
    let mut spC0: s32 = 0;
    let mut sp38: s32 = 0;
    let mut spB8: s32 = 0;
    let mut spB4: s32 = 0;
    let mut spB0: s32 = 0;
    if (*titleCtx).alpha as libc::c_int != 0 as libc::c_int {
        spCC = (*titleCtx).width as s32;
        spC8 = (*titleCtx).height as s32;
        spC0 =
            (*titleCtx).x as libc::c_int * 4 as libc::c_int -
                spCC * 2 as libc::c_int;
        spB8 =
            (*titleCtx).y as libc::c_int * 4 as libc::c_int -
                spC8 * 2 as libc::c_int;
        sp38 = spCC * 2 as libc::c_int;
        let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
        let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
        __gfxCtx = (*globalCtx).state.gfxCtx;
        Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                        b"../z_actor.c\x00" as *const u8 as
                            *const libc::c_char, 2824 as libc::c_int);
        spB0 = spCC * spC8 * gSaveContext.language as libc::c_int;
        spC8 =
            if spCC * spC8 > 0x1000 as libc::c_int {
                (0x1000 as libc::c_int) / spCC
            } else { spC8 };
        spB4 = spB8 + spC8 * 4 as libc::c_int;
        (*__gfxCtx).overlay.p = func_80093808((*__gfxCtx).overlay.p);
        let fresh14 = (*__gfxCtx).overlay.p;
        (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
        let mut _g: *mut Gfx = fresh14;
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
            ((*titleCtx).intensity as u8_0 as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((*titleCtx).intensity as u8_0 as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                ((*titleCtx).intensity as u8_0 as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                ((*titleCtx).alpha as u8_0 as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh15 = (*__gfxCtx).overlay.p;
        (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
        let mut _g_0: *mut Gfx = fresh15;
        (*_g_0).words.w0 =
            (0xfd as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (3 as libc::c_int as u32_0 &
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
        (*_g_0).words.w1 =
            ((*titleCtx).texture as s32 + spB0) as libc::c_uint;
        let fresh16 = (*__gfxCtx).overlay.p;
        (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
        let mut _g_1: *mut Gfx = fresh16;
        (*_g_1).words.w0 =
            (0xf5 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (3 as libc::c_int as u32_0 &
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
        (*_g_1).words.w1 =
            (7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    20 as libc::c_int |
                ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    18 as libc::c_int |
                (0 as libc::c_int as u32_0 &
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
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    4 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh17 = (*__gfxCtx).overlay.p;
        (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
        let mut _g_2: *mut Gfx = fresh17;
        (*_g_2).words.w0 =
            (0xe6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_2).words.w1 = 0 as libc::c_int as libc::c_uint;
        let fresh18 = (*__gfxCtx).overlay.p;
        (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
        let mut _g_3: *mut Gfx = fresh18;
        (*_g_3).words.w0 =
            (0xf3 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_3).words.w1 =
            (7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((if ((spCC * spC8 + 1 as libc::c_int >> 1 as libc::c_int) -
                          1 as libc::c_int) < 2047 as libc::c_int {
                      (spCC * spC8 + 1 as libc::c_int >> 1 as libc::c_int) -
                          1 as libc::c_int
                  } else { 2047 as libc::c_int }) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                (((((1 as libc::c_int) << 11 as libc::c_int) +
                       (if 1 as libc::c_int >
                               spCC * 1 as libc::c_int / 8 as libc::c_int {
                            1 as libc::c_int
                        } else {
                            (spCC * 1 as libc::c_int) / 8 as libc::c_int
                        }) - 1 as libc::c_int) /
                      (if 1 as libc::c_int >
                              spCC * 1 as libc::c_int / 8 as libc::c_int {
                           1 as libc::c_int
                       } else {
                           (spCC * 1 as libc::c_int) / 8 as libc::c_int
                       })) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh19 = (*__gfxCtx).overlay.p;
        (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
        let mut _g_4: *mut Gfx = fresh19;
        (*_g_4).words.w0 =
            (0xe7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_4).words.w1 = 0 as libc::c_int as libc::c_uint;
        let fresh20 = (*__gfxCtx).overlay.p;
        (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
        let mut _g_5: *mut Gfx = fresh20;
        (*_g_5).words.w0 =
            (0xf5 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (3 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    21 as libc::c_int |
                (1 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    19 as libc::c_int |
                ((spCC * 1 as libc::c_int + 7 as libc::c_int >>
                      3 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 9 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    9 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 9 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_5).words.w1 =
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    20 as libc::c_int |
                ((0 as libc::c_int | 0 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 2 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    18 as libc::c_int |
                (0 as libc::c_int as u32_0 &
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
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    4 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 4 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh21 = (*__gfxCtx).overlay.p;
        (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
        let mut _g_6: *mut Gfx = fresh21;
        (*_g_6).words.w0 =
            (0xf2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_6).words.w1 =
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (((spCC - 1 as libc::c_int) << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                (((spC8 - 1 as libc::c_int) << 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh22 = (*__gfxCtx).overlay.p;
        (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
        let mut _g_7: *mut Gfx = fresh22;
        (*_g_7).words.w0 =
            (0xe4 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((sp38 * 2 as libc::c_int + spC0 - 4 as libc::c_int) as u32_0
                     &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                ((spB8 + spC8 * 4 as libc::c_int - 1 as libc::c_int) as u32_0
                     &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_7).words.w1 =
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (spC0 as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                (spB8 as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh23 = (*__gfxCtx).overlay.p;
        (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
        let mut _g_8: *mut Gfx = fresh23;
        (*_g_8).words.w0 =
            (0xe1 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_8).words.w1 =
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh24 = (*__gfxCtx).overlay.p;
        (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
        let mut _g_9: *mut Gfx = fresh24;
        (*_g_9).words.w0 =
            (0xf1 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_9).words.w1 =
            (((1 as libc::c_int) << 10 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
                |
                (((1 as libc::c_int) << 10 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        spC8 = (*titleCtx).height as libc::c_int - spC8;
        if spC8 > 0 as libc::c_int {
            let fresh25 = (*__gfxCtx).overlay.p;
            (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
            let mut _g_10: *mut Gfx = fresh25;
            (*_g_10).words.w0 =
                (0xfd as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (3 as libc::c_int as u32_0 &
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
            (*_g_10).words.w1 =
                ((*titleCtx).texture as s32 + spB0 + 0x1000 as libc::c_int) as
                    libc::c_uint;
            let fresh26 = (*__gfxCtx).overlay.p;
            (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
            let mut _g_11: *mut Gfx = fresh26;
            (*_g_11).words.w0 =
                (0xf5 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (3 as libc::c_int as u32_0 &
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
            (*_g_11).words.w1 =
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
                    (0 as libc::c_int as u32_0 &
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
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        4 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh27 = (*__gfxCtx).overlay.p;
            (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
            let mut _g_12: *mut Gfx = fresh27;
            (*_g_12).words.w0 =
                (0xe6 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_12).words.w1 = 0 as libc::c_int as libc::c_uint;
            let fresh28 = (*__gfxCtx).overlay.p;
            (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
            let mut _g_13: *mut Gfx = fresh28;
            (*_g_13).words.w0 =
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
            (*_g_13).words.w1 =
                (7 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((if ((spCC * spC8 + 1 as libc::c_int >> 1 as libc::c_int)
                              - 1 as libc::c_int) < 2047 as libc::c_int {
                          (spCC * spC8 + 1 as libc::c_int >> 1 as libc::c_int)
                              - 1 as libc::c_int
                      } else { 2047 as libc::c_int }) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        12 as libc::c_int |
                    (((((1 as libc::c_int) << 11 as libc::c_int) +
                           (if 1 as libc::c_int >
                                   spCC * 1 as libc::c_int / 8 as libc::c_int
                               {
                                1 as libc::c_int
                            } else {
                                (spCC * 1 as libc::c_int) / 8 as libc::c_int
                            }) - 1 as libc::c_int) /
                          (if 1 as libc::c_int >
                                  spCC * 1 as libc::c_int / 8 as libc::c_int {
                               1 as libc::c_int
                           } else {
                               (spCC * 1 as libc::c_int) / 8 as libc::c_int
                           })) as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh29 = (*__gfxCtx).overlay.p;
            (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
            let mut _g_14: *mut Gfx = fresh29;
            (*_g_14).words.w0 =
                (0xe7 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_14).words.w1 = 0 as libc::c_int as libc::c_uint;
            let fresh30 = (*__gfxCtx).overlay.p;
            (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
            let mut _g_15: *mut Gfx = fresh30;
            (*_g_15).words.w0 =
                (0xf5 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (3 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 3 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        21 as libc::c_int |
                    (1 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 2 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        19 as libc::c_int |
                    ((spCC * 1 as libc::c_int + 7 as libc::c_int >>
                          3 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 9 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        9 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 9 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_15).words.w1 =
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
                    (0 as libc::c_int as u32_0 &
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
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        4 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 4 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh31 = (*__gfxCtx).overlay.p;
            (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
            let mut _g_16: *mut Gfx = fresh31;
            (*_g_16).words.w0 =
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
            (*_g_16).words.w1 =
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (((spCC - 1 as libc::c_int) << 2 as libc::c_int) as u32_0
                         &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        12 as libc::c_int |
                    (((spC8 - 1 as libc::c_int) << 2 as libc::c_int) as u32_0
                         &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh32 = (*__gfxCtx).overlay.p;
            (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
            let mut _g_17: *mut Gfx = fresh32;
            (*_g_17).words.w0 =
                (0xe4 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((sp38 * 2 as libc::c_int + spC0 - 4 as libc::c_int) as
                         u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        12 as libc::c_int |
                    ((spB4 + spC8 * 4 as libc::c_int - 1 as libc::c_int) as
                         u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_17).words.w1 =
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (spC0 as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        12 as libc::c_int |
                    (spB4 as u32_0 &
                         (((0x1 as libc::c_int) << 12 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh33 = (*__gfxCtx).overlay.p;
            (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
            let mut _g_18: *mut Gfx = fresh33;
            (*_g_18).words.w0 =
                (0xe1 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_18).words.w1 =
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh34 = (*__gfxCtx).overlay.p;
            (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
            let mut _g_19: *mut Gfx = fresh34;
            (*_g_19).words.w0 =
                (0xf1 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_19).words.w1 =
                (((1 as libc::c_int) << 10 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                    (((1 as libc::c_int) << 10 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int
        }
        Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                         b"../z_actor.c\x00" as *const u8 as
                             *const libc::c_char, 2880 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_8002D53C(mut globalCtx: *mut GlobalContext,
                                       mut titleCtx: *mut TitleCardContext)
 -> s32 {
    if (*globalCtx).actorCtx.titleCtx.delayTimer as libc::c_int !=
           0 as libc::c_int ||
           (*globalCtx).actorCtx.titleCtx.alpha as libc::c_int !=
               0 as libc::c_int {
        (*titleCtx).durationTimer = 0 as libc::c_int as u8_0;
        (*titleCtx).delayTimer = 0 as libc::c_int as u8_0;
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Actor_Kill(mut actor: *mut Actor) {
    (*actor).draw = None;
    (*actor).update = None;
    (*actor).flags &=
        !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn Actor_SetWorldToHome(mut actor: *mut Actor) {
    (*actor).world = (*actor).home;
}
#[no_mangle]
pub unsafe extern "C" fn Actor_SetFocus(mut actor: *mut Actor,
                                        mut yOffset: f32_0) {
    (*actor).focus.pos.x = (*actor).world.pos.x;
    (*actor).focus.pos.y = (*actor).world.pos.y + yOffset;
    (*actor).focus.pos.z = (*actor).world.pos.z;
    (*actor).focus.rot.x = (*actor).world.rot.x;
    (*actor).focus.rot.y = (*actor).world.rot.y;
    (*actor).focus.rot.z = (*actor).world.rot.z;
}
#[no_mangle]
pub unsafe extern "C" fn Actor_SetWorldRotToShape(mut actor: *mut Actor) {
    (*actor).world.rot = (*actor).shape.rot;
}
#[no_mangle]
pub unsafe extern "C" fn Actor_SetShapeRotToWorld(mut actor: *mut Actor) {
    (*actor).shape.rot = (*actor).world.rot;
}
#[no_mangle]
pub unsafe extern "C" fn Actor_SetScale(mut actor: *mut Actor,
                                        mut scale: f32_0) {
    (*actor).scale.z = scale;
    (*actor).scale.y = scale;
    (*actor).scale.x = scale;
}
#[no_mangle]
pub unsafe extern "C" fn Actor_SetObjectDependency(mut globalCtx:
                                                       *mut GlobalContext,
                                                   mut actor: *mut Actor) {
    gSegments[6 as libc::c_int as usize] =
        ((*globalCtx).objectCtx.status[(*actor).objBankIndex as usize].segment
             as *mut u8_0).offset(-(0x80000000 as libc::c_uint as isize)) as
            u32_0;
}
#[no_mangle]
pub unsafe extern "C" fn Actor_Init(mut actor: *mut Actor,
                                    mut globalCtx: *mut GlobalContext) {
    Actor_SetWorldToHome(actor);
    Actor_SetShapeRotToWorld(actor);
    Actor_SetFocus(actor, 0.0f32);
    Math_Vec3f_Copy(&mut (*actor).prevPos, &mut (*actor).world.pos);
    Actor_SetScale(actor, 0.01f32);
    (*actor).targetMode = 3 as libc::c_int as s8;
    (*actor).minVelocityY = -20.0f32;
    (*actor).xyzDistToPlayerSq = 340282346638528859811704183484516925440.0f32;
    (*actor).naviEnemyId = 0xff as libc::c_int as u8_0;
    (*actor).uncullZoneForward = 1000.0f32;
    (*actor).uncullZoneScale = 350.0f32;
    (*actor).uncullZoneDownward = 700.0f32;
    CollisionCheck_InitInfo(&mut (*actor).colChkInfo);
    (*actor).floorBgId = 50 as libc::c_int as u8_0;
    ActorShape_Init(&mut (*actor).shape, 0.0f32, None, 0.0f32);
    if Object_IsLoaded(&mut (*globalCtx).objectCtx,
                       (*actor).objBankIndex as s32) != 0 {
        Actor_SetObjectDependency(globalCtx, actor);
        (*actor).init.expect("non-null function pointer")(actor, globalCtx);
        (*actor).init = None
    };
}
#[no_mangle]
pub unsafe extern "C" fn Actor_Destroy(mut actor: *mut Actor,
                                       mut globalCtx: *mut GlobalContext) {
    let mut overlayEntry: *mut ActorOverlay = 0 as *mut ActorOverlay;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*actor).destroy.is_some() {
        (*actor).destroy.expect("non-null function pointer")(actor,
                                                             globalCtx);
        (*actor).destroy = None
    } else {
        overlayEntry = (*actor).overlayEntry;
        name =
            if !(*overlayEntry).name.is_null() {
                (*overlayEntry).name as *const libc::c_char
            } else { b"\x00" as *const u8 as *const libc::c_char } as
                *mut libc::c_char;
        // "No Actor class destruct [%s]"
        osSyncPrintf(b"\xef\xbc\xa1\xef\xbd\x83\xef\xbd\x94\xef\xbd\x8f\xef\xbd\x92\xe3\x82\xaf\xe3\x83\xa9\xe3\x82\xb9 \xe3\x83\x87\xe3\x82\xb9\xe3\x83\x88\xe3\x83\xa9\xe3\x82\xaf\xe3\x83\x88\xe3\x81\x8c\xe3\x81\x82\xe3\x82\x8a\xe3\x81\xbe\xe3\x81\x9b\xe3\x82\x93 [%s]\n\x1b[m\x00"
                         as *const u8 as *const libc::c_char, name);
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_8002D7EC(mut actor: *mut Actor) {
    let mut speedRate: f32_0 =
        (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 30 as libc::c_int) as
                              usize] as libc::c_int as libc::c_float * 0.5f32;
    (*actor).world.pos.x +=
        (*actor).velocity.x * speedRate + (*actor).colChkInfo.displacement.x;
    (*actor).world.pos.y +=
        (*actor).velocity.y * speedRate + (*actor).colChkInfo.displacement.y;
    (*actor).world.pos.z +=
        (*actor).velocity.z * speedRate + (*actor).colChkInfo.displacement.z;
}
#[no_mangle]
pub unsafe extern "C" fn func_8002D868(mut actor: *mut Actor) {
    (*actor).velocity.x = Math_SinS((*actor).world.rot.y) * (*actor).speedXZ;
    (*actor).velocity.z = Math_CosS((*actor).world.rot.y) * (*actor).speedXZ;
    (*actor).velocity.y += (*actor).gravity;
    if (*actor).velocity.y < (*actor).minVelocityY {
        (*actor).velocity.y = (*actor).minVelocityY
    };
}
#[no_mangle]
pub unsafe extern "C" fn Actor_MoveForward(mut actor: *mut Actor) {
    func_8002D868(actor);
    func_8002D7EC(actor);
}
#[no_mangle]
pub unsafe extern "C" fn func_8002D908(mut actor: *mut Actor) {
    let mut sp24: f32_0 = Math_CosS((*actor).world.rot.x) * (*actor).speedXZ;
    (*actor).velocity.x = Math_SinS((*actor).world.rot.y) * sp24;
    (*actor).velocity.y = Math_SinS((*actor).world.rot.x) * (*actor).speedXZ;
    (*actor).velocity.z = Math_CosS((*actor).world.rot.y) * sp24;
}
#[no_mangle]
pub unsafe extern "C" fn func_8002D97C(mut actor: *mut Actor) {
    func_8002D908(actor);
    func_8002D7EC(actor);
}
#[no_mangle]
pub unsafe extern "C" fn func_8002D9A4(mut actor: *mut Actor,
                                       mut arg1: f32_0) {
    (*actor).speedXZ = Math_CosS((*actor).world.rot.x) * arg1;
    (*actor).velocity.y = -Math_SinS((*actor).world.rot.x) * arg1;
}
#[no_mangle]
pub unsafe extern "C" fn func_8002D9F8(mut actor: *mut Actor,
                                       mut skelAnime: *mut SkelAnime) {
    let mut sp1C: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    SkelAnime_UpdateTranslation(skelAnime, &mut sp1C, (*actor).shape.rot.y);
    (*actor).world.pos.x += sp1C.x * (*actor).scale.x;
    (*actor).world.pos.y += sp1C.y * (*actor).scale.y;
    (*actor).world.pos.z += sp1C.z * (*actor).scale.z;
}
#[no_mangle]
pub unsafe extern "C" fn Actor_WorldYawTowardActor(mut actorA: *mut Actor,
                                                   mut actorB: *mut Actor)
 -> s16 {
    return Math_Vec3f_Yaw(&mut (*actorA).world.pos, &mut (*actorB).world.pos);
}
#[no_mangle]
pub unsafe extern "C" fn Actor_FocusYawTowardActor(mut actorA: *mut Actor,
                                                   mut actorB: *mut Actor)
 -> s16 {
    return Math_Vec3f_Yaw(&mut (*actorA).focus.pos, &mut (*actorB).focus.pos);
}
#[no_mangle]
pub unsafe extern "C" fn Actor_WorldYawTowardPoint(mut actor: *mut Actor,
                                                   mut refPoint: *mut Vec3f)
 -> s16 {
    return Math_Vec3f_Yaw(&mut (*actor).world.pos, refPoint);
}
#[no_mangle]
pub unsafe extern "C" fn Actor_WorldPitchTowardActor(mut actorA: *mut Actor,
                                                     mut actorB: *mut Actor)
 -> s16 {
    return Math_Vec3f_Pitch(&mut (*actorA).world.pos,
                            &mut (*actorB).world.pos);
}
#[no_mangle]
pub unsafe extern "C" fn Actor_FocusPitchTowardActor(mut actorA: *mut Actor,
                                                     mut actorB: *mut Actor)
 -> s16 {
    return Math_Vec3f_Pitch(&mut (*actorA).focus.pos,
                            &mut (*actorB).focus.pos);
}
#[no_mangle]
pub unsafe extern "C" fn Actor_WorldPitchTowardPoint(mut actor: *mut Actor,
                                                     mut refPoint: *mut Vec3f)
 -> s16 {
    return Math_Vec3f_Pitch(&mut (*actor).world.pos, refPoint);
}
#[no_mangle]
pub unsafe extern "C" fn Actor_WorldDistXYZToActor(mut actorA: *mut Actor,
                                                   mut actorB: *mut Actor)
 -> f32_0 {
    return Math_Vec3f_DistXYZ(&mut (*actorA).world.pos,
                              &mut (*actorB).world.pos);
}
#[no_mangle]
pub unsafe extern "C" fn Actor_WorldDistXYZToPoint(mut actor: *mut Actor,
                                                   mut refPoint: *mut Vec3f)
 -> f32_0 {
    return Math_Vec3f_DistXYZ(&mut (*actor).world.pos, refPoint);
}
#[no_mangle]
pub unsafe extern "C" fn Actor_WorldDistXZToActor(mut actorA: *mut Actor,
                                                  mut actorB: *mut Actor)
 -> f32_0 {
    return Math_Vec3f_DistXZ(&mut (*actorA).world.pos,
                             &mut (*actorB).world.pos);
}
#[no_mangle]
pub unsafe extern "C" fn Actor_WorldDistXZToPoint(mut actor: *mut Actor,
                                                  mut refPoint: *mut Vec3f)
 -> f32_0 {
    return Math_Vec3f_DistXZ(&mut (*actor).world.pos, refPoint);
}
#[no_mangle]
pub unsafe extern "C" fn func_8002DBD0(mut actor: *mut Actor,
                                       mut result: *mut Vec3f,
                                       mut arg2: *mut Vec3f) {
    let mut cosRot2Y: f32_0 = 0.;
    let mut sinRot2Y: f32_0 = 0.;
    let mut deltaX: f32_0 = 0.;
    let mut deltaZ: f32_0 = 0.;
    cosRot2Y = Math_CosS((*actor).shape.rot.y);
    sinRot2Y = Math_SinS((*actor).shape.rot.y);
    deltaX = (*arg2).x - (*actor).world.pos.x;
    deltaZ = (*arg2).z - (*actor).world.pos.z;
    (*result).x = deltaX * cosRot2Y - deltaZ * sinRot2Y;
    (*result).z = deltaX * sinRot2Y + deltaZ * cosRot2Y;
    (*result).y = (*arg2).y - (*actor).world.pos.y;
}
#[no_mangle]
pub unsafe extern "C" fn Actor_HeightDiff(mut actorA: *mut Actor,
                                          mut actorB: *mut Actor) -> f32_0 {
    return (*actorB).world.pos.y - (*actorA).world.pos.y;
}
#[no_mangle]
pub unsafe extern "C" fn Player_GetHeight(mut player: *mut Player) -> f32_0 {
    let mut offset: f32_0 =
        if (*player).stateFlags1 & 0x800000 as libc::c_int as libc::c_uint !=
               0 {
            32.0f32
        } else { 0.0f32 };
    if gSaveContext.linkAge == 0 as libc::c_int {
        return offset + 68.0f32
    } else { return offset + 44.0f32 };
}
#[no_mangle]
pub unsafe extern "C" fn func_8002DCE4(mut player: *mut Player) -> f32_0 {
    if (*player).stateFlags1 & 0x800000 as libc::c_int as libc::c_uint != 0 {
        return 8.0f32
    } else if (*player).stateFlags1 & 0x8000000 as libc::c_int as libc::c_uint
                  != 0 {
        return (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int *
                                      16 as libc::c_int + 45 as libc::c_int)
                                     as usize] as libc::c_int as libc::c_float
                   / 100.0f32 * 0.6f32
    } else {
        return (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int *
                                      16 as libc::c_int + 45 as libc::c_int)
                                     as usize] as libc::c_int as libc::c_float
                   / 100.0f32
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_8002DD6C(mut player: *mut Player) -> s32 {
    return ((*player).stateFlags1 & 0x8 as libc::c_int as libc::c_uint) as
               s32;
}
#[no_mangle]
pub unsafe extern "C" fn func_8002DD78(mut player: *mut Player) -> s32 {
    return (func_8002DD6C(player) != 0 &&
                (*player).unk_834 as libc::c_int != 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_8002DDA8(mut globalCtx: *mut GlobalContext)
 -> s32 {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    return ((*player).stateFlags1 & 0x800 as libc::c_int as libc::c_uint != 0
                || func_8002DD78(player) != 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_8002DDE4(mut globalCtx: *mut GlobalContext)
 -> s32 {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    return ((*player).stateFlags2 & 0x8 as libc::c_int as libc::c_uint) as
               s32;
}
#[no_mangle]
pub unsafe extern "C" fn func_8002DDF4(mut globalCtx: *mut GlobalContext)
 -> s32 {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    return ((*player).stateFlags2 & 0x1000 as libc::c_int as libc::c_uint) as
               s32;
}
#[no_mangle]
pub unsafe extern "C" fn func_8002DE04(mut globalCtx: *mut GlobalContext,
                                       mut actorA: *mut Actor,
                                       mut actorB: *mut Actor) {
    let mut hookshot: *mut ArmsHook =
        Actor_Find(&mut (*globalCtx).actorCtx, ACTOR_ARMS_HOOK as libc::c_int,
                   ACTORCAT_ITEMACTION as libc::c_int) as *mut ArmsHook;
    (*hookshot).grabbed = actorB;
    (*hookshot).grabbedDistDiff.x = 0.0f32;
    (*hookshot).grabbedDistDiff.y = 0.0f32;
    (*hookshot).grabbedDistDiff.z = 0.0f32;
    (*actorB).flags |=
        ((1 as libc::c_int) << 13 as libc::c_int) as libc::c_uint;
    (*actorA).flags &=
        !((1 as libc::c_int) << 13 as libc::c_int) as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn func_8002DE74(mut globalCtx: *mut GlobalContext,
                                       mut player: *mut Player) {
    if (*globalCtx).roomCtx.curRoom.unk_03 as libc::c_int != 4 as libc::c_int
           && func_800C0CB8(globalCtx) != 0 {
        Camera_ChangeSetting(Gameplay_GetCamera(globalCtx,
                                                0 as libc::c_int as s16),
                             CAM_SET_HORSE as libc::c_int as s16);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Actor_MountHorse(mut globalCtx: *mut GlobalContext,
                                          mut player: *mut Player,
                                          mut horse: *mut Actor) {
    (*player).rideActor = horse;
    (*player).stateFlags1 |= 0x800000 as libc::c_int as libc::c_uint;
    (*horse).child = &mut (*player).actor;
}
#[no_mangle]
pub unsafe extern "C" fn func_8002DEEC(mut player: *mut Player) -> s32 {
    return ((*player).stateFlags1 & 0x20000080 as libc::c_int as libc::c_uint
                != 0 || (*player).csMode as libc::c_int != 0 as libc::c_int)
               as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_8002DF18(mut globalCtx: *mut GlobalContext,
                                       mut player: *mut Player) {
    func_8006DC68(globalCtx, player);
}
#[no_mangle]
pub unsafe extern "C" fn func_8002DF38(mut globalCtx: *mut GlobalContext,
                                       mut actor: *mut Actor,
                                       mut csMode: u8_0) -> s32 {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    (*player).csMode = csMode;
    (*player).unk_448 = actor;
    (*player).unk_46A = 0 as libc::c_int as s16;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_8002DF54(mut globalCtx: *mut GlobalContext,
                                       mut actor: *mut Actor,
                                       mut csMode: u8_0) -> s32 {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    func_8002DF38(globalCtx, actor, csMode);
    (*player).unk_46A = 1 as libc::c_int as s16;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_8002DF90(mut dynaActor: *mut DynaPolyActor) {
    (*dynaActor).unk_154 = 0.0f32;
    (*dynaActor).unk_150 = 0.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn func_8002DFA4(mut dynaActor: *mut DynaPolyActor,
                                       mut arg1: f32_0, mut arg2: s16) {
    (*dynaActor).unk_150 += arg1;
    (*dynaActor).unk_158 = arg2;
}
/* *
 * Chcek if the player is facing the specified actor.
 * The maximum angle difference that qualifies as "facing" is specified by `maxAngle`.
 */
#[no_mangle]
pub unsafe extern "C" fn Player_IsFacingActor(mut actor: *mut Actor,
                                              mut maxAngle: s16,
                                              mut globalCtx:
                                                  *mut GlobalContext) -> s32 {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut yawDiff: s16 =
        (((*actor).yawTowardsPlayer as libc::c_int + 0x8000 as libc::c_int) as
             s16 as libc::c_int - (*player).actor.shape.rot.y as libc::c_int)
            as s16;
    if (if yawDiff as libc::c_int >= 0 as libc::c_int {
            yawDiff as libc::c_int
        } else { -(yawDiff as libc::c_int) }) < maxAngle as libc::c_int {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/* *
 * Chcek if `actorB` is facing `actorA`.
 * The maximum angle difference that qualifies as "facing" is specified by `maxAngle`.
 *
 * This function is unused in the original game.
 */
#[no_mangle]
pub unsafe extern "C" fn Actor_ActorBIsFacingActorA(mut actorA: *mut Actor,
                                                    mut actorB: *mut Actor,
                                                    mut maxAngle: s16)
 -> s32 {
    let mut yawDiff: s16 =
        ((Actor_WorldYawTowardActor(actorA, actorB) as libc::c_int +
              0x8000 as libc::c_int) as s16 as libc::c_int -
             (*actorB).shape.rot.y as libc::c_int) as s16;
    if (if yawDiff as libc::c_int >= 0 as libc::c_int {
            yawDiff as libc::c_int
        } else { -(yawDiff as libc::c_int) }) < maxAngle as libc::c_int {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/* *
 * Chcek if the specified actor is facing the player.
 * The maximum angle difference that qualifies as "facing" is specified by `maxAngle`.
 */
#[no_mangle]
pub unsafe extern "C" fn Actor_IsFacingPlayer(mut actor: *mut Actor,
                                              mut maxAngle: s16) -> s32 {
    let mut yawDiff: s16 =
        ((*actor).yawTowardsPlayer as libc::c_int -
             (*actor).shape.rot.y as libc::c_int) as s16;
    if (if yawDiff as libc::c_int >= 0 as libc::c_int {
            yawDiff as libc::c_int
        } else { -(yawDiff as libc::c_int) }) < maxAngle as libc::c_int {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/* *
 * Chcek if `actorA` is facing `actorB`.
 * The maximum angle difference that qualifies as "facing" is specified by `maxAngle`.
 *
 * This function is unused in the original game.
 */
#[no_mangle]
pub unsafe extern "C" fn Actor_ActorAIsFacingActorB(mut actorA: *mut Actor,
                                                    mut actorB: *mut Actor,
                                                    mut maxAngle: s16)
 -> s32 {
    let mut yawDiff: s16 =
        (Actor_WorldYawTowardActor(actorA, actorB) as libc::c_int -
             (*actorA).shape.rot.y as libc::c_int) as s16;
    if (if yawDiff as libc::c_int >= 0 as libc::c_int {
            yawDiff as libc::c_int
        } else { -(yawDiff as libc::c_int) }) < maxAngle as libc::c_int {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/* *
 * Chcek if the specified actor is facing the player and is nearby.
 * The maximum angle difference that qualifies as "facing" is specified by `maxAngle`.
 * The minimum distance that qualifies as "nearby" is specified by `range`.
 */
#[no_mangle]
pub unsafe extern "C" fn Actor_IsFacingAndNearPlayer(mut actor: *mut Actor,
                                                     mut range: f32_0,
                                                     mut maxAngle: s16)
 -> s32 {
    let mut yawDiff: s16 =
        ((*actor).yawTowardsPlayer as libc::c_int -
             (*actor).shape.rot.y as libc::c_int) as s16;
    if (if yawDiff as libc::c_int >= 0 as libc::c_int {
            yawDiff as libc::c_int
        } else { -(yawDiff as libc::c_int) }) < maxAngle as libc::c_int {
        let mut xyzDistanceFromLink: f32_0 =
            sqrtf((*actor).xzDistToPlayer * (*actor).xzDistToPlayer +
                      (*actor).yDistToPlayer * (*actor).yDistToPlayer);
        if xyzDistanceFromLink < range { return 1 as libc::c_int }
    }
    return 0 as libc::c_int;
}
/* *
 * Chcek if `actorA` is facing `actorB` and is nearby.
 * The maximum angle difference that qualifies as "facing" is specified by `maxAngle`.
 * The minimum distance that qualifies as "nearby" is specified by `range`.
 */
#[no_mangle]
pub unsafe extern "C" fn Actor_ActorAIsFacingAndNearActorB(mut actorA:
                                                               *mut Actor,
                                                           mut actorB:
                                                               *mut Actor,
                                                           mut range: f32_0,
                                                           mut maxAngle: s16)
 -> s32 {
    if Actor_WorldDistXYZToActor(actorA, actorB) < range {
        let mut yawDiff: s16 =
            (Actor_WorldYawTowardActor(actorA, actorB) as libc::c_int -
                 (*actorA).shape.rot.y as libc::c_int) as
                s16; // actor is above ground
        if (if yawDiff as libc::c_int >= 0 as libc::c_int {
                yawDiff as libc::c_int
            } else { -(yawDiff as libc::c_int) }) < maxAngle as libc::c_int {
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_8002E234(mut actor: *mut Actor, mut arg1: f32_0,
                                       mut arg2: s32) -> s32 {
    if (*actor).bgCheckFlags as libc::c_int & 0x1 as libc::c_int != 0 &&
           arg1 < -11.0f32 {
        (*actor).bgCheckFlags =
            ((*actor).bgCheckFlags as libc::c_int & !(0x1 as libc::c_int)) as
                u16_0;
        (*actor).bgCheckFlags =
            ((*actor).bgCheckFlags as libc::c_int | 0x4 as libc::c_int) as
                u16_0;
        if (*actor).velocity.y < 0.0f32 && arg2 & 0x10 as libc::c_int != 0 {
            (*actor).velocity.y = 0.0f32
        }
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_8002E2AC(mut globalCtx: *mut GlobalContext,
                                       mut actor: *mut Actor,
                                       mut arg2: *mut Vec3f, mut arg3: s32)
 -> s32 {
    let mut floorHeightDiff: f32_0 = 0.;
    let mut floorBgId: s32 = 0;
    (*arg2).y += 50.0f32;
    (*actor).floorHeight =
        BgCheck_EntityRaycastFloor5(globalCtx, &mut (*globalCtx).colCtx,
                                    &mut (*actor).floorPoly, &mut floorBgId,
                                    actor, arg2);
    (*actor).bgCheckFlags =
        ((*actor).bgCheckFlags as libc::c_int & !(0x86 as libc::c_int)) as
            u16_0;
    if (*actor).floorHeight <= -32000.0f32 {
        return func_8002E234(actor, -32000.0f32, arg3)
    }
    floorHeightDiff = (*actor).floorHeight - (*actor).world.pos.y;
    (*actor).floorBgId = floorBgId as u8_0;
    if floorHeightDiff >= 0.0f32 {
        // actor is on or below the ground
        (*actor).bgCheckFlags =
            ((*actor).bgCheckFlags as libc::c_int | 0x80 as libc::c_int) as
                u16_0;
        if (*actor).bgCheckFlags as libc::c_int & 0x10 as libc::c_int != 0 {
            if floorBgId != sCurCeilingBgId {
                if floorHeightDiff > 15.0f32 {
                    (*actor).bgCheckFlags =
                        ((*actor).bgCheckFlags as libc::c_int |
                             0x100 as libc::c_int) as u16_0
                }
            } else {
                (*actor).world.pos.x = (*actor).prevPos.x;
                (*actor).world.pos.z = (*actor).prevPos.z
            }
        }
        (*actor).world.pos.y = (*actor).floorHeight;
        if (*actor).velocity.y <= 0.0f32 {
            if (*actor).bgCheckFlags as libc::c_int & 0x1 as libc::c_int == 0
               {
                (*actor).bgCheckFlags =
                    ((*actor).bgCheckFlags as libc::c_int |
                         0x2 as libc::c_int) as u16_0
            } else if arg3 & 0x8 as libc::c_int != 0 &&
                          (*actor).gravity < 0.0f32 {
                (*actor).velocity.y = -4.0f32
            } else { (*actor).velocity.y = 0.0f32 }
            (*actor).bgCheckFlags =
                ((*actor).bgCheckFlags as libc::c_int | 0x1 as libc::c_int) as
                    u16_0;
            func_80043334(&mut (*globalCtx).colCtx, actor,
                          (*actor).floorBgId as s32);
        }
    } else {
        if (*actor).bgCheckFlags as libc::c_int & 0x1 as libc::c_int != 0 &&
               floorHeightDiff >= -11.0f32 {
            func_80043334(&mut (*globalCtx).colCtx, actor,
                          (*actor).floorBgId as s32);
        }
        return func_8002E234(actor, floorHeightDiff, arg3)
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Actor_UpdateBgCheckInfo(mut globalCtx:
                                                     *mut GlobalContext,
                                                 mut actor: *mut Actor,
                                                 mut wallCheckHeight: f32_0,
                                                 mut wallCheckRadius: f32_0,
                                                 mut ceilingCheckHeight:
                                                     f32_0, mut flags: s32) {
    let mut sp74: f32_0 = 0.;
    let mut pad: s32 = 0;
    let mut sp64: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut bgId: s32 = 0;
    let mut wallPoly: *mut CollisionPoly = 0 as *mut CollisionPoly;
    let mut sp58: f32_0 = 0.;
    let mut waterBox: *mut WaterBox = 0 as *mut WaterBox;
    let mut waterBoxYSurface: f32_0 = 0.;
    let mut ripplePos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    sp74 = (*actor).world.pos.y - (*actor).prevPos.y;
    if (*actor).floorBgId as libc::c_int != 50 as libc::c_int &&
           (*actor).bgCheckFlags as libc::c_int & 1 as libc::c_int != 0 {
        func_800433A4(&mut (*globalCtx).colCtx, (*actor).floorBgId as s32,
                      actor);
    }
    if flags & 1 as libc::c_int != 0 {
        if flags & 0x80 as libc::c_int == 0 &&
               BgCheck_EntitySphVsWall3(&mut (*globalCtx).colCtx, &mut sp64,
                                        &mut (*actor).world.pos,
                                        &mut (*actor).prevPos,
                                        wallCheckRadius,
                                        &mut (*actor).wallPoly, &mut bgId,
                                        actor, wallCheckHeight) != 0 ||
               flags & 0x80 as libc::c_int != 0 &&
                   BgCheck_EntitySphVsWall4(&mut (*globalCtx).colCtx,
                                            &mut sp64,
                                            &mut (*actor).world.pos,
                                            &mut (*actor).prevPos,
                                            wallCheckRadius,
                                            &mut (*actor).wallPoly, &mut bgId,
                                            actor, wallCheckHeight) != 0 {
            wallPoly = (*actor).wallPoly;
            Math_Vec3f_Copy(&mut (*actor).world.pos, &mut sp64);
            (*actor).wallYaw =
                Math_Atan2S((*wallPoly).normal.z as f32_0,
                            (*wallPoly).normal.x as f32_0);
            (*actor).bgCheckFlags =
                ((*actor).bgCheckFlags as libc::c_int | 8 as libc::c_int) as
                    u16_0;
            (*actor).wallBgId = bgId as u8_0
        } else {
            (*actor).bgCheckFlags =
                ((*actor).bgCheckFlags as libc::c_int & !(8 as libc::c_int))
                    as u16_0
        }
    }
    sp64.x = (*actor).world.pos.x;
    sp64.z = (*actor).world.pos.z;
    if flags & 2 as libc::c_int != 0 {
        sp64.y = (*actor).prevPos.y + 10.0f32;
        if BgCheck_EntityCheckCeiling(&mut (*globalCtx).colCtx, &mut sp58,
                                      &mut sp64,
                                      ceilingCheckHeight + sp74 - 10.0f32,
                                      &mut sCurCeilingPoly,
                                      &mut sCurCeilingBgId, actor) != 0 {
            (*actor).bgCheckFlags =
                ((*actor).bgCheckFlags as libc::c_int | 0x10 as libc::c_int)
                    as u16_0;
            (*actor).world.pos.y = sp58 + sp74 - 10.0f32
        } else {
            (*actor).bgCheckFlags =
                ((*actor).bgCheckFlags as libc::c_int &
                     !(0x10 as libc::c_int)) as u16_0
        }
    }
    if flags & 4 as libc::c_int != 0 {
        sp64.y = (*actor).prevPos.y;
        func_8002E2AC(globalCtx, actor, &mut sp64, flags);
        waterBoxYSurface = (*actor).world.pos.y;
        if WaterBox_GetSurface1(globalCtx, &mut (*globalCtx).colCtx,
                                (*actor).world.pos.x, (*actor).world.pos.z,
                                &mut waterBoxYSurface, &mut waterBox) != 0 {
            (*actor).yDistToWater = waterBoxYSurface - (*actor).world.pos.y;
            if (*actor).yDistToWater < 0.0f32 {
                (*actor).bgCheckFlags =
                    ((*actor).bgCheckFlags as libc::c_int &
                         !(0x60 as libc::c_int)) as u16_0
            } else {
                if (*actor).bgCheckFlags as libc::c_int & 0x20 as libc::c_int
                       == 0 {
                    (*actor).bgCheckFlags =
                        ((*actor).bgCheckFlags as libc::c_int |
                             0x40 as libc::c_int) as u16_0;
                    if flags & 0x40 as libc::c_int == 0 {
                        ripplePos.x = (*actor).world.pos.x;
                        ripplePos.y = waterBoxYSurface;
                        ripplePos.z = (*actor).world.pos.z;
                        EffectSsGRipple_Spawn(globalCtx, &mut ripplePos,
                                              100 as libc::c_int as s16,
                                              500 as libc::c_int as s16,
                                              0 as libc::c_int as s16);
                        EffectSsGRipple_Spawn(globalCtx, &mut ripplePos,
                                              100 as libc::c_int as s16,
                                              500 as libc::c_int as s16,
                                              4 as libc::c_int as s16);
                        EffectSsGRipple_Spawn(globalCtx, &mut ripplePos,
                                              100 as libc::c_int as s16,
                                              500 as libc::c_int as s16,
                                              8 as libc::c_int as s16);
                    }
                }
                (*actor).bgCheckFlags =
                    ((*actor).bgCheckFlags as libc::c_int |
                         0x20 as libc::c_int) as u16_0
            }
        } else {
            (*actor).bgCheckFlags =
                ((*actor).bgCheckFlags as libc::c_int &
                     !(0x60 as libc::c_int)) as u16_0;
            (*actor).yDistToWater = -32000.0f32
        }
    };
}
#[no_mangle]
pub static mut D_8015BBA8: Mtx = Mtx{m: [[0; 4]; 4],};
#[no_mangle]
pub unsafe extern "C" fn func_8002E830(mut object: *mut Vec3f,
                                       mut eye: *mut Vec3f,
                                       mut lightDir: *mut Vec3f,
                                       mut gfxCtx: *mut GraphicsContext,
                                       mut gfx: *mut Gfx,
                                       mut hilite: *mut *mut Hilite)
 -> *mut Gfx {
    let mut lookAt: *mut LookAt = 0 as *mut LookAt;
    let mut correctedEyeX: f32_0 = 0.;
    lookAt =
        Graph_Alloc(gfxCtx,
                    ::std::mem::size_of::<LookAt>() as libc::c_ulong as
                        size_t) as *mut LookAt;
    correctedEyeX =
        if (*eye).x == (*object).x && (*eye).z == (*object).z {
            ((*eye).x) + 0.001f32
        } else { (*eye).x };
    *hilite =
        Graph_Alloc(gfxCtx,
                    ::std::mem::size_of::<Hilite>() as libc::c_ulong as
                        size_t) as *mut Hilite;
    if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 80 as libc::c_int) as usize]
           as libc::c_int == 6 as libc::c_int {
        osSyncPrintf(b"z_actor.c 3529 eye=[%f(%f) %f %f] object=[%f %f %f] light_direction=[%f %f %f]\n\x00"
                         as *const u8 as *const libc::c_char,
                     correctedEyeX as libc::c_double,
                     (*eye).x as libc::c_double, (*eye).y as libc::c_double,
                     (*eye).z as libc::c_double,
                     (*object).x as libc::c_double,
                     (*object).y as libc::c_double,
                     (*object).z as libc::c_double,
                     (*lightDir).x as libc::c_double,
                     (*lightDir).y as libc::c_double,
                     (*lightDir).z as libc::c_double);
    }
    func_800ABE74(correctedEyeX, (*eye).y, (*eye).z);
    guLookAtHilite(&mut D_8015BBA8, lookAt, *hilite, correctedEyeX, (*eye).y,
                   (*eye).z, (*object).x, (*object).y, (*object).z, 0.0f32,
                   1.0f32, 0.0f32, (*lightDir).x, (*lightDir).y,
                   (*lightDir).z, (*lightDir).x, (*lightDir).y, (*lightDir).z,
                   0x10 as libc::c_int, 0x10 as libc::c_int);
    let fresh35 = gfx;
    gfx = gfx.offset(1);
    let mut _g: *mut Gfx = fresh35;
    (*_g).words.w0 =
        (0xdc as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((::std::mem::size_of::<Light>() as
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
            ((0 as libc::c_int * 24 as libc::c_int / 8 as libc::c_int) as
                 u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (10 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g).words.w1 = lookAt as libc::c_uint;
    let fresh36 = gfx;
    gfx = gfx.offset(1);
    let mut _g_0: *mut Gfx = fresh36;
    (*_g_0).words.w0 =
        (0xdc as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((::std::mem::size_of::<Light>() as
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
            ((1 as libc::c_int * 24 as libc::c_int / 8 as libc::c_int) as
                 u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (10 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_0).words.w1 =
        (lookAt as *mut libc::c_char).offset(16 as libc::c_int as isize) as
            libc::c_uint;
    let fresh37 = gfx;
    gfx = gfx.offset(1);
    let mut _g_1: *mut Gfx = fresh37;
    (*_g_1).words.w0 =
        (0xf2 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (((**hilite).h.x1 & 0xfff as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            (((**hilite).h.y1 & 0xfff as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_1).words.w1 =
        (1 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 3 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (((0x10 as libc::c_int - 1 as libc::c_int) * 4 as libc::c_int +
                  (**hilite).h.x1 & 0xfff as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            (((0x10 as libc::c_int - 1 as libc::c_int) * 4 as libc::c_int +
                  (**hilite).h.y1 & 0xfff as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    return gfx;
}
#[no_mangle]
pub unsafe extern "C" fn func_8002EABC(mut object: *mut Vec3f,
                                       mut eye: *mut Vec3f,
                                       mut lightDir: *mut Vec3f,
                                       mut gfxCtx: *mut GraphicsContext)
 -> *mut Hilite {
    let mut hilite: *mut Hilite = 0 as *mut Hilite;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), gfxCtx,
                    b"../z_actor.c\x00" as *const u8 as *const libc::c_char,
                    4306 as libc::c_int);
    (*__gfxCtx).polyOpa.p =
        func_8002E830(object, eye, lightDir, gfxCtx, (*__gfxCtx).polyOpa.p,
                      &mut hilite);
    Graph_CloseDisps(dispRefs.as_mut_ptr(), gfxCtx,
                     b"../z_actor.c\x00" as *const u8 as *const libc::c_char,
                     4313 as libc::c_int);
    return hilite;
}
#[no_mangle]
pub unsafe extern "C" fn func_8002EB44(mut object: *mut Vec3f,
                                       mut eye: *mut Vec3f,
                                       mut lightDir: *mut Vec3f,
                                       mut gfxCtx: *mut GraphicsContext)
 -> *mut Hilite {
    let mut hilite: *mut Hilite = 0 as *mut Hilite;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), gfxCtx,
                    b"../z_actor.c\x00" as *const u8 as *const libc::c_char,
                    4332 as libc::c_int);
    (*__gfxCtx).polyXlu.p =
        func_8002E830(object, eye, lightDir, gfxCtx, (*__gfxCtx).polyXlu.p,
                      &mut hilite);
    Graph_CloseDisps(dispRefs.as_mut_ptr(), gfxCtx,
                     b"../z_actor.c\x00" as *const u8 as *const libc::c_char,
                     4339 as libc::c_int);
    return hilite;
}
#[no_mangle]
pub unsafe extern "C" fn func_8002EBCC(mut actor: *mut Actor,
                                       mut globalCtx: *mut GlobalContext,
                                       mut flag: s32) {
    let mut hilite: *mut Hilite = 0 as *mut Hilite;
    let mut lightDir: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut displayListHead: *mut Gfx = 0 as *mut Gfx;
    let mut displayList: *mut Gfx = 0 as *mut Gfx;
    lightDir.x = (*globalCtx).envCtx.dirLight1.params.dir.x as f32_0;
    lightDir.y = (*globalCtx).envCtx.dirLight1.params.dir.y as f32_0;
    lightDir.z = (*globalCtx).envCtx.dirLight1.params.dir.z as f32_0;
    if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 80 as libc::c_int) as usize]
           as libc::c_int == 6 as libc::c_int {
        osSyncPrintf(b"z_actor.c 3637 game_play->view.eye=[%f(%f) %f %f]\n\x00"
                         as *const u8 as *const libc::c_char,
                     (*globalCtx).view.eye.x as libc::c_double,
                     (*globalCtx).view.eye.y as libc::c_double,
                     (*globalCtx).view.eye.z as libc::c_double);
    }
    hilite =
        func_8002EABC(&mut (*actor).world.pos, &mut (*globalCtx).view.eye,
                      &mut lightDir, (*globalCtx).state.gfxCtx);
    if flag != 0 as libc::c_int {
        displayList =
            Graph_Alloc((*globalCtx).state.gfxCtx,
                        (2 as libc::c_int as
                             libc::c_uint).wrapping_mul(::std::mem::size_of::<Gfx>()
                                                            as libc::c_ulong)
                            as size_t) as *mut Gfx;
        displayListHead = displayList;
        let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
        let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
        __gfxCtx = (*globalCtx).state.gfxCtx;
        Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                        b"../z_actor.c\x00" as *const u8 as
                            *const libc::c_char, 4384 as libc::c_int);
        let fresh38 = displayListHead;
        displayListHead = displayListHead.offset(1);
        let mut _g: *mut Gfx = fresh38;
        (*_g).words.w0 =
            (0xf2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (((*hilite).h.x1 & 0xfff as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                (((*hilite).h.y1 & 0xfff as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g).words.w1 =
            (1 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (((0x10 as libc::c_int - 1 as libc::c_int) * 4 as libc::c_int
                      + (*hilite).h.x1 & 0xfff as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                (((0x10 as libc::c_int - 1 as libc::c_int) * 4 as libc::c_int
                      + (*hilite).h.y1 & 0xfff as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let mut _g_0: *mut Gfx = displayListHead;
        (*_g_0).words.w0 =
            (0xdf as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_0).words.w1 = 0 as libc::c_int as libc::c_uint;
        let fresh39 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_1: *mut Gfx = fresh39;
        (*_g_1).words.w0 =
            (0xdb as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0x6 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                ((0x7 as libc::c_int * 4 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_1).words.w1 = displayList as libc::c_uint;
        Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                         b"../z_actor.c\x00" as *const u8 as
                             *const libc::c_char, 4394 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_8002ED80(mut actor: *mut Actor,
                                       mut globalCtx: *mut GlobalContext,
                                       mut flag: s32) {
    let mut hilite: *mut Hilite = 0 as *mut Hilite;
    let mut lightDir: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut displayListHead: *mut Gfx = 0 as *mut Gfx;
    let mut displayList: *mut Gfx = 0 as *mut Gfx;
    lightDir.x = (*globalCtx).envCtx.dirLight1.params.dir.x as f32_0;
    lightDir.y = (*globalCtx).envCtx.dirLight1.params.dir.y as f32_0;
    lightDir.z = (*globalCtx).envCtx.dirLight1.params.dir.z as f32_0;
    hilite =
        func_8002EB44(&mut (*actor).world.pos, &mut (*globalCtx).view.eye,
                      &mut lightDir, (*globalCtx).state.gfxCtx);
    if flag != 0 as libc::c_int {
        displayList =
            Graph_Alloc((*globalCtx).state.gfxCtx,
                        (2 as libc::c_int as
                             libc::c_uint).wrapping_mul(::std::mem::size_of::<Gfx>()
                                                            as libc::c_ulong)
                            as size_t) as *mut Gfx;
        displayListHead = displayList;
        let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
        let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
        __gfxCtx = (*globalCtx).state.gfxCtx;
        Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                        b"../z_actor.c\x00" as *const u8 as
                            *const libc::c_char, 4429 as libc::c_int);
        let fresh40 = displayListHead;
        displayListHead = displayListHead.offset(1);
        let mut _g: *mut Gfx = fresh40;
        (*_g).words.w0 =
            (0xf2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (((*hilite).h.x1 & 0xfff as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                (((*hilite).h.y1 & 0xfff as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g).words.w1 =
            (1 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (((0x10 as libc::c_int - 1 as libc::c_int) * 4 as libc::c_int
                      + (*hilite).h.x1 & 0xfff as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                (((0x10 as libc::c_int - 1 as libc::c_int) * 4 as libc::c_int
                      + (*hilite).h.y1 & 0xfff as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 12 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let mut _g_0: *mut Gfx = displayListHead;
        (*_g_0).words.w0 =
            (0xdf as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_0).words.w1 = 0 as libc::c_int as libc::c_uint;
        let fresh41 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_1: *mut Gfx = fresh41;
        (*_g_1).words.w0 =
            (0xdb as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0x6 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                ((0x7 as libc::c_int * 4 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_1).words.w1 = displayList as libc::c_uint;
        Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                         b"../z_actor.c\x00" as *const u8 as
                             *const libc::c_char, 4439 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Actor_GetFocus(mut dest: *mut PosRot,
                                        mut actor: *mut Actor)
 -> *mut PosRot {
    *dest = (*actor).focus;
    return dest;
}
#[no_mangle]
pub unsafe extern "C" fn Actor_GetWorld(mut dest: *mut PosRot,
                                        mut actor: *mut Actor)
 -> *mut PosRot {
    *dest = (*actor).world;
    return dest;
}
#[no_mangle]
pub unsafe extern "C" fn Actor_GetWorldPosShapeRot(mut arg0: *mut PosRot,
                                                   mut actor: *mut Actor)
 -> *mut PosRot {
    let mut sp1C: PosRot =
        PosRot{pos: Vec3f{x: 0., y: 0., z: 0.,},
               rot: Vec3s{x: 0, y: 0, z: 0,},};
    Math_Vec3f_Copy(&mut sp1C.pos, &mut (*actor).world.pos);
    sp1C.rot = (*actor).shape.rot;
    *arg0 = sp1C;
    return arg0;
}
#[no_mangle]
pub unsafe extern "C" fn func_8002EFC0(mut actor: *mut Actor,
                                       mut player: *mut Player, mut arg2: s16)
 -> f32_0 {
    let mut yawTemp: s16 =
        (((*actor).yawTowardsPlayer as libc::c_int - 0x8000 as libc::c_int) as
             s16 as libc::c_int - arg2 as libc::c_int) as s16;
    let mut yawTempAbs: s16 =
        if yawTemp as libc::c_int >= 0 as libc::c_int {
            yawTemp as libc::c_int
        } else { -(yawTemp as libc::c_int) } as s16;
    if !(*player).unk_664.is_null() {
        if yawTempAbs as libc::c_int > 0x4000 as libc::c_int ||
               (*actor).flags &
                   ((1 as libc::c_int) << 27 as libc::c_int) as libc::c_uint
                   != 0 {
            return 340282346638528859811704183484516925440.0f32
        } else {
            let mut ret: f32_0 =
                (*actor).xyzDistToPlayerSq -
                    (*actor).xyzDistToPlayerSq * 0.8f32 *
                        ((0x4000 as libc::c_int - yawTempAbs as libc::c_int)
                             as libc::c_float *
                             (1.0f32 /
                                  0x8000 as libc::c_int as libc::c_float));
            return ret
        }
    }
    if yawTempAbs as libc::c_int > 0x2aaa as libc::c_int {
        return 340282346638528859811704183484516925440.0f32
    }
    return (*actor).xyzDistToPlayerSq;
}
#[no_mangle]
pub static mut D_80115FF8: [TargetRangeParams; 10] =
    [{
         let mut init =
             TargetRangeParams{rangeSq:
                                   (70 as libc::c_int * 70 as libc::c_int) as
                                       f32_0,
                               leashScale:
                                   70 as libc::c_int as f32_0 /
                                       140 as libc::c_int as libc::c_float,};
         init
     },
     {
         let mut init =
             TargetRangeParams{rangeSq:
                                   (170 as libc::c_int * 170 as libc::c_int)
                                       as f32_0,
                               leashScale:
                                   170 as libc::c_int as f32_0 /
                                       255 as libc::c_int as libc::c_float,};
         init
     },
     {
         let mut init =
             TargetRangeParams{rangeSq:
                                   (280 as libc::c_int * 280 as libc::c_int)
                                       as f32_0,
                               leashScale:
                                   280 as libc::c_int as f32_0 /
                                       5600 as libc::c_int as libc::c_float,};
         init
     },
     {
         let mut init =
             TargetRangeParams{rangeSq:
                                   (350 as libc::c_int * 350 as libc::c_int)
                                       as f32_0,
                               leashScale:
                                   350 as libc::c_int as f32_0 /
                                       525 as libc::c_int as libc::c_float,};
         init
     },
     {
         let mut init =
             TargetRangeParams{rangeSq:
                                   (700 as libc::c_int * 700 as libc::c_int)
                                       as f32_0,
                               leashScale:
                                   700 as libc::c_int as f32_0 /
                                       1050 as libc::c_int as libc::c_float,};
         init
     },
     {
         let mut init =
             TargetRangeParams{rangeSq:
                                   (1000 as libc::c_int * 1000 as libc::c_int)
                                       as f32_0,
                               leashScale:
                                   1000 as libc::c_int as f32_0 /
                                       1500 as libc::c_int as libc::c_float,};
         init
     },
     {
         let mut init =
             TargetRangeParams{rangeSq:
                                   (100 as libc::c_int * 100 as libc::c_int)
                                       as f32_0,
                               leashScale:
                                   (100 as libc::c_int as f32_0 as
                                        libc::c_double / 105.36842f64) as
                                       f32_0,};
         init
     },
     {
         let mut init =
             TargetRangeParams{rangeSq:
                                   (140 as libc::c_int * 140 as libc::c_int)
                                       as f32_0,
                               leashScale:
                                   (140 as libc::c_int as f32_0 as
                                        libc::c_double / 163.33333f64) as
                                       f32_0,};
         init
     },
     {
         let mut init =
             TargetRangeParams{rangeSq:
                                   (240 as libc::c_int * 240 as libc::c_int)
                                       as f32_0,
                               leashScale:
                                   240 as libc::c_int as f32_0 /
                                       576 as libc::c_int as libc::c_float,};
         init
     },
     {
         let mut init =
             TargetRangeParams{rangeSq:
                                   (280 as libc::c_int * 280 as libc::c_int)
                                       as f32_0,
                               leashScale:
                                   280 as libc::c_int as f32_0 /
                                       280000 as libc::c_int as
                                           libc::c_float,};
         init
     }];
#[no_mangle]
pub unsafe extern "C" fn func_8002F090(mut actor: *mut Actor, mut arg1: f32_0)
 -> u32_0 {
    return (arg1 < D_80115FF8[(*actor).targetMode as usize].rangeSq) as
               libc::c_int as u32_0;
}
#[no_mangle]
pub unsafe extern "C" fn func_8002F0C8(mut actor: *mut Actor,
                                       mut player: *mut Player, mut flag: s32)
 -> s32 {
    if (*actor).update.is_none() ||
           (*actor).flags &
               ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint == 0 {
        return 1 as libc::c_int
    }
    if flag == 0 {
        let mut var: s16 =
            (((*actor).yawTowardsPlayer as libc::c_int -
                  0x8000 as libc::c_int) as s16 as libc::c_int -
                 (*player).actor.shape.rot.y as libc::c_int) as s16;
        let mut abs_var: s16 =
            if var as libc::c_int >= 0 as libc::c_int {
                var as libc::c_int
            } else { -(var as libc::c_int) } as s16;
        let mut dist: f32_0 = 0.;
        if (*player).unk_664.is_null() &&
               abs_var as libc::c_int > 0x2aaa as libc::c_int {
            dist = 340282346638528859811704183484516925440.0f32
        } else { dist = (*actor).xyzDistToPlayerSq }
        return (func_8002F090(actor,
                              D_80115FF8[(*actor).targetMode as
                                             usize].leashScale * dist) == 0)
                   as libc::c_int
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Actor_ProcessTalkRequest(mut actor: *mut Actor,
                                                  mut globalCtx:
                                                      *mut GlobalContext)
 -> u32_0 {
    if (*actor).flags &
           ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_uint != 0 {
        (*actor).flags &=
            !((1 as libc::c_int) << 8 as libc::c_int) as libc::c_uint;
        return 1 as libc::c_int as u32_0
    }
    return 0 as libc::c_int as u32_0;
}
#[no_mangle]
pub unsafe extern "C" fn func_8002F1C4(mut actor: *mut Actor,
                                       mut globalCtx: *mut GlobalContext,
                                       mut arg2: f32_0, mut arg3: f32_0,
                                       mut exchangeItemId: u32_0) -> s32 {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    // This is convoluted but it seems like it must be a single if statement to match
    if (*player).actor.flags &
           ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_uint != 0 ||
           exchangeItemId != EXCH_ITEM_NONE as libc::c_int as libc::c_uint &&
               Player_InCsMode(globalCtx) != 0 ||
           (*actor).isTargeted == 0 &&
               (arg3 < fabsf((*actor).yDistToPlayer) ||
                    (*player).targetActorDistance < (*actor).xzDistToPlayer ||
                    arg2 < (*actor).xzDistToPlayer) {
        return 0 as libc::c_int
    }
    (*player).targetActor = actor;
    (*player).targetActorDistance = (*actor).xzDistToPlayer;
    (*player).exchangeItemId = exchangeItemId as s8;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_8002F298(mut actor: *mut Actor,
                                       mut globalCtx: *mut GlobalContext,
                                       mut arg2: f32_0,
                                       mut exchangeItemId: u32_0) -> s32 {
    return func_8002F1C4(actor, globalCtx, arg2, arg2, exchangeItemId);
}
#[no_mangle]
pub unsafe extern "C" fn func_8002F2CC(mut actor: *mut Actor,
                                       mut globalCtx: *mut GlobalContext,
                                       mut arg2: f32_0) -> s32 {
    return func_8002F298(actor, globalCtx, arg2,
                         EXCH_ITEM_NONE as libc::c_int as u32_0);
}
#[no_mangle]
pub unsafe extern "C" fn func_8002F2F4(mut actor: *mut Actor,
                                       mut globalCtx: *mut GlobalContext)
 -> s32 {
    let mut var1: f32_0 =
        50.0f32 +
            (*actor).colChkInfo.cylRadius as libc::c_int as libc::c_float;
    return func_8002F2CC(actor, globalCtx, var1);
}
#[no_mangle]
pub unsafe extern "C" fn Actor_TextboxIsClosing(mut actor: *mut Actor,
                                                mut globalCtx:
                                                    *mut GlobalContext)
 -> u32_0 {
    if Message_GetState(&mut (*globalCtx).msgCtx) as libc::c_int ==
           TEXT_STATE_CLOSING as libc::c_int {
        return 1 as libc::c_int as u32_0
    } else { return 0 as libc::c_int as u32_0 };
}
#[no_mangle]
pub unsafe extern "C" fn func_8002F368(mut globalCtx: *mut GlobalContext)
 -> s8 {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    return (*player).exchangeItemId;
}
#[no_mangle]
pub unsafe extern "C" fn Actor_GetScreenPos(mut globalCtx: *mut GlobalContext,
                                            mut actor: *mut Actor,
                                            mut x: *mut s16,
                                            mut y: *mut s16) {
    let mut projectedPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut w: f32_0 = 0.;
    func_8002BE04(globalCtx, &mut (*actor).focus.pos, &mut projectedPos,
                  &mut w);
    *x =
        (projectedPos.x * w *
             (320 as libc::c_int / 2 as libc::c_int) as libc::c_float +
             (320 as libc::c_int / 2 as libc::c_int) as libc::c_float) as s16;
    *y =
        (projectedPos.y * w *
             -(240 as libc::c_int / 2 as libc::c_int) as libc::c_float +
             (240 as libc::c_int / 2 as libc::c_int) as libc::c_float) as s16;
}
#[no_mangle]
pub unsafe extern "C" fn Actor_HasParent(mut actor: *mut Actor,
                                         mut globalCtx: *mut GlobalContext)
 -> u32_0 {
    if !(*actor).parent.is_null() {
        return 1 as libc::c_int as u32_0
    } else { return 0 as libc::c_int as u32_0 };
}
#[no_mangle]
pub unsafe extern "C" fn func_8002F434(mut actor: *mut Actor,
                                       mut globalCtx: *mut GlobalContext,
                                       mut getItemId: s32, mut xzRange: f32_0,
                                       mut yRange: f32_0) -> s32 {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    if (*player).stateFlags1 & 0x3c7080 as libc::c_int as libc::c_uint == 0 &&
           Player_GetExplosiveHeld(player) < 0 as libc::c_int {
        if (!(*player).heldActor.is_null() || actor == (*player).targetActor)
               && getItemId > GI_NONE as libc::c_int &&
               getItemId < GI_MAX as libc::c_int ||
               (*player).stateFlags1 &
                   0x20000800 as libc::c_int as libc::c_uint == 0 {
            if (*actor).xzDistToPlayer < xzRange &&
                   fabsf((*actor).yDistToPlayer) < yRange {
                let mut yawDiff: s16 =
                    ((*actor).yawTowardsPlayer as libc::c_int -
                         (*player).actor.shape.rot.y as libc::c_int) as s16;
                let mut absYawDiff: s32 =
                    if yawDiff as libc::c_int >= 0 as libc::c_int {
                        yawDiff as libc::c_int
                    } else { -(yawDiff as libc::c_int) };
                if getItemId != GI_NONE as libc::c_int ||
                       ((*player).getItemDirection as libc::c_int) <
                           absYawDiff {
                    (*player).getItemId = getItemId as s8;
                    (*player).interactRangeActor = actor;
                    (*player).getItemDirection = absYawDiff as u16_0;
                    return 1 as libc::c_int
                }
            }
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_8002F554(mut actor: *mut Actor,
                                       mut globalCtx: *mut GlobalContext,
                                       mut getItemId: s32) {
    func_8002F434(actor, globalCtx, getItemId, 50.0f32, 10.0f32);
}
#[no_mangle]
pub unsafe extern "C" fn func_8002F580(mut actor: *mut Actor,
                                       mut globalCtx: *mut GlobalContext) {
    func_8002F554(actor, globalCtx, GI_NONE as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Actor_HasNoParent(mut actor: *mut Actor,
                                           mut globalCtx: *mut GlobalContext)
 -> u32_0 {
    if (*actor).parent.is_null() {
        return 1 as libc::c_int as u32_0
    } else { return 0 as libc::c_int as u32_0 };
}
#[no_mangle]
pub unsafe extern "C" fn func_8002F5C4(mut actorA: *mut Actor,
                                       mut actorB: *mut Actor,
                                       mut globalCtx: *mut GlobalContext) {
    let mut parent: *mut Actor = (*actorA).parent;
    if (*parent).id as libc::c_int == ACTOR_PLAYER as libc::c_int {
        let mut player: *mut Player = parent as *mut Player;
        (*player).heldActor = actorB;
        (*player).interactRangeActor = actorB
    }
    (*parent).child = actorB;
    (*actorB).parent = parent;
    (*actorA).parent = 0 as *mut Actor;
}
#[no_mangle]
pub unsafe extern "C" fn func_8002F5F0(mut actor: *mut Actor,
                                       mut globalCtx: *mut GlobalContext) {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    if (*actor).xyzDistToPlayerSq < (*player).unk_6A4 {
        (*player).unk_6A4 = (*actor).xyzDistToPlayerSq
    };
}
#[no_mangle]
pub unsafe extern "C" fn Actor_IsMounted(mut globalCtx: *mut GlobalContext,
                                         mut horse: *mut Actor) -> s32 {
    if !(*horse).child.is_null() {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn Actor_SetRideActor(mut globalCtx: *mut GlobalContext,
                                            mut horse: *mut Actor,
                                            mut mountSide: s32) -> u32_0 {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    if (*player).stateFlags1 & 0x3c7880 as libc::c_int as libc::c_uint == 0 {
        (*player).rideActor = horse;
        (*player).mountSide = mountSide as s8;
        return 1 as libc::c_int as u32_0
    }
    return 0 as libc::c_int as u32_0;
}
#[no_mangle]
pub unsafe extern "C" fn Actor_NotMounted(mut globalCtx: *mut GlobalContext,
                                          mut horse: *mut Actor) -> s32 {
    if (*horse).child.is_null() {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn func_8002F698(mut globalCtx: *mut GlobalContext,
                                       mut actor: *mut Actor, mut arg2: f32_0,
                                       mut arg3: s16, mut arg4: f32_0,
                                       mut arg5: u32_0, mut arg6: u32_0) {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    (*player).unk_8A0 = arg6 as u8_0;
    (*player).unk_8A1 = arg5 as u8_0;
    (*player).unk_8A2 = arg3;
    (*player).unk_8A4 = arg2;
    (*player).unk_8A8 = arg4;
}
#[no_mangle]
pub unsafe extern "C" fn func_8002F6D4(mut globalCtx: *mut GlobalContext,
                                       mut actor: *mut Actor, mut arg2: f32_0,
                                       mut arg3: s16, mut arg4: f32_0,
                                       mut arg5: u32_0) {
    func_8002F698(globalCtx, actor, arg2, arg3, arg4,
                  2 as libc::c_int as u32_0, arg5);
}
#[no_mangle]
pub unsafe extern "C" fn func_8002F71C(mut globalCtx: *mut GlobalContext,
                                       mut actor: *mut Actor, mut arg2: f32_0,
                                       mut arg3: s16, mut arg4: f32_0) {
    func_8002F6D4(globalCtx, actor, arg2, arg3, arg4,
                  0 as libc::c_int as u32_0);
}
#[no_mangle]
pub unsafe extern "C" fn func_8002F758(mut globalCtx: *mut GlobalContext,
                                       mut actor: *mut Actor, mut arg2: f32_0,
                                       mut arg3: s16, mut arg4: f32_0,
                                       mut arg5: u32_0) {
    func_8002F698(globalCtx, actor, arg2, arg3, arg4,
                  1 as libc::c_int as u32_0, arg5);
}
#[no_mangle]
pub unsafe extern "C" fn func_8002F7A0(mut globalCtx: *mut GlobalContext,
                                       mut actor: *mut Actor, mut arg2: f32_0,
                                       mut arg3: s16, mut arg4: f32_0) {
    func_8002F758(globalCtx, actor, arg2, arg3, arg4,
                  0 as libc::c_int as u32_0);
}
#[no_mangle]
pub unsafe extern "C" fn func_8002F7DC(mut actor: *mut Actor,
                                       mut sfxId: u16_0) {
    Audio_PlaySoundGeneral(sfxId, &mut (*actor).projectedPos,
                           4 as libc::c_int as u8_0, &mut D_801333E0,
                           &mut D_801333E0, &mut D_801333E8);
}
#[no_mangle]
pub unsafe extern "C" fn Audio_PlayActorSound2(mut actor: *mut Actor,
                                               mut sfxId: u16_0) {
    func_80078914(&mut (*actor).projectedPos, sfxId);
}
#[no_mangle]
pub unsafe extern "C" fn func_8002F850(mut globalCtx: *mut GlobalContext,
                                       mut actor: *mut Actor) {
    let mut sfxId: s32 = 0;
    if (*actor).bgCheckFlags as libc::c_int & 0x20 as libc::c_int != 0 {
        if (*actor).yDistToWater < 20.0f32 {
            sfxId = 0x804 as libc::c_int - 0x800 as libc::c_int
        } else { sfxId = 0x805 as libc::c_int - 0x800 as libc::c_int }
    } else {
        sfxId =
            SurfaceType_GetSfx(&mut (*globalCtx).colCtx, (*actor).floorPoly,
                               (*actor).floorBgId as s32) as s32
    }
    func_80078914(&mut (*actor).projectedPos, 0x282f as libc::c_int as u16_0);
    func_80078914(&mut (*actor).projectedPos,
                  (sfxId + 0x800 as libc::c_int) as u16_0);
}
#[no_mangle]
pub unsafe extern "C" fn func_8002F8F0(mut actor: *mut Actor,
                                       mut sfxId: u16_0) {
    (*actor).sfx = sfxId;
    (*actor).flags |=
        ((1 as libc::c_int) << 19 as libc::c_int) as libc::c_uint;
    (*actor).flags &=
        !((1 as libc::c_int) << 20 as libc::c_int |
              (1 as libc::c_int) << 21 as libc::c_int |
              (1 as libc::c_int) << 28 as libc::c_int) as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn func_8002F91C(mut actor: *mut Actor,
                                       mut sfxId: u16_0) {
    (*actor).sfx = sfxId;
    (*actor).flags |=
        ((1 as libc::c_int) << 20 as libc::c_int) as libc::c_uint;
    (*actor).flags &=
        !((1 as libc::c_int) << 19 as libc::c_int |
              (1 as libc::c_int) << 21 as libc::c_int |
              (1 as libc::c_int) << 28 as libc::c_int) as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn func_8002F948(mut actor: *mut Actor,
                                       mut sfxId: u16_0) {
    (*actor).sfx = sfxId;
    (*actor).flags |=
        ((1 as libc::c_int) << 21 as libc::c_int) as libc::c_uint;
    (*actor).flags &=
        !((1 as libc::c_int) << 19 as libc::c_int |
              (1 as libc::c_int) << 20 as libc::c_int |
              (1 as libc::c_int) << 28 as libc::c_int) as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn func_8002F974(mut actor: *mut Actor,
                                       mut sfxId: u16_0) {
    (*actor).flags &=
        !((1 as libc::c_int) << 19 as libc::c_int |
              (1 as libc::c_int) << 20 as libc::c_int |
              (1 as libc::c_int) << 21 as libc::c_int |
              (1 as libc::c_int) << 28 as libc::c_int) as libc::c_uint;
    (*actor).sfx = sfxId;
}
#[no_mangle]
pub unsafe extern "C" fn func_8002F994(mut actor: *mut Actor, mut arg1: s32) {
    (*actor).flags |=
        ((1 as libc::c_int) << 28 as libc::c_int) as libc::c_uint;
    (*actor).flags &=
        !((1 as libc::c_int) << 19 as libc::c_int |
              (1 as libc::c_int) << 20 as libc::c_int |
              (1 as libc::c_int) << 21 as libc::c_int) as libc::c_uint;
    if arg1 < 40 as libc::c_int {
        (*actor).sfx = (0x803 as libc::c_int - 0x800 as libc::c_int) as u16_0
    } else if arg1 < 100 as libc::c_int {
        (*actor).sfx = (0x802 as libc::c_int - 0x800 as libc::c_int) as u16_0
    } else {
        (*actor).sfx = (0x801 as libc::c_int - 0x800 as libc::c_int) as u16_0
    };
}
// Tests if something hit Jabu Jabu surface, displaying hit splash and playing sfx if true
#[no_mangle]
pub unsafe extern "C" fn func_8002F9EC(mut globalCtx: *mut GlobalContext,
                                       mut actor: *mut Actor,
                                       mut poly: *mut CollisionPoly,
                                       mut bgId: s32, mut pos: *mut Vec3f)
 -> s32 {
    if func_80041D4C(&mut (*globalCtx).colCtx, poly, bgId) ==
           8 as libc::c_int as libc::c_uint {
        (*globalCtx).roomCtx.unk_74[0 as libc::c_int as usize] =
            1 as libc::c_int as s16;
        CollisionCheck_BlueBlood(globalCtx, 0 as *mut Collider, pos);
        Audio_PlayActorSound2(actor, 0x182c as libc::c_int as u16_0);
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
// Local data used for Farore's Wind light (stored in BSS, possibly a struct?)
#[no_mangle]
pub static mut D_8015BC00: LightInfo =
    LightInfo{type_0: 0,
              params:
                  LightParams{point:
                                  LightPoint{x: 0,
                                             y: 0,
                                             z: 0,
                                             color: [0; 3],
                                             drawGlow: 0,
                                             radius: 0,},},};
#[no_mangle]
pub static mut D_8015BC10: *mut LightNode =
    0 as *const LightNode as *mut LightNode;
#[no_mangle]
pub static mut D_8015BC14: s32 = 0;
#[no_mangle]
pub static mut D_8015BC18: f32_0 = 0.;
#[no_mangle]
pub unsafe extern "C" fn func_8002FA60(mut globalCtx: *mut GlobalContext) {
    let mut lightPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    if gSaveContext.fw.set != 0 {
        gSaveContext.respawn[RESPAWN_MODE_TOP as libc::c_int as usize].data =
            0x28 as libc::c_int as s8;
        gSaveContext.respawn[RESPAWN_MODE_TOP as libc::c_int as usize].pos.x =
            gSaveContext.fw.pos.x as f32_0;
        gSaveContext.respawn[RESPAWN_MODE_TOP as libc::c_int as usize].pos.y =
            gSaveContext.fw.pos.y as f32_0;
        gSaveContext.respawn[RESPAWN_MODE_TOP as libc::c_int as usize].pos.z =
            gSaveContext.fw.pos.z as f32_0;
        gSaveContext.respawn[RESPAWN_MODE_TOP as libc::c_int as usize].yaw =
            gSaveContext.fw.yaw as s16;
        gSaveContext.respawn[RESPAWN_MODE_TOP as libc::c_int as
                                 usize].playerParams =
            gSaveContext.fw.playerParams as s16;
        gSaveContext.respawn[RESPAWN_MODE_TOP as libc::c_int as
                                 usize].entranceIndex =
            gSaveContext.fw.entranceIndex as s16;
        gSaveContext.respawn[RESPAWN_MODE_TOP as libc::c_int as
                                 usize].roomIndex =
            gSaveContext.fw.roomIndex as u8_0;
        gSaveContext.respawn[RESPAWN_MODE_TOP as libc::c_int as
                                 usize].tempSwchFlags =
            gSaveContext.fw.tempSwchFlags as u32_0;
        gSaveContext.respawn[RESPAWN_MODE_TOP as libc::c_int as
                                 usize].tempCollectFlags =
            gSaveContext.fw.tempCollectFlags as u32_0
    } else {
        gSaveContext.respawn[RESPAWN_MODE_TOP as libc::c_int as usize].data =
            0 as libc::c_int as s8;
        gSaveContext.respawn[RESPAWN_MODE_TOP as libc::c_int as usize].pos.x =
            0.0f32;
        gSaveContext.respawn[RESPAWN_MODE_TOP as libc::c_int as usize].pos.y =
            0.0f32;
        gSaveContext.respawn[RESPAWN_MODE_TOP as libc::c_int as usize].pos.z =
            0.0f32
    }
    lightPos.x =
        gSaveContext.respawn[RESPAWN_MODE_TOP as libc::c_int as usize].pos.x;
    lightPos.y =
        gSaveContext.respawn[RESPAWN_MODE_TOP as libc::c_int as usize].pos.y +
            80.0f32;
    lightPos.z =
        gSaveContext.respawn[RESPAWN_MODE_TOP as libc::c_int as usize].pos.z;
    Lights_PointNoGlowSetInfo(&mut D_8015BC00, lightPos.x as s16,
                              lightPos.y as s16, lightPos.z as s16,
                              0xff as libc::c_int as u8_0,
                              0xff as libc::c_int as u8_0,
                              0xff as libc::c_int as u8_0,
                              -(1 as libc::c_int) as s16);
    D_8015BC10 =
        LightContext_InsertLight(globalCtx, &mut (*globalCtx).lightCtx,
                                 &mut D_8015BC00);
    D_8015BC14 = 0 as libc::c_int;
    D_8015BC18 = 0.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn Actor_DrawFaroresWindPointer(mut globalCtx:
                                                          *mut GlobalContext) {
    let mut lightRadius: s32 = -(1 as libc::c_int);
    let mut params: s32 = 0;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_actor.c\x00" as *const u8 as *const libc::c_char,
                    5308 as libc::c_int);
    params =
        gSaveContext.respawn[RESPAWN_MODE_TOP as libc::c_int as usize].data as
            s32;
    if params != 0 {
        let mut yOffset: f32_0 =
            if gSaveContext.linkAge == 0 as libc::c_int {
                80.0f32
            } else { 60.0f32 };
        let mut ratio: f32_0 = 1.0f32;
        let mut alpha: s32 = 255 as libc::c_int;
        let mut temp: s32 = params - 40 as libc::c_int;
        if temp < 0 as libc::c_int {
            params += 1;
            gSaveContext.respawn[RESPAWN_MODE_TOP as libc::c_int as
                                     usize].data = params as s8;
            ratio =
                (if params >= 0 as libc::c_int { params } else { -params }) as
                    libc::c_float * 0.025f32;
            D_8015BC14 = 60 as libc::c_int;
            D_8015BC18 = 1.0f32
        } else if D_8015BC14 != 0 {
            D_8015BC14 -= 1
        } else if D_8015BC18 > 0.0f32 {
            static mut effectVel: Vec3f =
                {
                    let mut init = Vec3f{x: 0.0f32, y: -0.05f32, z: 0.0f32,};
                    init
                };
            static mut effectAccel: Vec3f =
                {
                    let mut init = Vec3f{x: 0.0f32, y: -0.025f32, z: 0.0f32,};
                    init
                };
            static mut effectPrimCol: Color_RGBA8 =
                {
                    let mut init =
                        Color_RGBA8{r: 255 as libc::c_int as u8_0,
                                    g: 255 as libc::c_int as u8_0,
                                    b: 255 as libc::c_int as u8_0,
                                    a: 0 as libc::c_int as u8_0,};
                    init
                };
            static mut effectEnvCol: Color_RGBA8 =
                {
                    let mut init =
                        Color_RGBA8{r: 100 as libc::c_int as u8_0,
                                    g: 200 as libc::c_int as u8_0,
                                    b: 0 as libc::c_int as u8_0,
                                    a: 0 as libc::c_int as u8_0,};
                    init
                };
            let mut curPos: *mut Vec3f =
                &mut (*gSaveContext.respawn.as_mut_ptr().offset(RESPAWN_MODE_TOP
                                                                    as
                                                                    libc::c_int
                                                                    as
                                                                    isize)).pos;
            let mut nextPos: *mut Vec3f =
                &mut (*gSaveContext.respawn.as_mut_ptr().offset(RESPAWN_MODE_DOWN
                                                                    as
                                                                    libc::c_int
                                                                    as
                                                                    isize)).pos;
            let mut prevNum: f32_0 = D_8015BC18;
            let mut dist: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
            let mut diff: f32_0 =
                Math_Vec3f_DistXYZAndStoreDiff(nextPos, curPos, &mut dist);
            let mut effectPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
            let mut factor: f32_0 = 0.;
            let mut length: f32_0 = 0.;
            let mut dx: f32_0 = 0.;
            let mut speed: f32_0 = 0.;
            if diff < 20.0f32 {
                D_8015BC18 = 0.0f32;
                Math_Vec3f_Copy(curPos, nextPos);
            } else {
                length = diff * (1.0f32 / D_8015BC18);
                speed = 20.0f32 / length;
                speed = if speed < 0.05f32 { 0.05f32 } else { speed };
                Math_StepToF(&mut D_8015BC18, 0.0f32, speed);
                factor = diff * (D_8015BC18 / prevNum) / diff;
                (*curPos).x = (*nextPos).x + dist.x * factor;
                (*curPos).y = (*nextPos).y + dist.y * factor;
                (*curPos).z = (*nextPos).z + dist.z * factor;
                length *= 0.5f32;
                dx = diff - length;
                yOffset += sqrtf(length * length - dx * dx) * 0.2f32;
                osSyncPrintf(b"-------- DISPLAY Y=%f\n\x00" as *const u8 as
                                 *const libc::c_char,
                             yOffset as libc::c_double);
            }
            effectPos.x = (*curPos).x + Rand_CenteredFloat(6.0f32);
            effectPos.y = (*curPos).y + 80.0f32 + 6.0f32 * Rand_ZeroOne();
            effectPos.z = (*curPos).z + Rand_CenteredFloat(6.0f32);
            EffectSsKiraKira_SpawnDispersed(globalCtx, &mut effectPos,
                                            &mut effectVel, &mut effectAccel,
                                            &mut effectPrimCol,
                                            &mut effectEnvCol,
                                            1000 as libc::c_int as s16,
                                            16 as libc::c_int);
            if D_8015BC18 == 0.0f32 {
                gSaveContext.respawn[RESPAWN_MODE_TOP as libc::c_int as usize]
                    =
                    gSaveContext.respawn[RESPAWN_MODE_DOWN as libc::c_int as
                                             usize];
                gSaveContext.respawn[RESPAWN_MODE_TOP as libc::c_int as
                                         usize].playerParams =
                    0x6ff as libc::c_int as s16;
                gSaveContext.respawn[RESPAWN_MODE_TOP as libc::c_int as
                                         usize].data = 40 as libc::c_int as s8
            }
            gSaveContext.respawn[RESPAWN_MODE_TOP as libc::c_int as usize].pos
                = *curPos
        } else if temp > 0 as libc::c_int {
            let mut curPos_0: *mut Vec3f =
                &mut (*gSaveContext.respawn.as_mut_ptr().offset(RESPAWN_MODE_TOP
                                                                    as
                                                                    libc::c_int
                                                                    as
                                                                    isize)).pos;
            let mut nextRatio: f32_0 =
                1.0f32 - temp as libc::c_float * 0.1f32;
            let mut curRatio: f32_0 =
                1.0f32 - (temp - 1 as libc::c_int) as f32_0 * 0.1f32;
            let mut eye: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
            let mut dist_0: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
            let mut diff_0: f32_0 = 0.;
            if nextRatio > 0.0f32 {
                eye.x = (*globalCtx).view.eye.x;
                eye.y = (*globalCtx).view.eye.y - yOffset;
                eye.z = (*globalCtx).view.eye.z;
                diff_0 =
                    Math_Vec3f_DistXYZAndStoreDiff(&mut eye, curPos_0,
                                                   &mut dist_0);
                diff_0 = diff_0 * (nextRatio / curRatio) / diff_0;
                (*curPos_0).x = eye.x + dist_0.x * diff_0;
                (*curPos_0).y = eye.y + dist_0.y * diff_0;
                (*curPos_0).z = eye.z + dist_0.z * diff_0;
                gSaveContext.respawn[RESPAWN_MODE_TOP as libc::c_int as
                                         usize].pos = *curPos_0
            }
            alpha = 255 as libc::c_int - temp * 30 as libc::c_int;
            if alpha < 0 as libc::c_int {
                gSaveContext.fw.set = 0 as libc::c_int;
                gSaveContext.respawn[RESPAWN_MODE_TOP as libc::c_int as
                                         usize].data = 0 as libc::c_int as s8;
                alpha = 0 as libc::c_int
            } else {
                params += 1;
                gSaveContext.respawn[RESPAWN_MODE_TOP as libc::c_int as
                                         usize].data = params as s8
            }
            ratio =
                (1.0f32 as libc::c_double +
                     temp as f32_0 as libc::c_double * 0.2f64) as f32_0
            // required to match
        }
        lightRadius = (500.0f32 * ratio) as s32;
        if (*globalCtx).csCtx.state as libc::c_int ==
               CS_STATE_IDLE as libc::c_int &&
               gSaveContext.respawn[RESPAWN_MODE_TOP as libc::c_int as
                                        usize].entranceIndex as libc::c_int ==
                   gSaveContext.entranceIndex &&
               gSaveContext.respawn[RESPAWN_MODE_TOP as libc::c_int as
                                        usize].roomIndex as libc::c_int ==
                   (*globalCtx).roomCtx.curRoom.num as libc::c_int {
            let mut scale: f32_0 = 0.025f32 * ratio;
            (*__gfxCtx).polyXlu.p =
                Gfx_CallSetupDL((*__gfxCtx).polyXlu.p,
                                0x19 as libc::c_int as u32_0);
            Matrix_Translate(gSaveContext.respawn[RESPAWN_MODE_TOP as
                                                      libc::c_int as
                                                      usize].pos.x,
                             gSaveContext.respawn[RESPAWN_MODE_TOP as
                                                      libc::c_int as
                                                      usize].pos.y + yOffset,
                             gSaveContext.respawn[RESPAWN_MODE_TOP as
                                                      libc::c_int as
                                                      usize].pos.z,
                             MTXMODE_NEW as libc::c_int as u8_0);
            Matrix_Scale(scale, scale, scale,
                         MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_Mult(&mut (*globalCtx).billboardMtxF,
                        MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_Push();
            let fresh42 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g: *mut Gfx = fresh42;
            (*_g).words.w0 =
                (0xe7 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g).words.w1 = 0 as libc::c_int as libc::c_uint;
            let fresh43 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_0: *mut Gfx = fresh43;
            (*_g_0).words.w0 =
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
            (*_g_0).words.w1 =
                (255 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (255 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (200 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    (alpha as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh44 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_1: *mut Gfx = fresh44;
            (*_g_1).words.w0 =
                (0xfb as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_1).words.w1 =
                (100 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (200 as libc::c_int as u32_0 &
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
            Matrix_RotateZ(((*globalCtx).gameplayFrames.wrapping_mul(1500 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint)
                                & 0xffff as libc::c_int as libc::c_uint) as
                               libc::c_float * 3.14159265358979323846f32 /
                               32768.0f32,
                           MTXMODE_APPLY as libc::c_int as u8_0);
            let fresh45 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_2: *mut Gfx = fresh45;
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
                              b"../z_actor.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              5458 as libc::c_int) as libc::c_uint;
            let fresh46 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_3: *mut Gfx = fresh46;
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
            (*_g_3).words.w1 = gEffFlash1DL.as_mut_ptr() as libc::c_uint;
            Matrix_Pop();
            Matrix_RotateZ(!((*globalCtx).gameplayFrames.wrapping_mul(1200 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint)
                                 & 0xffff as libc::c_int as libc::c_uint) as
                               libc::c_float * 3.14159265358979323846f32 /
                               32768.0f32,
                           MTXMODE_APPLY as libc::c_int as u8_0);
            let fresh47 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_4: *mut Gfx = fresh47;
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
                              b"../z_actor.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              5463 as libc::c_int) as libc::c_uint;
            let fresh48 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_5: *mut Gfx = fresh48;
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
            (*_g_5).words.w1 = gEffFlash1DL.as_mut_ptr() as libc::c_uint
        }
        Lights_PointNoGlowSetInfo(&mut D_8015BC00,
                                  gSaveContext.respawn[RESPAWN_MODE_TOP as
                                                           libc::c_int as
                                                           usize].pos.x as
                                      s16,
                                  (gSaveContext.respawn[RESPAWN_MODE_TOP as
                                                            libc::c_int as
                                                            usize].pos.y +
                                       yOffset) as s16,
                                  gSaveContext.respawn[RESPAWN_MODE_TOP as
                                                           libc::c_int as
                                                           usize].pos.z as
                                      s16, 255 as libc::c_int as u8_0,
                                  255 as libc::c_int as u8_0,
                                  255 as libc::c_int as u8_0,
                                  lightRadius as s16);
        Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                         b"../z_actor.c\x00" as *const u8 as
                             *const libc::c_char, 5474 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_80030488(mut globalCtx: *mut GlobalContext) {
    LightContext_RemoveLight(globalCtx, &mut (*globalCtx).lightCtx,
                             D_8015BC10);
}
#[no_mangle]
pub unsafe extern "C" fn func_800304B0(mut globalCtx: *mut GlobalContext) {
    if (*globalCtx).actorCtx.unk_03 as libc::c_int != 0 as libc::c_int {
        (*globalCtx).actorCtx.unk_03 = 0 as libc::c_int as u8_0;
        func_800876C8(globalCtx);
    };
}
// Actor_InitContext
#[no_mangle]
pub unsafe extern "C" fn func_800304DC(mut globalCtx: *mut GlobalContext,
                                       mut actorCtx: *mut ActorContext,
                                       mut actorEntry: *mut ActorEntry) {
    let mut overlayEntry: *mut ActorOverlay =
        0 as *mut ActorOverlay; // "Actor name (%08x:%s)"
    let mut savedSceneFlags: *mut SavedSceneFlags = 0 as *mut SavedSceneFlags;
    let mut i: s32 = 0;
    savedSceneFlags =
        &mut *gSaveContext.sceneFlags.as_mut_ptr().offset((*globalCtx).sceneNum
                                                              as isize) as
            *mut SavedSceneFlags;
    bzero(actorCtx as *mut libc::c_void,
          ::std::mem::size_of::<ActorContext>() as libc::c_ulong);
    ActorOverlayTable_Init();
    Matrix_MtxFCopy(&mut (*globalCtx).billboardMtxF, &mut gMtxFClear);
    Matrix_MtxFCopy(&mut (*globalCtx).viewProjectionMtxF, &mut gMtxFClear);
    overlayEntry =
        &mut *gActorOverlayTable.as_mut_ptr().offset(0 as libc::c_int as
                                                         isize) as
            *mut ActorOverlay;
    i = 0 as libc::c_int;
    while i <
              (::std::mem::size_of::<[ActorOverlay; 471]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<ActorOverlay>()
                                                   as libc::c_ulong) as s32 {
        (*overlayEntry).loadedRamAddr = 0 as *mut libc::c_void;
        (*overlayEntry).numLoaded = 0 as libc::c_int as s8;
        overlayEntry = overlayEntry.offset(1);
        i += 1
    }
    (*actorCtx).flags.chest = (*savedSceneFlags).chest;
    (*actorCtx).flags.swch = (*savedSceneFlags).swch;
    (*actorCtx).flags.clear = (*savedSceneFlags).clear;
    (*actorCtx).flags.collect = (*savedSceneFlags).collect;
    func_8002CDE4(globalCtx, &mut (*actorCtx).titleCtx);
    (*actorCtx).absoluteSpace = 0 as *mut libc::c_void;
    Actor_SpawnEntry(actorCtx, actorEntry, globalCtx);
    func_8002C0C0(&mut (*actorCtx).targetCtx,
                  (*actorCtx).actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head, globalCtx);
    func_8002FA60(globalCtx);
}
#[no_mangle]
pub static mut D_80116068: [u32_0; 12] =
    [0x100000c0 as libc::c_int as u32_0, 0x100000c0 as libc::c_int as u32_0,
     0 as libc::c_int as u32_0, 0x100004c0 as libc::c_int as u32_0,
     0x80 as libc::c_int as u32_0, 0x300000c0 as libc::c_int as u32_0,
     0x10000080 as libc::c_int as u32_0, 0 as libc::c_int as u32_0,
     0x300000c0 as libc::c_int as u32_0, 0x100004c0 as libc::c_int as u32_0,
     0 as libc::c_int as u32_0, 0x100000c0 as libc::c_int as u32_0];
#[no_mangle]
pub unsafe extern "C" fn Actor_UpdateAll(mut globalCtx: *mut GlobalContext,
                                         mut actorCtx: *mut ActorContext) {
    let mut refActor: *mut Actor = 0 as *mut Actor;
    let mut actor: *mut Actor = 0 as *mut Actor;
    let mut player: *mut Player = 0 as *mut Player;
    let mut sp80: *mut u32_0 = 0 as *mut u32_0;
    let mut unkFlag: u32_0 = 0;
    let mut unkCondition: u32_0 = 0;
    let mut sp74: *mut Actor = 0 as *mut Actor;
    let mut actorEntry: *mut ActorEntry = 0 as *mut ActorEntry;
    let mut i: s32 = 0;
    player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    sp74 = 0 as *mut Actor;
    unkFlag = 0 as libc::c_int as u32_0;
    if (*globalCtx).numSetupActors as libc::c_int != 0 as libc::c_int {
        actorEntry =
            &mut *(*globalCtx).setupActorList.offset(0 as libc::c_int as
                                                         isize) as
                *mut ActorEntry;
        i = 0 as libc::c_int;
        while i < (*globalCtx).numSetupActors as libc::c_int {
            let fresh49 = actorEntry;
            actorEntry = actorEntry.offset(1);
            Actor_SpawnEntry(&mut (*globalCtx).actorCtx, fresh49, globalCtx);
            i += 1
        }
        (*globalCtx).numSetupActors = 0 as libc::c_int as u8_0
    }
    if (*actorCtx).unk_02 as libc::c_int != 0 as libc::c_int {
        (*actorCtx).unk_02 = (*actorCtx).unk_02.wrapping_sub(1)
    }
    if (*gGameInfo).data[(13 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 0 as libc::c_int) as usize]
           as libc::c_int == -(100 as libc::c_int) {
        refActor =
            &mut (*((*(*globalCtx).actorCtx.actorLists.as_mut_ptr().offset(ACTORCAT_PLAYER
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize)).head
                        as *mut Player)).actor;
        (*gGameInfo).data[(13 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 0 as libc::c_int) as usize]
            = 0 as libc::c_int as s16;
        Actor_Spawn(&mut (*globalCtx).actorCtx, globalCtx,
                    ACTOR_EN_CLEAR_TAG as libc::c_int as s16,
                    (*refActor).world.pos.x,
                    (*refActor).world.pos.y + 100.0f32,
                    (*refActor).world.pos.z, 0 as libc::c_int as s16,
                    0 as libc::c_int as s16, 0 as libc::c_int as s16,
                    1 as libc::c_int as s16);
    }
    sp80 =
        &mut *D_80116068.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut u32_0;
    if (*player).stateFlags2 & 0x8000000 as libc::c_int as libc::c_uint != 0 {
        unkFlag = ((1 as libc::c_int) << 25 as libc::c_int) as u32_0
    }
    if (*player).stateFlags1 & 0x40 as libc::c_int as libc::c_uint != 0 &&
           (*player).actor.textId as libc::c_int & 0xff00 as libc::c_int !=
               0x600 as libc::c_int {
        sp74 = (*player).targetActor
    }
    i = 0 as libc::c_int;
    while i <
              (::std::mem::size_of::<[ActorListEntry; 12]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<ActorListEntry>()
                                                   as libc::c_ulong) as s32 {
        unkCondition = *sp80 & (*player).stateFlags1;
        actor = (*actorCtx).actorLists[i as usize].head;
        while !actor.is_null() {
            if (*actor).world.pos.y < -25000.0f32 {
                (*actor).world.pos.y = -25000.0f32
            }
            (*actor).sfx = 0 as libc::c_int as u16_0;
            if (*actor).init.is_some() {
                if Object_IsLoaded(&mut (*globalCtx).objectCtx,
                                   (*actor).objBankIndex as s32) != 0 {
                    Actor_SetObjectDependency(globalCtx, actor);
                    (*actor).init.expect("non-null function pointer")(actor,
                                                                      globalCtx);
                    (*actor).init = None
                }
                actor = (*actor).next
            } else if Object_IsLoaded(&mut (*globalCtx).objectCtx,
                                      (*actor).objBankIndex as s32) == 0 {
                Actor_Kill(actor);
                actor = (*actor).next
            } else if unkFlag != 0 && (*actor).flags & unkFlag == 0 ||
                          unkFlag == 0 && unkCondition != 0 && sp74 != actor
                              && actor != (*player).naviActor &&
                              actor != (*player).heldActor &&
                              &mut (*player).actor as *mut Actor !=
                                  (*actor).parent {
                CollisionCheck_ResetDamage(&mut (*actor).colChkInfo);
                actor = (*actor).next
            } else if (*actor).update.is_none() {
                if (*actor).isDrawn == 0 {
                    actor =
                        Actor_Delete(&mut (*globalCtx).actorCtx, actor,
                                     globalCtx)
                } else {
                    Actor_Destroy(actor, globalCtx);
                    actor = (*actor).next
                }
            } else {
                Math_Vec3f_Copy(&mut (*actor).prevPos,
                                &mut (*actor).world.pos);
                (*actor).xzDistToPlayer =
                    Actor_WorldDistXZToActor(actor, &mut (*player).actor);
                (*actor).yDistToPlayer =
                    Actor_HeightDiff(actor, &mut (*player).actor);
                (*actor).xyzDistToPlayerSq =
                    (*actor).xzDistToPlayer * (*actor).xzDistToPlayer +
                        (*actor).yDistToPlayer * (*actor).yDistToPlayer;
                (*actor).yawTowardsPlayer =
                    Actor_WorldYawTowardActor(actor, &mut (*player).actor);
                (*actor).flags &=
                    !((1 as libc::c_int) << 24 as libc::c_int) as
                        libc::c_uint;
                if (if (*actor).freezeTimer as libc::c_int == 0 as libc::c_int
                       {
                        0 as libc::c_int
                    } else {
                        (*actor).freezeTimer =
                            (*actor).freezeTimer.wrapping_sub(1);
                        (*actor).freezeTimer as libc::c_int
                    }) == 0 as libc::c_int &&
                       (*actor).flags &
                           ((1 as libc::c_int) << 4 as libc::c_int |
                                (1 as libc::c_int) << 6 as libc::c_int) as
                               libc::c_uint != 0 {
                    if actor == (*player).unk_664 {
                        (*actor).isTargeted = 1 as libc::c_int as u8_0
                    } else { (*actor).isTargeted = 0 as libc::c_int as u8_0 }
                    if (*actor).targetPriority as libc::c_int !=
                           0 as libc::c_int && (*player).unk_664.is_null() {
                        (*actor).targetPriority = 0 as libc::c_int as u8_0
                    }
                    Actor_SetObjectDependency(globalCtx, actor);
                    if (*actor).colorFilterTimer as libc::c_int !=
                           0 as libc::c_int {
                        (*actor).colorFilterTimer =
                            (*actor).colorFilterTimer.wrapping_sub(1)
                    }
                    (*actor).update.expect("non-null function pointer")(actor,
                                                                        globalCtx);
                    func_8003F8EC(globalCtx, &mut (*globalCtx).colCtx.dyna,
                                  actor);
                }
                CollisionCheck_ResetDamage(&mut (*actor).colChkInfo);
                actor = (*actor).next
            }
        }
        if i == ACTORCAT_BG as libc::c_int {
            DynaPoly_Setup(globalCtx, &mut (*globalCtx).colCtx.dyna);
        }
        i += 1;
        sp80 = sp80.offset(1)
    }
    actor = (*player).unk_664;
    if !actor.is_null() && (*actor).update.is_none() {
        actor = 0 as *mut Actor;
        func_8008EDF0(player);
    }
    if actor.is_null() || (*player).unk_66C < 5 as libc::c_int {
        actor = 0 as *mut Actor;
        if (*actorCtx).targetCtx.unk_4B as libc::c_int != 0 as libc::c_int {
            (*actorCtx).targetCtx.unk_4B = 0 as libc::c_int as u8_0;
            func_80078884(0x480f as libc::c_int as u16_0);
        }
    }
    func_8002C7BC(&mut (*actorCtx).targetCtx, player, actor, globalCtx);
    TitleCard_Update(globalCtx, &mut (*actorCtx).titleCtx);
    DynaPoly_UpdateBgActorTransforms(globalCtx,
                                     &mut (*globalCtx).colCtx.dyna);
}
#[no_mangle]
pub unsafe extern "C" fn Actor_FaultPrint(mut actor: *mut Actor,
                                          mut command: *mut libc::c_char) {
    let mut overlayEntry: *mut ActorOverlay = 0 as *mut ActorOverlay;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    if actor.is_null() || (*actor).overlayEntry.is_null() {
        FaultDrawer_SetCursor(48 as libc::c_int, 24 as libc::c_int);
        FaultDrawer_Printf(b"ACTOR NAME is NULL\x00" as *const u8 as
                               *const libc::c_char);
    }
    overlayEntry = (*actor).overlayEntry;
    name =
        if !(*overlayEntry).name.is_null() {
            (*overlayEntry).name as *const libc::c_char
        } else { b"\x00" as *const u8 as *const libc::c_char } as
            *mut libc::c_char;
    osSyncPrintf(b"\xe3\x82\xa2\xe3\x82\xaf\xe3\x82\xbf\xe3\x83\xbc\xe3\x81\xae\xe5\x90\x8d\xe5\x89\x8d(%08x:%s)\n\x00"
                     as *const u8 as *const libc::c_char, actor, name);
    if !command.is_null() {
        osSyncPrintf(b"\xe3\x82\xb3\xe3\x83\xa1\xe3\x83\xb3\xe3\x83\x88:%s\n\x00"
                         as *const u8 as *const libc::c_char, command);
        // "Command:%s"
    } // "Magic lens START"
    FaultDrawer_SetCursor(48 as libc::c_int, 24 as libc::c_int);
    FaultDrawer_Printf(b"ACTOR NAME %08x:%s\x00" as *const u8 as
                           *const libc::c_char, actor, name);
}
#[no_mangle]
pub unsafe extern "C" fn Actor_Draw(mut globalCtx: *mut GlobalContext,
                                    mut actor: *mut Actor) {
    let mut faultClient: FaultClient =
        FaultClient{next: 0 as *mut FaultClient,
                    callback: 0,
                    param1: 0,
                    param2: 0,};
    let mut lights: *mut Lights = 0 as *mut Lights;
    Fault_AddClient(&mut faultClient,
                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                            *mut Actor,
                                                                        _:
                                                                            *mut libc::c_char)
                                                       -> ()>,
                                            *mut libc::c_void>(Some(Actor_FaultPrint
                                                                        as
                                                                        unsafe extern "C" fn(_:
                                                                                                 *mut Actor,
                                                                                             _:
                                                                                                 *mut libc::c_char)
                                                                            ->
                                                                                ())),
                    actor as *mut libc::c_void,
                    b"Actor_draw\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_void);
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_actor.c\x00" as *const u8 as *const libc::c_char,
                    6035 as libc::c_int);
    lights =
        LightContext_NewLights(&mut (*globalCtx).lightCtx,
                               (*globalCtx).state.gfxCtx);
    Lights_BindAll(lights, (*globalCtx).lightCtx.listHead,
                   if (*actor).flags &
                          ((1 as libc::c_int) << 22 as libc::c_int) as
                              libc::c_uint != 0 {
                       0 as *mut Vec3f
                   } else { &mut (*actor).world.pos });
    Lights_Draw(lights, (*globalCtx).state.gfxCtx);
    if (*actor).flags &
           ((1 as libc::c_int) << 12 as libc::c_int) as libc::c_uint != 0 {
        func_800D1694((*actor).world.pos.x +
                          (*globalCtx).mainCamera.skyboxOffset.x,
                      (*actor).world.pos.y +
                          ((*actor).shape.yOffset * (*actor).scale.y +
                               (*globalCtx).mainCamera.skyboxOffset.y),
                      (*actor).world.pos.z +
                          (*globalCtx).mainCamera.skyboxOffset.z,
                      &mut (*actor).shape.rot);
    } else {
        func_800D1694((*actor).world.pos.x,
                      (*actor).world.pos.y +
                          (*actor).shape.yOffset * (*actor).scale.y,
                      (*actor).world.pos.z, &mut (*actor).shape.rot);
    }
    Matrix_Scale((*actor).scale.x, (*actor).scale.y, (*actor).scale.z,
                 MTXMODE_APPLY as libc::c_int as u8_0);
    Actor_SetObjectDependency(globalCtx, actor);
    let fresh50 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g: *mut Gfx = fresh50;
    (*_g).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((0x6 as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g).words.w1 =
        (*globalCtx).objectCtx.status[(*actor).objBankIndex as usize].segment
            as libc::c_uint;
    let fresh51 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_0: *mut Gfx = fresh51;
    (*_g_0).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((0x6 as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_0).words.w1 =
        (*globalCtx).objectCtx.status[(*actor).objBankIndex as usize].segment
            as libc::c_uint;
    if (*actor).colorFilterTimer as libc::c_int != 0 as libc::c_int {
        let mut color: Color_RGBA8 =
            {
                let mut init =
                    Color_RGBA8{r: 0 as libc::c_int as u8_0,
                                g: 0 as libc::c_int as u8_0,
                                b: 0 as libc::c_int as u8_0,
                                a: 255 as libc::c_int as u8_0,};
                init
            };
        if (*actor).colorFilterParams as libc::c_int & 0x8000 as libc::c_int
               != 0 {
            color.b =
                (((*actor).colorFilterParams as libc::c_int &
                      0x1f00 as libc::c_int) >> 5 as libc::c_int |
                     7 as libc::c_int) as u8_0;
            color.g = color.b;
            color.r = color.g
        } else if (*actor).colorFilterParams as libc::c_int &
                      0x4000 as libc::c_int != 0 {
            color.r =
                (((*actor).colorFilterParams as libc::c_int &
                      0x1f00 as libc::c_int) >> 5 as libc::c_int |
                     7 as libc::c_int) as u8_0
        } else {
            color.b =
                (((*actor).colorFilterParams as libc::c_int &
                      0x1f00 as libc::c_int) >> 5 as libc::c_int |
                     7 as libc::c_int) as u8_0
        }
        if (*actor).colorFilterParams as libc::c_int & 0x2000 as libc::c_int
               != 0 {
            func_80026860(globalCtx, &mut color,
                          (*actor).colorFilterTimer as s16,
                          ((*actor).colorFilterParams as libc::c_int &
                               0xff as libc::c_int) as s16);
        } else {
            func_80026400(globalCtx, &mut color,
                          (*actor).colorFilterTimer as s16,
                          ((*actor).colorFilterParams as libc::c_int &
                               0xff as libc::c_int) as s16);
        }
    }
    (*actor).draw.expect("non-null function pointer")(actor, globalCtx);
    if (*actor).colorFilterTimer as libc::c_int != 0 as libc::c_int {
        if (*actor).colorFilterParams as libc::c_int & 0x2000 as libc::c_int
               != 0 {
            func_80026A6C(globalCtx);
        } else { func_80026608(globalCtx); }
    }
    if (*actor).shape.shadowDraw.is_some() {
        (*actor).shape.shadowDraw.expect("non-null function pointer")(actor,
                                                                      lights,
                                                                      globalCtx);
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_actor.c\x00" as *const u8 as *const libc::c_char,
                     6119 as libc::c_int);
    Fault_RemoveClient(&mut faultClient);
}
#[no_mangle]
pub unsafe extern "C" fn func_80030ED8(mut actor: *mut Actor) {
    if (*actor).flags &
           ((1 as libc::c_int) << 19 as libc::c_int) as libc::c_uint != 0 {
        Audio_PlaySoundGeneral((*actor).sfx, &mut (*actor).projectedPos,
                               4 as libc::c_int as u8_0, &mut D_801333E0,
                               &mut D_801333E0, &mut D_801333E8);
    } else if (*actor).flags &
                  ((1 as libc::c_int) << 20 as libc::c_int) as libc::c_uint !=
                  0 {
        func_80078884((*actor).sfx);
    } else if (*actor).flags &
                  ((1 as libc::c_int) << 21 as libc::c_int) as libc::c_uint !=
                  0 {
        func_800788CC((*actor).sfx);
    } else if (*actor).flags &
                  ((1 as libc::c_int) << 28 as libc::c_int) as libc::c_uint !=
                  0 {
        func_800F4C58(&mut D_801333D4,
                      (0x2821 as libc::c_int - 0x800 as libc::c_int) as u16_0,
                      ((*actor).sfx as libc::c_int - 1 as libc::c_int) as s8
                          as u8_0);
    } else { func_80078914(&mut (*actor).projectedPos, (*actor).sfx); };
}
#[no_mangle]
pub unsafe extern "C" fn func_80030FA8(mut gfxCtx: *mut GraphicsContext) {
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), gfxCtx,
                    b"../z_actor.c\x00" as *const u8 as *const libc::c_char,
                    6161 as libc::c_int);
    let fresh52 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g: *mut Gfx = fresh52;
    (*_g).words.w0 =
        (0xfd as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (4 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 21 as libc::c_int
            |
            (2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 19 as libc::c_int
            |
            ((1 as libc::c_int - 1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g).words.w1 = gLensOfTruthMaskTex.as_mut_ptr() as libc::c_uint;
    let fresh53 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_0: *mut Gfx = fresh53;
    (*_g_0).words.w0 =
        (0xf5 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (4 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 21 as libc::c_int
            |
            (2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 19 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 9 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 9 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 9 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_0).words.w1 =
        (7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 3 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 20 as libc::c_int
            |
            ((0x1 as libc::c_int | 0x2 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 18 as libc::c_int
            |
            (6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 14 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 10 as libc::c_int
            |
            ((0x1 as libc::c_int | 0x2 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 4 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh54 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_1: *mut Gfx = fresh54;
    (*_g_1).words.w0 =
        (0xe6 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_1).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh55 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_2: *mut Gfx = fresh55;
    (*_g_2).words.w0 =
        (0xf3 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_2).words.w1 =
        (7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 3 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((if ((64 as libc::c_int * 64 as libc::c_int + 1 as libc::c_int >>
                       1 as libc::c_int) - 1 as libc::c_int) <
                     2047 as libc::c_int {
                  (64 as libc::c_int * 64 as libc::c_int + 1 as libc::c_int >>
                       1 as libc::c_int) - 1 as libc::c_int
              } else { 2047 as libc::c_int }) as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            (((((1 as libc::c_int) << 11 as libc::c_int) +
                   (if 1 as libc::c_int >
                           64 as libc::c_int * 1 as libc::c_int /
                               8 as libc::c_int {
                        1 as libc::c_int
                    } else {
                        (64 as libc::c_int * 1 as libc::c_int) /
                            8 as libc::c_int
                    }) - 1 as libc::c_int) /
                  (if 1 as libc::c_int >
                          64 as libc::c_int * 1 as libc::c_int /
                              8 as libc::c_int {
                       1 as libc::c_int
                   } else {
                       (64 as libc::c_int * 1 as libc::c_int) /
                           8 as libc::c_int
                   })) as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh56 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_3: *mut Gfx = fresh56;
    (*_g_3).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_3).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh57 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_4: *mut Gfx = fresh57;
    (*_g_4).words.w0 =
        (0xf5 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (4 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 21 as libc::c_int
            |
            (1 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 19 as libc::c_int
            |
            ((64 as libc::c_int * 1 as libc::c_int + 7 as libc::c_int >>
                  3 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 9 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 9 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 9 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_4).words.w1 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 3 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 20 as libc::c_int
            |
            ((0x1 as libc::c_int | 0x2 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 18 as libc::c_int
            |
            (6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 14 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 10 as libc::c_int
            |
            ((0x1 as libc::c_int | 0x2 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 4 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh58 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_5: *mut Gfx = fresh58;
    (*_g_5).words.w0 =
        (0xf2 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_5).words.w1 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 3 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (((64 as libc::c_int - 1 as libc::c_int) << 2 as libc::c_int) as
                 u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            (((64 as libc::c_int - 1 as libc::c_int) << 2 as libc::c_int) as
                 u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh59 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_6: *mut Gfx = fresh59;
    (*_g_6).words.w0 =
        (0xf2 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (384 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            (224 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_6).words.w1 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 3 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (892 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            (732 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh60 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_7: *mut Gfx = fresh60;
    (*_g_7).words.w0 =
        (0xe4 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (1280 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            (960 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_7).words.w1 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 3 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh61 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_8: *mut Gfx = fresh61;
    (*_g_8).words.w0 =
        (0xe1 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_8).words.w1 =
        (2240 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 16 as libc::c_int |
            (1600 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh62 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_9: *mut Gfx = fresh62;
    (*_g_9).words.w0 =
        (0xf1 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_9).words.w1 =
        (576 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 16 as libc::c_int |
            (597 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh63 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_10: *mut Gfx = fresh63;
    (*_g_10).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_10).words.w1 = 0 as libc::c_int as libc::c_uint;
    Graph_CloseDisps(dispRefs.as_mut_ptr(), gfxCtx,
                     b"../z_actor.c\x00" as *const u8 as *const libc::c_char,
                     6183 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn func_8003115C(mut globalCtx: *mut GlobalContext,
                                       mut numInvisibleActors: s32,
                                       mut invisibleActors: *mut *mut Actor) {
    let mut invisibleActor: *mut *mut Actor = 0 as *mut *mut Actor;
    let mut gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut i: s32 = 0;
    gfxCtx = (*globalCtx).state.gfxCtx;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), gfxCtx,
                    b"../z_actor.c\x00" as *const u8 as *const libc::c_char,
                    6197 as libc::c_int);
    let fresh64 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g: *mut Gfx = fresh64;
    (*_g).words.w0 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g).words.w1 =
        b"\xe9\xad\x94\xe6\xb3\x95\xe3\x81\xae\xe3\x83\xa1\xe3\x82\xac\xe3\x83\x8d START\x00"
            as *const u8 as *const libc::c_char as libc::c_uint;
    let fresh65 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_0: *mut Gfx = fresh65;
    (*_g_0).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_0).words.w1 = 0 as libc::c_int as libc::c_uint;
    if (*globalCtx).roomCtx.curRoom.showInvisActors as libc::c_int ==
           0 as libc::c_int {
        let fresh66 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_1: *mut Gfx = fresh66;
        (*_g_1).words.w0 =
            (0xef as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (((3 as libc::c_int) << 4 as libc::c_int |
                      (0 as libc::c_int) << 6 as libc::c_int |
                      (0 as libc::c_int) << 8 as libc::c_int |
                      (6 as libc::c_int) << 9 as libc::c_int |
                      (2 as libc::c_int) << 12 as libc::c_int |
                      (0 as libc::c_int) << 14 as libc::c_int |
                      (0 as libc::c_int) << 16 as libc::c_int |
                      (0 as libc::c_int) << 17 as libc::c_int |
                      (0 as libc::c_int) << 19 as libc::c_int |
                      (0 as libc::c_int) << 20 as libc::c_int |
                      (0 as libc::c_int) << 23 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 24 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_1).words.w1 =
            ((1 as libc::c_int) << 0 as libc::c_int |
                 (1 as libc::c_int) << 2 as libc::c_int | 0x20 as libc::c_int
                 | 0x40 as libc::c_int | 0x300 as libc::c_int |
                 0x4000 as libc::c_int | 0 as libc::c_int |
                 (0 as libc::c_int) << 30 as libc::c_int |
                 (0 as libc::c_int) << 26 as libc::c_int |
                 (1 as libc::c_int) << 22 as libc::c_int |
                 (0 as libc::c_int) << 18 as libc::c_int | 0x40 as libc::c_int
                 | 0x300 as libc::c_int | 0x4000 as libc::c_int |
                 0 as libc::c_int | (0 as libc::c_int) << 28 as libc::c_int |
                 (0 as libc::c_int) << 24 as libc::c_int |
                 (1 as libc::c_int) << 20 as libc::c_int |
                 (0 as libc::c_int) << 16 as libc::c_int) as libc::c_uint;
        let fresh67 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_2: *mut Gfx = fresh67;
        (*_g_2).words.w0 =
            (0xfc as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (((1 as libc::c_int as u32_0 &
                       (((0x1 as libc::c_int) << 4 as libc::c_int) -
                            1 as libc::c_int) as libc::c_uint) <<
                      20 as libc::c_int |
                      (3 as libc::c_int as u32_0 &
                           (((0x1 as libc::c_int) << 5 as libc::c_int) -
                                1 as libc::c_int) as libc::c_uint) <<
                          15 as libc::c_int |
                      (1 as libc::c_int as u32_0 &
                           (((0x1 as libc::c_int) << 3 as libc::c_int) -
                                1 as libc::c_int) as libc::c_uint) <<
                          12 as libc::c_int |
                      (3 as libc::c_int as u32_0 &
                           (((0x1 as libc::c_int) << 3 as libc::c_int) -
                                1 as libc::c_int) as libc::c_uint) <<
                          9 as libc::c_int |
                      ((1 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 4 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           5 as libc::c_int |
                           (3 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 5 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int)) &
                     (((0x1 as libc::c_int) << 24 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_2).words.w1 =
            (31 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 28 as libc::c_int
                |
                (31 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
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
                     24 as libc::c_int |
                     (1 as libc::c_int as u32_0 &
                          (((0x1 as libc::c_int) << 3 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         21 as libc::c_int |
                     (3 as libc::c_int as u32_0 &
                          (((0x1 as libc::c_int) << 3 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         18 as libc::c_int |
                     (31 as libc::c_int as u32_0 &
                          (((0x1 as libc::c_int) << 3 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         6 as libc::c_int |
                     (7 as libc::c_int as u32_0 &
                          (((0x1 as libc::c_int) << 3 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         3 as libc::c_int |
                     (7 as libc::c_int as u32_0 &
                          (((0x1 as libc::c_int) << 3 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         0 as libc::c_int);
        let fresh68 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_3: *mut Gfx = fresh68;
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
                    0 as libc::c_int
    } else {
        let fresh69 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_4: *mut Gfx = fresh69;
        (*_g_4).words.w0 =
            (0xef as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (((3 as libc::c_int) << 4 as libc::c_int |
                      (0 as libc::c_int) << 6 as libc::c_int |
                      (0 as libc::c_int) << 8 as libc::c_int |
                      (6 as libc::c_int) << 9 as libc::c_int |
                      (2 as libc::c_int) << 12 as libc::c_int |
                      (0 as libc::c_int) << 14 as libc::c_int |
                      (0 as libc::c_int) << 16 as libc::c_int |
                      (0 as libc::c_int) << 17 as libc::c_int |
                      (0 as libc::c_int) << 19 as libc::c_int |
                      (0 as libc::c_int) << 20 as libc::c_int |
                      (0 as libc::c_int) << 23 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 24 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_4).words.w1 =
            ((1 as libc::c_int) << 0 as libc::c_int |
                 (1 as libc::c_int) << 2 as libc::c_int | 0x20 as libc::c_int
                 | 0x40 as libc::c_int | 0x300 as libc::c_int |
                 0 as libc::c_int | 0x4000 as libc::c_int |
                 (2 as libc::c_int) << 30 as libc::c_int |
                 (3 as libc::c_int) << 26 as libc::c_int |
                 (1 as libc::c_int) << 22 as libc::c_int |
                 (0 as libc::c_int) << 18 as libc::c_int |
                 (2 as libc::c_int) << 28 as libc::c_int |
                 (3 as libc::c_int) << 24 as libc::c_int |
                 (1 as libc::c_int) << 20 as libc::c_int |
                 (0 as libc::c_int) << 16 as libc::c_int) as libc::c_uint;
        let fresh70 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_5: *mut Gfx = fresh70;
        (*_g_5).words.w0 =
            (0xfc as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (((3 as libc::c_int as u32_0 &
                       (((0x1 as libc::c_int) << 4 as libc::c_int) -
                            1 as libc::c_int) as libc::c_uint) <<
                      20 as libc::c_int |
                      (14 as libc::c_int as u32_0 &
                           (((0x1 as libc::c_int) << 5 as libc::c_int) -
                                1 as libc::c_int) as libc::c_uint) <<
                          15 as libc::c_int |
                      (3 as libc::c_int as u32_0 &
                           (((0x1 as libc::c_int) << 3 as libc::c_int) -
                                1 as libc::c_int) as libc::c_uint) <<
                          12 as libc::c_int |
                      (6 as libc::c_int as u32_0 &
                           (((0x1 as libc::c_int) << 3 as libc::c_int) -
                                1 as libc::c_int) as libc::c_uint) <<
                          9 as libc::c_int |
                      ((3 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 4 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           5 as libc::c_int |
                           (14 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 5 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int)) &
                     (((0x1 as libc::c_int) << 24 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_5).words.w1 =
            (1 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 28 as libc::c_int
                |
                (31 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    15 as libc::c_int |
                (1 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    12 as libc::c_int |
                (7 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    9 as libc::c_int |
                ((1 as libc::c_int as u32_0 &
                      (((0x1 as libc::c_int) << 4 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     24 as libc::c_int |
                     (3 as libc::c_int as u32_0 &
                          (((0x1 as libc::c_int) << 3 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         21 as libc::c_int |
                     (6 as libc::c_int as u32_0 &
                          (((0x1 as libc::c_int) << 3 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         18 as libc::c_int |
                     (31 as libc::c_int as u32_0 &
                          (((0x1 as libc::c_int) << 3 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         6 as libc::c_int |
                     (1 as libc::c_int as u32_0 &
                          (((0x1 as libc::c_int) << 3 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         3 as libc::c_int |
                     (7 as libc::c_int as u32_0 &
                          (((0x1 as libc::c_int) << 3 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         0 as libc::c_int);
        let fresh71 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_6: *mut Gfx = fresh71;
        (*_g_6).words.w0 =
            (0xfa as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (0xff as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_6).words.w1 =
            (74 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (74 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                (74 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (74 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int
    }
    let fresh72 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_7: *mut Gfx = fresh72;
    (*_g_7).words.w0 =
        (0xee as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_7).words.w1 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 16 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    func_80030FA8(gfxCtx);
    // "Magic lens invisible Actor display START"
    let fresh73 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_8: *mut Gfx = fresh73;
    (*_g_8).words.w0 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (numInvisibleActors as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_8).words.w1 =
        b"\xe9\xad\x94\xe6\xb3\x95\xe3\x81\xae\xe3\x83\xa1\xe3\x82\xac\xe3\x83\x8d \xe8\xa6\x8b\xe3\x81\x88\xe3\x81\xaa\xe3\x81\x84\xef\xbc\xa1c\xef\xbd\x94\xef\xbd\x8f\xef\xbd\x92\xe8\xa1\xa8\xe7\xa4\xba START\x00"
            as *const u8 as *const libc::c_char as libc::c_uint;
    invisibleActor =
        &mut *invisibleActors.offset(0 as libc::c_int as isize) as
            *mut *mut Actor;
    i = 0 as libc::c_int;
    while i < numInvisibleActors {
        // "Magic lens invisible Actor display"
        let fresh74 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_9: *mut Gfx = fresh74;
        (*_g_9).words.w0 =
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (2 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                (i as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_9).words.w1 =
            b"\xe9\xad\x94\xe6\xb3\x95\xe3\x81\xae\xe3\x83\xa1\xe3\x82\xac\xe3\x83\x8d \xe8\xa6\x8b\xe3\x81\x88\xe3\x81\xaa\xe3\x81\x84\xef\xbc\xa1c\xef\xbd\x94\xef\xbd\x8f\xef\xbd\x92\xe8\xa1\xa8\xe7\xa4\xba\x00"
                as *const u8 as *const libc::c_char as libc::c_uint;
        let fresh75 = invisibleActor;
        invisibleActor = invisibleActor.offset(1);
        Actor_Draw(globalCtx, *fresh75);
        i += 1
    }
    // "Magic lens invisible Actor display END"
    let fresh76 = (*__gfxCtx).polyOpa.p; // "Blue spectacles (exterior)"
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_10: *mut Gfx = fresh76;
    (*_g_10).words.w0 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (numInvisibleActors as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_10).words.w1 =
        b"\xe9\xad\x94\xe6\xb3\x95\xe3\x81\xae\xe3\x83\xa1\xe3\x82\xac\xe3\x83\x8d \xe8\xa6\x8b\xe3\x81\x88\xe3\x81\xaa\xe3\x81\x84\xef\xbc\xa1c\xef\xbd\x94\xef\xbd\x8f\xef\xbd\x92\xe8\xa1\xa8\xe7\xa4\xba END\x00"
            as *const u8 as *const libc::c_char as libc::c_uint;
    if (*globalCtx).roomCtx.curRoom.showInvisActors as libc::c_int !=
           0 as libc::c_int {
        let fresh77 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_11: *mut Gfx = fresh77;
        (*_g_11).words.w0 =
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (2 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_11).words.w1 =
            b"\xe9\x9d\x92\xe3\x81\x84\xe7\x9c\xbc\xe9\x8f\xa1(\xe5\xa4\x96\xe5\x81\xb4)\x00"
                as *const u8 as *const libc::c_char as libc::c_uint;
        let fresh78 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_12: *mut Gfx = fresh78;
        (*_g_12).words.w0 =
            (0xe7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_12).words.w1 = 0 as libc::c_int as libc::c_uint;
        let fresh79 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_13: *mut Gfx = fresh79;
        (*_g_13).words.w0 =
            (0xef as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (((3 as libc::c_int) << 4 as libc::c_int |
                      (0 as libc::c_int) << 6 as libc::c_int |
                      (0 as libc::c_int) << 8 as libc::c_int |
                      (6 as libc::c_int) << 9 as libc::c_int |
                      (2 as libc::c_int) << 12 as libc::c_int |
                      (0 as libc::c_int) << 14 as libc::c_int |
                      (0 as libc::c_int) << 16 as libc::c_int |
                      (0 as libc::c_int) << 17 as libc::c_int |
                      (0 as libc::c_int) << 19 as libc::c_int |
                      (0 as libc::c_int) << 20 as libc::c_int |
                      (0 as libc::c_int) << 23 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 24 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_13).words.w1 =
            ((1 as libc::c_int) << 0 as libc::c_int |
                 (1 as libc::c_int) << 2 as libc::c_int | 0x40 as libc::c_int
                 | 0x300 as libc::c_int | 0x4000 as libc::c_int |
                 0 as libc::c_int | (0 as libc::c_int) << 30 as libc::c_int |
                 (0 as libc::c_int) << 26 as libc::c_int |
                 (1 as libc::c_int) << 22 as libc::c_int |
                 (0 as libc::c_int) << 18 as libc::c_int | 0x40 as libc::c_int
                 | 0x300 as libc::c_int | 0x4000 as libc::c_int |
                 0 as libc::c_int | (0 as libc::c_int) << 28 as libc::c_int |
                 (0 as libc::c_int) << 24 as libc::c_int |
                 (1 as libc::c_int) << 20 as libc::c_int |
                 (0 as libc::c_int) << 16 as libc::c_int) as libc::c_uint;
        let fresh80 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_14: *mut Gfx = fresh80;
        (*_g_14).words.w0 =
            (0xfc as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (((1 as libc::c_int as u32_0 &
                       (((0x1 as libc::c_int) << 4 as libc::c_int) -
                            1 as libc::c_int) as libc::c_uint) <<
                      20 as libc::c_int |
                      (3 as libc::c_int as u32_0 &
                           (((0x1 as libc::c_int) << 5 as libc::c_int) -
                                1 as libc::c_int) as libc::c_uint) <<
                          15 as libc::c_int |
                      (1 as libc::c_int as u32_0 &
                           (((0x1 as libc::c_int) << 3 as libc::c_int) -
                                1 as libc::c_int) as libc::c_uint) <<
                          12 as libc::c_int |
                      (3 as libc::c_int as u32_0 &
                           (((0x1 as libc::c_int) << 3 as libc::c_int) -
                                1 as libc::c_int) as libc::c_uint) <<
                          9 as libc::c_int |
                      ((1 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 4 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           5 as libc::c_int |
                           (3 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 5 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int)) &
                     (((0x1 as libc::c_int) << 24 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_14).words.w1 =
            (31 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 4 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 28 as libc::c_int
                |
                (31 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 3 as libc::c_int) -
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
                     24 as libc::c_int |
                     (1 as libc::c_int as u32_0 &
                          (((0x1 as libc::c_int) << 3 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         21 as libc::c_int |
                     (3 as libc::c_int as u32_0 &
                          (((0x1 as libc::c_int) << 3 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         18 as libc::c_int |
                     (31 as libc::c_int as u32_0 &
                          (((0x1 as libc::c_int) << 3 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         6 as libc::c_int |
                     (7 as libc::c_int as u32_0 &
                          (((0x1 as libc::c_int) << 3 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         3 as libc::c_int |
                     (7 as libc::c_int as u32_0 &
                          (((0x1 as libc::c_int) << 3 as libc::c_int) -
                               1 as libc::c_int) as libc::c_uint) <<
                         0 as libc::c_int);
        let fresh81 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_15: *mut Gfx = fresh81;
        (*_g_15).words.w0 =
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
        (*_g_15).words.w1 =
            (255 as libc::c_int as u32_0 &
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
        func_80030FA8(gfxCtx);
        let fresh82 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_16: *mut Gfx = fresh82;
        (*_g_16).words.w0 =
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (2 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                (1 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_16).words.w1 =
            b"\xe9\x9d\x92\xe3\x81\x84\xe7\x9c\xbc\xe9\x8f\xa1(\xe5\xa4\x96\xe5\x81\xb4)\x00"
                as *const u8 as *const libc::c_char as libc::c_uint
        // "Blue spectacles (exterior)"
    } // "Magic lens END"
    let fresh83 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_17: *mut Gfx = fresh83;
    (*_g_17).words.w0 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_17).words.w1 =
        b"\xe9\xad\x94\xe6\xb3\x95\xe3\x81\xae\xe3\x83\xa1\xe3\x82\xac\xe3\x83\x8d END\x00"
            as *const u8 as *const libc::c_char as libc::c_uint;
    Graph_CloseDisps(dispRefs.as_mut_ptr(), gfxCtx,
                     b"../z_actor.c\x00" as *const u8 as *const libc::c_char,
                     6284 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn func_800314B0(mut globalCtx: *mut GlobalContext,
                                       mut actor: *mut Actor) -> s32 {
    return func_800314D4(globalCtx, actor, &mut (*actor).projectedPos,
                         (*actor).projectedW);
}
#[no_mangle]
pub unsafe extern "C" fn func_800314D4(mut globalCtx: *mut GlobalContext,
                                       mut actor: *mut Actor,
                                       mut arg2: *mut Vec3f, mut arg3: f32_0)
 -> s32 {
    let mut var: f32_0 = 0.;
    if (*arg2).z > -(*actor).uncullZoneScale &&
           (*arg2).z < (*actor).uncullZoneForward + (*actor).uncullZoneScale {
        var = if arg3 < 1.0f32 { 1.0f32 } else { (1.0f32) / arg3 };
        if (fabsf((*arg2).x) - (*actor).uncullZoneScale) * var < 1.0f32 &&
               ((*arg2).y + (*actor).uncullZoneDownward) * var > -1.0f32 &&
               ((*arg2).y - (*actor).uncullZoneScale) * var < 1.0f32 {
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_800315AC(mut globalCtx: *mut GlobalContext,
                                       mut actorCtx: *mut ActorContext) {
    let mut invisibleActorCounter: s32 = 0;
    let mut invisibleActors: [*mut Actor; 20] = [0 as *mut Actor; 20];
    let mut actorListEntry: *mut ActorListEntry = 0 as *mut ActorListEntry;
    let mut actor: *mut Actor = 0 as *mut Actor;
    let mut i: s32 = 0;
    invisibleActorCounter = 0 as libc::c_int;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_actor.c\x00" as *const u8 as *const libc::c_char,
                    6336 as libc::c_int);
    actorListEntry =
        &mut *(*actorCtx).actorLists.as_mut_ptr().offset(0 as libc::c_int as
                                                             isize) as
            *mut ActorListEntry;
    i = 0 as libc::c_int;
    while i <
              (::std::mem::size_of::<[ActorListEntry; 12]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<ActorListEntry>()
                                                   as libc::c_ulong) as s32 {
        actor = (*actorListEntry).head;
        while !actor.is_null() {
            let mut overlayEntry: *mut ActorOverlay = (*actor).overlayEntry;
            let mut actorName: *mut libc::c_char =
                if !(*overlayEntry).name.is_null() {
                    (*overlayEntry).name as *const libc::c_char
                } else { b"\x00" as *const u8 as *const libc::c_char } as
                    *mut libc::c_char;
            let fresh84 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g: *mut Gfx = fresh84;
            (*_g).words.w0 =
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (2 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (i as u32_0 &
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g).words.w1 = actorName as libc::c_uint;
            let fresh85 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_0: *mut Gfx = fresh85;
            (*_g_0).words.w0 =
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (2 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (i as u32_0 &
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_0).words.w1 = actorName as libc::c_uint;
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 66 as libc::c_int) as
                                  usize] = i as s16;
            if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                      16 as libc::c_int + 64 as libc::c_int)
                                     as usize] as libc::c_int !=
                   1 as libc::c_int ||
                   (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          65 as libc::c_int) as usize] as
                       libc::c_int != -(1 as libc::c_int) &&
                       (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int
                                              * 16 as libc::c_int +
                                              65 as libc::c_int) as usize] as
                           libc::c_int !=
                           (*gGameInfo).data[(21 as libc::c_int *
                                                  6 as libc::c_int *
                                                  16 as libc::c_int +
                                                  66 as libc::c_int) as usize]
                               as libc::c_int ||
                   (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          68 as libc::c_int) as usize] as
                       libc::c_int == 0 as libc::c_int {
                SkinMatrix_Vec3fMtxFMultXYZW(&mut (*globalCtx).viewProjectionMtxF,
                                             &mut (*actor).world.pos,
                                             &mut (*actor).projectedPos,
                                             &mut (*actor).projectedW);
            }
            if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                      16 as libc::c_int + 64 as libc::c_int)
                                     as usize] as libc::c_int !=
                   1 as libc::c_int ||
                   (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          65 as libc::c_int) as usize] as
                       libc::c_int != -(1 as libc::c_int) &&
                       (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int
                                              * 16 as libc::c_int +
                                              65 as libc::c_int) as usize] as
                           libc::c_int !=
                           (*gGameInfo).data[(21 as libc::c_int *
                                                  6 as libc::c_int *
                                                  16 as libc::c_int +
                                                  66 as libc::c_int) as usize]
                               as libc::c_int ||
                   (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          69 as libc::c_int) as usize] as
                       libc::c_int == 0 as libc::c_int {
                if (*actor).sfx as libc::c_int != 0 as libc::c_int {
                    func_80030ED8(actor);
                }
            }
            if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                      16 as libc::c_int + 64 as libc::c_int)
                                     as usize] as libc::c_int !=
                   1 as libc::c_int ||
                   (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          65 as libc::c_int) as usize] as
                       libc::c_int != -(1 as libc::c_int) &&
                       (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int
                                              * 16 as libc::c_int +
                                              65 as libc::c_int) as usize] as
                           libc::c_int !=
                           (*gGameInfo).data[(21 as libc::c_int *
                                                  6 as libc::c_int *
                                                  16 as libc::c_int +
                                                  66 as libc::c_int) as usize]
                               as libc::c_int ||
                   (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          70 as libc::c_int) as usize] as
                       libc::c_int == 0 as libc::c_int {
                if func_800314B0(globalCtx, actor) != 0 {
                    (*actor).flags |=
                        ((1 as libc::c_int) << 6 as libc::c_int) as
                            libc::c_uint
                } else {
                    (*actor).flags &=
                        !((1 as libc::c_int) << 6 as libc::c_int) as
                            libc::c_uint
                }
            }
            (*actor).isDrawn = 0 as libc::c_int as u8_0;
            if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                      16 as libc::c_int + 64 as libc::c_int)
                                     as usize] as libc::c_int !=
                   1 as libc::c_int ||
                   (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          65 as libc::c_int) as usize] as
                       libc::c_int != -(1 as libc::c_int) &&
                       (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int
                                              * 16 as libc::c_int +
                                              65 as libc::c_int) as usize] as
                           libc::c_int !=
                           (*gGameInfo).data[(21 as libc::c_int *
                                                  6 as libc::c_int *
                                                  16 as libc::c_int +
                                                  66 as libc::c_int) as usize]
                               as libc::c_int ||
                   (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          71 as libc::c_int) as usize] as
                       libc::c_int == 0 as libc::c_int {
                if (*actor).init.is_none() && (*actor).draw.is_some() &&
                       (*actor).flags &
                           ((1 as libc::c_int) << 5 as libc::c_int |
                                (1 as libc::c_int) << 6 as libc::c_int) as
                               libc::c_uint != 0 {
                    if (*actor).flags &
                           ((1 as libc::c_int) << 7 as libc::c_int) as
                               libc::c_uint != 0 &&
                           ((*globalCtx).roomCtx.curRoom.showInvisActors as
                                libc::c_int == 0 as libc::c_int ||
                                (*globalCtx).actorCtx.unk_03 as libc::c_int !=
                                    0 as libc::c_int ||
                                (*actor).room as libc::c_int !=
                                    (*globalCtx).roomCtx.curRoom.num as
                                        libc::c_int) {
                        if invisibleActorCounter < 20 as libc::c_int {
                        } else {
                            __assert(b"invisible_actor_counter < INVISIBLE_ACTOR_MAX\x00"
                                         as *const u8 as *const libc::c_char,
                                     b"../z_actor.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     6464 as libc::c_int);
                        };
                        invisibleActors[invisibleActorCounter as usize] =
                            actor;
                        invisibleActorCounter += 1
                    } else if (*gGameInfo).data[(21 as libc::c_int *
                                                     6 as libc::c_int *
                                                     16 as libc::c_int +
                                                     64 as libc::c_int) as
                                                    usize] as libc::c_int !=
                                  1 as libc::c_int ||
                                  (*gGameInfo).data[(21 as libc::c_int *
                                                         6 as libc::c_int *
                                                         16 as libc::c_int +
                                                         65 as libc::c_int) as
                                                        usize] as libc::c_int
                                      != -(1 as libc::c_int) &&
                                      (*gGameInfo).data[(21 as libc::c_int *
                                                             6 as libc::c_int
                                                             *
                                                             16 as libc::c_int
                                                             +
                                                             65 as
                                                                 libc::c_int)
                                                            as usize] as
                                          libc::c_int !=
                                          (*gGameInfo).data[(21 as libc::c_int
                                                                 *
                                                                 6 as
                                                                     libc::c_int
                                                                 *
                                                                 16 as
                                                                     libc::c_int
                                                                 +
                                                                 66 as
                                                                     libc::c_int)
                                                                as usize] as
                                              libc::c_int ||
                                  (*gGameInfo).data[(21 as libc::c_int *
                                                         6 as libc::c_int *
                                                         16 as libc::c_int +
                                                         72 as libc::c_int) as
                                                        usize] as libc::c_int
                                      == 0 as libc::c_int {
                        Actor_Draw(globalCtx, actor);
                        (*actor).isDrawn = 1 as libc::c_int as u8_0
                    }
                }
            }
            actor = (*actor).next
        }
        i += 1;
        actorListEntry = actorListEntry.offset(1)
    }
    if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 64 as libc::c_int) as usize]
           as libc::c_int != 1 as libc::c_int ||
           (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 73 as libc::c_int) as
                                 usize] as libc::c_int != 0 as libc::c_int {
        Effect_DrawAll((*globalCtx).state.gfxCtx);
    }
    if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 64 as libc::c_int) as usize]
           as libc::c_int != 1 as libc::c_int ||
           (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 74 as libc::c_int) as
                                 usize] as libc::c_int != 0 as libc::c_int {
        EffectSs_DrawAll(globalCtx);
    }
    if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 64 as libc::c_int) as usize]
           as libc::c_int != 1 as libc::c_int ||
           (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 72 as libc::c_int) as
                                 usize] as libc::c_int != 0 as libc::c_int {
        if (*globalCtx).actorCtx.unk_03 as libc::c_int != 0 as libc::c_int {
            func_8003115C(globalCtx, invisibleActorCounter,
                          invisibleActors.as_mut_ptr());
            if (*globalCtx).csCtx.state as libc::c_int !=
                   CS_STATE_IDLE as libc::c_int ||
                   Player_InCsMode(globalCtx) != 0 {
                func_800304B0(globalCtx);
            }
        }
    }
    Actor_DrawFaroresWindPointer(globalCtx);
    if (*gGameInfo).data[(9 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 32 as libc::c_int) as usize]
           as libc::c_int == 0 as libc::c_int {
        Lights_DrawGlow(globalCtx);
    }
    if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 64 as libc::c_int) as usize]
           as libc::c_int != 1 as libc::c_int ||
           (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 75 as libc::c_int) as
                                 usize] as libc::c_int != 0 as libc::c_int {
        TitleCard_Draw(globalCtx, &mut (*actorCtx).titleCtx);
    }
    if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 64 as libc::c_int) as usize]
           as libc::c_int != 1 as libc::c_int ||
           (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 76 as libc::c_int) as
                                 usize] as libc::c_int != 0 as libc::c_int {
        CollisionCheck_DrawCollision(globalCtx, &mut (*globalCtx).colChkCtx);
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_actor.c\x00" as *const u8 as *const libc::c_char,
                     6563 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn func_80031A28(mut globalCtx: *mut GlobalContext,
                                       mut actorCtx: *mut ActorContext) {
    let mut actor: *mut Actor = 0 as *mut Actor;
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i <
              (::std::mem::size_of::<[ActorListEntry; 12]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<ActorListEntry>()
                                                   as libc::c_ulong) as s32 {
        actor = (*actorCtx).actorLists[i as usize].head;
        while !actor.is_null() {
            if Object_IsLoaded(&mut (*globalCtx).objectCtx,
                               (*actor).objBankIndex as s32) == 0 {
                Actor_Kill(actor);
            }
            actor = (*actor).next
        }
        i += 1
    };
}
#[no_mangle]
pub static mut sEnemyActorCategories: [u8_0; 2] =
    [ACTORCAT_ENEMY as libc::c_int as u8_0,
     ACTORCAT_BOSS as libc::c_int as u8_0];
#[no_mangle]
pub unsafe extern "C" fn Actor_FreezeAllEnemies(mut globalCtx:
                                                    *mut GlobalContext,
                                                mut actorCtx:
                                                    *mut ActorContext,
                                                mut duration: s32) {
    let mut actor: *mut Actor = 0 as *mut Actor;
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i <
              (::std::mem::size_of::<[u8_0; 2]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<u8_0>()
                                                   as libc::c_ulong) as s32 {
        actor =
            (*actorCtx).actorLists[sEnemyActorCategories[i as usize] as
                                       usize].head;
        while !actor.is_null() {
            (*actor).freezeTimer = duration as u16_0;
            actor = (*actor).next
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_80031B14(mut globalCtx: *mut GlobalContext,
                                       mut actorCtx: *mut ActorContext) {
    let mut actor: *mut Actor = 0 as *mut Actor;
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i <
              (::std::mem::size_of::<[ActorListEntry; 12]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<ActorListEntry>()
                                                   as libc::c_ulong) as s32 {
        actor = (*actorCtx).actorLists[i as usize].head;
        while !actor.is_null() {
            if (*actor).room as libc::c_int >= 0 as libc::c_int &&
                   (*actor).room as libc::c_int !=
                       (*globalCtx).roomCtx.curRoom.num as libc::c_int &&
                   (*actor).room as libc::c_int !=
                       (*globalCtx).roomCtx.prevRoom.num as libc::c_int {
                if (*actor).isDrawn == 0 {
                    actor = Actor_Delete(actorCtx, actor, globalCtx)
                } else {
                    Actor_Kill(actor);
                    Actor_Destroy(actor, globalCtx);
                    actor = (*actor).next
                }
            } else { actor = (*actor).next }
        }
        i += 1
    }
    CollisionCheck_ClearContext(globalCtx, &mut (*globalCtx).colChkCtx);
    (*actorCtx).flags.tempClear = 0 as libc::c_int as u32_0;
    (*actorCtx).flags.tempSwch &= 0xffffff as libc::c_int as libc::c_uint;
    (*globalCtx).msgCtx.unk_E3F4 = 0 as libc::c_int as u16_0;
}
// Actor_CleanupContext
#[no_mangle]
pub unsafe extern "C" fn func_80031C3C(mut actorCtx: *mut ActorContext,
                                       mut globalCtx: *mut GlobalContext) {
    let mut actor: *mut Actor = 0 as *mut Actor;
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i <
              (::std::mem::size_of::<[ActorListEntry; 12]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<ActorListEntry>()
                                                   as libc::c_ulong) as s32 {
        actor = (*actorCtx).actorLists[i as usize].head;
        while !actor.is_null() {
            Actor_Delete(actorCtx, actor, globalCtx);
            actor = (*actorCtx).actorLists[i as usize].head
        }
        i += 1
    }
    if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 20 as libc::c_int) as usize]
           as libc::c_int != 0 as libc::c_int {
        osSyncPrintf(b"\xe7\xb5\xb6\xe5\xaf\xbe\xe9\xad\x94\xe6\xb3\x95\xe9\xa0\x98\xe5\x9f\x9f\xe8\xa7\xa3\xe6\x94\xbe\n\x00"
                         as *const u8 as *const libc::c_char);
        // "Absolute magic field deallocation"
    }
    if !(*actorCtx).absoluteSpace.is_null() {
        ZeldaArena_FreeDebug((*actorCtx).absoluteSpace,
                             b"../z_actor.c\x00" as *const u8 as
                                 *const libc::c_char, 6731 as libc::c_int);
        (*actorCtx).absoluteSpace = 0 as *mut libc::c_void
    }
    Gameplay_SaveSceneFlags(globalCtx);
    func_80030488(globalCtx);
    ActorOverlayTable_Cleanup();
}
/* *
 * Adds a given actor instance at the front of the actor list of the specified category.
 * Also sets the actor instance as being of that category.
 */
#[no_mangle]
pub unsafe extern "C" fn Actor_AddToCategory(mut actorCtx: *mut ActorContext,
                                             mut actorToAdd: *mut Actor,
                                             mut actorCategory: u8_0) {
    let mut prevHead: *mut Actor = 0 as *mut Actor;
    (*actorToAdd).category = actorCategory;
    (*actorCtx).total = (*actorCtx).total.wrapping_add(1);
    (*actorCtx).actorLists[actorCategory as usize].length += 1;
    prevHead = (*actorCtx).actorLists[actorCategory as usize].head;
    if !prevHead.is_null() { (*prevHead).prev = actorToAdd }
    (*actorCtx).actorLists[actorCategory as usize].head = actorToAdd;
    (*actorToAdd).next = prevHead;
}
/* *
 * Removes a given actor instance from its actor list.
 * Also sets the temp clear flag of the current room if the actor removed was the last enemy loaded.
 */
#[no_mangle]
pub unsafe extern "C" fn Actor_RemoveFromCategory(mut globalCtx:
                                                      *mut GlobalContext,
                                                  mut actorCtx:
                                                      *mut ActorContext,
                                                  mut actorToRemove:
                                                      *mut Actor)
 -> *mut Actor {
    let mut newHead: *mut Actor = 0 as *mut Actor;
    (*actorCtx).total = (*actorCtx).total.wrapping_sub(1);
    (*actorCtx).actorLists[(*actorToRemove).category as usize].length -= 1;
    if !(*actorToRemove).prev.is_null() {
        (*(*actorToRemove).prev).next = (*actorToRemove).next
    } else {
        (*actorCtx).actorLists[(*actorToRemove).category as usize].head =
            (*actorToRemove).next
    }
    newHead = (*actorToRemove).next;
    if !newHead.is_null() { (*newHead).prev = (*actorToRemove).prev }
    (*actorToRemove).next = 0 as *mut Actor;
    (*actorToRemove).prev = 0 as *mut Actor;
    if (*actorToRemove).room as libc::c_int ==
           (*globalCtx).roomCtx.curRoom.num as libc::c_int &&
           (*actorToRemove).category as libc::c_int ==
               ACTORCAT_ENEMY as libc::c_int &&
           (*actorCtx).actorLists[ACTORCAT_ENEMY as libc::c_int as
                                      usize].length == 0 as libc::c_int {
        Flags_SetTempClear(globalCtx,
                           (*globalCtx).roomCtx.curRoom.num as s32);
    }
    return newHead;
}
#[no_mangle]
pub unsafe extern "C" fn Actor_FreeOverlay(mut actorOverlay:
                                               *mut ActorOverlay) {
    osSyncPrintf(b"\x1b[36m\x00" as *const u8 as *const libc::c_char);
    if (*actorOverlay).numLoaded as libc::c_int == 0 as libc::c_int {
        if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 20 as libc::c_int) as
                                 usize] as libc::c_int != 0 as libc::c_int {
            osSyncPrintf(b"\xe3\x82\xa2\xe3\x82\xaf\xe3\x82\xbf\xe3\x83\xbc\xe3\x82\xaf\xe3\x83\xa9\xe3\x82\xa4\xe3\x82\xa2\xe3\x83\xb3\xe3\x83\x88\xe3\x81\x8c\xef\xbc\x90\xe3\x81\xab\xe3\x81\xaa\xe3\x82\x8a\xe3\x81\xbe\xe3\x81\x97\xe3\x81\x9f\n\x00"
                             as *const u8 as *const libc::c_char);
            // "Actor client is now 0"
        }
        if !(*actorOverlay).loadedRamAddr.is_null() {
            if (*actorOverlay).allocType as libc::c_int &
                   ALLOCTYPE_PERMANENT as libc::c_int != 0 {
                if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          20 as libc::c_int) as usize] as
                       libc::c_int != 0 as libc::c_int {
                    osSyncPrintf(b"\xe3\x82\xaa\xe3\x83\xbc\xe3\x83\x90\xe3\x83\xbc\xe3\x83\xac\xe3\x82\xa4\xe8\xa7\xa3\xe6\x94\xbe\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x9b\xe3\x82\x93\n\x00"
                                     as *const u8 as *const libc::c_char);
                    // "Overlay will not be deallocated"
                }
            } else if (*actorOverlay).allocType as libc::c_int &
                          ALLOCTYPE_ABSOLUTE as libc::c_int != 0 {
                if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          20 as libc::c_int) as usize] as
                       libc::c_int != 0 as libc::c_int {
                    // "Absolute magic field reserved, so deallocation will not occur"
                    osSyncPrintf(b"\xe7\xb5\xb6\xe5\xaf\xbe\xe9\xad\x94\xe6\xb3\x95\xe9\xa0\x98\xe5\x9f\x9f\xe7\xa2\xba\xe4\xbf\x9d\xe3\x81\xaa\xe3\x81\xae\xe3\x81\xa7\xe8\xa7\xa3\xe6\x94\xbe\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x9b\xe3\x82\x93\n\x00"
                                     as *const u8 as *const libc::c_char);
                }
                (*actorOverlay).loadedRamAddr = 0 as *mut libc::c_void
            } else {
                if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          20 as libc::c_int) as usize] as
                       libc::c_int != 0 as libc::c_int {
                    osSyncPrintf(b"\xe3\x82\xaa\xe3\x83\xbc\xe3\x83\x90\xe3\x83\xbc\xe3\x83\xac\xe3\x82\xa4\xe8\xa7\xa3\xe6\x94\xbe\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x99\n\x00"
                                     as *const u8 as *const libc::c_char);
                    // "Overlay deallocated"
                }
                ZeldaArena_FreeDebug((*actorOverlay).loadedRamAddr,
                                     b"../z_actor.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     6834 as libc::c_int);
                (*actorOverlay).loadedRamAddr = 0 as *mut libc::c_void
            }
        }
    } else if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                     16 as libc::c_int + 20 as libc::c_int) as
                                    usize] as libc::c_int != 0 as libc::c_int
     {
        // "%d of actor client remains"
        osSyncPrintf(b"\xe3\x82\xa2\xe3\x82\xaf\xe3\x82\xbf\xe3\x83\xbc\xe3\x82\xaf\xe3\x83\xa9\xe3\x82\xa4\xe3\x82\xa2\xe3\x83\xb3\xe3\x83\x88\xe3\x81\xaf\xe3\x81\x82\xe3\x81\xa8 %d \xe6\xae\x8b\xe3\x81\xa3\xe3\x81\xa6\xe3\x81\x84\xe3\x81\xbe\xe3\x81\x99\n\x00"
                         as *const u8 as *const libc::c_char,
                     (*actorOverlay).numLoaded as libc::c_int);
    }
    osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn Actor_Spawn(mut actorCtx: *mut ActorContext,
                                     mut globalCtx: *mut GlobalContext,
                                     mut actorId: s16, mut posX: f32_0,
                                     mut posY: f32_0, mut posZ: f32_0,
                                     mut rotX: s16, mut rotY: s16,
                                     mut rotZ: s16, mut params: s16)
 -> *mut Actor {
    let mut pad: s32 = 0;
    let mut actor: *mut Actor = 0 as *mut Actor;
    let mut actorInit: *mut ActorInit = 0 as *mut ActorInit;
    let mut objBankIndex: s32 = 0;
    let mut overlayEntry: *mut ActorOverlay = 0 as *mut ActorOverlay;
    let mut temp: u32_0 = 0;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut overlaySize: u32_0 = 0;
    overlayEntry =
        &mut *gActorOverlayTable.as_mut_ptr().offset(actorId as isize) as
            *mut ActorOverlay;
    if (actorId as libc::c_int) < ACTOR_ID_MAX as libc::c_int {
    } else {
        __assert(b"profile < ACTOR_DLF_MAX\x00" as *const u8 as
                     *const libc::c_char,
                 b"../z_actor.c\x00" as *const u8 as *const libc::c_char,
                 6883 as libc::c_int);
    };
    name =
        if !(*overlayEntry).name.is_null() {
            (*overlayEntry).name as *const libc::c_char
        } else { b"\x00" as *const u8 as *const libc::c_char } as
            *mut libc::c_char;
    overlaySize =
        ((*overlayEntry).vramEnd as
             u32_0).wrapping_sub((*overlayEntry).vramStart as u32_0);
    if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 20 as libc::c_int) as usize]
           as libc::c_int != 0 as libc::c_int {
        // "Actor class addition [%d:%s]"
        osSyncPrintf(b"\xe3\x82\xa2\xe3\x82\xaf\xe3\x82\xbf\xe3\x83\xbc\xe3\x82\xaf\xe3\x83\xa9\xe3\x82\xb9\xe8\xbf\xbd\xe5\x8a\xa0 [%d:%s]\n\x00"
                         as *const u8 as *const libc::c_char,
                     actorId as libc::c_int, name);
    }
    if (*actorCtx).total as libc::c_int > 200 as libc::c_int {
        // "Ａｃｔｏｒ set number exceeded"
        osSyncPrintf(b"\x1b[43;30m\xef\xbc\xa1\xef\xbd\x83\xef\xbd\x94\xef\xbd\x8f\xef\xbd\x92\xe3\x82\xbb\xe3\x83\x83\xe3\x83\x88\xe6\x95\xb0\xe3\x82\xaa\xe3\x83\xbc\xe3\x83\x90\xe3\x83\xbc\n\x1b[m\x00"
                         as *const u8 as *const libc::c_char);
        return 0 as *mut Actor
    }
    if (*overlayEntry).vramStart.is_null() {
        if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 20 as libc::c_int) as
                                 usize] as libc::c_int != 0 as libc::c_int {
            osSyncPrintf(b"\xe3\x82\xaa\xe3\x83\xbc\xe3\x83\x90\xe3\x83\xbc\xe3\x83\xac\xe3\x82\xa4\xe3\x81\xa7\xe3\x81\xaf\xe3\x81\x82\xe3\x82\x8a\xe3\x81\xbe\xe3\x81\x9b\xe3\x82\x93\n\x00"
                             as *const u8 as *const libc::c_char);
            // "Not an overlay"
        }
        actorInit = (*overlayEntry).initInfo
    } else {
        if !(*overlayEntry).loadedRamAddr.is_null() {
            if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                      16 as libc::c_int + 20 as libc::c_int)
                                     as usize] as libc::c_int !=
                   0 as libc::c_int {
                osSyncPrintf(b"\xe6\x97\xa2\xe3\x81\xab\xe3\x83\xad\xe3\x83\xbc\xe3\x83\x89\xe3\x81\x95\xe3\x82\x8c\xe3\x81\xa6\xe3\x81\x84\xe3\x81\xbe\xe3\x81\x99\n\x00"
                                 as *const u8 as *const libc::c_char);
                // "Already loaded"
            }
        } else {
            if (*overlayEntry).allocType as libc::c_int &
                   ALLOCTYPE_ABSOLUTE as libc::c_int != 0 {
                if overlaySize <= 0x27a0 as libc::c_int as libc::c_uint {
                } else {
                    __assert(b"actor_segsize <= AM_FIELD_SIZE\x00" as
                                 *const u8 as *const libc::c_char,
                             b"../z_actor.c\x00" as *const u8 as
                                 *const libc::c_char, 6934 as libc::c_int);
                };
                if (*actorCtx).absoluteSpace.is_null() {
                    // "AMF: absolute magic field"
                    (*actorCtx).absoluteSpace =
                        ZeldaArena_MallocRDebug(0x27a0 as libc::c_int as
                                                    u32_0,
                                                b"AMF:\xe7\xb5\xb6\xe5\xaf\xbe\xe9\xad\x94\xe6\xb3\x95\xe9\xa0\x98\xe5\x9f\x9f\x00"
                                                    as *const u8 as
                                                    *const libc::c_char,
                                                0 as libc::c_int);
                    if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int
                                              * 16 as libc::c_int +
                                              20 as libc::c_int) as usize] as
                           libc::c_int != 0 as libc::c_int {
                        // "Absolute magic field reservation - %d bytes reserved"
                        osSyncPrintf(b"\xe7\xb5\xb6\xe5\xaf\xbe\xe9\xad\x94\xe6\xb3\x95\xe9\xa0\x98\xe5\x9f\x9f\xe7\xa2\xba\xe4\xbf\x9d %d \xe3\x83\x90\xe3\x82\xa4\xe3\x83\x88\xe7\xa2\xba\xe4\xbf\x9d\n\x00"
                                         as *const u8 as *const libc::c_char,
                                     0x27a0 as libc::c_int);
                    }
                }
                (*overlayEntry).loadedRamAddr = (*actorCtx).absoluteSpace
            } else if (*overlayEntry).allocType as libc::c_int &
                          ALLOCTYPE_PERMANENT as libc::c_int != 0 {
                (*overlayEntry).loadedRamAddr =
                    ZeldaArena_MallocRDebug(overlaySize, name,
                                            0 as libc::c_int)
            } else {
                (*overlayEntry).loadedRamAddr =
                    ZeldaArena_MallocDebug(overlaySize, name,
                                           0 as libc::c_int)
            }
            if (*overlayEntry).loadedRamAddr.is_null() {
                // "Cannot reserve actor program memory"
                osSyncPrintf(b"\x1b[41;37m\xef\xbc\xa1\xef\xbd\x83\xef\xbd\x94\xef\xbd\x8f\xef\xbd\x92\xe3\x83\x97\xe3\x83\xad\xe3\x82\xb0\xe3\x83\xa9\xe3\x83\xa0\xe3\x83\xa1\xe3\x83\xa2\xe3\x83\xaa\xe3\x81\x8c\xe7\xa2\xba\xe4\xbf\x9d\xe3\x81\xa7\xe3\x81\x8d\xe3\x81\xbe\xe3\x81\x9b\xe3\x82\x93\n\x1b[m\x00"
                                 as *const u8 as *const libc::c_char);
                return 0 as *mut Actor
            }
            Overlay_Load((*overlayEntry).vromStart, (*overlayEntry).vromEnd,
                         (*overlayEntry).vramStart, (*overlayEntry).vramEnd,
                         (*overlayEntry).loadedRamAddr);
            osSyncPrintf(b"\x1b[32m\x00" as *const u8 as *const libc::c_char);
            osSyncPrintf(b"OVL(a):Seg:%08x-%08x Ram:%08x-%08x Off:%08x %s\n\x00"
                             as *const u8 as *const libc::c_char,
                         (*overlayEntry).vramStart, (*overlayEntry).vramEnd,
                         (*overlayEntry).loadedRamAddr,
                         ((*overlayEntry).loadedRamAddr as
                              u32_0).wrapping_add((*overlayEntry).vramEnd as
                                                      u32_0).wrapping_sub((*overlayEntry).vramStart
                                                                              as
                                                                              u32_0),
                         ((*overlayEntry).vramStart as
                              u32_0).wrapping_sub((*overlayEntry).loadedRamAddr
                                                      as u32_0), name);
            osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
            (*overlayEntry).numLoaded = 0 as libc::c_int as s8
        }
        actorInit =
            if !(*overlayEntry).initInfo.is_null() {
                ((*overlayEntry).initInfo as
                     u32_0).wrapping_sub(((*overlayEntry).vramStart as
                                              u32_0).wrapping_sub((*overlayEntry).loadedRamAddr
                                                                      as
                                                                      u32_0)
                                             as s32 as libc::c_uint) as
                    *mut libc::c_void
            } else { 0 as *mut libc::c_void } as u32_0 as *mut libc::c_void as
                *mut ActorInit
    }
    objBankIndex =
        Object_GetIndex(&mut (*globalCtx).objectCtx, (*actorInit).objectId);
    if objBankIndex < 0 as libc::c_int ||
           (*actorInit).category as libc::c_int ==
               ACTORCAT_ENEMY as libc::c_int &&
               Flags_GetClear(globalCtx,
                              (*globalCtx).roomCtx.curRoom.num as s32) != 0 {
        // "No data bank!! <data bank＝%d> (profilep->bank=%d)"
        osSyncPrintf(b"\x1b[41;37m\xe3\x83\x87\xe3\x83\xbc\xe3\x82\xbf\xe3\x83\x90\xe3\x83\xb3\xe3\x82\xaf\xe7\x84\xa1\xe3\x81\x97\xef\xbc\x81\xef\xbc\x81<\xe3\x83\x87\xe3\x83\xbc\xe3\x82\xbf\xe3\x83\x90\xe3\x83\xb3\xe3\x82\xaf\xef\xbc\x9d%d>(profilep->bank=%d)\n\x1b[m\x00"
                         as *const u8 as *const libc::c_char, objBankIndex,
                     (*actorInit).objectId as libc::c_int);
        Actor_FreeOverlay(overlayEntry);
        return 0 as *mut Actor
    }
    actor =
        ZeldaArena_MallocDebug((*actorInit).instanceSize, name,
                               1 as libc::c_int) as *mut Actor;
    if actor.is_null() {
        // "Actor class cannot be reserved! %s <size＝%d bytes>"
        osSyncPrintf(b"\x1b[41;37m\xef\xbc\xa1\xef\xbd\x83\xef\xbd\x94\xef\xbd\x8f\xef\xbd\x92\xe3\x82\xaf\xe3\x83\xa9\xe3\x82\xb9\xe7\xa2\xba\xe4\xbf\x9d\xe3\x81\xa7\xe3\x81\x8d\xe3\x81\xbe\xe3\x81\x9b\xe3\x82\x93\xef\xbc\x81 %s <\xe3\x82\xb5\xe3\x82\xa4\xe3\x82\xba\xef\xbc\x9d%d\xe3\x83\x90\xe3\x82\xa4\xe3\x83\x88>\n\x00"
                         as *const u8 as *const libc::c_char,
                     b"\x1b[m\x00" as *const u8 as *const libc::c_char, name,
                     (*actorInit).instanceSize);
        Actor_FreeOverlay(overlayEntry);
        return 0 as *mut Actor
    }
    if ((*overlayEntry).numLoaded as libc::c_int) < 255 as libc::c_int {
    } else {
        __assert(b"actor_dlftbl->clients < 255\x00" as *const u8 as
                     *const libc::c_char,
                 b"../z_actor.c\x00" as *const u8 as *const libc::c_char,
                 7031 as libc::c_int);
    };
    (*overlayEntry).numLoaded += 1;
    if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 20 as libc::c_int) as usize]
           as libc::c_int != 0 as libc::c_int {
        // "Actor client No. %d"
        osSyncPrintf(b"\xe3\x82\xa2\xe3\x82\xaf\xe3\x82\xbf\xe3\x83\xbc\xe3\x82\xaf\xe3\x83\xa9\xe3\x82\xa4\xe3\x82\xa2\xe3\x83\xb3\xe3\x83\x88\xe3\x81\xaf %d \xe5\x80\x8b\xe7\x9b\xae\xe3\x81\xa7\xe3\x81\x99\n\x00"
                         as *const u8 as *const libc::c_char,
                     (*overlayEntry).numLoaded as libc::c_int);
    }
    Lib_MemSet(actor as *mut u8_0, (*actorInit).instanceSize as size_t,
               0 as libc::c_int as u8_0);
    (*actor).overlayEntry = overlayEntry;
    (*actor).id = (*actorInit).id;
    (*actor).flags = (*actorInit).flags;
    if (*actorInit).id as libc::c_int == ACTOR_EN_PART as libc::c_int {
        (*actor).objBankIndex = rotZ as s8;
        rotZ = 0 as libc::c_int as s16
    } else { (*actor).objBankIndex = objBankIndex as s8 }
    (*actor).init = (*actorInit).init;
    (*actor).destroy = (*actorInit).destroy;
    (*actor).update = (*actorInit).update;
    (*actor).draw = (*actorInit).draw;
    (*actor).room = (*globalCtx).roomCtx.curRoom.num;
    (*actor).home.pos.x = posX;
    (*actor).home.pos.y = posY;
    (*actor).home.pos.z = posZ;
    (*actor).home.rot.x = rotX;
    (*actor).home.rot.y = rotY;
    (*actor).home.rot.z = rotZ;
    (*actor).params = params;
    Actor_AddToCategory(actorCtx, actor, (*actorInit).category);
    temp = gSegments[6 as libc::c_int as usize];
    Actor_Init(actor, globalCtx);
    gSegments[6 as libc::c_int as usize] = temp;
    return actor;
}
#[no_mangle]
pub unsafe extern "C" fn Actor_SpawnAsChild(mut actorCtx: *mut ActorContext,
                                            mut parent: *mut Actor,
                                            mut globalCtx: *mut GlobalContext,
                                            mut actorId: s16, mut posX: f32_0,
                                            mut posY: f32_0, mut posZ: f32_0,
                                            mut rotX: s16, mut rotY: s16,
                                            mut rotZ: s16, mut params: s16)
 -> *mut Actor {
    let mut spawnedActor: *mut Actor =
        Actor_Spawn(actorCtx, globalCtx, actorId, posX, posY, posZ, rotX,
                    rotY, rotZ, params);
    if spawnedActor.is_null() { return 0 as *mut Actor }
    (*parent).child = spawnedActor;
    (*spawnedActor).parent = parent;
    if (*spawnedActor).room as libc::c_int >= 0 as libc::c_int {
        (*spawnedActor).room = (*parent).room
    }
    return spawnedActor;
}
#[no_mangle]
pub unsafe extern "C" fn Actor_SpawnTransitionActors(mut globalCtx:
                                                         *mut GlobalContext,
                                                     mut actorCtx:
                                                         *mut ActorContext) {
    let mut transitionActor: *mut TransitionActorEntry =
        0 as *mut TransitionActorEntry;
    let mut numActors: u8_0 = 0;
    let mut i: s32 = 0;
    transitionActor = (*globalCtx).transiActorCtx.list;
    numActors = (*globalCtx).transiActorCtx.numActors;
    i = 0 as libc::c_int;
    while i < numActors as libc::c_int {
        if (*transitionActor).id as libc::c_int >= 0 as libc::c_int {
            if (*transitionActor).sides[0 as libc::c_int as usize].room as
                   libc::c_int >= 0 as libc::c_int &&
                   ((*transitionActor).sides[0 as libc::c_int as usize].room
                        as libc::c_int ==
                        (*globalCtx).roomCtx.curRoom.num as libc::c_int ||
                        (*transitionActor).sides[0 as libc::c_int as
                                                     usize].room as
                            libc::c_int ==
                            (*globalCtx).roomCtx.prevRoom.num as libc::c_int)
                   ||
                   (*transitionActor).sides[1 as libc::c_int as usize].room as
                       libc::c_int >= 0 as libc::c_int &&
                       ((*transitionActor).sides[1 as libc::c_int as
                                                     usize].room as
                            libc::c_int ==
                            (*globalCtx).roomCtx.curRoom.num as libc::c_int ||
                            (*transitionActor).sides[1 as libc::c_int as
                                                         usize].room as
                                libc::c_int ==
                                (*globalCtx).roomCtx.prevRoom.num as
                                    libc::c_int) {
                Actor_Spawn(actorCtx, globalCtx,
                            ((*transitionActor).id as libc::c_int &
                                 0x1fff as libc::c_int) as s16,
                            (*transitionActor).pos.x as f32_0,
                            (*transitionActor).pos.y as f32_0,
                            (*transitionActor).pos.z as f32_0,
                            0 as libc::c_int as s16, (*transitionActor).rotY,
                            0 as libc::c_int as s16,
                            ((i << 0xa as libc::c_int) +
                                 (*transitionActor).params as libc::c_int) as
                                s16);
                (*transitionActor).id =
                    -((*transitionActor).id as libc::c_int) as s16;
                numActors = (*globalCtx).transiActorCtx.numActors
            }
        }
        transitionActor = transitionActor.offset(1);
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn Actor_SpawnEntry(mut actorCtx: *mut ActorContext,
                                          mut actorEntry: *mut ActorEntry,
                                          mut globalCtx: *mut GlobalContext)
 -> *mut Actor {
    return Actor_Spawn(actorCtx, globalCtx, (*actorEntry).id,
                       (*actorEntry).pos.x as f32_0,
                       (*actorEntry).pos.y as f32_0,
                       (*actorEntry).pos.z as f32_0, (*actorEntry).rot.x,
                       (*actorEntry).rot.y, (*actorEntry).rot.z,
                       (*actorEntry).params);
}
#[no_mangle]
pub unsafe extern "C" fn Actor_Delete(mut actorCtx: *mut ActorContext,
                                      mut actor: *mut Actor,
                                      mut globalCtx: *mut GlobalContext)
 -> *mut Actor {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut player: *mut Player = 0 as *mut Player;
    let mut newHead: *mut Actor = 0 as *mut Actor;
    let mut overlayEntry: *mut ActorOverlay = 0 as *mut ActorOverlay;
    player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    overlayEntry = (*actor).overlayEntry;
    name =
        if !(*overlayEntry).name.is_null() {
            (*overlayEntry).name as *const libc::c_char
        } else { b"\x00" as *const u8 as *const libc::c_char } as
            *mut libc::c_char;
    if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 20 as libc::c_int) as usize]
           as libc::c_int != 0 as libc::c_int {
        osSyncPrintf(b"\xe3\x82\xa2\xe3\x82\xaf\xe3\x82\xbf\xe3\x83\xbc\xe3\x82\xaf\xe3\x83\xa9\xe3\x82\xb9\xe5\x89\x8a\xe9\x99\xa4 [%s]\n\x00"
                         as *const u8 as *const libc::c_char, name);
        // "Actor class deleted [%s]"
    }
    if !player.is_null() && actor == (*player).unk_664 {
        func_8008EDF0(player);
        Camera_ChangeMode(Gameplay_GetCamera(globalCtx,
                                             Gameplay_GetActiveCamId(globalCtx)),
                          0 as libc::c_int as s16);
    }
    if actor == (*actorCtx).targetCtx.arrowPointedActor {
        (*actorCtx).targetCtx.arrowPointedActor = 0 as *mut Actor
    }
    if actor == (*actorCtx).targetCtx.unk_8C {
        (*actorCtx).targetCtx.unk_8C = 0 as *mut Actor
    }
    if actor == (*actorCtx).targetCtx.bgmEnemy {
        (*actorCtx).targetCtx.bgmEnemy = 0 as *mut Actor
    }
    Audio_StopSfxByPos(&mut (*actor).projectedPos);
    Actor_Destroy(actor, globalCtx);
    newHead = Actor_RemoveFromCategory(globalCtx, actorCtx, actor);
    ZeldaArena_FreeDebug(actor as *mut libc::c_void,
                         b"../z_actor.c\x00" as *const u8 as
                             *const libc::c_char, 7242 as libc::c_int);
    if (*overlayEntry).vramStart.is_null() {
        if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 20 as libc::c_int) as
                                 usize] as libc::c_int != 0 as libc::c_int {
            osSyncPrintf(b"\xe3\x82\xaa\xe3\x83\xbc\xe3\x83\x90\xe3\x83\xbc\xe3\x83\xac\xe3\x82\xa4\xe3\x81\xa7\xe3\x81\xaf\xe3\x81\x82\xe3\x82\x8a\xe3\x81\xbe\xe3\x81\x9b\xe3\x82\x93\n\x00"
                             as *const u8 as *const libc::c_char);
            // "Not an overlay"
        }
    } else {
        if !(*overlayEntry).loadedRamAddr.is_null() {
        } else {
            __assert(b"actor_dlftbl->allocp != NULL\x00" as *const u8 as
                         *const libc::c_char,
                     b"../z_actor.c\x00" as *const u8 as *const libc::c_char,
                     7251 as libc::c_int);
        };
        if (*overlayEntry).numLoaded as libc::c_int > 0 as libc::c_int {
        } else {
            __assert(b"actor_dlftbl->clients > 0\x00" as *const u8 as
                         *const libc::c_char,
                     b"../z_actor.c\x00" as *const u8 as *const libc::c_char,
                     7252 as libc::c_int);
        };
        (*overlayEntry).numLoaded -= 1;
        Actor_FreeOverlay(overlayEntry);
    }
    return newHead;
}
#[no_mangle]
pub unsafe extern "C" fn func_80032880(mut globalCtx: *mut GlobalContext,
                                       mut actor: *mut Actor) -> s32 {
    let mut sp1E: s16 = 0;
    let mut sp1C: s16 = 0;
    Actor_GetScreenPos(globalCtx, actor, &mut sp1E, &mut sp1C);
    return (sp1E as libc::c_int > -(20 as libc::c_int) &&
                (sp1E as libc::c_int) < 340 as libc::c_int &&
                sp1C as libc::c_int > -(160 as libc::c_int) &&
                (sp1C as libc::c_int) < 400 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub static mut D_8015BBE8: *mut Actor = 0 as *const Actor as *mut Actor;
#[no_mangle]
pub static mut D_8015BBEC: *mut Actor = 0 as *const Actor as *mut Actor;
#[no_mangle]
pub static mut D_8015BBF0: f32_0 = 0.;
#[no_mangle]
pub static mut sbgmEnemyDistSq: f32_0 = 0.;
#[no_mangle]
pub static mut D_8015BBF8: s32 = 0;
#[no_mangle]
pub static mut D_8015BBFC: s16 = 0;
#[no_mangle]
pub unsafe extern "C" fn func_800328D4(mut globalCtx: *mut GlobalContext,
                                       mut actorCtx: *mut ActorContext,
                                       mut player: *mut Player,
                                       mut actorCategory: u32_0) {
    let mut var: f32_0 = 0.;
    let mut actor: *mut Actor = 0 as *mut Actor;
    let mut sp84: *mut Actor = 0 as *mut Actor;
    let mut sp80: *mut CollisionPoly = 0 as *mut CollisionPoly;
    let mut sp7C: s32 = 0;
    let mut sp70: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    actor = (*actorCtx).actorLists[actorCategory as usize].head;
    sp84 = (*player).unk_664;
    while !actor.is_null() {
        if (*actor).update.is_some() && actor as *mut Player != player &&
               (*actor).flags &
                   ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint ==
                   ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint {
            // This block below is for determining the closest actor to player in determining the volume
            // used while playing enemy bgm music
            if actorCategory == ACTORCAT_ENEMY as libc::c_int as libc::c_uint
                   &&
                   (*actor).flags &
                       ((1 as libc::c_int) << 0 as libc::c_int |
                            (1 as libc::c_int) << 2 as libc::c_int) as
                           libc::c_uint ==
                       ((1 as libc::c_int) << 0 as libc::c_int |
                            (1 as libc::c_int) << 2 as libc::c_int) as
                           libc::c_uint &&
                   (*actor).xyzDistToPlayerSq < 500.0f32 * 500.0f32 &&
                   (*actor).xyzDistToPlayerSq < sbgmEnemyDistSq {
                (*actorCtx).targetCtx.bgmEnemy = actor;
                sbgmEnemyDistSq = (*actor).xyzDistToPlayerSq
            }
            if actor != sp84 {
                var = func_8002EFC0(actor, player, D_8015BBFC);
                if var < D_8015BBF0 && func_8002F090(actor, var) != 0 &&
                       func_80032880(globalCtx, actor) != 0 &&
                       (BgCheck_CameraLineTest1(&mut (*globalCtx).colCtx,
                                                &mut (*player).actor.focus.pos,
                                                &mut (*actor).focus.pos,
                                                &mut sp70, &mut sp80,
                                                1 as libc::c_int,
                                                1 as libc::c_int,
                                                1 as libc::c_int,
                                                1 as libc::c_int, &mut sp7C)
                            == 0 ||
                            SurfaceType_IsIgnoredByProjectiles(&mut (*globalCtx).colCtx,
                                                               sp80, sp7C) !=
                                0) {
                    if (*actor).targetPriority as libc::c_int !=
                           0 as libc::c_int {
                        if ((*actor).targetPriority as libc::c_int) <
                               D_8015BBF8 {
                            D_8015BBEC = actor;
                            D_8015BBF8 = (*actor).targetPriority as s32
                        }
                    } else { D_8015BBE8 = actor; D_8015BBF0 = var }
                }
            }
        }
        actor = (*actor).next
    };
}
#[no_mangle]
pub static mut D_801160A0: [u8_0; 12] =
    [ACTORCAT_BOSS as libc::c_int as u8_0,
     ACTORCAT_ENEMY as libc::c_int as u8_0,
     ACTORCAT_BG as libc::c_int as u8_0,
     ACTORCAT_EXPLOSIVE as libc::c_int as u8_0,
     ACTORCAT_NPC as libc::c_int as u8_0,
     ACTORCAT_ITEMACTION as libc::c_int as u8_0,
     ACTORCAT_CHEST as libc::c_int as u8_0,
     ACTORCAT_SWITCH as libc::c_int as u8_0,
     ACTORCAT_PROP as libc::c_int as u8_0,
     ACTORCAT_MISC as libc::c_int as u8_0,
     ACTORCAT_DOOR as libc::c_int as u8_0,
     ACTORCAT_SWITCH as libc::c_int as u8_0];
#[no_mangle]
pub unsafe extern "C" fn func_80032AF0(mut globalCtx: *mut GlobalContext,
                                       mut actorCtx: *mut ActorContext,
                                       mut actorPtr: *mut *mut Actor,
                                       mut player: *mut Player)
 -> *mut Actor {
    let mut i: s32 = 0;
    let mut entry: *mut u8_0 = 0 as *mut u8_0;
    D_8015BBEC = 0 as *mut Actor;
    D_8015BBE8 = D_8015BBEC;
    sbgmEnemyDistSq = 340282346638528859811704183484516925440.0f32;
    D_8015BBF0 = sbgmEnemyDistSq;
    D_8015BBF8 = 0x7fffffff as libc::c_int;
    if Player_InCsMode(globalCtx) == 0 {
        entry =
            &mut *D_801160A0.as_mut_ptr().offset(0 as libc::c_int as isize) as
                *mut u8_0;
        (*actorCtx).targetCtx.bgmEnemy = 0 as *mut Actor;
        D_8015BBFC = (*player).actor.shape.rot.y;
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            func_800328D4(globalCtx, actorCtx, player, *entry as u32_0);
            entry = entry.offset(1);
            i += 1
        }
        if D_8015BBE8.is_null() {
            while i <
                      (::std::mem::size_of::<[u8_0; 12]>() as
                           libc::c_ulong).wrapping_div(::std::mem::size_of::<u8_0>()
                                                           as libc::c_ulong)
                          as s32 {
                func_800328D4(globalCtx, actorCtx, player, *entry as u32_0);
                entry = entry.offset(1);
                i += 1
            }
        }
    }
    if D_8015BBE8.is_null() {
        *actorPtr = D_8015BBEC
    } else { *actorPtr = D_8015BBE8 }
    return *actorPtr;
}
/* *
 * Finds the first actor instance of a specified ID and category if there is one.
 */
#[no_mangle]
pub unsafe extern "C" fn Actor_Find(mut actorCtx: *mut ActorContext,
                                    mut actorId: s32, mut actorCategory: s32)
 -> *mut Actor {
    let mut actor: *mut Actor =
        (*actorCtx).actorLists[actorCategory as usize].head;
    while !actor.is_null() {
        if actorId == (*actor).id as libc::c_int { return actor }
        actor = (*actor).next
    }
    return 0 as *mut Actor;
}
/* *
 * Play the death sound effect and flash the screen white for 4 frames.
 * While the screen flashes, the game freezes.
 */
#[no_mangle]
pub unsafe extern "C" fn Enemy_StartFinishingBlow(mut globalCtx:
                                                      *mut GlobalContext,
                                                  mut actor: *mut Actor) {
    (*globalCtx).actorCtx.freezeFlashTimer = 5 as libc::c_int as u8_0;
    Audio_PlaySoundAtPosition(globalCtx, &mut (*actor).world.pos,
                              20 as libc::c_int,
                              0x388b as libc::c_int as u16_0);
}
#[no_mangle]
pub unsafe extern "C" fn func_80032CB4(mut arg0: *mut s16, mut arg1: s16,
                                       mut arg2: s16, mut arg3: s16) -> s16 {
    if (if *arg0.offset(1 as libc::c_int as isize) as libc::c_int ==
               0 as libc::c_int {
            0 as libc::c_int
        } else {
            let ref mut fresh86 = *arg0.offset(1 as libc::c_int as isize);
            *fresh86 -= 1;
            *fresh86 as libc::c_int
        }) == 0 as libc::c_int {
        *arg0.offset(1 as libc::c_int as isize) = Rand_S16Offset(arg1, arg2)
    }
    if *arg0.offset(1 as libc::c_int as isize) as libc::c_int -
           arg3 as libc::c_int > 0 as libc::c_int {
        *arg0.offset(0 as libc::c_int as isize) = 0 as libc::c_int as s16
    } else if *arg0.offset(1 as libc::c_int as isize) as libc::c_int -
                  arg3 as libc::c_int > -(2 as libc::c_int) ||
                  (*arg0.offset(1 as libc::c_int as isize) as libc::c_int) <
                      2 as libc::c_int {
        *arg0.offset(0 as libc::c_int as isize) = 1 as libc::c_int as s16
    } else {
        *arg0.offset(0 as libc::c_int as isize) = 2 as libc::c_int as s16
    }
    return *arg0.offset(0 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn func_80032D60(mut arg0: *mut s16, mut arg1: s16,
                                       mut arg2: s16, mut arg3: s16) -> s16 {
    if (if *arg0.offset(1 as libc::c_int as isize) as libc::c_int ==
               0 as libc::c_int {
            0 as libc::c_int
        } else {
            let ref mut fresh87 = *arg0.offset(1 as libc::c_int as isize);
            *fresh87 -= 1;
            *fresh87 as libc::c_int
        }) == 0 as libc::c_int {
        *arg0.offset(1 as libc::c_int as isize) = Rand_S16Offset(arg1, arg2);
        let ref mut fresh88 = *arg0.offset(0 as libc::c_int as isize);
        *fresh88 += 1;
        if *arg0.offset(0 as libc::c_int as isize) as libc::c_int %
               3 as libc::c_int == 0 as libc::c_int {
            *arg0.offset(0 as libc::c_int as isize) =
                ((Rand_ZeroOne() * arg3 as libc::c_int as libc::c_float) as
                     s32 * 3 as libc::c_int) as s16
        }
    }
    return *arg0.offset(0 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn BodyBreak_Alloc(mut bodyBreak: *mut BodyBreak,
                                         mut count: s32,
                                         mut globalCtx: *mut GlobalContext) {
    let mut matricesSize: u32_0 = 0;
    let mut dListsSize: u32_0 = 0;
    let mut objectIdsSize: u32_0 = 0;
    matricesSize =
        ((count + 1 as libc::c_int) as
             libc::c_uint).wrapping_mul(::std::mem::size_of::<MtxF>() as
                                            libc::c_ulong);
    (*bodyBreak).matrices =
        ZeldaArena_MallocDebug(matricesSize,
                               b"../z_actor.c\x00" as *const u8 as
                                   *const libc::c_char, 7540 as libc::c_int)
            as *mut MtxF;
    if !(*bodyBreak).matrices.is_null() {
        dListsSize =
            ((count + 1 as libc::c_int) as
                 libc::c_uint).wrapping_mul(::std::mem::size_of::<*mut Gfx>()
                                                as libc::c_ulong);
        (*bodyBreak).dLists =
            ZeldaArena_MallocDebug(dListsSize,
                                   b"../z_actor.c\x00" as *const u8 as
                                       *const libc::c_char,
                                   7543 as libc::c_int) as *mut *mut Gfx;
        if !(*bodyBreak).dLists.is_null() {
            objectIdsSize =
                ((count + 1 as libc::c_int) as
                     libc::c_uint).wrapping_mul(::std::mem::size_of::<s16>()
                                                    as libc::c_ulong);
            (*bodyBreak).objectIds =
                ZeldaArena_MallocDebug(objectIdsSize,
                                       b"../z_actor.c\x00" as *const u8 as
                                           *const libc::c_char,
                                       7546 as libc::c_int) as *mut s16;
            if !(*bodyBreak).objectIds.is_null() {
                Lib_MemSet((*bodyBreak).matrices as *mut u8_0,
                           matricesSize as size_t, 0 as libc::c_int as u8_0);
                Lib_MemSet((*bodyBreak).dLists as *mut u8_0,
                           dListsSize as size_t, 0 as libc::c_int as u8_0);
                Lib_MemSet((*bodyBreak).objectIds as *mut u8_0,
                           objectIdsSize as size_t, 0 as libc::c_int as u8_0);
                (*bodyBreak).val = 1 as libc::c_int;
                return
            }
        }
    }
    if !(*bodyBreak).matrices.is_null() {
        ZeldaArena_FreeDebug((*bodyBreak).matrices as *mut libc::c_void,
                             b"../z_actor.c\x00" as *const u8 as
                                 *const libc::c_char, 7558 as libc::c_int);
    }
    if !(*bodyBreak).dLists.is_null() {
        ZeldaArena_FreeDebug((*bodyBreak).dLists as *mut libc::c_void,
                             b"../z_actor.c\x00" as *const u8 as
                                 *const libc::c_char, 7561 as libc::c_int);
    }
    if !(*bodyBreak).objectIds.is_null() {
        ZeldaArena_FreeDebug((*bodyBreak).objectIds as *mut libc::c_void,
                             b"../z_actor.c\x00" as *const u8 as
                                 *const libc::c_char, 7564 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BodyBreak_SetInfo(mut bodyBreak: *mut BodyBreak,
                                           mut limbIndex: s32,
                                           mut minLimbIndex: s32,
                                           mut maxLimbIndex: s32,
                                           mut count: u32_0,
                                           mut dList: *mut *mut Gfx,
                                           mut objectId: s16) {
    let mut globalCtx: *mut GlobalContext = Effect_GetGlobalCtx();
    if (*globalCtx).actorCtx.freezeFlashTimer as libc::c_int ==
           0 as libc::c_int && (*bodyBreak).val > 0 as libc::c_int {
        if limbIndex >= minLimbIndex && limbIndex <= maxLimbIndex &&
               !(*dList).is_null() {
            let ref mut fresh89 =
                *(*bodyBreak).dLists.offset((*bodyBreak).val as isize);
            *fresh89 = *dList;
            Matrix_Get(&mut *(*bodyBreak).matrices.offset((*bodyBreak).val as
                                                              isize));
            *(*bodyBreak).objectIds.offset((*bodyBreak).val as isize) =
                objectId;
            (*bodyBreak).val += 1
        }
        if limbIndex != (*bodyBreak).prevLimbIndex { (*bodyBreak).count += 1 }
        if (*bodyBreak).count as u32_0 >= count {
            (*bodyBreak).count = ((*bodyBreak).val - 1 as libc::c_int) as s16;
            (*bodyBreak).val = -(1 as libc::c_int)
        }
    }
    (*bodyBreak).prevLimbIndex = limbIndex;
}
#[no_mangle]
pub unsafe extern "C" fn BodyBreak_SpawnParts(mut actor: *mut Actor,
                                              mut bodyBreak: *mut BodyBreak,
                                              mut globalCtx:
                                                  *mut GlobalContext,
                                              mut type_0: s16) -> s32 {
    let mut spawnedEnPart: *mut EnPart = 0 as *mut EnPart;
    let mut mtx: *mut MtxF = 0 as *mut MtxF;
    let mut objBankIndex: s16 = 0;
    if (*bodyBreak).val != -(1 as libc::c_int) { return 0 as libc::c_int }
    while (*bodyBreak).count as libc::c_int > 0 as libc::c_int {
        Matrix_Put(&mut *(*bodyBreak).matrices.offset((*bodyBreak).count as
                                                          isize));
        Matrix_Scale(1.0f32 / (*actor).scale.x, 1.0f32 / (*actor).scale.y,
                     1.0f32 / (*actor).scale.z,
                     MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_Get(&mut *(*bodyBreak).matrices.offset((*bodyBreak).count as
                                                          isize));
        if *(*bodyBreak).objectIds.offset((*bodyBreak).count as isize) as
               libc::c_int >= 0 as libc::c_int {
            objBankIndex =
                *(*bodyBreak).objectIds.offset((*bodyBreak).count as isize)
        } else { objBankIndex = (*actor).objBankIndex as s16 }
        mtx =
            &mut *(*bodyBreak).matrices.offset((*bodyBreak).count as isize) as
                *mut MtxF;
        spawnedEnPart =
            Actor_SpawnAsChild(&mut (*globalCtx).actorCtx, actor, globalCtx,
                               ACTOR_EN_PART as libc::c_int as s16,
                               (*mtx).c2rust_unnamed.xw,
                               (*mtx).c2rust_unnamed.yw,
                               (*mtx).c2rust_unnamed.zw,
                               0 as libc::c_int as s16,
                               0 as libc::c_int as s16, objBankIndex, type_0)
                as *mut EnPart;
        if !spawnedEnPart.is_null() {
            Matrix_MtxFToYXZRotS(&mut *(*bodyBreak).matrices.offset((*bodyBreak).count
                                                                        as
                                                                        isize),
                                 &mut (*spawnedEnPart).actor.shape.rot,
                                 0 as libc::c_int);
            (*spawnedEnPart).displayList =
                *(*bodyBreak).dLists.offset((*bodyBreak).count as isize);
            (*spawnedEnPart).actor.scale = (*actor).scale
        }
        (*bodyBreak).count -= 1
    }
    (*bodyBreak).val = 0 as libc::c_int;
    ZeldaArena_FreeDebug((*bodyBreak).matrices as *mut libc::c_void,
                         b"../z_actor.c\x00" as *const u8 as
                             *const libc::c_char, 7678 as libc::c_int);
    ZeldaArena_FreeDebug((*bodyBreak).dLists as *mut libc::c_void,
                         b"../z_actor.c\x00" as *const u8 as
                             *const libc::c_char, 7679 as libc::c_int);
    ZeldaArena_FreeDebug((*bodyBreak).objectIds as *mut libc::c_void,
                         b"../z_actor.c\x00" as *const u8 as
                             *const libc::c_char, 7680 as libc::c_int);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Actor_SpawnFloorDustRing(mut globalCtx:
                                                      *mut GlobalContext,
                                                  mut actor: *mut Actor,
                                                  mut posXZ: *mut Vec3f,
                                                  mut radius: f32_0,
                                                  mut amountMinusOne: s32,
                                                  mut randAccelWeight: f32_0,
                                                  mut scale: s16,
                                                  mut scaleStep: s16,
                                                  mut useLighting: u8_0) {
    let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut velocity: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
    let mut accel: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.3f32, z: 0.0f32,}; init };
    let mut angle: f32_0 = 0.;
    let mut i: s32 = 0;
    angle = (Rand_ZeroOne() - 0.5f32) * (2.0f32 * 3.14f32);
    pos.y = (*actor).floorHeight;
    accel.y += (Rand_ZeroOne() - 0.5f32) * 0.2f32;
    i = amountMinusOne;
    while i >= 0 as libc::c_int {
        pos.x = Math_SinF(angle) * radius + (*posXZ).x;
        pos.z = Math_CosF(angle) * radius + (*posXZ).z;
        accel.x = (Rand_ZeroOne() - 0.5f32) * randAccelWeight;
        accel.z = (Rand_ZeroOne() - 0.5f32) * randAccelWeight;
        if scale as libc::c_int == 0 as libc::c_int {
            func_8002857C(globalCtx, &mut pos, &mut velocity, &mut accel);
        } else if useLighting != 0 {
            func_800286CC(globalCtx, &mut pos, &mut velocity, &mut accel,
                          scale, scaleStep);
        } else {
            func_8002865C(globalCtx, &mut pos, &mut velocity, &mut accel,
                          scale, scaleStep);
        }
        angle +=
            2.0f32 * 3.14f32 / (amountMinusOne as libc::c_float + 1.0f32);
        i -= 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_80033480(mut globalCtx: *mut GlobalContext,
                                       mut posBase: *mut Vec3f,
                                       mut randRangeDiameter: f32_0,
                                       mut amountMinusOne: s32,
                                       mut scaleBase: s16, mut scaleStep: s16,
                                       mut arg6: u8_0) {
    let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut velocity: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
    let mut accel: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.3f32, z: 0.0f32,}; init };
    let mut scale: s16 = 0;
    let mut var2: u32_0 = 0;
    let mut i: s32 = 0;
    i = amountMinusOne;
    while i >= 0 as libc::c_int {
        pos.x = (*posBase).x + (Rand_ZeroOne() - 0.5f32) * randRangeDiameter;
        pos.y = (*posBase).y + (Rand_ZeroOne() - 0.5f32) * randRangeDiameter;
        pos.z = (*posBase).z + (Rand_ZeroOne() - 0.5f32) * randRangeDiameter;
        scale =
            ((Rand_ZeroOne() * scaleBase as libc::c_int as libc::c_float *
                  0.2f32) as s16 as libc::c_int + scaleBase as libc::c_int) as
                s16;
        var2 = arg6 as u32_0;
        if var2 != 0 as libc::c_int as libc::c_uint {
            func_800286CC(globalCtx, &mut pos, &mut velocity, &mut accel,
                          scale, scaleStep);
        } else {
            func_8002865C(globalCtx, &mut pos, &mut velocity, &mut accel,
                          scale, scaleStep);
        }
        i -= 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn Actor_GetCollidedExplosive(mut globalCtx:
                                                        *mut GlobalContext,
                                                    mut collider:
                                                        *mut Collider)
 -> *mut Actor {
    if (*collider).acFlags as libc::c_int &
           (1 as libc::c_int) << 1 as libc::c_int != 0 &&
           (*(*collider).ac).category as libc::c_int ==
               ACTORCAT_EXPLOSIVE as libc::c_int {
        (*collider).acFlags =
            ((*collider).acFlags as libc::c_int &
                 !((1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
        return (*collider).ac
    }
    return 0 as *mut Actor;
}
#[no_mangle]
pub unsafe extern "C" fn func_80033684(mut globalCtx: *mut GlobalContext,
                                       mut explosiveActor: *mut Actor)
 -> *mut Actor {
    let mut actor: *mut Actor =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_EXPLOSIVE as libc::c_int as
                                             usize].head;
    while !actor.is_null() {
        if actor == explosiveActor ||
               (*actor).params as libc::c_int != 1 as libc::c_int {
            actor = (*actor).next
        } else if Actor_WorldDistXYZToActor(explosiveActor, actor) <=
                      ((*actor).shape.rot.z as libc::c_int *
                           10 as libc::c_int) as libc::c_float + 80.0f32 {
            return actor
        } else { actor = (*actor).next }
    }
    return 0 as *mut Actor;
}
/* *
 * Dynamically changes the category of a given actor instance.
 * This is done by moving it to the corresponding category list and setting its category variable accordingly.
 */
#[no_mangle]
pub unsafe extern "C" fn Actor_ChangeCategory(mut globalCtx:
                                                  *mut GlobalContext,
                                              mut actorCtx: *mut ActorContext,
                                              mut actor: *mut Actor,
                                              mut actorCategory: u8_0) {
    Actor_RemoveFromCategory(globalCtx, actorCtx, actor);
    Actor_AddToCategory(actorCtx, actor, actorCategory);
}
/* *
 * Checks if a hookshot or arrow actor is going to collide with the cylinder denoted by the
 * actor's `cylRadius` and `cylHeight`.
 * The check is only peformed if the projectile actor is within the provided sphere radius.
 *
 * Returns the actor if there will be collision, NULL otherwise.
 */
#[no_mangle]
pub unsafe extern "C" fn Actor_GetProjectileActor(mut globalCtx:
                                                      *mut GlobalContext,
                                                  mut refActor: *mut Actor,
                                                  mut radius: f32_0)
 -> *mut Actor {
    let mut actor: *mut Actor = 0 as *mut Actor;
    let mut spA8: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut deltaX: f32_0 = 0.;
    let mut deltaY: f32_0 = 0.;
    let mut deltaZ: f32_0 = 0.;
    let mut sp90: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp84: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    actor =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_ITEMACTION as libc::c_int as
                                             usize].head;
    while !actor.is_null() {
        if (*actor).id as libc::c_int != ACTOR_ARMS_HOOK as libc::c_int &&
               (*actor).id as libc::c_int != ACTOR_EN_ARROW as libc::c_int ||
               actor == refActor {
            actor = (*actor).next
        } else if Math_Vec3f_DistXYZ(&mut (*refActor).world.pos,
                                     &mut (*actor).world.pos) > radius ||
                      (*(actor as *mut ArmsHook)).timer as libc::c_int ==
                          0 as libc::c_int {
            actor = (*actor).next
        } else {
            deltaX =
                Math_SinS((*actor).world.rot.y) *
                    ((*actor).speedXZ * 10.0f32);
            deltaY = (*actor).velocity.y + (*actor).gravity * 10.0f32;
            deltaZ =
                Math_CosS((*actor).world.rot.y) *
                    ((*actor).speedXZ * 10.0f32);
            spA8.x = (*actor).world.pos.x + deltaX;
            spA8.y = (*actor).world.pos.y + deltaY;
            spA8.z = (*actor).world.pos.z + deltaZ;
            if CollisionCheck_CylSideVsLineSeg((*refActor).colChkInfo.cylRadius
                                                   as f32_0,
                                               (*refActor).colChkInfo.cylHeight
                                                   as f32_0, 0.0f32,
                                               &mut (*refActor).world.pos,
                                               &mut (*actor).world.pos,
                                               &mut spA8, &mut sp90,
                                               &mut sp84) != 0 {
                return actor
            } else { actor = (*actor).next }
        }
    }
    return 0 as *mut Actor;
}
// ! @bug The projectile actor gets unsafely casted to a hookshot to check its timer, even though
            //  it can also be an arrow.
            //  Luckily, the field at the same offset in the arrow actor is the x component of a vector
            //  which will rarely ever be 0. So it's very unlikely for this bug to cause an issue.
/* *
 * Sets the actor's text id with a dynamic prefix based on the current scene.
 */
#[no_mangle]
pub unsafe extern "C" fn Actor_SetTextWithPrefix(mut globalCtx:
                                                     *mut GlobalContext,
                                                 mut actor: *mut Actor,
                                                 mut baseTextId: s16) {
    let mut prefix: s16 = 0;
    match (*globalCtx).sceneNum as libc::c_int {
        0 | 17 | 20 | 38 | 39 | 40 | 41 | 45 | 52 | 85 | 86 | 91 | 112 => {
            prefix = 0x1000 as libc::c_int as s16
        }
        54 | 81 | 99 => { prefix = 0x2000 as libc::c_int as s16 }
        4 | 18 | 21 | 96 | 97 | 98 => {
            prefix = 0x3000 as libc::c_int as s16
        }
        2 | 19 | 84 | 88 | 89 => { prefix = 0x4000 as libc::c_int as s16 }
        7 | 24 | 42 | 43 | 53 | 58 | 63 | 72 | 82 | 83 => {
            prefix = 0x5000 as libc::c_int as s16
        }
        6 | 23 | 55 | 57 | 87 | 90 | 92 => {
            prefix = 0x6000 as libc::c_int as s16
        }
        27 | 30 | 31 | 32 | 33 | 34 | 95 => {
            prefix = 0x7000 as libc::c_int as s16
        }
        _ => { prefix = 0 as libc::c_int as s16 }
    }
    (*actor).textId =
        (prefix as libc::c_int | baseTextId as libc::c_int) as u16_0;
}
/* *
 * Checks if a given actor will be standing on the ground after being translated
 * by the provided distance and angle.
 *
 * Returns true if the actor will be standing on ground.
 */
#[no_mangle]
pub unsafe extern "C" fn Actor_TestFloorInDirection(mut actor: *mut Actor,
                                                    mut globalCtx:
                                                        *mut GlobalContext,
                                                    mut distance: f32_0,
                                                    mut angle: s16) -> s16 {
    let mut ret: s16 = 0;
    let mut prevBgCheckFlags: s16 = 0;
    let mut dx: f32_0 = 0.;
    let mut dz: f32_0 = 0.;
    let mut prevActorPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    Math_Vec3f_Copy(&mut prevActorPos, &mut (*actor).world.pos);
    prevBgCheckFlags = (*actor).bgCheckFlags as s16;
    dx = Math_SinS(angle) * distance;
    dz = Math_CosS(angle) * distance;
    (*actor).world.pos.x += dx;
    (*actor).world.pos.z += dz;
    Actor_UpdateBgCheckInfo(globalCtx, actor, 0.0f32, 0.0f32, 0.0f32,
                            4 as libc::c_int);
    Math_Vec3f_Copy(&mut (*actor).world.pos, &mut prevActorPos);
    ret = ((*actor).bgCheckFlags as libc::c_int & 1 as libc::c_int) as s16;
    (*actor).bgCheckFlags = prevBgCheckFlags as u16_0;
    return ret;
}
/* *
 * Returns true if the player is targeting the provided actor
 */
#[no_mangle]
pub unsafe extern "C" fn Actor_IsTargeted(mut globalCtx: *mut GlobalContext,
                                          mut actor: *mut Actor) -> s32 {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    if (*player).stateFlags1 & 0x10 as libc::c_int as libc::c_uint != 0 &&
           (*actor).isTargeted as libc::c_int != 0 {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
/* *
 * Returns true if the player is targeting an actor other than the provided actor
 */
#[no_mangle]
pub unsafe extern "C" fn Actor_OtherIsTargeted(mut globalCtx:
                                                   *mut GlobalContext,
                                               mut actor: *mut Actor) -> s32 {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    if (*player).stateFlags1 & 0x10 as libc::c_int as libc::c_uint != 0 &&
           (*actor).isTargeted == 0 {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn func_80033AEC(mut arg0: *mut Vec3f,
                                       mut arg1: *mut Vec3f, mut arg2: f32_0,
                                       mut arg3: f32_0, mut arg4: f32_0,
                                       mut arg5: f32_0) -> f32_0 {
    let mut ret: f32_0 = 0.0f32;
    if arg4 <= Math_Vec3f_DistXYZ(arg0, arg1) {
        ret =
            Math_SmoothStepToF(&mut (*arg1).x, (*arg0).x, arg2, arg3, 0.0f32);
        ret +=
            Math_SmoothStepToF(&mut (*arg1).y, (*arg0).y, arg2, arg3, 0.0f32);
        ret +=
            Math_SmoothStepToF(&mut (*arg1).z, (*arg0).z, arg2, arg3, 0.0f32)
    } else if arg5 < Math_Vec3f_DistXYZ(arg0, arg1) {
        ret =
            Math_SmoothStepToF(&mut (*arg1).x, (*arg0).x, arg2, arg3, 0.0f32);
        ret +=
            Math_SmoothStepToF(&mut (*arg1).y, (*arg0).y, arg2, arg3, 0.0f32);
        ret +=
            Math_SmoothStepToF(&mut (*arg1).z, (*arg0).z, arg2, arg3, 0.0f32)
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn func_80033C30(mut arg0: *mut Vec3f,
                                       mut arg1: *mut Vec3f, mut alpha: u8_0,
                                       mut globalCtx: *mut GlobalContext) {
    let mut sp60: MtxF = MtxF{mf: [[0.; 4]; 4],};
    let mut var: f32_0 = 0.;
    let mut sp50: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp4C: *mut CollisionPoly = 0 as *mut CollisionPoly;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_actor.c\x00" as *const u8 as *const libc::c_char,
                    8120 as libc::c_int);
    // Necessary to match
    (*__gfxCtx).polyOpa.p =
        Gfx_CallSetupDL((*__gfxCtx).polyOpa.p, 0x2c as libc::c_int as u32_0);
    let fresh90 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g: *mut Gfx = fresh90;
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
            (alpha as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    sp50.x = (*arg0).x;
    sp50.y = (*arg0).y + 1.0f32;
    sp50.z = (*arg0).z;
    var =
        BgCheck_EntityRaycastFloor2(globalCtx, &mut (*globalCtx).colCtx,
                                    &mut sp4C, &mut sp50);
    if !sp4C.is_null() {
        func_80038A28(sp4C, (*arg0).x, var, (*arg0).z, &mut sp60);
        Matrix_Put(&mut sp60);
    } else {
        Matrix_Translate((*arg0).x, (*arg0).y, (*arg0).z,
                         MTXMODE_NEW as libc::c_int as u8_0);
    }
    Matrix_Scale((*arg1).x, 1.0f32, (*arg1).z,
                 MTXMODE_APPLY as libc::c_int as u8_0);
    let fresh91 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_0: *mut Gfx = fresh91;
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
            (((0 as libc::c_int | 0x2 as libc::c_int) ^ 0x1 as libc::c_int) as
                 u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_0).words.w1 =
        Matrix_NewMtx((*globalCtx).state.gfxCtx,
                      b"../z_actor.c\x00" as *const u8 as *const libc::c_char
                          as *mut libc::c_char, 8149 as libc::c_int) as
            libc::c_uint;
    let fresh92 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_1: *mut Gfx = fresh92;
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
    (*_g_1).words.w1 = gCircleShadowDL.as_mut_ptr() as libc::c_uint;
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_actor.c\x00" as *const u8 as *const libc::c_char,
                     8155 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn func_80033DB8(mut globalCtx: *mut GlobalContext,
                                       mut arg1: s16, mut arg2: s16) {
    let mut var: s16 =
        Quake_Add(&mut (*globalCtx).mainCamera, 3 as libc::c_int as u32_0);
    Quake_SetSpeed(var, 20000 as libc::c_int as s16);
    Quake_SetQuakeValues(var, arg1, 0 as libc::c_int as s16,
                         0 as libc::c_int as s16, 0 as libc::c_int as s16);
    Quake_SetCountdown(var, arg2);
}
#[no_mangle]
pub unsafe extern "C" fn func_80033E1C(mut globalCtx: *mut GlobalContext,
                                       mut arg1: s16, mut arg2: s16,
                                       mut arg3: s16) {
    let mut var: s16 =
        Quake_Add(&mut (*globalCtx).mainCamera, 3 as libc::c_int as u32_0);
    Quake_SetSpeed(var, arg3);
    Quake_SetQuakeValues(var, arg1, 0 as libc::c_int as s16,
                         0 as libc::c_int as s16, 0 as libc::c_int as s16);
    Quake_SetCountdown(var, arg2);
}
#[no_mangle]
pub unsafe extern "C" fn func_80033E88(mut actor: *mut Actor,
                                       mut globalCtx: *mut GlobalContext,
                                       mut arg2: s16, mut arg3: s16) {
    if arg2 as libc::c_int >= 5 as libc::c_int {
        func_800AA000((*actor).xyzDistToPlayerSq, 0xff as libc::c_int as u8_0,
                      0x14 as libc::c_int as u8_0,
                      0x96 as libc::c_int as u8_0);
    } else {
        func_800AA000((*actor).xyzDistToPlayerSq, 0xb4 as libc::c_int as u8_0,
                      0x14 as libc::c_int as u8_0,
                      0x64 as libc::c_int as u8_0);
    }
    func_80033DB8(globalCtx, arg2, arg3);
}
#[no_mangle]
pub unsafe extern "C" fn Rand_ZeroFloat(mut f: f32_0) -> f32_0 {
    return Rand_ZeroOne() * f;
}
#[no_mangle]
pub unsafe extern "C" fn Rand_CenteredFloat(mut f: f32_0) -> f32_0 {
    return (Rand_ZeroOne() - 0.5f32) * f;
}
// size = 0x1C
static mut sDoorLocksInfo: [DoorLockInfo; 3] =
    unsafe {
        [{
             let mut init =
                 DoorLockInfo{chainAngle: 0.54f32,
                              chainLength: 6000.0f32,
                              yShift: 5000.0f32,
                              chainsScale: 1.0f32,
                              chainsRotZInit: 0.0f32,
                              chainDL: gDoorChainsDL.as_ptr() as *mut _,
                              lockDL: gDoorLockDL.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 DoorLockInfo{chainAngle: 0.644f32,
                              chainLength: 12000.0f32,
                              yShift: 8000.0f32,
                              chainsScale: 1.0f32,
                              chainsRotZInit: 0.0f32,
                              chainDL:
                                  object_bdoor_DL_001530.as_ptr() as *mut _,
                              lockDL:
                                  object_bdoor_DL_001400.as_ptr() as *mut _,};
             init
         },
         {
             let mut init =
                 DoorLockInfo{chainAngle: 0.64000005f32,
                              chainLength: 8500.0f32,
                              yShift: 8000.0f32,
                              chainsScale: 1.75f32,
                              chainsRotZInit: 0.1f32,
                              chainDL: gDoorChainsDL.as_ptr() as *mut _,
                              lockDL: gDoorLockDL.as_ptr() as *mut _,};
             init
         }]
    };
/* *
 * Draws chains and lock of a locked door, of the specified `type` (see `DoorLockType`).
 * `frame` can be 0 to 10, where 0 is "open" and 10 is "closed", the chains slide accordingly.
 */
#[no_mangle]
pub unsafe extern "C" fn Actor_DrawDoorLock(mut globalCtx: *mut GlobalContext,
                                            mut frame: s32, mut type_0: s32) {
    let mut entry: *mut DoorLockInfo = 0 as *mut DoorLockInfo;
    let mut i: s32 = 0;
    let mut baseMtxF: MtxF = MtxF{mf: [[0.; 4]; 4],};
    let mut chainRotZ: f32_0 = 0.;
    let mut chainsTranslateX: f32_0 = 0.;
    let mut chainsTranslateY: f32_0 = 0.;
    let mut rotZStep: f32_0 = 0.;
    entry =
        &mut *sDoorLocksInfo.as_mut_ptr().offset(type_0 as isize) as
            *mut DoorLockInfo;
    chainRotZ = (*entry).chainsRotZInit;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_actor.c\x00" as *const u8 as *const libc::c_char,
                    8265 as libc::c_int);
    Matrix_Translate(0.0f32, (*entry).yShift, 500.0f32,
                     MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_Get(&mut baseMtxF);
    chainsTranslateX =
        sinf((*entry).chainAngle - chainRotZ) *
            -(10 as libc::c_int - frame) as libc::c_float * 0.1f32 *
            (*entry).chainLength;
    chainsTranslateY =
        cosf((*entry).chainAngle - chainRotZ) *
            (10 as libc::c_int - frame) as libc::c_float * 0.1f32 *
            (*entry).chainLength;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        Matrix_Put(&mut baseMtxF);
        Matrix_RotateZ(chainRotZ, MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_Translate(chainsTranslateX, chainsTranslateY, 0.0f32,
                         MTXMODE_APPLY as libc::c_int as u8_0);
        if (*entry).chainsScale != 1.0f32 {
            Matrix_Scale((*entry).chainsScale, (*entry).chainsScale,
                         (*entry).chainsScale,
                         MTXMODE_APPLY as libc::c_int as u8_0);
        }
        let fresh93 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g: *mut Gfx = fresh93;
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
                (((0 as libc::c_int | 0x2 as libc::c_int) ^
                      0x1 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g).words.w1 =
            Matrix_NewMtx((*globalCtx).state.gfxCtx,
                          b"../z_actor.c\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                          8299 as libc::c_int) as libc::c_uint;
        let fresh94 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_0: *mut Gfx = fresh94;
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
        (*_g_0).words.w1 = (*entry).chainDL as libc::c_uint;
        if i % 2 as libc::c_int != 0 {
            rotZStep = 2.0f32 * (*entry).chainAngle
        } else {
            rotZStep =
                3.14159265358979323846f32 - 2.0f32 * (*entry).chainAngle
        }
        chainRotZ += rotZStep;
        i += 1
    }
    Matrix_Put(&mut baseMtxF);
    Matrix_Scale(frame as libc::c_float * 0.1f32,
                 frame as libc::c_float * 0.1f32,
                 frame as libc::c_float * 0.1f32,
                 MTXMODE_APPLY as libc::c_int as u8_0);
    let fresh95 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_1: *mut Gfx = fresh95;
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
            (((0 as libc::c_int | 0x2 as libc::c_int) ^ 0x1 as libc::c_int) as
                 u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_1).words.w1 =
        Matrix_NewMtx((*globalCtx).state.gfxCtx,
                      b"../z_actor.c\x00" as *const u8 as *const libc::c_char
                          as *mut libc::c_char, 8314 as libc::c_int) as
            libc::c_uint;
    let fresh96 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_2: *mut Gfx = fresh96;
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
    (*_g_2).words.w1 = (*entry).lockDL as libc::c_uint;
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_actor.c\x00" as *const u8 as *const libc::c_char,
                     8319 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn func_8003424C(mut globalCtx: *mut GlobalContext,
                                       mut arg1: *mut Vec3f) {
    CollisionCheck_SpawnShieldParticlesMetal(globalCtx, arg1);
}
#[no_mangle]
pub unsafe extern "C" fn Actor_SetColorFilter(mut actor: *mut Actor,
                                              mut colorFlag: s16,
                                              mut colorIntensityMax: s16,
                                              mut xluFlag: s16,
                                              mut duration: s16) {
    if colorFlag as libc::c_int == 0x8000 as libc::c_int &&
           colorIntensityMax as libc::c_int & 0x8000 as libc::c_int == 0 {
        Audio_PlayActorSound2(actor, 0x3836 as libc::c_int as u16_0);
    }
    (*actor).colorFilterParams =
        (colorFlag as libc::c_int | xluFlag as libc::c_int |
             (colorIntensityMax as libc::c_int & 0xf8 as libc::c_int) <<
                 5 as libc::c_int | duration as libc::c_int) as u16_0;
    (*actor).colorFilterTimer = duration as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn func_800342EC(mut object: *mut Vec3f,
                                       mut globalCtx: *mut GlobalContext)
 -> *mut Hilite {
    let mut lightDir: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    lightDir.x = (*globalCtx).envCtx.dirLight1.params.dir.x as f32_0;
    lightDir.y = (*globalCtx).envCtx.dirLight1.params.dir.y as f32_0;
    lightDir.z = (*globalCtx).envCtx.dirLight1.params.dir.z as f32_0;
    return func_8002EABC(object, &mut (*globalCtx).view.eye, &mut lightDir,
                         (*globalCtx).state.gfxCtx);
}
#[no_mangle]
pub unsafe extern "C" fn func_8003435C(mut object: *mut Vec3f,
                                       mut globalCtx: *mut GlobalContext)
 -> *mut Hilite {
    let mut lightDir: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    lightDir.x = (*globalCtx).envCtx.dirLight1.params.dir.x as f32_0;
    lightDir.y = (*globalCtx).envCtx.dirLight1.params.dir.y as f32_0;
    lightDir.z = (*globalCtx).envCtx.dirLight1.params.dir.z as f32_0;
    return func_8002EB44(object, &mut (*globalCtx).view.eye, &mut lightDir,
                         (*globalCtx).state.gfxCtx);
}
#[no_mangle]
pub unsafe extern "C" fn func_800343CC(mut globalCtx: *mut GlobalContext,
                                       mut actor: *mut Actor,
                                       mut arg2: *mut s16,
                                       mut interactRange: f32_0,
                                       mut unkFunc1: callback1_800343CC,
                                       mut unkFunc2: callback2_800343CC)
 -> s32 {
    let mut x: s16 = 0;
    let mut y: s16 = 0;
    if Actor_ProcessTalkRequest(actor, globalCtx) != 0 {
        *arg2 = 1 as libc::c_int as s16;
        return 1 as libc::c_int
    }
    if *arg2 as libc::c_int != 0 as libc::c_int {
        *arg2 =
            unkFunc2.expect("non-null function pointer")(globalCtx, actor);
        return 0 as libc::c_int
    }
    Actor_GetScreenPos(globalCtx, actor, &mut x, &mut y);
    if (x as libc::c_int) < 0 as libc::c_int ||
           x as libc::c_int > 320 as libc::c_int ||
           (y as libc::c_int) < 0 as libc::c_int ||
           y as libc::c_int > 240 as libc::c_int {
        return 0 as libc::c_int
    }
    if func_8002F2CC(actor, globalCtx, interactRange) == 0 {
        return 0 as libc::c_int
    }
    (*actor).textId =
        unkFunc1.expect("non-null function pointer")(globalCtx, actor);
    return 0 as libc::c_int;
}
// size = 0x18
static mut D_80116130: [struct_80116130; 13] =
    [{
         let mut init =
             struct_80116130{sub_00:
                                 {
                                     let mut init =
                                         struct_80116130_0{unk_00:
                                                               0x2aa8 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_02:
                                                               0xf1c8 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_04:
                                                               0x18e2 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_06:
                                                               0x1554 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_08:
                                                               0 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_0A:
                                                               0 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_0C:
                                                               1 as
                                                                   libc::c_int
                                                                   as u8_0,};
                                     init
                                 },
                             unk_10: 170.0f32,
                             unk_14: 0x3ffc as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             struct_80116130{sub_00:
                                 {
                                     let mut init =
                                         struct_80116130_0{unk_00:
                                                               0x2aa8 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_02:
                                                               0xeaac as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_04:
                                                               0x1554 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_06:
                                                               0x1554 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_08:
                                                               0xf8e4 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_0A:
                                                               0xe38 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_0C:
                                                               1 as
                                                                   libc::c_int
                                                                   as u8_0,};
                                     init
                                 },
                             unk_10: 170.0f32,
                             unk_14: 0x3ffc as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             struct_80116130{sub_00:
                                 {
                                     let mut init =
                                         struct_80116130_0{unk_00:
                                                               0x31c4 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_02:
                                                               0xe390 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_04:
                                                               0xe38 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_06:
                                                               0xe38 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_08:
                                                               0xf1c8 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_0A:
                                                               0x71c as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_0C:
                                                               1 as
                                                                   libc::c_int
                                                                   as u8_0,};
                                     init
                                 },
                             unk_10: 170.0f32,
                             unk_14: 0x3ffc as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             struct_80116130{sub_00:
                                 {
                                     let mut init =
                                         struct_80116130_0{unk_00:
                                                               0x1554 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_02:
                                                               0xf1c8 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_04:
                                                               0 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_06:
                                                               0x71c as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_08:
                                                               0xf8e4 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_0A:
                                                               0 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_0C:
                                                               1 as
                                                                   libc::c_int
                                                                   as u8_0,};
                                     init
                                 },
                             unk_10: 170.0f32,
                             unk_14: 0x3ffc as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             struct_80116130{sub_00:
                                 {
                                     let mut init =
                                         struct_80116130_0{unk_00:
                                                               0x2aa8 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_02:
                                                               0xf8e4 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_04:
                                                               0x71c as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_06:
                                                               0xe38 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_08:
                                                               0xd558 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_0A:
                                                               0x2aa8 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_0C:
                                                               1 as
                                                                   libc::c_int
                                                                   as u8_0,};
                                     init
                                 },
                             unk_10: 170.0f32,
                             unk_14: 0x3ffc as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             struct_80116130{sub_00:
                                 {
                                     let mut init =
                                         struct_80116130_0{unk_00:
                                                               0 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_02:
                                                               0xe390 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_04:
                                                               0x2aa8 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_06:
                                                               0x3ffc as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_08:
                                                               0xf1c8 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_0A:
                                                               0xe38 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_0C:
                                                               1 as
                                                                   libc::c_int
                                                                   as u8_0,};
                                     init
                                 },
                             unk_10: 170.0f32,
                             unk_14: 0x3ffc as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             struct_80116130{sub_00:
                                 {
                                     let mut init =
                                         struct_80116130_0{unk_00:
                                                               0x2aa8 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_02:
                                                               0xf1c8 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_04:
                                                               0xe38 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_06:
                                                               0xe38 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_08:
                                                               0 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_0A:
                                                               0 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_0C:
                                                               1 as
                                                                   libc::c_int
                                                                   as u8_0,};
                                     init
                                 },
                             unk_10: 0.0f32,
                             unk_14: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             struct_80116130{sub_00:
                                 {
                                     let mut init =
                                         struct_80116130_0{unk_00:
                                                               0x2aa8 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_02:
                                                               0xf1c8 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_04:
                                                               0 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_06:
                                                               0xe38 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_08:
                                                               0 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_0A:
                                                               0x1c70 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_0C:
                                                               1 as
                                                                   libc::c_int
                                                                   as u8_0,};
                                     init
                                 },
                             unk_10: 0.0f32,
                             unk_14: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             struct_80116130{sub_00:
                                 {
                                     let mut init =
                                         struct_80116130_0{unk_00:
                                                               0x2aa8 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_02:
                                                               0xf1c8 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_04:
                                                               0xf1c8 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_06:
                                                               0 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_08:
                                                               0 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_0A:
                                                               0 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_0C:
                                                               1 as
                                                                   libc::c_int
                                                                   as u8_0,};
                                     init
                                 },
                             unk_10: 0.0f32,
                             unk_14: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             struct_80116130{sub_00:
                                 {
                                     let mut init =
                                         struct_80116130_0{unk_00:
                                                               0x71c as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_02:
                                                               0xf1c8 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_04:
                                                               0xe38 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_06:
                                                               0x1c70 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_08:
                                                               0 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_0A:
                                                               0 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_0C:
                                                               1 as
                                                                   libc::c_int
                                                                   as u8_0,};
                                     init
                                 },
                             unk_10: 0.0f32,
                             unk_14: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             struct_80116130{sub_00:
                                 {
                                     let mut init =
                                         struct_80116130_0{unk_00:
                                                               0xe38 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_02:
                                                               0xf1c8 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_04:
                                                               0 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_06:
                                                               0x1c70 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_08:
                                                               0 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_0A:
                                                               0xe38 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_0C:
                                                               1 as
                                                                   libc::c_int
                                                                   as u8_0,};
                                     init
                                 },
                             unk_10: 0.0f32,
                             unk_14: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             struct_80116130{sub_00:
                                 {
                                     let mut init =
                                         struct_80116130_0{unk_00:
                                                               0x2aa8 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_02:
                                                               0xe390 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_04:
                                                               0x1c70 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_06:
                                                               0xe38 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_08:
                                                               0xf1c8 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_0A:
                                                               0xe38 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_0C:
                                                               1 as
                                                                   libc::c_int
                                                                   as u8_0,};
                                     init
                                 },
                             unk_10: 0.0f32,
                             unk_14: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             struct_80116130{sub_00:
                                 {
                                     let mut init =
                                         struct_80116130_0{unk_00:
                                                               0x18e2 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_02:
                                                               0xf1c8 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_04:
                                                               0xe38 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_06:
                                                               0xe38 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_08:
                                                               0 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_0A:
                                                               0 as
                                                                   libc::c_int
                                                                   as s16,
                                                           unk_0C:
                                                               1 as
                                                                   libc::c_int
                                                                   as u8_0,};
                                     init
                                 },
                             unk_10: 0.0f32,
                             unk_14: 0 as libc::c_int as s16,};
         init
     }];
#[no_mangle]
pub unsafe extern "C" fn func_800344BC(mut actor: *mut Actor,
                                       mut arg1: *mut struct_80034A14_arg1,
                                       mut arg2: s16, mut arg3: s16,
                                       mut arg4: s16, mut arg5: s16,
                                       mut arg6: s16, mut arg7: s16,
                                       mut arg8: u8_0) {
    let mut sp46: s16 = 0;
    let mut sp44: s16 = 0;
    let mut temp2: s16 = 0;
    let mut sp40: s16 = 0;
    let mut temp1: s16 = 0;
    let mut sp30: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    sp30.x = (*actor).world.pos.x;
    sp30.y = (*actor).world.pos.y + (*arg1).unk_14;
    sp30.z = (*actor).world.pos.z;
    sp46 = Math_Vec3f_Pitch(&mut sp30, &mut (*arg1).unk_18);
    sp44 = Math_Vec3f_Yaw(&mut sp30, &mut (*arg1).unk_18);
    sp40 =
        (Math_Vec3f_Yaw(&mut (*actor).world.pos, &mut (*arg1).unk_18) as
             libc::c_int - (*actor).shape.rot.y as libc::c_int) as s16;
    temp1 =
        if (sp40 as libc::c_int) < -(arg2 as libc::c_int) {
            -(arg2 as libc::c_int)
        } else if sp40 as libc::c_int > arg2 as libc::c_int {
            arg2 as libc::c_int
        } else { sp40 as libc::c_int } as s16;
    Math_SmoothStepToS(&mut (*arg1).unk_08.y, temp1, 6 as libc::c_int as s16,
                       2000 as libc::c_int as s16, 1 as libc::c_int as s16);
    temp1 =
        if (if sp40 as libc::c_int >= 0 as libc::c_int {
                sp40 as libc::c_int
            } else { -(sp40 as libc::c_int) }) >= 0x8000 as libc::c_int {
            0 as libc::c_int
        } else if sp40 as libc::c_int >= 0 as libc::c_int {
            sp40 as libc::c_int
        } else { -(sp40 as libc::c_int) } as s16;
    (*arg1).unk_08.y =
        if ((*arg1).unk_08.y as libc::c_int) < -(temp1 as libc::c_int) {
            -(temp1 as libc::c_int)
        } else if (*arg1).unk_08.y as libc::c_int > temp1 as libc::c_int {
            temp1 as libc::c_int
        } else { (*arg1).unk_08.y as libc::c_int } as s16;
    sp40 = (sp40 as libc::c_int - (*arg1).unk_08.y as libc::c_int) as s16;
    temp1 =
        if (sp40 as libc::c_int) < -(arg5 as libc::c_int) {
            -(arg5 as libc::c_int)
        } else if sp40 as libc::c_int > arg5 as libc::c_int {
            arg5 as libc::c_int
        } else { sp40 as libc::c_int } as s16;
    Math_SmoothStepToS(&mut (*arg1).unk_0E.y, temp1, 6 as libc::c_int as s16,
                       2000 as libc::c_int as s16, 1 as libc::c_int as s16);
    temp1 =
        if (if sp40 as libc::c_int >= 0 as libc::c_int {
                sp40 as libc::c_int
            } else { -(sp40 as libc::c_int) }) >= 0x8000 as libc::c_int {
            0 as libc::c_int
        } else if sp40 as libc::c_int >= 0 as libc::c_int {
            sp40 as libc::c_int
        } else { -(sp40 as libc::c_int) } as s16;
    (*arg1).unk_0E.y =
        if ((*arg1).unk_0E.y as libc::c_int) < -(temp1 as libc::c_int) {
            -(temp1 as libc::c_int)
        } else if (*arg1).unk_0E.y as libc::c_int > temp1 as libc::c_int {
            temp1 as libc::c_int
        } else { (*arg1).unk_0E.y as libc::c_int } as s16;
    if arg8 != 0 {
        Math_SmoothStepToS(&mut (*actor).shape.rot.y, sp44,
                           6 as libc::c_int as s16,
                           2000 as libc::c_int as s16,
                           1 as libc::c_int as s16);
    }
    temp1 =
        if (sp46 as libc::c_int) < arg4 as libc::c_int {
            arg4 as libc::c_int
        } else if sp46 as libc::c_int > arg3 as u16_0 as s16 as libc::c_int {
            arg3 as u16_0 as s16 as libc::c_int
        } else { sp46 as libc::c_int } as s16;
    Math_SmoothStepToS(&mut (*arg1).unk_08.x, temp1, 6 as libc::c_int as s16,
                       2000 as libc::c_int as s16, 1 as libc::c_int as s16);
    temp2 = (sp46 as libc::c_int - (*arg1).unk_08.x as libc::c_int) as s16;
    temp1 =
        if (temp2 as libc::c_int) < arg7 as libc::c_int {
            arg7 as libc::c_int
        } else if temp2 as libc::c_int > arg6 as libc::c_int {
            arg6 as libc::c_int
        } else { temp2 as libc::c_int } as s16;
    Math_SmoothStepToS(&mut (*arg1).unk_0E.x, temp1, 6 as libc::c_int as s16,
                       2000 as libc::c_int as s16, 1 as libc::c_int as s16);
}
#[no_mangle]
pub unsafe extern "C" fn func_800347E8(mut arg0: s16) -> s16 {
    return D_80116130[arg0 as usize].unk_14;
}
#[no_mangle]
pub unsafe extern "C" fn func_80034810(mut actor: *mut Actor,
                                       mut arg1: *mut struct_80034A14_arg1,
                                       mut arg2: f32_0, mut arg3: s16,
                                       mut arg4: s16) -> s16 {
    let mut pad: s32 = 0;
    let mut var: s16 = 0;
    let mut abs_var: s16 = 0;
    if arg4 as libc::c_int != 0 as libc::c_int { return arg4 }
    if (*arg1).unk_00 as libc::c_int != 0 as libc::c_int {
        return 4 as libc::c_int as s16
    }
    if arg2 < Math_Vec3f_DistXYZ(&mut (*actor).world.pos, &mut (*arg1).unk_18)
       {
        (*arg1).unk_04 = 0 as libc::c_int as s16;
        (*arg1).unk_06 = 0 as libc::c_int as s16;
        return 1 as libc::c_int as s16
    }
    var = Math_Vec3f_Yaw(&mut (*actor).world.pos, &mut (*arg1).unk_18);
    abs_var =
        if (var as f32_0 -
                (*actor).shape.rot.y as libc::c_int as libc::c_float) as s16
               as libc::c_int >= 0 as libc::c_int {
            (var as f32_0 -
                 (*actor).shape.rot.y as libc::c_int as libc::c_float) as s16
                as libc::c_int
        } else {
            -((var as f32_0 -
                   (*actor).shape.rot.y as libc::c_int as libc::c_float) as
                  s16 as libc::c_int)
        } as s16;
    if arg3 as libc::c_int >= abs_var as libc::c_int {
        (*arg1).unk_04 = 0 as libc::c_int as s16;
        (*arg1).unk_06 = 0 as libc::c_int as s16;
        return 2 as libc::c_int as s16
    }
    if (if (*arg1).unk_04 as libc::c_int == 0 as libc::c_int {
            0 as libc::c_int
        } else { (*arg1).unk_04 -= 1; (*arg1).unk_04 as libc::c_int }) !=
           0 as libc::c_int {
        return (*arg1).unk_02
    }
    match (*arg1).unk_06 as libc::c_int {
        0 | 2 => {
            (*arg1).unk_04 =
                Rand_S16Offset(30 as libc::c_int as s16,
                               30 as libc::c_int as s16);
            (*arg1).unk_06 += 1;
            return 1 as libc::c_int as s16
        }
        1 => {
            (*arg1).unk_04 =
                Rand_S16Offset(10 as libc::c_int as s16,
                               10 as libc::c_int as s16);
            (*arg1).unk_06 += 1;
            return 3 as libc::c_int as s16
        }
        _ => { }
    }
    return 4 as libc::c_int as s16;
}
#[no_mangle]
pub unsafe extern "C" fn func_80034A14(mut actor: *mut Actor,
                                       mut arg1: *mut struct_80034A14_arg1,
                                       mut arg2: s16, mut arg3: s16) {
    let mut sp38: struct_80116130_0 =
        struct_80116130_0{unk_00: 0,
                          unk_02: 0,
                          unk_04: 0,
                          unk_06: 0,
                          unk_08: 0,
                          unk_0A: 0,
                          unk_0C: 0,};
    (*arg1).unk_02 =
        func_80034810(actor, arg1, D_80116130[arg2 as usize].unk_10,
                      D_80116130[arg2 as usize].unk_14, arg3);
    sp38 = D_80116130[arg2 as usize].sub_00;
    let mut current_block_8: u64;
    match (*arg1).unk_02 as libc::c_int {
        1 => {
            sp38.unk_00 = 0 as libc::c_int as s16;
            sp38.unk_04 = 0 as libc::c_int as s16;
            sp38.unk_02 = 0 as libc::c_int as s16;
            current_block_8 = 12116240597457431448;
        }
        3 => { current_block_8 = 12116240597457431448; }
        2 => { current_block_8 = 17649788095959871570; }
        _ => { current_block_8 = 1917311967535052937; }
    }
    match current_block_8 {
        12116240597457431448 => {
            sp38.unk_06 = 0 as libc::c_int as s16;
            sp38.unk_0A = 0 as libc::c_int as s16;
            sp38.unk_08 = 0 as libc::c_int as s16;
            current_block_8 = 17649788095959871570;
        }
        _ => { }
    }
    match current_block_8 {
        17649788095959871570 => { sp38.unk_0C = 0 as libc::c_int as u8_0 }
        _ => { }
    }
    func_800344BC(actor, arg1, sp38.unk_00, sp38.unk_04, sp38.unk_02,
                  sp38.unk_06, sp38.unk_0A, sp38.unk_08, sp38.unk_0C);
}
#[no_mangle]
pub unsafe extern "C" fn func_80034B28(mut gfxCtx: *mut GraphicsContext)
 -> *mut Gfx {
    let mut displayList: *mut Gfx = 0 as *mut Gfx;
    displayList =
        Graph_Alloc(gfxCtx,
                    ::std::mem::size_of::<Gfx>() as libc::c_ulong as size_t)
            as *mut Gfx;
    let mut _g: *mut Gfx = displayList;
    (*_g).words.w0 =
        (0xdf as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g).words.w1 = 0 as libc::c_int as libc::c_uint;
    return displayList;
}
#[no_mangle]
pub unsafe extern "C" fn func_80034B54(mut gfxCtx: *mut GraphicsContext)
 -> *mut Gfx {
    let mut displayListHead: *mut Gfx = 0 as *mut Gfx;
    let mut displayList: *mut Gfx = 0 as *mut Gfx;
    displayListHead =
        Graph_Alloc(gfxCtx,
                    (2 as libc::c_int as
                         libc::c_uint).wrapping_mul(::std::mem::size_of::<Gfx>()
                                                        as libc::c_ulong) as
                        size_t) as *mut Gfx;
    displayList = displayListHead;
    let fresh97 = displayListHead;
    displayListHead = displayListHead.offset(1);
    let mut _g: *mut Gfx = fresh97;
    (*_g).words.w0 =
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
    (*_g).words.w1 =
        ((3 as libc::c_int) << 30 as libc::c_int |
             (2 as libc::c_int) << 26 as libc::c_int |
             (0 as libc::c_int) << 22 as libc::c_int |
             (0 as libc::c_int) << 18 as libc::c_int |
             (0x8 as libc::c_int | 0x10 as libc::c_int | 0x20 as libc::c_int |
                  0x40 as libc::c_int | 0x80 as libc::c_int |
                  0x100 as libc::c_int | 0x800 as libc::c_int |
                  0x4000 as libc::c_int |
                  (0 as libc::c_int) << 28 as libc::c_int |
                  (0 as libc::c_int) << 24 as libc::c_int |
                  (1 as libc::c_int) << 20 as libc::c_int |
                  (0 as libc::c_int) << 16 as libc::c_int)) as libc::c_uint;
    let fresh98 = displayListHead;
    displayListHead = displayListHead.offset(1);
    let mut _g_0: *mut Gfx = fresh98;
    (*_g_0).words.w0 =
        (0xdf as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_0).words.w1 = 0 as libc::c_int as libc::c_uint;
    return displayList;
}
#[no_mangle]
pub unsafe extern "C" fn func_80034BA0(mut globalCtx: *mut GlobalContext,
                                       mut skelAnime: *mut SkelAnime,
                                       mut overrideLimbDraw: OverrideLimbDraw,
                                       mut postLimbDraw: PostLimbDraw,
                                       mut actor: *mut Actor,
                                       mut alpha: s16) {
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_actor.c\x00" as *const u8 as *const libc::c_char,
                    8831 as libc::c_int);
    func_80093D18((*globalCtx).state.gfxCtx);
    let fresh99 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g: *mut Gfx = fresh99;
    (*_g).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh100 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_0: *mut Gfx = fresh100;
    (*_g_0).words.w0 =
        (0xfb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
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
            (alpha as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh101 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_1: *mut Gfx = fresh101;
    (*_g_1).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_1).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh102 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_2: *mut Gfx = fresh102;
    (*_g_2).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((0xc as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_2).words.w1 =
        func_80034B28((*globalCtx).state.gfxCtx) as libc::c_uint;
    (*__gfxCtx).polyOpa.p =
        SkelAnime_DrawFlex(globalCtx, (*skelAnime).skeleton,
                           (*skelAnime).jointTable,
                           (*skelAnime).dListCount as s32, overrideLimbDraw,
                           postLimbDraw, actor as *mut libc::c_void,
                           (*__gfxCtx).polyOpa.p);
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_actor.c\x00" as *const u8 as *const libc::c_char,
                     8860 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn func_80034CC4(mut globalCtx: *mut GlobalContext,
                                       mut skelAnime: *mut SkelAnime,
                                       mut overrideLimbDraw: OverrideLimbDraw,
                                       mut postLimbDraw: PostLimbDraw,
                                       mut actor: *mut Actor,
                                       mut alpha: s16) {
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_actor.c\x00" as *const u8 as *const libc::c_char,
                    8876 as libc::c_int);
    func_80093D84((*globalCtx).state.gfxCtx);
    let fresh103 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g: *mut Gfx = fresh103;
    (*_g).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh104 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_0: *mut Gfx = fresh104;
    (*_g_0).words.w0 =
        (0xfb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
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
            (alpha as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh105 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_1: *mut Gfx = fresh105;
    (*_g_1).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((0xc as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_1).words.w1 =
        func_80034B54((*globalCtx).state.gfxCtx) as libc::c_uint;
    (*__gfxCtx).polyXlu.p =
        SkelAnime_DrawFlex(globalCtx, (*skelAnime).skeleton,
                           (*skelAnime).jointTable,
                           (*skelAnime).dListCount as s32, overrideLimbDraw,
                           postLimbDraw, actor as *mut libc::c_void,
                           (*__gfxCtx).polyXlu.p);
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_actor.c\x00" as *const u8 as *const libc::c_char,
                     8904 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn func_80034DD4(mut actor: *mut Actor,
                                       mut globalCtx: *mut GlobalContext,
                                       mut arg2: s16, mut arg3: f32_0)
 -> s16 {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut var: f32_0 = 0.;
    if (*globalCtx).csCtx.state as libc::c_int != CS_STATE_IDLE as libc::c_int
           || gDbgCamEnabled != 0 {
        var =
            Math_Vec3f_DistXYZ(&mut (*actor).world.pos,
                               &mut (*globalCtx).view.eye) * 0.25f32
    } else {
        var =
            Math_Vec3f_DistXYZ(&mut (*actor).world.pos,
                               &mut (*player).actor.world.pos)
    }
    if arg3 < var {
        (*actor).flags &=
            !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
        Math_SmoothStepToS(&mut arg2, 0 as libc::c_int as s16,
                           6 as libc::c_int as s16,
                           0x14 as libc::c_int as s16,
                           1 as libc::c_int as s16);
    } else {
        (*actor).flags |=
            ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
        Math_SmoothStepToS(&mut arg2, 0xff as libc::c_int as s16,
                           6 as libc::c_int as s16,
                           0x14 as libc::c_int as s16,
                           1 as libc::c_int as s16);
    }
    return arg2;
}
#[no_mangle]
pub unsafe extern "C" fn func_80034EC0(mut skelAnime: *mut SkelAnime,
                                       mut animations:
                                           *mut struct_80034EC0_Entry,
                                       mut index: s32) {
    let mut frameCount: f32_0 = 0.;
    animations = animations.offset(index as isize);
    if (*animations).frameCount > 0.0f32 {
        frameCount = (*animations).frameCount
    } else {
        frameCount =
            Animation_GetLastFrame((*animations).animation as
                                       *mut libc::c_void) as f32_0
    }
    Animation_Change(skelAnime, (*animations).animation,
                     (*animations).playbackSpeed, (*animations).startFrame,
                     frameCount, (*animations).mode,
                     (*animations).transitionRate);
}
#[no_mangle]
pub unsafe extern "C" fn func_80034F54(mut globalCtx: *mut GlobalContext,
                                       mut arg1: *mut s16, mut arg2: *mut s16,
                                       mut arg3: s32) {
    let mut frames: u32_0 = (*globalCtx).gameplayFrames;
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i < arg3 {
        *arg1.offset(i as isize) =
            ((0x814 as libc::c_int + 50 as libc::c_int * i) as
                 libc::c_uint).wrapping_mul(frames) as s16;
        *arg2.offset(i as isize) =
            ((0x940 as libc::c_int + 50 as libc::c_int * i) as
                 libc::c_uint).wrapping_mul(frames) as s16;
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn Actor_Noop(mut actor: *mut Actor,
                                    mut globalCtx: *mut GlobalContext) {
}
#[no_mangle]
pub unsafe extern "C" fn func_80035124(mut actor: *mut Actor,
                                       mut globalCtx: *mut GlobalContext)
 -> s32 {
    let mut ret: s32 = 0 as libc::c_int;
    match (*actor).params as libc::c_int {
        0 => {
            if Actor_HasParent(actor, globalCtx) != 0 {
                (*actor).params = 1 as libc::c_int as s16
            } else if (*actor).bgCheckFlags as libc::c_int & 1 as libc::c_int
                          == 0 {
                Actor_MoveForward(actor);
                Math_SmoothStepToF(&mut (*actor).speedXZ, 0.0f32, 1.0f32,
                                   0.1f32, 0.0f32);
            } else if (*actor).bgCheckFlags as libc::c_int & 2 as libc::c_int
                          != 0 && (*actor).velocity.y < -4.0f32 {
                ret = 1 as libc::c_int
            } else {
                (*actor).shape.rot.z = 0 as libc::c_int as s16;
                (*actor).shape.rot.x = (*actor).shape.rot.z;
                func_8002F580(actor, globalCtx);
            }
        }
        1 => {
            if Actor_HasNoParent(actor, globalCtx) != 0 {
                (*actor).params = 0 as libc::c_int as s16
            }
        }
        _ => { }
    }
    Actor_UpdateBgCheckInfo(globalCtx, actor,
                            (*actor).colChkInfo.cylHeight as f32_0,
                            (*actor).colChkInfo.cylRadius as f32_0,
                            (*actor).colChkInfo.cylRadius as f32_0,
                            0x1d as libc::c_int);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn Gfx_DrawDListOpa(mut globalCtx: *mut GlobalContext,
                                          mut dlist: *mut Gfx) {
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_cheap_proc.c\x00" as *const u8 as
                        *const libc::c_char, 214 as libc::c_int);
    func_80093D18((*globalCtx).state.gfxCtx);
    let fresh106 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g: *mut Gfx = fresh106;
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
            (((0 as libc::c_int | 0x2 as libc::c_int) ^ 0x1 as libc::c_int) as
                 u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g).words.w1 =
        Matrix_NewMtx((*globalCtx).state.gfxCtx,
                      b"../z_cheap_proc.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      216 as libc::c_int) as libc::c_uint;
    let fresh107 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_0: *mut Gfx = fresh107;
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
    (*_g_0).words.w1 = dlist as libc::c_uint;
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_cheap_proc.c\x00" as *const u8 as
                         *const libc::c_char, 219 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Gfx_DrawDListXlu(mut globalCtx: *mut GlobalContext,
                                          mut dlist: *mut Gfx) {
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_cheap_proc.c\x00" as *const u8 as
                        *const libc::c_char, 228 as libc::c_int);
    func_80093D84((*globalCtx).state.gfxCtx);
    let fresh108 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g: *mut Gfx = fresh108;
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
            (((0 as libc::c_int | 0x2 as libc::c_int) ^ 0x1 as libc::c_int) as
                 u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g).words.w1 =
        Matrix_NewMtx((*globalCtx).state.gfxCtx,
                      b"../z_cheap_proc.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      230 as libc::c_int) as libc::c_uint;
    let fresh109 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_0: *mut Gfx = fresh109;
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
    (*_g_0).words.w1 = dlist as libc::c_uint;
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_cheap_proc.c\x00" as *const u8 as
                         *const libc::c_char, 233 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn func_800353E8(mut globalCtx: *mut GlobalContext)
 -> u8_0 {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    return (*player).unk_845;
}
/* *
 * Finds the first actor instance of a specified ID and category within a given range from
 * an actor if there is one. If the ID provided is -1, this will look for any actor of the
 * specified category rather than a specific ID.
 */
#[no_mangle]
pub unsafe extern "C" fn Actor_FindNearby(mut globalCtx: *mut GlobalContext,
                                          mut refActor: *mut Actor,
                                          mut actorId: s16,
                                          mut actorCategory: u8_0,
                                          mut range: f32_0) -> *mut Actor {
    let mut actor: *mut Actor =
        (*globalCtx).actorCtx.actorLists[actorCategory as usize].head;
    while !actor.is_null() {
        if actor == refActor ||
               actorId as libc::c_int != -(1 as libc::c_int) &&
                   actorId as libc::c_int != (*actor).id as libc::c_int {
            actor = (*actor).next
        } else if Actor_WorldDistXYZToActor(refActor, actor) <= range {
            return actor
        } else { actor = (*actor).next }
    }
    return 0 as *mut Actor;
}
#[no_mangle]
pub unsafe extern "C" fn func_800354B4(mut globalCtx: *mut GlobalContext,
                                       mut actor: *mut Actor,
                                       mut range: f32_0, mut arg3: s16,
                                       mut arg4: s16, mut arg5: s16) -> s32 {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut var1: s16 = 0;
    let mut var2: s16 = 0;
    var1 =
        (((*actor).yawTowardsPlayer as libc::c_int + 0x8000 as libc::c_int) as
             s16 as libc::c_int - (*player).actor.shape.rot.y as libc::c_int)
            as s16;
    var2 =
        ((*actor).yawTowardsPlayer as libc::c_int - arg5 as libc::c_int) as
            s16;
    if (*actor).xzDistToPlayer <= range &&
           (*player).swordState as libc::c_int != 0 as libc::c_int &&
           arg4 as libc::c_int >=
               (if var1 as libc::c_int >= 0 as libc::c_int {
                    var1 as libc::c_int
                } else { -(var1 as libc::c_int) }) &&
           arg3 as libc::c_int >=
               (if var2 as libc::c_int >= 0 as libc::c_int {
                    var2 as libc::c_int
                } else { -(var2 as libc::c_int) }) {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn func_8003555C(mut globalCtx: *mut GlobalContext,
                                       mut pos: *mut Vec3f,
                                       mut velocity: *mut Vec3f,
                                       mut accel: *mut Vec3f) {
    let mut color1: Color_RGBA8 = Color_RGBA8{r: 0, g: 0, b: 0, a: 0,};
    let mut color2: Color_RGBA8 = Color_RGBA8{r: 0, g: 0, b: 0, a: 0,};
    color1.r = 200 as libc::c_int as u8_0;
    color1.g = 160 as libc::c_int as u8_0;
    color1.b = 120 as libc::c_int as u8_0;
    color2.r = 130 as libc::c_int as u8_0;
    color2.g = 90 as libc::c_int as u8_0;
    color2.b = 50 as libc::c_int as u8_0;
    // ! @bug color1 and color2 alpha components not set before being passed on
    EffectSsKiraKira_SpawnSmall(globalCtx, pos, velocity, accel, &mut color1,
                                &mut color2);
}
#[no_mangle]
pub static mut D_80116268: Vec3f =
    { let mut init = Vec3f{x: 0.0f32, y: -1.5f32, z: 0.0f32,}; init };
#[no_mangle]
pub static mut D_80116274: Vec3f =
    { let mut init = Vec3f{x: 0.0f32, y: -0.2f32, z: 0.0f32,}; init };
#[no_mangle]
pub static mut D_80116280: [Gfx; 3] =
    [Gfx{words:
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
                                ((3 as libc::c_int) << 30 as libc::c_int |
                                     (2 as libc::c_int) << 26 as libc::c_int |
                                     (0 as libc::c_int) << 22 as libc::c_int |
                                     (0 as libc::c_int) << 18 as libc::c_int |
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
                                (0xe2 as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int |
                                    ((32 as libc::c_int - 0 as libc::c_int -
                                          2 as libc::c_int) as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) << 8 as libc::c_int
                                    |
                                    ((2 as libc::c_int - 1 as libc::c_int) as
                                         u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        0 as libc::c_int,
                            w1:
                                ((1 as libc::c_int) << 0 as libc::c_int) as
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
#[no_mangle]
pub unsafe extern "C" fn func_800355B8(mut globalCtx: *mut GlobalContext,
                                       mut pos: *mut Vec3f) {
    func_8003555C(globalCtx, pos, &mut D_80116268, &mut D_80116274);
}
#[no_mangle]
pub unsafe extern "C" fn func_800355E4(mut globalCtx: *mut GlobalContext,
                                       mut collider: *mut Collider) -> u8_0 {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    if (*collider).acFlags as libc::c_int &
           (1 as libc::c_int) << 3 as libc::c_int != 0 &&
           (*player).swordState as libc::c_int != 0 as libc::c_int &&
           (*player).swordAnimation as libc::c_int == 0x16 as libc::c_int {
        return 1 as libc::c_int as u8_0
    } else { return 0 as libc::c_int as u8_0 };
}
#[no_mangle]
pub unsafe extern "C" fn Actor_ApplyDamage(mut actor: *mut Actor) -> u8_0 {
    if (*actor).colChkInfo.damage as libc::c_int >=
           (*actor).colChkInfo.health as libc::c_int {
        (*actor).colChkInfo.health = 0 as libc::c_int as u8_0
    } else {
        (*actor).colChkInfo.health =
            ((*actor).colChkInfo.health as libc::c_int -
                 (*actor).colChkInfo.damage as libc::c_int) as u8_0
    }
    return (*actor).colChkInfo.health;
}
#[no_mangle]
pub unsafe extern "C" fn Actor_SetDropFlag(mut actor: *mut Actor,
                                           mut colInfo: *mut ColliderInfo,
                                           mut freezeFlag: s32) {
    if (*colInfo).acHitInfo.is_null() {
        (*actor).dropFlag = 0 as libc::c_int as u8_0
    } else if freezeFlag != 0 &&
                  (*(*colInfo).acHitInfo).toucher.dmgFlags &
                      0x10060000 as libc::c_int as libc::c_uint != 0 {
        (*actor).freezeTimer =
            (*(*colInfo).acHitInfo).toucher.damage as u16_0;
        (*actor).dropFlag = 0 as libc::c_int as u8_0
    } else if (*(*colInfo).acHitInfo).toucher.dmgFlags &
                  0x800 as libc::c_int as libc::c_uint != 0 {
        (*actor).dropFlag = 0x1 as libc::c_int as u8_0
    } else if (*(*colInfo).acHitInfo).toucher.dmgFlags &
                  0x1000 as libc::c_int as libc::c_uint != 0 {
        (*actor).dropFlag = 0x2 as libc::c_int as u8_0
    } else if (*(*colInfo).acHitInfo).toucher.dmgFlags &
                  0x4000 as libc::c_int as libc::c_uint != 0 {
        (*actor).dropFlag = 0x4 as libc::c_int as u8_0
    } else if (*(*colInfo).acHitInfo).toucher.dmgFlags &
                  0x8000 as libc::c_int as libc::c_uint != 0 {
        (*actor).dropFlag = 0x8 as libc::c_int as u8_0
    } else if (*(*colInfo).acHitInfo).toucher.dmgFlags &
                  0x10000 as libc::c_int as libc::c_uint != 0 {
        (*actor).dropFlag = 0x10 as libc::c_int as u8_0
    } else if (*(*colInfo).acHitInfo).toucher.dmgFlags &
                  0x2000 as libc::c_int as libc::c_uint != 0 {
        (*actor).dropFlag = 0x20 as libc::c_int as u8_0
    } else if (*(*colInfo).acHitInfo).toucher.dmgFlags &
                  0x80000 as libc::c_int as libc::c_uint != 0 {
        if freezeFlag != 0 {
            (*actor).freezeTimer =
                (*(*colInfo).acHitInfo).toucher.damage as u16_0
        }
        (*actor).dropFlag = 0x40 as libc::c_int as u8_0
    } else { (*actor).dropFlag = 0 as libc::c_int as u8_0 };
}
#[no_mangle]
pub unsafe extern "C" fn Actor_SetDropFlagJntSph(mut actor: *mut Actor,
                                                 mut jntSph:
                                                     *mut ColliderJntSph,
                                                 mut freezeFlag: s32) {
    let mut curColInfo: *mut ColliderInfo = 0 as *mut ColliderInfo;
    let mut flag: s32 = 0;
    let mut i: s32 = 0;
    (*actor).dropFlag = 0 as libc::c_int as u8_0;
    i = (*jntSph).count - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        curColInfo = &mut (*(*jntSph).elements.offset(i as isize)).info;
        if (*curColInfo).acHitInfo.is_null() {
            flag = 0 as libc::c_int
        } else if freezeFlag != 0 &&
                      (*(*curColInfo).acHitInfo).toucher.dmgFlags &
                          0x10060000 as libc::c_int as libc::c_uint != 0 {
            (*actor).freezeTimer =
                (*(*curColInfo).acHitInfo).toucher.damage as u16_0;
            flag = 0 as libc::c_int
        } else if (*(*curColInfo).acHitInfo).toucher.dmgFlags &
                      0x800 as libc::c_int as libc::c_uint != 0 {
            flag = 0x1 as libc::c_int
        } else if (*(*curColInfo).acHitInfo).toucher.dmgFlags &
                      0x1000 as libc::c_int as libc::c_uint != 0 {
            flag = 0x2 as libc::c_int
        } else if (*(*curColInfo).acHitInfo).toucher.dmgFlags &
                      0x4000 as libc::c_int as libc::c_uint != 0 {
            flag = 0x4 as libc::c_int
        } else if (*(*curColInfo).acHitInfo).toucher.dmgFlags &
                      0x8000 as libc::c_int as libc::c_uint != 0 {
            flag = 0x8 as libc::c_int
        } else if (*(*curColInfo).acHitInfo).toucher.dmgFlags &
                      0x10000 as libc::c_int as libc::c_uint != 0 {
            flag = 0x10 as libc::c_int
        } else if (*(*curColInfo).acHitInfo).toucher.dmgFlags &
                      0x2000 as libc::c_int as libc::c_uint != 0 {
            flag = 0x20 as libc::c_int
        } else if (*(*curColInfo).acHitInfo).toucher.dmgFlags &
                      0x80000 as libc::c_int as libc::c_uint != 0 {
            if freezeFlag != 0 {
                (*actor).freezeTimer =
                    (*(*curColInfo).acHitInfo).toucher.damage as u16_0
            }
            flag = 0x40 as libc::c_int
        } else { flag = 0 as libc::c_int }
        (*actor).dropFlag = ((*actor).dropFlag as libc::c_int | flag) as u8_0;
        i -= 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_80035844(mut arg0: *mut Vec3f,
                                       mut arg1: *mut Vec3f,
                                       mut arg2: *mut Vec3s, mut arg3: s32) {
    let mut dx: f32_0 = (*arg1).x - (*arg0).x;
    let mut dz: f32_0 = (*arg1).z - (*arg0).z;
    let mut dy: f32_0 =
        if arg3 != 0 {
            ((*arg1).y) - (*arg0).y
        } else { ((*arg0).y) - (*arg1).y };
    (*arg2).y = Math_Atan2S(dz, dx);
    (*arg2).x = Math_Atan2S(sqrtf(dx * dx + dz * dz), dy);
}
/* *
 * Spawns En_Part (Dissipating Flames) actor as a child of the given actor.
 */
#[no_mangle]
pub unsafe extern "C" fn func_800358DC(mut actor: *mut Actor,
                                       mut spawnPos: *mut Vec3f,
                                       mut spawnRot: *mut Vec3s,
                                       mut arg3: *mut f32_0, mut timer: s32,
                                       mut unused: *mut s16,
                                       mut globalCtx: *mut GlobalContext,
                                       mut params: s16, mut arg8: s32)
 -> *mut Actor {
    let mut spawnedEnPart: *mut EnPart = 0 as *mut EnPart;
    spawnedEnPart =
        Actor_SpawnAsChild(&mut (*globalCtx).actorCtx, actor, globalCtx,
                           ACTOR_EN_PART as libc::c_int as s16, (*spawnPos).x,
                           (*spawnPos).y, (*spawnPos).z, (*spawnRot).x,
                           (*spawnRot).y, (*actor).objBankIndex as s16,
                           params) as *mut EnPart;
    if !spawnedEnPart.is_null() {
        (*spawnedEnPart).actor.scale = (*actor).scale;
        (*spawnedEnPart).actor.speedXZ =
            *arg3.offset(0 as libc::c_int as isize);
        (*spawnedEnPart).displayList = arg8 as *mut Gfx;
        (*spawnedEnPart).action = 2 as libc::c_int as u8_0;
        (*spawnedEnPart).timer = timer as s16;
        (*spawnedEnPart).rotZ = *arg3.offset(1 as libc::c_int as isize);
        (*spawnedEnPart).rotZSpeed = *arg3.offset(2 as libc::c_int as isize);
        return &mut (*spawnedEnPart).actor
    }
    return 0 as *mut Actor;
}
#[no_mangle]
pub unsafe extern "C" fn func_800359B8(mut actor: *mut Actor, mut arg1: s16,
                                       mut arg2: *mut Vec3s) {
    let mut sp44: f32_0 = 0.;
    let mut sp40: f32_0 = 0.;
    let mut sp3C: f32_0 = 0.;
    let mut sp38: f32_0 = 0.;
    let mut sp34: f32_0 = 0.;
    let mut sp30: f32_0 = 0.;
    let mut sp2C: f32_0 = 0.;
    let mut sp28: f32_0 = 0.;
    let mut sp24: f32_0 = 0.;
    let mut floorPoly: *mut CollisionPoly = 0 as *mut CollisionPoly;
    let mut pad: s32 = 0;
    if !(*actor).floorPoly.is_null() {
        floorPoly = (*actor).floorPoly;
        sp44 =
            (*floorPoly).normal.x as libc::c_int as libc::c_float *
                (1.0f32 / 32767.0f32);
        sp40 =
            (*floorPoly).normal.y as libc::c_int as libc::c_float *
                (1.0f32 / 32767.0f32);
        sp3C =
            (*floorPoly).normal.z as libc::c_int as libc::c_float *
                (1.0f32 / 32767.0f32);
        sp38 = Math_SinS(arg1);
        sp34 = Math_CosS(arg1);
        sp28 = -(sp44 * sp38) - sp3C * sp34;
        (*arg2).x =
            -((Math_FAtan2F(sp28 * sp40, 1.0f32) *
                   (32768 as libc::c_int as libc::c_float /
                        3.14159265358979323846f32)) as s16 as libc::c_int) as
                s16;
        sp2C = Math_SinS((arg1 as libc::c_int - 16375 as libc::c_int) as s16);
        sp30 = Math_CosS((arg1 as libc::c_int - 16375 as libc::c_int) as s16);
        sp24 = -(sp44 * sp2C) - sp3C * sp30;
        (*arg2).z =
            -((Math_FAtan2F(sp24 * sp40, 1.0f32) *
                   (32768 as libc::c_int as libc::c_float /
                        3.14159265358979323846f32)) as s16 as libc::c_int) as
                s16
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_80035B18(mut globalCtx: *mut GlobalContext,
                                       mut actor: *mut Actor,
                                       mut textId: u16_0) {
    Message_ContinueTextbox(globalCtx, textId);
    (*actor).textId = textId;
}
/* *
 * Tests if event_chk_inf flag is set.
 */
#[no_mangle]
pub unsafe extern "C" fn Flags_GetEventChkInf(mut flag: s32) -> s32 {
    return gSaveContext.eventChkInf[(flag >> 4 as libc::c_int) as usize] as
               libc::c_int &
               (1 as libc::c_int) << (flag & 0xf as libc::c_int);
}
/* *
 * Sets event_chk_inf flag.
 */
#[no_mangle]
pub unsafe extern "C" fn Flags_SetEventChkInf(mut flag: s32) {
    gSaveContext.eventChkInf[(flag >> 4 as libc::c_int) as usize] =
        (gSaveContext.eventChkInf[(flag >> 4 as libc::c_int) as usize] as
             libc::c_int | (1 as libc::c_int) << (flag & 0xf as libc::c_int))
            as u16_0;
}
/* *
 * Tests if "inf_table flag is set.
 */
#[no_mangle]
pub unsafe extern "C" fn Flags_GetInfTable(mut flag: s32) -> s32 {
    return gSaveContext.infTable[(flag >> 4 as libc::c_int) as usize] as
               libc::c_int &
               (1 as libc::c_int) << (flag & 0xf as libc::c_int);
}
/* *
 * Sets "inf_table" flag.
 */
#[no_mangle]
pub unsafe extern "C" fn Flags_SetInfTable(mut flag: s32) {
    gSaveContext.infTable[(flag >> 4 as libc::c_int) as usize] =
        (gSaveContext.infTable[(flag >> 4 as libc::c_int) as usize] as
             libc::c_int | (1 as libc::c_int) << (flag & 0xf as libc::c_int))
            as u16_0;
}
#[no_mangle]
pub unsafe extern "C" fn func_80035BFC(mut globalCtx: *mut GlobalContext,
                                       mut arg1: s16) -> u32_0 {
    let mut retTextId: u16_0 = 0 as libc::c_int as u16_0;
    match arg1 as libc::c_int {
        0 => {
            if Flags_GetEventChkInf(0x9 as libc::c_int) != 0 {
                if Flags_GetInfTable(0x5 as libc::c_int) != 0 {
                    retTextId = 0x1048 as libc::c_int as u16_0
                } else { retTextId = 0x1047 as libc::c_int as u16_0 }
            } else if Flags_GetEventChkInf(0x2 as libc::c_int) != 0 {
                if Flags_GetInfTable(0x3 as libc::c_int) != 0 {
                    retTextId = 0x1032 as libc::c_int as u16_0
                } else { retTextId = 0x1031 as libc::c_int as u16_0 }
            } else if Flags_GetInfTable(0 as libc::c_int) != 0 {
                if Flags_GetInfTable(0x1 as libc::c_int) != 0 {
                    retTextId = 0x1003 as libc::c_int as u16_0
                } else { retTextId = 0x1002 as libc::c_int as u16_0 }
            } else { retTextId = 0x1001 as libc::c_int as u16_0 }
        }
        1 => {
            if !(gSaveContext.linkAge == 0 as libc::c_int) {
                if Flags_GetEventChkInf(0x9 as libc::c_int) != 0 {
                    if Flags_GetInfTable(0x10 as libc::c_int) != 0 {
                        retTextId = 0x1046 as libc::c_int as u16_0
                    } else { retTextId = 0x1045 as libc::c_int as u16_0 }
                } else if Flags_GetEventChkInf(0x3 as libc::c_int) != 0 {
                    if Flags_GetInfTable(0xe as libc::c_int) != 0 {
                        retTextId = 0x1034 as libc::c_int as u16_0
                    } else { retTextId = 0x1033 as libc::c_int as u16_0 }
                } else if Flags_GetInfTable(0xc as libc::c_int) != 0 {
                    retTextId = 0x1030 as libc::c_int as u16_0
                } else { retTextId = 0x102f as libc::c_int as u16_0 }
            } else if Flags_GetEventChkInf(0x5c as libc::c_int) != 0 {
                if Flags_GetInfTable(0x19 as libc::c_int) != 0 {
                    retTextId = 0x1071 as libc::c_int as u16_0
                } else { retTextId = 0x1070 as libc::c_int as u16_0 }
            } else if Flags_GetEventChkInf(0xb as libc::c_int) != 0 {
                if Flags_GetInfTable(0x17 as libc::c_int) != 0 {
                    retTextId = 0x1068 as libc::c_int as u16_0
                } else { retTextId = 0x1067 as libc::c_int as u16_0 }
            } else if Flags_GetInfTable(0x15 as libc::c_int) != 0 {
                retTextId = 0x1061 as libc::c_int as u16_0
            } else { retTextId = 0x1060 as libc::c_int as u16_0 }
        }
        2 => {
            if !(gSaveContext.linkAge == 0 as libc::c_int) {
                if Flags_GetEventChkInf(0x9 as libc::c_int) != 0 {
                    retTextId = 0x1042 as libc::c_int as u16_0
                } else { retTextId = 0x1004 as libc::c_int as u16_0 }
            } else if Flags_GetEventChkInf(0x5c as libc::c_int) != 0 {
                retTextId = 0x1072 as libc::c_int as u16_0
            } else if Flags_GetInfTable(0x41 as libc::c_int) != 0 {
                retTextId = 0x1055 as libc::c_int as u16_0
            } else { retTextId = 0x1056 as libc::c_int as u16_0 }
        }
        3 => {
            if !(gSaveContext.linkAge == 0 as libc::c_int) {
                if Flags_GetEventChkInf(0x9 as libc::c_int) != 0 {
                    retTextId = 0x1043 as libc::c_int as u16_0
                } else if Flags_GetInfTable(0x1e as libc::c_int) != 0 {
                    retTextId = 0x1006 as libc::c_int as u16_0
                } else { retTextId = 0x1005 as libc::c_int as u16_0 }
            } else if Flags_GetEventChkInf(0x5c as libc::c_int) != 0 {
                retTextId = 0x1073 as libc::c_int as u16_0
            } else { retTextId = 0x105a as libc::c_int as u16_0 }
        }
        4 => {
            if !(gSaveContext.linkAge == 0 as libc::c_int) {
                if Flags_GetEventChkInf(0x9 as libc::c_int) != 0 {
                    retTextId = 0x1042 as libc::c_int as u16_0
                } else { retTextId = 0x1007 as libc::c_int as u16_0 }
            } else if Flags_GetEventChkInf(0x5c as libc::c_int) != 0 {
                retTextId = 0x1072 as libc::c_int as u16_0
            } else if Flags_GetInfTable(0x47 as libc::c_int) != 0 {
                retTextId = 0x105e as libc::c_int as u16_0
            } else { retTextId = 0x105d as libc::c_int as u16_0 }
        }
        5 => {
            if !(gSaveContext.linkAge == 0 as libc::c_int) {
                if Flags_GetEventChkInf(0x9 as libc::c_int) != 0 {
                    retTextId = 0x1044 as libc::c_int as u16_0
                } else if Flags_GetInfTable(0x22 as libc::c_int) != 0 {
                    retTextId = 0x1009 as libc::c_int as u16_0
                } else { retTextId = 0x1008 as libc::c_int as u16_0 }
            } else if Flags_GetEventChkInf(0x5c as libc::c_int) != 0 {
                retTextId = 0x1075 as libc::c_int as u16_0
            } else { retTextId = 0x105b as libc::c_int as u16_0 }
        }
        6 => {
            if !(gSaveContext.linkAge == 0 as libc::c_int) {
                if Flags_GetEventChkInf(0x9 as libc::c_int) != 0 {
                    retTextId = 0x1042 as libc::c_int as u16_0
                } else if Flags_GetInfTable(0x24 as libc::c_int) != 0 {
                    retTextId = 0x100b as libc::c_int as u16_0
                } else { retTextId = 0x100a as libc::c_int as u16_0 }
            } else if Flags_GetEventChkInf(0x5c as libc::c_int) != 0 {
                retTextId = 0x1056 as libc::c_int as u16_0
            } else { retTextId = 0x105f as libc::c_int as u16_0 }
        }
        7 => {
            if !(gSaveContext.linkAge == 0 as libc::c_int) {
                if Flags_GetEventChkInf(0x9 as libc::c_int) != 0 {
                    retTextId = 0x1043 as libc::c_int as u16_0
                } else if Flags_GetInfTable(0x26 as libc::c_int) != 0 {
                    retTextId = 0x100d as libc::c_int as u16_0
                } else { retTextId = 0x100c as libc::c_int as u16_0 }
            } else if Flags_GetEventChkInf(0x5c as libc::c_int) != 0 {
                retTextId = 0x1057 as libc::c_int as u16_0
            } else { retTextId = 0x1057 as libc::c_int as u16_0 }
        }
        8 => {
            if !(gSaveContext.linkAge == 0 as libc::c_int) {
                if Flags_GetEventChkInf(0x9 as libc::c_int) != 0 {
                    retTextId = 0x1043 as libc::c_int as u16_0
                } else if Flags_GetInfTable(0x28 as libc::c_int) != 0 {
                    retTextId = 0x1019 as libc::c_int as u16_0
                } else { retTextId = 0x100e as libc::c_int as u16_0 }
            } else if Flags_GetEventChkInf(0x5c as libc::c_int) != 0 {
                retTextId = 0x1077 as libc::c_int as u16_0
            } else if Flags_GetInfTable(0x51 as libc::c_int) != 0 {
                retTextId = 0x1058 as libc::c_int as u16_0
            } else { retTextId = 0x1059 as libc::c_int as u16_0 }
        }
        9 => {
            if !(gSaveContext.linkAge == 0 as libc::c_int) {
                if Flags_GetEventChkInf(0x9 as libc::c_int) != 0 {
                    retTextId = 0x1049 as libc::c_int as u16_0
                } else { retTextId = 0x1035 as libc::c_int as u16_0 }
            } else if Flags_GetEventChkInf(0x5c as libc::c_int) != 0 {
                retTextId = 0x1079 as libc::c_int as u16_0
            } else { retTextId = 0x104e as libc::c_int as u16_0 }
        }
        10 => {
            if !(gSaveContext.linkAge == 0 as libc::c_int) {
                if Flags_GetEventChkInf(0x9 as libc::c_int) != 0 {
                    retTextId = 0x104a as libc::c_int as u16_0
                } else { retTextId = 0x1038 as libc::c_int as u16_0 }
            } else if Flags_GetEventChkInf(0x5c as libc::c_int) != 0 {
                retTextId = 0x1079 as libc::c_int as u16_0
            } else if Flags_GetInfTable(0x59 as libc::c_int) != 0 {
                retTextId = 0x1050 as libc::c_int as u16_0
            } else { retTextId = 0x104f as libc::c_int as u16_0 }
        }
        11 => {
            if !(gSaveContext.linkAge == 0 as libc::c_int) {
                if Flags_GetEventChkInf(0x9 as libc::c_int) != 0 {
                    retTextId = 0x104b as libc::c_int as u16_0
                } else { retTextId = 0x103c as libc::c_int as u16_0 }
            } else if Flags_GetEventChkInf(0x5c as libc::c_int) != 0 {
                retTextId = 0x107b as libc::c_int as u16_0
            } else { retTextId = 0x1051 as libc::c_int as u16_0 }
        }
        12 => {
            if !(gSaveContext.linkAge == 0 as libc::c_int) {
                if Flags_GetEventChkInf(0x9 as libc::c_int) != 0 {
                    retTextId = 0x104c as libc::c_int as u16_0
                } else { retTextId = 0x103d as libc::c_int as u16_0 }
            } else if Flags_GetEventChkInf(0x5c as libc::c_int) != 0 {
                retTextId = 0x107c as libc::c_int as u16_0
            } else { retTextId = 0x1052 as libc::c_int as u16_0 }
        }
        13 => {
            if !(gSaveContext.linkAge == 0 as libc::c_int) {
                if Flags_GetEventChkInf(0x9 as libc::c_int) != 0 {
                    retTextId = 0x104d as libc::c_int as u16_0
                } else { retTextId = 0x103e as libc::c_int as u16_0 }
            } else if Flags_GetEventChkInf(0x5c as libc::c_int) != 0 {
                retTextId = 0x106e as libc::c_int as u16_0
            } else if Flags_GetInfTable(0x61 as libc::c_int) != 0 {
                retTextId = 0x1053 as libc::c_int as u16_0
            } else { retTextId = 0x1054 as libc::c_int as u16_0 }
        }
        15 => {
            if Flags_GetEventChkInf(0x5c as libc::c_int) != 0 {
                retTextId = 0x1078 as libc::c_int as u16_0
            } else if Flags_GetInfTable(0x66 as libc::c_int) != 0 {
                retTextId = 0x1066 as libc::c_int as u16_0
            } else { retTextId = 0x1062 as libc::c_int as u16_0 }
        }
        16 => {
            if (*globalCtx).sceneNum as libc::c_int ==
                   SCENE_SPOT15 as libc::c_int {
                retTextId = 0x7002 as libc::c_int as u16_0
            } else if Flags_GetInfTable(0x6a as libc::c_int) != 0 {
                retTextId = 0x7004 as libc::c_int as u16_0
            } else if gSaveContext.dayTime as libc::c_int >=
                          0x4000 as libc::c_int &&
                          (gSaveContext.dayTime as libc::c_int) <
                              0xc556 as libc::c_int {
                retTextId = 0x7002 as libc::c_int as u16_0
            } else { retTextId = 0x7003 as libc::c_int as u16_0 }
        }
        17 => {
            if Flags_GetEventChkInf(0x9 as libc::c_int) != 0 &&
                   Flags_GetEventChkInf(0x25 as libc::c_int) != 0 &&
                   Flags_GetEventChkInf(0x37 as libc::c_int) != 0 {
                if Flags_GetInfTable(0x6c as libc::c_int) != 0 {
                    retTextId = 0x7008 as libc::c_int as u16_0
                } else { retTextId = 0x7007 as libc::c_int as u16_0 }
            } else { retTextId = 0 as libc::c_int as u16_0 }
        }
        19 => { retTextId = 0x702d as libc::c_int as u16_0 }
        18 => {
            if Flags_GetEventChkInf(0x9 as libc::c_int) != 0 &&
                   Flags_GetEventChkInf(0x25 as libc::c_int) != 0 &&
                   Flags_GetEventChkInf(0x37 as libc::c_int) != 0 {
                retTextId = 0x7006 as libc::c_int as u16_0
            } else if Flags_GetEventChkInf(0x12 as libc::c_int) != 0 {
                if Flags_GetInfTable(0x71 as libc::c_int) != 0 {
                    retTextId = 0x7072 as libc::c_int as u16_0
                } else { retTextId = 0x7071 as libc::c_int as u16_0 }
            } else { retTextId = 0x7029 as libc::c_int as u16_0 }
        }
        20 | 21 => {
            if Flags_GetEventChkInf(0x42 as libc::c_int) != 0 {
                retTextId = 0x2012 as libc::c_int as u16_0
            } else if Flags_GetEventChkInf(0x41 as libc::c_int) != 0 {
                if Flags_GetInfTable(0x76 as libc::c_int) != 0 {
                    retTextId = 0x2011 as libc::c_int as u16_0
                } else { retTextId = 0x2010 as libc::c_int as u16_0 }
            } else if Flags_GetEventChkInf(0x40 as libc::c_int) != 0 {
                retTextId = 0x200f as libc::c_int as u16_0
            } else { retTextId = 0x200e as libc::c_int as u16_0 }
        }
        24 => {
            if Flags_GetEventChkInf(0x9 as libc::c_int) != 0 &&
                   Flags_GetEventChkInf(0x25 as libc::c_int) != 0 &&
                   Flags_GetEventChkInf(0x37 as libc::c_int) != 0 {
                retTextId = 0x7044 as libc::c_int as u16_0
            } else { retTextId = 0x7015 as libc::c_int as u16_0 }
        }
        25 => {
            if Flags_GetEventChkInf(0x9 as libc::c_int) != 0 &&
                   Flags_GetEventChkInf(0x25 as libc::c_int) != 0 &&
                   Flags_GetEventChkInf(0x37 as libc::c_int) != 0 {
                retTextId = 0x7045 as libc::c_int as u16_0
            } else {
                Flags_GetInfTable(0xc2 as libc::c_int);
                retTextId = 0x7016 as libc::c_int as u16_0
            }
        }
        26 => {
            if Flags_GetEventChkInf(0x9 as libc::c_int) != 0 &&
                   Flags_GetEventChkInf(0x25 as libc::c_int) != 0 &&
                   Flags_GetEventChkInf(0x37 as libc::c_int) != 0 {
                retTextId = 0x7046 as libc::c_int as u16_0
            } else {
                Flags_GetInfTable(0xc2 as libc::c_int);
                retTextId = 0x7018 as libc::c_int as u16_0
            }
        }
        27 => {
            if Flags_GetEventChkInf(0x9 as libc::c_int) != 0 &&
                   Flags_GetEventChkInf(0x25 as libc::c_int) != 0 &&
                   Flags_GetEventChkInf(0x37 as libc::c_int) != 0 {
                retTextId = 0x7047 as libc::c_int as u16_0
            } else if Flags_GetEventChkInf(0x14 as libc::c_int) != 0 {
                retTextId = 0x701a as libc::c_int as u16_0
            } else if Flags_GetEventChkInf(0x11 as libc::c_int) != 0 {
                if Flags_GetInfTable(0xc6 as libc::c_int) != 0 {
                    retTextId = 0x701c as libc::c_int as u16_0
                } else { retTextId = 0x701b as libc::c_int as u16_0 }
            } else { retTextId = 0x701a as libc::c_int as u16_0 }
        }
        28 => {
            if Flags_GetEventChkInf(0x9 as libc::c_int) != 0 &&
                   Flags_GetEventChkInf(0x25 as libc::c_int) != 0 &&
                   Flags_GetEventChkInf(0x37 as libc::c_int) != 0 {
                retTextId = 0x7048 as libc::c_int as u16_0
            } else {
                Flags_GetInfTable(0xca as libc::c_int);
                retTextId = 0x701d as libc::c_int as u16_0
            }
        }
        29 => {
            if Flags_GetEventChkInf(0x9 as libc::c_int) != 0 &&
                   Flags_GetEventChkInf(0x25 as libc::c_int) != 0 &&
                   Flags_GetEventChkInf(0x37 as libc::c_int) != 0 {
                retTextId = 0x7049 as libc::c_int as u16_0
            } else {
                Flags_GetInfTable(0xcc as libc::c_int);
                retTextId = 0x701f as libc::c_int as u16_0
            }
        }
        30 => {
            if Flags_GetEventChkInf(0x9 as libc::c_int) != 0 &&
                   Flags_GetEventChkInf(0x25 as libc::c_int) != 0 &&
                   Flags_GetEventChkInf(0x37 as libc::c_int) != 0 {
                retTextId = 0x704a as libc::c_int as u16_0
            } else {
                Flags_GetInfTable(0xce as libc::c_int);
                retTextId = 0x7021 as libc::c_int as u16_0
            }
        }
        31 => {
            if Flags_GetEventChkInf(0x9 as libc::c_int) != 0 &&
                   Flags_GetEventChkInf(0x25 as libc::c_int) != 0 &&
                   Flags_GetEventChkInf(0x37 as libc::c_int) != 0 {
                retTextId = 0x704b as libc::c_int as u16_0
            } else {
                Flags_GetInfTable(0xd0 as libc::c_int);
                retTextId = 0x7023 as libc::c_int as u16_0
            }
        }
        32 => {
            if Flags_GetEventChkInf(0x9 as libc::c_int) != 0 &&
                   Flags_GetEventChkInf(0x25 as libc::c_int) != 0 &&
                   Flags_GetEventChkInf(0x37 as libc::c_int) != 0 {
                retTextId = 0x704c as libc::c_int as u16_0
            } else {
                Flags_GetInfTable(0xd2 as libc::c_int);
                retTextId = 0x7025 as libc::c_int as u16_0
            }
        }
        33 => {
            if Flags_GetEventChkInf(0x9 as libc::c_int) != 0 &&
                   Flags_GetEventChkInf(0x25 as libc::c_int) != 0 &&
                   Flags_GetEventChkInf(0x37 as libc::c_int) != 0 {
                retTextId = 0x704d as libc::c_int as u16_0
            } else {
                Flags_GetInfTable(0xd4 as libc::c_int);
                retTextId = 0x7027 as libc::c_int as u16_0
            }
        }
        34 => {
            Flags_GetInfTable(0xd6 as libc::c_int);
            retTextId = 0x403c as libc::c_int as u16_0
        }
        35 => {
            if Flags_GetInfTable(0xd8 as libc::c_int) != 0 {
                retTextId = 0x5029 as libc::c_int as u16_0
            } else { retTextId = 0x5028 as libc::c_int as u16_0 }
        }
        37 => { retTextId = 0x5002 as libc::c_int as u16_0 }
        38 => {
            if !(gSaveContext.linkAge == 0 as libc::c_int) {
                if Flags_GetEventChkInf(0x25 as libc::c_int) != 0 {
                    retTextId = 0x3027 as libc::c_int as u16_0
                } else if Flags_GetEventChkInf(0x23 as libc::c_int) != 0 {
                    retTextId = 0x3021 as libc::c_int as u16_0
                } else if Flags_GetInfTable(0xe0 as libc::c_int) != 0 {
                    retTextId = 0x302a as libc::c_int as u16_0
                } else { retTextId = 0x3008 as libc::c_int as u16_0 }
            } else if Flags_GetEventChkInf(0x20 as libc::c_int) != 0 {
                retTextId = 0x4043 as libc::c_int as u16_0
            } else { retTextId = 0x302a as libc::c_int as u16_0 }
        }
        39 => {
            if !(gSaveContext.linkAge == 0 as libc::c_int) {
                if Flags_GetEventChkInf(0x25 as libc::c_int) != 0 {
                    retTextId = 0x3027 as libc::c_int as u16_0
                } else if Flags_GetEventChkInf(0x23 as libc::c_int) != 0 {
                    retTextId = 0x3026 as libc::c_int as u16_0
                } else { retTextId = 0x3009 as libc::c_int as u16_0 }
            } else if Flags_GetEventChkInf(0x2a as libc::c_int) != 0 {
                retTextId = 0x4043 as libc::c_int as u16_0
            } else { retTextId = 0x302a as libc::c_int as u16_0 }
        }
        40 => {
            if !(gSaveContext.linkAge == 0 as libc::c_int) {
                if Flags_GetEventChkInf(0x25 as libc::c_int) != 0 {
                    retTextId = 0x3027 as libc::c_int as u16_0
                } else if Flags_GetEventChkInf(0x23 as libc::c_int) != 0 {
                    retTextId = 0x3026 as libc::c_int as u16_0
                } else if Flags_GetInfTable(0xeb as libc::c_int) != 0 {
                    retTextId = 0x302b as libc::c_int as u16_0
                } else { retTextId = 0x300a as libc::c_int as u16_0 }
            } else if Flags_GetEventChkInf(0x2b as libc::c_int) != 0 {
                retTextId = 0x4043 as libc::c_int as u16_0
            } else { retTextId = 0x302a as libc::c_int as u16_0 }
        }
        41 => {
            if !(gSaveContext.linkAge == 0 as libc::c_int) {
                if Flags_GetEventChkInf(0x25 as libc::c_int) != 0 {
                    retTextId = 0x3027 as libc::c_int as u16_0
                } else if Flags_GetInfTable(0xf0 as libc::c_int) != 0 {
                    retTextId = 0x3015 as libc::c_int as u16_0
                } else { retTextId = 0x3014 as libc::c_int as u16_0 }
            } else if Flags_GetEventChkInf(0x2c as libc::c_int) != 0 {
                retTextId = 0x4043 as libc::c_int as u16_0
            } else { retTextId = 0x302a as libc::c_int as u16_0 }
        }
        42 => {
            if !(gSaveContext.linkAge == 0 as libc::c_int) {
                if Flags_GetEventChkInf(0x25 as libc::c_int) != 0 {
                    retTextId = 0x3027 as libc::c_int as u16_0
                } else if Flags_GetInfTable(0xf4 as libc::c_int) != 0 {
                    retTextId = 0x3017 as libc::c_int as u16_0
                } else { retTextId = 0x3016 as libc::c_int as u16_0 }
            } else if Flags_GetEventChkInf(0x2c as libc::c_int) != 0 {
                retTextId = 0x4043 as libc::c_int as u16_0
            } else { retTextId = 0x302a as libc::c_int as u16_0 }
        }
        43 => {
            if !(gSaveContext.linkAge == 0 as libc::c_int) {
                if Flags_GetEventChkInf(0x25 as libc::c_int) != 0 {
                    retTextId = 0x3027 as libc::c_int as u16_0
                } else if Flags_GetInfTable(0xf8 as libc::c_int) != 0 {
                    retTextId = 0x3019 as libc::c_int as u16_0
                } else { retTextId = 0x3018 as libc::c_int as u16_0 }
            } else if Flags_GetEventChkInf(0x2d as libc::c_int) != 0 {
                retTextId = 0x4043 as libc::c_int as u16_0
            } else { retTextId = 0x302a as libc::c_int as u16_0 }
        }
        48 => {
            if Flags_GetEventChkInf(0x25 as libc::c_int) != 0 {
                retTextId = 0x3029 as libc::c_int as u16_0
            } else if Flags_GetEventChkInf(0x20 as libc::c_int) != 0 &&
                          Flags_GetEventChkInf(0x21 as libc::c_int) != 0 {
                retTextId = 0x301b as libc::c_int as u16_0
            } else { retTextId = 0x301a as libc::c_int as u16_0 }
        }
        49 => {
            if Flags_GetEventChkInf(0x37 as libc::c_int) != 0 {
                retTextId = 0x402d as libc::c_int as u16_0
            } else if Flags_GetEventChkInf(0x30 as libc::c_int) != 0 {
                retTextId = 0x4007 as libc::c_int as u16_0
            } else { retTextId = 0x4006 as libc::c_int as u16_0 }
        }
        50 => {
            if Flags_GetEventChkInf(0x37 as libc::c_int) != 0 {
                retTextId = 0x402e as libc::c_int as u16_0
            } else if Flags_GetEventChkInf(0x30 as libc::c_int) != 0 {
                if Flags_GetInfTable(0x124 as libc::c_int) != 0 {
                    retTextId = 0x4009 as libc::c_int as u16_0
                } else { retTextId = 0x4008 as libc::c_int as u16_0 }
            } else { retTextId = 0x4006 as libc::c_int as u16_0 }
        }
        51 => {
            if Flags_GetEventChkInf(0x37 as libc::c_int) != 0 {
                retTextId = 0x402d as libc::c_int as u16_0
            } else if Flags_GetEventChkInf(0x31 as libc::c_int) != 0 {
                if Flags_GetInfTable(0x12a as libc::c_int) != 0 {
                    retTextId = 0x400b as libc::c_int as u16_0
                } else { retTextId = 0x402f as libc::c_int as u16_0 }
            } else if Flags_GetEventChkInf(0x30 as libc::c_int) != 0 {
                retTextId = 0x400a as libc::c_int as u16_0
            } else { retTextId = 0x4006 as libc::c_int as u16_0 }
        }
        52 => {
            if Flags_GetEventChkInf(0x37 as libc::c_int) != 0 {
                retTextId = 0x402e as libc::c_int as u16_0
            } else if Flags_GetEventChkInf(0x30 as libc::c_int) != 0 {
                retTextId = 0x400c as libc::c_int as u16_0
            } else { retTextId = 0x4006 as libc::c_int as u16_0 }
        }
        53 => {
            if Flags_GetEventChkInf(0x37 as libc::c_int) != 0 {
                retTextId = 0x402d as libc::c_int as u16_0
            } else if Flags_GetEventChkInf(0x33 as libc::c_int) != 0 {
                retTextId = 0x4010 as libc::c_int as u16_0
            } else if Flags_GetEventChkInf(0x30 as libc::c_int) != 0 {
                retTextId = 0x400f as libc::c_int as u16_0
            } else { retTextId = 0x4006 as libc::c_int as u16_0 }
        }
        54 => {
            if Flags_GetEventChkInf(0x37 as libc::c_int) != 0 {
                retTextId = 0x402e as libc::c_int as u16_0
            } else if Flags_GetEventChkInf(0x30 as libc::c_int) != 0 {
                retTextId = 0x4011 as libc::c_int as u16_0
            } else { retTextId = 0x4006 as libc::c_int as u16_0 }
        }
        55 => {
            if !(gSaveContext.linkAge == 0 as libc::c_int) {
                if Flags_GetEventChkInf(0x37 as libc::c_int) != 0 {
                    retTextId = 0x402b as libc::c_int as u16_0
                } else if Flags_GetEventChkInf(0x31 as libc::c_int) != 0 {
                    if Flags_GetInfTable(0x138 as libc::c_int) != 0 {
                        retTextId = 0x401c as libc::c_int as u16_0
                    } else { retTextId = 0x401b as libc::c_int as u16_0 }
                } else { retTextId = 0x401a as libc::c_int as u16_0 }
            } else { retTextId = 0 as libc::c_int as u16_0 }
        }
        58 => { retTextId = 0x500f as libc::c_int as u16_0 }
        59 => { retTextId = 0x5010 as libc::c_int as u16_0 }
        60 => { retTextId = 0x5012 as libc::c_int as u16_0 }
        61 => {
            if Flags_GetInfTable(0x166 as libc::c_int) != 0 {
                retTextId = 0x5001 as libc::c_int as u16_0
            } else { retTextId = 0x5000 as libc::c_int as u16_0 }
        }
        62 => { retTextId = 0x5012 as libc::c_int as u16_0 }
        63 => {
            if Flags_GetInfTable(0x16a as libc::c_int) != 0 {
                retTextId = 0x5001 as libc::c_int as u16_0
            } else { retTextId = 0x5000 as libc::c_int as u16_0 }
        }
        71 => {
            if Flags_GetEventChkInf(0x16 as libc::c_int) != 0 {
                retTextId = 0x2049 as libc::c_int as u16_0
            } else if Flags_GetEventChkInf(0x15 as libc::c_int) != 0 {
                retTextId = 0x2048 as libc::c_int as u16_0
            } else if Flags_GetEventChkInf(0x14 as libc::c_int) != 0 {
                retTextId = 0x2047 as libc::c_int as u16_0
            } else if Flags_GetEventChkInf(0x12 as libc::c_int) != 0 &&
                          Flags_GetEventChkInf(0x14 as libc::c_int) == 0 {
                retTextId = 0x2044 as libc::c_int as u16_0
            } else if Flags_GetEventChkInf(0x10 as libc::c_int) != 0 {
                if Flags_GetEventChkInf(0x11 as libc::c_int) != 0 {
                    retTextId = 0x2043 as libc::c_int as u16_0
                } else { retTextId = 0x2042 as libc::c_int as u16_0 }
            } else { retTextId = 0x2041 as libc::c_int as u16_0 }
        }
        72 => {
            if !(gSaveContext.linkAge == 0 as libc::c_int) {
                if Flags_GetEventChkInf(0x14 as libc::c_int) != 0 {
                    retTextId = 0x2040 as libc::c_int as u16_0
                } else if Flags_GetInfTable(0x94 as libc::c_int) != 0 {
                    retTextId = 0x2040 as libc::c_int as u16_0
                } else { retTextId = 0x203f as libc::c_int as u16_0 }
            } else if Flags_GetEventChkInf(0x18 as libc::c_int) == 0 {
                if !(gSaveContext.nightFlag == 0 as libc::c_int) {
                    retTextId = 0x204e as libc::c_int as u16_0
                } else if Flags_GetInfTable(0x9a as libc::c_int) != 0 {
                    retTextId = 0x2031 as libc::c_int as u16_0
                } else { retTextId = 0x2030 as libc::c_int as u16_0 }
            } else { retTextId = 0 as libc::c_int as u16_0 }
        }
        _ => { }
    }
    if retTextId as libc::c_int == 0 as libc::c_int {
        retTextId = 1 as libc::c_int as u16_0
    }
    return retTextId as u32_0;
}
#[no_mangle]
pub unsafe extern "C" fn func_80036E50(mut textId: u16_0, mut arg1: s16) {
    match arg1 as libc::c_int {
        0 => {
            match textId as libc::c_int {
                4097 => { Flags_SetInfTable(0 as libc::c_int); return }
                4098 => { Flags_SetInfTable(0x1 as libc::c_int); return }
                4145 => {
                    Flags_SetEventChkInf(0x3 as libc::c_int);
                    Flags_SetInfTable(0x3 as libc::c_int);
                    return
                }
                4167 => { Flags_SetInfTable(0x5 as libc::c_int); return }
                _ => { }
            }
            return
        }
        1 => {
            match textId as libc::c_int {
                4143 => {
                    Flags_SetEventChkInf(0x2 as libc::c_int);
                    Flags_SetInfTable(0xc as libc::c_int);
                    return
                }
                4147 => {
                    Audio_PlaySoundGeneral(0x4802 as libc::c_int as u16_0,
                                           &mut D_801333D4,
                                           4 as libc::c_int as u8_0,
                                           &mut D_801333E0, &mut D_801333E0,
                                           &mut D_801333E8);
                    Flags_SetEventChkInf(0x4 as libc::c_int);
                    Flags_SetInfTable(0xe as libc::c_int);
                    return
                }
                4165 => { Flags_SetInfTable(0x10 as libc::c_int); return }
                4192 => { Flags_SetInfTable(0x15 as libc::c_int); return }
                4199 => {
                    Flags_SetEventChkInf(0xa as libc::c_int);
                    Flags_SetInfTable(0x17 as libc::c_int);
                    return
                }
                4208 => { Flags_SetInfTable(0x19 as libc::c_int); return }
                _ => { }
            }
            return
        }
        2 => {
            if textId as libc::c_int == 0x1056 as libc::c_int {
                Flags_SetInfTable(0x41 as libc::c_int);
            }
            return
        }
        3 => {
            if textId as libc::c_int == 0x1005 as libc::c_int {
                Flags_SetInfTable(0x1e as libc::c_int);
            }
            return
        }
        4 => {
            if textId as libc::c_int == 0x105d as libc::c_int {
                Flags_SetInfTable(0x47 as libc::c_int);
            }
            return
        }
        5 => {
            if textId as libc::c_int == 0x1008 as libc::c_int {
                Flags_SetInfTable(0x22 as libc::c_int);
            }
            return
        }
        6 => {
            if textId as libc::c_int == 0x100a as libc::c_int {
                Flags_SetInfTable(0x24 as libc::c_int);
            }
            return
        }
        7 => {
            if textId as libc::c_int == 0x100c as libc::c_int {
                Flags_SetInfTable(0x26 as libc::c_int);
            }
            return
        }
        8 => {
            if textId as libc::c_int == 0x100e as libc::c_int {
                Flags_SetInfTable(0x28 as libc::c_int);
            }
            if textId as libc::c_int == 0x1059 as libc::c_int {
                Flags_SetInfTable(0x51 as libc::c_int);
            }
            return
        }
        10 => {
            if textId as libc::c_int == 0x104f as libc::c_int {
                Flags_SetInfTable(0x59 as libc::c_int);
            }
            return
        }
        13 => {
            if textId as libc::c_int == 0x1054 as libc::c_int {
                Flags_SetInfTable(0x61 as libc::c_int);
            }
            return
        }
        15 => {
            if textId as libc::c_int == 0x1062 as libc::c_int {
                Flags_SetInfTable(0x66 as libc::c_int);
            }
            return
        }
        16 => {
            if textId as libc::c_int == 0x7002 as libc::c_int {
                Flags_SetInfTable(0x6a as libc::c_int);
            }
            if textId as libc::c_int == 0x7003 as libc::c_int {
                Flags_SetInfTable(0x6a as libc::c_int);
            }
            return
        }
        17 => {
            if textId as libc::c_int == 0x7007 as libc::c_int {
                Flags_SetInfTable(0x6c as libc::c_int);
            }
            return
        }
        18 => {
            if textId as libc::c_int == 0x7071 as libc::c_int {
                Flags_SetInfTable(0x71 as libc::c_int);
            }
            return
        }
        20 | 21 => {
            if textId as libc::c_int == 0x2010 as libc::c_int {
                Flags_SetInfTable(0x76 as libc::c_int);
            }
            return
        }
        25 => {
            if textId as libc::c_int == 0x7016 as libc::c_int {
                Flags_SetInfTable(0xc2 as libc::c_int);
            }
            return
        }
        26 => {
            if textId as libc::c_int == 0x7018 as libc::c_int {
                Flags_SetInfTable(0xc4 as libc::c_int);
            }
            return
        }
        28 => {
            if textId as libc::c_int == 0x701d as libc::c_int {
                Flags_SetInfTable(0xca as libc::c_int);
            }
            return
        }
        29 => {
            if textId as libc::c_int == 0x701f as libc::c_int {
                Flags_SetInfTable(0xcc as libc::c_int);
            }
            return
        }
        30 => {
            if textId as libc::c_int == 0x7021 as libc::c_int {
                Flags_SetInfTable(0xce as libc::c_int);
            }
            return
        }
        31 => {
            if textId as libc::c_int == 0x7023 as libc::c_int {
                Flags_SetInfTable(0xd0 as libc::c_int);
            }
            return
        }
        32 => {
            if textId as libc::c_int == 0x7025 as libc::c_int {
                Flags_SetInfTable(0xd2 as libc::c_int);
            }
            return
        }
        33 => {
            if textId as libc::c_int == 0x7027 as libc::c_int {
                Flags_SetInfTable(0xd4 as libc::c_int);
            }
            return
        }
        34 => {
            if textId as libc::c_int == 0x403c as libc::c_int {
                Flags_SetInfTable(0xd6 as libc::c_int);
            }
            return
        }
        35 => {
            if textId as libc::c_int == 0x5028 as libc::c_int {
                Flags_SetInfTable(0xd8 as libc::c_int);
            }
            return
        }
        38 => {
            if textId as libc::c_int == 0x3008 as libc::c_int {
                Flags_SetInfTable(0xe0 as libc::c_int);
            }
            return
        }
        40 => {
            if textId as libc::c_int == 0x300b as libc::c_int {
                Flags_SetInfTable(0xeb as libc::c_int);
            }
            return
        }
        41 => {
            if textId as libc::c_int == 0x3014 as libc::c_int {
                Flags_SetInfTable(0xf0 as libc::c_int);
            }
            return
        }
        42 => {
            if textId as libc::c_int == 0x3016 as libc::c_int {
                Flags_SetInfTable(0xf4 as libc::c_int);
            }
            return
        }
        43 => {
            if textId as libc::c_int == 0x3018 as libc::c_int {
                Flags_SetEventChkInf(0x20 as libc::c_int);
                Flags_SetInfTable(0xf8 as libc::c_int);
            }
            return
        }
        48 => {
            if textId as libc::c_int == 0x3020 as libc::c_int {
                Flags_SetEventChkInf(0x22 as libc::c_int);
                Flags_SetInfTable(0x113 as libc::c_int);
            }
            return
        }
        49 | 52 | 53 | 54 => {
            if textId as libc::c_int == 0x4006 as libc::c_int {
                Flags_SetEventChkInf(0x30 as libc::c_int);
            }
            return
        }
        50 => {
            if textId as libc::c_int == 0x4006 as libc::c_int {
                Flags_SetEventChkInf(0x30 as libc::c_int);
            }
            if textId as libc::c_int == 0x4008 as libc::c_int {
                Flags_SetInfTable(0x124 as libc::c_int);
            }
            return
        }
        51 => {
            if textId as libc::c_int == 0x4006 as libc::c_int {
                Flags_SetEventChkInf(0x30 as libc::c_int);
            }
            if textId as libc::c_int == 0x400a as libc::c_int {
                Flags_SetEventChkInf(0x32 as libc::c_int);
            }
            if textId as libc::c_int == 0x402f as libc::c_int {
                Flags_SetInfTable(0x12a as libc::c_int);
            }
            return
        }
        55 => {
            if textId as libc::c_int == 0x401b as libc::c_int {
                Flags_SetEventChkInf(0x33 as libc::c_int);
                Flags_SetInfTable(0x138 as libc::c_int);
            }
            return
        }
        61 => {
            if textId as libc::c_int == 0x5000 as libc::c_int {
                Flags_SetInfTable(0x166 as libc::c_int);
            }
            return
        }
        63 => {
            if textId as libc::c_int == 0x5013 as libc::c_int {
                Flags_SetInfTable(0x16a as libc::c_int);
            }
            return
        }
        71 => {
            if textId as libc::c_int == 0x2041 as libc::c_int {
                Flags_SetEventChkInf(0x10 as libc::c_int);
            }
            if textId as libc::c_int == 0x2044 as libc::c_int {
                Flags_SetEventChkInf(0x12 as libc::c_int);
            }
            if textId as libc::c_int == 0x2047 as libc::c_int {
                Flags_SetEventChkInf(0x15 as libc::c_int);
            }
            if textId as libc::c_int == 0x2048 as libc::c_int {
                Flags_SetEventChkInf(0x16 as libc::c_int);
            }
            return
        }
        72 => { return }
        _ => { }
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800374E0(mut globalCtx: *mut GlobalContext,
                                       mut actor: *mut Actor,
                                       mut textId: u16_0) -> s32 {
    let mut msgCtx: *mut MessageContext = &mut (*globalCtx).msgCtx;
    let mut ret: s32 = 1 as libc::c_int;
    match textId as libc::c_int {
        4149 => {
            if (*msgCtx).choiceIndex as libc::c_int == 0 as libc::c_int {
                if Flags_GetInfTable(0x2a as libc::c_int) != 0 {
                    func_80035B18(globalCtx, actor,
                                  0x1036 as libc::c_int as u16_0);
                } else {
                    func_80035B18(globalCtx, actor,
                                  0x1041 as libc::c_int as u16_0);
                }
            }
            if (*msgCtx).choiceIndex as libc::c_int == 1 as libc::c_int {
                if Flags_GetInfTable(0x2b as libc::c_int) != 0 {
                    func_80035B18(globalCtx, actor,
                                  0x1037 as libc::c_int as u16_0);
                } else {
                    func_80035B18(globalCtx, actor,
                                  0x1041 as libc::c_int as u16_0);
                }
            }
            ret = 0 as libc::c_int
        }
        4152 => {
            if (*msgCtx).choiceIndex as libc::c_int == 0 as libc::c_int {
                if Flags_GetInfTable(0x2e as libc::c_int) != 0 {
                    func_80035B18(globalCtx, actor,
                                  0x1039 as libc::c_int as u16_0);
                } else {
                    func_80035B18(globalCtx, actor,
                                  0x1041 as libc::c_int as u16_0);
                }
            }
            if (*msgCtx).choiceIndex as libc::c_int == 1 as libc::c_int {
                if Flags_GetInfTable(0x2f as libc::c_int) != 0 {
                    func_80035B18(globalCtx, actor,
                                  0x103a as libc::c_int as u16_0);
                } else {
                    func_80035B18(globalCtx, actor,
                                  0x1041 as libc::c_int as u16_0);
                }
            }
            if (*msgCtx).choiceIndex as libc::c_int == 2 as libc::c_int {
                if Flags_GetInfTable(0x30 as libc::c_int) != 0 {
                    func_80035B18(globalCtx, actor,
                                  0x103b as libc::c_int as u16_0);
                } else {
                    func_80035B18(globalCtx, actor,
                                  0x1041 as libc::c_int as u16_0);
                }
            }
            ret = 0 as libc::c_int
        }
        4158 => {
            if (*msgCtx).choiceIndex as libc::c_int == 0 as libc::c_int {
                func_80035B18(globalCtx, actor,
                              0x103f as libc::c_int as u16_0);
            }
            if (*msgCtx).choiceIndex as libc::c_int == 1 as libc::c_int {
                func_80035B18(globalCtx, actor,
                              0x1040 as libc::c_int as u16_0);
            }
            ret = 0 as libc::c_int
        }
        4161 => {
            if (*msgCtx).choiceTextId as libc::c_int == 0x1035 as libc::c_int
               {
                if (*msgCtx).choiceIndex as libc::c_int == 0 as libc::c_int {
                    func_80035B18(globalCtx, actor,
                                  0x1036 as libc::c_int as u16_0);
                    Flags_SetInfTable(0x2a as libc::c_int);
                }
                if (*msgCtx).choiceIndex as libc::c_int == 1 as libc::c_int {
                    func_80035B18(globalCtx, actor,
                                  0x1037 as libc::c_int as u16_0);
                    Flags_SetInfTable(0x2b as libc::c_int);
                }
            }
            if (*msgCtx).choiceTextId as libc::c_int == 0x1038 as libc::c_int
               {
                if (*msgCtx).choiceIndex as libc::c_int == 0 as libc::c_int {
                    func_80035B18(globalCtx, actor,
                                  0x1039 as libc::c_int as u16_0);
                    Flags_SetInfTable(0x2e as libc::c_int);
                }
                if (*msgCtx).choiceIndex as libc::c_int == 1 as libc::c_int {
                    func_80035B18(globalCtx, actor,
                                  0x103a as libc::c_int as u16_0);
                    Flags_SetInfTable(0x2f as libc::c_int);
                }
                if (*msgCtx).choiceIndex as libc::c_int == 2 as libc::c_int {
                    func_80035B18(globalCtx, actor,
                                  0x103b as libc::c_int as u16_0);
                    Flags_SetInfTable(0x30 as libc::c_int);
                }
            }
            ret = 0 as libc::c_int
        }
        4194 => {
            if (*msgCtx).choiceIndex as libc::c_int == 0 as libc::c_int {
                func_80035B18(globalCtx, actor,
                              0x1063 as libc::c_int as u16_0);
            }
            if (*msgCtx).choiceIndex as libc::c_int == 1 as libc::c_int {
                func_80035B18(globalCtx, actor,
                              0x1064 as libc::c_int as u16_0);
            }
            ret = 0 as libc::c_int
        }
        8240 | 8241 => {
            if (*msgCtx).choiceIndex as libc::c_int == 0 as libc::c_int {
                if gSaveContext.rupees as libc::c_int >= 10 as libc::c_int {
                    func_80035B18(globalCtx, actor,
                                  0x2034 as libc::c_int as u16_0);
                    Rupees_ChangeBy(-(10 as libc::c_int) as s16);
                } else {
                    func_80035B18(globalCtx, actor,
                                  0x2032 as libc::c_int as u16_0);
                }
            }
            if (*msgCtx).choiceIndex as libc::c_int == 1 as libc::c_int {
                func_80035B18(globalCtx, actor,
                              0x2032 as libc::c_int as u16_0);
            }
            Flags_SetInfTable(0x9a as libc::c_int);
            ret = 0 as libc::c_int
        }
        8246 | 8247 => {
            if (*msgCtx).choiceIndex as libc::c_int == 0 as libc::c_int {
                func_80035B18(globalCtx, actor,
                              0x201f as libc::c_int as u16_0);
            }
            if (*msgCtx).choiceIndex as libc::c_int == 1 as libc::c_int {
                func_80035B18(globalCtx, actor,
                              0x205a as libc::c_int as u16_0);
            }
            ret = 0 as libc::c_int
        }
        8248 => {
            if !((*msgCtx).choiceIndex as libc::c_int == 0 as libc::c_int) {
                if (*msgCtx).choiceIndex as libc::c_int == 1 as libc::c_int {
                    func_80035B18(globalCtx, actor,
                                  0x205a as libc::c_int as u16_0);
                }
                ret = 0 as libc::c_int
            }
        }
        8244 => {
            if !((*msgCtx).choiceIndex as libc::c_int != 0 as libc::c_int) {
                func_80035B18(globalCtx, actor,
                              0x2035 as libc::c_int as u16_0);
                ret = 0 as libc::c_int
            }
        }
        8259 => {
            if !(Flags_GetEventChkInf(0x12 as libc::c_int) != 0) {
                func_80035B18(globalCtx, actor,
                              0x2044 as libc::c_int as u16_0);
                ret = 0 as libc::c_int
            }
        }
        12298 => {
            if (*msgCtx).choiceIndex as libc::c_int == 0 as libc::c_int {
                if Flags_GetEventChkInf(0x22 as libc::c_int) != 0 {
                    func_80035B18(globalCtx, actor,
                                  0x300b as libc::c_int as u16_0);
                } else {
                    func_80035B18(globalCtx, actor,
                                  0x300c as libc::c_int as u16_0);
                }
            }
            if (*msgCtx).choiceIndex as libc::c_int == 1 as libc::c_int {
                func_80035B18(globalCtx, actor,
                              0x300d as libc::c_int as u16_0);
            }
            ret = 0 as libc::c_int
        }
        12315 => {
            if (*msgCtx).choiceIndex as libc::c_int == 0 as libc::c_int {
                func_80035B18(globalCtx, actor,
                              0x301d as libc::c_int as u16_0);
            }
            if (*msgCtx).choiceIndex as libc::c_int == 1 as libc::c_int {
                if Flags_GetInfTable(0x113 as libc::c_int) != 0 {
                    func_80035B18(globalCtx, actor,
                                  0x301f as libc::c_int as u16_0);
                } else {
                    func_80035B18(globalCtx, actor,
                                  0x301e as libc::c_int as u16_0);
                }
            }
            ret = 0 as libc::c_int
        }
        12318 => {
            func_80035B18(globalCtx, actor, 0x3020 as libc::c_int as u16_0);
            ret = 0 as libc::c_int
        }
        16396 => {
            if (*msgCtx).choiceIndex as libc::c_int == 0 as libc::c_int {
                func_80035B18(globalCtx, actor,
                              0x400d as libc::c_int as u16_0);
            }
            if (*msgCtx).choiceIndex as libc::c_int == 1 as libc::c_int {
                func_80035B18(globalCtx, actor,
                              0x400e as libc::c_int as u16_0);
            }
            ret = 0 as libc::c_int
        }
        28679 => {
            func_80035B18(globalCtx, actor, 0x703e as libc::c_int as u16_0);
            ret = 0 as libc::c_int
        }
        28734 => {
            func_80035B18(globalCtx, actor, 0x703f as libc::c_int as u16_0);
            ret = 0 as libc::c_int
        }
        28735 => {
            func_80035B18(globalCtx, actor, 0x7042 as libc::c_int as u16_0);
            ret = 0 as libc::c_int
        }
        8282 | _ => { }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn func_80037C30(mut globalCtx: *mut GlobalContext,
                                       mut arg1: s16) -> u16_0 {
    return func_80035BFC(globalCtx, arg1) as u16_0;
}
#[no_mangle]
pub unsafe extern "C" fn func_80037C5C(mut globalCtx: *mut GlobalContext,
                                       mut arg1: s16, mut textId: u16_0)
 -> s32 {
    func_80036E50(textId, arg1);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_80037C94(mut globalCtx: *mut GlobalContext,
                                       mut actor: *mut Actor, mut arg2: s32)
 -> s32 {
    return func_800374E0(globalCtx, actor, (*actor).textId);
}
#[no_mangle]
pub unsafe extern "C" fn func_80037CB8(mut globalCtx: *mut GlobalContext,
                                       mut actor: *mut Actor, mut arg2: s16)
 -> s32 {
    let mut msgCtx: *mut MessageContext = &mut (*globalCtx).msgCtx;
    let mut ret: s32 = 0 as libc::c_int;
    match Message_GetState(msgCtx) as libc::c_int {
        2 => {
            func_80037C5C(globalCtx, arg2, (*actor).textId);
            ret = 1 as libc::c_int
        }
        4 | 5 => {
            if Message_ShouldAdvance(globalCtx) as libc::c_int != 0 &&
                   func_80037C94(globalCtx, actor, arg2 as s32) != 0 {
                Audio_PlaySoundGeneral(0x480a as libc::c_int as u16_0,
                                       &mut D_801333D4,
                                       4 as libc::c_int as u8_0,
                                       &mut D_801333E0, &mut D_801333E0,
                                       &mut D_801333E8);
                (*msgCtx).msgMode =
                    MSGMODE_TEXT_CLOSING as libc::c_int as u8_0;
                ret = 1 as libc::c_int
            }
        }
        _ => { }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn func_80037D98(mut globalCtx: *mut GlobalContext,
                                       mut actor: *mut Actor, mut arg2: s16,
                                       mut arg3: *mut s32) -> s32 {
    let mut var: s16 = 0;
    let mut sp2C: s16 = 0;
    let mut sp2A: s16 = 0;
    let mut abs_var: s16 = 0;
    if Actor_ProcessTalkRequest(actor, globalCtx) != 0 {
        *arg3 = 1 as libc::c_int;
        return 1 as libc::c_int
    }
    if *arg3 == 1 as libc::c_int {
        if func_80037CB8(globalCtx, actor, arg2) != 0 {
            *arg3 = 0 as libc::c_int
        }
        return 0 as libc::c_int
    }
    Actor_GetScreenPos(globalCtx, actor, &mut sp2C, &mut sp2A);
    // Necessary to match
    if (sp2C as libc::c_int) < 0 as libc::c_int ||
           sp2C as libc::c_int > 320 as libc::c_int ||
           (sp2A as libc::c_int) < 0 as libc::c_int ||
           sp2A as libc::c_int > 240 as libc::c_int {
        return 0 as libc::c_int
    }
    var =
        ((*actor).yawTowardsPlayer as libc::c_int -
             (*actor).shape.rot.y as libc::c_int) as s16;
    abs_var =
        if var as libc::c_int >= 0 as libc::c_int {
            var as libc::c_int
        } else { -(var as libc::c_int) } as s16;
    if abs_var as libc::c_int >= 0x4300 as libc::c_int {
        return 0 as libc::c_int
    }
    if (*actor).xyzDistToPlayerSq > 160.0f32 * 160.0f32 &&
           (*actor).isTargeted == 0 {
        return 0 as libc::c_int
    }
    if (*actor).xyzDistToPlayerSq <= 80.0f32 * 80.0f32 {
        if func_8002F2CC(actor, globalCtx, 80.0f32) != 0 {
            (*actor).textId = func_80037C30(globalCtx, arg2)
        }
    } else if func_8002F2F4(actor, globalCtx) != 0 {
        (*actor).textId = func_80037C30(globalCtx, arg2)
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_80037F30(mut arg0: *mut Vec3s,
                                       mut arg1: *mut Vec3s) -> s32 {
    Math_SmoothStepToS(&mut (*arg0).y, 0 as libc::c_int as s16,
                       6 as libc::c_int as s16, 6200 as libc::c_int as s16,
                       100 as libc::c_int as s16);
    Math_SmoothStepToS(&mut (*arg0).x, 0 as libc::c_int as s16,
                       6 as libc::c_int as s16, 6200 as libc::c_int as s16,
                       100 as libc::c_int as s16);
    Math_SmoothStepToS(&mut (*arg1).y, 0 as libc::c_int as s16,
                       6 as libc::c_int as s16, 6200 as libc::c_int as s16,
                       100 as libc::c_int as s16);
    Math_SmoothStepToS(&mut (*arg1).x, 0 as libc::c_int as s16,
                       6 as libc::c_int as s16, 6200 as libc::c_int as s16,
                       100 as libc::c_int as s16);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_80037FC8(mut actor: *mut Actor,
                                       mut arg1: *mut Vec3f,
                                       mut arg2: *mut Vec3s,
                                       mut arg3: *mut Vec3s) -> s32 {
    let mut sp36: s16 = 0;
    let mut sp34: s16 = 0;
    let mut var: s16 = 0;
    sp36 = Math_Vec3f_Pitch(&mut (*actor).focus.pos, arg1);
    sp34 =
        (Math_Vec3f_Yaw(&mut (*actor).focus.pos, arg1) as libc::c_int -
             (*actor).world.rot.y as libc::c_int) as s16;
    Math_SmoothStepToS(&mut (*arg2).x, sp36, 6 as libc::c_int as s16,
                       2000 as libc::c_int as s16, 1 as libc::c_int as s16);
    (*arg2).x =
        if ((*arg2).x as libc::c_int) < -(6000 as libc::c_int) {
            -(6000 as libc::c_int)
        } else if (*arg2).x as libc::c_int > 6000 as libc::c_int {
            6000 as libc::c_int
        } else { (*arg2).x as libc::c_int } as s16;
    var =
        Math_SmoothStepToS(&mut (*arg2).y, sp34, 6 as libc::c_int as s16,
                           2000 as libc::c_int as s16,
                           1 as libc::c_int as s16);
    (*arg2).y =
        if ((*arg2).y as libc::c_int) < -(8000 as libc::c_int) {
            -(8000 as libc::c_int)
        } else if (*arg2).y as libc::c_int > 8000 as libc::c_int {
            8000 as libc::c_int
        } else { (*arg2).y as libc::c_int } as s16;
    if var as libc::c_int != 0 &&
           (if (*arg2).y as libc::c_int >= 0 as libc::c_int {
                (*arg2).y as libc::c_int
            } else { -((*arg2).y as libc::c_int) }) < 8000 as libc::c_int {
        return 0 as libc::c_int
    }
    Math_SmoothStepToS(&mut (*arg3).y,
                       (sp34 as libc::c_int - (*arg2).y as libc::c_int) as
                           s16, 4 as libc::c_int as s16,
                       2000 as libc::c_int as s16, 1 as libc::c_int as s16);
    (*arg3).y =
        if ((*arg3).y as libc::c_int) < -(12000 as libc::c_int) {
            -(12000 as libc::c_int)
        } else if (*arg3).y as libc::c_int > 12000 as libc::c_int {
            12000 as libc::c_int
        } else { (*arg3).y as libc::c_int } as s16;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_80038154(mut globalCtx: *mut GlobalContext,
                                       mut actor: *mut Actor,
                                       mut arg2: *mut Vec3s,
                                       mut arg3: *mut Vec3s, mut arg4: f32_0)
 -> s32 {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut pad: s32 = 0;
    let mut sp2C: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut var: s16 = 0;
    let mut abs_var: s16 = 0;
    (*actor).focus.pos = (*actor).world.pos;
    (*actor).focus.pos.y += arg4;
    if !(((*globalCtx).csCtx.state as libc::c_int !=
              CS_STATE_IDLE as libc::c_int || gDbgCamEnabled != 0) &&
             gSaveContext.entranceIndex == 0xee as libc::c_int) {
        var =
            ((*actor).yawTowardsPlayer as libc::c_int -
                 (*actor).shape.rot.y as libc::c_int) as s16;
        abs_var =
            if var as libc::c_int >= 0 as libc::c_int {
                var as libc::c_int
            } else { -(var as libc::c_int) } as s16;
        if abs_var as libc::c_int >= 0x4300 as libc::c_int {
            func_80037F30(arg2, arg3);
            return 0 as libc::c_int
        }
    }
    if ((*globalCtx).csCtx.state as libc::c_int !=
            CS_STATE_IDLE as libc::c_int || gDbgCamEnabled != 0) &&
           gSaveContext.entranceIndex == 0xee as libc::c_int {
        sp2C = (*globalCtx).view.eye
    } else { sp2C = (*player).actor.focus.pos }
    func_80037FC8(actor, &mut sp2C, arg2, arg3);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_80038290(mut globalCtx: *mut GlobalContext,
                                       mut actor: *mut Actor,
                                       mut arg2: *mut Vec3s,
                                       mut arg3: *mut Vec3s, mut arg4: Vec3f)
 -> s32 {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut pad: s32 = 0;
    let mut sp24: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut var: s16 = 0;
    let mut abs_var: s16 = 0;
    (*actor).focus.pos = arg4;
    if !(((*globalCtx).csCtx.state as libc::c_int !=
              CS_STATE_IDLE as libc::c_int || gDbgCamEnabled != 0) &&
             gSaveContext.entranceIndex == 0xee as libc::c_int) {
        var =
            ((*actor).yawTowardsPlayer as libc::c_int -
                 (*actor).shape.rot.y as libc::c_int) as s16;
        abs_var =
            if var as libc::c_int >= 0 as libc::c_int {
                var as libc::c_int
            } else { -(var as libc::c_int) } as s16;
        if abs_var as libc::c_int >= 0x4300 as libc::c_int {
            func_80037F30(arg2, arg3);
            return 0 as libc::c_int
        }
    }
    if ((*globalCtx).csCtx.state as libc::c_int !=
            CS_STATE_IDLE as libc::c_int || gDbgCamEnabled != 0) &&
           gSaveContext.entranceIndex == 0xee as libc::c_int {
        sp24 = (*globalCtx).view.eye
    } else { sp24 = (*player).actor.focus.pos }
    func_80037FC8(actor, &mut sp24, arg2, arg3);
    return 1 as libc::c_int;
}
