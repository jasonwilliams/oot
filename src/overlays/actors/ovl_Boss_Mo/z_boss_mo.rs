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
    fn EffectSsBubble_Spawn(globalCtx: *mut GlobalContext, pos: *mut Vec3f,
                            yPosOffset: f32_0, yPosRandScale: f32_0,
                            xzPosRandScale: f32_0, scale: f32_0);
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
    fn Actor_MoveForward(actor: *mut Actor);
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
    fn func_8002EB44(object: *mut Vec3f, eye: *mut Vec3f,
                     lightDir: *mut Vec3f, gfxCtx: *mut GraphicsContext)
     -> *mut Hilite;
    #[no_mangle]
    fn func_8002ED80(actor: *mut Actor, globalCtx: *mut GlobalContext,
                     flag: s32);
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
    fn Actor_ChangeCategory(globalCtx: *mut GlobalContext,
                            actorCtx: *mut ActorContext, actor: *mut Actor,
                            actorCategory: u8_0);
    #[no_mangle]
    fn Rand_ZeroFloat(f: f32_0) -> f32_0;
    #[no_mangle]
    fn Rand_CenteredFloat(f: f32_0) -> f32_0;
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
    fn Audio_PlaySoundAtPosition(globalCtx: *mut GlobalContext,
                                 pos: *mut Vec3f, duration: s32,
                                 sfxId: u16_0);
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
    fn Interface_ChangeAlpha(alphaType: u16_0);
    #[no_mangle]
    fn Gfx_CallSetupDL(gfx: *mut Gfx, i: u32_0) -> *mut Gfx;
    #[no_mangle]
    fn func_80093D18(gfxCtx: *mut GraphicsContext);
    #[no_mangle]
    fn func_80093D84(gfxCtx: *mut GraphicsContext);
    #[no_mangle]
    fn func_80094044(gfxCtx: *mut GraphicsContext);
    #[no_mangle]
    fn func_80094BC4(gfxCtx: *mut GraphicsContext);
    #[no_mangle]
    fn Gfx_TwoTexScroll(gfxCtx: *mut GraphicsContext, tile1: s32, x1: u32_0,
                        y1: u32_0, width1: s32, height1: s32, tile2: s32,
                        x2: u32_0, y2: u32_0, width2: s32, height2: s32)
     -> *mut Gfx;
    #[no_mangle]
    fn SkinMatrix_Vec3fMtxFMultXYZW(mf: *mut MtxF, src: *mut Vec3f,
                                    xyzDest: *mut Vec3f, wDest: *mut f32_0);
    #[no_mangle]
    fn func_800AA000(_: f32_0, _: u8_0, _: u8_0, _: u8_0);
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
    fn Gameplay_CameraSetAtEyeUp(globalCtx: *mut GlobalContext, camId: s16,
                                 at: *mut Vec3f, eye: *mut Vec3f,
                                 up: *mut Vec3f) -> s32;
    #[no_mangle]
    fn Gameplay_CameraSetFov(globalCtx: *mut GlobalContext, camId: s16,
                             fov: f32_0) -> s32;
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
    fn Matrix_Get(dest: *mut MtxF);
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
    fn Matrix_ToMtx(dest: *mut Mtx, file: *mut libc::c_char, line: s32)
     -> *mut Mtx;
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
    fn Audio_PlaySoundIncreasinglyTransposed(pos: *mut Vec3f, sfxId: s16,
                                             semitones: *mut u8_0);
    #[no_mangle]
    fn Audio_ResetIncreasingTranspose();
    #[no_mangle]
    fn Audio_PlaySoundGeneral(sfxId: u16_0, pos: *mut Vec3f, token: u8_0,
                              freqScale: *mut f32_0, a4: *mut f32_0,
                              reverbAdd: *mut s8);
    #[no_mangle]
    fn Audio_StopSfxByPos(_: *mut Vec3f);
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
    fn Message_CloseTextbox(_: *mut GlobalContext);
    #[no_mangle]
    fn Message_StartTextbox(globalCtx: *mut GlobalContext, textId: u16_0,
                            actor: *mut Actor);
    #[no_mangle]
    static mut gMorphaTransposeTable: [u8_0; 16];
    #[no_mangle]
    static mut D_801333E0: f32_0;
    #[no_mangle]
    static mut D_801333E8: s8;
    #[no_mangle]
    static mut gSaveContext: SaveContext;
    #[no_mangle]
    static mut gGameInfo: *mut GameInfo;
    #[no_mangle]
    static mut gSegments: [u32_0; 16];
    #[no_mangle]
    static mut gMorphaBubbleDL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaDropletMaterialDL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaDropletModelDL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaWetSpotModelDL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaTitleCardTex: [u64_0; 0];
    #[no_mangle]
    static mut gMorphaWaterDL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaCoreMembraneDL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaCoreNucleusDL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaTentacleBaseDL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaTentaclePart0DL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaTentaclePart1DL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaTentaclePart2DL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaTentaclePart3DL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaTentaclePart4DL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaTentaclePart5DL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaTentaclePart6DL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaTentaclePart7DL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaTentaclePart8DL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaTentaclePart9DL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaTentaclePart10DL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaTentaclePart11DL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaTentaclePart12DL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaTentaclePart13DL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaTentaclePart14DL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaTentaclePart15DL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaTentaclePart16DL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaTentaclePart17DL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaTentaclePart18DL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaTentaclePart19DL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaTentaclePart20DL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaTentaclePart21DL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaTentaclePart22DL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaTentaclePart23DL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaTentaclePart24DL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaTentaclePart25DL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaTentaclePart26DL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaTentaclePart27DL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaTentaclePart28DL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaTentaclePart29DL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaTentaclePart30DL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaTentaclePart31DL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaTentaclePart32DL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaTentaclePart33DL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaTentaclePart34DL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaTentaclePart35DL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaTentaclePart36DL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaTentaclePart37DL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaTentaclePart38DL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaTentaclePart39DL: [Gfx; 0];
    #[no_mangle]
    static mut gMorphaTentaclePart40DL: [Gfx; 0];
    #[no_mangle]
    static mut gEffShockwaveDL: [Gfx; 0];
    #[no_mangle]
    static mut gEffWaterRippleDL: [Gfx; 0];
    #[no_mangle]
    static mut gCircleShadowDL: [Gfx; 0];
    #[no_mangle]
    static mut gDust1Tex: [u64_0; 0];
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
pub struct Vec2f {
    pub x: f32_0,
    pub y: f32_0,
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
pub struct BossMo {
    pub actor: Actor,
    pub otherTent: *mut Actor,
    pub actionFunc: BossMoActionFunc,
    pub tent2KillTimer: u8_0,
    pub hitCount: u8_0,
    pub tentSpawnPos: u8_0,
    pub work: [s16; 9],
    pub widthIndex: s16,
    pub pulsePhase: s16,
    pub xSwing: s16,
    pub zSwing: s16,
    pub cutIndex: s16,
    pub meltIndex: s16,
    pub linkToLeft: s16,
    pub mashCounter: s16,
    pub noBubbles: s16,
    pub sfxTimer: s16,
    pub timers: [s16; 5],
    pub fwork: [f32_0; 7],
    pub baseAlpha: f32_0,
    pub cutScale: f32_0,
    pub waterTex1x: f32_0,
    pub waterTex1y: f32_0,
    pub waterTex2x: f32_0,
    pub waterTex2y: f32_0,
    pub waterLevel: f32_0,
    pub flattenRate: f32_0,
    pub waterTexAlpha: f32_0,
    pub waterLevelMod: f32_0,
    pub baseBubblesTimer: s16,
    pub attackAngleMod: s16,
    pub unk_1D0: u8_0,
    pub drawActor: u8_0,
    pub linkHitTimer: u8_0,
    pub targetPos: Vec3f,
    pub tentRippleSize: f32_0,
    pub grabPosRot: PosRot,
    pub tentWidth: [f32_0; 300],
    pub tentStretch: [Vec3f; 41],
    pub tentScale: [Vec3f; 41],
    pub tentRipple: [Vec3f; 41],
    pub tentRot: [Vec3s; 41],
    pub tentMaxAngle: f32_0,
    pub tentSpeed: f32_0,
    pub tentPulse: f32_0,
    pub tentPos: [Vec3f; 41],
    pub cameraZoom: f32_0,
    pub csState: s16,
    pub csCamera: s16,
    pub targetIndex: s16,
    pub cameraEye: Vec3f,
    pub cameraAt: Vec3f,
    pub cameraUp: Vec3f,
    pub unk_F8C: [libc::c_char; 24],
    pub cameraEyeVel: Vec3f,
    pub cameraAtVel: Vec3f,
    pub cameraNextEye: Vec3f,
    pub cameraEyeMaxVel: Vec3f,
    pub cameraNextAt: Vec3f,
    pub cameraAtMaxVel: Vec3f,
    pub cameraSpeedMod: f32_0,
    pub cameraAccel: f32_0,
    pub unk_FF4: [libc::c_char; 8],
    pub cameraDist: f32_0,
    pub cameraSpeed: f32_0,
    pub cameraYaw: f32_0,
    pub cameraYawRate: f32_0,
    pub cameraYawShake: f32_0,
    pub tentTipPos: Vec3f,
    pub tentCollider: ColliderJntSph,
    pub tentElements: [ColliderJntSphElement; 19],
    pub coreCollider: ColliderCylinder,
    pub unk_1548: [libc::c_char; 68],
}
pub type BossMoActionFunc
    =
    Option<unsafe extern "C" fn(_: *mut BossMo, _: *mut GlobalContext) -> ()>;
pub type C2RustUnnamed_23 = libc::c_uint;
pub const MO_TENT_SHORT_MAX: C2RustUnnamed_23 = 9;
pub const MO_TENT_BASE_TEX2_Y: C2RustUnnamed_23 = 8;
pub const MO_TENT_BASE_TEX2_X: C2RustUnnamed_23 = 7;
pub const MO_TENT_BASE_TEX1_Y: C2RustUnnamed_23 = 6;
pub const MO_TENT_BASE_TEX1_X: C2RustUnnamed_23 = 5;
pub const MO_TENT_INVINC_TIMER: C2RustUnnamed_23 = 4;
pub const MO_TENT_UNK_TIMER: C2RustUnnamed_23 = 3;
pub const MO_TENT_VAR_TIMER: C2RustUnnamed_23 = 2;
pub const MO_TENT_MOVE_TIMER: C2RustUnnamed_23 = 1;
pub const MO_TENT_ACTION_STATE: C2RustUnnamed_23 = 0;
pub type C2RustUnnamed_24 = libc::c_uint;
pub const MO_CORE_SHORT_MAX: C2RustUnnamed_24 = 9;
pub const MO_CORE_WAIT_IN_WATER: C2RustUnnamed_24 = 8;
pub const MO_CORE_DRAW_SHADOW: C2RustUnnamed_24 = 7;
pub const MO_CORE_POS_IN_TENT: C2RustUnnamed_24 = 6;
pub const MO_CORE_SHORT_5: C2RustUnnamed_24 = 5;
pub const MO_CORE_INVINC_TIMER: C2RustUnnamed_24 = 4;
pub const MO_CORE_DMG_FLASH_TIMER: C2RustUnnamed_24 = 3;
pub const MO_CORE_VAR_TIMER: C2RustUnnamed_24 = 2;
pub const MO_CORE_MOVE_TIMER: C2RustUnnamed_24 = 1;
pub const MO_CORE_ACTION_STATE: C2RustUnnamed_24 = 0;
pub type C2RustUnnamed_25 = libc::c_uint;
pub const MO_TENT_FLOAT_MAX: C2RustUnnamed_25 = 7;
pub const MO_TENT_MAX_STRETCH: C2RustUnnamed_25 = 6;
pub const MO_TENT_SWING_RATE_Z: C2RustUnnamed_25 = 5;
pub const MO_TENT_SWING_SIZE_Z: C2RustUnnamed_25 = 4;
pub const MO_TENT_SWING_LAG_Z: C2RustUnnamed_25 = 3;
pub const MO_TENT_SWING_RATE_X: C2RustUnnamed_25 = 2;
pub const MO_TENT_SWING_SIZE_X: C2RustUnnamed_25 = 1;
pub const MO_TENT_SWING_LAG_X: C2RustUnnamed_25 = 0;
pub type C2RustUnnamed_26 = libc::c_uint;
pub const MO_CORE_FLOAT_MAX: C2RustUnnamed_26 = 1;
pub const MO_CORE_INTRO_WATER_ALPHA: C2RustUnnamed_26 = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BossMoEffect {
    pub pos: Vec3f,
    pub vel: Vec3f,
    pub accel: Vec3f,
    pub type_0: u8_0,
    pub timer: u8_0,
    pub stopTimer: u8_0,
    pub unk_28: s16,
    pub alpha: s16,
    pub rippleMode: s16,
    pub maxAlpha: s16,
    pub scale: f32_0,
    pub fwork: [f32_0; 2],
    pub targetPos: *mut Vec3f,
}
pub const MO_TENT_WAIT: C2RustUnnamed_29 = 10;
pub const MO_FX_DROPLET: C2RustUnnamed_28 = 3;
pub const MO_FX_SPLASH_TRAIL: C2RustUnnamed_28 = 5;
pub const MO_FX_NONE: C2RustUnnamed_28 = 0;
pub const MO_TENT_DEATH_2: C2RustUnnamed_29 = 202;
pub const MO_TENT_GRAB: C2RustUnnamed_29 = 4;
pub const MO_TENT_CURL: C2RustUnnamed_29 = 3;
pub const MO_TENT_ATTACK: C2RustUnnamed_29 = 2;
pub const MO_TENT_DEATH_6: C2RustUnnamed_29 = 206;
pub const MO_TENT_DEATH_5: C2RustUnnamed_29 = 205;
pub const MO_TENT_DEATH_1: C2RustUnnamed_29 = 201;
pub const MO_TENT_DEATH_3: C2RustUnnamed_29 = 203;
pub const MO_TENT_DEATH_START: C2RustUnnamed_29 = 200;
pub const MO_TENT_DESPAWN: C2RustUnnamed_29 = 102;
pub const MO_TENT_RETREAT: C2RustUnnamed_29 = 101;
pub const MO_TENT_CUT: C2RustUnnamed_29 = 100;
pub const MO_TENT_SHAKE: C2RustUnnamed_29 = 5;
pub const MO_TENT_READY: C2RustUnnamed_29 = 0;
pub const MO_TENT_SWING: C2RustUnnamed_29 = 1;
pub const MO_BATTLE: C2RustUnnamed_31 = 0;
pub const MO_FX_SMALL_RIPPLE: C2RustUnnamed_28 = 1;
pub const MO_TENT_SPAWN: C2RustUnnamed_29 = 11;
pub const MO_FX_SPLASH: C2RustUnnamed_28 = 4;
pub const MO_FX_WET_SPOT: C2RustUnnamed_28 = 6;
pub const MO_FX_BUBBLE: C2RustUnnamed_28 = 7;
pub const MO_DEATH_START: C2RustUnnamed_31 = 100;
pub const MO_FX_BIG_RIPPLE: C2RustUnnamed_28 = 2;
pub const MO_CORE_STUNNED: C2RustUnnamed_30 = 5;
pub const MO_CORE_ATTACK: C2RustUnnamed_30 = 10;
pub const MO_CORE_UNDERWATER: C2RustUnnamed_30 = 2;
pub const MO_CORE_RETREAT: C2RustUnnamed_30 = 11;
pub const MO_CORE_MAKE_TENT: C2RustUnnamed_30 = 1;
pub const MO_CORE_MOVE: C2RustUnnamed_30 = 0;
pub const MO_CORE_INTRO_REVEAL: C2RustUnnamed_30 = 21;
pub const MO_CORE_UNUSED: C2RustUnnamed_30 = -11;
pub const MO_DEATH_FINISH: C2RustUnnamed_31 = 105;
pub const MO_DEATH_DROPLET: C2RustUnnamed_31 = 104;
pub const MO_DEATH_CEILING: C2RustUnnamed_31 = 103;
pub const MO_DEATH_DRAIN_WATER_2: C2RustUnnamed_31 = 102;
pub const MO_DEATH_DRAIN_WATER_1: C2RustUnnamed_31 = 101;
pub const MO_DEATH_MO_CORE_BURST: C2RustUnnamed_31 = 150;
pub const MO_CORE_INTRO_WAIT: C2RustUnnamed_30 = 20;
pub const MO_INTRO_START: C2RustUnnamed_31 = 2;
pub const MO_INTRO_REVEAL: C2RustUnnamed_31 = 4;
pub const MO_INTRO_FINISH: C2RustUnnamed_31 = 5;
pub const MO_INTRO_SWIM: C2RustUnnamed_31 = 3;
pub const MO_INTRO_WAIT: C2RustUnnamed_31 = 1;
pub type C2RustUnnamed_28 = libc::c_uint;
pub type C2RustUnnamed_29 = libc::c_uint;
pub type C2RustUnnamed_30 = libc::c_int;
pub type C2RustUnnamed_31 = libc::c_uint;
#[no_mangle]
pub static mut Boss_Mo_InitVars: ActorInit =
    unsafe {
        {
            let mut init =
                ActorInit{id: ACTOR_BOSS_MO as libc::c_int as s16,
                          category: ACTORCAT_BOSS as libc::c_int as u8_0,
                          flags:
                              ((1 as libc::c_int) << 0 as libc::c_int |
                                   (1 as libc::c_int) << 2 as libc::c_int |
                                   (1 as libc::c_int) << 4 as libc::c_int |
                                   (1 as libc::c_int) << 5 as libc::c_int) as
                                  u32_0,
                          objectId: OBJECT_MO as libc::c_int as s16,
                          instanceSize:
                              ::std::mem::size_of::<BossMo>() as
                                  libc::c_ulong,
                          init:
                              ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                      *mut Actor,
                                                                                  _:
                                                                                      *mut GlobalContext)
                                                                 -> ()>,
                                                      ActorFunc>(Some(BossMo_Init
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
                                                      ActorFunc>(Some(BossMo_Destroy
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
                                                      ActorFunc>(Some(BossMo_UpdateTent
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
                                                      ActorFunc>(Some(BossMo_DrawTent
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
static mut sMorphaCore: *mut BossMo = 0 as *const BossMo as *mut BossMo;
static mut sMorphaTent1: *mut BossMo = 0 as *const BossMo as *mut BossMo;
static mut sMorphaTent2: *mut BossMo = 0 as *const BossMo as *mut BossMo;
static mut sFlatWidth: [f32_0; 41] =
    [15.0f32, 12.0f32, 9.0f32, 6.5f32, 4.8f32, 4.0f32, 3.4f32, 3.1f32, 3.0f32,
     3.1f32, 3.2f32, 3.4f32, 3.6f32, 3.8f32, 4.0f32, 4.6f32, 5.1f32, 5.5f32,
     6.1f32, 6.6f32, 7.3f32, 7.7f32, 8.4f32, 8.5f32, 8.7f32, 8.8f32, 8.8f32,
     8.7f32, 8.6f32, 8.3f32, 8.2f32, 8.1f32, 7.2f32, 6.7f32, 5.9f32, 4.9f32,
     2.7f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32];
static mut sJntSphElementsInit: [ColliderJntSphElementInit; 19] =
    [{
         let mut init =
             ColliderJntSphElementInit{info:
                                           {
                                               let mut init =
                                                   ColliderInfoInit{elemType:
                                                                        ELEMTYPE_UNK4
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
                                                                                                         0
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
                                                                        ELEMTYPE_UNK4
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
                                                                                    1
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
                                                                                                         0
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
                                                                        ELEMTYPE_UNK4
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
                                                                        ELEMTYPE_UNK4
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
                                                                                                         24
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
                                                                        ELEMTYPE_UNK4
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
                                                                        ELEMTYPE_UNK4
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
     },
     {
         let mut init =
             ColliderJntSphElementInit{info:
                                           {
                                               let mut init =
                                                   ColliderInfoInit{elemType:
                                                                        ELEMTYPE_UNK4
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
                                                                                                         18
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
                                                                        ELEMTYPE_UNK4
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
                                                                                                         16
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
                                                                        ELEMTYPE_UNK4
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
                                                                                                         14
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
                                                                        ELEMTYPE_UNK4
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
                                                                                    9
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
                                                                                                         12
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
                                                                        ELEMTYPE_UNK4
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
                                                                                                         10
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
                                                                        ELEMTYPE_UNK4
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
                                                                                                         10
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
                                                                        ELEMTYPE_UNK4
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
                                                                                    12
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
                                                                                                         10
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
                                                                        ELEMTYPE_UNK4
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
                                                                                    13
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
                                                                                                         10
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
                                                                        ELEMTYPE_UNK4
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
                                                                                    14
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
                                                                                                         10
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
                                                                        ELEMTYPE_UNK4
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
                                                                                                         10
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
                                                                        ELEMTYPE_UNK4
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
                                                                                                         10
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
                                                                        ELEMTYPE_UNK4
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
                                                                                    17
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
                                                                                                         10
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
                                                                        ELEMTYPE_UNK4
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
                                                                                    18
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
                                                                                                         10
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
                                                                COLSHAPE_JNTSPH
                                                                    as
                                                                    libc::c_int
                                                                    as u8_0,};
                                           init
                                       },
                                   count: 19 as libc::c_int,
                                   elements:
                                       sJntSphElementsInit.as_ptr() as
                                           *mut _,};
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
                                                                                               0xffdfffff
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
                                                            20 as libc::c_int
                                                                as s16,
                                                        height:
                                                            40 as libc::c_int
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
static mut sEffects: [BossMoEffect; 300] =
    [BossMoEffect{pos: Vec3f{x: 0., y: 0., z: 0.,},
                  vel: Vec3f{x: 0., y: 0., z: 0.,},
                  accel: Vec3f{x: 0., y: 0., z: 0.,},
                  type_0: 0,
                  timer: 0,
                  stopTimer: 0,
                  unk_28: 0,
                  alpha: 0,
                  rippleMode: 0,
                  maxAlpha: 0,
                  scale: 0.,
                  fwork: [0.; 2],
                  targetPos: 0 as *const Vec3f as *mut Vec3f,}; 300];
static mut sSeed1: s32 = 0;
static mut sSeed2: s32 = 0;
static mut sSeed3: s32 = 0;
#[no_mangle]
pub unsafe extern "C" fn BossMo_InitRand(mut seedInit0: s32,
                                         mut seedInit1: s32,
                                         mut seedInit2: s32) {
    sSeed1 = seedInit0;
    sSeed2 = seedInit1;
    sSeed3 = seedInit2;
}
#[no_mangle]
pub unsafe extern "C" fn BossMo_RandZeroOne() -> f32_0 {
    // Wichmann-Hill algorithm
    let mut randFloat: f32_0 = 0.;
    sSeed1 = sSeed1 * 171 as libc::c_int % 30269 as libc::c_int;
    sSeed2 = sSeed2 * 172 as libc::c_int % 30307 as libc::c_int;
    sSeed3 = sSeed3 * 170 as libc::c_int % 30323 as libc::c_int;
    randFloat =
        sSeed1 as libc::c_float / 30269.0f32 +
            sSeed2 as libc::c_float / 30307.0f32 +
            sSeed3 as libc::c_float / 30323.0f32;
    while randFloat >= 1.0f32 { randFloat -= 1.0f32 }
    return fabsf(randFloat);
}
#[no_mangle]
pub unsafe extern "C" fn BossMo_NearLand(mut pos: *mut Vec3f,
                                         mut margin: f32_0) -> s32 {
    if 450.0f32 - margin <= fabsf((*pos).x) { return 1 as libc::c_int }
    if 450.0f32 - margin <= fabsf((*pos).z) { return 1 as libc::c_int }
    if fabsf((*pos).x - 180.0f32) < 90.0f32 + margin ||
           fabsf((*pos).x - -180.0f32) < 90.0f32 + margin {
        if fabsf((*pos).z - 180.0f32) < 90.0f32 + margin {
            return 1 as libc::c_int
        }
        if fabsf((*pos).z - -180.0f32) < 90.0f32 + margin {
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BossMo_SpawnRipple(mut effect: *mut BossMoEffect,
                                            mut pos: *mut Vec3f,
                                            mut scale: f32_0,
                                            mut maxScale: f32_0,
                                            mut maxAlpha: s16,
                                            mut partLimit: s16,
                                            mut type_0: u8_0) {
    static mut zeroVec: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
    let mut i: s16 = 0;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < partLimit as libc::c_int {
        if (*effect).type_0 as libc::c_int == MO_FX_NONE as libc::c_int {
            (*effect).stopTimer = 0 as libc::c_int as u8_0;
            (*effect).type_0 = type_0;
            (*effect).pos = *pos;
            (*effect).vel = zeroVec;
            (*effect).accel = zeroVec;
            (*effect).scale = scale * 0.0025f32;
            (*effect).fwork[0 as libc::c_int as usize] = maxScale * 0.0025f32;
            if scale > 300.0f32 {
                (*effect).alpha = 0 as libc::c_int as s16;
                (*effect).maxAlpha = maxAlpha;
                (*effect).rippleMode = 0 as libc::c_int as s16;
                (*effect).fwork[1 as libc::c_int as usize] =
                    ((*effect).fwork[0 as libc::c_int as usize] -
                         (*effect).scale) * 0.05f32
            } else {
                (*effect).alpha = maxAlpha;
                (*effect).rippleMode = 1 as libc::c_int as s16;
                (*effect).fwork[1 as libc::c_int as usize] =
                    ((*effect).fwork[0 as libc::c_int as usize] -
                         (*effect).scale) * 0.1f32
            }
            break ;
        } else { i += 1; effect = effect.offset(1) }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossMo_SpawnDroplet(mut type_0: s16,
                                             mut effect: *mut BossMoEffect,
                                             mut pos: *mut Vec3f,
                                             mut vel: *mut Vec3f,
                                             mut scale: f32_0) {
    let mut i: s16 = 0;
    let mut gravity: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: -1.0f32, z: 0.0f32,}; init };
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < 290 as libc::c_int {
        if (*effect).type_0 as libc::c_int == MO_FX_NONE as libc::c_int {
            (*effect).type_0 = type_0 as u8_0;
            (*effect).pos = *pos;
            (*effect).vel = *vel;
            (*effect).accel = gravity;
            if type_0 as libc::c_int == MO_FX_SPLASH_TRAIL as libc::c_int {
                (*effect).accel.y = 0.0f32
            }
            (*effect).scale = scale;
            (*effect).fwork[1 as libc::c_int as usize] = 1.0f32;
            (*effect).stopTimer = 0 as libc::c_int as u8_0;
            break ;
        } else { i += 1; effect = effect.offset(1) }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossMo_SpawnStillDroplet(mut effect:
                                                      *mut BossMoEffect,
                                                  mut pos: *mut Vec3f,
                                                  mut scale: f32_0) {
    let mut i: s16 = 0;
    let mut zeroVec: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < 290 as libc::c_int {
        if (*effect).type_0 as libc::c_int == MO_FX_NONE as libc::c_int {
            (*effect).type_0 = MO_FX_DROPLET as libc::c_int as u8_0;
            (*effect).stopTimer = 2 as libc::c_int as u8_0;
            (*effect).pos = *pos;
            (*effect).vel = zeroVec;
            (*effect).accel = zeroVec;
            (*effect).scale = scale;
            (*effect).fwork[1 as libc::c_int as usize] = 1.0f32;
            break ;
        } else { i += 1; effect = effect.offset(1) }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossMo_SpawnBubble(mut effect: *mut BossMoEffect,
                                            mut pos: *mut Vec3f,
                                            mut vel: *mut Vec3f,
                                            mut accel: *mut Vec3f,
                                            mut scale: f32_0,
                                            mut targetPos: *mut Vec3f) {
    let mut i: s16 = 0;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < 280 as libc::c_int {
        if (*effect).type_0 as libc::c_int == MO_FX_NONE as libc::c_int {
            (*effect).type_0 = MO_FX_BUBBLE as libc::c_int as u8_0;
            (*effect).stopTimer = 0 as libc::c_int as u8_0;
            (*effect).pos = *pos;
            (*effect).vel = *vel;
            (*effect).accel = *accel;
            (*effect).scale = scale;
            (*effect).fwork[0 as libc::c_int as usize] = 0.0f32;
            (*effect).targetPos = targetPos;
            if targetPos.is_null() {
                (*effect).alpha = 255 as libc::c_int as s16
            } else { (*effect).alpha = 0 as libc::c_int as s16 }
            (*effect).timer = 0 as libc::c_int as u8_0;
            break ;
        } else { i += 1; effect = effect.offset(1) }
    };
}
static mut sCurlRot: [s16; 41] =
    [0 as libc::c_int as s16, 0 as libc::c_int as s16,
     0 as libc::c_int as s16, 0 as libc::c_int as s16,
     0 as libc::c_int as s16, 0 as libc::c_int as s16,
     0 as libc::c_int as s16, 0 as libc::c_int as s16,
     0 as libc::c_int as s16, 0 as libc::c_int as s16,
     0 as libc::c_int as s16, 0 as libc::c_int as s16,
     0 as libc::c_int as s16, 2 as libc::c_int as s16,
     4 as libc::c_int as s16, 8 as libc::c_int as s16,
     8 as libc::c_int as s16, 8 as libc::c_int as s16,
     9 as libc::c_int as s16, 9 as libc::c_int as s16,
     9 as libc::c_int as s16, 9 as libc::c_int as s16,
     9 as libc::c_int as s16, 9 as libc::c_int as s16,
     12 as libc::c_int as s16, 15 as libc::c_int as s16,
     15 as libc::c_int as s16, 15 as libc::c_int as s16,
     15 as libc::c_int as s16, 15 as libc::c_int as s16,
     15 as libc::c_int as s16, 15 as libc::c_int as s16,
     20 as libc::c_int as s16, 20 as libc::c_int as s16,
     20 as libc::c_int as s16, 0 as libc::c_int as s16,
     0 as libc::c_int as s16, 0 as libc::c_int as s16,
     0 as libc::c_int as s16, 0 as libc::c_int as s16,
     0 as libc::c_int as s16];
static mut sGrabRot: [s16; 41] =
    [0 as libc::c_int as s16, 0 as libc::c_int as s16,
     0 as libc::c_int as s16, 0 as libc::c_int as s16,
     0 as libc::c_int as s16, 0 as libc::c_int as s16,
     0 as libc::c_int as s16, 0 as libc::c_int as s16,
     0 as libc::c_int as s16, 0 as libc::c_int as s16,
     0 as libc::c_int as s16, 0 as libc::c_int as s16,
     0 as libc::c_int as s16, 0 as libc::c_int as s16,
     0 as libc::c_int as s16, 0 as libc::c_int as s16,
     0 as libc::c_int as s16, 0 as libc::c_int as s16,
     -(5 as libc::c_int) as s16, -(5 as libc::c_int) as s16,
     -(5 as libc::c_int) as s16, 0 as libc::c_int as s16,
     5 as libc::c_int as s16, 10 as libc::c_int as s16,
     20 as libc::c_int as s16, 20 as libc::c_int as s16,
     20 as libc::c_int as s16, 20 as libc::c_int as s16,
     20 as libc::c_int as s16, 20 as libc::c_int as s16,
     20 as libc::c_int as s16, 20 as libc::c_int as s16,
     20 as libc::c_int as s16, 20 as libc::c_int as s16,
     20 as libc::c_int as s16, 20 as libc::c_int as s16,
     0 as libc::c_int as s16, 0 as libc::c_int as s16,
     0 as libc::c_int as s16, 0 as libc::c_int as s16,
     0 as libc::c_int as s16];
static mut sAttackRot: [s16; 41] =
    [0 as libc::c_int as s16, 5 as libc::c_int as s16,
     6 as libc::c_int as s16, 7 as libc::c_int as s16,
     8 as libc::c_int as s16, 8 as libc::c_int as s16,
     7 as libc::c_int as s16, 6 as libc::c_int as s16,
     6 as libc::c_int as s16, 2 as libc::c_int as s16,
     2 as libc::c_int as s16, 2 as libc::c_int as s16,
     1 as libc::c_int as s16, 1 as libc::c_int as s16,
     1 as libc::c_int as s16, 1 as libc::c_int as s16,
     1 as libc::c_int as s16, 1 as libc::c_int as s16,
     1 as libc::c_int as s16, 1 as libc::c_int as s16,
     1 as libc::c_int as s16, 0 as libc::c_int as s16,
     0 as libc::c_int as s16, 0 as libc::c_int as s16,
     0 as libc::c_int as s16, 0 as libc::c_int as s16,
     0 as libc::c_int as s16, 0 as libc::c_int as s16,
     0 as libc::c_int as s16, 0 as libc::c_int as s16,
     0 as libc::c_int as s16, 0 as libc::c_int as s16,
     0 as libc::c_int as s16, 0 as libc::c_int as s16,
     0 as libc::c_int as s16, 0 as libc::c_int as s16,
     0 as libc::c_int as s16, 0 as libc::c_int as s16,
     0 as libc::c_int as s16, 0 as libc::c_int as s16,
     0 as libc::c_int as s16];
// Initialized in run_static_initializers
static mut sInitChain: [InitChainEntry; 4] =
    [InitChainEntry{cont_type_0_offset_value: [0; 4],}; 4];
static mut sAudioZeroVec: Vec3f =
    { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
static mut sTentSpawnIndex: [u8_0; 21] =
    [0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     2 as libc::c_int as u8_0, 3 as libc::c_int as u8_0,
     4 as libc::c_int as u8_0, 15 as libc::c_int as u8_0,
     19 as libc::c_int as u8_0, 5 as libc::c_int as u8_0,
     14 as libc::c_int as u8_0, 16 as libc::c_int as u8_0,
     17 as libc::c_int as u8_0, 18 as libc::c_int as u8_0,
     6 as libc::c_int as u8_0, 13 as libc::c_int as u8_0,
     20 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     12 as libc::c_int as u8_0, 11 as libc::c_int as u8_0,
     10 as libc::c_int as u8_0, 9 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0];
static mut sTentSpawnPos: [Vec2f; 21] =
    [{ let mut init = Vec2f{x: -360.0f32, y: -360.0f32,}; init },
     { let mut init = Vec2f{x: -180.0f32, y: -360.0f32,}; init },
     { let mut init = Vec2f{x: 0.0f32, y: -360.0f32,}; init },
     { let mut init = Vec2f{x: 180.0f32, y: -360.0f32,}; init },
     { let mut init = Vec2f{x: 360.0f32, y: -360.0f32,}; init },
     { let mut init = Vec2f{x: -360.0f32, y: -180.0f32,}; init },
     { let mut init = Vec2f{x: 0.0f32, y: -180.0f32,}; init },
     { let mut init = Vec2f{x: 360.0f32, y: -180.0f32,}; init },
     { let mut init = Vec2f{x: -360.0f32, y: 0.0f32,}; init },
     { let mut init = Vec2f{x: -180.0f32, y: 0.0f32,}; init },
     { let mut init = Vec2f{x: 0.0f32, y: 0.0f32,}; init },
     { let mut init = Vec2f{x: 180.0f32, y: 0.0f32,}; init },
     { let mut init = Vec2f{x: 360.0f32, y: 0.0f32,}; init },
     { let mut init = Vec2f{x: -360.0f32, y: 180.0f32,}; init },
     { let mut init = Vec2f{x: 0.0f32, y: 180.0f32,}; init },
     { let mut init = Vec2f{x: 360.0f32, y: 180.0f32,}; init },
     { let mut init = Vec2f{x: -360.0f32, y: 360.0f32,}; init },
     { let mut init = Vec2f{x: -180.0f32, y: 360.0f32,}; init },
     { let mut init = Vec2f{x: 0.0f32, y: 360.0f32,}; init },
     { let mut init = Vec2f{x: 180.0f32, y: 360.0f32,}; init },
     { let mut init = Vec2f{x: 360.0f32, y: 360.0f32,}; init }];
static mut sTentWidth: [f32_0; 41] =
    [3.56f32, 3.25f32, 2.96f32, 2.69f32, 2.44f32, 2.21f32, 2.0f32, 1.81f32,
     1.64f32, 1.49f32, 1.36f32, 1.25f32, 1.16f32, 1.09f32, 1.04f32, 1.01f32,
     1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32,
     1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 0.98f32, 0.95f32,
     0.9f32, 0.8f32, 0.6f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32];
static mut sDropletWidth: [f32_0; 41] =
    [0.0f32, 2.95804f32, 4.123106f32, 4.974937f32, 5.656854f32, 6.22495f32,
     6.708204f32, 7.123903f32, 7.483315f32, 7.794229f32, 8.062258f32,
     8.291562f32, 8.485281f32, 8.645808f32, 8.774964f32, 8.87412f32,
     8.944272f32, 8.9861f32, 9.0f32, 8.9861f32, 8.944272f32, 8.87412f32,
     8.774964f32, 8.645808f32, 8.485281f32, 8.291562f32, 8.062258f32,
     7.794229f32, 7.483315f32, 7.123903f32, 6.708204f32, 6.22495f32,
     5.656854f32, 4.974937f32, 4.123106f32, 2.95804f32, 0.0f32, 0.0f32,
     0.0f32, 0.0f32, 0.0f32];
// These are sqrt(9^2 - (i/2 - 9)^2), a sphere of radius 9.
#[no_mangle]
pub unsafe extern "C" fn BossMo_Init(mut thisx: *mut Actor,
                                     mut globalCtx2: *mut GlobalContext) {
    let mut globalCtx: *mut GlobalContext = globalCtx2;
    let mut this: *mut BossMo = thisx as *mut BossMo;
    let mut i: u16_0 = 0;
    Actor_ProcessInitChain(&mut (*this).actor, sInitChain.as_mut_ptr());
    ActorShape_Init(&mut (*this).actor.shape, 0.0f32, None, 0.0f32);
    if (*this).actor.params as libc::c_int != 100 as libc::c_int {
        Flags_SetSwitch(globalCtx, 0x14 as libc::c_int);
        sMorphaCore = this;
        (*this).waterLevel =
            (*(*(*globalCtx).colCtx.colHeader).waterBoxes.offset(0 as
                                                                     libc::c_int
                                                                     as
                                                                     isize)).ySurface
                as f32_0;
        (*(*(*globalCtx).colCtx.colHeader).waterBoxes.offset(0 as libc::c_int
                                                                 as
                                                                 isize)).ySurface
            = (*this).waterLevel as s16;
        (*globalCtx).roomCtx.unk_74[0 as libc::c_int as usize] =
            0xa0 as libc::c_int as s16;
        (*globalCtx).specialEffects =
            sEffects.as_mut_ptr() as *mut libc::c_void;
        i = 0 as libc::c_int as u16_0;
        while (i as libc::c_int) <
                  (::std::mem::size_of::<[BossMoEffect; 300]>() as
                       libc::c_ulong).wrapping_div(::std::mem::size_of::<BossMoEffect>()
                                                       as libc::c_ulong) as
                      s32 {
            sEffects[i as usize].type_0 = MO_FX_NONE as libc::c_int as u8_0;
            i = i.wrapping_add(1)
        }
        (*this).actor.world.pos.x = 200.0f32;
        (*this).actor.world.pos.y =
            (*(*(*globalCtx).colCtx.colHeader).waterBoxes.offset(0 as
                                                                     libc::c_int
                                                                     as
                                                                     isize)).ySurface
                as libc::c_int as libc::c_float + 50.0f32;
        (*this).fwork[MO_TENT_SWING_SIZE_X as libc::c_int as usize] = 5.0f32;
        (*this).drawActor = 1 as libc::c_int as u8_0;
        (*this).actor.colChkInfo.health = 20 as libc::c_int as u8_0;
        (*this).actor.colChkInfo.mass = 0 as libc::c_int as u8_0;
        (*this).actor.params = 0 as libc::c_int as s16;
        Actor_SetScale(&mut (*this).actor, 0.01f32);
        Collider_InitCylinder(globalCtx, &mut (*this).coreCollider);
        Collider_SetCylinder(globalCtx, &mut (*this).coreCollider,
                             &mut (*this).actor, &mut sCylinderInit);
        if Flags_GetClear(globalCtx, (*globalCtx).roomCtx.curRoom.num as s32)
               != 0 {
            Actor_Kill(&mut (*this).actor);
            Actor_SpawnAsChild(&mut (*globalCtx).actorCtx, &mut (*this).actor,
                               globalCtx,
                               ACTOR_DOOR_WARP1 as libc::c_int as s16, 0.0f32,
                               -280.0f32, 0.0f32, 0 as libc::c_int as s16,
                               0 as libc::c_int as s16,
                               0 as libc::c_int as s16,
                               WARP_DUNGEON_ADULT as libc::c_int as s16);
            Actor_Spawn(&mut (*globalCtx).actorCtx, globalCtx,
                        ACTOR_ITEM_B_HEART as libc::c_int as s16, -200.0f32,
                        -280.0f32, 0.0f32, 0 as libc::c_int as s16,
                        0 as libc::c_int as s16, 0 as libc::c_int as s16,
                        0 as libc::c_int as s16);
            (*globalCtx).roomCtx.unk_74[0 as libc::c_int as usize] =
                0xff as libc::c_int as s16;
            (*(*(*globalCtx).colCtx.colHeader).waterBoxes.offset(0 as
                                                                     libc::c_int
                                                                     as
                                                                     isize)).ySurface
                = -(500 as libc::c_int) as s16;
            return
        }
        if gSaveContext.eventChkInf[7 as libc::c_int as usize] as libc::c_int
               & 0x10 as libc::c_int != 0 {
            Audio_QueueSeqCmd(((SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                                   24 as libc::c_int | 0x1b as libc::c_int) as
                                  u32_0);
            (*this).tentMaxAngle = 5.0f32;
            (*this).timers[0 as libc::c_int as usize] =
                50 as libc::c_int as s16
        } else {
            (*this).csState = MO_INTRO_WAIT as libc::c_int as s16;
            (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] =
                MO_CORE_INTRO_WAIT as libc::c_int as s16;
            (*this).actor.world.pos.x = 1000.0f32;
            (*this).timers[0 as libc::c_int as usize] =
                60 as libc::c_int as s16
        }
        sMorphaTent1 =
            Actor_SpawnAsChild(&mut (*globalCtx).actorCtx, &mut (*this).actor,
                               globalCtx, ACTOR_BOSS_MO as libc::c_int as s16,
                               (*this).actor.world.pos.x,
                               (*this).actor.world.pos.y,
                               (*this).actor.world.pos.z,
                               0 as libc::c_int as s16,
                               0 as libc::c_int as s16,
                               0 as libc::c_int as s16,
                               100 as libc::c_int as s16) as *mut BossMo;
        (*this).actor.draw =
            Some(BossMo_DrawCore as
                     unsafe extern "C" fn(_: *mut Actor,
                                          _: *mut GlobalContext) -> ());
        (*this).actor.update =
            Some(BossMo_UpdateCore as
                     unsafe extern "C" fn(_: *mut Actor,
                                          _: *mut GlobalContext) -> ());
        Actor_ChangeCategory(globalCtx, &mut (*globalCtx).actorCtx,
                             &mut (*this).actor,
                             ACTORCAT_BOSS as libc::c_int as u8_0);
    } else {
        Actor_SetScale(&mut (*this).actor, 0.01f32);
        BossMo_SetupTentacle(this, globalCtx);
        (*this).actor.colChkInfo.mass = 0xff as libc::c_int as u8_0;
        (*(*(*globalCtx).colCtx.colHeader).waterBoxes.offset(0 as libc::c_int
                                                                 as
                                                                 isize)).ySurface
            = -(50 as libc::c_int) as s16;
        (*this).waterTexAlpha = 90.0f32;
        (*this).actor.world.pos.y =
            (*(*(*globalCtx).colCtx.colHeader).waterBoxes.offset(0 as
                                                                     libc::c_int
                                                                     as
                                                                     isize)).ySurface
                as f32_0;
        (*this).targetPos = (*this).actor.world.pos;
        (*this).actor.prevPos = (*this).targetPos;
        Collider_InitJntSph(globalCtx, &mut (*this).tentCollider);
        Collider_SetJntSph(globalCtx, &mut (*this).tentCollider,
                           &mut (*this).actor, &mut sJntSphInit,
                           (*this).tentElements.as_mut_ptr());
        (*this).tentMaxAngle = 1.0f32
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossMo_Destroy(mut thisx: *mut Actor,
                                        mut globalCtx: *mut GlobalContext) {
    let mut pad: s32 = 0;
    let mut this: *mut BossMo = thisx as *mut BossMo;
    if (*this).actor.params as libc::c_int >= 100 as libc::c_int {
        Collider_DestroyJntSph(globalCtx, &mut (*this).tentCollider);
    } else {
        Collider_DestroyCylinder(globalCtx, &mut (*this).coreCollider);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossMo_SetupTentacle(mut this: *mut BossMo,
                                              mut globalCtx:
                                                  *mut GlobalContext) {
    (*this).actionFunc =
        Some(BossMo_Tentacle as
                 unsafe extern "C" fn(_: *mut BossMo, _: *mut GlobalContext)
                     -> ());
    (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] =
        MO_TENT_WAIT as libc::c_int as s16;
    (*this).timers[0 as libc::c_int as usize] =
        (50 as libc::c_int + Rand_ZeroFloat(20.0f32) as s16 as libc::c_int) as
            s16;
}
#[no_mangle]
pub unsafe extern "C" fn BossMo_Tentacle(mut this: *mut BossMo,
                                         mut globalCtx: *mut GlobalContext) {
    let mut tentXrot: s16 = 0;
    let mut sp1B4: s16 = 0 as libc::c_int as s16;
    let mut buttons: s32 = 0;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut indS0: s16 = 0;
    let mut indS1: s16 = 0;
    let mut camera1: *mut Camera = 0 as *mut Camera;
    let mut camera2: *mut Camera = 0 as *mut Camera;
    let mut otherTent: *mut BossMo = (*this).otherTent as *mut BossMo;
    let mut maxSwingRateX: f32_0 = 0.;
    let mut maxSwingLagX: f32_0 = 0.;
    let mut maxSwingSizeX: f32_0 = 0.;
    let mut maxSwingRateZ: f32_0 = 0.;
    let mut maxSwingLagZ: f32_0 = 0.;
    let mut maxSwingSizeZ: f32_0 = 0.;
    let mut swingRateAccel: f32_0 = 0.;
    let mut swingSizeAccel: f32_0 = 0.;
    let mut rippleCount: s16 = 0;
    let mut indT5: s16 = 0;
    let mut ripplePos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut randAngle: f32_0 = 0.;
    let mut randFloat: f32_0 = 0.;
    let mut tempf1: f32_0 = 0.;
    let mut tempf2: f32_0 = 0.;
    let mut sin: f32_0 = 0.;
    let mut cos: f32_0 = 0.;
    let mut temp: f32_0 = 0.;
    let mut dx: f32_0 = 0.;
    let mut dy: f32_0 = 0.;
    let mut dz: f32_0 = 0.;
    let mut sp138: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp12C: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp120: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut pad11C: s32 = 0;
    let mut pad118: s32 = 0;
    let mut pad114: s32 = 0;
    let mut pad110: s32 = 0;
    let mut pad10C: s32 = 0;
    let mut pad108: s32 = 0;
    let mut spFC: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut spF0: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut padEC: f32_0 = 0.;
    let mut spE0: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut spD4: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut spC8: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    if (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
           libc::c_int <= MO_TENT_DEATH_3 as libc::c_int {
        (*this).actor.world.pos.y =
            (*(*(*globalCtx).colCtx.colHeader).waterBoxes.offset(0 as
                                                                     libc::c_int
                                                                     as
                                                                     isize)).ySurface
                as f32_0
    }
    if (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
           libc::c_int == MO_TENT_READY as libc::c_int ||
           (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
               libc::c_int >= MO_TENT_DEATH_START as libc::c_int ||
           (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
               libc::c_int == MO_TENT_RETREAT as libc::c_int ||
           (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
               libc::c_int == MO_TENT_SWING as libc::c_int ||
           (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
               libc::c_int == MO_TENT_SHAKE as libc::c_int {
        if (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
               libc::c_int == MO_TENT_READY as libc::c_int {
            if (*sMorphaCore).csState as libc::c_int !=
                   MO_BATTLE as libc::c_int {
                maxSwingRateX = 2000.0f32;
                maxSwingLagX = 3000.0f32;
                maxSwingSizeX = 1000.0f32;
                maxSwingRateZ = 1500.0f32;
                maxSwingLagZ = 2500.0f32;
                maxSwingSizeZ = 1000.0f32;
                swingRateAccel = 10.0f32;
                swingSizeAccel = 10.0f32
            } else {
                maxSwingRateX = 2000.0f32;
                maxSwingLagX = 3000.0f32;
                maxSwingSizeX = 1000.0f32;
                maxSwingRateZ = 1500.0f32;
                maxSwingLagZ = 2500.0f32;
                maxSwingSizeZ = 1000.0f32;
                swingRateAccel = 20.0f32;
                swingSizeAccel = 30.0f32
            }
        } else if (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize]
                      as libc::c_int == MO_TENT_SWING as libc::c_int {
            maxSwingRateX = 2500.0f32;
            maxSwingLagX = -1000.0f32;
            maxSwingSizeX = 3000.0f32;
            maxSwingRateZ = 1500.0f32;
            maxSwingLagZ = 2500.0f32;
            maxSwingSizeZ = 0.0f64 as f32_0;
            swingRateAccel = 30.0f32;
            swingSizeAccel = 60.0f32;
            if (*this).sfxTimer as libc::c_int % 16 as libc::c_int ==
                   0 as libc::c_int &&
                   ((*this).timers[0 as libc::c_int as usize] as libc::c_int)
                       < 30 as libc::c_int {
                Audio_PlaySoundIncreasinglyTransposed(&mut (*this).tentTipPos,
                                                      0x38f2 as libc::c_int as
                                                          s16,
                                                      gMorphaTransposeTable.as_mut_ptr());
            }
        } else if (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize]
                      as libc::c_int == MO_TENT_SHAKE as libc::c_int {
            if (*this).timers[0 as libc::c_int as usize] as libc::c_int >
                   40 as libc::c_int {
                maxSwingRateX = 1300.0f32;
                maxSwingLagX = -3200.0f32;
                maxSwingSizeX = 7000.0f32;
                maxSwingRateZ = 800.0f32;
                maxSwingLagZ = 2500.0f32;
                maxSwingSizeZ = 5000.0f32;
                swingRateAccel = 30.0f32;
                swingSizeAccel = 60.0f32;
                if (*this).sfxTimer as libc::c_int % 32 as libc::c_int ==
                       0 as libc::c_int {
                    Audio_PlaySoundIncreasinglyTransposed(&mut (*this).tentTipPos,
                                                          0x38f2 as
                                                              libc::c_int as
                                                              s16,
                                                          gMorphaTransposeTable.as_mut_ptr());
                    func_800AA000(0 as libc::c_int as f32_0,
                                  100 as libc::c_int as u8_0,
                                  5 as libc::c_int as u8_0,
                                  2 as libc::c_int as u8_0);
                    func_8002F7DC(&mut (*player).actor,
                                  (0x6806 as libc::c_int +
                                       (*(*player).ageProperties).unk_92 as
                                           libc::c_int) as u16_0);
                }
            } else {
                maxSwingRateX = 2000.0f32;
                maxSwingLagX = -1000.0f32;
                maxSwingSizeX = 5000.0f32;
                maxSwingRateZ = 1500.0f32;
                maxSwingLagZ = 2500.0f32;
                maxSwingSizeZ = 100.0f32;
                swingRateAccel = 70.0f32;
                swingSizeAccel = 70.0f32;
                if (*this).sfxTimer as libc::c_int % 16 as libc::c_int ==
                       0 as libc::c_int {
                    Audio_PlaySoundIncreasinglyTransposed(&mut (*this).tentTipPos,
                                                          0x38f2 as
                                                              libc::c_int as
                                                              s16,
                                                          gMorphaTransposeTable.as_mut_ptr());
                    func_800AA000(0 as libc::c_int as f32_0,
                                  160 as libc::c_int as u8_0,
                                  5 as libc::c_int as u8_0,
                                  4 as libc::c_int as u8_0);
                    func_8002F7DC(&mut (*player).actor,
                                  (0x6806 as libc::c_int +
                                       (*(*player).ageProperties).unk_92 as
                                           libc::c_int) as u16_0);
                }
            }
        } else if (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize]
                      as libc::c_int == MO_TENT_RETREAT as libc::c_int {
            maxSwingRateX = 1300.0f32;
            maxSwingLagX = 3200.0f32;
            maxSwingSizeX = 7000.0f32;
            maxSwingRateZ = 800.0f32;
            maxSwingLagZ = 2500.0f32;
            maxSwingSizeZ = 5000.0f32;
            swingRateAccel = 30.0f32;
            swingSizeAccel = 30.0f32
        } else if (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize]
                      as libc::c_int >= MO_TENT_DEATH_START as libc::c_int {
            maxSwingRateX = -400.0f32;
            maxSwingLagX = -3200.0f32;
            maxSwingSizeX = 0.0f32;
            maxSwingRateZ = 2300.0f32;
            maxSwingLagZ = 3200.0f32;
            maxSwingSizeZ = 1000.0f64 as f32_0;
            swingRateAccel = 30.0f32;
            swingSizeAccel = 60.0f32
        }
        Math_ApproachF(&mut *(*this).fwork.as_mut_ptr().offset(MO_TENT_SWING_RATE_X
                                                                   as
                                                                   libc::c_int
                                                                   as isize),
                       maxSwingRateX, 1.0f32, swingRateAccel);
        Math_ApproachF(&mut *(*this).fwork.as_mut_ptr().offset(MO_TENT_SWING_LAG_X
                                                                   as
                                                                   libc::c_int
                                                                   as isize),
                       maxSwingLagX, 1.0f32, 30.0f32);
        Math_ApproachF(&mut *(*this).fwork.as_mut_ptr().offset(MO_TENT_SWING_SIZE_X
                                                                   as
                                                                   libc::c_int
                                                                   as isize),
                       maxSwingSizeX, 1.0f32, swingSizeAccel);
        Math_ApproachF(&mut *(*this).fwork.as_mut_ptr().offset(MO_TENT_SWING_RATE_Z
                                                                   as
                                                                   libc::c_int
                                                                   as isize),
                       maxSwingRateZ, 1.0f32, swingRateAccel);
        Math_ApproachF(&mut *(*this).fwork.as_mut_ptr().offset(MO_TENT_SWING_LAG_Z
                                                                   as
                                                                   libc::c_int
                                                                   as isize),
                       maxSwingLagZ, 1.0f32, 30.0f32);
        Math_ApproachF(&mut *(*this).fwork.as_mut_ptr().offset(MO_TENT_SWING_SIZE_Z
                                                                   as
                                                                   libc::c_int
                                                                   as isize),
                       maxSwingSizeZ, 1.0f32, swingSizeAccel);
        (*this).xSwing =
            ((*this).xSwing as libc::c_int +
                 (*this).fwork[MO_TENT_SWING_RATE_X as libc::c_int as usize]
                     as s16 as libc::c_int) as s16;
        (*this).zSwing =
            ((*this).zSwing as libc::c_int +
                 (*this).fwork[MO_TENT_SWING_RATE_Z as libc::c_int as usize]
                     as s16 as libc::c_int) as s16
    }
    let mut current_block_601: u64;
    match (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
              libc::c_int {
        10 => {
            (*this).actor.flags &=
                !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
            if this == sMorphaTent2 {
                (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] =
                    MO_TENT_SPAWN as libc::c_int as s16;
                (*this).timers[0 as libc::c_int as usize] =
                    70 as libc::c_int as s16;
                (*this).actor.shape.rot.y = (*this).actor.yawTowardsPlayer
            }
            current_block_601 = 7905081044627426364;
        }
        11 => {
            (*this).drawActor = 1 as libc::c_int as u8_0;
            (*this).baseBubblesTimer = 20 as libc::c_int as s16;
            if ((*this).timers[0 as libc::c_int as usize] as libc::c_int) <
                   20 as libc::c_int {
                Math_ApproachF(&mut (*this).tentRippleSize, 0.15f32, 0.5f32,
                               0.01f64 as f32_0);
                Math_ApproachF(&mut (*this).baseAlpha, 150.0f32, 1.0f32,
                               5.0f32);
                if (*this).baseAlpha >= 150.0f32 {
                    (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize]
                        = MO_TENT_READY as libc::c_int as s16;
                    (*this).timers[0 as libc::c_int as usize] =
                        60 as libc::c_int as s16
                }
            }
            if (*this).timers[0 as libc::c_int as usize] as libc::c_int >
                   50 as libc::c_int {
                rippleCount = 1 as libc::c_int as s16
            } else if (*this).timers[0 as libc::c_int as usize] as libc::c_int
                          > 40 as libc::c_int {
                rippleCount = 3 as libc::c_int as s16
            } else if (*this).timers[0 as libc::c_int as usize] as libc::c_int
                          > 30 as libc::c_int {
                rippleCount = 5 as libc::c_int as s16
            } else if (*this).timers[0 as libc::c_int as usize] as libc::c_int
                          > 20 as libc::c_int {
                rippleCount = 8 as libc::c_int as s16
            } else { rippleCount = 3 as libc::c_int as s16 }
            indS1 = 0 as libc::c_int as s16;
            while (indS1 as libc::c_int) < rippleCount as libc::c_int {
                randFloat = Rand_ZeroFloat(50.0f32);
                randAngle = Rand_ZeroFloat(0x10000 as libc::c_int as f32_0);
                ripplePos = (*this).actor.world.pos;
                ripplePos.x += sinf(randAngle) * randFloat;
                ripplePos.z += cosf(randAngle) * randFloat;
                ripplePos.y =
                    (*(*(*globalCtx).colCtx.colHeader).waterBoxes.offset(0 as
                                                                             libc::c_int
                                                                             as
                                                                             isize)).ySurface
                        as f32_0;
                BossMo_SpawnRipple((*globalCtx).specialEffects as
                                       *mut BossMoEffect, &mut ripplePos,
                                   40.0f32, 110.0f32,
                                   80 as libc::c_int as s16,
                                   290 as libc::c_int as s16,
                                   MO_FX_SMALL_RIPPLE as libc::c_int as u8_0);
                indS1 += 1
            }
            current_block_601 = 7905081044627426364;
        }
        0 | 1 => {
            if (*sMorphaCore).csState as libc::c_int ==
                   MO_BATTLE as libc::c_int {
                func_80078914(&mut (*this).tentTipPos,
                              (0x38f0 as libc::c_int - 0x800 as libc::c_int)
                                  as u16_0);
            }
            Math_ApproachF(&mut (*this).waterLevelMod, -5.0f32, 0.1f32,
                           0.4f32);
            indS1 = 0 as libc::c_int as s16;
            while (indS1 as libc::c_int) < 41 as libc::c_int {
                sin =
                    Math_SinS(((*this).fwork[MO_TENT_SWING_LAG_X as
                                                 libc::c_int as usize] as s16
                                   as libc::c_int * indS1 as libc::c_int +
                                   (*this).xSwing as libc::c_int) as s16);
                tempf1 =
                    (*this).fwork[MO_TENT_SWING_SIZE_X as libc::c_int as
                                      usize] *
                        (indS1 as libc::c_int as libc::c_float * 0.025f32 *
                             sin);
                cos =
                    Math_SinS(((*this).fwork[MO_TENT_SWING_LAG_Z as
                                                 libc::c_int as usize] as s16
                                   as libc::c_int * indS1 as libc::c_int +
                                   (*this).zSwing as libc::c_int) as s16);
                tempf2 =
                    (*this).fwork[MO_TENT_SWING_SIZE_Z as libc::c_int as
                                      usize] *
                        (indS1 as libc::c_int as libc::c_float * 0.025f32 *
                             cos);
                Math_ApproachF(&mut (*(*this).tentStretch.as_mut_ptr().offset(indS1
                                                                                  as
                                                                                  isize)).y,
                               (*this).fwork[MO_TENT_MAX_STRETCH as
                                                 libc::c_int as usize] *
                                   5.0f32, 0.1f32, 0.4f32);
                if indS1 as libc::c_int == 28 as libc::c_int {
                    sp1B4 = (*this).tentRot[indS1 as usize].x
                }
                Math_ApproachS(&mut (*(*this).tentRot.as_mut_ptr().offset(indS1
                                                                              as
                                                                              isize)).x,
                               tempf1 as s16,
                               (1.0f32 / (*this).tentMaxAngle) as s16,
                               (*this).tentSpeed as s16);
                Math_ApproachS(&mut (*(*this).tentRot.as_mut_ptr().offset(indS1
                                                                              as
                                                                              isize)).z,
                               tempf2 as s16,
                               (1.0f32 / (*this).tentMaxAngle) as s16,
                               (*this).tentSpeed as s16);
                indS1 += 1
            }
            (*this).targetPos = (*this).actor.world.pos;
            Math_ApproachF(&mut (*this).actor.speedXZ, 0.75f32, 1.0f32,
                           0.04f32);
            if (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
                   libc::c_int == MO_TENT_SWING as libc::c_int {
                Math_ApproachS(&mut (*this).actor.shape.rot.y,
                               ((*this).actor.yawTowardsPlayer as libc::c_int
                                    + (*this).attackAngleMod as libc::c_int)
                                   as s16, 0xa as libc::c_int as s16,
                               0x1f4 as libc::c_int as s16);
            }
            Math_ApproachF(&mut *(*this).fwork.as_mut_ptr().offset(MO_TENT_MAX_STRETCH
                                                                       as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                           1.0f32, 0.5f32, 0.04f64 as f32_0);
            if (*sMorphaCore).csState as libc::c_int !=
                   MO_BATTLE as libc::c_int {
                Math_ApproachF(&mut (*this).tentMaxAngle, 1.0f32, 1.0f32,
                               0.001f32);
                Math_ApproachF(&mut (*this).tentSpeed, 240.0f32, 1.0f32,
                               3.0f64 as f32_0);
            } else {
                Math_ApproachF(&mut (*this).tentMaxAngle, 1.0f32, 1.0f32,
                               0.002f32);
                Math_ApproachF(&mut (*this).tentSpeed, 400.0f32, 1.0f32,
                               6.0f32);
            }
            if (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
                   libc::c_int == MO_TENT_READY as libc::c_int {
                if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
                       0 as libc::c_int &&
                       !(!otherTent.is_null() &&
                             ((*otherTent).work[MO_TENT_ACTION_STATE as
                                                    libc::c_int as usize] as
                                  libc::c_int == MO_TENT_GRAB as libc::c_int
                                  ||
                                  (*otherTent).work[MO_TENT_ACTION_STATE as
                                                        libc::c_int as usize]
                                      as libc::c_int ==
                                      MO_TENT_SHAKE as libc::c_int)) {
                    (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize]
                        = MO_TENT_SWING as libc::c_int as s16;
                    (*this).timers[0 as libc::c_int as usize] =
                        50 as libc::c_int as s16;
                    Audio_ResetIncreasingTranspose();
                    (*this).attackAngleMod =
                        Rand_CenteredFloat(0x1000 as libc::c_int as f32_0) as
                            s16
                }
            } else {
                tentXrot = (*this).tentRot[28 as libc::c_int as usize].x;
                if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
                       0 as libc::c_int &&
                       tentXrot as libc::c_int >= 0 as libc::c_int &&
                       (sp1B4 as libc::c_int) < 0 as libc::c_int {
                    (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize]
                        = MO_TENT_ATTACK as libc::c_int as s16;
                    if this == sMorphaTent1 {
                        (*this).timers[0 as libc::c_int as usize] =
                            175 as libc::c_int as s16
                    } else {
                        (*this).timers[0 as libc::c_int as usize] =
                            55 as libc::c_int as s16
                    }
                }
            }
            current_block_601 = 7905081044627426364;
        }
        2 => {
            (*this).actor.flags |=
                ((1 as libc::c_int) << 24 as libc::c_int) as libc::c_uint;
            func_80078914(&mut (*this).tentTipPos,
                          (0x38f1 as libc::c_int - 0x800 as libc::c_int) as
                              u16_0);
            Math_ApproachF(&mut (*this).waterLevelMod, -5.0f32, 0.1f32,
                           0.4f32);
            indS1 = 0 as libc::c_int as s16;
            while (indS1 as libc::c_int) < 41 as libc::c_int {
                Math_ApproachF(&mut (*(*this).tentStretch.as_mut_ptr().offset(indS1
                                                                                  as
                                                                                  isize)).y,
                               (*this).fwork[MO_TENT_MAX_STRETCH as
                                                 libc::c_int as usize] *
                                   ((40 as libc::c_int - indS1 as libc::c_int)
                                        as libc::c_float * 25.0f32 / 100.0f32
                                        + 5.0f32), 0.5f32, 0.7f32);
                Math_ApproachS(&mut (*(*this).tentRot.as_mut_ptr().offset(indS1
                                                                              as
                                                                              isize)).x,
                               (sAttackRot[indS1 as usize] as libc::c_int *
                                    0x100 as libc::c_int) as s16,
                               (1.0f32 / (*this).tentMaxAngle) as s16,
                               (*this).tentSpeed as s16);
                Math_ApproachS(&mut (*(*this).tentRot.as_mut_ptr().offset(indS1
                                                                              as
                                                                              isize)).z,
                               0 as libc::c_int as s16,
                               (1.0f32 / (*this).tentMaxAngle) as s16,
                               (*this).tentSpeed as s16);
                indS1 += 1
            }
            (*this).targetPos = (*this).actor.world.pos;
            Math_ApproachF(&mut (*this).tentMaxAngle, 0.5f32, 1.0f32,
                           0.01f64 as f32_0);
            Math_ApproachF(&mut (*this).tentSpeed, 160.0f32, 1.0f32, 50.0f32);
            if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
                   0 as libc::c_int ||
                   (*this).linkHitTimer as libc::c_int != 0 as libc::c_int {
                dx =
                    (*this).tentPos[22 as libc::c_int as usize].x -
                        (*player).actor.world.pos.x;
                dy =
                    (*this).tentPos[22 as libc::c_int as usize].y -
                        (*player).actor.world.pos.y;
                dz =
                    (*this).tentPos[22 as libc::c_int as usize].z -
                        (*player).actor.world.pos.z;
                if fabsf(dy) < 50.0f32 &&
                       !(!otherTent.is_null() &&
                             ((*otherTent).work[MO_TENT_ACTION_STATE as
                                                    libc::c_int as usize] as
                                  libc::c_int == MO_TENT_GRAB as libc::c_int
                                  ||
                                  (*otherTent).work[MO_TENT_ACTION_STATE as
                                                        libc::c_int as usize]
                                      as libc::c_int ==
                                      MO_TENT_SHAKE as libc::c_int)) &&
                       sqrtf(dx * dx + dy * dy + dz * dz) < 120.0f32 {
                    (*this).tentMaxAngle = 0.001f32;
                    (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize]
                        = MO_TENT_CURL as libc::c_int as s16;
                    (*this).timers[0 as libc::c_int as usize] =
                        40 as libc::c_int as s16;
                    (*this).tentSpeed = 0 as libc::c_int as f32_0;
                    if ((*this).actor.shape.rot.y as libc::c_int -
                            (*this).actor.yawTowardsPlayer as libc::c_int) as
                           s16 as libc::c_int >= 0 as libc::c_int {
                        (*this).linkToLeft = 0 as libc::c_int as s16
                    } else { (*this).linkToLeft = 1 as libc::c_int as s16 }
                } else {
                    (*this).tentMaxAngle = 0.001f32;
                    (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize]
                        = MO_TENT_READY as libc::c_int as s16;
                    (*this).tentSpeed = 0 as libc::c_int as f32_0;
                    (*this).fwork[MO_TENT_SWING_RATE_X as libc::c_int as
                                      usize] = 0 as libc::c_int as f32_0;
                    (*this).fwork[MO_TENT_SWING_RATE_Z as libc::c_int as
                                      usize] = 0 as libc::c_int as f32_0;
                    (*this).fwork[MO_TENT_SWING_SIZE_X as libc::c_int as
                                      usize] = 0 as libc::c_int as f32_0;
                    (*this).fwork[MO_TENT_SWING_SIZE_Z as libc::c_int as
                                      usize] = 0 as libc::c_int as f32_0;
                    (*this).timers[0 as libc::c_int as usize] =
                        30 as libc::c_int as s16;
                    if fabsf((*player).actor.world.pos.x -
                                 (*this).actor.world.pos.x) > 300.0f32 ||
                           (*player).actor.world.pos.y <
                               (*(*(*globalCtx).colCtx.colHeader).waterBoxes.offset(0
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)).ySurface
                                   as libc::c_int as libc::c_float ||
                           !otherTent.is_null() &&
                               ((*otherTent).work[MO_TENT_ACTION_STATE as
                                                      libc::c_int as usize] as
                                    libc::c_int == MO_TENT_GRAB as libc::c_int
                                    ||
                                    (*otherTent).work[MO_TENT_ACTION_STATE as
                                                          libc::c_int as
                                                          usize] as
                                        libc::c_int ==
                                        MO_TENT_SHAKE as libc::c_int) ||
                           fabsf((*player).actor.world.pos.z -
                                     (*this).actor.world.pos.z) > 300.0f32 {
                        (*this).work[MO_TENT_ACTION_STATE as libc::c_int as
                                         usize] =
                            MO_TENT_RETREAT as libc::c_int as s16;
                        (*this).timers[0 as libc::c_int as usize] =
                            75 as libc::c_int as s16
                    }
                }
            }
            current_block_601 = 7905081044627426364;
        }
        3 | 4 => {
            Math_ApproachF(&mut (*this).waterLevelMod, -5.0f32, 0.1f32,
                           0.4f32);
            if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
                   125 as libc::c_int {
                (*this).tentMaxAngle = 0.001f32;
                (*this).tentSpeed = 0 as libc::c_int as f32_0
            }
            indS1 = 0 as libc::c_int as s16;
            while (indS1 as libc::c_int) < 41 as libc::c_int {
                if (*this).timers[0 as libc::c_int as usize] as libc::c_int >
                       25 as libc::c_int {
                    if (*this).linkToLeft == 0 {
                        Math_ApproachS(&mut (*(*this).tentRot.as_mut_ptr().offset(indS1
                                                                                      as
                                                                                      isize)).z,
                                       (sCurlRot[indS1 as usize] as
                                            libc::c_int *
                                            0x100 as libc::c_int) as s16,
                                       (1.0f32 / (*this).tentMaxAngle) as s16,
                                       (*this).tentSpeed as s16);
                    } else {
                        Math_ApproachS(&mut (*(*this).tentRot.as_mut_ptr().offset(indS1
                                                                                      as
                                                                                      isize)).z,
                                       (sCurlRot[indS1 as usize] as
                                            libc::c_int *
                                            -(0x100 as libc::c_int)) as s16,
                                       (1.0f32 / (*this).tentMaxAngle) as s16,
                                       (*this).tentSpeed as s16);
                    }
                } else if (*this).linkToLeft == 0 {
                    Math_ApproachS(&mut (*(*this).tentRot.as_mut_ptr().offset(indS1
                                                                                  as
                                                                                  isize)).z,
                                   (sGrabRot[indS1 as usize] as libc::c_int *
                                        0x100 as libc::c_int) as s16,
                                   (1.0f32 / (*this).tentMaxAngle) as s16,
                                   (*this).tentSpeed as s16);
                } else {
                    Math_ApproachS(&mut (*(*this).tentRot.as_mut_ptr().offset(indS1
                                                                                  as
                                                                                  isize)).z,
                                   (sGrabRot[indS1 as usize] as libc::c_int *
                                        -(0x100 as libc::c_int)) as s16,
                                   (1.0f32 / (*this).tentMaxAngle) as s16,
                                   (*this).tentSpeed as s16);
                }
                indS1 += 1
            }
            Math_ApproachF(&mut (*this).tentMaxAngle, 0.1f32, 1.0f32,
                           0.01f32);
            Math_ApproachF(&mut (*this).tentSpeed, 960.0f32, 1.0f32, 30.0f32);
            if (*this).timers[0 as libc::c_int as usize] as libc::c_int >=
                   30 as libc::c_int {
                Math_ApproachS(&mut (*this).actor.shape.rot.y,
                               (*this).actor.yawTowardsPlayer,
                               5 as libc::c_int as s16,
                               0xc8 as libc::c_int as s16);
            }
            if (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
                   libc::c_int == MO_TENT_CURL as libc::c_int {
                if (*this).timers[0 as libc::c_int as usize] as libc::c_int >=
                       5 as libc::c_int &&
                       (*this).linkHitTimer as libc::c_int != 0 as libc::c_int
                       && (*player).actor.parent.is_null() {
                    if (*globalCtx).grabPlayer.expect("non-null function pointer")(globalCtx,
                                                                                   player)
                           != 0 {
                        (*player).actor.parent = &mut (*this).actor;
                        (*this).work[MO_TENT_ACTION_STATE as libc::c_int as
                                         usize] =
                            MO_TENT_GRAB as libc::c_int as s16;
                        func_80078914(&mut (*this).tentTipPos,
                                      0x38f3 as libc::c_int as u16_0);
                        Audio_PlaySoundGeneral(0x6805 as libc::c_int as u16_0,
                                               &mut (*player).actor.projectedPos,
                                               4 as libc::c_int as u8_0,
                                               &mut D_801333E0,
                                               &mut D_801333E0,
                                               &mut D_801333E8);
                    } else {
                        (*this).work[MO_TENT_ACTION_STATE as libc::c_int as
                                         usize] =
                            MO_TENT_READY as libc::c_int as s16;
                        (*this).tentMaxAngle = 0.001f32;
                        (*this).tentSpeed = 0 as libc::c_int as f32_0;
                        (*this).fwork[MO_TENT_SWING_SIZE_Z as libc::c_int as
                                          usize] = 0 as libc::c_int as f32_0;
                        (*this).fwork[MO_TENT_SWING_SIZE_X as libc::c_int as
                                          usize] = 0 as libc::c_int as f32_0;
                        (*this).fwork[MO_TENT_SWING_RATE_Z as libc::c_int as
                                          usize] = 0 as libc::c_int as f32_0;
                        (*this).fwork[MO_TENT_SWING_RATE_X as libc::c_int as
                                          usize] = 0 as libc::c_int as f32_0;
                        (*this).timers[0 as libc::c_int as usize] =
                            30 as libc::c_int as s16
                    }
                }
                if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
                       4 as libc::c_int {
                    (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize]
                        = MO_TENT_READY as libc::c_int as s16;
                    (*this).tentMaxAngle = 0.001f32;
                    (*this).tentSpeed = 0 as libc::c_int as f32_0;
                    (*this).fwork[MO_TENT_SWING_SIZE_Z as libc::c_int as
                                      usize] = 0 as libc::c_int as f32_0;
                    (*this).fwork[MO_TENT_SWING_SIZE_X as libc::c_int as
                                      usize] = 0 as libc::c_int as f32_0;
                    (*this).fwork[MO_TENT_SWING_RATE_Z as libc::c_int as
                                      usize] = 0 as libc::c_int as f32_0;
                    (*this).fwork[MO_TENT_SWING_RATE_X as libc::c_int as
                                      usize] = 0 as libc::c_int as f32_0;
                    (*this).timers[0 as libc::c_int as usize] =
                        30 as libc::c_int as s16
                }
            }
            if (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
                   libc::c_int == MO_TENT_GRAB as libc::c_int {
                (*player).unk_850 = 0xa as libc::c_int as s16;
                (*player).actor.velocity.y = 0 as libc::c_int as f32_0;
                (*player).actor.speedXZ = (*player).actor.velocity.y;
                Math_ApproachF(&mut (*player).actor.world.pos.x,
                               (*this).grabPosRot.pos.x, 0.5f32, 20.0f32);
                Math_ApproachF(&mut (*player).actor.world.pos.y,
                               (*this).grabPosRot.pos.y, 0.5f32, 20.0f32);
                Math_ApproachF(&mut (*player).actor.world.pos.z,
                               (*this).grabPosRot.pos.z, 0.5f32, 20.0f32);
                Math_ApproachS(&mut (*player).actor.shape.rot.x,
                               (*this).grabPosRot.rot.x,
                               2 as libc::c_int as s16,
                               0x7d0 as libc::c_int as s16);
                Math_ApproachS(&mut (*player).actor.shape.rot.y,
                               (*this).grabPosRot.rot.y,
                               2 as libc::c_int as s16,
                               0x7d0 as libc::c_int as s16);
                Math_ApproachS(&mut (*player).actor.shape.rot.z,
                               (*this).grabPosRot.rot.z,
                               2 as libc::c_int as s16,
                               0x7d0 as libc::c_int as s16);
                if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
                       0 as libc::c_int {
                    camera1 =
                        Gameplay_GetCamera(globalCtx,
                                           0 as libc::c_int as s16);
                    (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize]
                        = MO_TENT_SHAKE as libc::c_int as s16;
                    (*this).tentMaxAngle = 0.001f32;
                    (*this).tentSpeed = 0 as libc::c_int as f32_0;
                    (*this).fwork[MO_TENT_SWING_SIZE_Z as libc::c_int as
                                      usize] = (*this).tentSpeed;
                    (*this).fwork[MO_TENT_SWING_SIZE_X as libc::c_int as
                                      usize] =
                        (*this).fwork[MO_TENT_SWING_SIZE_Z as libc::c_int as
                                          usize];
                    (*this).fwork[MO_TENT_SWING_RATE_Z as libc::c_int as
                                      usize] =
                        (*this).fwork[MO_TENT_SWING_SIZE_X as libc::c_int as
                                          usize];
                    (*this).fwork[MO_TENT_SWING_RATE_X as libc::c_int as
                                      usize] =
                        (*this).fwork[MO_TENT_SWING_RATE_Z as libc::c_int as
                                          usize];
                    (*this).timers[0 as libc::c_int as usize] =
                        150 as libc::c_int as s16;
                    (*this).mashCounter = 0 as libc::c_int as s16;
                    (*this).sfxTimer = 30 as libc::c_int as s16;
                    Audio_ResetIncreasingTranspose();
                    func_80064520(globalCtx, &mut (*globalCtx).csCtx);
                    (*this).csCamera = Gameplay_CreateSubCamera(globalCtx);
                    Gameplay_ChangeCameraStatus(globalCtx,
                                                0 as libc::c_int as s16,
                                                1 as libc::c_int as s16);
                    Gameplay_ChangeCameraStatus(globalCtx, (*this).csCamera,
                                                7 as libc::c_int as s16);
                    (*this).cameraEye = (*camera1).eye;
                    (*this).cameraAt = (*camera1).at;
                    (*this).cameraYaw =
                        Math_FAtan2F((*this).cameraEye.x -
                                         (*this).actor.world.pos.x,
                                     (*this).cameraEye.z -
                                         (*this).actor.world.pos.z);
                    (*this).cameraYawRate = 0 as libc::c_int as f32_0;
                    current_block_601 = 4269203575280480064;
                } else { current_block_601 = 7905081044627426364; }
            } else { current_block_601 = 7905081044627426364; }
        }
        5 => { current_block_601 = 4269203575280480064; }
        100 => {
            func_80078914(&mut (*this).tentTipPos,
                          (0x2828 as libc::c_int - 0x800 as libc::c_int) as
                              u16_0);
            if &mut (*this).actor as *mut Actor == (*player).actor.parent {
                (*player).unk_850 = 0x65 as libc::c_int as s16;
                (*player).actor.parent = 0 as *mut Actor;
                (*player).csMode = 0 as libc::c_int as u8_0
            }
            Math_ApproachF(&mut (*this).tentRippleSize, 0.15f32, 0.5f32,
                           0.01f64 as f32_0);
            if ((*this).meltIndex as libc::c_int) < 41 as libc::c_int {
                indS0 = 0 as libc::c_int as s16;
                while (indS0 as libc::c_int) < 10 as libc::c_int {
                    sp120 = (*this).tentPos[(*this).meltIndex as usize];
                    sp120.x += Rand_CenteredFloat(30.0f32);
                    sp120.y += Rand_CenteredFloat(30.0f32);
                    sp120.z += Rand_CenteredFloat(30.0f32);
                    BossMo_SpawnStillDroplet((*globalCtx).specialEffects as
                                                 *mut BossMoEffect,
                                             &mut sp120,
                                             Rand_ZeroFloat(0.1f32) + 0.2f32);
                    indS0 += 1
                }
                (*this).meltIndex += 1
            }
            Math_ApproachF(&mut (*this).cutScale, 0.0f64 as f32_0, 1.0f32,
                           0.2f32);
            if (*this).meltIndex as libc::c_int >= 41 as libc::c_int ||
                   (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
                       0 as libc::c_int {
                (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] =
                    MO_TENT_RETREAT as libc::c_int as s16;
                (*this).timers[0 as libc::c_int as usize] =
                    75 as libc::c_int as s16;
                (*this).tentMaxAngle = 0.005f32;
                (*this).tentSpeed = 50.0f32;
                (*this).fwork[MO_TENT_SWING_SIZE_X as libc::c_int as usize] =
                    7000.0f32;
                (*this).fwork[MO_TENT_SWING_SIZE_Z as libc::c_int as usize] =
                    5000.0f32
            }
            current_block_601 = 7905081044627426364;
        }
        101 => {
            if (*this).csCamera as libc::c_int != 0 as libc::c_int {
                Math_ApproachF(&mut (*this).cameraAt.x,
                               (*player).actor.world.pos.x, 0.5f32, 50.0f32);
                Math_ApproachF(&mut (*this).cameraAt.y,
                               (*player).actor.world.pos.y, 0.5f32, 50.0f32);
                Math_ApproachF(&mut (*this).cameraAt.z,
                               (*player).actor.world.pos.z, 0.5f32, 50.0f32);
                Gameplay_CameraSetAtEye(globalCtx, (*this).csCamera,
                                        &mut (*this).cameraAt,
                                        &mut (*this).cameraEye);
                if (*player).actor.world.pos.y <= 42.0f32 {
                    camera2 =
                        Gameplay_GetCamera(globalCtx,
                                           0 as libc::c_int as s16);
                    (*camera2).eye = (*this).cameraEye;
                    (*camera2).eyeNext = (*this).cameraEye;
                    (*camera2).at = (*this).cameraAt;
                    func_800C08AC(globalCtx, (*this).csCamera,
                                  0 as libc::c_int as s16);
                    (*this).csCamera = 0 as libc::c_int as s16;
                    func_80064534(globalCtx, &mut (*globalCtx).csCtx);
                }
            }
            indS1 = 0 as libc::c_int as s16;
            while (indS1 as libc::c_int) < 41 as libc::c_int {
                sin =
                    Math_SinS(((*this).fwork[MO_TENT_SWING_LAG_X as
                                                 libc::c_int as usize] as s16
                                   as libc::c_int * indS1 as libc::c_int +
                                   (*this).xSwing as libc::c_int) as s16);
                tempf1 =
                    indS1 as libc::c_int as libc::c_float * 0.025f32 * sin *
                        (*this).fwork[MO_TENT_SWING_SIZE_X as libc::c_int as
                                          usize] *
                        (*this).fwork[MO_TENT_MAX_STRETCH as libc::c_int as
                                          usize];
                cos =
                    Math_SinS(((*this).fwork[MO_TENT_SWING_LAG_Z as
                                                 libc::c_int as usize] as s16
                                   as libc::c_int * indS1 as libc::c_int +
                                   (*this).zSwing as libc::c_int) as s16);
                tempf2 =
                    indS1 as libc::c_int as libc::c_float * 0.025f32 * cos *
                        (*this).fwork[MO_TENT_SWING_SIZE_Z as libc::c_int as
                                          usize] *
                        (*this).fwork[MO_TENT_MAX_STRETCH as libc::c_int as
                                          usize];
                Math_ApproachF(&mut (*(*this).tentStretch.as_mut_ptr().offset(indS1
                                                                                  as
                                                                                  isize)).y,
                               (*this).fwork[MO_TENT_MAX_STRETCH as
                                                 libc::c_int as usize] *
                                   5.0f32, 0.5f32, 0.2f32);
                Math_ApproachS(&mut (*(*this).tentRot.as_mut_ptr().offset(indS1
                                                                              as
                                                                              isize)).x,
                               tempf1 as s16,
                               (1.0f32 / (*this).tentMaxAngle) as s16,
                               (*this).tentSpeed as s16);
                Math_ApproachS(&mut (*(*this).tentRot.as_mut_ptr().offset(indS1
                                                                              as
                                                                              isize)).z,
                               tempf2 as s16,
                               (1.0f32 / (*this).tentMaxAngle) as s16,
                               (*this).tentSpeed as s16);
                indS1 += 1
            }
            Math_ApproachF(&mut *(*this).fwork.as_mut_ptr().offset(MO_TENT_MAX_STRETCH
                                                                       as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                           0 as libc::c_int as f32_0, 0.5f32, 0.02f32);
            Math_ApproachF(&mut (*this).tentMaxAngle, 0.5f32, 1.0f32,
                           0.01f64 as f32_0);
            Math_ApproachF(&mut (*this).tentSpeed, 320.0f32, 1.0f32, 50.0f32);
            if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
                   0 as libc::c_int {
                (*this).actor.flags &=
                    !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
                Math_ApproachF(&mut (*this).baseAlpha, 0.0f64 as f32_0,
                               1.0f32, 5.0f32);
                indS1 = 0 as libc::c_int as s16;
                while (indS1 as libc::c_int) < 40 as libc::c_int {
                    ((*sMorphaTent2).tentSpawnPos) != 0;
                    indT5 = Rand_ZeroFloat(20.9f32) as s16;
                    indS0 = sTentSpawnIndex[indT5 as usize] as s16;
                    spFC.x = 0 as libc::c_int as f32_0;
                    spFC.y = 0 as libc::c_int as f32_0;
                    spFC.z = 0 as libc::c_int as f32_0;
                    Matrix_RotateY((*player).actor.world.rot.y as libc::c_int
                                       as libc::c_float /
                                       0x8000 as libc::c_int as f32_0 *
                                       3.14159265358979323846f32,
                                   MTXMODE_NEW as libc::c_int as u8_0);
                    Matrix_MultVec3f(&mut spFC, &mut spF0);
                    spF0.x = (*player).actor.world.pos.x + spF0.x;
                    spF0.z = (*player).actor.world.pos.z + spF0.z;
                    if fabsf(spF0.x - sTentSpawnPos[indS0 as usize].x) <=
                           320 as libc::c_int as libc::c_float &&
                           fabsf(spF0.z - sTentSpawnPos[indS0 as usize].y) <=
                               320 as libc::c_int as libc::c_float &&
                           (sMorphaTent2.is_null() ||
                                (*sMorphaTent2).tentSpawnPos as libc::c_int !=
                                    indS0 as libc::c_int) {
                        (*this).targetPos.x = sTentSpawnPos[indS0 as usize].x;
                        (*this).targetPos.z = sTentSpawnPos[indS0 as usize].y;
                        (*this).tentSpawnPos = indS0 as u8_0;
                        (*this).timers[0 as libc::c_int as usize] =
                            (Rand_ZeroFloat(20.0f32) as s16 as libc::c_int +
                                 30 as libc::c_int) as s16;
                        (*this).work[MO_TENT_ACTION_STATE as libc::c_int as
                                         usize] =
                            MO_TENT_DESPAWN as libc::c_int as s16;
                        break ;
                    } else { indS1 += 1 }
                }
            }
            if this == sMorphaTent1 &&
                   (*sMorphaCore).hitCount as libc::c_int >= 3 as libc::c_int
                   && sMorphaTent2.is_null() {
                sMorphaTent2 =
                    Actor_Spawn(&mut (*globalCtx).actorCtx, globalCtx,
                                ACTOR_BOSS_MO as libc::c_int as s16,
                                (*this).actor.world.pos.x,
                                (*this).actor.world.pos.y,
                                (*this).actor.world.pos.z,
                                0 as libc::c_int as s16,
                                0 as libc::c_int as s16,
                                0 as libc::c_int as s16,
                                100 as libc::c_int as s16) as *mut BossMo;
                (*sMorphaTent2).tentSpawnPos = (*this).tentSpawnPos;
                if (*sMorphaTent2).tentSpawnPos as libc::c_int >
                       10 as libc::c_int {
                    (*sMorphaTent2).tentSpawnPos =
                        (*sMorphaTent2).tentSpawnPos.wrapping_sub(1)
                } else {
                    (*sMorphaTent2).tentSpawnPos =
                        (*sMorphaTent2).tentSpawnPos.wrapping_add(1)
                }
                (*sMorphaTent2).targetPos.x =
                    sTentSpawnPos[(*sMorphaTent2).tentSpawnPos as usize].x;
                (*sMorphaTent2).targetPos.z =
                    sTentSpawnPos[(*sMorphaTent2).tentSpawnPos as usize].y;
                (*sMorphaTent2).timers[0 as libc::c_int as usize] =
                    100 as libc::c_int as s16;
                (*sMorphaTent2).work[MO_TENT_ACTION_STATE as libc::c_int as
                                         usize] =
                    MO_TENT_DESPAWN as libc::c_int as s16;
                (*sMorphaTent2).otherTent = &mut (*sMorphaTent1).actor;
                (*sMorphaTent1).otherTent = &mut (*sMorphaTent2).actor
            }
            current_block_601 = 7905081044627426364;
        }
        102 => {
            (*this).actor.flags &=
                !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
            Math_ApproachF(&mut (*this).baseAlpha, 0 as libc::c_int as f32_0,
                           1.0f32, 5.0f32);
            if (*this).baseAlpha <= 0.5f32 &&
                   (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
                       0 as libc::c_int {
                (*this).meltIndex = 0 as libc::c_int as s16;
                (*this).actor.world.pos.x = (*this).targetPos.x;
                (*this).actor.world.pos.z = (*this).targetPos.z;
                (*this).actor.prevPos = (*this).actor.world.pos;
                (*this).cutScale = 1.0f32;
                (*this).cutIndex = (*this).meltIndex;
                (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] =
                    MO_TENT_WAIT as libc::c_int as s16;
                (*this).timers[0 as libc::c_int as usize] =
                    (Rand_ZeroFloat(20.0f32) as s16 as libc::c_int +
                         10 as libc::c_int) as s16;
                (*this).tentSpeed = 0 as libc::c_int as f32_0;
                (*this).fwork[MO_TENT_SWING_RATE_X as libc::c_int as usize] =
                    0 as libc::c_int as f32_0;
                (*this).fwork[MO_TENT_SWING_RATE_Z as libc::c_int as usize] =
                    0 as libc::c_int as f32_0;
                (*this).fwork[MO_TENT_SWING_SIZE_X as libc::c_int as usize] =
                    0 as libc::c_int as f32_0;
                (*this).fwork[MO_TENT_SWING_SIZE_Z as libc::c_int as usize] =
                    0 as libc::c_int as f32_0;
                (*this).tentMaxAngle = 0.001f32
            }
            current_block_601 = 7905081044627426364;
        }
        200 => {
            (*this).actor.shape.rot.y = 0x4000 as libc::c_int as s16;
            current_block_601 = 7905081044627426364;
        }
        203 => {
            (*this).baseBubblesTimer = 20 as libc::c_int as s16;
            Math_ApproachF(&mut (*sMorphaCore).waterLevel, -300.0f32, 0.1f32,
                           0.8f32);
            (*this).actor.flags &=
                !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
            indS1 = 0 as libc::c_int as s16;
            while (indS1 as libc::c_int) < 41 as libc::c_int {
                sin =
                    Math_SinS(((*this).fwork[MO_TENT_SWING_LAG_X as
                                                 libc::c_int as usize] as s16
                                   as libc::c_int * indS1 as libc::c_int +
                                   (*this).xSwing as libc::c_int) as s16);
                tempf1 =
                    (*this).fwork[MO_TENT_SWING_SIZE_X as libc::c_int as
                                      usize] *
                        (indS1 as libc::c_int as libc::c_float * 0.025f32 *
                             sin);
                cos =
                    Math_SinS(((*this).fwork[MO_TENT_SWING_LAG_Z as
                                                 libc::c_int as usize] as s16
                                   as libc::c_int * indS1 as libc::c_int +
                                   (*this).zSwing as libc::c_int) as s16);
                tempf2 =
                    (*this).fwork[MO_TENT_SWING_SIZE_Z as libc::c_int as
                                      usize] *
                        (indS1 as libc::c_int as libc::c_float * 0.025f32 *
                             cos);
                Math_ApproachF(&mut (*(*this).tentStretch.as_mut_ptr().offset(indS1
                                                                                  as
                                                                                  isize)).y,
                               (*this).fwork[MO_TENT_MAX_STRETCH as
                                                 libc::c_int as usize] *
                                   5.0f32, 0.1f32, 0.4f32);
                Math_ApproachS(&mut (*(*this).tentRot.as_mut_ptr().offset(indS1
                                                                              as
                                                                              isize)).x,
                               tempf1 as s16,
                               (1.0f32 / (*this).tentMaxAngle) as s16,
                               (*this).tentSpeed as s16);
                Math_ApproachS(&mut (*(*this).tentRot.as_mut_ptr().offset(indS1
                                                                              as
                                                                              isize)).z,
                               tempf2 as s16,
                               (1.0f32 / (*this).tentMaxAngle) as s16,
                               (*this).tentSpeed as s16);
                indS1 += 1
            }
            (*this).actor.speedXZ = 0.0f64 as f32_0;
            Math_ApproachF(&mut *(*this).fwork.as_mut_ptr().offset(MO_TENT_MAX_STRETCH
                                                                       as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                           4.3f32, 0.5f32, 0.04f64 as f32_0);
            Math_ApproachF(&mut (*this).tentPulse, 1.3f32, 0.5f32, 0.05f32);
            current_block_601 = 7905081044627426364;
        }
        201 => {
            (*this).baseBubblesTimer = 20 as libc::c_int as s16;
            (*this).actor.shape.rot.y = 0x4000 as libc::c_int as s16;
            (*this).actor.shape.rot.x = -(0x8000 as libc::c_int) as s16;
            (*this).actor.world.pos.y = (*sMorphaCore).waterLevel + 650.0f32;
            Math_ApproachF(&mut (*sMorphaCore).waterLevel, -300.0f32, 0.1f32,
                           1.3f32);
            indS1 = 0 as libc::c_int as s16;
            while (indS1 as libc::c_int) < 41 as libc::c_int {
                sin =
                    Math_SinS(((*this).fwork[MO_TENT_SWING_LAG_X as
                                                 libc::c_int as usize] as s16
                                   as libc::c_int * indS1 as libc::c_int +
                                   (*this).xSwing as libc::c_int) as s16);
                tempf1 =
                    (*this).fwork[MO_TENT_SWING_SIZE_X as libc::c_int as
                                      usize] *
                        (indS1 as libc::c_int as libc::c_float * 0.025f32 *
                             sin);
                cos =
                    Math_SinS(((*this).fwork[MO_TENT_SWING_LAG_Z as
                                                 libc::c_int as usize] as s16
                                   as libc::c_int * indS1 as libc::c_int +
                                   (*this).zSwing as libc::c_int) as s16);
                tempf2 =
                    (*this).fwork[MO_TENT_SWING_SIZE_Z as libc::c_int as
                                      usize] *
                        (indS1 as libc::c_int as libc::c_float * 0.025f32 *
                             cos);
                Math_ApproachF(&mut (*(*this).tentStretch.as_mut_ptr().offset(indS1
                                                                                  as
                                                                                  isize)).y,
                               (*this).fwork[MO_TENT_MAX_STRETCH as
                                                 libc::c_int as usize] *
                                   5.0f32, 0.1f32, 0.4f32);
                Math_ApproachS(&mut (*(*this).tentRot.as_mut_ptr().offset(indS1
                                                                              as
                                                                              isize)).x,
                               tempf1 as s16,
                               (1.0f32 / (*this).tentMaxAngle) as s16,
                               (*this).tentSpeed as s16);
                Math_ApproachS(&mut (*(*this).tentRot.as_mut_ptr().offset(indS1
                                                                              as
                                                                              isize)).z,
                               tempf2 as s16,
                               (1.0f32 / (*this).tentMaxAngle) as s16,
                               (*this).tentSpeed as s16);
                indS1 += 1
            }
            (*this).actor.speedXZ = 0.0f64 as f32_0;
            Math_ApproachF(&mut (*this).tentPulse, 1.3f32, 0.5f32, 0.05f32);
            current_block_601 = 7905081044627426364;
        }
        202 => {
            (*this).baseBubblesTimer = 20 as libc::c_int as s16;
            Math_ApproachF(&mut (*sMorphaCore).waterLevel, -295.0f32, 0.1f32,
                           1.3f32);
            (*this).actor.world.pos.y = (*sMorphaCore).waterLevel + 650.0f32;
            indS1 = 0 as libc::c_int as s16;
            while (indS1 as libc::c_int) < 41 as libc::c_int {
                sin =
                    Math_SinS(((*this).fwork[MO_TENT_SWING_LAG_X as
                                                 libc::c_int as usize] as s16
                                   as libc::c_int * indS1 as libc::c_int +
                                   (*this).xSwing as libc::c_int) as s16);
                tempf1 =
                    (*this).fwork[MO_TENT_SWING_SIZE_X as libc::c_int as
                                      usize] *
                        (indS1 as libc::c_int as libc::c_float * 0.025f32 *
                             sin);
                cos =
                    Math_SinS(((*this).fwork[MO_TENT_SWING_LAG_Z as
                                                 libc::c_int as usize] as s16
                                   as libc::c_int * indS1 as libc::c_int +
                                   (*this).zSwing as libc::c_int) as s16);
                tempf2 =
                    (*this).fwork[MO_TENT_SWING_SIZE_Z as libc::c_int as
                                      usize] *
                        (indS1 as libc::c_int as libc::c_float * 0.025f32 *
                             cos);
                Math_ApproachF(&mut (*(*this).tentStretch.as_mut_ptr().offset(indS1
                                                                                  as
                                                                                  isize)).y,
                               (*this).fwork[MO_TENT_MAX_STRETCH as
                                                 libc::c_int as usize] *
                                   5.0f32, 0.1f32, 0.4f32);
                Math_ApproachS(&mut (*(*this).tentRot.as_mut_ptr().offset(indS1
                                                                              as
                                                                              isize)).x,
                               tempf1 as s16,
                               (1.0f32 / (*this).tentMaxAngle) as s16,
                               (*this).tentSpeed as s16);
                Math_ApproachS(&mut (*(*this).tentRot.as_mut_ptr().offset(indS1
                                                                              as
                                                                              isize)).z,
                               tempf2 as s16,
                               (1.0f32 / (*this).tentMaxAngle) as s16,
                               (*this).tentSpeed as s16);
                indS1 += 1
            }
            (*this).actor.speedXZ = 0.0f64 as f32_0;
            (*this).noBubbles -= 1;
            Math_ApproachF(&mut *(*this).fwork.as_mut_ptr().offset(MO_TENT_MAX_STRETCH
                                                                       as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                           0.1f32, 0.1f32, 0.03f64 as f32_0);
            Math_ApproachF(&mut (*this).tentPulse, 0.02f32, 0.5f32, 0.015f32);
            if (*this).timers[0 as libc::c_int as usize] as libc::c_int >
                   0 as libc::c_int &&
                   ((*this).timers[0 as libc::c_int as usize] as libc::c_int)
                       < 40 as libc::c_int {
                Math_ApproachF(&mut (*this).actor.scale.x, 0.035f32, 0.05f32,
                               (*this).flattenRate);
                if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
                       1 as libc::c_int {
                    (*this).flattenRate = 0.0f64 as f32_0
                }
            } else if (*this).timers[0 as libc::c_int as usize] as libc::c_int
                          == 0 as libc::c_int {
                Math_ApproachF(&mut (*this).actor.scale.x, 0.001f32, 0.05f32,
                               (*this).flattenRate);
            }
            Math_ApproachF(&mut (*this).flattenRate, 0.00045f32, 0.1f32,
                           0.00001f32);
            current_block_601 = 7905081044627426364;
        }
        205 => {
            indS1 = 0 as libc::c_int as s16;
            while (indS1 as libc::c_int) < 41 as libc::c_int {
                if (*this).timers[0 as libc::c_int as usize] as libc::c_int !=
                       0 as libc::c_int {
                    Math_ApproachF(&mut (*(*this).tentStretch.as_mut_ptr().offset(indS1
                                                                                      as
                                                                                      isize)).y,
                                   (*this).fwork[MO_TENT_MAX_STRETCH as
                                                     libc::c_int as usize] *
                                       5.0f32, 0.05f32, (*this).tentSpeed);
                } else {
                    Math_ApproachF(&mut (*(*this).tentStretch.as_mut_ptr().offset(indS1
                                                                                      as
                                                                                      isize)).y,
                                   (*this).fwork[MO_TENT_MAX_STRETCH as
                                                     libc::c_int as usize] *
                                       5.0f32, 0.3f32, 100.0f32);
                }
                (*this).tentRot[indS1 as usize].z = 0 as libc::c_int as s16;
                (*this).tentRot[indS1 as usize].x =
                    (*this).tentRot[indS1 as usize].z;
                indS1 += 1
            }
            (*this).tentPulse = 0.0f64 as f32_0;
            if (*this).timers[0 as libc::c_int as usize] as libc::c_int !=
                   0 as libc::c_int {
                (*this).actor.world.pos.y =
                    (*sMorphaCore).waterLevel + 650.0f32;
                (*this).fwork[MO_TENT_MAX_STRETCH as libc::c_int as usize] =
                    0.5f32;
                Math_ApproachF(&mut (*this).actor.scale.x, 0.0015f32, 0.05f32,
                               (*this).tentMaxAngle);
                Math_ApproachF(&mut (*this).tentMaxAngle, 0.00035f32, 1.0f32,
                               0.0000175f32);
                Math_ApproachF(&mut (*this).tentSpeed, 0.1f32, 1.0f32,
                               0.005f32);
                (*this).actor.velocity.y = 0.0f64 as f32_0
            } else {
                (*this).fwork[MO_TENT_MAX_STRETCH as libc::c_int as usize] =
                    0.2f32;
                (*this).fwork[MO_TENT_MAX_STRETCH as libc::c_int as usize] +=
                    Math_SinS(((*this).work[MO_TENT_MOVE_TIMER as libc::c_int
                                                as usize] as libc::c_int *
                                   0x2000 as libc::c_int) as s16) * 0.05f32;
                padEC =
                    Math_CosS(((*this).work[MO_TENT_MOVE_TIMER as libc::c_int
                                                as usize] as libc::c_int *
                                   0x2000 as libc::c_int) as s16) * 0.0005f32;
                Math_ApproachF(&mut (*this).actor.scale.x, 0.002f32 + padEC,
                               0.5f32, 0.0005f32);
                (*this).actor.world.pos.y += (*this).actor.velocity.y;
                (*this).actor.velocity.y -= 1.0f32;
                if (*this).actor.world.pos.y < -250.0f32 {
                    (*this).actor.world.pos.y = -250.0f32;
                    (*this).actor.velocity.y = 0.0f64 as f32_0;
                    (*this).drawActor = 0 as libc::c_int as u8_0;
                    (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize]
                        = MO_TENT_DEATH_6 as libc::c_int as s16;
                    (*this).timers[0 as libc::c_int as usize] =
                        60 as libc::c_int as s16;
                    func_80078914(&mut (*this).tentTipPos,
                                  0x38f7 as libc::c_int as u16_0);
                    indS1 = 0 as libc::c_int as s16;
                    while (indS1 as libc::c_int) < 300 as libc::c_int {
                        spC8.x = 0.0f64 as f32_0;
                        spC8.y = 0.0f64 as f32_0;
                        spC8.z =
                            indS1 as libc::c_int as libc::c_float * 0.03f32;
                        Matrix_RotateY(indS1 as libc::c_int as libc::c_float *
                                           0.23f32,
                                       MTXMODE_NEW as libc::c_int as u8_0);
                        Matrix_MultVec3f(&mut spC8, &mut spE0);
                        spE0.y = Rand_ZeroFloat(7.0f32) + 4.0f32;
                        spD4 = (*this).actor.world.pos;
                        spD4.x += spE0.x * 3.0f32;
                        spD4.y += spE0.y * 3.0f32 - 30.0f32;
                        if spD4.y < -280.0f32 { spD4.y = -280.0f32 }
                        spD4.z += spE0.z * 3.0f32;
                        BossMo_SpawnDroplet(MO_FX_DROPLET as libc::c_int as
                                                s16,
                                            (*globalCtx).specialEffects as
                                                *mut BossMoEffect, &mut spD4,
                                            &mut spE0,
                                            (300 as libc::c_int -
                                                 indS1 as libc::c_int) as
                                                libc::c_float * 0.0015f32 +
                                                0.13f32);
                        indS1 += 1
                    }
                    Actor_SpawnAsChild(&mut (*globalCtx).actorCtx,
                                       &mut (*this).actor, globalCtx,
                                       ACTOR_DOOR_WARP1 as libc::c_int as s16,
                                       (*this).actor.world.pos.x, -280.0f32,
                                       (*this).actor.world.pos.z,
                                       0 as libc::c_int as s16,
                                       0 as libc::c_int as s16,
                                       0 as libc::c_int as s16,
                                       WARP_DUNGEON_ADULT as libc::c_int as
                                           s16);
                    Actor_Spawn(&mut (*globalCtx).actorCtx, globalCtx,
                                ACTOR_ITEM_B_HEART as libc::c_int as s16,
                                (*this).actor.world.pos.x + 200.0f32,
                                -280.0f32, (*this).actor.world.pos.z,
                                0 as libc::c_int as s16,
                                0 as libc::c_int as s16,
                                0 as libc::c_int as s16,
                                0 as libc::c_int as s16);
                    Audio_QueueSeqCmd(((SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                                           24 as libc::c_int |
                                           0x21 as libc::c_int) as u32_0);
                    Flags_SetClear(globalCtx,
                                   (*globalCtx).roomCtx.curRoom.num as s32);
                }
            }
            current_block_601 = 7905081044627426364;
        }
        206 | _ => { current_block_601 = 7905081044627426364; }
    }
    match current_block_601 {
        4269203575280480064 => {
            if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
                   138 as libc::c_int {
                ShrinkWindow_SetVal(0 as libc::c_int);
                Interface_ChangeAlpha(0xb as libc::c_int as u16_0);
            }
            if (*this).timers[0 as libc::c_int as usize] as libc::c_int %
                   8 as libc::c_int == 0 as libc::c_int {
                (*globalCtx).damagePlayer.expect("non-null function pointer")(globalCtx,
                                                                              -(1
                                                                                    as
                                                                                    libc::c_int));
            }
            Math_ApproachF(&mut (*this).waterLevelMod, -5.0f32, 0.1f32,
                           0.4f32);
            sp1B4 = (*this).tentRot[15 as libc::c_int as usize].x;
            buttons =
                (*globalCtx).state.input[0 as libc::c_int as
                                             usize].press.button as s32;
            if !(buttons | !(0x8000 as libc::c_int)) == 0 as libc::c_int ||
                   !(buttons | !(0x4000 as libc::c_int)) == 0 as libc::c_int {
                (*this).mashCounter += 1
            }
            indS1 = 0 as libc::c_int as s16;
            while (indS1 as libc::c_int) < 41 as libc::c_int {
                if (indS1 as libc::c_int) < 20 as libc::c_int {
                    sin =
                        Math_SinS(((*this).fwork[MO_TENT_SWING_LAG_X as
                                                     libc::c_int as usize] as
                                       s16 as libc::c_int *
                                       indS1 as libc::c_int +
                                       (*this).xSwing as libc::c_int) as s16);
                    tempf1 =
                        (*this).fwork[MO_TENT_SWING_SIZE_X as libc::c_int as
                                          usize] *
                            (indS1 as libc::c_int as libc::c_float * 0.025f32
                                 * sin);
                    cos =
                        Math_SinS(((*this).fwork[MO_TENT_SWING_LAG_Z as
                                                     libc::c_int as usize] as
                                       s16 as libc::c_int *
                                       indS1 as libc::c_int +
                                       (*this).zSwing as libc::c_int) as s16);
                    tempf2 =
                        (*this).fwork[MO_TENT_SWING_SIZE_Z as libc::c_int as
                                          usize] *
                            (indS1 as libc::c_int as libc::c_float * 0.025f32
                                 * cos);
                    temp =
                        (40 as libc::c_int - indS1 as libc::c_int) as
                            libc::c_float * 25.0f32 / 100.0f32 + 5.0f32;
                    Math_ApproachF(&mut (*(*this).tentStretch.as_mut_ptr().offset(indS1
                                                                                      as
                                                                                      isize)).y,
                                   (*this).fwork[MO_TENT_MAX_STRETCH as
                                                     libc::c_int as usize] *
                                       temp, 0.1f32, 0.1f32);
                    Math_ApproachS(&mut (*(*this).tentRot.as_mut_ptr().offset(indS1
                                                                                  as
                                                                                  isize)).x,
                                   tempf1 as s16,
                                   (1.0f32 / (*this).tentMaxAngle) as s16,
                                   (*this).tentSpeed as s16);
                    Math_ApproachS(&mut (*(*this).tentRot.as_mut_ptr().offset(indS1
                                                                                  as
                                                                                  isize)).z,
                                   tempf2 as s16,
                                   (1.0f32 / (*this).tentMaxAngle) as s16,
                                   (*this).tentSpeed as s16);
                }
                indS1 += 1
            }
            (*player).unk_850 = 0xa as libc::c_int as s16;
            (*player).actor.world.pos.x = (*this).grabPosRot.pos.x;
            (*player).actor.world.pos.y = (*this).grabPosRot.pos.y;
            (*player).actor.world.pos.z = (*this).grabPosRot.pos.z;
            (*player).actor.shape.rot.x = (*this).grabPosRot.rot.x;
            (*player).actor.world.rot.x = (*player).actor.shape.rot.x;
            (*player).actor.shape.rot.y = (*this).grabPosRot.rot.y;
            (*player).actor.world.rot.y = (*player).actor.shape.rot.y;
            (*player).actor.shape.rot.z = (*this).grabPosRot.rot.z;
            (*player).actor.world.rot.z = (*player).actor.shape.rot.z;
            (*player).actor.velocity.y = 0 as libc::c_int as f32_0;
            (*player).actor.speedXZ = 0 as libc::c_int as f32_0;
            Math_ApproachF(&mut *(*this).fwork.as_mut_ptr().offset(MO_TENT_MAX_STRETCH
                                                                       as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                           1.0f32, 0.5f32, 0.01f64 as f32_0);
            Math_ApproachF(&mut (*this).tentMaxAngle, 0.5f32, 1.0f32,
                           0.005f32);
            Math_ApproachF(&mut (*this).tentSpeed, 480.0f32, 1.0f32, 10.0f32);
            Math_ApproachF(&mut (*this).tentPulse, 0.3f32, 0.5f32, 0.03f32);
            if (*this).mashCounter as libc::c_int >= 40 as libc::c_int ||
                   (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
                       0 as libc::c_int {
                tentXrot = (*this).tentRot[15 as libc::c_int as usize].x;
                if (tentXrot as libc::c_int) < 0 as libc::c_int &&
                       sp1B4 as libc::c_int >= 0 as libc::c_int {
                    (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize]
                        = MO_TENT_RETREAT as libc::c_int as s16;
                    (*this).work[MO_TENT_INVINC_TIMER as libc::c_int as usize]
                        = 50 as libc::c_int as s16;
                    if &mut (*this).actor as *mut Actor ==
                           (*player).actor.parent {
                        (*player).unk_850 = 0x65 as libc::c_int as s16;
                        (*player).actor.parent = 0 as *mut Actor;
                        (*player).csMode = 0 as libc::c_int as u8_0;
                        if (*this).timers[0 as libc::c_int as usize] as
                               libc::c_int == 0 as libc::c_int {
                            func_8002F6D4(globalCtx, &mut (*this).actor,
                                          20.0f32,
                                          ((*this).actor.shape.rot.y as
                                               libc::c_int +
                                               0x8000 as libc::c_int) as s16,
                                          10.0f32, 0 as libc::c_int as u32_0);
                        }
                    }
                    (*this).timers[0 as libc::c_int as usize] =
                        75 as libc::c_int as s16
                }
            }
            if (*this).csCamera as libc::c_int != 0 as libc::c_int {
                sp138.x = 0 as libc::c_int as f32_0;
                sp138.y = 100.0f32;
                sp138.z = 200.0f32;
                (*this).cameraYaw -= (*this).cameraYawRate;
                Math_ApproachF(&mut (*this).cameraYawRate, 0.01f64 as f32_0,
                               1.0f32, 0.002f32);
                Matrix_RotateY((*this).cameraYaw,
                               MTXMODE_NEW as libc::c_int as u8_0);
                Matrix_MultVec3f(&mut sp138, &mut sp12C);
                Math_ApproachF(&mut (*this).cameraEye.x,
                               (*this).actor.world.pos.x + sp12C.x, 0.1f32,
                               10.0f32);
                Math_ApproachF(&mut (*this).cameraEye.y,
                               (*this).actor.world.pos.y + sp12C.y, 0.1f32,
                               10.0f32);
                Math_ApproachF(&mut (*this).cameraEye.z,
                               (*this).actor.world.pos.z + sp12C.z, 0.1f32,
                               10.0f32);
                Math_ApproachF(&mut (*this).cameraAt.x,
                               (*player).actor.world.pos.x, 0.5f32, 50.0f32);
                Math_ApproachF(&mut (*this).cameraAt.y,
                               (*player).actor.world.pos.y, 0.5f32, 50.0f32);
                Math_ApproachF(&mut (*this).cameraAt.z,
                               (*player).actor.world.pos.z, 0.5f32, 50.0f32);
                Gameplay_CameraSetAtEye(globalCtx, (*this).csCamera,
                                        &mut (*this).cameraAt,
                                        &mut (*this).cameraEye);
            }
        }
        _ => { }
    }
    (*this).actor.scale.z = (*this).actor.scale.x;
    (*this).actor.scale.y = (*this).actor.scale.z;
    if ((*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
            libc::c_int == MO_TENT_ATTACK as libc::c_int ||
            (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
                libc::c_int == MO_TENT_DEATH_2 as libc::c_int ||
            (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
                libc::c_int == MO_TENT_CURL as libc::c_int ||
            (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
                libc::c_int == MO_TENT_GRAB as libc::c_int) &&
           Rand_ZeroOne() < 0.8f32 && (*this).actor.scale.x > 0.001f32 {
        let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
        let mut velocity: Vec3f =
            { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
        let mut scale: f32_0 = 0.;
        let mut temp_0: f32_0 = 0.;
        if (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
               libc::c_int >= MO_TENT_DEATH_2 as libc::c_int {
            indS1 = 38 as libc::c_int as s16;
            scale = Rand_ZeroFloat(0.1f32) + 0.1f32;
            pos.y = (*this).tentPos[indS1 as usize].y
        } else {
            indS1 =
                (Rand_ZeroFloat(20.0f32) as s16 as libc::c_int +
                     18 as libc::c_int) as s16;
            scale = Rand_ZeroFloat(0.02f32) + 0.05f32;
            pos.y = (*this).tentPos[indS1 as usize].y - 10.0f32
        }
        temp_0 = (*this).actor.scale.x * 100.0f32 * 20.0f32;
        pos.x =
            (*this).tentPos[indS1 as usize].x + Rand_CenteredFloat(temp_0);
        pos.z =
            (*this).tentPos[indS1 as usize].z + Rand_CenteredFloat(temp_0);
        BossMo_SpawnDroplet(MO_FX_DROPLET as libc::c_int as s16,
                            (*globalCtx).specialEffects as *mut BossMoEffect,
                            &mut pos, &mut velocity, scale);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossMo_TentCollisionCheck(mut this: *mut BossMo,
                                                   mut globalCtx:
                                                       *mut GlobalContext) {
    let mut i1: s16 = 0;
    i1 = 0 as libc::c_int as s16;
    while (i1 as libc::c_int) <
              (::std::mem::size_of::<[ColliderJntSphElement; 19]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<ColliderJntSphElement>()
                                                   as libc::c_ulong) as s32 {
        if (*(*this).tentCollider.elements.offset(i1 as
                                                      isize)).info.bumperFlags
               as libc::c_int & (1 as libc::c_int) << 1 as libc::c_int != 0 {
            let mut i2: s16 = 0;
            let mut hurtbox: *mut ColliderInfo = 0 as *mut ColliderInfo;
            i2 = 0 as libc::c_int as s16;
            while (i2 as libc::c_int) < 19 as libc::c_int {
                let ref mut fresh0 =
                    (*(*this).tentCollider.elements.offset(i2 as
                                                               isize)).info.bumperFlags;
                *fresh0 =
                    (*fresh0 as libc::c_int &
                         !((1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
                let ref mut fresh1 =
                    (*(*this).tentCollider.elements.offset(i2 as
                                                               isize)).info.toucherFlags;
                *fresh1 =
                    (*fresh1 as libc::c_int &
                         !((1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
                i2 += 1
            }
            hurtbox =
                (*(*this).tentCollider.elements.offset(i1 as
                                                           isize)).info.acHitInfo;
            (*this).work[MO_TENT_INVINC_TIMER as libc::c_int as usize] =
                5 as libc::c_int as s16;
            if (*hurtbox).toucher.dmgFlags &
                   0x20000 as libc::c_int as libc::c_uint != 0 {
                func_80078914(&mut (*this).tentTipPos,
                              0x38f4 as libc::c_int as u16_0);
                (*this).cutIndex = 15 as libc::c_int as s16;
                (*this).meltIndex =
                    ((*this).cutIndex as libc::c_int + 1 as libc::c_int) as
                        s16;
                (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] =
                    MO_TENT_CUT as libc::c_int as s16;
                (*this).timers[0 as libc::c_int as usize] =
                    40 as libc::c_int as s16;
                (*this).cutScale = 1.0f32
            } else if (*hurtbox).toucher.dmgFlags &
                          0xd800600 as libc::c_int as libc::c_uint != 0 {
                (*this).linkHitTimer = 5 as libc::c_int as u8_0
            }
            (*this).tentRippleSize = 0.2f32;
            i2 = 0 as libc::c_int as s16;
            while (i2 as libc::c_int) < 10 as libc::c_int {
                let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                let mut velocity: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                velocity.x = Rand_CenteredFloat(8.0f32);
                velocity.y = Rand_ZeroFloat(7.0f32) + 4.0f32;
                velocity.z = Rand_CenteredFloat(8.0f32);
                pos =
                    (*this).tentPos[(2 as libc::c_int * i1 as libc::c_int) as
                                        usize];
                pos.x += velocity.x * 3.0f32;
                pos.z += velocity.z * 3.0f32;
                BossMo_SpawnDroplet(MO_FX_DROPLET as libc::c_int as s16,
                                    (*globalCtx).specialEffects as
                                        *mut BossMoEffect, &mut pos,
                                    &mut velocity,
                                    Rand_ZeroFloat(0.08f32) + 0.13f32);
                i2 += 1
            }
            break ;
        } else if (*(*this).tentCollider.elements.offset(i1 as
                                                             isize)).info.toucherFlags
                      as libc::c_int & (1 as libc::c_int) << 1 as libc::c_int
                      != 0 {
            let ref mut fresh2 =
                (*(*this).tentCollider.elements.offset(i1 as
                                                           isize)).info.toucherFlags;
            *fresh2 =
                (*fresh2 as libc::c_int &
                     !((1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
            (*this).linkHitTimer = 5 as libc::c_int as u8_0;
            break ;
        } else { i1 += 1 }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossMo_IntroCs(mut this: *mut BossMo,
                                        mut globalCtx: *mut GlobalContext) {
    static mut cutsceneTargets: [Vec3f; 6] =
        [{
             let mut init = Vec3f{x: -360.0f32, y: -190.0f32, z: 0.0f32,};
             init
         },
         {
             let mut init = Vec3f{x: 250.0f32, y: -190.0f32, z: 0.0f32,};
             init
         },
         {
             let mut init = Vec3f{x: 300.0f32, y: -120.0f32, z: -278.0f32,};
             init
         },
         {
             let mut init = Vec3f{x: 180.0f32, y: -80.0f32, z: -340.0f32,};
             init
         },
         {
             let mut init = Vec3f{x: 180.0f32, y: 0.0f32, z: -340.0f32,};
             init
         },
         {
             let mut init = Vec3f{x: 180.0f32, y: 60.0f32, z: -230.0f32,};
             init
         }];
    let mut sp9F: u8_0 = 0 as libc::c_int as u8_0;
    let mut dx: f32_0 = 0.;
    let mut dy: f32_0 = 0.;
    let mut dz: f32_0 = 0.;
    let mut tempX: f32_0 = 0.;
    let mut tempY: f32_0 = 0.;
    let mut pad84: s32 = 0;
    let mut sp80: f32_0 = 0.;
    let mut sp7C: f32_0 = 0.;
    let mut sp78: f32_0 = 0.;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut camera: *mut Camera =
        Gameplay_GetCamera(globalCtx, 0 as libc::c_int as s16);
    let mut bubblePos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut bubblePos2: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut camera2: *mut Camera = 0 as *mut Camera;
    let mut pad50: f32_0 = 0.;
    let mut pad4C: f32_0 = 0.;
    let mut pad48: f32_0 = 0.;
    if ((*this).csState as libc::c_int) < MO_INTRO_REVEAL as libc::c_int {
        (*this).cameraZoom = 80.0f32
    }
    let mut current_block_216: u64;
    match (*this).csState as libc::c_int {
        1 => {
            if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
                   1 as libc::c_int {
                Message_StartTextbox(globalCtx,
                                     0x403f as libc::c_int as u16_0,
                                     0 as *mut Actor);
            }
            if fabsf((*player).actor.world.pos.z - 180.0f32) < 40.0f32 &&
                   fabsf((*player).actor.world.pos.x - 180.0f32) < 40.0f32 ||
                   fabsf((*player).actor.world.pos.z - -180.0f32) < 40.0f32 &&
                       fabsf((*player).actor.world.pos.x - 180.0f32) < 40.0f32
                   ||
                   fabsf((*player).actor.world.pos.z - 180.0f32) < 40.0f32 &&
                       fabsf((*player).actor.world.pos.x - -180.0f32) <
                           40.0f32 ||
                   fabsf((*player).actor.world.pos.z - -180.0f32) < 40.0f32 &&
                       fabsf((*player).actor.world.pos.x - -180.0f32) <
                           40.0f32 {
                // checks if Link is on one of the four platforms
                func_80064520(globalCtx,
                              &mut (*globalCtx).csCtx); // Super fake, but it works
                func_8002DF54(globalCtx, &mut (*this).actor,
                              8 as libc::c_int as u8_0);
                (*this).csCamera = Gameplay_CreateSubCamera(globalCtx);
                Gameplay_ChangeCameraStatus(globalCtx,
                                            0 as libc::c_int as s16,
                                            1 as libc::c_int as s16);
                Gameplay_ChangeCameraStatus(globalCtx, (*this).csCamera,
                                            7 as libc::c_int as s16);
                (*this).actor.speedXZ = 0.0f32;
                (*this).csState = MO_INTRO_START as libc::c_int as s16;
                (*this).timers[2 as libc::c_int as usize] =
                    50 as libc::c_int as s16;
                (*this).work[MO_TENT_MOVE_TIMER as libc::c_int as usize] =
                    0 as libc::c_int as s16;
                (*this).work[MO_TENT_VAR_TIMER as libc::c_int as usize] =
                    (*this).work[MO_TENT_MOVE_TIMER as libc::c_int as usize];
                (*this).actor.world.rot.y = 0x721a as libc::c_int as s16;
                (*sMorphaTent1).work[MO_TENT_ACTION_STATE as libc::c_int as
                                         usize] =
                    MO_TENT_READY as libc::c_int as s16;
                (*sMorphaTent1).timers[0 as libc::c_int as usize] =
                    30000 as libc::c_int as s16;
                Audio_QueueSeqCmd(((0x1 as libc::c_int) << 28 as libc::c_int |
                                       (SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                                           24 as libc::c_int |
                                       0x3200ff as libc::c_int) as u32_0);
                Message_CloseTextbox(globalCtx);
                current_block_216 = 5036859594838248742;
            } else { current_block_216 = 8499731551232998623; }
        }
        2 => { current_block_216 = 5036859594838248742; }
        3 => {
            Math_ApproachF(&mut (*this).cameraYawShake, 0.1f32, 1.0f32,
                           0.002f32);
            (*this).targetPos = cutsceneTargets[(*this).targetIndex as usize];
            if (*this).targetIndex as libc::c_int == 5 as libc::c_int {
                tempY =
                    Math_SinS(((*this).work[MO_TENT_MOVE_TIMER as libc::c_int
                                                as usize] as libc::c_int *
                                   0x500 as libc::c_int) as s16) * 20.0f32
            } else {
                tempY =
                    Math_SinS(((*this).work[MO_TENT_MOVE_TIMER as libc::c_int
                                                as usize] as libc::c_int *
                                   0x500 as libc::c_int) as s16) * 5.0f32
            }
            dx = (*this).targetPos.x - (*this).cameraEye.x;
            dy = (*this).targetPos.y - (*this).cameraEye.y + tempY;
            dz = (*this).targetPos.z - (*this).cameraEye.z;
            tempY = Math_FAtan2F(dx, dz);
            tempX = Math_FAtan2F(dy, sqrtf(dx * dx + dz * dz));
            Math_ApproachS(&mut (*this).actor.world.rot.y,
                           (tempY *
                                (0x8000 as libc::c_int as libc::c_float /
                                     3.14159265358979323846f32)) as s16,
                           5 as libc::c_int as s16,
                           (*this).cameraYawRate as s16);
            Math_ApproachS(&mut (*this).actor.world.rot.x,
                           (tempX *
                                (0x8000 as libc::c_int as libc::c_float /
                                     3.14159265358979323846f32)) as s16,
                           5 as libc::c_int as s16,
                           (*this).cameraYawRate as s16);
            if (*this).work[MO_TENT_MOVE_TIMER as libc::c_int as usize] as
                   libc::c_int == 150 as libc::c_int {
                (*this).cameraAtVel.x =
                    fabsf((*this).cameraAt.x - (*player).actor.world.pos.x);
                (*this).cameraAtVel.y =
                    fabsf((*this).cameraAt.y - (*player).actor.world.pos.y);
                (*this).cameraAtVel.z =
                    fabsf((*this).cameraAt.z - (*player).actor.world.pos.z)
            }
            if (*this).work[MO_TENT_MOVE_TIMER as libc::c_int as usize] as
                   libc::c_int >= 150 as libc::c_int {
                Math_ApproachF(&mut (*this).cameraAt.x,
                               (*player).actor.world.pos.x, 0.1f32,
                               (*this).cameraAtVel.x *
                                   (*this).cameraSpeedMod);
                Math_ApproachF(&mut (*this).cameraAt.y,
                               (*player).actor.world.pos.y + 50.0f32, 0.1f32,
                               (*this).cameraAtVel.y *
                                   (*this).cameraSpeedMod);
                Math_ApproachF(&mut (*this).cameraAt.z,
                               (*player).actor.world.pos.z, 0.1f32,
                               (*this).cameraAtVel.z *
                                   (*this).cameraSpeedMod);
                Math_ApproachF(&mut (*this).cameraSpeedMod, 0.02f32, 1.0f32,
                               0.001f32);
            }
            if (*this).work[MO_TENT_MOVE_TIMER as libc::c_int as usize] as
                   libc::c_int == 190 as libc::c_int {
                func_80078914(&mut sAudioZeroVec,
                              0x38f6 as libc::c_int as u16_0);
            }
            if (*this).work[MO_TENT_MOVE_TIMER as libc::c_int as usize] as
                   libc::c_int > 150 as libc::c_int &&
                   ((*this).work[MO_TENT_MOVE_TIMER as libc::c_int as usize]
                        as libc::c_int) < 180 as libc::c_int {
                bubblePos2.x = (*this).cameraEye.x + 20.0f32 + 10.0f32;
                bubblePos2.y = -250.0f32;
                bubblePos2.z = (*this).cameraEye.z;
                EffectSsBubble_Spawn(globalCtx, &mut bubblePos2, 0.0f32,
                                     10.0f32, 50.0f32,
                                     Rand_ZeroFloat(0.05f32) + 0.13f32);
            }
            sp7C = 0x1000 as libc::c_int as f32_0;
            sp78 = 0.1f32;
            if (*this).work[MO_TENT_MOVE_TIMER as libc::c_int as usize] as
                   libc::c_int > 100 as libc::c_int &&
                   ((*this).work[MO_TENT_MOVE_TIMER as libc::c_int as usize]
                        as libc::c_int) < 220 as libc::c_int {
                sp80 = 0.0f32
            } else if (*this).work[MO_TENT_MOVE_TIMER as libc::c_int as usize]
                          as libc::c_int > 350 as libc::c_int {
                sp80 = 2.0f32;
                sp78 = 0.4f32
            } else if (*this).work[MO_TENT_MOVE_TIMER as libc::c_int as usize]
                          as libc::c_int > 220 as libc::c_int {
                sp80 = 7.0f32;
                sp78 = 0.3f32;
                sp7C = 0x2000 as libc::c_int as f32_0
            } else { sp80 = 4.0f32 }
            if (*this).work[MO_TENT_MOVE_TIMER as libc::c_int as usize] as
                   libc::c_int > 250 as libc::c_int {
                Math_ApproachF(&mut *(*this).fwork.as_mut_ptr().offset(MO_CORE_INTRO_WATER_ALPHA
                                                                           as
                                                                           libc::c_int
                                                                           as
                                                                           isize),
                               100.0f32, 1.0f32, 1.0f32);
            }
            if ((*this).targetIndex as libc::c_int) < 5 as libc::c_int {
                if sqrtf(dx * dx + dz * dz + dy * dy) < 40.0f32 {
                    (*this).targetIndex += 1;
                    (*this).cameraYawRate = 0.0f32
                }
            } else { sp80 = 1.5f32; sp7C = 0x600 as libc::c_int as f32_0 }
            Math_ApproachF(&mut (*this).actor.speedXZ, sp80, 1.0f32, sp78);
            Math_ApproachF(&mut (*this).cameraYawRate, sp7C, 1.0f32,
                           128.0f32);
            if (*this).work[MO_TENT_MOVE_TIMER as libc::c_int as usize] as
                   libc::c_int == 525 as libc::c_int {
                func_8002DF54(globalCtx, &mut (*this).actor,
                              2 as libc::c_int as u8_0);
            }
            if (*this).work[MO_TENT_MOVE_TIMER as libc::c_int as usize] as
                   libc::c_int > 540 as libc::c_int {
                (*this).csState = MO_INTRO_REVEAL as libc::c_int as s16;
                func_8002DF54(globalCtx, &mut (*this).actor,
                              1 as libc::c_int as u8_0);
                (*sMorphaTent1).drawActor = 1 as libc::c_int as u8_0;
                (*player).actor.world.pos.x = 180.0f32;
                (*player).actor.world.pos.z = -210.0f32;
                (*player).actor.world.rot.y = -(0x8000 as libc::c_int) as s16;
                (*player).actor.shape.rot.y = (*player).actor.world.rot.y;
                (*this).cameraYawShake = 0.0f32;
                (*sMorphaTent1).baseAlpha = 150.0f64 as f32_0;
                (*this).actor.speedXZ = 0.0f32;
                (*this).timers[2 as libc::c_int as usize] =
                    200 as libc::c_int as s16;
                (*this).cameraZoom = 60.0f32;
                (*this).actor.world.pos = (*sMorphaTent1).actor.world.pos;
                (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] =
                    MO_CORE_INTRO_REVEAL as libc::c_int as s16;
                (*this).actor.flags &=
                    !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
                (*sMorphaTent1).actor.flags |=
                    ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
                current_block_216 = 5470224165756977677;
            } else {
                (*sMorphaTent1).xSwing = 0xcec as libc::c_int as s16;
                (*sMorphaTent1).fwork[MO_TENT_SWING_RATE_X as libc::c_int as
                                          usize] = 0.0f32;
                (*sMorphaTent1).fwork[MO_TENT_SWING_LAG_X as libc::c_int as
                                          usize] = 1000.0f32;
                (*sMorphaTent1).fwork[MO_TENT_SWING_SIZE_X as libc::c_int as
                                          usize] = 2500.0f32;
                current_block_216 = 8499731551232998623;
            }
        }
        4 => { current_block_216 = 5470224165756977677; }
        5 => {
            sp9F = 1 as libc::c_int as u8_0;
            (*this).cameraNextEye.x = 111.0f32;
            (*this).cameraNextEye.y = 133.0f32;
            (*this).cameraNextEye.z = -191.0f32;
            (*this).cameraNextAt.x = 160.0f32;
            (*this).cameraNextAt.y = 58.0f32;
            (*this).cameraNextAt.z = -247.0f32;
            if (*this).timers[2 as libc::c_int as usize] as libc::c_int ==
                   100 as libc::c_int {
                (*sMorphaTent1).work[MO_TENT_ACTION_STATE as libc::c_int as
                                         usize] =
                    MO_TENT_RETREAT as libc::c_int as s16;
                (*sMorphaTent1).timers[0 as libc::c_int as usize] =
                    50 as libc::c_int as s16
            }
            if (*this).timers[2 as libc::c_int as usize] as libc::c_int ==
                   20 as libc::c_int {
                camera2 =
                    Gameplay_GetCamera(globalCtx, 0 as libc::c_int as s16);
                (*camera2).eye = (*this).cameraEye;
                (*camera2).eyeNext = (*this).cameraEye;
                (*camera2).at = (*this).cameraAt;
                func_800C08AC(globalCtx, (*this).csCamera,
                              0 as libc::c_int as s16);
                (*this).csCamera = MO_BATTLE as libc::c_int as s16;
                (*this).csState = (*this).csCamera;
                func_80064534(globalCtx, &mut (*globalCtx).csCtx);
                func_8002DF54(globalCtx, &mut (*this).actor,
                              7 as libc::c_int as u8_0);
            }
            current_block_216 = 8499731551232998623;
        }
        _ => { current_block_216 = 8499731551232998623; }
    }
    match current_block_216 {
        5470224165756977677 => {
            if (*this).timers[2 as libc::c_int as usize] as libc::c_int >=
                   160 as libc::c_int {
                (*this).cameraEye.x = 150.0f32;
                (*this).cameraEye.y = 60.0f32;
                (*this).cameraEye.z = -230.0f32;
                (*this).cameraAt.x = 170.0f32;
                (*this).cameraAt.y = 40.0f64 as f32_0;
                (*this).cameraAt.z = -280.0f32;
                (*sMorphaTent1).xSwing = 0xcec as libc::c_int as s16;
                (*sMorphaTent1).fwork[MO_TENT_SWING_RATE_X as libc::c_int as
                                          usize] = 0.0f32;
                (*sMorphaTent1).fwork[MO_TENT_SWING_LAG_X as libc::c_int as
                                          usize] = 1000.0f32;
                (*sMorphaTent1).fwork[MO_TENT_SWING_SIZE_X as libc::c_int as
                                          usize] = 2500.0f32;
                if (*this).timers[2 as libc::c_int as usize] as libc::c_int ==
                       160 as libc::c_int {
                    (*this).cameraNextAt.y = 65.0f32;
                    (*this).cameraNextAt.z = -280.0f32;
                    (*this).cameraEyeVel.x =
                        fabsf((*this).cameraEye.x - 150.0f32) * 0.1f32;
                    (*this).cameraEyeVel.y =
                        fabsf((*this).cameraEye.y - 60.0f32) * 0.1f32;
                    (*this).cameraEyeVel.z =
                        fabsf((*this).cameraEye.z - -260.0f32) * 0.1f32;
                    (*this).cameraNextEye.x = 150.0f32;
                    (*this).cameraNextEye.y = 60.0f32;
                    (*this).cameraNextEye.z = -260.0f32;
                    (*this).cameraNextAt.x = 155.0f32;
                    (*this).cameraAtMaxVel.z = 0.1f32;
                    (*this).cameraAtMaxVel.y = (*this).cameraAtMaxVel.z;
                    (*this).cameraAtMaxVel.x = (*this).cameraAtMaxVel.y;
                    (*this).cameraAtVel.x =
                        fabsf((*this).cameraAt.x - (*this).cameraNextAt.x) *
                            0.1f32;
                    (*this).cameraAtVel.y =
                        fabsf((*this).cameraAt.y - (*this).cameraNextAt.y) *
                            0.1f32;
                    (*this).cameraAtVel.z =
                        fabsf((*this).cameraAt.z - (*this).cameraNextAt.z) *
                            0.1f32;
                    (*this).cameraEyeMaxVel.z = 0.1f32;
                    (*this).cameraEyeMaxVel.y = (*this).cameraEyeMaxVel.z;
                    (*this).cameraEyeMaxVel.x = (*this).cameraEyeMaxVel.y;
                    (*this).cameraSpeedMod = 0.0f32;
                    (*this).cameraAccel = 0.01f32;
                    (*this).tentMaxAngle = 0.001f32;
                    (*this).tentSpeed = 0.0f32;
                    sp9F = 1 as libc::c_int as u8_0
                }
            } else { sp9F = 1 as libc::c_int as u8_0 }
            if (*this).timers[2 as libc::c_int as usize] as libc::c_int ==
                   50 as libc::c_int {
                (*this).cameraNextAt.x = 160.0f32;
                (*this).cameraNextAt.y = 58.0f32;
                (*this).cameraNextAt.z = -247.0f32;
                (*this).cameraEyeVel.x =
                    fabsf((*this).cameraEye.x - 111.0f32) * 0.1f32;
                (*this).cameraEyeVel.y =
                    fabsf((*this).cameraEye.y - 133.0f32) * 0.1f32;
                (*this).cameraEyeVel.z =
                    fabsf((*this).cameraEye.z - -191.0f32) * 0.1f32;
                (*this).csState = MO_INTRO_FINISH as libc::c_int as s16;
                (*this).timers[2 as libc::c_int as usize] =
                    110 as libc::c_int as s16;
                (*this).cameraNextEye.x = 111.0f32;
                (*this).cameraNextEye.y = 133.0f32;
                (*this).cameraNextEye.z = -191.0f32;
                (*this).cameraAtVel.x =
                    fabsf((*this).cameraAt.x - (*this).cameraNextAt.x) *
                        0.1f32;
                (*this).cameraAtVel.y =
                    fabsf((*this).cameraAt.y - (*this).cameraNextAt.y) *
                        0.1f32;
                (*this).cameraAtVel.z =
                    fabsf((*this).cameraAt.z - (*this).cameraNextAt.z) *
                        0.1f32;
                (*this).cameraEyeMaxVel.y = 0.03f32;
                (*this).cameraAtMaxVel.y = 0.03f32;
                (*this).cameraSpeedMod = 0.0f32;
                (*this).cameraAccel = 0.01f32
            }
            if (*this).timers[2 as libc::c_int as usize] as libc::c_int ==
                   150 as libc::c_int {
                Audio_QueueSeqCmd(((SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                                       24 as libc::c_int |
                                       0x1b as libc::c_int) as u32_0);
            }
            if (*this).timers[2 as libc::c_int as usize] as libc::c_int ==
                   130 as libc::c_int {
                TitleCard_InitBossName(globalCtx,
                                       &mut (*globalCtx).actorCtx.titleCtx,
                                       gSegments[((gMorphaTitleCardTex.as_mut_ptr()
                                                       as u32_0) <<
                                                      4 as libc::c_int >>
                                                      28 as libc::c_int) as
                                                     usize].wrapping_add(gMorphaTitleCardTex.as_mut_ptr()
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
                         libc::c_int | 0x10 as libc::c_int) as u16_0
            }
        }
        5036859594838248742 => {
            (*player).actor.world.pos.x = 180.0f32;
            (*player).actor.world.pos.z = -130.0f32;
            (*player).actor.world.rot.y = 0 as libc::c_int as s16;
            (*player).actor.shape.rot.y = (*player).actor.world.rot.y;
            (*player).actor.speedXZ = 0.0f32;
            (*this).cameraEye.x = -424.0f32;
            (*this).cameraEye.y = -190.0f32;
            (*this).cameraEye.z = 180.0f32;
            (*this).cameraAt.x = (*player).actor.world.pos.x;
            (*this).cameraAt.y = -330.0f32;
            (*this).cameraAt.z = 0.0f32;
            if (*this).timers[2 as libc::c_int as usize] as libc::c_int ==
                   0 as libc::c_int {
                (*this).csState = MO_INTRO_SWIM as libc::c_int as s16;
                (*this).work[MO_TENT_MOVE_TIMER as libc::c_int as usize] =
                    0 as libc::c_int as s16
            } else if ((*this).timers[2 as libc::c_int as usize] as
                           libc::c_int) < 50 as libc::c_int {
                bubblePos.x = (*this).cameraEye.x + 20.0f32 + 10.0f32;
                bubblePos.y = -250.0f32;
                bubblePos.z = (*this).cameraEye.z;
                EffectSsBubble_Spawn(globalCtx, &mut bubblePos, 0.0f32,
                                     10.0f32, 50.0f32,
                                     Rand_ZeroFloat(0.05f32) + 0.13f32);
            }
            if (*this).timers[2 as libc::c_int as usize] as libc::c_int ==
                   40 as libc::c_int {
                func_80078914(&mut sAudioZeroVec,
                              0x38f6 as libc::c_int as u16_0);
            }
        }
        _ => { }
    }
    if (*sMorphaTent1).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
           libc::c_int == MO_TENT_READY as libc::c_int {
        (*sMorphaTent1).actor.world.pos.x = 180.0f32;
        (*sMorphaTent1).actor.world.pos.z = -360.0f32;
        (*sMorphaTent1).actor.prevPos = (*sMorphaTent1).actor.world.pos;
        (*sMorphaTent1).actor.speedXZ = 0.0f32;
        (*sMorphaTent1).actor.shape.rot.y =
            (*sMorphaTent1).actor.yawTowardsPlayer
    }
    if (*this).csCamera as libc::c_int != 0 as libc::c_int {
        if sp9F != 0 {
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
        } else if ((*this).csState as libc::c_int) <
                      MO_INTRO_REVEAL as libc::c_int {
            func_8002D908(&mut (*this).actor);
            (*this).cameraEye.x += (*this).actor.velocity.x;
            (*this).cameraEye.y += (*this).actor.velocity.y;
            (*this).cameraEye.z += (*this).actor.velocity.z
        }
        (*this).cameraUp.z =
            sinf((*this).work[MO_TENT_VAR_TIMER as libc::c_int as usize] as
                     libc::c_int as libc::c_float * 0.03f32) *
                (*this).cameraYawShake * -2.0f32;
        (*this).cameraUp.x = (*this).cameraUp.z;
        (*this).cameraUp.y = 1.0f32;
        Gameplay_CameraSetAtEyeUp(globalCtx, (*this).csCamera,
                                  &mut (*this).cameraAt,
                                  &mut (*this).cameraEye,
                                  &mut (*this).cameraUp);
        (*camera).eye = (*this).cameraEye;
        (*camera).eyeNext = (*this).cameraEye;
        (*camera).at = (*this).cameraAt;
        Gameplay_CameraSetFov(globalCtx, (*this).csCamera,
                              (*this).cameraZoom);
    }
    if (*this).csState as libc::c_int > MO_INTRO_START as libc::c_int &&
           (*this).work[MO_TENT_MOVE_TIMER as libc::c_int as usize] as
               libc::c_int > 540 as libc::c_int {
        func_80078914(&mut (*sMorphaTent1).tentTipPos,
                      (0x38f0 as libc::c_int - 0x800 as libc::c_int) as
                          u16_0);
    } else if (*this).csState as libc::c_int >= MO_INTRO_START as libc::c_int
     {
        func_80078914(&mut sAudioZeroVec,
                      (0x38f5 as libc::c_int - 0x800 as libc::c_int) as
                          u16_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossMo_DeathCs(mut this: *mut BossMo,
                                        mut globalCtx: *mut GlobalContext) {
    let mut i: s16 = 0;
    let mut one: s16 = 0;
    let mut dx: f32_0 = 0.;
    let mut dz: f32_0 = 0.;
    let mut sp80: f32_0 = 0.;
    let mut sp7C: f32_0 = 0.;
    let mut sp70: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp64: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut camera: *mut Camera =
        Gameplay_GetCamera(globalCtx, 0 as libc::c_int as s16);
    let mut velocity: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut current_block_167: u64;
    match (*this).csState as libc::c_int {
        100 => {
            func_80064520(globalCtx, &mut (*globalCtx).csCtx);
            func_8002DF54(globalCtx, &mut (*this).actor,
                          8 as libc::c_int as u8_0);
            (*this).csCamera = Gameplay_CreateSubCamera(globalCtx);
            Gameplay_ChangeCameraStatus(globalCtx, 0 as libc::c_int as s16,
                                        1 as libc::c_int as s16);
            Gameplay_ChangeCameraStatus(globalCtx, (*this).csCamera,
                                        7 as libc::c_int as s16);
            (*this).csState = MO_DEATH_MO_CORE_BURST as libc::c_int as s16;
            (*this).cameraEye = (*camera).eye;
            (*this).timers[0 as libc::c_int as usize] =
                90 as libc::c_int as s16;
            dx = (*this).actor.world.pos.x - (*this).cameraEye.x;
            dz = (*this).actor.world.pos.z - (*this).cameraEye.z;
            (*this).cameraYaw = Math_FAtan2F(dx, dz);
            (*this).cameraDist = sqrtf(dx * dx + dz * dz);
            (*this).cameraYawRate = 0.0f32;
            current_block_167 = 3512525685711344514;
        }
        150 => { current_block_167 = 3512525685711344514; }
        101 => {
            if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
                   0 as libc::c_int {
                (*this).csState =
                    MO_DEATH_DRAIN_WATER_2 as libc::c_int as s16;
                (*this).cameraAt.y = -200.0f32;
                (*this).cameraNextAt.y = 320.0f32;
                (*this).cameraAtMaxVel.y = 0.05f32;
                (*this).cameraAtVel.y = 4.0f32;
                (*this).cameraSpeedMod = 0.0f32;
                (*this).cameraAccel = 0.0f32;
                (*sMorphaTent1).work[MO_TENT_ACTION_STATE as libc::c_int as
                                         usize] =
                    MO_TENT_DEATH_1 as libc::c_int as s16;
                (*this).timers[0 as libc::c_int as usize] =
                    125 as libc::c_int as s16;
                (*sMorphaTent1).fwork[MO_TENT_MAX_STRETCH as libc::c_int as
                                          usize] = 3.7000003f32;
                (*this).cameraYaw = 0.5f32;
                (*this).cameraDist = 200.0f32;
                return
            }
            current_block_167 = 1745632252074978848;
        }
        102 => {
            if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
                   0 as libc::c_int {
                (*this).cameraAccel = 0.02f32;
                (*sMorphaTent1).work[MO_TENT_ACTION_STATE as libc::c_int as
                                         usize] =
                    MO_TENT_DEATH_2 as libc::c_int as s16;
                (*this).csState = MO_DEATH_CEILING as libc::c_int as s16;
                (*sMorphaTent1).timers[0 as libc::c_int as usize] =
                    120 as libc::c_int as s16;
                (*this).timers[0 as libc::c_int as usize] =
                    150 as libc::c_int as s16
            }
            current_block_167 = 16021065330066866284;
        }
        103 => { current_block_167 = 16021065330066866284; }
        104 => {
            if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
                   30 as libc::c_int {
                (*sMorphaTent1).work[MO_TENT_ACTION_STATE as libc::c_int as
                                         usize] =
                    MO_TENT_DEATH_5 as libc::c_int as s16;
                (*sMorphaTent1).timers[0 as libc::c_int as usize] =
                    30 as libc::c_int as s16;
                (*sMorphaTent1).tentMaxAngle = 0.0f32;
                (*sMorphaTent1).tentSpeed = (*sMorphaTent1).tentMaxAngle
            }
            if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
                   0 as libc::c_int {
                if -100.0f32 < (*this).cameraEye.y {
                    Math_ApproachF(&mut (*this).cameraEye.y,
                                   (*sMorphaTent1).actor.world.pos.y -
                                       100.0f32, 0.1f32, 2000.0f32);
                } else {
                    Math_ApproachF(&mut (*this).cameraEye.y, -200.0f32,
                                   0.1f32, 2000.0f32);
                }
                Math_ApproachF(&mut (*this).cameraAt.y,
                               (*sMorphaTent1).actor.world.pos.y - 50.0f32 +
                                   30.0f32, 0.5f32, 2000.0f32);
                (*this).cameraNextAt.y = (*this).cameraAt.y
            } else {
                Math_ApproachF(&mut (*this).cameraEye.y, 300.0f32, 0.05f32,
                               (*this).cameraSpeed);
            }
            Math_ApproachF(&mut (*this).cameraYaw,
                           -3.14159265358979323846f32 / 2.0f32, 0.05f32,
                           (*this).cameraYawRate);
            Math_ApproachF(&mut (*this).cameraSpeed, 3.0f32, 1.0f32, 0.05f32);
            Math_ApproachF(&mut (*this).cameraYawRate, 0.012999999f32, 1.0f32,
                           0.0005f32);
            if (*sMorphaTent1).work[MO_TENT_ACTION_STATE as libc::c_int as
                                        usize] as libc::c_int ==
                   MO_TENT_DEATH_6 as libc::c_int {
                Math_ApproachF(&mut (*this).cameraDist, 200.0f32, 0.02f32,
                               (*this).cameraSpeed);
                if (*sMorphaTent1).timers[0 as libc::c_int as usize] as
                       libc::c_int == 0 as libc::c_int {
                    (*this).csState = MO_DEATH_FINISH as libc::c_int as s16;
                    (*camera).eye = (*this).cameraEye;
                    (*camera).eyeNext = (*this).cameraEye;
                    (*camera).at = (*this).cameraAt;
                    func_800C08AC(globalCtx, (*this).csCamera,
                                  0 as libc::c_int as s16);
                    (*this).csCamera = 0 as libc::c_int as s16;
                    func_80064534(globalCtx, &mut (*globalCtx).csCtx);
                    func_8002DF54(globalCtx, &mut (*this).actor,
                                  7 as libc::c_int as u8_0);
                    (*sMorphaTent1).actor.world.pos.y = -1000.0f32
                }
            } else {
                Math_ApproachF(&mut (*this).cameraDist, 150.0f32, 0.05f32,
                               (*this).cameraSpeed);
            }
            current_block_167 = 1745632252074978848;
        }
        105 | _ => { current_block_167 = 1745632252074978848; }
    }
    match current_block_167 {
        16021065330066866284 => {
            Math_ApproachF(&mut (*this).cameraYaw, 0.0f32, 0.05f32,
                           0.0029999996f32);
            Math_ApproachF(&mut (*this).cameraDist, 490.0f32, 0.1f32, 1.0f32);
            if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
                   0 as libc::c_int {
                (*this).csState = MO_DEATH_DROPLET as libc::c_int as s16;
                (*this).timers[0 as libc::c_int as usize] =
                    140 as libc::c_int as s16;
                (*this).cameraYawRate = 0.0f32;
                (*this).cameraSpeed = 0.0f32
            }
        }
        3512525685711344514 => {
            (*this).baseAlpha = 0.0f32;
            if (*this).timers[0 as libc::c_int as usize] as libc::c_int &
                   4 as libc::c_int != 0 {
                sp80 = 0.005f32;
                sp7C = 0.015f32
            } else { sp80 = 0.015f32; sp7C = 0.005f32 }
            Math_ApproachF(&mut (*this).actor.scale.x, sp80, 0.5f32,
                           0.002f32);
            (*this).actor.scale.z = (*this).actor.scale.x;
            Math_ApproachF(&mut (*this).actor.scale.y, sp7C, 0.5f32,
                           0.002f32);
            (*this).cameraYaw += (*this).cameraYawRate;
            if (*this).timers[0 as libc::c_int as usize] as libc::c_int >=
                   30 as libc::c_int {
                Math_ApproachF(&mut (*this).cameraYawRate, 0.05f32, 1.0f32,
                               0.002f32);
            } else {
                Math_ApproachF(&mut (*this).cameraYawRate, 0.0f32, 1.0f32,
                               0.002f32);
            }
            Math_ApproachF(&mut (*this).actor.world.pos.y, 150.0f32, 0.05f32,
                           5.0f32);
            Math_ApproachF(&mut (*this).cameraEye.y, 100.0f32, 0.05f32,
                           2.0f32);
            (*this).cameraNextAt = (*this).actor.world.pos;
            (*this).cameraAt = (*this).cameraNextAt;
            if (*this).timers[0 as libc::c_int as usize] as libc::c_int >
                   20 as libc::c_int {
                Audio_PlayActorSound2(&mut (*this).actor,
                                      (0x38f8 as libc::c_int -
                                           0x800 as libc::c_int) as u16_0);
            }
            if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
                   20 as libc::c_int {
                i = 0 as libc::c_int as s16;
                while (i as libc::c_int) < 300 as libc::c_int {
                    velocity.x = Rand_CenteredFloat(10.0f32);
                    velocity.y = Rand_CenteredFloat(10.0f32);
                    velocity.z = Rand_CenteredFloat(10.0f32);
                    pos = (*this).actor.world.pos;
                    pos.x += 2.0f32 * velocity.x;
                    pos.y += 2.0f32 * velocity.y;
                    pos.z += 2.0f32 * velocity.z;
                    BossMo_SpawnDroplet(MO_FX_DROPLET as libc::c_int as s16,
                                        (*globalCtx).specialEffects as
                                            *mut BossMoEffect, &mut pos,
                                        &mut velocity,
                                        Rand_ZeroFloat(0.08f32) + 0.13f32);
                    i += 1
                }
                (*this).drawActor = 0 as libc::c_int as u8_0;
                (*this).actor.flags &=
                    !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
                Audio_PlayActorSound2(&mut (*this).actor,
                                      0x38f7 as libc::c_int as u16_0);
                Audio_PlaySoundAtPosition(globalCtx,
                                          &mut (*this).actor.world.pos,
                                          70 as libc::c_int,
                                          0x38f9 as libc::c_int as u16_0);
            }
            if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
                   0 as libc::c_int {
                (*this).csState =
                    MO_DEATH_DRAIN_WATER_1 as libc::c_int as s16;
                (*this).cameraDist = 490.0f32;
                (*this).actor.world.pos.y = -1000.0f32;
                (*this).fwork[MO_TENT_SWING_SIZE_X as libc::c_int as usize] =
                    15.0f32;
                (*this).cameraYaw = 0.0f32;
                (*this).cameraEye.x = 490.0f32;
                (*this).cameraEye.y = 50.0f32;
                (*this).cameraEye.z = 0.0f32;
                (*this).cameraAt.x = 0 as libc::c_int as f32_0;
                (*this).cameraAt.y = -100.0f32;
                (*this).cameraAt.z = 0.0f32;
                (*this).work[MO_TENT_MOVE_TIMER as libc::c_int as usize] =
                    0 as libc::c_int as s16;
                (*this).work[MO_TENT_VAR_TIMER as libc::c_int as usize] =
                    (*this).work[MO_TENT_MOVE_TIMER as libc::c_int as usize];
                (*this).cameraAtMaxVel.y = 0.05f32;
                (*this).cameraAtVel.y = 4.0f32;
                (*this).cameraSpeedMod = 0.0f32;
                (*this).cameraAccel = 0.02f32;
                (*this).cameraNextAt.y = 320.0f32;
                (*this).timers[0 as libc::c_int as usize] =
                    100 as libc::c_int as s16;
                (*sMorphaTent1).drawActor = 1 as libc::c_int as u8_0;
                (*sMorphaTent1).work[MO_TENT_ACTION_STATE as libc::c_int as
                                         usize] =
                    MO_TENT_DEATH_3 as libc::c_int as s16;
                (*sMorphaTent1).actor.shape.rot.x = 0 as libc::c_int as s16;
                (*sMorphaTent1).actor.world.pos.x = 0.0f32;
                (*sMorphaTent1).actor.world.pos.y = -50.0f32;
                (*sMorphaTent1).actor.world.pos.z = 0.0f32;
                (*sMorphaTent1).fwork[MO_TENT_MAX_STRETCH as libc::c_int as
                                          usize] = 1.0f32;
                (*sMorphaTent1).tentPulse = 0.2f32;
                (*sMorphaCore).waterLevel = -50.0f32;
                (*sMorphaTent1).flattenRate = 0.0f32;
                (*sMorphaTent1).noBubbles = 0 as libc::c_int as s16;
                i = 0 as libc::c_int as s16;
                while (i as libc::c_int) < 41 as libc::c_int {
                    (*sMorphaTent1).tentStretch[i as usize].y = 5.0f32;
                    i += 1
                }
                (*sMorphaTent1).fwork[MO_TENT_SWING_RATE_X as libc::c_int as
                                          usize] = -400.0f32;
                (*sMorphaTent1).fwork[MO_TENT_SWING_LAG_X as libc::c_int as
                                          usize] = -3200.0f32;
                (*sMorphaTent1).fwork[MO_TENT_SWING_SIZE_X as libc::c_int as
                                          usize] = 0.0f32;
                (*sMorphaTent1).fwork[MO_TENT_SWING_RATE_Z as libc::c_int as
                                          usize] = 3000.0f32;
                (*sMorphaTent1).fwork[MO_TENT_SWING_LAG_Z as libc::c_int as
                                          usize] = 2500.0f32;
                (*sMorphaTent1).fwork[MO_TENT_SWING_SIZE_Z as libc::c_int as
                                          usize] = 4000.0f32;
                (*sMorphaTent1).tentMaxAngle = 1.0f32;
                (*sMorphaTent1).tentSpeed = 20480.0f32;
                (*sMorphaTent1).baseAlpha = 150.0f32;
                (*sMorphaTent1).meltIndex = 0 as libc::c_int as s16;
                (*sMorphaTent1).cutIndex = (*sMorphaTent1).meltIndex;
                (*sMorphaTent1).cutScale = 1.0f32;
                Actor_SetScale(&mut (*sMorphaTent1).actor, 0.01f32);
            }
        }
        _ => { }
    }
    if (*this).csState as libc::c_int > MO_DEATH_START as libc::c_int &&
           ((*this).csState as libc::c_int) < MO_DEATH_FINISH as libc::c_int {
        if ((*this).work[MO_TENT_MOVE_TIMER as libc::c_int as usize] as
                libc::c_int) < 500 as libc::c_int {
            func_80078914(&mut sAudioZeroVec,
                          (0x38f0 as libc::c_int - 0x800 as libc::c_int) as
                              u16_0);
        }
        if ((*this).work[MO_TENT_MOVE_TIMER as libc::c_int as usize] as
                libc::c_int) < 490 as libc::c_int &&
               (*this).work[MO_TENT_MOVE_TIMER as libc::c_int as usize] as
                   libc::c_int > 230 as libc::c_int {
            func_80078914(&mut sAudioZeroVec,
                          (0x2875 as libc::c_int - 0x800 as libc::c_int) as
                              u16_0);
        }
        if ((*this).work[MO_TENT_MOVE_TIMER as libc::c_int as usize] as
                libc::c_int) < 220 as libc::c_int {
            func_80078914(&mut sAudioZeroVec,
                          (0x2874 as libc::c_int - 0x800 as libc::c_int) as
                              u16_0);
        }
    }
    if (*sMorphaCore).waterLevel < -200.0f32 {
        (*globalCtx).roomCtx.unk_74[0 as libc::c_int as usize] += 1;
        if (*globalCtx).roomCtx.unk_74[0 as libc::c_int as usize] as
               libc::c_int >= 0xff as libc::c_int {
            (*globalCtx).roomCtx.unk_74[0 as libc::c_int as usize] =
                0xff as libc::c_int as s16
        }
    }
    if (*sMorphaCore).waterLevel < -250.0f32 {
        Math_ApproachF(&mut (*sMorphaTent1).waterTexAlpha, 0.0f32, 1.0f32,
                       3.0f32);
    }
    Math_ApproachF(&mut *(*this).fwork.as_mut_ptr().offset(MO_TENT_SWING_SIZE_X
                                                               as libc::c_int
                                                               as isize),
                   0.0f32, 0.1f32, 0.05f32);
    sp70.x = (*this).cameraDist;
    sp70.y = 0.0f32;
    sp70.z = 0.0f32;
    Matrix_RotateY((*this).cameraYaw, MTXMODE_NEW as libc::c_int as u8_0);
    Matrix_MultVec3f(&mut sp70, &mut sp64);
    (*this).cameraEye.x = sp64.x + (*this).cameraAt.x;
    (*this).cameraEye.z = sp64.z + (*this).cameraAt.z;
    one = 1 as libc::c_int as s16;
    if (*this).csCamera as libc::c_int != 0 as libc::c_int {
        if one != 0 {
            Math_ApproachF(&mut (*this).cameraAt.y, (*this).cameraNextAt.y,
                           (*this).cameraAtMaxVel.y,
                           (*this).cameraAtVel.y * (*this).cameraSpeedMod);
            Math_ApproachF(&mut (*this).cameraSpeedMod, 1.0f32, 1.0f32,
                           (*this).cameraAccel);
        }
        Gameplay_CameraSetAtEye(globalCtx, (*this).csCamera,
                                &mut (*this).cameraAt,
                                &mut (*this).cameraEye);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossMo_CoreCollisionCheck(mut this: *mut BossMo,
                                                   mut globalCtx:
                                                       *mut GlobalContext) {
    let mut i: s16 = 0;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    osSyncPrintf(b"\x1b[33m\x00" as *const u8 as *const libc::c_char);
    osSyncPrintf(b"Core_Damage_check START\n\x00" as *const u8 as
                     *const libc::c_char);
    if (*this).coreCollider.base.atFlags as libc::c_int &
           (1 as libc::c_int) << 1 as libc::c_int != 0 {
        (*this).coreCollider.base.atFlags =
            ((*this).coreCollider.base.atFlags as libc::c_int &
                 !((1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
        if (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
               libc::c_int == MO_CORE_UNDERWATER as libc::c_int {
            (*this).work[MO_CORE_WAIT_IN_WATER as libc::c_int as usize] =
                1 as libc::c_int as s16;
            (*this).timers[0 as libc::c_int as usize] =
                150 as libc::c_int as s16
        }
    }
    if (*this).coreCollider.base.acFlags as libc::c_int &
           (1 as libc::c_int) << 1 as libc::c_int != 0 {
        let mut hurtbox: *mut ColliderInfo =
            (*this).coreCollider.info.acHitInfo;
        // "hit!!"
        osSyncPrintf(b"Core_Damage_check \xe5\xbd\x93\xe3\x82\x8a\xef\xbc\x81\xef\xbc\x81\n\x00"
                         as *const u8 as *const libc::c_char);
        (*this).coreCollider.base.acFlags =
            ((*this).coreCollider.base.acFlags as libc::c_int &
                 !((1 as libc::c_int) << 1 as libc::c_int)) as u8_0;
        if (*hurtbox).toucher.dmgFlags &
               0x20000 as libc::c_int as libc::c_uint != 0 &&
               (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
                   libc::c_int == MO_CORE_ATTACK as libc::c_int {
            (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] =
                MO_CORE_RETREAT as libc::c_int as s16
        }
        // "hit 2 !!"
        osSyncPrintf(b"Core_Damage_check \xe5\xbd\x93\xe3\x82\x8a 2 \xef\xbc\x81\xef\xbc\x81\n\x00"
                         as *const u8 as *const libc::c_char);
        if (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
               libc::c_int != MO_CORE_UNDERWATER as libc::c_int &&
               (*this).work[MO_TENT_INVINC_TIMER as libc::c_int as usize] as
                   libc::c_int == 0 as libc::c_int {
            let mut damage: u8_0 =
                CollisionCheck_GetSwordDamage((*hurtbox).toucher.dmgFlags as
                                                  s32);
            if damage as libc::c_int != 0 as libc::c_int &&
                   ((*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize]
                        as libc::c_int) < MO_CORE_ATTACK as libc::c_int {
                // "sword hit !!"
                osSyncPrintf(b"Core_Damage_check \xe5\x89\xa3 \xe5\xbd\x93\xe3\x82\x8a\xef\xbc\x81\xef\xbc\x81\n\x00"
                                 as *const u8 as *const libc::c_char);
                (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] =
                    MO_CORE_STUNNED as libc::c_int as s16;
                (*this).timers[0 as libc::c_int as usize] =
                    25 as libc::c_int as s16;
                (*this).actor.speedXZ = 15.0f32;
                (*this).actor.world.rot.y =
                    ((*this).actor.yawTowardsPlayer as libc::c_int +
                         0x8000 as libc::c_int) as s16;
                (*this).work[MO_CORE_DMG_FLASH_TIMER as libc::c_int as usize]
                    = 15 as libc::c_int as s16;
                Audio_PlayActorSound2(&mut (*this).actor,
                                      0x39ec as libc::c_int as u16_0);
                (*this).actor.colChkInfo.health =
                    ((*this).actor.colChkInfo.health as libc::c_int -
                         damage as libc::c_int) as u8_0;
                (*this).hitCount = (*this).hitCount.wrapping_add(1);
                if (*this).actor.colChkInfo.health as s8 as libc::c_int <=
                       0 as libc::c_int {
                    if (*sMorphaTent1).csCamera as libc::c_int ==
                           0 as libc::c_int && sMorphaTent2.is_null() ||
                           (*sMorphaTent1).csCamera as libc::c_int ==
                               0 as libc::c_int && !sMorphaTent2.is_null() &&
                               (*sMorphaTent2).csCamera as libc::c_int ==
                                   0 as libc::c_int {
                        Enemy_StartFinishingBlow(globalCtx,
                                                 &mut (*this).actor);
                        Audio_QueueSeqCmd(((0x1 as libc::c_int) <<
                                               28 as libc::c_int |
                                               (SEQ_PLAYER_BGM_MAIN as
                                                    libc::c_int) <<
                                                   24 as libc::c_int |
                                               0x100ff as libc::c_int) as
                                              u32_0);
                        (*this).csState =
                            MO_DEATH_START as libc::c_int as s16;
                        (*sMorphaTent1).drawActor = 0 as libc::c_int as u8_0;
                        (*sMorphaTent1).work[MO_TENT_ACTION_STATE as
                                                 libc::c_int as usize] =
                            MO_TENT_DEATH_START as libc::c_int as s16;
                        (*sMorphaTent1).baseAlpha = 0.0f32;
                        if !sMorphaTent2.is_null() {
                            (*sMorphaTent2).tent2KillTimer =
                                1 as libc::c_int as u8_0
                        }
                        if !(*player).actor.parent.is_null() {
                            (*player).unk_850 = 0x65 as libc::c_int as s16;
                            (*player).actor.parent = 0 as *mut Actor;
                            (*player).csMode = 0 as libc::c_int as u8_0
                        }
                    } else {
                        (*this).actor.colChkInfo.health =
                            1 as libc::c_int as u8_0
                    }
                }
                (*this).work[MO_TENT_INVINC_TIMER as libc::c_int as usize] =
                    10 as libc::c_int as s16
            } else if (*hurtbox).toucher.dmgFlags &
                          0x100000 as libc::c_int as libc::c_uint == 0 &&
                          (*hurtbox).toucher.dmgFlags &
                              0x80 as libc::c_int as libc::c_uint != 0 {
                if (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize]
                       as libc::c_int >= MO_CORE_ATTACK as libc::c_int {
                    func_80078914(&mut (*sMorphaTent1).tentTipPos,
                                  0x38f4 as libc::c_int as u16_0);
                    (*sMorphaTent1).cutIndex =
                        (*this).work[MO_CORE_POS_IN_TENT as libc::c_int as
                                         usize];
                    (*sMorphaTent1).meltIndex =
                        ((*sMorphaTent1).cutIndex as libc::c_int +
                             1 as libc::c_int) as s16;
                    (*sMorphaTent1).cutScale = 1.0f32;
                    (*sMorphaTent1).work[MO_TENT_ACTION_STATE as libc::c_int
                                             as usize] =
                        MO_TENT_CUT as libc::c_int as s16;
                    (*sMorphaTent1).timers[0 as libc::c_int as usize] =
                        40 as libc::c_int as s16;
                    (*sMorphaTent1).actor.flags &=
                        !((1 as libc::c_int) << 0 as libc::c_int) as
                            libc::c_uint;
                    if (*player).actor.parent ==
                           &mut (*sMorphaTent1).actor as *mut Actor {
                        (*player).unk_850 = 0x65 as libc::c_int as s16;
                        (*player).actor.parent = 0 as *mut Actor;
                        (*player).csMode = 0 as libc::c_int as u8_0
                    }
                }
                (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] =
                    MO_CORE_STUNNED as libc::c_int as s16;
                (*this).timers[0 as libc::c_int as usize] =
                    30 as libc::c_int as s16;
                (*this).work[MO_TENT_INVINC_TIMER as libc::c_int as usize] =
                    10 as libc::c_int as s16;
                (*this).actor.speedXZ = 0.0f32
            }
            i = 0 as libc::c_int as s16;
            while (i as libc::c_int) < 10 as libc::c_int {
                let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                let mut velocity: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                velocity.x = Rand_CenteredFloat(4.0f32);
                velocity.y = Rand_ZeroFloat(2.0f32) + 3.0f32;
                velocity.z = Rand_CenteredFloat(4.0f32);
                pos = (*this).actor.world.pos;
                pos.x += velocity.x * 3.0f32;
                pos.z += velocity.z * 3.0f32;
                BossMo_SpawnDroplet(MO_FX_DROPLET as libc::c_int as s16,
                                    (*globalCtx).specialEffects as
                                        *mut BossMoEffect, &mut pos,
                                    &mut velocity,
                                    Rand_ZeroFloat(0.08f32) + 0.13f32);
                i += 1
            }
        }
    }
    // "end !!"
    osSyncPrintf(b"Core_Damage_check \xe7\xb5\x82\xe3\x82\x8f\xe3\x82\x8a \xef\xbc\x81\xef\xbc\x81\n\x00"
                     as *const u8 as *const libc::c_char); // not on stack
    osSyncPrintf(b"\x1b[m\x00" as *const u8 as
                     *const libc::c_char); // not on stack
}
#[no_mangle]
pub unsafe extern "C" fn BossMo_Core(mut this: *mut BossMo,
                                     mut globalCtx: *mut GlobalContext) {
    static mut coreBulge: [f32_0; 11] =
        [0.1f32, 0.15f32, 0.2f32, 0.3f32, 0.4f32, 0.43f32, 0.4f32, 0.3f32,
         0.2f32, 0.15f32, 0.1f32]; // not on stack
    let mut nearLand: u8_0 = 0; // not on stack
    let mut i: s16 = 0; // not on stack
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut spDC: f32_0 = 0.;
    let mut spD8: f32_0 = 0.;
    let mut spD4: f32_0 = 0.;
    let mut spD0: f32_0 = 0.;
    let mut spCC: f32_0 = 0.;
    let mut padC8: s32 = 0;
    let mut temp: s32 = 0;
    let mut xScaleTarget: f32_0 = 0.;
    let mut yScaleTarget: f32_0 = 0.;
    let mut effectPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut effectVelocity: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut effectAccel: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut pad94: s32 = 0;
    let mut pad90: s32 = 0;
    let mut j: s16 = 0;
    let mut index: s16 = 0;
    let mut sp88: f32_0 = 0.;
    let mut pad84: s32 = 0;
    let mut sp80: f32_0 = 0.;
    let mut sp7C: f32_0 = 0.;
    let mut sp70: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp64: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp60: f32_0 = 0.;
    let mut sp5C: f32_0 = 0.;
    let mut sp58: f32_0 = 0.;
    (*this).waterTex1x += -1.0f32;
    (*this).waterTex1y += -1.0f32;
    (*this).waterTex2x = (*this).waterTex2x;
    (*this).waterTex2y += 1.;
    Math_ApproachF(&mut (*this).baseAlpha, 255.0f32, 1.0f32, 10.0f32);
    if (*this).csState as libc::c_int != MO_BATTLE as libc::c_int &&
           ((*this).csState as libc::c_int) < MO_DEATH_START as libc::c_int {
        BossMo_IntroCs(this, globalCtx);
        if (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
               libc::c_int == MO_CORE_INTRO_WAIT as libc::c_int {
            (*this).actor.flags &=
                !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
            return
        }
    } else if (*this).csState as libc::c_int >= MO_DEATH_START as libc::c_int
     {
        BossMo_DeathCs(this, globalCtx);
        return
    }
    if ((*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
            libc::c_int) < MO_CORE_ATTACK as libc::c_int &&
           (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
               libc::c_int >= MO_CORE_MOVE as libc::c_int &&
           (*this).actor.world.pos.y >
               (*(*(*globalCtx).colCtx.colHeader).waterBoxes.offset(0 as
                                                                        libc::c_int
                                                                        as
                                                                        isize)).ySurface
                   as libc::c_int as libc::c_float {
        if (*this).actor.velocity.y > 0.0f32 {
            xScaleTarget = 0.005f32;
            yScaleTarget = 0.015f32
        } else { xScaleTarget = 0.015f32; yScaleTarget = 0.005f32 }
    } else { yScaleTarget = 0.008f32; xScaleTarget = yScaleTarget }
    Math_ApproachF(&mut (*this).actor.scale.x, xScaleTarget, 0.2f32,
                   0.001f32);
    (*this).actor.scale.z = (*this).actor.scale.x;
    Math_ApproachF(&mut (*this).actor.scale.y, yScaleTarget, 0.2f32,
                   0.001f32);
    (*this).work[MO_CORE_DRAW_SHADOW as libc::c_int as usize] =
        BossMo_NearLand(&mut (*this).actor.world.pos, 15.0f32) as s16;
    nearLand = BossMo_NearLand(&mut (*this).actor.world.pos, 0.0f32) as u8_0;
    if (*player).actor.world.pos.y <
           (*(*(*globalCtx).colCtx.colHeader).waterBoxes.offset(0 as
                                                                    libc::c_int
                                                                    as
                                                                    isize)).ySurface
               as libc::c_int as libc::c_float - 50.0f32 &&
           ((*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
                libc::c_int == MO_CORE_MOVE as libc::c_int ||
                (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
                    libc::c_int == MO_CORE_MAKE_TENT as libc::c_int) {
        (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] =
            MO_CORE_UNDERWATER as libc::c_int as s16;
        (*this).actor.speedXZ = 0.0f32;
        (*this).work[MO_CORE_WAIT_IN_WATER as libc::c_int as usize] =
            0 as libc::c_int as s16
    }
    match (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
              libc::c_int {
        0 => {
            (*this).actor.flags |=
                ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
            if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
                   0 as libc::c_int &&
                   ((*sMorphaTent1).work[MO_TENT_ACTION_STATE as libc::c_int
                                             as usize] as libc::c_int ==
                        MO_TENT_WAIT as libc::c_int ||
                        (*sMorphaTent1).work[MO_TENT_ACTION_STATE as
                                                 libc::c_int as usize] as
                            libc::c_int == MO_TENT_READY as libc::c_int) &&
                   (*this).actor.world.pos.y <
                       (*(*(*globalCtx).colCtx.colHeader).waterBoxes.offset(0
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize)).ySurface
                           as libc::c_int as libc::c_float {
                (*this).actor.speedXZ = 0.0f32;
                (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] =
                    MO_CORE_MAKE_TENT as libc::c_int as s16;
                if (*sMorphaTent1).work[MO_TENT_ACTION_STATE as libc::c_int as
                                            usize] as libc::c_int ==
                       MO_TENT_WAIT as libc::c_int {
                    (*sMorphaTent1).work[MO_TENT_ACTION_STATE as libc::c_int
                                             as usize] =
                        MO_TENT_SPAWN as libc::c_int as s16;
                    (*sMorphaTent1).timers[0 as libc::c_int as usize] =
                        70 as libc::c_int as s16;
                    (*sMorphaTent1).actor.shape.rot.y =
                        (*sMorphaTent1).actor.yawTowardsPlayer
                }
            }
        }
        1 => {
            if (*sMorphaTent1).work[MO_TENT_ACTION_STATE as libc::c_int as
                                        usize] as libc::c_int ==
                   MO_TENT_DESPAWN as libc::c_int ||
                   (*sMorphaTent1).work[MO_TENT_ACTION_STATE as libc::c_int as
                                            usize] as libc::c_int ==
                       MO_TENT_WAIT as libc::c_int {
                (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] =
                    MO_CORE_MOVE as libc::c_int as s16;
                (*this).timers[0 as libc::c_int as usize] =
                    70 as libc::c_int as s16
            }
            if (*sMorphaTent1).work[MO_TENT_ACTION_STATE as libc::c_int as
                                        usize] as libc::c_int ==
                   MO_TENT_CUT as libc::c_int {
                (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] =
                    MO_CORE_ATTACK as libc::c_int as s16;
                (*this).work[MO_CORE_POS_IN_TENT as libc::c_int as usize] =
                    0 as libc::c_int as s16;
                (*this).timers[0 as libc::c_int as usize] =
                    0 as libc::c_int as s16
            }
            if (*sMorphaTent1).work[MO_TENT_ACTION_STATE as libc::c_int as
                                        usize] as libc::c_int ==
                   MO_TENT_ATTACK as libc::c_int {
                (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] =
                    MO_CORE_ATTACK as libc::c_int as s16;
                (*this).work[MO_CORE_POS_IN_TENT as libc::c_int as usize] =
                    0 as libc::c_int as s16;
                (*this).timers[0 as libc::c_int as usize] =
                    0 as libc::c_int as s16;
                (*this).actor.speedXZ = 0.0f32
            }
        }
        2 => {
            if (*player).actor.world.pos.y >=
                   (*(*(*globalCtx).colCtx.colHeader).waterBoxes.offset(0 as
                                                                            libc::c_int
                                                                            as
                                                                            isize)).ySurface
                       as libc::c_int as libc::c_float {
                (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] =
                    MO_CORE_MOVE as libc::c_int as s16;
                (*this).actor.speedXZ = 0.0f32
            }
        }
        5 => {
            (*this).actor.flags |=
                ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
            if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
                   0 as libc::c_int {
                (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] =
                    MO_CORE_MOVE as libc::c_int as s16;
                (*this).timers[0 as libc::c_int as usize] =
                    30 as libc::c_int as s16
            }
            if (*this).actor.world.pos.y <
                   (*(*(*globalCtx).colCtx.colHeader).waterBoxes.offset(0 as
                                                                            libc::c_int
                                                                            as
                                                                            isize)).ySurface
                       as libc::c_int as libc::c_float {
                (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] =
                    MO_CORE_MAKE_TENT as libc::c_int as s16;
                (*this).timers[0 as libc::c_int as usize] =
                    50 as libc::c_int as s16;
                (*this).actor.speedXZ = 0.0f32
            }
        }
        -11 | _ => { }
    }
    if (*this).timers[0 as libc::c_int as usize] as libc::c_int ==
           0 as libc::c_int {
        match (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
                  libc::c_int {
            10 => {
                (*this).actor.flags |=
                    ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
                (*this).work[MO_CORE_POS_IN_TENT as libc::c_int as usize] +=
                    1;
                if (*sMorphaTent1).work[MO_TENT_ACTION_STATE as libc::c_int as
                                            usize] as libc::c_int ==
                       MO_TENT_ATTACK as libc::c_int {
                    temp =
                        (Math_SinS(((*this).work[MO_TENT_MOVE_TIMER as
                                                     libc::c_int as usize] as
                                        libc::c_int * 0x300 as libc::c_int) as
                                       s16) * 10.0f32) as s16 as libc::c_int +
                            15 as libc::c_int;
                    if (*this).work[MO_CORE_POS_IN_TENT as libc::c_int as
                                        usize] as libc::c_int >= temp {
                        (*this).work[MO_CORE_POS_IN_TENT as libc::c_int as
                                         usize] = temp as s16
                    }
                }
                if (*sMorphaTent1).work[MO_TENT_ACTION_STATE as libc::c_int as
                                            usize] as libc::c_int !=
                       MO_TENT_ATTACK as libc::c_int &&
                       (*sMorphaTent1).work[MO_TENT_ACTION_STATE as
                                                libc::c_int as usize] as
                           libc::c_int != MO_TENT_CUT as libc::c_int {
                    (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize]
                        = MO_CORE_RETREAT as libc::c_int as s16;
                    (*this).timers[0 as libc::c_int as usize] =
                        0 as libc::c_int as s16
                }
            }
            11 => {
                (*this).work[MO_CORE_POS_IN_TENT as libc::c_int as usize] -=
                    1;
                if (*this).work[MO_CORE_POS_IN_TENT as libc::c_int as usize]
                       as libc::c_int <= 0 as libc::c_int {
                    (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize]
                        = MO_CORE_MAKE_TENT as libc::c_int as s16;
                    (*this).timers[0 as libc::c_int as usize] =
                        100 as libc::c_int as s16;
                    (*this).tentSpeed = 0.0f32;
                    (*this).actor.speedXZ = 0.0f32
                }
                (*this).timers[0 as libc::c_int as usize] =
                    0 as libc::c_int as s16
            }
            21 => {
                (*this).actor.flags &=
                    !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
                (*this).work[MO_CORE_POS_IN_TENT as libc::c_int as usize] +=
                    1;
                temp =
                    (Math_SinS(((*this).work[MO_TENT_MOVE_TIMER as libc::c_int
                                                 as usize] as libc::c_int *
                                    0x500 as libc::c_int) as s16) * 10.0f32)
                        as s16 as libc::c_int + 15 as libc::c_int;
                if (*this).work[MO_CORE_POS_IN_TENT as libc::c_int as usize]
                       as libc::c_int >= temp {
                    (*this).work[MO_CORE_POS_IN_TENT as libc::c_int as usize]
                        = temp as s16
                }
                if (*sMorphaTent1).work[MO_TENT_ACTION_STATE as libc::c_int as
                                            usize] as libc::c_int !=
                       MO_TENT_READY as libc::c_int {
                    (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize]
                        = MO_CORE_RETREAT as libc::c_int as s16;
                    (*this).timers[0 as libc::c_int as usize] =
                        0 as libc::c_int as s16
                }
            }
            _ => { }
        }
    }
    if (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
           libc::c_int >= MO_CORE_ATTACK as libc::c_int {
        if ((*this).work[MO_CORE_POS_IN_TENT as libc::c_int as usize] as
                libc::c_int) < 0 as libc::c_int {
            (*this).work[MO_CORE_POS_IN_TENT as libc::c_int as usize] =
                0 as libc::c_int as s16
        } else if (*this).work[MO_CORE_POS_IN_TENT as libc::c_int as usize] as
                      libc::c_int >= 41 as libc::c_int {
            (*this).work[MO_CORE_POS_IN_TENT as libc::c_int as usize] =
                40 as libc::c_int as s16
        }
        index =
            ((300 as libc::c_int -
                  (*this).work[MO_CORE_POS_IN_TENT as libc::c_int as usize] as
                      libc::c_int * 2 as libc::c_int +
                  (*sMorphaTent1).widthIndex as libc::c_int) %
                 300 as libc::c_int) as s16;
        sp88 =
            (*sMorphaTent1).tentWidth[index as usize] *
                sTentWidth[(*this).work[MO_CORE_POS_IN_TENT as libc::c_int as
                                            usize] as usize];
        j = -(5 as libc::c_int) as s16;
        while (j as libc::c_int) < 6 as libc::c_int {
            index =
                ((*this).work[MO_CORE_POS_IN_TENT as libc::c_int as usize] as
                     libc::c_int + j as libc::c_int - 2 as libc::c_int) as
                    s16;
            if 0 as libc::c_int <= index as libc::c_int &&
                   (index as libc::c_int) < 41 as libc::c_int {
                Math_ApproachF(&mut (*(*sMorphaTent1).tentScale.as_mut_ptr().offset(index
                                                                                        as
                                                                                        isize)).x,
                               coreBulge[(j as libc::c_int + 5 as libc::c_int)
                                             as usize] * 300.0f32 / 100.0f32 +
                                   sp88, 0.75f32, 5.0f32);
            }
            j += 1
        }
        (*this).targetPos.x =
            (*sMorphaTent1).tentPos[(*this).work[MO_CORE_POS_IN_TENT as
                                                     libc::c_int as usize] as
                                        usize].x;
        (*this).targetPos.y =
            (*sMorphaTent1).tentPos[(*this).work[MO_CORE_POS_IN_TENT as
                                                     libc::c_int as usize] as
                                        usize].y;
        (*this).targetPos.z =
            (*sMorphaTent1).tentPos[(*this).work[MO_CORE_POS_IN_TENT as
                                                     libc::c_int as usize] as
                                        usize].z;
        if (*this).work[MO_CORE_POS_IN_TENT as libc::c_int as usize] as
               libc::c_int <= 1 as libc::c_int {
            (*this).targetPos.y -= 20.0f32
        }
        Math_ApproachF(&mut (*this).actor.world.pos.x, (*this).targetPos.x,
                       0.5f32, (*this).actor.speedXZ);
        Math_ApproachF(&mut (*this).actor.world.pos.y, (*this).targetPos.y,
                       0.5f32, (*this).actor.speedXZ);
        Math_ApproachF(&mut (*this).actor.world.pos.z, (*this).targetPos.z,
                       0.5f32, (*this).actor.speedXZ);
        Math_ApproachF(&mut (*this).actor.speedXZ, 30.0f32, 1.0f32, 1.0f32);
    } else {
        match (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
                  libc::c_int {
            0 => {
                sp80 =
                    Math_SinS(((*this).work[MO_TENT_VAR_TIMER as libc::c_int
                                                as usize] as libc::c_int *
                                   0x800 as libc::c_int) as s16) * 100.0f32;
                sp7C =
                    Math_CosS(((*this).work[MO_TENT_VAR_TIMER as libc::c_int
                                                as usize] as libc::c_int *
                                   0x800 as libc::c_int) as s16) * 100.0f32;
                Math_ApproachF(&mut (*this).actor.world.pos.x,
                               (*sMorphaTent1).targetPos.x + sp80, 0.05f32,
                               (*this).actor.speedXZ);
                Math_ApproachF(&mut (*this).actor.world.pos.z,
                               (*sMorphaTent1).targetPos.z + sp7C, 0.05f32,
                               (*this).actor.speedXZ);
                Math_ApproachF(&mut (*this).actor.speedXZ, 10.0f32, 1.0f32,
                               0.5f32);
            }
            5 => {
                (*this).actor.velocity.x =
                    Math_SinS((*this).actor.world.rot.y) *
                        (*this).actor.speedXZ;
                (*this).actor.velocity.z =
                    Math_CosS((*this).actor.world.rot.y) *
                        (*this).actor.speedXZ;
                (*this).actor.world.pos.x += (*this).actor.velocity.x;
                (*this).actor.world.pos.z += (*this).actor.velocity.z
            }
            _ => { }
        }
        if (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
               libc::c_int == MO_CORE_MOVE as libc::c_int ||
               (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
                   libc::c_int == MO_CORE_STUNNED as libc::c_int {
            (*this).actor.world.pos.y += (*this).actor.velocity.y;
            (*this).actor.velocity.y -= 1.0f32;
            Actor_UpdateBgCheckInfo(globalCtx, &mut (*this).actor, 50.0f32,
                                    20.0f32, 100.0f32, 1 as libc::c_int);
            effectVelocity.z = 0.0f32;
            effectVelocity.y = effectVelocity.z;
            effectVelocity.x = effectVelocity.y;
            i = 0 as libc::c_int as s16;
            while (i as libc::c_int) < 1 as libc::c_int {
                effectPos.x =
                    Rand_CenteredFloat(20.0f32) + (*this).actor.world.pos.x;
                effectPos.y =
                    Rand_CenteredFloat(20.0f32) + (*this).actor.world.pos.y;
                effectPos.z =
                    Rand_CenteredFloat(20.0f32) + (*this).actor.world.pos.z;
                BossMo_SpawnDroplet(MO_FX_DROPLET as libc::c_int as s16,
                                    (*globalCtx).specialEffects as
                                        *mut BossMoEffect, &mut effectPos,
                                    &mut effectVelocity,
                                    Rand_ZeroFloat(0.02f32) + 0.05f32);
                i += 1
            }
            if nearLand != 0 {
                if (*this).actor.world.pos.y <=
                       10 as libc::c_int as libc::c_float {
                    (*this).actor.world.pos.y = 10 as libc::c_int as f32_0;
                    (*this).actor.velocity.y = -0.01f32;
                    if (*this).timers[1 as libc::c_int as usize] as
                           libc::c_int != 0 as libc::c_int {
                        if (*this).timers[1 as libc::c_int as usize] as
                               libc::c_int == 1 as libc::c_int {
                            (*this).actor.velocity.y = 6.0f32
                        }
                    } else {
                        (*this).timers[1 as libc::c_int as usize] =
                            2 as libc::c_int as s16;
                        Audio_PlayActorSound2(&mut (*this).actor,
                                              0x38dd as libc::c_int as u16_0);
                        i = 0 as libc::c_int as s16;
                        while (i as libc::c_int) < 10 as libc::c_int {
                            effectVelocity.x = Rand_CenteredFloat(4.0f32);
                            effectVelocity.y =
                                Rand_ZeroFloat(2.0f32) + 3.0f32;
                            effectVelocity.z = Rand_CenteredFloat(4.0f32);
                            effectPos = (*this).actor.world.pos;
                            effectPos.x += effectVelocity.x;
                            effectPos.z += effectVelocity.z;
                            BossMo_SpawnDroplet(MO_FX_DROPLET as libc::c_int
                                                    as s16,
                                                (*globalCtx).specialEffects as
                                                    *mut BossMoEffect,
                                                &mut effectPos,
                                                &mut effectVelocity,
                                                Rand_ZeroFloat(0.08f32) +
                                                    0.13f32);
                            i += 1
                        }
                        effectVelocity.z = 0.0f32;
                        effectVelocity.y = effectVelocity.z;
                        effectVelocity.x = effectVelocity.y;
                        effectPos = (*this).actor.world.pos;
                        effectPos.y = 0.0f32;
                        BossMo_SpawnDroplet(MO_FX_DROPLET as libc::c_int as
                                                s16,
                                            (*globalCtx).specialEffects as
                                                *mut BossMoEffect,
                                            &mut effectPos,
                                            &mut effectVelocity, 0.4f32);
                    }
                }
            } else if (*this).actor.world.pos.y <
                          (*(*(*globalCtx).colCtx.colHeader).waterBoxes.offset(0
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)).ySurface
                              as libc::c_int as libc::c_float {
                (*this).actor.velocity.y =
                    if BossMo_NearLand(&mut (*this).actor.world.pos, 40.0f32)
                           != 0 {
                        15.0f32
                    } else { 6.0f32 };
                if (*this).actor.world.pos.y + 15.0f32 >=
                       (*(*(*globalCtx).colCtx.colHeader).waterBoxes.offset(0
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize)).ySurface
                           as libc::c_int as libc::c_float {
                    Audio_PlayActorSound2(&mut (*this).actor,
                                          0x38f7 as libc::c_int as u16_0);
                }
            }
        } else if (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize]
                      as libc::c_int >= MO_CORE_MOVE as libc::c_int {
            if (*this).actor.world.pos.y <
                   (*(*(*globalCtx).colCtx.colHeader).waterBoxes.offset(0 as
                                                                            libc::c_int
                                                                            as
                                                                            isize)).ySurface
                       as libc::c_int as libc::c_float {
                if (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize]
                       as libc::c_int == MO_CORE_MAKE_TENT as libc::c_int {
                    (*this).targetPos.x = (*sMorphaTent1).targetPos.x;
                    (*this).targetPos.y =
                        (*sMorphaTent1).actor.world.pos.y - 40.0f32;
                    (*this).targetPos.z = (*sMorphaTent1).targetPos.z;
                    Math_ApproachF(&mut (*this).actor.speedXZ, 10.0f32,
                                   1.0f32, 0.5f32);
                } else if (*this).work[MO_TENT_ACTION_STATE as libc::c_int as
                                           usize] as libc::c_int ==
                              MO_CORE_UNDERWATER as libc::c_int {
                    match (*this).work[MO_CORE_WAIT_IN_WATER as libc::c_int as
                                           usize] as libc::c_int {
                        0 => {
                            (*this).targetPos = (*player).actor.world.pos;
                            (*this).targetPos.y += 30.0f32;
                            sp70.x = 0.0f32;
                            sp70.y = 0.0f32;
                            sp70.z = 100.0f32;
                            Matrix_RotateY((*player).actor.world.rot.y as
                                               libc::c_int as libc::c_float /
                                               0x8000 as libc::c_int as f32_0
                                               * 3.14159265358979323846f32,
                                           MTXMODE_NEW as libc::c_int as
                                               u8_0);
                            Matrix_MultVec3f(&mut sp70, &mut sp64);
                            (*this).targetPos.x =
                                (*player).actor.world.pos.x + sp64.x;
                            (*this).targetPos.y =
                                (*player).actor.world.pos.y + 30.0f32;
                            (*this).targetPos.z =
                                (*player).actor.world.pos.z + sp64.z;
                            Math_ApproachF(&mut (*this).actor.speedXZ,
                                           10.0f32, 1.0f32, 1.0f32);
                            if (*this).timers[0 as libc::c_int as usize] as
                                   libc::c_int == 0 as libc::c_int {
                                (*this).work[MO_CORE_WAIT_IN_WATER as
                                                 libc::c_int as usize] =
                                    1 as libc::c_int as s16;
                                (*this).timers[0 as libc::c_int as usize] =
                                    (Rand_ZeroFloat(50.0f32) as s16 as
                                         libc::c_int + 50 as libc::c_int) as
                                        s16
                            }
                        }
                        1 => {
                            Math_ApproachF(&mut (*this).actor.speedXZ, 1.0f32,
                                           1.0f32, 0.5f32);
                            if (*this).timers[0 as libc::c_int as usize] as
                                   libc::c_int == 0 as libc::c_int {
                                (*this).work[MO_CORE_WAIT_IN_WATER as
                                                 libc::c_int as usize] =
                                    0 as libc::c_int as s16;
                                (*this).timers[0 as libc::c_int as usize] =
                                    (Rand_ZeroFloat(20.0f32) as s16 as
                                         libc::c_int + 20 as libc::c_int) as
                                        s16;
                                Audio_PlayActorSound2(&mut (*this).actor,
                                                      0x38de as libc::c_int as
                                                          u16_0);
                            }
                        }
                        _ => { }
                    }
                }
                (*this).targetPos.x +=
                    Math_SinS(((*this).work[MO_TENT_MOVE_TIMER as libc::c_int
                                                as usize] as libc::c_int as
                                   libc::c_float * 3096.0f32) as s16) *
                        30.0f32;
                (*this).targetPos.y +=
                    Math_SinS(((*this).work[MO_TENT_MOVE_TIMER as libc::c_int
                                                as usize] as libc::c_int as
                                   libc::c_float * 2096.0f32) as s16) *
                        30.0f32;
                (*this).targetPos.z +=
                    Math_SinS(((*this).work[MO_TENT_MOVE_TIMER as libc::c_int
                                                as usize] as libc::c_int as
                                   libc::c_float * 2796.0f32) as s16) *
                        30.0f32;
                (*this).tentMaxAngle = 5.0f32;
                (*this).tentSpeed = 4000.0f32;
                spDC = (*this).targetPos.x - (*this).actor.world.pos.x;
                spD8 = (*this).targetPos.y - (*this).actor.world.pos.y;
                spD4 = (*this).targetPos.z - (*this).actor.world.pos.z;
                spCC =
                    (Math_FAtan2F(spDC, spD4) *
                         (0x8000 as libc::c_int as libc::c_float /
                              3.14159265358979323846f32)) as s16 as f32_0;
                spD0 =
                    (Math_FAtan2F(spD8, sqrtf(spDC * spDC + spD4 * spD4)) *
                         (0x8000 as libc::c_int as libc::c_float /
                              3.14159265358979323846f32)) as s16 as f32_0;
                Math_ApproachS(&mut (*this).actor.world.rot.y, spCC as s16,
                               (*this).tentMaxAngle as s16,
                               (*this).tentSpeed as s16);
                Math_ApproachS(&mut (*this).actor.world.rot.x, spD0 as s16,
                               (*this).tentMaxAngle as s16,
                               (*this).tentSpeed as s16);
                func_8002D908(&mut (*this).actor);
            } else {
                (*this).actor.world.pos.y += (*this).actor.velocity.y;
                (*this).actor.velocity.y -= 1.0f32
            }
            func_8002D7EC(&mut (*this).actor);
            temp =
                if (*this).actor.world.pos.y < -200.0f32 {
                    5 as libc::c_int
                } else { 1 as libc::c_int };
            (*this).actor.world.pos.y -= 20.0f32;
            Actor_UpdateBgCheckInfo(globalCtx, &mut (*this).actor, 50.0f32,
                                    20.0f32, 100.0f32, temp);
            (*this).actor.world.pos.y += 20.0f32
        }
    }
    if (*this).actor.world.pos.y <
           (*(*(*globalCtx).colCtx.colHeader).waterBoxes.offset(0 as
                                                                    libc::c_int
                                                                    as
                                                                    isize)).ySurface
               as libc::c_int as libc::c_float &&
           (*(*(*globalCtx).colCtx.colHeader).waterBoxes.offset(0 as
                                                                    libc::c_int
                                                                    as
                                                                    isize)).ySurface
               as libc::c_int as libc::c_float <= (*this).actor.prevPos.y {
        if (*this).actor.velocity.y < -5.0f32 {
            Audio_PlayActorSound2(&mut (*this).actor,
                                  0x38f7 as libc::c_int as u16_0);
        } else {
            Audio_PlayActorSound2(&mut (*this).actor,
                                  0x38df as libc::c_int as u16_0);
        }
        if !((*this).timers[3 as libc::c_int as usize] as libc::c_int !=
                 0 as libc::c_int ||
                 (*sMorphaTent1).fwork[MO_TENT_MAX_STRETCH as libc::c_int as
                                           usize] > 0.2f32 &&
                     fabsf((*this).actor.world.pos.x -
                               (*sMorphaTent1).actor.world.pos.x) < 30.0f32 &&
                     fabsf((*this).actor.world.pos.z -
                               (*sMorphaTent1).actor.world.pos.z) < 30.0f32) {
            (*this).timers[3 as libc::c_int as usize] =
                8 as libc::c_int as s16;
            i = 0 as libc::c_int as s16;
            while (i as libc::c_int) < 10 as libc::c_int {
                sp5C = Rand_ZeroFloat(3.14f32);
                sp60 = Rand_ZeroFloat(0.6f32) + 1.6f32;
                effectVelocity.x =
                    Math_SinS((i as libc::c_int as libc::c_float *
                                   0x10000 as libc::c_int as f32_0 / 10.0f32 +
                                   sp5C) as s16) * sp60;
                effectVelocity.z =
                    Math_CosS((i as libc::c_int as libc::c_float *
                                   0x10000 as libc::c_int as f32_0 / 10.0f32 +
                                   sp5C) as s16) * sp60;
                effectVelocity.y = Rand_ZeroFloat(0.3f32) + 3.0f32;
                effectPos = (*this).actor.world.pos;
                effectPos.x += effectVelocity.x * 3.0f32;
                effectPos.y =
                    (*(*(*globalCtx).colCtx.colHeader).waterBoxes.offset(0 as
                                                                             libc::c_int
                                                                             as
                                                                             isize)).ySurface
                        as f32_0;
                effectPos.z += effectVelocity.z * 3.0f32;
                BossMo_SpawnDroplet(MO_FX_SPLASH as libc::c_int as s16,
                                    (*globalCtx).specialEffects as
                                        *mut BossMoEffect, &mut effectPos,
                                    &mut effectVelocity,
                                    Rand_ZeroFloat(0.075f32) + 0.15f32);
                i += 1
            }
            effectPos = (*this).actor.world.pos;
            effectPos.y =
                (*(*(*globalCtx).colCtx.colHeader).waterBoxes.offset(0 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)).ySurface
                    as f32_0;
            BossMo_SpawnRipple((*globalCtx).specialEffects as
                                   *mut BossMoEffect, &mut effectPos,
                               100.0f32, 800.0f32, 100 as libc::c_int as s16,
                               290 as libc::c_int as s16,
                               MO_FX_SMALL_RIPPLE as libc::c_int as u8_0);
            BossMo_SpawnRipple((*globalCtx).specialEffects as
                                   *mut BossMoEffect, &mut effectPos, 50.0f32,
                               600.0f32, 70 as libc::c_int as s16,
                               290 as libc::c_int as s16,
                               MO_FX_SMALL_RIPPLE as libc::c_int as u8_0);
            BossMo_SpawnRipple((*globalCtx).specialEffects as
                                   *mut BossMoEffect, &mut effectPos,
                               0 as libc::c_int as f32_0, 400.0f32,
                               50 as libc::c_int as s16,
                               290 as libc::c_int as s16,
                               MO_FX_SMALL_RIPPLE as libc::c_int as u8_0);
        }
    }
    if (*this).actor.world.pos.y <
           (*(*(*globalCtx).colCtx.colHeader).waterBoxes.offset(0 as
                                                                    libc::c_int
                                                                    as
                                                                    isize)).ySurface
               as libc::c_int as libc::c_float ||
           (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
               libc::c_int >= MO_CORE_ATTACK as libc::c_int {
        i = 0 as libc::c_int as s16;
        while (i as libc::c_int) < 3 as libc::c_int {
            effectAccel.z = 0.0f32;
            effectAccel.x = effectAccel.z;
            effectVelocity.z = 0.0f32;
            effectVelocity.y = effectVelocity.z;
            effectVelocity.x = effectVelocity.y;
            if (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
                   libc::c_int >= MO_CORE_ATTACK as libc::c_int {
                effectAccel.y = 0.0f32;
                sp58 = 10.0f32
            } else { effectAccel.y = 0.1f32; sp58 = 20.0f32 }
            effectPos.x =
                Rand_CenteredFloat(sp58) + (*this).actor.world.pos.x;
            effectPos.y =
                Rand_CenteredFloat(sp58) + (*this).actor.world.pos.y;
            effectPos.z =
                Rand_CenteredFloat(sp58) + (*this).actor.world.pos.z;
            BossMo_SpawnBubble((*globalCtx).specialEffects as
                                   *mut BossMoEffect, &mut effectPos,
                               &mut effectVelocity, &mut effectAccel,
                               Rand_ZeroFloat(0.05f32) + 0.1f32,
                               0 as *mut Vec3f);
            i += 1
        }
    }
    BossMo_CoreCollisionCheck(this, globalCtx);
}
#[no_mangle]
pub unsafe extern "C" fn BossMo_UpdateCore(mut thisx: *mut Actor,
                                           mut globalCtx:
                                               *mut GlobalContext) {
    let mut pad: s32 = 0;
    let mut this: *mut BossMo = thisx as *mut BossMo;
    let mut i: s16 = 0;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    osSyncPrintf(b"CORE mode = <%d>\n\x00" as *const u8 as
                     *const libc::c_char,
                 (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
                     libc::c_int);
    if sMorphaTent2.is_null() {
        (*(*(*globalCtx).colCtx.colHeader).waterBoxes.offset(0 as libc::c_int
                                                                 as
                                                                 isize)).ySurface
            =
            ((*sMorphaTent1).waterLevelMod +
                 (*this).waterLevel as s16 as libc::c_int as libc::c_float) as
                s16
    } else {
        (*(*(*globalCtx).colCtx.colHeader).waterBoxes.offset(0 as libc::c_int
                                                                 as
                                                                 isize)).ySurface
            =
            ((*sMorphaTent2).waterLevelMod +
                 ((*this).waterLevel as s16 as libc::c_int as libc::c_float +
                      (*sMorphaTent1).waterLevelMod)) as s16
    }
    (*this).actor.flags |=
        ((1 as libc::c_int) << 9 as libc::c_int) as libc::c_uint;
    (*this).actor.focus.pos = (*this).actor.world.pos;
    (*this).work[MO_TENT_VAR_TIMER as libc::c_int as usize] += 1;
    if (*this).work[MO_CORE_DMG_FLASH_TIMER as libc::c_int as usize] as
           libc::c_int != 0 as libc::c_int {
        (*this).work[MO_CORE_DMG_FLASH_TIMER as libc::c_int as usize] -= 1
    }
    if (*this).work[MO_TENT_INVINC_TIMER as libc::c_int as usize] as
           libc::c_int != 0 as libc::c_int {
        (*this).work[MO_TENT_INVINC_TIMER as libc::c_int as usize] -= 1
    }
    (*this).work[MO_TENT_MOVE_TIMER as libc::c_int as usize] += 1;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[s16; 5]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<s16>() as
                                                   libc::c_ulong) as s32 {
        if (*this).timers[i as usize] as libc::c_int != 0 as libc::c_int {
            (*this).timers[i as usize] -= 1
        }
        i += 1
    }
    BossMo_Core(this, globalCtx);
    Collider_UpdateCylinder(&mut (*this).actor, &mut (*this).coreCollider);
    CollisionCheck_SetAC(globalCtx, &mut (*globalCtx).colChkCtx,
                         &mut (*this).coreCollider.base);
    if (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
           libc::c_int != MO_CORE_STUNNED as libc::c_int ||
           (*this).actor.world.pos.y <
               (*(*(*globalCtx).colCtx.colHeader).waterBoxes.offset(0 as
                                                                        libc::c_int
                                                                        as
                                                                        isize)).ySurface
                   as libc::c_int as libc::c_float {
        CollisionCheck_SetAT(globalCtx, &mut (*globalCtx).colChkCtx,
                             &mut (*this).coreCollider.base);
    } else {
        CollisionCheck_SetOC(globalCtx, &mut (*globalCtx).colChkCtx,
                             &mut (*this).coreCollider.base);
    }
    BossMo_UpdateEffects(this, globalCtx);
    if !(*player).actor.parent.is_null() {
        (*this).actor.flags &=
            !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
    }
    BossMo_Unknown();
}
#[no_mangle]
pub unsafe extern "C" fn BossMo_UpdateTent(mut thisx: *mut Actor,
                                           mut globalCtx:
                                               *mut GlobalContext) {
    let mut i: s16 = 0;
    let mut index: s16 = 0;
    let mut pad: s32 = 0;
    let mut this: *mut BossMo = thisx as *mut BossMo;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut phi_f0: f32_0 = 0.;
    if this == sMorphaTent2 &&
           (*this).tent2KillTimer as libc::c_int != 0 as libc::c_int {
        (*this).tent2KillTimer = (*this).tent2KillTimer.wrapping_add(1);
        (*this).actor.draw = None;
        if (*this).tent2KillTimer as libc::c_int > 20 as libc::c_int {
            Actor_Kill(&mut (*this).actor);
            Audio_StopSfxByPos(&mut (*this).tentTipPos);
            sMorphaTent2 = 0 as *mut BossMo
        }
        return
    }
    SkinMatrix_Vec3fMtxFMultXYZW(&mut (*globalCtx).viewProjectionMtxF,
                                 &mut *(*this).tentPos.as_mut_ptr().offset(40
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                 &mut (*this).tentTipPos,
                                 &mut (*this).actor.projectedW);
    osSyncPrintf(b"MO : Move mode = <%d>\n\x00" as *const u8 as
                     *const libc::c_char,
                 (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
                     libc::c_int);
    Math_ApproachS(&mut (*player).actor.shape.rot.x, 0 as libc::c_int as s16,
                   5 as libc::c_int as s16, 0x3e8 as libc::c_int as s16);
    Math_ApproachS(&mut (*player).actor.shape.rot.z, 0 as libc::c_int as s16,
                   5 as libc::c_int as s16, 0x3e8 as libc::c_int as s16);
    (*this).work[MO_TENT_VAR_TIMER as libc::c_int as usize] += 1;
    (*this).sfxTimer += 1;
    (*this).work[MO_TENT_MOVE_TIMER as libc::c_int as usize] += 1;
    (*this).widthIndex += 1;
    if (*this).widthIndex as libc::c_int >= 300 as libc::c_int {
        (*this).widthIndex = 0 as libc::c_int as s16
    }
    (*this).pulsePhase =
        ((*this).pulsePhase as libc::c_int - 3000 as libc::c_int) as s16;
    index = (*this).widthIndex;
    (*this).tentWidth[index as usize] =
        Math_SinS((*this).pulsePhase) * (*this).tentPulse +
            (1.0f32 + (*this).tentPulse);
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < 41 as libc::c_int {
        if (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
               libc::c_int >= MO_TENT_DEATH_START as libc::c_int {
            if (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
                   libc::c_int >= MO_TENT_DEATH_1 as libc::c_int {
                if (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize]
                       as libc::c_int == MO_TENT_DEATH_5 as libc::c_int {
                    phi_f0 =
                        if (*this).timers[0 as libc::c_int as usize] as
                               libc::c_int != 0 as libc::c_int {
                            sFlatWidth[i as usize]
                        } else { sDropletWidth[i as usize] };
                    Math_ApproachF(&mut (*(*this).tentScale.as_mut_ptr().offset(i
                                                                                    as
                                                                                    isize)).x,
                                   phi_f0, 0.5f32, 100.0f32);
                } else {
                    index =
                        (((*this).widthIndex as libc::c_int +
                              i as libc::c_int * 2 as libc::c_int +
                              220 as libc::c_int) % 300 as libc::c_int) as
                            s16;
                    phi_f0 =
                        (*this).tentWidth[index as usize] +
                            sTentWidth[i as usize] * sTentWidth[i as usize];
                    Math_ApproachF(&mut (*(*this).tentScale.as_mut_ptr().offset(i
                                                                                    as
                                                                                    isize)).x,
                                   phi_f0, 0.5f32, 0.3f32);
                }
            } else {
                index =
                    (((*this).widthIndex as libc::c_int -
                          i as libc::c_int * 2 as libc::c_int +
                          300 as libc::c_int) % 300 as libc::c_int) as s16;
                phi_f0 =
                    (*this).tentWidth[index as usize] *
                        sTentWidth[i as usize];
                (*this).tentScale[i as usize].x = phi_f0
            }
        } else {
            index =
                (((*this).widthIndex as libc::c_int -
                      i as libc::c_int * 2 as libc::c_int +
                      300 as libc::c_int) % 300 as libc::c_int) as s16;
            phi_f0 =
                (*this).tentWidth[index as usize] * sTentWidth[i as usize];
            Math_ApproachF(&mut (*(*this).tentScale.as_mut_ptr().offset(i as
                                                                            isize)).x,
                           phi_f0, 0.5f32, 0.3f32);
        }
        phi_f0 =
            Math_SinS(((*this).work[MO_TENT_VAR_TIMER as libc::c_int as usize]
                           as libc::c_int as libc::c_float * 12000.0f32 +
                           i as libc::c_int as libc::c_float * 20000.0f32) as
                          s16);
        (*this).tentRipple[i as usize].x =
            1.0f32 * phi_f0 * (*this).tentRippleSize;
        (*this).tentScale[i as usize].z = (*this).tentScale[i as usize].x;
        (*this).tentScale[i as usize].y = (*this).tentScale[i as usize].z;
        (*this).tentRipple[i as usize].z = (*this).tentRipple[i as usize].x;
        (*this).tentRipple[i as usize].y = (*this).tentRipple[i as usize].z;
        i += 1
    }
    Math_ApproachF(&mut (*this).tentRippleSize, 0.0f32, 0.1f32, 0.005f32);
    Math_ApproachF(&mut (*this).tentPulse, 0.2f32, 0.5f32, 0.01f32);
    (*this).actionFunc.expect("non-null function pointer")(this, globalCtx);
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[s16; 5]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<s16>() as
                                                   libc::c_ulong) as s32 {
        if (*this).timers[i as usize] as libc::c_int != 0 as libc::c_int {
            (*this).timers[i as usize] -= 1
        }
        i += 1
    }
    Math_ApproachS(&mut (*this).actor.world.rot.y,
                   (*this).actor.yawTowardsPlayer, 0xa as libc::c_int as s16,
                   0xc8 as libc::c_int as s16);
    Actor_MoveForward(&mut (*this).actor);
    Math_ApproachF(&mut (*this).actor.speedXZ, 0.0f64 as f32_0, 1.0f32,
                   0.02f32);
    if BossMo_NearLand(&mut (*this).actor.world.pos,
                       40 as libc::c_int as f32_0) != 0 {
        (*this).actor.world.pos = (*this).actor.prevPos
    }
    if (*this).work[MO_TENT_VAR_TIMER as libc::c_int as usize] as libc::c_int
           % 8 as libc::c_int == 0 as libc::c_int {
        let mut rippleScale: f32_0 = 0.;
        let mut pos: Vec3f = (*this).actor.world.pos;
        if ((*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
                libc::c_int) < MO_TENT_DEATH_START as libc::c_int {
            rippleScale = 400.0f32
        } else {
            rippleScale = 0.0f64 as f32_0;
            if (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
                   libc::c_int >= MO_TENT_DEATH_1 as libc::c_int {
                pos = (*this).tentPos[38 as libc::c_int as usize]
            }
        }
        BossMo_SpawnRipple((*globalCtx).specialEffects as *mut BossMoEffect,
                           &mut pos, rippleScale, rippleScale * 3.0f32,
                           ((*this).baseAlpha * 0.6666f32) as s16,
                           300 as libc::c_int as s16,
                           MO_FX_BIG_RIPPLE as libc::c_int as u8_0);
    }
    if (*this).baseBubblesTimer as libc::c_int != 0 as libc::c_int {
        let mut sp88: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
        let mut sp7C: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
        let mut bubblePos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
        let mut zeroVec: Vec3f =
            { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
        let mut pad_0: s32 = 0;
        (*this).baseBubblesTimer -= 1;
        sp88.x = 0.0f64 as f32_0;
        sp88.y = 0.0f64 as f32_0;
        sp88.z = 100.0f32;
        Matrix_RotateY(Rand_ZeroFloat(2.0f32 * 3.14159265358979323846f32),
                       MTXMODE_NEW as libc::c_int as u8_0);
        Matrix_MultVec3f(&mut sp88, &mut sp7C);
        if (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
               libc::c_int >= MO_TENT_DEATH_1 as libc::c_int &&
               (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
                   libc::c_int != MO_TENT_DEATH_3 as libc::c_int {
            i = 38 as libc::c_int as s16
        } else {
            i = 0 as libc::c_int as s16;
            if ((*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
                    libc::c_int) < MO_TENT_CUT as libc::c_int {
                func_80078914(&mut (*this).tentTipPos,
                              (0x38fa as libc::c_int - 0x800 as libc::c_int)
                                  as u16_0);
            }
        }
        bubblePos.x = (*this).tentPos[i as usize].x + sp7C.x;
        bubblePos.y =
            (*(*(*globalCtx).colCtx.colHeader).waterBoxes.offset(0 as
                                                                     libc::c_int
                                                                     as
                                                                     isize)).ySurface
                as libc::c_int as libc::c_float - 40.0f32 +
                Rand_ZeroFloat(20.0f32);
        bubblePos.z = (*this).tentPos[i as usize].z + sp7C.z;
        BossMo_SpawnBubble((*globalCtx).specialEffects as *mut BossMoEffect,
                           &mut bubblePos, &mut zeroVec, &mut zeroVec,
                           Rand_ZeroFloat(0.05f32) + 0.2f32,
                           &mut *(*this).tentPos.as_mut_ptr().offset(i as
                                                                         isize));
    }
    if (*this).work[MO_CORE_DMG_FLASH_TIMER as libc::c_int as usize] as
           libc::c_int != 0 as libc::c_int {
        (*this).work[MO_CORE_DMG_FLASH_TIMER as libc::c_int as usize] -= 1
    }
    if (*this).work[MO_TENT_INVINC_TIMER as libc::c_int as usize] as
           libc::c_int != 0 as libc::c_int {
        (*this).work[MO_TENT_INVINC_TIMER as libc::c_int as usize] -= 1
    }
    if (*this).linkHitTimer as libc::c_int != 0 as libc::c_int {
        (*this).linkHitTimer = (*this).linkHitTimer.wrapping_sub(1)
    }
    if (*this).drawActor != 0 {
        BossMo_TentCollisionCheck(this, globalCtx);
        if (*this).work[MO_TENT_INVINC_TIMER as libc::c_int as usize] as
               libc::c_int == 0 as libc::c_int &&
               (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
                   libc::c_int != MO_TENT_GRAB as libc::c_int &&
               (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
                   libc::c_int != MO_TENT_SHAKE as libc::c_int {
            let mut otherTent: *mut BossMo = (*this).otherTent as *mut BossMo;
            if !(!otherTent.is_null() &&
                     ((*otherTent).work[MO_TENT_ACTION_STATE as libc::c_int as
                                            usize] as libc::c_int ==
                          MO_TENT_GRAB as libc::c_int ||
                          (*otherTent).work[MO_TENT_ACTION_STATE as
                                                libc::c_int as usize] as
                              libc::c_int == MO_TENT_SHAKE as libc::c_int)) &&
                   (*this).cutIndex as libc::c_int == 0 as libc::c_int {
                CollisionCheck_SetOC(globalCtx, &mut (*globalCtx).colChkCtx,
                                     &mut (*this).tentCollider.base);
                CollisionCheck_SetAT(globalCtx, &mut (*globalCtx).colChkCtx,
                                     &mut (*this).tentCollider.base);
            }
        }
        if (*this).cutIndex as libc::c_int == 0 as libc::c_int {
            CollisionCheck_SetAC(globalCtx, &mut (*globalCtx).colChkCtx,
                                 &mut (*this).tentCollider.base);
        }
    }
    (*this).work[MO_TENT_BASE_TEX1_X as libc::c_int as usize] += 1;
    (*this).work[MO_TENT_BASE_TEX1_Y as libc::c_int as usize] += 1;
    (*this).work[MO_TENT_BASE_TEX2_X as libc::c_int as usize] =
        ((*this).work[MO_TENT_BASE_TEX2_X as libc::c_int as usize] as
             libc::c_int - 3 as libc::c_int) as s16;
    (*this).work[MO_TENT_BASE_TEX2_Y as libc::c_int as usize] += 1;
    Math_ApproachZeroF(&mut (*this).waterLevelMod, 0.1f32, 0.2f32);
}
#[no_mangle]
pub unsafe extern "C" fn BossMo_UpdateTentColliders(mut this: *mut BossMo,
                                                    mut item: s32,
                                                    mut tentCollider:
                                                        *mut ColliderJntSph,
                                                    mut center: *mut Vec3f) {
    (*(*tentCollider).elements.offset(item as isize)).dim.worldSphere.center.x
        = (*center).x as s16;
    (*(*tentCollider).elements.offset(item as isize)).dim.worldSphere.center.y
        = (*center).y as s16;
    (*(*tentCollider).elements.offset(item as isize)).dim.worldSphere.center.z
        = (*center).z as s16;
    if (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
           libc::c_int <= MO_TENT_SHAKE as libc::c_int {
        (*(*tentCollider).elements.offset(item as
                                              isize)).dim.worldSphere.radius =
            ((*(*tentCollider).elements.offset(item as
                                                   isize)).dim.modelSphere.radius
                 as libc::c_int as libc::c_float *
                 (*(*tentCollider).elements.offset(item as isize)).dim.scale)
                as s16
    } else {
        (*(*tentCollider).elements.offset(item as
                                              isize)).dim.worldSphere.radius =
            0 as libc::c_int as s16
    };
}
static mut sTentDLists: [*mut Gfx; 41] =
    unsafe {
        [gMorphaTentaclePart0DL.as_ptr() as *mut _,
         gMorphaTentaclePart1DL.as_ptr() as *mut _,
         gMorphaTentaclePart2DL.as_ptr() as *mut _,
         gMorphaTentaclePart3DL.as_ptr() as *mut _,
         gMorphaTentaclePart4DL.as_ptr() as *mut _,
         gMorphaTentaclePart5DL.as_ptr() as *mut _,
         gMorphaTentaclePart6DL.as_ptr() as *mut _,
         gMorphaTentaclePart7DL.as_ptr() as *mut _,
         gMorphaTentaclePart8DL.as_ptr() as *mut _,
         gMorphaTentaclePart9DL.as_ptr() as *mut _,
         gMorphaTentaclePart10DL.as_ptr() as *mut _,
         gMorphaTentaclePart11DL.as_ptr() as *mut _,
         gMorphaTentaclePart12DL.as_ptr() as *mut _,
         gMorphaTentaclePart13DL.as_ptr() as *mut _,
         gMorphaTentaclePart14DL.as_ptr() as *mut _,
         gMorphaTentaclePart15DL.as_ptr() as *mut _,
         gMorphaTentaclePart16DL.as_ptr() as *mut _,
         gMorphaTentaclePart17DL.as_ptr() as *mut _,
         gMorphaTentaclePart18DL.as_ptr() as *mut _,
         gMorphaTentaclePart19DL.as_ptr() as *mut _,
         gMorphaTentaclePart20DL.as_ptr() as *mut _,
         gMorphaTentaclePart21DL.as_ptr() as *mut _,
         gMorphaTentaclePart22DL.as_ptr() as *mut _,
         gMorphaTentaclePart23DL.as_ptr() as *mut _,
         gMorphaTentaclePart24DL.as_ptr() as *mut _,
         gMorphaTentaclePart25DL.as_ptr() as *mut _,
         gMorphaTentaclePart26DL.as_ptr() as *mut _,
         gMorphaTentaclePart27DL.as_ptr() as *mut _,
         gMorphaTentaclePart28DL.as_ptr() as *mut _,
         gMorphaTentaclePart29DL.as_ptr() as *mut _,
         gMorphaTentaclePart30DL.as_ptr() as *mut _,
         gMorphaTentaclePart31DL.as_ptr() as *mut _,
         gMorphaTentaclePart32DL.as_ptr() as *mut _,
         gMorphaTentaclePart33DL.as_ptr() as *mut _,
         gMorphaTentaclePart34DL.as_ptr() as *mut _,
         gMorphaTentaclePart35DL.as_ptr() as *mut _,
         gMorphaTentaclePart36DL.as_ptr() as *mut _,
         gMorphaTentaclePart37DL.as_ptr() as *mut _,
         gMorphaTentaclePart38DL.as_ptr() as *mut _,
         gMorphaTentaclePart39DL.as_ptr() as *mut _,
         gMorphaTentaclePart40DL.as_ptr() as *mut _]
    };
#[no_mangle]
pub unsafe extern "C" fn BossMo_DrawTentacle(mut this: *mut BossMo,
                                             mut globalCtx:
                                                 *mut GlobalContext) {
    static mut zeroVec: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
    let mut i: s16 = 0;
    let mut notCut: s16 = 0;
    let mut index: s16 = 0;
    let mut matrix: *mut Mtx =
        Graph_Alloc((*globalCtx).state.gfxCtx,
                    (41 as libc::c_int as
                         libc::c_uint).wrapping_mul(::std::mem::size_of::<Mtx>()
                                                        as libc::c_ulong) as
                        size_t) as *mut Mtx;
    let mut phi_f20: f32_0 = 0.;
    let mut phi_f22: f32_0 = 0.;
    let mut sp110: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_boss_mo.c\x00" as *const u8 as *const libc::c_char,
                    6366 as libc::c_int);
    sp110.x = (*globalCtx).envCtx.dirLight1.params.dir.x as f32_0;
    sp110.y = (*globalCtx).envCtx.dirLight1.params.dir.y as f32_0;
    sp110.z = (*globalCtx).envCtx.dirLight1.params.dir.z as f32_0;
    Matrix_Push();
    let fresh3 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g: *mut Gfx = fresh3;
    (*_g).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh4 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_0: *mut Gfx = fresh4;
    (*_g_0).words.w0 =
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
    (*_g_0).words.w1 = matrix as libc::c_uint;
    Matrix_Translate((*this).actor.world.pos.x, (*this).actor.world.pos.y,
                     (*this).actor.world.pos.z,
                     MTXMODE_NEW as libc::c_int as u8_0);
    Matrix_RotateY((*this).actor.shape.rot.y as libc::c_int as libc::c_float /
                       0x8000 as libc::c_int as f32_0 *
                       3.14159265358979323846f32,
                   MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_RotateX((*this).actor.shape.rot.x as libc::c_int as libc::c_float /
                       0x8000 as libc::c_int as f32_0 *
                       3.14159265358979323846f32,
                   MTXMODE_APPLY as libc::c_int as u8_0);
    BossMo_InitRand(1 as libc::c_int, 29100 as libc::c_int,
                    9786 as libc::c_int);
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < 41 as libc::c_int {
        let mut pad: s32 = 0;
        let mut pad2: s32 = 0;
        if (i as libc::c_int) < 2 as libc::c_int {
            Matrix_Push();
            Matrix_Scale(0.0f32, 0.0f32, 0.0f32,
                         MTXMODE_APPLY as libc::c_int as u8_0);
            notCut = 1 as libc::c_int as s16
        } else {
            if i as libc::c_int >= 3 as libc::c_int {
                Matrix_Translate(0.0f32,
                                 (*this).tentStretch[(i as libc::c_int -
                                                          2 as libc::c_int) as
                                                         usize].y, 0.0f32,
                                 MTXMODE_APPLY as libc::c_int as u8_0);
                Matrix_RotateX((*this).tentRot[(i as libc::c_int -
                                                    2 as libc::c_int) as
                                                   usize].x as libc::c_int as
                                   libc::c_float /
                                   0x8000 as libc::c_int as f32_0 *
                                   3.14159265358979323846f32,
                               MTXMODE_APPLY as libc::c_int as u8_0);
                Matrix_RotateZ((*this).tentRot[(i as libc::c_int -
                                                    2 as libc::c_int) as
                                                   usize].z as libc::c_int as
                                   libc::c_float /
                                   0x8000 as libc::c_int as f32_0 *
                                   3.14159265358979323846f32,
                               MTXMODE_APPLY as libc::c_int as u8_0);
            }
            Matrix_Push();
            Matrix_Scale(((*this).tentScale[(i as libc::c_int -
                                                 2 as libc::c_int) as usize].x
                              +
                              (*this).tentRipple[(i as libc::c_int -
                                                      2 as libc::c_int) as
                                                     usize].x) *
                             (*this).actor.scale.x,
                         ((*this).tentScale[(i as libc::c_int -
                                                 2 as libc::c_int) as usize].y
                              +
                              (*this).tentRipple[(i as libc::c_int -
                                                      2 as libc::c_int) as
                                                     usize].y) *
                             (*this).actor.scale.y,
                         ((*this).tentScale[(i as libc::c_int -
                                                 2 as libc::c_int) as usize].z
                              +
                              (*this).tentRipple[(i as libc::c_int -
                                                      2 as libc::c_int) as
                                                     usize].z) *
                             (*this).actor.scale.z,
                         MTXMODE_APPLY as libc::c_int as u8_0);
            notCut = 1 as libc::c_int as s16;
            if i as libc::c_int >= (*this).cutIndex as libc::c_int &&
                   (*this).meltIndex as libc::c_int >= i as libc::c_int {
                Matrix_Scale((*this).cutScale, (*this).cutScale,
                             (*this).cutScale,
                             MTXMODE_APPLY as libc::c_int as u8_0);
                notCut = 0 as libc::c_int as s16
            }
        }
        index =
            (((*this).widthIndex as libc::c_int -
                  i as libc::c_int * 2 as libc::c_int + 300 as libc::c_int) %
                 300 as libc::c_int) as s16;
        if ((*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
                libc::c_int) < MO_TENT_DEATH_START as libc::c_int {
            Matrix_RotateY(((*this).tentWidth[index as usize] - 1.0f32 -
                                (*this).tentPulse) *
                               1000 as libc::c_int as libc::c_float /
                               1000.0f32 *
                               (*this).fwork[MO_TENT_MAX_STRETCH as
                                                 libc::c_int as usize],
                           MTXMODE_APPLY as libc::c_int as u8_0);
        }
        Matrix_RotateX(3.14159265358979323846f32 / 2.0f32,
                       MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_ToMtx(matrix,
                     b"../z_boss_mo.c\x00" as *const u8 as *const libc::c_char
                         as *mut libc::c_char, 6452 as libc::c_int);
        let fresh5 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_1: *mut Gfx = fresh5;
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
        (*_g_1).words.w1 = matrix as libc::c_uint;
        if i as libc::c_int == 0 as libc::c_int {
            func_8002EB44(&mut *(*this).tentPos.as_mut_ptr().offset(i as
                                                                        isize),
                          &mut (*globalCtx).view.eye, &mut sp110,
                          (*globalCtx).state.gfxCtx);
        }
        if i as libc::c_int == 0 as libc::c_int {
            let fresh6 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_2: *mut Gfx = fresh6;
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
                gMorphaTentacleBaseDL.as_mut_ptr() as libc::c_uint
        } else {
            let fresh7 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_3: *mut Gfx = fresh7;
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
            (*_g_3).words.w1 = sTentDLists[i as usize] as libc::c_uint
        }
        Matrix_Pop();
        if i as libc::c_int >= 2 as libc::c_int && notCut as libc::c_int != 0
               &&
               (i as libc::c_int) <
                   (*this).noBubbles as libc::c_int + 38 as libc::c_int {
            if (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize] as
                   libc::c_int == MO_TENT_DEATH_1 as libc::c_int ||
                   (*this).work[MO_TENT_ACTION_STATE as libc::c_int as usize]
                       as libc::c_int == MO_TENT_DEATH_2 as libc::c_int {
                phi_f20 =
                    ((*this).work[MO_TENT_MOVE_TIMER as libc::c_int as usize]
                         as libc::c_int & 3 as libc::c_int) as f32_0;
                phi_f20 *= -15.0f32;
                phi_f22 =
                    (0.18f32 + BossMo_RandZeroOne() * 0.1f32) *
                        (*this).actor.scale.x * 100.0f32
            } else {
                phi_f20 = 0.0f32;
                phi_f22 =
                    (BossMo_RandZeroOne() * 0.08f32 + 0.08f32) *
                        (*this).actor.scale.x * 100.0f32
            }
            Matrix_Push();
            Matrix_Translate((BossMo_RandZeroOne() - 0.5f32) * 10.0f32 *
                                 (*this).tentScale[(i as libc::c_int -
                                                        2 as libc::c_int) as
                                                       usize].x,
                             (BossMo_RandZeroOne() - 0.5f32) * 3.0f32 +
                                 phi_f20,
                             (BossMo_RandZeroOne() - 0.5f32) * 10.0f32 *
                                 (*this).tentScale[(i as libc::c_int -
                                                        2 as libc::c_int) as
                                                       usize].z,
                             MTXMODE_APPLY as libc::c_int as u8_0);
            func_800D1FD4(&mut (*globalCtx).billboardMtxF);
            Matrix_Scale(phi_f22, phi_f22, 1.0f32,
                         MTXMODE_APPLY as libc::c_int as u8_0);
            let fresh8 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_4: *mut Gfx = fresh8;
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
                              b"../z_boss_mo.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              6511 as libc::c_int) as libc::c_uint;
            let fresh9 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_5: *mut Gfx = fresh9;
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
            (*_g_5).words.w1 = gMorphaBubbleDL.as_mut_ptr() as libc::c_uint;
            Matrix_Pop();
        }
        Matrix_MultVec3f(&mut zeroVec,
                         &mut *(*this).tentPos.as_mut_ptr().offset(i as
                                                                       isize));
        if i as libc::c_int == 36 as libc::c_int {
            Matrix_MultVec3f(&mut zeroVec, &mut (*this).actor.focus.pos);
        }
        if i as libc::c_int == 24 as libc::c_int {
            let mut sp98: MtxF = MtxF{mf: [[0.; 4]; 4],};
            let mut sp8C: Vec3f =
                {
                    let mut init =
                        Vec3f{x: -16.0f32, y: 13.0f32, z: 30.0f32,};
                    init
                };
            let mut sp84: Vec3s = Vec3s{x: 0, y: 0, z: 0,};
            Matrix_Push();
            if (*this).linkToLeft != 0 { sp8C.x *= -1.0f32 }
            Matrix_MultVec3f(&mut sp8C, &mut (*this).grabPosRot.pos);
            Matrix_RotateX(-(35 as libc::c_int) as libc::c_float *
                               3.14159265358979323846f32 /
                               64 as libc::c_int as libc::c_float,
                           MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_Get(&mut sp98);
            Matrix_MtxFToYXZRotS(&mut sp98, &mut sp84, 0 as libc::c_int);
            (*this).grabPosRot.rot.x = sp84.x;
            (*this).grabPosRot.rot.y = sp84.y;
            (*this).grabPosRot.rot.z = sp84.z;
            Matrix_Pop();
        }
        if (i as libc::c_int) < 38 as libc::c_int &&
               i as libc::c_int & 1 as libc::c_int == 1 as libc::c_int {
            BossMo_UpdateTentColliders(this,
                                       i as libc::c_int / 2 as libc::c_int,
                                       &mut (*this).tentCollider,
                                       &mut *(*this).tentPos.as_mut_ptr().offset(i
                                                                                     as
                                                                                     isize));
        }
        i += 1;
        matrix = matrix.offset(1)
    }
    Matrix_Pop();
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_boss_mo.c\x00" as *const u8 as
                         *const libc::c_char, 6571 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn BossMo_DrawWater(mut this: *mut BossMo,
                                          mut globalCtx: *mut GlobalContext) {
    let mut pad: s32 = 0;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_boss_mo.c\x00" as *const u8 as *const libc::c_char,
                    6582 as libc::c_int);
    Matrix_Push();
    func_80093D84((*globalCtx).state.gfxCtx);
    Matrix_Translate(0.0f32,
                     (*(*(*globalCtx).colCtx.colHeader).waterBoxes.offset(0 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).ySurface
                         as f32_0, 0.0f32,
                     MTXMODE_NEW as libc::c_int as u8_0);
    let fresh10 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g: *mut Gfx = fresh10;
    (*_g).words.w0 =
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
    (*_g).words.w1 =
        Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                         (*this).waterTex1x as s16 as u32_0,
                         (*this).waterTex1y as s16 as u32_0,
                         32 as libc::c_int, 32 as libc::c_int,
                         1 as libc::c_int, (*this).waterTex2x as s16 as u32_0,
                         (*this).waterTex2y as s16 as u32_0,
                         32 as libc::c_int, 32 as libc::c_int) as
            libc::c_uint;
    let fresh11 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_0: *mut Gfx = fresh11;
    (*_g_0).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_0).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh12 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_1: *mut Gfx = fresh12;
    (*_g_1).words.w0 =
        (0xfa as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0xff as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (0xff as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_1).words.w1 =
        (200 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            ((*sMorphaTent1).waterTexAlpha as s8 as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh13 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_2: *mut Gfx = fresh13;
    (*_g_2).words.w0 =
        (0xfb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_2).words.w1 =
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
            (80 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    Matrix_Scale(0.5f32, 1.0f32, 0.5f32,
                 MTXMODE_APPLY as libc::c_int as u8_0);
    let fresh14 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_3: *mut Gfx = fresh14;
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
                      b"../z_boss_mo.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      6675 as libc::c_int) as libc::c_uint;
    let fresh15 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_4: *mut Gfx = fresh15;
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
    (*_g_4).words.w1 = gMorphaWaterDL.as_mut_ptr() as libc::c_uint;
    Matrix_Pop();
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_boss_mo.c\x00" as *const u8 as
                         *const libc::c_char, 6680 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn BossMo_DrawCore(mut thisx: *mut Actor,
                                         mut globalCtx: *mut GlobalContext) {
    let mut pad: s32 = 0;
    let mut this: *mut BossMo = thisx as *mut BossMo;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_boss_mo.c\x00" as *const u8 as *const libc::c_char,
                    6688 as libc::c_int);
    if (*this).actor.world.pos.y >
           (*(*(*globalCtx).colCtx.colHeader).waterBoxes.offset(0 as
                                                                    libc::c_int
                                                                    as
                                                                    isize)).ySurface
               as libc::c_int as libc::c_float {
        BossMo_DrawWater(this, globalCtx);
    }
    if (*this).drawActor != 0 {
        func_80093D84((*globalCtx).state.gfxCtx);
        let fresh16 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g: *mut Gfx = fresh16;
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
            Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                             ((*sMorphaTent1).work[MO_TENT_VAR_TIMER as
                                                       libc::c_int as usize]
                                  as libc::c_int * 3 as libc::c_int) as u32_0,
                             ((*sMorphaTent1).work[MO_TENT_VAR_TIMER as
                                                       libc::c_int as usize]
                                  as libc::c_int * 3 as libc::c_int) as u32_0,
                             32 as libc::c_int, 32 as libc::c_int,
                             1 as libc::c_int,
                             ((*sMorphaTent1).work[MO_TENT_VAR_TIMER as
                                                       libc::c_int as usize]
                                  as libc::c_int * -(3 as libc::c_int)) as
                                 u32_0,
                             ((*sMorphaTent1).work[MO_TENT_VAR_TIMER as
                                                       libc::c_int as usize]
                                  as libc::c_int * -(3 as libc::c_int)) as
                                 u32_0, 32 as libc::c_int, 32 as libc::c_int)
                as libc::c_uint;
        let fresh17 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_0: *mut Gfx = fresh17;
        (*_g_0).words.w0 =
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
        (*_g_0).words.w1 =
            Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                             ((*sMorphaTent1).work[MO_TENT_VAR_TIMER as
                                                       libc::c_int as usize]
                                  as libc::c_int * 5 as libc::c_int) as u32_0,
                             0 as libc::c_int as u32_0, 32 as libc::c_int,
                             32 as libc::c_int, 1 as libc::c_int,
                             0 as libc::c_int as u32_0,
                             ((*sMorphaTent1).work[MO_TENT_VAR_TIMER as
                                                       libc::c_int as usize]
                                  as libc::c_int * -(10 as libc::c_int)) as
                                 u32_0, 32 as libc::c_int, 32 as libc::c_int)
                as libc::c_uint;
        Matrix_RotateX((*this).work[MO_TENT_MOVE_TIMER as libc::c_int as
                                        usize] as libc::c_int as libc::c_float
                           * 0.5f32, MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_RotateZ((*this).work[MO_TENT_MOVE_TIMER as libc::c_int as
                                        usize] as libc::c_int as libc::c_float
                           * 0.8f32, MTXMODE_APPLY as libc::c_int as u8_0);
        let fresh18 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_1: *mut Gfx = fresh18;
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
                          b"../z_boss_mo.c\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                          6735 as libc::c_int) as libc::c_uint;
        let fresh19 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_2: *mut Gfx = fresh19;
        (*_g_2).words.w0 =
            (0xfa as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0x80 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (0x80 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_2).words.w1 =
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
                ((*this).baseAlpha as s8 as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        func_8002ED80(&mut (*this).actor, globalCtx, 0 as libc::c_int);
        let fresh20 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_3: *mut Gfx = fresh20;
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
        (*_g_3).words.w1 =
            gSegments[((gMorphaCoreMembraneDL.as_mut_ptr() as u32_0) <<
                           4 as libc::c_int >> 28 as libc::c_int) as
                          usize].wrapping_add(gMorphaCoreMembraneDL.as_mut_ptr()
                                                  as u32_0 &
                                                  0xffffff as libc::c_int as
                                                      libc::c_uint).wrapping_add(0x80000000
                                                                                     as
                                                                                     libc::c_uint)
                as *mut libc::c_void as libc::c_uint;
        let fresh21 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_4: *mut Gfx = fresh21;
        (*_g_4).words.w0 =
            (0xe7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_4).words.w1 = 0 as libc::c_int as libc::c_uint;
        let fresh22 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_5: *mut Gfx = fresh22;
        (*_g_5).words.w0 =
            (0xfb as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_5).words.w1 =
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (220 as libc::c_int as u32_0 &
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
        if (*this).work[MO_CORE_DMG_FLASH_TIMER as libc::c_int as usize] as
               libc::c_int % 2 as libc::c_int != 0 as libc::c_int {
            let fresh23 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_6: *mut Gfx = fresh23;
            (*_g_6).words.w0 =
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
            (*_g_6).words.w1 =
                (255 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (60 as libc::c_int as u32_0 &
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
            let fresh24 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_7: *mut Gfx = fresh24;
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
                        0 as libc::c_int
        }
        let fresh25 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_8: *mut Gfx = fresh25;
        (*_g_8).words.w0 =
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
        (*_g_8).words.w1 =
            gSegments[((gMorphaCoreNucleusDL.as_mut_ptr() as u32_0) <<
                           4 as libc::c_int >> 28 as libc::c_int) as
                          usize].wrapping_add(gMorphaCoreNucleusDL.as_mut_ptr()
                                                  as u32_0 &
                                                  0xffffff as libc::c_int as
                                                      libc::c_uint).wrapping_add(0x80000000
                                                                                     as
                                                                                     libc::c_uint)
                as *mut libc::c_void as libc::c_uint;
        if (*this).work[MO_CORE_DRAW_SHADOW as libc::c_int as usize] as
               libc::c_int != 0 && (*this).actor.world.pos.y >= 0.0f32 ||
               (*this).actor.world.pos.y <
                   (*(*(*globalCtx).colCtx.colHeader).waterBoxes.offset(0 as
                                                                            libc::c_int
                                                                            as
                                                                            isize)).ySurface
                       as libc::c_int as libc::c_float {
            let mut groundLevel: f32_0 = 0.;
            let mut shadowAlpha: s16 = 0;
            if (*this).actor.world.pos.y <
                   (*(*(*globalCtx).colCtx.colHeader).waterBoxes.offset(0 as
                                                                            libc::c_int
                                                                            as
                                                                            isize)).ySurface
                       as libc::c_int as libc::c_float {
                groundLevel = -280.0f32;
                shadowAlpha = 100 as libc::c_int as s16
            } else {
                groundLevel = 0.0f32;
                shadowAlpha = 160 as libc::c_int as s16
            }
            func_80094044((*globalCtx).state.gfxCtx);
            let fresh26 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_9: *mut Gfx = fresh26;
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
                    (shadowAlpha as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            Matrix_Translate((*this).actor.world.pos.x, groundLevel,
                             (*this).actor.world.pos.z,
                             MTXMODE_NEW as libc::c_int as u8_0);
            Matrix_Scale(0.23f32, 1.0f32, 0.23f32,
                         MTXMODE_APPLY as libc::c_int as u8_0);
            let fresh27 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_10: *mut Gfx = fresh27;
            (*_g_10).words.w0 =
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
            (*_g_10).words.w1 =
                Matrix_NewMtx((*globalCtx).state.gfxCtx,
                              b"../z_boss_mo.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              6820 as libc::c_int) as libc::c_uint;
            let fresh28 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_11: *mut Gfx = fresh28;
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
                gSegments[((gCircleShadowDL.as_mut_ptr() as u32_0) <<
                               4 as libc::c_int >> 28 as libc::c_int) as
                              usize].wrapping_add(gCircleShadowDL.as_mut_ptr()
                                                      as u32_0 &
                                                      0xffffff as libc::c_int
                                                          as
                                                          libc::c_uint).wrapping_add(0x80000000
                                                                                         as
                                                                                         libc::c_uint)
                    as *mut libc::c_void as libc::c_uint
        }
    }
    if (*this).actor.world.pos.y <
           (*(*(*globalCtx).colCtx.colHeader).waterBoxes.offset(0 as
                                                                    libc::c_int
                                                                    as
                                                                    isize)).ySurface
               as libc::c_int as libc::c_float {
        BossMo_DrawWater(this, globalCtx);
    }
    if (*this).csCamera as libc::c_int != 0 as libc::c_int &&
           ((*this).csState as libc::c_int) < MO_INTRO_REVEAL as libc::c_int {
        let mut sp8C: f32_0 = 0.;
        let mut sp88: f32_0 = 0.;
        let mut sp84: f32_0 = 0.;
        let mut temp: f32_0 = 0.;
        let mut sp7C: f32_0 = 0.;
        let mut sp78: f32_0 = 0.;
        let mut sp6C: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
        let mut sp60: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
        func_80093D84((*globalCtx).state.gfxCtx);
        let fresh29 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_12: *mut Gfx = fresh29;
        (*_g_12).words.w0 =
            (0xfa as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0xff as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (0xff as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_12).words.w1 =
            (200 as libc::c_int as u32_0 &
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
                ((*this).fwork[MO_CORE_INTRO_WATER_ALPHA as libc::c_int as
                                   usize] as s8 as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh30 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_13: *mut Gfx = fresh30;
        (*_g_13).words.w0 =
            (0xfb as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_13).words.w1 =
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (100 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                (255 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                ((*this).fwork[MO_CORE_INTRO_WATER_ALPHA as libc::c_int as
                                   usize] as s8 as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh31 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_14: *mut Gfx = fresh31;
        (*_g_14).words.w0 =
            (0xdb as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
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
                             (*sMorphaTent1).waterTex1x as s16 as u32_0,
                             (*sMorphaTent1).waterTex1y as s16 as u32_0,
                             32 as libc::c_int, 32 as libc::c_int,
                             1 as libc::c_int,
                             (*sMorphaTent1).waterTex2x as s16 as u32_0,
                             (*sMorphaTent1).waterTex2y as s16 as u32_0,
                             32 as libc::c_int, 32 as libc::c_int) as
                libc::c_uint;
        sp8C = (*this).cameraAt.x - (*this).cameraEye.x;
        sp88 = (*this).cameraAt.y - (*this).cameraEye.y;
        sp84 = (*this).cameraAt.z - (*this).cameraEye.z;
        temp = sp8C * sp8C + sp84 * sp84;
        sp7C = Math_FAtan2F(sp8C, sp84);
        sp78 = -Math_FAtan2F(sp88, sqrtf(temp));
        sp6C.x = 0.0f32;
        sp6C.y = 0.0f32;
        sp6C.z = 10.0f32;
        Matrix_RotateY(sp7C, MTXMODE_NEW as libc::c_int as u8_0);
        Matrix_RotateX(sp78, MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_MultVec3f(&mut sp6C, &mut sp60);
        sp8C = sp60.x + (*this).cameraEye.x;
        sp88 = sp60.y + (*this).cameraEye.y;
        sp84 = sp60.z + (*this).cameraEye.z;
        Matrix_Translate(sp8C, sp88, sp84,
                         MTXMODE_NEW as libc::c_int as u8_0);
        Matrix_RotateY(sp7C, MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_RotateX(sp78, MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_RotateZ(-(0.01f32 *
                             (*this).work[MO_TENT_VAR_TIMER as libc::c_int as
                                              usize] as libc::c_int as
                                 libc::c_float),
                       MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_RotateZ(0.1f32 *
                           (*this).work[MO_TENT_VAR_TIMER as libc::c_int as
                                            usize] as libc::c_int as
                               libc::c_float,
                       MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_Scale(0.825f32, 1.175f32, 0.825f32,
                     MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_RotateZ(-((*this).work[MO_TENT_VAR_TIMER as libc::c_int as
                                          usize] as libc::c_int as
                             libc::c_float * 0.1f32),
                       MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_RotateX(3.14159265358979323846f32 / 2.0f32,
                       MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_Scale(0.05f32, 1.0f32, 0.05f32,
                     MTXMODE_APPLY as libc::c_int as u8_0);
        let fresh32 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_15: *mut Gfx = fresh32;
        (*_g_15).words.w0 =
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
        (*_g_15).words.w1 =
            Matrix_NewMtx((*globalCtx).state.gfxCtx,
                          b"../z_boss_mo.c\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                          6941 as libc::c_int) as libc::c_uint;
        let fresh33 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_16: *mut Gfx = fresh33;
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
        (*_g_16).words.w1 = gMorphaWaterDL.as_mut_ptr() as libc::c_uint
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_boss_mo.c\x00" as *const u8 as
                         *const libc::c_char, 6945 as libc::c_int);
    BossMo_DrawEffects((*globalCtx).specialEffects as *mut BossMoEffect,
                       globalCtx);
}
#[no_mangle]
pub unsafe extern "C" fn BossMo_DrawTent(mut thisx: *mut Actor,
                                         mut globalCtx: *mut GlobalContext) {
    let mut pad: s32 = 0;
    let mut this: *mut BossMo = thisx as *mut BossMo;
    let mut scroll: u16_0 = 0;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_boss_mo.c\x00" as *const u8 as *const libc::c_char,
                    6958 as libc::c_int);
    func_80093D18((*globalCtx).state.gfxCtx);
    let fresh34 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g: *mut Gfx = fresh34;
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
            (((*this).baseAlpha * 1.5f32) as s8 as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh35 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_0: *mut Gfx = fresh35;
    (*_g_0).words.w0 =
        (0xfb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_0).words.w1 =
        (150 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (150 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (150 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    func_80093D84((*globalCtx).state.gfxCtx);
    let fresh36 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_1: *mut Gfx = fresh36;
    (*_g_1).words.w0 =
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
    (*_g_1).words.w1 =
        Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                         (*this).work[MO_TENT_BASE_TEX1_X as libc::c_int as
                                          usize] as u32_0,
                         (*this).work[MO_TENT_BASE_TEX1_Y as libc::c_int as
                                          usize] as u32_0, 32 as libc::c_int,
                         32 as libc::c_int, 1 as libc::c_int,
                         (*this).work[MO_TENT_BASE_TEX2_X as libc::c_int as
                                          usize] as u32_0,
                         (*this).work[MO_TENT_BASE_TEX2_Y as libc::c_int as
                                          usize] as u32_0, 32 as libc::c_int,
                         32 as libc::c_int) as libc::c_uint;
    let fresh37 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_2: *mut Gfx = fresh37;
    (*_g_2).words.w0 =
        (0xfa as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0xff as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (0xff as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_2).words.w1 =
        (200 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (((*this).baseAlpha * 12.0f32 / 10.0f32) as s8 as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh38 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_3: *mut Gfx = fresh38;
    (*_g_3).words.w0 =
        (0xfb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_3).words.w1 =
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
            ((*this).baseAlpha as s8 as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    scroll =
        ((Math_SinS(((*this).work[MO_TENT_VAR_TIMER as libc::c_int as usize]
                         as libc::c_int * 0xb00 as libc::c_int) as s16) *
              30.0f32) as s16 as libc::c_int + 350 as libc::c_int) as u16_0;
    let fresh39 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_4: *mut Gfx = fresh39;
    (*_g_4).words.w0 =
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
    (*_g_4).words.w1 =
        (scroll as u32_0 &
             (((0x1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 16 as libc::c_int |
            (scroll as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    if (*this).drawActor != 0 { BossMo_DrawTentacle(this, globalCtx); }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_boss_mo.c\x00" as *const u8 as
                         *const libc::c_char, 7023 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn BossMo_UpdateEffects(mut this: *mut BossMo,
                                              mut globalCtx:
                                                  *mut GlobalContext) {
    let mut effect: *mut BossMoEffect =
        (*globalCtx).specialEffects as *mut BossMoEffect;
    let mut i: s16 = 0;
    let mut targetPos: *mut Vec3f = 0 as *mut Vec3f;
    let mut dx: f32_0 = 0.;
    let mut dz: f32_0 = 0.;
    let mut bubbleSpeed: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
    let mut bubbleVel: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[BossMoEffect; 300]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<BossMoEffect>()
                                                   as libc::c_ulong) as s32 {
        if (*effect).type_0 as libc::c_int != MO_FX_NONE as libc::c_int {
            (*effect).timer = (*effect).timer.wrapping_add(1);
            if (*effect).stopTimer as libc::c_int == 0 as libc::c_int {
                (*effect).pos.x += (*effect).vel.x;
                (*effect).pos.y += (*effect).vel.y;
                (*effect).pos.z += (*effect).vel.z;
                (*effect).vel.x += (*effect).accel.x;
                (*effect).vel.y += (*effect).accel.y;
                (*effect).vel.z += (*effect).accel.z
            } else {
                (*effect).stopTimer = (*effect).stopTimer.wrapping_sub(1)
            }
            if (*effect).type_0 as libc::c_int <=
                   MO_FX_BIG_RIPPLE as libc::c_int {
                if (*this).csState as libc::c_int >=
                       MO_DEATH_START as libc::c_int {
                    (*effect).pos.y =
                        (*(*(*globalCtx).colCtx.colHeader).waterBoxes.offset(0
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 isize)).ySurface
                            as f32_0
                }
                Math_ApproachF(&mut (*effect).scale,
                               (*effect).fwork[0 as libc::c_int as usize],
                               0.2f32,
                               (*effect).fwork[1 as libc::c_int as usize]);
                if (*effect).rippleMode as libc::c_int == 0 as libc::c_int {
                    (*effect).alpha =
                        ((*effect).alpha as libc::c_int + 15 as libc::c_int)
                            as s16;
                    if (*effect).alpha as libc::c_int >=
                           (*effect).maxAlpha as libc::c_int {
                        (*effect).alpha = (*effect).maxAlpha;
                        (*effect).rippleMode += 1
                    }
                } else {
                    (*effect).alpha =
                        ((*effect).alpha as libc::c_int - 5 as libc::c_int) as
                            s16;
                    if (*effect).alpha as libc::c_int <= 0 as libc::c_int {
                        (*effect).alpha = 0 as libc::c_int as s16;
                        (*effect).type_0 = MO_FX_NONE as libc::c_int as u8_0
                    }
                }
            } else if (*effect).type_0 as libc::c_int ==
                          MO_FX_BUBBLE as libc::c_int {
                if (*effect).targetPos.is_null() {
                    if (*effect).accel.y > 0.0f32 &&
                           (*effect).pos.y >=
                               (*(*(*globalCtx).colCtx.colHeader).waterBoxes.offset(0
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)).ySurface
                                   as libc::c_int as libc::c_float {
                        (*effect).type_0 = MO_FX_NONE as libc::c_int as u8_0
                    } else {
                        if (*effect).vel.y > 2.0f32 {
                            (*effect).vel.y = 2.0f32
                        }
                        (*effect).alpha =
                            ((*effect).alpha as libc::c_int -
                                 20 as libc::c_int) as s16;
                        if (*effect).alpha as libc::c_int <= 0 as libc::c_int
                           {
                            (*effect).alpha = 0 as libc::c_int as s16;
                            (*effect).type_0 =
                                MO_FX_NONE as libc::c_int as u8_0
                        }
                    }
                } else {
                    if (*effect).timer as libc::c_int % 4 as libc::c_int ==
                           0 as libc::c_int {
                        targetPos = (*effect).targetPos;
                        dx = (*targetPos).x - (*effect).pos.x;
                        dz = (*targetPos).z - (*effect).pos.z;
                        bubbleSpeed.z =
                            (*effect).fwork[0 as libc::c_int as usize];
                        Matrix_RotateY(Math_FAtan2F(dx, dz),
                                       MTXMODE_NEW as libc::c_int as u8_0);
                        Matrix_MultVec3f(&mut bubbleSpeed, &mut bubbleVel);
                        (*effect).vel.x = bubbleVel.x;
                        (*effect).vel.z = bubbleVel.z
                    }
                    Math_ApproachF(&mut *(*effect).fwork.as_mut_ptr().offset(0
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 isize),
                                   5.0f32, 1.0f32, 0.5f32);
                    if (*effect).timer as libc::c_int > 20 as libc::c_int {
                        (*effect).alpha =
                            ((*effect).alpha as libc::c_int -
                                 30 as libc::c_int) as s16;
                        (*effect).accel.y = 1.5f32;
                        if (*effect).alpha as libc::c_int <= 0 as libc::c_int
                               ||
                               (*effect).pos.y >=
                                   (*(*(*globalCtx).colCtx.colHeader).waterBoxes.offset(0
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            isize)).ySurface
                                       as libc::c_int as libc::c_float {
                            (*effect).alpha = 0 as libc::c_int as s16;
                            (*effect).type_0 =
                                MO_FX_NONE as libc::c_int as u8_0
                        }
                    } else {
                        (*effect).alpha =
                            ((*effect).alpha as libc::c_int +
                                 30 as libc::c_int) as s16;
                        if (*effect).alpha as libc::c_int >=
                               255 as libc::c_int {
                            (*effect).alpha = 255 as libc::c_int as s16
                        }
                    }
                }
            } else if (*effect).type_0 as libc::c_int ==
                          MO_FX_DROPLET as libc::c_int ||
                          (*effect).type_0 as libc::c_int ==
                              MO_FX_SPLASH as libc::c_int ||
                          (*effect).type_0 as libc::c_int ==
                              MO_FX_SPLASH_TRAIL as libc::c_int ||
                          (*effect).type_0 as libc::c_int ==
                              MO_FX_WET_SPOT as libc::c_int {
                let mut shimmer: f32_0 =
                    if (*effect).timer as libc::c_int & 6 as libc::c_int != 0
                       {
                        80.0f32
                    } else { 200.0f32 };
                Math_ApproachF(&mut *(*effect).fwork.as_mut_ptr().offset(0 as
                                                                             libc::c_int
                                                                             as
                                                                             isize),
                               shimmer, 1.0f32, 80.0f32);
                if (*effect).type_0 as libc::c_int ==
                       MO_FX_WET_SPOT as libc::c_int {
                    Math_ApproachF(&mut (*effect).scale,
                                   (*effect).fwork[1 as libc::c_int as usize],
                                   0.1f32, 0.6f32);
                    (*effect).alpha =
                        ((*effect).alpha as libc::c_int - 15 as libc::c_int)
                            as s16;
                    if (*effect).alpha as libc::c_int <= 0 as libc::c_int {
                        (*effect).alpha = 0 as libc::c_int as s16;
                        (*effect).type_0 = MO_FX_NONE as libc::c_int as u8_0
                    }
                } else {
                    (*effect).alpha =
                        (*effect).fwork[0 as libc::c_int as usize] as s16;
                    if (*effect).type_0 as libc::c_int ==
                           MO_FX_SPLASH_TRAIL as libc::c_int {
                        Math_ApproachF(&mut (*effect).scale, 0.0f32, 1.0f32,
                                       0.02f32);
                        if (*effect).scale <= 0.0f32 {
                            (*effect).type_0 =
                                MO_FX_NONE as libc::c_int as u8_0
                        }
                    } else {
                        if (*effect).type_0 as libc::c_int ==
                               MO_FX_SPLASH as libc::c_int {
                            let mut velocity: Vec3f =
                                {
                                    let mut init =
                                        Vec3f{x: 0.0f32,
                                              y: 0.0f32,
                                              z: 0.0f32,};
                                    init
                                };
                            BossMo_SpawnDroplet(MO_FX_SPLASH_TRAIL as
                                                    libc::c_int as s16,
                                                (*globalCtx).specialEffects as
                                                    *mut BossMoEffect,
                                                &mut (*effect).pos,
                                                &mut velocity,
                                                (*effect).scale);
                        }
                        if (*effect).vel.y < -20.0f32 {
                            (*effect).vel.y = -20.0f32;
                            (*effect).accel.y = 0.0f32
                        }
                        if (*effect).stopTimer as libc::c_int ==
                               0 as libc::c_int {
                            if (*effect).vel.y < -5.0f32 {
                                Math_ApproachF(&mut *(*effect).fwork.as_mut_ptr().offset(1
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             isize),
                                               5.0f32, 0.1f32, 0.15f32);
                            }
                        } else if (*effect).stopTimer as libc::c_int ==
                                      1 as libc::c_int {
                            (*effect).vel.x = Rand_CenteredFloat(3.0f32);
                            (*effect).vel.z = Rand_CenteredFloat(3.0f32);
                            (*effect).accel.y = -1.0f32
                        }
                        if (*effect).pos.y <= -280.0f32 ||
                               1.0f32 >= (*effect).pos.y &&
                                   (*effect).pos.y >= -20.0f32 &&
                                   BossMo_NearLand(&mut (*effect).pos, 0.0f32)
                                       != 0 {
                            (*effect).accel.y = 0.0f32;
                            (*effect).vel.z = 0.0f32;
                            (*effect).vel.y = 0.0f32;
                            (*effect).vel.x = 0.0f32;
                            if (*effect).pos.y <= -280.0f32 {
                                (*effect).pos.y = -280.0f32
                            } else { (*effect).pos.y = 0.0f32 }
                            (*effect).type_0 =
                                MO_FX_WET_SPOT as libc::c_int as u8_0;
                            (*effect).alpha = 150 as libc::c_int as s16;
                            (*effect).fwork[1 as libc::c_int as usize] =
                                (*effect).scale * 15.0f32 * 0.15f32
                        } else if (*effect).pos.y <=
                                      (*(*(*globalCtx).colCtx.colHeader).waterBoxes.offset(0
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               isize)).ySurface
                                          as libc::c_int as libc::c_float {
                            let mut pos: Vec3f = (*effect).pos;
                            pos.y =
                                (*(*(*globalCtx).colCtx.colHeader).waterBoxes.offset(0
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         isize)).ySurface
                                    as f32_0;
                            if (*effect).type_0 as libc::c_int ==
                                   MO_FX_SPLASH as libc::c_int {
                                BossMo_SpawnRipple((*globalCtx).specialEffects
                                                       as *mut BossMoEffect,
                                                   &mut pos, 60.0f32,
                                                   160.0f32,
                                                   80 as libc::c_int as s16,
                                                   290 as libc::c_int as s16,
                                                   MO_FX_SMALL_RIPPLE as
                                                       libc::c_int as u8_0);
                            } else {
                                BossMo_SpawnRipple((*globalCtx).specialEffects
                                                       as *mut BossMoEffect,
                                                   &mut pos, 40.0f32,
                                                   110.0f32,
                                                   80 as libc::c_int as s16,
                                                   290 as libc::c_int as s16,
                                                   MO_FX_SMALL_RIPPLE as
                                                       libc::c_int as u8_0);
                            }
                            (*effect).type_0 =
                                MO_FX_NONE as libc::c_int as u8_0
                        }
                    }
                }
            }
        }
        i += 1;
        effect = effect.offset(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossMo_DrawEffects(mut effect: *mut BossMoEffect,
                                            mut globalCtx:
                                                *mut GlobalContext) {
    let mut flag: u8_0 = 0 as libc::c_int as u8_0;
    let mut i: s16 = 0;
    let mut pad: s32 = 0;
    let mut gfxCtx: *mut GraphicsContext = (*globalCtx).state.gfxCtx;
    let mut effectHead: *mut BossMoEffect = effect;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), gfxCtx,
                    b"../z_boss_mo.c\x00" as *const u8 as *const libc::c_char,
                    7264 as libc::c_int);
    Matrix_Push();
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[BossMoEffect; 300]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<BossMoEffect>()
                                                   as libc::c_ulong) as s32 {
        if (*effect).type_0 as libc::c_int == MO_FX_BIG_RIPPLE as libc::c_int
           {
            if flag as libc::c_int == 0 as libc::c_int {
                func_80094BC4(gfxCtx);
                let fresh40 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g: *mut Gfx = fresh40;
                (*_g).words.w0 =
                    (0xfb as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int;
                (*_g).words.w1 =
                    (155 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (155 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            16 as libc::c_int |
                        (255 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            8 as libc::c_int |
                        (0 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                flag = flag.wrapping_add(1)
            }
            let fresh41 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_0: *mut Gfx = fresh41;
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
            Matrix_Scale((*effect).scale, 1.0f32, (*effect).scale,
                         MTXMODE_APPLY as libc::c_int as u8_0);
            let fresh42 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_1: *mut Gfx = fresh42;
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
                              b"../z_boss_mo.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              7294 as libc::c_int) as libc::c_uint;
            let fresh43 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_2: *mut Gfx = fresh43;
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
            (*_g_2).words.w1 = gEffWaterRippleDL.as_mut_ptr() as libc::c_uint
        }
        i += 1;
        effect = effect.offset(1)
    }
    effect = effectHead;
    flag = 0 as libc::c_int as u8_0;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[BossMoEffect; 300]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<BossMoEffect>()
                                                   as libc::c_ulong) as s32 {
        if (*effect).type_0 as libc::c_int ==
               MO_FX_SMALL_RIPPLE as libc::c_int {
            if flag as libc::c_int == 0 as libc::c_int {
                func_80093D84((*globalCtx).state.gfxCtx);
                let fresh44 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_3: *mut Gfx = fresh44;
                (*_g_3).words.w0 =
                    (0xfb as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int;
                (*_g_3).words.w1 =
                    (155 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (155 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            16 as libc::c_int |
                        (255 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            8 as libc::c_int |
                        (0 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                flag = flag.wrapping_add(1)
            }
            let fresh45 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_4: *mut Gfx = fresh45;
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
            Matrix_Scale((*effect).scale, 1.0f32, (*effect).scale,
                         MTXMODE_APPLY as libc::c_int as u8_0);
            let fresh46 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_5: *mut Gfx = fresh46;
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
                              b"../z_boss_mo.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              7330 as libc::c_int) as libc::c_uint;
            let fresh47 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_6: *mut Gfx = fresh47;
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
            (*_g_6).words.w1 = gEffShockwaveDL.as_mut_ptr() as libc::c_uint
        }
        i += 1;
        effect = effect.offset(1)
    }
    effect = effectHead;
    flag = 0 as libc::c_int as u8_0;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[BossMoEffect; 300]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<BossMoEffect>()
                                                   as libc::c_ulong) as s32 {
        if (*effect).type_0 as libc::c_int == MO_FX_DROPLET as libc::c_int ||
               (*effect).type_0 as libc::c_int == MO_FX_SPLASH as libc::c_int
               ||
               (*effect).type_0 as libc::c_int ==
                   MO_FX_SPLASH_TRAIL as libc::c_int {
            if flag as libc::c_int == 0 as libc::c_int {
                (*__gfxCtx).polyXlu.p =
                    Gfx_CallSetupDL((*__gfxCtx).polyXlu.p,
                                    0 as libc::c_int as u32_0);
                let fresh48 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_7: *mut Gfx = fresh48;
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
                    gSegments[((gDust1Tex.as_mut_ptr() as u32_0) <<
                                   4 as libc::c_int >> 28 as libc::c_int) as
                                  usize].wrapping_add(gDust1Tex.as_mut_ptr()
                                                          as u32_0 &
                                                          0xffffff as
                                                              libc::c_int as
                                                              libc::c_uint).wrapping_add(0x80000000
                                                                                             as
                                                                                             libc::c_uint)
                        as *mut libc::c_void as libc::c_uint;
                let fresh49 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_8: *mut Gfx = fresh49;
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
                    gMorphaDropletMaterialDL.as_mut_ptr() as libc::c_uint;
                let fresh50 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_9: *mut Gfx = fresh50;
                (*_g_9).words.w0 =
                    (0xfb as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int;
                (*_g_9).words.w1 =
                    (250 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (250 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            16 as libc::c_int |
                        (255 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            8 as libc::c_int |
                        (0 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                flag = flag.wrapping_add(1)
            }
            let fresh51 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_10: *mut Gfx = fresh51;
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
                ((*effect).fwork[0 as libc::c_int as usize] as s16 as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((*effect).fwork[0 as libc::c_int as usize] as s16 as
                         u32_0 &
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
            func_800D1FD4(&mut (*globalCtx).billboardMtxF);
            Matrix_Scale((*effect).scale /
                             (*effect).fwork[1 as libc::c_int as usize],
                         (*effect).fwork[1 as libc::c_int as usize] *
                             (*effect).scale, 1.0f32,
                         MTXMODE_APPLY as libc::c_int as u8_0);
            let fresh52 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_11: *mut Gfx = fresh52;
            (*_g_11).words.w0 =
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
            (*_g_11).words.w1 =
                Matrix_NewMtx(gfxCtx,
                              b"../z_boss_mo.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              7373 as libc::c_int) as libc::c_uint;
            let fresh53 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_12: *mut Gfx = fresh53;
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
                gMorphaDropletModelDL.as_mut_ptr() as libc::c_uint
        }
        i += 1;
        effect = effect.offset(1)
    }
    effect = effectHead;
    flag = 0 as libc::c_int as u8_0;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[BossMoEffect; 300]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<BossMoEffect>()
                                                   as libc::c_ulong) as s32 {
        if (*effect).type_0 as libc::c_int == MO_FX_WET_SPOT as libc::c_int {
            if flag as libc::c_int == 0 as libc::c_int {
                func_80094044(gfxCtx);
                let fresh54 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_13: *mut Gfx = fresh54;
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
                    gSegments[((gDust1Tex.as_mut_ptr() as u32_0) <<
                                   4 as libc::c_int >> 28 as libc::c_int) as
                                  usize].wrapping_add(gDust1Tex.as_mut_ptr()
                                                          as u32_0 &
                                                          0xffffff as
                                                              libc::c_int as
                                                              libc::c_uint).wrapping_add(0x80000000
                                                                                             as
                                                                                             libc::c_uint)
                        as *mut libc::c_void as libc::c_uint;
                let fresh55 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_14: *mut Gfx = fresh55;
                (*_g_14).words.w0 =
                    (0xfb as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int;
                (*_g_14).words.w1 =
                    (250 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (250 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            16 as libc::c_int |
                        (255 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            8 as libc::c_int |
                        (0 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                let fresh56 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_15: *mut Gfx = fresh56;
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
                    gMorphaDropletMaterialDL.as_mut_ptr() as libc::c_uint;
                flag = flag.wrapping_add(1)
            }
            let fresh57 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_16: *mut Gfx = fresh57;
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
                ((*effect).fwork[0 as libc::c_int as usize] as s16 as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((*effect).fwork[0 as libc::c_int as usize] as s16 as
                         u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (0xff as libc::c_int as u32_0 &
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
            Matrix_Scale((*effect).scale, 1.0f32, (*effect).scale,
                         MTXMODE_APPLY as libc::c_int as u8_0);
            let fresh58 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_17: *mut Gfx = fresh58;
            (*_g_17).words.w0 =
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
            (*_g_17).words.w1 =
                Matrix_NewMtx(gfxCtx,
                              b"../z_boss_mo.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              7441 as libc::c_int) as libc::c_uint;
            let fresh59 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_18: *mut Gfx = fresh59;
            (*_g_18).words.w0 =
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
            (*_g_18).words.w1 =
                gMorphaWetSpotModelDL.as_mut_ptr() as libc::c_uint
        }
        i += 1;
        effect = effect.offset(1)
    }
    effect = effectHead;
    flag = 0 as libc::c_int as u8_0;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[BossMoEffect; 300]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<BossMoEffect>()
                                                   as libc::c_ulong) as s32 {
        if (*effect).type_0 as libc::c_int == MO_FX_BUBBLE as libc::c_int {
            if flag as libc::c_int == 0 as libc::c_int {
                func_80093D18((*globalCtx).state.gfxCtx);
                let fresh60 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_19: *mut Gfx = fresh60;
                (*_g_19).words.w0 =
                    (0xfb as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int;
                (*_g_19).words.w1 =
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
                flag = flag.wrapping_add(1)
            }
            let fresh61 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_20: *mut Gfx = fresh61;
            (*_g_20).words.w0 =
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
            (*_g_20).words.w1 =
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
            func_800D1FD4(&mut (*globalCtx).billboardMtxF);
            Matrix_Scale((*effect).scale, (*effect).scale, 1.0f32,
                         MTXMODE_APPLY as libc::c_int as u8_0);
            let fresh62 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_21: *mut Gfx = fresh62;
            (*_g_21).words.w0 =
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
            (*_g_21).words.w1 =
                Matrix_NewMtx(gfxCtx,
                              b"../z_boss_mo.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              7476 as libc::c_int) as libc::c_uint;
            let fresh63 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_22: *mut Gfx = fresh63;
            (*_g_22).words.w0 =
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
            (*_g_22).words.w1 = gMorphaBubbleDL.as_mut_ptr() as libc::c_uint
        }
        i += 1;
        effect = effect.offset(1)
    }
    Matrix_Pop();
    Graph_CloseDisps(dispRefs.as_mut_ptr(), gfxCtx,
                     b"../z_boss_mo.c\x00" as *const u8 as
                         *const libc::c_char, 7482 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn BossMo_Unknown() {
    // Appears to be a test function for sound effects.
    static mut zeroVec: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
    static mut unkSfx: [u16_0; 532] =
        [0x800 as libc::c_int as u16_0, 0x800 as libc::c_int as u16_0,
         0x800 as libc::c_int as u16_0, 0x801 as libc::c_int as u16_0,
         0x802 as libc::c_int as u16_0, 0x803 as libc::c_int as u16_0,
         0x804 as libc::c_int as u16_0, 0x805 as libc::c_int as u16_0,
         0x806 as libc::c_int as u16_0, 0x807 as libc::c_int as u16_0,
         0x808 as libc::c_int as u16_0, 0x80b as libc::c_int as u16_0,
         0x80a as libc::c_int as u16_0, 0x80b as libc::c_int as u16_0,
         0x80c as libc::c_int as u16_0, 0x80d as libc::c_int as u16_0,
         0x80f as libc::c_int as u16_0, 0x810 as libc::c_int as u16_0,
         0x810 as libc::c_int as u16_0, 0x811 as libc::c_int as u16_0,
         0x812 as libc::c_int as u16_0, 0x813 as libc::c_int as u16_0,
         0x814 as libc::c_int as u16_0, 0x815 as libc::c_int as u16_0,
         0x816 as libc::c_int as u16_0, 0x817 as libc::c_int as u16_0,
         0x818 as libc::c_int as u16_0, 0x81b as libc::c_int as u16_0,
         0x81a as libc::c_int as u16_0, 0x81b as libc::c_int as u16_0,
         0x81d as libc::c_int as u16_0, 0x81f as libc::c_int as u16_0,
         0x820 as libc::c_int as u16_0, 0x820 as libc::c_int as u16_0,
         0x821 as libc::c_int as u16_0, 0x822 as libc::c_int as u16_0,
         0x823 as libc::c_int as u16_0, 0x824 as libc::c_int as u16_0,
         0x825 as libc::c_int as u16_0, 0x826 as libc::c_int as u16_0,
         0x827 as libc::c_int as u16_0, 0x828 as libc::c_int as u16_0,
         0x82b as libc::c_int as u16_0, 0x82a as libc::c_int as u16_0,
         0x82b as libc::c_int as u16_0, 0x82d as libc::c_int as u16_0,
         0x82f as libc::c_int as u16_0, 0x830 as libc::c_int as u16_0,
         0x831 as libc::c_int as u16_0, 0x831 as libc::c_int as u16_0,
         0x832 as libc::c_int as u16_0, 0x833 as libc::c_int as u16_0,
         0x834 as libc::c_int as u16_0, 0x834 as libc::c_int as u16_0,
         0x834 as libc::c_int as u16_0, 0x834 as libc::c_int as u16_0,
         0x834 as libc::c_int as u16_0, 0x834 as libc::c_int as u16_0,
         0x835 as libc::c_int as u16_0, 0x835 as libc::c_int as u16_0,
         0x835 as libc::c_int as u16_0, 0x835 as libc::c_int as u16_0,
         0x835 as libc::c_int as u16_0, 0x835 as libc::c_int as u16_0,
         0x836 as libc::c_int as u16_0, 0x2889 as libc::c_int as u16_0,
         0x288a as libc::c_int as u16_0, 0x839 as libc::c_int as u16_0,
         0x83a as libc::c_int as u16_0, 0x83b as libc::c_int as u16_0,
         0x83c as libc::c_int as u16_0, 0x83d as libc::c_int as u16_0,
         0x83e as libc::c_int as u16_0, 0x83f as libc::c_int as u16_0,
         0x840 as libc::c_int as u16_0, 0x840 as libc::c_int as u16_0,
         0x840 as libc::c_int as u16_0, 0x841 as libc::c_int as u16_0,
         0x842 as libc::c_int as u16_0, 0x843 as libc::c_int as u16_0,
         0x844 as libc::c_int as u16_0, 0x845 as libc::c_int as u16_0,
         0x846 as libc::c_int as u16_0, 0x847 as libc::c_int as u16_0,
         0x848 as libc::c_int as u16_0, 0x84b as libc::c_int as u16_0,
         0x84a as libc::c_int as u16_0, 0x84b as libc::c_int as u16_0,
         0x84d as libc::c_int as u16_0, 0x84f as libc::c_int as u16_0,
         0x850 as libc::c_int as u16_0, 0x850 as libc::c_int as u16_0,
         0x851 as libc::c_int as u16_0, 0x852 as libc::c_int as u16_0,
         0x853 as libc::c_int as u16_0, 0x854 as libc::c_int as u16_0,
         0x855 as libc::c_int as u16_0, 0x856 as libc::c_int as u16_0,
         0x857 as libc::c_int as u16_0, 0x858 as libc::c_int as u16_0,
         0x85b as libc::c_int as u16_0, 0x85a as libc::c_int as u16_0,
         0x85b as libc::c_int as u16_0, 0x85d as libc::c_int as u16_0,
         0x85f as libc::c_int as u16_0, 0x863 as libc::c_int as u16_0,
         0x864 as libc::c_int as u16_0, 0x865 as libc::c_int as u16_0,
         0x866 as libc::c_int as u16_0, 0x867 as libc::c_int as u16_0,
         (0x868 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         0x1800 as libc::c_int as u16_0, 0x1801 as libc::c_int as u16_0,
         0x1802 as libc::c_int as u16_0, 0x1803 as libc::c_int as u16_0,
         0x1804 as libc::c_int as u16_0, 0x1805 as libc::c_int as u16_0,
         0x1806 as libc::c_int as u16_0, 0x1806 as libc::c_int as u16_0,
         0x1807 as libc::c_int as u16_0, 0x1808 as libc::c_int as u16_0,
         0x1809 as libc::c_int as u16_0, 0x180a as libc::c_int as u16_0,
         (0x180b as libc::c_int - 0x800 as libc::c_int) as u16_0,
         0x180c as libc::c_int as u16_0,
         (0x180d as libc::c_int - 0x800 as libc::c_int) as u16_0,
         0x180e as libc::c_int as u16_0, 0x180f as libc::c_int as u16_0,
         (0x1810 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         0x1811 as libc::c_int as u16_0, 0x1812 as libc::c_int as u16_0,
         0x1813 as libc::c_int as u16_0, 0x1814 as libc::c_int as u16_0,
         0x1814 as libc::c_int as u16_0, 0x1815 as libc::c_int as u16_0,
         (0x800 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         (0x800 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         0x1818 as libc::c_int as u16_0, 0x181a as libc::c_int as u16_0,
         0x181b as libc::c_int as u16_0, 0x181b as libc::c_int as u16_0,
         0x181c as libc::c_int as u16_0, 0x181d as libc::c_int as u16_0,
         0x181e as libc::c_int as u16_0, 0x181f as libc::c_int as u16_0,
         0x1820 as libc::c_int as u16_0, 0x1821 as libc::c_int as u16_0,
         (0x1822 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         0x1823 as libc::c_int as u16_0, 0x1824 as libc::c_int as u16_0,
         0x1825 as libc::c_int as u16_0, 0x1826 as libc::c_int as u16_0,
         0x1827 as libc::c_int as u16_0, 0x1828 as libc::c_int as u16_0,
         0x1829 as libc::c_int as u16_0, 0x182a as libc::c_int as u16_0,
         0x182b as libc::c_int as u16_0, 0x1830 as libc::c_int as u16_0,
         0x1830 as libc::c_int as u16_0, 0x1830 as libc::c_int as u16_0,
         0x1831 as libc::c_int as u16_0, 0x1832 as libc::c_int as u16_0,
         0x1833 as libc::c_int as u16_0, 0x1834 as libc::c_int as u16_0,
         0x1835 as libc::c_int as u16_0, 0x1836 as libc::c_int as u16_0,
         0x1837 as libc::c_int as u16_0, 0x1838 as libc::c_int as u16_0,
         0x1839 as libc::c_int as u16_0, 0x183a as libc::c_int as u16_0,
         0x183b as libc::c_int as u16_0,
         (0x1850 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         (0x1851 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         0x5802 as libc::c_int as u16_0, 0x2801 as libc::c_int as u16_0,
         0x2802 as libc::c_int as u16_0, 0x2803 as libc::c_int as u16_0,
         0x2804 as libc::c_int as u16_0, 0x2805 as libc::c_int as u16_0,
         (0x2806 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         (0x2807 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         0x2809 as libc::c_int as u16_0, 0x2808 as libc::c_int as u16_0,
         (0x280a as libc::c_int - 0x800 as libc::c_int) as u16_0,
         (0x280b as libc::c_int - 0x800 as libc::c_int) as u16_0,
         (0x280b as libc::c_int - 0x800 as libc::c_int) as u16_0,
         (0x280c as libc::c_int - 0x800 as libc::c_int) as u16_0,
         (0x280d as libc::c_int - 0x800 as libc::c_int) as u16_0,
         0x280e as libc::c_int as u16_0, 0x280f as libc::c_int as u16_0,
         0x2810 as libc::c_int as u16_0, 0x2811 as libc::c_int as u16_0,
         0x2812 as libc::c_int as u16_0, 0x2813 as libc::c_int as u16_0,
         0x2814 as libc::c_int as u16_0, 0x2815 as libc::c_int as u16_0,
         0x2816 as libc::c_int as u16_0, 0x2817 as libc::c_int as u16_0,
         0x2817 as libc::c_int as u16_0, 0x2818 as libc::c_int as u16_0,
         0x2819 as libc::c_int as u16_0, 0x281a as libc::c_int as u16_0,
         0x281b as libc::c_int as u16_0, 0x281c as libc::c_int as u16_0,
         0x281d as libc::c_int as u16_0,
         (0x281e as libc::c_int - 0x800 as libc::c_int) as u16_0,
         0x281f as libc::c_int as u16_0, 0x2820 as libc::c_int as u16_0,
         (0x2821 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         0x2822 as libc::c_int as u16_0, 0x2823 as libc::c_int as u16_0,
         (0x2824 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         (0x2825 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         0x2826 as libc::c_int as u16_0,
         (0x2827 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         (0x2828 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         (0x2829 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         (0x282a as libc::c_int - 0x800 as libc::c_int) as u16_0,
         0x282b as libc::c_int as u16_0, 0x282c as libc::c_int as u16_0,
         0x282f as libc::c_int as u16_0, 0x282f as libc::c_int as u16_0,
         (0x2830 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         (0x2831 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         (0x2832 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         (0x2833 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         (0x2834 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         0x2835 as libc::c_int as u16_0,
         (0x2836 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         0x2837 as libc::c_int as u16_0, 0x2838 as libc::c_int as u16_0,
         0x2839 as libc::c_int as u16_0, 0x283a as libc::c_int as u16_0,
         (0x283b as libc::c_int - 0x800 as libc::c_int) as u16_0,
         0x283c as libc::c_int as u16_0, 0x283d as libc::c_int as u16_0,
         0x283e as libc::c_int as u16_0, 0x283f as libc::c_int as u16_0,
         (0x2840 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         (0x2841 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         0x2842 as libc::c_int as u16_0, 0x2843 as libc::c_int as u16_0,
         0x2844 as libc::c_int as u16_0, 0x2845 as libc::c_int as u16_0,
         0x2846 as libc::c_int as u16_0,
         (0x2847 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         0x2848 as libc::c_int as u16_0, 0x2830 as libc::c_int as u16_0,
         0x2831 as libc::c_int as u16_0,
         (0x800 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         (0x800 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         0x3800 as libc::c_int as u16_0, 0x3801 as libc::c_int as u16_0,
         (0x3802 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         0x3803 as libc::c_int as u16_0, 0x3804 as libc::c_int as u16_0,
         0x3805 as libc::c_int as u16_0, 0x3806 as libc::c_int as u16_0,
         0x3807 as libc::c_int as u16_0, 0x3808 as libc::c_int as u16_0,
         (0x3809 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         0x380a as libc::c_int as u16_0, 0x380b as libc::c_int as u16_0,
         0x380c as libc::c_int as u16_0, 0x380d as libc::c_int as u16_0,
         0x380e as libc::c_int as u16_0, 0x380f as libc::c_int as u16_0,
         0x3810 as libc::c_int as u16_0, 0x3811 as libc::c_int as u16_0,
         0x3812 as libc::c_int as u16_0, 0x3813 as libc::c_int as u16_0,
         0x3961 as libc::c_int as u16_0, 0x3962 as libc::c_int as u16_0,
         0x395c as libc::c_int as u16_0, 0x395d as libc::c_int as u16_0,
         0x395e as libc::c_int as u16_0, 0x395f as libc::c_int as u16_0,
         0x3960 as libc::c_int as u16_0, 0x381b as libc::c_int as u16_0,
         (0x381c as libc::c_int - 0x800 as libc::c_int) as u16_0,
         0x381d as libc::c_int as u16_0, 0x3820 as libc::c_int as u16_0,
         0x3821 as libc::c_int as u16_0, 0x3822 as libc::c_int as u16_0,
         0x3823 as libc::c_int as u16_0, 0x3824 as libc::c_int as u16_0,
         0x387b as libc::c_int as u16_0, 0x3829 as libc::c_int as u16_0,
         0x382a as libc::c_int as u16_0, 0x382b as libc::c_int as u16_0,
         0x382c as libc::c_int as u16_0, 0x382d as libc::c_int as u16_0,
         0x382e as libc::c_int as u16_0, 0x382f as libc::c_int as u16_0,
         0x3828 as libc::c_int as u16_0, 0x387a as libc::c_int as u16_0,
         0x3838 as libc::c_int as u16_0, 0x3839 as libc::c_int as u16_0,
         0x383a as libc::c_int as u16_0, 0x383b as libc::c_int as u16_0,
         0x386c as libc::c_int as u16_0, 0x383d as libc::c_int as u16_0,
         0x387a as libc::c_int as u16_0, 0x3840 as libc::c_int as u16_0,
         0x3841 as libc::c_int as u16_0, 0x3842 as libc::c_int as u16_0,
         0x3844 as libc::c_int as u16_0, 0x3845 as libc::c_int as u16_0,
         0x3846 as libc::c_int as u16_0, 0x3847 as libc::c_int as u16_0,
         0x3848 as libc::c_int as u16_0, 0x384c as libc::c_int as u16_0,
         0x384d as libc::c_int as u16_0,
         (0x384e as libc::c_int - 0x800 as libc::c_int) as u16_0,
         (0x384f as libc::c_int - 0x800 as libc::c_int) as u16_0,
         0x3850 as libc::c_int as u16_0,
         (0x3851 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         0x3852 as libc::c_int as u16_0, 0x3853 as libc::c_int as u16_0,
         (0x3854 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         0x3855 as libc::c_int as u16_0,
         (0x3858 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         0x3859 as libc::c_int as u16_0, 0x387a as libc::c_int as u16_0,
         0x385c as libc::c_int as u16_0, 0x385d as libc::c_int as u16_0,
         0x385e as libc::c_int as u16_0, 0x385f as libc::c_int as u16_0,
         0x3860 as libc::c_int as u16_0, 0x3861 as libc::c_int as u16_0,
         0x3862 as libc::c_int as u16_0, 0x387b as libc::c_int as u16_0,
         (0x3864 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         0x3865 as libc::c_int as u16_0, 0x3868 as libc::c_int as u16_0,
         0x3869 as libc::c_int as u16_0, 0x386a as libc::c_int as u16_0,
         0x386b as libc::c_int as u16_0, 0x386c as libc::c_int as u16_0,
         0x387b as libc::c_int as u16_0, 0x386e as libc::c_int as u16_0,
         0x386f as libc::c_int as u16_0, 0x3870 as libc::c_int as u16_0,
         (0x3871 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         (0x3872 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         0x3873 as libc::c_int as u16_0, 0x3874 as libc::c_int as u16_0,
         0x3875 as libc::c_int as u16_0, 0x3876 as libc::c_int as u16_0,
         0x3877 as libc::c_int as u16_0, 0x3878 as libc::c_int as u16_0,
         0x387c as libc::c_int as u16_0, 0x387d as libc::c_int as u16_0,
         0x387e as libc::c_int as u16_0, 0x387f as libc::c_int as u16_0,
         0x3880 as libc::c_int as u16_0, 0x3881 as libc::c_int as u16_0,
         0x3884 as libc::c_int as u16_0, 0x3885 as libc::c_int as u16_0,
         0x386d as libc::c_int as u16_0, 0x3890 as libc::c_int as u16_0,
         0x3891 as libc::c_int as u16_0, 0x3892 as libc::c_int as u16_0,
         0x3893 as libc::c_int as u16_0, 0x3894 as libc::c_int as u16_0,
         0x3895 as libc::c_int as u16_0, 0x3898 as libc::c_int as u16_0,
         0x3899 as libc::c_int as u16_0,
         (0x389a as libc::c_int - 0x800 as libc::c_int) as u16_0,
         0x38a1 as libc::c_int as u16_0, 0x38a1 as libc::c_int as u16_0,
         0x38a2 as libc::c_int as u16_0, 0x38a3 as libc::c_int as u16_0,
         (0x38a4 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         0x38a5 as libc::c_int as u16_0, 0x38a6 as libc::c_int as u16_0,
         (0x38a7 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         0x38a8 as libc::c_int as u16_0, 0x38a9 as libc::c_int as u16_0,
         0x38aa as libc::c_int as u16_0, 0x38ab as libc::c_int as u16_0,
         0x38ac as libc::c_int as u16_0, 0x38ad as libc::c_int as u16_0,
         0x38ae as libc::c_int as u16_0, 0x38af as libc::c_int as u16_0,
         0x38b0 as libc::c_int as u16_0, 0x38b1 as libc::c_int as u16_0,
         0x38b2 as libc::c_int as u16_0, 0x38b8 as libc::c_int as u16_0,
         0x38b9 as libc::c_int as u16_0, 0x38ba as libc::c_int as u16_0,
         0x38bb as libc::c_int as u16_0, 0x38bc as libc::c_int as u16_0,
         0x38bd as libc::c_int as u16_0, 0x38be as libc::c_int as u16_0,
         0x387e as libc::c_int as u16_0, 0x38c1 as libc::c_int as u16_0,
         0x38c2 as libc::c_int as u16_0, 0x38c3 as libc::c_int as u16_0,
         0x38c4 as libc::c_int as u16_0, 0x38c5 as libc::c_int as u16_0,
         (0x800 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         (0x800 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         (0x800 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         (0x800 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         (0x800 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         (0x800 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         (0x800 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         0x4800 as libc::c_int as u16_0, 0x4801 as libc::c_int as u16_0,
         0x4802 as libc::c_int as u16_0, 0x4803 as libc::c_int as u16_0,
         0x4804 as libc::c_int as u16_0, 0x4805 as libc::c_int as u16_0,
         0x4806 as libc::c_int as u16_0, 0x4807 as libc::c_int as u16_0,
         0x4807 as libc::c_int as u16_0, 0x4808 as libc::c_int as u16_0,
         0x4809 as libc::c_int as u16_0, 0x480a as libc::c_int as u16_0,
         0x480b as libc::c_int as u16_0, 0x480c as libc::c_int as u16_0,
         0x480c as libc::c_int as u16_0,
         (0x800 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         0x4830 as libc::c_int as u16_0, 0x4830 as libc::c_int as u16_0,
         0x480f as libc::c_int as u16_0, 0x4810 as libc::c_int as u16_0,
         0x4813 as libc::c_int as u16_0, 0x4814 as libc::c_int as u16_0,
         0x4817 as libc::c_int as u16_0, 0x4837 as libc::c_int as u16_0,
         0x4818 as libc::c_int as u16_0,
         (0x800 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         (0x800 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         (0x800 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         0x4823 as libc::c_int as u16_0, 0x4824 as libc::c_int as u16_0,
         0x4825 as libc::c_int as u16_0, 0x4826 as libc::c_int as u16_0,
         0x4827 as libc::c_int as u16_0, 0x4828 as libc::c_int as u16_0,
         0x4829 as libc::c_int as u16_0, 0x482a as libc::c_int as u16_0,
         0x482b as libc::c_int as u16_0, 0x480c as libc::c_int as u16_0,
         0x4837 as libc::c_int as u16_0, 0x5800 as libc::c_int as u16_0,
         (0x800 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         (0x820 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         0x6800 as libc::c_int as u16_0, 0x6800 as libc::c_int as u16_0,
         0x6800 as libc::c_int as u16_0, 0x6800 as libc::c_int as u16_0,
         0x6800 as libc::c_int as u16_0, 0x6800 as libc::c_int as u16_0,
         0x6800 as libc::c_int as u16_0, 0x6801 as libc::c_int as u16_0,
         0x6801 as libc::c_int as u16_0, 0x681c as libc::c_int as u16_0,
         0x6802 as libc::c_int as u16_0, 0x6803 as libc::c_int as u16_0,
         0x6814 as libc::c_int as u16_0, 0x6804 as libc::c_int as u16_0,
         0x6804 as libc::c_int as u16_0, 0x6804 as libc::c_int as u16_0,
         0x6804 as libc::c_int as u16_0, 0x6805 as libc::c_int as u16_0,
         0x6805 as libc::c_int as u16_0, 0x6808 as libc::c_int as u16_0,
         0x6807 as libc::c_int as u16_0, 0x6808 as libc::c_int as u16_0,
         0x6808 as libc::c_int as u16_0, 0x6809 as libc::c_int as u16_0,
         0x6809 as libc::c_int as u16_0, 0x680b as libc::c_int as u16_0,
         0x680c as libc::c_int as u16_0, 0x680d as libc::c_int as u16_0,
         0x6841 as libc::c_int as u16_0, 0x6842 as libc::c_int as u16_0,
         0x6840 as libc::c_int as u16_0, 0x6841 as libc::c_int as u16_0,
         0x6842 as libc::c_int as u16_0, 0x6840 as libc::c_int as u16_0,
         0x6850 as libc::c_int as u16_0,
         (0x38d0 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         0x38d1 as libc::c_int as u16_0, 0x38d2 as libc::c_int as u16_0,
         0x38d3 as libc::c_int as u16_0, 0x38d4 as libc::c_int as u16_0,
         0x38d5 as libc::c_int as u16_0, 0x38d6 as libc::c_int as u16_0,
         0x38d7 as libc::c_int as u16_0, 0x3927 as libc::c_int as u16_0,
         0x38d9 as libc::c_int as u16_0,
         (0x38da as libc::c_int - 0x800 as libc::c_int) as u16_0,
         (0x38db as libc::c_int - 0x800 as libc::c_int) as u16_0,
         0x3889 as libc::c_int as u16_0, 0x39ef as libc::c_int as u16_0,
         0x38e1 as libc::c_int as u16_0, 0x38e2 as libc::c_int as u16_0,
         0x38e4 as libc::c_int as u16_0, 0x38e5 as libc::c_int as u16_0,
         0x38e6 as libc::c_int as u16_0, 0x387a as libc::c_int as u16_0,
         0x38e7 as libc::c_int as u16_0, 0x38e8 as libc::c_int as u16_0,
         0x38ec as libc::c_int as u16_0, 0x38ed as libc::c_int as u16_0,
         0x38ee as libc::c_int as u16_0, 0x38ef as libc::c_int as u16_0,
         (0x38f0 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         (0x38f1 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         0x38f2 as libc::c_int as u16_0, 0x38f3 as libc::c_int as u16_0,
         0x39ec as libc::c_int as u16_0, 0x38f4 as libc::c_int as u16_0,
         (0x38f5 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         0x38f6 as libc::c_int as u16_0, 0x38f7 as libc::c_int as u16_0,
         0x38fc as libc::c_int as u16_0, 0x38fd as libc::c_int as u16_0,
         0x387b as libc::c_int as u16_0, 0x3900 as libc::c_int as u16_0,
         0x3901 as libc::c_int as u16_0, 0x3902 as libc::c_int as u16_0,
         (0x800 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         (0x800 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         (0x800 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         (0x800 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         (0x800 as libc::c_int - 0x800 as libc::c_int) as u16_0,
         (0x800 as libc::c_int - 0x800 as libc::c_int) as u16_0];
    if (*gGameInfo).data[(25 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 32 as libc::c_int) as usize]
           as libc::c_int != 0 as libc::c_int {
        (*gGameInfo).data[(25 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 32 as libc::c_int) as
                              usize] -= 1;
        Audio_QueueSeqCmd(((0x1 as libc::c_int) << 28 as libc::c_int |
                               (SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                                   24 as libc::c_int | 0x100ff as libc::c_int)
                              as u32_0);
        func_80078914(&mut zeroVec,
                      unkSfx[(*gGameInfo).data[(25 as libc::c_int *
                                                    6 as libc::c_int *
                                                    16 as libc::c_int +
                                                    33 as libc::c_int) as
                                                   usize] as usize]);
    }
    if (*gGameInfo).data[(25 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 34 as libc::c_int) as usize]
           as libc::c_int != 0 as libc::c_int {
        (*gGameInfo).data[(25 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 34 as libc::c_int) as
                              usize] = 0 as libc::c_int as s16;
        Audio_QueueSeqCmd(((SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                               24 as libc::c_int |
                               (*gGameInfo).data[(25 as libc::c_int *
                                                      6 as libc::c_int *
                                                      16 as libc::c_int +
                                                      35 as libc::c_int) as
                                                     usize] as u16_0 as
                                   libc::c_int) as u32_0);
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
             init.set_type_0(ICHAINTYPE_S8 as libc::c_int as u32_0);
             init.set_offset(&mut (*(0 as *mut Actor)).naviEnemyId as
                                 *mut u8_0 as size_t as u32_0);
             init.set_value(0x25 as libc::c_int);
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
