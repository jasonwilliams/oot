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
    fn func_8002836C(globalCtx: *mut GlobalContext, pos: *mut Vec3f,
                     velocity: *mut Vec3f, accel: *mut Vec3f,
                     primColor: *mut Color_RGBA8, envColor: *mut Color_RGBA8,
                     scale: s16, scaleStep: s16, life: s16);
    #[no_mangle]
    fn EffectSsGMagma_Spawn(globalCtx: *mut GlobalContext, pos: *mut Vec3f);
    #[no_mangle]
    fn EffectSsGMagma2_Spawn(globalCtx: *mut GlobalContext, pos: *mut Vec3f,
                             primColor: *mut Color_RGBA8,
                             envColor: *mut Color_RGBA8, updateRate: s16,
                             drawMode: s16, scale: s16);
    #[no_mangle]
    fn ActorShape_Init(shape: *mut ActorShape, yOffset: f32_0,
                       shadowDraw: ActorShadowFunc, shadowScale: f32_0);
    #[no_mangle]
    fn ActorShadow_DrawCircle(actor: *mut Actor, lights: *mut Lights,
                              globalCtx: *mut GlobalContext);
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
    fn Actor_WorldYawTowardActor(actorA: *mut Actor, actorB: *mut Actor)
     -> s16;
    #[no_mangle]
    fn func_8002DF54(globalCtx: *mut GlobalContext, actor: *mut Actor,
                     arg2: u8_0) -> s32;
    #[no_mangle]
    fn Actor_UpdateBgCheckInfo(globalCtx: *mut GlobalContext,
                               actor: *mut Actor, wallCheckHeight: f32_0,
                               wallCheckRadius: f32_0,
                               ceilingCheckHeight: f32_0, flags: s32);
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
    fn Actor_SpawnFloorDustRing(globalCtx: *mut GlobalContext,
                                actor: *mut Actor, posXZ: *mut Vec3f,
                                radius: f32_0, amountMinusOne: s32,
                                randAccelWeight: f32_0, scale: s16,
                                scaleStep: s16, useLighting: u8_0);
    #[no_mangle]
    fn func_80033480(globalCtx: *mut GlobalContext, posBase: *mut Vec3f,
                     randRangeDiameter: f32_0, amountMinusOne: s32,
                     scaleBase: s16, scaleStep: s16, arg6: u8_0);
    #[no_mangle]
    fn func_80033E88(actor: *mut Actor, globalCtx: *mut GlobalContext,
                     arg2: s16, arg3: s16);
    #[no_mangle]
    fn Rand_ZeroFloat(f: f32_0) -> f32_0;
    #[no_mangle]
    fn Rand_CenteredFloat(f: f32_0) -> f32_0;
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
    fn Collider_UpdateSpheres(limb: s32, collider: *mut ColliderJntSph);
    #[no_mangle]
    fn CollisionCheck_GetSwordDamage(dmgFlags: s32) -> u8_0;
    #[no_mangle]
    fn func_80064520(globalCtx: *mut GlobalContext,
                     csCtx: *mut CutsceneContext);
    #[no_mangle]
    fn func_80064534(globalCtx: *mut GlobalContext,
                     csCtx: *mut CutsceneContext);
    #[no_mangle]
    static mut gSegments: [u32_0; 16];
    #[no_mangle]
    static mut gGameInfo: *mut GameInfo;
    #[no_mangle]
    static mut gSaveContext: SaveContext;
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
    fn Math_SmoothStepToS(pValue: *mut s16, target: s16, scale: s16,
                          step: s16, minStep: s16) -> s16;
    #[no_mangle]
    fn func_80078884(sfxId: u16_0);
    #[no_mangle]
    fn Gfx_SetFog(gfx: *mut Gfx, r: s32, g: s32, b: s32, a: s32, near: s32,
                  far: s32) -> *mut Gfx;
    #[no_mangle]
    fn func_80093D18(gfxCtx: *mut GraphicsContext);
    #[no_mangle]
    fn func_80093D84(gfxCtx: *mut GraphicsContext);
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
    fn Animation_PlayLoop(skelAnime: *mut SkelAnime,
                          animation: *mut AnimationHeader);
    #[no_mangle]
    fn Animation_OnFrame(skelAnime: *mut SkelAnime, frame: f32_0) -> s32;
    #[no_mangle]
    fn SkelAnime_Free(skelAnime: *mut SkelAnime,
                      globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn func_800A9F6C(_: f32_0, _: u8_0, _: u8_0, _: u8_0);
    #[no_mangle]
    fn func_800D1FD4(mf: *mut MtxF);
    #[no_mangle]
    fn Matrix_MultVec3f(src: *mut Vec3f, dest: *mut Vec3f);
    #[no_mangle]
    fn Matrix_NewMtx(gfxCtx: *mut GraphicsContext, file: *mut libc::c_char,
                     line: s32) -> *mut Mtx;
    #[no_mangle]
    fn Matrix_TranslateRotateZYX(translation: *mut Vec3f,
                                 rotation: *mut Vec3s);
    #[no_mangle]
    fn Matrix_RotateZ(z: f32_0, mode: u8_0);
    #[no_mangle]
    fn Matrix_RotateY(y: f32_0, mode: u8_0);
    #[no_mangle]
    fn Matrix_RotateX(x: f32_0, mode: u8_0);
    #[no_mangle]
    fn Matrix_Scale(x: f32_0, y: f32_0, z: f32_0, mode: u8_0);
    #[no_mangle]
    fn Matrix_Translate(x: f32_0, y: f32_0, z: f32_0, mode: u8_0);
    #[no_mangle]
    fn Matrix_Pop();
    #[no_mangle]
    fn Matrix_Push();
    #[no_mangle]
    fn Graph_CloseDisps(dispRefs: *mut *mut Gfx, gfxCtx: *mut GraphicsContext,
                        file: *const libc::c_char, line: s32);
    #[no_mangle]
    fn Graph_OpenDisps(dispRefs: *mut *mut Gfx, gfxCtx: *mut GraphicsContext,
                       file: *const libc::c_char, line: s32);
    #[no_mangle]
    fn func_800C08AC(globalCtx: *mut GlobalContext, camId: s16, arg2: s16);
    #[no_mangle]
    fn Gameplay_CameraSetAtEyeUp(globalCtx: *mut GlobalContext, camId: s16,
                                 at: *mut Vec3f, eye: *mut Vec3f,
                                 up: *mut Vec3f) -> s32;
    #[no_mangle]
    fn Gameplay_CameraSetAtEye(globalCtx: *mut GlobalContext, camId: s16,
                               at: *mut Vec3f, eye: *mut Vec3f) -> s32;
    #[no_mangle]
    fn Gameplay_GetCamera(globalCtx: *mut GlobalContext, camId: s16)
     -> *mut Camera;
    #[no_mangle]
    fn Gameplay_ClearAllSubCameras(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn Gameplay_ChangeCameraStatus(globalCtx: *mut GlobalContext, camId: s16,
                                   status: s16) -> s16;
    #[no_mangle]
    fn Gameplay_CreateSubCamera(globalCtx: *mut GlobalContext) -> s16;
    #[no_mangle]
    fn Gameplay_SetFog(globalCtx: *mut GlobalContext, gfx: *mut Gfx)
     -> *mut Gfx;
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
    static mut object_kingdodongo_Anim_001074: AnimationHeader;
    #[no_mangle]
    static mut object_kingdodongo_Anim_002D0C: AnimationHeader;
    #[no_mangle]
    static mut object_kingdodongo_Anim_003CF8: AnimationHeader;
    #[no_mangle]
    static mut object_kingdodongo_Anim_0042A8: AnimationHeader;
    #[no_mangle]
    static mut object_kingdodongo_Anim_004E0C: AnimationHeader;
    #[no_mangle]
    static mut object_kingdodongo_Anim_0061D4: AnimationHeader;
    #[no_mangle]
    static mut object_kingdodongo_Anim_008EEC: AnimationHeader;
    #[no_mangle]
    static mut object_kingdodongo_Anim_009D10: AnimationHeader;
    #[no_mangle]
    static mut object_kingdodongo_DL_009D50: [Gfx; 0];
    #[no_mangle]
    static mut object_kingdodongo_DL_009DD0: [Gfx; 0];
    #[no_mangle]
    static mut object_kingdodongo_Anim_00DF38: AnimationHeader;
    #[no_mangle]
    static mut object_kingdodongo_Anim_00E848: AnimationHeader;
    #[no_mangle]
    static mut object_kingdodongo_Anim_00F0D8: AnimationHeader;
    #[no_mangle]
    static mut object_kingdodongo_Tex_015890: [u64_0; 0];
    #[no_mangle]
    static mut object_kingdodongo_Tex_015990: [u64_0; 0];
    #[no_mangle]
    static mut object_kingdodongo_Tex_015D90: [u64_0; 0];
    #[no_mangle]
    static mut object_kingdodongo_Tex_015F90: [u64_0; 0];
    #[no_mangle]
    static mut object_kingdodongo_Tex_016390: [u64_0; 0];
    #[no_mangle]
    static mut object_kingdodongo_Tex_016590: [u64_0; 0];
    #[no_mangle]
    static mut object_kingdodongo_Tex_016790: [u64_0; 0];
    #[no_mangle]
    static mut object_kingdodongo_Tex_016990: [u64_0; 0];
    #[no_mangle]
    static mut object_kingdodongo_Tex_016E10: [u64_0; 0];
    #[no_mangle]
    static mut object_kingdodongo_Tex_017210: [u64_0; 0];
    #[no_mangle]
    static mut object_kingdodongo_Blob_017410: [u8_0; 0];
    #[no_mangle]
    static mut object_kingdodongo_Skel_01B310: SkeletonHeader;
    #[no_mangle]
    static mut object_kingdodongo_Anim_01CAE0: AnimationHeader;
    #[no_mangle]
    static mut object_kingdodongo_Anim_01D934: AnimationHeader;
    #[no_mangle]
    static mut gDodongosCavernBossLavaFloorTex: [u64_0; 0];
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
pub type C2RustUnnamed_23 = libc::c_uint;
pub const MTXMODE_APPLY: C2RustUnnamed_23 = 1;
pub const MTXMODE_NEW: C2RustUnnamed_23 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BossDodongo {
    pub actor: Actor,
    pub skelAnime: SkelAnime,
    pub actionFunc: BossDodongoActionFunc,
    pub health: s16,
    pub unk_196: s16,
    pub unk_198: s16,
    pub unk_19A: s16,
    pub csState: s16,
    pub unk_19E: s16,
    pub unk_1A0: s16,
    pub unk_1A2: s16,
    pub unk_1A4: s16,
    pub unk_1A6: s16,
    pub numWallCollisions: s16,
    pub unk_1AA: s16,
    pub unk_1AC: s16,
    pub unk_1AE: s16,
    pub unk_1B0: s16,
    pub unk_1B2: [libc::c_char; 2],
    pub cutsceneCamera: s16,
    pub unk_1B6: s16,
    pub playerYawInRange: s16,
    pub playerPosInRange: s16,
    pub unk_1BC: s16,
    pub unk_1BE: s16,
    pub unk_1C0: s16,
    pub unk_1C2: s16,
    pub unk_1C4: s16,
    pub unk_1C6: s16,
    pub unk_1C8: s16,
    pub unk_1CA: [libc::c_char; 2],
    pub unk_1CC: s16,
    pub unk_1CE: [libc::c_char; 12],
    pub unk_1DA: s16,
    pub unk_1DC: s16,
    pub unk_1DE: s16,
    pub unk_1E0: s16,
    pub unk_1E2: u8_0,
    pub unk_1E3: s8,
    pub unk_1E4: f32_0,
    pub unk_1E8: f32_0,
    pub unk_1EC: f32_0,
    pub unk_1F0: [libc::c_char; 8],
    pub unk_1F8: f32_0,
    pub unk_1FC: f32_0,
    pub unk_200: f32_0,
    pub unk_204: f32_0,
    pub unk_208: f32_0,
    pub unk_20C: f32_0,
    pub colorFilterR: f32_0,
    pub colorFilterG: f32_0,
    pub colorFilterB: f32_0,
    pub colorFilterMin: f32_0,
    pub colorFilterMax: f32_0,
    pub unk_224: f32_0,
    pub unk_228: f32_0,
    pub unk_22C: f32_0,
    pub unk_230: f32_0,
    pub unk_234: f32_0,
    pub unk_238: f32_0,
    pub unk_23C: f32_0,
    pub unk_240: f32_0,
    pub unk_244: f32_0,
    pub unk_248: [libc::c_char; 20],
    pub unk_25C: [f32_0; 50],
    pub unk_324: [f32_0; 50],
    pub vec: Vec3f,
    pub firePos: Vec3f,
    pub unk_404: Vec3f,
    pub unk_410: Vec3f,
    pub mouthPos: Vec3f,
    pub cameraEye: Vec3f,
    pub cameraAt: Vec3f,
    pub collider: ColliderJntSph,
    pub items: [ColliderJntSphElement; 19],
    pub effects: [BossDodongoEffect; 80],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BossDodongoEffect {
    pub unk_00: Vec3f,
    pub unk_0C: Vec3f,
    pub unk_18: Vec3f,
    pub unk_24: u8_0,
    pub unk_25: u8_0,
    pub color: Color_RGB8,
    pub alpha: s16,
    pub unk_2C: f32_0,
}
pub type BossDodongoActionFunc
    =
    Option<unsafe extern "C" fn(_: *mut BossDodongo, _: *mut GlobalContext)
               -> ()>;
pub type C2RustUnnamed_24 = libc::c_int;
pub const WARP_RED: C2RustUnnamed_24 = 10;
pub const WARP_GREEN: C2RustUnnamed_24 = 9;
pub const WARP_ORANGE: C2RustUnnamed_24 = 8;
pub const WARP_UNK_7: C2RustUnnamed_24 = 7;
pub const WARP_DESTINATION: C2RustUnnamed_24 = 6;
pub const WARP_BLUE_RUTO: C2RustUnnamed_24 = 5;
pub const WARP_YELLOW: C2RustUnnamed_24 = 4;
pub const WARP_PURPLE_CRYSTAL: C2RustUnnamed_24 = 3;
pub const WARP_SAGES: C2RustUnnamed_24 = 2;
pub const WARP_CLEAR_FLAG: C2RustUnnamed_24 = 1;
pub const WARP_DUNGEON_CHILD: C2RustUnnamed_24 = 0;
pub const WARP_DUNGEON_ADULT: C2RustUnnamed_24 = -1;
pub const WARP_BLUE_CRYSTAL: C2RustUnnamed_24 = -2;
#[no_mangle]
pub static mut Boss_Dodongo_InitVars: ActorInit =
    unsafe {
        {
            let mut init =
                ActorInit{id: ACTOR_EN_DODONGO as libc::c_int as s16,
                          category: ACTORCAT_BOSS as libc::c_int as u8_0,
                          flags:
                              ((1 as libc::c_int) << 0 as libc::c_int |
                                   (1 as libc::c_int) << 2 as libc::c_int |
                                   (1 as libc::c_int) << 4 as libc::c_int |
                                   (1 as libc::c_int) << 5 as libc::c_int) as
                                  u32_0,
                          objectId: OBJECT_KINGDODONGO as libc::c_int as s16,
                          instanceSize:
                              ::std::mem::size_of::<BossDodongo>() as
                                  libc::c_ulong,
                          init:
                              ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                      *mut Actor,
                                                                                  _:
                                                                                      *mut GlobalContext)
                                                                 -> ()>,
                                                      ActorFunc>(Some(BossDodongo_Init
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
                                                      ActorFunc>(Some(BossDodongo_Destroy
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
                                                      ActorFunc>(Some(BossDodongo_Update
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
                                                      ActorFunc>(Some(BossDodongo_Draw
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
static mut sLavaFloorRockTex: [u64_0; 512] =
    [0x48c5418b41cb2041 as libc::c_longlong as u64_0,
     0x314741cd394739c9 as libc::c_longlong as u64_0,
     0x398729014a8f4a8f as libc::c_longlong as u64_0,
     0x494159836a496a47 as libc::c_longlong as u64_0,
     0x514159c18b517b0d as libc::c_longlong as u64_0,
     0x39c939852041420b as libc::c_longlong as u64_0,
     0x7a8d594540c34041 as libc::c_longlong as u64_0,
     0x3041280128413001 as libc::c_longlong as u64_0,
     0x4949594748834105 as libc::c_longlong as u64_0,
     0x698b824d59456183 as libc::c_longlong as u64_0,
     0x8a47a309c3cd8a87 as libc::c_ulonglong,
     0x404148c1510548c3 as libc::c_longlong as u64_0,
     0x38011843321129cf as libc::c_longlong as u64_0,
     0x84130c1284120c7 as libc::c_longlong as u64_0,
     0x3885380128011801 as libc::c_longlong as u64_0,
     0x2001200138414001 as libc::c_longlong as u64_0,
     0x50c7494750c56149 as libc::c_longlong as u64_0,
     0x7a0f59cb388169cb as libc::c_longlong as u64_0,
     0x9acf92cf7a8b5103 as libc::c_ulonglong,
     0x2001100120012001 as libc::c_longlong as u64_0,
     0x28014885494b414b as libc::c_longlong as u64_0,
     0x2001404140413841 as libc::c_longlong as u64_0,
     0x3001200118012001 as libc::c_longlong as u64_0,
     0x2001304338414885 as libc::c_longlong as u64_0,
     0x388348c550c7820f as libc::c_longlong as u64_0,
     0x8a51518b388169c9 as libc::c_ulonglong,
     0x7a0d594751014081 as libc::c_longlong as u64_0,
     0x1001180130013001 as libc::c_longlong as u64_0,
     0x1801180120412801 as libc::c_longlong as u64_0,
     0x2001200118011001 as libc::c_longlong as u64_0,
     0x801080118411041 as libc::c_longlong as u64_0,
     0x1803204118412001 as libc::c_longlong as u64_0,
     0x2001204130435109 as libc::c_longlong as u64_0,
     0x3885388328813885 as libc::c_longlong as u64_0,
     0x5103490359052841 as libc::c_longlong as u64_0,
     0x1001300140013001 as libc::c_longlong as u64_0,
     0x3041080108011001 as libc::c_longlong as u64_0,
     0x1801180120411843 as libc::c_longlong as u64_0,
     0x20413083308338c3 as libc::c_longlong as u64_0,
     0x5107514730832041 as libc::c_longlong as u64_0,
     0x1001100110011801 as libc::c_longlong as u64_0,
     0x3041388530832883 as libc::c_longlong as u64_0,
     0x50c5514740c51001 as libc::c_longlong as u64_0,
     0x2001380140834083 as libc::c_longlong as u64_0,
     0x2041100120413883 as libc::c_longlong as u64_0,
     0x4083304330014081 as libc::c_longlong as u64_0,
     0x5949598759cd5989 as libc::c_longlong as u64_0,
     0x49476a4d59854883 as libc::c_longlong as u64_0,
     0x1801100110011001 as libc::c_longlong as u64_0,
     0x2001300148c53885 as libc::c_longlong as u64_0,
     0x5107514928431801 as libc::c_longlong as u64_0,
     0x2801400148c73083 as libc::c_longlong as u64_0,
     0x1001084120012801 as libc::c_longlong as u64_0,
     0x4081280128013083 as libc::c_longlong as u64_0,
     0x5105594959894105 as libc::c_longlong as u64_0,
     0x5147598751454083 as libc::c_longlong as u64_0,
     0x1801200128013001 as libc::c_longlong as u64_0,
     0x1801180128011001 as libc::c_longlong as u64_0,
     0x2001200118012001 as libc::c_longlong as u64_0,
     0x3001408540832001 as libc::c_longlong as u64_0,
     0x1001080110011001 as libc::c_longlong as u64_0,
     0x1801100110011801 as libc::c_longlong as u64_0,
     0x3841510740815147 as libc::c_longlong as u64_0,
     0x59cb490551052801 as libc::c_longlong as u64_0,
     0x3041304338c34105 as libc::c_longlong as u64_0,
     0x3083200118012001 as libc::c_longlong as u64_0,
     0x3001380130012001 as libc::c_longlong as u64_0,
     0x1801204118011801 as libc::c_longlong as u64_0,
     0x1001180118010801 as libc::c_longlong as u64_0,
     0x801000108011801 as libc::c_longlong as u64_0,
     0x300151075147620d as libc::c_longlong as u64_0,
     0x69cd61cb40832801 as libc::c_longlong as u64_0,
     0x510569c569c748c3 as libc::c_longlong as u64_0,
     0x51454881384140c5 as libc::c_longlong as u64_0,
     0x720f48c738013841 as libc::c_longlong as u64_0,
     0x1001080108011001 as libc::c_longlong as u64_0,
     0x384348c538013001 as libc::c_longlong as u64_0,
     0x1001000108011801 as libc::c_longlong as u64_0,
     0x28415909514548c5 as libc::c_longlong as u64_0,
     0x518b48c738013041 as libc::c_longlong as u64_0,
     0x59455107510340c1 as libc::c_longlong as u64_0,
     0x50c5510351456189 as libc::c_longlong as u64_0,
     0x4905388330812841 as libc::c_longlong as u64_0,
     0x1801080108012843 as libc::c_longlong as u64_0,
     0x6a0b7a0b30414881 as libc::c_longlong as u64_0,
     0x2843200108012001 as libc::c_longlong as u64_0,
     0x2801408540413801 as libc::c_longlong as u64_0,
     0x5087300128013841 as libc::c_longlong as u64_0,
     0x4001484340014041 as libc::c_longlong as u64_0,
     0x50c5488530015105 as libc::c_longlong as u64_0,
     0x38c538c138812883 as libc::c_longlong as u64_0,
     0x1801100108012083 as libc::c_longlong as u64_0,
     0x71875181180159c9 as libc::c_longlong as u64_0,
     0x5107404130013001 as libc::c_longlong as u64_0,
     0x2801200120012001 as libc::c_longlong as u64_0,
     0x2001200128014041 as libc::c_longlong as u64_0,
     0x388338c720013043 as libc::c_longlong as u64_0,
     0x498b384538014843 as libc::c_longlong as u64_0,
     0x40c5384148c348c5 as libc::c_longlong as u64_0,
     0x2883100110012841 as libc::c_longlong as u64_0,
     0x514150c169c19b93 as libc::c_longlong as u64_0,
     0x49cb38c161875149 as libc::c_longlong as u64_0,
     0x4083280110010801 as libc::c_longlong as u64_0,
     0x1001204130455945 as libc::c_longlong as u64_0,
     0x79c7720582077a45 as libc::c_longlong as u64_0,
     0x79c5514130014887 as libc::c_longlong as u64_0,
     0x4083400130413883 as libc::c_longlong as u64_0,
     0x3883184118012001 as libc::c_longlong as u64_0,
     0x49038a43dc8fbc13 as libc::c_longlong as u64_0,
     0x51815a057a0b5101 as libc::c_longlong as u64_0,
     0x4081300110011001 as libc::c_longlong as u64_0,
     0x204140c772096987 as libc::c_longlong as u64_0,
     0xa34bb38be4d1cc4b as libc::c_ulonglong,
     0xcc0d69c169c551cb as libc::c_ulonglong,
     0x28014001388348c3 as libc::c_longlong as u64_0,
     0x48c5388318412001 as libc::c_longlong as u64_0,
     0x408341859ac98247 as libc::c_longlong as u64_0,
     0x69c37a0769894903 as libc::c_longlong as u64_0,
     0x5105304118012801 as libc::c_longlong as u64_0,
     0x38c5618959877a89 as libc::c_longlong as u64_0,
     0x72cb72cb5a035181 as libc::c_longlong as u64_0,
     0x7a4561818a4b728f as libc::c_longlong as u64_0,
     0x1043514b38433841 as libc::c_longlong as u64_0,
     0x488140c130812001 as libc::c_longlong as u64_0,
     0x3001380161c38ac9 as libc::c_longlong as u64_0,
     0x820771c972095143 as libc::c_ulonglong,
     0x6a0d3843200148c7 as libc::c_longlong as u64_0,
     0x510361cb59855187 as libc::c_longlong as u64_0,
     0x6a475a0349414981 as libc::c_longlong as u64_0,
     0x498149838a8d51c9 as libc::c_longlong as u64_0,
     0x5987418d40014001 as libc::c_longlong as u64_0,
     0x48c1594549052001 as libc::c_longlong as u64_0,
     0x80138837a4b7acd as libc::c_longlong as u64_0,
     0x4941494140c148c1 as libc::c_longlong as u64_0,
     0x59473001280148c5 as libc::c_longlong as u64_0,
     0x510161876a0951c7 as libc::c_longlong as u64_0,
     0x72cb6a8b5a476247 as libc::c_longlong as u64_0,
     0x7b5372cf7a096189 as libc::c_longlong as u64_0,
     0x620b20c738014001 as libc::c_longlong as u64_0,
     0x48c7510338c31001 as libc::c_longlong as u64_0,
     0x1200151076a49 as libc::c_longlong as u64_0,
     0x6187724d69c94145 as libc::c_longlong as u64_0,
     0x4883380140414905 as libc::c_longlong as u64_0,
     0x61857209824982cb as libc::c_longlong as u64_0,
     0x624751c1520351c3 as libc::c_longlong as u64_0,
     0x73115a0940c15a0b as libc::c_longlong as u64_0,
     0x4a112881280150c7 as libc::c_longlong as u64_0,
     0x4843404338431801 as libc::c_longlong as u64_0,
     0x801180138012801 as libc::c_longlong as u64_0,
     0x300130c730013801 as libc::c_longlong as u64_0,
     0x4001404351057a4d as libc::c_longlong as u64_0,
     0x7a497a497a078a89 as libc::c_longlong as u64_0,
     0x620359c15a4551c5 as libc::c_longlong as u64_0,
     0x7b535185280159cb as libc::c_longlong as u64_0,
     0x4a0f180138c35907 as libc::c_longlong as u64_0,
     0x4001384330031801 as libc::c_longlong as u64_0,
     0x1100138014043 as libc::c_longlong as u64_0,
     0x4885400140014001 as libc::c_longlong as u64_0,
     0x488340c551038207 as libc::c_longlong as u64_0,
     0x720769c361c169c1 as libc::c_longlong as u64_0,
     0x624559c362cb5a07 as libc::c_longlong as u64_0,
     0x72cf620930416a0f as libc::c_longlong as u64_0,
     0x4a51180149015947 as libc::c_longlong as u64_0,
     0x48c1404138411801 as libc::c_longlong as u64_0,
     0x801100148c54905 as libc::c_longlong as u64_0,
     0x618b5989590759c9 as libc::c_longlong as u64_0,
     0x5105594369838249 as libc::c_longlong as u64_0,
     0x724569816a037a87 as libc::c_longlong as u64_0,
     0x520939c952d15a49 as libc::c_longlong as u64_0,
     0x59c759c940c1620d as libc::c_longlong as u64_0,
     0x41cd1001620d820b as libc::c_longlong as u64_0,
     0x69c540c120410801 as libc::c_longlong as u64_0,
     0x1180148c35941 as libc::c_longlong as u64_0,
     0x698559c541476a07 as libc::c_longlong as u64_0,
     0x594161417a4992cf as libc::c_longlong as u64_0,
     0x69c561817a497247 as libc::c_longlong as u64_0,
     0x420b320f4a4f4141 as libc::c_longlong as u64_0,
     0x494149435145620f as libc::c_longlong as u64_0,
     0x3949294941cd59c7 as libc::c_longlong as u64_0,
     0x594348c318410001 as libc::c_longlong as u64_0,
     0x100140c569c5ab0b as libc::c_longlong as u64_0,
     0xc40f71c369836181 as libc::c_ulonglong,
     0x8a838a47824992cf as libc::c_ulonglong,
     0x51017a497acb7a8d as libc::c_longlong as u64_0,
     0x528f528f5a4738c1 as libc::c_longlong as u64_0,
     0x51856a0b72916a0d as libc::c_longlong as u64_0,
     0x398b4251314740c1 as libc::c_longlong as u64_0,
     0x5103404120011001 as libc::c_longlong as u64_0,
     0x2001598772056a49 as libc::c_longlong as u64_0,
     0x7a8761418a8171c3 as libc::c_longlong as u64_0,
     0x82877207828d9311 as libc::c_ulonglong,
     0x5901828d8acf728b as libc::c_longlong as u64_0,
     0x4a09528f6a896207 as libc::c_longlong as u64_0,
     0x831382d3620b5145 as libc::c_ulonglong,
     0x28c541cd310561c9 as libc::c_longlong as u64_0,
     0x5947304118011001 as libc::c_longlong as u64_0,
     0x3883828b71c351c5 as libc::c_longlong as u64_0,
     0x51c351c561c572cd as libc::c_longlong as u64_0,
     0x7a8d62057a4b71c9 as libc::c_longlong as u64_0,
     0x59417a4d720969c5 as libc::c_longlong as u64_0,
     0x49c749c749c34981 as libc::c_longlong as u64_0,
     0x7a4f598540c15145 as libc::c_longlong as u64_0,
     0x310730c528815145 as libc::c_longlong as u64_0,
     0x4881280110412001 as libc::c_longlong as u64_0,
     0x48c3518530815a07 as libc::c_longlong as u64_0,
     0x8353624962098b53 as libc::c_ulonglong,
     0x830f61c76a096185 as libc::c_ulonglong,
     0x6187824d59856a09 as libc::c_longlong as u64_0,
     0x72438a89720359c5 as libc::c_longlong as u64_0,
     0x724d514359856a0b as libc::c_longlong as u64_0,
     0x30c53945418969cb as libc::c_longlong as u64_0,
     0x4083200118013001 as libc::c_longlong as u64_0,
     0x38414101494172d1 as libc::c_longlong as u64_0,
     0x7b115185728d730f as libc::c_longlong as u64_0,
     0x5183490149015943 as libc::c_longlong as u64_0,
     0x69c7618559c569c7 as libc::c_longlong as u64_0,
     0x82879acb8a8b61c5 as libc::c_ulonglong,
     0x6a09598561476187 as libc::c_longlong as u64_0,
     0x61cb71cb5a0d69c9 as libc::c_longlong as u64_0,
     0x40c3180110011801 as libc::c_longlong as u64_0,
     0x4043514569c97ad1 as libc::c_longlong as u64_0,
     0x5a0940c1620951c5 as libc::c_longlong as u64_0,
     0x490151c55983724d as libc::c_longlong as u64_0,
     0x724b69856a077207 as libc::c_longlong as u64_0,
     0x69c97a8b7a4b5101 as libc::c_longlong as u64_0,
     0x6147598569894905 as libc::c_longlong as u64_0,
     0x510548c3490748c3 as libc::c_longlong as u64_0,
     0x2041100100010001 as libc::c_longlong as u64_0,
     0x1801284140415949 as libc::c_longlong as u64_0,
     0x50c130016209624b as libc::c_longlong as u64_0,
     0x61c96a8b7a4b5a51 as libc::c_longlong as u64_0,
     0x294940c939075987 as libc::c_longlong as u64_0,
     0x494b510940853001 as libc::c_longlong as u64_0,
     0x4885388540853801 as libc::c_longlong as u64_0,
     0x4001300128011001 as libc::c_longlong as u64_0,
     0x1001080100010801 as libc::c_longlong as u64_0,
     0x1180128013801 as libc::c_longlong as u64_0,
     0x4001280139cf18c7 as libc::c_longlong as u64_0,
     0x394751c930854087 as libc::c_longlong as u64_0,
     0x4843304130012801 as libc::c_longlong as u64_0,
     0x2801200120011801 as libc::c_longlong as u64_0,
     0x2001200118012801 as libc::c_longlong as u64_0,
     0x2001200118011801 as libc::c_longlong as u64_0,
     0x1801200120010801 as libc::c_longlong as u64_0,
     0x801080118012001 as libc::c_longlong as u64_0,
     0x2801384340433801 as libc::c_longlong as u64_0,
     0x4085408540013001 as libc::c_longlong as u64_0,
     0x2001100118011801 as libc::c_longlong as u64_0,
     0x2001180110011001 as libc::c_longlong as u64_0,
     0x1001100118013043 as libc::c_longlong as u64_0,
     0x3841408140014881 as libc::c_longlong as u64_0,
     0x48c5404148c52043 as libc::c_longlong as u64_0,
     0x1001100130013043 as libc::c_longlong as u64_0,
     0x1801080120012001 as libc::c_longlong as u64_0,
     0x2801200120011001 as libc::c_longlong as u64_0,
     0x8011801384148c3 as libc::c_longlong as u64_0,
     0x50c538c128431843 as libc::c_longlong as u64_0,
     0x1841104120412843 as libc::c_longlong as u64_0,
     0x38c1614559c98acf as libc::c_longlong as u64_0,
     0x824b594549052041 as libc::c_ulonglong,
     0x1001100120412885 as libc::c_longlong as u64_0,
     0x1841100108011801 as libc::c_longlong as u64_0,
     0x1801180110011801 as libc::c_longlong as u64_0,
     0x2881594782458a47 as libc::c_longlong as u64_0,
     0x41435985614938c3 as libc::c_longlong as u64_0,
     0x30833085288138c5 as libc::c_longlong as u64_0,
     0x5945824d5a4d7249 as libc::c_longlong as u64_0,
     0x82077a038a8b4903 as libc::c_ulonglong,
     0x20011001288348c5 as libc::c_longlong as u64_0,
     0x5143514540c54081 as libc::c_longlong as u64_0,
     0x4841514751038245 as libc::c_longlong as u64_0,
     0xab49c409a381a341 as libc::c_ulonglong,
     0x49c749cb724b5907 as libc::c_longlong as u64_0,
     0x510748c538815909 as libc::c_longlong as u64_0,
     0x5987510730814943 as libc::c_longlong as u64_0,
     0x6a0769c57acf61c7 as libc::c_longlong as u64_0,
     0x4885200140834949 as libc::c_longlong as u64_0,
     0x5985514151412881 as libc::c_longlong as u64_0,
     0x828d9acd92898287 as libc::c_ulonglong,
     0x6a4551c151c15a45 as libc::c_longlong as u64_0,
     0x4a07318759476189 as libc::c_longlong as u64_0,
     0x698b510748c55149 as libc::c_longlong as u64_0,
     0x598d388348815905 as libc::c_longlong as u64_0,
     0x61856183728b7185 as libc::c_longlong as u64_0,
     0x40811801200148c5 as libc::c_longlong as u64_0,
     0x48c35983624941c7 as libc::c_longlong as u64_0,
     0x7a8d92cd820559c1 as libc::c_longlong as u64_0,
     0x6a8762475a455207 as libc::c_longlong as u64_0,
     0x418539855143418b as libc::c_longlong as u64_0,
     0x5211408540433801 as libc::c_longlong as u64_0,
     0x40434001400150c5 as libc::c_longlong as u64_0,
     0x40c52801410961c5 as libc::c_longlong as u64_0,
     0x4901284320412001 as libc::c_longlong as u64_0,
     0x2001408182cf528f as libc::c_longlong as u64_0,
     0x3987620751815a01 as libc::c_longlong as u64_0,
     0x7acd6a87520749c5 as libc::c_longlong as u64_0,
     0x4187420b2903310b as libc::c_longlong as u64_0,
     0x5149380120012001 as libc::c_longlong as u64_0,
     0x4001488148c3590b as libc::c_longlong as u64_0,
     0x4083380148874083 as libc::c_longlong as u64_0,
     0x300150c528411801 as libc::c_longlong as u64_0,
     0x801180161874187 as libc::c_longlong as u64_0,
     0x28c161c55a036a49 as libc::c_longlong as u64_0,
     0x6a4751c15a0559c7 as libc::c_longlong as u64_0,
     0x494551c949475109 as libc::c_longlong as u64_0,
     0x3001180120013001 as libc::c_longlong as u64_0,
     0x48c3824f5a0d5945 as libc::c_longlong as u64_0,
     0x4881410161474041 as libc::c_longlong as u64_0,
     0x4001488530412001 as libc::c_longlong as u64_0,
     0x801100140413841 as libc::c_longlong as u64_0,
     0x200169c77a8b7249 as libc::c_longlong as u64_0,
     0x61c5598151415141 as libc::c_longlong as u64_0,
     0x50c55a0f40433001 as libc::c_longlong as u64_0,
     0x1801200140c55949 as libc::c_longlong as u64_0,
     0x6989720d28013081 as libc::c_longlong as u64_0,
     0x618582059acb5983 as libc::c_longlong as u64_0,
     0x5945698740c32841 as libc::c_longlong as u64_0,
     0x1001100140013801 as libc::c_longlong as u64_0,
     0x40c7594959474081 as libc::c_longlong as u64_0,
     0x3881384130413881 as libc::c_longlong as u64_0,
     0x2841280120011801 as libc::c_longlong as u64_0,
     0x280148c57a4f6a0b as libc::c_longlong as u64_0,
     0x618959435945198b as libc::c_longlong as u64_0,
     0x29cf424b8a898a47 as libc::c_longlong as u64_0,
     0xab49a30559433883 as libc::c_ulonglong,
     0x1001100128014041 as libc::c_longlong as u64_0,
     0x3843300120011001 as libc::c_longlong as u64_0,
     0x1001100110011001 as libc::c_longlong as u64_0,
     0x1801180120012801 as libc::c_longlong as u64_0,
     0x5107598761455945 as libc::c_longlong as u64_0,
     0x514579c97a8f41c9 as libc::c_longlong as u64_0,
     0x4a91830f6a475a47 as libc::c_longlong as u64_0,
     0x51c16a8b72494041 as libc::c_longlong as u64_0,
     0x18010801204138c5 as libc::c_longlong as u64_0,
     0x3001200118011801 as libc::c_longlong as u64_0,
     0x1801100110011001 as libc::c_longlong as u64_0,
     0x2001280140c57a0d as libc::c_longlong as u64_0,
     0x8a8d618371c9828d as libc::c_ulonglong,
     0x6a0b728d20c12041 as libc::c_longlong as u64_0,
     0x39cb728951816acd as libc::c_longlong as u64_0,
     0x728b7a8d59434001 as libc::c_longlong as u64_0,
     0x18010801204338c1 as libc::c_longlong as u64_0,
     0x38c3388330013801 as libc::c_longlong as u64_0,
     0x488548c3490750c5 as libc::c_longlong as u64_0,
     0x61cb7a0b930b928b as libc::c_longlong as u64_0,
     0xa30771c5a35159c9 as libc::c_ulonglong,
     0x59c76a8b20c1398b as libc::c_longlong as u64_0,
     0x428f59c5518151cb as libc::c_longlong as u64_0,
     0x3907384540413001 as libc::c_longlong as u64_0,
     0x1001080118413081 as libc::c_longlong as u64_0,
     0x40c3618740814041 as libc::c_longlong as u64_0,
     0x59c951c949cb4987 as libc::c_longlong as u64_0,
     0x314329457a4769c1 as libc::c_longlong as u64_0,
     0x69c151c55a4938c1 as libc::c_longlong as u64_0,
     0x29473a0d08c79395 as libc::c_longlong as u64_0,
     0x7b0f69c551c940c7 as libc::c_longlong as u64_0,
     0x4001280128012001 as libc::c_longlong as u64_0,
     0x801080110012801 as libc::c_longlong as u64_0,
     0x3083594361435983 as libc::c_longlong as u64_0,
     0x418539474a0b3987 as libc::c_longlong as u64_0,
     0x2081000100851881 as libc::c_longlong as u64_0,
     0x414141015a4b628d as libc::c_longlong as u64_0,
     0x294700c700015181 as libc::c_longlong as u64_0,
     0x61c1614149053801 as libc::c_longlong as u64_0,
     0x1801100110011801 as libc::c_longlong as u64_0,
     0x1801080108011001 as libc::c_longlong as u64_0,
     0x1001204351435945 as libc::c_longlong as u64_0,
     0x6a0561c751c720c1 as libc::c_longlong as u64_0,
     0x49c5290331473901 as libc::c_longlong as u64_0,
     0x41016ad172d13a0d as libc::c_longlong as u64_0,
     0x1081000149834141 as libc::c_longlong as u64_0,
     0x720561c730411801 as libc::c_longlong as u64_0,
     0x1001284330833883 as libc::c_longlong as u64_0,
     0x2841100108011001 as libc::c_longlong as u64_0,
     0x1801180120012881 as libc::c_longlong as u64_0,
     0x490569856a0759c7 as libc::c_longlong as u64_0,
     0x6a073943294759c9 as libc::c_longlong as u64_0,
     0x7b118b975a4d2947 as libc::c_longlong as u64_0,
     0x4308c78bd59b0f as libc::c_longlong as u64_0,
     0x720948c520011001 as libc::c_longlong as u64_0,
     0x30435947488140c1 as libc::c_longlong as u64_0,
     0x3001180110013041 as libc::c_longlong as u64_0,
     0x3803380128010801 as libc::c_longlong as u64_0,
     0x180140c3618569c7 as libc::c_longlong as u64_0,
     0x5987518731896a8f as libc::c_longlong as u64_0,
     0x6a4f414338814941 as libc::c_longlong as u64_0,
     0x18c352055a096141 as libc::c_longlong as u64_0,
     0x4081384310012001 as libc::c_longlong as u64_0,
     0x4041380128013801 as libc::c_longlong as u64_0,
     0x3001180120436147 as libc::c_longlong as u64_0,
     0x69c940c138432001 as libc::c_longlong as u64_0,
     0x801280130835103 as libc::c_longlong as u64_0,
     0x50c34987318b7ad1 as libc::c_longlong as u64_0,
     0x620d518749454101 as libc::c_longlong as u64_0,
     0x31475a0951014881 as libc::c_longlong as u64_0,
     0x3801180100011001 as libc::c_longlong as u64_0,
     0x2001280140413803 as libc::c_longlong as u64_0,
     0x3041300148c569c7 as libc::c_longlong as u64_0,
     0x82437a0769c93801 as libc::c_ulonglong,
     0x2801100118013001 as libc::c_longlong as u64_0,
     0x488340c34a0f7ad1 as libc::c_longlong as u64_0,
     0x518769cd41893883 as libc::c_longlong as u64_0,
     0x29cf62d361873883 as libc::c_longlong as u64_0,
     0x2001000100011001 as libc::c_longlong as u64_0,
     0x8012001388350c5 as libc::c_longlong as u64_0,
     0x50c548c3724d7207 as libc::c_longlong as u64_0,
     0x79c5824761815983 as libc::c_longlong as u64_0,
     0x3801200110012001 as libc::c_longlong as u64_0,
     0x4083598b5a515105 as libc::c_longlong as u64_0,
     0x3801404140013801 as libc::c_longlong as u64_0,
     0x5949618930411801 as libc::c_longlong as u64_0,
     0x1001180118012841 as libc::c_longlong as u64_0,
     0x308340c548c130c1 as libc::c_longlong as u64_0,
     0x4149310141896205 as libc::c_longlong as u64_0,
     0x51c572038a057a47 as libc::c_longlong as u64_0,
     0x5901308318011801 as libc::c_longlong as u64_0,
     0x2801300138014001 as libc::c_longlong as u64_0,
     0x4001300128012801 as libc::c_longlong as u64_0,
     0x3043284310012001 as libc::c_longlong as u64_0,
     0x380148c550c361c7 as libc::c_longlong as u64_0,
     0x8a49720550c16183 as libc::c_ulonglong,
     0x6a09514151418249 as libc::c_longlong as u64_0,
     0x494141017a476981 as libc::c_longlong as u64_0,
     0x7205824949053841 as libc::c_longlong as u64_0,
     0x3001200118012001 as libc::c_longlong as u64_0,
     0x2801200118011801 as libc::c_longlong as u64_0,
     0x801080110013001 as libc::c_longlong as u64_0,
     0x4881724d698569c5 as libc::c_longlong as u64_0,
     0x618361417a05b38b as libc::c_longlong as u64_0,
     0xa2cd7a0571c39b11 as libc::c_ulonglong,
     0x7a8b72cd6a8d4101 as libc::c_longlong as u64_0,
     0x6a8f4a4f39474903 as libc::c_longlong as u64_0,
     0x4881200110012801 as libc::c_longlong as u64_0,
     0x3841404348c53841 as libc::c_longlong as u64_0,
     0x1001080108013883 as libc::c_longlong as u64_0,
     0x598b7acf720571c3 as libc::c_longlong as u64_0,
     0x51416a45a393ab4f as libc::c_longlong as u64_0,
     0xcc11a30b71c382cf as libc::c_ulonglong,
     0x7b0f82d172cf51c9 as libc::c_longlong as u64_0,
     0x7ad3290700010001 as libc::c_longlong as u64_0,
     0x4881388118432843 as libc::c_longlong as u64_0,
     0x40c538c359453885 as libc::c_longlong as u64_0,
     0x10010001080140c5 as libc::c_longlong as u64_0,
     0x418749cb6a496a05 as libc::c_longlong as u64_0,
     0x7a89934fa3d182cb as libc::c_longlong as u64_0,
     0x7acb724749017a8b as libc::c_longlong as u64_0,
     0x7b0f620940c140c1 as libc::c_longlong as u64_0,
     0x598b410520831883 as libc::c_longlong as u64_0,
     0x5907514749055147 as libc::c_longlong as u64_0,
     0x594940c161472883 as libc::c_longlong as u64_0,
     0x801000110013883 as libc::c_longlong as u64_0,
     0x598561c37a475981 as libc::c_longlong as u64_0,
     0x72477ac961816201 as libc::c_longlong as u64_0,
     0x6a05620151417acd as libc::c_longlong as u64_0,
     0x5a0740c138013881 as libc::c_longlong as u64_0,
     0x620f598b288530c5 as libc::c_longlong as u64_0,
     0x5147594951454945 as libc::c_longlong as u64_0,
     0x6a0d288138012001 as libc::c_longlong as u64_0,
     0x801080110012841 as libc::c_longlong as u64_0,
     0x51036981618161c1 as libc::c_longlong as u64_0,
     0x82897a8761c16a43 as libc::c_ulonglong,
     0x7a896a897a8b7b0f as libc::c_longlong as u64_0,
     0x51835187620d59cb as libc::c_longlong as u64_0,
     0x6a0f40c538414107 as libc::c_longlong as u64_0,
     0x598b61cb620b6a0d as libc::c_longlong as u64_0,
     0x720d384120011001 as libc::c_longlong as u64_0,
     0x1001208318012801 as libc::c_longlong as u64_0,
     0x510559816a039b4f as libc::c_longlong as u64_0,
     0x82cb69c17a8769c1 as libc::c_ulonglong,
     0x6a8772495a035181 as libc::c_longlong as u64_0,
     0x3001598972d35147 as libc::c_longlong as u64_0,
     0x3843480140835907 as libc::c_longlong as u64_0,
     0x61cb618b51474083 as libc::c_longlong as u64_0,
     0x2803100110012043 as libc::c_longlong as u64_0,
     0x40c548c538412801 as libc::c_longlong as u64_0,
     0x484151018b0db3d5 as libc::c_longlong as u64_0,
     0x7245510172457a89 as libc::c_longlong as u64_0,
     0x82cd830d62074141 as libc::c_ulonglong,
     0x40c1620b59cd5107 as libc::c_longlong as u64_0,
     0x49096147614b598b as libc::c_longlong as u64_0,
     0x4881384120011801 as libc::c_longlong as u64_0,
     0x801100130435107 as libc::c_longlong as u64_0,
     0x50c1388140013801 as libc::c_longlong as u64_0,
     0x404169c7abd38b0d as libc::c_longlong as u64_0,
     0x50c15981934f8acd as libc::c_longlong as u64_0,
     0x72898311728b6a8d as libc::c_longlong as u64_0,
     0x7ad1620d518961cd as libc::c_longlong as u64_0,
     0x69cf494941073947 as libc::c_longlong as u64_0,
     0x3841200118012001 as libc::c_longlong as u64_0,
     0x2841304351471801 as libc::c_longlong as u64_0,
     0x40c17a8d51053001 as libc::c_longlong as u64_0,
     0x304161879b5371c5 as libc::c_longlong as u64_0,
     0x5141720982cd6a47 as libc::c_longlong as u64_0,
     0x72497b0f59c35185 as libc::c_longlong as u64_0,
     0x6209494728c50885 as libc::c_longlong as u64_0,
     0x1188328813907 as libc::c_longlong as u64_0,
     0x2001200138014841 as libc::c_longlong as u64_0,
     0x7a4f828f394720c1 as libc::c_longlong as u64_0,
     0x6a4b7a8d48c12801 as libc::c_longlong as u64_0,
     0x280148c172096205 as libc::c_longlong as u64_0,
     0x6a096a076a47724b as libc::c_longlong as u64_0,
     0x728b620939014901 as libc::c_longlong as u64_0,
     0x5185410128c50909 as libc::c_longlong as u64_0,
     0x2107188530814841 as libc::c_longlong as u64_0,
     0x2801384140416a0b as libc::c_longlong as u64_0,
     0x9353624920812081 as libc::c_ulonglong,
     0x5a076a0749052801 as libc::c_longlong as u64_0,
     0x200150c382cf7a8b as libc::c_longlong as u64_0,
     0x598151016a4782cf as libc::c_longlong as u64_0,
     0x724959c551c161c7 as libc::c_longlong as u64_0,
     0x6a8d620b59893105 as libc::c_longlong as u64_0,
     0x30c5408340413001 as libc::c_longlong as u64_0];
static mut sJntSphInit: ColliderJntSphInit =
    unsafe {
        {
            let mut init =
                ColliderJntSphInit{base:
                                       {
                                           let mut init =
                                               ColliderInit{colType:
                                                                COLTYPE_HIT3
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
static mut sJntSphElementsInit: [ColliderJntSphElementInit; 19] =
    [{
         let mut init =
             ColliderJntSphElementInit{info:
                                           {
                                               let mut init =
                                                   ColliderInfoInit{elemType:
                                                                        ELEMTYPE_UNK3
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
                                                                                                                           8500
                                                                                                                               as
                                                                                                                               libc::c_int
                                                                                                                               as
                                                                                                                               s16,
                                                                                                                       y:
                                                                                                                           1200
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
                                                                                                         40
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
                                                                        ELEMTYPE_UNK3
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
                                                                                    33
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
                                                                                                                           -(2000
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
                                                                        ELEMTYPE_UNK2
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
                                                                        ELEMTYPE_UNK2
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
                                                                        ELEMTYPE_UNK2
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
                                                                        ELEMTYPE_UNK2
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
                                                                                                         40
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
                                                                        ELEMTYPE_UNK2
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
                                                                                    22
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
                                                                        ELEMTYPE_UNK2
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
                                                                                    23
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
                                                                        ELEMTYPE_UNK2
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
                                                                                    24
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
                                                                        ELEMTYPE_UNK2
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
                                                                                    29
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
                                                                        ELEMTYPE_UNK2
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
                                                                                    30
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
                                                                        ELEMTYPE_UNK2
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
                                                                                    31
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
                                                                        ELEMTYPE_UNK2
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
                                                                                    32
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
                                                                                                         50
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
                                                                        ELEMTYPE_UNK2
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
                                                                                    38
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
                                                                        ELEMTYPE_UNK2
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
                                                                                    39
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
                                                                        ELEMTYPE_UNK2
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
                                                                                                         40
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
                                                                        ELEMTYPE_UNK2
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
                                                                                    45
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
                                                                        ELEMTYPE_UNK2
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
                                                                                    46
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
                                                                        ELEMTYPE_UNK2
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
                                                                                    47
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
                                                                                                         40
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
static mut sLavaFloorLavaTex: [u64_0; 1024] =
    [0x6883510751073001 as libc::c_longlong as u64_0,
     0x4083514748c54905 as libc::c_longlong as u64_0,
     0x4903388161c959c9 as libc::c_longlong as u64_0,
     0x58c1690181c58185 as libc::c_longlong as u64_0,
     0x68c169019a8b8a07 as libc::c_longlong as u64_0,
     0x4905410328015145 as libc::c_longlong as u64_0,
     0x9209790388817841 as libc::c_ulonglong,
     0x9841a801a0019841 as libc::c_ulonglong,
     0x60c578c5b905b945 as libc::c_longlong as u64_0,
     0xead1b24d8103b9c3 as libc::c_ulonglong,
     0xcac5d307ebcbcac5 as libc::c_ulonglong,
     0x90418081a1437081 as libc::c_ulonglong,
     0x40012001394b3909 as libc::c_longlong as u64_0,
     0x1001384138013043 as libc::c_longlong as u64_0,
     0x40416801c001d001 as libc::c_longlong as u64_0,
     0xc001a00188017001 as libc::c_ulonglong,
     0x788368c57883a189 as libc::c_longlong as u64_0,
     0xba4f69074841a1c7 as libc::c_ulonglong,
     0xc2cfba8da2079101 as libc::c_ulonglong,
     0xd001e001d801c801 as libc::c_ulonglong,
     0xc801704358c75085 as libc::c_ulonglong,
     0x3001680190818001 as libc::c_longlong as u64_0,
     0x9001b801a801a001 as libc::c_ulonglong,
     0xa801904180417843 as libc::c_ulonglong,
     0x9883808388c5b20f as libc::c_ulonglong,
     0xba5169074801b20b as libc::c_ulonglong,
     0xc28d994580c19081 as libc::c_ulonglong,
     0xd801b80190019001 as libc::c_ulonglong,
     0xc001c001c041c041 as libc::c_ulonglong,
     0xa801c001d801e801 as libc::c_ulonglong,
     0xe801e8c3d903c881 as libc::c_ulonglong,
     0xe145d903b841b001 as libc::c_ulonglong,
     0xb001a841a883b149 as libc::c_ulonglong,
     0xd149c105b8c3c987 as libc::c_ulonglong,
     0x90c391039103a881 as libc::c_ulonglong,
     0xd001900158014001 as libc::c_ulonglong,
     0xa041d801e001e001 as libc::c_ulonglong,
     0xc801b801b041c8c3 as libc::c_ulonglong,
     0xe185d9c7c985c185 as libc::c_ulonglong,
     0xe28beacde1c7c081 as libc::c_ulonglong,
     0xc041d001c801b801 as libc::c_ulonglong,
     0x900198c5c947c145 as libc::c_ulonglong,
     0x91039947b147d801 as libc::c_ulonglong,
     0xb801800158018841 as libc::c_ulonglong,
     0xb081d903b8819081 as libc::c_ulonglong,
     0x8883a04190019081 as libc::c_ulonglong,
     0x8947710579476905 as libc::c_ulonglong,
     0x60c3818979039081 as libc::c_longlong as u64_0,
     0xb001c801c801d001 as libc::c_ulonglong,
     0xd00180017083a0c5 as libc::c_ulonglong,
     0xb9cbc1cbc107c801 as libc::c_ulonglong,
     0xa001680188c5b0c5 as libc::c_ulonglong,
     0xd8c3d841b0019841 as libc::c_ulonglong,
     0x80419801a001a905 as libc::c_ulonglong,
     0x88c3790571075881 as libc::c_ulonglong,
     0x60c3714570c39081 as libc::c_longlong as u64_0,
     0xb001a80198018801 as libc::c_ulonglong,
     0xc001c801a001c001 as libc::c_ulonglong,
     0xc801c801b801a001 as libc::c_ulonglong,
     0x880170439885a001 as libc::c_ulonglong,
     0xc001c001c801c841 as libc::c_ulonglong,
     0xb841c801c001b801 as libc::c_ulonglong,
     0x90419105584160c3 as libc::c_ulonglong,
     0x710760c380c3a801 as libc::c_longlong as u64_0,
     0x9801c945da07b187 as libc::c_ulonglong,
     0xa0c3b801c001c801 as libc::c_ulonglong,
     0x980180018801b801 as libc::c_ulonglong,
     0xd001b043b001c001 as libc::c_ulonglong,
     0xc801d001d001d801 as libc::c_ulonglong,
     0xe001e001d001b001 as libc::c_ulonglong,
     0x9801910770c57989 as libc::c_ulonglong,
     0x794981498883a001 as libc::c_longlong as u64_0,
     0xa985d2c7d287c9c3 as libc::c_ulonglong,
     0x810388418001a947 as libc::c_ulonglong,
     0x898b688358018801 as libc::c_ulonglong,
     0xd801d001c801c801 as libc::c_ulonglong,
     0x988360436801a001 as libc::c_ulonglong,
     0xc801d801d001b001 as libc::c_ulonglong,
     0xa041910770c36883 as libc::c_ulonglong,
     0x6107688598019041 as libc::c_longlong as u64_0,
     0xa145a1459903b101 as libc::c_ulonglong,
     0x910388c3b183c249 as libc::c_ulonglong,
     0xea8bc985b8c1a8c1 as libc::c_ulonglong,
     0xc041c001d001b8c5 as libc::c_ulonglong,
     0xa1c991c748018081 as libc::c_ulonglong,
     0xa841b801e001b001 as libc::c_ulonglong,
     0xa801804360015001 as libc::c_ulonglong,
     0x80859001a8018801 as libc::c_ulonglong,
     0x6801700168017001 as libc::c_longlong as u64_0,
     0x9905684340019103 as libc::c_ulonglong,
     0xea09c1c3b141d143 as libc::c_ulonglong,
     0xb883c801e001c985 as libc::c_ulonglong,
     0xe2c9710120016105 as libc::c_ulonglong,
     0x78c37801a0019001 as libc::c_longlong as u64_0,
     0xa801b001b801b001 as libc::c_ulonglong,
     0xb001a801a8017801 as libc::c_ulonglong,
     0x5041484328014001 as libc::c_longlong as u64_0,
     0x58c7500148017001 as libc::c_longlong as u64_0,
     0xa0c590818081b187 as libc::c_ulonglong,
     0xb105c801d001a881 as libc::c_ulonglong,
     0xa18178819181a2cf as libc::c_ulonglong,
     0x5945484199477907 as libc::c_longlong as u64_0,
     0x8843a001c801c801 as libc::c_ulonglong,
     0xb801b841b105a947 as libc::c_ulonglong,
     0xc245ba45ba43c283 as libc::c_ulonglong,
     0xa9c370c148016043 as libc::c_ulonglong,
     0x904178019841b103 as libc::c_ulonglong,
     0xc185b881b001b801 as libc::c_ulonglong,
     0x9101c281ec4bd38f as libc::c_ulonglong,
     0x69017143b209b981 as libc::c_longlong as u64_0,
     0x88819801c001b801 as libc::c_ulonglong,
     0xb883ea8fba07ba07 as libc::c_ulonglong,
     0xcb07d349ec8bdbc7 as libc::c_ulonglong,
     0xe3c7c24199816147 as libc::c_ulonglong,
     0x38015801904390c3 as libc::c_longlong as u64_0,
     0xb985a0c1b801c001 as libc::c_ulonglong,
     0x90815103ba87d2c5 as libc::c_ulonglong,
     0xc201e309e2cbb981 as libc::c_ulonglong,
     0xa1459841b801a841 as libc::c_ulonglong,
     0xa907b20b71039205 as libc::c_ulonglong,
     0x8205820769416101 as libc::c_ulonglong,
     0x9a03c201eb4d81c9 as libc::c_ulonglong,
     0x180158c790438841 as libc::c_longlong as u64_0,
     0x90c1a101a081a001 as libc::c_ulonglong,
     0xa80158017141b245 as libc::c_ulonglong,
     0xdac7dac9aa0768c1 as libc::c_ulonglong,
     0x99cb9883c00190c5 as libc::c_ulonglong,
     0x78c3814779036103 as libc::c_longlong as u64_0,
     0x7983694158c158c1 as libc::c_longlong as u64_0,
     0x59016101aa496105 as libc::c_longlong as u64_0,
     0x710550c778017801 as libc::c_longlong as u64_0,
     0x8081b9c5a985c041 as libc::c_ulonglong,
     0xd801b0c399c78a09 as libc::c_ulonglong,
     0x58c158c158816041 as libc::c_longlong as u64_0,
     0xa1479801b0018883 as libc::c_ulonglong,
     0x7881814581456143 as libc::c_longlong as u64_0,
     0x820779c569836985 as libc::c_ulonglong,
     0x928d7a09a1c79185 as libc::c_ulonglong,
     0x7947284340017801 as libc::c_longlong as u64_0,
     0x70839103c983c801 as libc::c_longlong as u64_0,
     0xe001d00189058987 as libc::c_ulonglong,
     0x714389c981456083 as libc::c_longlong as u64_0,
     0x80417801904160c3 as libc::c_ulonglong,
     0x790191c5b2479a49 as libc::c_longlong as u64_0,
     0x7183614169416141 as libc::c_longlong as u64_0,
     0x824b694550817147 as libc::c_ulonglong,
     0x598b380138018885 as libc::c_longlong as u64_0,
     0x800168018883b801 as libc::c_ulonglong,
     0xd801d80160013801 as libc::c_ulonglong,
     0x3801404340014801 as libc::c_longlong as u64_0,
     0x78018843a1459a09 as libc::c_longlong as u64_0,
     0xca899a07c245aa47 as libc::c_ulonglong,
     0x7141694169836143 as libc::c_longlong as u64_0,
     0x8a4d610338017147 as libc::c_ulonglong,
     0x518b2801484180c5 as libc::c_longlong as u64_0,
     0x780180019843c001 as libc::c_longlong as u64_0,
     0xd801e00188017001 as libc::c_ulonglong,
     0x7043680168017001 as libc::c_longlong as u64_0,
     0x8043b145b181d2c7 as libc::c_ulonglong,
     0xca87814181418141 as libc::c_ulonglong,
     0x718361417a076983 as libc::c_longlong as u64_0,
     0x820b69454801798b as libc::c_ulonglong,
     0x598d20016081b187 as libc::c_longlong as u64_0,
     0xa10180418801b801 as libc::c_ulonglong,
     0xd001d80188c370c3 as libc::c_ulonglong,
     0x7947790570c58147 as libc::c_longlong as u64_0,
     0x70c38101b1c1d2c9 as libc::c_longlong as u64_0,
     0x91c38141898191c5 as libc::c_ulonglong,
     0x61454905620d7185 as libc::c_longlong as u64_0,
     0x6943694550417989 as libc::c_longlong as u64_0,
     0x490918017987eb4f as libc::c_longlong as u64_0,
     0xc243c181d141d001 as libc::c_ulonglong,
     0xe001d001a941b981 as libc::c_ulonglong,
     0xa183714350c38185 as libc::c_ulonglong,
     0x9941a181d289ba8d as libc::c_ulonglong,
     0x918389419a0589c5 as libc::c_ulonglong,
     0x5147418b59c958c1 as libc::c_longlong as u64_0,
     0x58c160c160c37989 as libc::c_longlong as u64_0,
     0x40c5388351496905 as libc::c_longlong as u64_0,
     0x9141d203d941d801 as libc::c_ulonglong,
     0xd001a945d285db49 as libc::c_ulonglong,
     0xec0bc241c203b9c1 as libc::c_ulonglong,
     0xba81d2c7dac9b28b as libc::c_ulonglong,
     0x80c191c58a078a09 as libc::c_ulonglong,
     0x61c961c969834841 as libc::c_longlong as u64_0,
     0x6103798789cb8189 as libc::c_longlong as u64_0,
     0x48c7598b40c55841 as libc::c_longlong as u64_0,
     0x68c178419801d801 as libc::c_longlong as u64_0,
     0xc801914599c371c5 as libc::c_ulonglong,
     0x9a05b181ba81c241 as libc::c_ulonglong,
     0xb2458985a209b2cd as libc::c_ulonglong,
     0x80c1aa4ba24b81c7 as libc::c_ulonglong,
     0x594561c979c57183 as libc::c_longlong as u64_0,
     0x928d924d798768c3 as libc::c_ulonglong,
     0x3883510740837945 as libc::c_longlong as u64_0,
     0x81058801c841d801 as libc::c_ulonglong,
     0xa903b24799836101 as libc::c_ulonglong,
     0x6141690171438a07 as libc::c_longlong as u64_0,
     0x8a0979439a099185 as libc::c_ulonglong,
     0x8101a20999c78143 as libc::c_ulonglong,
     0x59035943590160c1 as libc::c_longlong as u64_0,
     0x89c97103604168c3 as libc::c_ulonglong,
     0x40834083300170c3 as libc::c_longlong as u64_0,
     0x8081a801d001c801 as libc::c_ulonglong,
     0x6081610148416945 as libc::c_longlong as u64_0,
     0x928d79877185928d as libc::c_ulonglong,
     0x924b794389858103 as libc::c_ulonglong,
     0x8943a20b81437985 as libc::c_ulonglong,
     0x91c1a20599c37143 as libc::c_ulonglong,
     0x81c768c171038987 as libc::c_ulonglong,
     0x4081488350c58987 as libc::c_longlong as u64_0,
     0x90c3a801b8018801 as libc::c_ulonglong,
     0x4801588158c1820b as libc::c_longlong as u64_0,
     0x8a4b610381c98209 as libc::c_ulonglong,
     0x6901588160817101 as libc::c_longlong as u64_0,
     0x8145814379437943 as libc::c_ulonglong,
     0xa205c2c7c2897943 as libc::c_ulonglong,
     0x898779038103b9c9 as libc::c_ulonglong,
     0x8987918971899987 as libc::c_ulonglong,
     0xa103a801c801c801 as libc::c_ulonglong,
     0x904181037947920b as libc::c_ulonglong,
     0x7145504171856101 as libc::c_longlong as u64_0,
     0x58c16901710189c9 as libc::c_longlong as u64_0,
     0x91c78943918391c5 as libc::c_ulonglong,
     0x8185920991c778c1 as libc::c_ulonglong,
     0x890389039987b985 as libc::c_ulonglong,
     0xc1c798c380c3b145 as libc::c_ulonglong,
     0xa841d001e001e801 as libc::c_ulonglong,
     0xd001a84188419107 as libc::c_ulonglong,
     0x6881480171457987 as libc::c_longlong as u64_0,
     0x714581c789c969cb as libc::c_longlong as u64_0,
     0x38c7488550437105 as libc::c_longlong as u64_0,
     0x60c7608558434801 as libc::c_longlong as u64_0,
     0x5843584358434801 as libc::c_longlong as u64_0,
     0x70019001a801b801 as libc::c_longlong as u64_0,
     0xd001d801e001e801 as libc::c_ulonglong,
     0xe001b80198018801 as libc::c_ulonglong,
     0x5801380141092043 as libc::c_longlong as u64_0,
     0x50c3610540435043 as libc::c_longlong as u64_0,
     0x680190019801a801 as libc::c_longlong as u64_0,
     0xa001c801d001d001 as libc::c_ulonglong,
     0xd001d001c801c801 as libc::c_ulonglong,
     0xc001d001d001c801 as libc::c_ulonglong,
     0xc801a801b001d001 as libc::c_ulonglong,
     0xe001d001c801b001 as libc::c_ulonglong,
     0xa801984370014001 as libc::c_ulonglong,
     0x5843680180019801 as libc::c_longlong as u64_0,
     0xb001d801d001d801 as libc::c_ulonglong,
     0xb001b801c001c001 as libc::c_ulonglong,
     0xc001b001b041a083 as libc::c_ulonglong,
     0x8001784170018041 as libc::c_ulonglong,
     0x8083704188c5a883 as libc::c_ulonglong,
     0xd801b0018001a085 as libc::c_ulonglong,
     0xc801d001d001b001 as libc::c_ulonglong,
     0xa801a801b001c001 as libc::c_ulonglong,
     0xc801c80188417883 as libc::c_ulonglong,
     0x80c398c1d185d143 as libc::c_ulonglong,
     0xd103d103c903c143 as libc::c_ulonglong,
     0xb10189036945aa4b as libc::c_ulonglong,
     0xdb0dd245d205b8c1 as libc::c_ulonglong,
     0xd001d001b081ea4b as libc::c_ulonglong,
     0xd103d0c1e001e001 as libc::c_ulonglong,
     0xd001d801e001d801 as libc::c_ulonglong,
     0xd981eac9dac7db07 as libc::c_ulonglong,
     0x50c17903d249c185 as libc::c_longlong as u64_0,
     0xc987c989b0c1c9c7 as libc::c_ulonglong,
     0xd249aa4b698791c5 as libc::c_ulonglong,
     0xca85c243db0bd205 as libc::c_ulonglong,
     0xa841d001c103da49 as libc::c_ulonglong,
     0xca05d247e2078881 as libc::c_ulonglong,
     0x6801a987c205d2c3 as libc::c_longlong as u64_0,
     0xdb85d385c301bac1 as libc::c_ulonglong,
     0x5943594591c98105 as libc::c_longlong as u64_0,
     0x910590c38881a989 as libc::c_ulonglong,
     0xb1c960c348015081 as libc::c_ulonglong,
     0x8985814392098145 as libc::c_ulonglong,
     0x8083c801804360c5 as libc::c_ulonglong,
     0x710368c168c13841 as libc::c_longlong as u64_0,
     0x9a09db4ddb07b245 as libc::c_ulonglong,
     0x7183694161416983 as libc::c_longlong as u64_0,
     0x594340c371038105 as libc::c_longlong as u64_0,
     0xa1cba147808560c5 as libc::c_ulonglong,
     0x71095041684170c3 as libc::c_longlong as u64_0,
     0x9143810191c7a9c5 as libc::c_ulonglong,
     0x8881d801b80190c5 as libc::c_ulonglong,
     0x6081710179855105 as libc::c_longlong as u64_0,
     0x8a07c2c9aa036941 as libc::c_ulonglong,
     0x7983718369836983 as libc::c_longlong as u64_0,
     0x510348c168c15107 as libc::c_longlong as u64_0,
     0x598b584380018801 as libc::c_longlong as u64_0,
     0x7001680188017883 as libc::c_longlong as u64_0,
     0x6043380148c58143 as libc::c_longlong as u64_0,
     0xc9c1c943a841c001 as libc::c_ulonglong,
     0xc00198c19a4b61c9 as libc::c_ulonglong,
     0x4903794369016941 as libc::c_longlong as u64_0,
     0x820779c569435943 as libc::c_ulonglong,
     0x5103514538813885 as libc::c_longlong as u64_0,
     0x68c78801d001b001 as libc::c_longlong as u64_0,
     0x78017041808370c7 as libc::c_longlong as u64_0,
     0x6001500168435001 as libc::c_longlong as u64_0,
     0x480188c59841b801 as libc::c_longlong as u64_0,
     0xe801d001a1875105 as libc::c_ulonglong,
     0x38817141714179c5 as libc::c_longlong as u64_0,
     0x7985610169416143 as libc::c_longlong as u64_0,
     0x58c3610558c36885 as libc::c_longlong as u64_0,
     0x9001d001c0019001 as libc::c_ulonglong,
     0x88c3c28f71479103 as libc::c_ulonglong,
     0xb101c981b9c76801 as libc::c_ulonglong,
     0x600180438801a801 as libc::c_longlong as u64_0,
     0xe001d00180414001 as libc::c_ulonglong,
     0x30018143920789c7 as libc::c_longlong as u64_0,
     0x7943710168c168c1 as libc::c_longlong as u64_0,
     0x80c3714b90419001 as libc::c_ulonglong,
     0xd801c00188c58947 as libc::c_ulonglong,
     0xe2cfa20b30014041 as libc::c_ulonglong,
     0xb9c3dac7eb89d243 as libc::c_ulonglong,
     0xe285eb09c183a041 as libc::c_ulonglong,
     0xd801c00178015001 as libc::c_ulonglong,
     0x58838907810598c1 as libc::c_longlong as u64_0,
     0x9881984198419881 as libc::c_ulonglong,
     0xb041a001a801b001 as libc::c_ulonglong,
     0xc00180c39a0b8989 as libc::c_ulonglong,
     0xda8b7903790128c7 as libc::c_ulonglong,
     0x314b5147b245dac5 as libc::c_longlong as u64_0,
     0xdb87d305ca01a101 as libc::c_ulonglong,
     0xd801c80198017801 as libc::c_ulonglong,
     0x9843a001b001c801 as libc::c_ulonglong,
     0xd001d801c801c001 as libc::c_ulonglong,
     0xb001a801c001c001 as libc::c_ulonglong,
     0x88c57103a183ca05 as libc::c_ulonglong,
     0xca05c24992095105 as libc::c_ulonglong,
     0x61cb924b71837183 as libc::c_longlong as u64_0,
     0x610179c589c77001 as libc::c_longlong as u64_0,
     0xd801e801c901b145 as libc::c_ulonglong,
     0x9801a001c001c801 as libc::c_ulonglong,
     0xc001d001d001d001 as libc::c_ulonglong,
     0xd001c00188c3aa4b as libc::c_ulonglong,
     0xba8bc203eb0bba8b as libc::c_ulonglong,
     0x798781c928412801 as libc::c_longlong as u64_0,
     0x514779c5690179c7 as libc::c_longlong as u64_0,
     0x82078a0981016801 as libc::c_ulonglong,
     0xb801e001c943da05 as libc::c_ulonglong,
     0xb141b10390018801 as libc::c_ulonglong,
     0x784390c590c588c5 as libc::c_longlong as u64_0,
     0x8147a209ba89e349 as libc::c_ulonglong,
     0xcb05d287c30d6945 as libc::c_ulonglong,
     0x694581c728414905 as libc::c_longlong as u64_0,
     0x5189694361016147 as libc::c_longlong as u64_0,
     0x4843500160019001 as libc::c_longlong as u64_0,
     0xd001d801b081d181 as libc::c_ulonglong,
     0xd203914560416001 as libc::c_ulonglong,
     0x6905610561455103 as libc::c_longlong as u64_0,
     0x388138c19a038981 as libc::c_longlong as u64_0,
     0x8941614369454881 as libc::c_ulonglong,
     0x3083518908439acf as libc::c_longlong as u64_0,
     0x8a4b998159076043 as libc::c_ulonglong,
     0x80019801a001b001 as libc::c_ulonglong,
     0xd801e801b0019001 as libc::c_ulonglong,
     0xa8c1ba05a1817901 as libc::c_ulonglong,
     0x590348c359474903 as libc::c_longlong as u64_0,
     0x2841000100012841 as libc::c_longlong as u64_0,
     0x488150c1698571c9 as libc::c_longlong as u64_0,
     0x30c5084300015901 as libc::c_longlong as u64_0,
     0x8941914168839001 as libc::c_ulonglong,
     0xd001d801d001c001 as libc::c_ulonglong,
     0xa801d801d801c801 as libc::c_ulonglong,
     0xd001d945da45ca43 as libc::c_ulonglong,
     0x9985714361033041 as libc::c_ulonglong,
     0x5903388138c34841 as libc::c_longlong as u64_0,
     0x50817a0b820b4149 as libc::c_longlong as u64_0,
     0x20010001590150c1 as libc::c_longlong as u64_0,
     0x91c37903a841d001 as libc::c_ulonglong,
     0xd001b0c5a903a903 as libc::c_ulonglong,
     0x9881d801d001b801 as libc::c_ulonglong,
     0xd801b8019801d181 as libc::c_ulonglong,
     0xd205998589836943 as libc::c_ulonglong,
     0x798348c138c36945 as libc::c_longlong as u64_0,
     0x8a4d9ad1718730c5 as libc::c_ulonglong,
     0x108439b0fb28b as libc::c_longlong as u64_0,
     0xa1c77083c801d001 as libc::c_ulonglong,
     0xa0439147888198c1 as libc::c_ulonglong,
     0x8001d801c0019841 as libc::c_ulonglong,
     0x78018001b801d001 as libc::c_longlong as u64_0,
     0xc041b94399439185 as libc::c_ulonglong,
     0x79035903410579c9 as libc::c_longlong as u64_0,
     0x79c958c1484158c1 as libc::c_longlong as u64_0,
     0x2881614369858901 as libc::c_longlong as u64_0,
     0x60419041c801a001 as libc::c_longlong as u64_0,
     0x8001480130015801 as libc::c_ulonglong,
     0x9801d001b883b1c9 as libc::c_ulonglong,
     0x918760418843d001 as libc::c_ulonglong,
     0xd801a001a0c19103 as libc::c_ulonglong,
     0x708158c34107820d as libc::c_longlong as u64_0,
     0x7189610360c15881 as libc::c_longlong as u64_0,
     0x38c3694578c18881 as libc::c_longlong as u64_0,
     0x8001d001e801c801 as libc::c_ulonglong,
     0xb001a00180018801 as libc::c_ulonglong,
     0xa0019801b187b1c5 as libc::c_ulonglong,
     0xc281aa0589858001 as libc::c_ulonglong,
     0xc801c001b0019801 as libc::c_ulonglong,
     0x68415841598b8a0d as libc::c_longlong as u64_0,
     0x6905794758c54841 as libc::c_longlong as u64_0,
     0x3909720d89459083 as libc::c_longlong as u64_0,
     0xc801e801e801d001 as libc::c_ulonglong,
     0xd801d00190437083 as libc::c_ulonglong,
     0x78c360819a09a1c5 as libc::c_longlong as u64_0,
     0xd285d2c589417101 as libc::c_ulonglong,
     0x7001b801b801a001 as libc::c_longlong as u64_0,
     0x8843690771cd6883 as libc::c_ulonglong,
     0x4801784168015001 as libc::c_longlong as u64_0,
     0x70c59987a081c801 as libc::c_longlong as u64_0,
     0xe001d801c801a041 as libc::c_ulonglong,
     0xc987c9c970814041 as libc::c_ulonglong,
     0x50c5404151057143 as libc::c_longlong as u64_0,
     0x690191c1d2c5d2c7 as libc::c_longlong as u64_0,
     0xa981d183b801b001 as libc::c_ulonglong,
     0xa801a80178016001 as libc::c_ulonglong,
     0x60018801a001a001 as libc::c_longlong as u64_0,
     0xa885a883c801c801 as libc::c_ulonglong,
     0x7801808388c1b1c7 as libc::c_longlong as u64_0,
     0xe34bca85a941a181 as libc::c_ulonglong,
     0x818560c168c1b207 as libc::c_ulonglong,
     0x60c158819a05b1c1 as libc::c_longlong as u64_0,
     0xca85ec53c1c7a081 as libc::c_ulonglong,
     0x9001b001c001c001 as libc::c_ulonglong,
     0xc801c801d001d001 as libc::c_ulonglong,
     0xd001d801e0019001 as libc::c_ulonglong,
     0x684191c99983ca45 as libc::c_longlong as u64_0,
     0x99819141ca85db89 as libc::c_ulonglong,
     0xeb8dd285c241d30d as libc::c_ulonglong,
     0x8a09820979c75081 as libc::c_ulonglong,
     0x79c959cb30815881 as libc::c_longlong as u64_0,
     0x8041b001c801a001 as libc::c_ulonglong,
     0x7001704180838001 as libc::c_longlong as u64_0,
     0xb801e001e0019081 as libc::c_ulonglong,
     0x6907924ba1c3a1c1 as libc::c_longlong as u64_0,
     0x68c18183b30fdb4d as libc::c_longlong as u64_0,
     0xec0fe349a1c1924b as libc::c_ulonglong,
     0x924b8a4b82096945 as libc::c_ulonglong,
     0x8a0d308300010001 as libc::c_ulonglong,
     0x70419881d105d187 as libc::c_longlong as u64_0,
     0xd1c7c143b187a0c3 as libc::c_ulonglong,
     0xd801e801e001b145 as libc::c_ulonglong,
     0x50c3514781c57943 as libc::c_longlong as u64_0,
     0x8a05aacbab0d9a47 as libc::c_ulonglong,
     0x920581c358818a09 as libc::c_ulonglong,
     0x8a4b714550415041 as libc::c_ulonglong,
     0x7147588328412041 as libc::c_longlong as u64_0,
     0x9105a987ca09da4b as libc::c_ulonglong,
     0xe2cdb943a987a083 as libc::c_ulonglong,
     0xe001e801e881a0c1 as libc::c_ulonglong,
     0x81437143aa036901 as libc::c_ulonglong,
     0x89c3920771017941 as libc::c_ulonglong,
     0x8183714168c18a09 as libc::c_ulonglong,
     0x7143508140015041 as libc::c_longlong as u64_0,
     0x7949690738433843 as libc::c_longlong as u64_0,
     0x70c5710568c360c3 as libc::c_longlong as u64_0,
     0x818938015001c801 as libc::c_ulonglong,
     0xe001d881c0419801 as libc::c_ulonglong,
     0x9101a18199817941 as libc::c_ulonglong,
     0x9205920579418181 as libc::c_ulonglong,
     0x8a0581c58207924b as libc::c_ulonglong,
     0x6101610379897147 as libc::c_longlong as u64_0,
     0x798b584348015083 as libc::c_longlong as u64_0,
     0x7907714779498189 as libc::c_longlong as u64_0,
     0x99cb9841c001d801 as libc::c_ulonglong,
     0xc883b905c001b001 as libc::c_ulonglong,
     0x78c379018181aac9 as libc::c_longlong as u64_0,
     0xa247898192037941 as libc::c_ulonglong,
     0x81c381c571416101 as libc::c_ulonglong,
     0x400169058a0d68c5 as libc::c_longlong as u64_0,
     0x5001700158419107 as libc::c_longlong as u64_0,
     0x7947814799479883 as libc::c_longlong as u64_0,
     0xb041d801d001b883 as libc::c_ulonglong,
     0xb9898083a0019801 as libc::c_ulonglong,
     0x704170c1a289c34f as libc::c_longlong as u64_0,
     0x91c370c189c389c5 as libc::c_ulonglong,
     0x92499249714358c1 as libc::c_ulonglong,
     0x5081714771496083 as libc::c_longlong as u64_0,
     0x68c5990799497107 as libc::c_longlong as u64_0,
     0x70419041b001c001 as libc::c_longlong as u64_0,
     0xd801d801a04170c5 as libc::c_ulonglong,
     0x6881504160018801 as libc::c_longlong as u64_0,
     0x78019183bb4fa289 as libc::c_longlong as u64_0,
     0x68817901aa8b9a49 as libc::c_longlong as u64_0,
     0x89c5928b79c779c7 as libc::c_ulonglong,
     0x8a0b798968c57949 as libc::c_ulonglong,
     0x814b58c5508348c5 as libc::c_ulonglong,
     0x8801b001b001a801 as libc::c_ulonglong,
     0xb841b90560c32801 as libc::c_ulonglong,
     0x5881920980c5a801 as libc::c_longlong as u64_0,
     0x9801a987b2cd8983 as libc::c_ulonglong,
     0x70c189c59a498183 as libc::c_longlong as u64_0,
     0x81c58a4969416101 as libc::c_ulonglong,
     0x714560c330431043 as libc::c_longlong as u64_0,
     0x1200138414883 as libc::c_longlong as u64_0,
     0xa801b80180016801 as libc::c_ulonglong,
     0xba4fa24d48c33041 as libc::c_ulonglong,
     0x81c792097041a801 as libc::c_ulonglong,
     0xa00190c189857943 as libc::c_ulonglong,
     0x81858185818581c5 as libc::c_ulonglong,
     0x82077185508158c1 as libc::c_ulonglong,
     0x6103588138411085 as libc::c_longlong as u64_0,
     0x2885284140018841 as libc::c_longlong as u64_0,
     0xa801880158018187 as libc::c_ulonglong,
     0xa28f798530412801 as libc::c_ulonglong,
     0x7143798378c3b001 as libc::c_longlong as u64_0,
     0xa801a143a24989c7 as libc::c_ulonglong,
     0x710168c181859a49 as libc::c_longlong as u64_0,
     0x81c5714361017145 as libc::c_ulonglong,
     0x79c7718771454081 as libc::c_longlong as u64_0,
     0x384158419041a001 as libc::c_longlong as u64_0,
     0x6883510751073001 as libc::c_longlong as u64_0,
     0x4083514748c54905 as libc::c_longlong as u64_0,
     0x4903388161c959c9 as libc::c_longlong as u64_0,
     0x58c1690181c58185 as libc::c_longlong as u64_0,
     0x68c169019a8b8a07 as libc::c_longlong as u64_0,
     0x4905410328015145 as libc::c_longlong as u64_0,
     0x9209790388817841 as libc::c_ulonglong,
     0x9841a801a0019841 as libc::c_ulonglong,
     0x60c578c5b905b945 as libc::c_longlong as u64_0,
     0xead1b24d8103b9c3 as libc::c_ulonglong,
     0xcac5d307ebcbcac5 as libc::c_ulonglong,
     0x90418081a1437081 as libc::c_ulonglong,
     0x40012001394b3909 as libc::c_longlong as u64_0,
     0x1001384138013043 as libc::c_longlong as u64_0,
     0x40416801c001d001 as libc::c_longlong as u64_0,
     0xc001a00188017001 as libc::c_ulonglong,
     0x788368c57883a189 as libc::c_longlong as u64_0,
     0xba4f69074841a1c7 as libc::c_ulonglong,
     0xc2cfba8da2079101 as libc::c_ulonglong,
     0xd001e001d801c801 as libc::c_ulonglong,
     0xc801704358c75085 as libc::c_ulonglong,
     0x3001680190818001 as libc::c_longlong as u64_0,
     0x9001b801a801a001 as libc::c_ulonglong,
     0xa801904180417843 as libc::c_ulonglong,
     0x9883808388c5b20f as libc::c_ulonglong,
     0xba5169074801b20b as libc::c_ulonglong,
     0xc28d994580c19081 as libc::c_ulonglong,
     0xd801b80190019001 as libc::c_ulonglong,
     0xc001c001c041c041 as libc::c_ulonglong,
     0xa801c001d801e801 as libc::c_ulonglong,
     0xe801e8c3d903c881 as libc::c_ulonglong,
     0xe145d903b841b001 as libc::c_ulonglong,
     0xb001a841a883b149 as libc::c_ulonglong,
     0xd149c105b8c3c987 as libc::c_ulonglong,
     0x90c391039103a881 as libc::c_ulonglong,
     0xd001900158014001 as libc::c_ulonglong,
     0xa041d801e001e001 as libc::c_ulonglong,
     0xc801b801b041c8c3 as libc::c_ulonglong,
     0xe185d9c7c985c185 as libc::c_ulonglong,
     0xe28beacde1c7c081 as libc::c_ulonglong,
     0xc041d001c801b801 as libc::c_ulonglong,
     0x900198c5c947c145 as libc::c_ulonglong,
     0x91039947b147d801 as libc::c_ulonglong,
     0xb801800158018841 as libc::c_ulonglong,
     0xb081d903b8819081 as libc::c_ulonglong,
     0x8883a04190019081 as libc::c_ulonglong,
     0x8947710579476905 as libc::c_ulonglong,
     0x60c3818979039081 as libc::c_longlong as u64_0,
     0xb001c801c801d001 as libc::c_ulonglong,
     0xd00180017083a0c5 as libc::c_ulonglong,
     0xb9cbc1cbc107c801 as libc::c_ulonglong,
     0xa001680188c5b0c5 as libc::c_ulonglong,
     0xd8c3d841b0019841 as libc::c_ulonglong,
     0x80419801a001a905 as libc::c_ulonglong,
     0x88c3790571075881 as libc::c_ulonglong,
     0x60c3714570c39081 as libc::c_longlong as u64_0,
     0xb001a80198018801 as libc::c_ulonglong,
     0xc001c801a001c001 as libc::c_ulonglong,
     0xc801c801b801a001 as libc::c_ulonglong,
     0x880170439885a001 as libc::c_ulonglong,
     0xc001c001c801c841 as libc::c_ulonglong,
     0xb841c801c001b801 as libc::c_ulonglong,
     0x90419105584160c3 as libc::c_ulonglong,
     0x710760c380c3a801 as libc::c_longlong as u64_0,
     0x9801c945da07b187 as libc::c_ulonglong,
     0xa0c3b801c001c801 as libc::c_ulonglong,
     0x980180018801b801 as libc::c_ulonglong,
     0xd001b043b001c001 as libc::c_ulonglong,
     0xc801d001d001d801 as libc::c_ulonglong,
     0xe001e001d001b001 as libc::c_ulonglong,
     0x9801910770c57989 as libc::c_ulonglong,
     0x794981498883a001 as libc::c_longlong as u64_0,
     0xa985d2c7d287c9c3 as libc::c_ulonglong,
     0x810388418001a947 as libc::c_ulonglong,
     0x898b688358018801 as libc::c_ulonglong,
     0xd801d001c801c801 as libc::c_ulonglong,
     0x988360436801a001 as libc::c_ulonglong,
     0xc801d801d001b001 as libc::c_ulonglong,
     0xa041910770c36883 as libc::c_ulonglong,
     0x6107688598019041 as libc::c_longlong as u64_0,
     0xa145a1459903b101 as libc::c_ulonglong,
     0x910388c3b183c249 as libc::c_ulonglong,
     0xea8bc985b8c1a8c1 as libc::c_ulonglong,
     0xc041c001d001b8c5 as libc::c_ulonglong,
     0xa1c991c748018081 as libc::c_ulonglong,
     0xa841b801e001b001 as libc::c_ulonglong,
     0xa801804360015001 as libc::c_ulonglong,
     0x80859001a8018801 as libc::c_ulonglong,
     0x6801700168017001 as libc::c_longlong as u64_0,
     0x9905684340019103 as libc::c_ulonglong,
     0xea09c1c3b141d143 as libc::c_ulonglong,
     0xb883c801e001c985 as libc::c_ulonglong,
     0xe2c9710120016105 as libc::c_ulonglong,
     0x78c37801a0019001 as libc::c_longlong as u64_0,
     0xa801b001b801b001 as libc::c_ulonglong,
     0xb001a801a8017801 as libc::c_ulonglong,
     0x5041484328014001 as libc::c_longlong as u64_0,
     0x58c7500148017001 as libc::c_longlong as u64_0,
     0xa0c590818081b187 as libc::c_ulonglong,
     0xb105c801d001a881 as libc::c_ulonglong,
     0xa18178819181a2cf as libc::c_ulonglong,
     0x5945484199477907 as libc::c_longlong as u64_0,
     0x8843a001c801c801 as libc::c_ulonglong,
     0xb801b841b105a947 as libc::c_ulonglong,
     0xc245ba45ba43c283 as libc::c_ulonglong,
     0xa9c370c148016043 as libc::c_ulonglong,
     0x904178019841b103 as libc::c_ulonglong,
     0xc185b881b001b801 as libc::c_ulonglong,
     0x9101c281ec4bd38f as libc::c_ulonglong,
     0x69017143b209b981 as libc::c_longlong as u64_0,
     0x88819801c001b801 as libc::c_ulonglong,
     0xb883ea8fba07ba07 as libc::c_ulonglong,
     0xcb07d349ec8bdbc7 as libc::c_ulonglong,
     0xe3c7c24199816147 as libc::c_ulonglong,
     0x38015801904390c3 as libc::c_longlong as u64_0,
     0xb985a0c1b801c001 as libc::c_ulonglong,
     0x90815103ba87d2c5 as libc::c_ulonglong,
     0xc201e309e2cbb981 as libc::c_ulonglong,
     0xa1459841b801a841 as libc::c_ulonglong,
     0xa907b20b71039205 as libc::c_ulonglong,
     0x8205820769416101 as libc::c_ulonglong,
     0x9a03c201eb4d81c9 as libc::c_ulonglong,
     0x180158c790438841 as libc::c_longlong as u64_0,
     0x90c1a101a081a001 as libc::c_ulonglong,
     0xa80158017141b245 as libc::c_ulonglong,
     0xdac7dac9aa0768c1 as libc::c_ulonglong,
     0x99cb9883c00190c5 as libc::c_ulonglong,
     0x78c3814779036103 as libc::c_longlong as u64_0,
     0x7983694158c158c1 as libc::c_longlong as u64_0,
     0x59016101aa496105 as libc::c_longlong as u64_0,
     0x710550c778017801 as libc::c_longlong as u64_0,
     0x8081b9c5a985c041 as libc::c_ulonglong,
     0xd801b0c399c78a09 as libc::c_ulonglong,
     0x58c158c158816041 as libc::c_longlong as u64_0,
     0xa1479801b0018883 as libc::c_ulonglong,
     0x7881814581456143 as libc::c_longlong as u64_0,
     0x820779c569836985 as libc::c_ulonglong,
     0x928d7a09a1c79185 as libc::c_ulonglong,
     0x7947284340017801 as libc::c_longlong as u64_0,
     0x70839103c983c801 as libc::c_longlong as u64_0,
     0xe001d00189058987 as libc::c_ulonglong,
     0x714389c981456083 as libc::c_longlong as u64_0,
     0x80417801904160c3 as libc::c_ulonglong,
     0x790191c5b2479a49 as libc::c_longlong as u64_0,
     0x7183614169416141 as libc::c_longlong as u64_0,
     0x824b694550817147 as libc::c_ulonglong,
     0x598b380138018885 as libc::c_longlong as u64_0,
     0x800168018883b801 as libc::c_ulonglong,
     0xd801d80160013801 as libc::c_ulonglong,
     0x3801404340014801 as libc::c_longlong as u64_0,
     0x78018843a1459a09 as libc::c_longlong as u64_0,
     0xca899a07c245aa47 as libc::c_ulonglong,
     0x7141694169836143 as libc::c_longlong as u64_0,
     0x8a4d610338017147 as libc::c_ulonglong,
     0x518b2801484180c5 as libc::c_longlong as u64_0,
     0x780180019843c001 as libc::c_longlong as u64_0,
     0xd801e00188017001 as libc::c_ulonglong,
     0x7043680168017001 as libc::c_longlong as u64_0,
     0x8043b145b181d2c7 as libc::c_ulonglong,
     0xca87814181418141 as libc::c_ulonglong,
     0x718361417a076983 as libc::c_longlong as u64_0,
     0x820b69454801798b as libc::c_ulonglong,
     0x598d20016081b187 as libc::c_longlong as u64_0,
     0xa10180418801b801 as libc::c_ulonglong,
     0xd001d80188c370c3 as libc::c_ulonglong,
     0x7947790570c58147 as libc::c_longlong as u64_0,
     0x70c38101b1c1d2c9 as libc::c_longlong as u64_0,
     0x91c38141898191c5 as libc::c_ulonglong,
     0x61454905620d7185 as libc::c_longlong as u64_0,
     0x6943694550417989 as libc::c_longlong as u64_0,
     0x490918017987eb4f as libc::c_longlong as u64_0,
     0xc243c181d141d001 as libc::c_ulonglong,
     0xe001d001a941b981 as libc::c_ulonglong,
     0xa183714350c38185 as libc::c_ulonglong,
     0x9941a181d289ba8d as libc::c_ulonglong,
     0x918389419a0589c5 as libc::c_ulonglong,
     0x5147418b59c958c1 as libc::c_longlong as u64_0,
     0x58c160c160c37989 as libc::c_longlong as u64_0,
     0x40c5388351496905 as libc::c_longlong as u64_0,
     0x9141d203d941d801 as libc::c_ulonglong,
     0xd001a945d285db49 as libc::c_ulonglong,
     0xec0bc241c203b9c1 as libc::c_ulonglong,
     0xba81d2c7dac9b28b as libc::c_ulonglong,
     0x80c191c58a078a09 as libc::c_ulonglong,
     0x61c961c969834841 as libc::c_longlong as u64_0,
     0x6103798789cb8189 as libc::c_longlong as u64_0,
     0x48c7598b40c55841 as libc::c_longlong as u64_0,
     0x68c178419801d801 as libc::c_longlong as u64_0,
     0xc801914599c371c5 as libc::c_ulonglong,
     0x9a05b181ba81c241 as libc::c_ulonglong,
     0xb2458985a209b2cd as libc::c_ulonglong,
     0x80c1aa4ba24b81c7 as libc::c_ulonglong,
     0x594561c979c57183 as libc::c_longlong as u64_0,
     0x928d924d798768c3 as libc::c_ulonglong,
     0x3883510740837945 as libc::c_longlong as u64_0,
     0x81058801c841d801 as libc::c_ulonglong,
     0xa903b24799836101 as libc::c_ulonglong,
     0x6141690171438a07 as libc::c_longlong as u64_0,
     0x8a0979439a099185 as libc::c_ulonglong,
     0x8101a20999c78143 as libc::c_ulonglong,
     0x59035943590160c1 as libc::c_longlong as u64_0,
     0x89c97103604168c3 as libc::c_ulonglong,
     0x40834083300170c3 as libc::c_longlong as u64_0,
     0x8081a801d001c801 as libc::c_ulonglong,
     0x6081610148416945 as libc::c_longlong as u64_0,
     0x928d79877185928d as libc::c_ulonglong,
     0x924b794389858103 as libc::c_ulonglong,
     0x8943a20b81437985 as libc::c_ulonglong,
     0x91c1a20599c37143 as libc::c_ulonglong,
     0x81c768c171038987 as libc::c_ulonglong,
     0x4081488350c58987 as libc::c_longlong as u64_0,
     0x90c3a801b8018801 as libc::c_ulonglong,
     0x4801588158c1820b as libc::c_longlong as u64_0,
     0x8a4b610381c98209 as libc::c_ulonglong,
     0x6901588160817101 as libc::c_longlong as u64_0,
     0x8145814379437943 as libc::c_ulonglong,
     0xa205c2c7c2897943 as libc::c_ulonglong,
     0x898779038103b9c9 as libc::c_ulonglong,
     0x8987918971899987 as libc::c_ulonglong,
     0xa103a801c801c801 as libc::c_ulonglong,
     0x904181037947920b as libc::c_ulonglong,
     0x7145504171856101 as libc::c_longlong as u64_0,
     0x58c16901710189c9 as libc::c_longlong as u64_0,
     0x91c78943918391c5 as libc::c_ulonglong,
     0x8185920991c778c1 as libc::c_ulonglong,
     0x890389039987b985 as libc::c_ulonglong,
     0xc1c798c380c3b145 as libc::c_ulonglong,
     0xa841d001e001e801 as libc::c_ulonglong,
     0xd001a84188419107 as libc::c_ulonglong,
     0x6881480171457987 as libc::c_longlong as u64_0,
     0x714581c789c969cb as libc::c_longlong as u64_0,
     0x38c7488550437105 as libc::c_longlong as u64_0,
     0x60c7608558434801 as libc::c_longlong as u64_0,
     0x5843584358434801 as libc::c_longlong as u64_0,
     0x70019001a801b801 as libc::c_longlong as u64_0,
     0xd001d801e001e801 as libc::c_ulonglong,
     0xe001b80198018801 as libc::c_ulonglong,
     0x5801380141092043 as libc::c_longlong as u64_0,
     0x50c3610540435043 as libc::c_longlong as u64_0,
     0x680190019801a801 as libc::c_longlong as u64_0,
     0xa001c801d001d001 as libc::c_ulonglong,
     0xd001d001c801c801 as libc::c_ulonglong,
     0xc001d001d001c801 as libc::c_ulonglong,
     0xc801a801b001d001 as libc::c_ulonglong,
     0xe001d001c801b001 as libc::c_ulonglong,
     0xa801984370014001 as libc::c_ulonglong,
     0x5843680180019801 as libc::c_longlong as u64_0,
     0xb001d801d001d801 as libc::c_ulonglong,
     0xb001b801c001c001 as libc::c_ulonglong,
     0xc001b001b041a083 as libc::c_ulonglong,
     0x8001784170018041 as libc::c_ulonglong,
     0x8083704188c5a883 as libc::c_ulonglong,
     0xd801b0018001a085 as libc::c_ulonglong,
     0xc801d001d001b001 as libc::c_ulonglong,
     0xa801a801b001c001 as libc::c_ulonglong,
     0xc801c80188417883 as libc::c_ulonglong,
     0x80c398c1d185d143 as libc::c_ulonglong,
     0xd103d103c903c143 as libc::c_ulonglong,
     0xb10189036945aa4b as libc::c_ulonglong,
     0xdb0dd245d205b8c1 as libc::c_ulonglong,
     0xd001d001b081ea4b as libc::c_ulonglong,
     0xd103d0c1e001e001 as libc::c_ulonglong,
     0xd001d801e001d801 as libc::c_ulonglong,
     0xd981eac9dac7db07 as libc::c_ulonglong,
     0x50c17903d249c185 as libc::c_longlong as u64_0,
     0xc987c989b0c1c9c7 as libc::c_ulonglong,
     0xd249aa4b698791c5 as libc::c_ulonglong,
     0xca85c243db0bd205 as libc::c_ulonglong,
     0xa841d001c103da49 as libc::c_ulonglong,
     0xca05d247e2078881 as libc::c_ulonglong,
     0x6801a987c205d2c3 as libc::c_longlong as u64_0,
     0xdb85d385c301bac1 as libc::c_ulonglong,
     0x5943594591c98105 as libc::c_longlong as u64_0,
     0x910590c38881a989 as libc::c_ulonglong,
     0xb1c960c348015081 as libc::c_ulonglong,
     0x8985814392098145 as libc::c_ulonglong,
     0x8083c801804360c5 as libc::c_ulonglong,
     0x710368c168c13841 as libc::c_longlong as u64_0,
     0x9a09db4ddb07b245 as libc::c_ulonglong,
     0x7183694161416983 as libc::c_longlong as u64_0,
     0x594340c371038105 as libc::c_longlong as u64_0,
     0xa1cba147808560c5 as libc::c_ulonglong,
     0x71095041684170c3 as libc::c_longlong as u64_0,
     0x9143810191c7a9c5 as libc::c_ulonglong,
     0x8881d801b80190c5 as libc::c_ulonglong,
     0x6081710179855105 as libc::c_longlong as u64_0,
     0x8a07c2c9aa036941 as libc::c_ulonglong,
     0x7983718369836983 as libc::c_longlong as u64_0,
     0x510348c168c15107 as libc::c_longlong as u64_0,
     0x598b584380018801 as libc::c_longlong as u64_0,
     0x7001680188017883 as libc::c_longlong as u64_0,
     0x6043380148c58143 as libc::c_longlong as u64_0,
     0xc9c1c943a841c001 as libc::c_ulonglong,
     0xc00198c19a4b61c9 as libc::c_ulonglong,
     0x4903794369016941 as libc::c_longlong as u64_0,
     0x820779c569435943 as libc::c_ulonglong,
     0x5103514538813885 as libc::c_longlong as u64_0,
     0x68c78801d001b001 as libc::c_longlong as u64_0,
     0x78017041808370c7 as libc::c_longlong as u64_0,
     0x6001500168435001 as libc::c_longlong as u64_0,
     0x480188c59841b801 as libc::c_longlong as u64_0,
     0xe801d001a1875105 as libc::c_ulonglong,
     0x38817141714179c5 as libc::c_longlong as u64_0,
     0x7985610169416143 as libc::c_longlong as u64_0,
     0x58c3610558c36885 as libc::c_longlong as u64_0,
     0x9001d001c0019001 as libc::c_ulonglong,
     0x88c3c28f71479103 as libc::c_ulonglong,
     0xb101c981b9c76801 as libc::c_ulonglong,
     0x600180438801a801 as libc::c_longlong as u64_0,
     0xe001d00180414001 as libc::c_ulonglong,
     0x30018143920789c7 as libc::c_longlong as u64_0,
     0x7943710168c168c1 as libc::c_longlong as u64_0,
     0x80c3714b90419001 as libc::c_ulonglong,
     0xd801c00188c58947 as libc::c_ulonglong,
     0xe2cfa20b30014041 as libc::c_ulonglong,
     0xb9c3dac7eb89d243 as libc::c_ulonglong,
     0xe285eb09c183a041 as libc::c_ulonglong,
     0xd801c00178015001 as libc::c_ulonglong,
     0x58838907810598c1 as libc::c_longlong as u64_0,
     0x9881984198419881 as libc::c_ulonglong,
     0xb041a001a801b001 as libc::c_ulonglong,
     0xc00180c39a0b8989 as libc::c_ulonglong,
     0xda8b7903790128c7 as libc::c_ulonglong,
     0x314b5147b245dac5 as libc::c_longlong as u64_0,
     0xdb87d305ca01a101 as libc::c_ulonglong,
     0xd801c80198017801 as libc::c_ulonglong,
     0x9843a001b001c801 as libc::c_ulonglong,
     0xd001d801c801c001 as libc::c_ulonglong,
     0xb001a801c001c001 as libc::c_ulonglong,
     0x88c57103a183ca05 as libc::c_ulonglong,
     0xca05c24992095105 as libc::c_ulonglong,
     0x61cb924b71837183 as libc::c_longlong as u64_0,
     0x610179c589c77001 as libc::c_longlong as u64_0,
     0xd801e801c901b145 as libc::c_ulonglong,
     0x9801a001c001c801 as libc::c_ulonglong,
     0xc001d001d001d001 as libc::c_ulonglong,
     0xd001c00188c3aa4b as libc::c_ulonglong,
     0xba8bc203eb0bba8b as libc::c_ulonglong,
     0x798781c928412801 as libc::c_longlong as u64_0,
     0x514779c5690179c7 as libc::c_longlong as u64_0,
     0x82078a0981016801 as libc::c_ulonglong,
     0xb801e001c943da05 as libc::c_ulonglong,
     0xb141b10390018801 as libc::c_ulonglong,
     0x784390c590c588c5 as libc::c_longlong as u64_0,
     0x8147a209ba89e349 as libc::c_ulonglong,
     0xcb05d287c30d6945 as libc::c_ulonglong,
     0x694581c728414905 as libc::c_longlong as u64_0,
     0x5189694361016147 as libc::c_longlong as u64_0,
     0x4843500160019001 as libc::c_longlong as u64_0,
     0xd001d801b081d181 as libc::c_ulonglong,
     0xd203914560416001 as libc::c_ulonglong,
     0x6905610561455103 as libc::c_longlong as u64_0,
     0x388138c19a038981 as libc::c_longlong as u64_0,
     0x8941614369454881 as libc::c_ulonglong,
     0x3083518908439acf as libc::c_longlong as u64_0,
     0x8a4b998159076043 as libc::c_ulonglong,
     0x80019801a001b001 as libc::c_ulonglong,
     0xd801e801b0019001 as libc::c_ulonglong,
     0xa8c1ba05a1817901 as libc::c_ulonglong,
     0x590348c359474903 as libc::c_longlong as u64_0,
     0x2841000100012841 as libc::c_longlong as u64_0,
     0x488150c1698571c9 as libc::c_longlong as u64_0,
     0x30c5084300015901 as libc::c_longlong as u64_0,
     0x8941914168839001 as libc::c_ulonglong,
     0xd001d801d001c001 as libc::c_ulonglong,
     0xa801d801d801c801 as libc::c_ulonglong,
     0xd001d945da45ca43 as libc::c_ulonglong,
     0x9985714361033041 as libc::c_ulonglong,
     0x5903388138c34841 as libc::c_longlong as u64_0,
     0x50817a0b820b4149 as libc::c_longlong as u64_0,
     0x20010001590150c1 as libc::c_longlong as u64_0,
     0x91c37903a841d001 as libc::c_ulonglong,
     0xd001b0c5a903a903 as libc::c_ulonglong,
     0x9881d801d001b801 as libc::c_ulonglong,
     0xd801b8019801d181 as libc::c_ulonglong,
     0xd205998589836943 as libc::c_ulonglong,
     0x798348c138c36945 as libc::c_longlong as u64_0,
     0x8a4d9ad1718730c5 as libc::c_ulonglong,
     0x108439b0fb28b as libc::c_longlong as u64_0,
     0xa1c77083c801d001 as libc::c_ulonglong,
     0xa0439147888198c1 as libc::c_ulonglong,
     0x8001d801c0019841 as libc::c_ulonglong,
     0x78018001b801d001 as libc::c_longlong as u64_0,
     0xc041b94399439185 as libc::c_ulonglong,
     0x79035903410579c9 as libc::c_longlong as u64_0,
     0x79c958c1484158c1 as libc::c_longlong as u64_0,
     0x2881614369858901 as libc::c_longlong as u64_0,
     0x60419041c801a001 as libc::c_longlong as u64_0,
     0x8001480130015801 as libc::c_ulonglong,
     0x9801d001b883b1c9 as libc::c_ulonglong,
     0x918760418843d001 as libc::c_ulonglong,
     0xd801a001a0c19103 as libc::c_ulonglong,
     0x708158c34107820d as libc::c_longlong as u64_0,
     0x7189610360c15881 as libc::c_longlong as u64_0,
     0x38c3694578c18881 as libc::c_longlong as u64_0,
     0x8001d001e801c801 as libc::c_ulonglong,
     0xb001a00180018801 as libc::c_ulonglong,
     0xa0019801b187b1c5 as libc::c_ulonglong,
     0xc281aa0589858001 as libc::c_ulonglong,
     0xc801c001b0019801 as libc::c_ulonglong,
     0x68415841598b8a0d as libc::c_longlong as u64_0,
     0x6905794758c54841 as libc::c_longlong as u64_0,
     0x3909720d89459083 as libc::c_longlong as u64_0,
     0xc801e801e801d001 as libc::c_ulonglong,
     0xd801d00190437083 as libc::c_ulonglong,
     0x78c360819a09a1c5 as libc::c_longlong as u64_0,
     0xd285d2c589417101 as libc::c_ulonglong,
     0x7001b801b801a001 as libc::c_longlong as u64_0,
     0x8843690771cd6883 as libc::c_ulonglong,
     0x4801784168015001 as libc::c_longlong as u64_0,
     0x70c59987a081c801 as libc::c_longlong as u64_0,
     0xe001d801c801a041 as libc::c_ulonglong,
     0xc987c9c970814041 as libc::c_ulonglong,
     0x50c5404151057143 as libc::c_longlong as u64_0,
     0x690191c1d2c5d2c7 as libc::c_longlong as u64_0,
     0xa981d183b801b001 as libc::c_ulonglong,
     0xa801a80178016001 as libc::c_ulonglong,
     0x60018801a001a001 as libc::c_longlong as u64_0,
     0xa885a883c801c801 as libc::c_ulonglong,
     0x7801808388c1b1c7 as libc::c_longlong as u64_0,
     0xe34bca85a941a181 as libc::c_ulonglong,
     0x818560c168c1b207 as libc::c_ulonglong,
     0x60c158819a05b1c1 as libc::c_longlong as u64_0,
     0xca85ec53c1c7a081 as libc::c_ulonglong,
     0x9001b001c001c001 as libc::c_ulonglong,
     0xc801c801d001d001 as libc::c_ulonglong,
     0xd001d801e0019001 as libc::c_ulonglong,
     0x684191c99983ca45 as libc::c_longlong as u64_0,
     0x99819141ca85db89 as libc::c_ulonglong,
     0xeb8dd285c241d30d as libc::c_ulonglong,
     0x8a09820979c75081 as libc::c_ulonglong,
     0x79c959cb30815881 as libc::c_longlong as u64_0,
     0x8041b001c801a001 as libc::c_ulonglong,
     0x7001704180838001 as libc::c_longlong as u64_0,
     0xb801e001e0019081 as libc::c_ulonglong,
     0x6907924ba1c3a1c1 as libc::c_longlong as u64_0,
     0x68c18183b30fdb4d as libc::c_longlong as u64_0,
     0xec0fe349a1c1924b as libc::c_ulonglong,
     0x924b8a4b82096945 as libc::c_ulonglong,
     0x8a0d308300010001 as libc::c_ulonglong,
     0x70419881d105d187 as libc::c_longlong as u64_0,
     0xd1c7c143b187a0c3 as libc::c_ulonglong,
     0xd801e801e001b145 as libc::c_ulonglong,
     0x50c3514781c57943 as libc::c_longlong as u64_0,
     0x8a05aacbab0d9a47 as libc::c_ulonglong,
     0x920581c358818a09 as libc::c_ulonglong,
     0x8a4b714550415041 as libc::c_ulonglong,
     0x7147588328412041 as libc::c_longlong as u64_0,
     0x9105a987ca09da4b as libc::c_ulonglong,
     0xe2cdb943a987a083 as libc::c_ulonglong,
     0xe001e801e881a0c1 as libc::c_ulonglong,
     0x81437143aa036901 as libc::c_ulonglong,
     0x89c3920771017941 as libc::c_ulonglong,
     0x8183714168c18a09 as libc::c_ulonglong,
     0x7143508140015041 as libc::c_longlong as u64_0,
     0x7949690738433843 as libc::c_longlong as u64_0,
     0x70c5710568c360c3 as libc::c_longlong as u64_0,
     0x818938015001c801 as libc::c_ulonglong,
     0xe001d881c0419801 as libc::c_ulonglong,
     0x9101a18199817941 as libc::c_ulonglong,
     0x9205920579418181 as libc::c_ulonglong,
     0x8a0581c58207924b as libc::c_ulonglong,
     0x6101610379897147 as libc::c_longlong as u64_0,
     0x798b584348015083 as libc::c_longlong as u64_0,
     0x7907714779498189 as libc::c_longlong as u64_0,
     0x99cb9841c001d801 as libc::c_ulonglong,
     0xc883b905c001b001 as libc::c_ulonglong,
     0x78c379018181aac9 as libc::c_longlong as u64_0,
     0xa247898192037941 as libc::c_ulonglong,
     0x81c381c571416101 as libc::c_ulonglong,
     0x400169058a0d68c5 as libc::c_longlong as u64_0,
     0x5001700158419107 as libc::c_longlong as u64_0,
     0x7947814799479883 as libc::c_longlong as u64_0,
     0xb041d801d001b883 as libc::c_ulonglong,
     0xb9898083a0019801 as libc::c_ulonglong,
     0x704170c1a289c34f as libc::c_longlong as u64_0,
     0x91c370c189c389c5 as libc::c_ulonglong,
     0x92499249714358c1 as libc::c_ulonglong,
     0x5081714771496083 as libc::c_longlong as u64_0,
     0x68c5990799497107 as libc::c_longlong as u64_0,
     0x70419041b001c001 as libc::c_longlong as u64_0,
     0xd801d801a04170c5 as libc::c_ulonglong,
     0x6881504160018801 as libc::c_longlong as u64_0,
     0x78019183bb4fa289 as libc::c_longlong as u64_0,
     0x68817901aa8b9a49 as libc::c_longlong as u64_0,
     0x89c5928b79c779c7 as libc::c_ulonglong,
     0x8a0b798968c57949 as libc::c_ulonglong,
     0x814b58c5508348c5 as libc::c_ulonglong,
     0x8801b001b001a801 as libc::c_ulonglong,
     0xb841b90560c32801 as libc::c_ulonglong,
     0x5881920980c5a801 as libc::c_longlong as u64_0,
     0x9801a987b2cd8983 as libc::c_ulonglong,
     0x70c189c59a498183 as libc::c_longlong as u64_0,
     0x81c58a4969416101 as libc::c_ulonglong,
     0x714560c330431043 as libc::c_longlong as u64_0,
     0x1200138414883 as libc::c_longlong as u64_0,
     0xa801b80180016801 as libc::c_ulonglong,
     0xba4fa24d48c33041 as libc::c_ulonglong,
     0x81c792097041a801 as libc::c_ulonglong,
     0xa00190c189857943 as libc::c_ulonglong,
     0x81858185818581c5 as libc::c_ulonglong,
     0x82077185508158c1 as libc::c_ulonglong,
     0x6103588138411085 as libc::c_longlong as u64_0,
     0x2885284140018841 as libc::c_longlong as u64_0,
     0xa801880158018187 as libc::c_ulonglong,
     0xa28f798530412801 as libc::c_ulonglong,
     0x7143798378c3b001 as libc::c_longlong as u64_0,
     0xa801a143a24989c7 as libc::c_ulonglong,
     0x710168c181859a49 as libc::c_longlong as u64_0,
     0x81c5714361017145 as libc::c_ulonglong,
     0x79c7718771454081 as libc::c_longlong as u64_0,
     0x384158419041a001 as libc::c_longlong as u64_0];
static mut D_808C7000: [u8_0; 254] =
    [0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0];
// Initialized in run_static_initializers
static mut sInitChain: [InitChainEntry; 4] =
    [InitChainEntry{cont_type_0_offset_value: [0; 4],}; 4];
#[no_mangle]
pub unsafe extern "C" fn func_808C1190(mut arg0: *mut s16,
                                       mut arg1: *mut u8_0, mut arg2: s16) {
    if *arg1.offset(arg2 as isize) as libc::c_int != 0 as libc::c_int {
        *arg0.offset((arg2 as libc::c_int / 2 as libc::c_int) as isize) =
            0 as libc::c_int as s16
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_808C11D0(mut arg0: *mut s16,
                                       mut arg1: *mut u8_0, mut arg2: s16) {
    if *arg1.offset(arg2 as isize) as libc::c_int != 0 as libc::c_int {
        *arg0.offset(arg2 as isize) = 0 as libc::c_int as s16
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_808C1200(mut arg0: *mut s16,
                                       mut arg1: *mut u8_0, mut arg2: s16) {
    if *arg1.offset(arg2 as isize) as libc::c_int != 0 as libc::c_int {
        *arg0.offset(arg2 as isize) = 0 as libc::c_int as s16
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_808C1230(mut arg0: *mut s16,
                                       mut arg1: *mut u8_0, mut arg2: s16) {
    let mut index: s16 = 0;
    if *arg1.offset(arg2 as isize) as libc::c_int != 0 as libc::c_int {
        index =
            ((arg2 as libc::c_int & 0xf as libc::c_int) +
                 (arg2 as libc::c_int & 0xf0 as libc::c_int) *
                     2 as libc::c_int) as s16;
        *arg0.offset((index as libc::c_int + 16 as libc::c_int) as isize) =
            0 as libc::c_int as s16;
        *arg0.offset(index as isize) = 0 as libc::c_int as s16
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_808C1278(mut arg0: *mut s16,
                                       mut arg1: *mut u8_0, mut arg2: s16) {
    let mut index: s16 = 0;
    if *arg1.offset(arg2 as isize) as libc::c_int != 0 as libc::c_int {
        index =
            ((arg2 as libc::c_int & 0xf as libc::c_int) * 2 as libc::c_int +
                 (arg2 as libc::c_int & 0xf0 as libc::c_int) *
                     2 as libc::c_int) as s16;
        *arg0.offset((index as libc::c_int + 1 as libc::c_int) as isize) =
            0 as libc::c_int as s16;
        *arg0.offset(index as isize) = 0 as libc::c_int as s16
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_808C12C4(mut arg1: *mut u8_0, mut arg2: s16) {
    func_808C1190(gSegments[((object_kingdodongo_Tex_015890.as_mut_ptr() as
                                  u32_0) << 4 as libc::c_int >>
                                 28 as libc::c_int) as
                                usize].wrapping_add(object_kingdodongo_Tex_015890.as_mut_ptr()
                                                        as u32_0 &
                                                        0xffffff as
                                                            libc::c_int as
                                                            libc::c_uint).wrapping_add(0x80000000
                                                                                           as
                                                                                           libc::c_uint)
                      as *mut libc::c_void as *mut s16, arg1, arg2);
    func_808C1200(gSegments[((object_kingdodongo_Tex_017210.as_mut_ptr() as
                                  u32_0) << 4 as libc::c_int >>
                                 28 as libc::c_int) as
                                usize].wrapping_add(object_kingdodongo_Tex_017210.as_mut_ptr()
                                                        as u32_0 &
                                                        0xffffff as
                                                            libc::c_int as
                                                            libc::c_uint).wrapping_add(0x80000000
                                                                                           as
                                                                                           libc::c_uint)
                      as *mut libc::c_void as *mut s16, arg1, arg2);
    func_808C11D0(gSegments[((object_kingdodongo_Tex_015D90.as_mut_ptr() as
                                  u32_0) << 4 as libc::c_int >>
                                 28 as libc::c_int) as
                                usize].wrapping_add(object_kingdodongo_Tex_015D90.as_mut_ptr()
                                                        as u32_0 &
                                                        0xffffff as
                                                            libc::c_int as
                                                            libc::c_uint).wrapping_add(0x80000000
                                                                                           as
                                                                                           libc::c_uint)
                      as *mut libc::c_void as *mut s16, arg1, arg2);
    func_808C11D0(gSegments[((object_kingdodongo_Tex_016390.as_mut_ptr() as
                                  u32_0) << 4 as libc::c_int >>
                                 28 as libc::c_int) as
                                usize].wrapping_add(object_kingdodongo_Tex_016390.as_mut_ptr()
                                                        as u32_0 &
                                                        0xffffff as
                                                            libc::c_int as
                                                            libc::c_uint).wrapping_add(0x80000000
                                                                                           as
                                                                                           libc::c_uint)
                      as *mut libc::c_void as *mut s16, arg1, arg2);
    func_808C11D0(gSegments[((object_kingdodongo_Tex_016590.as_mut_ptr() as
                                  u32_0) << 4 as libc::c_int >>
                                 28 as libc::c_int) as
                                usize].wrapping_add(object_kingdodongo_Tex_016590.as_mut_ptr()
                                                        as u32_0 &
                                                        0xffffff as
                                                            libc::c_int as
                                                            libc::c_uint).wrapping_add(0x80000000
                                                                                           as
                                                                                           libc::c_uint)
                      as *mut libc::c_void as *mut s16, arg1, arg2);
    func_808C11D0(gSegments[((object_kingdodongo_Tex_016790.as_mut_ptr() as
                                  u32_0) << 4 as libc::c_int >>
                                 28 as libc::c_int) as
                                usize].wrapping_add(object_kingdodongo_Tex_016790.as_mut_ptr()
                                                        as u32_0 &
                                                        0xffffff as
                                                            libc::c_int as
                                                            libc::c_uint).wrapping_add(0x80000000
                                                                                           as
                                                                                           libc::c_uint)
                      as *mut libc::c_void as *mut s16, arg1, arg2);
    func_808C1230(gSegments[((object_kingdodongo_Tex_015990.as_mut_ptr() as
                                  u32_0) << 4 as libc::c_int >>
                                 28 as libc::c_int) as
                                usize].wrapping_add(object_kingdodongo_Tex_015990.as_mut_ptr()
                                                        as u32_0 &
                                                        0xffffff as
                                                            libc::c_int as
                                                            libc::c_uint).wrapping_add(0x80000000
                                                                                           as
                                                                                           libc::c_uint)
                      as *mut libc::c_void as *mut s16, arg1, arg2);
    func_808C1230(gSegments[((object_kingdodongo_Tex_015F90.as_mut_ptr() as
                                  u32_0) << 4 as libc::c_int >>
                                 28 as libc::c_int) as
                                usize].wrapping_add(object_kingdodongo_Tex_015F90.as_mut_ptr()
                                                        as u32_0 &
                                                        0xffffff as
                                                            libc::c_int as
                                                            libc::c_uint).wrapping_add(0x80000000
                                                                                           as
                                                                                           libc::c_uint)
                      as *mut libc::c_void as *mut s16, arg1, arg2);
    func_808C1278(gSegments[((object_kingdodongo_Tex_016990.as_mut_ptr() as
                                  u32_0) << 4 as libc::c_int >>
                                 28 as libc::c_int) as
                                usize].wrapping_add(object_kingdodongo_Tex_016990.as_mut_ptr()
                                                        as u32_0 &
                                                        0xffffff as
                                                            libc::c_int as
                                                            libc::c_uint).wrapping_add(0x80000000
                                                                                           as
                                                                                           libc::c_uint)
                      as *mut libc::c_void as *mut s16, arg1, arg2);
    func_808C1278(gSegments[((object_kingdodongo_Tex_016E10.as_mut_ptr() as
                                  u32_0) << 4 as libc::c_int >>
                                 28 as libc::c_int) as
                                usize].wrapping_add(object_kingdodongo_Tex_016E10.as_mut_ptr()
                                                        as u32_0 &
                                                        0xffffff as
                                                            libc::c_int as
                                                            libc::c_uint).wrapping_add(0x80000000
                                                                                           as
                                                                                           libc::c_uint)
                      as *mut libc::c_void as *mut s16, arg1, arg2);
}
#[no_mangle]
pub unsafe extern "C" fn func_808C1554(mut arg0: *mut libc::c_void,
                                       mut floorTex: *mut libc::c_void,
                                       mut arg2: s32, mut arg3: f32_0) {
    let mut temp_s3: *mut u16_0 =
        gSegments[((arg0 as u32_0) << 4 as libc::c_int >> 28 as libc::c_int)
                      as
                      usize].wrapping_add(arg0 as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut u16_0;
    let mut temp_s1: *mut u16_0 =
        gSegments[((floorTex as u32_0) << 4 as libc::c_int >>
                       28 as libc::c_int) as
                      usize].wrapping_add(floorTex as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut u16_0;
    let mut i: s16 = 0;
    let mut i2: s16 = 0;
    let mut sp54: [u16_0; 2048] = [0; 2048];
    let mut temp: s16 = 0;
    let mut temp2: s16 = 0;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < 2048 as libc::c_int {
        temp =
            (sinf((i as libc::c_int / 32 as libc::c_int +
                       (arg2 as libc::c_float * 50.0f32 / 100.0f32) as s16 as
                           libc::c_int & 0x1f as libc::c_int) as libc::c_float
                      *
                      (3.14159265358979323846f32 /
                           16 as libc::c_int as libc::c_float)) * arg3) as
                s16;
        i2 = 0 as libc::c_int as s16;
        while (i2 as libc::c_int) < 32 as libc::c_int {
            sp54[(i as libc::c_int +
                      (temp as libc::c_int + i2 as libc::c_int &
                           0x1f as libc::c_int)) as usize] =
                *temp_s1.offset((i as libc::c_int + i2 as libc::c_int) as
                                    isize);
            i2 += 1
        }
        i = (i as libc::c_int + 32 as libc::c_int) as s16
    }
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < 32 as libc::c_int {
        temp =
            (sinf((i as libc::c_int +
                       (arg2 as libc::c_float * 80.0f32 / 100.0f32) as s16 as
                           libc::c_int & 0x1f as libc::c_int) as libc::c_float
                      *
                      (3.14159265358979323846f32 /
                           16 as libc::c_int as libc::c_float)) * arg3) as
                s16;
        temp = (temp as libc::c_int * 32 as libc::c_int) as s16;
        i2 = 0 as libc::c_int as s16;
        while (i2 as libc::c_int) < 2048 as libc::c_int {
            temp2 =
                (temp as libc::c_int + i2 as libc::c_int &
                     0x7ff as libc::c_int) as s16;
            *temp_s3.offset((i as libc::c_int + temp2 as libc::c_int) as
                                isize) =
                sp54[(i as libc::c_int + i2 as libc::c_int) as usize];
            i2 = (i2 as libc::c_int + 32 as libc::c_int) as s16
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_808C17C8(mut globalCtx: *mut GlobalContext,
                                       mut arg1: *mut Vec3f,
                                       mut arg2: *mut Vec3f,
                                       mut arg3: *mut Vec3f, mut arg4: f32_0,
                                       mut arg5: s16) {
    let mut i: s16 = 0;
    let mut eff: *mut BossDodongoEffect =
        (*globalCtx).specialEffects as *mut BossDodongoEffect;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < arg5 as libc::c_int {
        if (*eff).unk_24 as libc::c_int == 0 as libc::c_int {
            (*eff).unk_24 = 1 as libc::c_int as u8_0;
            (*eff).unk_00 = *arg1;
            (*eff).unk_0C = *arg2;
            (*eff).unk_18 = *arg3;
            (*eff).unk_2C = arg4 / 1000.0f32;
            (*eff).alpha = 255 as libc::c_int as s16;
            (*eff).unk_25 = Rand_ZeroFloat(10.0f32) as s16 as u8_0;
            break ;
        } else { i += 1; eff = eff.offset(1) }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossDodongo_AteExplosive(mut this: *mut BossDodongo,
                                                  mut globalCtx:
                                                      *mut GlobalContext)
 -> s32 {
    let mut dx: f32_0 = 0.;
    let mut dy: f32_0 = 0.;
    let mut dz: f32_0 = 0.;
    let mut currentExplosive: *mut Actor =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_EXPLOSIVE as libc::c_int as
                                             usize].head;
    let mut thisx: *mut Actor = &mut (*this).actor;
    while !currentExplosive.is_null() {
        if currentExplosive == thisx {
            currentExplosive = (*currentExplosive).next
        } else {
            dx = (*currentExplosive).world.pos.x - (*this).mouthPos.x;
            dy = (*currentExplosive).world.pos.y - (*this).mouthPos.y;
            dz = (*currentExplosive).world.pos.z - (*this).mouthPos.z;
            if fabsf(dx) < 40.0f32 && fabsf(dy) < 40.0f32 &&
                   fabsf(dz) < 40.0f32 {
                Actor_Kill(currentExplosive);
                return 1 as libc::c_int
            }
            currentExplosive = (*currentExplosive).next
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BossDodongo_Init(mut thisx: *mut Actor,
                                          mut globalCtx: *mut GlobalContext) {
    let mut this: *mut BossDodongo = thisx as *mut BossDodongo;
    let mut i: s16 = 0;
    let mut temp_s1_3: *mut u16_0 = 0 as *mut u16_0;
    let mut temp_s2: *mut u16_0 = 0 as *mut u16_0;
    let mut temp_v0: u32_0 = 0;
    (*globalCtx).specialEffects =
        &mut (*this).effects as *mut [BossDodongoEffect; 80] as
            *mut libc::c_void;
    Actor_ProcessInitChain(&mut (*this).actor, sInitChain.as_mut_ptr());
    ActorShape_Init(&mut (*this).actor.shape, 9200.0f32,
                    Some(ActorShadow_DrawCircle as
                             unsafe extern "C" fn(_: *mut Actor,
                                                  _: *mut Lights,
                                                  _: *mut GlobalContext)
                                 -> ()), 250.0f32);
    Actor_SetScale(&mut (*this).actor, 0.01f32);
    SkelAnime_Init(globalCtx, &mut (*this).skelAnime,
                   &mut object_kingdodongo_Skel_01B310,
                   &mut object_kingdodongo_Anim_00F0D8, 0 as *mut Vec3s,
                   0 as *mut Vec3s, 0 as libc::c_int);
    Animation_PlayLoop(&mut (*this).skelAnime,
                       &mut object_kingdodongo_Anim_00F0D8);
    (*this).unk_1F8 = 1.0f32;
    BossDodongo_SetupIntroCutscene(this, globalCtx);
    (*this).health = 12 as libc::c_int as s16;
    (*this).colorFilterMin = 995.0f32;
    (*this).actor.colChkInfo.mass = 0xff as libc::c_int as u8_0;
    (*this).colorFilterMax = 1000.0f32;
    (*this).unk_224 = 2.0f32;
    (*this).unk_228 = 9200.0f32;
    Collider_InitJntSph(globalCtx, &mut (*this).collider);
    Collider_SetJntSph(globalCtx, &mut (*this).collider, &mut (*this).actor,
                       &mut sJntSphInit, (*this).items.as_mut_ptr());
    if Flags_GetClear(globalCtx, (*globalCtx).roomCtx.curRoom.num as s32) != 0
       {
        // KD is dead
        temp_s1_3 =
            gSegments[((gDodongosCavernBossLavaFloorTex.as_mut_ptr() as u32_0)
                           << 4 as libc::c_int >> 28 as libc::c_int) as
                          usize].wrapping_add(gDodongosCavernBossLavaFloorTex.as_mut_ptr()
                                                  as u32_0 &
                                                  0xffffff as libc::c_int as
                                                      libc::c_uint).wrapping_add(0x80000000
                                                                                     as
                                                                                     libc::c_uint)
                as *mut libc::c_void as *mut u16_0;
        temp_s2 =
            gSegments[((sLavaFloorRockTex.as_mut_ptr() as u32_0) <<
                           4 as libc::c_int >> 28 as libc::c_int) as
                          usize].wrapping_add(sLavaFloorRockTex.as_mut_ptr()
                                                  as u32_0 &
                                                  0xffffff as libc::c_int as
                                                      libc::c_uint).wrapping_add(0x80000000
                                                                                     as
                                                                                     libc::c_uint)
                as *mut libc::c_void as *mut u16_0;
        Actor_Kill(&mut (*this).actor);
        Actor_SpawnAsChild(&mut (*globalCtx).actorCtx, &mut (*this).actor,
                           globalCtx, ACTOR_DOOR_WARP1 as libc::c_int as s16,
                           -890.0f32, -1523.76f32, -3304.0f32,
                           0 as libc::c_int as s16, 0 as libc::c_int as s16,
                           0 as libc::c_int as s16,
                           WARP_DUNGEON_CHILD as libc::c_int as s16);
        Actor_Spawn(&mut (*globalCtx).actorCtx, globalCtx,
                    ACTOR_BG_BREAKWALL as libc::c_int as s16, -890.0f32,
                    -1523.76f32, -3304.0f32, 0 as libc::c_int as s16,
                    0 as libc::c_int as s16, 0 as libc::c_int as s16,
                    0x6000 as libc::c_int as s16);
        Actor_Spawn(&mut (*globalCtx).actorCtx, globalCtx,
                    ACTOR_ITEM_B_HEART as libc::c_int as s16, -690.0f32,
                    -1523.76f32, -3304.0f32, 0 as libc::c_int as s16,
                    0 as libc::c_int as s16, 0 as libc::c_int as s16,
                    0 as libc::c_int as s16);
        i = 0 as libc::c_int as s16;
        while (i as libc::c_int) < 2048 as libc::c_int {
            temp_v0 = i as u32_0;
            *temp_s1_3.offset(temp_v0 as isize) =
                *temp_s2.offset(temp_v0 as isize);
            i += 1
        }
    }
    (*this).actor.flags &=
        !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn BossDodongo_Destroy(mut thisx: *mut Actor,
                                             mut globalCtx:
                                                 *mut GlobalContext) {
    let mut this: *mut BossDodongo = thisx as *mut BossDodongo;
    SkelAnime_Free(&mut (*this).skelAnime, globalCtx);
    Collider_DestroyJntSph(globalCtx, &mut (*this).collider);
}
#[no_mangle]
pub unsafe extern "C" fn BossDodongo_SetupIntroCutscene(mut this:
                                                            *mut BossDodongo,
                                                        mut globalCtx:
                                                            *mut GlobalContext) {
    let mut frames: s16 =
        Animation_GetLastFrame(&mut object_kingdodongo_Anim_00F0D8 as
                                   *mut AnimationHeader as *mut libc::c_void);
    Animation_Change(&mut (*this).skelAnime,
                     &mut object_kingdodongo_Anim_00F0D8, 1.0f32, 0.0f32,
                     frames as f32_0, ANIMMODE_LOOP as libc::c_int as u8_0,
                     -10.0f32);
    (*this).actionFunc =
        Some(BossDodongo_IntroCutscene as
                 unsafe extern "C" fn(_: *mut BossDodongo,
                                      _: *mut GlobalContext) -> ());
    (*this).csState = 0 as libc::c_int as s16;
    (*this).unk_1BC = 1 as libc::c_int as s16;
}
#[no_mangle]
pub unsafe extern "C" fn BossDodongo_IntroCutscene(mut this: *mut BossDodongo,
                                                   mut globalCtx:
                                                       *mut GlobalContext) {
    let mut phi_f0: f32_0 = 0.;
    let mut camera: *mut Camera = 0 as *mut Camera;
    let mut player: *mut Player = 0 as *mut Player;
    let mut sp60: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp54: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp48: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    camera = Gameplay_GetCamera(globalCtx, 0 as libc::c_int as s16);
    if (*this).unk_196 as libc::c_int != 0 as libc::c_int {
        (*this).unk_196 -= 1
    }
    if (*this).unk_198 as libc::c_int != 0 as libc::c_int {
        (*this).unk_198 -= 1
    }
    if (*this).unk_19A as libc::c_int != 0 as libc::c_int {
        (*this).unk_19A -= 1
    }
    let mut current_block_142: u64;
    match (*this).csState as libc::c_int {
        0 => {
            if (*player).actor.world.pos.y < -1223.76f32 {
                (*this).csState = 1 as libc::c_int as s16;
                (*this).actor.world.pos.x = -1390.0f32;
                (*this).actor.world.pos.z = -3374.0f32;
                (*this).unk_1A0 = 1 as libc::c_int as s16
            }
            current_block_142 = 12890877304563811856;
        }
        1 => {
            func_80064520(globalCtx, &mut (*globalCtx).csCtx);
            func_8002DF54(globalCtx, &mut (*this).actor,
                          1 as libc::c_int as u8_0);
            Gameplay_ClearAllSubCameras(globalCtx);
            (*this).cutsceneCamera = Gameplay_CreateSubCamera(globalCtx);
            Gameplay_ChangeCameraStatus(globalCtx, 0 as libc::c_int as s16,
                                        1 as libc::c_int as s16);
            Gameplay_ChangeCameraStatus(globalCtx, (*this).cutsceneCamera,
                                        7 as libc::c_int as s16);
            (*this).csState = 2 as libc::c_int as s16;
            (*this).unk_196 = 0x3c as libc::c_int as s16;
            (*this).unk_198 = 160 as libc::c_int as s16;
            (*player).actor.world.pos.y = -1023.76f32;
            (*this).cameraEye.y =
                (*player).actor.world.pos.y - 480.0f32 + 50.0f32;
            current_block_142 = 18349273455539036228;
        }
        2 => { current_block_142 = 18349273455539036228; }
        3 => {
            BossDodongo_Walk(this, globalCtx);
            Math_SmoothStepToF(&mut (*this).unk_20C,
                               sinf((*this).unk_19E as libc::c_int as
                                        libc::c_float * 0.05f32) * 0.1f32,
                               1.0f32, 0.01f32, 0.0f32);
            Math_SmoothStepToF(&mut (*this).cameraEye.x,
                               (*this).vec.x + 90.0f32, 0.2f32,
                               (*this).unk_204 * 20.0f32, 0.0f32);
            Math_SmoothStepToF(&mut (*this).cameraEye.y,
                               (*this).vec.y + 50.0f32, 0.2f32,
                               (*this).unk_204 * 20.0f32, 0.0f32);
            Math_SmoothStepToF(&mut (*this).cameraEye.z, (*this).vec.z,
                               0.2f32, (*this).unk_204 * 20.0f32, 0.0f32);
            Math_SmoothStepToF(&mut (*this).cameraAt.y,
                               (*this).vec.y - 10.0f32, 0.2f32,
                               (*this).unk_204 * 20.0f32, 0.0f32);
            Math_SmoothStepToF(&mut (*this).unk_204, 1.0f32, 1.0f32, 0.02f32,
                               0.0f32);
            if fabsf((*player).actor.world.pos.x - (*this).actor.world.pos.x)
                   < 200.0f32 {
                (*this).csState = 4 as libc::c_int as s16;
                (*this).unk_196 = 0x1e as libc::c_int as s16;
                (*this).unk_198 = 0x96 as libc::c_int as s16;
                (*this).unk_204 = 0.0f32;
                Animation_Change(&mut (*this).skelAnime,
                                 &mut object_kingdodongo_Anim_008EEC, 1.0f32,
                                 0.0f32,
                                 Animation_GetLastFrame(&mut object_kingdodongo_Anim_008EEC
                                                            as
                                                            *mut AnimationHeader
                                                            as
                                                            *mut libc::c_void)
                                     as f32_0,
                                 ANIMMODE_ONCE as libc::c_int as u8_0,
                                 -5.0f32);
            }
            current_block_142 = 12890877304563811856;
        }
        4 => {
            Math_SmoothStepToF(&mut (*this).unk_20C, 0.0f32, 1.0f32, 0.01f32,
                               0.0f32);
            if gSaveContext.eventChkInf[7 as libc::c_int as usize] as
                   libc::c_int & 2 as libc::c_int != 0 {
                phi_f0 = -50.0f32
            } else { phi_f0 = 0.0f32 }
            Math_SmoothStepToF(&mut (*this).cameraEye.x,
                               (*player).actor.world.pos.x + phi_f0 + 70.0f32,
                               0.2f32, (*this).unk_204 * 20.0f32, 0.0f32);
            Math_SmoothStepToF(&mut (*this).cameraEye.y,
                               (*player).actor.world.pos.y + 10.0f32, 0.2f32,
                               (*this).unk_204 * 20.0f32, 0.0f32);
            Math_SmoothStepToF(&mut (*this).cameraEye.z,
                               (*player).actor.world.pos.z - 60.0f32, 0.2f32,
                               (*this).unk_204 * 20.0f32, 0.0f32);
            Math_SmoothStepToF(&mut (*this).cameraAt.x, (*this).vec.x, 0.2f32,
                               (*this).unk_204 * 20.0f32, 0.0f32);
            Math_SmoothStepToF(&mut (*this).cameraAt.y, (*this).vec.y, 0.2f32,
                               (*this).unk_204 * 20.0f32, 0.0f32);
            Math_SmoothStepToF(&mut (*this).cameraAt.z, (*this).vec.z, 0.2f32,
                               (*this).unk_204 * 20.0f32, 0.0f32);
            Math_SmoothStepToF(&mut (*this).unk_204, 1.0f32, 1.0f32, 0.02f32,
                               0.0f32);
            if (*this).unk_196 as libc::c_int == 0 as libc::c_int {
                SkelAnime_Update(&mut (*this).skelAnime);
                Math_SmoothStepToF(&mut (*this).unk_208, 0.05f32, 1.0f32,
                                   0.005f32, 0.0f32);
            }
            if (*this).unk_198 as libc::c_int == 0x64 as libc::c_int {
                Audio_PlayActorSound2(&mut (*this).actor,
                                      0x3852 as libc::c_int as u16_0);
            }
            if (*this).unk_198 as libc::c_int == 0x5a as libc::c_int {
                if gSaveContext.eventChkInf[7 as libc::c_int as usize] as
                       libc::c_int & 2 as libc::c_int == 0 {
                    TitleCard_InitBossName(globalCtx,
                                           &mut (*globalCtx).actorCtx.titleCtx,
                                           gSegments[((&mut object_kingdodongo_Blob_017410
                                                           as *mut [u8_0; 0]
                                                           as u32_0) <<
                                                          4 as libc::c_int >>
                                                          28 as libc::c_int)
                                                         as
                                                         usize].wrapping_add(&mut object_kingdodongo_Blob_017410
                                                                                 as
                                                                                 *mut [u8_0; 0]
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
                Audio_QueueSeqCmd(((SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                                       24 as libc::c_int |
                                       0x6b as libc::c_int) as u32_0);
            }
            if (*this).unk_198 as libc::c_int == 0 as libc::c_int {
                (*camera).eye = (*this).cameraEye;
                (*camera).eyeNext = (*this).cameraEye;
                (*camera).at = (*this).cameraAt;
                func_800C08AC(globalCtx, (*this).cutsceneCamera,
                              0 as libc::c_int as s16);
                (*this).cutsceneCamera = 0 as libc::c_int as s16;
                func_80064534(globalCtx, &mut (*globalCtx).csCtx);
                func_8002DF54(globalCtx, &mut (*this).actor,
                              7 as libc::c_int as u8_0);
                BossDodongo_SetupWalk(this);
                (*this).unk_1DA = 50 as libc::c_int as s16;
                (*this).unk_1BC = 0 as libc::c_int as s16;
                (*player).actor.shape.rot.y = -(0x4002 as libc::c_int) as s16;
                gSaveContext.eventChkInf[7 as libc::c_int as usize] =
                    (gSaveContext.eventChkInf[7 as libc::c_int as usize] as
                         libc::c_int | 2 as libc::c_int) as u16_0
            }
            current_block_142 = 12890877304563811856;
        }
        _ => { current_block_142 = 12890877304563811856; }
    }
    match current_block_142 {
        18349273455539036228 => {
            if (*this).unk_198 as libc::c_int >= 131 as libc::c_int {
                (*player).actor.world.pos.x = -890.0f32;
                (*player).actor.world.pos.z = -2804.0f32;
                (*player).actor.speedXZ = 0.0f32;
                (*player).actor.world.rot.y = 0x3fff as libc::c_int as s16;
                (*player).actor.shape.rot.y = (*player).actor.world.rot.y;
                (*this).cameraEye.x = -890.0f32;
                (*this).cameraEye.z = (*player).actor.world.pos.z - 100.0f32;
                (*this).cameraAt.x = (*player).actor.world.pos.x;
                (*this).cameraAt.y = (*player).actor.world.pos.y + 20.0f32;
                (*this).cameraAt.z = (*player).actor.world.pos.z
            }
            if (*this).unk_198 as libc::c_int == 110 as libc::c_int {
                func_8002DF54(globalCtx, &mut (*this).actor,
                              9 as libc::c_int as u8_0);
            }
            if (*this).unk_198 as libc::c_int == 5 as libc::c_int {
                func_8002DF54(globalCtx, &mut (*this).actor,
                              12 as libc::c_int as u8_0);
            }
            if ((*this).unk_198 as libc::c_int) < 6 as libc::c_int {
                (*player).actor.shape.rot.y = -(0x4001 as libc::c_int) as s16
            } else {
                (*player).actor.shape.rot.y = 0x3fff as libc::c_int as s16
            }
            if ((*this).unk_198 as libc::c_int) < 60 as libc::c_int {
                (*this).unk_1BC = 1 as libc::c_int as s16
            } else { (*this).unk_1BC = 2 as libc::c_int as s16 }
            BossDodongo_Walk(this, globalCtx);
            if (*this).unk_196 as libc::c_int == 1 as libc::c_int {
                Audio_QueueSeqCmd(((0x1 as libc::c_int) << 28 as libc::c_int |
                                       (SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                                           24 as libc::c_int |
                                       0x100ff as libc::c_int) as u32_0);
            }
            if (*this).unk_196 as libc::c_int == 0 as libc::c_int {
                Math_SmoothStepToF(&mut (*this).cameraEye.x,
                                   (*this).vec.x + 30.0f32, 0.2f32,
                                   (*this).unk_204 * 20.0f32, 0.0f32);
                Math_SmoothStepToF(&mut (*this).cameraEye.y, (*this).vec.y,
                                   0.2f32, (*this).unk_204 * 20.0f32, 0.0f32);
                Math_SmoothStepToF(&mut (*this).cameraEye.z,
                                   (*this).vec.z + 10.0f32, 0.2f32,
                                   (*this).unk_204 * 20.0f32, 0.0f32);
                Math_SmoothStepToF(&mut (*this).unk_204, 1.0f32, 1.0f32,
                                   0.02f32, 0.0f32);
            } else {
                (*this).cameraAt.x = (*player).actor.world.pos.x;
                (*this).cameraAt.y = (*player).actor.world.pos.y + 20.0f32;
                (*this).cameraAt.z = (*player).actor.world.pos.z
            }
            if gSaveContext.eventChkInf[7 as libc::c_int as usize] as
                   libc::c_int & 2 as libc::c_int != 0 {
                if (*this).unk_198 as libc::c_int == 100 as libc::c_int {
                    (*this).actor.world.pos.x = -1114.0f32;
                    (*this).actor.world.pos.z = -2804.0f32;
                    (*this).actor.world.rot.y = 0x3fff as libc::c_int as s16;
                    (*this).unk_1A2 = 0 as libc::c_int as s16;
                    (*this).unk_1A0 = 2 as libc::c_int as s16;
                    (*this).csState = 4 as libc::c_int as s16;
                    (*this).unk_196 = 30 as libc::c_int as s16;
                    (*this).unk_198 = 150 as libc::c_int as s16;
                    (*this).unk_204 = 0.0f32;
                    Animation_Change(&mut (*this).skelAnime,
                                     &mut object_kingdodongo_Anim_008EEC,
                                     1.0f32, 0.0f32,
                                     Animation_GetLastFrame(&mut object_kingdodongo_Anim_008EEC
                                                                as
                                                                *mut AnimationHeader
                                                                as
                                                                *mut libc::c_void)
                                         as f32_0,
                                     ANIMMODE_ONCE as libc::c_int as u8_0,
                                     0.0f32);
                    SkelAnime_Update(&mut (*this).skelAnime);
                }
            } else if (*this).unk_198 as libc::c_int == 0 as libc::c_int {
                (*this).csState = 3 as libc::c_int as s16;
                (*this).unk_19E = 0x14 as libc::c_int as s16;
                (*this).unk_204 = 0.0f32
            }
        }
        _ => { }
    }
    if (*this).cutsceneCamera as libc::c_int != 0 as libc::c_int {
        if (*this).unk_1B6 as libc::c_int != 0 as libc::c_int {
            (*this).unk_1B6 -= 1
        }
        sp60.x = (*this).cameraEye.x;
        phi_f0 =
            sinf((*this).unk_1B6 as libc::c_int as libc::c_float * 3.1415f32 *
                     90.0f32 / 180.0f32);
        sp60.y =
            (*this).unk_1B6 as libc::c_int as libc::c_float * phi_f0 * 0.7f32
                + (*this).cameraEye.y;
        sp60.z = (*this).cameraEye.z;
        sp54.x = (*this).cameraAt.x;
        phi_f0 =
            sinf((*this).unk_1B6 as libc::c_int as libc::c_float * 3.1415f32 *
                     90.0f32 / 180.0f32);
        sp54.y =
            (*this).unk_1B6 as libc::c_int as libc::c_float * phi_f0 * 0.7f32
                + (*this).cameraAt.y;
        sp54.z = (*this).cameraAt.z;
        sp48.x = (*this).unk_20C;
        sp48.y = 1.0f32;
        sp48.z = (*this).unk_20C;
        Gameplay_CameraSetAtEyeUp(globalCtx, (*this).cutsceneCamera,
                                  &mut sp54, &mut sp60, &mut sp48);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossDodongo_SetupDamaged(mut this:
                                                      *mut BossDodongo) {
    if (*this).actionFunc !=
           Some(BossDodongo_Damaged as
                    unsafe extern "C" fn(_: *mut BossDodongo,
                                         _: *mut GlobalContext) -> ()) {
        Animation_Change(&mut (*this).skelAnime,
                         &mut object_kingdodongo_Anim_001074, 1.0f32, 0.0f32,
                         Animation_GetLastFrame(&mut object_kingdodongo_Anim_001074
                                                    as *mut AnimationHeader as
                                                    *mut libc::c_void) as
                             f32_0, ANIMMODE_ONCE as libc::c_int as u8_0,
                         -5.0f32);
        (*this).actionFunc =
            Some(BossDodongo_Damaged as
                     unsafe extern "C" fn(_: *mut BossDodongo,
                                          _: *mut GlobalContext) -> ())
    }
    (*this).unk_1DA = 100 as libc::c_int as s16;
}
#[no_mangle]
pub unsafe extern "C" fn BossDodongo_SetupExplode(mut this:
                                                      *mut BossDodongo) {
    Animation_Change(&mut (*this).skelAnime,
                     &mut object_kingdodongo_Anim_00E848, 1.0f32, 0.0f32,
                     Animation_GetLastFrame(&mut object_kingdodongo_Anim_00E848
                                                as *mut AnimationHeader as
                                                *mut libc::c_void) as f32_0,
                     ANIMMODE_ONCE as libc::c_int as u8_0, -5.0f32);
    (*this).actionFunc =
        Some(BossDodongo_Explode as
                 unsafe extern "C" fn(_: *mut BossDodongo,
                                      _: *mut GlobalContext) -> ());
    (*this).unk_1B0 = 10 as libc::c_int as s16;
    (*this).unk_1C0 = 2 as libc::c_int as s16;
    (*this).unk_1DA = 35 as libc::c_int as s16;
    (*this).unk_1FC = 50.0f32;
    (*this).unk_200 = 300.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn BossDodongo_SetupWalk(mut this: *mut BossDodongo) {
    Animation_Change(&mut (*this).skelAnime,
                     &mut object_kingdodongo_Anim_01D934, 1.0f32, 0.0f32,
                     Animation_GetLastFrame(&mut object_kingdodongo_Anim_01D934
                                                as *mut AnimationHeader as
                                                *mut libc::c_void) as f32_0,
                     ANIMMODE_ONCE as libc::c_int as u8_0, -10.0f32);
    (*this).unk_1AA = 0 as libc::c_int as s16;
    (*this).actionFunc =
        Some(BossDodongo_Walk as
                 unsafe extern "C" fn(_: *mut BossDodongo,
                                      _: *mut GlobalContext) -> ());
    (*this).unk_1DA = 0 as libc::c_int as s16;
    (*this).actor.flags |=
        ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
    (*this).unk_1E4 = 0.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn BossDodongo_SetupRoll(mut this: *mut BossDodongo) {
    Animation_Change(&mut (*this).skelAnime,
                     &mut object_kingdodongo_Anim_00DF38, 1.0f32, 0.0f32,
                     59.0f32, ANIMMODE_ONCE as libc::c_int as u8_0, -5.0f32);
    (*this).actionFunc =
        Some(BossDodongo_Roll as
                 unsafe extern "C" fn(_: *mut BossDodongo,
                                      _: *mut GlobalContext) -> ());
    (*this).numWallCollisions = 0 as libc::c_int as s16;
    (*this).unk_1DA = 27 as libc::c_int as s16;
}
#[no_mangle]
pub unsafe extern "C" fn BossDodongo_SetupBlowFire(mut this:
                                                       *mut BossDodongo) {
    (*this).actor.speedXZ = 0.0f32;
    (*this).unk_1E4 = 0.0f32;
    Animation_Change(&mut (*this).skelAnime,
                     &mut object_kingdodongo_Anim_0061D4, 1.0f32, 0.0f32,
                     Animation_GetLastFrame(&mut object_kingdodongo_Anim_0061D4
                                                as *mut AnimationHeader as
                                                *mut libc::c_void) as f32_0,
                     ANIMMODE_ONCE as libc::c_int as u8_0, 0.0f32);
    (*this).actionFunc =
        Some(BossDodongo_BlowFire as
                 unsafe extern "C" fn(_: *mut BossDodongo,
                                      _: *mut GlobalContext) -> ());
    (*this).unk_1DA = 50 as libc::c_int as s16;
    (*this).unk_1AE = 0 as libc::c_int as s16;
}
#[no_mangle]
pub unsafe extern "C" fn BossDodongo_SetupInhale(mut this: *mut BossDodongo) {
    (*this).actor.speedXZ = 0.0f32;
    Animation_Change(&mut (*this).skelAnime,
                     &mut object_kingdodongo_Anim_008EEC, 1.0f32, 0.0f32,
                     Animation_GetLastFrame(&mut object_kingdodongo_Anim_008EEC
                                                as *mut AnimationHeader as
                                                *mut libc::c_void) as f32_0,
                     ANIMMODE_ONCE as libc::c_int as u8_0, -5.0f32);
    (*this).actionFunc =
        Some(BossDodongo_Inhale as
                 unsafe extern "C" fn(_: *mut BossDodongo,
                                      _: *mut GlobalContext) -> ());
    (*this).unk_1DA = 100 as libc::c_int as s16;
    (*this).unk_1AC = 0 as libc::c_int as s16;
    (*this).unk_1E2 = 1 as libc::c_int as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn BossDodongo_Damaged(mut this: *mut BossDodongo,
                                             mut globalCtx:
                                                 *mut GlobalContext) {
    SkelAnime_Update(&mut (*this).skelAnime);
    Math_SmoothStepToF(&mut (*this).unk_1F8, 1.0f32, 0.5f32, 0.02f32,
                       0.001f32);
    Math_SmoothStepToF(&mut (*this).unk_208, 0.05f32, 1.0f32, 0.005f32,
                       0.0f32);
    if Animation_OnFrame(&mut (*this).skelAnime,
                         Animation_GetLastFrame(&mut object_kingdodongo_Anim_001074
                                                    as *mut AnimationHeader as
                                                    *mut libc::c_void) as
                             f32_0) != 0 {
        BossDodongo_SetupRoll(this);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossDodongo_Explode(mut this: *mut BossDodongo,
                                             mut globalCtx:
                                                 *mut GlobalContext) {
    static mut dustPrimColor: Color_RGBA8 =
        {
            let mut init =
                Color_RGBA8{r: 255 as libc::c_int as u8_0,
                            g: 255 as libc::c_int as u8_0,
                            b: 0 as libc::c_int as u8_0,
                            a: 255 as libc::c_int as u8_0,};
            init
        };
    static mut dustEnvColor: Color_RGBA8 =
        {
            let mut init =
                Color_RGBA8{r: 255 as libc::c_int as u8_0,
                            g: 10 as libc::c_int as u8_0,
                            b: 0 as libc::c_int as u8_0,
                            a: 255 as libc::c_int as u8_0,};
            init
        };
    let mut pad: s16 = 0;
    let mut dustVel: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut dustAcell: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut dustPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut i: s16 = 0;
    Math_SmoothStepToF(&mut (*this).unk_208, 0.05f32, 1.0f32, 0.005f32,
                       0.0f32);
    SkelAnime_Update(&mut (*this).skelAnime);
    if (*this).unk_1DA as libc::c_int == 0 as libc::c_int {
        i = 0 as libc::c_int as s16;
        while (i as libc::c_int) < 30 as libc::c_int {
            dustVel.x = Rand_CenteredFloat(20.0f32);
            dustVel.y = Rand_CenteredFloat(20.0f32);
            dustVel.z = Rand_CenteredFloat(20.0f32);
            dustAcell.x = dustVel.x * -0.1f32;
            dustAcell.y = dustVel.y * -0.1f32;
            dustAcell.z = dustVel.z * -0.1f32;
            dustPos.x = (*this).actor.world.pos.x + dustVel.x * 3.0f32;
            dustPos.y =
                (*this).actor.world.pos.y + 90.0f32 + dustVel.y * 3.0f32;
            dustPos.z = (*this).actor.world.pos.z + dustVel.z * 3.0f32;
            func_8002836C(globalCtx, &mut dustPos, &mut dustVel,
                          &mut dustAcell, &mut dustPrimColor,
                          &mut dustEnvColor, 500 as libc::c_int as s16,
                          10 as libc::c_int as s16, 10 as libc::c_int as s16);
            i += 1
        }
        Animation_Change(&mut (*this).skelAnime,
                         &mut object_kingdodongo_Anim_004E0C, 1.0f32, 0.0f32,
                         Animation_GetLastFrame(&mut object_kingdodongo_Anim_004E0C
                                                    as *mut AnimationHeader as
                                                    *mut libc::c_void) as
                             f32_0, ANIMMODE_ONCE as libc::c_int as u8_0,
                         -5.0f32);
        (*this).actionFunc =
            Some(BossDodongo_LayDown as
                     unsafe extern "C" fn(_: *mut BossDodongo,
                                          _: *mut GlobalContext) -> ());
        Audio_PlayActorSound2(&mut (*this).actor,
                              0x180e as libc::c_int as u16_0);
        Audio_PlayActorSound2(&mut (*this).actor,
                              0x3806 as libc::c_int as u16_0);
        func_80033E88(&mut (*this).actor, globalCtx, 4 as libc::c_int as s16,
                      10 as libc::c_int as s16);
        (*this).health =
            ((*this).health as libc::c_int - 2 as libc::c_int) as s16;
        // make sure not to die from the bomb explosion
        if (*this).health as libc::c_int <= 0 as libc::c_int {
            (*this).health = 1 as libc::c_int as s16
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossDodongo_LayDown(mut this: *mut BossDodongo,
                                             mut globalCtx:
                                                 *mut GlobalContext) {
    (*this).unk_1BE = 10 as libc::c_int as s16;
    Math_SmoothStepToF(&mut (*this).unk_1F8, 1.3f32, 1.0f32, 0.1f32,
                       0.001f32);
    SkelAnime_Update(&mut (*this).skelAnime);
    if Animation_OnFrame(&mut (*this).skelAnime,
                         Animation_GetLastFrame(&mut object_kingdodongo_Anim_004E0C
                                                    as *mut AnimationHeader as
                                                    *mut libc::c_void) as
                             f32_0) != 0 {
        Animation_Change(&mut (*this).skelAnime,
                         &mut object_kingdodongo_Anim_0042A8, 1.0f32, 0.0f32,
                         Animation_GetLastFrame(&mut object_kingdodongo_Anim_0042A8
                                                    as *mut AnimationHeader as
                                                    *mut libc::c_void) as
                             f32_0, ANIMMODE_LOOP as libc::c_int as u8_0,
                         -5.0f32);
        (*this).actionFunc =
            Some(BossDodongo_Vulnerable as
                     unsafe extern "C" fn(_: *mut BossDodongo,
                                          _: *mut GlobalContext) -> ());
        (*this).unk_1DA = 100 as libc::c_int as s16
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossDodongo_Vulnerable(mut this: *mut BossDodongo,
                                                mut globalCtx:
                                                    *mut GlobalContext) {
    Audio_PlayActorSound2(&mut (*this).actor,
                          (0x3851 as libc::c_int - 0x800 as libc::c_int) as
                              u16_0);
    (*this).unk_1BE = 10 as libc::c_int as s16;
    Math_SmoothStepToF(&mut (*this).unk_1F8, 1.0f32, 0.5f32, 0.02f32,
                       0.001f32);
    Math_SmoothStepToF(&mut (*this).unk_208, 0.05f32, 1.0f32, 0.005f32,
                       0.0f32);
    SkelAnime_Update(&mut (*this).skelAnime);
    if (*this).unk_1DA as libc::c_int == 0 as libc::c_int {
        Animation_Change(&mut (*this).skelAnime,
                         &mut object_kingdodongo_Anim_009D10, 1.0f32, 0.0f32,
                         Animation_GetLastFrame(&mut object_kingdodongo_Anim_009D10
                                                    as *mut AnimationHeader as
                                                    *mut libc::c_void) as
                             f32_0, ANIMMODE_ONCE as libc::c_int as u8_0,
                         -5.0f32);
        (*this).actionFunc =
            Some(BossDodongo_GetUp as
                     unsafe extern "C" fn(_: *mut BossDodongo,
                                          _: *mut GlobalContext) -> ())
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossDodongo_GetUp(mut this: *mut BossDodongo,
                                           mut globalCtx:
                                               *mut GlobalContext) {
    SkelAnime_Update(&mut (*this).skelAnime);
    if Animation_OnFrame(&mut (*this).skelAnime,
                         Animation_GetLastFrame(&mut object_kingdodongo_Anim_009D10
                                                    as *mut AnimationHeader as
                                                    *mut libc::c_void) as
                             f32_0) != 0 {
        BossDodongo_SetupRoll(this);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossDodongo_BlowFire(mut this: *mut BossDodongo,
                                              mut globalCtx:
                                                  *mut GlobalContext) {
    let mut pad: s32 = 0;
    let mut unusedZeroVec1: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
    let mut unusedZeroVec2: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
    SkelAnime_Update(&mut (*this).skelAnime);
    if Animation_OnFrame(&mut (*this).skelAnime, 12.0f32) != 0 {
        Audio_PlayActorSound2(&mut (*this).actor,
                              0x3805 as libc::c_int as u16_0);
    }
    if Animation_OnFrame(&mut (*this).skelAnime, 17.0f32) != 0 {
        (*this).unk_1C8 = 28 as libc::c_int as s16
    }
    if (*this).skelAnime.curFrame > 17.0f32 &&
           (*this).skelAnime.curFrame < 35.0f32 {
        BossDodongo_SpawnFire(this, globalCtx, (*this).unk_1AE);
        (*this).unk_1AE += 1;
        Math_SmoothStepToF(&mut (*this).unk_244, 0.0f32, 1.0f32, 8.0f32,
                           0.0f32);
    }
    if (*this).unk_1DA as libc::c_int == 0 as libc::c_int {
        BossDodongo_SetupRoll(this);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossDodongo_Inhale(mut this: *mut BossDodongo,
                                            mut GlobalContext:
                                                *mut GlobalContext) {
    (*this).unk_1E2 = 1 as libc::c_int as u8_0;
    if (*this).unk_1AC as libc::c_int > 20 as libc::c_int {
        Audio_PlayActorSound2(&mut (*this).actor,
                              (0x384f as libc::c_int - 0x800 as libc::c_int)
                                  as u16_0);
    }
    Math_SmoothStepToF(&mut (*this).unk_208, 0.05f32, 1.0f32, 0.005f32,
                       0.0f32);
    SkelAnime_Update(&mut (*this).skelAnime);
    if (*this).unk_1DA as libc::c_int == 0 as libc::c_int {
        BossDodongo_SetupBlowFire(this);
    } else {
        (*this).unk_1AC += 1;
        if (*this).unk_1AC as libc::c_int > 20 as libc::c_int &&
               ((*this).unk_1AC as libc::c_int) < 82 as libc::c_int &&
               BossDodongo_AteExplosive(this, GlobalContext) != 0 {
            Audio_PlayActorSound2(&mut (*this).actor,
                                  0x3850 as libc::c_int as u16_0);
            BossDodongo_SetupExplode(this);
        }
    };
}
static mut sCornerPositions: [Vec3f; 4] =
    [{ let mut init = Vec3f{x: -1390.0f32, y: 0.0f32, z: -3804.0f32,}; init },
     { let mut init = Vec3f{x: -1390.0f32, y: 0.0f32, z: -2804.0f32,}; init },
     { let mut init = Vec3f{x: -390.0f32, y: 0.0f32, z: -2804.0f32,}; init },
     { let mut init = Vec3f{x: -390.0f32, y: 0.0f32, z: -3804.0f32,}; init }];
#[no_mangle]
pub unsafe extern "C" fn BossDodongo_Walk(mut this: *mut BossDodongo,
                                          mut globalCtx: *mut GlobalContext) {
    let mut sp4C: *mut Vec3f = 0 as *mut Vec3f;
    let mut sp48: f32_0 = 0.;
    let mut sp44: f32_0 = 0.;
    if (*this).unk_1AA as libc::c_int == 0 as libc::c_int {
        if Animation_OnFrame(&mut (*this).skelAnime, 14.0f32) != 0 {
            Animation_PlayLoop(&mut (*this).skelAnime,
                               &mut object_kingdodongo_Anim_01CAE0);
            (*this).unk_1AA = 1 as libc::c_int as s16
        }
    } else if (*this).unk_1BC as libc::c_int != 2 as libc::c_int {
        if (*this).skelAnime.curFrame as s32 == 1 as libc::c_int ||
               (*this).skelAnime.curFrame as s32 == 31 as libc::c_int {
            if (*this).skelAnime.curFrame as s32 == 1 as libc::c_int {
                Actor_SpawnFloorDustRing(globalCtx, &mut (*this).actor,
                                         &mut (*this).unk_410, 25.0f32,
                                         0xa as libc::c_int, 8.0f32,
                                         0x1f4 as libc::c_int as s16,
                                         0xa as libc::c_int as s16,
                                         0 as libc::c_int as u8_0);
            } else {
                Actor_SpawnFloorDustRing(globalCtx, &mut (*this).actor,
                                         &mut (*this).unk_404, 25.0f32,
                                         0xa as libc::c_int, 8.0f32,
                                         0x1f4 as libc::c_int as s16,
                                         0xa as libc::c_int as s16,
                                         0 as libc::c_int as u8_0);
            }
            if (*this).unk_1BC as libc::c_int != 0 as libc::c_int {
                func_80078884(0x3808 as libc::c_int as u16_0);
            } else {
                Audio_PlayActorSound2(&mut (*this).actor,
                                      0x3808 as libc::c_int as u16_0);
            }
            if (*this).cutsceneCamera as libc::c_int == 0 as libc::c_int {
                func_80033E88(&mut (*this).actor, globalCtx,
                              4 as libc::c_int as s16,
                              10 as libc::c_int as s16);
            } else {
                (*this).unk_1B6 = 10 as libc::c_int as s16;
                func_800A9F6C(0.0f32, 180 as libc::c_int as u8_0,
                              20 as libc::c_int as u8_0,
                              100 as libc::c_int as u8_0);
            }
        }
    }
    SkelAnime_Update(&mut (*this).skelAnime);
    sp4C =
        &mut *sCornerPositions.as_mut_ptr().offset((*this).unk_1A0 as isize)
            as *mut Vec3f;
    (*this).unk_1EC = 0.7f32;
    Math_SmoothStepToF(&mut (*this).unk_1E4, (*this).unk_1EC * 4.0f32, 1.0f32,
                       (*this).unk_1EC * 0.25f32, 0.0f32);
    Math_SmoothStepToF(&mut (*this).actor.world.pos.x, (*sp4C).x, 0.3f32,
                       (*this).unk_1E4, 0.0f32);
    Math_SmoothStepToF(&mut (*this).actor.world.pos.z, (*sp4C).z, 0.3f32,
                       (*this).unk_1E4, 0.0f32);
    sp48 = (*sp4C).x - (*this).actor.world.pos.x;
    sp44 = (*sp4C).z - (*this).actor.world.pos.z;
    Math_SmoothStepToF(&mut (*this).unk_1E8, 2000.0f32, 1.0f32,
                       (*this).unk_1EC * 80.0f32, 0.0f32);
    Math_SmoothStepToS(&mut (*this).actor.world.rot.y,
                       (Math_FAtan2F(sp48, sp44) *
                            (0x8000 as libc::c_int as libc::c_float /
                                 3.14159265358979323846f32)) as s16,
                       5 as libc::c_int as s16,
                       ((*this).unk_1EC * (*this).unk_1E8) as s16,
                       5 as libc::c_int as s16);
    Math_SmoothStepToS(&mut (*this).unk_1C4, 0 as libc::c_int as s16,
                       2 as libc::c_int as s16, 2000 as libc::c_int as s16,
                       0 as libc::c_int as s16);
    if fabsf(sp48) <= 5.0f32 && fabsf(sp44) <= 5.0f32 {
        (*this).unk_1E8 = 0.0f32;
        (*this).unk_1E4 = 0.0f32;
        if (*this).unk_1A2 as libc::c_int == 0 as libc::c_int {
            (*this).unk_1A0 += 1;
            if (*this).unk_1A0 as libc::c_int >= 4 as libc::c_int {
                (*this).unk_1A0 = 0 as libc::c_int as s16
            }
        } else {
            (*this).unk_1A0 -= 1;
            if ((*this).unk_1A0 as libc::c_int) < 0 as libc::c_int {
                (*this).unk_1A0 = 3 as libc::c_int as s16
            }
        }
    }
    if (*this).unk_1DA as libc::c_int == 0 as libc::c_int &&
           (*this).unk_1BC as libc::c_int == 0 as libc::c_int {
        if (*this).actor.xzDistToPlayer < 500.0f32 &&
               (*this).unk_1A4 as libc::c_int != 0 as libc::c_int &&
               (*this).playerPosInRange == 0 {
            BossDodongo_SetupInhale(this);
            BossDodongo_SpawnFire(this, globalCtx,
                                  -(1 as libc::c_int) as s16);
        }
        if (*this).playerPosInRange == 0 && (*this).playerYawInRange == 0 {
            BossDodongo_SetupRoll(this);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossDodongo_Roll(mut this: *mut BossDodongo,
                                          mut globalCtx: *mut GlobalContext) {
    let mut sp5C: *mut Vec3f = 0 as *mut Vec3f;
    let mut sp50: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp4C: f32_0 = 0.;
    let mut sp48: f32_0 = 0.;
    (*this).actor.flags |=
        ((1 as libc::c_int) << 24 as libc::c_int) as libc::c_uint;
    SkelAnime_Update(&mut (*this).skelAnime);
    if (*this).unk_1DA as libc::c_int == 10 as libc::c_int {
        (*this).actor.velocity.y = 15.0f32;
        Audio_PlayActorSound2(&mut (*this).actor,
                              0x3805 as libc::c_int as u16_0);
    }
    if (*this).unk_1DA as libc::c_int == 1 as libc::c_int {
        Audio_PlayActorSound2(&mut (*this).actor,
                              0x384d as libc::c_int as u16_0);
    }
    sp5C =
        &mut *sCornerPositions.as_mut_ptr().offset((*this).unk_1A0 as isize)
            as *mut Vec3f;
    (*this).unk_1EC = 3.0f32;
    if (*this).unk_1DA as libc::c_int == 0 as libc::c_int {
        Math_SmoothStepToF(&mut (*this).unk_1E4, (*this).unk_1EC * 5.0f32,
                           1.0f32, (*this).unk_1EC * 0.25f32, 0.0f32);
        Math_SmoothStepToF(&mut (*this).actor.world.pos.x, (*sp5C).x, 1.0f32,
                           (*this).unk_1E4, 0.0f32);
        Math_SmoothStepToF(&mut (*this).actor.world.pos.z, (*sp5C).z, 1.0f32,
                           (*this).unk_1E4, 0.0f32);
        (*this).unk_1C4 =
            ((*this).unk_1C4 as libc::c_int + 2000 as libc::c_int) as s16;
        if (*this).actor.bgCheckFlags as libc::c_int & 1 as libc::c_int != 0 {
            (*this).unk_228 = 7700.0f32;
            Audio_PlayActorSound2(&mut (*this).actor,
                                  (0x384e as libc::c_int -
                                       0x800 as libc::c_int) as u16_0);
            if (*this).unk_19E as libc::c_int & 7 as libc::c_int ==
                   0 as libc::c_int {
                Camera_AddQuake(&mut (*globalCtx).mainCamera,
                                2 as libc::c_int, 1 as libc::c_int as s16,
                                8 as libc::c_int);
            }
            if (*this).unk_19E as libc::c_int & 1 as libc::c_int == 0 {
                Actor_SpawnFloorDustRing(globalCtx, &mut (*this).actor,
                                         &mut (*this).actor.world.pos,
                                         40.0f32, 3 as libc::c_int, 8.0f32,
                                         0x1f4 as libc::c_int as s16,
                                         0xa as libc::c_int as s16,
                                         0 as libc::c_int as u8_0);
            }
        }
    }
    sp4C = (*sp5C).x - (*this).actor.world.pos.x;
    sp48 = (*sp5C).z - (*this).actor.world.pos.z;
    Math_SmoothStepToF(&mut (*this).unk_1E8, 2000.0f32, 1.0f32,
                       (*this).unk_1EC * 100.0f32, 0.0f32);
    Math_SmoothStepToS(&mut (*this).actor.world.rot.y,
                       (Math_FAtan2F(sp4C, sp48) *
                            (0x8000 as libc::c_int as libc::c_float /
                                 3.14159265358979323846f32)) as s16,
                       5 as libc::c_int as s16,
                       ((*this).unk_1EC * (*this).unk_1E8) as s16,
                       0 as libc::c_int as s16);
    if fabsf(sp4C) <= 15.0f32 && fabsf(sp48) <= 15.0f32 {
        (*this).numWallCollisions += 1;
        if (*this).numWallCollisions as libc::c_int >= 2 as libc::c_int {
            if (*this).unk_1A6 as libc::c_int != 0 as libc::c_int {
                (*this).unk_1A2 =
                    (1 as libc::c_int - (*this).unk_1A2 as libc::c_int) as s16
            }
            (*this).unk_1E8 = 0.0f32;
            (*this).unk_1E4 = 0.0f32;
            BossDodongo_SetupWalk(this);
            (*this).unk_228 = 9200.0f32;
            (*this).actor.velocity.y = 20.0f32;
            Audio_PlayActorSound2(&mut (*this).actor,
                                  0x384c as libc::c_int as u16_0);
            Camera_AddQuake(&mut (*globalCtx).mainCamera, 2 as libc::c_int,
                            6 as libc::c_int as s16, 8 as libc::c_int);
            sp50.x = (*this).actor.world.pos.x;
            sp50.y = (*this).actor.world.pos.y + 60.0f32;
            sp50.z = (*this).actor.world.pos.z;
            func_80033480(globalCtx, &mut sp50, 250.0f32, 40 as libc::c_int,
                          800 as libc::c_int as s16, 10 as libc::c_int as s16,
                          0 as libc::c_int as u8_0);
            func_80033E88(&mut (*this).actor, globalCtx,
                          6 as libc::c_int as s16, 15 as libc::c_int as s16);
        } else {
            (*this).actor.velocity.y = 15.0f32;
            Audio_PlayActorSound2(&mut (*this).actor,
                                  0x384d as libc::c_int as u16_0);
        }
        if (*this).unk_1A2 as libc::c_int == 0 as libc::c_int {
            (*this).unk_1A0 += 1;
            if (*this).unk_1A0 as libc::c_int >= 4 as libc::c_int {
                (*this).unk_1A0 = 0 as libc::c_int as s16
            }
        } else {
            (*this).unk_1A0 -= 1;
            if ((*this).unk_1A0 as libc::c_int) < 0 as libc::c_int {
                (*this).unk_1A0 = 3 as libc::c_int as s16
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossDodongo_Update(mut thisx: *mut Actor,
                                            mut globalCtx2:
                                                *mut GlobalContext) {
    let mut globalCtx: *mut GlobalContext = globalCtx2;
    let mut this: *mut BossDodongo = thisx as *mut BossDodongo;
    let mut temp_f0: f32_0 = 0.;
    let mut i: s16 = 0;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut player2: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut pad: s32 = 0;
    (*this).unk_1E2 = 0 as libc::c_int as u8_0;
    (*this).unk_19E += 1;
    if (*this).unk_1DA as libc::c_int != 0 as libc::c_int {
        (*this).unk_1DA -= 1
    }
    if (*this).unk_1DC as libc::c_int != 0 as libc::c_int {
        (*this).unk_1DC -= 1
    }
    if (*this).unk_1DE as libc::c_int != 0 as libc::c_int {
        (*this).unk_1DE -= 1
    }
    if (*this).unk_1C0 as libc::c_int != 0 as libc::c_int {
        (*this).unk_1C0 -= 1
    }
    if (*this).unk_1C8 as libc::c_int != 0 as libc::c_int {
        (*this).unk_1C8 -= 1
    }
    temp_f0 = func_808C4F6C(this, globalCtx);
    if temp_f0 > 0.0f32 {
        (*this).unk_1A4 = temp_f0 as s16
    } else { (*this).unk_1A4 = 0 as libc::c_int as s16 }
    temp_f0 = func_808C50A8(this, globalCtx);
    if temp_f0 > 0.0f32 {
        (*this).unk_1A6 = temp_f0 as s16
    } else { (*this).unk_1A6 = 0 as libc::c_int as s16 }
    BossDodongo_PlayerYawCheck(this, globalCtx);
    BossDodongo_PlayerPosCheck(this, globalCtx);
    (*this).actionFunc.expect("non-null function pointer")(this, globalCtx);
    (*thisx).shape.rot.y = (*thisx).world.rot.y;
    Math_SmoothStepToF(&mut (*thisx).shape.yOffset, (*this).unk_228, 1.0f32,
                       100.0f32, 0.0f32);
    Actor_MoveForward(thisx);
    BossDodongo_UpdateDamage(this, globalCtx);
    Actor_UpdateBgCheckInfo(globalCtx, thisx, 10.0f32, 10.0f32, 20.0f32,
                            4 as libc::c_int);
    Math_SmoothStepToF(&mut (*this).unk_208, 0 as libc::c_int as f32_0,
                       1 as libc::c_int as f32_0, 0.001f32, 0.0f64 as f32_0);
    Math_SmoothStepToF(&mut (*this).unk_20C, 0 as libc::c_int as f32_0,
                       1 as libc::c_int as f32_0, 0.001f32, 0.0f64 as f32_0);
    if (*this).unk_19E as libc::c_int % 128 as libc::c_int == 0 as libc::c_int
       {
        i = 0 as libc::c_int as s16;
        while (i as libc::c_int) < 50 as libc::c_int {
            (*this).unk_324[i as usize] = Rand_ZeroOne() * 0.25f32 + 0.5f32;
            i += 1
        }
    }
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < 50 as libc::c_int {
        (*this).unk_25C[i as usize] += (*this).unk_324[i as usize];
        i += 1
    }
    if (*this).unk_1C8 as libc::c_int != 0 as libc::c_int {
        if (*this).unk_1C8 as libc::c_int >= 11 as libc::c_int {
            Math_SmoothStepToF(&mut (*this).unk_240,
                               if (*this).unk_1C8 as libc::c_int &
                                      1 as libc::c_int != 0 {
                                   40.0f32
                               } else { 60.0f32 }, 1.0f32, 50.0f32, 0.0f32);
        } else {
            Math_SmoothStepToF(&mut (*this).unk_240, 0.0f32,
                               1 as libc::c_int as f32_0, 10.0f32,
                               0.0f64 as f32_0);
        }
        if (*globalCtx).envCtx.adjLight1Color[2 as libc::c_int as usize] as
               libc::c_int == 0 as libc::c_int &&
               (*globalCtx).envCtx.adjAmbientColor[2 as libc::c_int as usize]
                   as libc::c_int == 0 as libc::c_int {
            (*globalCtx).envCtx.adjLight1Color[0 as libc::c_int as usize] =
                (*this).unk_240 as u8_0 as s16;
            (*globalCtx).envCtx.adjLight1Color[1 as libc::c_int as usize] =
                ((*this).unk_240 * 0.1f32) as u8_0 as s16;
            (*globalCtx).envCtx.adjAmbientColor[0 as libc::c_int as usize] =
                (*this).unk_240 as u8_0 as s16;
            (*globalCtx).envCtx.adjAmbientColor[1 as libc::c_int as usize] =
                ((*this).unk_240 * 0.1f32) as u8_0 as s16
        }
    }
    if (*this).unk_1BE as libc::c_int != 0 as libc::c_int {
        if (*this).unk_1BE as libc::c_int >= 1000 as libc::c_int {
            Math_SmoothStepToF(&mut (*this).colorFilterR, 30.0f32,
                               1 as libc::c_int as f32_0, 20.0f32,
                               0.0f64 as f32_0);
            Math_SmoothStepToF(&mut (*this).colorFilterG, 10.0f32,
                               1 as libc::c_int as f32_0, 20.0f32,
                               0.0f64 as f32_0);
        } else {
            (*this).unk_1BE -= 1;
            Math_SmoothStepToF(&mut (*this).colorFilterR, 255.0f32,
                               1 as libc::c_int as f32_0, 20.0f32,
                               0.0f64 as f32_0);
            Math_SmoothStepToF(&mut (*this).colorFilterG, 0.0f32,
                               1 as libc::c_int as f32_0, 20.0f32,
                               0.0f64 as f32_0);
        }
        Math_SmoothStepToF(&mut (*this).colorFilterB, 0.0f32,
                           1 as libc::c_int as f32_0, 20.0f32,
                           0.0f64 as f32_0);
        Math_SmoothStepToF(&mut (*this).colorFilterMin, 900.0f32,
                           1 as libc::c_int as f32_0, 10.0f32,
                           0.0f64 as f32_0);
        Math_SmoothStepToF(&mut (*this).colorFilterMax, 1099.0f32,
                           1 as libc::c_int as f32_0, 10.0f32,
                           0.0f64 as f32_0);
    } else {
        Math_SmoothStepToF(&mut (*this).colorFilterR,
                           (*globalCtx).lightCtx.fogColor[0 as libc::c_int as
                                                              usize] as f32_0,
                           1 as libc::c_int as f32_0, 5.0f32,
                           0.0f64 as f32_0);
        Math_SmoothStepToF(&mut (*this).colorFilterG,
                           (*globalCtx).lightCtx.fogColor[1 as libc::c_int as
                                                              usize] as f32_0,
                           1.0f32, 5.0f32, 0.0f64 as f32_0);
        Math_SmoothStepToF(&mut (*this).colorFilterB,
                           (*globalCtx).lightCtx.fogColor[2 as libc::c_int as
                                                              usize] as f32_0,
                           1.0f32, 5.0f32, 0.0f64 as f32_0);
        Math_SmoothStepToF(&mut (*this).colorFilterMin,
                           (*globalCtx).lightCtx.fogNear as f32_0,
                           1.0f64 as f32_0, 5.0f32, 0.0f64 as f32_0);
        Math_SmoothStepToF(&mut (*this).colorFilterMax, 1000.0f32,
                           1 as libc::c_int as f32_0, 5.0f32,
                           0.0f64 as f32_0);
    }
    if (*player).actor.world.pos.y < -1000.0f32 {
        let mut phi_s0_3: s16 = 0;
        let mut sp90: s16 = 0;
        let mut magma2DrawMode: s16 = 0;
        let mut magmaScale: s16 = 0 as libc::c_int as s16;
        if (*this).unk_224 > 1.9f32 {
            phi_s0_3 = 1 as libc::c_int as s16;
            magma2DrawMode = 0 as libc::c_int as s16;
            sp90 = 0 as libc::c_int as s16
        } else if (*this).unk_224 > 1.7f32 {
            phi_s0_3 = 3 as libc::c_int as s16;
            sp90 = 1 as libc::c_int as s16;
            !globalCtx.is_null();
            magma2DrawMode = 0 as libc::c_int as s16
        } else if (*this).unk_224 > 1.4f32 {
            phi_s0_3 = 7 as libc::c_int as s16;
            sp90 = 3 as libc::c_int as s16;
            magma2DrawMode = (Rand_ZeroOne() * 1.9f32) as s16
        } else if (*this).unk_224 > 1.1f32 {
            phi_s0_3 = 7 as libc::c_int as s16;
            sp90 = 4095 as libc::c_int as s16;
            magma2DrawMode = (Rand_ZeroOne() * 1.9f32) as s16
        } else {
            phi_s0_3 = 1 as libc::c_int as s16;
            sp90 = -(1 as libc::c_int) as s16;
            magma2DrawMode = 1 as libc::c_int as s16;
            magmaScale =
                ((Rand_ZeroOne() * 50 as libc::c_int as libc::c_float) as s16
                     as libc::c_int - 50 as libc::c_int) as s16
        }
        if (*player2).csMode as libc::c_int >= 10 as libc::c_int {
            phi_s0_3 = -(1 as libc::c_int) as s16
        }
        if (*this).unk_19E as libc::c_int & phi_s0_3 as libc::c_int ==
               0 as libc::c_int {
            static mut magmaPrimColor: [Color_RGBA8; 2] =
                [{
                     let mut init =
                         Color_RGBA8{r: 255 as libc::c_int as u8_0,
                                     g: 255 as libc::c_int as u8_0,
                                     b: 0 as libc::c_int as u8_0,
                                     a: 255 as libc::c_int as u8_0,};
                     init
                 },
                 {
                     let mut init =
                         Color_RGBA8{r: 0 as libc::c_int as u8_0,
                                     g: 0 as libc::c_int as u8_0,
                                     b: 0 as libc::c_int as u8_0,
                                     a: 150 as libc::c_int as u8_0,};
                     init
                 }];
            static mut magmaEnvColor: [Color_RGBA8; 2] =
                [{
                     let mut init =
                         Color_RGBA8{r: 255 as libc::c_int as u8_0,
                                     g: 0 as libc::c_int as u8_0,
                                     b: 0 as libc::c_int as u8_0,
                                     a: 255 as libc::c_int as u8_0,};
                     init
                 },
                 {
                     let mut init =
                         Color_RGBA8{r: 0 as libc::c_int as u8_0,
                                     g: 0 as libc::c_int as u8_0,
                                     b: 0 as libc::c_int as u8_0,
                                     a: 0 as libc::c_int as u8_0,};
                     init
                 }];
            let mut sp84: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
            let mut temp_f12: f32_0 = 0.;
            let mut temp_f10: f32_0 = 0.;
            temp_f12 = Rand_ZeroOne() * 330.0f32;
            temp_f10 = Rand_ZeroOne() * 6.28f32;
            sp84.x = sinf(temp_f10) * temp_f12 + -890.0f32;
            sp84.y = -1523.76f32;
            sp84.z = cosf(temp_f10) * temp_f12 + -3304.0f32;
            EffectSsGMagma2_Spawn(globalCtx, &mut sp84,
                                  &mut *magmaPrimColor.as_mut_ptr().offset(magma2DrawMode
                                                                               as
                                                                               isize),
                                  &mut *magmaEnvColor.as_mut_ptr().offset(magma2DrawMode
                                                                              as
                                                                              isize),
                                  (10 as libc::c_int -
                                       magma2DrawMode as libc::c_int *
                                           5 as libc::c_int) as s16,
                                  magma2DrawMode,
                                  (magmaScale as libc::c_int +
                                       100 as libc::c_int) as s16);
        }
        if (*this).unk_19E as libc::c_int & sp90 as libc::c_int ==
               0 as libc::c_int {
            let mut sp6C: Vec3f =
                {
                    let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,};
                    init
                };
            let mut sp60: Vec3f =
                {
                    let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,};
                    init
                };
            let mut sp54: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
            let mut sp50: f32_0 = Rand_ZeroOne() * 330.0f32;
            let mut sp4C: f32_0 = Rand_ZeroOne() * 6.28f32;
            sp54.x = sinf(sp4C) * sp50 + -890.0f32;
            sp54.y = -1523.76f32;
            sp54.z = cosf(sp4C) * sp50 + -3304.0f32;
            EffectSsGMagma_Spawn(globalCtx, &mut sp54);
            i = 0 as libc::c_int as s16;
            while (i as libc::c_int) < 4 as libc::c_int {
                sp60.y = 0.4f32;
                sp60.x = Rand_CenteredFloat(0.5f32);
                sp60.z = Rand_CenteredFloat(0.5f32);
                sp50 = Rand_ZeroOne() * 330.0f32;
                sp4C = Rand_ZeroOne() * 6.28f32;
                sp54.x = sinf(sp4C) * sp50 + -890.0f32;
                sp54.y = -1513.76f32;
                sp54.z = cosf(sp4C) * sp50 + -3304.0f32;
                func_808C17C8(globalCtx, &mut sp54, &mut sp6C, &mut sp60,
                              (Rand_ZeroFloat(2.0f32) as s16 as libc::c_int +
                                   6 as libc::c_int) as f32_0,
                              0x50 as libc::c_int as s16);
                i += 1
            }
        }
        func_808C1554(gDodongosCavernBossLavaFloorTex.as_mut_ptr() as
                          *mut libc::c_void,
                      sLavaFloorLavaTex.as_mut_ptr() as *mut libc::c_void,
                      (*this).unk_19E as s32, (*this).unk_224);
    }
    if (*this).unk_1C6 as libc::c_int != 0 as libc::c_int {
        let mut ptr1: *mut u16_0 =
            gSegments[((sLavaFloorLavaTex.as_mut_ptr() as u32_0) <<
                           4 as libc::c_int >> 28 as libc::c_int) as
                          usize].wrapping_add(sLavaFloorLavaTex.as_mut_ptr()
                                                  as u32_0 &
                                                  0xffffff as libc::c_int as
                                                      libc::c_uint).wrapping_add(0x80000000
                                                                                     as
                                                                                     libc::c_uint)
                as *mut libc::c_void as *mut u16_0;
        let mut ptr2: *mut u16_0 =
            gSegments[((sLavaFloorRockTex.as_mut_ptr() as u32_0) <<
                           4 as libc::c_int >> 28 as libc::c_int) as
                          usize].wrapping_add(sLavaFloorRockTex.as_mut_ptr()
                                                  as u32_0 &
                                                  0xffffff as libc::c_int as
                                                      libc::c_uint).wrapping_add(0x80000000
                                                                                     as
                                                                                     libc::c_uint)
                as *mut libc::c_void as *mut u16_0;
        let mut i2: s16 = 0;
        i2 = 0 as libc::c_int as s16;
        while (i2 as libc::c_int) < 20 as libc::c_int {
            let mut new_var: s16 =
                ((*this).unk_1C2 as libc::c_int & 0x7ff as libc::c_int) as
                    s16;
            *ptr1.offset(new_var as isize) = *ptr2.offset(new_var as isize);
            (*this).unk_1C2 =
                ((*this).unk_1C2 as libc::c_int + 37 as libc::c_int) as s16;
            i2 += 1
        }
        Math_SmoothStepToF(&mut (*this).unk_224, 0.0f32, 1.0f32, 0.01f32,
                           0.0f32);
    }
    if (*this).unk_1BC as libc::c_int == 0 as libc::c_int {
        if (*this).actionFunc !=
               Some(BossDodongo_DeathCutscene as
                        unsafe extern "C" fn(_: *mut BossDodongo,
                                             _: *mut GlobalContext) -> ()) {
            CollisionCheck_SetAC(globalCtx, &mut (*globalCtx).colChkCtx,
                                 &mut (*this).collider.base);
        }
        CollisionCheck_SetOC(globalCtx, &mut (*globalCtx).colChkCtx,
                             &mut (*this).collider.base);
        if (*this).actionFunc ==
               Some(BossDodongo_Roll as
                        unsafe extern "C" fn(_: *mut BossDodongo,
                                             _: *mut GlobalContext) -> ()) {
            CollisionCheck_SetAT(globalCtx, &mut (*globalCtx).colChkCtx,
                                 &mut (*this).collider.base);
        }
    }
    (*(*this).collider.elements.offset(0 as libc::c_int as isize)).dim.scale =
        if (*this).actionFunc ==
               Some(BossDodongo_Inhale as
                        unsafe extern "C" fn(_: *mut BossDodongo,
                                             _: *mut GlobalContext) -> ()) {
            0.0f32
        } else { 1.0f32 };
    i = 6 as libc::c_int as s16;
    while (i as libc::c_int) < 19 as libc::c_int {
        if i as libc::c_int != 12 as libc::c_int {
            (*(*this).collider.elements.offset(i as isize)).dim.scale =
                if (*this).actionFunc ==
                       Some(BossDodongo_Roll as
                                unsafe extern "C" fn(_: *mut BossDodongo,
                                                     _: *mut GlobalContext)
                                    -> ()) {
                    0.0f32
                } else { 1.0f32 }
        }
        i += 1
    }
    if (*this).unk_244 != 0 as libc::c_int as libc::c_float {
        (*gGameInfo).data[(5 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 64 as libc::c_int) as
                              usize] = 1 as libc::c_int as s16;
        (*gGameInfo).data[(5 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 65 as libc::c_int) as
                              usize] = 255 as libc::c_int as s16;
        (*gGameInfo).data[(5 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 66 as libc::c_int) as
                              usize] = 80 as libc::c_int as s16;
        (*gGameInfo).data[(5 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 67 as libc::c_int) as
                              usize] = 0 as libc::c_int as s16;
        (*gGameInfo).data[(5 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 68 as libc::c_int) as
                              usize] = (*this).unk_244 as u8_0 as s16
    } else {
        (*gGameInfo).data[(5 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 64 as libc::c_int) as
                              usize] = 0 as libc::c_int as s16
    }
    Math_SmoothStepToF(&mut (*this).unk_244, 0.0f32, 1.0f32, 2.0f32, 0.0f32);
    BossDodongo_UpdateEffects(globalCtx);
}
#[no_mangle]
pub unsafe extern "C" fn BossDodongo_OverrideLimbDraw(mut globalCtx:
                                                          *mut GlobalContext,
                                                      mut limbIndex: s32,
                                                      mut dList:
                                                          *mut *mut Gfx,
                                                      mut pos: *mut Vec3f,
                                                      mut rot: *mut Vec3s,
                                                      mut thisx:
                                                          *mut libc::c_void)
 -> s32 {
    let mut mtxScaleY: f32_0 = 0.;
    let mut mtxScaleZ: f32_0 = 0.;
    let mut this: *mut BossDodongo = thisx as *mut BossDodongo;
    // required for matching
    if limbIndex == 6 as libc::c_int || limbIndex == 7 as libc::c_int {
        !(*this).unk_25C.as_mut_ptr().is_null(); // Required to match
    }
    Matrix_TranslateRotateZYX(pos, rot);
    if !(*dList).is_null() {
        let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
        let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
        __gfxCtx = (*globalCtx).state.gfxCtx;
        Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                        b"../z_boss_dodongo.c\x00" as *const u8 as
                            *const libc::c_char, 3787 as libc::c_int);
        mtxScaleZ = 1.0f32;
        mtxScaleY = 1.0f32;
        if limbIndex == 33 as libc::c_int || limbIndex == 48 as libc::c_int {
            mtxScaleZ = (*this).unk_1F8;
            mtxScaleY = mtxScaleZ
        }
        Matrix_Push();
        Matrix_Scale(1.0f32, mtxScaleY, mtxScaleZ,
                     MTXMODE_APPLY as libc::c_int as u8_0);
        if limbIndex != 6 as libc::c_int && limbIndex != 7 as libc::c_int {
            Matrix_RotateX((*this).unk_25C[limbIndex as usize] * 0.115f32,
                           MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_RotateY((*this).unk_25C[limbIndex as usize] * 0.13f32,
                           MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_RotateZ((*this).unk_25C[limbIndex as usize] * 0.1f32,
                           MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_Scale(1.0f32 - (*this).unk_208, (*this).unk_208 + 1.0f32,
                         1.0f32 - (*this).unk_208,
                         MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_RotateZ(-((*this).unk_25C[limbIndex as usize] * 0.1f32),
                           MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_RotateY(-((*this).unk_25C[limbIndex as usize] * 0.13f32),
                           MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_RotateX(-((*this).unk_25C[limbIndex as usize] * 0.115f32),
                           MTXMODE_APPLY as libc::c_int as u8_0);
        }
        let fresh0 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g: *mut Gfx = fresh0;
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
                          b"../z_boss_dodongo.c\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                          3822 as libc::c_int) as libc::c_uint;
        let fresh1 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_0: *mut Gfx = fresh1;
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
        (*_g_0).words.w1 = *dList as libc::c_uint;
        Matrix_Pop();
        Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                         b"../z_boss_dodongo.c\x00" as *const u8 as
                             *const libc::c_char, 3826 as libc::c_int);
    }
    let mut pad: s32 = 0;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BossDodongo_PostLimbDraw(mut globalCtx:
                                                      *mut GlobalContext,
                                                  mut limbIndex: s32,
                                                  mut dList: *mut *mut Gfx,
                                                  mut rot: *mut Vec3s,
                                                  mut thisx:
                                                      *mut libc::c_void) {
    static mut D_808CA450: Vec3f =
        {
            let mut init = Vec3f{x: 5000.0f32, y: -2500.0f32, z: 0.0f32,};
            init
        };
    static mut D_808CA45C: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
    static mut D_808CA468: Vec3f =
        {
            let mut init = Vec3f{x: 11500.0f32, y: -3000.0f32, z: 0.0f32,};
            init
        };
    static mut D_808CA474: Vec3f =
        {
            let mut init = Vec3f{x: 5000.0f32, y: -2000.0f32, z: 0.0f32,};
            init
        };
    static mut D_808CA480: Vec3f =
        { let mut init = Vec3f{x: 8000.0f32, y: 0.0f32, z: 0.0f32,}; init };
    static mut D_808CA48C: Vec3f =
        { let mut init = Vec3f{x: 8000.0f32, y: 0.0f32, z: 0.0f32,}; init };
    let mut this: *mut BossDodongo = thisx as *mut BossDodongo;
    if limbIndex == 6 as libc::c_int {
        Matrix_MultVec3f(&mut D_808CA45C, &mut (*this).vec);
        Matrix_MultVec3f(&mut D_808CA450, &mut (*this).actor.focus.pos);
        Matrix_MultVec3f(&mut D_808CA468, &mut (*this).firePos);
        Matrix_MultVec3f(&mut D_808CA474, &mut (*this).mouthPos);
    } else if limbIndex == 39 as libc::c_int {
        Matrix_MultVec3f(&mut D_808CA480, &mut (*this).unk_410);
    } else if limbIndex == 46 as libc::c_int {
        Matrix_MultVec3f(&mut D_808CA48C, &mut (*this).unk_404);
    }
    Collider_UpdateSpheres(limbIndex, &mut (*this).collider);
}
#[no_mangle]
pub unsafe extern "C" fn BossDodongo_Draw(mut thisx: *mut Actor,
                                          mut globalCtx: *mut GlobalContext) {
    let mut this: *mut BossDodongo = thisx as *mut BossDodongo;
    let mut pad: s32 = 0;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_boss_dodongo.c\x00" as *const u8 as
                        *const libc::c_char, 3922 as libc::c_int);
    func_80093D18((*globalCtx).state.gfxCtx);
    if (*this).unk_1C0 as libc::c_int >= 2 as libc::c_int &&
           (*this).unk_1C0 as libc::c_int & 1 as libc::c_int != 0 {
        (*__gfxCtx).polyOpa.p =
            Gfx_SetFog((*__gfxCtx).polyOpa.p, 255 as libc::c_int,
                       255 as libc::c_int, 255 as libc::c_int,
                       0 as libc::c_int, 900 as libc::c_int,
                       1099 as libc::c_int)
    } else {
        (*__gfxCtx).polyOpa.p =
            Gfx_SetFog((*__gfxCtx).polyOpa.p,
                       (*this).colorFilterR as u32_0 as s32,
                       (*this).colorFilterG as u32_0 as s32,
                       (*this).colorFilterB as u32_0 as s32, 0 as libc::c_int,
                       (*this).colorFilterMin as s32,
                       (*this).colorFilterMax as s32)
    }
    Matrix_RotateZ((*this).unk_23C, MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_RotateX((*this).unk_1C4 as libc::c_int as libc::c_float /
                       32768.0f32 * 3.14159f32,
                   MTXMODE_APPLY as libc::c_int as u8_0);
    SkelAnime_DrawOpa(globalCtx, (*this).skelAnime.skeleton,
                      (*this).skelAnime.jointTable,
                      Some(BossDodongo_OverrideLimbDraw as
                               unsafe extern "C" fn(_: *mut GlobalContext,
                                                    _: s32, _: *mut *mut Gfx,
                                                    _: *mut Vec3f,
                                                    _: *mut Vec3s,
                                                    _: *mut libc::c_void)
                                   -> s32),
                      Some(BossDodongo_PostLimbDraw as
                               unsafe extern "C" fn(_: *mut GlobalContext,
                                                    _: s32, _: *mut *mut Gfx,
                                                    _: *mut Vec3s,
                                                    _: *mut libc::c_void)
                                   -> ()), this as *mut libc::c_void);
    (*__gfxCtx).polyOpa.p = Gameplay_SetFog(globalCtx, (*__gfxCtx).polyOpa.p);
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_boss_dodongo.c\x00" as *const u8 as
                         *const libc::c_char, 3981 as libc::c_int);
    BossDodongo_DrawEffects(globalCtx);
}
#[no_mangle]
pub unsafe extern "C" fn func_808C4F6C(mut this: *mut BossDodongo,
                                       mut globalCtx: *mut GlobalContext)
 -> f32_0 {
    let mut xDiff: f32_0 = 0.;
    let mut zDiff: f32_0 = 0.;
    let mut sp2C: f32_0 = 0.;
    let mut pad: s32 = 0;
    let mut temp_f2: f32_0 = 0.;
    let mut rotation: f32_0 = 0.;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    xDiff = (*player).actor.world.pos.x - (*this).actor.world.pos.x;
    zDiff = (*player).actor.world.pos.z - (*this).actor.world.pos.z;
    rotation = Math_CosS(-((*this).actor.world.rot.y as libc::c_int) as s16);
    sp2C =
        Math_SinS(-((*this).actor.world.rot.y as libc::c_int) as s16) * zDiff
            + rotation * xDiff;
    rotation = Math_SinS(-((*this).actor.world.rot.y as libc::c_int) as s16);
    temp_f2 =
        Math_CosS(-((*this).actor.world.rot.y as libc::c_int) as s16) * zDiff
            + -rotation * xDiff;
    if fabsf(sp2C) < 150.0f32 && temp_f2 >= 100.0f32 && temp_f2 <= 2000.0f32 {
        return temp_f2
    }
    return -1.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn func_808C50A8(mut this: *mut BossDodongo,
                                       mut globalCtx: *mut GlobalContext)
 -> f32_0 {
    let mut xDiff: f32_0 = 0.;
    let mut zDiff: f32_0 = 0.;
    let mut sp2C: f32_0 = 0.;
    let mut pad: s32 = 0;
    let mut temp_f2: f32_0 = 0.;
    let mut rotation: f32_0 = 0.;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    xDiff = (*player).actor.world.pos.x - (*this).actor.world.pos.x;
    zDiff = (*player).actor.world.pos.z - (*this).actor.world.pos.z;
    rotation =
        Math_CosS((-(0x8000 as libc::c_int) -
                       (*this).actor.world.rot.y as libc::c_int) as s16);
    sp2C =
        Math_SinS((-(0x8000 as libc::c_int) -
                       (*this).actor.world.rot.y as libc::c_int) as s16) *
            zDiff + rotation * xDiff;
    rotation =
        Math_SinS((-(0x8000 as libc::c_int) -
                       (*this).actor.world.rot.y as libc::c_int) as s16);
    temp_f2 =
        Math_CosS((-(0x8000 as libc::c_int) -
                       (*this).actor.world.rot.y as libc::c_int) as s16) *
            zDiff + -rotation * xDiff;
    if fabsf(sp2C) < 150.0f32 && 100.0f32 <= temp_f2 && temp_f2 <= 2000.0f32 {
        return temp_f2
    }
    return -1.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn BossDodongo_PlayerYawCheck(mut this:
                                                        *mut BossDodongo,
                                                    mut globalCtx:
                                                        *mut GlobalContext) {
    let mut yawDiff: s16 =
        (Actor_WorldYawTowardActor(&mut (*this).actor,
                                   &mut (*((*(*globalCtx).actorCtx.actorLists.as_mut_ptr().offset(ACTORCAT_PLAYER
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      isize)).head
                                               as *mut Player)).actor) as
             libc::c_int - (*this).actor.world.rot.y as libc::c_int) as s16;
    if (yawDiff as libc::c_int) < 0x38e3 as libc::c_int &&
           (-(0x38e3 as libc::c_int)) < yawDiff as libc::c_int {
        (*this).playerYawInRange = 1 as libc::c_int as s16
    } else { (*this).playerYawInRange = 0 as libc::c_int as s16 };
}
#[no_mangle]
pub unsafe extern "C" fn BossDodongo_PlayerPosCheck(mut this:
                                                        *mut BossDodongo,
                                                    mut globalCtx:
                                                        *mut GlobalContext) {
    let mut temp_v1: *mut Vec3f = 0 as *mut Vec3f;
    let mut i: s16 = 0;
    (*this).playerPosInRange = 0 as libc::c_int as s16;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < 4 as libc::c_int {
        temp_v1 =
            &mut *sCornerPositions.as_mut_ptr().offset(i as isize) as
                *mut Vec3f;
        if fabsf((*this).actor.world.pos.x - (*temp_v1).x) < 200.0f32 &&
               fabsf((*this).actor.world.pos.z - (*temp_v1).z) < 200.0f32 {
            (*this).playerPosInRange = 1 as libc::c_int as s16;
            break ;
        } else { i += 1 }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossDodongo_SpawnFire(mut this: *mut BossDodongo,
                                               mut globalCtx:
                                                   *mut GlobalContext,
                                               mut params: s16) {
    Actor_SpawnAsChild(&mut (*globalCtx).actorCtx, &mut (*this).actor,
                       globalCtx, ACTOR_EN_BDFIRE as libc::c_int as s16,
                       (*this).vec.x, (*this).vec.y - 20.0f32, (*this).vec.z,
                       0 as libc::c_int as s16, (*this).actor.shape.rot.y,
                       0 as libc::c_int as s16, params);
}
#[no_mangle]
pub unsafe extern "C" fn BossDodongo_UpdateDamage(mut this: *mut BossDodongo,
                                                  mut globalCtx:
                                                      *mut GlobalContext) {
    let mut pad: s32 = 0;
    let mut item1: *mut ColliderInfo = 0 as *mut ColliderInfo;
    let mut swordDamage: u8_0 = 0;
    let mut damage: s32 = 0;
    let mut item2: *mut ColliderInfo = 0 as *mut ColliderInfo;
    let mut i: s16 = 0;
    if (*this).health as libc::c_int <= 0 as libc::c_int &&
           (*this).actionFunc !=
               Some(BossDodongo_DeathCutscene as
                        unsafe extern "C" fn(_: *mut BossDodongo,
                                             _: *mut GlobalContext) -> ()) {
        BossDodongo_SetupDeathCutscene(this);
        Enemy_StartFinishingBlow(globalCtx, &mut (*this).actor);
        return
    }
    if (*this).unk_1C0 as libc::c_int == 0 as libc::c_int {
        if (*this).actionFunc ==
               Some(BossDodongo_Inhale as
                        unsafe extern "C" fn(_: *mut BossDodongo,
                                             _: *mut GlobalContext) -> ()) {
            i = 0 as libc::c_int as s16;
            while (i as libc::c_int) < 19 as libc::c_int {
                if (*(*this).collider.elements.offset(i as
                                                          isize)).info.bumperFlags
                       as libc::c_int & 2 as libc::c_int != 0 {
                    item1 =
                        (*(*this).collider.elements.offset(i as
                                                               isize)).info.acHitInfo;
                    item2 = item1;
                    if (*item2).toucher.dmgFlags &
                           0x10 as libc::c_int as libc::c_uint != 0 ||
                           (*item2).toucher.dmgFlags &
                               4 as libc::c_int as libc::c_uint != 0 {
                        let ref mut fresh2 =
                            (*(*this).collider.elements.offset(i as
                                                                   isize)).info.bumperFlags;
                        *fresh2 =
                            (*fresh2 as libc::c_int & !(2 as libc::c_int)) as
                                u8_0;
                        (*this).unk_1C0 = 2 as libc::c_int as s16;
                        BossDodongo_SetupWalk(this);
                        (*this).unk_1DA = 0x32 as libc::c_int as s16;
                        return
                    }
                }
                i += 1
            }
        }
        if (*(*this).collider.elements).info.bumperFlags as libc::c_int &
               2 as libc::c_int != 0 {
            (*(*this).collider.elements).info.bumperFlags =
                ((*(*this).collider.elements).info.bumperFlags as libc::c_int
                     & !(2 as libc::c_int)) as u8_0;
            item1 =
                (*(*this).collider.elements.offset(0 as libc::c_int as
                                                       isize)).info.acHitInfo;
            if (*this).actionFunc ==
                   Some(BossDodongo_Vulnerable as
                            unsafe extern "C" fn(_: *mut BossDodongo,
                                                 _: *mut GlobalContext) -> ())
                   ||
                   (*this).actionFunc ==
                       Some(BossDodongo_LayDown as
                                unsafe extern "C" fn(_: *mut BossDodongo,
                                                     _: *mut GlobalContext)
                                    -> ()) {
                damage =
                    CollisionCheck_GetSwordDamage((*item1).toucher.dmgFlags as
                                                      s32) as s32;
                swordDamage = damage as u8_0;
                if damage != 0 as libc::c_int {
                    Audio_PlayActorSound2(&mut (*this).actor,
                                          0x3806 as libc::c_int as u16_0);
                    BossDodongo_SetupDamaged(this);
                    (*this).unk_1C0 = 5 as libc::c_int as s16;
                    (*this).health =
                        ((*this).health as libc::c_int -
                             swordDamage as libc::c_int) as s16
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossDodongo_SetupDeathCutscene(mut this:
                                                            *mut BossDodongo) {
    (*this).actor.speedXZ = 0.0f32;
    (*this).unk_1E4 = 0.0f32;
    Animation_Change(&mut (*this).skelAnime,
                     &mut object_kingdodongo_Anim_002D0C, 1.0f32, 0.0f32,
                     Animation_GetLastFrame(&mut object_kingdodongo_Anim_002D0C
                                                as *mut AnimationHeader as
                                                *mut libc::c_void) as f32_0,
                     ANIMMODE_ONCE as libc::c_int as u8_0, -5.0f32);
    (*this).actionFunc =
        Some(BossDodongo_DeathCutscene as
                 unsafe extern "C" fn(_: *mut BossDodongo,
                                      _: *mut GlobalContext) -> ());
    Audio_PlayActorSound2(&mut (*this).actor, 0x3807 as libc::c_int as u16_0);
    (*this).unk_1DA = 0 as libc::c_int as s16;
    (*this).csState = 0 as libc::c_int as s16;
    (*this).actor.flags &=
        !((1 as libc::c_int) << 0 as libc::c_int |
              (1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint;
    (*this).unk_1BC = 1 as libc::c_int as s16;
    Audio_QueueSeqCmd(((0x1 as libc::c_int) << 28 as libc::c_int |
                           (SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                               24 as libc::c_int | 0x100ff as libc::c_int) as
                          u32_0);
}
#[no_mangle]
pub unsafe extern "C" fn BossDodongo_DeathCutscene(mut this: *mut BossDodongo,
                                                   mut globalCtx:
                                                       *mut GlobalContext) {
    let mut cornerPos: *mut Vec3f = 0 as *mut Vec3f;
    let mut sp198: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp184: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut tempSin: f32_0 = 0.;
    let mut tempCos: f32_0 = 0.;
    let mut sp178: f32_0 = 0.;
    let mut i: s16 = 0;
    let mut effectPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut camera: *mut Camera = 0 as *mut Camera;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    SkelAnime_Update(&mut (*this).skelAnime);
    let mut current_block_256: u64;
    match (*this).csState as libc::c_int {
        0 => {
            (*this).csState = 5 as libc::c_int as s16;
            func_80064520(globalCtx, &mut (*globalCtx).csCtx);
            func_8002DF54(globalCtx, &mut (*this).actor,
                          1 as libc::c_int as u8_0);
            (*this).cutsceneCamera = Gameplay_CreateSubCamera(globalCtx);
            Gameplay_ChangeCameraStatus(globalCtx, 0 as libc::c_int as s16,
                                        3 as libc::c_int as s16);
            Gameplay_ChangeCameraStatus(globalCtx, (*this).cutsceneCamera,
                                        7 as libc::c_int as s16);
            camera = Gameplay_GetCamera(globalCtx, 0 as libc::c_int as s16);
            (*this).cameraEye.x = (*camera).eye.x;
            (*this).cameraEye.y = (*camera).eye.y;
            (*this).cameraEye.z = (*camera).eye.z;
            (*this).cameraAt.x = (*camera).at.x;
            (*this).cameraAt.y = (*camera).at.y;
            (*this).cameraAt.z = (*camera).at.z;
            current_block_256 = 2385072656283107554;
        }
        5 => {
            tempSin =
                Math_SinS(((*this).actor.shape.rot.y as libc::c_int -
                               0x1388 as libc::c_int) as s16) * 150.0f32;
            tempCos =
                Math_CosS(((*this).actor.shape.rot.y as libc::c_int -
                               0x1388 as libc::c_int) as s16) * 150.0f32;
            Math_SmoothStepToF(&mut (*player).actor.world.pos.x,
                               (*this).actor.world.pos.x + tempSin, 0.5f32,
                               5.0f32, 0.0f32);
            Math_SmoothStepToF(&mut (*player).actor.world.pos.z,
                               (*this).actor.world.pos.z + tempCos, 0.5f32,
                               5.0f32, 0.0f32);
            Math_SmoothStepToF(&mut (*this).unk_208, 0.07f32, 1.0f32,
                               0.005f32, 0.0f32);
            tempSin = Math_SinS((*this).actor.world.rot.y) * 230.0f32;
            tempCos = Math_CosS((*this).actor.world.rot.y) * 230.0f32;
            Math_SmoothStepToF(&mut (*this).cameraEye.x,
                               (*this).actor.world.pos.x + tempSin, 0.2f32,
                               50.0f32, 0.1f32);
            Math_SmoothStepToF(&mut (*this).cameraEye.y,
                               (*this).actor.world.pos.y + 20.0f32, 0.2f32,
                               50.0f32, 0.1f32);
            Math_SmoothStepToF(&mut (*this).cameraEye.z,
                               (*this).actor.world.pos.z + tempCos, 0.2f32,
                               50.0f32, 0.1f32);
            Math_SmoothStepToF(&mut (*this).cameraAt.x,
                               (*this).actor.world.pos.x, 0.2f32, 30.0f32,
                               0.1f32);
            Math_SmoothStepToF(&mut (*this).cameraAt.y,
                               (*this).actor.focus.pos.y - 70.0f32, 0.2f32,
                               30.0f32, 0.1f32);
            Math_SmoothStepToF(&mut (*this).cameraAt.z,
                               (*this).actor.world.pos.z, 0.2f32, 30.0f32,
                               0.1f32);
            if Animation_OnFrame(&mut (*this).skelAnime,
                                 Animation_GetLastFrame(&mut object_kingdodongo_Anim_002D0C
                                                            as
                                                            *mut AnimationHeader
                                                            as
                                                            *mut libc::c_void)
                                     as f32_0) != 0 {
                Animation_Change(&mut (*this).skelAnime,
                                 &mut object_kingdodongo_Anim_003CF8, 1.0f32,
                                 0.0f32,
                                 Animation_GetLastFrame(&mut object_kingdodongo_Anim_003CF8
                                                            as
                                                            *mut AnimationHeader
                                                            as
                                                            *mut libc::c_void)
                                     as f32_0,
                                 ANIMMODE_ONCE as libc::c_int as u8_0,
                                 -1.0f32);
                (*this).csState = 6 as libc::c_int as s16;
                Actor_Spawn(&mut (*globalCtx).actorCtx, globalCtx,
                            ACTOR_BG_BREAKWALL as libc::c_int as s16,
                            -890.0f32, -1523.76f32, -3304.0f32,
                            0 as libc::c_int as s16, 0 as libc::c_int as s16,
                            0 as libc::c_int as s16,
                            0x6000 as libc::c_int as s16);
            }
            current_block_256 = 2385072656283107554;
        }
        6 => {
            Math_SmoothStepToF(&mut (*this).cameraAt.x,
                               (*this).actor.world.pos.x, 0.2f32, 30.0f32,
                               0.1f32);
            Math_SmoothStepToF(&mut (*this).cameraAt.y,
                               (*this).actor.world.pos.y - 70.0f32 + 130.0f32,
                               0.2f32, 20.0f32, 0.1f32);
            Math_SmoothStepToF(&mut (*this).cameraAt.z,
                               (*this).actor.world.pos.z, 0.2f32, 30.0f32,
                               0.1f32);
            if Animation_OnFrame(&mut (*this).skelAnime,
                                 Animation_GetLastFrame(&mut object_kingdodongo_Anim_003CF8
                                                            as
                                                            *mut AnimationHeader
                                                            as
                                                            *mut libc::c_void)
                                     as f32_0) != 0 {
                Animation_Change(&mut (*this).skelAnime,
                                 &mut object_kingdodongo_Anim_00DF38, 1.0f32,
                                 30.0f32, 59.0f32,
                                 ANIMMODE_ONCE as libc::c_int as u8_0,
                                 -1.0f32);
                (*this).csState = 7 as libc::c_int as s16;
                (*this).unk_228 = 7700.0f32;
                (*this).unk_204 = 0.0f32;
                (*this).unk_1E4 = 0.0f32;
                (*this).numWallCollisions = 0 as libc::c_int as s16;
                (*this).unk_19E = 0 as libc::c_int as s16
            }
            current_block_256 = 2385072656283107554;
        }
        7 => {
            (*this).unk_1C4 =
                ((*this).unk_1C4 as libc::c_int + 0x7d0 as libc::c_int) as
                    s16;
            Math_SmoothStepToF(&mut (*this).cameraAt.x,
                               (*this).actor.world.pos.x, 0.2f32, 30.0f32,
                               0.0f32);
            Math_SmoothStepToF(&mut (*this).cameraAt.y,
                               (*this).actor.world.pos.y - 70.0f32 + 130.0f32,
                               0.2f32, 20.0f32, 0.0f32);
            Math_SmoothStepToF(&mut (*this).cameraAt.z,
                               (*this).actor.world.pos.z, 0.2f32, 30.0f32,
                               0.0f32);
            Math_SmoothStepToF(&mut (*this).cameraEye.x, -890.0f32, 0.1f32,
                               (*this).unk_204 * 5.0f32, 0.1f32);
            Math_SmoothStepToF(&mut (*this).cameraEye.z, -3304.0f32, 0.1f32,
                               (*this).unk_204 * 5.0f32, 0.1f32);
            Math_SmoothStepToF(&mut (*this).unk_204, 1.0f32, 1.0f32, 0.1f32,
                               0.0f32);
            if (*this).unk_1DA as libc::c_int == 1 as libc::c_int {
                (*this).csState = 8 as libc::c_int as s16;
                (*this).actor.speedXZ = (*this).unk_1E4 / 1.5f32;
                if (*this).unk_1A2 as libc::c_int == 0 as libc::c_int {
                    (*this).unk_238 = 250.0f32
                } else { (*this).unk_238 = -250.0f32 }
                (*this).unk_1DA = 1000 as libc::c_int as s16;
                (*this).unk_234 = 2000.0f32
            } else {
                cornerPos =
                    &mut *sCornerPositions.as_mut_ptr().offset((*this).unk_1A0
                                                                   as isize)
                        as *mut Vec3f;
                (*this).unk_1EC = 3.0f32;
                Math_SmoothStepToF(&mut (*this).unk_1E4,
                                   (*this).unk_1EC * 5.0f32, 1.0f32,
                                   (*this).unk_1EC * 0.25f32, 0.0f32);
                tempSin = (*cornerPos).x - (*this).actor.world.pos.x;
                tempCos = (*cornerPos).z - (*this).actor.world.pos.z;
                sp178 =
                    sqrtf(tempSin * tempSin + tempCos * tempCos) - 200.0f32;
                if sqrtf(tempSin * tempSin + tempCos * tempCos) < 200.0f32 ||
                       (*this).unk_1DA as libc::c_int != 0 as libc::c_int {
                    sp178 = 0.0f32
                }
                sp178 = if sp178 > 70.0f32 { 70.0f32 } else { sp178 };
                (*this).unk_23C =
                    Math_SinS(((*this).unk_19E as libc::c_int *
                                   1000 as libc::c_int) as s16) * -50.0f32 /
                        100.0f32;
                sp198.x =
                    Math_SinS(((*this).unk_19E as libc::c_int *
                                   1000 as libc::c_int) as s16) * sp178;
                sp198.z = 0.0f32;
                sp198.y = sp198.z;
                Matrix_RotateY((*this).actor.shape.rot.y as libc::c_int as
                                   libc::c_float *
                                   (3.14159265358979323846f32 /
                                        0x8000 as libc::c_int as
                                            libc::c_float),
                               MTXMODE_NEW as libc::c_int as u8_0);
                Matrix_MultVec3f(&mut sp198, &mut sp184);
                Math_SmoothStepToF(&mut (*this).actor.world.pos.x,
                                   (*cornerPos).x + sp184.x, 1.0f32,
                                   (*this).unk_1E4, 0.0f32);
                Math_SmoothStepToF(&mut (*this).actor.world.pos.z,
                                   (*cornerPos).z + sp184.z, 1.0f32,
                                   (*this).unk_1E4, 0.0f32);
                Audio_PlayActorSound2(&mut (*this).actor,
                                      (0x384e as libc::c_int -
                                           0x800 as libc::c_int) as u16_0);
                if (*this).unk_19E as libc::c_int & 7 as libc::c_int ==
                       0 as libc::c_int {
                    Camera_AddQuake(&mut (*globalCtx).mainCamera,
                                    2 as libc::c_int, 1 as libc::c_int as s16,
                                    8 as libc::c_int);
                }
                if (*this).unk_19E as libc::c_int & 1 as libc::c_int == 0 {
                    Actor_SpawnFloorDustRing(globalCtx, &mut (*this).actor,
                                             &mut (*this).actor.world.pos,
                                             40.0f32, 3 as libc::c_int,
                                             8.0f32,
                                             0x1f4 as libc::c_int as s16,
                                             0xa as libc::c_int as s16,
                                             0 as libc::c_int as u8_0);
                }
                tempSin = (*cornerPos).x - (*this).actor.world.pos.x;
                tempCos = (*cornerPos).z - (*this).actor.world.pos.z;
                Math_SmoothStepToF(&mut (*this).unk_1E8, 1500.0f32, 1.0f32,
                                   (*this).unk_1EC * 100.0f32, 0.0f32);
                Math_SmoothStepToS(&mut (*this).actor.world.rot.y,
                                   (Math_FAtan2F(tempSin, tempCos) *
                                        (0x8000 as libc::c_int as
                                             libc::c_float /
                                             3.14159265358979323846f32)) as
                                       s16, 5 as libc::c_int as s16,
                                   ((*this).unk_1EC * (*this).unk_1E8) as s16,
                                   0 as libc::c_int as s16);
                if fabsf(tempSin) <= 15.0f32 && fabsf(tempCos) <= 15.0f32 {
                    let mut dustPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                    (*this).actor.velocity.y = 15.0f32;
                    Audio_PlayActorSound2(&mut (*this).actor,
                                          0x384d as libc::c_int as u16_0);
                    if (*this).unk_1A2 as libc::c_int == 0 as libc::c_int {
                        (*this).unk_1A0 =
                            ((*this).unk_1A0 as libc::c_int +
                                 1 as libc::c_int) as s16;
                        if (*this).unk_1A0 as libc::c_int >= 4 as libc::c_int
                           {
                            (*this).unk_1A0 = 0 as libc::c_int as s16
                        }
                    } else {
                        (*this).unk_1A0 -= 1;
                        if ((*this).unk_1A0 as libc::c_int) < 0 as libc::c_int
                           {
                            (*this).unk_1A0 = 3 as libc::c_int as s16
                        }
                    }
                    (*this).unk_1DA = 0xa as libc::c_int as s16;
                    dustPos.x = (*this).actor.world.pos.x;
                    dustPos.y = (*this).actor.world.pos.y + 60.0f32;
                    dustPos.z = (*this).actor.world.pos.z;
                    func_80033480(globalCtx, &mut dustPos, 250.0f32,
                                  0x28 as libc::c_int,
                                  0x320 as libc::c_int as s16,
                                  0xa as libc::c_int as s16,
                                  0 as libc::c_int as u8_0);
                }
            }
            current_block_256 = 2385072656283107554;
        }
        8 | 9 => {
            if (*this).unk_1DA as libc::c_int == 884 as libc::c_int {
                Animation_Change(&mut (*this).skelAnime,
                                 &mut object_kingdodongo_Anim_0042A8, 1.0f32,
                                 0.0f32,
                                 Animation_GetLastFrame(&mut object_kingdodongo_Anim_0042A8
                                                            as
                                                            *mut AnimationHeader
                                                            as
                                                            *mut libc::c_void)
                                     as f32_0,
                                 ANIMMODE_LOOP as libc::c_int as u8_0,
                                 -20.0f32);
                tempSin = (*this).cameraEye.x - (*this).actor.world.pos.x;
                tempCos = (*this).cameraEye.z - (*this).actor.world.pos.z;
                (*this).unk_22C =
                    sqrtf(tempSin * tempSin + tempCos * tempCos);
                (*this).unk_230 = Math_FAtan2F(tempSin, tempCos);
                (*this).unk_1DC = 350 as libc::c_int as s16;
                (*this).csState = 9 as libc::c_int as s16
            }
            if ((*this).unk_1DA as libc::c_int) < 854 as libc::c_int {
                i = 0 as libc::c_int as s16;
                while (i as libc::c_int) < 2 as libc::c_int {
                    func_808C12C4(D_808C7000.as_mut_ptr(), (*this).unk_1CC);
                    if ((*this).unk_1CC as libc::c_int) < 256 as libc::c_int {
                        (*this).unk_1CC += 1
                    }
                    i += 1
                }
            }
            if ((*this).unk_1DA as libc::c_int) < 984 as libc::c_int {
                Math_SmoothStepToS(&mut (*this).unk_1C4,
                                   -(0x4000 as libc::c_int) as s16,
                                   0xa as libc::c_int as s16,
                                   0x12c as libc::c_int as s16,
                                   0 as libc::c_int as s16);
            }
            if (*this).unk_1DA as libc::c_int == 904 as libc::c_int {
                Audio_PlayActorSound2(&mut (*this).actor,
                                      0x3853 as libc::c_int as u16_0);
            }
            if ((*this).unk_1DA as libc::c_int) < 854 as libc::c_int {
                Audio_PlayActorSound2(&mut (*this).actor,
                                      (0x3854 as libc::c_int -
                                           0x800 as libc::c_int) as u16_0);
            }
            if (*this).unk_1DA as libc::c_int == 960 as libc::c_int {
                Audio_PlayActorSound2(&mut (*this).actor,
                                      0x3855 as libc::c_int as u16_0);
            }
            if ((*this).unk_1DA as libc::c_int) < 960 as libc::c_int {
                Math_SmoothStepToF(&mut (*this).actor.shape.shadowScale,
                                   0.0f32, 1.0f32, 10.0f32, 0.0f32);
                if (*this).unk_1DA as libc::c_int >= 710 as libc::c_int {
                    if (*this).unk_1DA as libc::c_int == 710 as libc::c_int {
                        let mut sp124: [Vec3f; 4] =
                            [{
                                 let mut init =
                                     Vec3f{x: -440.0f32,
                                           y: 0.0f32,
                                           z: -3304.0f32,};
                                 init
                             },
                             {
                                 let mut init =
                                     Vec3f{x: -890.0f32,
                                           y: 0.0f32,
                                           z: -3754.0f32,};
                                 init
                             },
                             {
                                 let mut init =
                                     Vec3f{x: -1340.0f32,
                                           y: 0.0f32,
                                           z: -3304.0f32,};
                                 init
                             },
                             {
                                 let mut init =
                                     Vec3f{x: -890.0f32,
                                           y: 0.0f32,
                                           z: -2854.0f32,};
                                 init
                             }];
                        let mut spF4: [Vec3f; 4] =
                            [{
                                 let mut init =
                                     Vec3f{x: -890.0f32,
                                           y: 0.0f32,
                                           z: -2854.0f32,};
                                 init
                             },
                             {
                                 let mut init =
                                     Vec3f{x: -440.0f32,
                                           y: 0.0f32,
                                           z: -3304.0f32,};
                                 init
                             },
                             {
                                 let mut init =
                                     Vec3f{x: -890.0f32,
                                           y: 0.0f32,
                                           z: -3754.0f32,};
                                 init
                             },
                             {
                                 let mut init =
                                     Vec3f{x: -1340.0f32,
                                           y: 0.0f32,
                                           z: -3304.0f32,};
                                 init
                             }];
                        let mut phi_v0_2: *mut Vec3f = 0 as *mut Vec3f;
                        (*this).unk_1C6 = 1 as libc::c_int as s16;
                        if (*this).unk_1A2 as libc::c_int == 0 as libc::c_int
                           {
                            phi_v0_2 =
                                &mut *sp124.as_mut_ptr().offset((*this).unk_1A0
                                                                    as isize)
                                    as *mut Vec3f
                        } else {
                            phi_v0_2 =
                                &mut *spF4.as_mut_ptr().offset((*this).unk_1A0
                                                                   as isize)
                                    as *mut Vec3f
                        }
                        (*player).actor.world.pos.x = (*phi_v0_2).x;
                        (*player).actor.world.pos.z = (*phi_v0_2).z;
                        (*this).unk_204 = 0.0f32
                    }
                    if (*this).unk_1DA as libc::c_int >= 885 as libc::c_int {
                        Math_SmoothStepToF(&mut (*this).unk_228,
                                           200.0f64 as f32_0, 0.2f32,
                                           100.0f32, 0.0f32);
                    } else {
                        Math_SmoothStepToF(&mut (*this).unk_228, -6600.0f32,
                                           0.2f32, 30.0f32, 0.0f32);
                    }
                    static mut dustVel: Vec3f =
                        {
                            let mut init =
                                Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,};
                            init
                        };
                    static mut dustAcell: Vec3f =
                        {
                            let mut init =
                                Vec3f{x: 0.0f32, y: 1.0f32, z: 0.0f32,};
                            init
                        };
                    static mut dustPrimColor: Color_RGBA8 =
                        {
                            let mut init =
                                Color_RGBA8{r: 255 as libc::c_int as u8_0,
                                            g: 255 as libc::c_int as u8_0,
                                            b: 100 as libc::c_int as u8_0,
                                            a: 255 as libc::c_int as u8_0,};
                            init
                        };
                    static mut dustEnvColor: Color_RGBA8 =
                        {
                            let mut init =
                                Color_RGBA8{r: 255 as libc::c_int as u8_0,
                                            g: 100 as libc::c_int as u8_0,
                                            b: 0 as libc::c_int as u8_0,
                                            a: 255 as libc::c_int as u8_0,};
                            init
                        };
                    let mut colorIndex: s16 = 0;
                    let mut magmaPrimColor2: [Color_RGBA8; 2] =
                        [{
                             let mut init =
                                 Color_RGBA8{r: 255 as libc::c_int as u8_0,
                                             g: 255 as libc::c_int as u8_0,
                                             b: 0 as libc::c_int as u8_0,
                                             a: 255 as libc::c_int as u8_0,};
                             init
                         },
                         {
                             let mut init =
                                 Color_RGBA8{r: 0 as libc::c_int as u8_0,
                                             g: 0 as libc::c_int as u8_0,
                                             b: 0 as libc::c_int as u8_0,
                                             a: 100 as libc::c_int as u8_0,};
                             init
                         }];
                    let mut magmaEnvColor2: [Color_RGBA8; 2] =
                        [{
                             let mut init =
                                 Color_RGBA8{r: 255 as libc::c_int as u8_0,
                                             g: 0 as libc::c_int as u8_0,
                                             b: 0 as libc::c_int as u8_0,
                                             a: 255 as libc::c_int as u8_0,};
                             init
                         },
                         {
                             let mut init =
                                 Color_RGBA8{r: 0 as libc::c_int as u8_0,
                                             g: 0 as libc::c_int as u8_0,
                                             b: 0 as libc::c_int as u8_0,
                                             a: 0 as libc::c_int as u8_0,};
                             init
                         }];
                    effectPos.x =
                        Rand_CenteredFloat(120.0f32) +
                            (*this).actor.focus.pos.x;
                    effectPos.y =
                        Rand_ZeroFloat(50.0f32) + (*this).actor.world.pos.y;
                    effectPos.z =
                        Rand_CenteredFloat(120.0f32) +
                            (*this).actor.focus.pos.z;
                    func_8002836C(globalCtx, &mut effectPos, &mut dustVel,
                                  &mut dustAcell, &mut dustPrimColor,
                                  &mut dustEnvColor,
                                  0x1f4 as libc::c_int as s16,
                                  0xa as libc::c_int as s16,
                                  0xa as libc::c_int as s16);
                    effectPos.x =
                        Rand_CenteredFloat(120.0f32) +
                            (*this).actor.focus.pos.x;
                    effectPos.y = -1498.76f32;
                    effectPos.z =
                        Rand_CenteredFloat(120.0f32) +
                            (*this).actor.focus.pos.z;
                    colorIndex = (Rand_ZeroOne() * 1.9f32) as s16;
                    EffectSsGMagma2_Spawn(globalCtx, &mut effectPos,
                                          &mut *magmaPrimColor2.as_mut_ptr().offset(colorIndex
                                                                                        as
                                                                                        isize),
                                          &mut *magmaEnvColor2.as_mut_ptr().offset(colorIndex
                                                                                       as
                                                                                       isize),
                                          (10 as libc::c_int -
                                               colorIndex as libc::c_int *
                                                   5 as libc::c_int) as s16,
                                          colorIndex,
                                          ((Rand_ZeroOne() * 100.0f32) as s16
                                               as libc::c_int +
                                               100 as libc::c_int) as s16);
                }
            } else {
                Audio_PlayActorSound2(&mut (*this).actor,
                                      (0x384e as libc::c_int -
                                           0x800 as libc::c_int) as u16_0);
                if (*this).unk_19E as libc::c_int & 1 as libc::c_int == 0 {
                    Actor_SpawnFloorDustRing(globalCtx, &mut (*this).actor,
                                             &mut (*this).actor.world.pos,
                                             40.0f32, 3 as libc::c_int,
                                             8.0f32,
                                             0x1f4 as libc::c_int as s16,
                                             0xa as libc::c_int as s16,
                                             0 as libc::c_int as u8_0);
                }
            }
            Math_SmoothStepToF(&mut (*this).actor.speedXZ, 0.0f32, 0.2f32,
                               0.1f32, 0.0f32);
            (*this).actor.world.rot.y =
                ((*this).actor.world.rot.y as libc::c_int +
                     (*this).unk_238 as s16 as libc::c_int) as s16;
            (*this).unk_1C4 =
                ((*this).unk_1C4 as libc::c_int +
                     (*this).unk_234 as s16 as libc::c_int) as s16;
            if (*this).unk_1DA as libc::c_int >= 0x367 as libc::c_int {
                if (*this).unk_1A2 as libc::c_int == 0 as libc::c_int {
                    if (*this).unk_238 < 450.0f32 {
                        (*this).unk_238 += 10.0f32
                    }
                } else if -450.0f32 < (*this).unk_238 {
                    (*this).unk_238 -= 10.0f32
                }
            } else {
                Math_SmoothStepToF(&mut (*this).unk_238, 0.0f32, 0.05f32,
                                   40.0f32, 0.0f32);
            }
            Math_SmoothStepToF(&mut (*this).unk_234, 0.0f32, 0.2f32, 17.0f32,
                               0.0f32);
            Math_SmoothStepToF(&mut (*this).cameraAt.x,
                               (*this).actor.world.pos.x, 0.2f32, 30.0f32,
                               0.0f32);
            Math_SmoothStepToF(&mut (*this).cameraAt.y,
                               (*this).actor.world.pos.y - 70.0f32 + 130.0f32,
                               0.2f32, 20.0f32, 0.0f32);
            Math_SmoothStepToF(&mut (*this).cameraAt.z,
                               (*this).actor.world.pos.z, 0.2f32, 30.0f32,
                               0.0f32);
            if (*this).csState as libc::c_int == 9 as libc::c_int {
                if ((*this).unk_1DA as libc::c_int) < 0x2c6 as libc::c_int {
                    let mut spAC: [Vec3f; 4] =
                        [{
                             let mut init =
                                 Vec3f{x: -390.0f32,
                                       y: 0.0f32,
                                       z: -3304.0f32,};
                             init
                         },
                         {
                             let mut init =
                                 Vec3f{x: -890.0f32,
                                       y: 0.0f32,
                                       z: -3804.0f32,};
                             init
                         },
                         {
                             let mut init =
                                 Vec3f{x: -1390.0f32,
                                       y: 0.0f32,
                                       z: -3304.0f32,};
                             init
                         },
                         {
                             let mut init =
                                 Vec3f{x: -890.0f32,
                                       y: 0.0f32,
                                       z: -2804.0f32,};
                             init
                         }];
                    let mut sp7C: [Vec3f; 4] =
                        [{
                             let mut init =
                                 Vec3f{x: -890.0f32,
                                       y: 0.0f32,
                                       z: -2804.0f32,};
                             init
                         },
                         {
                             let mut init =
                                 Vec3f{x: -390.0f32,
                                       y: 0.0f32,
                                       z: -3304.0f32,};
                             init
                         },
                         {
                             let mut init =
                                 Vec3f{x: -890.0f32,
                                       y: 0.0f32,
                                       z: -3804.0f32,};
                             init
                         },
                         {
                             let mut init =
                                 Vec3f{x: -1390.0f32,
                                       y: 0.0f32,
                                       z: -3304.0f32,};
                             init
                         }];
                    let mut sp78: *mut Vec3f = 0 as *mut Vec3f;
                    let mut pad74: s32 = 0;
                    if (*this).unk_1A2 as libc::c_int == 0 as libc::c_int {
                        sp78 =
                            &mut *spAC.as_mut_ptr().offset((*this).unk_1A0 as
                                                               isize) as
                                *mut Vec3f
                    } else {
                        sp78 =
                            &mut *sp7C.as_mut_ptr().offset((*this).unk_1A0 as
                                                               isize) as
                                *mut Vec3f
                    }
                    Math_SmoothStepToF(&mut (*this).cameraEye.x, (*sp78).x,
                                       0.2f32, (*this).unk_204 * 20.0f32,
                                       0.0f32);
                    Math_SmoothStepToF(&mut (*this).cameraEye.y,
                                       (*player).actor.world.pos.y + 30.0f32,
                                       0.1f32, (*this).unk_204 * 20.0f32,
                                       0.0f32);
                    Math_SmoothStepToF(&mut (*this).cameraEye.z, (*sp78).z,
                                       0.1f32, (*this).unk_204 * 20.0f32,
                                       0.0f32);
                    Math_SmoothStepToF(&mut (*this).unk_204, 1.0f32, 1.0f32,
                                       0.02f32, 0.0f32);
                } else {
                    if (*this).unk_1A2 as libc::c_int == 0 as libc::c_int {
                        (*this).unk_230 += 0.01f32
                    } else { (*this).unk_230 -= 0.01f32 }
                    Math_SmoothStepToF(&mut (*this).unk_22C, 220.0f32, 0.1f32,
                                       5.0f32, 0.1f32);
                    tempSin = sinf((*this).unk_230) * (*this).unk_22C;
                    tempCos = cosf((*this).unk_230) * (*this).unk_22C;
                    Math_SmoothStepToF(&mut (*this).cameraEye.x,
                                       (*this).actor.world.pos.x + tempSin,
                                       0.2f32, 50.0f32, 0.0f32);
                    Math_SmoothStepToF(&mut (*this).cameraEye.y,
                                       (*this).actor.world.pos.y + 20.0f32,
                                       0.2f32, 50.0f32, 0.0f32);
                    Math_SmoothStepToF(&mut (*this).cameraEye.z,
                                       (*this).actor.world.pos.z + tempCos,
                                       0.2f32, 50.0f32, 0.0f32);
                    Math_SmoothStepToF(&mut (*this).unk_23C, 0.0f32, 0.2f32,
                                       0.01f32, 0.0f32);
                }
            } else {
                if (*this).unk_1A2 as libc::c_int == 0 as libc::c_int {
                    Math_SmoothStepToF(&mut (*this).unk_23C, -0.5f32, 0.2f32,
                                       0.05f32, 0.0f32);
                } else {
                    Math_SmoothStepToF(&mut (*this).unk_23C, 0.5f32, 0.2f32,
                                       0.05f32, 0.0f32);
                }
                Math_SmoothStepToF(&mut (*this).cameraEye.x, -890.0f32,
                                   0.1f32, (*this).unk_204 * 5.0f32, 0.1f32);
                Math_SmoothStepToF(&mut (*this).cameraEye.z, -3304.0f32,
                                   0.1f32, (*this).unk_204 * 5.0f32, 0.1f32);
                Math_SmoothStepToF(&mut (*this).unk_204, 1.0f32, 1.0f32,
                                   0.05f32, 0.0f32);
            }
            if (*this).unk_1DA as libc::c_int == 820 as libc::c_int {
                Audio_QueueSeqCmd(((SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                                       24 as libc::c_int |
                                       0x21 as libc::c_int) as u32_0);
                Actor_Spawn(&mut (*globalCtx).actorCtx, globalCtx,
                            ACTOR_ITEM_B_HEART as libc::c_int as s16,
                            Math_SinS((*this).actor.shape.rot.y) * -50.0f32 +
                                (*this).actor.world.pos.x,
                            (*this).actor.world.pos.y,
                            Math_CosS((*this).actor.shape.rot.y) * -50.0f32 +
                                (*this).actor.world.pos.z,
                            0 as libc::c_int as s16, 0 as libc::c_int as s16,
                            0 as libc::c_int as s16, 0 as libc::c_int as s16);
            }
            if (*this).unk_1DA as libc::c_int == 600 as libc::c_int {
                camera =
                    Gameplay_GetCamera(globalCtx, 0 as libc::c_int as s16);
                (*camera).eye = (*this).cameraEye;
                (*camera).eyeNext = (*this).cameraEye;
                (*camera).at = (*this).cameraAt;
                func_800C08AC(globalCtx, (*this).cutsceneCamera,
                              0 as libc::c_int as s16);
                (*this).unk_1BC = 0 as libc::c_int as s16;
                (*this).cutsceneCamera = 0 as libc::c_int as s16;
                (*this).csState = 100 as libc::c_int as s16;
                Gameplay_ChangeCameraStatus(globalCtx,
                                            0 as libc::c_int as s16,
                                            7 as libc::c_int as s16);
                func_80064534(globalCtx, &mut (*globalCtx).csCtx);
                func_8002DF54(globalCtx, &mut (*this).actor,
                              7 as libc::c_int as u8_0);
                Actor_SpawnAsChild(&mut (*globalCtx).actorCtx,
                                   &mut (*this).actor, globalCtx,
                                   ACTOR_DOOR_WARP1 as libc::c_int as s16,
                                   -890.0f32, -1523.76f32, -3304.0f32,
                                   0 as libc::c_int as s16,
                                   0 as libc::c_int as s16,
                                   0 as libc::c_int as s16,
                                   WARP_DUNGEON_CHILD as libc::c_int as s16);
                (*this).skelAnime.playSpeed = 0.0f32;
                Flags_SetClear(globalCtx,
                               (*globalCtx).roomCtx.curRoom.num as s32);
            }
            current_block_256 = 966527531074818691;
        }
        100 => { current_block_256 = 966527531074818691; }
        _ => { current_block_256 = 2385072656283107554; }
    }
    match current_block_256 {
        966527531074818691 => {
            if ((*this).unk_1DA as libc::c_int) < 0x2c6 as libc::c_int &&
                   Rand_ZeroOne() < 0.5f32 {
                let mut sp68: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                let mut D_808CA568: Color_RGBA8 =
                    {
                        let mut init =
                            Color_RGBA8{r: 0 as libc::c_int as u8_0,
                                        g: 0 as libc::c_int as u8_0,
                                        b: 0 as libc::c_int as u8_0,
                                        a: 100 as libc::c_int as u8_0,};
                        init
                    };
                let mut D_808CA56C: Color_RGBA8 =
                    {
                        let mut init =
                            Color_RGBA8{r: 0 as libc::c_int as u8_0,
                                        g: 0 as libc::c_int as u8_0,
                                        b: 0 as libc::c_int as u8_0,
                                        a: 0 as libc::c_int as u8_0,};
                        init
                    };
                sp68.x =
                    Rand_CenteredFloat(60.0f32) + (*this).actor.focus.pos.x;
                sp68.y = Rand_ZeroOne() * 50.0f32 + -1498.76f32;
                sp68.z =
                    Rand_CenteredFloat(60.0f32) + (*this).actor.focus.pos.z;
                EffectSsGMagma2_Spawn(globalCtx, &mut sp68, &mut D_808CA568,
                                      &mut D_808CA56C,
                                      5 as libc::c_int as s16,
                                      1 as libc::c_int as s16,
                                      ((Rand_ZeroOne() * 50.0f32) as s16 as
                                           libc::c_int + 50 as libc::c_int) as
                                          s16);
            }
        }
        _ => { }
    }
    if (*this).cutsceneCamera as libc::c_int != 0 as libc::c_int {
        Gameplay_CameraSetAtEye(globalCtx, (*this).cutsceneCamera,
                                &mut (*this).cameraAt,
                                &mut (*this).cameraEye);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossDodongo_UpdateEffects(mut globalCtx:
                                                       *mut GlobalContext) {
    let mut eff: *mut BossDodongoEffect =
        (*globalCtx).specialEffects as *mut BossDodongoEffect;
    let mut effectColors: [Color_RGB8; 4] =
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
         }];
    let mut colorIndex: s16 = 0;
    let mut i: s16 = 0;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < 80 as libc::c_int {
        if (*eff).unk_24 as libc::c_int != 0 as libc::c_int {
            (*eff).unk_00.x += (*eff).unk_0C.x;
            (*eff).unk_00.y += (*eff).unk_0C.y;
            (*eff).unk_00.z += (*eff).unk_0C.z;
            (*eff).unk_25 = (*eff).unk_25.wrapping_add(1);
            (*eff).unk_0C.x += (*eff).unk_18.x;
            (*eff).unk_0C.y += (*eff).unk_18.y;
            (*eff).unk_0C.z += (*eff).unk_18.z;
            if (*eff).unk_24 as libc::c_int == 1 as libc::c_int {
                colorIndex =
                    ((*eff).unk_25 as libc::c_int % 4 as libc::c_int) as s16;
                (*eff).color.r = effectColors[colorIndex as usize].r;
                (*eff).color.g = effectColors[colorIndex as usize].g;
                (*eff).color.b = effectColors[colorIndex as usize].b;
                (*eff).alpha =
                    ((*eff).alpha as libc::c_int - 20 as libc::c_int) as s16;
                if (*eff).alpha as libc::c_int <= 0 as libc::c_int {
                    (*eff).alpha = 0 as libc::c_int as s16;
                    (*eff).unk_24 = 0 as libc::c_int as u8_0
                }
            }
        }
        i += 1;
        eff = eff.offset(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn BossDodongo_DrawEffects(mut globalCtx:
                                                     *mut GlobalContext) {
    let mut unkMtx: *mut MtxF = 0 as *mut MtxF;
    let mut i: s16 = 0;
    let mut phi_s3: u8_0 = 0 as libc::c_int as u8_0;
    let mut eff: *mut BossDodongoEffect = 0 as *mut BossDodongoEffect;
    let mut gfxCtx: *mut GraphicsContext = (*globalCtx).state.gfxCtx;
    eff = (*globalCtx).specialEffects as *mut BossDodongoEffect;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), gfxCtx,
                    b"../z_boss_dodongo.c\x00" as *const u8 as
                        *const libc::c_char, 5228 as libc::c_int);
    func_80093D84((*globalCtx).state.gfxCtx);
    unkMtx = &mut (*globalCtx).billboardMtxF;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < 80 as libc::c_int {
        if (*eff).unk_24 as libc::c_int == 1 as libc::c_int {
            let fresh3 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g: *mut Gfx = fresh3;
            (*_g).words.w0 =
                (0xe7 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g).words.w1 = 0 as libc::c_int as libc::c_uint;
            if phi_s3 as libc::c_int == 0 as libc::c_int {
                let fresh4 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_0: *mut Gfx = fresh4;
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
                    object_kingdodongo_DL_009D50.as_mut_ptr() as libc::c_uint;
                phi_s3 = phi_s3.wrapping_add(1)
            }
            let fresh5 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_1: *mut Gfx = fresh5;
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
                ((*eff).color.r as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((*eff).color.g as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    ((*eff).color.b as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    ((*eff).alpha as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            Matrix_Translate((*eff).unk_00.x, (*eff).unk_00.y,
                             (*eff).unk_00.z,
                             MTXMODE_NEW as libc::c_int as u8_0);
            func_800D1FD4(unkMtx);
            Matrix_Scale((*eff).unk_2C, (*eff).unk_2C, 1.0f32,
                         MTXMODE_APPLY as libc::c_int as u8_0);
            let fresh6 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_2: *mut Gfx = fresh6;
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
                              b"../z_boss_dodongo.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              5253 as libc::c_int) as libc::c_uint;
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
            (*_g_3).words.w1 =
                object_kingdodongo_DL_009DD0.as_mut_ptr() as libc::c_uint
        }
        i += 1;
        eff = eff.offset(1)
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), gfxCtx,
                     b"../z_boss_dodongo.c\x00" as *const u8 as
                         *const libc::c_char, 5258 as libc::c_int);
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
             init.set_value(0xc as libc::c_int);
             init
         },
         {
             let mut init = InitChainEntry{cont_type_0_offset_value: [0; 4],};
             init.set_cont(1 as libc::c_int as u32_0);
             init.set_type_0(ICHAINTYPE_F32_DIV1000 as libc::c_int as u32_0);
             init.set_offset(&mut (*(0 as *mut Actor)).gravity as *mut f32_0
                                 as size_t as u32_0);
             init.set_value(-3000.0f32 as s32);
             init
         },
         {
             let mut init = InitChainEntry{cont_type_0_offset_value: [0; 4],};
             init.set_cont(0 as libc::c_int as u32_0);
             init.set_type_0(ICHAINTYPE_F32 as libc::c_int as u32_0);
             init.set_offset(&mut (*(0 as *mut Actor)).targetArrowOffset as
                                 *mut f32_0 as size_t as u32_0);
             init.set_value(8200.0f32 as s32);
             init
         }]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
