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
    fn sqrt(d: f64_0) -> f64_0;
    #[no_mangle]
    fn __assert(exp: *const libc::c_char, file: *const libc::c_char,
                line: s32);
    #[no_mangle]
    fn func_800287AC(globalCtx: *mut GlobalContext, pos: *mut Vec3f,
                     velocity: *mut Vec3f, accel: *mut Vec3f, scale: s16,
                     scaleStep: s16, life: s16);
    #[no_mangle]
    fn func_80028A54(globalCtx: *mut GlobalContext, randScale: f32_0,
                     srcPos: *mut Vec3f);
    #[no_mangle]
    fn ActorShape_Init(shape: *mut ActorShape, yOffset: f32_0,
                       shadowDraw: ActorShadowFunc, shadowScale: f32_0);
    #[no_mangle]
    fn ActorShadow_DrawHorse(actor: *mut Actor, lights: *mut Lights,
                             globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn Actor_Kill(actor: *mut Actor);
    #[no_mangle]
    fn Actor_SetScale(actor: *mut Actor, scale: f32_0);
    #[no_mangle]
    fn Actor_SetObjectDependency(globalCtx: *mut GlobalContext,
                                 actor: *mut Actor);
    #[no_mangle]
    fn Actor_MoveForward(actor: *mut Actor);
    #[no_mangle]
    fn Actor_WorldYawTowardActor(actorA: *mut Actor, actorB: *mut Actor)
     -> s16;
    #[no_mangle]
    fn Actor_WorldDistXZToActor(actorA: *mut Actor, actorB: *mut Actor)
     -> f32_0;
    #[no_mangle]
    fn func_8002DD78(player: *mut Player) -> s32;
    #[no_mangle]
    fn Actor_UpdateBgCheckInfo(globalCtx: *mut GlobalContext,
                               actor: *mut Actor, wallCheckHeight: f32_0,
                               wallCheckRadius: f32_0,
                               ceilingCheckHeight: f32_0, flags: s32);
    #[no_mangle]
    fn Actor_IsMounted(globalCtx: *mut GlobalContext, horse: *mut Actor)
     -> s32;
    #[no_mangle]
    fn Actor_SetRideActor(globalCtx: *mut GlobalContext, horse: *mut Actor,
                          arg2: s32) -> u32_0;
    #[no_mangle]
    fn Actor_NotMounted(globalCtx: *mut GlobalContext, horse: *mut Actor)
     -> s32;
    #[no_mangle]
    fn func_800314D4(globalCtx: *mut GlobalContext, actorB: *mut Actor,
                     arg2: *mut Vec3f, arg3: f32_0) -> s32;
    #[no_mangle]
    fn Actor_Spawn(actorCtx: *mut ActorContext, globalCtx: *mut GlobalContext,
                   actorId: s16, posX: f32_0, posY: f32_0, posZ: f32_0,
                   rotX: s16, rotY: s16, rotZ: s16, params: s16)
     -> *mut Actor;
    #[no_mangle]
    fn Flags_GetEventChkInf(flag: s32) -> s32;
    #[no_mangle]
    fn BgCheck_EntityRaycastFloor3(colCtx: *mut CollisionContext,
                                   outPoly: *mut *mut CollisionPoly,
                                   bgId: *mut s32, pos: *mut Vec3f) -> f32_0;
    #[no_mangle]
    fn BgCheck_EntityLineTest1(colCtx: *mut CollisionContext,
                               posA: *mut Vec3f, posB: *mut Vec3f,
                               posResult: *mut Vec3f,
                               outPoly: *mut *mut CollisionPoly, chkWall: s32,
                               chkFloor: s32, chkCeil: s32, chkOneFace: s32,
                               bgId: *mut s32) -> s32;
    #[no_mangle]
    fn DynaPoly_GetActor(colCtx: *mut CollisionContext, bgId: s32)
     -> *mut DynaPolyActor;
    #[no_mangle]
    fn func_80041D4C(colCtx: *mut CollisionContext, poly: *mut CollisionPoly,
                     bgId: s32) -> u32_0;
    #[no_mangle]
    fn SurfaceType_IsHorseBlocked(colCtx: *mut CollisionContext,
                                  poly: *mut CollisionPoly, bgId: s32)
     -> u32_0;
    #[no_mangle]
    fn SurfaceType_GetConveyorDirection(colCtx: *mut CollisionContext,
                                        poly: *mut CollisionPoly, bgId: s32)
     -> u32_0;
    #[no_mangle]
    fn WaterBox_GetSurfaceImpl(globalCtx: *mut GlobalContext,
                               colCtx: *mut CollisionContext, x: f32_0,
                               z: f32_0, ySurface: *mut f32_0,
                               outWaterBox: *mut *mut WaterBox) -> s32;
    #[no_mangle]
    fn Camera_ChangeSetting(camera: *mut Camera, setting: s16) -> s32;
    #[no_mangle]
    fn Camera_SetParam(camera: *mut Camera, param: s32,
                       value: *mut libc::c_void) -> s32;
    #[no_mangle]
    fn Camera_SetCameraData(camera: *mut Camera, setDataFlags: s16,
                            data0: *mut libc::c_void,
                            data1: *mut libc::c_void, data2: s16, data3: s16,
                            arg6: s32);
    #[no_mangle]
    fn DamageTable_Get(index: s32) -> *mut DamageTable;
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
    fn func_8006DD9C(actor: *mut Actor, arg1: *mut Vec3f, arg2: s16);
    #[no_mangle]
    fn Math_CosS(angle: s16) -> f32_0;
    #[no_mangle]
    fn Math_SinS(angle: s16) -> f32_0;
    #[no_mangle]
    fn Math_Vec3f_Copy(dest: *mut Vec3f, src: *mut Vec3f);
    #[no_mangle]
    fn Math_Vec3f_Yaw(a: *mut Vec3f, b: *mut Vec3f) -> s16;
    #[no_mangle]
    fn Actor_ProcessInitChain(actor: *mut Actor,
                              initChain: *mut InitChainEntry);
    #[no_mangle]
    fn Interface_InitHorsebackArchery(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn func_80093D18(gfxCtx: *mut GraphicsContext);
    #[no_mangle]
    fn Object_GetIndex(objectCtx: *mut ObjectContext, objectId: s16) -> s32;
    #[no_mangle]
    fn Object_IsLoaded(objectCtx: *mut ObjectContext, bankIndex: s32) -> s32;
    #[no_mangle]
    fn Animation_GetLastFrame(animation: *mut libc::c_void) -> s16;
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
    fn func_800A5F60(gfxCtx: *mut GraphicsContext, skin: *mut PSkinAwb,
                     limbIndex: s32, arg3: *mut Gfx, arg4: s32);
    #[no_mangle]
    fn func_800A6360(this: *mut Actor, globalCtx: *mut GlobalContext,
                     skin: *mut PSkinAwb, callback: SkinCallback,
                     arg4: SkinCallback2, arg5: s32);
    #[no_mangle]
    fn func_800A6408(skin: *mut PSkinAwb, joint: s32, arg2: *mut Vec3f,
                     arg3: *mut Vec3f);
    #[no_mangle]
    fn func_800A663C(globalCtx: *mut GlobalContext, skin: *mut PSkinAwb,
                     skeletonHeader: *mut SkeletonHeader,
                     animationHeader: *mut AnimationHeader);
    #[no_mangle]
    fn func_800A6888(globalCtx: *mut GlobalContext, skin: *mut PSkinAwb);
    #[no_mangle]
    fn SkinMatrix_Vec3fMtxFMultXYZW(mf: *mut MtxF, src: *mut Vec3f,
                                    xyzDest: *mut Vec3f, wDest: *mut f32_0);
    #[no_mangle]
    fn func_800AA000(_: f32_0, _: u8_0, _: u8_0, _: u8_0);
    #[no_mangle]
    fn Graph_OpenDisps(dispRefs: *mut *mut Gfx, gfxCtx: *mut GraphicsContext,
                       file: *const libc::c_char, line: s32);
    #[no_mangle]
    fn Graph_CloseDisps(dispRefs: *mut *mut Gfx, gfxCtx: *mut GraphicsContext,
                        file: *const libc::c_char, line: s32);
    #[no_mangle]
    fn Math3D_Vec3fDistSq(a: *mut Vec3f, b: *mut Vec3f) -> f32_0;
    #[no_mangle]
    fn Math3D_Vec3f_DistXYZ(a: *mut Vec3f, b: *mut Vec3f) -> f32_0;
    #[no_mangle]
    fn Math3D_RotateXZPlane(pointOnPlane: *mut Vec3f, angle: s16,
                            a: *mut f32_0, c: *mut f32_0, d: *mut f32_0);
    #[no_mangle]
    fn Math3D_DistPlaneToPos(nx: f32_0, ny: f32_0, nz: f32_0,
                             originDist: f32_0, p: *mut Vec3f) -> f32_0;
    #[no_mangle]
    fn Math3D_PointDistToLine2D(x0: f32_0, y0: f32_0, x1: f32_0, y1: f32_0,
                                x2: f32_0, y2: f32_0, lineLenSq: *mut f32_0)
     -> s32;
    #[no_mangle]
    fn func_800F5A58(_: u8_0) -> s32;
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
    static mut D_801333D4: Vec3f;
    #[no_mangle]
    static mut gEponaGallopingAnim: AnimationHeader;
    #[no_mangle]
    static mut gEponaJumpingAnim: AnimationHeader;
    #[no_mangle]
    static mut gEponaJumpingHighAnim: AnimationHeader;
    #[no_mangle]
    static mut gEponaTrottingAnim: AnimationHeader;
    #[no_mangle]
    static mut gEponaRearingAnim: AnimationHeader;
    #[no_mangle]
    static mut gEponaRefuseAnim: AnimationHeader;
    #[no_mangle]
    static mut gEponaWhinnyAnim: AnimationHeader;
    #[no_mangle]
    static mut gEponaIdleAnim: AnimationHeader;
    #[no_mangle]
    static mut gEponaWalkingAnim: AnimationHeader;
    #[no_mangle]
    static mut gEponaSkel: SkeletonHeader;
    #[no_mangle]
    static mut gEponaEyeOpenTex: [u64_0; 0];
    #[no_mangle]
    static mut gEponaEyeHalfTex: [u64_0; 0];
    #[no_mangle]
    static mut gEponaEyeClosedTex: [u64_0; 0];
    #[no_mangle]
    static mut gHorseIngoSkel: SkeletonHeader;
    #[no_mangle]
    static mut gHorseIngoGallopingAnim: AnimationHeader;
    #[no_mangle]
    static mut gHorseIngoJumpingAnim: AnimationHeader;
    #[no_mangle]
    static mut gHorseIngoJumpingHighAnim: AnimationHeader;
    #[no_mangle]
    static mut gHorseIngoGerudoSaddleDL: [Gfx; 0];
    #[no_mangle]
    static mut gHorseIngoTrottingAnim: AnimationHeader;
    #[no_mangle]
    static mut gHorseIngoRearingAnim: AnimationHeader;
    #[no_mangle]
    static mut gHorseIngoRefuseAnim: AnimationHeader;
    #[no_mangle]
    static mut gHorseIngoIdleAnim: AnimationHeader;
    #[no_mangle]
    static mut gHorseIngoWhinnyAnim: AnimationHeader;
    #[no_mangle]
    static mut gHorseIngoWalkingAnim: AnimationHeader;
    #[no_mangle]
    static mut gGerudoValleyBridgeJumpFortressToFieldCs: [CutsceneData; 0];
    #[no_mangle]
    static mut gGerudoValleyBridgeJumpFieldFortressCs: [CutsceneData; 0];
}
pub type s8 = libc::c_schar;
pub type u8_0 = libc::c_uchar;
pub type s16 = libc::c_short;
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
pub type u64_0 = libc::c_ulonglong;
pub type f32_0 = libc::c_float;
pub type f64_0 = libc::c_double;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SkinAvb {
    pub unk_0: u8_0,
    pub buf: [*mut Vtx; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PSkinAwb {
    pub skeletonHeader: *mut SkeletonHeader,
    pub mtx: MtxF,
    pub avbCount: s32,
    pub avbTbl: *mut SkinAvb,
    pub skelAnime: SkelAnime,
}
pub type SkinCallback
    =
    Option<unsafe extern "C" fn(_: *mut Actor, _: *mut GlobalContext,
                                _: *mut PSkinAwb) -> ()>;
pub type SkinCallback2
    =
    Option<unsafe extern "C" fn(_: *mut Actor, _: *mut GlobalContext, _: s32,
                                _: *mut PSkinAwb) -> s32>;
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
pub union CutsceneData {
    pub i: s32,
    pub f: f32_0,
    pub s: [s16; 2],
    pub b: [s8; 4],
}
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
pub type EnHorseAction = libc::c_uint;
pub const ENHORSE_ACT_FLEE_PLAYER: EnHorseAction = 19;
pub const ENHORSE_ACT_HBA: EnHorseAction = 18;
pub const ENHORSE_ACT_CS_UPDATE: EnHorseAction = 17;
pub const ENHORSE_ACT_BRIDGE_JUMP: EnHorseAction = 16;
pub const ENHORSE_ACT_HIGH_JUMP: EnHorseAction = 15;
pub const ENHORSE_ACT_LOW_JUMP: EnHorseAction = 14;
pub const ENHORSE_ACT_REVERSE: EnHorseAction = 13;
pub const ENHORSE_ACT_STOPPING: EnHorseAction = 12;
pub const ENHORSE_ACT_MOUNTED_REARING: EnHorseAction = 11;
pub const ENHORSE_ACT_MOUNTED_GALLOP: EnHorseAction = 10;
pub const ENHORSE_ACT_MOUNTED_TROT: EnHorseAction = 9;
pub const ENHORSE_ACT_MOUNTED_WALK: EnHorseAction = 8;
pub const ENHORSE_ACT_MOUNTED_TURN: EnHorseAction = 7;
pub const ENHORSE_ACT_MOUNTED_IDLE_WHINNEYING: EnHorseAction = 6;
pub const ENHORSE_ACT_MOUNTED_IDLE: EnHorseAction = 5;
pub const ENHORSE_ACT_INGO_RACE: EnHorseAction = 4;
pub const ENHORSE_ACT_FOLLOW_PLAYER: EnHorseAction = 3;
pub const ENHORSE_ACT_IDLE: EnHorseAction = 2;
pub const ENHORSE_ACT_INACTIVE: EnHorseAction = 1;
pub const ENHORSE_ACT_FROZEN: EnHorseAction = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EnHorse {
    pub actor: Actor,
    pub action: EnHorseAction,
    pub noInputTimer: s32,
    pub noInputTimerMax: s32,
    pub type_0: s32,
    pub bankIndex: s8,
    pub skin: PSkinAwb,
    pub stateFlags: u32_0,
    pub lastPos: Vec3f,
    pub lastYaw: s16,
    pub curRaceWaypoint: s32,
    pub boostSpeed: s32,
    pub playerControlled: s32,
    pub animationIdx: s32,
    pub curFrame: f32_0,
    pub soundTimer: s32,
    pub unk_21C: Vec3f,
    pub unk_228: Vec3f,
    pub unk_234: s32,
    pub numBoosts: u8_0,
    pub boostRegenTime: s32,
    pub boostTimer: s32,
    pub postDrawFunc: EnHorsePostdrawFunc,
    pub yFront: f32_0,
    pub yBack: f32_0,
    pub followTimer: s16,
    pub unk_252: s16,
    pub prevAction: EnHorseAction,
    pub riderPos: Vec3f,
    pub curStick: Vec2f,
    pub lastStick: Vec2f,
    pub jumpStartY: f32_0,
    pub cyl1: ColliderCylinder,
    pub cyl2: ColliderCylinder,
    pub jntSph: ColliderJntSph,
    pub jntSphList: ColliderJntSphElement,
    pub playerDir: u32_0,
    pub unk_374: s16,
    pub angleToPlayer: s16,
    pub followPlayerTurnSpeed: s16,
    pub blinkTimer: u8_0,
    pub waitTimer: s16,
    pub unk_37E: s16,
    pub cutsceneAction: s32,
    pub cutsceneFlags: u16_0,
    pub inRace: s32,
    pub rider: *mut Actor,
    pub unk_390: u32_0,
    pub ingoRaceFlags: u16_0,
    pub ingoHorseMaxSpeed: f32_0,
    pub unk_39C: s32,
    pub hbaStarted: s32,
    pub hbaFlags: s32,
    pub hbaTimer: s32,
    pub bridgeJumpIdx: u8_0,
    pub bridgeJumpStart: Vec3f,
    pub bridgeJumpTimer: s32,
    pub bridgeJumpYVel: f32_0,
    pub bridgeJumpRelAngle: s16,
    pub unk_3C6: s16,
    pub dustFlags: u16_0,
    pub frontRightHoof: Vec3f,
    pub frontLeftHoof: Vec3f,
    pub backRightHoof: Vec3f,
    pub backLeftHoof: Vec3f,
}
pub type EnHorsePostdrawFunc
    =
    Option<unsafe extern "C" fn(_: *mut EnHorse, _: *mut GlobalContext)
               -> ()>;
pub type C2RustUnnamed_24 = libc::c_uint;
pub const PLAYER_DIR_SIDE_L: C2RustUnnamed_24 = 5;
pub const PLAYER_DIR_SIDE_R: C2RustUnnamed_24 = 4;
pub const PLAYER_DIR_BACK_L: C2RustUnnamed_24 = 3;
pub const PLAYER_DIR_BACK_R: C2RustUnnamed_24 = 2;
pub const PLAYER_DIR_FRONT_L: C2RustUnnamed_24 = 1;
pub const PLAYER_DIR_FRONT_R: C2RustUnnamed_24 = 0;
pub type C2RustUnnamed_25 = libc::c_uint;
pub const ENHORSE_ANIM_HIGH_JUMP: C2RustUnnamed_25 = 8;
pub const ENHORSE_ANIM_LOW_JUMP: C2RustUnnamed_25 = 7;
pub const ENHORSE_ANIM_GALLOP: C2RustUnnamed_25 = 6;
pub const ENHORSE_ANIM_TROT: C2RustUnnamed_25 = 5;
pub const ENHORSE_ANIM_WALK: C2RustUnnamed_25 = 4;
pub const ENHORSE_ANIM_REARING: C2RustUnnamed_25 = 3;
pub const ENHORSE_ANIM_STOPPING: C2RustUnnamed_25 = 2;
pub const ENHORSE_ANIM_WHINNEY: C2RustUnnamed_25 = 1;
pub const ENHORSE_ANIM_IDLE: C2RustUnnamed_25 = 0;
pub type C2RustUnnamed_26 = libc::c_uint;
pub const HORSE_HNI: C2RustUnnamed_26 = 1;
pub const HORSE_EPONA: C2RustUnnamed_26 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EnIn {
    pub actor: Actor,
    pub skelAnime: SkelAnime,
    pub actionFunc: EnInActionFunc,
    pub collider: ColliderCylinder,
    pub unk_1E0: f32_0,
    pub ingoObjBankIndex: s8,
    pub animationIdx: s16,
    pub unk_1E8: s16,
    pub blinkTimer: s16,
    pub unk_1EC: s16,
    pub eyeIndex: s16,
    pub camId: s16,
    pub activeCamId: s16,
    pub unk_1F4: [libc::c_char; 4],
    pub unk_1F8: s16,
    pub unk_1FA: s16,
    pub unk_1FC: s16,
    pub jointTable: [Vec3s; 20],
    pub morphTable: [Vec3s; 20],
    pub unk_2F0: f32_0,
    pub unk_2F4: f32_0,
    pub unk_2F8: f32_0,
    pub unk_2FC: f32_0,
    pub unk_300: f32_0,
    pub unk_304: f32_0,
    pub unk_308: struct_80034A14_arg1,
    pub unk_330: [Vec3s; 20],
}
pub type EnInActionFunc
    =
    Option<unsafe extern "C" fn(_: *mut EnIn, _: *mut GlobalContext) -> ()>;
pub type EnHorseCsFunc
    =
    Option<unsafe extern "C" fn(_: *mut EnHorse, _: *mut GlobalContext,
                                _: *mut CsCmdActorAction) -> ()>;
pub type EnHorseActionFunc
    =
    Option<unsafe extern "C" fn(_: *mut EnHorse, _: *mut GlobalContext)
               -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BridgeJumpPoint {
    pub zMin: s16,
    pub zMax: s16,
    pub xMin: s16,
    pub xMax: s16,
    pub xOffset: s16,
    pub angle: s16,
    pub angleRange: s16,
    pub pos: Vec3s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RaceInfo {
    pub numWaypoints: s32,
    pub waypoints: *mut RaceWaypoint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RaceWaypoint {
    pub x: s16,
    pub y: s16,
    pub z: s16,
    pub speed: s16,
    pub angle: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CsActionEntry {
    pub csAction: s32,
    pub csFuncIdx: s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EnHorseSpawnpoint {
    pub scene: s16,
    pub pos: Vec3s,
    pub angle: s16,
}
static mut sEponaAnimHeaders: [*mut AnimationHeader; 9] =
    unsafe {
        [&gEponaIdleAnim as *const AnimationHeader as *mut AnimationHeader,
         &gEponaWhinnyAnim as *const AnimationHeader as *mut AnimationHeader,
         &gEponaRefuseAnim as *const AnimationHeader as *mut AnimationHeader,
         &gEponaRearingAnim as *const AnimationHeader as *mut AnimationHeader,
         &gEponaWalkingAnim as *const AnimationHeader as *mut AnimationHeader,
         &gEponaTrottingAnim as *const AnimationHeader as
             *mut AnimationHeader,
         &gEponaGallopingAnim as *const AnimationHeader as
             *mut AnimationHeader,
         &gEponaJumpingAnim as *const AnimationHeader as *mut AnimationHeader,
         &gEponaJumpingHighAnim as *const AnimationHeader as
             *mut AnimationHeader]
    };
static mut sHniAnimHeaders: [*mut AnimationHeader; 9] =
    unsafe {
        [&gHorseIngoIdleAnim as *const AnimationHeader as
             *mut AnimationHeader,
         &gHorseIngoWhinnyAnim as *const AnimationHeader as
             *mut AnimationHeader,
         &gHorseIngoRefuseAnim as *const AnimationHeader as
             *mut AnimationHeader,
         &gHorseIngoRearingAnim as *const AnimationHeader as
             *mut AnimationHeader,
         &gHorseIngoWalkingAnim as *const AnimationHeader as
             *mut AnimationHeader,
         &gHorseIngoTrottingAnim as *const AnimationHeader as
             *mut AnimationHeader,
         &gHorseIngoGallopingAnim as *const AnimationHeader as
             *mut AnimationHeader,
         &gHorseIngoJumpingAnim as *const AnimationHeader as
             *mut AnimationHeader,
         &gHorseIngoJumpingHighAnim as *const AnimationHeader as
             *mut AnimationHeader]
    };
static mut sAnimationHeaders: [*mut *mut AnimationHeader; 2] =
    unsafe {
        [sEponaAnimHeaders.as_ptr() as *mut _,
         sHniAnimHeaders.as_ptr() as *mut _]
    };
static mut sPlaybackSpeeds: [f32_0; 9] =
    [2.0f32 / 3.0f32, 2.0f32 / 3.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32, 1.0f32,
     2.0f32 / 3.0f32, 2.0f32 / 3.0f32];
static mut sSkeletonHeaders: [*mut SkeletonHeader; 2] =
    unsafe {
        [&gEponaSkel as *const SkeletonHeader as *mut SkeletonHeader,
         &gHorseIngoSkel as *const SkeletonHeader as *mut SkeletonHeader]
    };
#[no_mangle]
pub static mut En_Horse_InitVars: ActorInit =
    unsafe {
        {
            let mut init =
                ActorInit{id: ACTOR_EN_HORSE as libc::c_int as s16,
                          category: ACTORCAT_BG as libc::c_int as u8_0,
                          flags:
                              ((1 as libc::c_int) << 4 as libc::c_int) as
                                  u32_0,
                          objectId: OBJECT_HORSE as libc::c_int as s16,
                          instanceSize:
                              ::std::mem::size_of::<EnHorse>() as
                                  libc::c_ulong,
                          init:
                              ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                      *mut Actor,
                                                                                  _:
                                                                                      *mut GlobalContext)
                                                                 -> ()>,
                                                      ActorFunc>(Some(EnHorse_Init
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
                                                      ActorFunc>(Some(EnHorse_Destroy
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
                                                      ActorFunc>(Some(EnHorse_Update
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
                                                      ActorFunc>(Some(EnHorse_Draw
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
static mut sCylinderInit1: ColliderCylinderInit =
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
                                                                   3 as
                                                                       libc::c_int)
                                                                  as u8_0,
                                                          acFlags:
                                                              0 as libc::c_int
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
                                                                       libc::c_int
                                                                   |
                                                                   (1 as
                                                                        libc::c_int)
                                                                       <<
                                                                       1 as
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
                                                                                            0x400
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
static mut sCylinderInit2: ColliderCylinderInit =
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
                                                              0 as libc::c_int
                                                                  as u8_0,
                                                          acFlags:
                                                              0 as libc::c_int
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
                                                                       libc::c_int
                                                                   |
                                                                   (1 as
                                                                        libc::c_int)
                                                                       <<
                                                                       1 as
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
                                                                  0 as
                                                                      libc::c_int
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
static mut sJntSphItemsInit: [ColliderJntSphElementInit; 1] =
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
                                                                                                     0x1f824
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
                                                                                 libc::c_int
                                                                             |
                                                                             (1
                                                                                  as
                                                                                  libc::c_int)
                                                                                 <<
                                                                                 3
                                                                                     as
                                                                                     libc::c_int
                                                                             |
                                                                             (1
                                                                                  as
                                                                                  libc::c_int)
                                                                                 <<
                                                                                 4
                                                                                     as
                                                                                     libc::c_int
                                                                             |
                                                                             (1
                                                                                  as
                                                                                  libc::c_int)
                                                                                 <<
                                                                                 5
                                                                                     as
                                                                                     libc::c_int
                                                                             |
                                                                             (1
                                                                                  as
                                                                                  libc::c_int)
                                                                                 <<
                                                                                 6
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
                                                                          (1
                                                                               as
                                                                               libc::c_int)
                                                                              <<
                                                                              4
                                                                                  as
                                                                                  libc::c_int
                                                                          |
                                                                          (1
                                                                               as
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
                                                                         libc::c_int
                                                                     |
                                                                     (1 as
                                                                          libc::c_int)
                                                                         <<
                                                                         1 as
                                                                             libc::c_int)
                                                                    as u8_0,
                                                            shape:
                                                                COLSHAPE_JNTSPH
                                                                    as
                                                                    libc::c_int
                                                                    as u8_0,};
                                           init
                                       },
                                   count: 1 as libc::c_int,
                                   elements:
                                       sJntSphItemsInit.as_ptr() as *mut _,};
            init
        }
    };
static mut D_80A65F38: CollisionCheckInfoInit =
    {
        let mut init =
            CollisionCheckInfoInit{health: 10 as libc::c_int as u8_0,
                                   cylRadius: 35 as libc::c_int as s16,
                                   cylHeight: 100 as libc::c_int as s16,
                                   mass: 0xfe as libc::c_int as u8_0,};
        init
    };
static mut sHorseSpawns: [EnHorseSpawnpoint; 169] =
    [{
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x: 16 as libc::c_int as s16,
                                                 y: 0 as libc::c_int as s16,
                                                 z:
                                                     1341 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(1297 as libc::c_int) as
                                                         s16,
                                                 y: 0 as libc::c_int as s16,
                                                 z:
                                                     1459 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(5416 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(300 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     1296 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(4667 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(300 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     3620 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(3837 as libc::c_int) as
                                                         s16,
                                                 y: 81 as libc::c_int as s16,
                                                 z:
                                                     5537 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(5093 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(226 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     6661 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(6588 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(79 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     5053 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(7072 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(500 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     7538 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(6139 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(500 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     8910 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(8497 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(300 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     7802 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(5481 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(499 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     12127 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(4808 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(700 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     13583 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(3416 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(490 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     12092 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(2915 as libc::c_int) as
                                                         s16,
                                                 y: 100 as libc::c_int as s16,
                                                 z:
                                                     8339 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(2277 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(500 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     13247 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(1026 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(500 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     12101 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     1427 as libc::c_int as
                                                         s16,
                                                 y:
                                                     -(500 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     13341 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(200 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(486 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     10205 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(1469 as libc::c_int) as
                                                         s16,
                                                 y: 100 as libc::c_int as s16,
                                                 z:
                                                     7496 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(995 as libc::c_int) as
                                                         s16,
                                                 y: 168 as libc::c_int as s16,
                                                 z:
                                                     5652 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     1938 as libc::c_int as
                                                         s16,
                                                 y: 89 as libc::c_int as s16,
                                                 z:
                                                     6232 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     1387 as libc::c_int as
                                                         s16,
                                                 y:
                                                     -(105 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     9206 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     1571 as libc::c_int as
                                                         s16,
                                                 y:
                                                     -(223 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     7701 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     3893 as libc::c_int as
                                                         s16,
                                                 y:
                                                     -(121 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     7068 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     3179 as libc::c_int as
                                                         s16,
                                                 y: 373 as libc::c_int as s16,
                                                 z:
                                                     5221 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     4678 as libc::c_int as
                                                         s16,
                                                 y:
                                                     -(20 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     3869 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     3460 as libc::c_int as
                                                         s16,
                                                 y: 246 as libc::c_int as s16,
                                                 z:
                                                     4207 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     3686 as libc::c_int as
                                                         s16,
                                                 y: 128 as libc::c_int as s16,
                                                 z:
                                                     2366 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     1791 as libc::c_int as
                                                         s16,
                                                 y: 18 as libc::c_int as s16,
                                                 z:
                                                     152 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     3667 as libc::c_int as
                                                         s16,
                                                 y:
                                                     -(16 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     1399 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     1827 as libc::c_int as
                                                         s16,
                                                 y:
                                                     -(15 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     983 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     1574 as libc::c_int as
                                                         s16,
                                                 y: 399 as libc::c_int as s16,
                                                 z:
                                                     4318 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x: 716 as libc::c_int as s16,
                                                 y: 95 as libc::c_int as s16,
                                                 z:
                                                     3391 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(1189 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(41 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     4739 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(1976 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(171 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     4172 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     1314 as libc::c_int as
                                                         s16,
                                                 y: 391 as libc::c_int as s16,
                                                 z:
                                                     5665 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x: 112 as libc::c_int as s16,
                                                 y: 0 as libc::c_int as s16,
                                                 z:
                                                     1959 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(3011 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(111 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     9397 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(5674 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(270 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     8585 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(8861 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(300 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     7836 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(6056 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(500 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     7810 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(7306 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(500 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     5994 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(7305 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(500 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     7605 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(7439 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(300 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     7600 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(7464 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(300 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     6268 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(8080 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(300 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     7553 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(8091 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(300 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     7349 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(8785 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(300 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     7383 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(8745 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(300 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     7508 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(8777 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(300 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     7788 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(8048 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(299 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     7738 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(7341 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(184 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     7730 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(6410 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(288 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     7824 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(6326 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(290 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     8205 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(6546 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(292 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     8400 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(7533 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(180 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     8459 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(8024 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(295 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     7903 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(8078 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(308 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     7994 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(9425 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(287 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     7696 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(9322 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(292 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     7577 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(9602 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(199 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     7160 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(9307 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(300 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     7758 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(9230 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(300 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     7642 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(7556 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(499 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     8695 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(6438 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(500 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     8606 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(6078 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(500 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     8258 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(6233 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(500 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     7613 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(5035 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(205 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     7814 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(5971 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(500 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     8501 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(5724 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(498 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     10123 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(5094 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(392 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     11106 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(5105 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(393 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     11312 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(4477 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(314 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     11132 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(3867 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(380 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     11419 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(2952 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(500 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     11944 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(2871 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(488 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     11743 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(3829 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(372 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     11327 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(4439 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(293 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     10989 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(5014 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(381 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     11086 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(5113 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(188 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     10968 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(5269 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(188 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     11156 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(5596 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(178 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     9972 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(5801 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(288 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     8518 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(4910 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(178 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     7931 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(3586 as libc::c_int) as
                                                         s16,
                                                 y: 73 as libc::c_int as s16,
                                                 z:
                                                     8140 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(4487 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(106 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     9362 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(4339 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(112 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     6412 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(3417 as libc::c_int) as
                                                         s16,
                                                 y: 105 as libc::c_int as s16,
                                                 z:
                                                     8194 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(4505 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(20 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     6608 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(5038 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(199 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     6603 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(4481 as libc::c_int) as
                                                         s16,
                                                 y: 1 as libc::c_int as s16,
                                                 z:
                                                     6448 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(5032 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(168 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     6418 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(5256 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(700 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     14329 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(5749 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(820 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     15380 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(3122 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(700 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     13608 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(3758 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(525 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     13228 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(3670 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(500 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     13123 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(2924 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(500 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     13526 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     1389 as libc::c_int as
                                                         s16,
                                                 y:
                                                     -(115 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     9370 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x: 548 as libc::c_int as s16,
                                                 y:
                                                     -(116 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     8889 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(106 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(107 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     8530 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(1608 as libc::c_int) as
                                                         s16,
                                                 y: 85 as libc::c_int as s16,
                                                 z:
                                                     7646 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(5300 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(700 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     13772 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(5114 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(700 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     13400 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(4561 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(700 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     13700 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(4762 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(700 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     14084 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(2954 as libc::c_int) as
                                                         s16,
                                                 y: 100 as libc::c_int as s16,
                                                 z:
                                                     8216 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     1460 as libc::c_int as
                                                         s16,
                                                 y:
                                                     -(104 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     9246 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x: 629 as libc::c_int as s16,
                                                 y:
                                                     -(105 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     8791 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(10 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(90 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     8419 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(1462 as libc::c_int) as
                                                         s16,
                                                 y: 100 as libc::c_int as s16,
                                                 z:
                                                     7504 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(3018 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(500 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     12493 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(2994 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(311 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     10331 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(4006 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(700 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     14152 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(4341 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(500 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     12743 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(5879 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(497 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     6799 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x: 22 as libc::c_int as s16,
                                                 y:
                                                     -(473 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     10103 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(1389 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(192 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     8874 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(4 as libc::c_int) as
                                                         s16,
                                                 y: 92 as libc::c_int as s16,
                                                 z:
                                                     6866 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x: 483 as libc::c_int as s16,
                                                 y: 104 as libc::c_int as s16,
                                                 z:
                                                     6637 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     1580 as libc::c_int as
                                                         s16,
                                                 y: 183 as libc::c_int as s16,
                                                 z:
                                                     5987 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     1548 as libc::c_int as
                                                         s16,
                                                 y: 308 as libc::c_int as s16,
                                                 z:
                                                     5077 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     1511 as libc::c_int as
                                                         s16,
                                                 y: 399 as libc::c_int as s16,
                                                 z:
                                                     4267 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     1358 as libc::c_int as
                                                         s16,
                                                 y: 385 as libc::c_int as s16,
                                                 z:
                                                     4271 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     1379 as libc::c_int as
                                                         s16,
                                                 y: 395 as libc::c_int as s16,
                                                 z:
                                                     5063 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     1360 as libc::c_int as
                                                         s16,
                                                 y: 394 as libc::c_int as s16,
                                                 z:
                                                     5870 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x: 813 as libc::c_int as s16,
                                                 y: 283 as libc::c_int as s16,
                                                 z:
                                                     6194 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(57 as libc::c_int) as
                                                         s16,
                                                 y: 101 as libc::c_int as s16,
                                                 z:
                                                     6743 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x: 91 as libc::c_int as s16,
                                                 y: 325 as libc::c_int as s16,
                                                 z:
                                                     5143 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     1425 as libc::c_int as
                                                         s16,
                                                 y:
                                                     -(214 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     7659 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     3487 as libc::c_int as
                                                         s16,
                                                 y:
                                                     -(19 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     880 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     2933 as libc::c_int as
                                                         s16,
                                                 y: 152 as libc::c_int as s16,
                                                 z:
                                                     2094 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     2888 as libc::c_int as
                                                         s16,
                                                 y:
                                                     -(145 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     6862 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     1511 as libc::c_int as
                                                         s16,
                                                 y: 67 as libc::c_int as s16,
                                                 z:
                                                     6471 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     4051 as libc::c_int as
                                                         s16,
                                                 y:
                                                     -(47 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     1722 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(7335 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(500 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     8627 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(7728 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(462 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     8498 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(7791 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(446 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     8832 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(2915 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(435 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     11334 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT00 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x: 165 as libc::c_int as s16,
                                                 y:
                                                     -(278 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     3352 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT06 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(2109 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(882 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     1724 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT06 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(328 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(1238 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     2705 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT06 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(3092 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(1033 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     3527 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT09 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     2687 as libc::c_int as
                                                         s16,
                                                 y:
                                                     -(269 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     753 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT09 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     2066 as libc::c_int as
                                                         s16,
                                                 y:
                                                     -(132 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     317 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT09 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x: 523 as libc::c_int as s16,
                                                 y:
                                                     -(8 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     635 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT09 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x: 558 as libc::c_int as s16,
                                                 y: 36 as libc::c_int as s16,
                                                 z:
                                                     -(323 as libc::c_int) as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT09 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x: 615 as libc::c_int as s16,
                                                 y: 51 as libc::c_int as s16,
                                                 z:
                                                     -(839 as libc::c_int) as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT09 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(614 as libc::c_int) as
                                                         s16,
                                                 y: 32 as libc::c_int as s16,
                                                 z:
                                                     29 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT09 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(639 as libc::c_int) as
                                                         s16,
                                                 y:
                                                     -(3 as libc::c_int) as
                                                         s16,
                                                 z:
                                                     553 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT09 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(540 as libc::c_int) as
                                                         s16,
                                                 y: 10 as libc::c_int as s16,
                                                 z:
                                                     -(889 as libc::c_int) as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT09 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(1666 as libc::c_int) as
                                                         s16,
                                                 y: 58 as libc::c_int as s16,
                                                 z:
                                                     378 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT09 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(3044 as libc::c_int) as
                                                         s16,
                                                 y: 210 as libc::c_int as s16,
                                                 z:
                                                     -(648 as libc::c_int) as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT12 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(678 as libc::c_int) as
                                                         s16,
                                                 y: 21 as libc::c_int as s16,
                                                 z:
                                                     -(623 as libc::c_int) as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT12 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x: 149 as libc::c_int as s16,
                                                 y: 333 as libc::c_int as s16,
                                                 z:
                                                     -(2499 as libc::c_int) as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT12 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x: 499 as libc::c_int as s16,
                                                 y: 581 as libc::c_int as s16,
                                                 z:
                                                     -(547 as libc::c_int) as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT12 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     3187 as libc::c_int as
                                                         s16,
                                                 y:
                                                     1413 as libc::c_int as
                                                         s16,
                                                 z:
                                                     -(3775 as libc::c_int) as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT12 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     3198 as libc::c_int as
                                                         s16,
                                                 y:
                                                     1413 as libc::c_int as
                                                         s16,
                                                 z:
                                                     307 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT12 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     3380 as libc::c_int as
                                                         s16,
                                                 y:
                                                     1413 as libc::c_int as
                                                         s16,
                                                 z:
                                                     -(1200 as libc::c_int) as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT12 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(966 as libc::c_int) as
                                                         s16,
                                                 y: 1 as libc::c_int as s16,
                                                 z:
                                                     -(56 as libc::c_int) as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT12 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(966 as libc::c_int) as
                                                         s16,
                                                 y: 24 as libc::c_int as s16,
                                                 z:
                                                     -(761 as libc::c_int) as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT12 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(694 as libc::c_int) as
                                                         s16,
                                                 y: 174 as libc::c_int as s16,
                                                 z:
                                                     -(2820 as libc::c_int) as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT20 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     1039 as libc::c_int as
                                                         s16,
                                                 y: 0 as libc::c_int as s16,
                                                 z:
                                                     2051 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT20 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(1443 as libc::c_int) as
                                                         s16,
                                                 y: 0 as libc::c_int as s16,
                                                 z:
                                                     1429 as libc::c_int as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT20 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x: 856 as libc::c_int as s16,
                                                 y: 0 as libc::c_int as s16,
                                                 z:
                                                     -(918 as libc::c_int) as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT20 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x: 882 as libc::c_int as s16,
                                                 y: 0 as libc::c_int as s16,
                                                 z:
                                                     -(2256 as libc::c_int) as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT20 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(1003 as libc::c_int) as
                                                         s16,
                                                 y: 0 as libc::c_int as s16,
                                                 z:
                                                     -(755 as libc::c_int) as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT20 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x:
                                                     -(2254 as libc::c_int) as
                                                         s16,
                                                 y: 0 as libc::c_int as s16,
                                                 z:
                                                     -(629 as libc::c_int) as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnHorseSpawnpoint{scene: SCENE_SPOT20 as libc::c_int as s16,
                               pos:
                                   {
                                       let mut init =
                                           Vec3s{x: 907 as libc::c_int as s16,
                                                 y: 0 as libc::c_int as s16,
                                                 z:
                                                     -(2336 as libc::c_int) as
                                                         s16,};
                                       init
                                   },
                               angle: 0 as libc::c_int as s16,};
         init
     }];
static mut sBridgeJumps: [BridgeJumpPoint; 2] =
    [{
         let mut init =
             BridgeJumpPoint{zMin: -(195 as libc::c_int) as s16,
                             zMax: -(40 as libc::c_int) as s16,
                             xMin: 225 as libc::c_int as s16,
                             xMax: 120 as libc::c_int as s16,
                             xOffset: 360 as libc::c_int as s16,
                             angle: -(0x4000 as libc::c_int) as s16,
                             angleRange: 0x7d0 as libc::c_int as s16,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x:
                                                   -(270 as libc::c_int) as
                                                       s16,
                                               y: -(52 as libc::c_int) as s16,
                                               z:
                                                   -(117 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     },
     {
         let mut init =
             BridgeJumpPoint{zMin: -(195 as libc::c_int) as s16,
                             zMax: -(40 as libc::c_int) as s16,
                             xMin: -(240 as libc::c_int) as s16,
                             xMax: -(120 as libc::c_int) as s16,
                             xOffset: -(360 as libc::c_int) as s16,
                             angle: 0x4000 as libc::c_int as s16,
                             angleRange: 0x7d0 as libc::c_int as s16,
                             pos:
                                 {
                                     let mut init =
                                         Vec3s{x: 270 as libc::c_int as s16,
                                               y: -(52 as libc::c_int) as s16,
                                               z:
                                                   -(117 as libc::c_int) as
                                                       s16,};
                                     init
                                 },};
         init
     }];
static mut sIngoRaceWaypoints: [RaceWaypoint; 8] =
    [{
         let mut init =
             RaceWaypoint{x: 1056 as libc::c_int as s16,
                          y: 1 as libc::c_int as s16,
                          z: -(1540 as libc::c_int) as s16,
                          speed: 11 as libc::c_int as s16,
                          angle: 0x2a8d as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             RaceWaypoint{x: 1593 as libc::c_int as s16,
                          y: 1 as libc::c_int as s16,
                          z: -(985 as libc::c_int) as s16,
                          speed: 10 as libc::c_int as s16,
                          angle: 0xfc27 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             RaceWaypoint{x: 1645 as libc::c_int as s16,
                          y: 1 as libc::c_int as s16,
                          z: -(221 as libc::c_int) as s16,
                          speed: 11 as libc::c_int as s16,
                          angle: 0xe891 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             RaceWaypoint{x: 985 as libc::c_int as s16,
                          y: 1 as libc::c_int as s16,
                          z: 403 as libc::c_int as s16,
                          speed: 10 as libc::c_int as s16,
                          angle: 0xbb9c as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             RaceWaypoint{x: -(1023 as libc::c_int) as s16,
                          y: 1 as libc::c_int as s16,
                          z: 354 as libc::c_int as s16,
                          speed: 11 as libc::c_int as s16,
                          angle: 0xa37d as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             RaceWaypoint{x: -(1679 as libc::c_int) as s16,
                          y: 1 as libc::c_int as s16,
                          z: -(213 as libc::c_int) as s16,
                          speed: 10 as libc::c_int as s16,
                          angle: 0x889c as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             RaceWaypoint{x: -(1552 as libc::c_int) as s16,
                          y: 1 as libc::c_int as s16,
                          z: -(1008 as libc::c_int) as s16,
                          speed: 11 as libc::c_int as s16,
                          angle: 0x638d as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             RaceWaypoint{x: -(947 as libc::c_int) as s16,
                          y: -(1 as libc::c_int) as s16,
                          z: -(1604 as libc::c_int) as s16,
                          speed: 10 as libc::c_int as s16,
                          angle: 0x4002 as libc::c_int as s16,};
         init
     }];
static mut sIngoRace: RaceInfo =
    unsafe {
        {
            let mut init =
                RaceInfo{numWaypoints: 8 as libc::c_int,
                         waypoints: sIngoRaceWaypoints.as_ptr() as *mut _,};
            init
        }
    };
static mut sAnimSoundFrames: [s32; 2] = [0 as libc::c_int, 16 as libc::c_int];
// Initialized in run_static_initializers
static mut sInitChain: [InitChainEntry; 2] =
    [InitChainEntry{cont_type_0_offset_value: [0; 4],}; 2];
static mut sResetNoInput: [u8_0; 12] =
    [0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0];
static mut sIdleAnimIds: [s32; 6] =
    [1 as libc::c_int, 3 as libc::c_int, 0 as libc::c_int, 3 as libc::c_int,
     1 as libc::c_int, 0 as libc::c_int];
static mut sIngoAnimations: [s16; 10] =
    [7 as libc::c_int as s16, 6 as libc::c_int as s16,
     2 as libc::c_int as s16, 2 as libc::c_int as s16,
     1 as libc::c_int as s16, 1 as libc::c_int as s16,
     0 as libc::c_int as s16, 0 as libc::c_int as s16,
     0 as libc::c_int as s16, 0 as libc::c_int as s16];
static mut sCutsceneInitFuncs: [EnHorseCsFunc; 6] =
    unsafe {
        [None,
         Some(EnHorse_CsMoveInit as
                  unsafe extern "C" fn(_: *mut EnHorse, _: *mut GlobalContext,
                                       _: *mut CsCmdActorAction) -> ()),
         Some(EnHorse_CsJumpInit as
                  unsafe extern "C" fn(_: *mut EnHorse, _: *mut GlobalContext,
                                       _: *mut CsCmdActorAction) -> ()),
         Some(EnHorse_CsRearingInit as
                  unsafe extern "C" fn(_: *mut EnHorse, _: *mut GlobalContext,
                                       _: *mut CsCmdActorAction) -> ()),
         Some(EnHorse_WarpMoveInit as
                  unsafe extern "C" fn(_: *mut EnHorse, _: *mut GlobalContext,
                                       _: *mut CsCmdActorAction) -> ()),
         Some(EnHorse_CsWarpRearingInit as
                  unsafe extern "C" fn(_: *mut EnHorse, _: *mut GlobalContext,
                                       _: *mut CsCmdActorAction) -> ())]
    };
static mut sCutsceneActionFuncs: [EnHorseCsFunc; 6] =
    unsafe {
        [None,
         Some(EnHorse_CsMoveToPoint as
                  unsafe extern "C" fn(_: *mut EnHorse, _: *mut GlobalContext,
                                       _: *mut CsCmdActorAction) -> ()),
         Some(EnHorse_CsJump as
                  unsafe extern "C" fn(_: *mut EnHorse, _: *mut GlobalContext,
                                       _: *mut CsCmdActorAction) -> ()),
         Some(EnHorse_CsRearing as
                  unsafe extern "C" fn(_: *mut EnHorse, _: *mut GlobalContext,
                                       _: *mut CsCmdActorAction) -> ()),
         Some(EnHorse_CsWarpMoveToPoint as
                  unsafe extern "C" fn(_: *mut EnHorse, _: *mut GlobalContext,
                                       _: *mut CsCmdActorAction) -> ()),
         Some(EnHorse_CsWarpRearing as
                  unsafe extern "C" fn(_: *mut EnHorse, _: *mut GlobalContext,
                                       _: *mut CsCmdActorAction) -> ())]
    };
static mut sCsActionTable: [CsActionEntry; 5] =
    [{
         let mut init =
             CsActionEntry{csAction: 36 as libc::c_int,
                           csFuncIdx: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             CsActionEntry{csAction: 37 as libc::c_int,
                           csFuncIdx: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             CsActionEntry{csAction: 38 as libc::c_int,
                           csFuncIdx: 3 as libc::c_int,};
         init
     },
     {
         let mut init =
             CsActionEntry{csAction: 64 as libc::c_int,
                           csFuncIdx: 4 as libc::c_int,};
         init
     },
     {
         let mut init =
             CsActionEntry{csAction: 65 as libc::c_int,
                           csFuncIdx: 5 as libc::c_int,};
         init
     }];
static mut sHbaWaypoints: [RaceWaypoint; 5] =
    [{
         let mut init =
             RaceWaypoint{x: 3600 as libc::c_int as s16,
                          y: 1413 as libc::c_int as s16,
                          z: -(5055 as libc::c_int) as s16,
                          speed: 11 as libc::c_int as s16,
                          angle: 0x8001 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             RaceWaypoint{x: 3360 as libc::c_int as s16,
                          y: 1413 as libc::c_int as s16,
                          z: -(5220 as libc::c_int) as s16,
                          speed: 5 as libc::c_int as s16,
                          angle: 0xc000 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             RaceWaypoint{x: 3100 as libc::c_int as s16,
                          y: 1413 as libc::c_int as s16,
                          z: -(4900 as libc::c_int) as s16,
                          speed: 5 as libc::c_int as s16,
                          angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             RaceWaypoint{x: 3600 as libc::c_int as s16,
                          y: 1413 as libc::c_int as s16,
                          z: -(4100 as libc::c_int) as s16,
                          speed: 11 as libc::c_int as s16,
                          angle: 0 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             RaceWaypoint{x: 3600 as libc::c_int as s16,
                          y: 1413 as libc::c_int as s16,
                          z: 360 as libc::c_int as s16,
                          speed: 11 as libc::c_int as s16,
                          angle: 0 as libc::c_int as s16,};
         init
     }];
static mut sHbaInfo: RaceInfo =
    unsafe {
        {
            let mut init =
                RaceInfo{numWaypoints: 5 as libc::c_int,
                         waypoints: sHbaWaypoints.as_ptr() as *mut _,};
            init
        }
    };
static mut sActionFuncs: [EnHorseActionFunc; 20] =
    unsafe {
        [Some(EnHorse_Frozen as
                  unsafe extern "C" fn(_: *mut EnHorse, _: *mut GlobalContext)
                      -> ()),
         Some(EnHorse_Inactive as
                  unsafe extern "C" fn(_: *mut EnHorse, _: *mut GlobalContext)
                      -> ()),
         Some(EnHorse_Idle as
                  unsafe extern "C" fn(_: *mut EnHorse, _: *mut GlobalContext)
                      -> ()),
         Some(EnHorse_FollowPlayer as
                  unsafe extern "C" fn(_: *mut EnHorse, _: *mut GlobalContext)
                      -> ()),
         Some(EnHorse_UpdateIngoRace as
                  unsafe extern "C" fn(_: *mut EnHorse, _: *mut GlobalContext)
                      -> ()),
         Some(EnHorse_MountedIdle as
                  unsafe extern "C" fn(_: *mut EnHorse, _: *mut GlobalContext)
                      -> ()),
         Some(EnHorse_MountedIdleWhinneying as
                  unsafe extern "C" fn(_: *mut EnHorse, _: *mut GlobalContext)
                      -> ()),
         Some(EnHorse_MountedTurn as
                  unsafe extern "C" fn(_: *mut EnHorse, _: *mut GlobalContext)
                      -> ()),
         Some(EnHorse_MountedWalk as
                  unsafe extern "C" fn(_: *mut EnHorse, _: *mut GlobalContext)
                      -> ()),
         Some(EnHorse_MountedTrot as
                  unsafe extern "C" fn(_: *mut EnHorse, _: *mut GlobalContext)
                      -> ()),
         Some(EnHorse_MountedGallop as
                  unsafe extern "C" fn(_: *mut EnHorse, _: *mut GlobalContext)
                      -> ()),
         Some(EnHorse_MountedRearing as
                  unsafe extern "C" fn(_: *mut EnHorse, _: *mut GlobalContext)
                      -> ()),
         Some(EnHorse_Stopping as
                  unsafe extern "C" fn(_: *mut EnHorse, _: *mut GlobalContext)
                      -> ()),
         Some(EnHorse_Reverse as
                  unsafe extern "C" fn(_: *mut EnHorse, _: *mut GlobalContext)
                      -> ()),
         Some(EnHorse_LowJump as
                  unsafe extern "C" fn(_: *mut EnHorse, _: *mut GlobalContext)
                      -> ()),
         Some(EnHorse_HighJump as
                  unsafe extern "C" fn(_: *mut EnHorse, _: *mut GlobalContext)
                      -> ()),
         Some(EnHorse_BridgeJump as
                  unsafe extern "C" fn(_: *mut EnHorse, _: *mut GlobalContext)
                      -> ()),
         Some(EnHorse_CutsceneUpdate as
                  unsafe extern "C" fn(_: *mut EnHorse, _: *mut GlobalContext)
                      -> ()),
         Some(EnHorse_UpdateHorsebackArchery as
                  unsafe extern "C" fn(_: *mut EnHorse, _: *mut GlobalContext)
                      -> ()),
         Some(EnHorse_FleePlayer as
                  unsafe extern "C" fn(_: *mut EnHorse, _: *mut GlobalContext)
                      -> ())]
    };
#[no_mangle]
pub unsafe extern "C" fn EnHorse_BgCheckBridgeJumpPoint(mut this:
                                                            *mut EnHorse,
                                                        mut globalCtx:
                                                            *mut GlobalContext)
 -> s32 {
    let mut xMin: f32_0 = 0.;
    let mut xMax: f32_0 = 0.;
    let mut i: s32 = 0;
    if (*globalCtx).sceneNum as libc::c_int != SCENE_SPOT09 as libc::c_int {
        return 0 as libc::c_int
    }
    if (*this).actor.speedXZ < 12.8f32 { return 0 as libc::c_int }
    if gSaveContext.eventChkInf[9 as libc::c_int as usize] as libc::c_int &
           0xf as libc::c_int == 0xf as libc::c_int {
        return 0 as libc::c_int
    }
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        xMin = sBridgeJumps[i as usize].xMin as f32_0;
        xMax =
            xMin +
                sBridgeJumps[i as usize].xMax as libc::c_int as libc::c_float
                +
                sBridgeJumps[i as usize].xOffset as libc::c_int as
                    libc::c_float;
        if xMax < xMin {
            let mut temp: f32_0 = xMin;
            xMin = xMax;
            xMax = temp
        }
        if (sBridgeJumps[i as usize].zMin as libc::c_int as libc::c_float) <
               (*this).actor.world.pos.z &&
               (*this).actor.world.pos.z <
                   sBridgeJumps[i as usize].zMax as libc::c_int as
                       libc::c_float {
            if xMin < (*this).actor.world.pos.x &&
                   (*this).actor.world.pos.x < xMax {
                if (sBridgeJumps[i as usize].angle as libc::c_int -
                        sBridgeJumps[i as usize].angleRange as libc::c_int) <
                       (*this).actor.world.rot.y as libc::c_int &&
                       ((*this).actor.world.rot.y as libc::c_int) <
                           sBridgeJumps[i as usize].angle as libc::c_int +
                               sBridgeJumps[i as usize].angleRange as
                                   libc::c_int {
                    return 1 as libc::c_int
                }
            }
        }
        i += 1
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_CheckBridgeJumps(mut this: *mut EnHorse,
                                                  mut globalCtx:
                                                      *mut GlobalContext)
 -> s32 {
    let mut xMin: f32_0 = 0.;
    let mut xMax: f32_0 = 0.;
    let mut i: s32 = 0;
    if (*this).actor.speedXZ < 12.8f32 { return 0 as libc::c_int }
    i = 0 as libc::c_int;
    while i != 2 as libc::c_int {
        xMin = sBridgeJumps[i as usize].xMin as f32_0;
        xMax =
            sBridgeJumps[i as usize].xMax as libc::c_int as libc::c_float +
                xMin;
        if xMax < xMin {
            let mut temp: f32_0 = xMin;
            xMin = xMax;
            xMax = temp
        }
        if (sBridgeJumps[i as usize].zMin as libc::c_int as libc::c_float) <
               (*this).actor.world.pos.z &&
               (*this).actor.world.pos.z <
                   sBridgeJumps[i as usize].zMax as libc::c_int as
                       libc::c_float {
            if xMin < (*this).actor.world.pos.x &&
                   (*this).actor.world.pos.x < xMax {
                if (sBridgeJumps[i as usize].angle as libc::c_int -
                        sBridgeJumps[i as usize].angleRange as libc::c_int) <
                       (*this).actor.world.rot.y as libc::c_int &&
                       ((*this).actor.world.rot.y as libc::c_int) <
                           sBridgeJumps[i as usize].angle as libc::c_int +
                               sBridgeJumps[i as usize].angleRange as
                                   libc::c_int {
                    (*this).bridgeJumpIdx = i as u8_0;
                    EnHorse_StartBridgeJump(this, globalCtx);
                    return 1 as libc::c_int
                }
            }
        }
        i += 1
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_RaceWaypointPos(mut waypoints:
                                                     *mut RaceWaypoint,
                                                 mut idx: s32,
                                                 mut pos: *mut Vec3f) {
    (*pos).x = (*waypoints.offset(idx as isize)).x as f32_0;
    (*pos).y = (*waypoints.offset(idx as isize)).y as f32_0;
    (*pos).z = (*waypoints.offset(idx as isize)).z as f32_0;
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_RotateToPoint(mut this: *mut EnHorse,
                                               mut globalCtx:
                                                   *mut GlobalContext,
                                               mut pos: *mut Vec3f,
                                               mut turnAmount: s16) {
    func_8006DD9C(&mut (*this).actor, pos, turnAmount);
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_UpdateIngoRaceInfo(mut this: *mut EnHorse,
                                                    mut globalCtx:
                                                        *mut GlobalContext,
                                                    mut raceInfo:
                                                        *mut RaceInfo) {
    let mut curWaypointPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut prevWaypointPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut playerDist: f32_0 = 0.;
    let mut sp50: f32_0 = 0.;
    let mut relPlayerYaw: s16 = 0;
    let mut px: f32_0 = 0.;
    let mut pz: f32_0 = 0.;
    let mut d: f32_0 = 0.;
    let mut dist: f32_0 = 0.;
    let mut prevWaypoint: s32 = 0;
    EnHorse_RaceWaypointPos((*raceInfo).waypoints, (*this).curRaceWaypoint,
                            &mut curWaypointPos);
    Math3D_RotateXZPlane(&mut curWaypointPos,
                         (*(*raceInfo).waypoints.offset((*this).curRaceWaypoint
                                                            as isize)).angle,
                         &mut px, &mut pz, &mut d);
    if (*this).actor.world.pos.x * px + pz * (*this).actor.world.pos.z + d >
           0.0f32 {
        (*this).curRaceWaypoint += 1;
        if (*this).curRaceWaypoint >= (*raceInfo).numWaypoints {
            (*this).curRaceWaypoint = 0 as libc::c_int
        }
    }
    EnHorse_RaceWaypointPos((*raceInfo).waypoints, (*this).curRaceWaypoint,
                            &mut curWaypointPos);
    prevWaypoint = (*this).curRaceWaypoint - 1 as libc::c_int;
    if prevWaypoint < 0 as libc::c_int {
        prevWaypoint = (*raceInfo).numWaypoints - 1 as libc::c_int
    }
    EnHorse_RaceWaypointPos((*raceInfo).waypoints, prevWaypoint,
                            &mut prevWaypointPos);
    Math3D_PointDistToLine2D((*this).actor.world.pos.x,
                             (*this).actor.world.pos.z, prevWaypointPos.x,
                             prevWaypointPos.z, curWaypointPos.x,
                             curWaypointPos.z, &mut dist);
    EnHorse_RotateToPoint(this, globalCtx, &mut curWaypointPos,
                          400 as libc::c_int as s16);
    if dist < 90000.0f32 {
        playerDist = (*this).actor.xzDistToPlayer;
        if playerDist < 130.0f32 ||
               (*(*this).jntSph.elements.offset(0 as libc::c_int as
                                                    isize)).info.ocElemFlags
                   as libc::c_int & 2 as libc::c_int != 0 {
            if Math_SinS(((*this).actor.yawTowardsPlayer as libc::c_int -
                              (*this).actor.world.rot.y as libc::c_int) as
                             s16) > 0.0f32 {
                (*this).actor.world.rot.y =
                    ((*this).actor.world.rot.y as libc::c_int -
                         280 as libc::c_int) as s16
            } else {
                (*this).actor.world.rot.y =
                    ((*this).actor.world.rot.y as libc::c_int +
                         280 as libc::c_int) as s16
            }
        } else if playerDist < 300.0f32 {
            if Math_SinS(((*this).actor.yawTowardsPlayer as libc::c_int -
                              (*this).actor.world.rot.y as libc::c_int) as
                             s16) > 0.0f32 {
                (*this).actor.world.rot.y =
                    ((*this).actor.world.rot.y as libc::c_int +
                         280 as libc::c_int) as s16
            } else {
                (*this).actor.world.rot.y =
                    ((*this).actor.world.rot.y as libc::c_int -
                         280 as libc::c_int) as s16
            }
        }
        (*this).actor.shape.rot.y = (*this).actor.world.rot.y
    }
    sp50 =
        Actor_WorldDistXZToActor(&mut (*this).actor,
                                 &mut (*((*(*globalCtx).actorCtx.actorLists.as_mut_ptr().offset(ACTORCAT_PLAYER
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    isize)).head
                                             as *mut Player)).actor);
    relPlayerYaw =
        (Actor_WorldYawTowardActor(&mut (*this).actor,
                                   &mut (*((*(*globalCtx).actorCtx.actorLists.as_mut_ptr().offset(ACTORCAT_PLAYER
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      isize)).head
                                               as *mut Player)).actor) as
             libc::c_int - (*this).actor.world.rot.y as libc::c_int) as s16;
    if sp50 <= 200.0f32 ||
           fabsf(Math_SinS(relPlayerYaw)) < 0.8f32 &&
               Math_CosS(relPlayerYaw) > 0.0f32 {
        if (*this).actor.speedXZ < (*this).ingoHorseMaxSpeed {
            (*this).actor.speedXZ += 0.47f32
        } else { (*this).actor.speedXZ -= 0.47f32 }
        (*this).ingoRaceFlags =
            ((*this).ingoRaceFlags as libc::c_int | 1 as libc::c_int) as
                u16_0;
        return
    }
    if (*this).actor.speedXZ <
           (*(*raceInfo).waypoints.offset((*this).curRaceWaypoint as
                                              isize)).speed as libc::c_int as
               libc::c_float {
        (*this).actor.speedXZ = (*this).actor.speedXZ + 0.4f32
    } else { (*this).actor.speedXZ = (*this).actor.speedXZ - 0.4f32 }
    (*this).ingoRaceFlags =
        ((*this).ingoRaceFlags as libc::c_int & !(0x1 as libc::c_int)) as
            u16_0;
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_PlayWalkingSound(mut this: *mut EnHorse) {
    if (sAnimSoundFrames[(*this).soundTimer as usize] as libc::c_float) <
           (*this).curFrame {
        if (*this).soundTimer == 0 as libc::c_int &&
               (sAnimSoundFrames[1 as libc::c_int as usize] as libc::c_float)
                   < (*this).curFrame {
            return
        }
        Audio_PlaySoundGeneral(0x2803 as libc::c_int as u16_0,
                               &mut (*this).actor.projectedPos,
                               4 as libc::c_int as u8_0, &mut D_801333E0,
                               &mut D_801333E0, &mut D_801333E8);
        (*this).soundTimer += 1;
        if (*this).soundTimer > 1 as libc::c_int {
            (*this).soundTimer = 0 as libc::c_int
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_PlayTrottingSound(mut this: *mut EnHorse) {
    Audio_PlaySoundGeneral(0x2804 as libc::c_int as u16_0,
                           &mut (*this).actor.projectedPos,
                           4 as libc::c_int as u8_0, &mut D_801333E0,
                           &mut D_801333E0, &mut D_801333E8);
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_PlayGallopingSound(mut this: *mut EnHorse) {
    Audio_PlaySoundGeneral(0x2804 as libc::c_int as u16_0,
                           &mut (*this).actor.projectedPos,
                           4 as libc::c_int as u8_0, &mut D_801333E0,
                           &mut D_801333E0, &mut D_801333E8);
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_SlopeSpeedMultiplier(mut this: *mut EnHorse,
                                                      mut globalCtx:
                                                          *mut GlobalContext)
 -> f32_0 {
    let mut multiplier: f32_0 = 1.0f32;
    if Math_CosS((*this).actor.shape.rot.x) < 0.939262f32 &&
           Math_SinS((*this).actor.shape.rot.x) < 0.0f32 {
        multiplier = 0.7f32
    }
    return multiplier;
}
#[no_mangle]
pub unsafe extern "C" fn func_80A5BB90(mut globalCtx: *mut GlobalContext,
                                       mut vec: *mut Vec3f,
                                       mut arg2: *mut Vec3f,
                                       mut arg3: *mut f32_0) {
    SkinMatrix_Vec3fMtxFMultXYZW(&mut (*globalCtx).viewProjectionMtxF, vec,
                                 arg2, arg3);
}
#[no_mangle]
pub unsafe extern "C" fn func_80A5BBBC(mut globalCtx: *mut GlobalContext,
                                       mut this: *mut EnHorse,
                                       mut pos: *mut Vec3f) -> s32 {
    let mut sp24: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp20: f32_0 = 0.;
    let mut eyeDist: f32_0 = 0.;
    func_80A5BB90(globalCtx, pos, &mut sp24, &mut sp20);
    if fabsf(sp20) < 0.008f32 { return 0 as libc::c_int }
    eyeDist = Math3D_Vec3f_DistXYZ(pos, &mut (*globalCtx).view.eye);
    return (func_800314D4(globalCtx, &mut (*this).actor, &mut sp24, sp20) != 0
                || eyeDist < 100.0f32) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_IdleAnimSounds(mut this: *mut EnHorse,
                                                mut globalCtx:
                                                    *mut GlobalContext) {
    if (*this).animationIdx == ENHORSE_ANIM_IDLE as libc::c_int &&
           ((*this).curFrame > 35.0f32 &&
                (*this).type_0 == HORSE_EPONA as libc::c_int ||
                (*this).curFrame > 28.0f32 &&
                    (*this).type_0 == HORSE_HNI as libc::c_int) &&
           (*this).stateFlags &
               ((1 as libc::c_int) << 12 as libc::c_int) as libc::c_uint == 0
       {
        (*this).stateFlags |=
            ((1 as libc::c_int) << 12 as libc::c_int) as libc::c_uint;
        Audio_PlaySoundGeneral(0x282c as libc::c_int as u16_0,
                               &mut (*this).actor.projectedPos,
                               4 as libc::c_int as u8_0, &mut D_801333E0,
                               &mut D_801333E0, &mut D_801333E8);
    } else if (*this).animationIdx == ENHORSE_ANIM_REARING as libc::c_int &&
                  (*this).curFrame > 25.0f32 &&
                  (*this).stateFlags &
                      ((1 as libc::c_int) << 11 as libc::c_int) as
                          libc::c_uint == 0 {
        (*this).stateFlags |=
            ((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint;
        Audio_PlaySoundGeneral(0x282b as libc::c_int as u16_0,
                               &mut (*this).actor.projectedPos,
                               4 as libc::c_int as u8_0, &mut D_801333E0,
                               &mut D_801333E0, &mut D_801333E8);
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_Spawn(mut this: *mut EnHorse,
                                       mut globalCtx: *mut GlobalContext)
 -> s32 {
    let mut minDist: f32_0 = 1e38f32;
    let mut spawn: s32 = 0 as libc::c_int;
    let mut dist: f32_0 = 0.;
    let mut i: s32 = 0;
    let mut player: *mut Player = 0 as *mut Player;
    let mut spawnPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    i = 0 as libc::c_int;
    while i < 169 as libc::c_int {
        if sHorseSpawns[i as usize].scene as libc::c_int ==
               (*globalCtx).sceneNum as libc::c_int {
            player =
                (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as
                                                     libc::c_int as
                                                     usize].head as
                    *mut Player;
            if (*globalCtx).sceneNum as libc::c_int !=
                   SCENE_SPOT20 as libc::c_int ||
                   Flags_GetEventChkInf(0x18 as libc::c_int) != 0 &&
                       (gSaveContext.eventInf[0 as libc::c_int as usize] as
                            libc::c_int & 0xf as libc::c_int !=
                            6 as libc::c_int ||
                            Flags_GetEventChkInf(0x18 as libc::c_int) != 0) ||
                   (sHorseSpawns[i as usize].pos.x as libc::c_int ==
                        856 as libc::c_int &&
                        sHorseSpawns[i as usize].pos.y as libc::c_int ==
                            0 as libc::c_int &&
                        sHorseSpawns[i as usize].pos.z as libc::c_int ==
                            -(918 as libc::c_int) ||
                        sHorseSpawns[i as usize].pos.x as libc::c_int ==
                            -(1003 as libc::c_int) &&
                            sHorseSpawns[i as usize].pos.y as libc::c_int ==
                                0 as libc::c_int &&
                            sHorseSpawns[i as usize].pos.z as libc::c_int ==
                                -(755 as libc::c_int)) {
                spawnPos.x = sHorseSpawns[i as usize].pos.x as f32_0;
                spawnPos.y = sHorseSpawns[i as usize].pos.y as f32_0;
                spawnPos.z = sHorseSpawns[i as usize].pos.z as f32_0;
                dist =
                    Math3D_Vec3f_DistXYZ(&mut (*player).actor.world.pos,
                                         &mut spawnPos);
                ((*globalCtx).sceneNum) != 0;
                if !(minDist < dist) &&
                       func_80A5BBBC(globalCtx, this, &mut spawnPos) == 0 {
                    minDist = dist;
                    (*this).actor.world.pos.x =
                        sHorseSpawns[i as usize].pos.x as f32_0;
                    (*this).actor.world.pos.y =
                        sHorseSpawns[i as usize].pos.y as f32_0;
                    (*this).actor.world.pos.z =
                        sHorseSpawns[i as usize].pos.z as f32_0;
                    (*this).actor.prevPos = (*this).actor.world.pos;
                    (*this).actor.world.rot.y =
                        sHorseSpawns[i as usize].angle;
                    (*this).actor.shape.rot.y =
                        Actor_WorldYawTowardActor(&mut (*this).actor,
                                                  &mut (*((*(*globalCtx).actorCtx.actorLists.as_mut_ptr().offset(ACTORCAT_PLAYER
                                                                                                                     as
                                                                                                                     libc::c_int
                                                                                                                     as
                                                                                                                     isize)).head
                                                              as
                                                              *mut Player)).actor);
                    spawn = 1 as libc::c_int;
                    SkinMatrix_Vec3fMtxFMultXYZW(&mut (*globalCtx).viewProjectionMtxF,
                                                 &mut (*this).actor.world.pos,
                                                 &mut (*this).actor.projectedPos,
                                                 &mut (*this).actor.projectedW);
                }
            }
        }
        i += 1
    }
    return spawn;
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_ResetCutscene(mut this: *mut EnHorse,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    (*this).cutsceneAction = -(1 as libc::c_int);
    (*this).cutsceneFlags = 0 as libc::c_int as u16_0;
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_ResetRace(mut this: *mut EnHorse,
                                           mut globalCtx:
                                               *mut GlobalContext) {
    (*this).inRace = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_PlayerCanMove(mut this: *mut EnHorse,
                                               mut globalCtx:
                                                   *mut GlobalContext)
 -> s32 {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    if (*player).stateFlags1 & 1 as libc::c_int as libc::c_uint != 0 ||
           func_8002DD78((*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as
                                                              libc::c_int as
                                                              usize].head as
                             *mut Player) == 1 as libc::c_int ||
           (*player).stateFlags1 & 0x100000 as libc::c_int as libc::c_uint !=
               0 ||
           (*this).stateFlags &
               ((1 as libc::c_int) << 19 as libc::c_int) as libc::c_uint != 0
               && (*this).inRace == 0 ||
           (*this).action as libc::c_uint ==
               ENHORSE_ACT_HBA as libc::c_int as libc::c_uint ||
           (*player).actor.flags &
               ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_uint != 0
           || (*globalCtx).csCtx.state as libc::c_int != 0 as libc::c_int {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_ResetHorsebackArchery(mut this: *mut EnHorse,
                                                       mut globalCtx:
                                                           *mut GlobalContext) {
    (*this).unk_39C = 0 as libc::c_int;
    (*this).hbaStarted = 0 as libc::c_int;
    (*this).hbaFlags = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_ClearDustFlags(mut dustFlags: *mut u16_0) {
    *dustFlags = 0 as libc::c_int as u16_0;
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_Init(mut thisx: *mut Actor,
                                      mut globalCtx2: *mut GlobalContext) {
    let mut this: *mut EnHorse = thisx as *mut EnHorse;
    let mut globalCtx: *mut GlobalContext = globalCtx2;
    (*gGameInfo).data[(19 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 6 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    Actor_ProcessInitChain(&mut (*this).actor, sInitChain.as_mut_ptr());
    EnHorse_ClearDustFlags(&mut (*this).dustFlags);
    (*gGameInfo).data[(7 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 53 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*this).riderPos = (*this).actor.world.pos;
    (*this).noInputTimer = 0 as libc::c_int;
    (*this).noInputTimerMax = 0 as libc::c_int;
    (*this).riderPos.y = (*this).riderPos.y + 70.0f32;
    if (*gGameInfo).data[(7 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 4 as libc::c_int) as usize]
           as libc::c_int == 0 as libc::c_int {
        (*gGameInfo).data[(7 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 4 as libc::c_int) as usize]
            = 70 as libc::c_int as s16
    }
    if (*this).actor.params as libc::c_int & 0x8000 as libc::c_int != 0 {
        (*this).actor.params =
            ((*this).actor.params as libc::c_int & !(0x8000 as libc::c_int))
                as s16;
        (*this).type_0 = HORSE_HNI as libc::c_int;
        (*this).bankIndex =
            Object_GetIndex(&mut (*globalCtx).objectCtx,
                            OBJECT_HNI as libc::c_int as s16) as s8;
        if ((*this).bankIndex as libc::c_int) < 0 as libc::c_int {
            Actor_Kill(&mut (*this).actor);
            return
        }
        while Object_IsLoaded(&mut (*globalCtx).objectCtx,
                              (*this).bankIndex as s32) == 0 {
        }
        (*this).actor.objBankIndex = (*this).bankIndex;
        Actor_SetObjectDependency(globalCtx, &mut (*this).actor);
        (*this).boostSpeed = 12 as libc::c_int
    } else {
        (*this).type_0 = HORSE_EPONA as libc::c_int;
        (*this).boostSpeed = 14 as libc::c_int
    }
    // params was -1
    if (*this).actor.params as libc::c_int == 0x7fff as libc::c_int {
        (*this).actor.params = 1 as libc::c_int as s16
    } // probably fake
    if (*globalCtx).sceneNum as libc::c_int == SCENE_SOUKO as libc::c_int {
        (*this).stateFlags =
            ((1 as libc::c_int) << 16 as libc::c_int) as u32_0
    } else if (*globalCtx).sceneNum as libc::c_int ==
                  SCENE_SPOT12 as libc::c_int &&
                  (*this).type_0 == HORSE_HNI as libc::c_int {
        (*this).stateFlags =
            ((1 as libc::c_int) << 18 as libc::c_int |
                 (1 as libc::c_int) << 16 as libc::c_int) as u32_0
    } else if (*this).actor.params as libc::c_int == 3 as libc::c_int {
        (*this).stateFlags =
            ((1 as libc::c_int) << 19 as libc::c_int |
                 (1 as libc::c_int) << 17 as libc::c_int |
                 (1 as libc::c_int) << 16 as libc::c_int) as u32_0
    } else if (*this).actor.params as libc::c_int == 6 as libc::c_int {
        (*this).stateFlags =
            ((1 as libc::c_int) << 19 as libc::c_int |
                 (1 as libc::c_int) << 17 as libc::c_int) as u32_0;
        if Flags_GetEventChkInf(0x18 as libc::c_int) != 0 ||
               (*gGameInfo).data[(7 as libc::c_int * 6 as libc::c_int *
                                      16 as libc::c_int + 1 as libc::c_int) as
                                     usize] as libc::c_int != 0 as libc::c_int
           {
            (*this).stateFlags &=
                !((1 as libc::c_int) << 17 as libc::c_int) as libc::c_uint;
            (*this).stateFlags |=
                ((1 as libc::c_int) << 26 as libc::c_int) as libc::c_uint
        } else if gSaveContext.eventInf[0 as libc::c_int as usize] as
                      libc::c_int & 0x40 as libc::c_int != 0 &&
                      (*this).type_0 == HORSE_HNI as libc::c_int {
            (*this).stateFlags |=
                ((1 as libc::c_int) << 21 as libc::c_int |
                     (1 as libc::c_int) << 20 as libc::c_int) as libc::c_uint
        }
    } else if (*this).actor.params as libc::c_int == 1 as libc::c_int {
        (*this).stateFlags = ((1 as libc::c_int) << 7 as libc::c_int) as u32_0
    } else { (*this).stateFlags = 0 as libc::c_int as u32_0 }
    if (*globalCtx).sceneNum as libc::c_int == SCENE_SPOT20 as libc::c_int &&
           gSaveContext.eventInf[0 as libc::c_int as usize] as libc::c_int &
               0xf as libc::c_int == 6 as libc::c_int &&
           Flags_GetEventChkInf(0x18 as libc::c_int) == 0 as libc::c_int &&
           (*gGameInfo).data[(7 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 1 as libc::c_int) as
                                 usize] == 0 {
        (*this).stateFlags |=
            ((1 as libc::c_int) << 25 as libc::c_int) as libc::c_uint
    }
    Actor_SetScale(&mut (*this).actor, 0.01f32);
    (*this).actor.gravity = -3.5f32;
    ActorShape_Init(&mut (*this).actor.shape, 0.0f32,
                    Some(ActorShadow_DrawHorse as
                             unsafe extern "C" fn(_: *mut Actor,
                                                  _: *mut Lights,
                                                  _: *mut GlobalContext)
                                 -> ()), 20.0f32);
    (*this).action = ENHORSE_ACT_IDLE;
    (*this).actor.speedXZ = 0.0f32;
    Collider_InitCylinder(globalCtx, &mut (*this).cyl1);
    Collider_SetCylinder(globalCtx, &mut (*this).cyl1, &mut (*this).actor,
                         &mut sCylinderInit1);
    Collider_InitCylinder(globalCtx, &mut (*this).cyl2);
    Collider_SetCylinder(globalCtx, &mut (*this).cyl2, &mut (*this).actor,
                         &mut sCylinderInit2);
    Collider_InitJntSph(globalCtx, &mut (*this).jntSph);
    Collider_SetJntSph(globalCtx, &mut (*this).jntSph, &mut (*this).actor,
                       &mut sJntSphInit, &mut (*this).jntSphList);
    CollisionCheck_SetInfo(&mut (*this).actor.colChkInfo,
                           DamageTable_Get(0xb as libc::c_int),
                           &mut D_80A65F38);
    (*this).actor.focus.pos = (*this).actor.world.pos;
    (*this).actor.focus.pos.y += 70.0f32;
    (*this).playerControlled = 0 as libc::c_int;
    if (*globalCtx).sceneNum as libc::c_int == SCENE_SPOT20 as libc::c_int &&
           gSaveContext.sceneSetupIndex < 4 as libc::c_int {
        if (*this).type_0 == HORSE_HNI as libc::c_int {
            if (*this).actor.world.rot.z as libc::c_int == 0 as libc::c_int ||
                   gSaveContext.nightFlag != 0 {
                Actor_Kill(&mut (*this).actor);
                return
            }
            if Flags_GetEventChkInf(0x18 as libc::c_int) != 0 {
                Actor_Kill(&mut (*this).actor);
                return
            }
            if (*this).actor.world.rot.z as libc::c_int != 5 as libc::c_int {
                Actor_Kill(&mut (*this).actor);
                return
            }
        } else if Flags_GetEventChkInf(0x18 as libc::c_int) == 0 &&
                      (*gGameInfo).data[(7 as libc::c_int * 6 as libc::c_int *
                                             16 as libc::c_int +
                                             1 as libc::c_int) as usize] == 0
                      && gSaveContext.nightFlag != 0 {
            Actor_Kill(&mut (*this).actor);
            return
        }
    } else if (*globalCtx).sceneNum as libc::c_int ==
                  SCENE_MALON_STABLE as libc::c_int {
        if gSaveContext.nightFlag == 0 as libc::c_int ||
               Flags_GetEventChkInf(0x18 as libc::c_int) != 0 ||
               (*gGameInfo).data[(7 as libc::c_int * 6 as libc::c_int *
                                      16 as libc::c_int + 1 as libc::c_int) as
                                     usize] as libc::c_int != 0 as libc::c_int
               || !(gSaveContext.linkAge == 0 as libc::c_int) {
            Actor_Kill(&mut (*this).actor);
            return
        }
        (*this).stateFlags |=
            ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_uint
    }
    func_800A663C(globalCtx, &mut (*this).skin,
                  sSkeletonHeaders[(*this).type_0 as usize],
                  *sAnimationHeaders[(*this).type_0 as
                                         usize].offset(ENHORSE_ANIM_IDLE as
                                                           libc::c_int as
                                                           isize));
    (*this).animationIdx = ENHORSE_ANIM_IDLE as libc::c_int;
    Animation_PlayOnce(&mut (*this).skin.skelAnime,
                       *sAnimationHeaders[(*this).type_0 as
                                              usize].offset((*this).animationIdx
                                                                as isize));
    (*this).numBoosts = 6 as libc::c_int as u8_0;
    (*this).boostRegenTime = 0 as libc::c_int;
    (*this).postDrawFunc =
        ::std::mem::transmute::<libc::intptr_t,
                                EnHorsePostdrawFunc>((*this).boostRegenTime as
                                                         libc::intptr_t);
    (*this).blinkTimer =
        ::std::mem::transmute::<EnHorsePostdrawFunc,
                                u8_0>((*this).postDrawFunc);
    EnHorse_ResetCutscene(this, globalCtx);
    EnHorse_ResetRace(this, globalCtx);
    EnHorse_ResetHorsebackArchery(this, globalCtx);
    if (*this).actor.params as libc::c_int == 2 as libc::c_int {
        EnHorse_InitInactive(this);
    } else if (*this).actor.params as libc::c_int == 3 as libc::c_int {
        EnHorse_InitIngoHorse(this);
        (*this).rider =
            Actor_Spawn(&mut (*globalCtx).actorCtx, globalCtx,
                        ACTOR_EN_IN as libc::c_int as s16,
                        (*this).actor.world.pos.x, (*this).actor.world.pos.y,
                        (*this).actor.world.pos.z, (*this).actor.shape.rot.x,
                        (*this).actor.shape.rot.y, 1 as libc::c_int as s16,
                        1 as libc::c_int as s16);
        if (*this).rider.is_null() {
            __assert(b"this->race.rider != NULL\x00" as *const u8 as
                         *const libc::c_char,
                     b"../z_en_horse.c\x00" as *const u8 as
                         *const libc::c_char, 3077 as libc::c_int);
        }
        if gSaveContext.eventInf[0 as libc::c_int as usize] as libc::c_int &
               0x40 as libc::c_int == 0 {
            (*this).ingoHorseMaxSpeed = 12.07f32
        } else { (*this).ingoHorseMaxSpeed = 12.625f32 }
    } else if (*this).actor.params as libc::c_int == 7 as libc::c_int {
        EnHorse_InitCutscene(this, globalCtx);
    } else if (*this).actor.params as libc::c_int == 8 as libc::c_int {
        EnHorse_InitHorsebackArchery(this);
        Interface_InitHorsebackArchery(globalCtx);
    } else if (*globalCtx).sceneNum as libc::c_int ==
                  SCENE_SPOT20 as libc::c_int &&
                  Flags_GetEventChkInf(0x18 as libc::c_int) == 0 &&
                  (*gGameInfo).data[(7 as libc::c_int * 6 as libc::c_int *
                                         16 as libc::c_int + 1 as libc::c_int)
                                        as usize] == 0 {
        EnHorse_InitFleePlayer(this);
    } else if (*globalCtx).sceneNum as libc::c_int ==
                  SCENE_SOUKO as libc::c_int {
        EnHorse_ResetIdleAnimation(this);
    } else if (*globalCtx).sceneNum as libc::c_int ==
                  SCENE_SPOT12 as libc::c_int &&
                  (*this).type_0 == HORSE_HNI as libc::c_int {
        EnHorse_ResetIdleAnimation(this);
    } else { EnHorse_StartIdleRidable(this); }
    (*this).actor.shape.rot.z = 0 as libc::c_int as s16;
    (*this).actor.world.rot.z = (*this).actor.shape.rot.z;
    (*this).actor.home.rot.z = (*this).actor.world.rot.z;
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_Destroy(mut thisx: *mut Actor,
                                         mut globalCtx: *mut GlobalContext) {
    let mut this: *mut EnHorse = thisx as *mut EnHorse;
    if (*this).stateFlags &
           ((1 as libc::c_int) << 27 as libc::c_int) as libc::c_uint != 0 {
        Audio_StopSfxByPos(&mut (*this).unk_21C);
    }
    func_800A6888(globalCtx, &mut (*this).skin);
    Collider_DestroyCylinder(globalCtx, &mut (*this).cyl1);
    Collider_DestroyCylinder(globalCtx, &mut (*this).cyl2);
    Collider_DestroyJntSph(globalCtx, &mut (*this).jntSph);
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_RotateToPlayer(mut this: *mut EnHorse,
                                                mut globalCtx:
                                                    *mut GlobalContext) {
    EnHorse_RotateToPoint(this, globalCtx,
                          &mut (*((*(*globalCtx).actorCtx.actorLists.as_mut_ptr().offset(ACTORCAT_PLAYER
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             isize)).head
                                      as *mut Player)).actor.world.pos,
                          400 as libc::c_int as s16);
    if (*this).stateFlags &
           ((1 as libc::c_int) << 14 as libc::c_int) as libc::c_uint != 0 {
        (*this).actor.world.rot.y =
            ((*this).actor.world.rot.y as libc::c_float + 800.0f32) as s16
    }
    (*this).actor.shape.rot.y = (*this).actor.world.rot.y;
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_Freeze(mut this: *mut EnHorse) {
    if (*this).action as libc::c_uint !=
           ENHORSE_ACT_CS_UPDATE as libc::c_int as libc::c_uint &&
           (*this).action as libc::c_uint !=
               ENHORSE_ACT_HBA as libc::c_int as libc::c_uint {
        if sResetNoInput[(*this).actor.params as usize] as libc::c_int !=
               0 as libc::c_int &&
               (*this).actor.params as libc::c_int != 4 as libc::c_int {
            (*this).noInputTimer = 0 as libc::c_int;
            (*this).noInputTimerMax = 0 as libc::c_int
        }
        (*this).prevAction = (*this).action;
        (*this).action = ENHORSE_ACT_FROZEN;
        (*this).cyl1.base.ocFlags1 =
            ((*this).cyl1.base.ocFlags1 as libc::c_int &
                 !((1 as libc::c_int) << 0 as libc::c_int)) as u8_0;
        (*this).cyl2.base.ocFlags1 =
            ((*this).cyl2.base.ocFlags1 as libc::c_int &
                 !((1 as libc::c_int) << 0 as libc::c_int)) as u8_0;
        (*this).jntSph.base.ocFlags1 =
            ((*this).jntSph.base.ocFlags1 as libc::c_int &
                 !((1 as libc::c_int) << 0 as libc::c_int)) as u8_0;
        (*this).animationIdx = ENHORSE_ANIM_IDLE as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_Frozen(mut this: *mut EnHorse,
                                        mut globalCtx: *mut GlobalContext) {
    (*this).actor.speedXZ = 0.0f32;
    (*this).noInputTimer -= 1;
    if (*this).noInputTimer < 0 as libc::c_int {
        (*this).cyl1.base.ocFlags1 =
            ((*this).cyl1.base.ocFlags1 as libc::c_int |
                 (1 as libc::c_int) << 0 as libc::c_int) as u8_0;
        (*this).cyl2.base.ocFlags1 =
            ((*this).cyl2.base.ocFlags1 as libc::c_int |
                 (1 as libc::c_int) << 0 as libc::c_int) as u8_0;
        (*this).jntSph.base.ocFlags1 =
            ((*this).jntSph.base.ocFlags1 as libc::c_int |
                 (1 as libc::c_int) << 0 as libc::c_int) as u8_0;
        if (*this).playerControlled == 1 as libc::c_int {
            (*this).stateFlags &=
                !((1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint;
            if (*this).actor.params as libc::c_int == 4 as libc::c_int {
                EnHorse_StartMountedIdleResetAnim(this);
            } else if (*this).actor.params as libc::c_int == 9 as libc::c_int
             {
                (*this).actor.params = 5 as libc::c_int as s16;
                if (*globalCtx).csCtx.state as libc::c_int != 0 as libc::c_int
                   {
                    EnHorse_StartMountedIdle(this);
                } else {
                    (*this).actor.speedXZ = 8.0f32;
                    EnHorse_StartGalloping(this);
                }
            } else if (*this).prevAction as libc::c_uint ==
                          2 as libc::c_int as libc::c_uint {
                EnHorse_StartMountedIdle(this);
            } else { EnHorse_StartMountedIdleResetAnim(this); }
            if (*this).actor.params as libc::c_int != 0 as libc::c_int {
                (*this).actor.params = 0 as libc::c_int as s16;
                return
            }
        } else {
            if (*this).prevAction as libc::c_uint ==
                   5 as libc::c_int as libc::c_uint {
                EnHorse_ChangeIdleAnimation(this, 0 as libc::c_int,
                                            0 as libc::c_int as f32_0);
                return
            }
            if (*this).prevAction as libc::c_uint ==
                   6 as libc::c_int as libc::c_uint {
                EnHorse_ChangeIdleAnimation(this, 0 as libc::c_int,
                                            0 as libc::c_int as f32_0);
                return
            }
            EnHorse_ChangeIdleAnimation(this, 0 as libc::c_int,
                                        0 as libc::c_int as f32_0);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_UpdateSpeed(mut this: *mut EnHorse,
                                             mut globalCtx:
                                                 *mut GlobalContext,
                                             mut brakeDecel: f32_0,
                                             mut brakeAngle: f32_0,
                                             mut minStickMag: f32_0,
                                             mut decel: f32_0,
                                             mut baseSpeed: f32_0,
                                             mut turnSpeed: s16) {
    let mut stickAnglePtr: *mut s16 = 0 as *mut s16;
    let mut stickMag: f32_0 = 0.;
    let mut stickAngle: s16 = 0;
    let mut temp_f12: f32_0 = 0.;
    let mut traction: f32_0 = 0.;
    let mut turn: s16 = 0;
    if EnHorse_PlayerCanMove(this, globalCtx) == 0 {
        if (*this).actor.speedXZ > 8 as libc::c_int as libc::c_float {
            (*this).actor.speedXZ -= decel
        } else if (*this).actor.speedXZ < 0 as libc::c_int as libc::c_float {
            (*this).actor.speedXZ = 0 as libc::c_int as f32_0
        }
        return
    }
    stickAnglePtr = &mut stickAngle;
    baseSpeed *= EnHorse_SlopeSpeedMultiplier(this, globalCtx);
    EnHorse_StickDirection(&mut (*this).curStick, &mut stickMag,
                           &mut stickAngle);
    if Math_CosS(stickAngle) <= brakeAngle {
        (*this).actor.speedXZ -= brakeDecel;
        (*this).actor.speedXZ =
            if (*this).actor.speedXZ < 0.0f32 {
                0.0f32
            } else { (*this).actor.speedXZ };
        return
    }
    if stickMag < minStickMag {
        (*this).stateFlags &=
            !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
        (*this).stateFlags &=
            !((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint;
        (*this).actor.speedXZ -= decel;
        if (*this).actor.speedXZ < 0.0f32 { (*this).actor.speedXZ = 0.0f32 }
        return
    }
    if (*this).stateFlags &
           ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint != 0 {
        if 16 as libc::c_int - (*this).boostTimer > 0 as libc::c_int {
            (*this).actor.speedXZ =
                (EnHorse_SlopeSpeedMultiplier(this, globalCtx) *
                     (*this).boostSpeed as libc::c_float -
                     (*this).actor.speedXZ) /
                    (16 as libc::c_int - (*this).boostTimer) as libc::c_float
                    + (*this).actor.speedXZ
        } else {
            (*this).actor.speedXZ =
                EnHorse_SlopeSpeedMultiplier(this, globalCtx) *
                    (*this).boostSpeed as libc::c_float
        }
        if EnHorse_SlopeSpeedMultiplier(this, globalCtx) *
               (*this).boostSpeed as libc::c_float <= (*this).actor.speedXZ {
            (*this).stateFlags &=
                !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
            (*this).stateFlags |=
                ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint
        }
    } else if (*this).stateFlags &
                  ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint !=
                  0 {
        if baseSpeed < (*this).actor.speedXZ {
            temp_f12 = (*this).actor.speedXZ;
            (*this).actor.speedXZ = temp_f12 - 0.06f32
        } else if (*this).actor.speedXZ < baseSpeed {
            (*this).actor.speedXZ = baseSpeed;
            (*this).stateFlags &=
                !((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint
        }
    } else {
        (*this).actor.speedXZ +=
            (if (*this).actor.speedXZ <=
                    baseSpeed * (1.0f32 / 54.0f32) * stickMag {
                 1.0f32
             } else { -1.0f32 }) * 50.0f32 * 0.01f32;
        if baseSpeed < (*this).actor.speedXZ {
            (*this).actor.speedXZ = (*this).actor.speedXZ - decel;
            if (*this).actor.speedXZ < baseSpeed {
                (*this).actor.speedXZ = baseSpeed
            }
        }
    }
    temp_f12 =
        *stickAnglePtr as libc::c_int as libc::c_float *
            (1 as libc::c_int as libc::c_float / 32236.0f32);
    traction =
        2.2f32 -
            (*this).actor.speedXZ *
                (1.0f32 / (*this).boostSpeed as libc::c_float);
    turn =
        (*stickAnglePtr as libc::c_int as libc::c_float * temp_f12 * temp_f12
             * traction) as s16;
    turn =
        if (turn as libc::c_int as libc::c_float) <
               -(turnSpeed as libc::c_int) as libc::c_float * traction {
            (-(turnSpeed as libc::c_int) as libc::c_float) * traction
        } else if turn as libc::c_int as libc::c_float >
                      turnSpeed as libc::c_int as libc::c_float * traction {
            (turnSpeed as libc::c_int as libc::c_float) * traction
        } else { turn as libc::c_int as libc::c_float } as s16;
    (*this).actor.world.rot.y =
        ((*this).actor.world.rot.y as libc::c_int + turn as libc::c_int) as
            s16;
    (*this).actor.shape.rot.y = (*this).actor.world.rot.y;
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_StartMountedIdleResetAnim(mut this:
                                                               *mut EnHorse) {
    (*this).skin.skelAnime.curFrame = 0.0f32;
    EnHorse_StartMountedIdle(this);
    (*this).stateFlags &=
        !((1 as libc::c_int) << 12 as libc::c_int) as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_StartMountedIdle(mut this: *mut EnHorse) {
    let mut curFrame: f32_0 = 0.;
    (*this).action = ENHORSE_ACT_MOUNTED_IDLE;
    (*this).animationIdx = ENHORSE_ANIM_IDLE as libc::c_int;
    if (*this).curFrame > 35.0f32 &&
           (*this).type_0 == HORSE_EPONA as libc::c_int ||
           (*this).curFrame > 28.0f32 &&
               (*this).type_0 == HORSE_HNI as libc::c_int {
        if (*this).stateFlags &
               ((1 as libc::c_int) << 12 as libc::c_int) as libc::c_uint == 0
           {
            (*this).stateFlags |=
                ((1 as libc::c_int) << 12 as libc::c_int) as libc::c_uint;
            Audio_PlaySoundGeneral(0x282c as libc::c_int as u16_0,
                                   &mut (*this).actor.projectedPos,
                                   4 as libc::c_int as u8_0, &mut D_801333E0,
                                   &mut D_801333E0, &mut D_801333E8);
        }
    }
    curFrame = (*this).skin.skelAnime.curFrame;
    Animation_Change(&mut (*this).skin.skelAnime,
                     *sAnimationHeaders[(*this).type_0 as
                                            usize].offset((*this).animationIdx
                                                              as isize),
                     1.0f32, curFrame,
                     Animation_GetLastFrame(*sAnimationHeaders[(*this).type_0
                                                                   as
                                                                   usize].offset((*this).animationIdx
                                                                                     as
                                                                                     isize)
                                                as *mut libc::c_void) as
                         f32_0, ANIMMODE_ONCE as libc::c_int as u8_0,
                     -3.0f32);
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_MountedIdle(mut this: *mut EnHorse,
                                             mut globalCtx:
                                                 *mut GlobalContext) {
    let mut mag: f32_0 = 0.;
    let mut angle: s16 = 0 as libc::c_int as s16;
    (*this).actor.speedXZ = 0 as libc::c_int as f32_0;
    EnHorse_StickDirection(&mut (*this).curStick, &mut mag, &mut angle);
    if mag > 10.0f32 &&
           EnHorse_PlayerCanMove(this, globalCtx) == 1 as libc::c_int {
        if Math_CosS(angle) <= -0.5f32 {
            EnHorse_StartReversingInterruptable(this);
        } else if Math_CosS(angle) as libc::c_double <= 0.7071f64 {
            // cos(45 degrees)
            EnHorse_StartTurning(this);
        } else { EnHorse_StartWalkingFromIdle(this); }
    }
    if SkelAnime_Update(&mut (*this).skin.skelAnime) != 0 {
        EnHorse_MountedIdleAnim(this);
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_MountedIdleAnim(mut this: *mut EnHorse) {
    (*this).skin.skelAnime.curFrame = 0.0f32;
    EnHorse_MountedIdleWhinney(this);
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_MountedIdleWhinney(mut this: *mut EnHorse) {
    let mut curFrame: f32_0 = 0.;
    (*this).action = ENHORSE_ACT_MOUNTED_IDLE_WHINNEYING;
    (*this).animationIdx = ENHORSE_ANIM_WHINNEY as libc::c_int;
    curFrame = (*this).skin.skelAnime.curFrame;
    Animation_Change(&mut (*this).skin.skelAnime,
                     *sAnimationHeaders[(*this).type_0 as
                                            usize].offset((*this).animationIdx
                                                              as isize),
                     1.0f32, curFrame,
                     Animation_GetLastFrame(*sAnimationHeaders[(*this).type_0
                                                                   as
                                                                   usize].offset((*this).animationIdx
                                                                                     as
                                                                                     isize)
                                                as *mut libc::c_void) as
                         f32_0, ANIMMODE_ONCE as libc::c_int as u8_0,
                     -3.0f32);
    (*this).unk_21C = (*this).unk_228;
    if (*this).stateFlags &
           ((1 as libc::c_int) << 27 as libc::c_int) as libc::c_uint != 0 {
        Audio_PlaySoundGeneral(0x2816 as libc::c_int as u16_0,
                               &mut (*this).unk_21C, 4 as libc::c_int as u8_0,
                               &mut D_801333E0, &mut D_801333E0,
                               &mut D_801333E8);
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_MountedIdleWhinneying(mut this: *mut EnHorse,
                                                       mut globalCtx:
                                                           *mut GlobalContext) {
    let mut stickMag: f32_0 = 0.;
    let mut stickAngle: s16 = 0 as libc::c_int as s16;
    (*this).actor.speedXZ = 0 as libc::c_int as f32_0;
    EnHorse_StickDirection(&mut (*this).curStick, &mut stickMag,
                           &mut stickAngle);
    if stickMag > 10.0f32 &&
           EnHorse_PlayerCanMove(this, globalCtx) == 1 as libc::c_int {
        if Math_CosS(stickAngle) <= -0.5f32 {
            EnHorse_StartReversingInterruptable(this);
        } else if Math_CosS(stickAngle) as libc::c_double <= 0.7071f64 {
            // cos(45 degrees)
            EnHorse_StartTurning(this);
        } else { EnHorse_StartWalkingFromIdle(this); }
    }
    if SkelAnime_Update(&mut (*this).skin.skelAnime) != 0 {
        EnHorse_StartMountedIdleResetAnim(this);
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_StartTurning(mut this: *mut EnHorse) {
    (*this).action = ENHORSE_ACT_MOUNTED_TURN;
    (*this).soundTimer = 0 as libc::c_int;
    (*this).animationIdx = ENHORSE_ANIM_WALK as libc::c_int;
    Animation_Change(&mut (*this).skin.skelAnime,
                     *sAnimationHeaders[(*this).type_0 as
                                            usize].offset((*this).animationIdx
                                                              as isize),
                     1.0f32, 0.0f32,
                     Animation_GetLastFrame(*sAnimationHeaders[(*this).type_0
                                                                   as
                                                                   usize].offset((*this).animationIdx
                                                                                     as
                                                                                     isize)
                                                as *mut libc::c_void) as
                         f32_0, ANIMMODE_ONCE as libc::c_int as u8_0,
                     -3.0f32);
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_MountedTurn(mut this: *mut EnHorse,
                                             mut globalCtx:
                                                 *mut GlobalContext) {
    let mut stickMag: f32_0 = 0.;
    let mut clampedYaw: s16 = 0;
    let mut stickAngle: s16 = 0;
    (*this).actor.speedXZ = 0 as libc::c_int as f32_0;
    EnHorse_PlayWalkingSound(this);
    EnHorse_StickDirection(&mut (*this).curStick, &mut stickMag,
                           &mut stickAngle);
    if stickMag > 10.0f32 {
        if EnHorse_PlayerCanMove(this, globalCtx) == 0 {
            EnHorse_StartMountedIdleResetAnim(this);
        } else if Math_CosS(stickAngle) <= -0.5f32 {
            EnHorse_StartReversingInterruptable(this);
        } else if Math_CosS(stickAngle) as libc::c_double <= 0.7071f64 {
            // cos(45 degrees)
            clampedYaw =
                if (stickAngle as libc::c_int as libc::c_float) < -800.0f32 {
                    -800.0f32
                } else if stickAngle as libc::c_int as libc::c_float >
                              800.0f32 {
                    800.0f32
                } else { stickAngle as libc::c_int as libc::c_float } as s16;
            (*this).actor.world.rot.y =
                ((*this).actor.world.rot.y as libc::c_int +
                     clampedYaw as libc::c_int) as s16;
            (*this).actor.shape.rot.y = (*this).actor.world.rot.y
        } else { EnHorse_StartWalkingInterruptable(this); }
    }
    if SkelAnime_Update(&mut (*this).skin.skelAnime) != 0 {
        if Math_CosS(stickAngle) as libc::c_double <= 0.7071f64 {
            // cos(45 degrees)
            EnHorse_StartTurning(this);
        } else { EnHorse_StartMountedIdleResetAnim(this); }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_StartWalkingFromIdle(mut this:
                                                          *mut EnHorse) {
    EnHorse_StartWalkingInterruptable(this);
    if (*this).stateFlags &
           ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_uint == 0 &&
           (*this).stateFlags &
               ((1 as libc::c_int) << 9 as libc::c_int) as libc::c_uint == 0 {
        (*this).stateFlags |=
            ((1 as libc::c_int) << 9 as libc::c_int) as libc::c_uint;
        (*this).waitTimer = 8 as libc::c_int as s16;
        return
    }
    (*this).waitTimer = 0 as libc::c_int as s16;
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_StartWalkingInterruptable(mut this:
                                                               *mut EnHorse) {
    (*this).noInputTimer = 0 as libc::c_int;
    (*this).noInputTimerMax = 0 as libc::c_int;
    EnHorse_StartWalking(this);
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_StartWalking(mut this: *mut EnHorse) {
    (*this).action = ENHORSE_ACT_MOUNTED_WALK;
    (*this).soundTimer = 0 as libc::c_int;
    (*this).animationIdx = ENHORSE_ANIM_WALK as libc::c_int;
    (*this).waitTimer = 0 as libc::c_int as s16;
    Animation_Change(&mut (*this).skin.skelAnime,
                     *sAnimationHeaders[(*this).type_0 as
                                            usize].offset((*this).animationIdx
                                                              as isize),
                     1.0f32, 0.0f32,
                     Animation_GetLastFrame(*sAnimationHeaders[(*this).type_0
                                                                   as
                                                                   usize].offset((*this).animationIdx
                                                                                     as
                                                                                     isize)
                                                as *mut libc::c_void) as
                         f32_0, ANIMMODE_ONCE as libc::c_int as u8_0,
                     -3.0f32);
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_MountedWalkingReset(mut this: *mut EnHorse) {
    (*this).action = ENHORSE_ACT_MOUNTED_WALK;
    (*this).soundTimer = 0 as libc::c_int;
    (*this).animationIdx = ENHORSE_ANIM_WALK as libc::c_int;
    (*this).waitTimer = 0 as libc::c_int as s16;
    Animation_PlayOnce(&mut (*this).skin.skelAnime,
                       *sAnimationHeaders[(*this).type_0 as
                                              usize].offset((*this).animationIdx
                                                                as isize));
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_MountedWalk(mut this: *mut EnHorse,
                                             mut globalCtx:
                                                 *mut GlobalContext) {
    let mut stickMag: f32_0 = 0.;
    let mut stickAngle: s16 = 0;
    EnHorse_PlayWalkingSound(this);
    EnHorse_StickDirection(&mut (*this).curStick, &mut stickMag,
                           &mut stickAngle);
    if (*this).noInputTimerMax as libc::c_float == 0.0f32 ||
           (*this).noInputTimer as libc::c_float > 0.0f32 &&
               ((*this).noInputTimer as libc::c_float) <
                   (*this).noInputTimerMax as libc::c_float - 20.0f32 {
        EnHorse_UpdateSpeed(this, globalCtx, 0.3f32, -0.5f32, 10.0f32,
                            0.06f32, 3.0f32, 400 as libc::c_int as s16);
    } else { (*this).actor.speedXZ = 3.0f32 }
    if (*this).actor.speedXZ == 0.0f32 {
        (*this).stateFlags &=
            !((1 as libc::c_int) << 9 as libc::c_int) as libc::c_uint;
        EnHorse_StartMountedIdleResetAnim(this);
        (*this).noInputTimer = 0 as libc::c_int;
        (*this).noInputTimerMax = 0 as libc::c_int
    } else if (*this).actor.speedXZ > 3.0f32 {
        (*this).stateFlags &=
            !((1 as libc::c_int) << 9 as libc::c_int) as libc::c_uint;
        EnHorse_StartTrotting(this);
        (*this).noInputTimer = 0 as libc::c_int;
        (*this).noInputTimerMax = 0 as libc::c_int
    }
    if (*this).noInputTimer as libc::c_float > 0.0f32 {
        (*this).noInputTimer -= 1;
        if (*this).noInputTimer as libc::c_float <= 0.0f32 {
            (*this).noInputTimerMax = 0 as libc::c_int
        }
    }
    if (*this).waitTimer as libc::c_int <= 0 as libc::c_int {
        (*this).stateFlags &=
            !((1 as libc::c_int) << 9 as libc::c_int) as libc::c_uint;
        (*this).skin.skelAnime.playSpeed = (*this).actor.speedXZ * 0.75f32;
        if SkelAnime_Update(&mut (*this).skin.skelAnime) != 0 ||
               (*this).actor.speedXZ == 0.0f32 {
            if (*this).noInputTimer as libc::c_float <= 0.0f32 {
                if (*this).actor.speedXZ > 3.0f32 {
                    EnHorse_StartTrotting(this);
                    (*this).noInputTimer = 0 as libc::c_int;
                    (*this).noInputTimerMax = 0 as libc::c_int
                } else if stickMag < 10.0f32 ||
                              Math_CosS(stickAngle) <= -0.5f32 {
                    EnHorse_StartMountedIdleResetAnim(this);
                    (*this).noInputTimer = 0 as libc::c_int;
                    (*this).noInputTimerMax = 0 as libc::c_int
                } else { EnHorse_MountedWalkingReset(this); }
            }
        }
    } else { (*this).actor.speedXZ = 0.0f32; (*this).waitTimer -= 1 };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_StartTrotting(mut this: *mut EnHorse) {
    (*this).action = ENHORSE_ACT_MOUNTED_TROT;
    (*this).animationIdx = ENHORSE_ANIM_TROT as libc::c_int;
    Animation_Change(&mut (*this).skin.skelAnime,
                     *sAnimationHeaders[(*this).type_0 as
                                            usize].offset((*this).animationIdx
                                                              as isize),
                     1.0f32, 0.0f32,
                     Animation_GetLastFrame(*sAnimationHeaders[(*this).type_0
                                                                   as
                                                                   usize].offset((*this).animationIdx
                                                                                     as
                                                                                     isize)
                                                as *mut libc::c_void) as
                         f32_0, ANIMMODE_ONCE as libc::c_int as u8_0,
                     -3.0f32);
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_MountedTrotReset(mut this: *mut EnHorse) {
    (*this).action = ENHORSE_ACT_MOUNTED_TROT;
    (*this).animationIdx = ENHORSE_ANIM_TROT as libc::c_int;
    Animation_PlayOnce(&mut (*this).skin.skelAnime,
                       *sAnimationHeaders[(*this).type_0 as
                                              usize].offset((*this).animationIdx
                                                                as isize));
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_MountedTrot(mut this: *mut EnHorse,
                                             mut globalCtx:
                                                 *mut GlobalContext) {
    let mut stickMag: f32_0 = 0.;
    let mut stickAngle: s16 = 0;
    EnHorse_UpdateSpeed(this, globalCtx, 0.3f32, -0.5f32, 10.0f32, 0.06f32,
                        6.0f32, 400 as libc::c_int as s16);
    EnHorse_StickDirection(&mut (*this).curStick, &mut stickMag,
                           &mut stickAngle);
    if (*this).actor.speedXZ < 3.0f32 {
        EnHorse_StartWalkingInterruptable(this);
    }
    (*this).skin.skelAnime.playSpeed = (*this).actor.speedXZ * 0.375f32;
    if SkelAnime_Update(&mut (*this).skin.skelAnime) != 0 {
        EnHorse_PlayTrottingSound(this);
        func_800AA000(0.0f32, 60 as libc::c_int as u8_0,
                      8 as libc::c_int as u8_0, 255 as libc::c_int as u8_0);
        if (*this).actor.speedXZ >= 6.0f32 {
            EnHorse_StartGallopingInterruptable(this);
        } else if (*this).actor.speedXZ < 3.0f32 {
            EnHorse_StartWalkingInterruptable(this);
        } else { EnHorse_MountedTrotReset(this); }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_StartGallopingInterruptable(mut this:
                                                                 *mut EnHorse) {
    (*this).noInputTimerMax = 0 as libc::c_int;
    (*this).noInputTimer = 0 as libc::c_int;
    EnHorse_StartGalloping(this);
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_StartGalloping(mut this: *mut EnHorse) {
    (*this).action = ENHORSE_ACT_MOUNTED_GALLOP;
    (*this).animationIdx = ENHORSE_ANIM_GALLOP as libc::c_int;
    (*this).unk_234 = 0 as libc::c_int;
    Animation_Change(&mut (*this).skin.skelAnime,
                     *sAnimationHeaders[(*this).type_0 as
                                            usize].offset((*this).animationIdx
                                                              as isize),
                     1.0f32, 0.0f32,
                     Animation_GetLastFrame(*sAnimationHeaders[(*this).type_0
                                                                   as
                                                                   usize].offset((*this).animationIdx
                                                                                     as
                                                                                     isize)
                                                as *mut libc::c_void) as
                         f32_0, ANIMMODE_ONCE as libc::c_int as u8_0,
                     -3.0f32);
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_MountedGallopReset(mut this: *mut EnHorse) {
    (*this).noInputTimerMax = 0 as libc::c_int;
    (*this).noInputTimer = 0 as libc::c_int;
    (*this).action = ENHORSE_ACT_MOUNTED_GALLOP;
    (*this).animationIdx = ENHORSE_ANIM_GALLOP as libc::c_int;
    (*this).unk_234 = 0 as libc::c_int;
    Animation_PlayOnce(&mut (*this).skin.skelAnime,
                       *sAnimationHeaders[(*this).type_0 as
                                              usize].offset((*this).animationIdx
                                                                as isize));
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_JumpLanding(mut this: *mut EnHorse,
                                             mut globalCtx:
                                                 *mut GlobalContext) {
    let mut jointTable: *mut Vec3s = 0 as *mut Vec3s;
    let mut y: f32_0 = 0.;
    (*this).action = ENHORSE_ACT_MOUNTED_GALLOP;
    (*this).animationIdx = ENHORSE_ANIM_GALLOP as libc::c_int;
    Animation_PlayOnce(&mut (*this).skin.skelAnime,
                       *sAnimationHeaders[(*this).type_0 as
                                              usize].offset((*this).animationIdx
                                                                as isize));
    jointTable = (*this).skin.skelAnime.jointTable;
    y = (*jointTable).y as f32_0;
    (*this).riderPos.y += y * 0.01f32;
    (*this).postDrawFunc = None;
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_MountedGallop(mut this: *mut EnHorse,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    let mut stickMag: f32_0 = 0.;
    let mut stickAngle: s16 = 0;
    EnHorse_StickDirection(&mut (*this).curStick, &mut stickMag,
                           &mut stickAngle);
    if (*this).noInputTimer as libc::c_float <= 0.0f32 {
        EnHorse_UpdateSpeed(this, globalCtx, 0.3f32, -0.5f32, 10.0f32,
                            0.06f32, 8.0f32, 0x190 as libc::c_int as s16);
    } else if (*this).noInputTimer as libc::c_float > 0.0f32 {
        (*this).noInputTimer -= 1 as libc::c_int;
        (*this).actor.speedXZ = 8.0f32
    }
    if (*this).actor.speedXZ < 6.0f32 { EnHorse_StartTrotting(this); }
    (*this).skin.skelAnime.playSpeed = (*this).actor.speedXZ * 0.3f32;
    if SkelAnime_Update(&mut (*this).skin.skelAnime) != 0 {
        EnHorse_PlayGallopingSound(this);
        func_800AA000(0 as libc::c_int as f32_0, 120 as libc::c_int as u8_0,
                      8 as libc::c_int as u8_0, 255 as libc::c_int as u8_0);
        if EnHorse_PlayerCanMove(this, globalCtx) == 1 as libc::c_int {
            if stickMag >= 10.0f32 && Math_CosS(stickAngle) <= -0.5f32 {
                EnHorse_StartBraking(this, globalCtx);
            } else if (*this).actor.speedXZ < 6.0f32 {
                EnHorse_StartTrotting(this);
            } else { EnHorse_MountedGallopReset(this); }
            return
        }
        EnHorse_MountedGallopReset(this);
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_StartRearing(mut this: *mut EnHorse) {
    (*this).action = ENHORSE_ACT_MOUNTED_REARING;
    (*this).animationIdx = ENHORSE_ANIM_REARING as libc::c_int;
    (*this).stateFlags &=
        !((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint;
    (*this).unk_21C = (*this).unk_228;
    if (*this).stateFlags &
           ((1 as libc::c_int) << 27 as libc::c_int) as libc::c_uint != 0 {
        Audio_PlaySoundGeneral(0x2805 as libc::c_int as u16_0,
                               &mut (*this).unk_21C, 4 as libc::c_int as u8_0,
                               &mut D_801333E0, &mut D_801333E0,
                               &mut D_801333E8);
    }
    func_800AA000(0.0f32, 180 as libc::c_int as u8_0,
                  20 as libc::c_int as u8_0, 100 as libc::c_int as u8_0);
    Animation_Change(&mut (*this).skin.skelAnime,
                     *sAnimationHeaders[(*this).type_0 as
                                            usize].offset((*this).animationIdx
                                                              as isize),
                     1.0f32, 0.0f32,
                     Animation_GetLastFrame(*sAnimationHeaders[(*this).type_0
                                                                   as
                                                                   usize].offset((*this).animationIdx
                                                                                     as
                                                                                     isize)
                                                as *mut libc::c_void) as
                         f32_0, ANIMMODE_ONCE as libc::c_int as u8_0,
                     -3.0f32);
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_MountedRearing(mut this: *mut EnHorse,
                                                mut globalCtx:
                                                    *mut GlobalContext) {
    let mut stickMag: f32_0 = 0.;
    let mut stickAngle: s16 = 0;
    (*this).actor.speedXZ = 0 as libc::c_int as f32_0;
    if (*this).curFrame > 25.0f32 {
        if (*this).stateFlags &
               ((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint == 0
           {
            (*this).stateFlags |=
                ((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint;
            Audio_PlaySoundGeneral(0x282b as libc::c_int as u16_0,
                                   &mut (*this).actor.projectedPos,
                                   4 as libc::c_int as u8_0, &mut D_801333E0,
                                   &mut D_801333E0, &mut D_801333E8);
            func_800AA000(0 as libc::c_int as f32_0,
                          180 as libc::c_int as u8_0,
                          20 as libc::c_int as u8_0,
                          100 as libc::c_int as u8_0);
        }
    }
    EnHorse_StickDirection(&mut (*this).curStick, &mut stickMag,
                           &mut stickAngle);
    if SkelAnime_Update(&mut (*this).skin.skelAnime) != 0 {
        if EnHorse_PlayerCanMove(this, globalCtx) == 1 as libc::c_int {
            if (*this).stateFlags &
                   ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_uint !=
                   0 {
                (*this).noInputTimer = 100 as libc::c_int;
                (*this).noInputTimerMax = 100 as libc::c_int;
                (*this).stateFlags &=
                    !((1 as libc::c_int) << 4 as libc::c_int) as libc::c_uint;
                EnHorse_StartReversing(this);
            } else if (*this).stateFlags &
                          ((1 as libc::c_int) << 5 as libc::c_int) as
                              libc::c_uint != 0 {
                (*this).noInputTimer = 100 as libc::c_int;
                (*this).noInputTimerMax = 100 as libc::c_int;
                (*this).stateFlags &=
                    !((1 as libc::c_int) << 5 as libc::c_int) as libc::c_uint;
                EnHorse_StartWalking(this);
            } else if Math_CosS(stickAngle) <= -0.5f32 {
                EnHorse_StartReversingInterruptable(this);
            } else { EnHorse_StartMountedIdleResetAnim(this); }
            return
        }
        EnHorse_StartMountedIdleResetAnim(this);
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_StartBraking(mut this: *mut EnHorse,
                                              mut globalCtx:
                                                  *mut GlobalContext) {
    (*this).action = ENHORSE_ACT_STOPPING;
    (*this).animationIdx = ENHORSE_ANIM_STOPPING as libc::c_int;
    Audio_PlaySoundGeneral(0x281a as libc::c_int as u16_0,
                           &mut (*this).actor.projectedPos,
                           4 as libc::c_int as u8_0, &mut D_801333E0,
                           &mut D_801333E0, &mut D_801333E8);
    Animation_Change(&mut (*this).skin.skelAnime,
                     *sAnimationHeaders[(*this).type_0 as
                                            usize].offset((*this).animationIdx
                                                              as isize),
                     1.5f32, 0.0f32,
                     Animation_GetLastFrame(*sAnimationHeaders[(*this).type_0
                                                                   as
                                                                   usize].offset((*this).animationIdx
                                                                                     as
                                                                                     isize)
                                                as *mut libc::c_void) as
                         f32_0, ANIMMODE_ONCE as libc::c_int as u8_0,
                     -3.0f32);
    (*this).stateFlags |=
        ((1 as libc::c_int) << 10 as libc::c_int) as libc::c_uint;
    (*this).stateFlags &=
        !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_Stopping(mut this: *mut EnHorse,
                                          mut globalCtx: *mut GlobalContext) {
    if (*this).actor.speedXZ > 0.0f32 {
        (*this).actor.speedXZ = (*this).actor.speedXZ - 0.6f32;
        if (*this).actor.speedXZ < 0.0f32 { (*this).actor.speedXZ = 0.0f32 }
    }
    if (*this).stateFlags &
           ((1 as libc::c_int) << 10 as libc::c_int) as libc::c_uint != 0 &&
           (*this).skin.skelAnime.curFrame > 29.0f32 {
        (*this).actor.speedXZ = 0.0f32;
        if Rand_ZeroOne() as libc::c_double > 0.5f64 {
            (*this).unk_21C = (*this).unk_228;
            if (*this).stateFlags &
                   ((1 as libc::c_int) << 27 as libc::c_int) as libc::c_uint
                   != 0 {
                Audio_PlaySoundGeneral(0x2805 as libc::c_int as u16_0,
                                       &mut (*this).unk_21C,
                                       4 as libc::c_int as u8_0,
                                       &mut D_801333E0, &mut D_801333E0,
                                       &mut D_801333E8);
            }
            func_800AA000(0.0f32, 180 as libc::c_int as u8_0,
                          20 as libc::c_int as u8_0,
                          100 as libc::c_int as u8_0);
            (*this).stateFlags &=
                !((1 as libc::c_int) << 10 as libc::c_int) as libc::c_uint
        } else { EnHorse_StartMountedIdleResetAnim(this); }
    }
    if (*this).skin.skelAnime.curFrame > 29.0f32 {
        (*this).actor.speedXZ = 0.0f32
    } else if (*this).actor.speedXZ > 3.0f32 &&
                  (*this).stateFlags &
                      ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_uint
                      != 0 {
        (*this).actor.speedXZ = 3.0f32
    }
    if SkelAnime_Update(&mut (*this).skin.skelAnime) != 0 {
        if (*this).stateFlags &
               ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_uint != 0 {
            (*this).noInputTimer = 100 as libc::c_int;
            (*this).noInputTimerMax = 100 as libc::c_int;
            EnHorse_StartReversing(this);
            (*this).stateFlags &=
                !((1 as libc::c_int) << 4 as libc::c_int) as libc::c_uint
        } else { EnHorse_StartMountedIdleResetAnim(this); }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_StartReversingInterruptable(mut this:
                                                                 *mut EnHorse) {
    (*this).noInputTimerMax = 0 as libc::c_int;
    (*this).noInputTimer = 0 as libc::c_int;
    EnHorse_StartReversing(this);
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_StartReversing(mut this: *mut EnHorse) {
    (*this).action = ENHORSE_ACT_REVERSE;
    (*this).animationIdx = ENHORSE_ANIM_WALK as libc::c_int;
    (*this).soundTimer = 0 as libc::c_int;
    Animation_Change(&mut (*this).skin.skelAnime,
                     *sAnimationHeaders[(*this).type_0 as
                                            usize].offset((*this).animationIdx
                                                              as isize),
                     1.0f32, 0.0f32,
                     Animation_GetLastFrame(*sAnimationHeaders[(*this).type_0
                                                                   as
                                                                   usize].offset((*this).animationIdx
                                                                                     as
                                                                                     isize)
                                                as *mut libc::c_void) as
                         f32_0, ANIMMODE_LOOP as libc::c_int as u8_0,
                     -3.0f32);
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_Reverse(mut this: *mut EnHorse,
                                         mut globalCtx: *mut GlobalContext) {
    let mut stickMag: f32_0 = 0.;
    let mut stickAngle: s16 = 0;
    let mut turnAmount: s16 = 0;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    EnHorse_PlayWalkingSound(this);
    EnHorse_StickDirection(&mut (*this).curStick, &mut stickMag,
                           &mut stickAngle);
    if EnHorse_PlayerCanMove(this, globalCtx) == 1 as libc::c_int {
        if (*this).noInputTimerMax as libc::c_float == 0.0f32 ||
               (*this).noInputTimer as libc::c_float > 0.0f32 &&
                   ((*this).noInputTimer as libc::c_float) <
                       (*this).noInputTimerMax as libc::c_float - 20.0f32 {
            if stickMag < 10.0f32 &&
                   (*this).noInputTimer as libc::c_float <= 0.0f32 {
                EnHorse_StartMountedIdleResetAnim(this);
                (*this).actor.speedXZ = 0.0f32;
                return
            }
            if stickMag < 10.0f32 {
                stickAngle = -(0x7fff as libc::c_int) as s16
            } else if Math_CosS(stickAngle) > -0.5f32 {
                (*this).noInputTimerMax = 0 as libc::c_int;
                EnHorse_StartMountedIdleResetAnim(this);
                (*this).actor.speedXZ = 0.0f32;
                return
            }
        } else if stickMag < 10.0f32 {
            stickAngle = -(0x7fff as libc::c_int) as s16
        }
    } else if (*player).actor.flags &
                  ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_uint !=
                  0 {
        EnHorse_StartMountedIdleResetAnim(this);
        (*this).actor.speedXZ = 0.0f32;
        return
    } else { stickAngle = -(0x7fff as libc::c_int) as s16 }
    (*this).actor.speedXZ = -2.0f32;
    turnAmount = (0x7fff as libc::c_int - stickAngle as libc::c_int) as s16;
    turnAmount =
        if (turnAmount as libc::c_int as libc::c_float) < -1200.0f32 {
            -1200.0f32
        } else if turnAmount as libc::c_int as libc::c_float > 1200.0f32 {
            1200.0f32
        } else { turnAmount as libc::c_int as libc::c_float } as s16;
    (*this).actor.world.rot.y =
        ((*this).actor.world.rot.y as libc::c_int + turnAmount as libc::c_int)
            as s16;
    (*this).actor.shape.rot.y = (*this).actor.world.rot.y;
    if (*this).noInputTimer as libc::c_float > 0.0f32 {
        (*this).noInputTimer -= 1;
        if (*this).noInputTimer as libc::c_float <= 0.0f32 {
            (*this).noInputTimerMax = 0 as libc::c_int
        }
    }
    (*this).skin.skelAnime.playSpeed =
        (*this).actor.speedXZ * 0.5f32 * 1.5f32;
    if SkelAnime_Update(&mut (*this).skin.skelAnime) != 0 &&
           (*this).noInputTimer as f32_0 <= 0.0f32 &&
           EnHorse_PlayerCanMove(this, globalCtx) == 1 as libc::c_int {
        if stickMag > 10.0f32 && Math_CosS(stickAngle) <= -0.5f32 {
            (*this).noInputTimerMax = 0 as libc::c_int;
            EnHorse_StartReversingInterruptable(this);
        } else if stickMag < 10.0f32 {
            (*this).noInputTimerMax = 0 as libc::c_int;
            EnHorse_StartMountedIdleResetAnim(this);
        } else { EnHorse_StartReversing(this); }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_LowJumpInit(mut this: *mut EnHorse,
                                             mut globalCtx:
                                                 *mut GlobalContext) {
    (*this).skin.skelAnime.curFrame = 0.0f32;
    EnHorse_StartLowJump(this, globalCtx);
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_StartLowJump(mut this: *mut EnHorse,
                                              mut globalCtx:
                                                  *mut GlobalContext) {
    let mut curFrame: f32_0 = 0.;
    let mut jointTable: *mut Vec3s = 0 as *mut Vec3s;
    let mut y: f32_0 = 0.;
    (*this).action = ENHORSE_ACT_LOW_JUMP;
    (*this).animationIdx = ENHORSE_ANIM_LOW_JUMP as libc::c_int;
    curFrame = (*this).skin.skelAnime.curFrame;
    Animation_Change(&mut (*this).skin.skelAnime,
                     *sAnimationHeaders[(*this).type_0 as
                                            usize].offset((*this).animationIdx
                                                              as isize),
                     1.5f32, curFrame,
                     Animation_GetLastFrame(*sAnimationHeaders[(*this).type_0
                                                                   as
                                                                   usize].offset((*this).animationIdx
                                                                                     as
                                                                                     isize)
                                                as *mut libc::c_void) as
                         f32_0, ANIMMODE_ONCE as libc::c_int as u8_0,
                     -3.0f32);
    (*this).postDrawFunc = None;
    (*this).jumpStartY = (*this).actor.world.pos.y;
    (*this).actor.gravity = 0.0f32;
    (*this).actor.velocity.y = 0 as libc::c_int as f32_0;
    jointTable = (*this).skin.skelAnime.jointTable;
    y = (*jointTable).y as f32_0;
    (*this).riderPos.y -= y * 0.01f32;
    Audio_PlaySoundGeneral(0x2818 as libc::c_int as u16_0,
                           &mut (*this).actor.projectedPos,
                           4 as libc::c_int as u8_0, &mut D_801333E0,
                           &mut D_801333E0, &mut D_801333E8);
    func_800AA000(0.0f32, 170 as libc::c_int as u8_0,
                  10 as libc::c_int as u8_0, 10 as libc::c_int as u8_0);
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_Stub1(mut this: *mut EnHorse) { }
#[no_mangle]
pub unsafe extern "C" fn EnHorse_LowJump(mut this: *mut EnHorse,
                                         mut globalCtx: *mut GlobalContext) {
    let mut pad: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut jointTable: *mut Vec3s = 0 as *mut Vec3s;
    let mut curFrame: f32_0 = 0.;
    let mut y: f32_0 = 0.;
    curFrame = (*this).skin.skelAnime.curFrame;
    (*this).stateFlags |=
        ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint;
    (*this).actor.speedXZ = 12.0f32;
    if curFrame > 17.0f32 {
        (*this).actor.gravity = -3.5f32;
        if (*this).actor.velocity.y == 0 as libc::c_int as libc::c_float {
            (*this).actor.velocity.y = -6.0f32
        }
        if (*this).actor.world.pos.y < (*this).actor.floorHeight + 90.0f32 {
            (*this).skin.skelAnime.playSpeed = 1.5f32
        } else {
            (*this).skin.skelAnime.playSpeed = 0 as libc::c_int as f32_0
        }
    } else {
        jointTable = (*this).skin.skelAnime.jointTable;
        y = (*jointTable).y as f32_0;
        (*this).actor.world.pos.y = (*this).jumpStartY + y * 0.01f32
    }
    if SkelAnime_Update(&mut (*this).skin.skelAnime) != 0 ||
           curFrame > 17.0f32 &&
               (*this).actor.world.pos.y <
                   (*this).actor.floorHeight - (*this).actor.velocity.y +
                       80.0f32 {
        Audio_PlaySoundGeneral(0x2819 as libc::c_int as u16_0,
                               &mut (*this).actor.projectedPos,
                               4 as libc::c_int as u8_0, &mut D_801333E0,
                               &mut D_801333E0, &mut D_801333E8);
        func_800AA000(0.0f32, 255 as libc::c_int as u8_0,
                      10 as libc::c_int as u8_0, 80 as libc::c_int as u8_0);
        (*this).stateFlags &=
            !((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint;
        (*this).actor.gravity = -3.5f32;
        (*this).actor.world.pos.y = (*this).actor.floorHeight;
        func_80028A54(globalCtx, 25.0f32, &mut (*this).actor.world.pos);
        EnHorse_JumpLanding(this, globalCtx);
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_HighJumpInit(mut this: *mut EnHorse,
                                              mut globalCtx:
                                                  *mut GlobalContext) {
    (*this).skin.skelAnime.curFrame = 0.0f32;
    EnHorse_StartHighJump(this, globalCtx);
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_StartHighJump(mut this: *mut EnHorse,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    let mut curFrame: f32_0 = 0.;
    let mut jointTable: *mut Vec3s = 0 as *mut Vec3s;
    let mut y: f32_0 = 0.;
    (*this).action = ENHORSE_ACT_HIGH_JUMP;
    (*this).animationIdx = ENHORSE_ANIM_HIGH_JUMP as libc::c_int;
    curFrame = (*this).skin.skelAnime.curFrame;
    Animation_Change(&mut (*this).skin.skelAnime,
                     *sAnimationHeaders[(*this).type_0 as
                                            usize].offset((*this).animationIdx
                                                              as isize),
                     1.5f32, curFrame,
                     Animation_GetLastFrame(*sAnimationHeaders[(*this).type_0
                                                                   as
                                                                   usize].offset((*this).animationIdx
                                                                                     as
                                                                                     isize)
                                                as *mut libc::c_void) as
                         f32_0, ANIMMODE_ONCE as libc::c_int as u8_0,
                     -3.0f32);
    (*this).jumpStartY = (*this).actor.world.pos.y;
    (*this).postDrawFunc = None;
    (*this).actor.gravity = 0 as libc::c_int as f32_0;
    (*this).actor.velocity.y = 0.0f32;
    jointTable = (*this).skin.skelAnime.jointTable;
    y = (*jointTable).y as f32_0;
    (*this).riderPos.y -= y * 0.01f32;
    (*this).stateFlags |=
        ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint;
    Audio_PlaySoundGeneral(0x2818 as libc::c_int as u16_0,
                           &mut (*this).actor.projectedPos,
                           4 as libc::c_int as u8_0, &mut D_801333E0,
                           &mut D_801333E0, &mut D_801333E8);
    func_800AA000(0.0f32, 170 as libc::c_int as u8_0,
                  10 as libc::c_int as u8_0, 10 as libc::c_int as u8_0);
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_Stub2(mut this: *mut EnHorse) { }
#[no_mangle]
pub unsafe extern "C" fn EnHorse_HighJump(mut this: *mut EnHorse,
                                          mut globalCtx: *mut GlobalContext) {
    let mut pad: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut jointTable: *mut Vec3s = 0 as *mut Vec3s;
    let mut curFrame: f32_0 = 0.;
    let mut y: f32_0 = 0.;
    curFrame = (*this).skin.skelAnime.curFrame;
    (*this).stateFlags |=
        ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint;
    (*this).actor.speedXZ = 13.0f32;
    if curFrame > 23.0f32 {
        (*this).actor.gravity = -3.5f32;
        if (*this).actor.velocity.y == 0 as libc::c_int as libc::c_float {
            (*this).actor.velocity.y = -10.5f32
        }
        if (*this).actor.world.pos.y < (*this).actor.floorHeight + 90.0f32 {
            (*this).skin.skelAnime.playSpeed = 1.5f32
        } else {
            (*this).skin.skelAnime.playSpeed = 0 as libc::c_int as f32_0
        }
    } else {
        jointTable = (*this).skin.skelAnime.jointTable;
        y = (*jointTable).y as f32_0;
        (*this).actor.world.pos.y = (*this).jumpStartY + y * 0.01f32
    }
    if SkelAnime_Update(&mut (*this).skin.skelAnime) != 0 ||
           curFrame > 23.0f32 &&
               (*this).actor.world.pos.y <
                   (*this).actor.floorHeight - (*this).actor.velocity.y +
                       80.0f32 {
        Audio_PlaySoundGeneral(0x2819 as libc::c_int as u16_0,
                               &mut (*this).actor.projectedPos,
                               4 as libc::c_int as u8_0, &mut D_801333E0,
                               &mut D_801333E0, &mut D_801333E8);
        func_800AA000(0.0f32, 255 as libc::c_int as u8_0,
                      10 as libc::c_int as u8_0, 80 as libc::c_int as u8_0);
        (*this).stateFlags &=
            !((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint;
        (*this).actor.gravity = -3.5f32;
        (*this).actor.world.pos.y = (*this).actor.floorHeight;
        func_80028A54(globalCtx, 25.0f32, &mut (*this).actor.world.pos);
        EnHorse_JumpLanding(this, globalCtx);
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_InitInactive(mut this: *mut EnHorse) {
    (*this).cyl1.base.ocFlags1 =
        ((*this).cyl1.base.ocFlags1 as libc::c_int &
             !((1 as libc::c_int) << 0 as libc::c_int)) as u8_0;
    (*this).cyl2.base.ocFlags1 =
        ((*this).cyl2.base.ocFlags1 as libc::c_int &
             !((1 as libc::c_int) << 0 as libc::c_int)) as u8_0;
    (*this).jntSph.base.ocFlags1 =
        ((*this).jntSph.base.ocFlags1 as libc::c_int &
             !((1 as libc::c_int) << 0 as libc::c_int)) as u8_0;
    (*this).action = ENHORSE_ACT_INACTIVE;
    (*this).animationIdx = ENHORSE_ANIM_WALK as libc::c_int;
    (*this).stateFlags |=
        ((1 as libc::c_int) << 13 as libc::c_int) as libc::c_uint;
    (*this).followTimer = 0 as libc::c_int as s16;
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_Inactive(mut this: *mut EnHorse,
                                          mut globalCtx2:
                                              *mut GlobalContext) {
    let mut globalCtx: *mut GlobalContext = globalCtx2;
    if (*gGameInfo).data[(7 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 53 as libc::c_int) as usize]
           as libc::c_int != 0 as libc::c_int &&
           (*this).type_0 == HORSE_EPONA as libc::c_int {
        (*gGameInfo).data[(7 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 53 as libc::c_int) as
                              usize] = 0 as libc::c_int as s16;
        if EnHorse_Spawn(this, globalCtx) != 0 as libc::c_int {
            Audio_PlaySoundGeneral(0x2805 as libc::c_int as u16_0,
                                   &mut (*this).actor.projectedPos,
                                   4 as libc::c_int as u8_0, &mut D_801333E0,
                                   &mut D_801333E0, &mut D_801333E8);
            (*this).stateFlags &=
                !((1 as libc::c_int) << 13 as libc::c_int) as libc::c_uint;
            gSaveContext.horseData.scene = (*globalCtx).sceneNum;
            // Focus the camera on Epona
            Camera_SetParam((*globalCtx).cameraPtrs[0 as libc::c_int as
                                                        usize],
                            8 as libc::c_int,
                            this as
                                *mut libc::c_void); // this forces anim 0 to play from the beginning
            Camera_ChangeSetting((*globalCtx).cameraPtrs[0 as libc::c_int as
                                                             usize],
                                 0x38 as libc::c_int as s16);
            Camera_SetCameraData((*globalCtx).cameraPtrs[0 as libc::c_int as
                                                             usize],
                                 4 as libc::c_int as s16,
                                 0 as *mut libc::c_void,
                                 0 as *mut libc::c_void,
                                 0x51 as libc::c_int as s16,
                                 0 as libc::c_int as s16, 0 as libc::c_int);
        }
    }
    if (*this).stateFlags &
           ((1 as libc::c_int) << 13 as libc::c_int) as libc::c_uint == 0 {
        (*this).followTimer = 0 as libc::c_int as s16;
        EnHorse_SetFollowAnimation(this, globalCtx);
        (*this).actor.params = 0 as libc::c_int as s16;
        (*this).cyl1.base.ocFlags1 =
            ((*this).cyl1.base.ocFlags1 as libc::c_int |
                 (1 as libc::c_int) << 0 as libc::c_int) as u8_0;
        (*this).cyl2.base.ocFlags1 =
            ((*this).cyl2.base.ocFlags1 as libc::c_int |
                 (1 as libc::c_int) << 0 as libc::c_int) as u8_0;
        (*this).jntSph.base.ocFlags1 =
            ((*this).jntSph.base.ocFlags1 as libc::c_int |
                 (1 as libc::c_int) << 0 as libc::c_int) as u8_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_PlayIdleAnimation(mut this: *mut EnHorse,
                                                   mut anim: s32,
                                                   mut morphFrames: f32_0,
                                                   mut startFrame: f32_0) {
    (*this).action = ENHORSE_ACT_IDLE;
    (*this).actor.speedXZ = 0.0f32;
    if anim != ENHORSE_ANIM_IDLE as libc::c_int &&
           anim != ENHORSE_ANIM_WHINNEY as libc::c_int &&
           anim != ENHORSE_ANIM_REARING as libc::c_int {
        anim = ENHORSE_ANIM_IDLE as libc::c_int
    }
    if anim != (*this).animationIdx {
        (*this).animationIdx = anim;
        if (*this).animationIdx == ENHORSE_ANIM_IDLE as libc::c_int {
            (*this).stateFlags &=
                !((1 as libc::c_int) << 12 as libc::c_int) as libc::c_uint
        } else if (*this).animationIdx == ENHORSE_ANIM_WHINNEY as libc::c_int
         {
            (*this).unk_21C = (*this).unk_228;
            if (*this).stateFlags &
                   ((1 as libc::c_int) << 27 as libc::c_int) as libc::c_uint
                   != 0 {
                Audio_PlaySoundGeneral(0x2816 as libc::c_int as u16_0,
                                       &mut (*this).unk_21C,
                                       4 as libc::c_int as u8_0,
                                       &mut D_801333E0, &mut D_801333E0,
                                       &mut D_801333E8);
            }
        } else if (*this).animationIdx == ENHORSE_ANIM_REARING as libc::c_int
         {
            (*this).unk_21C = (*this).unk_228;
            if (*this).stateFlags &
                   ((1 as libc::c_int) << 27 as libc::c_int) as libc::c_uint
                   != 0 {
                Audio_PlaySoundGeneral(0x2805 as libc::c_int as u16_0,
                                       &mut (*this).unk_21C,
                                       4 as libc::c_int as u8_0,
                                       &mut D_801333E0, &mut D_801333E0,
                                       &mut D_801333E8);
            }
            (*this).stateFlags &=
                !((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint
        }
        Animation_Change(&mut (*this).skin.skelAnime,
                         *sAnimationHeaders[(*this).type_0 as
                                                usize].offset((*this).animationIdx
                                                                  as isize),
                         1.0f32, startFrame,
                         Animation_GetLastFrame(*sAnimationHeaders[(*this).type_0
                                                                       as
                                                                       usize].offset((*this).animationIdx
                                                                                         as
                                                                                         isize)
                                                    as *mut libc::c_void) as
                             f32_0, ANIMMODE_ONCE as libc::c_int as u8_0,
                         morphFrames);
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_ChangeIdleAnimation(mut this: *mut EnHorse,
                                                     mut anim: s32,
                                                     mut morphFrames: f32_0) {
    EnHorse_PlayIdleAnimation(this, anim, morphFrames, (*this).curFrame);
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_ResetIdleAnimation(mut this: *mut EnHorse) {
    (*this).animationIdx = ENHORSE_ANIM_WALK as libc::c_int;
    EnHorse_PlayIdleAnimation(this, (*this).animationIdx,
                              0 as libc::c_int as f32_0,
                              0 as libc::c_int as f32_0);
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_StartIdleRidable(mut this: *mut EnHorse) {
    EnHorse_ResetIdleAnimation(this);
    (*this).stateFlags &=
        !((1 as libc::c_int) << 16 as libc::c_int) as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_Idle(mut this: *mut EnHorse,
                                      mut globalCtx: *mut GlobalContext) {
    (*this).actor.speedXZ = 0.0f32;
    EnHorse_IdleAnimSounds(this, globalCtx);
    if (*gGameInfo).data[(7 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 53 as libc::c_int) as usize]
           as libc::c_int != 0 && (*this).type_0 == HORSE_EPONA as libc::c_int
       {
        (*gGameInfo).data[(7 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 53 as libc::c_int) as
                              usize] = 0 as libc::c_int as s16;
        if func_80A5BBBC(globalCtx, this, &mut (*this).actor.world.pos) == 0 {
            if EnHorse_Spawn(this, globalCtx) != 0 {
                Audio_PlaySoundGeneral(0x2805 as libc::c_int as u16_0,
                                       &mut (*this).actor.projectedPos,
                                       4 as libc::c_int as u8_0,
                                       &mut D_801333E0, &mut D_801333E0,
                                       &mut D_801333E8);
                (*this).followTimer = 0 as libc::c_int as s16;
                EnHorse_SetFollowAnimation(this, globalCtx);
                Camera_SetParam((*globalCtx).cameraPtrs[0 as libc::c_int as
                                                            usize],
                                8 as libc::c_int, this as *mut libc::c_void);
                Camera_ChangeSetting((*globalCtx).cameraPtrs[0 as libc::c_int
                                                                 as usize],
                                     0x38 as libc::c_int as s16);
                Camera_SetCameraData((*globalCtx).cameraPtrs[0 as libc::c_int
                                                                 as usize],
                                     4 as libc::c_int as s16,
                                     0 as *mut libc::c_void,
                                     0 as *mut libc::c_void,
                                     0x51 as libc::c_int as s16,
                                     0 as libc::c_int as s16,
                                     0 as libc::c_int);
            }
        } else {
            Audio_PlaySoundGeneral(0x2805 as libc::c_int as u16_0,
                                   &mut (*this).actor.projectedPos,
                                   4 as libc::c_int as u8_0, &mut D_801333E0,
                                   &mut D_801333E0, &mut D_801333E8);
            (*this).followTimer = 0 as libc::c_int as s16;
            EnHorse_StartMovingAnimation(this, 6 as libc::c_int, -3.0f32,
                                         0.0f32);
        }
    }
    if SkelAnime_Update(&mut (*this).skin.skelAnime) != 0 {
        let mut idleAnimIdx: s32 = 0 as libc::c_int;
        if (*this).animationIdx != ENHORSE_ANIM_IDLE as libc::c_int {
            if (*this).animationIdx == ENHORSE_ANIM_WHINNEY as libc::c_int {
                idleAnimIdx = 1 as libc::c_int
            } else if (*this).animationIdx ==
                          ENHORSE_ANIM_REARING as libc::c_int {
                idleAnimIdx = 2 as libc::c_int
            }
        }
        // Play one of the two other idle animations
        EnHorse_PlayIdleAnimation(this,
                                  sIdleAnimIds[((if Rand_ZeroOne() > 0.5f32 {
                                                     0 as libc::c_int
                                                 } else { 1 as libc::c_int })
                                                    +
                                                    idleAnimIdx *
                                                        2 as libc::c_int) as
                                                   usize], 0.0f32, 0.0f32);
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_StartMovingAnimation(mut this: *mut EnHorse,
                                                      mut animId: s32,
                                                      mut morphFrames: f32_0,
                                                      mut startFrame: f32_0) {
    (*this).action = ENHORSE_ACT_FOLLOW_PLAYER;
    (*this).stateFlags &=
        !((1 as libc::c_int) << 15 as libc::c_int) as libc::c_uint;
    if animId != ENHORSE_ANIM_TROT as libc::c_int &&
           animId != ENHORSE_ANIM_GALLOP as libc::c_int &&
           animId != ENHORSE_ANIM_WALK as libc::c_int {
        animId = ENHORSE_ANIM_WALK as libc::c_int
    }
    if (*this).animationIdx != animId {
        (*this).animationIdx = animId;
        Animation_Change(&mut (*this).skin.skelAnime,
                         *sAnimationHeaders[(*this).type_0 as
                                                usize].offset((*this).animationIdx
                                                                  as isize),
                         1.0f32, startFrame,
                         Animation_GetLastFrame(*sAnimationHeaders[(*this).type_0
                                                                       as
                                                                       usize].offset((*this).animationIdx
                                                                                         as
                                                                                         isize)
                                                    as *mut libc::c_void) as
                             f32_0, ANIMMODE_ONCE as libc::c_int as u8_0,
                         morphFrames);
    } else {
        Animation_Change(&mut (*this).skin.skelAnime,
                         *sAnimationHeaders[(*this).type_0 as
                                                usize].offset((*this).animationIdx
                                                                  as isize),
                         1.0f32, startFrame,
                         Animation_GetLastFrame(*sAnimationHeaders[(*this).type_0
                                                                       as
                                                                       usize].offset((*this).animationIdx
                                                                                         as
                                                                                         isize)
                                                    as *mut libc::c_void) as
                             f32_0, ANIMMODE_ONCE as libc::c_int as u8_0,
                         0.0f32);
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_SetFollowAnimation(mut this: *mut EnHorse,
                                                    mut globalCtx:
                                                        *mut GlobalContext) {
    let mut animId: s32 = ENHORSE_ANIM_WALK as libc::c_int;
    let mut distToPlayer: f32_0 = 0.;
    distToPlayer =
        Actor_WorldDistXZToActor(&mut (*this).actor,
                                 &mut (*((*(*globalCtx).actorCtx.actorLists.as_mut_ptr().offset(ACTORCAT_PLAYER
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    isize)).head
                                             as *mut Player)).actor);
    if distToPlayer > 400.0f32 {
        animId = ENHORSE_ANIM_GALLOP as libc::c_int
    } else if !(distToPlayer <= 300.0f32) {
        if distToPlayer <= 400.0f32 {
            animId = ENHORSE_ANIM_TROT as libc::c_int
        }
    }
    if (*this).animationIdx == ENHORSE_ANIM_GALLOP as libc::c_int {
        if distToPlayer > 400.0f32 {
            animId = ENHORSE_ANIM_GALLOP as libc::c_int
        } else { animId = ENHORSE_ANIM_TROT as libc::c_int }
    } else if (*this).animationIdx == ENHORSE_ANIM_TROT as libc::c_int {
        if distToPlayer > 400.0f32 {
            animId = ENHORSE_ANIM_GALLOP as libc::c_int
        } else if distToPlayer < 300.0f32 {
            animId = ENHORSE_ANIM_WALK as libc::c_int
        } else { animId = ENHORSE_ANIM_TROT as libc::c_int }
    } else if (*this).animationIdx == ENHORSE_ANIM_WALK as libc::c_int {
        if distToPlayer > 300.0f32 {
            animId = ENHORSE_ANIM_TROT as libc::c_int
        } else { animId = ENHORSE_ANIM_WALK as libc::c_int }
    }
    EnHorse_StartMovingAnimation(this, animId, -3.0f32, 0.0f32);
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_FollowPlayer(mut this: *mut EnHorse,
                                              mut globalCtx:
                                                  *mut GlobalContext) {
    let mut distToPlayer: f32_0 = 0.;
    let mut angleDiff: f32_0 = 0.;
    (*gGameInfo).data[(7 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 53 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    distToPlayer =
        Actor_WorldDistXZToActor(&mut (*this).actor,
                                 &mut (*((*(*globalCtx).actorCtx.actorLists.as_mut_ptr().offset(ACTORCAT_PLAYER
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    isize)).head
                                             as *mut Player)).actor);
    // First rotate if the player is behind
    if ((*this).playerDir == PLAYER_DIR_BACK_R as libc::c_int as libc::c_uint
            ||
            (*this).playerDir ==
                PLAYER_DIR_BACK_L as libc::c_int as libc::c_uint) &&
           (distToPlayer > 300.0f32 &&
                (*this).stateFlags &
                    ((1 as libc::c_int) << 15 as libc::c_int) as libc::c_uint
                    == 0) {
        (*this).animationIdx =
            ENHORSE_ANIM_REARING as libc::c_int; // prevents unrolling
        (*this).stateFlags |=
            ((1 as libc::c_int) << 15 as libc::c_int) as libc::c_uint;
        (*this).angleToPlayer =
            Actor_WorldYawTowardActor(&mut (*this).actor,
                                      &mut (*((*(*globalCtx).actorCtx.actorLists.as_mut_ptr().offset(ACTORCAT_PLAYER
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         isize)).head
                                                  as *mut Player)).actor);
        angleDiff =
            (*this).angleToPlayer as f32_0 -
                (*this).actor.world.rot.y as f32_0;
        if angleDiff > 32767.0f32 {
            angleDiff -= 32767.0f32
        } else if angleDiff < -(32767 as libc::c_int) as libc::c_float {
            angleDiff += 32767 as libc::c_int as libc::c_float
        }
        (*this).followPlayerTurnSpeed =
            (angleDiff /
                 Animation_GetLastFrame(*sAnimationHeaders[(*this).type_0 as
                                                               usize].offset((*this).animationIdx
                                                                                 as
                                                                                 isize)
                                            as *mut libc::c_void) as
                     libc::c_int as libc::c_float) as s16;
        Animation_PlayOnce(&mut (*this).skin.skelAnime,
                           *sAnimationHeaders[(*this).type_0 as
                                                  usize].offset((*this).animationIdx
                                                                    as
                                                                    isize));
        (*this).skin.skelAnime.playSpeed = 1.0f32;
        (*this).stateFlags &=
            !((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint;
        (*this).unk_21C = (*this).unk_228
    } else if (*this).stateFlags &
                  ((1 as libc::c_int) << 15 as libc::c_int) as libc::c_uint !=
                  0 {
        (*this).actor.world.rot.y =
            ((*this).actor.world.rot.y as libc::c_int +
                 (*this).followPlayerTurnSpeed as libc::c_int) as s16;
        (*this).actor.shape.rot.y = (*this).actor.world.rot.y;
        if (*this).curFrame > 25.0f32 {
            if (*this).stateFlags &
                   ((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint
                   == 0 {
                (*this).stateFlags |=
                    ((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint;
                Audio_PlaySoundGeneral(0x282b as libc::c_int as u16_0,
                                       &mut (*this).actor.projectedPos,
                                       4 as libc::c_int as u8_0,
                                       &mut D_801333E0, &mut D_801333E0,
                                       &mut D_801333E8);
            }
        }
    } else { EnHorse_RotateToPlayer(this, globalCtx); }
    if (*this).animationIdx == ENHORSE_ANIM_GALLOP as libc::c_int {
        (*this).actor.speedXZ = 8 as libc::c_int as f32_0;
        (*this).skin.skelAnime.playSpeed = (*this).actor.speedXZ * 0.3f32
    } else if (*this).animationIdx == ENHORSE_ANIM_TROT as libc::c_int {
        (*this).actor.speedXZ = 6 as libc::c_int as f32_0;
        (*this).skin.skelAnime.playSpeed = (*this).actor.speedXZ * 0.375f32
    } else if (*this).animationIdx == ENHORSE_ANIM_WALK as libc::c_int {
        (*this).actor.speedXZ = 3 as libc::c_int as f32_0;
        EnHorse_PlayWalkingSound(this);
        (*this).skin.skelAnime.playSpeed = (*this).actor.speedXZ * 0.75f32
    } else {
        (*this).actor.speedXZ = 0 as libc::c_int as f32_0;
        (*this).skin.skelAnime.playSpeed = 1.0f32
    }
    if (*this).stateFlags &
           ((1 as libc::c_int) << 15 as libc::c_int) as libc::c_uint == 0 &&
           {
               (*this).followTimer += 1;
               ((*this).followTimer as libc::c_int) > 300 as libc::c_int
           } {
        EnHorse_StartIdleRidable(this);
        (*this).unk_21C = (*this).unk_228;
        if (*this).stateFlags &
               ((1 as libc::c_int) << 27 as libc::c_int) as libc::c_uint != 0
           {
            Audio_PlaySoundGeneral(0x2805 as libc::c_int as u16_0,
                                   &mut (*this).unk_21C,
                                   4 as libc::c_int as u8_0, &mut D_801333E0,
                                   &mut D_801333E0, &mut D_801333E8);
        }
    }
    if SkelAnime_Update(&mut (*this).skin.skelAnime) != 0 {
        if (*this).animationIdx == ENHORSE_ANIM_GALLOP as libc::c_int {
            EnHorse_PlayGallopingSound(this);
        } else if (*this).animationIdx == ENHORSE_ANIM_TROT as libc::c_int {
            EnHorse_PlayTrottingSound(this);
        }
        (*this).stateFlags &=
            !((1 as libc::c_int) << 15 as libc::c_int) as libc::c_uint;
        if distToPlayer < 100.0f32 {
            EnHorse_StartIdleRidable(this);
        } else { EnHorse_SetFollowAnimation(this, globalCtx); }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_InitIngoHorse(mut this: *mut EnHorse) {
    (*this).curRaceWaypoint = 0 as libc::c_int;
    (*this).soundTimer = 0 as libc::c_int;
    (*this).actor.speedXZ = 0.0f32;
    EnHorse_UpdateIngoHorseAnim(this);
    (*this).unk_21C = (*this).unk_228;
    if (*this).stateFlags &
           ((1 as libc::c_int) << 27 as libc::c_int) as libc::c_uint != 0 {
        Audio_PlaySoundGeneral(0x1844 as libc::c_int as u16_0,
                               &mut (*this).unk_21C, 4 as libc::c_int as u8_0,
                               &mut D_801333E0, &mut D_801333E0,
                               &mut D_801333E8);
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_SetIngoAnimation(mut idx: s32,
                                                  mut curFrame: f32_0,
                                                  mut arg2: s32,
                                                  mut animIdxOut: *mut s16,
                                                  mut curFrameOut:
                                                      *mut f32_0) {
    *animIdxOut = sIngoAnimations[idx as usize];
    *curFrameOut = curFrame;
    if idx == 3 as libc::c_int || idx == 7 as libc::c_int ||
           idx == 8 as libc::c_int || idx == 4 as libc::c_int {
        *curFrameOut = 0.0f32
    }
    if arg2 == 1 as libc::c_int {
        if idx == 5 as libc::c_int {
            *animIdxOut = 4 as libc::c_int as s16;
            *curFrameOut = curFrame
        } else if idx == 6 as libc::c_int {
            *animIdxOut = 3 as libc::c_int as s16;
            *curFrameOut = curFrame
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_UpdateIngoHorseAnim(mut this: *mut EnHorse) {
    let mut animChanged: s32 = 0 as libc::c_int;
    let mut animSpeed: f32_0 = 0.;
    (*this).action = ENHORSE_ACT_INGO_RACE;
    (*this).stateFlags &=
        !((1 as libc::c_int) << 12 as libc::c_int) as libc::c_uint;
    if (*this).actor.speedXZ == 0.0f32 {
        if (*this).animationIdx != ENHORSE_ANIM_IDLE as libc::c_int {
            animChanged = 1 as libc::c_int
        }
        (*this).animationIdx = ENHORSE_ANIM_IDLE as libc::c_int
    } else if (*this).actor.speedXZ <= 3.0f32 {
        if (*this).animationIdx != ENHORSE_ANIM_WALK as libc::c_int {
            animChanged = 1 as libc::c_int
        }
        (*this).animationIdx = ENHORSE_ANIM_WALK as libc::c_int
    } else if (*this).actor.speedXZ <= 6.0f32 {
        if (*this).animationIdx != ENHORSE_ANIM_TROT as libc::c_int {
            animChanged = 1 as libc::c_int
        }
        (*this).animationIdx = ENHORSE_ANIM_TROT as libc::c_int
    } else {
        if (*this).animationIdx != ENHORSE_ANIM_GALLOP as libc::c_int {
            animChanged = 1 as libc::c_int
        }
        (*this).animationIdx = ENHORSE_ANIM_GALLOP as libc::c_int
    }
    if (*this).animationIdx == ENHORSE_ANIM_WALK as libc::c_int {
        animSpeed = (*this).actor.speedXZ * 0.5f32
    } else if (*this).animationIdx == ENHORSE_ANIM_TROT as libc::c_int {
        animSpeed = (*this).actor.speedXZ * 0.25f32;
        Audio_PlaySoundGeneral(0x2804 as libc::c_int as u16_0,
                               &mut (*this).actor.projectedPos,
                               4 as libc::c_int as u8_0, &mut D_801333E0,
                               &mut D_801333E0, &mut D_801333E8);
    } else if (*this).animationIdx == ENHORSE_ANIM_GALLOP as libc::c_int {
        animSpeed = (*this).actor.speedXZ * 0.2f32;
        Audio_PlaySoundGeneral(0x2804 as libc::c_int as u16_0,
                               &mut (*this).actor.projectedPos,
                               4 as libc::c_int as u8_0, &mut D_801333E0,
                               &mut D_801333E0, &mut D_801333E8);
    } else { animSpeed = 1.0f32 }
    if animChanged == 1 as libc::c_int {
        Animation_Change(&mut (*this).skin.skelAnime,
                         *sAnimationHeaders[(*this).type_0 as
                                                usize].offset((*this).animationIdx
                                                                  as isize),
                         sPlaybackSpeeds[(*this).animationIdx as usize] *
                             animSpeed * 1.5f32, 0 as libc::c_int as f32_0,
                         Animation_GetLastFrame(*sAnimationHeaders[(*this).type_0
                                                                       as
                                                                       usize].offset((*this).animationIdx
                                                                                         as
                                                                                         isize)
                                                    as *mut libc::c_void) as
                             f32_0, ANIMMODE_ONCE as libc::c_int as u8_0,
                         -(3 as libc::c_int) as f32_0);
    } else {
        Animation_Change(&mut (*this).skin.skelAnime,
                         *sAnimationHeaders[(*this).type_0 as
                                                usize].offset((*this).animationIdx
                                                                  as isize),
                         sPlaybackSpeeds[(*this).animationIdx as usize] *
                             animSpeed * 1.5f32, 0 as libc::c_int as f32_0,
                         Animation_GetLastFrame(*sAnimationHeaders[(*this).type_0
                                                                       as
                                                                       usize].offset((*this).animationIdx
                                                                                         as
                                                                                         isize)
                                                    as *mut libc::c_void) as
                             f32_0, ANIMMODE_ONCE as libc::c_int as u8_0,
                         0 as libc::c_int as f32_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_UpdateIngoRace(mut this: *mut EnHorse,
                                                mut globalCtx:
                                                    *mut GlobalContext) {
    let mut playSpeed: f32_0 = 0.;
    if (*this).animationIdx == ENHORSE_ANIM_IDLE as libc::c_int ||
           (*this).animationIdx == ENHORSE_ANIM_WHINNEY as libc::c_int {
        EnHorse_IdleAnimSounds(this, globalCtx);
    } else if (*this).animationIdx == ENHORSE_ANIM_WALK as libc::c_int {
        EnHorse_PlayWalkingSound(this);
    }
    EnHorse_UpdateIngoRaceInfo(this, globalCtx, &mut sIngoRace);
    if (*this).inRace == 0 {
        (*this).actor.speedXZ = 0.0f32;
        (*(*this).rider).speedXZ = 0.0f32;
        if (*this).animationIdx != ENHORSE_ANIM_IDLE as libc::c_int {
            EnHorse_UpdateIngoHorseAnim(this);
        }
    }
    if (*this).animationIdx == ENHORSE_ANIM_WALK as libc::c_int {
        playSpeed = (*this).actor.speedXZ * 0.5f32
    } else if (*this).animationIdx == ENHORSE_ANIM_TROT as libc::c_int {
        playSpeed = (*this).actor.speedXZ * 0.25f32
    } else if (*this).animationIdx == ENHORSE_ANIM_GALLOP as libc::c_int {
        playSpeed = (*this).actor.speedXZ * 0.2f32
    } else { playSpeed = 1.0f32 }
    (*this).skin.skelAnime.playSpeed = playSpeed;
    if SkelAnime_Update(&mut (*this).skin.skelAnime) != 0 ||
           (*this).animationIdx == ENHORSE_ANIM_IDLE as libc::c_int &&
               (*this).actor.speedXZ != 0.0f32 {
        EnHorse_UpdateIngoHorseAnim(this);
    }
    if (*this).stateFlags &
           ((1 as libc::c_int) << 23 as libc::c_int) as libc::c_uint != 0 {
        (*((*this).rider as *mut EnIn)).animationIdx =
            7 as libc::c_int as s16;
        (*((*this).rider as *mut EnIn)).unk_1E0 = 0 as libc::c_int as f32_0;
        return
    }
    EnHorse_SetIngoAnimation((*this).animationIdx,
                             (*this).skin.skelAnime.curFrame,
                             (*this).ingoRaceFlags as libc::c_int &
                                 1 as libc::c_int,
                             &mut (*((*this).rider as
                                         *mut EnIn)).animationIdx,
                             &mut (*((*this).rider as *mut EnIn)).unk_1E0);
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_CsMoveInit(mut this: *mut EnHorse,
                                            mut globalCtx: *mut GlobalContext,
                                            mut action:
                                                *mut CsCmdActorAction) {
    (*this).animationIdx = ENHORSE_ANIM_GALLOP as libc::c_int;
    (*this).cutsceneAction = 1 as libc::c_int;
    Animation_PlayOnceSetSpeed(&mut (*this).skin.skelAnime,
                               *sAnimationHeaders[(*this).type_0 as
                                                      usize].offset((*this).animationIdx
                                                                        as
                                                                        isize),
                               (*this).actor.speedXZ * 0.3f32);
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_CsMoveToPoint(mut this: *mut EnHorse,
                                               mut globalCtx:
                                                   *mut GlobalContext,
                                               mut action:
                                                   *mut CsCmdActorAction) {
    let mut endPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut speed: f32_0 = 8.0f32;
    endPos.x = (*action).endPos.x as f32_0;
    endPos.y = (*action).endPos.y as f32_0;
    endPos.z = (*action).endPos.z as f32_0;
    if Math3D_Vec3f_DistXYZ(&mut endPos, &mut (*this).actor.world.pos) > speed
       {
        EnHorse_RotateToPoint(this, globalCtx, &mut endPos,
                              400 as libc::c_int as s16);
        (*this).actor.speedXZ = speed;
        (*this).skin.skelAnime.playSpeed = speed * 0.3f32
    } else {
        (*this).actor.world.pos = endPos;
        (*this).actor.speedXZ = 0.0f32
    }
    if SkelAnime_Update(&mut (*this).skin.skelAnime) != 0 {
        EnHorse_PlayGallopingSound(this);
        func_800AA000(0.0f32, 120 as libc::c_int as u8_0,
                      8 as libc::c_int as u8_0, 255 as libc::c_int as u8_0);
        Animation_PlayOnceSetSpeed(&mut (*this).skin.skelAnime,
                                   *sAnimationHeaders[(*this).type_0 as
                                                          usize].offset((*this).animationIdx
                                                                            as
                                                                            isize),
                                   (*this).actor.speedXZ * 0.3f32);
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_CsSetAnimHighJump(mut this: *mut EnHorse,
                                                   mut globalCtx:
                                                       *mut GlobalContext) {
    (*this).skin.skelAnime.curFrame = 0.0f32;
    EnHorse_CsPlayHighJumpAnim(this, globalCtx);
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_CsPlayHighJumpAnim(mut this: *mut EnHorse,
                                                    mut globalCtx:
                                                        *mut GlobalContext) {
    let mut curFrame: f32_0 = 0.;
    let mut y: f32_0 = 0.;
    let mut jointTable: *mut Vec3s = 0 as *mut Vec3s;
    (*this).animationIdx = ENHORSE_ANIM_HIGH_JUMP as libc::c_int;
    curFrame = (*this).skin.skelAnime.curFrame;
    Animation_Change(&mut (*this).skin.skelAnime,
                     *sAnimationHeaders[(*this).type_0 as
                                            usize].offset((*this).animationIdx
                                                              as isize),
                     1.5f32, curFrame,
                     Animation_GetLastFrame(*sAnimationHeaders[(*this).type_0
                                                                   as
                                                                   usize].offset((*this).animationIdx
                                                                                     as
                                                                                     isize)
                                                as *mut libc::c_void) as
                         f32_0, ANIMMODE_ONCE as libc::c_int as u8_0,
                     -3.0f32);
    (*this).postDrawFunc = None;
    (*this).jumpStartY = (*this).actor.world.pos.y;
    (*this).actor.gravity = 0.0f32;
    (*this).actor.velocity.y = 0 as libc::c_int as f32_0;
    jointTable = (*this).skin.skelAnime.jointTable;
    y = (*jointTable).y as f32_0;
    (*this).riderPos.y -= y * 0.01f32;
    (*this).stateFlags |=
        ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint;
    Audio_PlaySoundGeneral(0x2818 as libc::c_int as u16_0,
                           &mut (*this).actor.projectedPos,
                           4 as libc::c_int as u8_0, &mut D_801333E0,
                           &mut D_801333E0, &mut D_801333E8);
    func_800AA000(0.0f32, 170 as libc::c_int as u8_0,
                  10 as libc::c_int as u8_0, 10 as libc::c_int as u8_0);
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_CsJumpInit(mut this: *mut EnHorse,
                                            mut globalCtx: *mut GlobalContext,
                                            mut action:
                                                *mut CsCmdActorAction) {
    EnHorse_CsSetAnimHighJump(this, globalCtx);
    (*this).cutsceneAction = 2 as libc::c_int;
    (*this).cutsceneFlags =
        ((*this).cutsceneFlags as libc::c_int & !(1 as libc::c_int)) as u16_0;
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_CsJump(mut this: *mut EnHorse,
                                        mut globalCtx: *mut GlobalContext,
                                        mut action: *mut CsCmdActorAction) {
    let mut temp_f2: f32_0 = 0.;
    if (*this).cutsceneFlags as libc::c_int & 1 as libc::c_int != 0 {
        EnHorse_CsMoveToPoint(this, globalCtx, action);
        return
    }
    temp_f2 = (*this).skin.skelAnime.curFrame;
    (*this).stateFlags |=
        ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint;
    (*this).actor.speedXZ = 13.0f32;
    if temp_f2 > 19.0f32 {
        (*this).actor.gravity = -3.5f32;
        if (*this).actor.velocity.y == 0.0f32 {
            (*this).actor.velocity.y = -10.5f32
        }
        if (*this).actor.world.pos.y < (*this).actor.floorHeight + 90.0f32 {
            (*this).skin.skelAnime.playSpeed = 1.5f32
        } else { (*this).skin.skelAnime.playSpeed = 0.0f32 }
    } else {
        let mut jointTable: *mut Vec3s = 0 as *mut Vec3s;
        let mut y: f32_0 = 0.;
        jointTable = (*this).skin.skelAnime.jointTable;
        y = (*jointTable).y as f32_0;
        (*this).actor.world.pos.y = (*this).jumpStartY + y * 0.01f32
    }
    if SkelAnime_Update(&mut (*this).skin.skelAnime) != 0 ||
           temp_f2 > 19.0f32 &&
               (*this).actor.world.pos.y <
                   (*this).actor.floorHeight - (*this).actor.velocity.y +
                       80.0f32 {
        let mut jointTable_0: *mut Vec3s = 0 as *mut Vec3s;
        let mut y_0: f32_0 = 0.;
        (*this).cutsceneFlags =
            ((*this).cutsceneFlags as libc::c_int | 1 as libc::c_int) as
                u16_0;
        Audio_PlaySoundGeneral(0x2819 as libc::c_int as u16_0,
                               &mut (*this).actor.projectedPos,
                               4 as libc::c_int as u8_0, &mut D_801333E0,
                               &mut D_801333E0, &mut D_801333E8);
        func_800AA000(0.0f32, 255 as libc::c_int as u8_0,
                      10 as libc::c_int as u8_0, 80 as libc::c_int as u8_0);
        (*this).stateFlags &=
            !((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint;
        (*this).actor.gravity = -3.5f32;
        (*this).actor.velocity.y = 0 as libc::c_int as f32_0;
        (*this).actor.world.pos.y = (*this).actor.floorHeight;
        func_80028A54(globalCtx, 25.0f32, &mut (*this).actor.world.pos);
        (*this).animationIdx = ENHORSE_ANIM_GALLOP as libc::c_int;
        Animation_PlayOnceSetSpeed(&mut (*this).skin.skelAnime,
                                   *sAnimationHeaders[(*this).type_0 as
                                                          usize].offset((*this).animationIdx
                                                                            as
                                                                            isize),
                                   sPlaybackSpeeds[6 as libc::c_int as
                                                       usize]);
        jointTable_0 = (*this).skin.skelAnime.jointTable;
        y_0 = (*jointTable_0).y as f32_0;
        (*this).riderPos.y += y_0 * 0.01f32;
        (*this).postDrawFunc = None
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_CsRearingInit(mut this: *mut EnHorse,
                                               mut globalCtx:
                                                   *mut GlobalContext,
                                               mut action:
                                                   *mut CsCmdActorAction) {
    (*this).animationIdx = ENHORSE_ANIM_REARING as libc::c_int;
    (*this).cutsceneAction = 3 as libc::c_int;
    (*this).cutsceneFlags =
        ((*this).cutsceneFlags as libc::c_int & !(4 as libc::c_int)) as u16_0;
    (*this).stateFlags &=
        !((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint;
    (*this).unk_21C = (*this).unk_228;
    if (*this).stateFlags &
           ((1 as libc::c_int) << 27 as libc::c_int) as libc::c_uint != 0 {
        Audio_PlaySoundGeneral(0x2805 as libc::c_int as u16_0,
                               &mut (*this).unk_21C, 4 as libc::c_int as u8_0,
                               &mut D_801333E0, &mut D_801333E0,
                               &mut D_801333E8);
    }
    Animation_Change(&mut (*this).skin.skelAnime,
                     *sAnimationHeaders[(*this).type_0 as
                                            usize].offset((*this).animationIdx
                                                              as isize),
                     1.0f32, 0.0f32,
                     Animation_GetLastFrame(*sAnimationHeaders[(*this).type_0
                                                                   as
                                                                   usize].offset((*this).animationIdx
                                                                                     as
                                                                                     isize)
                                                as *mut libc::c_void) as
                         f32_0, ANIMMODE_ONCE as libc::c_int as u8_0,
                     -3.0f32);
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_CsRearing(mut this: *mut EnHorse,
                                           mut globalCtx: *mut GlobalContext,
                                           mut action:
                                               *mut CsCmdActorAction) {
    (*this).actor.speedXZ = 0.0f32;
    if (*this).curFrame > 25.0f32 {
        if (*this).stateFlags &
               ((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint == 0
           {
            (*this).stateFlags |=
                ((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint;
            Audio_PlaySoundGeneral(0x282b as libc::c_int as u16_0,
                                   &mut (*this).actor.projectedPos,
                                   4 as libc::c_int as u8_0, &mut D_801333E0,
                                   &mut D_801333E0, &mut D_801333E8);
        }
    }
    if SkelAnime_Update(&mut (*this).skin.skelAnime) != 0 {
        (*this).animationIdx = ENHORSE_ANIM_IDLE as libc::c_int;
        if (*this).cutsceneFlags as libc::c_int & 4 as libc::c_int == 0 {
            (*this).cutsceneFlags =
                ((*this).cutsceneFlags as libc::c_int | 4 as libc::c_int) as
                    u16_0;
            Animation_Change(&mut (*this).skin.skelAnime,
                             *sAnimationHeaders[(*this).type_0 as
                                                    usize].offset((*this).animationIdx
                                                                      as
                                                                      isize),
                             1.0f32, 0.0f32,
                             Animation_GetLastFrame(*sAnimationHeaders[(*this).type_0
                                                                           as
                                                                           usize].offset((*this).animationIdx
                                                                                             as
                                                                                             isize)
                                                        as *mut libc::c_void)
                                 as f32_0,
                             ANIMMODE_ONCE as libc::c_int as u8_0, -3.0f32);
        } else {
            Animation_Change(&mut (*this).skin.skelAnime,
                             *sAnimationHeaders[(*this).type_0 as
                                                    usize].offset((*this).animationIdx
                                                                      as
                                                                      isize),
                             1.0f32, 0.0f32,
                             Animation_GetLastFrame(*sAnimationHeaders[(*this).type_0
                                                                           as
                                                                           usize].offset((*this).animationIdx
                                                                                             as
                                                                                             isize)
                                                        as *mut libc::c_void)
                                 as f32_0, 0 as libc::c_int as u8_0, 0.0f32);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_WarpMoveInit(mut this: *mut EnHorse,
                                              mut globalCtx:
                                                  *mut GlobalContext,
                                              mut action:
                                                  *mut CsCmdActorAction) {
    (*this).actor.world.pos.x = (*action).startPos.x as f32_0;
    (*this).actor.world.pos.y = (*action).startPos.y as f32_0;
    (*this).actor.world.pos.z = (*action).startPos.z as f32_0;
    (*this).actor.prevPos = (*this).actor.world.pos;
    (*this).actor.world.rot.y = (*action).c2rust_unnamed.urot.y as s16;
    (*this).actor.shape.rot = (*this).actor.world.rot;
    (*this).animationIdx = ENHORSE_ANIM_GALLOP as libc::c_int;
    (*this).cutsceneAction = 4 as libc::c_int;
    Animation_PlayOnceSetSpeed(&mut (*this).skin.skelAnime,
                               *sAnimationHeaders[(*this).type_0 as
                                                      usize].offset((*this).animationIdx
                                                                        as
                                                                        isize),
                               (*this).actor.speedXZ * 0.3f32);
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_CsWarpMoveToPoint(mut this: *mut EnHorse,
                                                   mut globalCtx:
                                                       *mut GlobalContext,
                                                   mut action:
                                                       *mut CsCmdActorAction) {
    let mut endPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut speed: f32_0 = 8.0f32;
    endPos.x = (*action).endPos.x as f32_0;
    endPos.y = (*action).endPos.y as f32_0;
    endPos.z = (*action).endPos.z as f32_0;
    if Math3D_Vec3f_DistXYZ(&mut endPos, &mut (*this).actor.world.pos) > speed
       {
        EnHorse_RotateToPoint(this, globalCtx, &mut endPos,
                              400 as libc::c_int as s16);
        (*this).actor.speedXZ = speed;
        (*this).skin.skelAnime.playSpeed = speed * 0.3f32
    } else {
        (*this).actor.world.pos = endPos;
        (*this).actor.speedXZ = 0.0f32
    }
    if SkelAnime_Update(&mut (*this).skin.skelAnime) != 0 {
        EnHorse_PlayGallopingSound(this);
        func_800AA000(0.0f32, 120 as libc::c_int as u8_0,
                      8 as libc::c_int as u8_0, 255 as libc::c_int as u8_0);
        Animation_PlayOnceSetSpeed(&mut (*this).skin.skelAnime,
                                   *sAnimationHeaders[(*this).type_0 as
                                                          usize].offset((*this).animationIdx
                                                                            as
                                                                            isize),
                                   (*this).actor.speedXZ * 0.3f32);
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_CsWarpRearingInit(mut this: *mut EnHorse,
                                                   mut globalCtx:
                                                       *mut GlobalContext,
                                                   mut action:
                                                       *mut CsCmdActorAction) {
    (*this).actor.world.pos.x = (*action).startPos.x as f32_0;
    (*this).actor.world.pos.y = (*action).startPos.y as f32_0;
    (*this).actor.world.pos.z = (*action).startPos.z as f32_0;
    (*this).actor.prevPos = (*this).actor.world.pos;
    (*this).actor.world.rot.y = (*action).c2rust_unnamed.urot.y as s16;
    (*this).actor.shape.rot = (*this).actor.world.rot;
    (*this).animationIdx = ENHORSE_ANIM_REARING as libc::c_int;
    (*this).cutsceneAction = 5 as libc::c_int;
    (*this).cutsceneFlags =
        ((*this).cutsceneFlags as libc::c_int & !(4 as libc::c_int)) as u16_0;
    (*this).stateFlags &=
        !((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint;
    (*this).unk_21C = (*this).unk_228;
    if (*this).stateFlags &
           ((1 as libc::c_int) << 27 as libc::c_int) as libc::c_uint != 0 {
        Audio_PlaySoundGeneral(0x2805 as libc::c_int as u16_0,
                               &mut (*this).unk_21C, 4 as libc::c_int as u8_0,
                               &mut D_801333E0, &mut D_801333E0,
                               &mut D_801333E8);
    }
    Animation_Change(&mut (*this).skin.skelAnime,
                     *sAnimationHeaders[(*this).type_0 as
                                            usize].offset((*this).animationIdx
                                                              as isize),
                     1.0f32, 0.0f32,
                     Animation_GetLastFrame(*sAnimationHeaders[(*this).type_0
                                                                   as
                                                                   usize].offset((*this).animationIdx
                                                                                     as
                                                                                     isize)
                                                as *mut libc::c_void) as
                         f32_0, ANIMMODE_ONCE as libc::c_int as u8_0,
                     -3.0f32);
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_CsWarpRearing(mut this: *mut EnHorse,
                                               mut globalCtx:
                                                   *mut GlobalContext,
                                               mut action:
                                                   *mut CsCmdActorAction) {
    (*this).actor.speedXZ = 0.0f32;
    if (*this).curFrame > 25.0f32 {
        if (*this).stateFlags &
               ((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint == 0
           {
            (*this).stateFlags |=
                ((1 as libc::c_int) << 11 as libc::c_int) as libc::c_uint;
            Audio_PlaySoundGeneral(0x282b as libc::c_int as u16_0,
                                   &mut (*this).actor.projectedPos,
                                   4 as libc::c_int as u8_0, &mut D_801333E0,
                                   &mut D_801333E0, &mut D_801333E8);
        }
    }
    if SkelAnime_Update(&mut (*this).skin.skelAnime) != 0 {
        (*this).animationIdx = ENHORSE_ANIM_IDLE as libc::c_int;
        if (*this).cutsceneFlags as libc::c_int & 4 as libc::c_int == 0 {
            (*this).cutsceneFlags =
                ((*this).cutsceneFlags as libc::c_int | 4 as libc::c_int) as
                    u16_0;
            Animation_Change(&mut (*this).skin.skelAnime,
                             *sAnimationHeaders[(*this).type_0 as
                                                    usize].offset((*this).animationIdx
                                                                      as
                                                                      isize),
                             1.0f32, 0.0f32,
                             Animation_GetLastFrame(*sAnimationHeaders[(*this).type_0
                                                                           as
                                                                           usize].offset((*this).animationIdx
                                                                                             as
                                                                                             isize)
                                                        as *mut libc::c_void)
                                 as f32_0,
                             ANIMMODE_ONCE as libc::c_int as u8_0, -3.0f32);
        } else {
            Animation_Change(&mut (*this).skin.skelAnime,
                             *sAnimationHeaders[(*this).type_0 as
                                                    usize].offset((*this).animationIdx
                                                                      as
                                                                      isize),
                             1.0f32, 0.0f32,
                             Animation_GetLastFrame(*sAnimationHeaders[(*this).type_0
                                                                           as
                                                                           usize].offset((*this).animationIdx
                                                                                             as
                                                                                             isize)
                                                        as *mut libc::c_void)
                                 as f32_0, 0 as libc::c_int as u8_0, 0.0f32);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_InitCutscene(mut this: *mut EnHorse,
                                              mut globalCtx:
                                                  *mut GlobalContext) {
    (*this).playerControlled = 0 as libc::c_int;
    (*this).action = ENHORSE_ACT_CS_UPDATE;
    (*this).cutsceneAction = 0 as libc::c_int;
    (*this).actor.speedXZ = 0.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_GetCutsceneFunctionIndex(mut csAction: s32)
 -> s32 {
    let mut numActions: s32 =
        (::std::mem::size_of::<[CsActionEntry; 5]>() as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<CsActionEntry>()
                                             as libc::c_ulong) as s32;
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i < numActions {
        if csAction == sCsActionTable[i as usize].csAction {
            return sCsActionTable[i as usize].csFuncIdx
        }
        if csAction < sCsActionTable[i as usize].csAction {
            return 0 as libc::c_int
        }
        i += 1
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_CutsceneUpdate(mut this: *mut EnHorse,
                                                mut globalCtx:
                                                    *mut GlobalContext) {
    let mut csFunctionIdx: s32 = 0;
    let mut linkCsAction: *mut CsCmdActorAction =
        (*globalCtx).csCtx.linkAction;
    if (*globalCtx).csCtx.state as libc::c_int == 3 as libc::c_int {
        (*this).playerControlled = 1 as libc::c_int;
        (*this).actor.params = 10 as libc::c_int as s16;
        (*this).action = ENHORSE_ACT_IDLE;
        EnHorse_Freeze(this);
        return
    }
    if !linkCsAction.is_null() {
        csFunctionIdx =
            EnHorse_GetCutsceneFunctionIndex((*linkCsAction).action as s32);
        if csFunctionIdx != 0 as libc::c_int {
            if (*this).cutsceneAction != csFunctionIdx {
                if (*this).cutsceneAction == 0 as libc::c_int {
                    (*this).actor.world.pos.x =
                        (*linkCsAction).startPos.x as f32_0;
                    (*this).actor.world.pos.y =
                        (*linkCsAction).startPos.y as f32_0;
                    (*this).actor.world.pos.z =
                        (*linkCsAction).startPos.z as f32_0;
                    (*this).actor.world.rot.y =
                        (*linkCsAction).c2rust_unnamed.urot.y as s16;
                    (*this).actor.shape.rot = (*this).actor.world.rot;
                    (*this).actor.prevPos = (*this).actor.world.pos
                }
                (*this).cutsceneAction = csFunctionIdx;
                sCutsceneInitFuncs[(*this).cutsceneAction as
                                       usize].expect("non-null function pointer")(this,
                                                                                  globalCtx,
                                                                                  linkCsAction);
            }
            sCutsceneActionFuncs[(*this).cutsceneAction as
                                     usize].expect("non-null function pointer")(this,
                                                                                globalCtx,
                                                                                linkCsAction);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_UpdateHbaRaceInfo(mut this: *mut EnHorse,
                                                   mut globalCtx:
                                                       *mut GlobalContext,
                                                   mut raceInfo:
                                                       *mut RaceInfo) -> s32 {
    let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut px: f32_0 = 0.;
    let mut pz: f32_0 = 0.;
    let mut d: f32_0 = 0.;
    EnHorse_RaceWaypointPos((*raceInfo).waypoints, (*this).curRaceWaypoint,
                            &mut pos);
    Math3D_RotateXZPlane(&mut pos,
                         (*(*raceInfo).waypoints.offset((*this).curRaceWaypoint
                                                            as isize)).angle,
                         &mut px, &mut pz, &mut d);
    if (*this).curRaceWaypoint >= (*raceInfo).numWaypoints - 1 as libc::c_int
           &&
           Math3D_Vec3f_DistXYZ(&mut pos, &mut (*this).actor.world.pos) <
               (*gGameInfo).data[(7 as libc::c_int * 6 as libc::c_int *
                                      16 as libc::c_int + 8 as libc::c_int) as
                                     usize] as libc::c_int as libc::c_float {
        (*this).hbaFlags |= 2 as libc::c_int
    }
    if (*this).actor.world.pos.x * px + pz * (*this).actor.world.pos.z + d >
           0.0f32 {
        (*this).curRaceWaypoint += 1;
        if (*this).curRaceWaypoint >= (*raceInfo).numWaypoints {
            (*this).hbaFlags |= 1 as libc::c_int;
            return 1 as libc::c_int
        }
    }
    if (*this).hbaFlags & 1 as libc::c_int == 0 {
        EnHorse_RotateToPoint(this, globalCtx, &mut pos,
                              800 as libc::c_int as s16);
    }
    (*this).actor.shape.rot.y = (*this).actor.world.rot.y;
    if (*this).actor.speedXZ <
           (*(*raceInfo).waypoints.offset((*this).curRaceWaypoint as
                                              isize)).speed as libc::c_int as
               libc::c_float && (*this).hbaFlags & 1 as libc::c_int == 0 {
        (*this).actor.speedXZ += 0.4f32
    } else {
        (*this).actor.speedXZ -= 0.4f32;
        if (*this).actor.speedXZ < 0.0f32 { (*this).actor.speedXZ = 0.0f32 }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_InitHorsebackArchery(mut this:
                                                          *mut EnHorse) {
    (*this).hbaStarted = 0 as libc::c_int;
    (*this).soundTimer = 0 as libc::c_int;
    (*this).curRaceWaypoint = 0 as libc::c_int;
    (*this).hbaTimer = 0 as libc::c_int;
    (*this).actor.speedXZ = 0.0f32;
    EnHorse_UpdateHbaAnim(this);
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_UpdateHbaAnim(mut this: *mut EnHorse) {
    let mut animChanged: s32 = 0 as libc::c_int;
    let mut animSpeed: f32_0 = 0.;
    (*this).action = ENHORSE_ACT_HBA;
    if (*this).actor.speedXZ == 0.0f32 {
        if (*this).animationIdx != ENHORSE_ANIM_IDLE as libc::c_int {
            animChanged = 1 as libc::c_int
        }
        (*this).animationIdx = ENHORSE_ANIM_IDLE as libc::c_int
    } else if (*this).actor.speedXZ <= 3.0f32 {
        if (*this).animationIdx != ENHORSE_ANIM_WALK as libc::c_int {
            animChanged = 1 as libc::c_int
        }
        (*this).animationIdx = ENHORSE_ANIM_WALK as libc::c_int
    } else if (*this).actor.speedXZ <= 6.0f32 {
        if (*this).animationIdx != ENHORSE_ANIM_TROT as libc::c_int {
            animChanged = 1 as libc::c_int
        }
        (*this).animationIdx = ENHORSE_ANIM_TROT as libc::c_int
    } else {
        if (*this).animationIdx != ENHORSE_ANIM_GALLOP as libc::c_int {
            animChanged = 1 as libc::c_int
        }
        (*this).animationIdx = ENHORSE_ANIM_GALLOP as libc::c_int
    }
    if (*this).animationIdx == ENHORSE_ANIM_WALK as libc::c_int {
        animSpeed = (*this).actor.speedXZ * 0.5f32
    } else if (*this).animationIdx == ENHORSE_ANIM_TROT as libc::c_int {
        animSpeed = (*this).actor.speedXZ * 0.25f32;
        Audio_PlaySoundGeneral(0x2804 as libc::c_int as u16_0,
                               &mut (*this).actor.projectedPos,
                               4 as libc::c_int as u8_0, &mut D_801333E0,
                               &mut D_801333E0, &mut D_801333E8);
        func_800AA000(0.0f32, 60 as libc::c_int as u8_0,
                      8 as libc::c_int as u8_0, 255 as libc::c_int as u8_0);
    } else if (*this).animationIdx == ENHORSE_ANIM_GALLOP as libc::c_int {
        animSpeed = (*this).actor.speedXZ * 0.2f32;
        Audio_PlaySoundGeneral(0x2804 as libc::c_int as u16_0,
                               &mut (*this).actor.projectedPos,
                               4 as libc::c_int as u8_0, &mut D_801333E0,
                               &mut D_801333E0, &mut D_801333E8);
        func_800AA000(0.0f32, 120 as libc::c_int as u8_0,
                      8 as libc::c_int as u8_0, 255 as libc::c_int as u8_0);
    } else { animSpeed = 1.0f32 }
    if animChanged == 1 as libc::c_int {
        Animation_Change(&mut (*this).skin.skelAnime,
                         *sAnimationHeaders[(*this).type_0 as
                                                usize].offset((*this).animationIdx
                                                                  as isize),
                         sPlaybackSpeeds[(*this).animationIdx as usize] *
                             animSpeed * 1.5f32, 0 as libc::c_int as f32_0,
                         Animation_GetLastFrame(*sAnimationHeaders[(*this).type_0
                                                                       as
                                                                       usize].offset((*this).animationIdx
                                                                                         as
                                                                                         isize)
                                                    as *mut libc::c_void) as
                             f32_0, ANIMMODE_ONCE as libc::c_int as u8_0,
                         -3.0f32);
    } else {
        Animation_Change(&mut (*this).skin.skelAnime,
                         *sAnimationHeaders[(*this).type_0 as
                                                usize].offset((*this).animationIdx
                                                                  as isize),
                         sPlaybackSpeeds[(*this).animationIdx as usize] *
                             animSpeed * 1.5f32, 0 as libc::c_int as f32_0,
                         Animation_GetLastFrame(*sAnimationHeaders[(*this).type_0
                                                                       as
                                                                       usize].offset((*this).animationIdx
                                                                                         as
                                                                                         isize)
                                                    as *mut libc::c_void) as
                             f32_0, ANIMMODE_ONCE as libc::c_int as u8_0,
                         0 as libc::c_int as f32_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_UpdateHorsebackArchery(mut this:
                                                            *mut EnHorse,
                                                        mut globalCtx:
                                                            *mut GlobalContext) {
    let mut playSpeed: f32_0 = 0.;
    let mut sp20: s32 = 0;
    if (*this).animationIdx == ENHORSE_ANIM_WALK as libc::c_int {
        EnHorse_PlayWalkingSound(this);
    }
    if (*globalCtx).interfaceCtx.hbaAmmo as libc::c_int == 0 as libc::c_int {
        (*this).hbaTimer += 1
    }
    sp20 = func_800F5A58(0x41 as libc::c_int as u8_0);
    EnHorse_UpdateHbaRaceInfo(this, globalCtx, &mut sHbaInfo);
    if (*this).hbaFlags & 1 as libc::c_int != 0 ||
           (*this).hbaTimer >= 46 as libc::c_int {
        if sp20 != 1 as libc::c_int &&
               gSaveContext.minigameState as libc::c_int != 3 as libc::c_int {
            gSaveContext.cutsceneIndex = 0 as libc::c_int;
            (*globalCtx).nextEntranceIndex = 0x3b0 as libc::c_int as s16;
            (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
            (*globalCtx).fadeTransition = 0x20 as libc::c_int as u8_0
        }
    }
    if (*globalCtx).interfaceCtx.hbaAmmo as libc::c_int != 0 as libc::c_int {
        if (*this).hbaFlags & 2 as libc::c_int == 0 {
            if gSaveContext.infTable[25 as libc::c_int as usize] as
                   libc::c_int & 1 as libc::c_int != 0 {
                if gSaveContext.minigameScore as s32 >= 1500 as libc::c_int {
                    (*this).hbaFlags |= 4 as libc::c_int
                }
            } else if gSaveContext.minigameScore as s32 >= 1000 as libc::c_int
             {
                (*this).hbaFlags |= 4 as libc::c_int
            }
        }
    }
    if (*globalCtx).interfaceCtx.hbaAmmo as libc::c_int == 0 as libc::c_int ||
           (*this).hbaFlags & 2 as libc::c_int != 0 {
        if (*this).hbaFlags & 4 as libc::c_int != 0 {
            (*this).hbaFlags &= !(4 as libc::c_int);
            Audio_QueueSeqCmd(((SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                                   24 as libc::c_int | 0x41 as libc::c_int) as
                                  u32_0);
        }
    }
    if (*this).hbaStarted == 0 {
        (*this).actor.speedXZ = 0.0f32;
        if (*this).animationIdx != ENHORSE_ANIM_IDLE as libc::c_int {
            EnHorse_UpdateHbaAnim(this);
        }
    }
    if (*this).animationIdx == ENHORSE_ANIM_WALK as libc::c_int {
        playSpeed = (*this).actor.speedXZ * 0.5f32
    } else if (*this).animationIdx == ENHORSE_ANIM_TROT as libc::c_int {
        playSpeed = (*this).actor.speedXZ * 0.25f32
    } else if (*this).animationIdx == ENHORSE_ANIM_GALLOP as libc::c_int {
        playSpeed = (*this).actor.speedXZ * 0.2f32
    } else { playSpeed = 1.0f32 }
    (*this).skin.skelAnime.playSpeed = playSpeed;
    if SkelAnime_Update(&mut (*this).skin.skelAnime) != 0 ||
           (*this).animationIdx == ENHORSE_ANIM_IDLE as libc::c_int &&
               (*this).actor.speedXZ != 0.0f32 {
        EnHorse_UpdateHbaAnim(this);
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_InitFleePlayer(mut this: *mut EnHorse) {
    (*this).action = ENHORSE_ACT_FLEE_PLAYER;
    (*this).stateFlags |=
        ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_uint;
    (*this).actor.speedXZ = 0.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_FleePlayer(mut this: *mut EnHorse,
                                            mut globalCtx:
                                                *mut GlobalContext) {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut distToHome: f32_0 = 0.;
    let mut playerDistToHome: f32_0 = 0.;
    let mut distToPlayer: f32_0 = 0.;
    let mut nextAnim: s32 = (*this).animationIdx;
    let mut animFinished: s32 = 0;
    let mut yaw: s16 = 0;
    if (*gGameInfo).data[(7 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 53 as libc::c_int) as usize]
           as libc::c_int != 0 || (*this).type_0 == HORSE_HNI as libc::c_int {
        EnHorse_StartIdleRidable(this);
        Audio_PlaySoundGeneral(0x2805 as libc::c_int as u16_0,
                               &mut (*this).actor.projectedPos,
                               4 as libc::c_int as u8_0, &mut D_801333E0,
                               &mut D_801333E0, &mut D_801333E8);
    }
    distToHome =
        Math3D_Vec3f_DistXYZ(&mut (*this).actor.home.pos,
                             &mut (*this).actor.world.pos);
    playerDistToHome =
        Math3D_Vec3f_DistXYZ(&mut (*player).actor.world.pos,
                             &mut (*this).actor.home.pos);
    distToPlayer =
        Math3D_Vec3f_DistXYZ(&mut (*player).actor.world.pos,
                             &mut (*this).actor.world.pos);
    // Run home
    if playerDistToHome > 300.0f32 {
        if distToHome > 150.0f32 {
            (*this).actor.speedXZ += 0.4f32;
            if (*this).actor.speedXZ > 8.0f32 {
                (*this).actor.speedXZ = 8.0f32
            }
        } else {
            (*this).actor.speedXZ -= 0.47f32;
            if (*this).actor.speedXZ < 0.0f32 {
                (*this).actor.speedXZ = 0.0f32
            }
        }
    } else if distToPlayer < 300.0f32 {
        (*this).actor.speedXZ += 0.4f32;
        if (*this).actor.speedXZ > 8.0f32 { (*this).actor.speedXZ = 8.0f32 }
    } else {
        (*this).actor.speedXZ -= 0.47f32;
        if (*this).actor.speedXZ < 0.0f32 { (*this).actor.speedXZ = 0.0f32 }
    }
    if (*this).actor.speedXZ >= 6.0f32 {
        // Run away from Link
        // hoof it
        (*this).skin.skelAnime.playSpeed = (*this).actor.speedXZ * 0.3f32;
        nextAnim = ENHORSE_ANIM_GALLOP as libc::c_int
    } else if (*this).actor.speedXZ >= 3.0f32 {
        // trot
        (*this).skin.skelAnime.playSpeed =
            (*this).actor.speedXZ * 0.375f32; // idle
        nextAnim = ENHORSE_ANIM_TROT as libc::c_int
    } else if (*this).actor.speedXZ > 0.1f32 {
        // walk
        (*this).skin.skelAnime.playSpeed = (*this).actor.speedXZ * 0.75f32;
        nextAnim = ENHORSE_ANIM_WALK as libc::c_int;
        EnHorse_PlayWalkingSound(this);
    } else {
        nextAnim =
            if Rand_ZeroOne() > 0.5f32 {
                1 as libc::c_int
            } else { 0 as libc::c_int };
        EnHorse_IdleAnimSounds(this, globalCtx);
        (*this).skin.skelAnime.playSpeed = 1.0f32
    }
    // Turn away from Link, or towards home
    if nextAnim == ENHORSE_ANIM_GALLOP as libc::c_int ||
           nextAnim == ENHORSE_ANIM_TROT as libc::c_int ||
           nextAnim == ENHORSE_ANIM_WALK as libc::c_int {
        if playerDistToHome < 300.0f32 {
            yaw = (*player).actor.shape.rot.y;
            yaw =
                (yaw as libc::c_int +
                     (if Actor_WorldYawTowardActor(&mut (*this).actor,
                                                   &mut (*player).actor) as
                             libc::c_int > 0 as libc::c_int {
                          1 as libc::c_int
                      } else { -(1 as libc::c_int) }) * 0x3fff as libc::c_int)
                    as s16
        } else {
            yaw =
                (Math_Vec3f_Yaw(&mut (*this).actor.world.pos,
                                &mut (*this).actor.home.pos) as libc::c_int -
                     (*this).actor.world.rot.y as libc::c_int) as s16
        }
        if yaw as libc::c_int > 400 as libc::c_int {
            (*this).actor.world.rot.y =
                ((*this).actor.world.rot.y as libc::c_int +
                     400 as libc::c_int) as s16
        } else if (yaw as libc::c_int) < -(400 as libc::c_int) {
            (*this).actor.world.rot.y =
                ((*this).actor.world.rot.y as libc::c_int -
                     400 as libc::c_int) as s16
        } else {
            (*this).actor.world.rot.y =
                ((*this).actor.world.rot.y as libc::c_int +
                     yaw as libc::c_int) as s16
        }
        (*this).actor.shape.rot.y = (*this).actor.world.rot.y
    }
    animFinished = SkelAnime_Update(&mut (*this).skin.skelAnime);
    if (*this).animationIdx == ENHORSE_ANIM_IDLE as libc::c_int ||
           (*this).animationIdx == ENHORSE_ANIM_WHINNEY as libc::c_int {
        if nextAnim == ENHORSE_ANIM_GALLOP as libc::c_int ||
               nextAnim == ENHORSE_ANIM_TROT as libc::c_int ||
               nextAnim == ENHORSE_ANIM_WALK as libc::c_int {
            (*this).animationIdx = nextAnim;
            Animation_Change(&mut (*this).skin.skelAnime,
                             *sAnimationHeaders[(*this).type_0 as
                                                    usize].offset((*this).animationIdx
                                                                      as
                                                                      isize),
                             1.0f32, 0.0f32,
                             Animation_GetLastFrame(*sAnimationHeaders[(*this).type_0
                                                                           as
                                                                           usize].offset((*this).animationIdx
                                                                                             as
                                                                                             isize)
                                                        as *mut libc::c_void)
                                 as f32_0,
                             ANIMMODE_ONCE as libc::c_int as u8_0, -3.0f32);
            if (*this).animationIdx == ENHORSE_ANIM_GALLOP as libc::c_int {
                EnHorse_PlayGallopingSound(this);
            } else if (*this).animationIdx == ENHORSE_ANIM_TROT as libc::c_int
             {
                EnHorse_PlayTrottingSound(this);
            }
            return
        }
    }
    if animFinished != 0 {
        if nextAnim == ENHORSE_ANIM_GALLOP as libc::c_int {
            EnHorse_PlayGallopingSound(this);
        } else if nextAnim == ENHORSE_ANIM_TROT as libc::c_int {
            EnHorse_PlayTrottingSound(this);
        }
        if (*this).animationIdx == ENHORSE_ANIM_IDLE as libc::c_int ||
               (*this).animationIdx == ENHORSE_ANIM_WHINNEY as libc::c_int {
            if nextAnim != (*this).animationIdx {
                (*this).animationIdx = nextAnim;
                Animation_Change(&mut (*this).skin.skelAnime,
                                 *sAnimationHeaders[(*this).type_0 as
                                                        usize].offset((*this).animationIdx
                                                                          as
                                                                          isize),
                                 1.0f32, 0.0f32,
                                 Animation_GetLastFrame(*sAnimationHeaders[(*this).type_0
                                                                               as
                                                                               usize].offset((*this).animationIdx
                                                                                                 as
                                                                                                 isize)
                                                            as
                                                            *mut libc::c_void)
                                     as f32_0,
                                 ANIMMODE_ONCE as libc::c_int as u8_0,
                                 -3.0f32);
                return
            } else {
                if Rand_ZeroOne() > 0.5f32 {
                    (*this).animationIdx = ENHORSE_ANIM_IDLE as libc::c_int;
                    (*this).stateFlags &=
                        !((1 as libc::c_int) << 12 as libc::c_int) as
                            libc::c_uint
                } else {
                    (*this).animationIdx =
                        ENHORSE_ANIM_WHINNEY as libc::c_int;
                    (*this).unk_21C = (*this).unk_228;
                    if (*this).stateFlags &
                           ((1 as libc::c_int) << 27 as libc::c_int) as
                               libc::c_uint != 0 {
                        Audio_PlaySoundGeneral(0x2816 as libc::c_int as u16_0,
                                               &mut (*this).unk_21C,
                                               4 as libc::c_int as u8_0,
                                               &mut D_801333E0,
                                               &mut D_801333E0,
                                               &mut D_801333E8);
                    }
                }
                Animation_Change(&mut (*this).skin.skelAnime,
                                 *sAnimationHeaders[(*this).type_0 as
                                                        usize].offset((*this).animationIdx
                                                                          as
                                                                          isize),
                                 1.0f32, 0.0f32,
                                 Animation_GetLastFrame(*sAnimationHeaders[(*this).type_0
                                                                               as
                                                                               usize].offset((*this).animationIdx
                                                                                                 as
                                                                                                 isize)
                                                            as
                                                            *mut libc::c_void)
                                     as f32_0,
                                 ANIMMODE_ONCE as libc::c_int as u8_0,
                                 -3.0f32);
                return
            }
        }
        if nextAnim != (*this).animationIdx {
            (*this).animationIdx = nextAnim;
            Animation_Change(&mut (*this).skin.skelAnime,
                             *sAnimationHeaders[(*this).type_0 as
                                                    usize].offset((*this).animationIdx
                                                                      as
                                                                      isize),
                             1.0f32, 0.0f32,
                             Animation_GetLastFrame(*sAnimationHeaders[(*this).type_0
                                                                           as
                                                                           usize].offset((*this).animationIdx
                                                                                             as
                                                                                             isize)
                                                        as *mut libc::c_void)
                                 as f32_0,
                             ANIMMODE_ONCE as libc::c_int as u8_0, -3.0f32);
        } else {
            Animation_Change(&mut (*this).skin.skelAnime,
                             *sAnimationHeaders[(*this).type_0 as
                                                    usize].offset((*this).animationIdx
                                                                      as
                                                                      isize),
                             1.0f32, 0.0f32,
                             Animation_GetLastFrame(*sAnimationHeaders[(*this).type_0
                                                                           as
                                                                           usize].offset((*this).animationIdx
                                                                                             as
                                                                                             isize)
                                                        as *mut libc::c_void)
                                 as f32_0,
                             ANIMMODE_ONCE as libc::c_int as u8_0, 0.0f32);
        }
        return
    }
    if (*this).animationIdx == ENHORSE_ANIM_WALK as libc::c_int {
        if nextAnim == ENHORSE_ANIM_IDLE as libc::c_int ||
               nextAnim == ENHORSE_ANIM_WHINNEY as libc::c_int {
            (*this).animationIdx = nextAnim;
            Animation_Change(&mut (*this).skin.skelAnime,
                             *sAnimationHeaders[(*this).type_0 as
                                                    usize].offset((*this).animationIdx
                                                                      as
                                                                      isize),
                             1.0f32, 0.0f32,
                             Animation_GetLastFrame(*sAnimationHeaders[(*this).type_0
                                                                           as
                                                                           usize].offset((*this).animationIdx
                                                                                             as
                                                                                             isize)
                                                        as *mut libc::c_void)
                                 as f32_0,
                             ANIMMODE_ONCE as libc::c_int as u8_0, -3.0f32);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_BridgeJumpInit(mut this: *mut EnHorse,
                                                mut globalCtx:
                                                    *mut GlobalContext) {
    let mut y: f32_0 = 0.;
    func_80028A54(globalCtx, 25.0f32, &mut (*this).actor.world.pos);
    (*this).action = ENHORSE_ACT_BRIDGE_JUMP;
    (*this).stateFlags |=
        ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint;
    (*this).animationIdx = ENHORSE_ANIM_HIGH_JUMP as libc::c_int;
    y = (*(*this).skin.skelAnime.jointTable).y as f32_0;
    y = y * 0.01f32;
    (*this).bridgeJumpStart = (*this).actor.world.pos;
    (*this).bridgeJumpStart.y += y;
    (*this).bridgeJumpYVel =
        (sBridgeJumps[(*this).bridgeJumpIdx as usize].pos.y as libc::c_int as
             libc::c_float + 48.7f32 - (*this).bridgeJumpStart.y - -360.0f32)
            / 30.0f32;
    (*this).riderPos.y -= y;
    (*this).stateFlags |=
        ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint;
    (*this).bridgeJumpRelAngle =
        ((*this).actor.world.rot.y as libc::c_int -
             sBridgeJumps[(*this).bridgeJumpIdx as usize].angle as
                 libc::c_int) as s16;
    (*this).bridgeJumpTimer = 0 as libc::c_int;
    (*this).actor.gravity = 0.0f32;
    (*this).actor.speedXZ = 0 as libc::c_int as f32_0;
    Animation_Change(&mut (*this).skin.skelAnime,
                     *sAnimationHeaders[(*this).type_0 as
                                            usize].offset((*this).animationIdx
                                                              as isize),
                     1.5f32, 0.0f32,
                     Animation_GetLastFrame(*sAnimationHeaders[(*this).type_0
                                                                   as
                                                                   usize].offset((*this).animationIdx
                                                                                     as
                                                                                     isize)
                                                as *mut libc::c_void) as
                         f32_0, ANIMMODE_ONCE as libc::c_int as u8_0,
                     -3.0f32);
    (*this).unk_21C = (*this).unk_228;
    if (*this).stateFlags &
           ((1 as libc::c_int) << 27 as libc::c_int) as libc::c_uint != 0 {
        Audio_PlaySoundGeneral(0x2805 as libc::c_int as u16_0,
                               &mut (*this).unk_21C, 4 as libc::c_int as u8_0,
                               &mut D_801333E0, &mut D_801333E0,
                               &mut D_801333E8);
    }
    Audio_PlaySoundGeneral(0x2818 as libc::c_int as u16_0,
                           &mut (*this).actor.projectedPos,
                           4 as libc::c_int as u8_0, &mut D_801333E0,
                           &mut D_801333E0, &mut D_801333E8);
    func_800AA000(0.0f32, 170 as libc::c_int as u8_0,
                  10 as libc::c_int as u8_0, 10 as libc::c_int as u8_0);
    (*this).postDrawFunc = None;
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_StartBridgeJump(mut this: *mut EnHorse,
                                                 mut globalCtx:
                                                     *mut GlobalContext) {
    (*this).postDrawFunc =
        Some(EnHorse_BridgeJumpInit as
                 unsafe extern "C" fn(_: *mut EnHorse, _: *mut GlobalContext)
                     -> ());
    if (*this).bridgeJumpIdx as libc::c_int == 0 as libc::c_int {
        (*globalCtx).csCtx.segment =
            gSegments[((gGerudoValleyBridgeJumpFieldFortressCs.as_mut_ptr() as
                            u32_0) << 4 as libc::c_int >> 28 as libc::c_int)
                          as
                          usize].wrapping_add(gGerudoValleyBridgeJumpFieldFortressCs.as_mut_ptr()
                                                  as u32_0 &
                                                  0xffffff as libc::c_int as
                                                      libc::c_uint).wrapping_add(0x80000000
                                                                                     as
                                                                                     libc::c_uint)
                as *mut libc::c_void;
        gSaveContext.cutsceneTrigger = 1 as libc::c_int as u8_0
    } else {
        (*globalCtx).csCtx.segment =
            gSegments[((gGerudoValleyBridgeJumpFortressToFieldCs.as_mut_ptr()
                            as u32_0) << 4 as libc::c_int >>
                           28 as libc::c_int) as
                          usize].wrapping_add(gGerudoValleyBridgeJumpFortressToFieldCs.as_mut_ptr()
                                                  as u32_0 &
                                                  0xffffff as libc::c_int as
                                                      libc::c_uint).wrapping_add(0x80000000
                                                                                     as
                                                                                     libc::c_uint)
                as *mut libc::c_void;
        gSaveContext.cutsceneTrigger = 1 as libc::c_int as u8_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_BridgeJumpMove(mut this: *mut EnHorse,
                                                mut globalCtx:
                                                    *mut GlobalContext) {
    let mut interp: f32_0 = 0.;
    let mut timeSq: f32_0 = 0.;
    interp = (*this).bridgeJumpTimer as libc::c_float / 30.0f32;
    timeSq = ((*this).bridgeJumpTimer * (*this).bridgeJumpTimer) as f32_0;
    (*this).actor.world.pos.x =
        (sBridgeJumps[(*this).bridgeJumpIdx as usize].pos.x as libc::c_int as
             libc::c_float - (*this).bridgeJumpStart.x) * interp +
            (*this).bridgeJumpStart.x;
    (*this).actor.world.pos.z =
        (sBridgeJumps[(*this).bridgeJumpIdx as usize].pos.z as libc::c_int as
             libc::c_float - (*this).bridgeJumpStart.z) * interp +
            (*this).bridgeJumpStart.z;
    (*this).actor.world.pos.y =
        (*this).bridgeJumpStart.y +
            (*this).bridgeJumpYVel * (*this).bridgeJumpTimer as libc::c_float
            + -0.4f32 * timeSq;
    (*this).actor.shape.rot.y =
        (sBridgeJumps[(*this).bridgeJumpIdx as usize].angle as libc::c_int as
             libc::c_float +
             (1.0f32 - interp) *
                 (*this).bridgeJumpRelAngle as libc::c_int as libc::c_float)
            as s16;
    (*this).actor.world.rot.y = (*this).actor.shape.rot.y;
    (*this).skin.skelAnime.curFrame = 23.0f32 * interp;
    SkelAnime_Update(&mut (*this).skin.skelAnime);
    if (*this).bridgeJumpTimer < 30 as libc::c_int {
        (*this).stateFlags |=
            ((1 as libc::c_int) << 24 as libc::c_int) as libc::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_CheckBridgeJumpLanding(mut this:
                                                            *mut EnHorse,
                                                        mut globalCtx:
                                                            *mut GlobalContext) {
    (*this).actor.speedXZ = 8.0f32;
    (*this).skin.skelAnime.playSpeed = 1.5f32;
    if SkelAnime_Update(&mut (*this).skin.skelAnime) != 0 {
        (*this).stateFlags &=
            !((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint;
        (*this).actor.gravity = -3.5f32;
        (*this).actor.world.pos.y =
            sBridgeJumps[(*this).bridgeJumpIdx as usize].pos.y as f32_0;
        func_80028A54(globalCtx, 25.0f32, &mut (*this).actor.world.pos);
        EnHorse_JumpLanding(this, globalCtx);
        Audio_PlaySoundGeneral(0x2819 as libc::c_int as u16_0,
                               &mut (*this).actor.projectedPos,
                               4 as libc::c_int as u8_0, &mut D_801333E0,
                               &mut D_801333E0, &mut D_801333E8);
        func_800AA000(0.0f32, 255 as libc::c_int as u8_0,
                      10 as libc::c_int as u8_0, 80 as libc::c_int as u8_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_BridgeJump(mut this: *mut EnHorse,
                                            mut globalCtx:
                                                *mut GlobalContext) {
    (*this).bridgeJumpTimer += 1;
    if (*this).bridgeJumpTimer < 30 as libc::c_int {
        EnHorse_BridgeJumpMove(this, globalCtx);
        return
    }
    EnHorse_CheckBridgeJumpLanding(this, globalCtx);
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_Vec3fOffset(mut src: *mut Vec3f,
                                             mut yaw: s16, mut dist: f32_0,
                                             mut height: f32_0,
                                             mut dst: *mut Vec3f) {
    (*dst).x = (*src).x + Math_SinS(yaw) * dist;
    (*dst).y = (*src).y + height;
    (*dst).z = (*src).z + Math_CosS(yaw) * dist;
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_CalcFloorHeight(mut this: *mut EnHorse,
                                                 mut globalCtx:
                                                     *mut GlobalContext,
                                                 mut pos: *mut Vec3f,
                                                 mut floorPoly:
                                                     *mut *mut CollisionPoly,
                                                 mut floorHeight: *mut f32_0)
 -> s32 {
    let mut bgId: s32 = 0;
    let mut waterY: f32_0 = 0.;
    let mut waterBox: *mut WaterBox = 0 as *mut WaterBox;
    *floorPoly = 0 as *mut CollisionPoly;
    *floorHeight =
        BgCheck_EntityRaycastFloor3(&mut (*globalCtx).colCtx, floorPoly,
                                    &mut bgId, pos);
    if *floorHeight == -32000.0f32 {
        return 1 as libc::c_int
        // No floor
    }
    if WaterBox_GetSurfaceImpl(globalCtx, &mut (*globalCtx).colCtx, (*pos).x,
                               (*pos).z, &mut waterY, &mut waterBox) ==
           1 as libc::c_int && *floorHeight < waterY {
        return 2 as libc::c_int
        // Water
    }
    if (**floorPoly).normal.y as libc::c_int as libc::c_float *
           (1.0f32 / 32767.0f32) < 0.81915206f32 ||
           SurfaceType_IsHorseBlocked(&mut (*globalCtx).colCtx, *floorPoly,
                                      bgId) != 0 ||
           func_80041D4C(&mut (*globalCtx).colCtx, *floorPoly, bgId) ==
               7 as libc::c_int as libc::c_uint {
        return 3 as libc::c_int
        // Horse blocked surface
    }
    return 0 as libc::c_int;
}
/* *
 * obstacleType:
 *  1: Water in front
 *  2: Water behind?
 *  3: ?
 *  4: Obstructed in front
 *  5: Obstructed behind
 */
#[no_mangle]
pub unsafe extern "C" fn EnHorse_ObstructMovement(mut this: *mut EnHorse,
                                                  mut globalCtx:
                                                      *mut GlobalContext,
                                                  mut obstacleType: s32,
                                                  mut galloping: s32) {
    if (*this).action as libc::c_uint ==
           ENHORSE_ACT_CS_UPDATE as libc::c_int as libc::c_uint ||
           EnHorse_BgCheckBridgeJumpPoint(this, globalCtx) != 0 {
        return
    }
    (*this).actor.world.pos = (*this).lastPos;
    (*this).actor.shape.rot.y = (*this).lastYaw;
    (*this).actor.world.rot.y = (*this).lastYaw;
    (*this).stateFlags |=
        ((1 as libc::c_int) << 14 as libc::c_int) as libc::c_uint;
    if (*this).playerControlled == 0 {
        if (*this).animationIdx != ENHORSE_ANIM_REARING as libc::c_int {
            return
        }
    } else if (*this).action as libc::c_uint !=
                  ENHORSE_ACT_MOUNTED_REARING as libc::c_int as libc::c_uint {
        if (*this).stateFlags &
               ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint != 0 {
            (*this).stateFlags &=
                !((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint;
            (*this).actor.gravity = -3.5f32;
            (*this).actor.world.pos.y = (*this).actor.floorHeight
        }
        if obstacleType == 1 as libc::c_int ||
               obstacleType == 4 as libc::c_int {
            (*this).stateFlags |=
                ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_uint
        } else if obstacleType == 2 as libc::c_int ||
                      obstacleType == 5 as libc::c_int {
            (*this).stateFlags |=
                ((1 as libc::c_int) << 5 as libc::c_int) as libc::c_uint
        }
        if galloping == 1 as libc::c_int { EnHorse_StartRearing(this); }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_CheckFloors(mut this: *mut EnHorse,
                                             mut globalCtx:
                                                 *mut GlobalContext) {
    let mut status: s32 = 0;
    let mut frontFloor: *mut CollisionPoly = 0 as *mut CollisionPoly;
    let mut backFloor: *mut CollisionPoly = 0 as *mut CollisionPoly;
    let mut floorSlope: s16 = 0;
    let mut frontPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut backPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut nx: f32_0 = 0.;
    let mut ny: f32_0 = 0.;
    let mut nz: f32_0 = 0.;
    let mut galloping: s32 =
        ((*this).actor.speedXZ > 8 as libc::c_int as libc::c_float) as
            libc::c_int;
    let mut dist: f32_0 = 0.;
    let mut waterHeight: f32_0 = 0.;
    let mut waterBox: *mut WaterBox = 0 as *mut WaterBox;
    let mut pad: s32 = 0;
    if WaterBox_GetSurfaceImpl(globalCtx, &mut (*globalCtx).colCtx,
                               (*this).actor.world.pos.x,
                               (*this).actor.world.pos.z, &mut waterHeight,
                               &mut waterBox) == 1 as libc::c_int &&
           (*this).actor.floorHeight < waterHeight {
        EnHorse_ObstructMovement(this, globalCtx, 1 as libc::c_int,
                                 galloping);
        return
    }
    EnHorse_Vec3fOffset(&mut (*this).actor.world.pos,
                        (*this).actor.shape.rot.y, 30.0f32, 60.0f32,
                        &mut frontPos);
    status =
        EnHorse_CalcFloorHeight(this, globalCtx, &mut frontPos,
                                &mut frontFloor, &mut (*this).yFront);
    if status == 1 as libc::c_int {
        (*this).actor.shape.rot.x = 0 as libc::c_int as s16;
        EnHorse_ObstructMovement(this, globalCtx, 4 as libc::c_int,
                                 galloping);
        return
    }
    if status == 2 as libc::c_int {
        EnHorse_ObstructMovement(this, globalCtx, 4 as libc::c_int,
                                 galloping);
        return
    }
    if status == 3 as libc::c_int {
        EnHorse_ObstructMovement(this, globalCtx, 4 as libc::c_int,
                                 galloping);
        return
    }
    EnHorse_Vec3fOffset(&mut (*this).actor.world.pos,
                        (*this).actor.shape.rot.y, -30.0f32, 60.0f32,
                        &mut backPos);
    status =
        EnHorse_CalcFloorHeight(this, globalCtx, &mut backPos, &mut backFloor,
                                &mut (*this).yBack);
    if status == 1 as libc::c_int {
        (*this).actor.shape.rot.x = 0 as libc::c_int as s16;
        EnHorse_ObstructMovement(this, globalCtx, 5 as libc::c_int,
                                 galloping);
        return
    }
    if status == 2 as libc::c_int {
        EnHorse_ObstructMovement(this, globalCtx, 5 as libc::c_int,
                                 galloping);
        return
    }
    if status == 3 as libc::c_int {
        EnHorse_ObstructMovement(this, globalCtx, 5 as libc::c_int,
                                 galloping);
        return
    }
    floorSlope =
        (Math_FAtan2F((*this).yBack - (*this).yFront, 60.0f32) *
             (0x8000 as libc::c_int as libc::c_float /
                  3.14159265358979323846f32)) as s16;
    if !(*this).actor.floorPoly.is_null() {
        nx =
            (*(*this).actor.floorPoly).normal.x as libc::c_int as
                libc::c_float * (1.0f32 / 32767.0f32);
        ny =
            (*(*this).actor.floorPoly).normal.y as libc::c_int as
                libc::c_float * (1.0f32 / 32767.0f32);
        nz =
            (*(*this).actor.floorPoly).normal.z as libc::c_int as
                libc::c_float * (1.0f32 / 32767.0f32);
        pos = frontPos;
        pos.y = (*this).yFront;
        dist =
            Math3D_DistPlaneToPos(nx, ny, nz,
                                  (*(*this).actor.floorPoly).dist as f32_0,
                                  &mut pos);
        if frontFloor != (*this).actor.floorPoly &&
               (*this).actor.speedXZ >= 0.0f32 {
            if (*this).stateFlags &
                   ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint ==
                   0 && dist < -40.0f32 ||
                   (*this).stateFlags &
                       ((1 as libc::c_int) << 2 as libc::c_int) as
                           libc::c_uint != 0 && dist < -200.0f32 {
                EnHorse_ObstructMovement(this, globalCtx, 4 as libc::c_int,
                                         galloping);
                return
            }
        }
        pos = backPos;
        pos.y = (*this).yBack;
        dist =
            Math3D_DistPlaneToPos(nx, ny, nz,
                                  (*(*this).actor.floorPoly).dist as f32_0,
                                  &mut pos);
        if backFloor != (*this).actor.floorPoly &&
               (*this).actor.speedXZ <= 0.0f32 &&
               (*this).stateFlags &
                   ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint ==
                   0 && dist < -40.0f32 ||
               (*this).stateFlags &
                   ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint !=
                   0 && dist < -200.0f32 {
            EnHorse_ObstructMovement(this, globalCtx, 5 as libc::c_int,
                                     galloping);
            return
        }
        if ny < 0.81915206f32 ||
               SurfaceType_IsHorseBlocked(&mut (*globalCtx).colCtx,
                                          (*this).actor.floorPoly,
                                          (*this).actor.floorBgId as s32) != 0
               ||
               func_80041D4C(&mut (*globalCtx).colCtx,
                             (*this).actor.floorPoly,
                             (*this).actor.floorBgId as s32) ==
                   7 as libc::c_int as libc::c_uint {
            if (*this).actor.speedXZ >= 0.0f32 {
                EnHorse_ObstructMovement(this, globalCtx, 4 as libc::c_int,
                                         galloping);
            } else {
                EnHorse_ObstructMovement(this, globalCtx, 5 as libc::c_int,
                                         galloping);
            }
            return
        }
        if (*this).stateFlags &
               ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint != 0 {
            (*this).actor.shape.rot.x = 0 as libc::c_int as s16;
            return
        }
        if (*this).actor.floorHeight + 4.0f32 < (*this).actor.world.pos.y {
            (*this).actor.shape.rot.x = 0 as libc::c_int as s16;
            return
        }
        if fabsf(floorSlope as f32_0) > 8191.0f32 { return }
        (*this).actor.shape.rot.x = floorSlope;
        (*this).actor.shape.yOffset =
            (*this).yFront +
                ((*this).yBack - (*this).yFront) * 20.0f32 / 45.0f32 -
                (*this).actor.floorHeight
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_MountDismount(mut this: *mut EnHorse,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    let mut pad: [s32; 2] = [0; 2];
    let mut mountSide: s32 = 0;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    mountSide = EnHorse_GetMountSide(this, globalCtx);
    if mountSide != 0 as libc::c_int &&
           (*this).stateFlags &
               ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_uint == 0
           && (*player).rideActor.is_null() {
        Actor_SetRideActor(globalCtx, &mut (*this).actor, mountSide);
    }
    if (*this).playerControlled == 0 as libc::c_int &&
           Actor_IsMounted(globalCtx, &mut (*this).actor) == 1 as libc::c_int
       {
        (*this).noInputTimer = 55 as libc::c_int;
        (*this).noInputTimerMax = 55 as libc::c_int;
        (*this).playerControlled = 1 as libc::c_int;
        EnHorse_Freeze(this);
    } else if (*this).playerControlled == 1 as libc::c_int &&
                  Actor_NotMounted(globalCtx, &mut (*this).actor) ==
                      1 as libc::c_int {
        (*this).noInputTimer = 35 as libc::c_int;
        (*this).noInputTimerMax = 35 as libc::c_int;
        (*this).stateFlags &=
            !((1 as libc::c_int) << 16 as libc::c_int) as libc::c_uint;
        (*this).playerControlled = 0 as libc::c_int;
        EnHorse_Freeze(this);
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_StickDirection(mut curStick: *mut Vec2f,
                                                mut stickMag: *mut f32_0,
                                                mut angle: *mut s16) {
    let mut dist: f32_0 = 0.;
    let mut y: f32_0 = 0.;
    let mut x: f32_0 = 0.;
    x = (*curStick).x;
    y = (*curStick).y;
    dist = sqrtf(x * x + y * y);
    *stickMag = dist;
    if dist > 60.0f32 { *stickMag = 60.0f32 } else { *stickMag = *stickMag }
    *angle =
        (Math_FAtan2F(-(*curStick).x, (*curStick).y) *
             (32768.0f32 / 3.14159265358979323846f32)) as s16;
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_UpdateStick(mut this: *mut EnHorse,
                                             mut globalCtx:
                                                 *mut GlobalContext) {
    (*this).lastStick = (*this).curStick;
    (*this).curStick.x =
        (*globalCtx).state.input[0 as libc::c_int as usize].rel.stick_x as
            f32_0;
    (*this).curStick.y =
        (*globalCtx).state.input[0 as libc::c_int as usize].rel.stick_y as
            f32_0;
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_ResolveCollision(mut this: *mut EnHorse,
                                                  mut globalCtx:
                                                      *mut GlobalContext,
                                                  mut colPoly:
                                                      *mut CollisionPoly) {
    let mut dist: f32_0 = 0.;
    let mut nx: f32_0 = 0.;
    let mut ny: f32_0 = 0.;
    let mut nz: f32_0 = 0.;
    let mut offset: f32_0 = 0.;
    nx =
        (*colPoly).normal.x as libc::c_int as libc::c_float *
            (1.0f32 / 32767.0f32);
    ny =
        (*colPoly).normal.y as libc::c_int as libc::c_float *
            (1.0f32 / 32767.0f32);
    nz =
        (*colPoly).normal.z as libc::c_int as libc::c_float *
            (1.0f32 / 32767.0f32);
    if !(Math_CosS(((*this).actor.world.rot.y as libc::c_int -
                        (Math_FAtan2F((*colPoly).normal.x as f32_0,
                                      (*colPoly).normal.z as f32_0) *
                             (0x8000 as libc::c_int as libc::c_float /
                                  3.14159265358979323846f32)) as s16 as
                            libc::c_int - 0x7fff as libc::c_int) as s16) <
             0.7071f32) {
        // cos(45 degrees)
        dist =
            Math3D_DistPlaneToPos(nx, ny, nz, (*colPoly).dist as f32_0,
                                  &mut (*this).actor.world.pos);
        offset = 1.0f32 / sqrtf(nx * nx + nz * nz);
        offset = (30.0f32 - dist) * offset;
        (*this).actor.world.pos.x += offset * nx;
        (*this).actor.world.pos.z += offset * nz
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_BgCheckSlowMoving(mut this: *mut EnHorse,
                                                   mut globalCtx:
                                                       *mut GlobalContext) {
    let mut yOffset: f32_0 = 0.;
    let mut start: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut end: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut intersect: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut colPoly: *mut CollisionPoly = 0 as *mut CollisionPoly;
    let mut bgId: s32 = 0;
    if (*globalCtx).sceneNum as libc::c_int == SCENE_SPOT20 as libc::c_int {
        yOffset = 19.0f32
    } else { yOffset = 40.0f32 }
    Math_Vec3f_Copy(&mut start, &mut (*this).actor.world.pos);
    start.y = start.y + yOffset;
    Math_Vec3f_Copy(&mut end, &mut start);
    end.x += 30.0f32 * Math_SinS((*this).actor.world.rot.y);
    end.y +=
        30.0f32 *
            Math_SinS(-((*this).actor.shape.rot.x as libc::c_int) as s16);
    end.z += 30.0f32 * Math_CosS((*this).actor.world.rot.y);
    if BgCheck_EntityLineTest1(&mut (*globalCtx).colCtx, &mut start, &mut end,
                               &mut intersect, &mut colPoly, 1 as libc::c_int,
                               0 as libc::c_int, 0 as libc::c_int,
                               1 as libc::c_int, &mut bgId) !=
           0 as libc::c_int {
        EnHorse_ResolveCollision(this, globalCtx, colPoly);
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_UpdateBgCheckInfo(mut this: *mut EnHorse,
                                                   mut globalCtx:
                                                       *mut GlobalContext) {
    let mut pad: s32 = 0;
    let mut pad2: s32 = 0;
    let mut startPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut endPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut obstaclePos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut pad3: f32_0 = 0.;
    let mut intersectDist: f32_0 = 0.;
    let mut wall: *mut CollisionPoly = 0 as *mut CollisionPoly;
    let mut obstacleFloor: *mut CollisionPoly = 0 as *mut CollisionPoly;
    let mut bgId: s32 = 0;
    let mut obstacleHeight: f32_0 = 0.;
    let mut behindObstacleHeight: f32_0 = 0.;
    let mut ny: f32_0 = 0.;
    let mut movingFast: s32 = 0;
    let mut pad5: s32 = 0;
    let mut dynaPoly: *mut DynaPolyActor = 0 as *mut DynaPolyActor;
    let mut intersect: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut obstacleTop: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    Actor_UpdateBgCheckInfo(globalCtx, &mut (*this).actor,
                            if (*globalCtx).sceneNum as libc::c_int ==
                                   SCENE_SPOT20 as libc::c_int {
                                19.0f32
                            } else { 40.0f32 }, 35.0f32, 100.0f32,
                            29 as libc::c_int);
    if EnHorse_BgCheckBridgeJumpPoint(this, globalCtx) != 0 { return }
    // void 0 trick required to match, but is surely not real. revisit at a later time
    if (*this).actor.bgCheckFlags as libc::c_int & 8 as libc::c_int != 0 &&
           Math_CosS(((*this).actor.wallYaw as libc::c_int -
                          (*this).actor.world.rot.y as libc::c_int) as s16) <
               -0.3f32 {
        if (*this).actor.speedXZ > 4.0f32 {
            (*this).actor.speedXZ -= 1.0f32;
            Audio_PlaySoundGeneral(0x282c as libc::c_int as u16_0,
                                   &mut (*this).actor.projectedPos,
                                   4 as libc::c_int as u8_0, &mut D_801333E0,
                                   &mut D_801333E0, &mut D_801333E8);
        }
    }
    if (*this).stateFlags &
           ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint != 0 ||
           (*this).playerControlled == 0 {
        return
    }
    if (*this).actor.speedXZ < 0.0f32 { return }
    // Braking or rearing from obstacle
    if (*this).action as libc::c_uint ==
           ENHORSE_ACT_STOPPING as libc::c_int as libc::c_uint ||
           (*this).action as libc::c_uint ==
               ENHORSE_ACT_MOUNTED_REARING as libc::c_int as libc::c_uint {
        return
    }
    if (*this).actor.speedXZ > 8.0f32 {
        if (*this).actor.speedXZ < 12.8f32 {
            intersectDist = 160.0f32;
            movingFast = 0 as libc::c_int
        } else { intersectDist = 230.0f32; movingFast = 1 as libc::c_int }
    } else { EnHorse_BgCheckSlowMoving(this, globalCtx); return }
    startPos = (*this).actor.world.pos;
    startPos.y += 19.0f32;
    endPos = startPos;
    endPos.x += intersectDist * Math_SinS((*this).actor.world.rot.y);
    endPos.y +=
        intersectDist *
            Math_SinS(-((*this).actor.shape.rot.x as libc::c_int) as s16);
    endPos.z += intersectDist * Math_CosS((*this).actor.world.rot.y);
    intersect = endPos;
    wall = 0 as *mut CollisionPoly;
    if BgCheck_EntityLineTest1(&mut (*globalCtx).colCtx, &mut startPos,
                               &mut endPos, &mut intersect, &mut wall,
                               1 as libc::c_int, 0 as libc::c_int,
                               0 as libc::c_int, 1 as libc::c_int, &mut bgId)
           == 1 as libc::c_int {
        intersectDist =
            sqrt(Math3D_Vec3fDistSq(&mut startPos, &mut intersect) as f64_0)
                as f32_0;
        (*this).stateFlags |=
            ((1 as libc::c_int) << 14 as libc::c_int) as libc::c_uint
    }
    if !wall.is_null() {
        if intersectDist < 30.0f32 {
            EnHorse_ResolveCollision(this, globalCtx, wall);
        }
        if Math_CosS(((*this).actor.world.rot.y as libc::c_int -
                          (Math_FAtan2F((*wall).normal.x as f32_0,
                                        (*wall).normal.z as f32_0) *
                               (0x8000 as libc::c_int as libc::c_float /
                                    3.14159265358979323846f32)) as s16 as
                              libc::c_int - 0x7fff as libc::c_int) as s16) <
               0.5f32 ||
               SurfaceType_IsHorseBlocked(&mut (*globalCtx).colCtx, wall,
                                          bgId) !=
                   0 as libc::c_int as libc::c_uint {
            return
        }
        // too close to jump
        if movingFast == 0 as libc::c_int && intersectDist < 80.0f32 ||
               movingFast == 1 as libc::c_int && intersectDist < 150.0f32 {
            if movingFast == 0 as libc::c_int {
                (*this).stateFlags |=
                    ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_uint
            } else if movingFast == 1 as libc::c_int {
                (*this).stateFlags |=
                    ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_uint;
                EnHorse_StartBraking(this, globalCtx);
            }
            return
        }
        dynaPoly = DynaPoly_GetActor(&mut (*globalCtx).colCtx, bgId);
        if (*this).stateFlags &
               ((1 as libc::c_int) << 26 as libc::c_int) as libc::c_uint != 0
               &&
               (!dynaPoly.is_null() &&
                    (*dynaPoly).actor.id as libc::c_int !=
                        0x108 as libc::c_int || dynaPoly.is_null()) {
            if movingFast == 0 as libc::c_int {
                (*this).stateFlags |=
                    ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_uint
            } else if movingFast == 1 as libc::c_int {
                (*this).stateFlags |=
                    ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_uint;
                EnHorse_StartBraking(this, globalCtx);
            }
            return
        }
    }
    // Get obstacle's height
    intersectDist += 5.0f32;
    obstaclePos = startPos;
    obstaclePos.x += intersectDist * Math_SinS((*this).actor.world.rot.y);
    obstaclePos.y = (*this).actor.world.pos.y + 120.0f32;
    obstaclePos.z += intersectDist * Math_CosS((*this).actor.world.rot.y);
    obstacleTop = obstaclePos;
    obstacleTop.y =
        BgCheck_EntityRaycastFloor3(&mut (*globalCtx).colCtx,
                                    &mut obstacleFloor, &mut bgId,
                                    &mut obstaclePos);
    if obstacleTop.y == -32000.0f32 { return }
    obstacleHeight = obstacleTop.y - (*this).actor.world.pos.y;
    if (*this).actor.floorPoly.is_null() || obstacleFloor.is_null() { return }
    if Math3D_DistPlaneToPos((*(*this).actor.floorPoly).normal.x as
                                 libc::c_int as libc::c_float *
                                 (1.0f32 / 32767.0f32),
                             (*(*this).actor.floorPoly).normal.y as
                                 libc::c_int as libc::c_float *
                                 (1.0f32 / 32767.0f32),
                             (*(*this).actor.floorPoly).normal.z as
                                 libc::c_int as libc::c_float *
                                 (1.0f32 / 32767.0f32),
                             (*(*this).actor.floorPoly).dist as f32_0,
                             &mut obstacleTop) < -40.0f32 &&
           Math3D_DistPlaneToPos((*obstacleFloor).normal.x as libc::c_int as
                                     libc::c_float * (1.0f32 / 32767.0f32),
                                 (*obstacleFloor).normal.y as libc::c_int as
                                     libc::c_float * (1.0f32 / 32767.0f32),
                                 (*obstacleFloor).normal.z as libc::c_int as
                                     libc::c_float * (1.0f32 / 32767.0f32),
                                 (*obstacleFloor).dist as f32_0,
                                 &mut (*this).actor.world.pos) > 40.0f32 {
        if movingFast == 1 as libc::c_int &&
               (*this).action as libc::c_uint !=
                   ENHORSE_ACT_STOPPING as libc::c_int as libc::c_uint {
            (*this).stateFlags |=
                ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_uint;
            EnHorse_StartBraking(this, globalCtx);
        }
        (*this).stateFlags |=
            ((1 as libc::c_int) << 14 as libc::c_int) as libc::c_uint;
        return
    }
    ny =
        (*obstacleFloor).normal.y as libc::c_int as libc::c_float *
            (1.0f32 / 32767.0f32);
    if ny < 0.81915206f32 ||
           SurfaceType_IsHorseBlocked(&mut (*globalCtx).colCtx, obstacleFloor,
                                      bgId) !=
               0 as libc::c_int as libc::c_uint ||
           func_80041D4C(&mut (*globalCtx).colCtx, obstacleFloor, bgId) ==
               7 as libc::c_int as libc::c_uint {
        if movingFast == 1 as libc::c_int &&
               (*this).action as libc::c_uint !=
                   ENHORSE_ACT_STOPPING as libc::c_int as libc::c_uint {
            (*this).stateFlags |=
                ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_uint;
            EnHorse_StartBraking(this, globalCtx);
        }
        return
    }
    if wall.is_null() || obstacleTop.y < intersect.y ||
           (*this).stateFlags &
               ((1 as libc::c_int) << 17 as libc::c_int) as libc::c_uint != 0
       {
        return
    }
    obstaclePos = startPos;
    obstaclePos.y = (*this).actor.world.pos.y + 120.0f32;
    if movingFast == 0 as libc::c_int {
        obstaclePos.x += 276.0f32 * Math_SinS((*this).actor.world.rot.y);
        obstaclePos.z += 276.0f32 * Math_CosS((*this).actor.world.rot.y)
    } else {
        obstaclePos.x += 390.0f32 * Math_SinS((*this).actor.world.rot.y);
        obstaclePos.z += 390.0f32 * Math_CosS((*this).actor.world.rot.y)
    }
    obstacleTop = obstaclePos;
    obstacleTop.y =
        BgCheck_EntityRaycastFloor3(&mut (*globalCtx).colCtx,
                                    &mut obstacleFloor, &mut bgId,
                                    &mut obstaclePos);
    if obstacleTop.y == -32000.0f32 { return }
    behindObstacleHeight = obstacleTop.y - (*this).actor.world.pos.y;
    if obstacleFloor.is_null() { return }
    ny =
        (*obstacleFloor).normal.y as libc::c_int as libc::c_float *
            (1.0f32 / 32767.0f32);
    if ny < 0.81915206f32 ||
           SurfaceType_IsHorseBlocked(&mut (*globalCtx).colCtx, obstacleFloor,
                                      bgId) != 0 ||
           func_80041D4C(&mut (*globalCtx).colCtx, obstacleFloor, bgId) ==
               7 as libc::c_int as libc::c_uint {
        if movingFast == 1 as libc::c_int &&
               (*this).action as libc::c_uint !=
                   ENHORSE_ACT_STOPPING as libc::c_int as libc::c_uint {
            (*this).stateFlags |=
                ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_uint;
            EnHorse_StartBraking(this, globalCtx);
        }
    } else if behindObstacleHeight <
                  -((*gGameInfo).data[(7 as libc::c_int * 6 as libc::c_int *
                                           16 as libc::c_int +
                                           4 as libc::c_int) as usize] as
                        libc::c_int) as libc::c_float {
        // -70
        if movingFast == 1 as libc::c_int &&
               (*this).action as libc::c_uint !=
                   ENHORSE_ACT_STOPPING as libc::c_int as libc::c_uint {
            (*this).stateFlags |=
                ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_uint;
            EnHorse_StartBraking(this, globalCtx);
        }
    } else if movingFast == 0 as libc::c_int && obstacleHeight > 19.0f32 &&
                  obstacleHeight <= 40.0f32 {
        EnHorse_Stub1(this);
        (*this).postDrawFunc =
            Some(EnHorse_LowJumpInit as
                     unsafe extern "C" fn(_: *mut EnHorse,
                                          _: *mut GlobalContext) -> ())
    } else if movingFast == 1 as libc::c_int &&
                  (*this).actor.speedXZ < 13.8f32 && obstacleHeight > 19.0f32
                  && obstacleHeight <= 72.0f32 ||
                  (*this).actor.speedXZ > 13.8f32 &&
                      obstacleHeight <= 112.0f32 {
        EnHorse_Stub2(this);
        (*this).postDrawFunc =
            Some(EnHorse_HighJumpInit as
                     unsafe extern "C" fn(_: *mut EnHorse,
                                          _: *mut GlobalContext) -> ())
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_CheckBoost(mut thisx: *mut EnHorse,
                                            mut globalCtx2:
                                                *mut GlobalContext) {
    let mut this: *mut EnHorse = thisx;
    let mut globalCtx: *mut GlobalContext = globalCtx2;
    let mut pad: s32 = 0;
    if (*this).action as libc::c_uint ==
           ENHORSE_ACT_MOUNTED_WALK as libc::c_int as libc::c_uint ||
           (*this).action as libc::c_uint ==
               ENHORSE_ACT_MOUNTED_TROT as libc::c_int as libc::c_uint ||
           (*this).action as libc::c_uint ==
               ENHORSE_ACT_MOUNTED_GALLOP as libc::c_int as libc::c_uint {
        if !((*globalCtx).state.input[0 as libc::c_int as usize].press.button
                 as libc::c_int | !(0x8000 as libc::c_int)) ==
               0 as libc::c_int &&
               (*globalCtx).interfaceCtx.unk_1EE as libc::c_int ==
                   8 as libc::c_int {
            if (*this).stateFlags &
                   ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint ==
                   0 &&
                   (*this).stateFlags &
                       ((1 as libc::c_int) << 8 as libc::c_int) as
                           libc::c_uint == 0 &&
                   (*this).stateFlags &
                       ((1 as libc::c_int) << 9 as libc::c_int) as
                           libc::c_uint == 0 {
                if (*this).numBoosts as libc::c_int > 0 as libc::c_int {
                    func_800AA000(0.0f32, 180 as libc::c_int as u8_0,
                                  20 as libc::c_int as u8_0,
                                  100 as libc::c_int as u8_0);
                    (*this).stateFlags |=
                        ((1 as libc::c_int) << 0 as libc::c_int) as
                            libc::c_uint;
                    (*this).stateFlags |=
                        ((1 as libc::c_int) << 22 as libc::c_int) as
                            libc::c_uint;
                    (*this).stateFlags |=
                        ((1 as libc::c_int) << 8 as libc::c_int) as
                            libc::c_uint;
                    (*this).numBoosts = (*this).numBoosts.wrapping_sub(1);
                    (*this).boostTimer = 0 as libc::c_int;
                    if (*this).numBoosts as libc::c_int == 0 as libc::c_int {
                        (*this).boostRegenTime = 140 as libc::c_int;
                        return
                    }
                    if (*this).type_0 == HORSE_EPONA as libc::c_int {
                        if (*this).stateFlags &
                               ((1 as libc::c_int) << 22 as libc::c_int) as
                                   libc::c_uint != 0 {
                            (*this).boostRegenTime = 60 as libc::c_int;
                            (*this).stateFlags &=
                                !((1 as libc::c_int) << 22 as libc::c_int) as
                                    libc::c_uint
                        } else {
                            (*this).boostRegenTime = 8 as libc::c_int
                            // Never happens
                        }
                    } else { (*this).boostRegenTime = 70 as libc::c_int }
                    return
                }
                (*this).unk_21C = (*this).unk_228;
                if (*this).stateFlags &
                       ((1 as libc::c_int) << 27 as libc::c_int) as
                           libc::c_uint != 0 {
                    if Rand_ZeroOne() < 0.1f32 {
                        Audio_PlaySoundGeneral(0x2805 as libc::c_int as u16_0,
                                               &mut (*this).unk_21C,
                                               4 as libc::c_int as u8_0,
                                               &mut D_801333E0,
                                               &mut D_801333E0,
                                               &mut D_801333E8);
                    }
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_RegenBoost(mut this: *mut EnHorse,
                                            mut globalCtx:
                                                *mut GlobalContext) {
    if ((*this).numBoosts as libc::c_int) < 6 as libc::c_int &&
           (*this).numBoosts as libc::c_int > 0 as libc::c_int {
        (*this).boostRegenTime -= 1;
        (*this).boostTimer += 1;
        if (*this).boostRegenTime <= 0 as libc::c_int {
            (*this).numBoosts =
                ((*this).numBoosts as libc::c_int + 1 as libc::c_int) as u8_0;
            if if ((*this).action as libc::c_uint ==
                       ENHORSE_ACT_MOUNTED_IDLE as libc::c_int as libc::c_uint
                       ||
                       (*this).action as libc::c_uint ==
                           ENHORSE_ACT_FROZEN as libc::c_int as libc::c_uint
                       ||
                       (*this).action as libc::c_uint ==
                           ENHORSE_ACT_MOUNTED_IDLE_WHINNEYING as libc::c_int
                               as libc::c_uint) &&
                      (*this).stateFlags &
                          ((1 as libc::c_int) << 19 as libc::c_int) as
                              libc::c_uint == 0 &&
                      (*this).stateFlags &
                          ((1 as libc::c_int) << 25 as libc::c_int) as
                              libc::c_uint == 0 {
                   1 as libc::c_int
               } else { 0 as libc::c_int } == 0 {
                Audio_PlaySoundGeneral(0x4845 as libc::c_int as u16_0,
                                       &mut D_801333D4,
                                       4 as libc::c_int as u8_0,
                                       &mut D_801333E0, &mut D_801333E0,
                                       &mut D_801333E8);
            }
            if ((*this).numBoosts as libc::c_int) < 6 as libc::c_int {
                (*this).boostRegenTime = 11 as libc::c_int
            }
        }
    } else if (*this).numBoosts as libc::c_int == 0 as libc::c_int {
        (*this).boostRegenTime -= 1;
        (*this).boostTimer += 1;
        if (*this).boostRegenTime <= 0 as libc::c_int {
            (*this).boostRegenTime = 0 as libc::c_int;
            (*this).numBoosts = 6 as libc::c_int as u8_0;
            if if ((*this).action as libc::c_uint ==
                       ENHORSE_ACT_MOUNTED_IDLE as libc::c_int as libc::c_uint
                       ||
                       (*this).action as libc::c_uint ==
                           ENHORSE_ACT_FROZEN as libc::c_int as libc::c_uint
                       ||
                       (*this).action as libc::c_uint ==
                           ENHORSE_ACT_MOUNTED_IDLE_WHINNEYING as libc::c_int
                               as libc::c_uint) &&
                      (*this).stateFlags &
                          ((1 as libc::c_int) << 19 as libc::c_int) as
                              libc::c_uint == 0 &&
                      (*this).stateFlags &
                          ((1 as libc::c_int) << 25 as libc::c_int) as
                              libc::c_uint == 0 {
                   1 as libc::c_int
               } else { 0 as libc::c_int } == 0 {
                Audio_PlaySoundGeneral(0x4845 as libc::c_int as u16_0,
                                       &mut D_801333D4,
                                       4 as libc::c_int as u8_0,
                                       &mut D_801333E0, &mut D_801333E0,
                                       &mut D_801333E8);
            }
        }
    }
    if (*this).boostTimer == 8 as libc::c_int && Rand_ZeroOne() < 0.25f32 {
        (*this).unk_21C = (*this).unk_228;
        if (*this).stateFlags &
               ((1 as libc::c_int) << 27 as libc::c_int) as libc::c_uint != 0
           {
            Audio_PlaySoundGeneral(0x2805 as libc::c_int as u16_0,
                                   &mut (*this).unk_21C,
                                   4 as libc::c_int as u8_0, &mut D_801333E0,
                                   &mut D_801333E0, &mut D_801333E8);
        }
    }
    (*globalCtx).interfaceCtx.numHorseBoosts = (*this).numBoosts;
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_UpdatePlayerDir(mut this: *mut EnHorse,
                                                 mut globalCtx:
                                                     *mut GlobalContext) {
    let mut pad: *mut EnHorse = 0 as *mut EnHorse;
    let mut angle: s16 = 0;
    let mut s: f32_0 = 0.;
    let mut c: f32_0 = 0.;
    angle =
        (Actor_WorldYawTowardActor(&mut (*this).actor,
                                   &mut (*((*(*globalCtx).actorCtx.actorLists.as_mut_ptr().offset(ACTORCAT_PLAYER
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      isize)).head
                                               as *mut Player)).actor) as
             libc::c_int - (*this).actor.world.rot.y as libc::c_int) as s16;
    s = Math_SinS(angle);
    c = Math_CosS(angle);
    if s > 0.8660254f32 {
        // sin(60 degrees)
        (*this).playerDir = PLAYER_DIR_SIDE_L as libc::c_int as u32_0
    } else if s < -0.8660254f32 {
        // -sin(60 degrees)
        (*this).playerDir = PLAYER_DIR_SIDE_R as libc::c_int as u32_0
    } else if c > 0.0f32 {
        if s > 0 as libc::c_int as libc::c_float {
            (*this).playerDir = PLAYER_DIR_FRONT_L as libc::c_int as u32_0
        } else {
            (*this).playerDir = PLAYER_DIR_FRONT_R as libc::c_int as u32_0
        }
    } else if s > 0 as libc::c_int as libc::c_float {
        (*this).playerDir = PLAYER_DIR_BACK_L as libc::c_int as u32_0
    } else { (*this).playerDir = PLAYER_DIR_BACK_R as libc::c_int as u32_0 };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_TiltBody(mut this: *mut EnHorse,
                                          mut globalCtx: *mut GlobalContext) {
    let mut speed: f32_0 = 0.;
    let mut rollDiff: f32_0 = 0.;
    let mut targetRoll: s32 = 0;
    let mut turnVel: s16 = 0;
    speed = (*this).actor.speedXZ / (*this).boostSpeed as libc::c_float;
    turnVel =
        ((*this).actor.shape.rot.y as libc::c_int -
             (*this).lastYaw as libc::c_int) as s16;
    targetRoll =
        -((1820.0f32 * speed *
               (turnVel as libc::c_int as libc::c_float / 480.00003f32)) as
              s16 as libc::c_int);
    rollDiff =
        (targetRoll - (*this).actor.world.rot.z as libc::c_int) as f32_0;
    if fabsf(targetRoll as f32_0) < 100.0f32 {
        (*this).actor.world.rot.z = 0 as libc::c_int as s16
    } else if fabsf(rollDiff) < 100.0f32 {
        (*this).actor.world.rot.z = targetRoll as s16
    } else if rollDiff > 0.0f32 {
        (*this).actor.world.rot.z =
            ((*this).actor.world.rot.z as libc::c_int + 100 as libc::c_int) as
                s16
    } else {
        (*this).actor.world.rot.z =
            ((*this).actor.world.rot.z as libc::c_int - 100 as libc::c_int) as
                s16
    }
    (*this).actor.shape.rot.z = (*this).actor.world.rot.z;
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_UpdateConveyors(mut this: *mut EnHorse,
                                                 mut globalCtx:
                                                     *mut GlobalContext)
 -> s32 {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut conveyorDir: s16 = 0;
    if (*this).actor.floorPoly.is_null() &&
           this != (*player).rideActor as *mut EnHorse {
        return 0 as libc::c_int
    }
    conveyorDir =
        SurfaceType_GetConveyorDirection(&mut (*globalCtx).colCtx,
                                         (*this).actor.floorPoly,
                                         (*this).actor.floorBgId as s32) as
            s16;
    conveyorDir =
        (((conveyorDir as libc::c_int) << 10 as libc::c_int) -
             (*this).actor.world.rot.y as libc::c_int) as s16;
    if conveyorDir as libc::c_int as libc::c_float > 800.0f32 {
        (*this).actor.world.rot.y =
            ((*this).actor.world.rot.y as libc::c_float + 800.0f32) as s16
    } else if (conveyorDir as libc::c_int as libc::c_float) < -800.0f32 {
        (*this).actor.world.rot.y =
            ((*this).actor.world.rot.y as libc::c_float - 800.0f32) as s16
    } else {
        (*this).actor.world.rot.y =
            ((*this).actor.world.rot.y as libc::c_int +
                 conveyorDir as libc::c_int) as s16
    }
    (*this).actor.shape.rot.y = (*this).actor.world.rot.y;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_RandInt(mut range: f32_0) -> s32 {
    return (Rand_ZeroOne() * range) as s32;
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_Update(mut thisx: *mut Actor,
                                        mut globalCtx2: *mut GlobalContext) {
    let mut this: *mut EnHorse = thisx as *mut EnHorse;
    let mut globalCtx: *mut GlobalContext = globalCtx2;
    let mut dustAcc: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
    let mut dustVel: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 1.0f32, z: 0.0f32,}; init };
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    (*this).lastYaw = (*thisx).shape.rot.y;
    EnHorse_UpdateStick(this, globalCtx);
    EnHorse_UpdatePlayerDir(this, globalCtx);
    if (*this).stateFlags &
           ((1 as libc::c_int) << 13 as libc::c_int) as libc::c_uint == 0 {
        EnHorse_MountDismount(this, globalCtx);
    }
    if (*this).stateFlags &
           ((1 as libc::c_int) << 19 as libc::c_int) as libc::c_uint != 0 {
        if (*this).stateFlags &
               ((1 as libc::c_int) << 20 as libc::c_int) as libc::c_uint != 0
               && (*this).inRace == 1 as libc::c_int {
            (*this).stateFlags &=
                !((1 as libc::c_int) << 20 as libc::c_int) as libc::c_uint;
            EnHorse_StartRearing(this);
        } else if (*this).stateFlags &
                      ((1 as libc::c_int) << 20 as libc::c_int) as
                          libc::c_uint == 0 &&
                      (*this).stateFlags &
                          ((1 as libc::c_int) << 21 as libc::c_int) as
                              libc::c_uint != 0 &&
                      (*this).action as libc::c_uint !=
                          ENHORSE_ACT_MOUNTED_REARING as libc::c_int as
                              libc::c_uint &&
                      (*this).inRace == 1 as libc::c_int {
            (*this).stateFlags &=
                !((1 as libc::c_int) << 21 as libc::c_int) as libc::c_uint;
            EnHorse_StartRearing(this);
        }
    }
    sActionFuncs[(*this).action as
                     usize].expect("non-null function pointer")(this,
                                                                globalCtx);
    (*this).stateFlags &=
        !((1 as libc::c_int) << 14 as libc::c_int) as libc::c_uint;
    (*this).curFrame = (*this).skin.skelAnime.curFrame;
    (*this).lastPos = (*thisx).world.pos;
    if (*this).stateFlags &
           ((1 as libc::c_int) << 13 as libc::c_int) as libc::c_uint == 0 {
        if (*this).action as libc::c_uint ==
               ENHORSE_ACT_MOUNTED_GALLOP as libc::c_int as libc::c_uint ||
               (*this).action as libc::c_uint ==
                   ENHORSE_ACT_MOUNTED_TROT as libc::c_int as libc::c_uint ||
               (*this).action as libc::c_uint ==
                   ENHORSE_ACT_MOUNTED_WALK as libc::c_int as libc::c_uint {
            EnHorse_CheckBoost(this, globalCtx);
        }
        if (*this).playerControlled == 1 as libc::c_int {
            EnHorse_RegenBoost(this, globalCtx);
        }
        Actor_MoveForward(thisx);
        if (*this).action as libc::c_uint ==
               ENHORSE_ACT_INGO_RACE as libc::c_int as libc::c_uint {
            if !(*this).rider.is_null() {
                (*(*this).rider).world.pos.x = (*thisx).world.pos.x;
                (*(*this).rider).world.pos.y = (*thisx).world.pos.y + 10.0f32;
                (*(*this).rider).world.pos.z = (*thisx).world.pos.z;
                (*(*this).rider).shape.rot.x = (*thisx).shape.rot.x;
                (*(*this).rider).shape.rot.y = (*thisx).shape.rot.y
            }
        }
        if (*(*this).jntSph.elements.offset(0 as libc::c_int as
                                                isize)).info.ocElemFlags as
               libc::c_int & 2 as libc::c_int != 0 {
            if (*thisx).speedXZ > 6.0f32 { (*thisx).speedXZ -= 1.0f32 }
        }
        if (*this).jntSph.base.acFlags as libc::c_int & 2 as libc::c_int != 0
           {
            (*this).unk_21C = (*this).unk_228;
            if (*this).stateFlags &
                   ((1 as libc::c_int) << 27 as libc::c_int) as libc::c_uint
                   != 0 {
                Audio_PlaySoundGeneral(0x2805 as libc::c_int as u16_0,
                                       &mut (*this).unk_21C,
                                       4 as libc::c_int as u8_0,
                                       &mut D_801333E0, &mut D_801333E0,
                                       &mut D_801333E8);
            }
        }
        if (*this).action as libc::c_uint !=
               ENHORSE_ACT_INGO_RACE as libc::c_int as libc::c_uint {
            EnHorse_TiltBody(this, globalCtx);
        }
        Collider_UpdateCylinder(thisx, &mut (*this).cyl1);
        Collider_UpdateCylinder(thisx, &mut (*this).cyl2);
        // Required to match
        (*this).cyl1.dim.pos.x =
            ((*this).cyl1.dim.pos.x as libc::c_int +
                 (Math_SinS((*thisx).shape.rot.y) * 11.0f32) as s16 as
                     libc::c_int) as s16;
        (*this).cyl1.dim.pos.z =
            ((*this).cyl1.dim.pos.z as libc::c_int +
                 (Math_CosS((*thisx).shape.rot.y) * 11.0f32) as s16 as
                     libc::c_int) as s16;
        (*this).cyl2.dim.pos.x =
            ((*this).cyl2.dim.pos.x as libc::c_int +
                 (Math_SinS((*thisx).shape.rot.y) * -18.0f32) as s16 as
                     libc::c_int) as s16;
        (*this).cyl2.dim.pos.z =
            ((*this).cyl2.dim.pos.z as libc::c_int +
                 (Math_CosS((*thisx).shape.rot.y) * -18.0f32) as s16 as
                     libc::c_int) as s16;
        CollisionCheck_SetAT(globalCtx, &mut (*globalCtx).colChkCtx,
                             &mut (*this).cyl1.base);
        CollisionCheck_SetOC(globalCtx, &mut (*globalCtx).colChkCtx,
                             &mut (*this).cyl1.base);
        CollisionCheck_SetOC(globalCtx, &mut (*globalCtx).colChkCtx,
                             &mut (*this).cyl2.base);
        if (*player).stateFlags1 & 1 as libc::c_int as libc::c_uint != 0 &&
               !(*player).rideActor.is_null() {
            if (*globalCtx).sceneNum as libc::c_int !=
                   SCENE_SPOT20 as libc::c_int ||
                   (*globalCtx).sceneNum as libc::c_int ==
                       SCENE_SPOT20 as libc::c_int &&
                       (*thisx).world.pos.z < -2400.0f32 {
                EnHorse_UpdateConveyors(this, globalCtx);
            }
        }
        if (*this).stateFlags &
               ((1 as libc::c_int) << 24 as libc::c_int) as libc::c_uint == 0
           {
            EnHorse_UpdateBgCheckInfo(this, globalCtx);
            EnHorse_CheckFloors(this, globalCtx);
            if (*thisx).world.pos.y < (*this).yFront &&
                   (*thisx).world.pos.y < (*this).yBack {
                if (*this).yBack < (*this).yFront {
                    (*thisx).world.pos.y = (*this).yBack
                } else { (*thisx).world.pos.y = (*this).yFront }
            }
        } else {
            (*this).stateFlags &=
                !((1 as libc::c_int) << 24 as libc::c_int) as libc::c_uint
        }
        if (*globalCtx).sceneNum as libc::c_int == SCENE_SPOT09 as libc::c_int
               &&
               gSaveContext.eventChkInf[9 as libc::c_int as usize] as
                   libc::c_int & 0xf as libc::c_int != 0xf as libc::c_int {
            EnHorse_CheckBridgeJumps(this, globalCtx);
        }
        (*thisx).focus.pos = (*thisx).world.pos;
        (*thisx).focus.pos.y += 70.0f32;
        if Rand_ZeroOne() < 0.025f32 &&
               (*this).blinkTimer as libc::c_int == 0 as libc::c_int {
            (*this).blinkTimer = (*this).blinkTimer.wrapping_add(1)
        } else if (*this).blinkTimer as libc::c_int > 0 as libc::c_int {
            (*this).blinkTimer = (*this).blinkTimer.wrapping_add(1);
            if (*this).blinkTimer as libc::c_int >= 4 as libc::c_int {
                (*this).blinkTimer = 0 as libc::c_int as u8_0
            }
        }
        if (*thisx).speedXZ == 0.0f32 &&
               (*this).stateFlags &
                   ((1 as libc::c_int) << 19 as libc::c_int) as libc::c_uint
                   == 0 {
            (*thisx).colChkInfo.mass = 0xff as libc::c_int as u8_0
        } else { (*thisx).colChkInfo.mass = 0xfe as libc::c_int as u8_0 }
        if (*thisx).speedXZ >= 5.0f32 {
            (*this).cyl1.base.atFlags =
                ((*this).cyl1.base.atFlags as libc::c_int | 1 as libc::c_int)
                    as u8_0
        } else {
            (*this).cyl1.base.atFlags =
                ((*this).cyl1.base.atFlags as libc::c_int &
                     !(1 as libc::c_int)) as u8_0
        }
        if gSaveContext.entranceIndex != 343 as libc::c_int ||
               gSaveContext.sceneSetupIndex != 9 as libc::c_int {
            if (*this).dustFlags as libc::c_int & 1 as libc::c_int != 0 {
                (*this).dustFlags =
                    ((*this).dustFlags as libc::c_int & !(1 as libc::c_int))
                        as u16_0;
                func_800287AC(globalCtx, &mut (*this).frontRightHoof,
                              &mut dustVel, &mut dustAcc,
                              (EnHorse_RandInt(100 as libc::c_int as f32_0) +
                                   200 as libc::c_int) as s16,
                              (EnHorse_RandInt(10 as libc::c_int as f32_0) +
                                   30 as libc::c_int) as s16,
                              (EnHorse_RandInt(20 as libc::c_int as f32_0) +
                                   30 as libc::c_int) as s16);
            } else if (*this).dustFlags as libc::c_int & 2 as libc::c_int != 0
             {
                (*this).dustFlags =
                    ((*this).dustFlags as libc::c_int & !(2 as libc::c_int))
                        as u16_0;
                func_800287AC(globalCtx, &mut (*this).frontLeftHoof,
                              &mut dustVel, &mut dustAcc,
                              (EnHorse_RandInt(100 as libc::c_int as f32_0) +
                                   200 as libc::c_int) as s16,
                              (EnHorse_RandInt(10 as libc::c_int as f32_0) +
                                   30 as libc::c_int) as s16,
                              (EnHorse_RandInt(20 as libc::c_int as f32_0) +
                                   30 as libc::c_int) as s16);
            } else if (*this).dustFlags as libc::c_int & 4 as libc::c_int != 0
             {
                (*this).dustFlags =
                    ((*this).dustFlags as libc::c_int & !(4 as libc::c_int))
                        as u16_0;
                func_800287AC(globalCtx, &mut (*this).backRightHoof,
                              &mut dustVel, &mut dustAcc,
                              (EnHorse_RandInt(100 as libc::c_int as f32_0) +
                                   200 as libc::c_int) as s16,
                              (EnHorse_RandInt(10 as libc::c_int as f32_0) +
                                   30 as libc::c_int) as s16,
                              (EnHorse_RandInt(20 as libc::c_int as f32_0) +
                                   30 as libc::c_int) as s16);
            } else if (*this).dustFlags as libc::c_int & 8 as libc::c_int != 0
             {
                (*this).dustFlags =
                    ((*this).dustFlags as libc::c_int & !(8 as libc::c_int))
                        as u16_0;
                func_800287AC(globalCtx, &mut (*this).backLeftHoof,
                              &mut dustVel, &mut dustAcc,
                              (EnHorse_RandInt(100 as libc::c_int as f32_0) +
                                   200 as libc::c_int) as s16,
                              (EnHorse_RandInt(10 as libc::c_int as f32_0) +
                                   30 as libc::c_int) as s16,
                              (EnHorse_RandInt(20 as libc::c_int as f32_0) +
                                   30 as libc::c_int) as s16);
            }
        }
        (*this).stateFlags &=
            !((1 as libc::c_int) << 27 as libc::c_int) as libc::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_PlayerDirToMountSide(mut this: *mut EnHorse,
                                                      mut globalCtx:
                                                          *mut GlobalContext,
                                                      mut player: *mut Player)
 -> s32 {
    if (*this).playerDir == PLAYER_DIR_SIDE_L as libc::c_int as libc::c_uint {
        return -(1 as libc::c_int)
    }
    if (*this).playerDir == PLAYER_DIR_SIDE_R as libc::c_int as libc::c_uint {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_MountSideCheck(mut this: *mut EnHorse,
                                                mut globalCtx:
                                                    *mut GlobalContext,
                                                mut player: *mut Player)
 -> s32 {
    let mut mountSide: s32 = 0;
    if Actor_WorldDistXZToActor(&mut (*this).actor, &mut (*player).actor) >
           75.0f32 {
        return 0 as libc::c_int
    } else {
        if fabsf((*this).actor.world.pos.y - (*player).actor.world.pos.y) >
               30.0f32 {
            return 0 as libc::c_int
        } else {
            if Math_CosS((Actor_WorldYawTowardActor(&mut (*player).actor,
                                                    &mut (*this).actor) as
                              libc::c_int -
                              (*player).actor.world.rot.y as libc::c_int) as
                             s16) < 0.17364818f32 {
                // cos(80 degrees)
                return 0 as libc::c_int
            } else {
                mountSide =
                    EnHorse_PlayerDirToMountSide(this, globalCtx, player);
                if mountSide == -(1 as libc::c_int) {
                    return -(1 as libc::c_int)
                }
                if mountSide == 1 as libc::c_int { return 1 as libc::c_int }
            }
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_GetMountSide(mut this: *mut EnHorse,
                                              mut globalCtx:
                                                  *mut GlobalContext) -> s32 {
    if (*this).action as libc::c_uint !=
           ENHORSE_ACT_IDLE as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    }
    if (*this).animationIdx != ENHORSE_ANIM_IDLE as libc::c_int &&
           (*this).animationIdx != ENHORSE_ANIM_WHINNEY as libc::c_int {
        return 0 as libc::c_int
    }
    return EnHorse_MountSideCheck(this, globalCtx,
                                  (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER
                                                                       as
                                                                       libc::c_int
                                                                       as
                                                                       usize].head
                                      as *mut Player);
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_RandomOffset(mut src: *mut Vec3f,
                                              mut dist: f32_0,
                                              mut dst: *mut Vec3f) {
    (*dst).x = Rand_ZeroOne() * (dist * 2.0f32) + (*src).x - dist;
    (*dst).y = Rand_ZeroOne() * (dist * 2.0f32) + (*src).y - dist;
    (*dst).z = Rand_ZeroOne() * (dist * 2.0f32) + (*src).z - dist;
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_SkinCallback1(mut thisx: *mut Actor,
                                               mut globalCtx:
                                                   *mut GlobalContext,
                                               mut skin: *mut PSkinAwb) {
    let mut this: *mut EnHorse = thisx as *mut EnHorse;
    let mut pad: s32 = 0;
    let mut sp94: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
    let mut hoofOffset: Vec3f =
        { let mut init = Vec3f{x: 5.0f32, y: -4.0f32, z: 5.0f32,}; init };
    let mut riderOffset: Vec3f =
        {
            let mut init = Vec3f{x: 600.0f32, y: -1670.0f32, z: 0.0f32,};
            init
        };
    let mut sp70: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp64: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
    let mut sp58: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: -1.0f32, z: 0.0f32,}; init };
    let mut frame: f32_0 = (*this).skin.skelAnime.curFrame;
    let mut center: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut newCenter: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut i: s32 = 0;
    let mut sp2C: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp28: f32_0 = 0.;
    if (*this).stateFlags &
           ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint == 0 {
        func_800A6408(skin, 30 as libc::c_int, &mut riderOffset,
                      &mut (*this).riderPos);
        (*this).riderPos.x = (*this).riderPos.x - (*this).actor.world.pos.x;
        (*this).riderPos.y = (*this).riderPos.y - (*this).actor.world.pos.y;
        (*this).riderPos.z = (*this).riderPos.z - (*this).actor.world.pos.z
    } else {
        (*this).stateFlags &=
            !((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint
    }
    func_800A6408(skin, 13 as libc::c_int, &mut sp94, &mut sp2C);
    SkinMatrix_Vec3fMtxFMultXYZW(&mut (*globalCtx).viewProjectionMtxF,
                                 &mut sp2C, &mut (*this).unk_228, &mut sp28);
    if (*this).animationIdx == ENHORSE_ANIM_IDLE as libc::c_int &&
           (*this).action as libc::c_uint !=
               ENHORSE_ACT_FROZEN as libc::c_int as libc::c_uint &&
           (frame > 40.0f32 && frame < 45.0f32 &&
                (*this).type_0 == HORSE_EPONA as libc::c_int ||
                frame > 28.0f32 && frame < 33.0f32 &&
                    (*this).type_0 == HORSE_HNI as libc::c_int) {
        if Rand_ZeroOne() < 0.6f32 {
            (*this).dustFlags =
                ((*this).dustFlags as libc::c_int | 1 as libc::c_int) as
                    u16_0;
            func_800A6408(skin, 28 as libc::c_int, &mut hoofOffset,
                          &mut (*this).frontRightHoof);
            (*this).frontRightHoof.y = (*this).frontRightHoof.y - 5.0f32
        }
    } else if (*this).action as libc::c_uint ==
                  ENHORSE_ACT_STOPPING as libc::c_int as libc::c_uint {
        if frame > 10.0f32 && frame < 13.0f32 ||
               frame > 25.0f32 && frame < 33.0f32 {
            if Rand_ZeroOne() < 0.7f32 {
                (*this).dustFlags =
                    ((*this).dustFlags as libc::c_int | 2 as libc::c_int) as
                        u16_0;
                func_800A6408(skin, 20 as libc::c_int, &mut hoofOffset,
                              &mut sp70);
                EnHorse_RandomOffset(&mut sp70, 10.0f32,
                                     &mut (*this).frontLeftHoof);
            }
            if Rand_ZeroOne() < 0.7f32 {
                (*this).dustFlags =
                    ((*this).dustFlags as libc::c_int | 1 as libc::c_int) as
                        u16_0;
                func_800A6408(skin, 28 as libc::c_int, &mut hoofOffset,
                              &mut sp70);
                EnHorse_RandomOffset(&mut sp70, 10.0f32,
                                     &mut (*this).frontRightHoof);
            }
        }
        if frame > 6.0f32 && frame < 10.0f32 ||
               frame > 23.0f32 && frame < 29.0f32 {
            if Rand_ZeroOne() < 0.7f32 {
                (*this).dustFlags =
                    ((*this).dustFlags as libc::c_int | 8 as libc::c_int) as
                        u16_0;
                func_800A6408(skin, 37 as libc::c_int, &mut hoofOffset,
                              &mut sp70);
                EnHorse_RandomOffset(&mut sp70, 10.0f32,
                                     &mut (*this).backLeftHoof);
            }
        }
        if frame > 7.0f32 && frame < 14.0f32 ||
               frame > 26.0f32 && frame < 30.0f32 {
            if Rand_ZeroOne() < 0.7f32 {
                (*this).dustFlags =
                    ((*this).dustFlags as libc::c_int | 4 as libc::c_int) as
                        u16_0;
                func_800A6408(skin, 45 as libc::c_int, &mut hoofOffset,
                              &mut sp70);
                EnHorse_RandomOffset(&mut sp70, 10.0f32,
                                     &mut (*this).backRightHoof);
            }
        }
    } else if (*this).animationIdx == ENHORSE_ANIM_GALLOP as libc::c_int {
        if frame > 14.0f32 && frame < 16.0f32 {
            (*this).dustFlags =
                ((*this).dustFlags as libc::c_int | 1 as libc::c_int) as
                    u16_0;
            func_800A6408(skin, 28 as libc::c_int, &mut hoofOffset,
                          &mut sp70);
            EnHorse_RandomOffset(&mut sp70, 5.0f32,
                                 &mut (*this).frontRightHoof);
        } else if frame > 8.0f32 && frame < 10.0f32 {
            (*this).dustFlags =
                ((*this).dustFlags as libc::c_int | 2 as libc::c_int) as
                    u16_0;
            func_800A6408(skin, 20 as libc::c_int, &mut hoofOffset,
                          &mut sp70);
            EnHorse_RandomOffset(&mut sp70, 10.0f32,
                                 &mut (*this).frontLeftHoof);
        } else if frame > 1.0f32 && frame < 3.0f32 {
            (*this).dustFlags =
                ((*this).dustFlags as libc::c_int | 4 as libc::c_int) as
                    u16_0;
            func_800A6408(skin, 45 as libc::c_int, &mut hoofOffset,
                          &mut sp70);
            EnHorse_RandomOffset(&mut sp70, 10.0f32,
                                 &mut (*this).backRightHoof);
        } else if frame > 26.0f32 && frame < 28.0f32 {
            (*this).dustFlags =
                ((*this).dustFlags as libc::c_int | 8 as libc::c_int) as
                    u16_0;
            func_800A6408(skin, 37 as libc::c_int, &mut hoofOffset,
                          &mut sp70);
            EnHorse_RandomOffset(&mut sp70, 10.0f32,
                                 &mut (*this).backLeftHoof);
        }
    } else if (*this).action as libc::c_uint ==
                  ENHORSE_ACT_LOW_JUMP as libc::c_int as libc::c_uint &&
                  frame > 6.0f32 &&
                  Rand_ZeroOne() <
                      1.0f32 - (frame - 6.0f32) * (1.0f32 / 17.0f32) {
        if Rand_ZeroOne() < 0.5f32 {
            (*this).dustFlags =
                ((*this).dustFlags as libc::c_int | 8 as libc::c_int) as
                    u16_0;
            func_800A6408(skin, 37 as libc::c_int, &mut hoofOffset,
                          &mut sp70);
            EnHorse_RandomOffset(&mut sp70, 10.0f32,
                                 &mut (*this).backLeftHoof);
        }
        if Rand_ZeroOne() < 0.5f32 {
            (*this).dustFlags =
                ((*this).dustFlags as libc::c_int | 4 as libc::c_int) as
                    u16_0;
            func_800A6408(skin, 45 as libc::c_int, &mut hoofOffset,
                          &mut sp70);
            EnHorse_RandomOffset(&mut sp70, 10.0f32,
                                 &mut (*this).backRightHoof);
        }
    } else if (*this).action as libc::c_uint ==
                  ENHORSE_ACT_HIGH_JUMP as libc::c_int as libc::c_uint &&
                  frame > 5.0f32 &&
                  Rand_ZeroOne() <
                      1.0f32 - (frame - 5.0f32) * (1.0f32 / 25.0f32) {
        if Rand_ZeroOne() < 0.5f32 {
            (*this).dustFlags =
                ((*this).dustFlags as libc::c_int | 8 as libc::c_int) as
                    u16_0;
            func_800A6408(skin, 37 as libc::c_int, &mut hoofOffset,
                          &mut sp70);
            EnHorse_RandomOffset(&mut sp70, 10.0f32,
                                 &mut (*this).backLeftHoof);
        }
        if Rand_ZeroOne() < 0.5f32 {
            (*this).dustFlags =
                ((*this).dustFlags as libc::c_int | 4 as libc::c_int) as
                    u16_0;
            func_800A6408(skin, 45 as libc::c_int, &mut hoofOffset,
                          &mut sp70);
            EnHorse_RandomOffset(&mut sp70, 10.0f32,
                                 &mut (*this).backRightHoof);
        }
    } else if (*this).action as libc::c_uint ==
                  ENHORSE_ACT_BRIDGE_JUMP as libc::c_int as libc::c_uint &&
                  Rand_ZeroOne() < 0.5f32 {
        if Rand_ZeroOne() < 0.5f32 {
            (*this).dustFlags =
                ((*this).dustFlags as libc::c_int | 8 as libc::c_int) as
                    u16_0;
            func_800A6408(skin, 37 as libc::c_int, &mut hoofOffset,
                          &mut sp70);
            EnHorse_RandomOffset(&mut sp70, 10.0f32,
                                 &mut (*this).backLeftHoof);
        } else {
            (*this).dustFlags =
                ((*this).dustFlags as libc::c_int | 4 as libc::c_int) as
                    u16_0;
            func_800A6408(skin, 45 as libc::c_int, &mut hoofOffset,
                          &mut sp70);
            EnHorse_RandomOffset(&mut sp70, 10.0f32,
                                 &mut (*this).backRightHoof);
        }
    }
    i = 0 as libc::c_int;
    while i < (*this).jntSph.count {
        center.x =
            (*(*this).jntSph.elements.offset(i as
                                                 isize)).dim.modelSphere.center.x
                as f32_0;
        center.y =
            (*(*this).jntSph.elements.offset(i as
                                                 isize)).dim.modelSphere.center.y
                as f32_0;
        center.z =
            (*(*this).jntSph.elements.offset(i as
                                                 isize)).dim.modelSphere.center.z
                as f32_0;
        func_800A6408(skin,
                      (*(*this).jntSph.elements.offset(i as isize)).dim.limb
                          as s32, &mut center, &mut newCenter);
        (*(*this).jntSph.elements.offset(i as isize)).dim.worldSphere.center.x
            = newCenter.x as s16;
        (*(*this).jntSph.elements.offset(i as isize)).dim.worldSphere.center.y
            = newCenter.y as s16;
        (*(*this).jntSph.elements.offset(i as isize)).dim.worldSphere.center.z
            = newCenter.z as s16;
        (*(*this).jntSph.elements.offset(i as isize)).dim.worldSphere.radius =
            ((*(*this).jntSph.elements.offset(i as
                                                  isize)).dim.modelSphere.radius
                 as libc::c_int as libc::c_float *
                 (*(*this).jntSph.elements.offset(i as isize)).dim.scale) as
                s16;
        i += 1
    }
    // ! @bug Setting colliders in a draw function allows for duplicate entries to be added to their respective lists
    // ! under certain conditions, like when pausing and unpausing the game.
    // ! Actors will draw for a couple of frames between the pauses, but some important logic updates will not occur.
    // ! In the case of OC, this can cause unwanted effects such as a very large amount of displacement being applied to
    // ! a colliding actor.
    CollisionCheck_SetOC(globalCtx, &mut (*globalCtx).colChkCtx,
                         &mut (*this).jntSph.base);
    CollisionCheck_SetAC(globalCtx, &mut (*globalCtx).colChkCtx,
                         &mut (*this).jntSph.base);
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_SkinCallback2(mut thisx: *mut Actor,
                                               mut globalCtx:
                                                   *mut GlobalContext,
                                               mut limbIndex: s32,
                                               mut arg3: *mut PSkinAwb)
 -> s32 {
    static mut eyeTextures: [*mut libc::c_void; 3] =
        unsafe {
            [gEponaEyeOpenTex.as_ptr() as *mut _ as *mut libc::c_void,
             gEponaEyeHalfTex.as_ptr() as *mut _ as *mut libc::c_void,
             gEponaEyeClosedTex.as_ptr() as *mut _ as *mut libc::c_void]
        };
    static mut eyeBlinkIndexes: [u8_0; 4] =
        [0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
         2 as libc::c_int as u8_0, 1 as libc::c_int as u8_0];
    let mut this: *mut EnHorse = thisx as *mut EnHorse;
    let mut drawOriginalLimb: s32 = 1 as libc::c_int;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_en_horse.c\x00" as *const u8 as
                        *const libc::c_char, 8582 as libc::c_int);
    if limbIndex == 13 as libc::c_int &&
           (*this).type_0 == HORSE_EPONA as libc::c_int {
        let mut index: u8_0 = eyeBlinkIndexes[(*this).blinkTimer as usize];
        let fresh0 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g: *mut Gfx = fresh0;
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
            gSegments[((eyeTextures[index as usize] as u32_0) <<
                           4 as libc::c_int >> 28 as libc::c_int) as
                          usize].wrapping_add(eyeTextures[index as usize] as
                                                  u32_0 &
                                                  0xffffff as libc::c_int as
                                                      libc::c_uint).wrapping_add(0x80000000
                                                                                     as
                                                                                     libc::c_uint)
                as *mut libc::c_void as libc::c_uint
    } else if (*this).type_0 == HORSE_HNI as libc::c_int &&
                  (*this).stateFlags &
                      ((1 as libc::c_int) << 18 as libc::c_int) as
                          libc::c_uint != 0 && limbIndex == 30 as libc::c_int
     {
        func_800A5F60((*globalCtx).state.gfxCtx, &mut (*this).skin, limbIndex,
                      gHorseIngoGerudoSaddleDL.as_mut_ptr(),
                      0 as libc::c_int);
        drawOriginalLimb = 0 as libc::c_int
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_en_horse.c\x00" as *const u8 as
                         *const libc::c_char, 8601 as libc::c_int);
    return drawOriginalLimb;
}
#[no_mangle]
pub unsafe extern "C" fn EnHorse_Draw(mut thisx: *mut Actor,
                                      mut globalCtx: *mut GlobalContext) {
    let mut this: *mut EnHorse = thisx as *mut EnHorse;
    if (*this).stateFlags &
           ((1 as libc::c_int) << 13 as libc::c_int) as libc::c_uint == 0 {
        func_80093D18((*globalCtx).state.gfxCtx);
        (*this).stateFlags |=
            ((1 as libc::c_int) << 27 as libc::c_int) as libc::c_uint;
        if (*this).stateFlags &
               ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint != 0 {
            func_800A6360(thisx, globalCtx, &mut (*this).skin,
                          Some(EnHorse_SkinCallback1 as
                                   unsafe extern "C" fn(_: *mut Actor,
                                                        _: *mut GlobalContext,
                                                        _: *mut PSkinAwb)
                                       -> ()),
                          Some(EnHorse_SkinCallback2 as
                                   unsafe extern "C" fn(_: *mut Actor,
                                                        _: *mut GlobalContext,
                                                        _: s32,
                                                        _: *mut PSkinAwb)
                                       -> s32), 0 as libc::c_int);
        } else {
            func_800A6360(thisx, globalCtx, &mut (*this).skin,
                          Some(EnHorse_SkinCallback1 as
                                   unsafe extern "C" fn(_: *mut Actor,
                                                        _: *mut GlobalContext,
                                                        _: *mut PSkinAwb)
                                       -> ()),
                          Some(EnHorse_SkinCallback2 as
                                   unsafe extern "C" fn(_: *mut Actor,
                                                        _: *mut GlobalContext,
                                                        _: s32,
                                                        _: *mut PSkinAwb)
                                       -> s32), 1 as libc::c_int);
        }
        if (*this).postDrawFunc.is_some() {
            (*this).postDrawFunc.expect("non-null function pointer")(this,
                                                                     globalCtx);
        }
    };
}
unsafe extern "C" fn run_static_initializers() {
    sInitChain =
        [{
             let mut init = InitChainEntry{cont_type_0_offset_value: [0; 4],};
             init.set_cont(1 as libc::c_int as u32_0);
             init.set_type_0(ICHAINTYPE_F32 as libc::c_int as u32_0);
             init.set_offset(&mut (*(0 as *mut Actor)).uncullZoneScale as
                                 *mut f32_0 as size_t as u32_0);
             init.set_value(600 as libc::c_int);
             init
         },
         {
             let mut init = InitChainEntry{cont_type_0_offset_value: [0; 4],};
             init.set_cont(0 as libc::c_int as u32_0);
             init.set_type_0(ICHAINTYPE_F32 as libc::c_int as u32_0);
             init.set_offset(&mut (*(0 as *mut Actor)).uncullZoneDownward as
                                 *mut f32_0 as size_t as u32_0);
             init.set_value(300 as libc::c_int);
             init
         }]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
