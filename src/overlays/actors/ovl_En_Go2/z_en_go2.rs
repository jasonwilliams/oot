#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, register_tool)]
extern "C" {
    #[no_mangle]
    fn fabsf(f: f32_0) -> f32_0;
    #[no_mangle]
    fn ActorShape_Init(shape: *mut ActorShape, yOffset: f32_0,
                       shadowDraw: ActorShadowFunc, shadowScale: f32_0);
    #[no_mangle]
    fn ActorShadow_DrawCircle(actor: *mut Actor, lights: *mut Lights,
                              globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn Flags_GetSwitch(globalCtx: *mut GlobalContext, flag: s32) -> s32;
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
    fn Actor_ProcessTalkRequest(actor: *mut Actor,
                                globalCtx: *mut GlobalContext) -> u32_0;
    #[no_mangle]
    fn func_8002F2CC(actor: *mut Actor, globalCtx: *mut GlobalContext,
                     arg2: f32_0) -> s32;
    #[no_mangle]
    fn func_8002F368(globalCtx: *mut GlobalContext) -> s8;
    #[no_mangle]
    fn Actor_HasParent(actor: *mut Actor, globalCtx: *mut GlobalContext)
     -> u32_0;
    #[no_mangle]
    fn func_8002F434(actor: *mut Actor, globalCtx: *mut GlobalContext,
                     getItemId: s32, xzRange: f32_0, yRange: f32_0) -> s32;
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
    fn func_800343CC(globalCtx: *mut GlobalContext, actor: *mut Actor,
                     arg2: *mut s16, interactRange: f32_0,
                     unkFunc1:
                         Option<unsafe extern "C" fn(_: *mut GlobalContext,
                                                     _: *mut Actor) -> u16_0>,
                     unkFunc2:
                         Option<unsafe extern "C" fn(_: *mut GlobalContext,
                                                     _: *mut Actor) -> s16>)
     -> s32;
    #[no_mangle]
    fn func_80034A14(actor: *mut Actor, arg1: *mut struct_80034A14_arg1,
                     arg2: s16, arg3: s16);
    #[no_mangle]
    fn func_80034EC0(skelAnime: *mut SkelAnime,
                     animations: *mut struct_80034EC0_Entry, index: s32);
    #[no_mangle]
    fn func_80034F54(globalCtx: *mut GlobalContext, arg1: *mut s16,
                     arg2: *mut s16, arg3: s32);
    #[no_mangle]
    fn Camera_ChangeSetting(camera: *mut Camera, setting: s16) -> s32;
    #[no_mangle]
    fn func_8005ACFC(camera: *mut Camera, arg1: s16) -> s16;
    #[no_mangle]
    fn func_8005AD1C(camera: *mut Camera, arg1: s16) -> s16;
    #[no_mangle]
    fn func_8005B1A4(camera: *mut Camera) -> s16;
    #[no_mangle]
    fn Collider_InitCylinder(globalCtx: *mut GlobalContext,
                             collider: *mut ColliderCylinder) -> s32;
    #[no_mangle]
    fn Collider_SetCylinder(globalCtx: *mut GlobalContext,
                            collider: *mut ColliderCylinder,
                            actor: *mut Actor, src: *mut ColliderCylinderInit)
     -> s32;
    #[no_mangle]
    fn CollisionCheck_SetAC(globalCtx: *mut GlobalContext,
                            colChkCtx: *mut CollisionCheckContext,
                            collider: *mut Collider) -> s32;
    #[no_mangle]
    fn CollisionCheck_SetOC(globalCtx: *mut GlobalContext,
                            colChkCtx: *mut CollisionCheckContext,
                            collider: *mut Collider) -> s32;
    #[no_mangle]
    fn CollisionCheck_SetInfo2(info: *mut CollisionCheckInfo,
                               damageTable: *mut DamageTable,
                               init: *mut CollisionCheckInfoInit2);
    #[no_mangle]
    fn Audio_PlaySoundAtPosition(globalCtx: *mut GlobalContext,
                                 pos: *mut Vec3f, duration: s32,
                                 sfxId: u16_0);
    #[no_mangle]
    fn Text_GetFaceReaction(globalCtx: *mut GlobalContext, reactionSet: u32_0)
     -> u16_0;
    #[no_mangle]
    fn Environment_GetBgsDayCount() -> s32;
    #[no_mangle]
    fn Environment_ClearBgsDayCount();
    #[no_mangle]
    fn Math_CosS(angle: s16) -> f32_0;
    #[no_mangle]
    fn Math_SinS(angle: s16) -> f32_0;
    #[no_mangle]
    fn Rand_S16Offset(base: s16, range: s16) -> s16;
    #[no_mangle]
    fn Math_Vec3f_Yaw(a: *mut Vec3f, b: *mut Vec3f) -> s16;
    #[no_mangle]
    fn Math_ApproachF(pValue: *mut f32_0, target: f32_0, fraction: f32_0,
                      step: f32_0);
    #[no_mangle]
    fn Math_SmoothStepToS(pValue: *mut s16, target: s16, scale: s16,
                          step: s16, minStep: s16) -> s16;
    #[no_mangle]
    fn OnePointCutscene_Init(globalCtx: *mut GlobalContext, csId: s16,
                             timer: s16, actor: *mut Actor, camIdx: s16)
     -> s16;
    #[no_mangle]
    fn Path_GetByIndex(globalCtx: *mut GlobalContext, index: s16, max: s16)
     -> *mut Path;
    #[no_mangle]
    fn Path_OrientAndGetDistSq(actor: *mut Actor, path: *mut Path,
                               waypoint: s16, yaw: *mut s16) -> f32_0;
    #[no_mangle]
    fn Path_CopyLastPoint(path: *mut Path, dest: *mut Vec3f);
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
    fn func_80093D18(gfxCtx: *mut GraphicsContext);
    #[no_mangle]
    fn func_80093D84(gfxCtx: *mut GraphicsContext);
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
    fn Animation_OnFrame(skelAnime: *mut SkelAnime, frame: f32_0) -> s32;
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
    fn func_800F4524(pos: *mut Vec3f, sfxId: u16_0, arg2: s8);
    #[no_mangle]
    fn func_800F483C(targetVol: u8_0, volFadeTimer: u8_0);
    #[no_mangle]
    fn Audio_PlayFanfare(_: u16_0);
    #[no_mangle]
    fn Audio_PlaySoundGeneral(sfxId: u16_0, pos: *mut Vec3f, token: u8_0,
                              freqScale: *mut f32_0, a4: *mut f32_0,
                              reverbAdd: *mut s8);
    #[no_mangle]
    fn Audio_StopSfxById(sfxId: u32_0);
    #[no_mangle]
    fn Rand_ZeroOne() -> f32_0;
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
    static mut gSaveContext: SaveContext;
    #[no_mangle]
    static mut D_801333E8: s8;
    #[no_mangle]
    static mut D_801333E0: f32_0;
    #[no_mangle]
    static mut D_801333D4: Vec3f;
    #[no_mangle]
    static mut gItemSlots: [u8_0; 56];
    #[no_mangle]
    static mut gUpgradeCapacities: [[u16_0; 4]; 8];
    #[no_mangle]
    static mut gUpgradeShifts: [u8_0; 8];
    #[no_mangle]
    static mut gEquipShifts: [u8_0; 4];
    #[no_mangle]
    static mut gUpgradeMasks: [u32_0; 8];
    #[no_mangle]
    static mut gBitFlags: [u32_0; 32];
    #[no_mangle]
    static mut gDust1Tex: [u64_0; 0];
    #[no_mangle]
    static mut gDust2Tex: [u64_0; 0];
    #[no_mangle]
    static mut gDust3Tex: [u64_0; 0];
    #[no_mangle]
    static mut gDust4Tex: [u64_0; 0];
    #[no_mangle]
    static mut gDust5Tex: [u64_0; 0];
    #[no_mangle]
    static mut gDust6Tex: [u64_0; 0];
    #[no_mangle]
    static mut gDust7Tex: [u64_0; 0];
    #[no_mangle]
    static mut gDust8Tex: [u64_0; 0];
    #[no_mangle]
    static mut gGoronAnim_000750: AnimationHeader;
    #[no_mangle]
    static mut gGoronAnim_000D5C: AnimationHeader;
    #[no_mangle]
    static mut gGoronAnim_00161C: AnimationHeader;
    #[no_mangle]
    static mut gGoronAnim_001A00: AnimationHeader;
    #[no_mangle]
    static mut gGoronAnim_0021D0: AnimationHeader;
    #[no_mangle]
    static mut gGoronAnim_0029A8: AnimationHeader;
    #[no_mangle]
    static mut gGoronAnim_002D80: AnimationHeader;
    #[no_mangle]
    static mut gGoronAnim_003768: AnimationHeader;
    #[no_mangle]
    static mut gGoronAnim_0038E4: AnimationHeader;
    #[no_mangle]
    static mut gGoronAnim_004930: AnimationHeader;
    #[no_mangle]
    static mut gGoronDL_00BD80: [Gfx; 0];
    #[no_mangle]
    static mut gGoronDL_00C140: [Gfx; 0];
    #[no_mangle]
    static mut gGoronCsEyeOpenTex: [u64_0; 0];
    #[no_mangle]
    static mut gGoronCsEyeHalfTex: [u64_0; 0];
    #[no_mangle]
    static mut gGoronCsEyeClosedTex: [u64_0; 0];
    #[no_mangle]
    static mut gGoronCsEyeClosed2Tex: [u64_0; 0];
    #[no_mangle]
    static mut gGoronCsMouthNeutralTex: [u64_0; 0];
    #[no_mangle]
    static mut gGoronCsMouthSmileTex: [u64_0; 0];
    #[no_mangle]
    static mut gGoronDL_00FD40: [Gfx; 0];
    #[no_mangle]
    static mut gGoronDL_00FD50: [Gfx; 0];
    #[no_mangle]
    static mut gGoronSkel: FlexSkeletonHeader;
    #[no_mangle]
    static mut gGoronAnim_010590: AnimationHeader;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CollisionCheckInfoInit2 {
    pub health: u8_0,
    pub cylRadius: s16,
    pub cylHeight: s16,
    pub cylYShift: s16,
    pub mass: u8_0,
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
pub const EQUIP_BOOTS: C2RustUnnamed_21 = 3;
pub const EQUIP_TUNIC: C2RustUnnamed_21 = 2;
pub const EQUIP_SHIELD: C2RustUnnamed_21 = 1;
pub const EQUIP_SWORD: C2RustUnnamed_21 = 0;
pub type C2RustUnnamed_22 = libc::c_uint;
pub const UPG_NUTS: C2RustUnnamed_22 = 7;
pub const UPG_STICKS: C2RustUnnamed_22 = 6;
pub const UPG_BULLET_BAG: C2RustUnnamed_22 = 5;
pub const UPG_WALLET: C2RustUnnamed_22 = 4;
pub const UPG_SCALE: C2RustUnnamed_22 = 3;
pub const UPG_STRENGTH: C2RustUnnamed_22 = 2;
pub const UPG_BOMB_BAG: C2RustUnnamed_22 = 1;
pub const UPG_QUIVER: C2RustUnnamed_22 = 0;
pub type C2RustUnnamed_23 = libc::c_uint;
pub const QUEST_HEART_PIECE: C2RustUnnamed_23 = 24;
pub const QUEST_SKULL_TOKEN: C2RustUnnamed_23 = 23;
pub const QUEST_GERUDO_CARD: C2RustUnnamed_23 = 22;
pub const QUEST_STONE_OF_AGONY: C2RustUnnamed_23 = 21;
pub const QUEST_ZORA_SAPPHIRE: C2RustUnnamed_23 = 20;
pub const QUEST_GORON_RUBY: C2RustUnnamed_23 = 19;
pub const QUEST_KOKIRI_EMERALD: C2RustUnnamed_23 = 18;
pub const QUEST_SONG_STORMS: C2RustUnnamed_23 = 17;
pub const QUEST_SONG_TIME: C2RustUnnamed_23 = 16;
pub const QUEST_SONG_SUN: C2RustUnnamed_23 = 15;
pub const QUEST_SONG_SARIA: C2RustUnnamed_23 = 14;
pub const QUEST_SONG_EPONA: C2RustUnnamed_23 = 13;
pub const QUEST_SONG_LULLABY: C2RustUnnamed_23 = 12;
pub const QUEST_SONG_PRELUDE: C2RustUnnamed_23 = 11;
pub const QUEST_SONG_NOCTURNE: C2RustUnnamed_23 = 10;
pub const QUEST_SONG_REQUIEM: C2RustUnnamed_23 = 9;
pub const QUEST_SONG_SERENADE: C2RustUnnamed_23 = 8;
pub const QUEST_SONG_BOLERO: C2RustUnnamed_23 = 7;
pub const QUEST_SONG_MINUET: C2RustUnnamed_23 = 6;
pub const QUEST_MEDALLION_LIGHT: C2RustUnnamed_23 = 5;
pub const QUEST_MEDALLION_SHADOW: C2RustUnnamed_23 = 4;
pub const QUEST_MEDALLION_SPIRIT: C2RustUnnamed_23 = 3;
pub const QUEST_MEDALLION_WATER: C2RustUnnamed_23 = 2;
pub const QUEST_MEDALLION_FIRE: C2RustUnnamed_23 = 1;
pub const QUEST_MEDALLION_FOREST: C2RustUnnamed_23 = 0;
pub type C2RustUnnamed_24 = libc::c_uint;
pub const ITEM_NONE: C2RustUnnamed_24 = 255;
pub const ITEM_NONE_FE: C2RustUnnamed_24 = 254;
pub const ITEM_LAST_USED: C2RustUnnamed_24 = 252;
pub const ITEM_NUT_UPGRADE_40: C2RustUnnamed_24 = 155;
pub const ITEM_NUT_UPGRADE_30: C2RustUnnamed_24 = 154;
pub const ITEM_STICK_UPGRADE_30: C2RustUnnamed_24 = 153;
pub const ITEM_STICK_UPGRADE_20: C2RustUnnamed_24 = 152;
pub const ITEM_BOMBCHUS_20: C2RustUnnamed_24 = 151;
pub const ITEM_BOMBCHUS_5: C2RustUnnamed_24 = 150;
pub const ITEM_SEEDS_30: C2RustUnnamed_24 = 149;
pub const ITEM_ARROWS_LARGE: C2RustUnnamed_24 = 148;
pub const ITEM_ARROWS_MEDIUM: C2RustUnnamed_24 = 147;
pub const ITEM_ARROWS_SMALL: C2RustUnnamed_24 = 146;
pub const ITEM_BOMBS_30: C2RustUnnamed_24 = 145;
pub const ITEM_BOMBS_20: C2RustUnnamed_24 = 144;
pub const ITEM_BOMBS_10: C2RustUnnamed_24 = 143;
pub const ITEM_BOMBS_5: C2RustUnnamed_24 = 142;
pub const ITEM_NUTS_10: C2RustUnnamed_24 = 141;
pub const ITEM_NUTS_5: C2RustUnnamed_24 = 140;
pub const ITEM_STICKS_10: C2RustUnnamed_24 = 139;
pub const ITEM_STICKS_5: C2RustUnnamed_24 = 138;
pub const ITEM_INVALID_8: C2RustUnnamed_24 = 137;
pub const ITEM_RUPEE_GOLD: C2RustUnnamed_24 = 136;
pub const ITEM_RUPEE_PURPLE: C2RustUnnamed_24 = 135;
pub const ITEM_RUPEE_RED: C2RustUnnamed_24 = 134;
pub const ITEM_RUPEE_BLUE: C2RustUnnamed_24 = 133;
pub const ITEM_RUPEE_GREEN: C2RustUnnamed_24 = 132;
pub const ITEM_HEART: C2RustUnnamed_24 = 131;
pub const ITEM_MILK: C2RustUnnamed_24 = 130;
pub const ITEM_INVALID_7: C2RustUnnamed_24 = 129;
pub const ITEM_INVALID_6: C2RustUnnamed_24 = 128;
pub const ITEM_INVALID_5: C2RustUnnamed_24 = 127;
pub const ITEM_INVALID_4: C2RustUnnamed_24 = 126;
pub const ITEM_INVALID_3: C2RustUnnamed_24 = 125;
pub const ITEM_INVALID_2: C2RustUnnamed_24 = 124;
pub const ITEM_INVALID_1: C2RustUnnamed_24 = 123;
pub const ITEM_HEART_PIECE_2: C2RustUnnamed_24 = 122;
pub const ITEM_MAGIC_LARGE: C2RustUnnamed_24 = 121;
pub const ITEM_MAGIC_SMALL: C2RustUnnamed_24 = 120;
pub const ITEM_KEY_SMALL: C2RustUnnamed_24 = 119;
pub const ITEM_DUNGEON_MAP: C2RustUnnamed_24 = 118;
pub const ITEM_COMPASS: C2RustUnnamed_24 = 117;
pub const ITEM_KEY_BOSS: C2RustUnnamed_24 = 116;
pub const ITEM_HEART_PIECE: C2RustUnnamed_24 = 115;
pub const ITEM_HEART_CONTAINER: C2RustUnnamed_24 = 114;
pub const ITEM_SKULL_TOKEN: C2RustUnnamed_24 = 113;
pub const ITEM_GERUDO_CARD: C2RustUnnamed_24 = 112;
pub const ITEM_STONE_OF_AGONY: C2RustUnnamed_24 = 111;
pub const ITEM_ZORA_SAPPHIRE: C2RustUnnamed_24 = 110;
pub const ITEM_GORON_RUBY: C2RustUnnamed_24 = 109;
pub const ITEM_KOKIRI_EMERALD: C2RustUnnamed_24 = 108;
pub const ITEM_MEDALLION_LIGHT: C2RustUnnamed_24 = 107;
pub const ITEM_MEDALLION_SHADOW: C2RustUnnamed_24 = 106;
pub const ITEM_MEDALLION_SPIRIT: C2RustUnnamed_24 = 105;
pub const ITEM_MEDALLION_WATER: C2RustUnnamed_24 = 104;
pub const ITEM_MEDALLION_FIRE: C2RustUnnamed_24 = 103;
pub const ITEM_MEDALLION_FOREST: C2RustUnnamed_24 = 102;
pub const ITEM_SONG_STORMS: C2RustUnnamed_24 = 101;
pub const ITEM_SONG_TIME: C2RustUnnamed_24 = 100;
pub const ITEM_SONG_SUN: C2RustUnnamed_24 = 99;
pub const ITEM_SONG_SARIA: C2RustUnnamed_24 = 98;
pub const ITEM_SONG_EPONA: C2RustUnnamed_24 = 97;
pub const ITEM_SONG_LULLABY: C2RustUnnamed_24 = 96;
pub const ITEM_SONG_PRELUDE: C2RustUnnamed_24 = 95;
pub const ITEM_SONG_NOCTURNE: C2RustUnnamed_24 = 94;
pub const ITEM_SONG_REQUIEM: C2RustUnnamed_24 = 93;
pub const ITEM_SONG_SERENADE: C2RustUnnamed_24 = 92;
pub const ITEM_SONG_BOLERO: C2RustUnnamed_24 = 91;
pub const ITEM_SONG_MINUET: C2RustUnnamed_24 = 90;
pub const ITEM_FISHING_POLE: C2RustUnnamed_24 = 89;
pub const ITEM_SEEDS: C2RustUnnamed_24 = 88;
pub const ITEM_WALLET_GIANT: C2RustUnnamed_24 = 87;
pub const ITEM_WALLET_ADULT: C2RustUnnamed_24 = 86;
pub const ITEM_SWORD_KNIFE: C2RustUnnamed_24 = 85;
pub const ITEM_SCALE_GOLDEN: C2RustUnnamed_24 = 84;
pub const ITEM_SCALE_SILVER: C2RustUnnamed_24 = 83;
pub const ITEM_GAUNTLETS_GOLD: C2RustUnnamed_24 = 82;
pub const ITEM_GAUNTLETS_SILVER: C2RustUnnamed_24 = 81;
pub const ITEM_BRACELET: C2RustUnnamed_24 = 80;
pub const ITEM_BOMB_BAG_40: C2RustUnnamed_24 = 79;
pub const ITEM_BOMB_BAG_30: C2RustUnnamed_24 = 78;
pub const ITEM_BOMB_BAG_20: C2RustUnnamed_24 = 77;
pub const ITEM_QUIVER_50: C2RustUnnamed_24 = 76;
pub const ITEM_QUIVER_40: C2RustUnnamed_24 = 75;
pub const ITEM_QUIVER_30: C2RustUnnamed_24 = 74;
pub const ITEM_BULLET_BAG_50: C2RustUnnamed_24 = 73;
pub const ITEM_BULLET_BAG_40: C2RustUnnamed_24 = 72;
pub const ITEM_BULLET_BAG_30: C2RustUnnamed_24 = 71;
pub const ITEM_BOOTS_HOVER: C2RustUnnamed_24 = 70;
pub const ITEM_BOOTS_IRON: C2RustUnnamed_24 = 69;
pub const ITEM_BOOTS_KOKIRI: C2RustUnnamed_24 = 68;
pub const ITEM_TUNIC_ZORA: C2RustUnnamed_24 = 67;
pub const ITEM_TUNIC_GORON: C2RustUnnamed_24 = 66;
pub const ITEM_TUNIC_KOKIRI: C2RustUnnamed_24 = 65;
pub const ITEM_SHIELD_MIRROR: C2RustUnnamed_24 = 64;
pub const ITEM_SHIELD_HYLIAN: C2RustUnnamed_24 = 63;
pub const ITEM_SHIELD_DEKU: C2RustUnnamed_24 = 62;
pub const ITEM_SWORD_BGS: C2RustUnnamed_24 = 61;
pub const ITEM_SWORD_MASTER: C2RustUnnamed_24 = 60;
pub const ITEM_SWORD_KOKIRI: C2RustUnnamed_24 = 59;
pub const ITEM_BOW_ARROW_LIGHT: C2RustUnnamed_24 = 58;
pub const ITEM_BOW_ARROW_ICE: C2RustUnnamed_24 = 57;
pub const ITEM_BOW_ARROW_FIRE: C2RustUnnamed_24 = 56;
pub const ITEM_CLAIM_CHECK: C2RustUnnamed_24 = 55;
pub const ITEM_EYEDROPS: C2RustUnnamed_24 = 54;
pub const ITEM_FROG: C2RustUnnamed_24 = 53;
pub const ITEM_PRESCRIPTION: C2RustUnnamed_24 = 52;
pub const ITEM_SWORD_BROKEN: C2RustUnnamed_24 = 51;
pub const ITEM_SAW: C2RustUnnamed_24 = 50;
pub const ITEM_ODD_POTION: C2RustUnnamed_24 = 49;
pub const ITEM_ODD_MUSHROOM: C2RustUnnamed_24 = 48;
pub const ITEM_COJIRO: C2RustUnnamed_24 = 47;
pub const ITEM_POCKET_CUCCO: C2RustUnnamed_24 = 46;
pub const ITEM_POCKET_EGG: C2RustUnnamed_24 = 45;
pub const ITEM_SOLD_OUT: C2RustUnnamed_24 = 44;
pub const ITEM_MASK_TRUTH: C2RustUnnamed_24 = 43;
pub const ITEM_MASK_GERUDO: C2RustUnnamed_24 = 42;
pub const ITEM_MASK_ZORA: C2RustUnnamed_24 = 41;
pub const ITEM_MASK_GORON: C2RustUnnamed_24 = 40;
pub const ITEM_MASK_BUNNY: C2RustUnnamed_24 = 39;
pub const ITEM_MASK_SPOOKY: C2RustUnnamed_24 = 38;
pub const ITEM_MASK_SKULL: C2RustUnnamed_24 = 37;
pub const ITEM_MASK_KEATON: C2RustUnnamed_24 = 36;
pub const ITEM_LETTER_ZELDA: C2RustUnnamed_24 = 35;
pub const ITEM_CHICKEN: C2RustUnnamed_24 = 34;
pub const ITEM_WEIRD_EGG: C2RustUnnamed_24 = 33;
pub const ITEM_POE: C2RustUnnamed_24 = 32;
pub const ITEM_MILK_HALF: C2RustUnnamed_24 = 31;
pub const ITEM_BIG_POE: C2RustUnnamed_24 = 30;
pub const ITEM_BUG: C2RustUnnamed_24 = 29;
pub const ITEM_BLUE_FIRE: C2RustUnnamed_24 = 28;
pub const ITEM_LETTER_RUTO: C2RustUnnamed_24 = 27;
pub const ITEM_MILK_BOTTLE: C2RustUnnamed_24 = 26;
pub const ITEM_FISH: C2RustUnnamed_24 = 25;
pub const ITEM_FAIRY: C2RustUnnamed_24 = 24;
pub const ITEM_POTION_BLUE: C2RustUnnamed_24 = 23;
pub const ITEM_POTION_GREEN: C2RustUnnamed_24 = 22;
pub const ITEM_POTION_RED: C2RustUnnamed_24 = 21;
pub const ITEM_BOTTLE: C2RustUnnamed_24 = 20;
pub const ITEM_NAYRUS_LOVE: C2RustUnnamed_24 = 19;
pub const ITEM_ARROW_LIGHT: C2RustUnnamed_24 = 18;
pub const ITEM_HAMMER: C2RustUnnamed_24 = 17;
pub const ITEM_BEAN: C2RustUnnamed_24 = 16;
pub const ITEM_LENS: C2RustUnnamed_24 = 15;
pub const ITEM_BOOMERANG: C2RustUnnamed_24 = 14;
pub const ITEM_FARORES_WIND: C2RustUnnamed_24 = 13;
pub const ITEM_ARROW_ICE: C2RustUnnamed_24 = 12;
pub const ITEM_LONGSHOT: C2RustUnnamed_24 = 11;
pub const ITEM_HOOKSHOT: C2RustUnnamed_24 = 10;
pub const ITEM_BOMBCHU: C2RustUnnamed_24 = 9;
pub const ITEM_OCARINA_TIME: C2RustUnnamed_24 = 8;
pub const ITEM_OCARINA_FAIRY: C2RustUnnamed_24 = 7;
pub const ITEM_SLINGSHOT: C2RustUnnamed_24 = 6;
pub const ITEM_DINS_FIRE: C2RustUnnamed_24 = 5;
pub const ITEM_ARROW_FIRE: C2RustUnnamed_24 = 4;
pub const ITEM_BOW: C2RustUnnamed_24 = 3;
pub const ITEM_BOMB: C2RustUnnamed_24 = 2;
pub const ITEM_NUT: C2RustUnnamed_24 = 1;
pub const ITEM_STICK: C2RustUnnamed_24 = 0;
pub type C2RustUnnamed_25 = libc::c_uint;
pub const GI_MAX: C2RustUnnamed_25 = 126;
pub const GI_TEXT_0: C2RustUnnamed_25 = 125;
pub const GI_ICE_TRAP: C2RustUnnamed_25 = 124;
pub const GI_BULLET_BAG_50: C2RustUnnamed_25 = 123;
pub const GI_NUT_UPGRADE_40: C2RustUnnamed_25 = 122;
pub const GI_NUT_UPGRADE_30: C2RustUnnamed_25 = 121;
pub const GI_STICK_UPGRADE_30: C2RustUnnamed_25 = 120;
pub const GI_STICK_UPGRADE_20: C2RustUnnamed_25 = 119;
pub const GI_HEART_PIECE_WIN: C2RustUnnamed_25 = 118;
pub const GI_RUPEE_PURPLE_LOSE: C2RustUnnamed_25 = 117;
pub const GI_RUPEE_RED_LOSE: C2RustUnnamed_25 = 116;
pub const GI_RUPEE_BLUE_LOSE: C2RustUnnamed_25 = 115;
pub const GI_RUPEE_GREEN_LOSE: C2RustUnnamed_25 = 114;
pub const GI_DOOR_KEY: C2RustUnnamed_25 = 113;
pub const GI_BIG_POE: C2RustUnnamed_25 = 112;
pub const GI_POE: C2RustUnnamed_25 = 111;
pub const GI_BLUE_FIRE: C2RustUnnamed_25 = 110;
pub const GI_BUGS: C2RustUnnamed_25 = 109;
pub const GI_FISH: C2RustUnnamed_25 = 108;
pub const GI_BOMBCHUS_20: C2RustUnnamed_25 = 107;
pub const GI_BOMBCHUS_5: C2RustUnnamed_25 = 106;
pub const GI_SEEDS_30: C2RustUnnamed_25 = 105;
pub const GI_BOMBS_30: C2RustUnnamed_25 = 104;
pub const GI_BOMBS_20: C2RustUnnamed_25 = 103;
pub const GI_BOMBS_10: C2RustUnnamed_25 = 102;
pub const GI_BOMBS_1: C2RustUnnamed_25 = 101;
pub const GI_NUTS_10: C2RustUnnamed_25 = 100;
pub const GI_NUTS_5_2: C2RustUnnamed_25 = 99;
pub const GI_STICKS_10: C2RustUnnamed_25 = 98;
pub const GI_STICKS_5: C2RustUnnamed_25 = 97;
pub const GI_BULLET_BAG_40: C2RustUnnamed_25 = 96;
pub const GI_BULLET_BAG_30: C2RustUnnamed_25 = 95;
pub const GI_NAYRUS_LOVE: C2RustUnnamed_25 = 94;
pub const GI_FARORES_WIND: C2RustUnnamed_25 = 93;
pub const GI_DINS_FIRE: C2RustUnnamed_25 = 92;
pub const GI_SKULL_TOKEN: C2RustUnnamed_25 = 91;
pub const GI_ARROW_LIGHT: C2RustUnnamed_25 = 90;
pub const GI_ARROW_ICE: C2RustUnnamed_25 = 89;
pub const GI_ARROW_FIRE: C2RustUnnamed_25 = 88;
pub const GI_SWORD_BGS: C2RustUnnamed_25 = 87;
pub const GI_RUPEE_GOLD: C2RustUnnamed_25 = 86;
pub const GI_RUPEE_PURPLE: C2RustUnnamed_25 = 85;
pub const GI_BRACELET: C2RustUnnamed_25 = 84;
pub const GI_MASK_GERUDO: C2RustUnnamed_25 = 83;
pub const GI_MASK_ZORA: C2RustUnnamed_25 = 82;
pub const GI_MASK_GORON: C2RustUnnamed_25 = 81;
pub const GI_MILK: C2RustUnnamed_25 = 80;
pub const GI_HEART_CONTAINER_2: C2RustUnnamed_25 = 79;
pub const GI_RUPEE_RED: C2RustUnnamed_25 = 78;
pub const GI_RUPEE_BLUE: C2RustUnnamed_25 = 77;
pub const GI_RUPEE_GREEN: C2RustUnnamed_25 = 76;
pub const GI_ARROWS_LARGE: C2RustUnnamed_25 = 75;
pub const GI_ARROWS_MEDIUM: C2RustUnnamed_25 = 74;
pub const GI_ARROWS_SMALL: C2RustUnnamed_25 = 73;
pub const GI_HEART: C2RustUnnamed_25 = 72;
pub const GI_WEIRD_EGG: C2RustUnnamed_25 = 71;
pub const GI_WALLET_GIANT: C2RustUnnamed_25 = 70;
pub const GI_WALLET_ADULT: C2RustUnnamed_25 = 69;
pub const GI_MAGIC_LARGE: C2RustUnnamed_25 = 68;
pub const GI_MAGIC_SMALL: C2RustUnnamed_25 = 67;
pub const GI_KEY_SMALL: C2RustUnnamed_25 = 66;
pub const GI_MAP: C2RustUnnamed_25 = 65;
pub const GI_COMPASS: C2RustUnnamed_25 = 64;
pub const GI_KEY_BOSS: C2RustUnnamed_25 = 63;
pub const GI_HEART_PIECE: C2RustUnnamed_25 = 62;
pub const GI_HEART_CONTAINER: C2RustUnnamed_25 = 61;
pub const GI_SEEDS_5: C2RustUnnamed_25 = 60;
pub const GI_OCARINA_FAIRY: C2RustUnnamed_25 = 59;
pub const GI_GERUDO_CARD: C2RustUnnamed_25 = 58;
pub const GI_STONE_OF_AGONY: C2RustUnnamed_25 = 57;
pub const GI_SCALE_GOLD: C2RustUnnamed_25 = 56;
pub const GI_SCALE_SILVER: C2RustUnnamed_25 = 55;
pub const GI_GAUNTLETS_GOLD: C2RustUnnamed_25 = 54;
pub const GI_GAUNTLETS_SILVER: C2RustUnnamed_25 = 53;
pub const GI_BOMB_BAG_40: C2RustUnnamed_25 = 52;
pub const GI_BOMB_BAG_30: C2RustUnnamed_25 = 51;
pub const GI_BOMB_BAG_20: C2RustUnnamed_25 = 50;
pub const GI_QUIVER_50: C2RustUnnamed_25 = 49;
pub const GI_QUIVER_40: C2RustUnnamed_25 = 48;
pub const GI_BOOTS_HOVER: C2RustUnnamed_25 = 47;
pub const GI_BOOTS_IRON: C2RustUnnamed_25 = 46;
pub const GI_TUNIC_ZORA: C2RustUnnamed_25 = 45;
pub const GI_TUNIC_GORON: C2RustUnnamed_25 = 44;
pub const GI_SHIELD_MIRROR: C2RustUnnamed_25 = 43;
pub const GI_SHIELD_HYLIAN: C2RustUnnamed_25 = 42;
pub const GI_SHIELD_DEKU: C2RustUnnamed_25 = 41;
pub const GI_SWORD_KNIFE: C2RustUnnamed_25 = 40;
pub const GI_SWORD_KOKIRI: C2RustUnnamed_25 = 39;
pub const GI_CLAIM_CHECK: C2RustUnnamed_25 = 38;
pub const GI_EYEDROPS: C2RustUnnamed_25 = 37;
pub const GI_FROG: C2RustUnnamed_25 = 36;
pub const GI_PRESCRIPTION: C2RustUnnamed_25 = 35;
pub const GI_SWORD_BROKEN: C2RustUnnamed_25 = 34;
pub const GI_SAW: C2RustUnnamed_25 = 33;
pub const GI_ODD_POTION: C2RustUnnamed_25 = 32;
pub const GI_ODD_MUSHROOM: C2RustUnnamed_25 = 31;
pub const GI_POCKET_CUCCO: C2RustUnnamed_25 = 30;
pub const GI_POCKET_EGG: C2RustUnnamed_25 = 29;
pub const GI_MASK_TRUTH: C2RustUnnamed_25 = 28;
pub const GI_MASK_BUNNY: C2RustUnnamed_25 = 27;
pub const GI_MASK_KEATON: C2RustUnnamed_25 = 26;
pub const GI_CHICKEN: C2RustUnnamed_25 = 25;
pub const GI_MASK_SPOOKY: C2RustUnnamed_25 = 24;
pub const GI_MASK_SKULL: C2RustUnnamed_25 = 23;
pub const GI_BEAN: C2RustUnnamed_25 = 22;
pub const GI_LETTER_RUTO: C2RustUnnamed_25 = 21;
pub const GI_MILK_BOTTLE: C2RustUnnamed_25 = 20;
pub const GI_FAIRY: C2RustUnnamed_25 = 19;
pub const GI_POTION_BLUE: C2RustUnnamed_25 = 18;
pub const GI_POTION_GREEN: C2RustUnnamed_25 = 17;
pub const GI_POTION_RED: C2RustUnnamed_25 = 16;
pub const GI_BOTTLE: C2RustUnnamed_25 = 15;
pub const GI_COJIRO: C2RustUnnamed_25 = 14;
pub const GI_HAMMER: C2RustUnnamed_25 = 13;
pub const GI_OCARINA_OOT: C2RustUnnamed_25 = 12;
pub const GI_LETTER_ZELDA: C2RustUnnamed_25 = 11;
pub const GI_LENS: C2RustUnnamed_25 = 10;
pub const GI_LONGSHOT: C2RustUnnamed_25 = 9;
pub const GI_HOOKSHOT: C2RustUnnamed_25 = 8;
pub const GI_STICKS_1: C2RustUnnamed_25 = 7;
pub const GI_BOOMERANG: C2RustUnnamed_25 = 6;
pub const GI_SLINGSHOT: C2RustUnnamed_25 = 5;
pub const GI_BOW: C2RustUnnamed_25 = 4;
pub const GI_BOMBCHUS_10: C2RustUnnamed_25 = 3;
pub const GI_NUTS_5: C2RustUnnamed_25 = 2;
pub const GI_BOMBS_5: C2RustUnnamed_25 = 1;
pub const GI_NONE: C2RustUnnamed_25 = 0;
pub type C2RustUnnamed_26 = libc::c_uint;
pub const EXCH_ITEM_MAX: C2RustUnnamed_26 = 30;
pub const EXCH_ITEM_LETTER_RUTO: C2RustUnnamed_26 = 29;
pub const EXCH_ITEM_BIG_POE: C2RustUnnamed_26 = 28;
pub const EXCH_ITEM_POE: C2RustUnnamed_26 = 27;
pub const EXCH_ITEM_BUG: C2RustUnnamed_26 = 26;
pub const EXCH_ITEM_BLUE_FIRE: C2RustUnnamed_26 = 25;
pub const EXCH_ITEM_FISH: C2RustUnnamed_26 = 24;
pub const EXCH_ITEM_MASK_GERUDO: C2RustUnnamed_26 = 23;
pub const EXCH_ITEM_MASK_ZORA: C2RustUnnamed_26 = 22;
pub const EXCH_ITEM_MASK_GORON: C2RustUnnamed_26 = 21;
pub const EXCH_ITEM_MASK_TRUTH: C2RustUnnamed_26 = 20;
pub const EXCH_ITEM_MASK_BUNNY: C2RustUnnamed_26 = 19;
pub const EXCH_ITEM_MASK_KEATON: C2RustUnnamed_26 = 18;
pub const EXCH_ITEM_MASK_SPOOKY: C2RustUnnamed_26 = 17;
pub const EXCH_ITEM_MASK_SKULL: C2RustUnnamed_26 = 16;
pub const EXCH_ITEM_CLAIM_CHECK: C2RustUnnamed_26 = 15;
pub const EXCH_ITEM_EYEDROPS: C2RustUnnamed_26 = 14;
pub const EXCH_ITEM_FROG: C2RustUnnamed_26 = 13;
pub const EXCH_ITEM_PRESCRIPTION: C2RustUnnamed_26 = 12;
pub const EXCH_ITEM_SWORD_BROKEN: C2RustUnnamed_26 = 11;
pub const EXCH_ITEM_SAW: C2RustUnnamed_26 = 10;
pub const EXCH_ITEM_ODD_POTION: C2RustUnnamed_26 = 9;
pub const EXCH_ITEM_ODD_MUSHROOM: C2RustUnnamed_26 = 8;
pub const EXCH_ITEM_COJIRO: C2RustUnnamed_26 = 7;
pub const EXCH_ITEM_POCKET_CUCCO: C2RustUnnamed_26 = 6;
pub const EXCH_ITEM_POCKET_EGG: C2RustUnnamed_26 = 5;
pub const EXCH_ITEM_BEAN: C2RustUnnamed_26 = 4;
pub const EXCH_ITEM_CHICKEN: C2RustUnnamed_26 = 3;
pub const EXCH_ITEM_WEIRD_EGG: C2RustUnnamed_26 = 2;
pub const EXCH_ITEM_LETTER_ZELDA: C2RustUnnamed_26 = 1;
pub const EXCH_ITEM_NONE: C2RustUnnamed_26 = 0;
pub type C2RustUnnamed_27 = libc::c_uint;
pub const MSGMODE_PAUSED: C2RustUnnamed_27 = 55;
pub const MSGMODE_TEXT_CLOSING: C2RustUnnamed_27 = 54;
pub const MSGMODE_TEXT_DONE: C2RustUnnamed_27 = 53;
pub const MSGMODE_TEXT_AWAIT_NEXT: C2RustUnnamed_27 = 52;
pub const MSGMODE_FROGS_WAITING: C2RustUnnamed_27 = 51;
pub const MSGMODE_FROGS_PLAYING: C2RustUnnamed_27 = 50;
pub const MSGMODE_FROGS_START: C2RustUnnamed_27 = 49;
pub const MSGMODE_MEMORY_GAME_START_NEXT_ROUND: C2RustUnnamed_27 = 48;
pub const MSGMODE_MEMORY_GAME_ROUND_SUCCESS: C2RustUnnamed_27 = 47;
pub const MSGMODE_MEMORY_GAME_PLAYER_PLAYING: C2RustUnnamed_27 = 46;
pub const MSGMODE_MEMORY_GAME_RIGHT_SKULLKID_WAIT: C2RustUnnamed_27 = 45;
pub const MSGMODE_MEMORY_GAME_RIGHT_SKULLKID_PLAYING: C2RustUnnamed_27 = 44;
pub const MSGMODE_MEMORY_GAME_LEFT_SKULLKID_WAIT: C2RustUnnamed_27 = 43;
pub const MSGMODE_MEMORY_GAME_LEFT_SKULLKID_PLAYING: C2RustUnnamed_27 = 42;
pub const MSGMODE_MEMORY_GAME_START: C2RustUnnamed_27 = 41;
pub const MSGMODE_SCARECROW_PLAYBACK: C2RustUnnamed_27 = 40;
pub const MSGMODE_SCARECROW_RECORDING_DONE: C2RustUnnamed_27 = 39;
pub const MSGMODE_SCARECROW_RECORDING_FAILED: C2RustUnnamed_27 = 38;
pub const MSGMODE_SCARECROW_RECORDING_ONGOING: C2RustUnnamed_27 = 37;
pub const MSGMODE_SCARECROW_RECORDING_START: C2RustUnnamed_27 = 36;
pub const MSGMODE_SCARECROW_LONG_PLAYBACK: C2RustUnnamed_27 = 35;
pub const MSGMODE_SCARECROW_LONG_RECORDING_ONGOING: C2RustUnnamed_27 = 34;
pub const MSGMODE_SCARECROW_LONG_RECORDING_START: C2RustUnnamed_27 = 33;
pub const MSGMODE_UNK_20: C2RustUnnamed_27 = 32;
pub const MSGMODE_OCARINA_AWAIT_INPUT: C2RustUnnamed_27 = 31;
pub const MSGMODE_SONG_PLAYBACK_NOTES_DROP: C2RustUnnamed_27 = 30;
pub const MSGMODE_SONG_PLAYBACK_FAIL: C2RustUnnamed_27 = 29;
pub const MSGMODE_SONG_PLAYBACK_SUCCESS: C2RustUnnamed_27 = 28;
pub const MSGMODE_SONG_PLAYBACK: C2RustUnnamed_27 = 27;
pub const MSGMODE_SONG_DEMONSTRATION_DONE: C2RustUnnamed_27 = 26;
pub const MSGMODE_SONG_DEMONSTRATION: C2RustUnnamed_27 = 25;
pub const MSGMODE_SONG_DEMONSTRATION_SELECT_INSTRUMENT: C2RustUnnamed_27 = 24;
pub const MSGMODE_SONG_PLAYED_ACT: C2RustUnnamed_27 = 23;
pub const MSGMODE_SONG_PLAYED_ACT_BEGIN: C2RustUnnamed_27 = 22;
pub const MSGMODE_DISPLAY_SONG_PLAYED_TEXT: C2RustUnnamed_27 = 21;
pub const MSGMODE_DISPLAY_SONG_PLAYED_TEXT_BEGIN: C2RustUnnamed_27 = 20;
pub const MSGMODE_DISPLAY_SONG_PLAYED: C2RustUnnamed_27 = 19;
pub const MSGMODE_SETUP_DISPLAY_SONG_PLAYED: C2RustUnnamed_27 = 18;
pub const MSGMODE_SONG_PLAYED: C2RustUnnamed_27 = 17;
pub const MSGMODE_OCARINA_NOTES_DROP: C2RustUnnamed_27 = 16;
pub const MSGMODE_OCARINA_FAIL_NO_TEXT: C2RustUnnamed_27 = 15;
pub const MSGMODE_OCARINA_FAIL: C2RustUnnamed_27 = 14;
pub const MSGMODE_OCARINA_CORRECT_PLAYBACK: C2RustUnnamed_27 = 13;
pub const MSGMODE_OCARINA_PLAYING: C2RustUnnamed_27 = 12;
pub const MSGMODE_SONG_PLAYBACK_STARTING: C2RustUnnamed_27 = 11;
pub const MSGMODE_SONG_DEMONSTRATION_STARTING: C2RustUnnamed_27 = 10;
pub const MSGMODE_OCARINA_STARTING: C2RustUnnamed_27 = 9;
pub const MSGMODE_TEXT_DELAYED_BREAK: C2RustUnnamed_27 = 8;
pub const MSGMODE_TEXT_AWAIT_INPUT: C2RustUnnamed_27 = 7;
pub const MSGMODE_TEXT_DISPLAYING: C2RustUnnamed_27 = 6;
pub const MSGMODE_TEXT_CONTINUING: C2RustUnnamed_27 = 5;
pub const MSGMODE_TEXT_NEXT_MSG: C2RustUnnamed_27 = 4;
pub const MSGMODE_TEXT_STARTING: C2RustUnnamed_27 = 3;
pub const MSGMODE_TEXT_BOX_GROWING: C2RustUnnamed_27 = 2;
pub const MSGMODE_TEXT_START: C2RustUnnamed_27 = 1;
pub const MSGMODE_NONE: C2RustUnnamed_27 = 0;
pub type C2RustUnnamed_28 = libc::c_uint;
pub const TEXT_STATE_AWAITING_NEXT: C2RustUnnamed_28 = 10;
pub const TEXT_STATE_9: C2RustUnnamed_28 = 9;
pub const TEXT_STATE_8: C2RustUnnamed_28 = 8;
pub const TEXT_STATE_SONG_DEMO_DONE: C2RustUnnamed_28 = 7;
pub const TEXT_STATE_DONE: C2RustUnnamed_28 = 6;
pub const TEXT_STATE_EVENT: C2RustUnnamed_28 = 5;
pub const TEXT_STATE_CHOICE: C2RustUnnamed_28 = 4;
pub const TEXT_STATE_DONE_FADING: C2RustUnnamed_28 = 3;
pub const TEXT_STATE_CLOSING: C2RustUnnamed_28 = 2;
pub const TEXT_STATE_DONE_HAS_NEXT: C2RustUnnamed_28 = 1;
pub const TEXT_STATE_NONE: C2RustUnnamed_28 = 0;
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
pub type C2RustUnnamed_29 = libc::c_uint;
pub const MTXMODE_APPLY: C2RustUnnamed_29 = 1;
pub const MTXMODE_NEW: C2RustUnnamed_29 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EnGoEffect {
    pub type_0: u8_0,
    pub timer: u8_0,
    pub initialTimer: u8_0,
    pub scale: f32_0,
    pub scaleStep: f32_0,
    pub color: Color_RGBA8,
    pub unk_10: [libc::c_char; 4],
    pub pos: Vec3f,
    pub velocity: Vec3f,
    pub accel: Vec3f,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EnGo2 {
    pub actor: Actor,
    pub skelAnime: SkelAnime,
    pub actionFunc: EnGo2ActionFunc,
    pub unk_194: struct_80034A14_arg1,
    pub collider: ColliderCylinder,
    pub path: *mut Path,
    pub unk_20C: u8_0,
    pub dialogState: u8_0,
    pub reverse: u8_0,
    pub isAwake: u8_0,
    pub waypoint: s8,
    pub unk_211: u8_0,
    pub goronState: u8_0,
    pub eyeMouthTexState: u8_0,
    pub eyeTexIndex: u8_0,
    pub mouthTexIndex: u8_0,
    pub unk_216: u8_0,
    pub unk_218: f32_0,
    pub unk_21C: [libc::c_char; 4],
    pub alpha: f32_0,
    pub blinkTimer: s16,
    pub unk_226: [s16; 18],
    pub unk_24A: [s16; 18],
    pub unk_26E: u16_0,
    pub dustEffects: [EnGoEffect; 10],
    pub eye: Vec3f,
    pub at: Vec3f,
    pub jointTable: [Vec3s; 18],
    pub morphTable: [Vec3s; 18],
    pub unk_590: s16,
    pub animTimer: s16,
    pub getItemId: s32,
    pub unk_598: [libc::c_char; 2],
    pub camId: s16,
    pub unk_59C: s16,
}
pub type EnGo2ActionFunc
    =
    Option<unsafe extern "C" fn(_: *mut EnGo2, _: *mut GlobalContext) -> ()>;
pub type C2RustUnnamed_30 = libc::c_uint;
pub const GORON_MARKET_BAZAAR: C2RustUnnamed_30 = 13;
pub const GORON_DMT_FAIRY_HINT: C2RustUnnamed_30 = 12;
pub const GORON_CITY_LOST_WOODS: C2RustUnnamed_30 = 11;
pub const GORON_CITY_STAIRWELL: C2RustUnnamed_30 = 10;
pub const GORON_CITY_LOWEST_FLOOR: C2RustUnnamed_30 = 9;
pub const GORON_CITY_ISLAND: C2RustUnnamed_30 = 8;
pub const GORON_CITY_ENTRANCE: C2RustUnnamed_30 = 7;
pub const GORON_DMT_DC_ENTRANCE: C2RustUnnamed_30 = 6;
pub const GORON_DMT_ROLLING_SMALL: C2RustUnnamed_30 = 5;
pub const GORON_DMT_BOMB_FLOWER: C2RustUnnamed_30 = 4;
pub const GORON_FIRE_GENERIC: C2RustUnnamed_30 = 3;
pub const GORON_DMT_BIGGORON: C2RustUnnamed_30 = 2;
pub const GORON_CITY_LINK: C2RustUnnamed_30 = 1;
pub const GORON_CITY_ROLLING_BIG: C2RustUnnamed_30 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EnGo2DataStruct1 {
    pub unused: s16,
    pub yDist: s16,
    pub xzDist: s16,
    pub radius: s16,
    pub height: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EnGo2DataStruct2 {
    pub shape_unk_10: f32_0,
    pub scale: f32_0,
    pub actor_unk_1F: s8,
    pub unk_218: f32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EnGo2DustEffectData {
    pub initialTimer: u8_0,
    pub scale: f32_0,
    pub scaleStep: f32_0,
    pub numDustEffects: s32,
    pub radius: f32_0,
    pub yAccel: f32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EnBom {
    pub actor: Actor,
    pub bombCollider: ColliderCylinder,
    pub explosionCollider: ColliderJntSph,
    pub explosionColliderItems: [ColliderJntSphElement; 1],
    pub timer: s16,
    pub flashSpeedScale: s16,
    pub flashIntensity: f32_0,
    pub bumpOn: u8_0,
    pub actionFunc: EnBomActionFunc,
}
pub type EnBomActionFunc
    =
    Option<unsafe extern "C" fn(_: *mut EnBom, _: *mut GlobalContext) -> ()>;
static mut sDustTex: [*mut libc::c_void; 8] =
    unsafe {
        [gDust8Tex.as_ptr() as *mut _ as *mut libc::c_void,
         gDust7Tex.as_ptr() as *mut _ as *mut libc::c_void,
         gDust6Tex.as_ptr() as *mut _ as *mut libc::c_void,
         gDust5Tex.as_ptr() as *mut _ as *mut libc::c_void,
         gDust4Tex.as_ptr() as *mut _ as *mut libc::c_void,
         gDust3Tex.as_ptr() as *mut _ as *mut libc::c_void,
         gDust2Tex.as_ptr() as *mut _ as *mut libc::c_void,
         gDust1Tex.as_ptr() as *mut _ as *mut libc::c_void]
    };
static mut sPos: Vec3f =
    {
        let mut init =
            Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; // overwrites sPos data
        init
    };
static mut sVelocity: Vec3f =
    { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
static mut sAccel: Vec3f =
    { let mut init = Vec3f{x: 0.0f32, y: 0.3f32, z: 0.0f32,}; init };
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
                                                                                               0x8
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
                                                            40 as libc::c_int
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
static mut sColChkInfoInit: CollisionCheckInfoInit2 =
    {
        let mut init =
            CollisionCheckInfoInit2{health: 0 as libc::c_int as u8_0,
                                    cylRadius: 0 as libc::c_int as s16,
                                    cylHeight: 0 as libc::c_int as s16,
                                    cylYShift: 0 as libc::c_int as s16,
                                    mass: 0xff as libc::c_int as u8_0,};
        init
    };
#[no_mangle]
pub static mut En_Go2_InitVars: ActorInit =
    unsafe {
        {
            let mut init =
                ActorInit{id: ACTOR_EN_GO2 as libc::c_int as s16,
                          category: ACTORCAT_NPC as libc::c_int as u8_0,
                          flags:
                              ((1 as libc::c_int) << 0 as libc::c_int |
                                   (1 as libc::c_int) << 3 as libc::c_int |
                                   (1 as libc::c_int) << 4 as libc::c_int |
                                   (1 as libc::c_int) << 5 as libc::c_int) as
                                  u32_0,
                          objectId: OBJECT_OF1D_MAP as libc::c_int as s16,
                          instanceSize:
                              ::std::mem::size_of::<EnGo2>() as libc::c_ulong,
                          init:
                              ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                      *mut Actor,
                                                                                  _:
                                                                                      *mut GlobalContext)
                                                                 -> ()>,
                                                      ActorFunc>(Some(EnGo2_Init
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
                                                      ActorFunc>(Some(EnGo2_Destroy
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
                                                      ActorFunc>(Some(EnGo2_Update
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
                                                      ActorFunc>(Some(EnGo2_Draw
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
static mut D_80A4816C: [EnGo2DataStruct1; 14] =
    [{
         let mut init =
             EnGo2DataStruct1{unused: 0 as libc::c_int as s16,
                              yDist: 0 as libc::c_int as s16,
                              xzDist: 0 as libc::c_int as s16,
                              radius: 68 as libc::c_int as s16,
                              height: 148 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnGo2DataStruct1{unused: 0 as libc::c_int as s16,
                              yDist: 0 as libc::c_int as s16,
                              xzDist: 0 as libc::c_int as s16,
                              radius: 24 as libc::c_int as s16,
                              height: 52 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnGo2DataStruct1{unused: 0 as libc::c_int as s16,
                              yDist: 320 as libc::c_int as s16,
                              xzDist: 380 as libc::c_int as s16,
                              radius: 400 as libc::c_int as s16,
                              height: 120 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnGo2DataStruct1{unused: 0 as libc::c_int as s16,
                              yDist: 0 as libc::c_int as s16,
                              xzDist: 0 as libc::c_int as s16,
                              radius: 30 as libc::c_int as s16,
                              height: 68 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnGo2DataStruct1{unused: 0 as libc::c_int as s16,
                              yDist: 0 as libc::c_int as s16,
                              xzDist: 0 as libc::c_int as s16,
                              radius: 46 as libc::c_int as s16,
                              height: 90 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnGo2DataStruct1{unused: 0 as libc::c_int as s16,
                              yDist: 0 as libc::c_int as s16,
                              xzDist: 0 as libc::c_int as s16,
                              radius: 30 as libc::c_int as s16,
                              height: 68 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnGo2DataStruct1{unused: 0 as libc::c_int as s16,
                              yDist: 0 as libc::c_int as s16,
                              xzDist: 0 as libc::c_int as s16,
                              radius: 30 as libc::c_int as s16,
                              height: 68 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnGo2DataStruct1{unused: 0 as libc::c_int as s16,
                              yDist: 0 as libc::c_int as s16,
                              xzDist: 0 as libc::c_int as s16,
                              radius: 30 as libc::c_int as s16,
                              height: 68 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnGo2DataStruct1{unused: 0 as libc::c_int as s16,
                              yDist: 0 as libc::c_int as s16,
                              xzDist: 0 as libc::c_int as s16,
                              radius: 30 as libc::c_int as s16,
                              height: 68 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnGo2DataStruct1{unused: 0 as libc::c_int as s16,
                              yDist: 0 as libc::c_int as s16,
                              xzDist: 0 as libc::c_int as s16,
                              radius: 30 as libc::c_int as s16,
                              height: 68 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnGo2DataStruct1{unused: 0 as libc::c_int as s16,
                              yDist: 0 as libc::c_int as s16,
                              xzDist: 0 as libc::c_int as s16,
                              radius: 30 as libc::c_int as s16,
                              height: 68 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnGo2DataStruct1{unused: 0 as libc::c_int as s16,
                              yDist: 0 as libc::c_int as s16,
                              xzDist: 0 as libc::c_int as s16,
                              radius: 30 as libc::c_int as s16,
                              height: 68 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnGo2DataStruct1{unused: 0 as libc::c_int as s16,
                              yDist: 0 as libc::c_int as s16,
                              xzDist: 0 as libc::c_int as s16,
                              radius: 30 as libc::c_int as s16,
                              height: 68 as libc::c_int as s16,};
         init
     },
     {
         let mut init =
             EnGo2DataStruct1{unused: 0 as libc::c_int as s16,
                              yDist: 0 as libc::c_int as s16,
                              xzDist: 0 as libc::c_int as s16,
                              radius: 30 as libc::c_int as s16,
                              height: 68 as libc::c_int as s16,};
         init
     }];
static mut D_80A481F8: [EnGo2DataStruct2; 14] =
    [{
         let mut init =
             EnGo2DataStruct2{shape_unk_10: 30.0f32,
                              scale: 0.026f32,
                              actor_unk_1F: 6 as libc::c_int as s8,
                              unk_218: 60.0f32,};
         init
     },
     {
         let mut init =
             EnGo2DataStruct2{shape_unk_10: 24.0f32,
                              scale: 0.008f32,
                              actor_unk_1F: 6 as libc::c_int as s8,
                              unk_218: 30.0f32,};
         init
     },
     {
         let mut init =
             EnGo2DataStruct2{shape_unk_10: 28.0f32,
                              scale: 0.16f32,
                              actor_unk_1F: 5 as libc::c_int as s8,
                              unk_218: 380.0f32,};
         init
     },
     {
         let mut init =
             EnGo2DataStruct2{shape_unk_10: 28.0f32,
                              scale: 0.01f32,
                              actor_unk_1F: 7 as libc::c_int as s8,
                              unk_218: 40.0f32,};
         init
     },
     {
         let mut init =
             EnGo2DataStruct2{shape_unk_10: 30.0f32,
                              scale: 0.015f32,
                              actor_unk_1F: 6 as libc::c_int as s8,
                              unk_218: 30.0f32,};
         init
     },
     {
         let mut init =
             EnGo2DataStruct2{shape_unk_10: 28.0f32,
                              scale: 0.01f32,
                              actor_unk_1F: 6 as libc::c_int as s8,
                              unk_218: 30.0f32,};
         init
     },
     {
         let mut init =
             EnGo2DataStruct2{shape_unk_10: 28.0f32,
                              scale: 0.01f32,
                              actor_unk_1F: 6 as libc::c_int as s8,
                              unk_218: 30.0f32,};
         init
     },
     {
         let mut init =
             EnGo2DataStruct2{shape_unk_10: 28.0f32,
                              scale: 0.01f32,
                              actor_unk_1F: 6 as libc::c_int as s8,
                              unk_218: 30.0f32,};
         init
     },
     {
         let mut init =
             EnGo2DataStruct2{shape_unk_10: 28.0f32,
                              scale: 0.01f32,
                              actor_unk_1F: 6 as libc::c_int as s8,
                              unk_218: 30.0f32,};
         init
     },
     {
         let mut init =
             EnGo2DataStruct2{shape_unk_10: 28.0f32,
                              scale: 0.01f32,
                              actor_unk_1F: 6 as libc::c_int as s8,
                              unk_218: 30.0f32,};
         init
     },
     {
         let mut init =
             EnGo2DataStruct2{shape_unk_10: 28.0f32,
                              scale: 0.01f32,
                              actor_unk_1F: 6 as libc::c_int as s8,
                              unk_218: 30.0f32,};
         init
     },
     {
         let mut init =
             EnGo2DataStruct2{shape_unk_10: 28.0f32,
                              scale: 0.01f32,
                              actor_unk_1F: 6 as libc::c_int as s8,
                              unk_218: 30.0f32,};
         init
     },
     {
         let mut init =
             EnGo2DataStruct2{shape_unk_10: 28.0f32,
                              scale: 0.01f32,
                              actor_unk_1F: 6 as libc::c_int as s8,
                              unk_218: 30.0f32,};
         init
     },
     {
         let mut init =
             EnGo2DataStruct2{shape_unk_10: 28.0f32,
                              scale: 0.01f32,
                              actor_unk_1F: 6 as libc::c_int as s8,
                              unk_218: 30.0f32,};
         init
     }];
static mut D_80A482D8: [[f32_0; 2]; 14] =
    [[80.0f32, 80.0f32], [-10.0f32, -10.0f32], [800.0f32, 800.0f32],
     [0.0f32, 0.0f32], [20.0f32, 40.0f32], [20.0f32, 20.0f32],
     [20.0f32, 20.0f32], [20.0f32, 20.0f32], [20.0f32, 20.0f32],
     [20.0f32, 20.0f32], [20.0f32, 20.0f32], [20.0f32, 20.0f32],
     [20.0f32, 20.0f32], [20.0f32, 20.0f32]];
static mut sAnimations: [struct_80034EC0_Entry; 13] =
    unsafe {
        [{
             let mut init =
                 struct_80034EC0_Entry{animation:
                                           &gGoronAnim_004930 as
                                               *const AnimationHeader as
                                               *mut AnimationHeader,
                                       playbackSpeed: 0.0f32,
                                       startFrame: 0.0f32,
                                       frameCount: -1.0f32,
                                       mode: 0 as libc::c_int as u8_0,
                                       transitionRate: 0.0f32,};
             init
         },
         {
             let mut init =
                 struct_80034EC0_Entry{animation:
                                           &gGoronAnim_004930 as
                                               *const AnimationHeader as
                                               *mut AnimationHeader,
                                       playbackSpeed: 0.0f32,
                                       startFrame: 0.0f32,
                                       frameCount: -1.0f32,
                                       mode: 0 as libc::c_int as u8_0,
                                       transitionRate: -8.0f32,};
             init
         },
         {
             let mut init =
                 struct_80034EC0_Entry{animation:
                                           &gGoronAnim_0029A8 as
                                               *const AnimationHeader as
                                               *mut AnimationHeader,
                                       playbackSpeed: 1.0f32,
                                       startFrame: 0.0f32,
                                       frameCount: -1.0f32,
                                       mode: 0 as libc::c_int as u8_0,
                                       transitionRate: -8.0f32,};
             init
         },
         {
             let mut init =
                 struct_80034EC0_Entry{animation:
                                           &gGoronAnim_010590 as
                                               *const AnimationHeader as
                                               *mut AnimationHeader,
                                       playbackSpeed: 1.0f32,
                                       startFrame: 0.0f32,
                                       frameCount: -1.0f32,
                                       mode: 0 as libc::c_int as u8_0,
                                       transitionRate: -8.0f32,};
             init
         },
         {
             let mut init =
                 struct_80034EC0_Entry{animation:
                                           &gGoronAnim_003768 as
                                               *const AnimationHeader as
                                               *mut AnimationHeader,
                                       playbackSpeed: 1.0f32,
                                       startFrame: 0.0f32,
                                       frameCount: -1.0f32,
                                       mode: 0 as libc::c_int as u8_0,
                                       transitionRate: -8.0f32,};
             init
         },
         {
             let mut init =
                 struct_80034EC0_Entry{animation:
                                           &gGoronAnim_0038E4 as
                                               *const AnimationHeader as
                                               *mut AnimationHeader,
                                       playbackSpeed: 1.0f32,
                                       startFrame: 0.0f32,
                                       frameCount: -1.0f32,
                                       mode: 0x2 as libc::c_int as u8_0,
                                       transitionRate: -8.0f32,};
             init
         },
         {
             let mut init =
                 struct_80034EC0_Entry{animation:
                                           &gGoronAnim_002D80 as
                                               *const AnimationHeader as
                                               *mut AnimationHeader,
                                       playbackSpeed: 1.0f32,
                                       startFrame: 0.0f32,
                                       frameCount: -1.0f32,
                                       mode: 0x2 as libc::c_int as u8_0,
                                       transitionRate: -8.0f32,};
             init
         },
         {
             let mut init =
                 struct_80034EC0_Entry{animation:
                                           &gGoronAnim_00161C as
                                               *const AnimationHeader as
                                               *mut AnimationHeader,
                                       playbackSpeed: 1.0f32,
                                       startFrame: 0.0f32,
                                       frameCount: -1.0f32,
                                       mode: 0 as libc::c_int as u8_0,
                                       transitionRate: -8.0f32,};
             init
         },
         {
             let mut init =
                 struct_80034EC0_Entry{animation:
                                           &gGoronAnim_001A00 as
                                               *const AnimationHeader as
                                               *mut AnimationHeader,
                                       playbackSpeed: 1.0f32,
                                       startFrame: 0.0f32,
                                       frameCount: -1.0f32,
                                       mode: 0 as libc::c_int as u8_0,
                                       transitionRate: -8.0f32,};
             init
         },
         {
             let mut init =
                 struct_80034EC0_Entry{animation:
                                           &gGoronAnim_0021D0 as
                                               *const AnimationHeader as
                                               *mut AnimationHeader,
                                       playbackSpeed: 1.0f32,
                                       startFrame: 0.0f32,
                                       frameCount: -1.0f32,
                                       mode: 0 as libc::c_int as u8_0,
                                       transitionRate: -8.0f32,};
             init
         },
         {
             let mut init =
                 struct_80034EC0_Entry{animation:
                                           &gGoronAnim_004930 as
                                               *const AnimationHeader as
                                               *mut AnimationHeader,
                                       playbackSpeed: 0.0f32,
                                       startFrame: 0.0f32,
                                       frameCount: -1.0f32,
                                       mode: 0x1 as libc::c_int as u8_0,
                                       transitionRate: -8.0f32,};
             init
         },
         {
             let mut init =
                 struct_80034EC0_Entry{animation:
                                           &gGoronAnim_000750 as
                                               *const AnimationHeader as
                                               *mut AnimationHeader,
                                       playbackSpeed: 1.0f32,
                                       startFrame: 0.0f32,
                                       frameCount: -1.0f32,
                                       mode: 0 as libc::c_int as u8_0,
                                       transitionRate: -8.0f32,};
             init
         },
         {
             let mut init =
                 struct_80034EC0_Entry{animation:
                                           &gGoronAnim_000D5C as
                                               *const AnimationHeader as
                                               *mut AnimationHeader,
                                       playbackSpeed: 1.0f32,
                                       startFrame: 0.0f32,
                                       frameCount: -1.0f32,
                                       mode: 0 as libc::c_int as u8_0,
                                       transitionRate: -8.0f32,};
             init
         }]
    };
static mut sDustEffectData: [[EnGo2DustEffectData; 4]; 2] =
    [[{
          let mut init =
              EnGo2DustEffectData{initialTimer: 12 as libc::c_int as u8_0,
                                  scale: 0.2f32,
                                  scaleStep: 0.2f32,
                                  numDustEffects: 1 as libc::c_int,
                                  radius: 18.0f32,
                                  yAccel: 0.0f32,};
          init
      },
      {
          let mut init =
              EnGo2DustEffectData{initialTimer: 12 as libc::c_int as u8_0,
                                  scale: 0.1f32,
                                  scaleStep: 0.2f32,
                                  numDustEffects: 12 as libc::c_int,
                                  radius: 26.0f32,
                                  yAccel: 0.0f32,};
          init
      },
      {
          let mut init =
              EnGo2DustEffectData{initialTimer: 12 as libc::c_int as u8_0,
                                  scale: 0.1f32,
                                  scaleStep: 0.3f32,
                                  numDustEffects: 4 as libc::c_int,
                                  radius: 10.0f32,
                                  yAccel: 0.0f32,};
          init
      },
      {
          let mut init =
              EnGo2DustEffectData{initialTimer: 12 as libc::c_int as u8_0,
                                  scale: 0.2f32,
                                  scaleStep: 0.2f32,
                                  numDustEffects: 1 as libc::c_int,
                                  radius: 18.0f32,
                                  yAccel: 0.0f32,};
          init
      }],
     [{
          let mut init =
              EnGo2DustEffectData{initialTimer: 12 as libc::c_int as u8_0,
                                  scale: 0.5f32,
                                  scaleStep: 0.4f32,
                                  numDustEffects: 3 as libc::c_int,
                                  radius: 42.0f32,
                                  yAccel: 0.0f32,};
          init
      },
      {
          let mut init =
              EnGo2DustEffectData{initialTimer: 12 as libc::c_int as u8_0,
                                  scale: 0.5f32,
                                  scaleStep: 0.4f32,
                                  numDustEffects: 3 as libc::c_int,
                                  radius: 42.0f32,
                                  yAccel: 0.0f32,};
          init
      },
      {
          let mut init =
              EnGo2DustEffectData{initialTimer: 12 as libc::c_int as u8_0,
                                  scale: 0.5f32,
                                  scaleStep: 0.4f32,
                                  numDustEffects: 3 as libc::c_int,
                                  radius: 42.0f32,
                                  yAccel: 0.0f32,};
          init
      },
      {
          let mut init =
              EnGo2DustEffectData{initialTimer: 12 as libc::c_int as u8_0,
                                  scale: 0.5f32,
                                  scaleStep: 0.4f32,
                                  numDustEffects: 3 as libc::c_int,
                                  radius: 42.0f32,
                                  yAccel: 0.0f32,};
          init
      }]];
#[no_mangle]
pub unsafe extern "C" fn EnGo2_AddDust(mut this: *mut EnGo2,
                                       mut pos: *mut Vec3f,
                                       mut velocity: *mut Vec3f,
                                       mut accel: *mut Vec3f,
                                       mut initialTimer: u8_0,
                                       mut scale: f32_0,
                                       mut scaleStep: f32_0) {
    let mut dustEffect: *mut EnGoEffect = (*this).dustEffects.as_mut_ptr();
    let mut i: s16 = 0;
    let mut timer: s16 = 0;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[EnGoEffect; 10]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<EnGoEffect>()
                                                   as libc::c_ulong) as s32 {
        if (*dustEffect).type_0 as libc::c_int != 1 as libc::c_int {
            (*dustEffect).scale = scale;
            (*dustEffect).scaleStep = scaleStep;
            timer = initialTimer as s16;
            (*dustEffect).timer = timer as u8_0;
            (*dustEffect).type_0 = 1 as libc::c_int as u8_0;
            (*dustEffect).initialTimer = initialTimer;
            (*dustEffect).pos = *pos;
            (*dustEffect).accel = *accel;
            (*dustEffect).velocity = *velocity;
            return
        }
        i += 1;
        dustEffect = dustEffect.offset(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_UpdateDust(mut this: *mut EnGo2) {
    let mut dustEffect: *mut EnGoEffect = (*this).dustEffects.as_mut_ptr();
    let mut randomNumber: f32_0 = 0.;
    let mut i: s16 = 0;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[EnGoEffect; 10]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<EnGoEffect>()
                                                   as libc::c_ulong) as s32 {
        if (*dustEffect).type_0 != 0 {
            (*dustEffect).timer = (*dustEffect).timer.wrapping_sub(1);
            if (*dustEffect).timer as libc::c_int == 0 as libc::c_int {
                (*dustEffect).type_0 = 0 as libc::c_int as u8_0
            }
            (*dustEffect).accel.x = Rand_ZeroOne() * 0.4f32 - 0.2f32;
            randomNumber = Rand_ZeroOne() * 0.4f32;
            (*dustEffect).accel.z = randomNumber - 0.2f32;
            (*dustEffect).pos.x += (*dustEffect).velocity.x;
            (*dustEffect).pos.y += (*dustEffect).velocity.y;
            (*dustEffect).pos.z += (*dustEffect).velocity.z;
            (*dustEffect).velocity.x += (*dustEffect).accel.x;
            (*dustEffect).velocity.y += (*dustEffect).accel.y;
            (*dustEffect).velocity.z += randomNumber - 0.2f32;
            (*dustEffect).scale += (*dustEffect).scaleStep
        }
        i += 1;
        dustEffect = dustEffect.offset(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_DrawDust(mut this: *mut EnGo2,
                                        mut globalCtx: *mut GlobalContext) {
    let mut dustEffect: *mut EnGoEffect = (*this).dustEffects.as_mut_ptr();
    let mut alpha: s16 = 0;
    let mut firstDone: s16 = 0;
    let mut index: s16 = 0;
    let mut i: s16 = 0;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_en_go2_eff.c\x00" as *const u8 as
                        *const libc::c_char, 111 as libc::c_int);
    firstDone = 0 as libc::c_int as s16;
    func_80093D84((*globalCtx).state.gfxCtx);
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[EnGoEffect; 10]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<EnGoEffect>()
                                                   as libc::c_ulong) as s32 {
        if (*dustEffect).type_0 != 0 {
            if firstDone == 0 {
                (*__gfxCtx).polyXlu.p =
                    Gfx_CallSetupDL((*__gfxCtx).polyXlu.p,
                                    0 as libc::c_int as u32_0);
                let fresh0 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g: *mut Gfx = fresh0;
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
                (*_g).words.w1 = gGoronDL_00FD40.as_mut_ptr() as libc::c_uint;
                let fresh1 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_0: *mut Gfx = fresh1;
                (*_g_0).words.w0 =
                    (0xfb as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int;
                (*_g_0).words.w1 =
                    (100 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (60 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            16 as libc::c_int |
                        (20 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            8 as libc::c_int |
                        (0 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                firstDone = 1 as libc::c_int as s16
            }
            alpha =
                ((*dustEffect).timer as libc::c_int as libc::c_float *
                     (255.0f32 /
                          (*dustEffect).initialTimer as libc::c_int as
                              libc::c_float)) as s16;
            let fresh2 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
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
                (170 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (130 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (90 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    (alpha as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh3 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_2: *mut Gfx = fresh3;
            (*_g_2).words.w0 =
                (0xe7 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_2).words.w1 = 0 as libc::c_int as libc::c_uint;
            Matrix_Translate((*dustEffect).pos.x, (*dustEffect).pos.y,
                             (*dustEffect).pos.z,
                             MTXMODE_NEW as libc::c_int as u8_0);
            func_800D1FD4(&mut (*globalCtx).billboardMtxF);
            Matrix_Scale((*dustEffect).scale, (*dustEffect).scale, 1.0f32,
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
                              b"../z_en_go2_eff.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              137 as libc::c_int) as libc::c_uint;
            index =
                ((*dustEffect).timer as libc::c_int as libc::c_float *
                     (8.0f32 /
                          (*dustEffect).initialTimer as libc::c_int as
                              libc::c_float)) as s16;
            let fresh5 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_4: *mut Gfx = fresh5;
            (*_g_4).words.w0 =
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
            (*_g_4).words.w1 =
                gSegments[((sDustTex[index as usize] as u32_0) <<
                               4 as libc::c_int >> 28 as libc::c_int) as
                              usize].wrapping_add(sDustTex[index as usize] as
                                                      u32_0 &
                                                      0xffffff as libc::c_int
                                                          as
                                                          libc::c_uint).wrapping_add(0x80000000
                                                                                         as
                                                                                         libc::c_uint)
                    as *mut libc::c_void as libc::c_uint;
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
            (*_g_5).words.w1 = gGoronDL_00FD50.as_mut_ptr() as libc::c_uint
        }
        i += 1;
        dustEffect = dustEffect.offset(1)
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_en_go2_eff.c\x00" as *const u8 as
                         *const libc::c_char, 151 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_SpawnDust(mut this: *mut EnGo2,
                                         mut initialTimer: u8_0,
                                         mut scale: f32_0,
                                         mut scaleStep: f32_0,
                                         mut numDustEffects: s32,
                                         mut radius: f32_0, mut yAccel: f32_0)
 -> s32 {
    let mut pos: Vec3f = sPos;
    let mut velocity: Vec3f = sVelocity;
    let mut accel: Vec3f = sAccel;
    let mut i: s32 = 0;
    let mut angle: s16 = 0;
    pos = (*this).actor.world.pos;
    pos.y = (*this).actor.floorHeight;
    angle =
        ((Rand_ZeroOne() - 0.5f32) * 0x10000 as libc::c_int as libc::c_float)
            as s16;
    i = numDustEffects;
    while i >= 0 as libc::c_int {
        accel.y += Rand_ZeroOne() * yAccel;
        pos.x = Math_SinS(angle) * radius + (*this).actor.world.pos.x;
        pos.z = Math_CosS(angle) * radius + (*this).actor.world.pos.z;
        EnGo2_AddDust(this, &mut pos, &mut velocity, &mut accel, initialTimer,
                      scale, scaleStep);
        angle =
            (angle as libc::c_int +
                 (0x10000 as libc::c_int / numDustEffects) as s16 as
                     libc::c_int) as s16;
        i -= 1
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GetItem(mut this: *mut EnGo2,
                                       mut globalCtx: *mut GlobalContext,
                                       mut getItemId: s32) {
    (*this).getItemId = getItemId;
    func_8002F434(&mut (*this).actor, globalCtx, getItemId,
                  (*this).actor.xzDistToPlayer + 1.0f32,
                  fabsf((*this).actor.yDistToPlayer) + 1.0f32);
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GetDialogState(mut this: *mut EnGo2,
                                              mut globalCtx:
                                                  *mut GlobalContext) -> s32 {
    let mut dialogState: s16 =
        Message_GetState(&mut (*globalCtx).msgCtx) as s16;
    if (*this).dialogState as libc::c_int ==
           TEXT_STATE_AWAITING_NEXT as libc::c_int ||
           (*this).dialogState as libc::c_int ==
               TEXT_STATE_EVENT as libc::c_int ||
           (*this).dialogState as libc::c_int ==
               TEXT_STATE_CLOSING as libc::c_int ||
           (*this).dialogState as libc::c_int ==
               TEXT_STATE_DONE_HAS_NEXT as libc::c_int {
        if dialogState as libc::c_int != (*this).dialogState as libc::c_int {
            (*this).unk_20C = (*this).unk_20C.wrapping_add(1)
        }
    }
    (*this).dialogState = dialogState as u8_0;
    return dialogState as s32;
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GoronFireGenericGetTextId(mut this: *mut EnGo2)
 -> u16_0 {
    match ((*this).actor.params as libc::c_int & 0xfc00 as libc::c_int) >>
              0xa as libc::c_int {
        3 => { return 0x3069 as libc::c_int as u16_0 }
        5 => { return 0x306a as libc::c_int as u16_0 }
        4 => { return 0x306b as libc::c_int as u16_0 }
        2 => { return 0x306c as libc::c_int as u16_0 }
        10 => { return 0x306d as libc::c_int as u16_0 }
        8 => { return 0x306e as libc::c_int as u16_0 }
        11 => { return 0x306f as libc::c_int as u16_0 }
        1 => { return 0x3070 as libc::c_int as u16_0 }
        _ => { return 0x3052 as libc::c_int as u16_0 }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GetTextIdGoronCityRollingBig(mut globalCtx:
                                                                *mut GlobalContext,
                                                            mut this:
                                                                *mut EnGo2)
 -> u16_0 {
    if gSaveContext.infTable[17 as libc::c_int as usize] as libc::c_int &
           0x4000 as libc::c_int != 0 {
        return 0x3013 as libc::c_int as u16_0
    } else if gUpgradeCapacities[UPG_BOMB_BAG as libc::c_int as
                                     usize][((gSaveContext.inventory.upgrades
                                                  &
                                                  gUpgradeMasks[UPG_BOMB_BAG
                                                                    as
                                                                    libc::c_int
                                                                    as usize])
                                                 as s32 >>
                                                 gUpgradeShifts[UPG_BOMB_BAG
                                                                    as
                                                                    libc::c_int
                                                                    as usize]
                                                     as libc::c_int) as usize]
                  as libc::c_int >= 20 as libc::c_int &&
                  (*this).waypoint as libc::c_int > 7 as libc::c_int &&
                  ((*this).waypoint as libc::c_int) < 12 as libc::c_int {
        return 0x3012 as libc::c_int as u16_0
    } else { return 0x3011 as libc::c_int as u16_0 };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GetStateGoronCityRollingBig(mut globalCtx:
                                                               *mut GlobalContext,
                                                           mut this:
                                                               *mut EnGo2)
 -> s16 {
    let mut bombBagUpgrade: s32 = 0;
    match Message_GetState(&mut (*globalCtx).msgCtx) as libc::c_int {
        2 => { return 2 as libc::c_int as s16 }
        5 => {
            if Message_ShouldAdvance(globalCtx) != 0 {
                if (*this).actor.textId as libc::c_int ==
                       0x3012 as libc::c_int {
                    (*this).actionFunc =
                        Some(EnGo2_SetupGetItem as
                                 unsafe extern "C" fn(_: *mut EnGo2,
                                                      _: *mut GlobalContext)
                                     -> ());
                    bombBagUpgrade =
                        if gUpgradeCapacities[UPG_BOMB_BAG as libc::c_int as
                                                  usize][((gSaveContext.inventory.upgrades
                                                               &
                                                               gUpgradeMasks[UPG_BOMB_BAG
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 usize])
                                                              as s32 >>
                                                              gUpgradeShifts[UPG_BOMB_BAG
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 usize]
                                                                  as
                                                                  libc::c_int)
                                                             as usize] as
                               libc::c_int == 30 as libc::c_int {
                            GI_BOMB_BAG_40 as libc::c_int
                        } else { GI_BOMB_BAG_30 as libc::c_int };
                    EnGo2_GetItem(this, globalCtx, bombBagUpgrade);
                    Message_CloseTextbox(globalCtx);
                    gSaveContext.infTable[17 as libc::c_int as usize] =
                        (gSaveContext.infTable[17 as libc::c_int as usize] as
                             libc::c_int | 0x4000 as libc::c_int) as u16_0;
                    return 2 as libc::c_int as s16
                } else { return 2 as libc::c_int as s16 }
            }
        }
        _ => { }
    }
    return 1 as libc::c_int as s16;
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GetTextIdGoronDmtBombFlower(mut globalCtx:
                                                               *mut GlobalContext,
                                                           mut this:
                                                               *mut EnGo2)
 -> u16_0 {
    return if gBitFlags[QUEST_GORON_RUBY as libc::c_int as usize] &
                  gSaveContext.inventory.questItems != 0 {
               0x3027 as libc::c_int
           } else { 0x300a as libc::c_int } as u16_0;
}
// DMT Goron by Bomb Flower Choice
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GetStateGoronDmtBombFlower(mut globalCtx:
                                                              *mut GlobalContext,
                                                          mut this:
                                                              *mut EnGo2)
 -> s16 {
    match Message_GetState(&mut (*globalCtx).msgCtx) as libc::c_int {
        2 => {
            if (*this).actor.textId as libc::c_int == 0x300b as libc::c_int &&
                   gSaveContext.infTable[14 as libc::c_int as usize] as
                       libc::c_int & 0x800 as libc::c_int == 0 as libc::c_int
               {
                gSaveContext.infTable[14 as libc::c_int as usize] =
                    (gSaveContext.infTable[14 as libc::c_int as usize] as
                         libc::c_int | 0x800 as libc::c_int) as u16_0;
                return 2 as libc::c_int as s16
            } else { return 0 as libc::c_int as s16 }
        }
        4 => {
            if Message_ShouldAdvance(globalCtx) != 0 {
                // Ask question to DMT Goron by bomb flower
                if (*this).actor.textId as libc::c_int ==
                       0x300a as libc::c_int {
                    if (*globalCtx).msgCtx.choiceIndex as libc::c_int ==
                           0 as libc::c_int {
                        (*this).actor.textId =
                            if (gSaveContext.inventory.upgrades &
                                    gUpgradeMasks[UPG_STRENGTH as libc::c_int
                                                      as usize]) as s32 >>
                                   gUpgradeShifts[UPG_STRENGTH as libc::c_int
                                                      as usize] as libc::c_int
                                   != 0 as libc::c_int {
                                0x300b as libc::c_int
                            } else { 0x300c as libc::c_int } as u16_0
                    } else {
                        (*this).actor.textId = 0x300d as libc::c_int as u16_0
                    }
                    Message_ContinueTextbox(globalCtx, (*this).actor.textId);
                }
                return 1 as libc::c_int as s16
            }
        }
        _ => { }
    }
    return 1 as libc::c_int as s16;
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GetTextIdGoronDmtRollingSmall(mut globalCtx:
                                                                 *mut GlobalContext,
                                                             mut this:
                                                                 *mut EnGo2)
 -> u16_0 {
    if gBitFlags[QUEST_GORON_RUBY as libc::c_int as usize] &
           gSaveContext.inventory.questItems != 0 {
        return 0x3027 as libc::c_int as u16_0
    } else {
        return if gSaveContext.eventChkInf[2 as libc::c_int as usize] as
                      libc::c_int & 0x8 as libc::c_int != 0 {
                   0x3026 as libc::c_int
               } else { 0x3009 as libc::c_int } as u16_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GetStateGoronDmtRollingSmall(mut globalCtx:
                                                                *mut GlobalContext,
                                                            mut this:
                                                                *mut EnGo2)
 -> s16 {
    if Message_GetState(&mut (*globalCtx).msgCtx) as libc::c_int ==
           TEXT_STATE_CLOSING as libc::c_int {
        return 0 as libc::c_int as s16
    } else { return 1 as libc::c_int as s16 };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GetTextIdGoronDmtDcEntrance(mut globalCtx:
                                                               *mut GlobalContext,
                                                           mut this:
                                                               *mut EnGo2)
 -> u16_0 {
    if gBitFlags[QUEST_MEDALLION_FIRE as libc::c_int as usize] &
           gSaveContext.inventory.questItems != 0 &&
           gSaveContext.linkAge == 0 as libc::c_int {
        return 0x3043 as libc::c_int as u16_0
    } else if gBitFlags[QUEST_GORON_RUBY as libc::c_int as usize] &
                  gSaveContext.inventory.questItems != 0 {
        return 0x3027 as libc::c_int as u16_0
    } else {
        return if gSaveContext.eventChkInf[2 as libc::c_int as usize] as
                      libc::c_int & 0x8 as libc::c_int != 0 {
                   0x3021 as libc::c_int
               } else if gSaveContext.infTable[14 as libc::c_int as usize] as
                             libc::c_int & 0x1 as libc::c_int != 0 {
                   0x302a as libc::c_int
               } else { 0x3008 as libc::c_int } as u16_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GetStateGoronDmtDcEntrance(mut globalCtx:
                                                              *mut GlobalContext,
                                                          mut this:
                                                              *mut EnGo2)
 -> s16 {
    if Message_GetState(&mut (*globalCtx).msgCtx) as libc::c_int ==
           TEXT_STATE_CLOSING as libc::c_int {
        if (*this).actor.textId as libc::c_int == 0x3008 as libc::c_int {
            gSaveContext.infTable[14 as libc::c_int as usize] =
                (gSaveContext.infTable[14 as libc::c_int as usize] as
                     libc::c_int | 0x1 as libc::c_int) as u16_0
        }
        return 0 as libc::c_int as s16
    } else { return 1 as libc::c_int as s16 };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GetTextIdGoronCityEntrance(mut globalCtx:
                                                              *mut GlobalContext,
                                                          mut this:
                                                              *mut EnGo2)
 -> u16_0 {
    if gBitFlags[QUEST_MEDALLION_FIRE as libc::c_int as usize] &
           gSaveContext.inventory.questItems != 0 &&
           gSaveContext.linkAge == 0 as libc::c_int {
        return 0x3043 as libc::c_int as u16_0
    } else if gBitFlags[QUEST_GORON_RUBY as libc::c_int as usize] &
                  gSaveContext.inventory.questItems != 0 {
        return 0x3027 as libc::c_int as u16_0
    } else {
        return if gSaveContext.infTable[15 as libc::c_int as usize] as
                      libc::c_int & 0x1 as libc::c_int != 0 {
                   0x3015 as libc::c_int
               } else { 0x3014 as libc::c_int } as u16_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GetStateGoronCityEntrance(mut globalCtx:
                                                             *mut GlobalContext,
                                                         mut this: *mut EnGo2)
 -> s16 {
    if Message_GetState(&mut (*globalCtx).msgCtx) as libc::c_int ==
           TEXT_STATE_CLOSING as libc::c_int {
        if (*this).actor.textId as libc::c_int == 0x3014 as libc::c_int {
            gSaveContext.infTable[15 as libc::c_int as usize] =
                (gSaveContext.infTable[15 as libc::c_int as usize] as
                     libc::c_int | 0x1 as libc::c_int) as u16_0
        }
        return 0 as libc::c_int as s16
    } else { return 1 as libc::c_int as s16 };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GetTextIdGoronCityIsland(mut globalCtx:
                                                            *mut GlobalContext,
                                                        mut this: *mut EnGo2)
 -> u16_0 {
    if gBitFlags[QUEST_MEDALLION_FIRE as libc::c_int as usize] &
           gSaveContext.inventory.questItems != 0 &&
           gSaveContext.linkAge == 0 as libc::c_int {
        return 0x3043 as libc::c_int as u16_0
    } else if gBitFlags[QUEST_GORON_RUBY as libc::c_int as usize] &
                  gSaveContext.inventory.questItems != 0 {
        return 0x3067 as libc::c_int as u16_0
    } else {
        return if gSaveContext.infTable[15 as libc::c_int as usize] as
                      libc::c_int & 0x10 as libc::c_int != 0 {
                   0x3017 as libc::c_int
               } else { 0x3016 as libc::c_int } as u16_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GetStateGoronCityIsland(mut globalCtx:
                                                           *mut GlobalContext,
                                                       mut this: *mut EnGo2)
 -> s16 {
    if Message_GetState(&mut (*globalCtx).msgCtx) as libc::c_int ==
           TEXT_STATE_CLOSING as libc::c_int {
        if (*this).actor.textId as libc::c_int == 0x3016 as libc::c_int {
            gSaveContext.infTable[15 as libc::c_int as usize] =
                (gSaveContext.infTable[15 as libc::c_int as usize] as
                     libc::c_int | 0x10 as libc::c_int) as u16_0
        }
        return 0 as libc::c_int as s16
    } else { return 1 as libc::c_int as s16 };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GetTextIdGoronCityLowestFloor(mut globalCtx:
                                                                 *mut GlobalContext,
                                                             mut this:
                                                                 *mut EnGo2)
 -> u16_0 {
    if gBitFlags[QUEST_MEDALLION_FIRE as libc::c_int as usize] &
           gSaveContext.inventory.questItems != 0 &&
           gSaveContext.linkAge == 0 as libc::c_int {
        return 0x3043 as libc::c_int as u16_0
    } else if gBitFlags[QUEST_GORON_RUBY as libc::c_int as usize] &
                  gSaveContext.inventory.questItems != 0 {
        return 0x3027 as libc::c_int as u16_0
    } else {
        return if (gSaveContext.inventory.upgrades &
                       gUpgradeMasks[UPG_STRENGTH as libc::c_int as usize]) as
                      s32 >>
                      gUpgradeShifts[UPG_STRENGTH as libc::c_int as usize] as
                          libc::c_int != 0 as libc::c_int {
                   0x302c as libc::c_int
               } else if Flags_GetSwitch(globalCtx, 0x1b as libc::c_int) == 0
                {
                   0x3017 as libc::c_int
               } else if gSaveContext.infTable[15 as libc::c_int as usize] as
                             libc::c_int & 0x100 as libc::c_int != 0 {
                   0x3019 as libc::c_int
               } else { 0x3018 as libc::c_int } as u16_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GetStateGoronCityLowestFloor(mut globalCtx:
                                                                *mut GlobalContext,
                                                            mut this:
                                                                *mut EnGo2)
 -> s16 {
    if Message_GetState(&mut (*globalCtx).msgCtx) as libc::c_int ==
           TEXT_STATE_CLOSING as libc::c_int {
        if (*this).actor.textId as libc::c_int == 0x3018 as libc::c_int {
            gSaveContext.infTable[15 as libc::c_int as usize] =
                (gSaveContext.infTable[15 as libc::c_int as usize] as
                     libc::c_int | 0x100 as libc::c_int) as u16_0
        }
        return 0 as libc::c_int as s16
    } else { return 1 as libc::c_int as s16 };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GetTextIdGoronCityLink(mut globalCtx:
                                                          *mut GlobalContext,
                                                      mut this: *mut EnGo2)
 -> u16_0 {
    if gBitFlags[QUEST_MEDALLION_FIRE as libc::c_int as usize] &
           gSaveContext.inventory.questItems != 0 {
        return if gSaveContext.infTable[16 as libc::c_int as usize] as
                      libc::c_int & 0x8000 as libc::c_int != 0 {
                   0x3042 as libc::c_int
               } else { 0x3041 as libc::c_int } as u16_0
    } else if gBitFlags[1 as libc::c_int as usize] <<
                  gEquipShifts[EQUIP_TUNIC as libc::c_int as usize] as
                      libc::c_int &
                  gSaveContext.inventory.equipment as libc::c_uint != 0 {
        return if gSaveContext.infTable[16 as libc::c_int as usize] as
                      libc::c_int & 0x4000 as libc::c_int != 0 {
                   0x3038 as libc::c_int
               } else { 0x3037 as libc::c_int } as u16_0
    } else if gSaveContext.infTable[16 as libc::c_int as usize] as libc::c_int
                  & 0x1000 as libc::c_int != 0 {
        (*this).unk_20C = 0 as libc::c_int as u8_0;
        (*this).dialogState = TEXT_STATE_NONE as libc::c_int as u8_0;
        return if gSaveContext.infTable[16 as libc::c_int as usize] as
                      libc::c_int & 0x400 as libc::c_int != 0 {
                   0x3033 as libc::c_int
               } else { 0x3032 as libc::c_int } as u16_0
    } else { return 0x3030 as libc::c_int as u16_0 };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GetStateGoronCityLink(mut globalCtx:
                                                         *mut GlobalContext,
                                                     mut this: *mut EnGo2)
 -> s16 {
    match EnGo2_GetDialogState(this, globalCtx) {
        2 => {
            match (*this).actor.textId as libc::c_int {
                12342 => {
                    EnGo2_GetItem(this, globalCtx,
                                  GI_TUNIC_GORON as libc::c_int);
                    (*this).actionFunc =
                        Some(EnGo2_SetupGetItem as
                                 unsafe extern "C" fn(_: *mut EnGo2,
                                                      _: *mut GlobalContext)
                                     -> ());
                    return 2 as libc::c_int as s16
                }
                12343 => {
                    gSaveContext.infTable[16 as libc::c_int as usize] =
                        (gSaveContext.infTable[16 as libc::c_int as usize] as
                             libc::c_int | 0x4000 as libc::c_int) as u16_0
                }
                _ => { }
            }
            return 0 as libc::c_int as s16
        }
        4 => {
            if Message_ShouldAdvance(globalCtx) != 0 {
                if (*this).actor.textId as libc::c_int ==
                       0x3034 as libc::c_int {
                    if (*globalCtx).msgCtx.choiceIndex as libc::c_int ==
                           0 as libc::c_int {
                        (*this).actor.textId =
                            if gSaveContext.infTable[16 as libc::c_int as
                                                         usize] as libc::c_int
                                   & 0x800 as libc::c_int != 0 {
                                0x3033 as libc::c_int
                            } else { 0x3035 as libc::c_int } as u16_0;
                        if (*this).actor.textId as libc::c_int ==
                               0x3035 as libc::c_int {
                            Audio_StopSfxById(0x39eb as libc::c_int as u32_0);
                        }
                    } else {
                        (*this).actor.textId =
                            if gSaveContext.infTable[16 as libc::c_int as
                                                         usize] as libc::c_int
                                   & 0x800 as libc::c_int != 0 {
                                0x3036 as libc::c_int
                            } else { 0x3033 as libc::c_int } as u16_0;
                        if (*this).actor.textId as libc::c_int ==
                               0x3036 as libc::c_int {
                            Audio_StopSfxById(0x39eb as libc::c_int as u32_0);
                        }
                    }
                    Message_ContinueTextbox(globalCtx, (*this).actor.textId);
                    (*this).unk_20C = 0 as libc::c_int as u8_0
                }
                return 1 as libc::c_int as s16
            }
        }
        5 => {
            if Message_ShouldAdvance(globalCtx) != 0 {
                match (*this).actor.textId as libc::c_int {
                    12341 => {
                        gSaveContext.infTable[16 as libc::c_int as usize] =
                            (gSaveContext.infTable[16 as libc::c_int as usize]
                                 as libc::c_int | 0x800 as libc::c_int) as
                                u16_0
                    }
                    12338 | 12339 => { }
                    _ => { return 2 as libc::c_int as s16 }
                }
                (*this).actor.textId = 0x3034 as libc::c_int as u16_0;
                Message_ContinueTextbox(globalCtx, (*this).actor.textId);
                return 1 as libc::c_int as s16
            }
        }
        _ => { }
    }
    return 1 as libc::c_int as s16;
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GetTextIdGoronDmtBiggoron(mut globalCtx:
                                                             *mut GlobalContext,
                                                         mut this: *mut EnGo2)
 -> u16_0 {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    if gSaveContext.bgsFlag != 0 {
        (*player).exchangeItemId = EXCH_ITEM_CLAIM_CHECK as libc::c_int as s8;
        return 0x305e as libc::c_int as u16_0
    } else if gSaveContext.inventory.items[gItemSlots[ITEM_POCKET_EGG as
                                                          libc::c_int as
                                                          usize] as usize] as
                  libc::c_int >= ITEM_CLAIM_CHECK as libc::c_int {
        (*player).exchangeItemId = EXCH_ITEM_CLAIM_CHECK as libc::c_int as s8;
        return 0x305e as libc::c_int as u16_0
    } else if gSaveContext.inventory.items[gItemSlots[ITEM_POCKET_EGG as
                                                          libc::c_int as
                                                          usize] as usize] as
                  libc::c_int >= ITEM_PRESCRIPTION as libc::c_int {
        (*player).exchangeItemId = EXCH_ITEM_EYEDROPS as libc::c_int as s8;
        return 0x3058 as libc::c_int as u16_0
    } else {
        (*player).exchangeItemId =
            EXCH_ITEM_SWORD_BROKEN as libc::c_int as s8;
        return 0x3053 as libc::c_int as u16_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GetStateGoronDmtBiggoron(mut globalCtx:
                                                            *mut GlobalContext,
                                                        mut this: *mut EnGo2)
 -> s16 {
    let mut unusedPad: s32 = 0;
    let mut dialogState: u8_0 = (*this).dialogState;
    match EnGo2_GetDialogState(this, globalCtx) {
        6 => {
            if (*this).actor.textId as libc::c_int == 0x305e as libc::c_int {
                if gSaveContext.bgsFlag == 0 {
                    EnGo2_GetItem(this, globalCtx,
                                  GI_SWORD_BGS as libc::c_int);
                    (*this).actionFunc =
                        Some(EnGo2_SetupGetItem as
                                 unsafe extern "C" fn(_: *mut EnGo2,
                                                      _: *mut GlobalContext)
                                     -> ());
                    return 2 as libc::c_int as s16
                } else { return 0 as libc::c_int as s16 }
            } else { return 0 as libc::c_int as s16 }
        }
        3 => {
            let mut current_block_16: u64;
            match (*this).actor.textId as libc::c_int {
                12382 => {
                    if func_8002F368(globalCtx) as libc::c_int !=
                           EXCH_ITEM_CLAIM_CHECK as libc::c_int {
                        current_block_16 = 4808432441040389987;
                    } else { current_block_16 = 11664675996575349030; }
                }
                12377 => { current_block_16 = 11664675996575349030; }
                12372 => { current_block_16 = 5612100202445834412; }
                _ => { current_block_16 = 4808432441040389987; }
            }
            match current_block_16 {
                11664675996575349030 => {
                    if dialogState as libc::c_int ==
                           TEXT_STATE_NONE as libc::c_int {
                        func_800F4524(&mut D_801333D4,
                                      0x38fc as libc::c_int as u16_0,
                                      60 as libc::c_int as s8);
                    }
                    current_block_16 = 5612100202445834412;
                }
                _ => { }
            }
            match current_block_16 {
                5612100202445834412 => {
                    if dialogState as libc::c_int ==
                           TEXT_STATE_NONE as libc::c_int {
                        Audio_PlaySoundGeneral(0x4807 as libc::c_int as u16_0,
                                               &mut D_801333D4,
                                               4 as libc::c_int as u8_0,
                                               &mut D_801333E0,
                                               &mut D_801333E0,
                                               &mut D_801333E8);
                    }
                }
                _ => { }
            }
            return 1 as libc::c_int as s16
        }
        4 => {
            if Message_ShouldAdvance(globalCtx) != 0 {
                if (*this).actor.textId as libc::c_int ==
                       0x3054 as libc::c_int ||
                       (*this).actor.textId as libc::c_int ==
                           0x3055 as libc::c_int {
                    if (*globalCtx).msgCtx.choiceIndex as libc::c_int ==
                           0 as libc::c_int {
                        EnGo2_GetItem(this, globalCtx,
                                      GI_PRESCRIPTION as libc::c_int);
                        (*this).actionFunc =
                            Some(EnGo2_SetupGetItem as
                                     unsafe extern "C" fn(_: *mut EnGo2,
                                                          _:
                                                              *mut GlobalContext)
                                         -> ());
                        return 2 as libc::c_int as s16
                    }
                    (*this).actor.textId = 0x3056 as libc::c_int as u16_0;
                    Message_ContinueTextbox(globalCtx, (*this).actor.textId);
                }
                return 1 as libc::c_int as s16
            }
        }
        5 => {
            if Message_ShouldAdvance(globalCtx) != 0 {
                if (*this).actor.textId as libc::c_int ==
                       0x3059 as libc::c_int {
                    (*globalCtx).msgCtx.msgMode =
                        MSGMODE_PAUSED as libc::c_int as u8_0;
                    (*this).actionFunc =
                        Some(EnGo2_BiggoronEyedrops as
                                 unsafe extern "C" fn(_: *mut EnGo2,
                                                      _: *mut GlobalContext)
                                     -> ())
                }
                return 2 as libc::c_int as s16
            }
        }
        _ => { }
    }
    return 1 as libc::c_int as s16;
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GetTextIdGoronFireGeneric(mut globalCtx:
                                                             *mut GlobalContext,
                                                         mut this: *mut EnGo2)
 -> u16_0 {
    if Flags_GetSwitch(globalCtx,
                       ((*this).actor.params as libc::c_int &
                            0xfc00 as libc::c_int) >> 0xa as libc::c_int) != 0
       {
        return 0x3071 as libc::c_int as u16_0
    } else { return 0x3051 as libc::c_int as u16_0 };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GetStateGoronFireGeneric(mut globalCtx:
                                                            *mut GlobalContext,
                                                        mut this: *mut EnGo2)
 -> s16 {
    match Message_GetState(&mut (*globalCtx).msgCtx) as libc::c_int {
        2 => { return 0 as libc::c_int as s16 }
        5 => {
            if Message_ShouldAdvance(globalCtx) != 0 {
                if (*this).actor.textId as libc::c_int ==
                       0x3071 as libc::c_int {
                    (*this).actor.textId =
                        EnGo2_GoronFireGenericGetTextId(this);
                    Message_ContinueTextbox(globalCtx, (*this).actor.textId);
                }
                return 1 as libc::c_int as s16
            }
        }
        _ => { }
    }
    return 1 as libc::c_int as s16;
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GetTextIdGoronCityStairwell(mut globalCtx:
                                                               *mut GlobalContext,
                                                           mut this:
                                                               *mut EnGo2)
 -> u16_0 {
    return if !(gSaveContext.linkAge == 0 as libc::c_int) {
               if gSaveContext.infTable[14 as libc::c_int as usize] as
                      libc::c_int & 0x8 as libc::c_int != 0 {
                   0x3022 as libc::c_int
               } else { 0x300e as libc::c_int }
           } else { 0x3043 as libc::c_int } as u16_0;
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GetStateGoronCityStairwell(mut globalCtx:
                                                              *mut GlobalContext,
                                                          mut this:
                                                              *mut EnGo2)
 -> s16 {
    if Message_GetState(&mut (*globalCtx).msgCtx) as libc::c_int ==
           TEXT_STATE_CLOSING as libc::c_int {
        if (*this).actor.textId as libc::c_int == 0x300e as libc::c_int {
            gSaveContext.infTable[14 as libc::c_int as usize] =
                (gSaveContext.infTable[14 as libc::c_int as usize] as
                     libc::c_int | 0x8 as libc::c_int) as u16_0
        }
        return 0 as libc::c_int as s16
    } else { return 1 as libc::c_int as s16 };
}
// Goron in child market bazaar after obtaining Goron Ruby
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GetTextIdGoronMarketBazaar(mut globalCtx:
                                                              *mut GlobalContext,
                                                          mut this:
                                                              *mut EnGo2)
 -> u16_0 {
    return 0x7122 as libc::c_int as u16_0;
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GetStateGoronMarketBazaar(mut globalCtx:
                                                             *mut GlobalContext,
                                                         mut this: *mut EnGo2)
 -> s16 {
    if Message_GetState(&mut (*globalCtx).msgCtx) as libc::c_int ==
           TEXT_STATE_CLOSING as libc::c_int {
        return 0 as libc::c_int as s16
    } else { return 1 as libc::c_int as s16 };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GetTextIdGoronCityLostWoods(mut globalCtx:
                                                               *mut GlobalContext,
                                                           mut this:
                                                               *mut EnGo2)
 -> u16_0 {
    if !(gSaveContext.linkAge == 0 as libc::c_int) {
        if Flags_GetSwitch(globalCtx, 0x1c as libc::c_int) != 0 {
            return 0x302f as libc::c_int as u16_0
        } else {
            return if gSaveContext.infTable[14 as libc::c_int as usize] as
                          libc::c_int & 0x40 as libc::c_int != 0 {
                       0x3025 as libc::c_int
                   } else { 0x3024 as libc::c_int } as u16_0
        }
    } else { return 0x3043 as libc::c_int as u16_0 };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GetStateGoronCityLostWoods(mut globalCtx:
                                                              *mut GlobalContext,
                                                          mut this:
                                                              *mut EnGo2)
 -> s16 {
    if Message_GetState(&mut (*globalCtx).msgCtx) as libc::c_int ==
           TEXT_STATE_CLOSING as libc::c_int {
        if (*this).actor.textId as libc::c_int == 0x3024 as libc::c_int {
            gSaveContext.infTable[14 as libc::c_int as usize] =
                (gSaveContext.infTable[14 as libc::c_int as usize] as
                     libc::c_int | 0x40 as libc::c_int) as u16_0
        }
        return 0 as libc::c_int as s16
    } else { return 1 as libc::c_int as s16 };
}
// Goron at base of DMT summit
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GetTextIdGoronDmtFairyHint(mut globalCtx:
                                                              *mut GlobalContext,
                                                          mut this:
                                                              *mut EnGo2)
 -> u16_0 {
    if !(gSaveContext.linkAge == 0 as libc::c_int) {
        return if gBitFlags[QUEST_GORON_RUBY as libc::c_int as usize] &
                      gSaveContext.inventory.questItems != 0 {
                   0x3065 as libc::c_int
               } else { 0x3064 as libc::c_int } as u16_0
    } else { return 0x3043 as libc::c_int as u16_0 };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GetStateGoronDmtFairyHint(mut globalCtx:
                                                             *mut GlobalContext,
                                                         mut this: *mut EnGo2)
 -> s16 {
    if Message_GetState(&mut (*globalCtx).msgCtx) as libc::c_int ==
           TEXT_STATE_CLOSING as libc::c_int {
        return 0 as libc::c_int as s16
    } else { return 1 as libc::c_int as s16 };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GetTextId(mut globalCtx: *mut GlobalContext,
                                         mut thisx: *mut Actor) -> u16_0 {
    let mut this: *mut EnGo2 = thisx as *mut EnGo2;
    let mut faceReaction: u16_0 =
        Text_GetFaceReaction(globalCtx, 0x20 as libc::c_int as u32_0);
    if faceReaction != 0 {
        return faceReaction
    } else {
        match (*this).actor.params as libc::c_int & 0x1f as libc::c_int {
            0 => {
                return EnGo2_GetTextIdGoronCityRollingBig(globalCtx, this)
            }
            1 => { return EnGo2_GetTextIdGoronCityLink(globalCtx, this) }
            2 => { return EnGo2_GetTextIdGoronDmtBiggoron(globalCtx, this) }
            3 => { return EnGo2_GetTextIdGoronFireGeneric(globalCtx, this) }
            4 => { return EnGo2_GetTextIdGoronDmtBombFlower(globalCtx, this) }
            5 => {
                return EnGo2_GetTextIdGoronDmtRollingSmall(globalCtx, this)
            }
            6 => { return EnGo2_GetTextIdGoronDmtDcEntrance(globalCtx, this) }
            7 => { return EnGo2_GetTextIdGoronCityEntrance(globalCtx, this) }
            8 => { return EnGo2_GetTextIdGoronCityIsland(globalCtx, this) }
            9 => {
                return EnGo2_GetTextIdGoronCityLowestFloor(globalCtx, this)
            }
            10 => {
                return EnGo2_GetTextIdGoronCityStairwell(globalCtx, this)
            }
            11 => {
                return EnGo2_GetTextIdGoronCityLostWoods(globalCtx, this)
            }
            12 => { return EnGo2_GetTextIdGoronDmtFairyHint(globalCtx, this) }
            13 => { return EnGo2_GetTextIdGoronMarketBazaar(globalCtx, this) }
            _ => { }
        }
    }
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GetState(mut globalCtx: *mut GlobalContext,
                                        mut thisx: *mut Actor) -> s16 {
    let mut this: *mut EnGo2 = thisx as *mut EnGo2;
    match (*this).actor.params as libc::c_int & 0x1f as libc::c_int {
        0 => { return EnGo2_GetStateGoronCityRollingBig(globalCtx, this) }
        1 => { return EnGo2_GetStateGoronCityLink(globalCtx, this) }
        2 => { return EnGo2_GetStateGoronDmtBiggoron(globalCtx, this) }
        3 => { return EnGo2_GetStateGoronFireGeneric(globalCtx, this) }
        4 => { return EnGo2_GetStateGoronDmtBombFlower(globalCtx, this) }
        5 => { return EnGo2_GetStateGoronDmtRollingSmall(globalCtx, this) }
        6 => { return EnGo2_GetStateGoronDmtDcEntrance(globalCtx, this) }
        7 => { return EnGo2_GetStateGoronCityEntrance(globalCtx, this) }
        8 => { return EnGo2_GetStateGoronCityIsland(globalCtx, this) }
        9 => { return EnGo2_GetStateGoronCityLowestFloor(globalCtx, this) }
        10 => { return EnGo2_GetStateGoronCityStairwell(globalCtx, this) }
        11 => { return EnGo2_GetStateGoronCityLostWoods(globalCtx, this) }
        12 => { return EnGo2_GetStateGoronDmtFairyHint(globalCtx, this) }
        13 => { return EnGo2_GetStateGoronMarketBazaar(globalCtx, this) }
        _ => { }
    }
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn func_80A44790(mut this: *mut EnGo2,
                                       mut globalCtx: *mut GlobalContext)
 -> s32 {
    if (*this).actor.params as libc::c_int & 0x1f as libc::c_int !=
           GORON_DMT_BIGGORON as libc::c_int &&
           (*this).actor.params as libc::c_int & 0x1f as libc::c_int !=
               GORON_CITY_ROLLING_BIG as libc::c_int {
        return func_800343CC(globalCtx, &mut (*this).actor,
                             &mut (*this).unk_194.unk_00, (*this).unk_218,
                             Some(EnGo2_GetTextId as
                                      unsafe extern "C" fn(_:
                                                               *mut GlobalContext,
                                                           _: *mut Actor)
                                          -> u16_0),
                             Some(EnGo2_GetState as
                                      unsafe extern "C" fn(_:
                                                               *mut GlobalContext,
                                                           _: *mut Actor)
                                          -> s16))
    } else if (*this).actor.params as libc::c_int & 0x1f as libc::c_int ==
                  GORON_DMT_BIGGORON as libc::c_int &&
                  (*this).collider.base.ocFlags2 as libc::c_int &
                      1 as libc::c_int == 0 as libc::c_int {
        return 0 as libc::c_int
    } else {
        if Actor_ProcessTalkRequest(&mut (*this).actor, globalCtx) != 0 {
            (*this).unk_194.unk_00 = 1 as libc::c_int as s16;
            return 1 as libc::c_int
        } else {
            if (*this).unk_194.unk_00 as libc::c_int != 0 as libc::c_int {
                (*this).unk_194.unk_00 =
                    EnGo2_GetState(globalCtx, &mut (*this).actor);
                return 0 as libc::c_int
            } else {
                if func_8002F2CC(&mut (*this).actor, globalCtx,
                                 (*this).unk_218) != 0 {
                    (*this).actor.textId =
                        EnGo2_GetTextId(globalCtx, &mut (*this).actor)
                }
            }
        }
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_SetColliderDim(mut this: *mut EnGo2) {
    let mut index: u8_0 =
        ((*this).actor.params as libc::c_int & 0x1f as libc::c_int) as u8_0;
    (*this).collider.dim.radius = D_80A4816C[index as usize].radius;
    (*this).collider.dim.height = D_80A4816C[index as usize].height;
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_SetShape(mut this: *mut EnGo2) {
    let mut index: u8_0 =
        ((*this).actor.params as libc::c_int & 0x1f as libc::c_int) as u8_0;
    (*this).actor.shape.shadowScale = D_80A481F8[index as usize].shape_unk_10;
    Actor_SetScale(&mut (*this).actor, D_80A481F8[index as usize].scale);
    (*this).actor.targetMode = D_80A481F8[index as usize].actor_unk_1F;
    (*this).unk_218 = D_80A481F8[index as usize].unk_218;
    (*this).unk_218 +=
        (*this).collider.dim.radius as libc::c_int as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_CheckCollision(mut this: *mut EnGo2,
                                              mut globalCtx:
                                                  *mut GlobalContext) {
    let mut pos: Vec3s = Vec3s{x: 0, y: 0, z: 0,};
    let mut xzDist: f32_0 = 0.;
    pos.x = (*this).actor.world.pos.x as s16;
    pos.y = (*this).actor.world.pos.y as s16;
    pos.z = (*this).actor.world.pos.z as s16;
    xzDist =
        D_80A4816C[((*this).actor.params as libc::c_int & 0x1f as libc::c_int)
                       as usize].xzDist as f32_0;
    pos.x =
        (pos.x as libc::c_int +
             (xzDist * Math_SinS((*this).actor.shape.rot.y)) as s16 as
                 libc::c_int) as s16;
    pos.z =
        (pos.z as libc::c_int +
             (xzDist * Math_CosS((*this).actor.shape.rot.y)) as s16 as
                 libc::c_int) as s16;
    pos.y =
        (pos.y as libc::c_int +
             D_80A4816C[((*this).actor.params as libc::c_int &
                             0x1f as libc::c_int) as usize].yDist as
                 libc::c_int) as s16;
    (*this).collider.dim.pos = pos;
    CollisionCheck_SetOC(globalCtx, &mut (*globalCtx).colChkCtx,
                         &mut (*this).collider.base);
    CollisionCheck_SetAC(globalCtx, &mut (*globalCtx).colChkCtx,
                         &mut (*this).collider.base);
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_SwapInitialFrameAnimFrameCount(mut this:
                                                                  *mut EnGo2) {
    let mut initialFrame: f32_0 = 0.;
    initialFrame = (*this).skelAnime.startFrame;
    (*this).skelAnime.startFrame = (*this).skelAnime.endFrame;
    (*this).skelAnime.endFrame = initialFrame;
}
#[no_mangle]
pub unsafe extern "C" fn func_80A44AB0(mut this: *mut EnGo2,
                                       mut globalCtx: *mut GlobalContext)
 -> s32 {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut arg2: f32_0 = 0.;
    if (*this).actor.params as libc::c_int & 0x1f as libc::c_int ==
           GORON_DMT_BIGGORON as libc::c_int {
        return 0 as libc::c_int
    } else {
        if (*this).actionFunc !=
               Some(EnGo2_SlowRolling as
                        unsafe extern "C" fn(_: *mut EnGo2,
                                             _: *mut GlobalContext) -> ()) &&
               (*this).actionFunc !=
                   Some(EnGo2_ReverseRolling as
                            unsafe extern "C" fn(_: *mut EnGo2,
                                                 _: *mut GlobalContext) -> ())
               &&
               (*this).actionFunc !=
                   Some(EnGo2_ContinueRolling as
                            unsafe extern "C" fn(_: *mut EnGo2,
                                                 _: *mut GlobalContext) -> ())
           {
            return 0 as libc::c_int
        } else {
            if (*this).collider.base.acFlags as libc::c_int & 2 as libc::c_int
                   != 0 {
                Audio_PlaySoundGeneral(0x4802 as libc::c_int as u16_0,
                                       &mut D_801333D4,
                                       4 as libc::c_int as u8_0,
                                       &mut D_801333E0, &mut D_801333E0,
                                       &mut D_801333E8);
                (*this).actor.flags &=
                    !((1 as libc::c_int) << 24 as libc::c_int) as
                        libc::c_uint;
                (*this).collider.base.acFlags =
                    ((*this).collider.base.acFlags as libc::c_int &
                         !(0x2 as libc::c_int)) as u8_0;
                EnGo2_StopRolling(this, globalCtx);
                return 1 as libc::c_int
            }
            if (*player).invincibilityTimer as libc::c_int <= 0 as libc::c_int
               {
                (*this).collider.base.ocFlags1 =
                    ((*this).collider.base.ocFlags1 as libc::c_int |
                         8 as libc::c_int) as u8_0
            } else { return 0 as libc::c_int }
            if (*this).collider.base.ocFlags2 as libc::c_int &
                   1 as libc::c_int != 0 {
                (*this).collider.base.ocFlags2 =
                    ((*this).collider.base.ocFlags2 as libc::c_int &
                         !(1 as libc::c_int)) as u8_0;
                arg2 =
                    if (*this).actionFunc ==
                           Some(EnGo2_ContinueRolling as
                                    unsafe extern "C" fn(_: *mut EnGo2,
                                                         _:
                                                             *mut GlobalContext)
                                        -> ()) {
                        1.5f32
                    } else { ((*this).actor.speedXZ) * 1.5f32 };
                (*globalCtx).damagePlayer.expect("non-null function pointer")(globalCtx,
                                                                              -(4
                                                                                    as
                                                                                    libc::c_int));
                func_8002F71C(globalCtx, &mut (*this).actor, arg2,
                              (*this).actor.yawTowardsPlayer, 6.0f32);
                Audio_PlayActorSound2(&mut (*player).actor,
                                      0x83e as libc::c_int as u16_0);
                (*this).collider.base.ocFlags1 =
                    ((*this).collider.base.ocFlags1 as libc::c_int &
                         !(0x8 as libc::c_int)) as u8_0
            }
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_UpdateWaypoint(mut this: *mut EnGo2,
                                              mut globalCtx:
                                                  *mut GlobalContext) -> s32 {
    let mut change: s32 = 0;
    if (*this).path.is_null() { return 0 as libc::c_int }
    change =
        ((*(*this).path).count as libc::c_int - 1 as libc::c_int) as u8_0 as
            s32;
    if (*this).reverse != 0 {
        (*this).waypoint -= 1;
        if ((*this).waypoint as libc::c_int) < 0 as libc::c_int {
            (*this).waypoint = (change - 1 as libc::c_int) as s8
        }
    } else {
        (*this).waypoint += 1;
        if (*this).waypoint as libc::c_int >= change {
            (*this).waypoint = 0 as libc::c_int as s8
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_Orient(mut this: *mut EnGo2,
                                      mut globalCtx: *mut GlobalContext)
 -> s32 {
    let mut targetYaw: s16 = 0;
    let mut waypointDistSq: f32_0 =
        Path_OrientAndGetDistSq(&mut (*this).actor, (*this).path,
                                (*this).waypoint as s16, &mut targetYaw);
    Math_SmoothStepToS(&mut (*this).actor.world.rot.y, targetYaw,
                       6 as libc::c_int as s16, 4000 as libc::c_int as s16,
                       1 as libc::c_int as s16);
    if waypointDistSq > 0.0f32 && waypointDistSq < 30.0f32 * 30.0f32 {
        return EnGo2_UpdateWaypoint(this, globalCtx)
    } else { return 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn func_80A44D84(mut this: *mut EnGo2) -> s32 {
    let mut targetYaw: s16 = 0;
    Path_OrientAndGetDistSq(&mut (*this).actor, (*this).path,
                            (*this).waypoint as s16, &mut targetYaw);
    (*this).actor.world.rot.y = targetYaw;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_IsWakingUp(mut this: *mut EnGo2) -> s32 {
    let mut yawDiff: s16 = 0;
    let mut xyzDist: f32_0 =
        if (*this).actor.params as libc::c_int & 0x1f as libc::c_int ==
               GORON_DMT_BIGGORON as libc::c_int {
            800.0f32
        } else { 200.0f32 };
    let mut yDist: f32_0 =
        if (*this).actor.params as libc::c_int & 0x1f as libc::c_int ==
               GORON_DMT_BIGGORON as libc::c_int {
            400.0f32
        } else { 60.0f32 };
    let mut yawDiffAbs: s16 = 0;
    if (*this).actor.params as libc::c_int & 0x1f as libc::c_int ==
           GORON_DMT_BIGGORON as libc::c_int {
        if (*this).collider.base.ocFlags2 as libc::c_int & 1 as libc::c_int ==
               0 as libc::c_int {
            (*this).actor.flags &=
                !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
            return 0 as libc::c_int
        } else {
            (*this).actor.flags |=
                ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
            return 1 as libc::c_int
        }
    }
    xyzDist = xyzDist * xyzDist;
    yawDiff =
        ((*this).actor.yawTowardsPlayer as f32_0 -
             (*this).actor.shape.rot.y as f32_0) as s16;
    yawDiffAbs =
        if yawDiff as libc::c_int >= 0 as libc::c_int {
            yawDiff as libc::c_int
        } else { -(yawDiff as libc::c_int) } as s16;
    if (*this).actor.xyzDistToPlayerSq <= xyzDist &&
           fabsf((*this).actor.yDistToPlayer) < yDist &&
           (yawDiffAbs as libc::c_int) < 0x2aa8 as libc::c_int {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_IsRollingOnGround(mut this: *mut EnGo2,
                                                 mut arg1: s16,
                                                 mut arg2: f32_0,
                                                 mut arg3: s16) -> s32 {
    if (*this).actor.bgCheckFlags as libc::c_int & 1 as libc::c_int ==
           0 as libc::c_int || (*this).actor.velocity.y > 0.0f32 {
        return 0 as libc::c_int
    }
    if if (*this).unk_590 as libc::c_int == 0 as libc::c_int {
           0 as libc::c_int
       } else { (*this).unk_590 -= 1; (*this).unk_590 as libc::c_int } != 0 {
        if arg3 == 0 {
            return 1 as libc::c_int
        } else {
            (*this).actor.world.pos.y =
                if (*this).unk_590 as libc::c_int & 1 as libc::c_int != 0 {
                    ((*this).actor.world.pos.y) + 1.5f32
                } else { ((*this).actor.world.pos.y) - 1.5f32 };
            Audio_PlayActorSound2(&mut (*this).actor,
                                  (0x28b8 as libc::c_int -
                                       0x800 as libc::c_int) as u16_0);
            return 1 as libc::c_int
        }
    }
    if (*this).unk_59C as libc::c_int >= 2 as libc::c_int {
        Audio_PlayActorSound2(&mut (*this).actor,
                              if (*this).actor.params as libc::c_int &
                                     0x1f as libc::c_int ==
                                     GORON_CITY_ROLLING_BIG as libc::c_int {
                                  0x3879 as libc::c_int
                              } else { 0x387b as libc::c_int } as u16_0);
    }
    (*this).unk_59C -= 1;
    if (*this).unk_59C as libc::c_int <= 0 as libc::c_int {
        if (*this).unk_59C as libc::c_int == 0 as libc::c_int {
            (*this).unk_590 =
                Rand_S16Offset(60 as libc::c_int as s16,
                               30 as libc::c_int as s16);
            (*this).unk_59C = 0 as libc::c_int as s16;
            (*this).actor.velocity.y = 0.0f32;
            return 1 as libc::c_int
        } else { (*this).unk_59C = arg1 }
    }
    (*this).actor.velocity.y =
        (*this).unk_59C as f32_0 / arg1 as f32_0 * arg2;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_BiggoronSetTextId(mut this: *mut EnGo2,
                                                 mut globalCtx:
                                                     *mut GlobalContext,
                                                 mut player: *mut Player) {
    let mut textId: u16_0 = 0;
    if (*this).actor.params as libc::c_int & 0x1f as libc::c_int ==
           GORON_DMT_BIGGORON as libc::c_int {
        if gSaveContext.bgsFlag != 0 {
            if func_8002F368(globalCtx) as libc::c_int ==
                   EXCH_ITEM_CLAIM_CHECK as libc::c_int {
                (*this).actor.textId = 0x3003 as libc::c_int as u16_0
            } else { (*this).actor.textId = 0x305e as libc::c_int as u16_0 }
            (*player).actor.textId = (*this).actor.textId
        } else if gSaveContext.bgsFlag == 0 &&
                      gSaveContext.inventory.items[gItemSlots[ITEM_POCKET_EGG
                                                                  as
                                                                  libc::c_int
                                                                  as usize] as
                                                       usize] as libc::c_int
                          == ITEM_CLAIM_CHECK as libc::c_int {
            if func_8002F368(globalCtx) as libc::c_int ==
                   EXCH_ITEM_CLAIM_CHECK as libc::c_int {
                if Environment_GetBgsDayCount() >= 3 as libc::c_int {
                    textId = 0x305e as libc::c_int as u16_0
                } else { textId = 0x305d as libc::c_int as u16_0 }
                (*this).actor.textId = textId
            } else {
                if Environment_GetBgsDayCount() >= 3 as libc::c_int {
                    textId = 0x3002 as libc::c_int as u16_0
                } else { textId = 0x305d as libc::c_int as u16_0 }
                (*this).actor.textId = textId
            }
            (*player).actor.textId = (*this).actor.textId
        } else if gSaveContext.inventory.items[gItemSlots[ITEM_POCKET_EGG as
                                                              libc::c_int as
                                                              usize] as usize]
                      as libc::c_int >= ITEM_PRESCRIPTION as libc::c_int &&
                      gSaveContext.inventory.items[gItemSlots[ITEM_POCKET_EGG
                                                                  as
                                                                  libc::c_int
                                                                  as usize] as
                                                       usize] as libc::c_int
                          <= ITEM_CLAIM_CHECK as libc::c_int {
            if func_8002F368(globalCtx) as libc::c_int ==
                   EXCH_ITEM_EYEDROPS as libc::c_int {
                (*this).actor.textId = 0x3059 as libc::c_int as u16_0
            } else { (*this).actor.textId = 0x3058 as libc::c_int as u16_0 }
            if (*this).actor.textId as libc::c_int == 0x3059 as libc::c_int {
                gSaveContext.timer2State = 0 as libc::c_int as s16
            }
            (*player).actor.textId = (*this).actor.textId
        } else if gSaveContext.inventory.items[gItemSlots[ITEM_POCKET_EGG as
                                                              libc::c_int as
                                                              usize] as usize]
                      as libc::c_int <= ITEM_SWORD_BROKEN as libc::c_int {
            if func_8002F368(globalCtx) as libc::c_int ==
                   EXCH_ITEM_SWORD_BROKEN as libc::c_int {
                if gSaveContext.infTable[11 as libc::c_int as usize] as
                       libc::c_int & 0x10 as libc::c_int != 0 {
                    textId = 0x3055 as libc::c_int as u16_0
                } else { textId = 0x3054 as libc::c_int as u16_0 }
                (*this).actor.textId = textId
            } else { (*this).actor.textId = 0x3053 as libc::c_int as u16_0 }
            (*player).actor.textId = (*this).actor.textId
        } else {
            (*this).actor.textId = 0x3053 as libc::c_int as u16_0;
            (*player).actor.textId = (*this).actor.textId
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_80A45288(mut this: *mut EnGo2,
                                       mut globalCtx: *mut GlobalContext) {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut linkAge: s32 = 0;
    if (*this).actionFunc !=
           Some(EnGo2_GoronFireGenericAction as
                    unsafe extern "C" fn(_: *mut EnGo2, _: *mut GlobalContext)
                        -> ()) {
        (*this).unk_194.unk_18 = (*player).actor.world.pos;
        linkAge = gSaveContext.linkAge;
        (*this).unk_194.unk_14 =
            D_80A482D8[((*this).actor.params as libc::c_int &
                            0x1f as libc::c_int) as usize][linkAge as usize];
        func_80034A14(&mut (*this).actor, &mut (*this).unk_194,
                      4 as libc::c_int as s16, (*this).unk_26E as s16);
    }
    if (*this).actionFunc !=
           Some(EnGo2_SetGetItem as
                    unsafe extern "C" fn(_: *mut EnGo2, _: *mut GlobalContext)
                        -> ()) &&
           (*this).isAwake as libc::c_int == 1 as libc::c_int {
        if func_80A44790(this, globalCtx) != 0 {
            EnGo2_BiggoronSetTextId(this, globalCtx, player);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_80A45360(mut this: *mut EnGo2,
                                       mut alpha: *mut f32_0) {
    let mut alphaTarget: f32_0 =
        if (*this).skelAnime.animation ==
               &mut gGoronAnim_004930 as *mut AnimationHeader as
                   *mut libc::c_void && (*this).skelAnime.curFrame <= 32.0f32
           {
            0.0f32
        } else { 255.0f32 };
    Math_ApproachF(alpha, alphaTarget, 0.4f32, 100.0f32);
    (*this).actor.shape.shadowAlpha = *alpha as u32_0 as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_RollForward(mut this: *mut EnGo2) {
    let mut speedXZ: f32_0 = (*this).actor.speedXZ;
    if (*this).unk_194.unk_00 as libc::c_int != 0 as libc::c_int {
        (*this).actor.speedXZ = 0.0f32
    }
    if (*this).actionFunc !=
           Some(EnGo2_ContinueRolling as
                    unsafe extern "C" fn(_: *mut EnGo2, _: *mut GlobalContext)
                        -> ()) {
        Actor_MoveForward(&mut (*this).actor);
    }
    (*this).actor.speedXZ = speedXZ;
}
#[no_mangle]
pub unsafe extern "C" fn func_80A454CC(mut this: *mut EnGo2) {
    let mut current_block_2: u64;
    match (*this).actor.params as libc::c_int & 0x1f as libc::c_int {
        0 | 6 | 7 | 10 | 12 => {
            func_80034EC0(&mut (*this).skelAnime, sAnimations.as_mut_ptr(),
                          9 as libc::c_int);
            current_block_2 = 13513818773234778473;
        }
        2 => {
            if gSaveContext.inventory.items[gItemSlots[ITEM_POCKET_EGG as
                                                           libc::c_int as
                                                           usize] as usize] as
                   libc::c_int >= ITEM_SWORD_BROKEN as libc::c_int &&
                   gSaveContext.inventory.items[gItemSlots[ITEM_POCKET_EGG as
                                                               libc::c_int as
                                                               usize] as
                                                    usize] as libc::c_int <=
                       ITEM_EYEDROPS as libc::c_int {
                func_80034EC0(&mut (*this).skelAnime,
                              sAnimations.as_mut_ptr(), 4 as libc::c_int);
                current_block_2 = 13513818773234778473;
            } else { current_block_2 = 12586913696805566328; }
        }
        _ => { current_block_2 = 12586913696805566328; }
    }
    match current_block_2 {
        12586913696805566328 => { (*this).skelAnime.playSpeed = 0.0f32 }
        _ => { }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GetTargetXZSpeed(mut this: *mut EnGo2)
 -> f32_0 {
    let mut yDist: f32_0 =
        if (*this).actor.params as libc::c_int & 0x1f as libc::c_int ==
               GORON_DMT_BIGGORON as libc::c_int {
            400.0f32
        } else { 60.0f32 };
    let mut index: s32 =
        (*this).actor.params as libc::c_int & 0x1f as libc::c_int;
    if index == GORON_CITY_LINK as libc::c_int &&
           fabsf((*this).actor.yDistToPlayer) < yDist &&
           (*this).actor.xzDistToPlayer < 400.0f32 {
        return 9.0f32
    } else {
        return if index == GORON_CITY_ROLLING_BIG as libc::c_int {
                   3.6000001f32
               } else { 6.0f32 }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_IsCameraModified(mut this: *mut EnGo2,
                                                mut globalCtx:
                                                    *mut GlobalContext)
 -> s32 {
    let mut camera: *mut Camera =
        (*globalCtx).cameraPtrs[0 as libc::c_int as usize];
    if (*this).actor.params as libc::c_int & 0x1f as libc::c_int ==
           GORON_DMT_BIGGORON as libc::c_int {
        if EnGo2_IsWakingUp(this) != 0 {
            Camera_ChangeSetting(camera,
                                 CAM_SET_DIRECTED_YAW as libc::c_int as s16);
            func_8005AD1C(camera, 4 as libc::c_int as s16);
        } else if EnGo2_IsWakingUp(this) == 0 &&
                      (*camera).setting as libc::c_int ==
                          CAM_SET_DIRECTED_YAW as libc::c_int {
            Camera_ChangeSetting(camera,
                                 CAM_SET_DUNGEON1 as libc::c_int as s16);
            func_8005ACFC(camera, 4 as libc::c_int as s16);
        }
    }
    if (*this).actor.params as libc::c_int & 0x1f as libc::c_int ==
           GORON_FIRE_GENERIC as libc::c_int ||
           (*this).actor.params as libc::c_int & 0x1f as libc::c_int ==
               GORON_CITY_ROLLING_BIG as libc::c_int ||
           (*this).actor.params as libc::c_int & 0x1f as libc::c_int ==
               GORON_CITY_STAIRWELL as libc::c_int ||
           (*this).actor.params as libc::c_int & 0x1f as libc::c_int ==
               GORON_DMT_BIGGORON as libc::c_int ||
           (*this).actor.params as libc::c_int & 0x1f as libc::c_int ==
               GORON_MARKET_BAZAAR as libc::c_int {
        return 1 as libc::c_int
    } else if gBitFlags[QUEST_MEDALLION_FIRE as libc::c_int as usize] &
                  gSaveContext.inventory.questItems == 0 &&
                  gBitFlags[1 as libc::c_int as usize] <<
                      gEquipShifts[EQUIP_TUNIC as libc::c_int as usize] as
                          libc::c_int &
                      gSaveContext.inventory.equipment as libc::c_uint != 0 {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_DefaultWakingUp(mut this: *mut EnGo2) {
    if EnGo2_IsWakingUp(this) != 0 {
        (*this).unk_26E = 2 as libc::c_int as u16_0
    } else { (*this).unk_26E = 1 as libc::c_int as u16_0 }
    if (*this).unk_194.unk_00 as libc::c_int != 0 as libc::c_int {
        (*this).unk_26E = 4 as libc::c_int as u16_0
    }
    (*this).isAwake = 1 as libc::c_int as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_WakingUp(mut this: *mut EnGo2) {
    let mut xyzDist: f32_0 =
        if (*this).actor.params as libc::c_int & 0x1f as libc::c_int ==
               GORON_DMT_BIGGORON as libc::c_int {
            800.0f32
        } else { 200.0f32 };
    let mut isTrue: s32 = 1 as libc::c_int;
    xyzDist = xyzDist * xyzDist;
    (*this).unk_26E = 1 as libc::c_int as u16_0;
    if (*this).actor.xyzDistToPlayerSq <= xyzDist ||
           (*this).unk_194.unk_00 as libc::c_int != 0 as libc::c_int {
        (*this).unk_26E = 4 as libc::c_int as u16_0
    }
    (*this).isAwake = isTrue as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_BiggoronWakingUp(mut this: *mut EnGo2) {
    if EnGo2_IsWakingUp(this) != 0 ||
           (*this).unk_194.unk_00 as libc::c_int != 0 as libc::c_int {
        (*this).unk_26E = 2 as libc::c_int as u16_0;
        (*this).isAwake = 1 as libc::c_int as u8_0
    } else {
        (*this).unk_26E = 1 as libc::c_int as u16_0;
        (*this).isAwake = 0 as libc::c_int as u8_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_SelectGoronWakingUp(mut this: *mut EnGo2) {
    let mut current_block_5: u64;
    match (*this).actor.params as libc::c_int & 0x1f as libc::c_int {
        4 => {
            (*this).isAwake = 1 as libc::c_int as u8_0;
            (*this).unk_26E =
                if EnGo2_IsWakingUp(this) != 0 {
                    2 as libc::c_int
                } else { 1 as libc::c_int } as u16_0;
            current_block_5 = 13536709405535804910;
        }
        3 => { EnGo2_WakingUp(this); current_block_5 = 13536709405535804910; }
        2 => {
            EnGo2_BiggoronWakingUp(this);
            current_block_5 = 13536709405535804910;
        }
        1 => {
            if gBitFlags[QUEST_MEDALLION_FIRE as libc::c_int as usize] &
                   gSaveContext.inventory.questItems == 0 &&
                   gBitFlags[1 as libc::c_int as usize] <<
                       gEquipShifts[EQUIP_TUNIC as libc::c_int as usize] as
                           libc::c_int &
                       gSaveContext.inventory.equipment as libc::c_uint != 0 {
                EnGo2_WakingUp(this);
                current_block_5 = 13536709405535804910;
            } else { current_block_5 = 14471661779576412141; }
        }
        _ => { current_block_5 = 14471661779576412141; }
    }
    match current_block_5 {
        14471661779576412141 => { EnGo2_DefaultWakingUp(this); }
        _ => { }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_EyeMouthTexState(mut this: *mut EnGo2) {
    match (*this).eyeMouthTexState as libc::c_int {
        1 => {
            (*this).blinkTimer = 0 as libc::c_int as s16;
            (*this).eyeTexIndex = 0 as libc::c_int as u8_0;
            (*this).mouthTexIndex = 0 as libc::c_int as u8_0
        }
        2 => {
            (*this).blinkTimer = 0 as libc::c_int as s16;
            (*this).eyeTexIndex = 1 as libc::c_int as u8_0;
            (*this).mouthTexIndex = 0 as libc::c_int as u8_0
        }
        3 => {
            // case 3 only when biggoron is given eyedrops. Biggoron smiles. (only use of second mouth texture)
            (*this).blinkTimer = 0 as libc::c_int as s16; // Speeding up
            (*this).eyeTexIndex = 0 as libc::c_int as u8_0;
            (*this).mouthTexIndex = 1 as libc::c_int as u8_0
        }
        _ => {
            if (if (*this).blinkTimer as libc::c_int == 0 as libc::c_int {
                    0 as libc::c_int
                } else {
                    (*this).blinkTimer -= 1;
                    (*this).blinkTimer as libc::c_int
                }) == 0 as libc::c_int {
                (*this).eyeTexIndex = (*this).eyeTexIndex.wrapping_add(1);
                if (*this).eyeTexIndex as libc::c_int >= 4 as libc::c_int {
                    (*this).blinkTimer =
                        Rand_S16Offset(30 as libc::c_int as s16,
                                       30 as libc::c_int as s16);
                    (*this).eyeTexIndex = 1 as libc::c_int as u8_0
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_SitDownAnimation(mut this: *mut EnGo2) {
    if (*this).skelAnime.playSpeed != 0.0f32 &&
           (*this).skelAnime.animation ==
               &mut gGoronAnim_004930 as *mut AnimationHeader as
                   *mut libc::c_void {
        if (*this).skelAnime.playSpeed > 0.0f32 &&
               (*this).skelAnime.curFrame == 14.0f32 {
            if (*this).actor.params as libc::c_int & 0x1f as libc::c_int !=
                   GORON_DMT_BIGGORON as libc::c_int {
                Audio_PlayActorSound2(&mut (*this).actor,
                                      0x38fd as libc::c_int as u16_0);
            } else {
                func_800F4524(&mut D_801333D4, 0x38fd as libc::c_int as u16_0,
                              60 as libc::c_int as s8);
            }
        }
        if (*this).skelAnime.playSpeed < 0.0f32 {
            if (*this).skelAnime.curFrame == 1.0f32 {
                Audio_PlayActorSound2(&mut (*this).actor,
                                      0x387b as libc::c_int as u16_0);
            }
            if (*this).skelAnime.curFrame == 40.0f32 {
                Audio_PlayActorSound2(&mut (*this).actor,
                                      0x38fd as libc::c_int as u16_0);
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GetDustData(mut this: *mut EnGo2,
                                           mut index2: s32) {
    let mut index1: s32 =
        if (*this).actor.params as libc::c_int & 0x1f as libc::c_int ==
               GORON_CITY_ROLLING_BIG as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    let mut dustEffectData: *mut EnGo2DustEffectData =
        &mut *(*sDustEffectData.as_mut_ptr().offset(index1 as
                                                        isize)).as_mut_ptr().offset(index2
                                                                                        as
                                                                                        isize)
            as *mut EnGo2DustEffectData;
    EnGo2_SpawnDust(this, (*dustEffectData).initialTimer,
                    (*dustEffectData).scale, (*dustEffectData).scaleStep,
                    (*dustEffectData).numDustEffects,
                    (*dustEffectData).radius, (*dustEffectData).yAccel);
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_RollingAnimation(mut this: *mut EnGo2,
                                                mut globalCtx:
                                                    *mut GlobalContext) {
    if (*this).actor.params as libc::c_int & 0x1f as libc::c_int ==
           GORON_DMT_BIGGORON as libc::c_int {
        (*this).actor.flags &=
            !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
        func_80034EC0(&mut (*this).skelAnime, sAnimations.as_mut_ptr(),
                      10 as libc::c_int);
        (*this).skelAnime.playSpeed = -0.5f32
    } else {
        func_80034EC0(&mut (*this).skelAnime, sAnimations.as_mut_ptr(),
                      1 as libc::c_int);
        (*this).skelAnime.playSpeed = -1.0f32
    }
    EnGo2_SwapInitialFrameAnimFrameCount(this);
    (*this).unk_26E = 1 as libc::c_int as u16_0;
    (*this).unk_211 = 0 as libc::c_int as u8_0;
    (*this).isAwake = 0 as libc::c_int as u8_0;
    (*this).actionFunc =
        Some(EnGo2_CurledUp as
                 unsafe extern "C" fn(_: *mut EnGo2, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_WakeUp(mut this: *mut EnGo2,
                                      mut globalCtx: *mut GlobalContext) {
    if (*this).skelAnime.playSpeed == 0.0f32 {
        if (*this).actor.params as libc::c_int & 0x1f as libc::c_int !=
               GORON_DMT_BIGGORON as libc::c_int {
            Audio_PlayActorSound2(&mut (*this).actor,
                                  0x38fc as libc::c_int as u16_0);
        } else {
            func_800F4524(&mut D_801333D4, 0x38fc as libc::c_int as u16_0,
                          60 as libc::c_int as s8);
        }
    }
    if (*this).actor.params as libc::c_int & 0x1f as libc::c_int ==
           GORON_DMT_BIGGORON as libc::c_int {
        OnePointCutscene_Init(globalCtx, 4200 as libc::c_int as s16,
                              -(99 as libc::c_int) as s16, &mut (*this).actor,
                              0 as libc::c_int as s16);
        func_80034EC0(&mut (*this).skelAnime, sAnimations.as_mut_ptr(),
                      10 as libc::c_int);
        (*this).skelAnime.playSpeed = 0.5f32
    } else {
        func_80034EC0(&mut (*this).skelAnime, sAnimations.as_mut_ptr(),
                      1 as libc::c_int);
        (*this).skelAnime.playSpeed = 1.0f32
    }
    (*this).actionFunc =
        Some(func_80A46B40 as
                 unsafe extern "C" fn(_: *mut EnGo2, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GetItemAnimation(mut this: *mut EnGo2,
                                                mut globalCtx:
                                                    *mut GlobalContext) {
    func_80034EC0(&mut (*this).skelAnime, sAnimations.as_mut_ptr(),
                  1 as libc::c_int);
    (*this).unk_211 = 1 as libc::c_int as u8_0;
    (*this).actionFunc =
        Some(func_80A46B40 as
                 unsafe extern "C" fn(_: *mut EnGo2, _: *mut GlobalContext)
                     -> ());
    (*this).skelAnime.playSpeed = 0.0f32;
    (*this).actor.speedXZ = 0.0f32;
    (*this).skelAnime.curFrame = (*this).skelAnime.endFrame;
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_SetupRolling(mut this: *mut EnGo2,
                                            mut globalCtx:
                                                *mut GlobalContext) {
    if (*this).actor.params as libc::c_int & 0x1f as libc::c_int ==
           GORON_CITY_ROLLING_BIG as libc::c_int ||
           (*this).actor.params as libc::c_int & 0x1f as libc::c_int ==
               GORON_CITY_LINK as libc::c_int {
        (*this).collider.info.bumperFlags = 1 as libc::c_int as u8_0;
        (*this).actor.speedXZ =
            if gSaveContext.infTable[17 as libc::c_int as usize] as
                   libc::c_int & 0x4000 as libc::c_int != 0 {
                6.0f32
            } else { 3.6000001f32 }
    } else { (*this).actor.speedXZ = 6.0f32 }
    (*this).actor.flags |=
        ((1 as libc::c_int) << 24 as libc::c_int) as libc::c_uint;
    (*this).animTimer = 10 as libc::c_int as s16;
    (*this).actor.shape.yOffset = 1800.0f32;
    (*this).actor.speedXZ += (*this).actor.speedXZ;
    (*this).actionFunc =
        Some(EnGo2_ContinueRolling as
                 unsafe extern "C" fn(_: *mut EnGo2, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_StopRolling(mut this: *mut EnGo2,
                                           mut globalCtx:
                                               *mut GlobalContext) {
    let mut bomb: *mut EnBom = 0 as *mut EnBom;
    if (*this).actor.params as libc::c_int & 0x1f as libc::c_int !=
           GORON_CITY_ROLLING_BIG as libc::c_int &&
           (*this).actor.params as libc::c_int & 0x1f as libc::c_int !=
               GORON_CITY_LINK as libc::c_int {
        if (*this).actor.params as libc::c_int & 0x1f as libc::c_int ==
               GORON_DMT_ROLLING_SMALL as libc::c_int {
            bomb =
                Actor_Spawn(&mut (*globalCtx).actorCtx, globalCtx,
                            ACTOR_EN_BOM as libc::c_int as s16,
                            (*this).actor.world.pos.x,
                            (*this).actor.world.pos.y,
                            (*this).actor.world.pos.z,
                            0 as libc::c_int as s16, 0 as libc::c_int as s16,
                            0 as libc::c_int as s16, 0 as libc::c_int as s16)
                    as *mut EnBom;
            if !bomb.is_null() { (*bomb).timer = 0 as libc::c_int as s16 }
        }
    } else { (*this).collider.info.bumperFlags = 0 as libc::c_int as u8_0 }
    (*this).actor.shape.rot = (*this).actor.world.rot;
    (*this).unk_59C = 0 as libc::c_int as s16;
    (*this).unk_590 = 0 as libc::c_int as s16;
    (*this).actionFunc =
        Some(EnGo2_GroundRolling as
                 unsafe extern "C" fn(_: *mut EnGo2, _: *mut GlobalContext)
                     -> ());
    (*this).actor.shape.yOffset = 0.0f32;
    (*this).actor.speedXZ = 0.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_IsFreeingGoronInFire(mut this: *mut EnGo2,
                                                    mut globalCtx:
                                                        *mut GlobalContext)
 -> s32 {
    if (*this).actor.params as libc::c_int & 0x1f as libc::c_int !=
           GORON_FIRE_GENERIC as libc::c_int {
        return 0 as libc::c_int
    }
    // shaking curled up
    (*this).actor.world.pos.x +=
        if (*globalCtx).state.frames & 1 as libc::c_int as libc::c_uint != 0 {
            1.0f32
        } else { -1.0f32 };
    if Flags_GetSwitch(globalCtx,
                       ((*this).actor.params as libc::c_int &
                            0xfc00 as libc::c_int) >> 0xa as libc::c_int) != 0
       {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_IsGoronDmtBombFlower(mut this: *mut EnGo2)
 -> s32 {
    if (*this).actor.params as libc::c_int & 0x1f as libc::c_int !=
           GORON_DMT_BOMB_FLOWER as libc::c_int ||
           (*this).unk_194.unk_00 as libc::c_int != 2 as libc::c_int {
        return 0 as libc::c_int
    }
    func_80034EC0(&mut (*this).skelAnime, sAnimations.as_mut_ptr(),
                  3 as libc::c_int);
    (*this).unk_194.unk_00 = 0 as libc::c_int as s16;
    (*this).isAwake = 0 as libc::c_int as u8_0;
    (*this).unk_26E = 1 as libc::c_int as u16_0;
    (*this).actionFunc =
        Some(EnGo2_GoronDmtBombFlowerAnimation as
                 unsafe extern "C" fn(_: *mut EnGo2, _: *mut GlobalContext)
                     -> ());
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_IsGoronRollingBig(mut this: *mut EnGo2,
                                                 mut globalCtx:
                                                     *mut GlobalContext)
 -> s32 {
    if (*this).actor.params as libc::c_int & 0x1f as libc::c_int !=
           GORON_CITY_ROLLING_BIG as libc::c_int ||
           (*this).unk_194.unk_00 as libc::c_int != 2 as libc::c_int {
        return 0 as libc::c_int
    }
    (*this).unk_194.unk_00 = 0 as libc::c_int as s16;
    EnGo2_RollingAnimation(this, globalCtx);
    (*this).actionFunc =
        Some(EnGo2_GoronRollingBigContinueRolling as
                 unsafe extern "C" fn(_: *mut EnGo2, _: *mut GlobalContext)
                     -> ());
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_IsGoronFireGeneric(mut this: *mut EnGo2)
 -> s32 {
    if (*this).actor.params as libc::c_int & 0x1f as libc::c_int !=
           GORON_FIRE_GENERIC as libc::c_int ||
           (*this).unk_194.unk_00 as libc::c_int == 0 as libc::c_int {
        return 0 as libc::c_int
    }
    (*this).actionFunc =
        Some(EnGo2_GoronFireGenericAction as
                 unsafe extern "C" fn(_: *mut EnGo2, _: *mut GlobalContext)
                     -> ());
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_IsGoronLinkReversing(mut this: *mut EnGo2)
 -> s32 {
    if (*this).actor.params as libc::c_int & 0x1f as libc::c_int !=
           GORON_CITY_LINK as libc::c_int ||
           (*this).waypoint as libc::c_int >= (*this).unk_216 as libc::c_int
           || EnGo2_IsWakingUp(this) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_IsRolling(mut this: *mut EnGo2) -> s32 {
    if (*this).unk_194.unk_00 as libc::c_int == 0 as libc::c_int ||
           (*this).actor.speedXZ < 1.0f32 {
        return 0 as libc::c_int
    }
    if EnGo2_IsRollingOnGround(this, 2 as libc::c_int as s16,
                               (20.0f64 / 3.0f32 as libc::c_double) as f32_0,
                               0 as libc::c_int as s16) != 0 {
        if (*this).unk_590 as libc::c_int >= 9 as libc::c_int &&
               (*this).unk_59C as libc::c_int == 0 as libc::c_int {
            (*this).unk_590 = 8 as libc::c_int as s16
        }
        EnGo2_GetDustData(this, 0 as libc::c_int);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GoronLinkAnimation(mut this: *mut EnGo2,
                                                  mut globalCtx:
                                                      *mut GlobalContext) {
    let mut animation: s32 = 13 as libc::c_int;
    if (*this).actor.params as libc::c_int & 0x1f as libc::c_int ==
           GORON_CITY_LINK as libc::c_int {
        if (*this).actor.textId as libc::c_int == 0x3035 as libc::c_int &&
               (*this).unk_20C as libc::c_int == 0 as libc::c_int ||
               (*this).actor.textId as libc::c_int == 0x3036 as libc::c_int &&
                   (*this).unk_20C as libc::c_int == 0 as libc::c_int {
            if (*this).skelAnime.animation !=
                   &mut gGoronAnim_000D5C as *mut AnimationHeader as
                       *mut libc::c_void {
                animation = 12 as libc::c_int;
                (*this).eyeMouthTexState = 0 as libc::c_int as u8_0
            }
        }
        if (*this).actor.textId as libc::c_int == 0x3032 as libc::c_int &&
               (*this).unk_20C as libc::c_int == 12 as libc::c_int ||
               (*this).actor.textId as libc::c_int == 0x3033 as libc::c_int ||
               (*this).actor.textId as libc::c_int == 0x3035 as libc::c_int &&
                   (*this).unk_20C as libc::c_int == 6 as libc::c_int {
            if (*this).skelAnime.animation !=
                   &mut gGoronAnim_000750 as *mut AnimationHeader as
                       *mut libc::c_void {
                animation = 11 as libc::c_int;
                (*this).eyeMouthTexState = 1 as libc::c_int as u8_0
            }
        }
        if (*this).skelAnime.animation ==
               &mut gGoronAnim_000750 as *mut AnimationHeader as
                   *mut libc::c_void {
            if (*this).skelAnime.curFrame == 20.0f32 {
                Audio_PlayActorSound2(&mut (*this).actor,
                                      0x39eb as libc::c_int as u16_0);
            }
        }
        if animation != 13 as libc::c_int {
            func_80034EC0(&mut (*this).skelAnime, sAnimations.as_mut_ptr(),
                          animation);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GoronFireCamera(mut this: *mut EnGo2,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    let mut yaw: s16 = 0;
    (*this).camId = Gameplay_CreateSubCamera(globalCtx);
    Gameplay_ChangeCameraStatus(globalCtx, 0 as libc::c_int as s16,
                                1 as libc::c_int as s16);
    Gameplay_ChangeCameraStatus(globalCtx, (*this).camId,
                                7 as libc::c_int as s16);
    Path_CopyLastPoint((*this).path, &mut (*this).at);
    yaw =
        (Math_Vec3f_Yaw(&mut (*this).actor.world.pos, &mut (*this).at) as
             libc::c_int + 0xe38 as libc::c_int) as s16;
    (*this).eye.x = Math_SinS(yaw) * 100.0f32 + (*this).actor.world.pos.x;
    (*this).eye.z = Math_CosS(yaw) * 100.0f32 + (*this).actor.world.pos.z;
    (*this).eye.y = (*this).actor.world.pos.y + 20.0f32;
    (*this).at.x = (*this).actor.world.pos.x;
    (*this).at.y = (*this).actor.world.pos.y + 40.0f32;
    (*this).at.z = (*this).actor.world.pos.z;
    Gameplay_CameraSetAtEye(globalCtx, (*this).camId, &mut (*this).at,
                            &mut (*this).eye);
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GoronFireClearCamera(mut this: *mut EnGo2,
                                                    mut globalCtx:
                                                        *mut GlobalContext) {
    Gameplay_ChangeCameraStatus(globalCtx, 0 as libc::c_int as s16,
                                7 as libc::c_int as s16);
    Gameplay_ClearCamera(globalCtx, (*this).camId);
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_BiggoronAnimation(mut this: *mut EnGo2) {
    if gSaveContext.inventory.items[gItemSlots[ITEM_POCKET_EGG as libc::c_int
                                                   as usize] as usize] as
           libc::c_int >= ITEM_SWORD_BROKEN as libc::c_int &&
           gSaveContext.inventory.items[gItemSlots[ITEM_POCKET_EGG as
                                                       libc::c_int as usize]
                                            as usize] as libc::c_int <=
               ITEM_EYEDROPS as libc::c_int &&
           (*this).actor.params as libc::c_int & 0x1f as libc::c_int ==
               GORON_DMT_BIGGORON as libc::c_int &&
           (*this).unk_194.unk_00 as libc::c_int == 0 as libc::c_int {
        if (if (*this).animTimer as libc::c_int == 0 as libc::c_int {
                0 as libc::c_int
            } else {
                (*this).animTimer -= 1;
                (*this).animTimer as libc::c_int
            }) == 0 as libc::c_int {
            (*this).animTimer =
                Rand_S16Offset(30 as libc::c_int as s16,
                               30 as libc::c_int as s16);
            func_800F4524(&mut D_801333D4, 0x391c as libc::c_int as u16_0,
                          60 as libc::c_int as s8);
        }
    };
}
/*
FLAGS

gSaveContext.eventChkInf[2] & 0x8 - DC entrance boulder blown up as child

InfTable

gSaveContext.infTable[11] & 0x10
gSaveContext.infTable[14] & 0x1 - Talked to DMT Goron at DC entrance (Before DC is opened as child)
gSaveContext.infTable[14] & 0x8 - Talked to GC Goron in bottom level stairwell
gSaveContext.infTable[14] & 0x40 - Talked to GC Goron at LW entrance (Before LW shortcut is opened)
gSaveContext.infTable[14] & 0x800 - Talked to DMT Goron at Bomb Flower with goron bracelet
gSaveContext.infTable[15] & 0x1 - Talked to Goron at GC Entrance (Before goron ruby is obtained)
gSaveContext.infTable[15] & 0x10 - Talked to Goron at GC Island (Before goron ruby is obtained)
gSaveContext.infTable[15] & 0x100 - (not on cloud modding) Talked to GC Goron outside Darunias door (after opening door,
before getting goron bracelet) gSaveContext.infTable[16] & 0x200 - Obtained Fire Tunic from Goron Link
gSaveContext.infTable[16] & 0x400 - (not on cloud modding)
gSaveContext.infTable[16] & 0x800 - Spoke to Goron Link About Volvagia
gSaveContext.infTable[16] & 0x1000 - Stopped Goron Link's Rolling
gSaveContext.infTable[16] & 0x2000 - EnGo Exclusive
gSaveContext.infTable[16] & 0x4000 - Spoke to Goron Link
gSaveContext.infTable[16] & 0x8000 - (not on cloud modding)

gSaveContext.infTable[17] & 0x4000 - Bomb bag upgrade obtained from rolling Goron

EnGo
pathIndex: this->actor.params & 0xF
Goron: this->actor.params & 0xF0

EnGo2
(this->actor.params & 0x3E0) >> 5
(this->actor.params & 0xFC00) >> 0xA - Gorons in Fire Temple
this->actor.params & 0x1F

Gorons only move when this->unk_194.unk_00 == 0
*/
#[no_mangle]
pub unsafe extern "C" fn EnGo2_Init(mut thisx: *mut Actor,
                                    mut globalCtx: *mut GlobalContext) {
    let mut this: *mut EnGo2 = thisx as *mut EnGo2;
    let mut pad: s32 = 0;
    ActorShape_Init(&mut (*this).actor.shape, 0.0f32,
                    Some(ActorShadow_DrawCircle as
                             unsafe extern "C" fn(_: *mut Actor,
                                                  _: *mut Lights,
                                                  _: *mut GlobalContext)
                                 -> ()), 28.0f32);
    SkelAnime_InitFlex(globalCtx, &mut (*this).skelAnime, &mut gGoronSkel,
                       0 as *mut AnimationHeader,
                       (*this).jointTable.as_mut_ptr(),
                       (*this).morphTable.as_mut_ptr(), 18 as libc::c_int);
    Collider_InitCylinder(globalCtx, &mut (*this).collider);
    Collider_SetCylinder(globalCtx, &mut (*this).collider, &mut (*this).actor,
                         &mut sCylinderInit);
    CollisionCheck_SetInfo2(&mut (*this).actor.colChkInfo,
                            0 as *mut DamageTable, &mut sColChkInfoInit);
    // Not GORON_CITY_ROLLING_BIG, GORON_CITY_LINK, GORON_DMT_BIGGORON
    match (*this).actor.params as libc::c_int & 0x1f as libc::c_int {
        3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 => {
            (*this).actor.flags &=
                !((1 as libc::c_int) << 4 as libc::c_int) as
                    libc::c_uint; // OC_PLAYER | OC_NO_PUSH | OC_ON
            (*this).actor.flags &=
                !((1 as libc::c_int) << 5 as libc::c_int) as libc::c_uint
        }
        _ => { }
    }
    EnGo2_SetColliderDim(this);
    EnGo2_SetShape(this);
    func_80034EC0(&mut (*this).skelAnime, sAnimations.as_mut_ptr(),
                  0 as libc::c_int);
    (*this).actor.gravity = -1.0f32;
    (*this).actor.shape.shadowAlpha = 0 as libc::c_int as u8_0;
    (*this).alpha = (*this).actor.shape.shadowAlpha as f32_0;
    (*this).reverse = 0 as libc::c_int as u8_0;
    (*this).isAwake = 0 as libc::c_int as u8_0;
    (*this).unk_211 = 0 as libc::c_int as u8_0;
    (*this).goronState = 0 as libc::c_int as u8_0;
    (*this).waypoint = 0 as libc::c_int as s8;
    (*this).unk_216 = (*this).actor.shape.rot.z as u8_0;
    (*this).unk_26E = 1 as libc::c_int as u16_0;
    (*this).path =
        Path_GetByIndex(globalCtx,
                        (((*this).actor.params as libc::c_int &
                              0x3e0 as libc::c_int) >> 5 as libc::c_int) as
                            s16, 0x1f as libc::c_int as s16);
    let mut current_block_63: u64;
    match (*this).actor.params as libc::c_int & 0x1f as libc::c_int {
        7 | 8 | 9 | 10 | 11 => {
            if gBitFlags[QUEST_MEDALLION_FIRE as libc::c_int as usize] &
                   gSaveContext.inventory.questItems == 0 &&
                   gSaveContext.linkAge == 0 as libc::c_int {
                Actor_Kill(&mut (*this).actor);
            }
            (*this).actionFunc =
                Some(EnGo2_CurledUp as
                         unsafe extern "C" fn(_: *mut EnGo2,
                                              _: *mut GlobalContext) -> ());
            current_block_63 = 10512632378975961025;
        }
        13 => {
            if gSaveContext.linkAge == 0 as libc::c_int ||
                   gBitFlags[QUEST_GORON_RUBY as libc::c_int as usize] &
                       gSaveContext.inventory.questItems == 0 {
                Actor_Kill(&mut (*this).actor);
            }
            EnGo2_GetItemAnimation(this, globalCtx);
            current_block_63 = 10512632378975961025;
        }
        1 => {
            if gSaveContext.infTable[16 as libc::c_int as usize] as
                   libc::c_int & 0x200 as libc::c_int != 0 {
                Path_CopyLastPoint((*this).path,
                                   &mut (*this).actor.world.pos);
                (*this).actor.home.pos = (*this).actor.world.pos;
                if gBitFlags[QUEST_MEDALLION_FIRE as libc::c_int as usize] &
                       gSaveContext.inventory.questItems == 0 &&
                       gBitFlags[1 as libc::c_int as usize] <<
                           gEquipShifts[EQUIP_TUNIC as libc::c_int as usize]
                               as libc::c_int &
                           gSaveContext.inventory.equipment as libc::c_uint !=
                           0 {
                    EnGo2_GetItemAnimation(this, globalCtx);
                } else {
                    (*this).actionFunc =
                        Some(EnGo2_CurledUp as
                                 unsafe extern "C" fn(_: *mut EnGo2,
                                                      _: *mut GlobalContext)
                                     -> ())
                }
            } else {
                gSaveContext.infTable[16 as libc::c_int as usize] =
                    (gSaveContext.infTable[16 as libc::c_int as usize] as
                         libc::c_int & !(0x1000 as libc::c_int)) as u16_0;
                (*this).collider.dim.height =
                    (D_80A4816C[((*this).actor.params as libc::c_int &
                                     0x1f as libc::c_int) as usize].height as
                         libc::c_int as libc::c_float * 0.6f32) as s16;
                EnGo2_SetupRolling(this, globalCtx);
                (*this).isAwake = 1 as libc::c_int as u8_0
            }
            current_block_63 = 10512632378975961025;
        }
        0 | 5 => {
            (*this).collider.dim.height =
                (D_80A4816C[((*this).actor.params as libc::c_int &
                                 0x1f as libc::c_int) as usize].height as
                     libc::c_int as libc::c_float * 0.6f32) as s16;
            EnGo2_SetupRolling(this, globalCtx);
            current_block_63 = 10512632378975961025;
        }
        3 => {
            if Flags_GetSwitch(globalCtx,
                               ((*this).actor.params as libc::c_int &
                                    0xfc00 as libc::c_int) >>
                                   0xa as libc::c_int) != 0 {
                Actor_Kill(&mut (*this).actor);
            } else {
                (*this).isAwake = 1 as libc::c_int as u8_0;
                (*this).actionFunc =
                    Some(EnGo2_CurledUp as
                             unsafe extern "C" fn(_: *mut EnGo2,
                                                  _: *mut GlobalContext)
                                 -> ())
            }
            current_block_63 = 10512632378975961025;
        }
        2 => {
            (*this).actor.shape.shadowDraw = None;
            (*this).actor.flags &=
                !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
            if gSaveContext.inventory.items[gItemSlots[ITEM_POCKET_EGG as
                                                           libc::c_int as
                                                           usize] as usize] as
                   libc::c_int >= ITEM_SWORD_BROKEN as libc::c_int &&
                   gSaveContext.inventory.items[gItemSlots[ITEM_POCKET_EGG as
                                                               libc::c_int as
                                                               usize] as
                                                    usize] as libc::c_int <=
                       ITEM_EYEDROPS as libc::c_int {
                (*this).eyeMouthTexState = 1 as libc::c_int as u8_0
            }
            (*this).collider.base.acFlags = 0 as libc::c_int as u8_0;
            (*this).collider.base.ocFlags1 = 0xd as libc::c_int as u8_0;
            (*this).actionFunc =
                Some(EnGo2_CurledUp as
                         unsafe extern "C" fn(_: *mut EnGo2,
                                              _: *mut GlobalContext) -> ());
            current_block_63 = 10512632378975961025;
        }
        4 => {
            if gSaveContext.infTable[14 as libc::c_int as usize] as
                   libc::c_int & 0x800 as libc::c_int != 0 {
                Path_CopyLastPoint((*this).path,
                                   &mut (*this).actor.world.pos);
                (*this).actor.home.pos = (*this).actor.world.pos
            }
            current_block_63 = 3331985739160374782;
        }
        6 | 12 | _ => { current_block_63 = 3331985739160374782; }
    }
    match current_block_63 {
        3331985739160374782 => {
            (*this).actionFunc =
                Some(EnGo2_CurledUp as
                         unsafe extern "C" fn(_: *mut EnGo2,
                                              _: *mut GlobalContext) -> ())
        }
        _ => { }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_Destroy(mut thisx: *mut Actor,
                                       mut globalCtx: *mut GlobalContext) {
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_CurledUp(mut this: *mut EnGo2,
                                        mut globalCtx: *mut GlobalContext) {
    let mut index: u8_0 =
        ((*this).actor.params as libc::c_int & 0x1f as libc::c_int) as u8_0;
    let mut height: s16 = 0;
    let mut quake: s32 = 0;
    if Animation_OnFrame(&mut (*this).skelAnime, (*this).skelAnime.endFrame)
           != 0 {
        if (*this).actor.params as libc::c_int & 0x1f as libc::c_int ==
               GORON_DMT_BIGGORON as libc::c_int {
            quake =
                Quake_Add((*globalCtx).cameraPtrs[(*globalCtx).activeCamera as
                                                      usize],
                          3 as libc::c_int as u32_0) as s32;
            Quake_SetSpeed(quake as s16, -(0x3cb0 as libc::c_int) as s16);
            Quake_SetQuakeValues(quake as s16, 8 as libc::c_int as s16,
                                 0 as libc::c_int as s16,
                                 0 as libc::c_int as s16,
                                 0 as libc::c_int as s16);
            Quake_SetCountdown(quake as s16, 16 as libc::c_int as s16);
        } else { EnGo2_GetDustData(this, 1 as libc::c_int); }
        (*this).skelAnime.playSpeed = 0.0f32
    }
    if (*this).skelAnime.curFrame as s32 == 0 as libc::c_int {
        (*this).collider.dim.height =
            (D_80A4816C[index as usize].height as libc::c_int as libc::c_float
                 * 0.6f32) as s16
    } else {
        height = D_80A4816C[index as usize].height;
        (*this).collider.dim.height =
            (D_80A4816C[index as usize].height as libc::c_int as libc::c_float
                 * 0.4f32 *
                 ((*this).skelAnime.curFrame / (*this).skelAnime.startFrame) +
                 height as libc::c_int as libc::c_float * 0.6f32) as s16
    }
    if EnGo2_IsFreeingGoronInFire(this, globalCtx) != 0 {
        (*this).isAwake = 0 as libc::c_int as u8_0;
        EnGo2_WakeUp(this, globalCtx);
    }
    if (*this).actor.params as libc::c_int & 0x1f as libc::c_int !=
           GORON_FIRE_GENERIC as libc::c_int && EnGo2_IsWakingUp(this) != 0 {
        EnGo2_WakeUp(this, globalCtx);
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_80A46B40(mut this: *mut EnGo2,
                                       mut globalCtx: *mut GlobalContext) {
    let mut index: u8_0 =
        ((*this).actor.params as libc::c_int & 0x1f as libc::c_int) as u8_0;
    let mut height: f32_0 = 0.;
    if (*this).unk_211 as libc::c_int == 1 as libc::c_int {
        EnGo2_BiggoronAnimation(this);
        EnGo2_GoronLinkAnimation(this, globalCtx);
        EnGo2_SelectGoronWakingUp(this);
        if EnGo2_IsGoronRollingBig(this, globalCtx) == 0 &&
               EnGo2_IsGoronFireGeneric(this) == 0 {
            if EnGo2_IsGoronDmtBombFlower(this) != 0 { return }
        } else { return }
    } else if Animation_OnFrame(&mut (*this).skelAnime,
                                (*this).skelAnime.endFrame) != 0 {
        if (*this).actor.params as libc::c_int & 0x1f as libc::c_int ==
               GORON_DMT_BIGGORON as libc::c_int {
            (*this).actor.flags |=
                ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
        }
        func_80A454CC(this);
        (*this).unk_211 = 1 as libc::c_int as u8_0;
        (*this).collider.dim.height = D_80A4816C[index as usize].height
    } else {
        height = D_80A4816C[index as usize].height as f32_0;
        (*this).collider.dim.height =
            (height * 0.4f32 *
                 ((*this).skelAnime.curFrame / (*this).skelAnime.endFrame) +
                 height * 0.6f32) as s16
    }
    if EnGo2_IsCameraModified(this, globalCtx) == 0 &&
           EnGo2_IsWakingUp(this) == 0 {
        EnGo2_RollingAnimation(this, globalCtx);
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GoronDmtBombFlowerAnimation(mut this:
                                                               *mut EnGo2,
                                                           mut globalCtx:
                                                               *mut GlobalContext) {
    let mut float1: f32_0 = (*this).skelAnime.endFrame;
    let mut float2: f32_0 =
        (*this).skelAnime.curFrame *
            (0x8000 as libc::c_int as f32_0 / float1);
    (*this).actor.speedXZ = Math_SinS(float2 as s16);
    if EnGo2_Orient(this, globalCtx) != 0 &&
           (*this).waypoint as libc::c_int == 0 as libc::c_int {
        EnGo2_GetItemAnimation(this, globalCtx);
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GoronRollingBigContinueRolling(mut this:
                                                                  *mut EnGo2,
                                                              mut globalCtx:
                                                                  *mut GlobalContext) {
    if Animation_OnFrame(&mut (*this).skelAnime, (*this).skelAnime.endFrame)
           != 0 {
        EnGo2_GetDustData(this, 1 as libc::c_int);
        (*this).skelAnime.playSpeed = 0.0f32;
        EnGo2_SetupRolling(this, globalCtx);
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_ContinueRolling(mut this: *mut EnGo2,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    let mut float1: f32_0 = 1000.0f32;
    if ((*this).actor.params as libc::c_int & 0x1f as libc::c_int !=
            GORON_DMT_ROLLING_SMALL as libc::c_int ||
            !((*this).actor.xyzDistToPlayerSq > float1 * float1)) &&
           (if (*this).animTimer as libc::c_int == 0 as libc::c_int {
                0 as libc::c_int
            } else {
                (*this).animTimer -= 1;
                (*this).animTimer as libc::c_int
            }) == 0 as libc::c_int {
        (*this).actionFunc =
            Some(EnGo2_SlowRolling as
                     unsafe extern "C" fn(_: *mut EnGo2,
                                          _: *mut GlobalContext) -> ());
        (*this).actor.speedXZ *= 0.5f32
        // slowdown
    } // eyeDrops animation timer
    EnGo2_GetDustData(this, 2 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_SlowRolling(mut this: *mut EnGo2,
                                           mut globalCtx:
                                               *mut GlobalContext) {
    let mut orientation: s32 = 0;
    let mut index: s32 = 0;
    if EnGo2_IsRolling(this) == 0 {
        if EnGo2_IsRollingOnGround(this, 4 as libc::c_int as s16, 8.0f32,
                                   1 as libc::c_int as s16) ==
               1 as libc::c_int {
            if EnGo2_IsGoronLinkReversing(this) != 0 {
                (*this).actionFunc =
                    Some(EnGo2_ReverseRolling as
                             unsafe extern "C" fn(_: *mut EnGo2,
                                                  _: *mut GlobalContext)
                                 -> ());
                return
            }
            EnGo2_GetDustData(this, 3 as libc::c_int);
        }
        orientation = EnGo2_Orient(this, globalCtx);
        index = (*this).actor.params as libc::c_int & 0x1f as libc::c_int;
        if index != GORON_CITY_LINK as libc::c_int {
            if index == GORON_DMT_ROLLING_SMALL as libc::c_int &&
                   orientation == 1 as libc::c_int &&
                   (*this).waypoint as libc::c_int == 0 as libc::c_int {
                EnGo2_StopRolling(this, globalCtx);
                return
            }
        } else if orientation == 2 as libc::c_int &&
                      (*this).waypoint as libc::c_int == 1 as libc::c_int {
            EnGo2_StopRolling(this, globalCtx);
            return
        }
        Math_ApproachF(&mut (*this).actor.speedXZ,
                       EnGo2_GetTargetXZSpeed(this), 0.4f32, 0.6f32);
        (*this).actor.shape.rot = (*this).actor.world.rot
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GroundRolling(mut this: *mut EnGo2,
                                             mut globalCtx:
                                                 *mut GlobalContext) {
    if EnGo2_IsRollingOnGround(this, 4 as libc::c_int as s16, 8.0f32,
                               0 as libc::c_int as s16) != 0 {
        EnGo2_GetDustData(this, 0 as libc::c_int);
        if (*this).unk_59C as libc::c_int == 0 as libc::c_int {
            match (*this).actor.params as libc::c_int & 0x1f as libc::c_int {
                1 => {
                    (*this).goronState = 0 as libc::c_int as u8_0;
                    (*this).actionFunc =
                        Some(EnGo2_GoronLinkStopRolling as
                                 unsafe extern "C" fn(_: *mut EnGo2,
                                                      _: *mut GlobalContext)
                                     -> ())
                }
                0 => { EnGo2_WakeUp(this, globalCtx); }
                _ => {
                    (*this).actionFunc =
                        Some(EnGo2_CurledUp as
                                 unsafe extern "C" fn(_: *mut EnGo2,
                                                      _: *mut GlobalContext)
                                     -> ())
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_ReverseRolling(mut this: *mut EnGo2,
                                              mut globalCtx:
                                                  *mut GlobalContext) {
    if EnGo2_IsRolling(this) == 0 {
        Math_ApproachF(&mut (*this).actor.speedXZ, 0.0f32, 0.6f32, 0.8f32);
        if (*this).actor.speedXZ >= 1.0f32 {
            EnGo2_GetDustData(this, 3 as libc::c_int);
        }
        if (*this).actor.speedXZ as s32 == 0 as libc::c_int {
            (*this).actor.world.rot.y =
                ((*this).actor.world.rot.y as libc::c_int ^
                     0x8000 as libc::c_int) as s16;
            (*this).actor.shape.rot.y = (*this).actor.world.rot.y;
            (*this).reverse =
                ((*this).reverse as libc::c_int ^ 1 as libc::c_int) as u8_0;
            EnGo2_UpdateWaypoint(this, globalCtx);
            EnGo2_SetupRolling(this, globalCtx);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_SetupGetItem(mut this: *mut EnGo2,
                                            mut globalCtx:
                                                *mut GlobalContext) {
    if Actor_HasParent(&mut (*this).actor, globalCtx) != 0 {
        (*this).actor.parent = 0 as *mut Actor;
        (*this).actionFunc =
            Some(EnGo2_SetGetItem as
                     unsafe extern "C" fn(_: *mut EnGo2,
                                          _: *mut GlobalContext) -> ())
    } else {
        func_8002F434(&mut (*this).actor, globalCtx, (*this).getItemId,
                      (*this).actor.xzDistToPlayer + 1.0f32,
                      fabsf((*this).actor.yDistToPlayer) + 1.0f32);
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_SetGetItem(mut this: *mut EnGo2,
                                          mut globalCtx: *mut GlobalContext) {
    if Message_GetState(&mut (*globalCtx).msgCtx) as libc::c_int ==
           TEXT_STATE_DONE as libc::c_int &&
           Message_ShouldAdvance(globalCtx) as libc::c_int != 0 {
        (*this).unk_194.unk_00 = 0 as libc::c_int as s16;
        match (*this).getItemId {
            38 => {
                Environment_ClearBgsDayCount();
                EnGo2_GetItemAnimation(this, globalCtx);
                return
            }
            44 => {
                gSaveContext.infTable[16 as libc::c_int as usize] =
                    (gSaveContext.infTable[16 as libc::c_int as usize] as
                         libc::c_int | 0x200 as libc::c_int) as u16_0;
                EnGo2_GetItemAnimation(this, globalCtx);
                return
            }
            87 => { gSaveContext.bgsFlag = 1 as libc::c_int as u8_0 }
            51 | 52 => {
                EnGo2_RollingAnimation(this, globalCtx);
                (*this).actionFunc =
                    Some(EnGo2_GoronRollingBigContinueRolling as
                             unsafe extern "C" fn(_: *mut EnGo2,
                                                  _: *mut GlobalContext)
                                 -> ());
                return
            }
            _ => { }
        }
        (*this).actionFunc =
            Some(func_80A46B40 as
                     unsafe extern "C" fn(_: *mut EnGo2,
                                          _: *mut GlobalContext) -> ())
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_BiggoronEyedrops(mut this: *mut EnGo2,
                                                mut globalCtx:
                                                    *mut GlobalContext) {
    match (*this).goronState as libc::c_int {
        0 => {
            func_80034EC0(&mut (*this).skelAnime, sAnimations.as_mut_ptr(),
                          5 as libc::c_int);
            (*this).actor.flags &=
                !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
            (*this).actor.shape.rot.y =
                ((*this).actor.shape.rot.y as libc::c_int +
                     0x5b0 as libc::c_int) as s16;
            (*this).unk_26E = 1 as libc::c_int as u16_0;
            (*this).animTimer =
                ((*this).skelAnime.endFrame + 60.0f32 + 60.0f32) as s16;
            (*this).eyeMouthTexState = 2 as libc::c_int as u8_0;
            (*this).unk_20C = 0 as libc::c_int as u8_0;
            (*this).goronState = (*this).goronState.wrapping_add(1);
            func_800F483C(0x28 as libc::c_int as u8_0,
                          5 as libc::c_int as u8_0);
            OnePointCutscene_Init(globalCtx, 4190 as libc::c_int as s16,
                                  -(99 as libc::c_int) as s16,
                                  &mut (*this).actor,
                                  0 as libc::c_int as s16);
        }
        1 => {
            if if (*this).animTimer as libc::c_int == 0 as libc::c_int {
                   0 as libc::c_int
               } else {
                   (*this).animTimer -= 1;
                   (*this).animTimer as libc::c_int
               } != 0 {
                if (*this).animTimer as libc::c_int == 60 as libc::c_int ||
                       (*this).animTimer as libc::c_int == 120 as libc::c_int
                   {
                    func_8005B1A4((*globalCtx).cameraPtrs[(*globalCtx).activeCamera
                                                              as usize]);
                    func_800F4524(&mut D_801333D4,
                                  0x28b5 as libc::c_int as u16_0,
                                  60 as libc::c_int as s8);
                }
            } else {
                func_800F4524(&mut D_801333D4, 0x391d as libc::c_int as u16_0,
                              60 as libc::c_int as s8);
                func_80034EC0(&mut (*this).skelAnime,
                              sAnimations.as_mut_ptr(), 6 as libc::c_int);
                Message_ContinueTextbox(globalCtx,
                                        0x305a as libc::c_int as u16_0);
                (*this).eyeMouthTexState = 3 as libc::c_int as u8_0;
                (*this).goronState = (*this).goronState.wrapping_add(1);
                func_800F483C(0x7f as libc::c_int as u8_0,
                              5 as libc::c_int as u8_0);
            }
        }
        2 => {
            if Animation_OnFrame(&mut (*this).skelAnime,
                                 (*this).skelAnime.endFrame) != 0 {
                (*this).eyeMouthTexState = 0 as libc::c_int as u8_0
            }
            if Message_GetState(&mut (*globalCtx).msgCtx) as libc::c_int ==
                   TEXT_STATE_CLOSING as libc::c_int {
                func_80034EC0(&mut (*this).skelAnime,
                              sAnimations.as_mut_ptr(), 1 as libc::c_int);
                (*this).actor.flags |=
                    ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
                (*this).unk_26E = 2 as libc::c_int as u16_0;
                (*this).skelAnime.playSpeed = 0.0f32;
                (*this).skelAnime.curFrame = (*this).skelAnime.endFrame;
                EnGo2_GetItem(this, globalCtx, GI_CLAIM_CHECK as libc::c_int);
                (*this).actionFunc =
                    Some(EnGo2_SetupGetItem as
                             unsafe extern "C" fn(_: *mut EnGo2,
                                                  _: *mut GlobalContext)
                                 -> ());
                (*this).goronState = 0 as libc::c_int as u8_0
            }
        }
        _ => { }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GoronLinkStopRolling(mut this: *mut EnGo2,
                                                    mut globalCtx:
                                                        *mut GlobalContext) {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    match (*this).goronState as libc::c_int {
        0 => {
            if Message_GetState(&mut (*globalCtx).msgCtx) as libc::c_int !=
                   TEXT_STATE_NONE as libc::c_int {
                return
            } else {
                Message_StartTextbox(globalCtx,
                                     0x3031 as libc::c_int as u16_0,
                                     0 as *mut Actor);
                (*player).actor.freezeTimer = 10 as libc::c_int as u16_0;
                (*this).goronState = (*this).goronState.wrapping_add(1)
            }
        }
        1 => { }
        _ => { return }
    }
    if Message_GetState(&mut (*globalCtx).msgCtx) as libc::c_int !=
           TEXT_STATE_CLOSING as libc::c_int {
        (*player).actor.freezeTimer = 10 as libc::c_int as u16_0
    } else {
        gSaveContext.infTable[16 as libc::c_int as usize] =
            (gSaveContext.infTable[16 as libc::c_int as usize] as libc::c_int
                 | 0x1000 as libc::c_int) as u16_0;
        (*this).unk_26E = 1 as libc::c_int as u16_0;
        (*this).unk_211 = 0 as libc::c_int as u8_0;
        (*this).isAwake = 0 as libc::c_int as u8_0;
        (*this).actionFunc =
            Some(EnGo2_CurledUp as
                     unsafe extern "C" fn(_: *mut EnGo2,
                                          _: *mut GlobalContext) -> ())
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_GoronFireGenericAction(mut this: *mut EnGo2,
                                                      mut globalCtx:
                                                          *mut GlobalContext) {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut D_80A4854C: Vec3s =
        {
            let mut init =
                Vec3s{x: 0 as libc::c_int as s16,
                      y: 0 as libc::c_int as s16,
                      z: 0 as libc::c_int as s16,};
            init
        };
    let mut current_block_47: u64;
    match (*this).goronState as libc::c_int {
        0 => {
            // Wake up
            if Message_GetState(&mut (*globalCtx).msgCtx) as libc::c_int ==
                   TEXT_STATE_CLOSING as libc::c_int {
                EnGo2_GoronFireCamera(this, globalCtx);
                (*globalCtx).msgCtx.msgMode =
                    MSGMODE_PAUSED as libc::c_int as u8_0;
                func_80034EC0(&mut (*this).skelAnime,
                              sAnimations.as_mut_ptr(), 2 as libc::c_int);
                (*this).waypoint = 1 as libc::c_int as s8;
                (*this).skelAnime.playSpeed = 2.0f32;
                func_80A44D84(this);
                (*this).actor.shape.rot = (*this).actor.world.rot;
                (*this).animTimer = 60 as libc::c_int as s16;
                (*this).actor.gravity = 0.0f32;
                (*this).actor.speedXZ = 2.0f32;
                (*this).unk_194.unk_08 = D_80A4854C;
                (*this).unk_194.unk_0E = D_80A4854C;
                (*this).goronState = (*this).goronState.wrapping_add(1);
                (*this).goronState = (*this).goronState.wrapping_add(1);
                (*player).actor.world.rot.y = (*this).actor.world.rot.y;
                (*player).actor.shape.rot.y = (*this).actor.world.rot.y;
                (*player).actor.world.pos.x =
                    Math_SinS((*this).actor.world.rot.y) * -30.0f32 +
                        (*this).actor.world.pos.x;
                (*player).actor.world.pos.z =
                    Math_CosS((*this).actor.world.rot.y) * -30.0f32 +
                        (*this).actor.world.pos.z;
                func_8002DF54(globalCtx, &mut (*this).actor,
                              8 as libc::c_int as u8_0);
                Audio_PlayFanfare(0x51 as libc::c_int as u16_0);
            }
            current_block_47 = 1924505913685386279;
        }
        2 => {
            // Walking away
            if if (*this).animTimer as libc::c_int == 0 as libc::c_int {
                   0 as libc::c_int
               } else {
                   (*this).animTimer -= 1;
                   (*this).animTimer as libc::c_int
               } != 0 {
                if (*this).animTimer as libc::c_int % 8 as libc::c_int == 0 {
                    Audio_PlayActorSound2(&mut (*this).actor,
                                          0x38b8 as libc::c_int as u16_0);
                }
                Actor_MoveForward(&mut (*this).actor);
            } else {
                (*this).animTimer = 0 as libc::c_int as s16;
                (*this).actor.speedXZ = 0.0f32;
                if ((*this).actor.params as libc::c_int &
                        0xfc00 as libc::c_int) >> 0xa as libc::c_int !=
                       1 as libc::c_int &&
                       ((*this).actor.params as libc::c_int &
                            0xfc00 as libc::c_int) >> 0xa as libc::c_int !=
                           2 as libc::c_int &&
                       ((*this).actor.params as libc::c_int &
                            0xfc00 as libc::c_int) >> 0xa as libc::c_int !=
                           4 as libc::c_int &&
                       ((*this).actor.params as libc::c_int &
                            0xfc00 as libc::c_int) >> 0xa as libc::c_int !=
                           5 as libc::c_int &&
                       ((*this).actor.params as libc::c_int &
                            0xfc00 as libc::c_int) >> 0xa as libc::c_int !=
                           9 as libc::c_int &&
                       ((*this).actor.params as libc::c_int &
                            0xfc00 as libc::c_int) >> 0xa as libc::c_int !=
                           11 as libc::c_int {
                    (*this).goronState = (*this).goronState.wrapping_add(1)
                }
                (*this).goronState = (*this).goronState.wrapping_add(1)
            }
            current_block_47 = 1924505913685386279;
        }
        3 => {
            // Walking away
            (*this).animTimer += 1;
            if (*this).animTimer as libc::c_int % 8 as libc::c_int == 0 &&
                   ((*this).animTimer as libc::c_int) < 10 as libc::c_int {
                Audio_PlayActorSound2(&mut (*this).actor,
                                      0x38b8 as libc::c_int as u16_0);
            }
            if (*this).animTimer as libc::c_int == 10 as libc::c_int {
                Audio_PlayActorSound2(&mut (*this).actor,
                                      0x28db as libc::c_int as u16_0);
            }
            if (*this).animTimer as libc::c_int > 44 as libc::c_int {
                Audio_PlaySoundAtPosition(globalCtx,
                                          &mut (*this).actor.world.pos,
                                          20 as libc::c_int,
                                          0x28dc as libc::c_int as u16_0);
                current_block_47 = 978202059311449935;
            } else { current_block_47 = 1924505913685386279; }
        }
        4 => { current_block_47 = 978202059311449935; }
        1 | _ => { current_block_47 = 1924505913685386279; }
    }
    match current_block_47 {
        978202059311449935 => {
            // Finalize walking away
            Message_CloseTextbox(globalCtx);
            EnGo2_GoronFireClearCamera(this, globalCtx);
            func_8002DF54(globalCtx, &mut (*this).actor,
                          7 as libc::c_int as u8_0);
            Actor_Kill(&mut (*this).actor);
        }
        _ => { }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_Update(mut thisx: *mut Actor,
                                      mut globalCtx: *mut GlobalContext) {
    let mut this: *mut EnGo2 = thisx as *mut EnGo2;
    func_80A45360(this, &mut (*this).alpha);
    EnGo2_SitDownAnimation(this);
    SkelAnime_Update(&mut (*this).skelAnime);
    EnGo2_RollForward(this);
    Actor_UpdateBgCheckInfo(globalCtx, &mut (*this).actor,
                            (*this).collider.dim.height as f32_0 * 0.5f32,
                            (*this).collider.dim.radius as f32_0 * 0.6f32,
                            0.0f32, 5 as libc::c_int);
    if (*this).unk_194.unk_00 as libc::c_int == 0 as libc::c_int {
        func_80A44AB0(this, globalCtx);
    }
    (*this).actionFunc.expect("non-null function pointer")(this, globalCtx);
    if (*this).unk_211 as libc::c_int == 1 as libc::c_int {
        func_80034F54(globalCtx, (*this).unk_226.as_mut_ptr(),
                      (*this).unk_24A.as_mut_ptr(), 18 as libc::c_int);
    }
    func_80A45288(this, globalCtx);
    EnGo2_EyeMouthTexState(this);
    EnGo2_CheckCollision(this, globalCtx);
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_DrawCurledUp(mut this: *mut EnGo2,
                                            mut globalCtx: *mut GlobalContext)
 -> s32 {
    let mut D_80A48554: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_en_go2.c\x00" as *const u8 as *const libc::c_char,
                    2881 as libc::c_int);
    func_80093D18((*globalCtx).state.gfxCtx);
    let fresh7 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g: *mut Gfx = fresh7;
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
                      b"../z_en_go2.c\x00" as *const u8 as *const libc::c_char
                          as *mut libc::c_char, 2884 as libc::c_int) as
            libc::c_uint;
    let fresh8 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_0: *mut Gfx = fresh8;
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
    (*_g_0).words.w1 = gGoronDL_00BD80.as_mut_ptr() as libc::c_uint;
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_en_go2.c\x00" as *const u8 as *const libc::c_char,
                     2889 as libc::c_int);
    Matrix_MultVec3f(&mut D_80A48554, &mut (*this).actor.focus.pos);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_DrawRolling(mut this: *mut EnGo2,
                                           mut globalCtx: *mut GlobalContext)
 -> s32 {
    let mut pad: s32 = 0;
    let mut D_80A48560: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
    let mut speedXZ: f32_0 = 0.;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_en_go2.c\x00" as *const u8 as *const libc::c_char,
                    2914 as libc::c_int);
    func_80093D18((*globalCtx).state.gfxCtx);
    speedXZ =
        if (*this).actionFunc ==
               Some(EnGo2_ReverseRolling as
                        unsafe extern "C" fn(_: *mut EnGo2,
                                             _: *mut GlobalContext) -> ()) {
            0.0f32
        } else { (*this).actor.speedXZ };
    Matrix_RotateZYX((*globalCtx).state.frames.wrapping_mul((speedXZ as s16 as
                                                                 libc::c_int *
                                                                 1400 as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_uint)
                         as s16, 0 as libc::c_int as s16,
                     (*this).actor.shape.rot.z,
                     MTXMODE_APPLY as libc::c_int as u8_0);
    let fresh9 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g: *mut Gfx = fresh9;
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
                      b"../z_en_go2.c\x00" as *const u8 as *const libc::c_char
                          as *mut libc::c_char, 2926 as libc::c_int) as
            libc::c_uint;
    let fresh10 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_0: *mut Gfx = fresh10;
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
    (*_g_0).words.w1 = gGoronDL_00C140.as_mut_ptr() as libc::c_uint;
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_en_go2.c\x00" as *const u8 as *const libc::c_char,
                     2930 as libc::c_int);
    Matrix_MultVec3f(&mut D_80A48560, &mut (*this).actor.focus.pos);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_OverrideLimbDraw(mut globalCtx:
                                                    *mut GlobalContext,
                                                mut limb: s32,
                                                mut dList: *mut *mut Gfx,
                                                mut pos: *mut Vec3f,
                                                mut rot: *mut Vec3s,
                                                mut thisx: *mut libc::c_void)
 -> s32 {
    let mut this: *mut EnGo2 = thisx as *mut EnGo2;
    let mut vec1: Vec3s = Vec3s{x: 0, y: 0, z: 0,};
    let mut float1: f32_0 = 0.;
    if limb == 17 as libc::c_int {
        Matrix_Translate(2800.0f32, 0.0f32, 0.0f32,
                         MTXMODE_APPLY as libc::c_int as u8_0);
        vec1 = (*this).unk_194.unk_08;
        float1 =
            vec1.y as libc::c_int as libc::c_float /
                0x8000 as libc::c_int as f32_0 * 3.14159265358979323846f32;
        Matrix_RotateX(float1, MTXMODE_APPLY as libc::c_int as u8_0);
        float1 =
            vec1.x as libc::c_int as libc::c_float /
                0x8000 as libc::c_int as f32_0 * 3.14159265358979323846f32;
        Matrix_RotateZ(float1, MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_Translate(-2800.0f32, 0.0f32, 0.0f32,
                         MTXMODE_APPLY as libc::c_int as u8_0);
    }
    if limb == 10 as libc::c_int {
        vec1 = (*this).unk_194.unk_0E;
        float1 =
            vec1.y as libc::c_int as libc::c_float /
                0x8000 as libc::c_int as f32_0 * 3.14159265358979323846f32;
        Matrix_RotateY(float1, MTXMODE_APPLY as libc::c_int as u8_0);
        float1 =
            vec1.x as libc::c_int as libc::c_float /
                0x8000 as libc::c_int as f32_0 * 3.14159265358979323846f32;
        Matrix_RotateX(float1, MTXMODE_APPLY as libc::c_int as u8_0);
    }
    if limb == 10 as libc::c_int || limb == 11 as libc::c_int ||
           limb == 14 as libc::c_int {
        float1 = Math_SinS((*this).unk_226[limb as usize]);
        (*rot).y = ((*rot).y as libc::c_float + float1 * 200.0f32) as s16;
        float1 = Math_CosS((*this).unk_24A[limb as usize]);
        (*rot).z = ((*rot).z as libc::c_float + float1 * 200.0f32) as s16
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_PostLimbDraw(mut globalCtx: *mut GlobalContext,
                                            mut limbIndex: s32,
                                            mut dList: *mut *mut Gfx,
                                            mut rot: *mut Vec3s,
                                            mut thisx: *mut libc::c_void) {
    let mut this: *mut EnGo2 = thisx as *mut EnGo2;
    let mut D_80A4856C: Vec3f =
        { let mut init = Vec3f{x: 600.0f32, y: 0.0f32, z: 0.0f32,}; init };
    if limbIndex == 17 as libc::c_int {
        Matrix_MultVec3f(&mut D_80A4856C, &mut (*this).actor.focus.pos);
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnGo2_Draw(mut thisx: *mut Actor,
                                    mut globalCtx: *mut GlobalContext) {
    let mut this: *mut EnGo2 = thisx as *mut EnGo2;
    let mut eyeTextures: [*mut libc::c_void; 4] =
        [gGoronCsEyeClosed2Tex.as_mut_ptr() as *mut libc::c_void,
         gGoronCsEyeOpenTex.as_mut_ptr() as *mut libc::c_void,
         gGoronCsEyeHalfTex.as_mut_ptr() as *mut libc::c_void,
         gGoronCsEyeClosedTex.as_mut_ptr() as *mut libc::c_void];
    let mut mouthTextures: [*mut libc::c_void; 2] =
        [gGoronCsMouthNeutralTex.as_mut_ptr() as *mut libc::c_void,
         gGoronCsMouthSmileTex.as_mut_ptr() as *mut libc::c_void];
    EnGo2_UpdateDust(this);
    Matrix_Push();
    EnGo2_DrawDust(this, globalCtx);
    Matrix_Pop();
    if (*this).actionFunc ==
           Some(EnGo2_CurledUp as
                    unsafe extern "C" fn(_: *mut EnGo2, _: *mut GlobalContext)
                        -> ()) && (*this).skelAnime.playSpeed == 0.0f32 &&
           (*this).skelAnime.curFrame == 0.0f32 {
        EnGo2_DrawCurledUp(this, globalCtx);
    } else if (*this).actionFunc ==
                  Some(EnGo2_SlowRolling as
                           unsafe extern "C" fn(_: *mut EnGo2,
                                                _: *mut GlobalContext) -> ())
                  ||
                  (*this).actionFunc ==
                      Some(EnGo2_ReverseRolling as
                               unsafe extern "C" fn(_: *mut EnGo2,
                                                    _: *mut GlobalContext)
                                   -> ()) ||
                  (*this).actionFunc ==
                      Some(EnGo2_ContinueRolling as
                               unsafe extern "C" fn(_: *mut EnGo2,
                                                    _: *mut GlobalContext)
                                   -> ()) {
        EnGo2_DrawRolling(this, globalCtx);
    } else {
        let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
        let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
        __gfxCtx = (*globalCtx).state.gfxCtx;
        Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                        b"../z_en_go2.c\x00" as *const u8 as
                            *const libc::c_char, 3063 as libc::c_int);
        func_80093D18((*globalCtx).state.gfxCtx);
        let fresh11 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g: *mut Gfx = fresh11;
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
            gSegments[((eyeTextures[(*this).eyeTexIndex as usize] as u32_0) <<
                           4 as libc::c_int >> 28 as libc::c_int) as
                          usize].wrapping_add(eyeTextures[(*this).eyeTexIndex
                                                              as usize] as
                                                  u32_0 &
                                                  0xffffff as libc::c_int as
                                                      libc::c_uint).wrapping_add(0x80000000
                                                                                     as
                                                                                     libc::c_uint)
                as *mut libc::c_void as libc::c_uint;
        let fresh12 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_0: *mut Gfx = fresh12;
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
            gSegments[((mouthTextures[(*this).mouthTexIndex as usize] as
                            u32_0) << 4 as libc::c_int >> 28 as libc::c_int)
                          as
                          usize].wrapping_add(mouthTextures[(*this).mouthTexIndex
                                                                as usize] as
                                                  u32_0 &
                                                  0xffffff as libc::c_int as
                                                      libc::c_uint).wrapping_add(0x80000000
                                                                                     as
                                                                                     libc::c_uint)
                as *mut libc::c_void as libc::c_uint;
        SkelAnime_DrawFlexOpa(globalCtx, (*this).skelAnime.skeleton,
                              (*this).skelAnime.jointTable,
                              (*this).skelAnime.dListCount as s32,
                              Some(EnGo2_OverrideLimbDraw as
                                       unsafe extern "C" fn(_:
                                                                *mut GlobalContext,
                                                            _: s32,
                                                            _: *mut *mut Gfx,
                                                            _: *mut Vec3f,
                                                            _: *mut Vec3s,
                                                            _:
                                                                *mut libc::c_void)
                                           -> s32),
                              Some(EnGo2_PostLimbDraw as
                                       unsafe extern "C" fn(_:
                                                                *mut GlobalContext,
                                                            _: s32,
                                                            _: *mut *mut Gfx,
                                                            _: *mut Vec3s,
                                                            _:
                                                                *mut libc::c_void)
                                           -> ()), this as *mut libc::c_void);
        Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                         b"../z_en_go2.c\x00" as *const u8 as
                             *const libc::c_char, 3081 as libc::c_int);
    };
}
