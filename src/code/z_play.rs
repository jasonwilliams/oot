#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn sqrtf(f: f32_0) -> f32_0;
    #[no_mangle]
    fn DmaMgr_DmaRomToRam(rom: u32_0, ram: u32_0, size: u32_0) -> s32;
    #[no_mangle]
    fn DmaMgr_SendRequest1(ram0: *mut libc::c_void, vrom: u32_0, size: u32_0,
                           file: *const libc::c_char, line: s32) -> s32;
    #[no_mangle]
    fn __assert(exp: *const libc::c_char, file: *const libc::c_char,
                line: s32);
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn LogUtils_LogThreadId(name: *const libc::c_char, line: s32);
    #[no_mangle]
    fn bzero(__s: *mut libc::c_void, __n: u32_0);
    #[no_mangle]
    fn osGetTime() -> OSTime;
    #[no_mangle]
    fn Effect_InitContext(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn Effect_UpdateAll(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn Effect_DeleteAll(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn EffectSs_InitInfo(globalCtx: *mut GlobalContext, tableSize: s32);
    #[no_mangle]
    fn EffectSs_ClearAll(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn EffectSs_UpdateAll(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn func_8002DF18(globalCtx: *mut GlobalContext, player: *mut Player);
    #[no_mangle]
    fn func_800304DC(globalCtx: *mut GlobalContext,
                     actorCtx: *mut ActorContext,
                     actorEntry: *mut ActorEntry);
    #[no_mangle]
    fn Actor_UpdateAll(globalCtx: *mut GlobalContext,
                       actorCtx: *mut ActorContext);
    #[no_mangle]
    fn func_800315AC(globalCtx: *mut GlobalContext,
                     actorCtx: *mut ActorContext);
    #[no_mangle]
    fn func_80031C3C(actorCtx: *mut ActorContext,
                     globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn ActorOverlayTable_LogPrint();
    #[no_mangle]
    fn BgCheck_EntityRaycastFloor3(colCtx: *mut CollisionContext,
                                   outPoly: *mut *mut CollisionPoly,
                                   bgId: *mut s32, pos: *mut Vec3f) -> f32_0;
    #[no_mangle]
    fn BgCheck_AnyRaycastFloor1(colCtx: *mut CollisionContext,
                                outPoly: *mut CollisionPoly, pos: *mut Vec3f)
     -> f32_0;
    #[no_mangle]
    fn WaterBox_GetSurface1(globalCtx: *mut GlobalContext,
                            colCtx: *mut CollisionContext, x: f32_0, z: f32_0,
                            ySurface: *mut f32_0,
                            outWaterBox: *mut *mut WaterBox) -> s32;
    #[no_mangle]
    fn Camera_Init(camera: *mut Camera, view: *mut View,
                   colCtx: *mut CollisionContext,
                   globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn Camera_InitPlayerSettings(camera: *mut Camera, player: *mut Player);
    #[no_mangle]
    fn Camera_ChangeStatus(camera: *mut Camera, status: s16) -> s16;
    #[no_mangle]
    fn Camera_Update(camera: *mut Camera) -> Vec3s;
    #[no_mangle]
    fn Camera_Finish(camera: *mut Camera);
    #[no_mangle]
    fn Camera_ChangeMode(camera: *mut Camera, mode: s16) -> s32;
    #[no_mangle]
    fn Camera_ChangeSetting(camera: *mut Camera, setting: s16) -> s32;
    #[no_mangle]
    fn Camera_ChangeDataIdx(camera: *mut Camera, camDataIdx: s32) -> s32;
    #[no_mangle]
    fn Camera_SetParam(camera: *mut Camera, param: s32,
                       value: *mut libc::c_void) -> s32;
    #[no_mangle]
    fn func_8005AC48(camera: *mut Camera, arg1: s16) -> s32;
    #[no_mangle]
    fn Camera_Copy(dstCamera: *mut Camera, srcCamera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_GetSkyboxOffset(dst: *mut Vec3f, camera: *mut Camera)
     -> *mut Vec3f;
    #[no_mangle]
    fn CollisionCheck_InitContext(globalCtx: *mut GlobalContext,
                                  colChkCtx: *mut CollisionCheckContext);
    #[no_mangle]
    fn CollisionCheck_DestroyContext(globalCtx: *mut GlobalContext,
                                     colChkCtx: *mut CollisionCheckContext);
    #[no_mangle]
    fn CollisionCheck_ClearContext(globalCtx: *mut GlobalContext,
                                   colChkCtx: *mut CollisionCheckContext);
    #[no_mangle]
    fn CollisionCheck_AT(globalCtx: *mut GlobalContext,
                         colChkCtx: *mut CollisionCheckContext);
    #[no_mangle]
    fn CollisionCheck_OC(globalCtx: *mut GlobalContext,
                         colChkCtx: *mut CollisionCheckContext);
    #[no_mangle]
    fn CollisionCheck_Damage(globalCtx: *mut GlobalContext,
                             colChkCtx: *mut CollisionCheckContext);
    #[no_mangle]
    fn DebugDisplay_Init();
    #[no_mangle]
    fn DebugDisplay_DrawObjects(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn func_8006450C(globalCtx: *mut GlobalContext,
                     csCtx: *mut CutsceneContext);
    #[no_mangle]
    fn func_80064558(globalCtx: *mut GlobalContext,
                     csCtx: *mut CutsceneContext);
    #[no_mangle]
    fn func_800645A0(globalCtx: *mut GlobalContext,
                     csCtx: *mut CutsceneContext);
    #[no_mangle]
    fn Cutscene_HandleEntranceTriggers(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn Cutscene_HandleConditionalTriggers(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn func_8006BA00(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn func_8006BA30(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn Flags_UnsetAllEnv(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn KaleidoSetup_Update(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn Environment_Init(globalCtx: *mut GlobalContext,
                        envCtx: *mut EnvironmentContext, unused: s32);
    #[no_mangle]
    fn Environment_UpdateSkybox(skyboxId: u8_0,
                                envCtx: *mut EnvironmentContext,
                                skyboxCtx: *mut SkyboxContext);
    #[no_mangle]
    fn Environment_Update(globalCtx: *mut GlobalContext,
                          envCtx: *mut EnvironmentContext,
                          lightCtx: *mut LightContext,
                          pauseCtx: *mut PauseContext,
                          msgCtx: *mut MessageContext,
                          gameOverCtx: *mut GameOverContext,
                          gfxCtx: *mut GraphicsContext);
    #[no_mangle]
    fn Environment_DrawSunAndMoon(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn Environment_DrawSunLensFlare(globalCtx: *mut GlobalContext,
                                    envCtx: *mut EnvironmentContext,
                                    view: *mut View,
                                    gfxCtx: *mut GraphicsContext, pos: Vec3f,
                                    unused: s32);
    #[no_mangle]
    fn Environment_DrawRain(globalCtx: *mut GlobalContext, view: *mut View,
                            gfxCtx: *mut GraphicsContext);
    #[no_mangle]
    fn Environment_DrawSkyboxFilters(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn Environment_UpdateLightningStrike(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn Environment_DrawLightning(globalCtx: *mut GlobalContext, unused: s32);
    #[no_mangle]
    fn func_800758AC(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn Environment_DrawCustomLensFlare(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn Environment_FillScreen(gfxCtx: *mut GraphicsContext, red: u8_0,
                              green: u8_0, blue: u8_0, alpha: u8_0,
                              drawFlags: u8_0);
    #[no_mangle]
    fn Environment_DrawSandstorm(globalCtx: *mut GlobalContext,
                                 sandstormState: u8_0);
    #[no_mangle]
    fn Environment_IsForcedSequenceDisabled() -> s32;
    #[no_mangle]
    fn Lights_Draw(lights: *mut Lights, gfxCtx: *mut GraphicsContext);
    #[no_mangle]
    fn Lights_BindAll(lights: *mut Lights, listHead: *mut LightNode,
                      vec: *mut Vec3f);
    #[no_mangle]
    fn LightContext_Init(globalCtx: *mut GlobalContext,
                         lightCtx: *mut LightContext);
    #[no_mangle]
    fn LightContext_NewLights(lightCtx: *mut LightContext,
                              gfxCtx: *mut GraphicsContext) -> *mut Lights;
    #[no_mangle]
    fn ZeldaArena_Display();
    #[no_mangle]
    fn ZeldaArena_Init(start: *mut libc::c_void, size: u32_0);
    #[no_mangle]
    fn ZeldaArena_Cleanup();
    #[no_mangle]
    fn MsgEvent_SendNullTask();
    #[no_mangle]
    fn OnePointCutscene_Init(globalCtx: *mut GlobalContext, csId: s16,
                             timer: s16, actor: *mut Actor, camIdx: s16)
     -> s16;
    #[no_mangle]
    fn Interface_ChangeAlpha(alphaType: u16_0);
    #[no_mangle]
    fn Interface_SetSceneRestrictions(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn Inventory_SwapAgeEquipment();
    #[no_mangle]
    fn Item_Give(globalCtx: *mut GlobalContext, item: u8_0) -> u8_0;
    #[no_mangle]
    fn Inventory_ReplaceItem(globalCtx: *mut GlobalContext, oldItem: u16_0,
                             newItem: u16_0) -> s32;
    #[no_mangle]
    fn func_800876C8(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn Interface_Draw(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn Interface_Update(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn FrameAdvance_Init(frameAdvCtx: *mut FrameAdvanceContext);
    #[no_mangle]
    fn FrameAdvance_Update(frameAdvCtx: *mut FrameAdvanceContext,
                           input: *mut Input) -> s32;
    #[no_mangle]
    fn Player_InCsMode(globalCtx: *mut GlobalContext) -> s32;
    #[no_mangle]
    fn Player_SetEquipmentData(globalCtx: *mut GlobalContext,
                               player: *mut Player);
    #[no_mangle]
    fn Quake_Init();
    #[no_mangle]
    fn Gfx_SetFog2(gfx: *mut Gfx, r: s32, g: s32, b: s32, a: s32, near: s32,
                   far: s32) -> *mut Gfx;
    #[no_mangle]
    fn func_80095248(gfxCtx: *mut GraphicsContext, r: u8_0, g: u8_0, b: u8_0);
    #[no_mangle]
    fn func_80095AA0(globalCtx: *mut GlobalContext, room: *mut Room,
                     arg2: *mut Input, arg3: s32);
    #[no_mangle]
    fn func_80096FD4(globalCtx: *mut GlobalContext, room: *mut Room);
    #[no_mangle]
    fn func_80096FE8(globalCtx: *mut GlobalContext, roomCtx: *mut RoomContext)
     -> u32_0;
    #[no_mangle]
    fn func_800973FC(globalCtx: *mut GlobalContext, roomCtx: *mut RoomContext)
     -> s32;
    #[no_mangle]
    fn Room_Draw(globalCtx: *mut GlobalContext, room: *mut Room,
                 flags: u32_0);
    #[no_mangle]
    fn Object_InitBank(globalCtx: *mut GlobalContext,
                       objectCtx: *mut ObjectContext);
    #[no_mangle]
    fn Object_UpdateBank(objectCtx: *mut ObjectContext);
    #[no_mangle]
    fn Scene_ExecuteCommands(globalCtx: *mut GlobalContext,
                             sceneCmd: *mut SceneCmd) -> s32;
    #[no_mangle]
    fn TransitionActor_InitContext(state: *mut GameState,
                                   transiActorCtx:
                                       *mut TransitionActorContext);
    #[no_mangle]
    fn Scene_Draw(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn AnimationContext_Reset(animationCtx: *mut AnimationContext);
    #[no_mangle]
    fn AnimationContext_Update(globalCtx: *mut GlobalContext,
                               animationCtx: *mut AnimationContext);
    #[no_mangle]
    fn Sram_Init(globalCtx: *mut GlobalContext, sramCtx: *mut SramContext);
    #[no_mangle]
    fn func_800AA178(_: u32_0);
    #[no_mangle]
    fn View_Init(_: *mut View, _: *mut GraphicsContext);
    #[no_mangle]
    fn func_800AA460(view: *mut View, fovy: f32_0, near: f32_0, far: f32_0);
    #[no_mangle]
    fn View_SetViewport(view: *mut View, viewport: *mut Viewport);
    #[no_mangle]
    fn func_800AAA50(view: *mut View, arg1: s32);
    #[no_mangle]
    fn func_800AB944(view: *mut View) -> s32;
    #[no_mangle]
    fn func_800AB9EC(view: *mut View, arg1: s32, p: *mut *mut Gfx) -> s32;
    #[no_mangle]
    fn VisMono_Init(this: *mut VisMono);
    #[no_mangle]
    fn VisMono_Destroy(this: *mut VisMono);
    #[no_mangle]
    fn VisMono_Draw(this: *mut VisMono, gfxp: *mut *mut Gfx);
    #[no_mangle]
    fn Skybox_Init(state: *mut GameState, skyboxCtx: *mut SkyboxContext,
                   skyboxId: s16);
    #[no_mangle]
    fn SkyboxDraw_UpdateMatrix(skyboxCtx: *mut SkyboxContext, x: f32_0,
                               y: f32_0, z: f32_0) -> *mut Mtx;
    #[no_mangle]
    fn SkyboxDraw_Draw(skyboxCtx: *mut SkyboxContext,
                       gfxCtx: *mut GraphicsContext, skyboxId: s16,
                       blend: s16, x: f32_0, y: f32_0, z: f32_0);
    #[no_mangle]
    fn SkyboxDraw_Update(skyboxCtx: *mut SkyboxContext);
    #[no_mangle]
    fn TransitionUnk_Destroy(this: *mut TransitionUnk);
    #[no_mangle]
    fn TransitionUnk_Init(this: *mut TransitionUnk, row: s32, col: s32)
     -> *mut TransitionUnk;
    #[no_mangle]
    fn TransitionUnk_Draw(this: *mut TransitionUnk, _: *mut *mut Gfx);
    #[no_mangle]
    fn func_800B23E8(this: *mut TransitionUnk);
    #[no_mangle]
    fn TransitionTriforce_Start(this: *mut libc::c_void);
    #[no_mangle]
    fn TransitionTriforce_Init(this: *mut libc::c_void) -> *mut libc::c_void;
    #[no_mangle]
    fn TransitionTriforce_Destroy(this: *mut libc::c_void);
    #[no_mangle]
    fn TransitionTriforce_Update(this: *mut libc::c_void, updateRate: s32);
    #[no_mangle]
    fn TransitionTriforce_SetColor(this: *mut libc::c_void, color: u32_0);
    #[no_mangle]
    fn TransitionTriforce_SetType(this: *mut libc::c_void, type_0: s32);
    #[no_mangle]
    fn TransitionTriforce_Draw(this: *mut libc::c_void, gfxP: *mut *mut Gfx);
    #[no_mangle]
    fn TransitionTriforce_IsDone(this: *mut libc::c_void) -> s32;
    #[no_mangle]
    fn TransitionWipe_Start(this: *mut libc::c_void);
    #[no_mangle]
    fn TransitionWipe_Init(this: *mut libc::c_void) -> *mut libc::c_void;
    #[no_mangle]
    fn TransitionWipe_Destroy(this: *mut libc::c_void);
    #[no_mangle]
    fn TransitionWipe_Update(this: *mut libc::c_void, updateRate: s32);
    #[no_mangle]
    fn TransitionWipe_Draw(this: *mut libc::c_void, gfxP: *mut *mut Gfx);
    #[no_mangle]
    fn TransitionWipe_IsDone(this: *mut libc::c_void) -> s32;
    #[no_mangle]
    fn TransitionWipe_SetType(this: *mut libc::c_void, type_0: s32);
    #[no_mangle]
    fn TransitionWipe_SetColor(this: *mut libc::c_void, color: u32_0);
    #[no_mangle]
    fn TransitionCircle_Start(thisx: *mut libc::c_void);
    #[no_mangle]
    fn TransitionCircle_Init(thisx: *mut libc::c_void) -> *mut libc::c_void;
    #[no_mangle]
    fn TransitionCircle_Destroy(thisx: *mut libc::c_void);
    #[no_mangle]
    fn TransitionCircle_Update(thisx: *mut libc::c_void, updateRate: s32);
    #[no_mangle]
    fn TransitionCircle_Draw(thisx: *mut libc::c_void, gfxP: *mut *mut Gfx);
    #[no_mangle]
    fn TransitionCircle_IsDone(thisx: *mut libc::c_void) -> s32;
    #[no_mangle]
    fn TransitionCircle_SetType(thisx: *mut libc::c_void, type_0: s32);
    #[no_mangle]
    fn TransitionCircle_SetColor(thisx: *mut libc::c_void, color: u32_0);
    #[no_mangle]
    fn TransitionCircle_SetEnvColor(thisx: *mut libc::c_void, color: u32_0);
    #[no_mangle]
    fn TransitionFade_Start(this: *mut libc::c_void);
    #[no_mangle]
    fn TransitionFade_Init(this: *mut libc::c_void) -> *mut libc::c_void;
    #[no_mangle]
    fn TransitionFade_Destroy(this: *mut libc::c_void);
    #[no_mangle]
    fn TransitionFade_Update(this: *mut libc::c_void, updateRate: s32);
    #[no_mangle]
    fn TransitionFade_Draw(this: *mut libc::c_void, gfxP: *mut *mut Gfx);
    #[no_mangle]
    fn TransitionFade_IsDone(this: *mut libc::c_void) -> s32;
    #[no_mangle]
    fn TransitionFade_SetColor(this: *mut libc::c_void, color: u32_0);
    #[no_mangle]
    fn TransitionFade_SetType(this: *mut libc::c_void, type_0: s32);
    #[no_mangle]
    fn ShrinkWindow_Init();
    #[no_mangle]
    fn ShrinkWindow_Destroy();
    #[no_mangle]
    fn ShrinkWindow_Update(updateRate: s32);
    #[no_mangle]
    fn KaleidoManager_Init(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn KaleidoManager_Destroy();
    #[no_mangle]
    fn KaleidoScopeCall_Init(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn KaleidoScopeCall_Destroy(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn KaleidoScopeCall_Update(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn KaleidoScopeCall_Draw(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn PreRender_SetValuesSave(this: *mut PreRender, width: u32_0,
                               height: u32_0, fbuf: *mut libc::c_void,
                               zbuf: *mut libc::c_void,
                               cvg: *mut libc::c_void);
    #[no_mangle]
    fn PreRender_Init(this: *mut PreRender);
    #[no_mangle]
    fn PreRender_SetValues(this: *mut PreRender, width: u32_0, height: u32_0,
                           fbuf: *mut libc::c_void, zbuf: *mut libc::c_void);
    #[no_mangle]
    fn PreRender_Destroy(this: *mut PreRender);
    #[no_mangle]
    fn func_800C1F20(this: *mut PreRender, gfxp: *mut *mut Gfx);
    #[no_mangle]
    fn func_800C20B4(this: *mut PreRender, gfxp: *mut *mut Gfx);
    #[no_mangle]
    fn func_800C24BC(this: *mut PreRender, gfxp: *mut *mut Gfx);
    #[no_mangle]
    fn PreRender_Calc(this: *mut PreRender);
    #[no_mangle]
    fn THA_GetSize(tha: *mut TwoHeadArena) -> s32;
    #[no_mangle]
    fn GameState_Realloc(gameState: *mut GameState, size: size_t);
    #[no_mangle]
    fn GameState_Alloc(gameState: *mut GameState, size: size_t,
                       file: *mut libc::c_char, line: s32)
     -> *mut libc::c_void;
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
    fn Graph_GfxPlusOne(gfx: *mut Gfx) -> *mut Gfx;
    #[no_mangle]
    fn Graph_BranchDlist(gfx: *mut Gfx, dst: *mut Gfx) -> *mut Gfx;
    #[no_mangle]
    fn Math3D_Vec3f_DistXYZ(a: *mut Vec3f, b: *mut Vec3f) -> f32_0;
    #[no_mangle]
    fn Matrix_Init(gameState: *mut GameState);
    #[no_mangle]
    fn Matrix_Get(dest: *mut MtxF);
    #[no_mangle]
    fn Matrix_Mult(mf: *mut MtxF, mode: u8_0);
    #[no_mangle]
    fn Matrix_MtxFToMtx(src: *mut MtxF, dest: *mut Mtx) -> *mut Mtx;
    #[no_mangle]
    fn Matrix_MultVec3f(src: *mut Vec3f, dest: *mut Vec3f);
    #[no_mangle]
    fn Matrix_MtxToMtxF(src: *mut Mtx, dest: *mut MtxF);
    #[no_mangle]
    fn Matrix_Transpose(mf: *mut MtxF);
    #[no_mangle]
    fn Matrix_CheckFloats(mf: *mut MtxF, file: *mut libc::c_char, line: s32)
     -> *mut MtxF;
    #[no_mangle]
    fn Fault_AddClient(_: *mut FaultClient, _: *mut libc::c_void,
                       _: *mut libc::c_void, _: *mut libc::c_void);
    #[no_mangle]
    fn Fault_RemoveClient(_: *mut FaultClient);
    #[no_mangle]
    fn Fault_AddHungupAndCrash(_: *const libc::c_char, _: u32_0);
    #[no_mangle]
    static mut D_801333E8: s8;
    #[no_mangle]
    static mut D_801333E0: f32_0;
    #[no_mangle]
    static mut D_801333D4: Vec3f;
    #[no_mangle]
    fn Audio_PlaySoundGeneral(sfxId: u16_0, pos: *mut Vec3f, token: u8_0,
                              freqScale: *mut f32_0, a4: *mut f32_0,
                              reverbAdd: *mut s8);
    #[no_mangle]
    static mut gSaveContext: SaveContext;
    #[no_mangle]
    static mut gGameInfo: *mut GameInfo;
    #[no_mangle]
    fn func_80110990(globalCtx: *mut GlobalContext);
    #[no_mangle]
    static mut gEntranceTable: [EntranceInfo; 1556];
    #[no_mangle]
    fn GameOver_FadeInLights(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn Message_Draw(globalCtx: *mut GlobalContext);
    #[no_mangle]
    static mut gZBuffer: [[u16_0; 320]; 240];
    #[no_mangle]
    static mut gSegments: [u32_0; 16];
    #[no_mangle]
    static mut gDbgCamEnabled: s32;
    #[no_mangle]
    fn Message_Update(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn GameOver_Update(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn FileChoose_Init(thisx: *mut GameState);
    #[no_mangle]
    fn func_800F6964(_: u16_0);
    #[no_mangle]
    static mut gObjectTable: [RomFile; 402];
    #[no_mangle]
    static mut gObjectTableSize: u32_0;
    #[no_mangle]
    fn Rand_Seed(seed: u32_0);
    #[no_mangle]
    fn Message_StartTextbox(globalCtx: *mut GlobalContext, textId: u16_0,
                            actor: *mut Actor);
    #[no_mangle]
    fn func_801109B0(globalCtx: *mut GlobalContext);
    #[no_mangle]
    static mut gSceneTable: [SceneTableEntry; 110];
    #[no_mangle]
    static mut gBitFlags: [u32_0; 32];
    #[no_mangle]
    fn GameOver_Init(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn Message_Init(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn func_80112098(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn Audio_SetExtraFilter(_: u8_0);
    #[no_mangle]
    fn SystemArena_Display();
    #[no_mangle]
    fn Opening_Init(thisx: *mut GameState);
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
pub type OSTime = u64_0;
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
pub type C2RustUnnamed_15 = libc::c_uint;
pub const ACTORCAT_CHEST: C2RustUnnamed_15 = 11;
pub const ACTORCAT_DOOR: C2RustUnnamed_15 = 10;
pub const ACTORCAT_BOSS: C2RustUnnamed_15 = 9;
pub const ACTORCAT_MISC: C2RustUnnamed_15 = 8;
pub const ACTORCAT_ITEMACTION: C2RustUnnamed_15 = 7;
pub const ACTORCAT_PROP: C2RustUnnamed_15 = 6;
pub const ACTORCAT_ENEMY: C2RustUnnamed_15 = 5;
pub const ACTORCAT_NPC: C2RustUnnamed_15 = 4;
pub const ACTORCAT_EXPLOSIVE: C2RustUnnamed_15 = 3;
pub const ACTORCAT_PLAYER: C2RustUnnamed_15 = 2;
pub const ACTORCAT_BG: C2RustUnnamed_15 = 1;
pub const ACTORCAT_SWITCH: C2RustUnnamed_15 = 0;
pub type C2RustUnnamed_16 = libc::c_uint;
pub const CS_STATE_UNSKIPPABLE_EXEC: C2RustUnnamed_16 = 4;
pub const CS_STATE_UNSKIPPABLE_INIT: C2RustUnnamed_16 = 3;
pub const CS_STATE_SKIPPABLE_EXEC: C2RustUnnamed_16 = 2;
pub const CS_STATE_SKIPPABLE_INIT: C2RustUnnamed_16 = 1;
pub const CS_STATE_IDLE: C2RustUnnamed_16 = 0;
pub type C2RustUnnamed_17 = libc::c_uint;
pub const CAM_SET_MAX: C2RustUnnamed_17 = 66;
pub const CAM_SET_NORMAL4: C2RustUnnamed_17 = 65;
pub const CAM_SET_PIVOT_FROM_SIDE: C2RustUnnamed_17 = 64;
pub const CAM_SET_DIRECTED_YAW: C2RustUnnamed_17 = 63;
pub const CAM_SET_DUNGEON2: C2RustUnnamed_17 = 62;
pub const CAM_SET_JABU_TENTACLE: C2RustUnnamed_17 = 61;
pub const CAM_SET_CS_C: C2RustUnnamed_17 = 60;
pub const CAM_SET_FISHING: C2RustUnnamed_17 = 59;
pub const CAM_SET_NORMAL2: C2RustUnnamed_17 = 58;
pub const CAM_SET_PIVOT_VERTICAL: C2RustUnnamed_17 = 57;
pub const CAM_SET_TURN_AROUND: C2RustUnnamed_17 = 56;
pub const CAM_SET_FIRE_BIRDS_EYE: C2RustUnnamed_17 = 55;
pub const CAM_SET_MEADOW_UNUSED: C2RustUnnamed_17 = 54;
pub const CAM_SET_MEADOW_BIRDS_EYE: C2RustUnnamed_17 = 53;
pub const CAM_SET_BIG_OCTO: C2RustUnnamed_17 = 52;
pub const CAM_SET_FOREST_DEFEAT_POE: C2RustUnnamed_17 = 51;
pub const CAM_SET_FOREST_UNUSED: C2RustUnnamed_17 = 50;
pub const CAM_SET_FIRE_STAIRCASE: C2RustUnnamed_17 = 49;
pub const CAM_SET_FIRE_PLATFORM: C2RustUnnamed_17 = 48;
pub const CAM_SET_SCENE_TRANSITION: C2RustUnnamed_17 = 47;
pub const CAM_SET_SCENE_UNUSED: C2RustUnnamed_17 = 46;
pub const CAM_SET_BEAN_LOST_WOODS: C2RustUnnamed_17 = 45;
pub const CAM_SET_BEAN_GENERIC: C2RustUnnamed_17 = 44;
pub const CAM_SET_CS_ATTENTION: C2RustUnnamed_17 = 43;
pub const CAM_SET_CS_3: C2RustUnnamed_17 = 42;
pub const CAM_SET_ITEM_UNUSED: C2RustUnnamed_17 = 41;
pub const CAM_SET_SLOW_CHEST_CS: C2RustUnnamed_17 = 40;
pub const CAM_SET_FOREST_BIRDS_EYE: C2RustUnnamed_17 = 39;
pub const CAM_SET_CS_TWISTED_HALLWAY: C2RustUnnamed_17 = 38;
pub const CAM_SET_CS_0: C2RustUnnamed_17 = 37;
pub const CAM_SET_PIVOT_WATER_SURFACE: C2RustUnnamed_17 = 36;
pub const CAM_SET_PIVOT_CORNER: C2RustUnnamed_17 = 35;
pub const CAM_SET_FREE2: C2RustUnnamed_17 = 34;
pub const CAM_SET_FREE0: C2RustUnnamed_17 = 33;
pub const CAM_SET_START1: C2RustUnnamed_17 = 32;
pub const CAM_SET_START0: C2RustUnnamed_17 = 31;
pub const CAM_SET_CRAWLSPACE: C2RustUnnamed_17 = 30;
pub const CAM_SET_DOORC: C2RustUnnamed_17 = 29;
pub const CAM_SET_DOOR0: C2RustUnnamed_17 = 28;
pub const CAM_SET_PREREND_SIDE_SCROLL: C2RustUnnamed_17 = 27;
pub const CAM_SET_PREREND_PIVET: C2RustUnnamed_17 = 26;
pub const CAM_SET_PREREND_FIXED: C2RustUnnamed_17 = 25;
pub const CAM_SET_PIVOT_IN_FRONT: C2RustUnnamed_17 = 24;
pub const CAM_SET_PIVOT_SHOP_BROWSING: C2RustUnnamed_17 = 23;
pub const CAM_SET_PIVOT_CRAWLSPACE: C2RustUnnamed_17 = 22;
pub const CAM_SET_CHU_BOWLING: C2RustUnnamed_17 = 21;
pub const CAM_SET_MARKET_BALCONY: C2RustUnnamed_17 = 20;
pub const CAM_SET_TOWER_UNUSED: C2RustUnnamed_17 = 19;
pub const CAM_SET_TOWER_CLIMB: C2RustUnnamed_17 = 18;
pub const CAM_SET_BOSS_GANON: C2RustUnnamed_17 = 17;
pub const CAM_SET_BOSS_GANONDORF: C2RustUnnamed_17 = 16;
pub const CAM_SET_BOSS_TWINROVA_FLOOR: C2RustUnnamed_17 = 15;
pub const CAM_SET_BOSS_TWINROVA_PLATFORM: C2RustUnnamed_17 = 14;
pub const CAM_SET_BOSS_MORPHA: C2RustUnnamed_17 = 13;
pub const CAM_SET_BOSS_BONGO: C2RustUnnamed_17 = 12;
pub const CAM_SET_BOSS_VOLVAGIA: C2RustUnnamed_17 = 11;
pub const CAM_SET_BOSS_PHANTOM_GANON: C2RustUnnamed_17 = 10;
pub const CAM_SET_BOSS_BARINADE: C2RustUnnamed_17 = 9;
pub const CAM_SET_BOSS_DODONGO: C2RustUnnamed_17 = 8;
pub const CAM_SET_BOSS_GOHMA: C2RustUnnamed_17 = 7;
pub const CAM_SET_HORSE: C2RustUnnamed_17 = 6;
pub const CAM_SET_NORMAL3: C2RustUnnamed_17 = 5;
pub const CAM_SET_DUNGEON1: C2RustUnnamed_17 = 4;
pub const CAM_SET_DUNGEON0: C2RustUnnamed_17 = 3;
pub const CAM_SET_NORMAL1: C2RustUnnamed_17 = 2;
pub const CAM_SET_NORMAL0: C2RustUnnamed_17 = 1;
pub const CAM_SET_NONE: C2RustUnnamed_17 = 0;
pub type C2RustUnnamed_18 = libc::c_uint;
pub const CAM_MODE_MAX: C2RustUnnamed_18 = 21;
pub const CAM_MODE_FOLLOWBOOMERANG: C2RustUnnamed_18 = 20;
pub const CAM_MODE_PUSHPULL: C2RustUnnamed_18 = 19;
pub const CAM_MODE_STILL: C2RustUnnamed_18 = 18;
pub const CAM_MODE_CHARGE: C2RustUnnamed_18 = 17;
pub const CAM_MODE_FREEFALL: C2RustUnnamed_18 = 16;
pub const CAM_MODE_HANGZ: C2RustUnnamed_18 = 15;
pub const CAM_MODE_HANG: C2RustUnnamed_18 = 14;
pub const CAM_MODE_JUMP: C2RustUnnamed_18 = 13;
pub const CAM_MODE_CLIMBZ: C2RustUnnamed_18 = 12;
pub const CAM_MODE_SLINGSHOT: C2RustUnnamed_18 = 11;
pub const CAM_MODE_BOOMERANG: C2RustUnnamed_18 = 10;
pub const CAM_MODE_HOOKSHOT: C2RustUnnamed_18 = 9;
pub const CAM_MODE_BOWARROWZ: C2RustUnnamed_18 = 8;
pub const CAM_MODE_BOWARROW: C2RustUnnamed_18 = 7;
pub const CAM_MODE_FIRSTPERSON: C2RustUnnamed_18 = 6;
pub const CAM_MODE_CLIMB: C2RustUnnamed_18 = 5;
pub const CAM_MODE_BATTLE: C2RustUnnamed_18 = 4;
pub const CAM_MODE_TALK: C2RustUnnamed_18 = 3;
pub const CAM_MODE_FOLLOWTARGET: C2RustUnnamed_18 = 2;
pub const CAM_MODE_TARGET: C2RustUnnamed_18 = 1;
pub const CAM_MODE_NORMAL: C2RustUnnamed_18 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdBase {
    pub code: u8_0,
    pub data1: u8_0,
    pub data2: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdSpawnList {
    pub code: u8_0,
    pub data1: u8_0,
    pub segment: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdActorList {
    pub code: u8_0,
    pub num: u8_0,
    pub segment: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdUnused02 {
    pub code: u8_0,
    pub data1: u8_0,
    pub segment: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdColHeader {
    pub code: u8_0,
    pub data1: u8_0,
    pub segment: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdRoomList {
    pub code: u8_0,
    pub num: u8_0,
    pub segment: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdWindSettings {
    pub code: u8_0,
    pub data1: u8_0,
    pub pad: [libc::c_char; 2],
    pub x: u8_0,
    pub y: u8_0,
    pub z: u8_0,
    pub unk_07: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdEntranceList {
    pub code: u8_0,
    pub data1: u8_0,
    pub segment: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdSpecialFiles {
    pub code: u8_0,
    pub cUpElfMsgNum: u8_0,
    pub keepObjectId: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdRoomBehavior {
    pub code: u8_0,
    pub gpFlag1: u8_0,
    pub gpFlag2: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdMesh {
    pub code: u8_0,
    pub data1: u8_0,
    pub segment: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdObjectList {
    pub code: u8_0,
    pub num: u8_0,
    pub segment: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdLightList {
    pub code: u8_0,
    pub num: u8_0,
    pub segment: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdPathList {
    pub code: u8_0,
    pub data1: u8_0,
    pub segment: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdTransiActorList {
    pub code: u8_0,
    pub num: u8_0,
    pub segment: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdLightSettingList {
    pub code: u8_0,
    pub num: u8_0,
    pub segment: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdTimeSettings {
    pub code: u8_0,
    pub data1: u8_0,
    pub pad: [libc::c_char; 2],
    pub hour: u8_0,
    pub min: u8_0,
    pub unk_06: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdSkyboxSettings {
    pub code: u8_0,
    pub data1: u8_0,
    pub pad: [libc::c_char; 2],
    pub skyboxId: u8_0,
    pub unk_05: u8_0,
    pub unk_06: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdSkyboxDisables {
    pub code: u8_0,
    pub data1: u8_0,
    pub pad: [libc::c_char; 2],
    pub unk_04: u8_0,
    pub unk_05: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdEndMarker {
    pub code: u8_0,
    pub data1: u8_0,
    pub data2: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdExitList {
    pub code: u8_0,
    pub data1: u8_0,
    pub segment: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdSoundSettings {
    pub code: u8_0,
    pub specId: u8_0,
    pub pad: [libc::c_char; 4],
    pub natureAmbienceId: u8_0,
    pub seqId: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdEchoSettings {
    pub code: u8_0,
    pub data1: u8_0,
    pub pad: [libc::c_char; 5],
    pub echo: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdCutsceneData {
    pub code: u8_0,
    pub data1: u8_0,
    pub segment: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdAltHeaders {
    pub code: u8_0,
    pub data1: u8_0,
    pub segment: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SCmdMiscSettings {
    pub code: u8_0,
    pub cameraMovement: u8_0,
    pub area: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union SceneCmd {
    pub base: SCmdBase,
    pub spawnList: SCmdSpawnList,
    pub actorList: SCmdActorList,
    pub unused02: SCmdUnused02,
    pub roomList: SCmdRoomList,
    pub entranceList: SCmdEntranceList,
    pub objectList: SCmdObjectList,
    pub lightList: SCmdLightList,
    pub pathList: SCmdPathList,
    pub transiActorList: SCmdTransiActorList,
    pub lightSettingList: SCmdLightSettingList,
    pub exitList: SCmdExitList,
    pub colHeader: SCmdColHeader,
    pub mesh: SCmdMesh,
    pub specialFiles: SCmdSpecialFiles,
    pub cutsceneData: SCmdCutsceneData,
    pub roomBehavior: SCmdRoomBehavior,
    pub windSettings: SCmdWindSettings,
    pub timeSettings: SCmdTimeSettings,
    pub skyboxSettings: SCmdSkyboxSettings,
    pub skyboxDisables: SCmdSkyboxDisables,
    pub endMarker: SCmdEndMarker,
    pub soundSettings: SCmdSoundSettings,
    pub echoSettings: SCmdEchoSettings,
    pub miscSettings: SCmdMiscSettings,
    pub altHeaders: SCmdAltHeaders,
}
pub type C2RustUnnamed_19 = libc::c_uint;
pub const SCENE_ID_MAX: C2RustUnnamed_19 = 110;
pub const SCENE_TESTROOM: C2RustUnnamed_19 = 109;
pub const SCENE_SASATEST: C2RustUnnamed_19 = 108;
pub const SCENE_HAIRAL_NIWA2: C2RustUnnamed_19 = 107;
pub const SCENE_SUTARU: C2RustUnnamed_19 = 106;
pub const SCENE_SYOTES2: C2RustUnnamed_19 = 105;
pub const SCENE_SYOTES: C2RustUnnamed_19 = 104;
pub const SCENE_DEPTH_TEST: C2RustUnnamed_19 = 103;
pub const SCENE_BESITU: C2RustUnnamed_19 = 102;
pub const SCENE_TEST01: C2RustUnnamed_19 = 101;
pub const SCENE_GANON_TOU: C2RustUnnamed_19 = 100;
pub const SCENE_SPOT20: C2RustUnnamed_19 = 99;
pub const SCENE_SPOT18: C2RustUnnamed_19 = 98;
pub const SCENE_SPOT17: C2RustUnnamed_19 = 97;
pub const SCENE_SPOT16: C2RustUnnamed_19 = 96;
pub const SCENE_SPOT15: C2RustUnnamed_19 = 95;
pub const SCENE_SPOT13: C2RustUnnamed_19 = 94;
pub const SCENE_SPOT12: C2RustUnnamed_19 = 93;
pub const SCENE_SPOT11: C2RustUnnamed_19 = 92;
pub const SCENE_SPOT10: C2RustUnnamed_19 = 91;
pub const SCENE_SPOT09: C2RustUnnamed_19 = 90;
pub const SCENE_SPOT08: C2RustUnnamed_19 = 89;
pub const SCENE_SPOT07: C2RustUnnamed_19 = 88;
pub const SCENE_SPOT06: C2RustUnnamed_19 = 87;
pub const SCENE_SPOT05: C2RustUnnamed_19 = 86;
pub const SCENE_SPOT04: C2RustUnnamed_19 = 85;
pub const SCENE_SPOT03: C2RustUnnamed_19 = 84;
pub const SCENE_SPOT02: C2RustUnnamed_19 = 83;
pub const SCENE_SPOT01: C2RustUnnamed_19 = 82;
pub const SCENE_SPOT00: C2RustUnnamed_19 = 81;
pub const SCENE_KINSUTA: C2RustUnnamed_19 = 80;
pub const SCENE_GANON_DEMO: C2RustUnnamed_19 = 79;
pub const SCENE_MAHOUYA: C2RustUnnamed_19 = 78;
pub const SCENE_MIHARIGOYA: C2RustUnnamed_19 = 77;
pub const SCENE_SOUKO: C2RustUnnamed_19 = 76;
pub const SCENE_BOWLING: C2RustUnnamed_19 = 75;
pub const SCENE_NAKANIWA: C2RustUnnamed_19 = 74;
pub const SCENE_TURIBORI: C2RustUnnamed_19 = 73;
pub const SCENE_HAKASITARELAY: C2RustUnnamed_19 = 72;
pub const SCENE_HIRAL_DEMO: C2RustUnnamed_19 = 71;
pub const SCENE_HAIRAL_NIWA_N: C2RustUnnamed_19 = 70;
pub const SCENE_HAIRAL_NIWA: C2RustUnnamed_19 = 69;
pub const SCENE_KENJYANOMA: C2RustUnnamed_19 = 68;
pub const SCENE_TOKINOMA: C2RustUnnamed_19 = 67;
pub const SCENE_SYATEKIJYOU: C2RustUnnamed_19 = 66;
pub const SCENE_HAKAANA_OUKE: C2RustUnnamed_19 = 65;
pub const SCENE_HAKAANA2: C2RustUnnamed_19 = 64;
pub const SCENE_HAKAANA: C2RustUnnamed_19 = 63;
pub const SCENE_KAKUSIANA: C2RustUnnamed_19 = 62;
pub const SCENE_YOUSEI_IZUMI_YOKO: C2RustUnnamed_19 = 61;
pub const SCENE_YOUSEI_IZUMI_TATE: C2RustUnnamed_19 = 60;
pub const SCENE_DAIYOUSEI_IZUMI: C2RustUnnamed_19 = 59;
pub const SCENE_HUT: C2RustUnnamed_19 = 58;
pub const SCENE_TENT: C2RustUnnamed_19 = 57;
pub const SCENE_HYLIA_LABO: C2RustUnnamed_19 = 56;
pub const SCENE_LABO: C2RustUnnamed_19 = 55;
pub const SCENE_MALON_STABLE: C2RustUnnamed_19 = 54;
pub const SCENE_IMPA: C2RustUnnamed_19 = 53;
pub const SCENE_LINK_HOME: C2RustUnnamed_19 = 52;
pub const SCENE_FACE_SHOP: C2RustUnnamed_19 = 51;
pub const SCENE_NIGHT_SHOP: C2RustUnnamed_19 = 50;
pub const SCENE_ALLEY_SHOP: C2RustUnnamed_19 = 49;
pub const SCENE_DRAG: C2RustUnnamed_19 = 48;
pub const SCENE_ZOORA: C2RustUnnamed_19 = 47;
pub const SCENE_GOLON: C2RustUnnamed_19 = 46;
pub const SCENE_KOKIRI_SHOP: C2RustUnnamed_19 = 45;
pub const SCENE_SHOP1: C2RustUnnamed_19 = 44;
pub const SCENE_KAKARIKO3: C2RustUnnamed_19 = 43;
pub const SCENE_KAKARIKO: C2RustUnnamed_19 = 42;
pub const SCENE_KOKIRI_HOME5: C2RustUnnamed_19 = 41;
pub const SCENE_KOKIRI_HOME4: C2RustUnnamed_19 = 40;
pub const SCENE_KOKIRI_HOME3: C2RustUnnamed_19 = 39;
pub const SCENE_KOKIRI_HOME: C2RustUnnamed_19 = 38;
pub const SCENE_SHRINE_R: C2RustUnnamed_19 = 37;
pub const SCENE_SHRINE_N: C2RustUnnamed_19 = 36;
pub const SCENE_SHRINE: C2RustUnnamed_19 = 35;
pub const SCENE_MARKET_RUINS: C2RustUnnamed_19 = 34;
pub const SCENE_MARKET_NIGHT: C2RustUnnamed_19 = 33;
pub const SCENE_MARKET_DAY: C2RustUnnamed_19 = 32;
pub const SCENE_MARKET_ALLEY_N: C2RustUnnamed_19 = 31;
pub const SCENE_MARKET_ALLEY: C2RustUnnamed_19 = 30;
pub const SCENE_ENRUI: C2RustUnnamed_19 = 29;
pub const SCENE_ENTRA_N: C2RustUnnamed_19 = 28;
pub const SCENE_ENTRA: C2RustUnnamed_19 = 27;
pub const SCENE_GANON_FINAL: C2RustUnnamed_19 = 26;
pub const SCENE_GANON_BOSS: C2RustUnnamed_19 = 25;
pub const SCENE_HAKADAN_BS: C2RustUnnamed_19 = 24;
pub const SCENE_JYASINBOSS: C2RustUnnamed_19 = 23;
pub const SCENE_MIZUSIN_BS: C2RustUnnamed_19 = 22;
pub const SCENE_FIRE_BS: C2RustUnnamed_19 = 21;
pub const SCENE_MORIBOSSROOM: C2RustUnnamed_19 = 20;
pub const SCENE_BDAN_BOSS: C2RustUnnamed_19 = 19;
pub const SCENE_DDAN_BOSS: C2RustUnnamed_19 = 18;
pub const SCENE_YDAN_BOSS: C2RustUnnamed_19 = 17;
pub const SCENE_TAKARAYA: C2RustUnnamed_19 = 16;
pub const SCENE_GANONTIKA_SONOGO: C2RustUnnamed_19 = 15;
pub const SCENE_GANON_SONOGO: C2RustUnnamed_19 = 14;
pub const SCENE_GANONTIKA: C2RustUnnamed_19 = 13;
pub const SCENE_GERUDOWAY: C2RustUnnamed_19 = 12;
pub const SCENE_MEN: C2RustUnnamed_19 = 11;
pub const SCENE_GANON: C2RustUnnamed_19 = 10;
pub const SCENE_ICE_DOUKUTO: C2RustUnnamed_19 = 9;
pub const SCENE_HAKADANCH: C2RustUnnamed_19 = 8;
pub const SCENE_HAKADAN: C2RustUnnamed_19 = 7;
pub const SCENE_JYASINZOU: C2RustUnnamed_19 = 6;
pub const SCENE_MIZUSIN: C2RustUnnamed_19 = 5;
pub const SCENE_HIDAN: C2RustUnnamed_19 = 4;
pub const SCENE_BMORI1: C2RustUnnamed_19 = 3;
pub const SCENE_BDAN: C2RustUnnamed_19 = 2;
pub const SCENE_DDAN: C2RustUnnamed_19 = 1;
pub const SCENE_YDAN: C2RustUnnamed_19 = 0;
pub type C2RustUnnamed_20 = libc::c_uint;
pub const QUEST_HEART_PIECE: C2RustUnnamed_20 = 24;
pub const QUEST_SKULL_TOKEN: C2RustUnnamed_20 = 23;
pub const QUEST_GERUDO_CARD: C2RustUnnamed_20 = 22;
pub const QUEST_STONE_OF_AGONY: C2RustUnnamed_20 = 21;
pub const QUEST_ZORA_SAPPHIRE: C2RustUnnamed_20 = 20;
pub const QUEST_GORON_RUBY: C2RustUnnamed_20 = 19;
pub const QUEST_KOKIRI_EMERALD: C2RustUnnamed_20 = 18;
pub const QUEST_SONG_STORMS: C2RustUnnamed_20 = 17;
pub const QUEST_SONG_TIME: C2RustUnnamed_20 = 16;
pub const QUEST_SONG_SUN: C2RustUnnamed_20 = 15;
pub const QUEST_SONG_SARIA: C2RustUnnamed_20 = 14;
pub const QUEST_SONG_EPONA: C2RustUnnamed_20 = 13;
pub const QUEST_SONG_LULLABY: C2RustUnnamed_20 = 12;
pub const QUEST_SONG_PRELUDE: C2RustUnnamed_20 = 11;
pub const QUEST_SONG_NOCTURNE: C2RustUnnamed_20 = 10;
pub const QUEST_SONG_REQUIEM: C2RustUnnamed_20 = 9;
pub const QUEST_SONG_SERENADE: C2RustUnnamed_20 = 8;
pub const QUEST_SONG_BOLERO: C2RustUnnamed_20 = 7;
pub const QUEST_SONG_MINUET: C2RustUnnamed_20 = 6;
pub const QUEST_MEDALLION_LIGHT: C2RustUnnamed_20 = 5;
pub const QUEST_MEDALLION_SHADOW: C2RustUnnamed_20 = 4;
pub const QUEST_MEDALLION_SPIRIT: C2RustUnnamed_20 = 3;
pub const QUEST_MEDALLION_WATER: C2RustUnnamed_20 = 2;
pub const QUEST_MEDALLION_FIRE: C2RustUnnamed_20 = 1;
pub const QUEST_MEDALLION_FOREST: C2RustUnnamed_20 = 0;
pub type C2RustUnnamed_21 = libc::c_uint;
pub const ITEM_NONE: C2RustUnnamed_21 = 255;
pub const ITEM_NONE_FE: C2RustUnnamed_21 = 254;
pub const ITEM_LAST_USED: C2RustUnnamed_21 = 252;
pub const ITEM_NUT_UPGRADE_40: C2RustUnnamed_21 = 155;
pub const ITEM_NUT_UPGRADE_30: C2RustUnnamed_21 = 154;
pub const ITEM_STICK_UPGRADE_30: C2RustUnnamed_21 = 153;
pub const ITEM_STICK_UPGRADE_20: C2RustUnnamed_21 = 152;
pub const ITEM_BOMBCHUS_20: C2RustUnnamed_21 = 151;
pub const ITEM_BOMBCHUS_5: C2RustUnnamed_21 = 150;
pub const ITEM_SEEDS_30: C2RustUnnamed_21 = 149;
pub const ITEM_ARROWS_LARGE: C2RustUnnamed_21 = 148;
pub const ITEM_ARROWS_MEDIUM: C2RustUnnamed_21 = 147;
pub const ITEM_ARROWS_SMALL: C2RustUnnamed_21 = 146;
pub const ITEM_BOMBS_30: C2RustUnnamed_21 = 145;
pub const ITEM_BOMBS_20: C2RustUnnamed_21 = 144;
pub const ITEM_BOMBS_10: C2RustUnnamed_21 = 143;
pub const ITEM_BOMBS_5: C2RustUnnamed_21 = 142;
pub const ITEM_NUTS_10: C2RustUnnamed_21 = 141;
pub const ITEM_NUTS_5: C2RustUnnamed_21 = 140;
pub const ITEM_STICKS_10: C2RustUnnamed_21 = 139;
pub const ITEM_STICKS_5: C2RustUnnamed_21 = 138;
pub const ITEM_INVALID_8: C2RustUnnamed_21 = 137;
pub const ITEM_RUPEE_GOLD: C2RustUnnamed_21 = 136;
pub const ITEM_RUPEE_PURPLE: C2RustUnnamed_21 = 135;
pub const ITEM_RUPEE_RED: C2RustUnnamed_21 = 134;
pub const ITEM_RUPEE_BLUE: C2RustUnnamed_21 = 133;
pub const ITEM_RUPEE_GREEN: C2RustUnnamed_21 = 132;
pub const ITEM_HEART: C2RustUnnamed_21 = 131;
pub const ITEM_MILK: C2RustUnnamed_21 = 130;
pub const ITEM_INVALID_7: C2RustUnnamed_21 = 129;
pub const ITEM_INVALID_6: C2RustUnnamed_21 = 128;
pub const ITEM_INVALID_5: C2RustUnnamed_21 = 127;
pub const ITEM_INVALID_4: C2RustUnnamed_21 = 126;
pub const ITEM_INVALID_3: C2RustUnnamed_21 = 125;
pub const ITEM_INVALID_2: C2RustUnnamed_21 = 124;
pub const ITEM_INVALID_1: C2RustUnnamed_21 = 123;
pub const ITEM_HEART_PIECE_2: C2RustUnnamed_21 = 122;
pub const ITEM_MAGIC_LARGE: C2RustUnnamed_21 = 121;
pub const ITEM_MAGIC_SMALL: C2RustUnnamed_21 = 120;
pub const ITEM_KEY_SMALL: C2RustUnnamed_21 = 119;
pub const ITEM_DUNGEON_MAP: C2RustUnnamed_21 = 118;
pub const ITEM_COMPASS: C2RustUnnamed_21 = 117;
pub const ITEM_KEY_BOSS: C2RustUnnamed_21 = 116;
pub const ITEM_HEART_PIECE: C2RustUnnamed_21 = 115;
pub const ITEM_HEART_CONTAINER: C2RustUnnamed_21 = 114;
pub const ITEM_SKULL_TOKEN: C2RustUnnamed_21 = 113;
pub const ITEM_GERUDO_CARD: C2RustUnnamed_21 = 112;
pub const ITEM_STONE_OF_AGONY: C2RustUnnamed_21 = 111;
pub const ITEM_ZORA_SAPPHIRE: C2RustUnnamed_21 = 110;
pub const ITEM_GORON_RUBY: C2RustUnnamed_21 = 109;
pub const ITEM_KOKIRI_EMERALD: C2RustUnnamed_21 = 108;
pub const ITEM_MEDALLION_LIGHT: C2RustUnnamed_21 = 107;
pub const ITEM_MEDALLION_SHADOW: C2RustUnnamed_21 = 106;
pub const ITEM_MEDALLION_SPIRIT: C2RustUnnamed_21 = 105;
pub const ITEM_MEDALLION_WATER: C2RustUnnamed_21 = 104;
pub const ITEM_MEDALLION_FIRE: C2RustUnnamed_21 = 103;
pub const ITEM_MEDALLION_FOREST: C2RustUnnamed_21 = 102;
pub const ITEM_SONG_STORMS: C2RustUnnamed_21 = 101;
pub const ITEM_SONG_TIME: C2RustUnnamed_21 = 100;
pub const ITEM_SONG_SUN: C2RustUnnamed_21 = 99;
pub const ITEM_SONG_SARIA: C2RustUnnamed_21 = 98;
pub const ITEM_SONG_EPONA: C2RustUnnamed_21 = 97;
pub const ITEM_SONG_LULLABY: C2RustUnnamed_21 = 96;
pub const ITEM_SONG_PRELUDE: C2RustUnnamed_21 = 95;
pub const ITEM_SONG_NOCTURNE: C2RustUnnamed_21 = 94;
pub const ITEM_SONG_REQUIEM: C2RustUnnamed_21 = 93;
pub const ITEM_SONG_SERENADE: C2RustUnnamed_21 = 92;
pub const ITEM_SONG_BOLERO: C2RustUnnamed_21 = 91;
pub const ITEM_SONG_MINUET: C2RustUnnamed_21 = 90;
pub const ITEM_FISHING_POLE: C2RustUnnamed_21 = 89;
pub const ITEM_SEEDS: C2RustUnnamed_21 = 88;
pub const ITEM_WALLET_GIANT: C2RustUnnamed_21 = 87;
pub const ITEM_WALLET_ADULT: C2RustUnnamed_21 = 86;
pub const ITEM_SWORD_KNIFE: C2RustUnnamed_21 = 85;
pub const ITEM_SCALE_GOLDEN: C2RustUnnamed_21 = 84;
pub const ITEM_SCALE_SILVER: C2RustUnnamed_21 = 83;
pub const ITEM_GAUNTLETS_GOLD: C2RustUnnamed_21 = 82;
pub const ITEM_GAUNTLETS_SILVER: C2RustUnnamed_21 = 81;
pub const ITEM_BRACELET: C2RustUnnamed_21 = 80;
pub const ITEM_BOMB_BAG_40: C2RustUnnamed_21 = 79;
pub const ITEM_BOMB_BAG_30: C2RustUnnamed_21 = 78;
pub const ITEM_BOMB_BAG_20: C2RustUnnamed_21 = 77;
pub const ITEM_QUIVER_50: C2RustUnnamed_21 = 76;
pub const ITEM_QUIVER_40: C2RustUnnamed_21 = 75;
pub const ITEM_QUIVER_30: C2RustUnnamed_21 = 74;
pub const ITEM_BULLET_BAG_50: C2RustUnnamed_21 = 73;
pub const ITEM_BULLET_BAG_40: C2RustUnnamed_21 = 72;
pub const ITEM_BULLET_BAG_30: C2RustUnnamed_21 = 71;
pub const ITEM_BOOTS_HOVER: C2RustUnnamed_21 = 70;
pub const ITEM_BOOTS_IRON: C2RustUnnamed_21 = 69;
pub const ITEM_BOOTS_KOKIRI: C2RustUnnamed_21 = 68;
pub const ITEM_TUNIC_ZORA: C2RustUnnamed_21 = 67;
pub const ITEM_TUNIC_GORON: C2RustUnnamed_21 = 66;
pub const ITEM_TUNIC_KOKIRI: C2RustUnnamed_21 = 65;
pub const ITEM_SHIELD_MIRROR: C2RustUnnamed_21 = 64;
pub const ITEM_SHIELD_HYLIAN: C2RustUnnamed_21 = 63;
pub const ITEM_SHIELD_DEKU: C2RustUnnamed_21 = 62;
pub const ITEM_SWORD_BGS: C2RustUnnamed_21 = 61;
pub const ITEM_SWORD_MASTER: C2RustUnnamed_21 = 60;
pub const ITEM_SWORD_KOKIRI: C2RustUnnamed_21 = 59;
pub const ITEM_BOW_ARROW_LIGHT: C2RustUnnamed_21 = 58;
pub const ITEM_BOW_ARROW_ICE: C2RustUnnamed_21 = 57;
pub const ITEM_BOW_ARROW_FIRE: C2RustUnnamed_21 = 56;
pub const ITEM_CLAIM_CHECK: C2RustUnnamed_21 = 55;
pub const ITEM_EYEDROPS: C2RustUnnamed_21 = 54;
pub const ITEM_FROG: C2RustUnnamed_21 = 53;
pub const ITEM_PRESCRIPTION: C2RustUnnamed_21 = 52;
pub const ITEM_SWORD_BROKEN: C2RustUnnamed_21 = 51;
pub const ITEM_SAW: C2RustUnnamed_21 = 50;
pub const ITEM_ODD_POTION: C2RustUnnamed_21 = 49;
pub const ITEM_ODD_MUSHROOM: C2RustUnnamed_21 = 48;
pub const ITEM_COJIRO: C2RustUnnamed_21 = 47;
pub const ITEM_POCKET_CUCCO: C2RustUnnamed_21 = 46;
pub const ITEM_POCKET_EGG: C2RustUnnamed_21 = 45;
pub const ITEM_SOLD_OUT: C2RustUnnamed_21 = 44;
pub const ITEM_MASK_TRUTH: C2RustUnnamed_21 = 43;
pub const ITEM_MASK_GERUDO: C2RustUnnamed_21 = 42;
pub const ITEM_MASK_ZORA: C2RustUnnamed_21 = 41;
pub const ITEM_MASK_GORON: C2RustUnnamed_21 = 40;
pub const ITEM_MASK_BUNNY: C2RustUnnamed_21 = 39;
pub const ITEM_MASK_SPOOKY: C2RustUnnamed_21 = 38;
pub const ITEM_MASK_SKULL: C2RustUnnamed_21 = 37;
pub const ITEM_MASK_KEATON: C2RustUnnamed_21 = 36;
pub const ITEM_LETTER_ZELDA: C2RustUnnamed_21 = 35;
pub const ITEM_CHICKEN: C2RustUnnamed_21 = 34;
pub const ITEM_WEIRD_EGG: C2RustUnnamed_21 = 33;
pub const ITEM_POE: C2RustUnnamed_21 = 32;
pub const ITEM_MILK_HALF: C2RustUnnamed_21 = 31;
pub const ITEM_BIG_POE: C2RustUnnamed_21 = 30;
pub const ITEM_BUG: C2RustUnnamed_21 = 29;
pub const ITEM_BLUE_FIRE: C2RustUnnamed_21 = 28;
pub const ITEM_LETTER_RUTO: C2RustUnnamed_21 = 27;
pub const ITEM_MILK_BOTTLE: C2RustUnnamed_21 = 26;
pub const ITEM_FISH: C2RustUnnamed_21 = 25;
pub const ITEM_FAIRY: C2RustUnnamed_21 = 24;
pub const ITEM_POTION_BLUE: C2RustUnnamed_21 = 23;
pub const ITEM_POTION_GREEN: C2RustUnnamed_21 = 22;
pub const ITEM_POTION_RED: C2RustUnnamed_21 = 21;
pub const ITEM_BOTTLE: C2RustUnnamed_21 = 20;
pub const ITEM_NAYRUS_LOVE: C2RustUnnamed_21 = 19;
pub const ITEM_ARROW_LIGHT: C2RustUnnamed_21 = 18;
pub const ITEM_HAMMER: C2RustUnnamed_21 = 17;
pub const ITEM_BEAN: C2RustUnnamed_21 = 16;
pub const ITEM_LENS: C2RustUnnamed_21 = 15;
pub const ITEM_BOOMERANG: C2RustUnnamed_21 = 14;
pub const ITEM_FARORES_WIND: C2RustUnnamed_21 = 13;
pub const ITEM_ARROW_ICE: C2RustUnnamed_21 = 12;
pub const ITEM_LONGSHOT: C2RustUnnamed_21 = 11;
pub const ITEM_HOOKSHOT: C2RustUnnamed_21 = 10;
pub const ITEM_BOMBCHU: C2RustUnnamed_21 = 9;
pub const ITEM_OCARINA_TIME: C2RustUnnamed_21 = 8;
pub const ITEM_OCARINA_FAIRY: C2RustUnnamed_21 = 7;
pub const ITEM_SLINGSHOT: C2RustUnnamed_21 = 6;
pub const ITEM_DINS_FIRE: C2RustUnnamed_21 = 5;
pub const ITEM_ARROW_FIRE: C2RustUnnamed_21 = 4;
pub const ITEM_BOW: C2RustUnnamed_21 = 3;
pub const ITEM_BOMB: C2RustUnnamed_21 = 2;
pub const ITEM_NUT: C2RustUnnamed_21 = 1;
pub const ITEM_STICK: C2RustUnnamed_21 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TransitionUnkData {
    pub unk_0: f32_0,
    pub unk_4: f32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TransitionUnk {
    pub row: s32,
    pub col: s32,
    pub frame: s32,
    pub unk_0C: *mut TransitionUnkData,
    pub vtxFrame1: *mut Vtx,
    pub vtxFrame2: *mut Vtx,
    pub projection: Mtx,
    pub modelView: Mtx,
    pub unk_98: Mtx,
    pub gfx: *mut Gfx,
    pub zBuffer: *mut u16_0,
}
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
pub const SKYBOX_UNSET_27: C2RustUnnamed_22 = 39;
pub const SKYBOX_HOUSE_ALLEY: C2RustUnnamed_22 = 34;
pub const SKYBOX_HOUSE_SARIA: C2RustUnnamed_22 = 33;
pub const SKYBOX_HOUSE_MIDO: C2RustUnnamed_22 = 32;
pub const SKYBOX_UNSET_1D: C2RustUnnamed_22 = 29;
pub const SKYBOX_TENT: C2RustUnnamed_22 = 28;
pub const SKYBOX_HOUSE_IMPA: C2RustUnnamed_22 = 27;
pub const SKYBOX_HOUSE_RICHARD: C2RustUnnamed_22 = 26;
pub const SKYBOX_BOMBCHU_SHOP: C2RustUnnamed_22 = 24;
pub const SKYBOX_POTION_SHOP_MARKET: C2RustUnnamed_22 = 23;
pub const SKYBOX_POTION_SHOP_KAKARIKO: C2RustUnnamed_22 = 22;
pub const SKYBOX_ZORA_SHOP: C2RustUnnamed_22 = 20;
pub const SKYBOX_GORON_SHOP: C2RustUnnamed_22 = 19;
pub const SKYBOX_KOKIRI_SHOP: C2RustUnnamed_22 = 17;
pub const SKYBOX_HOUSE_KAKARIKO: C2RustUnnamed_22 = 16;
pub const SKYBOX_STABLES: C2RustUnnamed_22 = 15;
pub const SKYBOX_HOUSE_OF_TWINS: C2RustUnnamed_22 = 14;
pub const SKYBOX_HOUSE_KNOW_IT_ALL_BROTHERS: C2RustUnnamed_22 = 12;
pub const SKYBOX_HAPPY_MASK_SHOP: C2RustUnnamed_22 = 11;
pub const SKYBOX_MARKET_CHILD_NIGHT: C2RustUnnamed_22 = 10;
pub const SKYBOX_MARKET_CHILD_DAY: C2RustUnnamed_22 = 9;
pub const SKYBOX_HOUSE_LINK: C2RustUnnamed_22 = 7;
pub const SKYBOX_CUTSCENE_MAP: C2RustUnnamed_22 = 5;
pub const SKYBOX_MARKET_ADULT: C2RustUnnamed_22 = 4;
pub const SKYBOX_OVERCAST_SUNSET: C2RustUnnamed_22 = 3;
pub const SKYBOX_BAZAAR: C2RustUnnamed_22 = 2;
pub const SKYBOX_NORMAL_SKY: C2RustUnnamed_22 = 1;
pub const SKYBOX_NONE: C2RustUnnamed_22 = 0;
pub type C2RustUnnamed_23 = libc::c_uint;
pub const MSGMODE_PAUSED: C2RustUnnamed_23 = 55;
pub const MSGMODE_TEXT_CLOSING: C2RustUnnamed_23 = 54;
pub const MSGMODE_TEXT_DONE: C2RustUnnamed_23 = 53;
pub const MSGMODE_TEXT_AWAIT_NEXT: C2RustUnnamed_23 = 52;
pub const MSGMODE_FROGS_WAITING: C2RustUnnamed_23 = 51;
pub const MSGMODE_FROGS_PLAYING: C2RustUnnamed_23 = 50;
pub const MSGMODE_FROGS_START: C2RustUnnamed_23 = 49;
pub const MSGMODE_MEMORY_GAME_START_NEXT_ROUND: C2RustUnnamed_23 = 48;
pub const MSGMODE_MEMORY_GAME_ROUND_SUCCESS: C2RustUnnamed_23 = 47;
pub const MSGMODE_MEMORY_GAME_PLAYER_PLAYING: C2RustUnnamed_23 = 46;
pub const MSGMODE_MEMORY_GAME_RIGHT_SKULLKID_WAIT: C2RustUnnamed_23 = 45;
pub const MSGMODE_MEMORY_GAME_RIGHT_SKULLKID_PLAYING: C2RustUnnamed_23 = 44;
pub const MSGMODE_MEMORY_GAME_LEFT_SKULLKID_WAIT: C2RustUnnamed_23 = 43;
pub const MSGMODE_MEMORY_GAME_LEFT_SKULLKID_PLAYING: C2RustUnnamed_23 = 42;
pub const MSGMODE_MEMORY_GAME_START: C2RustUnnamed_23 = 41;
pub const MSGMODE_SCARECROW_PLAYBACK: C2RustUnnamed_23 = 40;
pub const MSGMODE_SCARECROW_RECORDING_DONE: C2RustUnnamed_23 = 39;
pub const MSGMODE_SCARECROW_RECORDING_FAILED: C2RustUnnamed_23 = 38;
pub const MSGMODE_SCARECROW_RECORDING_ONGOING: C2RustUnnamed_23 = 37;
pub const MSGMODE_SCARECROW_RECORDING_START: C2RustUnnamed_23 = 36;
pub const MSGMODE_SCARECROW_LONG_PLAYBACK: C2RustUnnamed_23 = 35;
pub const MSGMODE_SCARECROW_LONG_RECORDING_ONGOING: C2RustUnnamed_23 = 34;
pub const MSGMODE_SCARECROW_LONG_RECORDING_START: C2RustUnnamed_23 = 33;
pub const MSGMODE_UNK_20: C2RustUnnamed_23 = 32;
pub const MSGMODE_OCARINA_AWAIT_INPUT: C2RustUnnamed_23 = 31;
pub const MSGMODE_SONG_PLAYBACK_NOTES_DROP: C2RustUnnamed_23 = 30;
pub const MSGMODE_SONG_PLAYBACK_FAIL: C2RustUnnamed_23 = 29;
pub const MSGMODE_SONG_PLAYBACK_SUCCESS: C2RustUnnamed_23 = 28;
pub const MSGMODE_SONG_PLAYBACK: C2RustUnnamed_23 = 27;
pub const MSGMODE_SONG_DEMONSTRATION_DONE: C2RustUnnamed_23 = 26;
pub const MSGMODE_SONG_DEMONSTRATION: C2RustUnnamed_23 = 25;
pub const MSGMODE_SONG_DEMONSTRATION_SELECT_INSTRUMENT: C2RustUnnamed_23 = 24;
pub const MSGMODE_SONG_PLAYED_ACT: C2RustUnnamed_23 = 23;
pub const MSGMODE_SONG_PLAYED_ACT_BEGIN: C2RustUnnamed_23 = 22;
pub const MSGMODE_DISPLAY_SONG_PLAYED_TEXT: C2RustUnnamed_23 = 21;
pub const MSGMODE_DISPLAY_SONG_PLAYED_TEXT_BEGIN: C2RustUnnamed_23 = 20;
pub const MSGMODE_DISPLAY_SONG_PLAYED: C2RustUnnamed_23 = 19;
pub const MSGMODE_SETUP_DISPLAY_SONG_PLAYED: C2RustUnnamed_23 = 18;
pub const MSGMODE_SONG_PLAYED: C2RustUnnamed_23 = 17;
pub const MSGMODE_OCARINA_NOTES_DROP: C2RustUnnamed_23 = 16;
pub const MSGMODE_OCARINA_FAIL_NO_TEXT: C2RustUnnamed_23 = 15;
pub const MSGMODE_OCARINA_FAIL: C2RustUnnamed_23 = 14;
pub const MSGMODE_OCARINA_CORRECT_PLAYBACK: C2RustUnnamed_23 = 13;
pub const MSGMODE_OCARINA_PLAYING: C2RustUnnamed_23 = 12;
pub const MSGMODE_SONG_PLAYBACK_STARTING: C2RustUnnamed_23 = 11;
pub const MSGMODE_SONG_DEMONSTRATION_STARTING: C2RustUnnamed_23 = 10;
pub const MSGMODE_OCARINA_STARTING: C2RustUnnamed_23 = 9;
pub const MSGMODE_TEXT_DELAYED_BREAK: C2RustUnnamed_23 = 8;
pub const MSGMODE_TEXT_AWAIT_INPUT: C2RustUnnamed_23 = 7;
pub const MSGMODE_TEXT_DISPLAYING: C2RustUnnamed_23 = 6;
pub const MSGMODE_TEXT_CONTINUING: C2RustUnnamed_23 = 5;
pub const MSGMODE_TEXT_NEXT_MSG: C2RustUnnamed_23 = 4;
pub const MSGMODE_TEXT_STARTING: C2RustUnnamed_23 = 3;
pub const MSGMODE_TEXT_BOX_GROWING: C2RustUnnamed_23 = 2;
pub const MSGMODE_TEXT_START: C2RustUnnamed_23 = 1;
pub const MSGMODE_NONE: C2RustUnnamed_23 = 0;
pub type C2RustUnnamed_24 = libc::c_uint;
pub const GAMEOVER_REVIVE_FADE_OUT: C2RustUnnamed_24 = 24;
pub const GAMEOVER_REVIVE_WAIT_FAIRY: C2RustUnnamed_24 = 23;
pub const GAMEOVER_REVIVE_WAIT_GROUND: C2RustUnnamed_24 = 22;
pub const GAMEOVER_REVIVE_RUMBLE: C2RustUnnamed_24 = 21;
pub const GAMEOVER_REVIVE_START: C2RustUnnamed_24 = 20;
pub const GAMEOVER_DEATH_MENU: C2RustUnnamed_24 = 4;
pub const GAMEOVER_DEATH_DELAY_MENU: C2RustUnnamed_24 = 3;
pub const GAMEOVER_DEATH_WAIT_GROUND: C2RustUnnamed_24 = 2;
pub const GAMEOVER_DEATH_START: C2RustUnnamed_24 = 1;
pub const GAMEOVER_INACTIVE: C2RustUnnamed_24 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OpeningContext {
    pub state: GameState,
    pub view: View,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FileChooseContext {
    pub state: GameState,
    pub windowVtx: *mut Vtx,
    pub staticSegment: *mut u8_0,
    pub parameterSegment: *mut u8_0,
    pub unk_B0: [libc::c_char; 8],
    pub view: View,
    pub sramCtx: SramContext,
    pub unk_1E4: [libc::c_char; 4],
    pub skyboxCtx: SkyboxContext,
    pub msgCtx: MessageContext,
    pub font: Font,
    pub envCtx: EnvironmentContext,
    pub unk_1C9E4: [libc::c_char; 4],
    pub windowContentVtx: *mut Vtx,
    pub keyboardVtx: *mut Vtx,
    pub nameEntryVtx: *mut Vtx,
    pub n64ddFlag: u8_0,
    pub deaths: [u16_0; 3],
    pub fileNames: [[u8_0; 8]; 3],
    pub healthCapacities: [u16_0; 3],
    pub questItems: [u32_0; 3],
    pub n64ddFlags: [s16; 3],
    pub defense: [s8; 3],
    pub health: [u16_0; 3],
    pub buttonIndex: s16,
    pub confirmButtonIndex: s16,
    pub menuMode: s16,
    pub configMode: s16,
    pub prevConfigMode: s16,
    pub nextConfigMode: s16,
    pub selectMode: s16,
    pub selectedFileIndex: s16,
    pub unk_1CA48: [libc::c_char; 2],
    pub fileNamesY: [s16; 3],
    pub actionTimer: s16,
    pub buttonYOffsets: [s16; 6],
    pub copyDestFileIndex: s16,
    pub warningLabel: s16,
    pub warningButtonIndex: s16,
    pub titleLabel: s16,
    pub nextTitleLabel: s16,
    pub windowColor: [s16; 3],
    pub titleAlpha: [s16; 2],
    pub windowAlpha: s16,
    pub fileButtonAlpha: [s16; 3],
    pub nameBoxAlpha: [s16; 3],
    pub nameAlpha: [s16; 3],
    pub connectorAlpha: [s16; 3],
    pub fileInfoAlpha: [s16; 3],
    pub actionButtonAlpha: [s16; 2],
    pub confirmButtonAlpha: [s16; 2],
    pub optionButtonAlpha: s16,
    pub nameEntryBoxAlpha: s16,
    pub controlsAlpha: s16,
    pub emptyFileTextAlpha: s16,
    pub highlightColor: [s16; 4],
    pub highlightPulseDir: s16,
    pub unk_1CAAC: s16,
    pub confirmButtonTexIndices: [s16; 2],
    pub inputTimerX: s16,
    pub inputTimerY: s16,
    pub stickXDir: s16,
    pub stickYDir: s16,
    pub stickRelX: s16,
    pub stickRelY: s16,
    pub nameEntryBoxPosX: s16,
    pub windowPosX: s16,
    pub windowRot: f32_0,
    pub kbdButton: s16,
    pub charPage: s16,
    pub charBgAlpha: s16,
    pub charIndex: s16,
    pub kbdX: s16,
    pub kbdY: s16,
    pub newFileNameCharCount: s16,
    pub unk_1CAD6: [s16; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EntranceInfo {
    pub scene: s8,
    pub spawn: s8,
    pub field: u16_0,
}
pub type C2RustUnnamed_25 = libc::c_uint;
pub const MTXMODE_APPLY: C2RustUnnamed_25 = 1;
pub const MTXMODE_NEW: C2RustUnnamed_25 = 0;
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
pub struct VisMono {
    pub unk_00: u32_0,
    pub setScissor: u32_0,
    pub primColor: Color_RGBA8_u32,
    pub envColor: Color_RGBA8_u32,
    pub tlut: *mut u16_0,
    pub monoDl: *mut Gfx,
}
#[no_mangle]
pub static mut D_8012D1F0: *mut libc::c_void =
    0 as *const libc::c_void as *mut libc::c_void;
#[no_mangle]
pub static mut D_8012D1F4: s32 = 0 as libc::c_int;
// unused
#[no_mangle]
pub static mut D_8012D1F8: *mut Input = 0 as *const Input as *mut Input;
#[no_mangle]
pub static mut sTrnsnUnk: TransitionUnk =
    TransitionUnk{row: 0,
                  col: 0,
                  frame: 0,
                  unk_0C:
                      0 as *const TransitionUnkData as *mut TransitionUnkData,
                  vtxFrame1: 0 as *const Vtx as *mut Vtx,
                  vtxFrame2: 0 as *const Vtx as *mut Vtx,
                  projection: Mtx{m: [[0; 4]; 4],},
                  modelView: Mtx{m: [[0; 4]; 4],},
                  unk_98: Mtx{m: [[0; 4]; 4],},
                  gfx: 0 as *const Gfx as *mut Gfx,
                  zBuffer: 0 as *const u16_0 as *mut u16_0,};
#[no_mangle]
pub static mut gTrnsnUnkState: s32 = 0;
#[no_mangle]
pub static mut D_80161498: VisMono =
    VisMono{unk_00: 0,
            setScissor: 0,
            primColor:
                Color_RGBA8_u32{c2rust_unnamed:
                                    C2RustUnnamed_3{r: 0,
                                                    g: 0,
                                                    b: 0,
                                                    a: 0,},},
            envColor:
                Color_RGBA8_u32{c2rust_unnamed:
                                    C2RustUnnamed_3{r: 0,
                                                    g: 0,
                                                    b: 0,
                                                    a: 0,},},
            tlut: 0 as *const u16_0 as *mut u16_0,
            monoDl: 0 as *const Gfx as *mut Gfx,};
#[no_mangle]
pub static mut D_801614B0: Color_RGBA8_u32 =
    Color_RGBA8_u32{c2rust_unnamed:
                        C2RustUnnamed_3{r: 0, g: 0, b: 0, a: 0,},};
#[no_mangle]
pub static mut D_801614B8: FaultClient =
    FaultClient{next: 0 as *const FaultClient as *mut FaultClient,
                callback: 0,
                param1: 0,
                param2: 0,};
#[no_mangle]
pub static mut D_801614C8: s16 = 0;
#[no_mangle]
pub static mut D_801614D0: [u64_0; 2560] = [0; 2560];
#[no_mangle]
pub unsafe extern "C" fn func_800BC450(mut globalCtx: *mut GlobalContext) {
    Camera_ChangeDataIdx((*globalCtx).cameraPtrs[(*globalCtx).activeCamera as
                                                     usize],
                         (*globalCtx).unk_1242B as libc::c_int -
                             1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn func_800BC490(mut globalCtx: *mut GlobalContext,
                                       mut point: s16) {
    if point as libc::c_int == 1 as libc::c_int ||
           point as libc::c_int == 2 as libc::c_int {
    } else {
        __assert(b"point == 1 || point == 2\x00" as *const u8 as
                     *const libc::c_char,
                 b"../z_play.c\x00" as *const u8 as *const libc::c_char,
                 2160 as libc::c_int);
    };
    (*globalCtx).unk_1242B = point as u8_0;
    if (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 15 as libc::c_int) as usize]
           as libc::c_int != 0x10 as libc::c_int &&
           gSaveContext.cutsceneIndex < 0xfff0 as libc::c_int {
        Audio_PlaySoundGeneral(if point as libc::c_int == 1 as libc::c_int {
                                   0x4814 as libc::c_int
                               } else { 0x4813 as libc::c_int } as u16_0,
                               &mut D_801333D4, 4 as libc::c_int as u8_0,
                               &mut D_801333E0, &mut D_801333E0,
                               &mut D_801333E8);
    }
    func_800BC450(globalCtx);
}
#[no_mangle]
pub unsafe extern "C" fn func_800BC56C(mut globalCtx: *mut GlobalContext,
                                       mut arg1: s16) -> s32 {
    return (arg1 as libc::c_int == (*globalCtx).unk_1242B as libc::c_int) as
               libc::c_int;
}
// original name: "Game_play_shop_pr_vr_switch_set"
#[no_mangle]
pub unsafe extern "C" fn func_800BC590(mut globalCtx: *mut GlobalContext) {
    osSyncPrintf(b"Game_play_shop_pr_vr_switch_set()\n\x00" as *const u8 as
                     *const libc::c_char);
    if (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 15 as libc::c_int) as usize]
           as libc::c_int == 0x10 as libc::c_int {
        (*globalCtx).unk_1242B = 2 as libc::c_int as u8_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800BC5E0(mut globalCtx: *mut GlobalContext,
                                       mut transitionType: s32) {
    let mut transitionCtx: *mut TransitionContext =
        &mut (*globalCtx).transitionCtx;
    bzero(transitionCtx as *mut libc::c_void,
          ::std::mem::size_of::<TransitionContext>() as libc::c_ulong);
    (*transitionCtx).transitionType = transitionType;
    if (*transitionCtx).transitionType >> 5 as libc::c_int == 1 as libc::c_int
       {
        (*transitionCtx).init =
            Some(TransitionCircle_Init as
                     unsafe extern "C" fn(_: *mut libc::c_void)
                         -> *mut libc::c_void);
        (*transitionCtx).destroy =
            Some(TransitionCircle_Destroy as
                     unsafe extern "C" fn(_: *mut libc::c_void) -> ());
        (*transitionCtx).start =
            Some(TransitionCircle_Start as
                     unsafe extern "C" fn(_: *mut libc::c_void) -> ());
        (*transitionCtx).isDone =
            Some(TransitionCircle_IsDone as
                     unsafe extern "C" fn(_: *mut libc::c_void) -> s32);
        (*transitionCtx).draw =
            Some(TransitionCircle_Draw as
                     unsafe extern "C" fn(_: *mut libc::c_void,
                                          _: *mut *mut Gfx) -> ());
        (*transitionCtx).update =
            Some(TransitionCircle_Update as
                     unsafe extern "C" fn(_: *mut libc::c_void, _: s32)
                         -> ());
        (*transitionCtx).setType =
            Some(TransitionCircle_SetType as
                     unsafe extern "C" fn(_: *mut libc::c_void, _: s32)
                         -> ());
        (*transitionCtx).setColor =
            Some(TransitionCircle_SetColor as
                     unsafe extern "C" fn(_: *mut libc::c_void, _: u32_0)
                         -> ());
        (*transitionCtx).setEnvColor =
            Some(TransitionCircle_SetEnvColor as
                     unsafe extern "C" fn(_: *mut libc::c_void, _: u32_0)
                         -> ())
    } else {
        match (*transitionCtx).transitionType {
            1 => {
                (*transitionCtx).init =
                    Some(TransitionTriforce_Init as
                             unsafe extern "C" fn(_: *mut libc::c_void)
                                 -> *mut libc::c_void);
                (*transitionCtx).destroy =
                    Some(TransitionTriforce_Destroy as
                             unsafe extern "C" fn(_: *mut libc::c_void)
                                 -> ());
                (*transitionCtx).start =
                    Some(TransitionTriforce_Start as
                             unsafe extern "C" fn(_: *mut libc::c_void)
                                 -> ());
                (*transitionCtx).isDone =
                    Some(TransitionTriforce_IsDone as
                             unsafe extern "C" fn(_: *mut libc::c_void)
                                 -> s32);
                (*transitionCtx).draw =
                    Some(TransitionTriforce_Draw as
                             unsafe extern "C" fn(_: *mut libc::c_void,
                                                  _: *mut *mut Gfx) -> ());
                (*transitionCtx).update =
                    Some(TransitionTriforce_Update as
                             unsafe extern "C" fn(_: *mut libc::c_void,
                                                  _: s32) -> ());
                (*transitionCtx).setType =
                    Some(TransitionTriforce_SetType as
                             unsafe extern "C" fn(_: *mut libc::c_void,
                                                  _: s32) -> ());
                (*transitionCtx).setColor =
                    Some(TransitionTriforce_SetColor as
                             unsafe extern "C" fn(_: *mut libc::c_void,
                                                  _: u32_0) -> ());
                (*transitionCtx).setEnvColor = None
            }
            0 | 8 => {
                (*transitionCtx).init =
                    Some(TransitionWipe_Init as
                             unsafe extern "C" fn(_: *mut libc::c_void)
                                 -> *mut libc::c_void);
                (*transitionCtx).destroy =
                    Some(TransitionWipe_Destroy as
                             unsafe extern "C" fn(_: *mut libc::c_void)
                                 -> ());
                (*transitionCtx).start =
                    Some(TransitionWipe_Start as
                             unsafe extern "C" fn(_: *mut libc::c_void)
                                 -> ());
                (*transitionCtx).isDone =
                    Some(TransitionWipe_IsDone as
                             unsafe extern "C" fn(_: *mut libc::c_void)
                                 -> s32);
                (*transitionCtx).draw =
                    Some(TransitionWipe_Draw as
                             unsafe extern "C" fn(_: *mut libc::c_void,
                                                  _: *mut *mut Gfx) -> ());
                (*transitionCtx).update =
                    Some(TransitionWipe_Update as
                             unsafe extern "C" fn(_: *mut libc::c_void,
                                                  _: s32) -> ());
                (*transitionCtx).setType =
                    Some(TransitionWipe_SetType as
                             unsafe extern "C" fn(_: *mut libc::c_void,
                                                  _: s32) -> ());
                (*transitionCtx).setColor =
                    Some(TransitionWipe_SetColor as
                             unsafe extern "C" fn(_: *mut libc::c_void,
                                                  _: u32_0) -> ());
                (*transitionCtx).setEnvColor = None
            }
            2 | 3 | 4 | 5 | 6 | 7 | 13 | 17 | 18 | 19 => {
                (*transitionCtx).init =
                    Some(TransitionFade_Init as
                             unsafe extern "C" fn(_: *mut libc::c_void)
                                 -> *mut libc::c_void);
                (*transitionCtx).destroy =
                    Some(TransitionFade_Destroy as
                             unsafe extern "C" fn(_: *mut libc::c_void)
                                 -> ());
                (*transitionCtx).start =
                    Some(TransitionFade_Start as
                             unsafe extern "C" fn(_: *mut libc::c_void)
                                 -> ());
                (*transitionCtx).isDone =
                    Some(TransitionFade_IsDone as
                             unsafe extern "C" fn(_: *mut libc::c_void)
                                 -> s32);
                (*transitionCtx).draw =
                    Some(TransitionFade_Draw as
                             unsafe extern "C" fn(_: *mut libc::c_void,
                                                  _: *mut *mut Gfx) -> ());
                (*transitionCtx).update =
                    Some(TransitionFade_Update as
                             unsafe extern "C" fn(_: *mut libc::c_void,
                                                  _: s32) -> ());
                (*transitionCtx).setType =
                    Some(TransitionFade_SetType as
                             unsafe extern "C" fn(_: *mut libc::c_void,
                                                  _: s32) -> ());
                (*transitionCtx).setColor =
                    Some(TransitionFade_SetColor as
                             unsafe extern "C" fn(_: *mut libc::c_void,
                                                  _: u32_0) -> ());
                (*transitionCtx).setEnvColor = None
            }
            9 | 10 => {
                (*globalCtx).transitionMode = 4 as libc::c_int as u8_0
            }
            11 => { (*globalCtx).transitionMode = 10 as libc::c_int as u8_0 }
            12 => { (*globalCtx).transitionMode = 7 as libc::c_int as u8_0 }
            14 => { (*globalCtx).transitionMode = 12 as libc::c_int as u8_0 }
            15 => { (*globalCtx).transitionMode = 14 as libc::c_int as u8_0 }
            16 => { (*globalCtx).transitionMode = 16 as libc::c_int as u8_0 }
            _ => {
                Fault_AddHungupAndCrash(b"../z_play.c\x00" as *const u8 as
                                            *const libc::c_char,
                                        2290 as libc::c_int as u32_0);
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800BC88C(mut globalCtx: *mut GlobalContext) {
    (*globalCtx).transitionCtx.transitionType = -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Gameplay_SetFog(mut globalCtx: *mut GlobalContext,
                                         mut gfx: *mut Gfx) -> *mut Gfx {
    return Gfx_SetFog2(gfx,
                       (*globalCtx).lightCtx.fogColor[0 as libc::c_int as
                                                          usize] as s32,
                       (*globalCtx).lightCtx.fogColor[1 as libc::c_int as
                                                          usize] as s32,
                       (*globalCtx).lightCtx.fogColor[2 as libc::c_int as
                                                          usize] as s32,
                       0 as libc::c_int, (*globalCtx).lightCtx.fogNear as s32,
                       1000 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Gameplay_Destroy(mut thisx: *mut GameState) {
    let mut globalCtx: *mut GlobalContext = thisx as *mut GlobalContext;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    (*(*globalCtx).state.gfxCtx).callback = None;
    (*(*globalCtx).state.gfxCtx).callbackParam = 0 as *mut libc::c_void;
    (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 91 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 94 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    PreRender_Destroy(&mut (*globalCtx).pauseBgPreRender);
    Effect_DeleteAll(globalCtx);
    EffectSs_ClearAll(globalCtx);
    CollisionCheck_DestroyContext(globalCtx, &mut (*globalCtx).colChkCtx);
    if gTrnsnUnkState == 3 as libc::c_int {
        TransitionUnk_Destroy(&mut sTrnsnUnk);
        gTrnsnUnkState = 0 as libc::c_int
    }
    if (*globalCtx).transitionMode as libc::c_int == 3 as libc::c_int {
        (*globalCtx).transitionCtx.destroy.expect("non-null function pointer")(&mut (*globalCtx).transitionCtx.c2rust_unnamed.data
                                                                                   as
                                                                                   *mut [libc::c_char; 552]
                                                                                   as
                                                                                   *mut libc::c_void);
        func_800BC88C(globalCtx);
        (*globalCtx).transitionMode = 0 as libc::c_int as u8_0
    }
    ShrinkWindow_Destroy();
    TransitionFade_Destroy(&mut (*globalCtx).transitionFade as
                               *mut TransitionFade as *mut libc::c_void);
    VisMono_Destroy(&mut D_80161498);
    if gSaveContext.linkAge != (*globalCtx).linkAgeOnLoad as libc::c_int {
        Inventory_SwapAgeEquipment();
        Player_SetEquipmentData(globalCtx, player);
    }
    func_80031C3C(&mut (*globalCtx).actorCtx, globalCtx);
    func_80110990(globalCtx);
    KaleidoScopeCall_Destroy(globalCtx);
    KaleidoManager_Destroy();
    ZeldaArena_Cleanup();
    Fault_RemoveClient(&mut D_801614B8);
}
#[no_mangle]
pub unsafe extern "C" fn Gameplay_Init(mut thisx: *mut GameState) {
    let mut globalCtx: *mut GlobalContext = thisx as *mut GlobalContext;
    let mut gfxCtx: *mut GraphicsContext = (*globalCtx).state.gfxCtx;
    let mut zAlloc: u32_0 = 0;
    let mut zAllocAligned: u32_0 = 0;
    let mut zAllocSize: size_t = 0;
    let mut player: *mut Player = 0 as *mut Player;
    let mut playerStartCamId: s32 = 0;
    let mut i: s32 = 0;
    let mut tempSetupIndex: u8_0 = 0;
    let mut pad: [s32; 2] = [0; 2];
    if gSaveContext.entranceIndex == -(1 as libc::c_int) {
        gSaveContext.entranceIndex = 0 as libc::c_int;
        (*globalCtx).state.running = 0 as libc::c_int as u32_0;
        (*globalCtx).state.init =
            Some(Opening_Init as
                     unsafe extern "C" fn(_: *mut GameState) -> ());
        (*globalCtx).state.size =
            ::std::mem::size_of::<OpeningContext>() as libc::c_ulong;
        return
    }
    SystemArena_Display();
    GameState_Realloc(&mut (*globalCtx).state,
                      0x1d4790 as libc::c_int as size_t);
    KaleidoManager_Init(globalCtx);
    View_Init(&mut (*globalCtx).view, gfxCtx);
    Audio_SetExtraFilter(0 as libc::c_int as u8_0);
    Quake_Init();
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        (*globalCtx).cameraPtrs[i as usize] = 0 as *mut Camera;
        i += 1
    }
    Camera_Init(&mut (*globalCtx).mainCamera, &mut (*globalCtx).view,
                &mut (*globalCtx).colCtx, globalCtx);
    Camera_ChangeStatus(&mut (*globalCtx).mainCamera,
                        7 as libc::c_int as s16);
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        Camera_Init(&mut *(*globalCtx).subCameras.as_mut_ptr().offset(i as
                                                                          isize),
                    &mut (*globalCtx).view, &mut (*globalCtx).colCtx,
                    globalCtx);
        Camera_ChangeStatus(&mut *(*globalCtx).subCameras.as_mut_ptr().offset(i
                                                                                  as
                                                                                  isize),
                            0x100 as libc::c_int as s16);
        i += 1
    }
    (*globalCtx).cameraPtrs[0 as libc::c_int as usize] =
        &mut (*globalCtx).mainCamera;
    (*(*globalCtx).cameraPtrs[0 as libc::c_int as usize]).uid =
        0 as libc::c_int as s16;
    (*globalCtx).activeCamera = 0 as libc::c_int as s16;
    func_8005AC48(&mut (*globalCtx).mainCamera, 0xff as libc::c_int as s16);
    Sram_Init(globalCtx, &mut (*globalCtx).sramCtx);
    func_80112098(globalCtx);
    Message_Init(globalCtx);
    GameOver_Init(globalCtx);
    func_8006BA00(globalCtx);
    Effect_InitContext(globalCtx);
    EffectSs_InitInfo(globalCtx, 0x55 as libc::c_int);
    CollisionCheck_InitContext(globalCtx, &mut (*globalCtx).colChkCtx);
    AnimationContext_Reset(&mut (*globalCtx).animationCtx);
    func_8006450C(globalCtx, &mut (*globalCtx).csCtx);
    if gSaveContext.nextCutsceneIndex as libc::c_int != 0xffef as libc::c_int
       {
        gSaveContext.cutsceneIndex = gSaveContext.nextCutsceneIndex as s32;
        gSaveContext.nextCutsceneIndex = 0xffef as libc::c_int as u16_0
    }
    if gSaveContext.cutsceneIndex == 0xfffd as libc::c_int {
        gSaveContext.cutsceneIndex = 0 as libc::c_int
    }
    if gSaveContext.nextDayTime as libc::c_int != 0xffff as libc::c_int {
        gSaveContext.dayTime = gSaveContext.nextDayTime;
        gSaveContext.skyboxTime = gSaveContext.nextDayTime
    }
    if gSaveContext.dayTime as libc::c_int > 0xc000 as libc::c_int ||
           (gSaveContext.dayTime as libc::c_int) < 0x4555 as libc::c_int {
        gSaveContext.nightFlag = 1 as libc::c_int
    } else { gSaveContext.nightFlag = 0 as libc::c_int }
    Cutscene_HandleConditionalTriggers(globalCtx);
    if gSaveContext.gameMode != 0 as libc::c_int ||
           gSaveContext.cutsceneIndex >= 0xfff0 as libc::c_int {
        gSaveContext.nayrusLoveTimer = 0 as libc::c_int as s16;
        func_800876C8(globalCtx);
        gSaveContext.sceneSetupIndex =
            (gSaveContext.cutsceneIndex & 0xf as libc::c_int) +
                4 as libc::c_int
    } else if !(gSaveContext.linkAge == 0 as libc::c_int) &&
                  gSaveContext.nightFlag == 0 as libc::c_int {
        gSaveContext.sceneSetupIndex = 0 as libc::c_int
    } else if !(gSaveContext.linkAge == 0 as libc::c_int) &&
                  !(gSaveContext.nightFlag == 0 as libc::c_int) {
        gSaveContext.sceneSetupIndex = 1 as libc::c_int
    } else if gSaveContext.linkAge == 0 as libc::c_int &&
                  gSaveContext.nightFlag == 0 as libc::c_int {
        gSaveContext.sceneSetupIndex = 2 as libc::c_int
    } else { gSaveContext.sceneSetupIndex = 3 as libc::c_int }
    tempSetupIndex = gSaveContext.sceneSetupIndex as u8_0;
    if gEntranceTable[gSaveContext.entranceIndex as usize].scene as
           libc::c_int == SCENE_SPOT00 as libc::c_int &&
           !(gSaveContext.linkAge == 0 as libc::c_int) &&
           gSaveContext.sceneSetupIndex < 4 as libc::c_int {
        if gBitFlags[QUEST_KOKIRI_EMERALD as libc::c_int as usize] &
               gSaveContext.inventory.questItems != 0 &&
               gBitFlags[QUEST_GORON_RUBY as libc::c_int as usize] &
                   gSaveContext.inventory.questItems != 0 &&
               gBitFlags[QUEST_ZORA_SAPPHIRE as libc::c_int as usize] &
                   gSaveContext.inventory.questItems != 0 {
            gSaveContext.sceneSetupIndex = 1 as libc::c_int
        } else { gSaveContext.sceneSetupIndex = 0 as libc::c_int }
    } else if gEntranceTable[gSaveContext.entranceIndex as usize].scene as
                  libc::c_int == SCENE_SPOT04 as libc::c_int &&
                  gSaveContext.linkAge == 0 as libc::c_int &&
                  gSaveContext.sceneSetupIndex < 4 as libc::c_int {
        gSaveContext.sceneSetupIndex =
            if gSaveContext.eventChkInf[4 as libc::c_int as usize] as
                   libc::c_int & 0x100 as libc::c_int != 0 {
                3 as libc::c_int
            } else { 2 as libc::c_int }
    }
    Gameplay_SpawnScene(globalCtx,
                        gEntranceTable[(gSaveContext.entranceIndex +
                                            gSaveContext.sceneSetupIndex) as
                                           usize].scene as s32,
                        gEntranceTable[(gSaveContext.sceneSetupIndex +
                                            gSaveContext.entranceIndex) as
                                           usize].spawn as s32);
    osSyncPrintf(b"\nSCENE_NO=%d COUNTER=%d\n\x00" as *const u8 as
                     *const libc::c_char, gSaveContext.entranceIndex,
                 gSaveContext.sceneSetupIndex);
    // When entering Gerudo Valley in the right setup, trigger the GC emulator to play the ending movie.
    // The emulator constantly checks whether PC is 0x81000000, so this works even though it's not a valid address.
    if gEntranceTable[gSaveContext.entranceIndex as usize].scene as
           libc::c_int == SCENE_SPOT09 as libc::c_int &&
           gSaveContext.sceneSetupIndex == 6 as libc::c_int {
        osSyncPrintf(b"\xe3\x82\xa8\xe3\x83\xb3\xe3\x83\x87\xe3\x82\xa3\xe3\x83\xb3\xe3\x82\xb0\xe3\x81\xaf\xe3\x81\x98\xe3\x81\xbe\xe3\x82\x8b\xe3\x82\x88\xe3\x83\xbc\n\x00"
                         as *const u8 as *const libc::c_char);
        ::std::mem::transmute::<_,
                                fn()>(::std::mem::transmute::<libc::intptr_t,
                                                              Option<unsafe extern "C" fn()
                                                                         ->
                                                                             ()>>(0x81000000
                                                                                      as
                                                                                      libc::c_uint
                                                                                      as
                                                                                      libc::intptr_t).expect("non-null function pointer"))();
        osSyncPrintf(b"\xe5\x87\xba\xe6\x88\xbb\xe3\x82\x8a\xef\xbc\x9f\n\x00"
                         as *const u8 as
                         *const libc::c_char); // "The ending starts"
        // "Return?"
    }
    Cutscene_HandleEntranceTriggers(globalCtx);
    KaleidoScopeCall_Init(globalCtx);
    func_801109B0(globalCtx);
    if gSaveContext.nextDayTime as libc::c_int != 0xffff as libc::c_int {
        if gSaveContext.nextDayTime as libc::c_int == 0x8001 as libc::c_int {
            gSaveContext.totalDays += 1;
            gSaveContext.bgsDayCount += 1;
            gSaveContext.dogIsLost = 1 as libc::c_int as u8_0;
            if Inventory_ReplaceItem(globalCtx,
                                     ITEM_WEIRD_EGG as libc::c_int as u16_0,
                                     ITEM_CHICKEN as libc::c_int as u16_0) !=
                   0 ||
                   Inventory_ReplaceItem(globalCtx,
                                         ITEM_POCKET_EGG as libc::c_int as
                                             u16_0,
                                         ITEM_POCKET_CUCCO as libc::c_int as
                                             u16_0) != 0 {
                Message_StartTextbox(globalCtx,
                                     0x3066 as libc::c_int as u16_0,
                                     0 as *mut Actor);
            }
            gSaveContext.nextDayTime = 0xfffe as libc::c_int as u16_0
        } else { gSaveContext.nextDayTime = 0xfffd as libc::c_int as u16_0 }
    }
    (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 91 as libc::c_int) as usize] =
        -(1 as libc::c_int) as s16;
    (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 94 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    PreRender_Init(&mut (*globalCtx).pauseBgPreRender);
    PreRender_SetValuesSave(&mut (*globalCtx).pauseBgPreRender,
                            320 as libc::c_int as u32_0,
                            240 as libc::c_int as u32_0,
                            0 as *mut libc::c_void, 0 as *mut libc::c_void,
                            0 as *mut libc::c_void);
    PreRender_SetValues(&mut (*globalCtx).pauseBgPreRender,
                        320 as libc::c_int as u32_0,
                        240 as libc::c_int as u32_0, 0 as *mut libc::c_void,
                        0 as *mut libc::c_void);
    gTrnsnUnkState = 0 as libc::c_int;
    (*globalCtx).transitionMode = 0 as libc::c_int as u8_0;
    FrameAdvance_Init(&mut (*globalCtx).frameAdvCtx);
    Rand_Seed(osGetTime() as u32_0);
    Matrix_Init(&mut (*globalCtx).state);
    (*globalCtx).state.main =
        Some(Gameplay_Main as unsafe extern "C" fn(_: *mut GameState) -> ());
    (*globalCtx).state.destroy =
        Some(Gameplay_Destroy as
                 unsafe extern "C" fn(_: *mut GameState) -> ());
    (*globalCtx).sceneLoadFlag = -(0x14 as libc::c_int) as s8;
    (*globalCtx).unk_11E16 = 0xff as libc::c_int as s16;
    (*globalCtx).unk_11E18 = 0 as libc::c_int as s16;
    (*globalCtx).unk_11DE9 = 0 as libc::c_int as u8_0;
    if gSaveContext.gameMode != 1 as libc::c_int {
        if gSaveContext.nextTransition as libc::c_int == 0xff as libc::c_int {
            (*globalCtx).fadeTransition =
                (gEntranceTable[(gSaveContext.entranceIndex +
                                     tempSetupIndex as libc::c_int) as
                                    usize].field as libc::c_int >>
                     7 as libc::c_int & 0x7f as libc::c_int) as u8_0
            // Fade In
        } else {
            (*globalCtx).fadeTransition = gSaveContext.nextTransition;
            gSaveContext.nextTransition = 0xff as libc::c_int as u8_0
        }
    } else { (*globalCtx).fadeTransition = 6 as libc::c_int as u8_0 }
    ShrinkWindow_Init();
    TransitionFade_Init(&mut (*globalCtx).transitionFade as
                            *mut TransitionFade as *mut libc::c_void);
    TransitionFade_SetType(&mut (*globalCtx).transitionFade as
                               *mut TransitionFade as *mut libc::c_void,
                           3 as libc::c_int);
    TransitionFade_SetColor(&mut (*globalCtx).transitionFade as
                                *mut TransitionFade as *mut libc::c_void,
                            ((160 as libc::c_int & 0xff as libc::c_int) <<
                                 24 as libc::c_int |
                                 (160 as libc::c_int & 0xff as libc::c_int) <<
                                     16 as libc::c_int |
                                 (160 as libc::c_int & 0xff as libc::c_int) <<
                                     8 as libc::c_int |
                                 (255 as libc::c_int & 0xff as libc::c_int) <<
                                     0 as libc::c_int) as u32_0);
    TransitionFade_Start(&mut (*globalCtx).transitionFade as
                             *mut TransitionFade as *mut libc::c_void);
    VisMono_Init(&mut D_80161498);
    D_801614B0.c2rust_unnamed.a = 0 as libc::c_int as u8_0;
    Flags_UnsetAllEnv(globalCtx);
    osSyncPrintf(b"ZELDA ALLOC SIZE=%x\n\x00" as *const u8 as
                     *const libc::c_char,
                 THA_GetSize(&mut (*globalCtx).state.tha));
    zAllocSize = THA_GetSize(&mut (*globalCtx).state.tha) as size_t;
    zAlloc =
        GameState_Alloc(&mut (*globalCtx).state, zAllocSize,
                        b"../z_play.c\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char, 2918 as libc::c_int) as
            u32_0;
    zAllocAligned =
        zAlloc.wrapping_add(8 as libc::c_int as libc::c_uint) &
            !(0xf as libc::c_int) as libc::c_uint;
    ZeldaArena_Init(zAllocAligned as *mut libc::c_void,
                    zAllocSize.wrapping_sub(zAllocAligned as
                                                libc::c_ulong).wrapping_add(zAlloc
                                                                                as
                                                                                libc::c_ulong)
                        as u32_0);
    // "Zelda Heap"
    osSyncPrintf(b"\xe3\x82\xbc\xe3\x83\xab\xe3\x83\x80\xe3\x83\x92\xe3\x83\xbc\xe3\x83\x97 %08x-%08x\n\x00"
                     as *const u8 as *const libc::c_char, zAllocAligned,
                 (zAllocAligned as libc::c_ulong).wrapping_add(zAllocSize) as
                     s32 - zAllocAligned.wrapping_sub(zAlloc) as s32);
    Fault_AddClient(&mut D_801614B8,
                    ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                       -> ()>,
                                            *mut libc::c_void>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                ->
                                                                                                    (),
                                                                                            unsafe extern "C" fn()
                                                                                                ->
                                                                                                    ()>(ZeldaArena_Display))),
                    0 as *mut libc::c_void, 0 as *mut libc::c_void);
    func_800304DC(globalCtx, &mut (*globalCtx).actorCtx,
                  (*globalCtx).linkActorEntry);
    // Empty Loop
    while func_800973FC(globalCtx, &mut (*globalCtx).roomCtx) == 0 {
    } // "fbdemo_init call failed!"
    player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    Camera_InitPlayerSettings(&mut (*globalCtx).mainCamera, player);
    Camera_ChangeMode(&mut (*globalCtx).mainCamera,
                      CAM_MODE_NORMAL as libc::c_int as s16);
    playerStartCamId =
        (*player).actor.params as libc::c_int & 0xff as libc::c_int;
    if playerStartCamId != 0xff as libc::c_int {
        osSyncPrintf(b"player has start camera ID (\x1b[34m%d\x1b[m)\n\x00" as
                         *const u8 as *const libc::c_char, playerStartCamId);
        Camera_ChangeDataIdx(&mut (*globalCtx).mainCamera, playerStartCamId);
    }
    if (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 15 as libc::c_int) as usize]
           as libc::c_int == 32 as libc::c_int {
        (*globalCtx).unk_1242B = 2 as libc::c_int as u8_0
    } else if (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int *
                                     16 as libc::c_int + 15 as libc::c_int) as
                                    usize] as libc::c_int == 16 as libc::c_int
     {
        (*globalCtx).unk_1242B = 1 as libc::c_int as u8_0
    } else { (*globalCtx).unk_1242B = 0 as libc::c_int as u8_0 }
    Interface_SetSceneRestrictions(globalCtx);
    func_800758AC(globalCtx);
    gSaveContext.seqId = (*globalCtx).sequenceCtx.seqId;
    gSaveContext.natureAmbienceId = (*globalCtx).sequenceCtx.natureAmbienceId;
    func_8002DF18(globalCtx,
                  (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as
                                                       libc::c_int as
                                                       usize].head as
                      *mut Player);
    AnimationContext_Update(globalCtx, &mut (*globalCtx).animationCtx);
    gSaveContext.respawnFlag = 0 as libc::c_int;
    if (*gGameInfo).data[(26 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 95 as libc::c_int) as usize]
           as libc::c_int != 0 as libc::c_int {
        D_8012D1F0 = D_801614D0.as_mut_ptr() as *mut libc::c_void;
        osSyncPrintf(b"\nkawauso_data=[%x]\x00" as *const u8 as
                         *const libc::c_char, D_8012D1F0);
        DmaMgr_DmaRomToRam(0x3feb000 as libc::c_int as u32_0,
                           D_8012D1F0 as u32_0,
                           ::std::mem::size_of::<[u64_0; 2560]>() as
                               libc::c_ulong);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Gameplay_Update(mut globalCtx: *mut GlobalContext) {
    let mut current_block: u64;
    let mut pad1: s32 = 0;
    let mut sp80: s32 = 0;
    let mut input: *mut Input = 0 as *mut Input;
    let mut i: u32_0 = 0;
    let mut pad2: s32 = 0;
    input = (*globalCtx).state.input.as_mut_ptr();
    if ((*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 1 as libc::c_int) as usize]
            as libc::c_int) < 0 as libc::c_int ||
           (*gGameInfo).data[(7 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 0 as libc::c_int) as
                                 usize] as libc::c_int != 0 as libc::c_int {
        (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 1 as libc::c_int) as usize]
            = 0 as libc::c_int as s16;
        ZeldaArena_Display();
    }
    if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 80 as libc::c_int) as usize]
           as libc::c_int == 18 as libc::c_int &&
           ((*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 81 as libc::c_int) as
                                  usize] as libc::c_int) < 0 as libc::c_int {
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 81 as libc::c_int) as
                              usize] = 0 as libc::c_int as s16;
        osSyncPrintf(b"object_exchange_rom_address %u\n\x00" as *const u8 as
                         *const libc::c_char, gObjectTableSize);
        osSyncPrintf(b"RomStart RomEnd   Size\n\x00" as *const u8 as
                         *const libc::c_char);
        i = 0 as libc::c_int as u32_0;
        while i < gObjectTableSize {
            let mut size: s32 =
                gObjectTable[i as
                                 usize].vromEnd.wrapping_sub(gObjectTable[i as
                                                                              usize].vromStart)
                    as s32;
            osSyncPrintf(b"%08x-%08x %08x(%8.3fKB)\n\x00" as *const u8 as
                             *const libc::c_char,
                         gObjectTable[i as usize].vromStart,
                         gObjectTable[i as usize].vromEnd, size,
                         (size as libc::c_float / 1024.0f32) as
                             libc::c_double);
            i = i.wrapping_add(1)
        }
        osSyncPrintf(b"\n\x00" as *const u8 as *const libc::c_char);
    }
    if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 81 as libc::c_int) as usize]
           as libc::c_int == 18 as libc::c_int &&
           ((*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 82 as libc::c_int) as
                                  usize] as libc::c_int) < 0 as libc::c_int {
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 82 as libc::c_int) as
                              usize] = 0 as libc::c_int as s16;
        ActorOverlayTable_LogPrint();
    }
    gSegments[4 as libc::c_int as usize] =
        ((*globalCtx).objectCtx.status[(*globalCtx).objectCtx.mainKeepIndex as
                                           usize].segment as
             *mut u8_0).offset(-(0x80000000 as libc::c_uint as isize)) as
            u32_0;
    gSegments[5 as libc::c_int as usize] =
        ((*globalCtx).objectCtx.status[(*globalCtx).objectCtx.subKeepIndex as
                                           usize].segment as
             *mut u8_0).offset(-(0x80000000 as libc::c_uint as isize)) as
            u32_0;
    gSegments[2 as libc::c_int as usize] =
        ((*globalCtx).sceneSegment as
             *mut u8_0).offset(-(0x80000000 as libc::c_uint as isize)) as
            u32_0;
    if FrameAdvance_Update(&mut (*globalCtx).frameAdvCtx,
                           &mut *input.offset(1 as libc::c_int as isize)) != 0
       {
        if (*globalCtx).transitionMode as libc::c_int == 0 as libc::c_int &&
               (*globalCtx).sceneLoadFlag as libc::c_int != 0 as libc::c_int {
            (*globalCtx).transitionMode = 1 as libc::c_int as u8_0
        }
        if gTrnsnUnkState != 0 as libc::c_int {
            match gTrnsnUnkState {
                2 => {
                    if TransitionUnk_Init(&mut sTrnsnUnk, 10 as libc::c_int,
                                          7 as libc::c_int).is_null() {
                        osSyncPrintf(b"fbdemo_init\xe5\x91\xbc\xe5\x87\xba\xe3\x81\x97\xe5\xa4\xb1\xe6\x95\x97\xef\xbc\x81\n\x00"
                                         as *const u8 as *const libc::c_char);
                        gTrnsnUnkState = 0 as libc::c_int
                    } else {
                        sTrnsnUnk.zBuffer =
                            gZBuffer.as_mut_ptr() as *mut u16_0;
                        gTrnsnUnkState = 3 as libc::c_int;
                        (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int
                                               * 16 as libc::c_int +
                                               30 as libc::c_int) as usize] =
                            1 as libc::c_int as s16
                    }
                }
                3 => { func_800B23E8(&mut sTrnsnUnk); }
                _ => { }
            }
        }
        if (*globalCtx).transitionMode != 0 {
            let mut current_block_147: u64;
            match (*globalCtx).transitionMode as libc::c_int {
                1 => {
                    if (*globalCtx).sceneLoadFlag as libc::c_int !=
                           -(0x14 as libc::c_int) {
                        let mut sp6E: s16 = 0 as libc::c_int as s16;
                        Interface_ChangeAlpha(1 as libc::c_int as u16_0);
                        if gSaveContext.cutsceneIndex >= 0xfff0 as libc::c_int
                           {
                            sp6E =
                                ((gSaveContext.cutsceneIndex &
                                      0xf as libc::c_int) + 4 as libc::c_int)
                                    as s16
                        }
                        if gEntranceTable[((*globalCtx).nextEntranceIndex as
                                               libc::c_int +
                                               sp6E as libc::c_int) as
                                              usize].field as libc::c_int &
                               0x8000 as libc::c_int == 0 {
                            // Continue BGM Off
                            // "Sound initalized. 111"
                            osSyncPrintf(b"\n\n\n\xe3\x82\xb5\xe3\x82\xa6\xe3\x83\xb3\xe3\x83\x89\xe3\x82\xa4\xe3\x83\x8b\xe3\x82\xb7\xe3\x83\xa3\xe3\x83\xab\xe6\x9d\xa5\xe3\x81\xbe\xe3\x81\x97\xe3\x81\x9f\xe3\x80\x82111\x00"
                                             as *const u8 as
                                             *const libc::c_char);
                            if ((*globalCtx).fadeTransition as libc::c_int) <
                                   56 as libc::c_int &&
                                   Environment_IsForcedSequenceDisabled() == 0
                               {
                                // "Sound initalized. 222"
                                osSyncPrintf(b"\n\n\n\xe3\x82\xb5\xe3\x82\xa6\xe3\x83\xb3\xe3\x83\x89\xe3\x82\xa4\xe3\x83\x8b\xe3\x82\xb7\xe3\x83\xa3\xe3\x83\xab\xe6\x9d\xa5\xe3\x81\xbe\xe3\x81\x97\xe3\x81\x9f\xe3\x80\x82222\x00"
                                                 as *const u8 as
                                                 *const libc::c_char);
                                func_800F6964(0x14 as libc::c_int as u16_0);
                                gSaveContext.seqId =
                                    0xffff as libc::c_int as u8_0;
                                gSaveContext.natureAmbienceId =
                                    0xff as libc::c_int as u8_0
                            }
                        }
                    }
                    if (*gGameInfo).data[(11 as libc::c_int * 6 as libc::c_int
                                              * 16 as libc::c_int +
                                              11 as libc::c_int) as usize] as
                           libc::c_int == 0 as libc::c_int {
                        func_800BC5E0(globalCtx,
                                      (*globalCtx).fadeTransition as s32);
                    } else {
                        func_800BC5E0(globalCtx,
                                      (*gGameInfo).data[(11 as libc::c_int *
                                                             6 as libc::c_int
                                                             *
                                                             16 as libc::c_int
                                                             +
                                                             12 as
                                                                 libc::c_int)
                                                            as usize] as s32);
                    }
                    if (*globalCtx).transitionMode as libc::c_int >=
                           4 as libc::c_int {
                        current_block_147 = 13810333397648094191;
                    } else { current_block_147 = 12014234568188996349; }
                }
                2 => { current_block_147 = 12014234568188996349; }
                3 => {
                    if (*globalCtx).transitionCtx.isDone.expect("non-null function pointer")(&mut (*globalCtx).transitionCtx
                                                                                                 as
                                                                                                 *mut TransitionContext
                                                                                                 as
                                                                                                 *mut libc::c_void)
                           != 0 as libc::c_int {
                        if (*globalCtx).transitionCtx.transitionType >=
                               56 as libc::c_int {
                            if (*globalCtx).sceneLoadFlag as libc::c_int ==
                                   -(0x14 as libc::c_int) {
                                (*globalCtx).transitionCtx.destroy.expect("non-null function pointer")(&mut (*globalCtx).transitionCtx
                                                                                                           as
                                                                                                           *mut TransitionContext
                                                                                                           as
                                                                                                           *mut libc::c_void);
                                func_800BC88C(globalCtx);
                                (*globalCtx).transitionMode =
                                    0 as libc::c_int as u8_0
                            }
                        } else if (*globalCtx).sceneLoadFlag as libc::c_int !=
                                      -(0x14 as libc::c_int) {
                            (*globalCtx).state.running =
                                0 as libc::c_int as u32_0;
                            if gSaveContext.gameMode != 2 as libc::c_int {
                                (*globalCtx).state.init =
                                    Some(Gameplay_Init as
                                             unsafe extern "C" fn(_:
                                                                      *mut GameState)
                                                 -> ());
                                (*globalCtx).state.size =
                                    ::std::mem::size_of::<GlobalContext>() as
                                        libc::c_ulong;
                                gSaveContext.entranceIndex =
                                    (*globalCtx).nextEntranceIndex as s32;
                                if gSaveContext.minigameState as libc::c_int
                                       == 1 as libc::c_int {
                                    gSaveContext.minigameState =
                                        3 as libc::c_int as u16_0
                                }
                            } else {
                                (*globalCtx).state.init =
                                    Some(FileChoose_Init as
                                             unsafe extern "C" fn(_:
                                                                      *mut GameState)
                                                 -> ());
                                (*globalCtx).state.size =
                                    ::std::mem::size_of::<FileChooseContext>()
                                        as libc::c_ulong
                            }
                        } else {
                            (*globalCtx).transitionCtx.destroy.expect("non-null function pointer")(&mut (*globalCtx).transitionCtx
                                                                                                       as
                                                                                                       *mut TransitionContext
                                                                                                       as
                                                                                                       *mut libc::c_void);
                            func_800BC88C(globalCtx);
                            (*globalCtx).transitionMode =
                                0 as libc::c_int as u8_0;
                            if gTrnsnUnkState == 3 as libc::c_int {
                                TransitionUnk_Destroy(&mut sTrnsnUnk);
                                gTrnsnUnkState = 0 as libc::c_int;
                                (*gGameInfo).data[(1 as libc::c_int *
                                                       6 as libc::c_int *
                                                       16 as libc::c_int +
                                                       30 as libc::c_int) as
                                                      usize] =
                                    3 as libc::c_int as s16
                            }
                        }
                        (*globalCtx).sceneLoadFlag = 0 as libc::c_int as s8
                    } else {
                        (*globalCtx).transitionCtx.update.expect("non-null function pointer")(&mut (*globalCtx).transitionCtx.c2rust_unnamed.data
                                                                                                  as
                                                                                                  *mut [libc::c_char; 552]
                                                                                                  as
                                                                                                  *mut libc::c_void,
                                                                                              (*gGameInfo).data[(1
                                                                                                                     as
                                                                                                                     libc::c_int
                                                                                                                     *
                                                                                                                     6
                                                                                                                         as
                                                                                                                         libc::c_int
                                                                                                                     *
                                                                                                                     16
                                                                                                                         as
                                                                                                                         libc::c_int
                                                                                                                     +
                                                                                                                     30
                                                                                                                         as
                                                                                                                         libc::c_int)
                                                                                                                    as
                                                                                                                    usize]
                                                                                                  as
                                                                                                  s32);
                    }
                    current_block_147 = 13810333397648094191;
                }
                _ => { current_block_147 = 13810333397648094191; }
            }
            match current_block_147 {
                12014234568188996349 => {
                    (*globalCtx).transitionCtx.init.expect("non-null function pointer")(&mut (*globalCtx).transitionCtx.c2rust_unnamed.data
                                                                                            as
                                                                                            *mut [libc::c_char; 552]
                                                                                            as
                                                                                            *mut libc::c_void);
                    if (*globalCtx).transitionCtx.transitionType >>
                           5 as libc::c_int == 1 as libc::c_int {
                        (*globalCtx).transitionCtx.setType.expect("non-null function pointer")(&mut (*globalCtx).transitionCtx.c2rust_unnamed.data
                                                                                                   as
                                                                                                   *mut [libc::c_char; 552]
                                                                                                   as
                                                                                                   *mut libc::c_void,
                                                                                               (*globalCtx).transitionCtx.transitionType
                                                                                                   |
                                                                                                   0x80
                                                                                                       as
                                                                                                       libc::c_int);
                    }
                    gSaveContext.unk_1419 = 14 as libc::c_int as u8_0;
                    if (*globalCtx).transitionCtx.transitionType ==
                           8 as libc::c_int ||
                           (*globalCtx).transitionCtx.transitionType ==
                               9 as libc::c_int {
                        gSaveContext.unk_1419 = 28 as libc::c_int as u8_0
                    }
                    gSaveContext.fadeDuration = 60 as libc::c_int as u8_0;
                    if (*globalCtx).transitionCtx.transitionType ==
                           4 as libc::c_int ||
                           (*globalCtx).transitionCtx.transitionType ==
                               5 as libc::c_int {
                        gSaveContext.fadeDuration = 20 as libc::c_int as u8_0
                    } else if (*globalCtx).transitionCtx.transitionType ==
                                  6 as libc::c_int ||
                                  (*globalCtx).transitionCtx.transitionType ==
                                      7 as libc::c_int {
                        gSaveContext.fadeDuration = 150 as libc::c_int as u8_0
                    } else if (*globalCtx).transitionCtx.transitionType ==
                                  17 as libc::c_int {
                        gSaveContext.fadeDuration = 2 as libc::c_int as u8_0
                    }
                    if (*globalCtx).transitionCtx.transitionType ==
                           3 as libc::c_int ||
                           (*globalCtx).transitionCtx.transitionType ==
                               5 as libc::c_int ||
                           (*globalCtx).transitionCtx.transitionType ==
                               7 as libc::c_int ||
                           (*globalCtx).transitionCtx.transitionType ==
                               13 as libc::c_int ||
                           (*globalCtx).transitionCtx.transitionType ==
                               17 as libc::c_int {
                        (*globalCtx).transitionCtx.setColor.expect("non-null function pointer")(&mut (*globalCtx).transitionCtx.c2rust_unnamed.data
                                                                                                    as
                                                                                                    *mut [libc::c_char; 552]
                                                                                                    as
                                                                                                    *mut libc::c_void,
                                                                                                ((160
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      &
                                                                                                      0xff
                                                                                                          as
                                                                                                          libc::c_int)
                                                                                                     <<
                                                                                                     24
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                     |
                                                                                                     (160
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          &
                                                                                                          0xff
                                                                                                              as
                                                                                                              libc::c_int)
                                                                                                         <<
                                                                                                         16
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                     |
                                                                                                     (160
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          &
                                                                                                          0xff
                                                                                                              as
                                                                                                              libc::c_int)
                                                                                                         <<
                                                                                                         8
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                     |
                                                                                                     (255
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          &
                                                                                                          0xff
                                                                                                              as
                                                                                                              libc::c_int)
                                                                                                         <<
                                                                                                         0
                                                                                                             as
                                                                                                             libc::c_int)
                                                                                                    as
                                                                                                    u32_0);
                        if (*globalCtx).transitionCtx.setEnvColor.is_some() {
                            (*globalCtx).transitionCtx.setEnvColor.expect("non-null function pointer")(&mut (*globalCtx).transitionCtx.c2rust_unnamed.data
                                                                                                           as
                                                                                                           *mut [libc::c_char; 552]
                                                                                                           as
                                                                                                           *mut libc::c_void,
                                                                                                       ((160
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             &
                                                                                                             0xff
                                                                                                                 as
                                                                                                                 libc::c_int)
                                                                                                            <<
                                                                                                            24
                                                                                                                as
                                                                                                                libc::c_int
                                                                                                            |
                                                                                                            (160
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 &
                                                                                                                 0xff
                                                                                                                     as
                                                                                                                     libc::c_int)
                                                                                                                <<
                                                                                                                16
                                                                                                                    as
                                                                                                                    libc::c_int
                                                                                                            |
                                                                                                            (160
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 &
                                                                                                                 0xff
                                                                                                                     as
                                                                                                                     libc::c_int)
                                                                                                                <<
                                                                                                                8
                                                                                                                    as
                                                                                                                    libc::c_int
                                                                                                            |
                                                                                                            (255
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 &
                                                                                                                 0xff
                                                                                                                     as
                                                                                                                     libc::c_int)
                                                                                                                <<
                                                                                                                0
                                                                                                                    as
                                                                                                                    libc::c_int)
                                                                                                           as
                                                                                                           u32_0);
                        }
                    } else if (*globalCtx).transitionCtx.transitionType ==
                                  18 as libc::c_int {
                        (*globalCtx).transitionCtx.setColor.expect("non-null function pointer")(&mut (*globalCtx).transitionCtx.c2rust_unnamed.data
                                                                                                    as
                                                                                                    *mut [libc::c_char; 552]
                                                                                                    as
                                                                                                    *mut libc::c_void,
                                                                                                ((140
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      &
                                                                                                      0xff
                                                                                                          as
                                                                                                          libc::c_int)
                                                                                                     <<
                                                                                                     24
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                     |
                                                                                                     (140
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          &
                                                                                                          0xff
                                                                                                              as
                                                                                                              libc::c_int)
                                                                                                         <<
                                                                                                         16
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                     |
                                                                                                     (100
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          &
                                                                                                          0xff
                                                                                                              as
                                                                                                              libc::c_int)
                                                                                                         <<
                                                                                                         8
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                     |
                                                                                                     (255
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          &
                                                                                                          0xff
                                                                                                              as
                                                                                                              libc::c_int)
                                                                                                         <<
                                                                                                         0
                                                                                                             as
                                                                                                             libc::c_int)
                                                                                                    as
                                                                                                    u32_0);
                        if (*globalCtx).transitionCtx.setEnvColor.is_some() {
                            (*globalCtx).transitionCtx.setEnvColor.expect("non-null function pointer")(&mut (*globalCtx).transitionCtx.c2rust_unnamed.data
                                                                                                           as
                                                                                                           *mut [libc::c_char; 552]
                                                                                                           as
                                                                                                           *mut libc::c_void,
                                                                                                       ((140
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             &
                                                                                                             0xff
                                                                                                                 as
                                                                                                                 libc::c_int)
                                                                                                            <<
                                                                                                            24
                                                                                                                as
                                                                                                                libc::c_int
                                                                                                            |
                                                                                                            (140
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 &
                                                                                                                 0xff
                                                                                                                     as
                                                                                                                     libc::c_int)
                                                                                                                <<
                                                                                                                16
                                                                                                                    as
                                                                                                                    libc::c_int
                                                                                                            |
                                                                                                            (100
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 &
                                                                                                                 0xff
                                                                                                                     as
                                                                                                                     libc::c_int)
                                                                                                                <<
                                                                                                                8
                                                                                                                    as
                                                                                                                    libc::c_int
                                                                                                            |
                                                                                                            (255
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 &
                                                                                                                 0xff
                                                                                                                     as
                                                                                                                     libc::c_int)
                                                                                                                <<
                                                                                                                0
                                                                                                                    as
                                                                                                                    libc::c_int)
                                                                                                           as
                                                                                                           u32_0);
                        }
                    } else if (*globalCtx).transitionCtx.transitionType ==
                                  19 as libc::c_int {
                        (*globalCtx).transitionCtx.setColor.expect("non-null function pointer")(&mut (*globalCtx).transitionCtx.c2rust_unnamed.data
                                                                                                    as
                                                                                                    *mut [libc::c_char; 552]
                                                                                                    as
                                                                                                    *mut libc::c_void,
                                                                                                ((70
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      &
                                                                                                      0xff
                                                                                                          as
                                                                                                          libc::c_int)
                                                                                                     <<
                                                                                                     24
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                     |
                                                                                                     (100
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          &
                                                                                                          0xff
                                                                                                              as
                                                                                                              libc::c_int)
                                                                                                         <<
                                                                                                         16
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                     |
                                                                                                     (110
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          &
                                                                                                          0xff
                                                                                                              as
                                                                                                              libc::c_int)
                                                                                                         <<
                                                                                                         8
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                     |
                                                                                                     (255
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          &
                                                                                                          0xff
                                                                                                              as
                                                                                                              libc::c_int)
                                                                                                         <<
                                                                                                         0
                                                                                                             as
                                                                                                             libc::c_int)
                                                                                                    as
                                                                                                    u32_0);
                        if (*globalCtx).transitionCtx.setEnvColor.is_some() {
                            (*globalCtx).transitionCtx.setEnvColor.expect("non-null function pointer")(&mut (*globalCtx).transitionCtx.c2rust_unnamed.data
                                                                                                           as
                                                                                                           *mut [libc::c_char; 552]
                                                                                                           as
                                                                                                           *mut libc::c_void,
                                                                                                       ((70
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             &
                                                                                                             0xff
                                                                                                                 as
                                                                                                                 libc::c_int)
                                                                                                            <<
                                                                                                            24
                                                                                                                as
                                                                                                                libc::c_int
                                                                                                            |
                                                                                                            (100
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 &
                                                                                                                 0xff
                                                                                                                     as
                                                                                                                     libc::c_int)
                                                                                                                <<
                                                                                                                16
                                                                                                                    as
                                                                                                                    libc::c_int
                                                                                                            |
                                                                                                            (110
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 &
                                                                                                                 0xff
                                                                                                                     as
                                                                                                                     libc::c_int)
                                                                                                                <<
                                                                                                                8
                                                                                                                    as
                                                                                                                    libc::c_int
                                                                                                            |
                                                                                                            (255
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 &
                                                                                                                 0xff
                                                                                                                     as
                                                                                                                     libc::c_int)
                                                                                                                <<
                                                                                                                0
                                                                                                                    as
                                                                                                                    libc::c_int)
                                                                                                           as
                                                                                                           u32_0);
                        }
                    } else {
                        (*globalCtx).transitionCtx.setColor.expect("non-null function pointer")(&mut (*globalCtx).transitionCtx.c2rust_unnamed.data
                                                                                                    as
                                                                                                    *mut [libc::c_char; 552]
                                                                                                    as
                                                                                                    *mut libc::c_void,
                                                                                                ((0
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      &
                                                                                                      0xff
                                                                                                          as
                                                                                                          libc::c_int)
                                                                                                     <<
                                                                                                     24
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                     |
                                                                                                     (0
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          &
                                                                                                          0xff
                                                                                                              as
                                                                                                              libc::c_int)
                                                                                                         <<
                                                                                                         16
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                     |
                                                                                                     (0
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          &
                                                                                                          0xff
                                                                                                              as
                                                                                                              libc::c_int)
                                                                                                         <<
                                                                                                         8
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                     |
                                                                                                     (0
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          &
                                                                                                          0xff
                                                                                                              as
                                                                                                              libc::c_int)
                                                                                                         <<
                                                                                                         0
                                                                                                             as
                                                                                                             libc::c_int)
                                                                                                    as
                                                                                                    u32_0);
                        if (*globalCtx).transitionCtx.setEnvColor.is_some() {
                            (*globalCtx).transitionCtx.setEnvColor.expect("non-null function pointer")(&mut (*globalCtx).transitionCtx.c2rust_unnamed.data
                                                                                                           as
                                                                                                           *mut [libc::c_char; 552]
                                                                                                           as
                                                                                                           *mut libc::c_void,
                                                                                                       ((0
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             &
                                                                                                             0xff
                                                                                                                 as
                                                                                                                 libc::c_int)
                                                                                                            <<
                                                                                                            24
                                                                                                                as
                                                                                                                libc::c_int
                                                                                                            |
                                                                                                            (0
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 &
                                                                                                                 0xff
                                                                                                                     as
                                                                                                                     libc::c_int)
                                                                                                                <<
                                                                                                                16
                                                                                                                    as
                                                                                                                    libc::c_int
                                                                                                            |
                                                                                                            (0
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 &
                                                                                                                 0xff
                                                                                                                     as
                                                                                                                     libc::c_int)
                                                                                                                <<
                                                                                                                8
                                                                                                                    as
                                                                                                                    libc::c_int
                                                                                                            |
                                                                                                            (0
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 &
                                                                                                                 0xff
                                                                                                                     as
                                                                                                                     libc::c_int)
                                                                                                                <<
                                                                                                                0
                                                                                                                    as
                                                                                                                    libc::c_int)
                                                                                                           as
                                                                                                           u32_0);
                        }
                    }
                    if (*globalCtx).sceneLoadFlag as libc::c_int ==
                           -(0x14 as libc::c_int) {
                        (*globalCtx).transitionCtx.setType.expect("non-null function pointer")(&mut (*globalCtx).transitionCtx.c2rust_unnamed.data
                                                                                                   as
                                                                                                   *mut [libc::c_char; 552]
                                                                                                   as
                                                                                                   *mut libc::c_void,
                                                                                               1
                                                                                                   as
                                                                                                   libc::c_int);
                    } else {
                        (*globalCtx).transitionCtx.setType.expect("non-null function pointer")(&mut (*globalCtx).transitionCtx.c2rust_unnamed.data
                                                                                                   as
                                                                                                   *mut [libc::c_char; 552]
                                                                                                   as
                                                                                                   *mut libc::c_void,
                                                                                               2
                                                                                                   as
                                                                                                   libc::c_int);
                    }
                    (*globalCtx).transitionCtx.start.expect("non-null function pointer")(&mut (*globalCtx).transitionCtx
                                                                                             as
                                                                                             *mut TransitionContext
                                                                                             as
                                                                                             *mut libc::c_void);
                    if (*globalCtx).transitionCtx.transitionType ==
                           13 as libc::c_int {
                        (*globalCtx).transitionMode =
                            11 as libc::c_int as u8_0
                    } else {
                        (*globalCtx).transitionMode = 3 as libc::c_int as u8_0
                    }
                }
                _ => { }
            }
            match (*globalCtx).transitionMode as libc::c_int {
                4 => {
                    D_801614C8 = 0 as libc::c_int as s16;
                    (*globalCtx).envCtx.fillScreen = 1 as libc::c_int as u8_0;
                    (*globalCtx).envCtx.screenFillColor[0 as libc::c_int as
                                                            usize] =
                        160 as libc::c_int as u8_0;
                    (*globalCtx).envCtx.screenFillColor[1 as libc::c_int as
                                                            usize] =
                        160 as libc::c_int as u8_0;
                    (*globalCtx).envCtx.screenFillColor[2 as libc::c_int as
                                                            usize] =
                        160 as libc::c_int as u8_0;
                    if (*globalCtx).sceneLoadFlag as libc::c_int !=
                           -(0x14 as libc::c_int) {
                        (*globalCtx).envCtx.screenFillColor[3 as libc::c_int
                                                                as usize] =
                            0 as libc::c_int as u8_0;
                        (*globalCtx).transitionMode = 5 as libc::c_int as u8_0
                    } else {
                        (*globalCtx).envCtx.screenFillColor[3 as libc::c_int
                                                                as usize] =
                            255 as libc::c_int as u8_0;
                        (*globalCtx).transitionMode = 6 as libc::c_int as u8_0
                    }
                }
                5 => {
                    (*globalCtx).envCtx.screenFillColor[3 as libc::c_int as
                                                            usize] =
                        (D_801614C8 as libc::c_int as libc::c_float / 20.0f32
                             * 255.0f32) as u8_0;
                    if D_801614C8 as libc::c_int >= 20 as libc::c_int &&
                           1 as libc::c_int != 0 {
                        (*globalCtx).state.running =
                            0 as libc::c_int as u32_0;
                        (*globalCtx).state.init =
                            Some(Gameplay_Init as
                                     unsafe extern "C" fn(_: *mut GameState)
                                         -> ());
                        (*globalCtx).state.size =
                            ::std::mem::size_of::<GlobalContext>() as
                                libc::c_ulong;
                        gSaveContext.entranceIndex =
                            (*globalCtx).nextEntranceIndex as s32;
                        (*globalCtx).sceneLoadFlag = 0 as libc::c_int as s8;
                        (*globalCtx).transitionMode = 0 as libc::c_int as u8_0
                    } else { D_801614C8 += 1 }
                }
                6 => {
                    (*globalCtx).envCtx.screenFillColor[3 as libc::c_int as
                                                            usize] =
                        ((1 as libc::c_int as libc::c_float -
                              D_801614C8 as libc::c_int as libc::c_float /
                                  20.0f32) * 255.0f32) as u8_0;
                    if D_801614C8 as libc::c_int >= 20 as libc::c_int &&
                           1 as libc::c_int != 0 {
                        gTrnsnUnkState = 0 as libc::c_int;
                        (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int
                                               * 16 as libc::c_int +
                                               30 as libc::c_int) as usize] =
                            3 as libc::c_int as s16;
                        (*globalCtx).sceneLoadFlag = 0 as libc::c_int as s8;
                        (*globalCtx).transitionMode =
                            0 as libc::c_int as u8_0;
                        (*globalCtx).envCtx.fillScreen =
                            0 as libc::c_int as u8_0
                    } else { D_801614C8 += 1 }
                }
                7 => {
                    D_801614C8 = 0 as libc::c_int as s16;
                    (*globalCtx).envCtx.fillScreen = 1 as libc::c_int as u8_0;
                    (*globalCtx).envCtx.screenFillColor[0 as libc::c_int as
                                                            usize] =
                        170 as libc::c_int as u8_0;
                    (*globalCtx).envCtx.screenFillColor[1 as libc::c_int as
                                                            usize] =
                        160 as libc::c_int as u8_0;
                    (*globalCtx).envCtx.screenFillColor[2 as libc::c_int as
                                                            usize] =
                        150 as libc::c_int as u8_0;
                    if (*globalCtx).sceneLoadFlag as libc::c_int !=
                           -(0x14 as libc::c_int) {
                        (*globalCtx).envCtx.screenFillColor[3 as libc::c_int
                                                                as usize] =
                            0 as libc::c_int as u8_0;
                        (*globalCtx).transitionMode = 5 as libc::c_int as u8_0
                    } else {
                        (*globalCtx).envCtx.screenFillColor[3 as libc::c_int
                                                                as usize] =
                            255 as libc::c_int as u8_0;
                        (*globalCtx).transitionMode = 6 as libc::c_int as u8_0
                    }
                }
                10 => {
                    if (*globalCtx).sceneLoadFlag as libc::c_int !=
                           -(0x14 as libc::c_int) {
                        (*globalCtx).state.running =
                            0 as libc::c_int as u32_0;
                        (*globalCtx).state.init =
                            Some(Gameplay_Init as
                                     unsafe extern "C" fn(_: *mut GameState)
                                         -> ());
                        (*globalCtx).state.size =
                            ::std::mem::size_of::<GlobalContext>() as
                                libc::c_ulong;
                        gSaveContext.entranceIndex =
                            (*globalCtx).nextEntranceIndex as s32;
                        (*globalCtx).sceneLoadFlag = 0 as libc::c_int as s8;
                        (*globalCtx).transitionMode = 0 as libc::c_int as u8_0
                    } else {
                        gTrnsnUnkState = 0 as libc::c_int;
                        (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int
                                               * 16 as libc::c_int +
                                               30 as libc::c_int) as usize] =
                            3 as libc::c_int as s16;
                        (*globalCtx).sceneLoadFlag = 0 as libc::c_int as s8;
                        (*globalCtx).transitionMode = 0 as libc::c_int as u8_0
                    }
                }
                11 => {
                    if gSaveContext.unk_1410 as libc::c_int !=
                           0 as libc::c_int {
                        (*globalCtx).transitionMode = 3 as libc::c_int as u8_0
                    }
                }
                12 => {
                    if (*globalCtx).sceneLoadFlag as libc::c_int !=
                           -(0x14 as libc::c_int) {
                        (*globalCtx).envCtx.sandstormState =
                            1 as libc::c_int as u8_0;
                        (*globalCtx).transitionMode =
                            13 as libc::c_int as u8_0
                    } else {
                        (*globalCtx).envCtx.sandstormState =
                            2 as libc::c_int as u8_0;
                        (*globalCtx).envCtx.sandstormPrimA =
                            255 as libc::c_int as u8_0;
                        (*globalCtx).envCtx.sandstormEnvA =
                            255 as libc::c_int as u8_0;
                        (*globalCtx).transitionMode =
                            13 as libc::c_int as u8_0
                    }
                }
                13 => {
                    Audio_PlaySoundGeneral((0x28c0 as libc::c_int -
                                                0x800 as libc::c_int) as
                                               u16_0, &mut D_801333D4,
                                           4 as libc::c_int as u8_0,
                                           &mut D_801333E0, &mut D_801333E0,
                                           &mut D_801333E8);
                    if (*globalCtx).sceneLoadFlag as libc::c_int ==
                           -(0x14 as libc::c_int) {
                        if ((*globalCtx).envCtx.sandstormPrimA as libc::c_int)
                               < 110 as libc::c_int {
                            gTrnsnUnkState = 0 as libc::c_int;
                            (*gGameInfo).data[(1 as libc::c_int *
                                                   6 as libc::c_int *
                                                   16 as libc::c_int +
                                                   30 as libc::c_int) as
                                                  usize] =
                                3 as libc::c_int as s16;
                            (*globalCtx).sceneLoadFlag =
                                0 as libc::c_int as s8;
                            (*globalCtx).transitionMode =
                                0 as libc::c_int as u8_0
                        }
                    } else if (*globalCtx).envCtx.sandstormEnvA as libc::c_int
                                  == 255 as libc::c_int {
                        (*globalCtx).state.running =
                            0 as libc::c_int as u32_0;
                        (*globalCtx).state.init =
                            Some(Gameplay_Init as
                                     unsafe extern "C" fn(_: *mut GameState)
                                         -> ());
                        (*globalCtx).state.size =
                            ::std::mem::size_of::<GlobalContext>() as
                                libc::c_ulong;
                        gSaveContext.entranceIndex =
                            (*globalCtx).nextEntranceIndex as s32;
                        (*globalCtx).sceneLoadFlag = 0 as libc::c_int as s8;
                        (*globalCtx).transitionMode = 0 as libc::c_int as u8_0
                    }
                }
                14 => {
                    if (*globalCtx).sceneLoadFlag as libc::c_int ==
                           -(0x14 as libc::c_int) {
                        (*globalCtx).envCtx.sandstormState =
                            4 as libc::c_int as u8_0;
                        (*globalCtx).envCtx.sandstormPrimA =
                            255 as libc::c_int as u8_0;
                        (*globalCtx).envCtx.sandstormEnvA =
                            255 as libc::c_int as u8_0;
                        // "It's here!!!!!!!!!"
                        LogUtils_LogThreadId(b"../z_play.c\x00" as *const u8
                                                 as *const libc::c_char,
                                             3471 as libc::c_int);
                        osSyncPrintf(b"\"\xe6\x9d\xa5\xe3\x81\x9f!!!!!!!!!!!!!!!!!!!!!\" = %s\n\x00"
                                         as *const u8 as *const libc::c_char,
                                     b"\xe6\x9d\xa5\xe3\x81\x9f!!!!!!!!!!!!!!!!!!!!!\x00"
                                         as *const u8 as *const libc::c_char);
                        (*globalCtx).transitionMode =
                            15 as libc::c_int as u8_0
                    } else {
                        (*globalCtx).transitionMode =
                            12 as libc::c_int as u8_0
                    }
                }
                15 => {
                    Audio_PlaySoundGeneral((0x28c0 as libc::c_int -
                                                0x800 as libc::c_int) as
                                               u16_0, &mut D_801333D4,
                                           4 as libc::c_int as u8_0,
                                           &mut D_801333E0, &mut D_801333E0,
                                           &mut D_801333E8);
                    if (*globalCtx).sceneLoadFlag as libc::c_int ==
                           -(0x14 as libc::c_int) {
                        if (*globalCtx).envCtx.sandstormPrimA as libc::c_int
                               <= 0 as libc::c_int {
                            gTrnsnUnkState = 0 as libc::c_int;
                            (*gGameInfo).data[(1 as libc::c_int *
                                                   6 as libc::c_int *
                                                   16 as libc::c_int +
                                                   30 as libc::c_int) as
                                                  usize] =
                                3 as libc::c_int as s16;
                            (*globalCtx).sceneLoadFlag =
                                0 as libc::c_int as s8;
                            (*globalCtx).transitionMode =
                                0 as libc::c_int as u8_0
                        }
                    }
                }
                16 => {
                    D_801614C8 = 0 as libc::c_int as s16;
                    (*globalCtx).envCtx.fillScreen = 1 as libc::c_int as u8_0;
                    (*globalCtx).envCtx.screenFillColor[0 as libc::c_int as
                                                            usize] =
                        0 as libc::c_int as u8_0;
                    (*globalCtx).envCtx.screenFillColor[1 as libc::c_int as
                                                            usize] =
                        0 as libc::c_int as u8_0;
                    (*globalCtx).envCtx.screenFillColor[2 as libc::c_int as
                                                            usize] =
                        0 as libc::c_int as u8_0;
                    (*globalCtx).envCtx.screenFillColor[3 as libc::c_int as
                                                            usize] =
                        255 as libc::c_int as u8_0;
                    (*globalCtx).transitionMode = 17 as libc::c_int as u8_0
                }
                17 => {
                    if gSaveContext.unk_1410 as libc::c_int !=
                           0 as libc::c_int {
                        (*globalCtx).envCtx.screenFillColor[3 as libc::c_int
                                                                as usize] =
                            gSaveContext.unk_1410;
                        if (gSaveContext.unk_1410 as libc::c_int) <
                               0x65 as libc::c_int {
                            gTrnsnUnkState = 0 as libc::c_int;
                            (*gGameInfo).data[(1 as libc::c_int *
                                                   6 as libc::c_int *
                                                   16 as libc::c_int +
                                                   30 as libc::c_int) as
                                                  usize] =
                                3 as libc::c_int as s16;
                            (*globalCtx).sceneLoadFlag =
                                0 as libc::c_int as s8;
                            (*globalCtx).transitionMode =
                                0 as libc::c_int as u8_0
                        }
                    }
                }
                _ => { }
            }
        }
        if 1 as libc::c_int != 0 &&
               (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                      16 as libc::c_int + 63 as libc::c_int)
                                     as usize] as libc::c_int != 0 {
            LogUtils_LogThreadId(b"../z_play.c\x00" as *const u8 as
                                     *const libc::c_char,
                                 3533 as libc::c_int);
            osSyncPrintf(b"1 = %d\n\x00" as *const u8 as *const libc::c_char,
                         1 as libc::c_int);
        }
        if 1 as libc::c_int != 0 && gTrnsnUnkState != 3 as libc::c_int {
            if 1 as libc::c_int != 0 &&
                   (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          63 as libc::c_int) as usize] as
                       libc::c_int != 0 {
                LogUtils_LogThreadId(b"../z_play.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     3542 as libc::c_int);
                osSyncPrintf(b"1 = %d\n\x00" as *const u8 as
                                 *const libc::c_char, 1 as libc::c_int);
            }
            if gSaveContext.gameMode == 0 as libc::c_int &&
                   (*globalCtx).msgCtx.msgMode as libc::c_int ==
                       MSGMODE_NONE as libc::c_int &&
                   (*globalCtx).gameOverCtx.state as libc::c_int ==
                       GAMEOVER_INACTIVE as libc::c_int {
                KaleidoSetup_Update(globalCtx);
            }
            if 1 as libc::c_int != 0 &&
                   (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          63 as libc::c_int) as usize] as
                       libc::c_int != 0 {
                LogUtils_LogThreadId(b"../z_play.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     3551 as libc::c_int);
                osSyncPrintf(b"1 = %d\n\x00" as *const u8 as
                                 *const libc::c_char, 1 as libc::c_int);
            }
            sp80 =
                ((*globalCtx).pauseCtx.state as libc::c_int !=
                     0 as libc::c_int ||
                     (*globalCtx).pauseCtx.debugState as libc::c_int !=
                         0 as libc::c_int) as libc::c_int;
            if 1 as libc::c_int != 0 &&
                   (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          63 as libc::c_int) as usize] as
                       libc::c_int != 0 {
                LogUtils_LogThreadId(b"../z_play.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     3555 as libc::c_int);
                osSyncPrintf(b"1 = %d\n\x00" as *const u8 as
                                 *const libc::c_char, 1 as libc::c_int);
            }
            AnimationContext_Reset(&mut (*globalCtx).animationCtx);
            if 1 as libc::c_int != 0 &&
                   (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          63 as libc::c_int) as usize] as
                       libc::c_int != 0 {
                LogUtils_LogThreadId(b"../z_play.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     3561 as libc::c_int);
                osSyncPrintf(b"1 = %d\n\x00" as *const u8 as
                                 *const libc::c_char, 1 as libc::c_int);
            }
            Object_UpdateBank(&mut (*globalCtx).objectCtx);
            if 1 as libc::c_int != 0 &&
                   (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          63 as libc::c_int) as usize] as
                       libc::c_int != 0 {
                LogUtils_LogThreadId(b"../z_play.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     3577 as libc::c_int);
                osSyncPrintf(b"1 = %d\n\x00" as *const u8 as
                                 *const libc::c_char, 1 as libc::c_int);
            }
            if sp80 == 0 as libc::c_int &&
                   (*gGameInfo).data[(9 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          72 as libc::c_int) as usize] as
                       libc::c_int == 0 as libc::c_int {
                if 1 as libc::c_int != 0 &&
                       (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int
                                              * 16 as libc::c_int +
                                              63 as libc::c_int) as usize] as
                           libc::c_int != 0 {
                    LogUtils_LogThreadId(b"../z_play.c\x00" as *const u8 as
                                             *const libc::c_char,
                                         3580 as libc::c_int);
                    osSyncPrintf(b"1 = %d\n\x00" as *const u8 as
                                     *const libc::c_char, 1 as libc::c_int);
                }
                (*globalCtx).gameplayFrames =
                    (*globalCtx).gameplayFrames.wrapping_add(1);
                func_800AA178(1 as libc::c_int as u32_0);
                if (*globalCtx).actorCtx.freezeFlashTimer as libc::c_int != 0
                       &&
                       {
                           let fresh0 =
                               (*globalCtx).actorCtx.freezeFlashTimer;
                           (*globalCtx).actorCtx.freezeFlashTimer =
                               (*globalCtx).actorCtx.freezeFlashTimer.wrapping_sub(1);
                           ((fresh0 as libc::c_int)) < 5 as libc::c_int
                       } {
                    osSyncPrintf(b"FINISH=%d\n\x00" as *const u8 as
                                     *const libc::c_char,
                                 (*globalCtx).actorCtx.freezeFlashTimer as
                                     libc::c_int);
                    if (*globalCtx).actorCtx.freezeFlashTimer as libc::c_int >
                           0 as libc::c_int &&
                           (*globalCtx).actorCtx.freezeFlashTimer as
                               libc::c_int % 2 as libc::c_int !=
                               0 as libc::c_int {
                        (*globalCtx).envCtx.fillScreen =
                            1 as libc::c_int as u8_0;
                        (*globalCtx).envCtx.screenFillColor[2 as libc::c_int
                                                                as usize] =
                            150 as libc::c_int as u8_0;
                        (*globalCtx).envCtx.screenFillColor[1 as libc::c_int
                                                                as usize] =
                            (*globalCtx).envCtx.screenFillColor[2 as
                                                                    libc::c_int
                                                                    as usize];
                        (*globalCtx).envCtx.screenFillColor[0 as libc::c_int
                                                                as usize] =
                            (*globalCtx).envCtx.screenFillColor[1 as
                                                                    libc::c_int
                                                                    as usize];
                        (*globalCtx).envCtx.screenFillColor[3 as libc::c_int
                                                                as usize] =
                            80 as libc::c_int as u8_0
                    } else {
                        (*globalCtx).envCtx.fillScreen =
                            0 as libc::c_int as u8_0
                    }
                } else {
                    if 1 as libc::c_int != 0 &&
                           (*gGameInfo).data[(21 as libc::c_int *
                                                  6 as libc::c_int *
                                                  16 as libc::c_int +
                                                  63 as libc::c_int) as usize]
                               as libc::c_int != 0 {
                        LogUtils_LogThreadId(b"../z_play.c\x00" as *const u8
                                                 as *const libc::c_char,
                                             3606 as libc::c_int);
                        osSyncPrintf(b"1 = %d\n\x00" as *const u8 as
                                         *const libc::c_char,
                                     1 as libc::c_int);
                    }
                    func_800973FC(globalCtx, &mut (*globalCtx).roomCtx);
                    if 1 as libc::c_int != 0 &&
                           (*gGameInfo).data[(21 as libc::c_int *
                                                  6 as libc::c_int *
                                                  16 as libc::c_int +
                                                  63 as libc::c_int) as usize]
                               as libc::c_int != 0 {
                        LogUtils_LogThreadId(b"../z_play.c\x00" as *const u8
                                                 as *const libc::c_char,
                                             3612 as libc::c_int);
                        osSyncPrintf(b"1 = %d\n\x00" as *const u8 as
                                         *const libc::c_char,
                                     1 as libc::c_int);
                    }
                    CollisionCheck_AT(globalCtx, &mut (*globalCtx).colChkCtx);
                    if 1 as libc::c_int != 0 &&
                           (*gGameInfo).data[(21 as libc::c_int *
                                                  6 as libc::c_int *
                                                  16 as libc::c_int +
                                                  63 as libc::c_int) as usize]
                               as libc::c_int != 0 {
                        LogUtils_LogThreadId(b"../z_play.c\x00" as *const u8
                                                 as *const libc::c_char,
                                             3618 as libc::c_int);
                        osSyncPrintf(b"1 = %d\n\x00" as *const u8 as
                                         *const libc::c_char,
                                     1 as libc::c_int);
                    }
                    CollisionCheck_OC(globalCtx, &mut (*globalCtx).colChkCtx);
                    if 1 as libc::c_int != 0 &&
                           (*gGameInfo).data[(21 as libc::c_int *
                                                  6 as libc::c_int *
                                                  16 as libc::c_int +
                                                  63 as libc::c_int) as usize]
                               as libc::c_int != 0 {
                        LogUtils_LogThreadId(b"../z_play.c\x00" as *const u8
                                                 as *const libc::c_char,
                                             3624 as libc::c_int);
                        osSyncPrintf(b"1 = %d\n\x00" as *const u8 as
                                         *const libc::c_char,
                                     1 as libc::c_int);
                    }
                    CollisionCheck_Damage(globalCtx,
                                          &mut (*globalCtx).colChkCtx);
                    if 1 as libc::c_int != 0 &&
                           (*gGameInfo).data[(21 as libc::c_int *
                                                  6 as libc::c_int *
                                                  16 as libc::c_int +
                                                  63 as libc::c_int) as usize]
                               as libc::c_int != 0 {
                        LogUtils_LogThreadId(b"../z_play.c\x00" as *const u8
                                                 as *const libc::c_char,
                                             3631 as libc::c_int);
                        osSyncPrintf(b"1 = %d\n\x00" as *const u8 as
                                         *const libc::c_char,
                                     1 as libc::c_int);
                    }
                    CollisionCheck_ClearContext(globalCtx,
                                                &mut (*globalCtx).colChkCtx);
                    if 1 as libc::c_int != 0 &&
                           (*gGameInfo).data[(21 as libc::c_int *
                                                  6 as libc::c_int *
                                                  16 as libc::c_int +
                                                  63 as libc::c_int) as usize]
                               as libc::c_int != 0 {
                        LogUtils_LogThreadId(b"../z_play.c\x00" as *const u8
                                                 as *const libc::c_char,
                                             3637 as libc::c_int);
                        osSyncPrintf(b"1 = %d\n\x00" as *const u8 as
                                         *const libc::c_char,
                                     1 as libc::c_int);
                    }
                    if (*globalCtx).unk_11DE9 as libc::c_int ==
                           0 as libc::c_int {
                        Actor_UpdateAll(globalCtx,
                                        &mut (*globalCtx).actorCtx);
                    }
                    if 1 as libc::c_int != 0 &&
                           (*gGameInfo).data[(21 as libc::c_int *
                                                  6 as libc::c_int *
                                                  16 as libc::c_int +
                                                  63 as libc::c_int) as usize]
                               as libc::c_int != 0 {
                        LogUtils_LogThreadId(b"../z_play.c\x00" as *const u8
                                                 as *const libc::c_char,
                                             3643 as libc::c_int);
                        osSyncPrintf(b"1 = %d\n\x00" as *const u8 as
                                         *const libc::c_char,
                                     1 as libc::c_int);
                    }
                    func_80064558(globalCtx, &mut (*globalCtx).csCtx);
                    if 1 as libc::c_int != 0 &&
                           (*gGameInfo).data[(21 as libc::c_int *
                                                  6 as libc::c_int *
                                                  16 as libc::c_int +
                                                  63 as libc::c_int) as usize]
                               as libc::c_int != 0 {
                        LogUtils_LogThreadId(b"../z_play.c\x00" as *const u8
                                                 as *const libc::c_char,
                                             3648 as libc::c_int);
                        osSyncPrintf(b"1 = %d\n\x00" as *const u8 as
                                         *const libc::c_char,
                                     1 as libc::c_int);
                    }
                    func_800645A0(globalCtx, &mut (*globalCtx).csCtx);
                    if 1 as libc::c_int != 0 &&
                           (*gGameInfo).data[(21 as libc::c_int *
                                                  6 as libc::c_int *
                                                  16 as libc::c_int +
                                                  63 as libc::c_int) as usize]
                               as libc::c_int != 0 {
                        LogUtils_LogThreadId(b"../z_play.c\x00" as *const u8
                                                 as *const libc::c_char,
                                             3651 as libc::c_int);
                        osSyncPrintf(b"1 = %d\n\x00" as *const u8 as
                                         *const libc::c_char,
                                     1 as libc::c_int);
                    }
                    Effect_UpdateAll(globalCtx);
                    if 1 as libc::c_int != 0 &&
                           (*gGameInfo).data[(21 as libc::c_int *
                                                  6 as libc::c_int *
                                                  16 as libc::c_int +
                                                  63 as libc::c_int) as usize]
                               as libc::c_int != 0 {
                        LogUtils_LogThreadId(b"../z_play.c\x00" as *const u8
                                                 as *const libc::c_char,
                                             3657 as libc::c_int);
                        osSyncPrintf(b"1 = %d\n\x00" as *const u8 as
                                         *const libc::c_char,
                                     1 as libc::c_int);
                    }
                    EffectSs_UpdateAll(globalCtx);
                    if 1 as libc::c_int != 0 &&
                           (*gGameInfo).data[(21 as libc::c_int *
                                                  6 as libc::c_int *
                                                  16 as libc::c_int +
                                                  63 as libc::c_int) as usize]
                               as libc::c_int != 0 {
                        LogUtils_LogThreadId(b"../z_play.c\x00" as *const u8
                                                 as *const libc::c_char,
                                             3662 as libc::c_int);
                        osSyncPrintf(b"1 = %d\n\x00" as *const u8 as
                                         *const libc::c_char,
                                     1 as libc::c_int);
                    }
                }
            } else { func_800AA178(0 as libc::c_int as u32_0); }
            if 1 as libc::c_int != 0 &&
                   (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          63 as libc::c_int) as usize] as
                       libc::c_int != 0 {
                LogUtils_LogThreadId(b"../z_play.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     3672 as libc::c_int);
                osSyncPrintf(b"1 = %d\n\x00" as *const u8 as
                                 *const libc::c_char, 1 as libc::c_int);
            }
            func_80095AA0(globalCtx, &mut (*globalCtx).roomCtx.curRoom,
                          &mut *input.offset(1 as libc::c_int as isize),
                          0 as libc::c_int);
            if 1 as libc::c_int != 0 &&
                   (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          63 as libc::c_int) as usize] as
                       libc::c_int != 0 {
                LogUtils_LogThreadId(b"../z_play.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     3675 as libc::c_int);
                osSyncPrintf(b"1 = %d\n\x00" as *const u8 as
                                 *const libc::c_char, 1 as libc::c_int);
            }
            func_80095AA0(globalCtx, &mut (*globalCtx).roomCtx.prevRoom,
                          &mut *input.offset(1 as libc::c_int as isize),
                          1 as libc::c_int);
            if 1 as libc::c_int != 0 &&
                   (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          63 as libc::c_int) as usize] as
                       libc::c_int != 0 {
                LogUtils_LogThreadId(b"../z_play.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     3677 as libc::c_int);
                osSyncPrintf(b"1 = %d\n\x00" as *const u8 as
                                 *const libc::c_char, 1 as libc::c_int);
            }
            if (*globalCtx).unk_1242B as libc::c_int != 0 as libc::c_int {
                if !((*input.offset(0 as libc::c_int as isize)).press.button
                         as libc::c_int | !(0x8 as libc::c_int)) ==
                       0 as libc::c_int {
                    if (*globalCtx).pauseCtx.state as libc::c_int !=
                           0 as libc::c_int ||
                           (*globalCtx).pauseCtx.debugState as libc::c_int !=
                               0 as libc::c_int {
                        // "Changing viewpoint is prohibited due to the kaleidoscope"
                        osSyncPrintf(b"\x1b[36m\xe3\x82\xab\xe3\x83\xac\xe3\x82\xa4\xe3\x83\x89\xe3\x82\xb9\xe3\x82\xb3\xe3\x83\xbc\xe3\x83\x97\xe4\xb8\xad\xe3\x81\xab\xe3\x81\xa4\xe3\x81\x8d\xe8\xa6\x96\xe7\x82\xb9\xe5\xa4\x89\xe6\x9b\xb4\xe3\x82\x92\xe7\xa6\x81\xe6\xad\xa2\xe3\x81\x97\xe3\x81\xa6\xe3\x81\x8a\xe3\x82\x8a\xe3\x81\xbe\xe3\x81\x99\n\x1b[m\x00"
                                         as *const u8 as *const libc::c_char);
                    } else if Player_InCsMode(globalCtx) != 0 {
                        // "Changing viewpoint is prohibited during the cutscene"
                        osSyncPrintf(b"\x1b[36m\xe3\x83\x87\xe3\x83\xa2\xe4\xb8\xad\xe3\x81\xab\xe3\x81\xa4\xe3\x81\x8d\xe8\xa6\x96\xe7\x82\xb9\xe5\xa4\x89\xe6\x9b\xb4\xe3\x82\x92\xe7\xa6\x81\xe6\xad\xa2\xe3\x81\x97\xe3\x81\xa6\xe3\x81\x8a\xe3\x82\x8a\xe3\x81\xbe\xe3\x81\x99\n\x1b[m\x00"
                                         as *const u8 as *const libc::c_char);
                    } else if (*gGameInfo).data[(6 as libc::c_int *
                                                     6 as libc::c_int *
                                                     16 as libc::c_int +
                                                     15 as libc::c_int) as
                                                    usize] as libc::c_int ==
                                  0x10 as libc::c_int {
                        Audio_PlaySoundGeneral(0x4806 as libc::c_int as u16_0,
                                               &mut D_801333D4,
                                               4 as libc::c_int as u8_0,
                                               &mut D_801333E0,
                                               &mut D_801333E0,
                                               &mut D_801333E8);
                    } else {
                        func_800BC490(globalCtx,
                                      ((*globalCtx).unk_1242B as libc::c_int ^
                                           3 as libc::c_int) as s16);
                    }
                }
                func_800BC450(globalCtx);
            }
            if 1 as libc::c_int != 0 &&
                   (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          63 as libc::c_int) as usize] as
                       libc::c_int != 0 {
                LogUtils_LogThreadId(b"../z_play.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     3708 as libc::c_int);
                osSyncPrintf(b"1 = %d\n\x00" as *const u8 as
                                 *const libc::c_char, 1 as libc::c_int);
            }
            SkyboxDraw_Update(&mut (*globalCtx).skyboxCtx);
            if 1 as libc::c_int != 0 &&
                   (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          63 as libc::c_int) as usize] as
                       libc::c_int != 0 {
                LogUtils_LogThreadId(b"../z_play.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     3716 as libc::c_int);
                osSyncPrintf(b"1 = %d\n\x00" as *const u8 as
                                 *const libc::c_char, 1 as libc::c_int);
            }
            if (*globalCtx).pauseCtx.state as libc::c_int != 0 as libc::c_int
                   ||
                   (*globalCtx).pauseCtx.debugState as libc::c_int !=
                       0 as libc::c_int {
                if 1 as libc::c_int != 0 &&
                       (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int
                                              * 16 as libc::c_int +
                                              63 as libc::c_int) as usize] as
                           libc::c_int != 0 {
                    LogUtils_LogThreadId(b"../z_play.c\x00" as *const u8 as
                                             *const libc::c_char,
                                         3721 as libc::c_int);
                    osSyncPrintf(b"1 = %d\n\x00" as *const u8 as
                                     *const libc::c_char, 1 as libc::c_int);
                }
                KaleidoScopeCall_Update(globalCtx);
            } else if (*globalCtx).gameOverCtx.state as libc::c_int !=
                          GAMEOVER_INACTIVE as libc::c_int {
                if 1 as libc::c_int != 0 &&
                       (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int
                                              * 16 as libc::c_int +
                                              63 as libc::c_int) as usize] as
                           libc::c_int != 0 {
                    LogUtils_LogThreadId(b"../z_play.c\x00" as *const u8 as
                                             *const libc::c_char,
                                         3727 as libc::c_int);
                    osSyncPrintf(b"1 = %d\n\x00" as *const u8 as
                                     *const libc::c_char, 1 as libc::c_int);
                }
                GameOver_Update(globalCtx);
            } else {
                if 1 as libc::c_int != 0 &&
                       (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int
                                              * 16 as libc::c_int +
                                              63 as libc::c_int) as usize] as
                           libc::c_int != 0 {
                    LogUtils_LogThreadId(b"../z_play.c\x00" as *const u8 as
                                             *const libc::c_char,
                                         3733 as libc::c_int);
                    osSyncPrintf(b"1 = %d\n\x00" as *const u8 as
                                     *const libc::c_char, 1 as libc::c_int);
                }
                Message_Update(globalCtx);
            }
            if 1 as libc::c_int != 0 &&
                   (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          63 as libc::c_int) as usize] as
                       libc::c_int != 0 {
                LogUtils_LogThreadId(b"../z_play.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     3737 as libc::c_int);
                osSyncPrintf(b"1 = %d\n\x00" as *const u8 as
                                 *const libc::c_char, 1 as libc::c_int);
            }
            if 1 as libc::c_int != 0 &&
                   (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          63 as libc::c_int) as usize] as
                       libc::c_int != 0 {
                LogUtils_LogThreadId(b"../z_play.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     3742 as libc::c_int);
                osSyncPrintf(b"1 = %d\n\x00" as *const u8 as
                                 *const libc::c_char, 1 as libc::c_int);
            }
            Interface_Update(globalCtx);
            if 1 as libc::c_int != 0 &&
                   (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          63 as libc::c_int) as usize] as
                       libc::c_int != 0 {
                LogUtils_LogThreadId(b"../z_play.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     3765 as libc::c_int);
                osSyncPrintf(b"1 = %d\n\x00" as *const u8 as
                                 *const libc::c_char, 1 as libc::c_int);
            }
            AnimationContext_Update(globalCtx,
                                    &mut (*globalCtx).animationCtx);
            if 1 as libc::c_int != 0 &&
                   (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          63 as libc::c_int) as usize] as
                       libc::c_int != 0 {
                LogUtils_LogThreadId(b"../z_play.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     3771 as libc::c_int);
                osSyncPrintf(b"1 = %d\n\x00" as *const u8 as
                                 *const libc::c_char, 1 as libc::c_int);
            }
            func_8006BA30(globalCtx);
            if 1 as libc::c_int != 0 &&
                   (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          63 as libc::c_int) as usize] as
                       libc::c_int != 0 {
                LogUtils_LogThreadId(b"../z_play.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     3777 as libc::c_int);
                osSyncPrintf(b"1 = %d\n\x00" as *const u8 as
                                 *const libc::c_char, 1 as libc::c_int);
            }
            ShrinkWindow_Update((*gGameInfo).data[(1 as libc::c_int *
                                                       6 as libc::c_int *
                                                       16 as libc::c_int +
                                                       30 as libc::c_int) as
                                                      usize] as s32);
            if 1 as libc::c_int != 0 &&
                   (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          63 as libc::c_int) as usize] as
                       libc::c_int != 0 {
                LogUtils_LogThreadId(b"../z_play.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     3783 as libc::c_int);
                osSyncPrintf(b"1 = %d\n\x00" as *const u8 as
                                 *const libc::c_char, 1 as libc::c_int);
            }
            TransitionFade_Update(&mut (*globalCtx).transitionFade as
                                      *mut TransitionFade as
                                      *mut libc::c_void,
                                  (*gGameInfo).data[(1 as libc::c_int *
                                                         6 as libc::c_int *
                                                         16 as libc::c_int +
                                                         30 as libc::c_int) as
                                                        usize] as s32);
            current_block = 17297573105595541569;
        } else { current_block = 1627774067207367960; }
    } else { current_block = 17297573105595541569; }
    match current_block {
        17297573105595541569 => {
            if 1 as libc::c_int != 0 &&
                   (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          63 as libc::c_int) as usize] as
                       libc::c_int != 0 {
                LogUtils_LogThreadId(b"../z_play.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     3799 as libc::c_int);
                osSyncPrintf(b"1 = %d\n\x00" as *const u8 as
                                 *const libc::c_char, 1 as libc::c_int);
            }
        }
        _ => { }
    }
    if 1 as libc::c_int != 0 &&
           (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 63 as libc::c_int) as
                                 usize] as libc::c_int != 0 {
        LogUtils_LogThreadId(b"../z_play.c\x00" as *const u8 as
                                 *const libc::c_char, 3801 as libc::c_int);
        osSyncPrintf(b"1 = %d\n\x00" as *const u8 as *const libc::c_char,
                     1 as libc::c_int);
    }
    if sp80 == 0 as libc::c_int || gDbgCamEnabled != 0 {
        let mut pad3: [s32; 5] = [0; 5];
        let mut i_0: s32 = 0;
        (*globalCtx).nextCamera = (*globalCtx).activeCamera;
        if 1 as libc::c_int != 0 &&
               (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                      16 as libc::c_int + 63 as libc::c_int)
                                     as usize] as libc::c_int != 0 {
            LogUtils_LogThreadId(b"../z_play.c\x00" as *const u8 as
                                     *const libc::c_char,
                                 3806 as libc::c_int);
            osSyncPrintf(b"1 = %d\n\x00" as *const u8 as *const libc::c_char,
                         1 as libc::c_int);
        }
        i_0 = 0 as libc::c_int;
        while i_0 < 4 as libc::c_int {
            if i_0 != (*globalCtx).nextCamera as libc::c_int &&
                   !(*globalCtx).cameraPtrs[i_0 as usize].is_null() {
                if 1 as libc::c_int != 0 &&
                       (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int
                                              * 16 as libc::c_int +
                                              63 as libc::c_int) as usize] as
                           libc::c_int != 0 {
                    LogUtils_LogThreadId(b"../z_play.c\x00" as *const u8 as
                                             *const libc::c_char,
                                         3809 as libc::c_int);
                    osSyncPrintf(b"1 = %d\n\x00" as *const u8 as
                                     *const libc::c_char, 1 as libc::c_int);
                }
                Camera_Update((*globalCtx).cameraPtrs[i_0 as usize]);
            }
            i_0 += 1
        }
        Camera_Update((*globalCtx).cameraPtrs[(*globalCtx).nextCamera as
                                                  usize]);
        if 1 as libc::c_int != 0 &&
               (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                      16 as libc::c_int + 63 as libc::c_int)
                                     as usize] as libc::c_int != 0 {
            LogUtils_LogThreadId(b"../z_play.c\x00" as *const u8 as
                                     *const libc::c_char,
                                 3814 as libc::c_int);
            osSyncPrintf(b"1 = %d\n\x00" as *const u8 as *const libc::c_char,
                         1 as libc::c_int);
        }
    }
    if 1 as libc::c_int != 0 &&
           (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 63 as libc::c_int) as
                                 usize] as libc::c_int != 0 {
        LogUtils_LogThreadId(b"../z_play.c\x00" as *const u8 as
                                 *const libc::c_char, 3816 as libc::c_int);
        osSyncPrintf(b"1 = %d\n\x00" as *const u8 as *const libc::c_char,
                     1 as libc::c_int);
    }
    Environment_Update(globalCtx, &mut (*globalCtx).envCtx,
                       &mut (*globalCtx).lightCtx, &mut (*globalCtx).pauseCtx,
                       &mut (*globalCtx).msgCtx,
                       &mut (*globalCtx).gameOverCtx,
                       (*globalCtx).state.gfxCtx);
}
#[no_mangle]
pub unsafe extern "C" fn Gameplay_DrawOverlayElements(mut globalCtx:
                                                          *mut GlobalContext) {
    if (*globalCtx).pauseCtx.state as libc::c_int != 0 as libc::c_int ||
           (*globalCtx).pauseCtx.debugState as libc::c_int != 0 as libc::c_int
       {
        KaleidoScopeCall_Draw(globalCtx);
    }
    if gSaveContext.gameMode == 0 as libc::c_int {
        Interface_Draw(globalCtx);
    }
    Message_Draw(globalCtx);
    if (*globalCtx).gameOverCtx.state as libc::c_int !=
           GAMEOVER_INACTIVE as libc::c_int {
        GameOver_FadeInLights(globalCtx);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Gameplay_Draw(mut globalCtx: *mut GlobalContext) {
    let mut sp80: s32 = 0;
    let mut current_block: u64;
    let mut gfxCtx: *mut GraphicsContext = (*globalCtx).state.gfxCtx;
    let mut sp228: *mut Lights = 0 as *mut Lights;
    let mut sp21C: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), gfxCtx,
                    b"../z_play.c\x00" as *const u8 as *const libc::c_char,
                    3907 as libc::c_int);
    gSegments[4 as libc::c_int as usize] =
        ((*globalCtx).objectCtx.status[(*globalCtx).objectCtx.mainKeepIndex as
                                           usize].segment as
             *mut u8_0).offset(-(0x80000000 as libc::c_uint as isize)) as
            u32_0;
    gSegments[5 as libc::c_int as usize] =
        ((*globalCtx).objectCtx.status[(*globalCtx).objectCtx.subKeepIndex as
                                           usize].segment as
             *mut u8_0).offset(-(0x80000000 as libc::c_uint as isize)) as
            u32_0;
    gSegments[2 as libc::c_int as usize] =
        ((*globalCtx).sceneSegment as
             *mut u8_0).offset(-(0x80000000 as libc::c_uint as isize)) as
            u32_0;
    let fresh1 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g: *mut Gfx = fresh1;
    (*_g).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((0 as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g).words.w1 = 0 as *mut libc::c_void as libc::c_uint;
    let fresh2 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_0: *mut Gfx = fresh2;
    (*_g_0).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((0 as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_0).words.w1 = 0 as *mut libc::c_void as libc::c_uint;
    let fresh3 = (*__gfxCtx).overlay.p;
    (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
    let mut _g_1: *mut Gfx = fresh3;
    (*_g_1).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((0 as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_1).words.w1 = 0 as *mut libc::c_void as libc::c_uint;
    let fresh4 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_2: *mut Gfx = fresh4;
    (*_g_2).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((0x4 as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_2).words.w1 =
        (*globalCtx).objectCtx.status[(*globalCtx).objectCtx.mainKeepIndex as
                                          usize].segment as libc::c_uint;
    let fresh5 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_3: *mut Gfx = fresh5;
    (*_g_3).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((0x4 as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_3).words.w1 =
        (*globalCtx).objectCtx.status[(*globalCtx).objectCtx.mainKeepIndex as
                                          usize].segment as libc::c_uint;
    let fresh6 = (*__gfxCtx).overlay.p;
    (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
    let mut _g_4: *mut Gfx = fresh6;
    (*_g_4).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((0x4 as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_4).words.w1 =
        (*globalCtx).objectCtx.status[(*globalCtx).objectCtx.mainKeepIndex as
                                          usize].segment as libc::c_uint;
    let fresh7 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_5: *mut Gfx = fresh7;
    (*_g_5).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((0x5 as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_5).words.w1 =
        (*globalCtx).objectCtx.status[(*globalCtx).objectCtx.subKeepIndex as
                                          usize].segment as libc::c_uint;
    let fresh8 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_6: *mut Gfx = fresh8;
    (*_g_6).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((0x5 as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_6).words.w1 =
        (*globalCtx).objectCtx.status[(*globalCtx).objectCtx.subKeepIndex as
                                          usize].segment as libc::c_uint;
    let fresh9 = (*__gfxCtx).overlay.p;
    (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
    let mut _g_7: *mut Gfx = fresh9;
    (*_g_7).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((0x5 as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_7).words.w1 =
        (*globalCtx).objectCtx.status[(*globalCtx).objectCtx.subKeepIndex as
                                          usize].segment as libc::c_uint;
    let fresh10 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_8: *mut Gfx = fresh10;
    (*_g_8).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((0x2 as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_8).words.w1 = (*globalCtx).sceneSegment as libc::c_uint;
    let fresh11 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_9: *mut Gfx = fresh11;
    (*_g_9).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((0x2 as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_9).words.w1 = (*globalCtx).sceneSegment as libc::c_uint;
    let fresh12 = (*__gfxCtx).overlay.p;
    (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
    let mut _g_10: *mut Gfx = fresh12;
    (*_g_10).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((0x2 as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_10).words.w1 = (*globalCtx).sceneSegment as libc::c_uint;
    func_80095248(gfxCtx, 0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
                  0 as libc::c_int as u8_0);
    if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 80 as libc::c_int) as usize]
           as libc::c_int != 10 as libc::c_int ||
           (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 82 as libc::c_int) as
                                 usize] as libc::c_int != 0 as libc::c_int {
        (*__gfxCtx).polyOpa.p =
            Gameplay_SetFog(globalCtx, (*__gfxCtx).polyOpa.p);
        (*__gfxCtx).polyXlu.p =
            Gameplay_SetFog(globalCtx, (*__gfxCtx).polyXlu.p);
        func_800AA460(&mut (*globalCtx).view, (*globalCtx).view.fovy,
                      (*globalCtx).view.zNear,
                      (*globalCtx).lightCtx.fogFar as f32_0);
        func_800AAA50(&mut (*globalCtx).view, 15 as libc::c_int);
        // The billboard matrix temporarily stores the viewing matrix
        Matrix_MtxToMtxF(&mut (*globalCtx).view.viewing,
                         &mut (*globalCtx).billboardMtxF);
        Matrix_MtxToMtxF(&mut (*globalCtx).view.projection,
                         &mut (*globalCtx).viewProjectionMtxF);
        Matrix_Mult(&mut (*globalCtx).viewProjectionMtxF,
                    MTXMODE_NEW as libc::c_int as u8_0);
        // The billboard is still a viewing matrix at this stage
        Matrix_Mult(&mut (*globalCtx).billboardMtxF,
                    MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_Get(&mut (*globalCtx).viewProjectionMtxF);
        (*globalCtx).billboardMtxF.mf[3 as libc::c_int as
                                          usize][2 as libc::c_int as usize] =
            0.0f32;
        (*globalCtx).billboardMtxF.mf[3 as libc::c_int as
                                          usize][1 as libc::c_int as usize] =
            (*globalCtx).billboardMtxF.mf[3 as libc::c_int as
                                              usize][2 as libc::c_int as
                                                         usize];
        (*globalCtx).billboardMtxF.mf[3 as libc::c_int as
                                          usize][0 as libc::c_int as usize] =
            (*globalCtx).billboardMtxF.mf[3 as libc::c_int as
                                              usize][1 as libc::c_int as
                                                         usize];
        (*globalCtx).billboardMtxF.mf[2 as libc::c_int as
                                          usize][3 as libc::c_int as usize] =
            (*globalCtx).billboardMtxF.mf[3 as libc::c_int as
                                              usize][0 as libc::c_int as
                                                         usize];
        (*globalCtx).billboardMtxF.mf[1 as libc::c_int as
                                          usize][3 as libc::c_int as usize] =
            (*globalCtx).billboardMtxF.mf[2 as libc::c_int as
                                              usize][3 as libc::c_int as
                                                         usize];
        (*globalCtx).billboardMtxF.mf[0 as libc::c_int as
                                          usize][3 as libc::c_int as usize] =
            (*globalCtx).billboardMtxF.mf[1 as libc::c_int as
                                              usize][3 as libc::c_int as
                                                         usize];
        // This transpose is where the viewing matrix is properly converted into a billboard matrix
        Matrix_Transpose(&mut (*globalCtx).billboardMtxF);
        (*globalCtx).billboardMtx =
            Matrix_MtxFToMtx(Matrix_CheckFloats(&mut (*globalCtx).billboardMtxF,
                                                b"../z_play.c\x00" as
                                                    *const u8 as
                                                    *const libc::c_char as
                                                    *mut libc::c_char,
                                                4005 as libc::c_int),
                             Graph_Alloc(gfxCtx,
                                         ::std::mem::size_of::<Mtx>() as
                                             libc::c_ulong as size_t) as
                                 *mut Mtx);
        let fresh13 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_11: *mut Gfx = fresh13;
        (*_g_11).words.w0 =
            (0xdb as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0x6 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                ((0x1 as libc::c_int * 4 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_11).words.w1 = (*globalCtx).billboardMtx as libc::c_uint;
        if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 80 as libc::c_int) as
                                 usize] as libc::c_int != 10 as libc::c_int ||
               (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                      16 as libc::c_int + 92 as libc::c_int)
                                     as usize] as libc::c_int !=
                   0 as libc::c_int {
            let mut gfxP: *mut Gfx = 0 as *mut Gfx;
            let mut sp1CC: *mut Gfx = (*__gfxCtx).polyOpa.p;
            gfxP = Graph_GfxPlusOne(sp1CC);
            let fresh14 = (*__gfxCtx).overlay.p;
            (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
            let mut _g_12: *mut Gfx = fresh14;
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
            (*_g_12).words.w1 = gfxP as libc::c_uint;
            if (*globalCtx).transitionMode as libc::c_int == 3 as libc::c_int
                   ||
                   (*globalCtx).transitionMode as libc::c_int ==
                       11 as libc::c_int ||
                   (*globalCtx).transitionCtx.transitionType >=
                       56 as libc::c_int {
                let mut view: View =
                    View{magic: 0,
                         gfxCtx: 0 as *mut GraphicsContext,
                         viewport:
                             Viewport{topY: 0,
                                      bottomY: 0,
                                      leftX: 0,
                                      rightX: 0,},
                         fovy: 0.,
                         zNear: 0.,
                         zFar: 0.,
                         scale: 0.,
                         eye: Vec3f{x: 0., y: 0., z: 0.,},
                         lookAt: Vec3f{x: 0., y: 0., z: 0.,},
                         up: Vec3f{x: 0., y: 0., z: 0.,},
                         vp: Vp{vp: Vp_t{vscale: [0; 4], vtrans: [0; 4],},},
                         projection: Mtx{m: [[0; 4]; 4],},
                         viewing: Mtx{m: [[0; 4]; 4],},
                         projectionPtr: 0 as *mut Mtx,
                         viewingPtr: 0 as *mut Mtx,
                         unk_E8: Vec3f{x: 0., y: 0., z: 0.,},
                         unk_F4: Vec3f{x: 0., y: 0., z: 0.,},
                         unk_100: 0.,
                         unk_104: Vec3f{x: 0., y: 0., z: 0.,},
                         unk_110: Vec3f{x: 0., y: 0., z: 0.,},
                         normal: 0,
                         flags: 0,
                         unk_124: 0,};
                View_Init(&mut view, gfxCtx);
                view.flags = 2 as libc::c_int | 8 as libc::c_int;
                let mut viewport: Viewport =
                    Viewport{topY: 0, bottomY: 0, leftX: 0, rightX: 0,};
                viewport.bottomY = 240 as libc::c_int;
                viewport.rightX = 320 as libc::c_int;
                viewport.topY = 0 as libc::c_int;
                viewport.leftX = 0 as libc::c_int;
                View_SetViewport(&mut view, &mut viewport);
                func_800AB9EC(&mut view, 15 as libc::c_int, &mut gfxP);
                (*globalCtx).transitionCtx.draw.expect("non-null function pointer")(&mut (*globalCtx).transitionCtx.c2rust_unnamed.data
                                                                                        as
                                                                                        *mut [libc::c_char; 552]
                                                                                        as
                                                                                        *mut libc::c_void,
                                                                                    &mut gfxP);
            }
            TransitionFade_Draw(&mut (*globalCtx).transitionFade as
                                    *mut TransitionFade as *mut libc::c_void,
                                &mut gfxP);
            if D_801614B0.c2rust_unnamed.a as libc::c_int > 0 as libc::c_int {
                D_80161498.primColor.rgba = D_801614B0.rgba;
                VisMono_Draw(&mut D_80161498, &mut gfxP);
            }
            let fresh15 = gfxP;
            gfxP = gfxP.offset(1);
            let mut _g_13: *mut Gfx = fresh15;
            (*_g_13).words.w0 =
                (0xdf as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_13).words.w1 = 0 as libc::c_int as libc::c_uint;
            Graph_BranchDlist(sp1CC, gfxP);
            (*__gfxCtx).polyOpa.p = gfxP
        }
        if gTrnsnUnkState == 3 as libc::c_int {
            let mut sp88: *mut Gfx = (*__gfxCtx).polyOpa.p;
            TransitionUnk_Draw(&mut sTrnsnUnk, &mut sp88);
            (*__gfxCtx).polyOpa.p = sp88;
            current_block = 7758133914218248493;
        } else {
            PreRender_SetValues(&mut (*globalCtx).pauseBgPreRender,
                                320 as libc::c_int as u32_0,
                                240 as libc::c_int as u32_0,
                                (*gfxCtx).curFrameBuffer as *mut libc::c_void,
                                gZBuffer.as_mut_ptr() as *mut libc::c_void);
            if (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                      16 as libc::c_int + 94 as libc::c_int)
                                     as usize] as libc::c_int ==
                   2 as libc::c_int {
                MsgEvent_SendNullTask();
                PreRender_Calc(&mut (*globalCtx).pauseBgPreRender);
                (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                       16 as libc::c_int + 94 as libc::c_int)
                                      as usize] = 3 as libc::c_int as s16
            } else if (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                             16 as libc::c_int +
                                             94 as libc::c_int) as usize] as
                          libc::c_int >= 4 as libc::c_int {
                (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                       16 as libc::c_int + 94 as libc::c_int)
                                      as usize] = 0 as libc::c_int as s16
            }
            if (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                      16 as libc::c_int + 94 as libc::c_int)
                                     as usize] as libc::c_int ==
                   3 as libc::c_int {
                let mut sp84: *mut Gfx = (*__gfxCtx).polyOpa.p;
                func_800C24BC(&mut (*globalCtx).pauseBgPreRender, &mut sp84);
                (*__gfxCtx).polyOpa.p = sp84;
                current_block = 7758133914218248493;
            } else {
                sp80 = 0;
                if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          80 as libc::c_int) as usize] as
                       libc::c_int != 10 as libc::c_int ||
                       (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int
                                              * 16 as libc::c_int +
                                              83 as libc::c_int) as usize] as
                           libc::c_int != 0 as libc::c_int {
                    if (*globalCtx).skyboxId as libc::c_int != 0 &&
                           (*globalCtx).skyboxId as libc::c_int !=
                               SKYBOX_UNSET_1D as libc::c_int &&
                           (*globalCtx).envCtx.skyboxDisabled == 0 {
                        if (*globalCtx).skyboxId as libc::c_int ==
                               SKYBOX_NORMAL_SKY as libc::c_int ||
                               (*globalCtx).skyboxId as libc::c_int ==
                                   SKYBOX_CUTSCENE_MAP as libc::c_int {
                            Environment_UpdateSkybox((*globalCtx).skyboxId,
                                                     &mut (*globalCtx).envCtx,
                                                     &mut (*globalCtx).skyboxCtx);
                            SkyboxDraw_Draw(&mut (*globalCtx).skyboxCtx,
                                            gfxCtx,
                                            (*globalCtx).skyboxId as s16,
                                            (*globalCtx).envCtx.skyboxBlend as
                                                s16, (*globalCtx).view.eye.x,
                                            (*globalCtx).view.eye.y,
                                            (*globalCtx).view.eye.z);
                        } else if (*globalCtx).skyboxCtx.unk_140 as
                                      libc::c_int == 0 as libc::c_int {
                            SkyboxDraw_Draw(&mut (*globalCtx).skyboxCtx,
                                            gfxCtx,
                                            (*globalCtx).skyboxId as s16,
                                            0 as libc::c_int as s16,
                                            (*globalCtx).view.eye.x,
                                            (*globalCtx).view.eye.y,
                                            (*globalCtx).view.eye.z);
                        }
                    }
                }
                if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          80 as libc::c_int) as usize] as
                       libc::c_int != 10 as libc::c_int ||
                       (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int
                                              * 16 as libc::c_int +
                                              90 as libc::c_int) as usize] as
                           libc::c_int & 2 as libc::c_int != 0 {
                    if (*globalCtx).envCtx.sunMoonDisabled == 0 {
                        Environment_DrawSunAndMoon(globalCtx);
                    }
                }
                if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          80 as libc::c_int) as usize] as
                       libc::c_int != 10 as libc::c_int ||
                       (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int
                                              * 16 as libc::c_int +
                                              90 as libc::c_int) as usize] as
                           libc::c_int & 1 as libc::c_int != 0 {
                    Environment_DrawSkyboxFilters(globalCtx);
                }
                if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          80 as libc::c_int) as usize] as
                       libc::c_int != 10 as libc::c_int ||
                       (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int
                                              * 16 as libc::c_int +
                                              90 as libc::c_int) as usize] as
                           libc::c_int & 4 as libc::c_int != 0 {
                    Environment_UpdateLightningStrike(globalCtx);
                    Environment_DrawLightning(globalCtx, 0 as libc::c_int);
                }
                if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          80 as libc::c_int) as usize] as
                       libc::c_int != 10 as libc::c_int ||
                       (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int
                                              * 16 as libc::c_int +
                                              90 as libc::c_int) as usize] as
                           libc::c_int & 8 as libc::c_int != 0 {
                    sp228 =
                        LightContext_NewLights(&mut (*globalCtx).lightCtx,
                                               gfxCtx);
                    Lights_BindAll(sp228, (*globalCtx).lightCtx.listHead,
                                   0 as *mut Vec3f);
                    Lights_Draw(sp228, gfxCtx);
                }
                if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          80 as libc::c_int) as usize] as
                       libc::c_int != 10 as libc::c_int ||
                       (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int
                                              * 16 as libc::c_int +
                                              84 as libc::c_int) as usize] as
                           libc::c_int != 0 as libc::c_int {
                    if (*gGameInfo).data[(20 as libc::c_int * 6 as libc::c_int
                                              * 16 as libc::c_int +
                                              94 as libc::c_int) as usize] as
                           libc::c_int == 0 as libc::c_int {
                        if (*gGameInfo).data[(21 as libc::c_int *
                                                  6 as libc::c_int *
                                                  16 as libc::c_int +
                                                  80 as libc::c_int) as usize]
                               as libc::c_int != 10 as libc::c_int {
                            sp80 = 3 as libc::c_int
                        } else {
                            sp80 =
                                (*gGameInfo).data[(21 as libc::c_int *
                                                       6 as libc::c_int *
                                                       16 as libc::c_int +
                                                       84 as libc::c_int) as
                                                      usize] as s32
                        }
                        Scene_Draw(globalCtx);
                        Room_Draw(globalCtx,
                                  &mut (*globalCtx).roomCtx.curRoom,
                                  (sp80 & 3 as libc::c_int) as u32_0);
                        Room_Draw(globalCtx,
                                  &mut (*globalCtx).roomCtx.prevRoom,
                                  (sp80 & 3 as libc::c_int) as u32_0);
                    }
                }
                if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          80 as libc::c_int) as usize] as
                       libc::c_int != 10 as libc::c_int ||
                       (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int
                                              * 16 as libc::c_int +
                                              83 as libc::c_int) as usize] as
                           libc::c_int != 0 as libc::c_int {
                    if (*globalCtx).skyboxCtx.unk_140 as libc::c_int !=
                           0 as libc::c_int &&
                           (*(*globalCtx).cameraPtrs[(*globalCtx).activeCamera
                                                         as usize]).setting as
                               libc::c_int !=
                               CAM_SET_PREREND_FIXED as libc::c_int {
                        let mut sp74: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                        Camera_GetSkyboxOffset(&mut sp74,
                                               (*globalCtx).cameraPtrs[(*globalCtx).activeCamera
                                                                           as
                                                                           usize]);
                        SkyboxDraw_Draw(&mut (*globalCtx).skyboxCtx, gfxCtx,
                                        (*globalCtx).skyboxId as s16,
                                        0 as libc::c_int as s16,
                                        (*globalCtx).view.eye.x + sp74.x,
                                        (*globalCtx).view.eye.y + sp74.y,
                                        (*globalCtx).view.eye.z + sp74.z);
                    }
                }
                if (*globalCtx).envCtx.unk_EE[1 as libc::c_int as usize] as
                       libc::c_int != 0 as libc::c_int {
                    Environment_DrawRain(globalCtx, &mut (*globalCtx).view,
                                         gfxCtx);
                }
                if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          80 as libc::c_int) as usize] as
                       libc::c_int != 10 as libc::c_int ||
                       (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int
                                              * 16 as libc::c_int +
                                              84 as libc::c_int) as usize] as
                           libc::c_int != 0 as libc::c_int {
                    Environment_FillScreen(gfxCtx, 0 as libc::c_int as u8_0,
                                           0 as libc::c_int as u8_0,
                                           0 as libc::c_int as u8_0,
                                           (*globalCtx).unk_11E18 as u8_0,
                                           ((1 as libc::c_int) <<
                                                0 as libc::c_int) as u8_0);
                }
                if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          80 as libc::c_int) as usize] as
                       libc::c_int != 10 as libc::c_int ||
                       (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int
                                              * 16 as libc::c_int +
                                              85 as libc::c_int) as usize] as
                           libc::c_int != 0 as libc::c_int {
                    func_800315AC(globalCtx, &mut (*globalCtx).actorCtx);
                }
                if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          80 as libc::c_int) as usize] as
                       libc::c_int != 10 as libc::c_int ||
                       (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int
                                              * 16 as libc::c_int +
                                              86 as libc::c_int) as usize] as
                           libc::c_int != 0 as libc::c_int {
                    if (*globalCtx).envCtx.sunMoonDisabled == 0 {
                        sp21C.x =
                            (*globalCtx).view.eye.x +
                                (*globalCtx).envCtx.sunPos.x;
                        sp21C.y =
                            (*globalCtx).view.eye.y +
                                (*globalCtx).envCtx.sunPos.y;
                        sp21C.z =
                            (*globalCtx).view.eye.z +
                                (*globalCtx).envCtx.sunPos.z;
                        Environment_DrawSunLensFlare(globalCtx,
                                                     &mut (*globalCtx).envCtx,
                                                     &mut (*globalCtx).view,
                                                     gfxCtx, sp21C,
                                                     0 as libc::c_int);
                    }
                    Environment_DrawCustomLensFlare(globalCtx);
                }
                if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          80 as libc::c_int) as usize] as
                       libc::c_int != 10 as libc::c_int ||
                       (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int
                                              * 16 as libc::c_int +
                                              87 as libc::c_int) as usize] as
                           libc::c_int != 0 as libc::c_int {
                    if (*gGameInfo).data[(5 as libc::c_int * 6 as libc::c_int
                                              * 16 as libc::c_int +
                                              64 as libc::c_int) as usize] as
                           libc::c_int != 0 as libc::c_int {
                        Environment_FillScreen(gfxCtx,
                                               (*gGameInfo).data[(5 as
                                                                      libc::c_int
                                                                      *
                                                                      6 as
                                                                          libc::c_int
                                                                      *
                                                                      16 as
                                                                          libc::c_int
                                                                      +
                                                                      65 as
                                                                          libc::c_int)
                                                                     as usize]
                                                   as u8_0,
                                               (*gGameInfo).data[(5 as
                                                                      libc::c_int
                                                                      *
                                                                      6 as
                                                                          libc::c_int
                                                                      *
                                                                      16 as
                                                                          libc::c_int
                                                                      +
                                                                      66 as
                                                                          libc::c_int)
                                                                     as usize]
                                                   as u8_0,
                                               (*gGameInfo).data[(5 as
                                                                      libc::c_int
                                                                      *
                                                                      6 as
                                                                          libc::c_int
                                                                      *
                                                                      16 as
                                                                          libc::c_int
                                                                      +
                                                                      67 as
                                                                          libc::c_int)
                                                                     as usize]
                                                   as u8_0,
                                               (*gGameInfo).data[(5 as
                                                                      libc::c_int
                                                                      *
                                                                      6 as
                                                                          libc::c_int
                                                                      *
                                                                      16 as
                                                                          libc::c_int
                                                                      +
                                                                      68 as
                                                                          libc::c_int)
                                                                     as usize]
                                                   as u8_0,
                                               ((1 as libc::c_int) <<
                                                    0 as libc::c_int |
                                                    (1 as libc::c_int) <<
                                                        1 as libc::c_int) as
                                                   u8_0);
                    }
                    match (*globalCtx).envCtx.fillScreen as libc::c_int {
                        1 => {
                            Environment_FillScreen(gfxCtx,
                                                   (*globalCtx).envCtx.screenFillColor[0
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           usize],
                                                   (*globalCtx).envCtx.screenFillColor[1
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           usize],
                                                   (*globalCtx).envCtx.screenFillColor[2
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           usize],
                                                   (*globalCtx).envCtx.screenFillColor[3
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           usize],
                                                   ((1 as libc::c_int) <<
                                                        0 as libc::c_int |
                                                        (1 as libc::c_int) <<
                                                            1 as libc::c_int)
                                                       as u8_0);
                        }
                        _ => { }
                    }
                }
                if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          80 as libc::c_int) as usize] as
                       libc::c_int != 10 as libc::c_int ||
                       (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int
                                              * 16 as libc::c_int +
                                              88 as libc::c_int) as usize] as
                           libc::c_int != 0 as libc::c_int {
                    if (*globalCtx).envCtx.sandstormState as libc::c_int !=
                           0 as libc::c_int {
                        Environment_DrawSandstorm(globalCtx,
                                                  (*globalCtx).envCtx.sandstormState);
                    }
                }
                if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          80 as libc::c_int) as usize] as
                       libc::c_int != 10 as libc::c_int ||
                       (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int
                                              * 16 as libc::c_int +
                                              93 as libc::c_int) as usize] as
                           libc::c_int != 0 as libc::c_int {
                    DebugDisplay_DrawObjects(globalCtx);
                }
                if (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          94 as libc::c_int) as usize] as
                       libc::c_int == 1 as libc::c_int ||
                       gTrnsnUnkState == 1 as libc::c_int {
                    let mut sp70: *mut Gfx = (*__gfxCtx).overlay.p;
                    (*globalCtx).pauseBgPreRender.fbuf =
                        (*gfxCtx).curFrameBuffer;
                    (*globalCtx).pauseBgPreRender.fbufSave =
                        gZBuffer.as_mut_ptr() as *mut u16_0;
                    func_800C1F20(&mut (*globalCtx).pauseBgPreRender,
                                  &mut sp70);
                    if (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int
                                              * 16 as libc::c_int +
                                              94 as libc::c_int) as usize] as
                           libc::c_int == 1 as libc::c_int {
                        (*globalCtx).pauseBgPreRender.cvgSave =
                            (*gfxCtx).curFrameBuffer as *mut u8_0;
                        func_800C20B4(&mut (*globalCtx).pauseBgPreRender,
                                      &mut sp70);
                        (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int
                                               * 16 as libc::c_int +
                                               94 as libc::c_int) as usize] =
                            2 as libc::c_int as s16
                    } else { gTrnsnUnkState = 2 as libc::c_int }
                    (*__gfxCtx).overlay.p = sp70;
                    (*globalCtx).unk_121C7 = 2 as libc::c_int as s8;
                    (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                           16 as libc::c_int +
                                           33 as libc::c_int) as usize] =
                        ((*gGameInfo).data[(1 as libc::c_int *
                                                6 as libc::c_int *
                                                16 as libc::c_int +
                                                33 as libc::c_int) as usize]
                             as libc::c_int | 1 as libc::c_int) as s16;
                    current_block = 4533671380017093834;
                } else { current_block = 7758133914218248493; }
            }
        }
        match current_block {
            4533671380017093834 => { }
            _ => {
                if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          80 as libc::c_int) as usize] as
                       libc::c_int != 10 as libc::c_int ||
                       (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int
                                              * 16 as libc::c_int +
                                              89 as libc::c_int) as usize] as
                           libc::c_int != 0 as libc::c_int {
                    Gameplay_DrawOverlayElements(globalCtx);
                }
            }
        }
    }
    if (*globalCtx).view.unk_124 != 0 as libc::c_int {
        Camera_Update((*globalCtx).cameraPtrs[(*globalCtx).activeCamera as
                                                  usize]);
        func_800AB944(&mut (*globalCtx).view);
        (*globalCtx).view.unk_124 = 0 as libc::c_int;
        if (*globalCtx).skyboxId as libc::c_int != 0 &&
               (*globalCtx).skyboxId as libc::c_int !=
                   SKYBOX_UNSET_1D as libc::c_int &&
               (*globalCtx).envCtx.skyboxDisabled == 0 {
            SkyboxDraw_UpdateMatrix(&mut (*globalCtx).skyboxCtx,
                                    (*globalCtx).view.eye.x,
                                    (*globalCtx).view.eye.y,
                                    (*globalCtx).view.eye.z);
        }
    }
    Camera_Finish((*globalCtx).cameraPtrs[(*globalCtx).activeCamera as
                                              usize]);
    Graph_CloseDisps(dispRefs.as_mut_ptr(), gfxCtx,
                     b"../z_play.c\x00" as *const u8 as *const libc::c_char,
                     4508 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Gameplay_Main(mut thisx: *mut GameState) {
    let mut globalCtx: *mut GlobalContext = thisx as *mut GlobalContext;
    D_8012D1F8 =
        &mut *(*globalCtx).state.input.as_mut_ptr().offset(0 as libc::c_int as
                                                               isize) as
            *mut Input;
    DebugDisplay_Init();
    if 1 as libc::c_int != 0 &&
           (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 63 as libc::c_int) as
                                 usize] as libc::c_int != 0 {
        LogUtils_LogThreadId(b"../z_play.c\x00" as *const u8 as
                                 *const libc::c_char, 4556 as libc::c_int);
        osSyncPrintf(b"1 = %d\n\x00" as *const u8 as *const libc::c_char,
                     1 as libc::c_int);
    }
    if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 80 as libc::c_int) as usize]
           as libc::c_int == 10 as libc::c_int &&
           (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 94 as libc::c_int) as
                                 usize] as libc::c_int != 10 as libc::c_int {
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 81 as libc::c_int) as
                              usize] = 1 as libc::c_int as s16;
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 82 as libc::c_int) as
                              usize] = 1 as libc::c_int as s16;
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 83 as libc::c_int) as
                              usize] = 1 as libc::c_int as s16;
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 84 as libc::c_int) as
                              usize] = 3 as libc::c_int as s16;
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 85 as libc::c_int) as
                              usize] = 1 as libc::c_int as s16;
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 86 as libc::c_int) as
                              usize] = 1 as libc::c_int as s16;
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 87 as libc::c_int) as
                              usize] = 1 as libc::c_int as s16;
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 88 as libc::c_int) as
                              usize] = 1 as libc::c_int as s16;
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 89 as libc::c_int) as
                              usize] = 1 as libc::c_int as s16;
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 90 as libc::c_int) as
                              usize] = 15 as libc::c_int as s16;
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 91 as libc::c_int) as
                              usize] = 1 as libc::c_int as s16;
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 92 as libc::c_int) as
                              usize] = 1 as libc::c_int as s16;
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 93 as libc::c_int) as
                              usize] = 1 as libc::c_int as s16;
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 94 as libc::c_int) as
                              usize] = 10 as libc::c_int as s16
    }
    if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 80 as libc::c_int) as usize]
           as libc::c_int != 10 as libc::c_int ||
           (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 81 as libc::c_int) as
                                 usize] as libc::c_int != 0 as libc::c_int {
        Gameplay_Update(globalCtx);
    }
    if 1 as libc::c_int != 0 &&
           (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 63 as libc::c_int) as
                                 usize] as libc::c_int != 0 {
        LogUtils_LogThreadId(b"../z_play.c\x00" as *const u8 as
                                 *const libc::c_char, 4583 as libc::c_int);
        osSyncPrintf(b"1 = %d\n\x00" as *const u8 as *const libc::c_char,
                     1 as libc::c_int);
    }
    Gameplay_Draw(globalCtx);
    if 1 as libc::c_int != 0 &&
           (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 63 as libc::c_int) as
                                 usize] as libc::c_int != 0 {
        LogUtils_LogThreadId(b"../z_play.c\x00" as *const u8 as
                                 *const libc::c_char, 4587 as libc::c_int);
        osSyncPrintf(b"1 = %d\n\x00" as *const u8 as *const libc::c_char,
                     1 as libc::c_int);
    };
}
// original name: "Game_play_demo_mode_check"
#[no_mangle]
pub unsafe extern "C" fn Gameplay_InCsMode(mut globalCtx: *mut GlobalContext)
 -> s32 {
    return ((*globalCtx).csCtx.state as libc::c_int !=
                CS_STATE_IDLE as libc::c_int ||
                Player_InCsMode(globalCtx) != 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_800BFCB8(mut globalCtx: *mut GlobalContext,
                                       mut mf: *mut MtxF, mut vec: *mut Vec3f)
 -> f32_0 {
    let mut poly: CollisionPoly =
        CollisionPoly{type_0: 0,
                      c2rust_unnamed: C2RustUnnamed_5{vtxData: [0; 3],},
                      normal: Vec3s{x: 0, y: 0, z: 0,},
                      dist: 0,};
    let mut temp1: f32_0 = 0.;
    let mut temp2: f32_0 = 0.;
    let mut temp3: f32_0 = 0.;
    let mut floorY: f32_0 = 0.;
    let mut nx: f32_0 = 0.;
    let mut ny: f32_0 = 0.;
    let mut nz: f32_0 = 0.;
    let mut pad: [s32; 5] = [0; 5];
    floorY =
        BgCheck_AnyRaycastFloor1(&mut (*globalCtx).colCtx, &mut poly, vec);
    if floorY > -32000.0f32 {
        nx =
            poly.normal.x as libc::c_int as libc::c_float *
                (1.0f32 / 32767.0f32);
        ny =
            poly.normal.y as libc::c_int as libc::c_float *
                (1.0f32 / 32767.0f32);
        nz =
            poly.normal.z as libc::c_int as libc::c_float *
                (1.0f32 / 32767.0f32);
        temp1 = sqrtf(1.0f32 - nx * nx);
        if temp1 != 0.0f32 {
            temp2 = ny * temp1;
            temp3 = -nz * temp1
        } else { temp3 = 0.0f32; temp2 = 0.0f32 }
        (*mf).c2rust_unnamed.xx = temp1;
        (*mf).c2rust_unnamed.yx = -nx * temp2;
        (*mf).c2rust_unnamed.zx = nx * temp3;
        (*mf).c2rust_unnamed.xy = nx;
        (*mf).c2rust_unnamed.yy = ny;
        (*mf).c2rust_unnamed.zy = nz;
        (*mf).c2rust_unnamed.yz = temp3;
        (*mf).c2rust_unnamed.zz = temp2;
        (*mf).c2rust_unnamed.wx = 0.0f32;
        (*mf).c2rust_unnamed.wy = 0.0f32;
        (*mf).c2rust_unnamed.xz = 0.0f32;
        (*mf).c2rust_unnamed.wz = 0.0f32;
        (*mf).c2rust_unnamed.xw = (*vec).x;
        (*mf).c2rust_unnamed.yw = floorY;
        (*mf).c2rust_unnamed.zw = (*vec).z;
        (*mf).c2rust_unnamed.ww = 1.0f32
    } else {
        (*mf).c2rust_unnamed.xy = 0.0f32;
        (*mf).c2rust_unnamed.zx = 0.0f32;
        (*mf).c2rust_unnamed.yx = 0.0f32;
        (*mf).c2rust_unnamed.xx = 0.0f32;
        (*mf).c2rust_unnamed.wz = 0.0f32;
        (*mf).c2rust_unnamed.xz = 0.0f32;
        (*mf).c2rust_unnamed.wy = 0.0f32;
        (*mf).c2rust_unnamed.wx = 0.0f32;
        (*mf).c2rust_unnamed.zz = 0.0f32;
        (*mf).c2rust_unnamed.yz = 0.0f32;
        (*mf).c2rust_unnamed.zy = 0.0f32;
        (*mf).c2rust_unnamed.yy = 1.0f32;
        (*mf).c2rust_unnamed.xw = (*vec).x;
        (*mf).c2rust_unnamed.yw = (*vec).y;
        (*mf).c2rust_unnamed.zw = (*vec).z;
        (*mf).c2rust_unnamed.ww = 1.0f32
    }
    return floorY;
}
#[no_mangle]
pub unsafe extern "C" fn Gameplay_LoadFile(mut globalCtx: *mut GlobalContext,
                                           mut file: *mut RomFile)
 -> *mut libc::c_void {
    let mut size: u32_0 = 0;
    let mut allocp: *mut libc::c_void = 0 as *mut libc::c_void;
    size = (*file).vromEnd.wrapping_sub((*file).vromStart);
    allocp =
        GameState_Alloc(&mut (*globalCtx).state, size as size_t,
                        b"../z_play.c\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char, 4692 as libc::c_int);
    DmaMgr_SendRequest1(allocp, (*file).vromStart, size,
                        b"../z_play.c\x00" as *const u8 as
                            *const libc::c_char, 4694 as libc::c_int);
    return allocp;
}
#[no_mangle]
pub unsafe extern "C" fn Gameplay_InitEnvironment(mut globalCtx:
                                                      *mut GlobalContext,
                                                  mut skyboxId: s16) {
    Skybox_Init(&mut (*globalCtx).state, &mut (*globalCtx).skyboxCtx,
                skyboxId);
    Environment_Init(globalCtx, &mut (*globalCtx).envCtx, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Gameplay_InitScene(mut globalCtx: *mut GlobalContext,
                                            mut spawn: s32) {
    (*globalCtx).curSpawn = spawn as u8_0;
    (*globalCtx).linkActorEntry = 0 as *mut ActorEntry;
    (*globalCtx).unk_11DFC = 0 as *mut libc::c_void;
    (*globalCtx).setupEntranceList = 0 as *mut EntranceEntry;
    (*globalCtx).setupExitList = 0 as *mut s16;
    (*globalCtx).cUpElfMsgs = 0 as *mut ElfMessage;
    (*globalCtx).setupPathList = 0 as *mut Path;
    (*globalCtx).numSetupActors = 0 as libc::c_int as u8_0;
    Object_InitBank(globalCtx, &mut (*globalCtx).objectCtx);
    LightContext_Init(globalCtx, &mut (*globalCtx).lightCtx);
    TransitionActor_InitContext(&mut (*globalCtx).state,
                                &mut (*globalCtx).transiActorCtx);
    func_80096FD4(globalCtx, &mut (*globalCtx).roomCtx.curRoom);
    (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 15 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    gSaveContext.worldMapArea = 0 as libc::c_int as s16;
    Scene_ExecuteCommands(globalCtx,
                          (*globalCtx).sceneSegment as *mut SceneCmd);
    Gameplay_InitEnvironment(globalCtx, (*globalCtx).skyboxId as s16);
}
#[no_mangle]
pub unsafe extern "C" fn Gameplay_SpawnScene(mut globalCtx:
                                                 *mut GlobalContext,
                                             mut sceneNum: s32,
                                             mut spawn: s32) {
    let mut scene: *mut SceneTableEntry =
        &mut *gSceneTable.as_mut_ptr().offset(sceneNum as isize) as
            *mut SceneTableEntry;
    (*scene).unk_13 = 0 as libc::c_int as u8_0;
    (*globalCtx).loadedScene = scene;
    (*globalCtx).sceneNum = sceneNum as s16;
    (*globalCtx).sceneConfig = (*scene).config;
    osSyncPrintf(b"\nSCENE SIZE %fK\n\x00" as *const u8 as
                     *const libc::c_char,
                 ((*scene).sceneFile.vromEnd.wrapping_sub((*scene).sceneFile.vromStart)
                      as libc::c_float / 1024.0f32) as libc::c_double);
    (*globalCtx).sceneSegment =
        Gameplay_LoadFile(globalCtx, &mut (*scene).sceneFile);
    (*scene).unk_13 = 0 as libc::c_int as u8_0;
    if !(*globalCtx).sceneSegment.is_null() {
    } else {
        __assert(b"this->sceneSegment != NULL\x00" as *const u8 as
                     *const libc::c_char,
                 b"../z_play.c\x00" as *const u8 as *const libc::c_char,
                 4960 as libc::c_int);
    };
    gSegments[2 as libc::c_int as usize] =
        ((*globalCtx).sceneSegment as
             *mut u8_0).offset(-(0x80000000 as libc::c_uint as isize)) as
            u32_0;
    Gameplay_InitScene(globalCtx, spawn);
    osSyncPrintf(b"ROOM SIZE=%fK\n\x00" as *const u8 as *const libc::c_char,
                 (func_80096FE8(globalCtx, &mut (*globalCtx).roomCtx) as
                      libc::c_float / 1024.0f32) as libc::c_double);
}
#[no_mangle]
pub unsafe extern "C" fn func_800C016C(mut globalCtx: *mut GlobalContext,
                                       mut src: *mut Vec3f,
                                       mut dest: *mut Vec3f) {
    let mut temp: f32_0 = 0.;
    Matrix_Mult(&mut (*globalCtx).viewProjectionMtxF,
                MTXMODE_NEW as libc::c_int as u8_0);
    Matrix_MultVec3f(src, dest);
    temp =
        (*globalCtx).viewProjectionMtxF.c2rust_unnamed.ww +
            ((*globalCtx).viewProjectionMtxF.c2rust_unnamed.wx * (*src).x +
                 (*globalCtx).viewProjectionMtxF.c2rust_unnamed.wy * (*src).y
                 +
                 (*globalCtx).viewProjectionMtxF.c2rust_unnamed.wz *
                     (*src).z);
    (*dest).x = 160.0f32 + (*dest).x / temp * 160.0f32;
    (*dest).y = 120.0f32 - (*dest).y / temp * 120.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn Gameplay_CreateSubCamera(mut globalCtx:
                                                      *mut GlobalContext)
 -> s16 {
    let mut i: s16 = 0;
    i = 1 as libc::c_int as s16;
    while (i as libc::c_int) < 4 as libc::c_int {
        if (*globalCtx).cameraPtrs[i as usize].is_null() { break ; }
        i += 1
    }
    if i as libc::c_int == 4 as libc::c_int {
        osSyncPrintf(b"\x1b[41;37mcamera control: error: fulled sub camera system area\n\x1b[m\x00"
                         as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int) as s16
    }
    osSyncPrintf(b"camera control: \x1b[46m \x1b[47;34m create new sub camera [%d] \x1b[46m \x1b[m\n\x00"
                     as *const u8 as *const libc::c_char, i as libc::c_int);
    (*globalCtx).cameraPtrs[i as usize] =
        &mut *(*globalCtx).subCameras.as_mut_ptr().offset((i as libc::c_int -
                                                               1 as
                                                                   libc::c_int)
                                                              as isize) as
            *mut Camera;
    Camera_Init((*globalCtx).cameraPtrs[i as usize], &mut (*globalCtx).view,
                &mut (*globalCtx).colCtx, globalCtx);
    (*(*globalCtx).cameraPtrs[i as usize]).thisIdx = i;
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn Gameplay_GetActiveCamId(mut globalCtx:
                                                     *mut GlobalContext)
 -> s16 {
    return (*globalCtx).activeCamera;
}
#[no_mangle]
pub unsafe extern "C" fn Gameplay_ChangeCameraStatus(mut globalCtx:
                                                         *mut GlobalContext,
                                                     mut camId: s16,
                                                     mut status: s16) -> s16 {
    let mut camIdx: s16 =
        if camId as libc::c_int == -(1 as libc::c_int) {
            (*globalCtx).activeCamera as libc::c_int
        } else { camId as libc::c_int } as s16;
    if status as libc::c_int == 7 as libc::c_int {
        (*globalCtx).activeCamera = camIdx
    }
    return Camera_ChangeStatus((*globalCtx).cameraPtrs[camIdx as usize],
                               status);
}
#[no_mangle]
pub unsafe extern "C" fn Gameplay_ClearCamera(mut globalCtx:
                                                  *mut GlobalContext,
                                              mut camId: s16) {
    let mut camIdx: s16 =
        if camId as libc::c_int == -(1 as libc::c_int) {
            (*globalCtx).activeCamera as libc::c_int
        } else { camId as libc::c_int } as s16;
    if camIdx as libc::c_int == 0 as libc::c_int {
        osSyncPrintf(b"\x1b[41;37mcamera control: error: never clear camera !!\n\x1b[m\x00"
                         as *const u8 as *const libc::c_char);
    }
    if !(*globalCtx).cameraPtrs[camIdx as usize].is_null() {
        Camera_ChangeStatus((*globalCtx).cameraPtrs[camIdx as usize],
                            0x100 as libc::c_int as s16);
        (*globalCtx).cameraPtrs[camIdx as usize] = 0 as *mut Camera;
        osSyncPrintf(b"camera control: \x1b[46m \x1b[47;34m clear sub camera [%d] \x1b[46m \x1b[m\n\x00"
                         as *const u8 as *const libc::c_char,
                     camIdx as libc::c_int);
    } else {
        osSyncPrintf(b"\x1b[41;37mcamera control: error: camera No.%d already cleared\n\x1b[m\x00"
                         as *const u8 as *const libc::c_char,
                     camIdx as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Gameplay_ClearAllSubCameras(mut globalCtx:
                                                         *mut GlobalContext) {
    let mut i: s16 = 0;
    i = 1 as libc::c_int as s16;
    while (i as libc::c_int) < 4 as libc::c_int {
        if !(*globalCtx).cameraPtrs[i as usize].is_null() {
            Gameplay_ClearCamera(globalCtx, i);
        }
        i += 1
    }
    (*globalCtx).activeCamera = 0 as libc::c_int as s16;
}
#[no_mangle]
pub unsafe extern "C" fn Gameplay_GetCamera(mut globalCtx: *mut GlobalContext,
                                            mut camId: s16) -> *mut Camera {
    let mut camIdx: s16 =
        if camId as libc::c_int == -(1 as libc::c_int) {
            (*globalCtx).activeCamera as libc::c_int
        } else { camId as libc::c_int } as s16;
    return (*globalCtx).cameraPtrs[camIdx as usize];
}
#[no_mangle]
pub unsafe extern "C" fn Gameplay_CameraSetAtEye(mut globalCtx:
                                                     *mut GlobalContext,
                                                 mut camId: s16,
                                                 mut at: *mut Vec3f,
                                                 mut eye: *mut Vec3f) -> s32 {
    let mut ret: s32 = 0 as libc::c_int;
    let mut camIdx: s16 =
        if camId as libc::c_int == -(1 as libc::c_int) {
            (*globalCtx).activeCamera as libc::c_int
        } else { camId as libc::c_int } as s16;
    let mut camera: *mut Camera = (*globalCtx).cameraPtrs[camIdx as usize];
    let mut player: *mut Player = 0 as *mut Player;
    ret |= Camera_SetParam(camera, 1 as libc::c_int, at as *mut libc::c_void);
    ret <<= 1 as libc::c_int;
    ret |=
        Camera_SetParam(camera, 2 as libc::c_int, eye as *mut libc::c_void);
    (*camera).dist = Math3D_Vec3f_DistXYZ(at, eye);
    player = (*camera).player;
    if !player.is_null() {
        (*camera).posOffset.x = (*at).x - (*player).actor.world.pos.x;
        (*camera).posOffset.y = (*at).y - (*player).actor.world.pos.y;
        (*camera).posOffset.z = (*at).z - (*player).actor.world.pos.z
    } else {
        (*camera).posOffset.z = 0.0f32;
        (*camera).posOffset.y = (*camera).posOffset.z;
        (*camera).posOffset.x = (*camera).posOffset.y
    }
    (*camera).atLERPStepScale = 0.01f32;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn Gameplay_CameraSetAtEyeUp(mut globalCtx:
                                                       *mut GlobalContext,
                                                   mut camId: s16,
                                                   mut at: *mut Vec3f,
                                                   mut eye: *mut Vec3f,
                                                   mut up: *mut Vec3f)
 -> s32 {
    let mut ret: s32 = 0 as libc::c_int;
    let mut camIdx: s16 =
        if camId as libc::c_int == -(1 as libc::c_int) {
            (*globalCtx).activeCamera as libc::c_int
        } else { camId as libc::c_int } as s16;
    let mut camera: *mut Camera = (*globalCtx).cameraPtrs[camIdx as usize];
    let mut player: *mut Player = 0 as *mut Player;
    ret |= Camera_SetParam(camera, 1 as libc::c_int, at as *mut libc::c_void);
    ret <<= 1 as libc::c_int;
    ret |=
        Camera_SetParam(camera, 2 as libc::c_int, eye as *mut libc::c_void);
    ret <<= 1 as libc::c_int;
    ret |= Camera_SetParam(camera, 4 as libc::c_int, up as *mut libc::c_void);
    (*camera).dist = Math3D_Vec3f_DistXYZ(at, eye);
    player = (*camera).player;
    if !player.is_null() {
        (*camera).posOffset.x = (*at).x - (*player).actor.world.pos.x;
        (*camera).posOffset.y = (*at).y - (*player).actor.world.pos.y;
        (*camera).posOffset.z = (*at).z - (*player).actor.world.pos.z
    } else {
        (*camera).posOffset.z = 0.0f32;
        (*camera).posOffset.y = (*camera).posOffset.z;
        (*camera).posOffset.x = (*camera).posOffset.y
    }
    (*camera).atLERPStepScale = 0.01f32;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn Gameplay_CameraSetFov(mut globalCtx:
                                                   *mut GlobalContext,
                                               mut camId: s16, mut fov: f32_0)
 -> s32 {
    let mut ret: s32 =
        Camera_SetParam((*globalCtx).cameraPtrs[camId as usize],
                        0x20 as libc::c_int,
                        &mut fov as *mut f32_0 as *mut libc::c_void) &
            1 as libc::c_int;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn Gameplay_SetCameraRoll(mut globalCtx:
                                                    *mut GlobalContext,
                                                mut camId: s16, mut roll: s16)
 -> s32 {
    let mut camIdx: s16 =
        if camId as libc::c_int == -(1 as libc::c_int) {
            (*globalCtx).activeCamera as libc::c_int
        } else { camId as libc::c_int } as s16;
    let mut camera: *mut Camera = (*globalCtx).cameraPtrs[camIdx as usize];
    (*camera).roll = roll;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Gameplay_CopyCamera(mut globalCtx:
                                                 *mut GlobalContext,
                                             mut camId1: s16,
                                             mut camId2: s16) {
    let mut camIdx2: s16 =
        if camId2 as libc::c_int == -(1 as libc::c_int) {
            (*globalCtx).activeCamera as libc::c_int
        } else { camId2 as libc::c_int } as s16;
    let mut camIdx1: s16 =
        if camId1 as libc::c_int == -(1 as libc::c_int) {
            (*globalCtx).activeCamera as libc::c_int
        } else { camId1 as libc::c_int } as s16;
    Camera_Copy((*globalCtx).cameraPtrs[camIdx1 as usize],
                (*globalCtx).cameraPtrs[camIdx2 as usize]);
}
#[no_mangle]
pub unsafe extern "C" fn func_800C0808(mut globalCtx: *mut GlobalContext,
                                       mut camId: s16,
                                       mut player: *mut Player,
                                       mut setting: s16) -> s32 {
    let mut camera: *mut Camera = 0 as *mut Camera;
    let mut camIdx: s16 =
        if camId as libc::c_int == -(1 as libc::c_int) {
            (*globalCtx).activeCamera as libc::c_int
        } else { camId as libc::c_int } as s16;
    camera = (*globalCtx).cameraPtrs[camIdx as usize];
    Camera_InitPlayerSettings(camera, player);
    return Camera_ChangeSetting(camera, setting);
}
#[no_mangle]
pub unsafe extern "C" fn Gameplay_CameraChangeSetting(mut globalCtx:
                                                          *mut GlobalContext,
                                                      mut camId: s16,
                                                      mut setting: s16)
 -> s32 {
    return Camera_ChangeSetting(Gameplay_GetCamera(globalCtx, camId),
                                setting);
}
#[no_mangle]
pub unsafe extern "C" fn func_800C08AC(mut globalCtx: *mut GlobalContext,
                                       mut camId: s16, mut arg2: s16) {
    let mut camIdx: s16 =
        if camId as libc::c_int == -(1 as libc::c_int) {
            (*globalCtx).activeCamera as libc::c_int
        } else { camId as libc::c_int } as s16;
    let mut i: s16 = 0;
    Gameplay_ClearCamera(globalCtx, camIdx);
    i = 1 as libc::c_int as s16;
    while (i as libc::c_int) < 4 as libc::c_int {
        if !(*globalCtx).cameraPtrs[i as usize].is_null() {
            osSyncPrintf(b"\x1b[41;37mcamera control: error: return to main, other camera left. %d cleared!!\n\x1b[m\x00"
                             as *const u8 as *const libc::c_char,
                         i as libc::c_int);
            Gameplay_ClearCamera(globalCtx, i);
        }
        i += 1
    }
    if arg2 as libc::c_int <= 0 as libc::c_int {
        Gameplay_ChangeCameraStatus(globalCtx, 0 as libc::c_int as s16,
                                    7 as libc::c_int as s16);
        (*(*globalCtx).cameraPtrs[0 as libc::c_int as usize]).parentCamIdx =
            0 as libc::c_int as s16;
        (*(*globalCtx).cameraPtrs[0 as libc::c_int as usize]).childCamIdx =
            (*(*globalCtx).cameraPtrs[0 as libc::c_int as usize]).parentCamIdx
    } else {
        OnePointCutscene_Init(globalCtx, 1020 as libc::c_int as s16, arg2,
                              0 as *mut Actor, 0 as libc::c_int as s16);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Gameplay_CameraGetUID(mut globalCtx:
                                                   *mut GlobalContext,
                                               mut camId: s16) -> s16 {
    let mut camera: *mut Camera = (*globalCtx).cameraPtrs[camId as usize];
    if !camera.is_null() {
        return (*camera).uid
    } else { return -(1 as libc::c_int) as s16 };
}
#[no_mangle]
pub unsafe extern "C" fn func_800C09D8(mut globalCtx: *mut GlobalContext,
                                       mut camId: s16, mut arg2: s16) -> s16 {
    let mut camera: *mut Camera = (*globalCtx).cameraPtrs[camId as usize];
    if !camera.is_null() {
        return 0 as libc::c_int as s16
    } else if (*camera).uid as libc::c_int != arg2 as libc::c_int {
        return 0 as libc::c_int as s16
    } else if (*camera).status as libc::c_int != 7 as libc::c_int {
        return 2 as libc::c_int as s16
    } else { return 1 as libc::c_int as s16 };
}
#[no_mangle]
pub unsafe extern "C" fn Gameplay_SaveSceneFlags(mut globalCtx:
                                                     *mut GlobalContext) {
    let mut savedSceneFlags: *mut SavedSceneFlags =
        &mut *gSaveContext.sceneFlags.as_mut_ptr().offset((*globalCtx).sceneNum
                                                              as isize) as
            *mut SavedSceneFlags;
    (*savedSceneFlags).chest = (*globalCtx).actorCtx.flags.chest;
    (*savedSceneFlags).swch = (*globalCtx).actorCtx.flags.swch;
    (*savedSceneFlags).clear = (*globalCtx).actorCtx.flags.clear;
    (*savedSceneFlags).collect = (*globalCtx).actorCtx.flags.collect;
}
#[no_mangle]
pub unsafe extern "C" fn Gameplay_SetRespawnData(mut globalCtx:
                                                     *mut GlobalContext,
                                                 mut respawnMode: s32,
                                                 mut entranceIndex: s16,
                                                 mut roomIndex: s32,
                                                 mut playerParams: s32,
                                                 mut pos: *mut Vec3f,
                                                 mut yaw: s16) {
    let mut respawnData: *mut RespawnData =
        &mut *gSaveContext.respawn.as_mut_ptr().offset(respawnMode as isize)
            as *mut RespawnData;
    (*respawnData).entranceIndex = entranceIndex;
    (*respawnData).roomIndex = roomIndex as u8_0;
    (*respawnData).pos = *pos;
    (*respawnData).yaw = yaw;
    (*respawnData).playerParams = playerParams as s16;
    (*respawnData).tempSwchFlags = (*globalCtx).actorCtx.flags.tempSwch;
    (*respawnData).tempCollectFlags = (*globalCtx).actorCtx.flags.tempCollect;
}
#[no_mangle]
pub unsafe extern "C" fn Gameplay_SetupRespawnPoint(mut globalCtx:
                                                        *mut GlobalContext,
                                                    mut respawnMode: s32,
                                                    mut playerParams: s32) {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut entranceIndex: s32 = 0;
    let mut roomIndex: s8 = 0;
    if (*globalCtx).sceneNum as libc::c_int !=
           SCENE_YOUSEI_IZUMI_TATE as libc::c_int &&
           (*globalCtx).sceneNum as libc::c_int !=
               SCENE_KAKUSIANA as libc::c_int {
        roomIndex = (*globalCtx).roomCtx.curRoom.num;
        entranceIndex = gSaveContext.entranceIndex;
        Gameplay_SetRespawnData(globalCtx, respawnMode, entranceIndex as s16,
                                roomIndex as s32, playerParams,
                                &mut (*player).actor.world.pos,
                                (*player).actor.shape.rot.y);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Gameplay_TriggerVoidOut(mut globalCtx:
                                                     *mut GlobalContext) {
    gSaveContext.respawn[RESPAWN_MODE_DOWN as libc::c_int as
                             usize].tempSwchFlags =
        (*globalCtx).actorCtx.flags.tempSwch;
    gSaveContext.respawn[RESPAWN_MODE_DOWN as libc::c_int as
                             usize].tempCollectFlags =
        (*globalCtx).actorCtx.flags.tempCollect;
    gSaveContext.respawnFlag = 1 as libc::c_int;
    (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
    (*globalCtx).nextEntranceIndex =
        gSaveContext.respawn[RESPAWN_MODE_DOWN as libc::c_int as
                                 usize].entranceIndex;
    (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn Gameplay_LoadToLastEntrance(mut globalCtx:
                                                         *mut GlobalContext) {
    gSaveContext.respawnFlag = -(1 as libc::c_int);
    (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
    if (*globalCtx).sceneNum as libc::c_int ==
           SCENE_GANON_SONOGO as libc::c_int ||
           (*globalCtx).sceneNum as libc::c_int ==
               SCENE_GANON_FINAL as libc::c_int ||
           (*globalCtx).sceneNum as libc::c_int ==
               SCENE_GANONTIKA_SONOGO as libc::c_int ||
           (*globalCtx).sceneNum as libc::c_int ==
               SCENE_GANON_DEMO as libc::c_int {
        (*globalCtx).nextEntranceIndex = 0x43f as libc::c_int as s16;
        Item_Give(globalCtx, ITEM_SWORD_MASTER as libc::c_int as u8_0);
    } else if gSaveContext.entranceIndex == 0x28a as libc::c_int ||
                  gSaveContext.entranceIndex == 0x28e as libc::c_int ||
                  gSaveContext.entranceIndex == 0x292 as libc::c_int ||
                  gSaveContext.entranceIndex == 0x476 as libc::c_int {
        (*globalCtx).nextEntranceIndex = 0x1f9 as libc::c_int as s16
    } else {
        (*globalCtx).nextEntranceIndex = gSaveContext.entranceIndex as s16
    }
    (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn Gameplay_TriggerRespawn(mut globalCtx:
                                                     *mut GlobalContext) {
    Gameplay_SetupRespawnPoint(globalCtx, RESPAWN_MODE_DOWN as libc::c_int,
                               0xdff as libc::c_int);
    Gameplay_LoadToLastEntrance(globalCtx);
}
#[no_mangle]
pub unsafe extern "C" fn func_800C0CB8(mut globalCtx: *mut GlobalContext)
 -> s32 {
    return ((*(*globalCtx).roomCtx.curRoom.mesh).polygon.type_0 as libc::c_int
                != 1 as libc::c_int &&
                (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int *
                                       16 as libc::c_int + 15 as libc::c_int)
                                      as usize] as libc::c_int !=
                    0x20 as libc::c_int &&
                (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int *
                                       16 as libc::c_int + 15 as libc::c_int)
                                      as usize] as libc::c_int !=
                    0x30 as libc::c_int &&
                (*gGameInfo).data[(6 as libc::c_int * 6 as libc::c_int *
                                       16 as libc::c_int + 15 as libc::c_int)
                                      as usize] as libc::c_int !=
                    0x40 as libc::c_int &&
                (*globalCtx).sceneNum as libc::c_int !=
                    SCENE_HAIRAL_NIWA as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn FrameAdvance_IsEnabled(mut globalCtx:
                                                    *mut GlobalContext)
 -> s32 {
    return ((*globalCtx).frameAdvCtx.enabled != 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_800C0D34(mut globalCtx: *mut GlobalContext,
                                       mut actor: *mut Actor,
                                       mut yaw: *mut s16) -> s32 {
    let mut transitionActor: *mut TransitionActorEntry =
        0 as *mut TransitionActorEntry;
    let mut frontRoom: s32 = 0;
    if (*actor).category as libc::c_int != ACTORCAT_DOOR as libc::c_int {
        return 0 as libc::c_int
    }
    transitionActor =
        &mut *(*globalCtx).transiActorCtx.list.offset(((*actor).params as
                                                           u16_0 as
                                                           libc::c_int >>
                                                           10 as libc::c_int)
                                                          as isize) as
            *mut TransitionActorEntry;
    frontRoom =
        (*transitionActor).sides[0 as libc::c_int as usize].room as s32;
    if frontRoom ==
           (*transitionActor).sides[1 as libc::c_int as usize].room as
               libc::c_int {
        return 0 as libc::c_int
    }
    if frontRoom == (*actor).room as libc::c_int {
        *yaw = (*actor).shape.rot.y
    } else {
        *yaw =
            ((*actor).shape.rot.y as libc::c_int + 0x8000 as libc::c_int) as
                s16
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_800C0DB4(mut globalCtx: *mut GlobalContext,
                                       mut pos: *mut Vec3f) -> s32 {
    let mut waterBox: *mut WaterBox = 0 as *mut WaterBox;
    let mut poly: *mut CollisionPoly = 0 as *mut CollisionPoly;
    let mut waterSurfacePos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut bgId: s32 = 0;
    waterSurfacePos = *pos;
    if WaterBox_GetSurface1(globalCtx, &mut (*globalCtx).colCtx,
                            waterSurfacePos.x, waterSurfacePos.z,
                            &mut waterSurfacePos.y, &mut waterBox) ==
           1 as libc::c_int && (*pos).y < waterSurfacePos.y &&
           BgCheck_EntityRaycastFloor3(&mut (*globalCtx).colCtx, &mut poly,
                                       &mut bgId, &mut waterSurfacePos) !=
               -32000.0f32 {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
