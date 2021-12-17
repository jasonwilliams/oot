#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn TitleCard_InitPlaceName(globalCtx: *mut GlobalContext,
                               titleCtx: *mut TitleCardContext,
                               texture: *mut libc::c_void, x: s32, y: s32,
                               width: s32, height: s32, delay: s32);
    #[no_mangle]
    fn Flags_GetEventChkInf(flag: s32) -> s32;
    #[no_mangle]
    fn Flags_SetEventChkInf(flag: s32);
    #[no_mangle]
    fn Camera_SetParam(camera: *mut Camera, param: s32,
                       value: *mut libc::c_void) -> s32;
    #[no_mangle]
    fn Camera_ResetAnim(camera: *mut Camera) -> s32;
    #[no_mangle]
    fn Camera_SetCSParams(camera: *mut Camera,
                          atPoints: *mut CutsceneCameraPoint,
                          eyePoints: *mut CutsceneCameraPoint,
                          player: *mut Player, relativeToPlayer: s16) -> s32;
    #[no_mangle]
    fn func_8005B1A4(camera: *mut Camera) -> s16;
    #[no_mangle]
    fn MemCopy(dest: *mut libc::c_void, src: *mut libc::c_void, size: s32)
     -> *mut libc::c_void;
    #[no_mangle]
    fn Flags_SetEnv(globalCtx: *mut GlobalContext, flag: s16);
    #[no_mangle]
    fn Flags_UnsetEnv(globalCtx: *mut GlobalContext, flag: s16);
    #[no_mangle]
    fn Environment_LerpWeight(max: u16_0, min: u16_0, val: u16_0) -> f32_0;
    #[no_mangle]
    fn Environment_AddLightningBolts(globalCtx: *mut GlobalContext,
                                     num: u8_0);
    #[no_mangle]
    fn Math_StepToF(pValue: *mut f32_0, target: f32_0, step: f32_0) -> s32;
    #[no_mangle]
    fn func_80078884(sfxId: u16_0);
    #[no_mangle]
    fn func_800788CC(sfxId: u16_0);
    #[no_mangle]
    fn Interface_ChangeAlpha(alphaType: u16_0);
    #[no_mangle]
    fn Item_Give(globalCtx: *mut GlobalContext, item: u8_0) -> u8_0;
    #[no_mangle]
    fn Player_InCsMode(globalCtx: *mut GlobalContext) -> s32;
    #[no_mangle]
    fn Player_SetEquipmentData(globalCtx: *mut GlobalContext,
                               player: *mut Player);
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
    fn Quake_RemoveFromIdx(idx: s16) -> u32_0;
    #[no_mangle]
    fn func_800AA000(_: f32_0, _: u8_0, _: u8_0, _: u8_0);
    #[no_mangle]
    fn ShrinkWindow_SetVal(value: s32);
    #[no_mangle]
    fn ShrinkWindow_SetCurrentVal(nowVal: s32);
    #[no_mangle]
    fn func_800BC490(globalCtx: *mut GlobalContext, point: s16);
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
    fn Gameplay_CameraSetFov(globalCtx: *mut GlobalContext, camId: s16,
                             fov: f32_0) -> s32;
    #[no_mangle]
    fn Gameplay_CopyCamera(globalCtx: *mut GlobalContext, camId1: s16,
                           camId2: s16);
    #[no_mangle]
    fn Gameplay_CameraChangeSetting(globalCtx: *mut GlobalContext, camId: s16,
                                    arg2: s16) -> s32;
    #[no_mangle]
    fn Gameplay_TriggerVoidOut(globalCtx: *mut GlobalContext);
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
    fn func_800EE824();
    #[no_mangle]
    fn func_800F595C(_: u16_0);
    #[no_mangle]
    fn func_800F59E8(_: u16_0);
    #[no_mangle]
    fn Audio_SetCutsceneFlag(flag: s8);
    #[no_mangle]
    fn func_800F6D58(_: u8_0, _: u8_0, _: u8_0);
    #[no_mangle]
    fn Audio_SetSoundBanksMute(muteMask: u16_0);
    #[no_mangle]
    fn Audio_PlaySoundGeneral(sfxId: u16_0, pos: *mut Vec3f, token: u8_0,
                              freqScale: *mut f32_0, a4: *mut f32_0,
                              reverbAdd: *mut s8);
    #[no_mangle]
    fn Audio_QueueSeqCmd(bgmID: u32_0);
    #[no_mangle]
    fn GfxPrint_SetColor(this: *mut GfxPrint, r: u32_0, g: u32_0, b: u32_0,
                         a: u32_0);
    #[no_mangle]
    fn GfxPrint_SetPos(this: *mut GfxPrint, x: s32, y: s32);
    #[no_mangle]
    fn GfxPrint_Init(this: *mut GfxPrint);
    #[no_mangle]
    fn GfxPrint_Destroy(this: *mut GfxPrint);
    #[no_mangle]
    fn GfxPrint_Open(this: *mut GfxPrint, dList: *mut Gfx);
    #[no_mangle]
    fn GfxPrint_Close(this: *mut GfxPrint) -> *mut Gfx;
    #[no_mangle]
    fn GfxPrint_Printf(this: *mut GfxPrint, fmt: *const libc::c_char, _: ...)
     -> s32;
    #[no_mangle]
    static mut gSaveContext: SaveContext;
    #[no_mangle]
    static mut D_801333E8: s8;
    #[no_mangle]
    static mut D_801333E0: f32_0;
    #[no_mangle]
    static mut D_801333D4: Vec3f;
    #[no_mangle]
    fn func_8010BD58(globalCtx: *mut GlobalContext, arg1: u16_0);
    #[no_mangle]
    fn Message_ShouldAdvance(globalCtx: *mut GlobalContext) -> u8_0;
    #[no_mangle]
    fn Message_ContinueTextbox(globalCtx: *mut GlobalContext, textId: u16_0);
    #[no_mangle]
    fn Message_GetState(msgCtx: *mut MessageContext) -> u8_0;
    #[no_mangle]
    fn Message_StartTextbox(globalCtx: *mut GlobalContext, textId: u16_0,
                            actor: *mut Actor);
    #[no_mangle]
    static mut gBitFlags: [u32_0; 32];
    #[no_mangle]
    static mut gSegments: [u32_0; 16];
    #[no_mangle]
    static mut gTimeIncrement: u16_0;
    #[no_mangle]
    static mut D_801614B0: Color_RGBA8_u32;
    #[no_mangle]
    static mut gWeatherMode: u8_0;
    #[no_mangle]
    static mut gLightningStrike: LightningStrike;
    #[no_mangle]
    static mut D_8012D1F0: *mut libc::c_void;
    #[no_mangle]
    static mut gGameInfo: *mut GameInfo;
    #[no_mangle]
    static mut gDbgCamEnabled: s32;
    #[no_mangle]
    static mut gEntranceTable: [EntranceInfo; 1556];
    #[no_mangle]
    static mut gTempleOfTimeIntroCs: [CutsceneData; 0];
    #[no_mangle]
    static mut gHyruleFieldGetOoTCs: [CutsceneData; 0];
    #[no_mangle]
    static mut gHyruleFieldSouthEponaJumpCs: [CutsceneData; 0];
    #[no_mangle]
    static mut gHyruleFieldEastEponaJumpCs: [CutsceneData; 0];
    #[no_mangle]
    static mut gHyruleFieldWestEponaJumpCs: [CutsceneData; 0];
    #[no_mangle]
    static mut gHyruleFieldGateEponaJumpCs: [CutsceneData; 0];
    #[no_mangle]
    static mut gHyruleFieldIntroCs: [CutsceneData; 0];
    #[no_mangle]
    static mut gKakarikoVillageIntroCs: [CutsceneData; 0];
    #[no_mangle]
    static mut gGraveyardIntroCs: [CutsceneData; 0];
    #[no_mangle]
    static mut gKokiriForestDekuSproutCs: [CutsceneData; 0];
    #[no_mangle]
    static mut gMinuetCs: [CutsceneData; 0];
    #[no_mangle]
    static mut gLakeHyliaIntroCs: [CutsceneData; 0];
    #[no_mangle]
    static mut gZorasDomainIntroCs: [CutsceneData; 0];
    #[no_mangle]
    static mut gZorasFountainIntroCs: [CutsceneData; 0];
    #[no_mangle]
    static mut gGerudoValleyIntroCs: [CutsceneData; 0];
    #[no_mangle]
    static mut gDesertColossusIntroCs: [CutsceneData; 0];
    #[no_mangle]
    static mut gGerudoFortressFirstCaptureCs: [CutsceneData; 0];
    #[no_mangle]
    static mut gGerudoFortressIntroCs: [CutsceneData; 0];
    #[no_mangle]
    static mut gHyruleCastleIntroCs: [CutsceneData; 0];
    #[no_mangle]
    static mut gDMTIntroCs: [CutsceneData; 0];
    #[no_mangle]
    static mut gDeathMountainCraterIntroCs: [CutsceneData; 0];
    #[no_mangle]
    static mut gGoronCityIntroCs: [CutsceneData; 0];
    #[no_mangle]
    static mut gLonLonRanchIntroCs: [CutsceneData; 0];
    #[no_mangle]
    static mut gJabuJabuIntroCs: [CutsceneData; 0];
    #[no_mangle]
    static mut gDcOpeningCs: [CutsceneData; 0];
    #[no_mangle]
    static mut gDekuTreeIntroCs: [CutsceneData; 0];
    #[no_mangle]
    static mut gTowerBarrierCs: [CutsceneData; 0];
    #[no_mangle]
    static mut gLightBarrierCs: [CutsceneData; 0];
    #[no_mangle]
    static mut gFireBarrierCs: [CutsceneData; 0];
    #[no_mangle]
    static mut gForestBarrierCs: [CutsceneData; 0];
    #[no_mangle]
    static mut gSpiritBarrierCs: [CutsceneData; 0];
    #[no_mangle]
    static mut gWaterBarrierCs: [CutsceneData; 0];
    #[no_mangle]
    static mut gShadowBarrierCs: [CutsceneData; 0];
    #[no_mangle]
    static mut gGanonsCastleIntroCs: [CutsceneData; 0];
    #[no_mangle]
    static mut gSpiritBossNabooruKnuckleIntroCs: [CutsceneData; 0];
    #[no_mangle]
    static mut gIceCavernSerenadeCs: [CutsceneData; 0];
    #[no_mangle]
    static mut gSunSongGraveSunSongTeachPart2Cs: [CutsceneData; 0];
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
pub type PrintCallback
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *const libc::c_char,
                                _: u32_0) -> *mut libc::c_void>;
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
pub type C2RustUnnamed_3 = libc::c_uint;
pub const SUNSSONG_SPECIAL: C2RustUnnamed_3 = 3;
pub const SUNSSONG_SPEED_TIME: C2RustUnnamed_3 = 2;
pub const SUNSSONG_START: C2RustUnnamed_3 = 1;
pub const SUNSSONG_INACTIVE: C2RustUnnamed_3 = 0;
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
    pub c2rust_unnamed: C2RustUnnamed_4,
    pub rgba: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
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
    pub c2rust_unnamed: C2RustUnnamed_5,
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
pub union C2RustUnnamed_5 {
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
    pub c2rust_unnamed: C2RustUnnamed_6,
    pub normal: Vec3s,
    pub dist: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub vtxData: [u16_0; 3],
    pub c2rust_unnamed: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
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
    pub sides: [C2RustUnnamed_8; 2],
    pub id: s16,
    pub pos: Vec3s,
    pub rotY: s16,
    pub params: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
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
    pub c2rust_unnamed: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub single: C2RustUnnamed_11,
    pub multi: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
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
pub struct C2RustUnnamed_11 {
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
    pub restrictions: C2RustUnnamed_12,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
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
    pub c2rust_unnamed: C2RustUnnamed_13,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_13 {
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
    pub c2rust_unnamed: C2RustUnnamed_14,
    pub startPos: Vec3i,
    pub endPos: Vec3i,
    pub normal: Vec3i,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_14 {
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
    pub flags: C2RustUnnamed_15,
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
pub struct C2RustUnnamed_15 {
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
pub type C2RustUnnamed_16 = libc::c_uint;
pub const ACTORCAT_CHEST: C2RustUnnamed_16 = 11;
pub const ACTORCAT_DOOR: C2RustUnnamed_16 = 10;
pub const ACTORCAT_BOSS: C2RustUnnamed_16 = 9;
pub const ACTORCAT_MISC: C2RustUnnamed_16 = 8;
pub const ACTORCAT_ITEMACTION: C2RustUnnamed_16 = 7;
pub const ACTORCAT_PROP: C2RustUnnamed_16 = 6;
pub const ACTORCAT_ENEMY: C2RustUnnamed_16 = 5;
pub const ACTORCAT_NPC: C2RustUnnamed_16 = 4;
pub const ACTORCAT_EXPLOSIVE: C2RustUnnamed_16 = 3;
pub const ACTORCAT_PLAYER: C2RustUnnamed_16 = 2;
pub const ACTORCAT_BG: C2RustUnnamed_16 = 1;
pub const ACTORCAT_SWITCH: C2RustUnnamed_16 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EntranceCutscene {
    pub entrance: u16_0,
    pub ageRestriction: u8_0,
    pub flag: u8_0,
    pub segAddr: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CsCmdBase {
    pub base: u16_0,
    pub startFrame: u16_0,
    pub endFrame: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CsCmdEnvLighting {
    pub unk_00: u8_0,
    pub setting: u8_0,
    pub startFrame: u16_0,
    pub endFrame: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CsCmdMusicChange {
    pub unk_00: u8_0,
    pub sequence: u8_0,
    pub startFrame: u16_0,
    pub endFrame: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CsCmdMusicFade {
    pub type_0: u16_0,
    pub startFrame: u16_0,
    pub endFrame: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CsCmdUnknown9 {
    pub unk_00: u16_0,
    pub startFrame: u16_0,
    pub endFrame: u16_0,
    pub unk_06: u8_0,
    pub unk_07: u8_0,
    pub unk_08: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CsCmdDayTime {
    pub unk_00: u16_0,
    pub startFrame: u16_0,
    pub endFrame: u16_0,
    pub hour: u8_0,
    pub minute: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CsCmdTextbox {
    pub base: u16_0,
    pub startFrame: u16_0,
    pub endFrame: u16_0,
    pub type_0: u16_0,
    pub textId1: u16_0,
    pub textId2: u16_0,
}
pub type C2RustUnnamed_17 = libc::c_uint;
pub const CS_STATE_UNSKIPPABLE_EXEC: C2RustUnnamed_17 = 4;
pub const CS_STATE_UNSKIPPABLE_INIT: C2RustUnnamed_17 = 3;
pub const CS_STATE_SKIPPABLE_EXEC: C2RustUnnamed_17 = 2;
pub const CS_STATE_SKIPPABLE_INIT: C2RustUnnamed_17 = 1;
pub const CS_STATE_IDLE: C2RustUnnamed_17 = 0;
pub type C2RustUnnamed_18 = libc::c_uint;
pub const CS_CMD_END: C2RustUnnamed_18 = 65535;
pub const CS_CMD_TERMINATOR: C2RustUnnamed_18 = 1000;
pub const CS_CMD_SETTIME: C2RustUnnamed_18 = 140;
pub const CS_CMD_FADEBGM: C2RustUnnamed_18 = 124;
pub const CS_CMD_STOPBGM: C2RustUnnamed_18 = 87;
pub const CS_CMD_PLAYBGM: C2RustUnnamed_18 = 86;
pub const CS_CMD_NOP: C2RustUnnamed_18 = 11;
pub const CS_CMD_SCENE_TRANS_FX: C2RustUnnamed_18 = 45;
pub const CS_CMD_SET_ACTOR_ACTION_10: C2RustUnnamed_18 = 143;
pub const CS_CMD_SET_ACTOR_ACTION_9: C2RustUnnamed_18 = 62;
pub const CS_CMD_SET_ACTOR_ACTION_8: C2RustUnnamed_18 = 49;
pub const CS_CMD_SET_ACTOR_ACTION_7: C2RustUnnamed_18 = 31;
pub const CS_CMD_SET_ACTOR_ACTION_6: C2RustUnnamed_18 = 44;
pub const CS_CMD_SET_ACTOR_ACTION_5: C2RustUnnamed_18 = 30;
pub const CS_CMD_SET_ACTOR_ACTION_4: C2RustUnnamed_18 = 29;
pub const CS_CMD_SET_ACTOR_ACTION_3: C2RustUnnamed_18 = 25;
pub const CS_CMD_SET_ACTOR_ACTION_2: C2RustUnnamed_18 = 14;
pub const CS_CMD_SET_ACTOR_ACTION_1: C2RustUnnamed_18 = 15;
pub const CS_CMD_SET_PLAYER_ACTION: C2RustUnnamed_18 = 10;
pub const CS_CMD_TEXTBOX: C2RustUnnamed_18 = 19;
pub const CS_CMD_09: C2RustUnnamed_18 = 9;
pub const CS_CMD_08: C2RustUnnamed_18 = 8;
pub const CS_CMD_07: C2RustUnnamed_18 = 7;
pub const CS_CMD_CAM_AT_REL_TO_PLAYER: C2RustUnnamed_18 = 6;
pub const CS_CMD_CAM_EYE_REL_TO_PLAYER: C2RustUnnamed_18 = 5;
pub const CS_CMD_SET_LIGHTING: C2RustUnnamed_18 = 4;
pub const CS_CMD_MISC: C2RustUnnamed_18 = 3;
pub const CS_CMD_CAM_AT: C2RustUnnamed_18 = 2;
pub const CS_CMD_CAM_EYE: C2RustUnnamed_18 = 1;
pub const CS_CMD_00: C2RustUnnamed_18 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union CutsceneData {
    pub i: s32,
    pub f: f32_0,
    pub s: [s16; 2],
    pub b: [s8; 4],
}
pub type C2RustUnnamed_19 = libc::c_uint;
pub const CAM_SET_MAX: C2RustUnnamed_19 = 66;
pub const CAM_SET_NORMAL4: C2RustUnnamed_19 = 65;
pub const CAM_SET_PIVOT_FROM_SIDE: C2RustUnnamed_19 = 64;
pub const CAM_SET_DIRECTED_YAW: C2RustUnnamed_19 = 63;
pub const CAM_SET_DUNGEON2: C2RustUnnamed_19 = 62;
pub const CAM_SET_JABU_TENTACLE: C2RustUnnamed_19 = 61;
pub const CAM_SET_CS_C: C2RustUnnamed_19 = 60;
pub const CAM_SET_FISHING: C2RustUnnamed_19 = 59;
pub const CAM_SET_NORMAL2: C2RustUnnamed_19 = 58;
pub const CAM_SET_PIVOT_VERTICAL: C2RustUnnamed_19 = 57;
pub const CAM_SET_TURN_AROUND: C2RustUnnamed_19 = 56;
pub const CAM_SET_FIRE_BIRDS_EYE: C2RustUnnamed_19 = 55;
pub const CAM_SET_MEADOW_UNUSED: C2RustUnnamed_19 = 54;
pub const CAM_SET_MEADOW_BIRDS_EYE: C2RustUnnamed_19 = 53;
pub const CAM_SET_BIG_OCTO: C2RustUnnamed_19 = 52;
pub const CAM_SET_FOREST_DEFEAT_POE: C2RustUnnamed_19 = 51;
pub const CAM_SET_FOREST_UNUSED: C2RustUnnamed_19 = 50;
pub const CAM_SET_FIRE_STAIRCASE: C2RustUnnamed_19 = 49;
pub const CAM_SET_FIRE_PLATFORM: C2RustUnnamed_19 = 48;
pub const CAM_SET_SCENE_TRANSITION: C2RustUnnamed_19 = 47;
pub const CAM_SET_SCENE_UNUSED: C2RustUnnamed_19 = 46;
pub const CAM_SET_BEAN_LOST_WOODS: C2RustUnnamed_19 = 45;
pub const CAM_SET_BEAN_GENERIC: C2RustUnnamed_19 = 44;
pub const CAM_SET_CS_ATTENTION: C2RustUnnamed_19 = 43;
pub const CAM_SET_CS_3: C2RustUnnamed_19 = 42;
pub const CAM_SET_ITEM_UNUSED: C2RustUnnamed_19 = 41;
pub const CAM_SET_SLOW_CHEST_CS: C2RustUnnamed_19 = 40;
pub const CAM_SET_FOREST_BIRDS_EYE: C2RustUnnamed_19 = 39;
pub const CAM_SET_CS_TWISTED_HALLWAY: C2RustUnnamed_19 = 38;
pub const CAM_SET_CS_0: C2RustUnnamed_19 = 37;
pub const CAM_SET_PIVOT_WATER_SURFACE: C2RustUnnamed_19 = 36;
pub const CAM_SET_PIVOT_CORNER: C2RustUnnamed_19 = 35;
pub const CAM_SET_FREE2: C2RustUnnamed_19 = 34;
pub const CAM_SET_FREE0: C2RustUnnamed_19 = 33;
pub const CAM_SET_START1: C2RustUnnamed_19 = 32;
pub const CAM_SET_START0: C2RustUnnamed_19 = 31;
pub const CAM_SET_CRAWLSPACE: C2RustUnnamed_19 = 30;
pub const CAM_SET_DOORC: C2RustUnnamed_19 = 29;
pub const CAM_SET_DOOR0: C2RustUnnamed_19 = 28;
pub const CAM_SET_PREREND_SIDE_SCROLL: C2RustUnnamed_19 = 27;
pub const CAM_SET_PREREND_PIVET: C2RustUnnamed_19 = 26;
pub const CAM_SET_PREREND_FIXED: C2RustUnnamed_19 = 25;
pub const CAM_SET_PIVOT_IN_FRONT: C2RustUnnamed_19 = 24;
pub const CAM_SET_PIVOT_SHOP_BROWSING: C2RustUnnamed_19 = 23;
pub const CAM_SET_PIVOT_CRAWLSPACE: C2RustUnnamed_19 = 22;
pub const CAM_SET_CHU_BOWLING: C2RustUnnamed_19 = 21;
pub const CAM_SET_MARKET_BALCONY: C2RustUnnamed_19 = 20;
pub const CAM_SET_TOWER_UNUSED: C2RustUnnamed_19 = 19;
pub const CAM_SET_TOWER_CLIMB: C2RustUnnamed_19 = 18;
pub const CAM_SET_BOSS_GANON: C2RustUnnamed_19 = 17;
pub const CAM_SET_BOSS_GANONDORF: C2RustUnnamed_19 = 16;
pub const CAM_SET_BOSS_TWINROVA_FLOOR: C2RustUnnamed_19 = 15;
pub const CAM_SET_BOSS_TWINROVA_PLATFORM: C2RustUnnamed_19 = 14;
pub const CAM_SET_BOSS_MORPHA: C2RustUnnamed_19 = 13;
pub const CAM_SET_BOSS_BONGO: C2RustUnnamed_19 = 12;
pub const CAM_SET_BOSS_VOLVAGIA: C2RustUnnamed_19 = 11;
pub const CAM_SET_BOSS_PHANTOM_GANON: C2RustUnnamed_19 = 10;
pub const CAM_SET_BOSS_BARINADE: C2RustUnnamed_19 = 9;
pub const CAM_SET_BOSS_DODONGO: C2RustUnnamed_19 = 8;
pub const CAM_SET_BOSS_GOHMA: C2RustUnnamed_19 = 7;
pub const CAM_SET_HORSE: C2RustUnnamed_19 = 6;
pub const CAM_SET_NORMAL3: C2RustUnnamed_19 = 5;
pub const CAM_SET_DUNGEON1: C2RustUnnamed_19 = 4;
pub const CAM_SET_DUNGEON0: C2RustUnnamed_19 = 3;
pub const CAM_SET_NORMAL1: C2RustUnnamed_19 = 2;
pub const CAM_SET_NORMAL0: C2RustUnnamed_19 = 1;
pub const CAM_SET_NONE: C2RustUnnamed_19 = 0;
pub type C2RustUnnamed_20 = libc::c_uint;
pub const LIGHTNING_STRIKE_END: C2RustUnnamed_20 = 2;
pub const LIGHTNING_STRIKE_START: C2RustUnnamed_20 = 1;
pub const LIGHTNING_STRIKE_WAIT: C2RustUnnamed_20 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LightningStrike {
    pub state: u8_0,
    pub flashRed: u8_0,
    pub flashGreen: u8_0,
    pub flashBlue: u8_0,
    pub flashAlphaTarget: u8_0,
    pub delayTimer: f32_0,
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
pub const QUEST_HEART_PIECE: C2RustUnnamed_22 = 24;
pub const QUEST_SKULL_TOKEN: C2RustUnnamed_22 = 23;
pub const QUEST_GERUDO_CARD: C2RustUnnamed_22 = 22;
pub const QUEST_STONE_OF_AGONY: C2RustUnnamed_22 = 21;
pub const QUEST_ZORA_SAPPHIRE: C2RustUnnamed_22 = 20;
pub const QUEST_GORON_RUBY: C2RustUnnamed_22 = 19;
pub const QUEST_KOKIRI_EMERALD: C2RustUnnamed_22 = 18;
pub const QUEST_SONG_STORMS: C2RustUnnamed_22 = 17;
pub const QUEST_SONG_TIME: C2RustUnnamed_22 = 16;
pub const QUEST_SONG_SUN: C2RustUnnamed_22 = 15;
pub const QUEST_SONG_SARIA: C2RustUnnamed_22 = 14;
pub const QUEST_SONG_EPONA: C2RustUnnamed_22 = 13;
pub const QUEST_SONG_LULLABY: C2RustUnnamed_22 = 12;
pub const QUEST_SONG_PRELUDE: C2RustUnnamed_22 = 11;
pub const QUEST_SONG_NOCTURNE: C2RustUnnamed_22 = 10;
pub const QUEST_SONG_REQUIEM: C2RustUnnamed_22 = 9;
pub const QUEST_SONG_SERENADE: C2RustUnnamed_22 = 8;
pub const QUEST_SONG_BOLERO: C2RustUnnamed_22 = 7;
pub const QUEST_SONG_MINUET: C2RustUnnamed_22 = 6;
pub const QUEST_MEDALLION_LIGHT: C2RustUnnamed_22 = 5;
pub const QUEST_MEDALLION_SHADOW: C2RustUnnamed_22 = 4;
pub const QUEST_MEDALLION_SPIRIT: C2RustUnnamed_22 = 3;
pub const QUEST_MEDALLION_WATER: C2RustUnnamed_22 = 2;
pub const QUEST_MEDALLION_FIRE: C2RustUnnamed_22 = 1;
pub const QUEST_MEDALLION_FOREST: C2RustUnnamed_22 = 0;
pub type C2RustUnnamed_23 = libc::c_uint;
pub const ITEM_NONE: C2RustUnnamed_23 = 255;
pub const ITEM_NONE_FE: C2RustUnnamed_23 = 254;
pub const ITEM_LAST_USED: C2RustUnnamed_23 = 252;
pub const ITEM_NUT_UPGRADE_40: C2RustUnnamed_23 = 155;
pub const ITEM_NUT_UPGRADE_30: C2RustUnnamed_23 = 154;
pub const ITEM_STICK_UPGRADE_30: C2RustUnnamed_23 = 153;
pub const ITEM_STICK_UPGRADE_20: C2RustUnnamed_23 = 152;
pub const ITEM_BOMBCHUS_20: C2RustUnnamed_23 = 151;
pub const ITEM_BOMBCHUS_5: C2RustUnnamed_23 = 150;
pub const ITEM_SEEDS_30: C2RustUnnamed_23 = 149;
pub const ITEM_ARROWS_LARGE: C2RustUnnamed_23 = 148;
pub const ITEM_ARROWS_MEDIUM: C2RustUnnamed_23 = 147;
pub const ITEM_ARROWS_SMALL: C2RustUnnamed_23 = 146;
pub const ITEM_BOMBS_30: C2RustUnnamed_23 = 145;
pub const ITEM_BOMBS_20: C2RustUnnamed_23 = 144;
pub const ITEM_BOMBS_10: C2RustUnnamed_23 = 143;
pub const ITEM_BOMBS_5: C2RustUnnamed_23 = 142;
pub const ITEM_NUTS_10: C2RustUnnamed_23 = 141;
pub const ITEM_NUTS_5: C2RustUnnamed_23 = 140;
pub const ITEM_STICKS_10: C2RustUnnamed_23 = 139;
pub const ITEM_STICKS_5: C2RustUnnamed_23 = 138;
pub const ITEM_INVALID_8: C2RustUnnamed_23 = 137;
pub const ITEM_RUPEE_GOLD: C2RustUnnamed_23 = 136;
pub const ITEM_RUPEE_PURPLE: C2RustUnnamed_23 = 135;
pub const ITEM_RUPEE_RED: C2RustUnnamed_23 = 134;
pub const ITEM_RUPEE_BLUE: C2RustUnnamed_23 = 133;
pub const ITEM_RUPEE_GREEN: C2RustUnnamed_23 = 132;
pub const ITEM_HEART: C2RustUnnamed_23 = 131;
pub const ITEM_MILK: C2RustUnnamed_23 = 130;
pub const ITEM_INVALID_7: C2RustUnnamed_23 = 129;
pub const ITEM_INVALID_6: C2RustUnnamed_23 = 128;
pub const ITEM_INVALID_5: C2RustUnnamed_23 = 127;
pub const ITEM_INVALID_4: C2RustUnnamed_23 = 126;
pub const ITEM_INVALID_3: C2RustUnnamed_23 = 125;
pub const ITEM_INVALID_2: C2RustUnnamed_23 = 124;
pub const ITEM_INVALID_1: C2RustUnnamed_23 = 123;
pub const ITEM_HEART_PIECE_2: C2RustUnnamed_23 = 122;
pub const ITEM_MAGIC_LARGE: C2RustUnnamed_23 = 121;
pub const ITEM_MAGIC_SMALL: C2RustUnnamed_23 = 120;
pub const ITEM_KEY_SMALL: C2RustUnnamed_23 = 119;
pub const ITEM_DUNGEON_MAP: C2RustUnnamed_23 = 118;
pub const ITEM_COMPASS: C2RustUnnamed_23 = 117;
pub const ITEM_KEY_BOSS: C2RustUnnamed_23 = 116;
pub const ITEM_HEART_PIECE: C2RustUnnamed_23 = 115;
pub const ITEM_HEART_CONTAINER: C2RustUnnamed_23 = 114;
pub const ITEM_SKULL_TOKEN: C2RustUnnamed_23 = 113;
pub const ITEM_GERUDO_CARD: C2RustUnnamed_23 = 112;
pub const ITEM_STONE_OF_AGONY: C2RustUnnamed_23 = 111;
pub const ITEM_ZORA_SAPPHIRE: C2RustUnnamed_23 = 110;
pub const ITEM_GORON_RUBY: C2RustUnnamed_23 = 109;
pub const ITEM_KOKIRI_EMERALD: C2RustUnnamed_23 = 108;
pub const ITEM_MEDALLION_LIGHT: C2RustUnnamed_23 = 107;
pub const ITEM_MEDALLION_SHADOW: C2RustUnnamed_23 = 106;
pub const ITEM_MEDALLION_SPIRIT: C2RustUnnamed_23 = 105;
pub const ITEM_MEDALLION_WATER: C2RustUnnamed_23 = 104;
pub const ITEM_MEDALLION_FIRE: C2RustUnnamed_23 = 103;
pub const ITEM_MEDALLION_FOREST: C2RustUnnamed_23 = 102;
pub const ITEM_SONG_STORMS: C2RustUnnamed_23 = 101;
pub const ITEM_SONG_TIME: C2RustUnnamed_23 = 100;
pub const ITEM_SONG_SUN: C2RustUnnamed_23 = 99;
pub const ITEM_SONG_SARIA: C2RustUnnamed_23 = 98;
pub const ITEM_SONG_EPONA: C2RustUnnamed_23 = 97;
pub const ITEM_SONG_LULLABY: C2RustUnnamed_23 = 96;
pub const ITEM_SONG_PRELUDE: C2RustUnnamed_23 = 95;
pub const ITEM_SONG_NOCTURNE: C2RustUnnamed_23 = 94;
pub const ITEM_SONG_REQUIEM: C2RustUnnamed_23 = 93;
pub const ITEM_SONG_SERENADE: C2RustUnnamed_23 = 92;
pub const ITEM_SONG_BOLERO: C2RustUnnamed_23 = 91;
pub const ITEM_SONG_MINUET: C2RustUnnamed_23 = 90;
pub const ITEM_FISHING_POLE: C2RustUnnamed_23 = 89;
pub const ITEM_SEEDS: C2RustUnnamed_23 = 88;
pub const ITEM_WALLET_GIANT: C2RustUnnamed_23 = 87;
pub const ITEM_WALLET_ADULT: C2RustUnnamed_23 = 86;
pub const ITEM_SWORD_KNIFE: C2RustUnnamed_23 = 85;
pub const ITEM_SCALE_GOLDEN: C2RustUnnamed_23 = 84;
pub const ITEM_SCALE_SILVER: C2RustUnnamed_23 = 83;
pub const ITEM_GAUNTLETS_GOLD: C2RustUnnamed_23 = 82;
pub const ITEM_GAUNTLETS_SILVER: C2RustUnnamed_23 = 81;
pub const ITEM_BRACELET: C2RustUnnamed_23 = 80;
pub const ITEM_BOMB_BAG_40: C2RustUnnamed_23 = 79;
pub const ITEM_BOMB_BAG_30: C2RustUnnamed_23 = 78;
pub const ITEM_BOMB_BAG_20: C2RustUnnamed_23 = 77;
pub const ITEM_QUIVER_50: C2RustUnnamed_23 = 76;
pub const ITEM_QUIVER_40: C2RustUnnamed_23 = 75;
pub const ITEM_QUIVER_30: C2RustUnnamed_23 = 74;
pub const ITEM_BULLET_BAG_50: C2RustUnnamed_23 = 73;
pub const ITEM_BULLET_BAG_40: C2RustUnnamed_23 = 72;
pub const ITEM_BULLET_BAG_30: C2RustUnnamed_23 = 71;
pub const ITEM_BOOTS_HOVER: C2RustUnnamed_23 = 70;
pub const ITEM_BOOTS_IRON: C2RustUnnamed_23 = 69;
pub const ITEM_BOOTS_KOKIRI: C2RustUnnamed_23 = 68;
pub const ITEM_TUNIC_ZORA: C2RustUnnamed_23 = 67;
pub const ITEM_TUNIC_GORON: C2RustUnnamed_23 = 66;
pub const ITEM_TUNIC_KOKIRI: C2RustUnnamed_23 = 65;
pub const ITEM_SHIELD_MIRROR: C2RustUnnamed_23 = 64;
pub const ITEM_SHIELD_HYLIAN: C2RustUnnamed_23 = 63;
pub const ITEM_SHIELD_DEKU: C2RustUnnamed_23 = 62;
pub const ITEM_SWORD_BGS: C2RustUnnamed_23 = 61;
pub const ITEM_SWORD_MASTER: C2RustUnnamed_23 = 60;
pub const ITEM_SWORD_KOKIRI: C2RustUnnamed_23 = 59;
pub const ITEM_BOW_ARROW_LIGHT: C2RustUnnamed_23 = 58;
pub const ITEM_BOW_ARROW_ICE: C2RustUnnamed_23 = 57;
pub const ITEM_BOW_ARROW_FIRE: C2RustUnnamed_23 = 56;
pub const ITEM_CLAIM_CHECK: C2RustUnnamed_23 = 55;
pub const ITEM_EYEDROPS: C2RustUnnamed_23 = 54;
pub const ITEM_FROG: C2RustUnnamed_23 = 53;
pub const ITEM_PRESCRIPTION: C2RustUnnamed_23 = 52;
pub const ITEM_SWORD_BROKEN: C2RustUnnamed_23 = 51;
pub const ITEM_SAW: C2RustUnnamed_23 = 50;
pub const ITEM_ODD_POTION: C2RustUnnamed_23 = 49;
pub const ITEM_ODD_MUSHROOM: C2RustUnnamed_23 = 48;
pub const ITEM_COJIRO: C2RustUnnamed_23 = 47;
pub const ITEM_POCKET_CUCCO: C2RustUnnamed_23 = 46;
pub const ITEM_POCKET_EGG: C2RustUnnamed_23 = 45;
pub const ITEM_SOLD_OUT: C2RustUnnamed_23 = 44;
pub const ITEM_MASK_TRUTH: C2RustUnnamed_23 = 43;
pub const ITEM_MASK_GERUDO: C2RustUnnamed_23 = 42;
pub const ITEM_MASK_ZORA: C2RustUnnamed_23 = 41;
pub const ITEM_MASK_GORON: C2RustUnnamed_23 = 40;
pub const ITEM_MASK_BUNNY: C2RustUnnamed_23 = 39;
pub const ITEM_MASK_SPOOKY: C2RustUnnamed_23 = 38;
pub const ITEM_MASK_SKULL: C2RustUnnamed_23 = 37;
pub const ITEM_MASK_KEATON: C2RustUnnamed_23 = 36;
pub const ITEM_LETTER_ZELDA: C2RustUnnamed_23 = 35;
pub const ITEM_CHICKEN: C2RustUnnamed_23 = 34;
pub const ITEM_WEIRD_EGG: C2RustUnnamed_23 = 33;
pub const ITEM_POE: C2RustUnnamed_23 = 32;
pub const ITEM_MILK_HALF: C2RustUnnamed_23 = 31;
pub const ITEM_BIG_POE: C2RustUnnamed_23 = 30;
pub const ITEM_BUG: C2RustUnnamed_23 = 29;
pub const ITEM_BLUE_FIRE: C2RustUnnamed_23 = 28;
pub const ITEM_LETTER_RUTO: C2RustUnnamed_23 = 27;
pub const ITEM_MILK_BOTTLE: C2RustUnnamed_23 = 26;
pub const ITEM_FISH: C2RustUnnamed_23 = 25;
pub const ITEM_FAIRY: C2RustUnnamed_23 = 24;
pub const ITEM_POTION_BLUE: C2RustUnnamed_23 = 23;
pub const ITEM_POTION_GREEN: C2RustUnnamed_23 = 22;
pub const ITEM_POTION_RED: C2RustUnnamed_23 = 21;
pub const ITEM_BOTTLE: C2RustUnnamed_23 = 20;
pub const ITEM_NAYRUS_LOVE: C2RustUnnamed_23 = 19;
pub const ITEM_ARROW_LIGHT: C2RustUnnamed_23 = 18;
pub const ITEM_HAMMER: C2RustUnnamed_23 = 17;
pub const ITEM_BEAN: C2RustUnnamed_23 = 16;
pub const ITEM_LENS: C2RustUnnamed_23 = 15;
pub const ITEM_BOOMERANG: C2RustUnnamed_23 = 14;
pub const ITEM_FARORES_WIND: C2RustUnnamed_23 = 13;
pub const ITEM_ARROW_ICE: C2RustUnnamed_23 = 12;
pub const ITEM_LONGSHOT: C2RustUnnamed_23 = 11;
pub const ITEM_HOOKSHOT: C2RustUnnamed_23 = 10;
pub const ITEM_BOMBCHU: C2RustUnnamed_23 = 9;
pub const ITEM_OCARINA_TIME: C2RustUnnamed_23 = 8;
pub const ITEM_OCARINA_FAIRY: C2RustUnnamed_23 = 7;
pub const ITEM_SLINGSHOT: C2RustUnnamed_23 = 6;
pub const ITEM_DINS_FIRE: C2RustUnnamed_23 = 5;
pub const ITEM_ARROW_FIRE: C2RustUnnamed_23 = 4;
pub const ITEM_BOW: C2RustUnnamed_23 = 3;
pub const ITEM_BOMB: C2RustUnnamed_23 = 2;
pub const ITEM_NUT: C2RustUnnamed_23 = 1;
pub const ITEM_STICK: C2RustUnnamed_23 = 0;
pub type C2RustUnnamed_24 = libc::c_uint;
pub const SEQ_PLAYER_BGM_SUB: C2RustUnnamed_24 = 3;
pub const SEQ_PLAYER_SFX: C2RustUnnamed_24 = 2;
pub const SEQ_PLAYER_FANFARE: C2RustUnnamed_24 = 1;
pub const SEQ_PLAYER_BGM_MAIN: C2RustUnnamed_24 = 0;
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
pub struct EntranceInfo {
    pub scene: s8,
    pub spawn: s8,
    pub field: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GfxPrint {
    pub callback: PrintCallback,
    pub dList: *mut Gfx,
    pub posX: u16_0,
    pub posY: u16_0,
    pub baseX: u16_0,
    pub baseY: u8_0,
    pub flags: u8_0,
    pub color: Color_RGBA8_u32,
    pub unk_14: [libc::c_char; 28],
}
pub type CutsceneStateHandler
    =
    Option<unsafe extern "C" fn(_: *mut GlobalContext,
                                _: *mut CutsceneContext) -> ()>;
#[no_mangle]
pub static mut D_8011E1C0: u16_0 = 0 as libc::c_int as u16_0;
#[no_mangle]
pub static mut D_8011E1C4: u16_0 = 0 as libc::c_int as u16_0;
#[no_mangle]
pub static mut sCsStateHandlers1: [CutsceneStateHandler; 5] =
    unsafe {
        [Some(func_80064720 as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut CutsceneContext) -> ()),
         Some(func_80064760 as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut CutsceneContext) -> ()),
         Some(func_80064720 as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut CutsceneContext) -> ()),
         Some(func_80068D84 as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut CutsceneContext) -> ()),
         Some(func_80064720 as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut CutsceneContext) -> ())]
    };
#[no_mangle]
pub static mut sCsStateHandlers2: [CutsceneStateHandler; 5] =
    unsafe {
        [Some(func_80064720 as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut CutsceneContext) -> ()),
         Some(func_800647C0 as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut CutsceneContext) -> ()),
         Some(func_80068C3C as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut CutsceneContext) -> ()),
         Some(func_80068DC0 as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut CutsceneContext) -> ()),
         Some(func_80068C3C as
                  unsafe extern "C" fn(_: *mut GlobalContext,
                                       _: *mut CutsceneContext) -> ())]
    };
#[no_mangle]
pub static mut sTitleCsState: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut sEntranceCutsceneTable: [EntranceCutscene; 34] =
    unsafe {
        [{
             let mut init =
                 EntranceCutscene{entrance: 0x185 as libc::c_int as u16_0,
                                  ageRestriction: 2 as libc::c_int as u8_0,
                                  flag: 0xa0 as libc::c_int as u8_0,
                                  segAddr:
                                      gHyruleFieldIntroCs.as_ptr() as *mut _
                                          as *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 EntranceCutscene{entrance: 0x13d as libc::c_int as u16_0,
                                  ageRestriction: 2 as libc::c_int as u8_0,
                                  flag: 0xa1 as libc::c_int as u8_0,
                                  segAddr:
                                      gDMTIntroCs.as_ptr() as *mut _ as
                                          *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 EntranceCutscene{entrance: 0xdb as libc::c_int as u16_0,
                                  ageRestriction: 2 as libc::c_int as u8_0,
                                  flag: 0xa3 as libc::c_int as u8_0,
                                  segAddr:
                                      gKakarikoVillageIntroCs.as_ptr() as
                                          *mut _ as *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 EntranceCutscene{entrance: 0x108 as libc::c_int as u16_0,
                                  ageRestriction: 2 as libc::c_int as u8_0,
                                  flag: 0xa4 as libc::c_int as u8_0,
                                  segAddr:
                                      gZorasDomainIntroCs.as_ptr() as *mut _
                                          as *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 EntranceCutscene{entrance: 0x138 as libc::c_int as u16_0,
                                  ageRestriction: 1 as libc::c_int as u8_0,
                                  flag: 0xa5 as libc::c_int as u8_0,
                                  segAddr:
                                      gHyruleCastleIntroCs.as_ptr() as *mut _
                                          as *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 EntranceCutscene{entrance: 0x14d as libc::c_int as u16_0,
                                  ageRestriction: 2 as libc::c_int as u8_0,
                                  flag: 0xa6 as libc::c_int as u8_0,
                                  segAddr:
                                      gGoronCityIntroCs.as_ptr() as *mut _ as
                                          *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 EntranceCutscene{entrance: 0x53 as libc::c_int as u16_0,
                                  ageRestriction: 2 as libc::c_int as u8_0,
                                  flag: 0xa7 as libc::c_int as u8_0,
                                  segAddr:
                                      gTempleOfTimeIntroCs.as_ptr() as *mut _
                                          as *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 EntranceCutscene{entrance: 0 as libc::c_int as u16_0,
                                  ageRestriction: 2 as libc::c_int as u8_0,
                                  flag: 0xa8 as libc::c_int as u8_0,
                                  segAddr:
                                      gDekuTreeIntroCs.as_ptr() as *mut _ as
                                          *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 EntranceCutscene{entrance: 0x28a as libc::c_int as u16_0,
                                  ageRestriction: 0 as libc::c_int as u8_0,
                                  flag: 0x18 as libc::c_int as u8_0,
                                  segAddr:
                                      gHyruleFieldSouthEponaJumpCs.as_ptr() as
                                          *mut _ as *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 EntranceCutscene{entrance: 0x292 as libc::c_int as u16_0,
                                  ageRestriction: 0 as libc::c_int as u8_0,
                                  flag: 0x18 as libc::c_int as u8_0,
                                  segAddr:
                                      gHyruleFieldEastEponaJumpCs.as_ptr() as
                                          *mut _ as *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 EntranceCutscene{entrance: 0x28e as libc::c_int as u16_0,
                                  ageRestriction: 0 as libc::c_int as u8_0,
                                  flag: 0x18 as libc::c_int as u8_0,
                                  segAddr:
                                      gHyruleFieldWestEponaJumpCs.as_ptr() as
                                          *mut _ as *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 EntranceCutscene{entrance: 0x476 as libc::c_int as u16_0,
                                  ageRestriction: 0 as libc::c_int as u8_0,
                                  flag: 0x18 as libc::c_int as u8_0,
                                  segAddr:
                                      gHyruleFieldGateEponaJumpCs.as_ptr() as
                                          *mut _ as *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 EntranceCutscene{entrance: 0x50f as libc::c_int as u16_0,
                                  ageRestriction: 1 as libc::c_int as u8_0,
                                  flag: 0xa9 as libc::c_int as u8_0,
                                  segAddr:
                                      gHyruleFieldGetOoTCs.as_ptr() as *mut _
                                          as *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 EntranceCutscene{entrance: 0x102 as libc::c_int as u16_0,
                                  ageRestriction: 2 as libc::c_int as u8_0,
                                  flag: 0xb1 as libc::c_int as u8_0,
                                  segAddr:
                                      gLakeHyliaIntroCs.as_ptr() as *mut _ as
                                          *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 EntranceCutscene{entrance: 0x117 as libc::c_int as u16_0,
                                  ageRestriction: 2 as libc::c_int as u8_0,
                                  flag: 0xb2 as libc::c_int as u8_0,
                                  segAddr:
                                      gGerudoValleyIntroCs.as_ptr() as *mut _
                                          as *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 EntranceCutscene{entrance: 0x129 as libc::c_int as u16_0,
                                  ageRestriction: 2 as libc::c_int as u8_0,
                                  flag: 0xb3 as libc::c_int as u8_0,
                                  segAddr:
                                      gGerudoFortressIntroCs.as_ptr() as
                                          *mut _ as *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 EntranceCutscene{entrance: 0x157 as libc::c_int as u16_0,
                                  ageRestriction: 2 as libc::c_int as u8_0,
                                  flag: 0xb4 as libc::c_int as u8_0,
                                  segAddr:
                                      gLonLonRanchIntroCs.as_ptr() as *mut _
                                          as *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 EntranceCutscene{entrance: 0x28 as libc::c_int as u16_0,
                                  ageRestriction: 2 as libc::c_int as u8_0,
                                  flag: 0xb5 as libc::c_int as u8_0,
                                  segAddr:
                                      gJabuJabuIntroCs.as_ptr() as *mut _ as
                                          *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 EntranceCutscene{entrance: 0xe4 as libc::c_int as u16_0,
                                  ageRestriction: 2 as libc::c_int as u8_0,
                                  flag: 0xb6 as libc::c_int as u8_0,
                                  segAddr:
                                      gGraveyardIntroCs.as_ptr() as *mut _ as
                                          *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 EntranceCutscene{entrance: 0x225 as libc::c_int as u16_0,
                                  ageRestriction: 2 as libc::c_int as u8_0,
                                  flag: 0xb7 as libc::c_int as u8_0,
                                  segAddr:
                                      gZorasFountainIntroCs.as_ptr() as *mut _
                                          as *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 EntranceCutscene{entrance: 0x123 as libc::c_int as u16_0,
                                  ageRestriction: 2 as libc::c_int as u8_0,
                                  flag: 0xb8 as libc::c_int as u8_0,
                                  segAddr:
                                      gDesertColossusIntroCs.as_ptr() as
                                          *mut _ as *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 EntranceCutscene{entrance: 0x147 as libc::c_int as u16_0,
                                  ageRestriction: 2 as libc::c_int as u8_0,
                                  flag: 0xb9 as libc::c_int as u8_0,
                                  segAddr:
                                      gDeathMountainCraterIntroCs.as_ptr() as
                                          *mut _ as *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 EntranceCutscene{entrance: 0x138 as libc::c_int as u16_0,
                                  ageRestriction: 0 as libc::c_int as u8_0,
                                  flag: 0xba as libc::c_int as u8_0,
                                  segAddr:
                                      gGanonsCastleIntroCs.as_ptr() as *mut _
                                          as *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 EntranceCutscene{entrance: 0x574 as libc::c_int as u16_0,
                                  ageRestriction: 2 as libc::c_int as u8_0,
                                  flag: 0x5a as libc::c_int as u8_0,
                                  segAddr:
                                      gSunSongGraveSunSongTeachPart2Cs.as_ptr()
                                          as *mut _ as *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 EntranceCutscene{entrance: 0x538 as libc::c_int as u16_0,
                                  ageRestriction: 2 as libc::c_int as u8_0,
                                  flag: 0xbb as libc::c_int as u8_0,
                                  segAddr:
                                      gForestBarrierCs.as_ptr() as *mut _ as
                                          *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 EntranceCutscene{entrance: 0x53c as libc::c_int as u16_0,
                                  ageRestriction: 2 as libc::c_int as u8_0,
                                  flag: 0xbc as libc::c_int as u8_0,
                                  segAddr:
                                      gWaterBarrierCs.as_ptr() as *mut _ as
                                          *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 EntranceCutscene{entrance: 0x540 as libc::c_int as u16_0,
                                  ageRestriction: 2 as libc::c_int as u8_0,
                                  flag: 0xbd as libc::c_int as u8_0,
                                  segAddr:
                                      gShadowBarrierCs.as_ptr() as *mut _ as
                                          *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 EntranceCutscene{entrance: 0x544 as libc::c_int as u16_0,
                                  ageRestriction: 2 as libc::c_int as u8_0,
                                  flag: 0xbe as libc::c_int as u8_0,
                                  segAddr:
                                      gFireBarrierCs.as_ptr() as *mut _ as
                                          *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 EntranceCutscene{entrance: 0x548 as libc::c_int as u16_0,
                                  ageRestriction: 2 as libc::c_int as u8_0,
                                  flag: 0xbf as libc::c_int as u8_0,
                                  segAddr:
                                      gLightBarrierCs.as_ptr() as *mut _ as
                                          *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 EntranceCutscene{entrance: 0x54c as libc::c_int as u16_0,
                                  ageRestriction: 2 as libc::c_int as u8_0,
                                  flag: 0xad as libc::c_int as u8_0,
                                  segAddr:
                                      gSpiritBarrierCs.as_ptr() as *mut _ as
                                          *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 EntranceCutscene{entrance: 0x8d as libc::c_int as u16_0,
                                  ageRestriction: 0 as libc::c_int as u8_0,
                                  flag: 0xc0 as libc::c_int as u8_0,
                                  segAddr:
                                      gSpiritBossNabooruKnuckleIntroCs.as_ptr()
                                          as *mut _ as *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 EntranceCutscene{entrance: 0x3b4 as libc::c_int as u16_0,
                                  ageRestriction: 0 as libc::c_int as u8_0,
                                  flag: 0xc7 as libc::c_int as u8_0,
                                  segAddr:
                                      gGerudoFortressFirstCaptureCs.as_ptr()
                                          as *mut _ as *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 EntranceCutscene{entrance: 0x246 as libc::c_int as u16_0,
                                  ageRestriction: 2 as libc::c_int as u8_0,
                                  flag: 0xb9 as libc::c_int as u8_0,
                                  segAddr:
                                      gDeathMountainCraterIntroCs.as_ptr() as
                                          *mut _ as *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 EntranceCutscene{entrance: 0x5e8 as libc::c_int as u16_0,
                                  ageRestriction: 2 as libc::c_int as u8_0,
                                  flag: 0xc6 as libc::c_int as u8_0,
                                  segAddr:
                                      gKokiriForestDekuSproutCs.as_ptr() as
                                          *mut _ as *mut libc::c_void,};
             init
         }]
    };
// Unused, seems to be an early list of dungeon entrance cutscene locations
#[no_mangle]
pub static mut D_8011E304: [*mut libc::c_void; 6] =
    unsafe {
        [gDekuTreeIntroCs.as_ptr() as *mut _ as *mut libc::c_void,
         gJabuJabuIntroCs.as_ptr() as *mut _ as *mut libc::c_void,
         gDcOpeningCs.as_ptr() as *mut _ as *mut libc::c_void,
         gMinuetCs.as_ptr() as *mut _ as *mut libc::c_void,
         gIceCavernSerenadeCs.as_ptr() as *mut _ as *mut libc::c_void,
         gTowerBarrierCs.as_ptr() as *mut _ as *mut libc::c_void]
    };
#[no_mangle]
pub static mut D_8015FCC0: u16_0 = 0;
#[no_mangle]
pub static mut D_8015FCC2: u16_0 = 0;
#[no_mangle]
pub static mut D_8015FCC4: u16_0 = 0;
#[no_mangle]
pub static mut D_8015FCC6: s16 = 0;
#[no_mangle]
pub static mut D_8015FCC8: u8_0 = 0;
#[no_mangle]
pub static mut sQuakeIndex: s16 = 0;
#[no_mangle]
pub static mut D_8015FCCC: u16_0 = 0;
// only written to, never read
#[no_mangle]
pub static mut D_8015FCD0: [libc::c_char; 20] = [0; 20];
// unreferenced
#[no_mangle]
pub static mut D_8015FCE4: u8_0 = 0;
#[no_mangle]
pub unsafe extern "C" fn Cutscene_DrawDebugInfo(mut globalCtx:
                                                    *mut GlobalContext,
                                                mut dlist: *mut *mut Gfx,
                                                mut csCtx:
                                                    *mut CutsceneContext) {
    let mut printer: GfxPrint =
        GfxPrint{callback: None,
                 dList: 0 as *mut Gfx,
                 posX: 0,
                 posY: 0,
                 baseX: 0,
                 baseY: 0,
                 flags: 0,
                 color:
                     Color_RGBA8_u32{c2rust_unnamed:
                                         C2RustUnnamed_4{r: 0,
                                                         g: 0,
                                                         b: 0,
                                                         a: 0,},},
                 unk_14: [0; 28],}; // "Cutscene start request announcement!"
    let mut pad: [s32; 2] = [0; 2];
    GfxPrint_Init(&mut printer);
    GfxPrint_Open(&mut printer, *dlist);
    GfxPrint_SetPos(&mut printer, 22 as libc::c_int, 25 as libc::c_int);
    GfxPrint_SetColor(&mut printer, 255 as libc::c_int as u32_0,
                      255 as libc::c_int as u32_0, 55 as libc::c_int as u32_0,
                      32 as libc::c_int as u32_0);
    GfxPrint_Printf(&mut printer as *mut GfxPrint,
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    b"FLAME \x00" as *const u8 as *const libc::c_char);
    GfxPrint_SetColor(&mut printer, 255 as libc::c_int as u32_0,
                      255 as libc::c_int as u32_0,
                      255 as libc::c_int as u32_0,
                      32 as libc::c_int as u32_0);
    GfxPrint_Printf(&mut printer as *mut GfxPrint,
                    b"%06d\x00" as *const u8 as *const libc::c_char,
                    (*csCtx).frames as libc::c_int);
    GfxPrint_SetColor(&mut printer, 50 as libc::c_int as u32_0,
                      255 as libc::c_int as u32_0,
                      255 as libc::c_int as u32_0,
                      60 as libc::c_int as u32_0);
    GfxPrint_SetPos(&mut printer, 4 as libc::c_int, 26 as libc::c_int);
    GfxPrint_Printf(&mut printer as *mut GfxPrint,
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    b"SKIP=(START) or (Cursole Right)\x00" as *const u8 as
                        *const libc::c_char);
    *dlist = GfxPrint_Close(&mut printer);
    GfxPrint_Destroy(&mut printer);
}
#[no_mangle]
pub unsafe extern "C" fn func_8006450C(mut globalCtx: *mut GlobalContext,
                                       mut csCtx: *mut CutsceneContext) {
    (*csCtx).state = CS_STATE_IDLE as libc::c_int as u8_0;
    (*csCtx).unk_0C = 0.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn func_80064520(mut globalCtx: *mut GlobalContext,
                                       mut csCtx: *mut CutsceneContext) {
    (*csCtx).state = CS_STATE_SKIPPABLE_INIT as libc::c_int as u8_0;
    (*csCtx).linkAction = 0 as *mut CsCmdActorAction;
}
#[no_mangle]
pub unsafe extern "C" fn func_80064534(mut globalCtx: *mut GlobalContext,
                                       mut csCtx: *mut CutsceneContext) {
    if (*csCtx).state as libc::c_int !=
           CS_STATE_UNSKIPPABLE_EXEC as libc::c_int {
        (*csCtx).state = CS_STATE_UNSKIPPABLE_INIT as libc::c_int as u8_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_80064558(mut globalCtx: *mut GlobalContext,
                                       mut csCtx: *mut CutsceneContext) {
    if gSaveContext.cutsceneIndex < 0xfff0 as libc::c_int {
        sCsStateHandlers1[(*csCtx).state as
                              usize].expect("non-null function pointer")(globalCtx,
                                                                         csCtx);
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800645A0(mut globalCtx: *mut GlobalContext,
                                       mut csCtx: *mut CutsceneContext) {
    let mut input: *mut Input =
        &mut *(*globalCtx).state.input.as_mut_ptr().offset(0 as libc::c_int as
                                                               isize) as
            *mut Input;
    if !((*input).press.button as libc::c_int | !(0x200 as libc::c_int)) ==
           0 as libc::c_int &&
           (*csCtx).state as libc::c_int == CS_STATE_IDLE as libc::c_int &&
           gSaveContext.sceneSetupIndex >= 4 as libc::c_int {
        D_8015FCC8 = 0 as libc::c_int as u8_0;
        gSaveContext.cutsceneIndex = 0xfffd as libc::c_int;
        gSaveContext.cutsceneTrigger = 1 as libc::c_int as u8_0
    }
    if !((*input).press.button as libc::c_int | !(0x800 as libc::c_int)) ==
           0 as libc::c_int &&
           (*csCtx).state as libc::c_int == CS_STATE_IDLE as libc::c_int &&
           gSaveContext.sceneSetupIndex >= 4 as libc::c_int &&
           gDbgCamEnabled == 0 {
        D_8015FCC8 = 1 as libc::c_int as u8_0;
        gSaveContext.cutsceneIndex = 0xfffd as libc::c_int;
        gSaveContext.cutsceneTrigger = 1 as libc::c_int as u8_0
    }
    if gSaveContext.cutsceneTrigger as libc::c_int != 0 as libc::c_int &&
           (*globalCtx).sceneLoadFlag as libc::c_int == 0x14 as libc::c_int {
        gSaveContext.cutsceneTrigger = 0 as libc::c_int as u8_0
    }
    if gSaveContext.cutsceneTrigger as libc::c_int != 0 as libc::c_int &&
           (*csCtx).state as libc::c_int == CS_STATE_IDLE as libc::c_int {
        osSyncPrintf(b"\n\xe3\x83\x87\xe3\x83\xa2\xe9\x96\x8b\xe5\xa7\x8b\xe8\xa6\x81\xe6\xb1\x82 \xe7\x99\xba\xe4\xbb\xa4\xef\xbc\x81\x00"
                         as *const u8 as *const libc::c_char);
        gSaveContext.cutsceneIndex = 0xfffd as libc::c_int;
        gSaveContext.cutsceneTrigger = 1 as libc::c_int as u8_0
    }
    if gSaveContext.cutsceneIndex >= 0xfff0 as libc::c_int {
        func_80068ECC(globalCtx, csCtx);
        sCsStateHandlers2[(*csCtx).state as
                              usize].expect("non-null function pointer")(globalCtx,
                                                                         csCtx);
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_80064720(mut globalCtx: *mut GlobalContext,
                                       mut csCtx: *mut CutsceneContext) {
}
#[no_mangle]
pub unsafe extern "C" fn func_8006472C(mut globalCtx: *mut GlobalContext,
                                       mut csCtx: *mut CutsceneContext,
                                       mut target: f32_0) -> u32_0 {
    return Math_StepToF(&mut (*csCtx).unk_0C, target, 0.1f32) as u32_0;
}
#[no_mangle]
pub unsafe extern "C" fn func_80064760(mut globalCtx: *mut GlobalContext,
                                       mut csCtx: *mut CutsceneContext) {
    Interface_ChangeAlpha(1 as libc::c_int as u16_0);
    ShrinkWindow_SetVal(0x20 as libc::c_int);
    if func_8006472C(globalCtx, csCtx, 1.0f32) != 0 {
        Audio_SetCutsceneFlag(1 as libc::c_int as s8);
        (*csCtx).state = (*csCtx).state.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800647C0(mut globalCtx: *mut GlobalContext,
                                       mut csCtx: *mut CutsceneContext) {
    func_80068C3C(globalCtx, csCtx);
    Interface_ChangeAlpha(1 as libc::c_int as u16_0);
    ShrinkWindow_SetVal(0x20 as libc::c_int);
    if func_8006472C(globalCtx, csCtx, 1.0f32) != 0 {
        Audio_SetCutsceneFlag(1 as libc::c_int as s8);
        (*csCtx).state = (*csCtx).state.wrapping_add(1)
    };
}
// Command 3: Misc. Actions
#[no_mangle]
pub unsafe extern "C" fn func_80064824(mut globalCtx: *mut GlobalContext,
                                       mut csCtx: *mut CutsceneContext,
                                       mut cmd: *mut CsCmdBase) {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut temp: f32_0 = 0.;
    let mut sp3F: u8_0 = 0 as libc::c_int as u8_0;
    if ((*csCtx).frames as libc::c_int) < (*cmd).startFrame as libc::c_int ||
           (*csCtx).frames as libc::c_int >= (*cmd).endFrame as libc::c_int &&
               (*cmd).endFrame as libc::c_int !=
                   (*cmd).startFrame as libc::c_int {
        return
    }
    temp =
        Environment_LerpWeight(((*cmd).endFrame as libc::c_int -
                                    1 as libc::c_int) as u16_0,
                               (*cmd).startFrame, (*csCtx).frames);
    if (*csCtx).frames as libc::c_int == (*cmd).startFrame as libc::c_int {
        sp3F = 1 as libc::c_int as u8_0
    }
    match (*cmd).base as libc::c_int {
        1 => {
            if sp3F as libc::c_int != 0 as libc::c_int {
                func_800F6D58(0xe as libc::c_int as u8_0,
                              4 as libc::c_int as u8_0,
                              0x3f as libc::c_int as u8_0);
                func_800F6D58(0xe as libc::c_int as u8_0,
                              1 as libc::c_int as u8_0,
                              1 as libc::c_int as u8_0);
                (*globalCtx).envCtx.unk_EE[0 as libc::c_int as usize] =
                    20 as libc::c_int as u8_0
            }
        }
        2 => {
            if sp3F as libc::c_int != 0 as libc::c_int {
                func_800F6D58(0xf as libc::c_int as u8_0,
                              0 as libc::c_int as u8_0,
                              0 as libc::c_int as u8_0);
                Environment_AddLightningBolts(globalCtx,
                                              3 as libc::c_int as u8_0);
                gLightningStrike.state =
                    LIGHTNING_STRIKE_START as libc::c_int as u8_0
            }
        }
        3 => {
            if sp3F as libc::c_int != 0 as libc::c_int {
                Flags_SetEnv(globalCtx, 0 as libc::c_int as s16);
                if gSaveContext.entranceIndex == 0x53 as libc::c_int {
                    Flags_SetEnv(globalCtx, 2 as libc::c_int as s16);
                }
            }
        }
        6 => {
            if ((*globalCtx).envCtx.adjFogFar as libc::c_int) <
                   12800 as libc::c_int {
                (*globalCtx).envCtx.adjFogFar =
                    ((*globalCtx).envCtx.adjFogFar as libc::c_int +
                         35 as libc::c_int) as s16
            }
        }
        7 => {
            if sp3F as libc::c_int != 0 as libc::c_int {
                (*globalCtx).envCtx.unk_19 = 1 as libc::c_int as u8_0;
                (*globalCtx).envCtx.unk_17 = 1 as libc::c_int as u8_0;
                (*globalCtx).envCtx.unk_18 = 0 as libc::c_int as u8_0;
                (*globalCtx).envCtx.unk_1A = 0x3c as libc::c_int as u16_0;
                (*globalCtx).envCtx.unk_21 = 1 as libc::c_int as u8_0;
                (*globalCtx).envCtx.unk_1F = 0 as libc::c_int as u8_0;
                (*globalCtx).envCtx.unk_20 = 1 as libc::c_int as u8_0;
                (*globalCtx).envCtx.unk_24 = 0x3c as libc::c_int as u16_0;
                (*globalCtx).envCtx.unk_22 = (*globalCtx).envCtx.unk_24
            }
        }
        8 => {
            if ((*globalCtx).roomCtx.unk_74[0 as libc::c_int as usize] as
                    libc::c_int) < 0x80 as libc::c_int {
                (*globalCtx).roomCtx.unk_74[0 as libc::c_int as usize] =
                    ((*globalCtx).roomCtx.unk_74[0 as libc::c_int as usize] as
                         libc::c_int + 4 as libc::c_int) as s16
            }
        }
        9 => {
            (*globalCtx).envCtx.unk_EE[3 as libc::c_int as usize] =
                16 as libc::c_int as u8_0
        }
        10 => { Flags_SetEnv(globalCtx, 1 as libc::c_int as s16); }
        11 => {
            if ((*globalCtx).roomCtx.unk_74[0 as libc::c_int as usize] as
                    libc::c_int) < 0x672 as libc::c_int {
                (*globalCtx).roomCtx.unk_74[0 as libc::c_int as usize] =
                    ((*globalCtx).roomCtx.unk_74[0 as libc::c_int as usize] as
                         libc::c_int + 0x14 as libc::c_int) as s16
            }
            if (*csCtx).frames as libc::c_int == 0x30f as libc::c_int {
                func_80078884(0x288e as libc::c_int as u16_0);
            } else if (*csCtx).frames as libc::c_int == 0x2cd as libc::c_int {
                (*globalCtx).roomCtx.unk_74[0 as libc::c_int as usize] =
                    0 as libc::c_int as s16
            }
        }
        12 => {
            if sp3F as libc::c_int != 0 as libc::c_int {
                if (*csCtx).state as libc::c_int !=
                       CS_STATE_UNSKIPPABLE_EXEC as libc::c_int {
                    (*csCtx).state =
                        CS_STATE_UNSKIPPABLE_INIT as libc::c_int as u8_0
                }
            }
        }
        13 => {
            if (*globalCtx).roomCtx.unk_74[1 as libc::c_int as usize] as
                   libc::c_int == 0 as libc::c_int {
                func_80078884(0x286f as libc::c_int as u16_0);
            }
            if ((*globalCtx).roomCtx.unk_74[1 as libc::c_int as usize] as
                    libc::c_int) < 0xff as libc::c_int {
                (*globalCtx).roomCtx.unk_74[1 as libc::c_int as usize] =
                    ((*globalCtx).roomCtx.unk_74[1 as libc::c_int as usize] as
                         libc::c_int + 5 as libc::c_int) as s16
            }
        }
        14 => {
            if sp3F as libc::c_int != 0 as libc::c_int {
                func_800BC490(globalCtx, 1 as libc::c_int as s16);
            }
        }
        15 => {
            if sp3F as libc::c_int != 0 as libc::c_int {
                TitleCard_InitPlaceName(globalCtx,
                                        &mut (*globalCtx).actorCtx.titleCtx,
                                        (*player).giObjectSegment,
                                        160 as libc::c_int,
                                        120 as libc::c_int,
                                        144 as libc::c_int, 24 as libc::c_int,
                                        20 as libc::c_int);
            }
        }
        16 => {
            if sp3F as libc::c_int != 0 as libc::c_int {
                sQuakeIndex =
                    Quake_Add((*globalCtx).cameraPtrs[(*globalCtx).activeCamera
                                                          as usize],
                              6 as libc::c_int as u32_0);
                Quake_SetSpeed(sQuakeIndex, 0x7fff as libc::c_int as s16);
                Quake_SetQuakeValues(sQuakeIndex, 4 as libc::c_int as s16,
                                     0 as libc::c_int as s16,
                                     1000 as libc::c_int as s16,
                                     0 as libc::c_int as s16);
                Quake_SetCountdown(sQuakeIndex, 800 as libc::c_int as s16);
            }
        }
        17 => {
            if sp3F as libc::c_int != 0 as libc::c_int {
                Quake_RemoveFromIdx(sQuakeIndex);
            }
        }
        18 => {
            (*globalCtx).envCtx.unk_EE[0 as libc::c_int as usize] =
                0 as libc::c_int as u8_0;
            (*globalCtx).envCtx.gloomySkyMode = 2 as libc::c_int as u8_0;
            if (gSaveContext.dayTime as libc::c_int) < 0x4aab as libc::c_int {
                gSaveContext.dayTime =
                    (gSaveContext.dayTime as libc::c_int + 30 as libc::c_int)
                        as u16_0
            }
            if (*globalCtx).envCtx.unk_EE[1 as libc::c_int as usize] as
                   libc::c_int == 0 as libc::c_int {
                gWeatherMode = 0 as libc::c_int as u8_0;
                func_800F6D58(14 as libc::c_int as u8_0,
                              1 as libc::c_int as u8_0,
                              0 as libc::c_int as u8_0);
            }
        }
        19 => {
            gSaveContext.eventChkInf[6 as libc::c_int as usize] =
                (gSaveContext.eventChkInf[6 as libc::c_int as usize] as
                     libc::c_int | 0x20 as libc::c_int) as u16_0
        }
        20 => {
            gSaveContext.eventChkInf[6 as libc::c_int as usize] =
                (gSaveContext.eventChkInf[6 as libc::c_int as usize] as
                     libc::c_int | 0x80 as libc::c_int) as u16_0
        }
        21 => {
            gSaveContext.eventChkInf[6 as libc::c_int as usize] =
                (gSaveContext.eventChkInf[6 as libc::c_int as usize] as
                     libc::c_int | 0x200 as libc::c_int) as u16_0
        }
        22 => {
            D_801614B0.c2rust_unnamed.r = 255 as libc::c_int as u8_0;
            D_801614B0.c2rust_unnamed.g = 255 as libc::c_int as u8_0;
            D_801614B0.c2rust_unnamed.b = 255 as libc::c_int as u8_0;
            D_801614B0.c2rust_unnamed.a = 255 as libc::c_int as u8_0
        }
        23 => {
            D_801614B0.c2rust_unnamed.r = 255 as libc::c_int as u8_0;
            D_801614B0.c2rust_unnamed.g = 180 as libc::c_int as u8_0;
            D_801614B0.c2rust_unnamed.b = 100 as libc::c_int as u8_0;
            D_801614B0.c2rust_unnamed.a = (255.0f32 * temp) as u8_0
        }
        24 => {
            (*globalCtx).roomCtx.curRoom.segment = 0 as *mut libc::c_void
        }
        25 => {
            gSaveContext.dayTime =
                (gSaveContext.dayTime as libc::c_int + 30 as libc::c_int) as
                    u16_0;
            if gSaveContext.dayTime as libc::c_int > 0xcaaa as libc::c_int {
                gSaveContext.dayTime = 0xcaaa as libc::c_int as u16_0
            }
        }
        26 => {
            if (gSaveContext.dayTime as libc::c_int) < 0x3000 as libc::c_int
                   ||
                   gSaveContext.dayTime as libc::c_int >=
                       0x4555 as libc::c_int {
                if gSaveContext.dayTime as libc::c_int >=
                       0x4555 as libc::c_int &&
                       (gSaveContext.dayTime as libc::c_int) <
                           0xaaab as libc::c_int {
                    (*globalCtx).envCtx.unk_BF = 1 as libc::c_int as u8_0
                } else if gSaveContext.dayTime as libc::c_int >=
                              0xaaab as libc::c_int &&
                              (gSaveContext.dayTime as libc::c_int) <
                                  0xc556 as libc::c_int {
                    (*globalCtx).envCtx.unk_BF = 2 as libc::c_int as u8_0
                } else {
                    (*globalCtx).envCtx.unk_BF = 3 as libc::c_int as u8_0
                }
            }
        }
        27 => {
            if (*globalCtx).state.frames & 8 as libc::c_int as libc::c_uint !=
                   0 {
                if ((*globalCtx).envCtx.adjAmbientColor[0 as libc::c_int as
                                                            usize] as
                        libc::c_int) < 40 as libc::c_int {
                    (*globalCtx).envCtx.adjAmbientColor[0 as libc::c_int as
                                                            usize] =
                        ((*globalCtx).envCtx.adjAmbientColor[0 as libc::c_int
                                                                 as usize] as
                             libc::c_int + 2 as libc::c_int) as s16;
                    (*globalCtx).envCtx.adjLight1Color[1 as libc::c_int as
                                                           usize] =
                        ((*globalCtx).envCtx.adjLight1Color[1 as libc::c_int
                                                                as usize] as
                             libc::c_int - 3 as libc::c_int) as s16;
                    (*globalCtx).envCtx.adjLight1Color[2 as libc::c_int as
                                                           usize] =
                        ((*globalCtx).envCtx.adjLight1Color[2 as libc::c_int
                                                                as usize] as
                             libc::c_int - 3 as libc::c_int) as s16
                }
            } else if (*globalCtx).envCtx.adjAmbientColor[0 as libc::c_int as
                                                              usize] as
                          libc::c_int > 2 as libc::c_int {
                (*globalCtx).envCtx.adjAmbientColor[0 as libc::c_int as usize]
                    =
                    ((*globalCtx).envCtx.adjAmbientColor[0 as libc::c_int as
                                                             usize] as
                         libc::c_int - 2 as libc::c_int) as s16;
                (*globalCtx).envCtx.adjLight1Color[1 as libc::c_int as usize]
                    =
                    ((*globalCtx).envCtx.adjLight1Color[1 as libc::c_int as
                                                            usize] as
                         libc::c_int + 3 as libc::c_int) as s16;
                (*globalCtx).envCtx.adjLight1Color[2 as libc::c_int as usize]
                    =
                    ((*globalCtx).envCtx.adjLight1Color[2 as libc::c_int as
                                                            usize] as
                         libc::c_int + 3 as libc::c_int) as s16
            }
        }
        28 => { (*globalCtx).unk_11DE9 = 1 as libc::c_int as u8_0 }
        29 => { (*globalCtx).unk_11DE9 = 0 as libc::c_int as u8_0 }
        30 => { Flags_SetEnv(globalCtx, 3 as libc::c_int as s16); }
        31 => { Flags_SetEnv(globalCtx, 4 as libc::c_int as s16); }
        32 => {
            if sp3F as libc::c_int != 0 as libc::c_int {
                (*globalCtx).envCtx.sandstormState = 1 as libc::c_int as u8_0
            }
            func_800788CC((0x28c0 as libc::c_int - 0x800 as libc::c_int) as
                              u16_0);
        }
        33 => {
            gSaveContext.sunsSongState = SUNSSONG_START as libc::c_int as s16
        }
        34 => {
            if gSaveContext.nightFlag == 0 as libc::c_int {
                gSaveContext.dayTime =
                    (gSaveContext.dayTime as libc::c_int -
                         gTimeIncrement as libc::c_int) as u16_0
            } else {
                gSaveContext.dayTime =
                    (gSaveContext.dayTime as libc::c_int -
                         gTimeIncrement as libc::c_int * 2 as libc::c_int) as
                        u16_0
            }
        }
        35 => {
            func_800EE824();
            (*csCtx).frames =
                ((*cmd).startFrame as libc::c_int - 1 as libc::c_int) as u16_0
        }
        _ => { }
    };
}
// Command 4: Set Environment Lighting
#[no_mangle]
pub unsafe extern "C" fn Cutscene_Command_SetLighting(mut globalCtx:
                                                          *mut GlobalContext,
                                                      mut csCtx:
                                                          *mut CutsceneContext,
                                                      mut cmd:
                                                          *mut CsCmdEnvLighting) {
    if (*csCtx).frames as libc::c_int == (*cmd).startFrame as libc::c_int {
        (*globalCtx).envCtx.unk_BF =
            ((*cmd).setting as libc::c_int - 1 as libc::c_int) as u8_0;
        (*globalCtx).envCtx.unk_D8 = 1.0f32
    };
}
// Command 0x56: Play Background Music
#[no_mangle]
pub unsafe extern "C" fn Cutscene_Command_PlayBGM(mut globalCtx:
                                                      *mut GlobalContext,
                                                  mut csCtx:
                                                      *mut CutsceneContext,
                                                  mut cmd:
                                                      *mut CsCmdMusicChange) {
    if (*csCtx).frames as libc::c_int == (*cmd).startFrame as libc::c_int {
        func_800F595C(((*cmd).sequence as libc::c_int - 1 as libc::c_int) as
                          u16_0);
    };
}
// Command 0x57: Stop Background Music
#[no_mangle]
pub unsafe extern "C" fn Cutscene_Command_StopBGM(mut globalCtx:
                                                      *mut GlobalContext,
                                                  mut csCtx:
                                                      *mut CutsceneContext,
                                                  mut cmd:
                                                      *mut CsCmdMusicChange) {
    if (*csCtx).frames as libc::c_int == (*cmd).startFrame as libc::c_int {
        func_800F59E8(((*cmd).sequence as libc::c_int - 1 as libc::c_int) as
                          u16_0);
    };
}
// Command 0x7C: Fade Background Music over duration
#[no_mangle]
pub unsafe extern "C" fn Cutscene_Command_FadeBGM(mut globalCtx:
                                                      *mut GlobalContext,
                                                  mut csCtx:
                                                      *mut CutsceneContext,
                                                  mut cmd:
                                                      *mut CsCmdMusicFade) {
    let mut var1: u8_0 = 0;
    if (*csCtx).frames as libc::c_int == (*cmd).startFrame as libc::c_int &&
           ((*csCtx).frames as libc::c_int) < (*cmd).endFrame as libc::c_int {
        var1 =
            ((*cmd).endFrame as libc::c_int -
                 (*cmd).startFrame as libc::c_int) as u8_0;
        if (*cmd).type_0 as libc::c_int == 3 as libc::c_int {
            Audio_QueueSeqCmd(((var1 as libc::c_int) << 0x10 as libc::c_int |
                                   ((0x1 as libc::c_int) << 28 as libc::c_int
                                        |
                                        (SEQ_PLAYER_FANFARE as libc::c_int) <<
                                            24 as libc::c_int |
                                        0xff as libc::c_int)) as u32_0);
        } else {
            Audio_QueueSeqCmd(((var1 as libc::c_int) << 0x10 as libc::c_int |
                                   ((0x1 as libc::c_int) << 28 as libc::c_int
                                        |
                                        (SEQ_PLAYER_BGM_MAIN as libc::c_int)
                                            << 24 as libc::c_int |
                                        0xff as libc::c_int)) as u32_0);
        }
    };
}
// Command 9: ?
#[no_mangle]
pub unsafe extern "C" fn Cutscene_Command_09(mut globalCtx:
                                                 *mut GlobalContext,
                                             mut csCtx: *mut CutsceneContext,
                                             mut cmd: *mut CsCmdUnknown9) {
    if (*csCtx).frames as libc::c_int == (*cmd).startFrame as libc::c_int {
        func_800AA000(0.0f32, (*cmd).unk_06, (*cmd).unk_07, (*cmd).unk_08);
    };
}
// Command 0x8C: Set Time of Day & Environment Time
#[no_mangle]
pub unsafe extern "C" fn func_80065134(mut globalCtx: *mut GlobalContext,
                                       mut csCtx: *mut CutsceneContext,
                                       mut cmd: *mut CsCmdDayTime) {
    let mut temp1: s16 = 0;
    let mut temp2: s16 = 0;
    if (*csCtx).frames as libc::c_int == (*cmd).startFrame as libc::c_int {
        temp1 =
            ((*cmd).hour as libc::c_int as libc::c_float * 60.0f32 /
                 (360.0f32 / 0x4000 as libc::c_int as libc::c_float)) as s16;
        temp2 =
            (((*cmd).minute as libc::c_int + 1 as libc::c_int) as
                 libc::c_float /
                 (360.0f32 / 0x4000 as libc::c_int as libc::c_float)) as s16;
        gSaveContext.dayTime =
            (temp1 as libc::c_int + temp2 as libc::c_int) as u16_0;
        gSaveContext.skyboxTime =
            (temp1 as libc::c_int + temp2 as libc::c_int) as u16_0
    };
}
// Command 0x3E8: Code Execution (& Terminates Cutscene?)
#[no_mangle]
pub unsafe extern "C" fn Cutscene_Command_Terminator(mut globalCtx:
                                                         *mut GlobalContext,
                                                     mut csCtx:
                                                         *mut CutsceneContext,
                                                     mut cmd:
                                                         *mut CsCmdBase) {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as
            *mut Player; // "Future fork designation=No. [%d]"
    let mut temp: s32 = 0 as libc::c_int;
    if gSaveContext.gameMode != 0 as libc::c_int &&
           gSaveContext.gameMode != 3 as libc::c_int &&
           (*globalCtx).sceneNum as libc::c_int != SCENE_SPOT00 as libc::c_int
           && (*csCtx).frames as libc::c_int > 20 as libc::c_int &&
           (!((*globalCtx).state.input[0 as libc::c_int as usize].press.button
                  as libc::c_int | !(0x8000 as libc::c_int)) ==
                0 as libc::c_int ||
                !((*globalCtx).state.input[0 as libc::c_int as
                                               usize].press.button as
                      libc::c_int | !(0x4000 as libc::c_int)) ==
                    0 as libc::c_int ||
                !((*globalCtx).state.input[0 as libc::c_int as
                                               usize].press.button as
                      libc::c_int | !(0x1000 as libc::c_int)) ==
                    0 as libc::c_int) &&
           gSaveContext.fileNum != 0xfedc as libc::c_int &&
           (*globalCtx).sceneLoadFlag as libc::c_int == 0 as libc::c_int {
        Audio_PlaySoundGeneral(0x4823 as libc::c_int as u16_0,
                               &mut D_801333D4, 4 as libc::c_int as u8_0,
                               &mut D_801333E0, &mut D_801333E0,
                               &mut D_801333E8);
        temp = 1 as libc::c_int
    }
    if (*csCtx).frames as libc::c_int == (*cmd).startFrame as libc::c_int ||
           temp != 0 as libc::c_int ||
           (*csCtx).frames as libc::c_int > 20 as libc::c_int &&
               !((*globalCtx).state.input[0 as libc::c_int as
                                              usize].press.button as
                     libc::c_int | !(0x1000 as libc::c_int)) ==
                   0 as libc::c_int &&
               gSaveContext.fileNum != 0xfedc as libc::c_int {
        (*csCtx).state = CS_STATE_UNSKIPPABLE_EXEC as libc::c_int as u8_0;
        Audio_SetCutsceneFlag(0 as libc::c_int as s8);
        gSaveContext.unk_1410 = 1 as libc::c_int as u8_0;
        osSyncPrintf(b"\n\xe5\x88\x86\xe5\xb2\x90\xe5\x85\x88\xe6\x8c\x87\xe5\xae\x9a\xef\xbc\x81\xef\xbc\x81=[%d]\xe7\x95\xaa\x00"
                         as *const u8 as *const libc::c_char,
                     (*cmd).base as libc::c_int);
        if gSaveContext.gameMode != 0 as libc::c_int &&
               (*csCtx).frames as libc::c_int !=
                   (*cmd).startFrame as libc::c_int {
            gSaveContext.unk_13E7 = 1 as libc::c_int as u8_0
        }
        gSaveContext.cutsceneIndex = 0 as libc::c_int;
        match (*cmd).base as libc::c_int {
            1 => {
                (*globalCtx).nextEntranceIndex = 0xa0 as libc::c_int as s16;
                gSaveContext.cutsceneIndex = 0xfff1 as libc::c_int;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0
            }
            2 => {
                (*globalCtx).nextEntranceIndex = 0xa0 as libc::c_int as s16;
                gSaveContext.cutsceneIndex = 0xfff0 as libc::c_int;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 10 as libc::c_int as u8_0
            }
            3 => {
                (*globalCtx).nextEntranceIndex = 0x117 as libc::c_int as s16;
                gSaveContext.cutsceneIndex = 0xfff1 as libc::c_int;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 10 as libc::c_int as u8_0
            }
            4 => {
                (*globalCtx).nextEntranceIndex = 0x13d as libc::c_int as s16;
                gSaveContext.cutsceneIndex = 0xfff0 as libc::c_int;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 10 as libc::c_int as u8_0
            }
            5 => {
                (*globalCtx).nextEntranceIndex = 0xee as libc::c_int as s16;
                gSaveContext.cutsceneIndex = 0xfff0 as libc::c_int;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 10 as libc::c_int as u8_0
            }
            6 => {
                (*globalCtx).nextEntranceIndex = 0xa0 as libc::c_int as s16;
                gSaveContext.cutsceneIndex = 0xfff2 as libc::c_int;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 10 as libc::c_int as u8_0
            }
            7 => {
                (*globalCtx).nextEntranceIndex = 0xee as libc::c_int as s16;
                gSaveContext.cutsceneIndex = 0xfff2 as libc::c_int;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 11 as libc::c_int as u8_0
            }
            8 => {
                gSaveContext.fw.set = 0 as libc::c_int;
                gSaveContext.respawn[RESPAWN_MODE_TOP as libc::c_int as
                                         usize].data = 0 as libc::c_int as s8;
                if gSaveContext.eventChkInf[4 as libc::c_int as usize] as
                       libc::c_int & 0x20 as libc::c_int == 0 {
                    gSaveContext.eventChkInf[4 as libc::c_int as usize] =
                        (gSaveContext.eventChkInf[4 as libc::c_int as usize]
                             as libc::c_int | 0x20 as libc::c_int) as u16_0;
                    (*globalCtx).nextEntranceIndex =
                        0xa0 as libc::c_int as s16;
                    (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                    gSaveContext.cutsceneIndex = 0xfff3 as libc::c_int;
                    (*globalCtx).fadeTransition = 11 as libc::c_int as u8_0
                } else {
                    if gSaveContext.sceneSetupIndex < 4 as libc::c_int {
                        if !(gSaveContext.linkAge == 0 as libc::c_int) {
                            (*globalCtx).linkAgeOnLoad =
                                0 as libc::c_int as u8_0
                        } else {
                            (*globalCtx).linkAgeOnLoad =
                                1 as libc::c_int as u8_0
                        }
                    }
                    (*globalCtx).nextEntranceIndex =
                        0x2ca as libc::c_int as s16;
                    (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                    (*globalCtx).fadeTransition = 3 as libc::c_int as u8_0;
                    gSaveContext.nextTransition = 3 as libc::c_int as u8_0
                }
            }
            9 => {
                (*globalCtx).nextEntranceIndex = 0x117 as libc::c_int as s16;
                gSaveContext.cutsceneIndex = 0xfff0 as libc::c_int;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 12 as libc::c_int as u8_0
            }
            10 => {
                (*globalCtx).nextEntranceIndex = 0xbb as libc::c_int as s16;
                gSaveContext.cutsceneIndex = 0xfff0 as libc::c_int;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0
            }
            11 => {
                (*globalCtx).nextEntranceIndex = 0xee as libc::c_int as s16;
                gSaveContext.cutsceneIndex = 0xfff3 as libc::c_int;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 3 as libc::c_int as u8_0
            }
            12 => {
                (*globalCtx).nextEntranceIndex = 0x47a as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0
            }
            13 => {
                (*globalCtx).nextEntranceIndex = 0x10e as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0;
                gSaveContext.nextTransition = 2 as libc::c_int as u8_0
            }
            14 => {
                (*globalCtx).nextEntranceIndex = 0x457 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0
            }
            15 => {
                (*globalCtx).nextEntranceIndex = 0x53 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                gSaveContext.cutsceneIndex = 0xfff4 as libc::c_int;
                (*globalCtx).fadeTransition = 3 as libc::c_int as u8_0
            }
            16 => {
                (*globalCtx).nextEntranceIndex = 0x53 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                gSaveContext.cutsceneIndex = 0xfff5 as libc::c_int;
                (*globalCtx).fadeTransition = 3 as libc::c_int as u8_0
            }
            17 => {
                (*globalCtx).nextEntranceIndex = 0x53 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                gSaveContext.cutsceneIndex = 0xfff6 as libc::c_int;
                (*globalCtx).fadeTransition = 3 as libc::c_int as u8_0
            }
            18 => {
                gSaveContext.eventChkInf[4 as libc::c_int as usize] =
                    (gSaveContext.eventChkInf[4 as libc::c_int as usize] as
                         libc::c_int | 0x8000 as libc::c_int) as u16_0;
                (*globalCtx).nextEntranceIndex = 0x324 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0;
                gSaveContext.nextTransition = 2 as libc::c_int as u8_0
            }
            19 => {
                (*globalCtx).nextEntranceIndex = 0x13d as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 4 as libc::c_int as u8_0;
                gSaveContext.cutsceneIndex = 0x8000 as libc::c_int
            }
            21 => {
                (*globalCtx).nextEntranceIndex = 0x102 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                gSaveContext.cutsceneIndex = 0xfff0 as libc::c_int;
                (*globalCtx).fadeTransition = 3 as libc::c_int as u8_0
            }
            22 => {
                Item_Give(globalCtx,
                          ITEM_SONG_REQUIEM as libc::c_int as u8_0);
                (*globalCtx).nextEntranceIndex = 0x123 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                gSaveContext.cutsceneIndex = 0xfff0 as libc::c_int;
                (*globalCtx).fadeTransition = 3 as libc::c_int as u8_0
            }
            23 => {
                (*globalCtx).nextEntranceIndex = 0xa0 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                gSaveContext.cutsceneIndex = 0xfff8 as libc::c_int;
                (*globalCtx).fadeTransition = 3 as libc::c_int as u8_0
            }
            24 => {
                (*globalCtx).nextEntranceIndex = 0x28 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0
            }
            25 => {
                (*globalCtx).linkAgeOnLoad = 0 as libc::c_int as u8_0;
                (*globalCtx).nextEntranceIndex = 0x6b as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                gSaveContext.cutsceneIndex = 0xfff0 as libc::c_int;
                (*globalCtx).fadeTransition = 3 as libc::c_int as u8_0
            }
            26 => {
                (*globalCtx).nextEntranceIndex = 0x53 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                gSaveContext.cutsceneIndex = 0xfff4 as libc::c_int;
                (*globalCtx).fadeTransition = 3 as libc::c_int as u8_0
            }
            27 => {
                (*globalCtx).nextEntranceIndex = 0x53 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                gSaveContext.cutsceneIndex = 0xfff5 as libc::c_int;
                (*globalCtx).fadeTransition = 3 as libc::c_int as u8_0
            }
            28 => {
                (*globalCtx).nextEntranceIndex = 0x53 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                gSaveContext.cutsceneIndex = 0xfff6 as libc::c_int;
                (*globalCtx).fadeTransition = 3 as libc::c_int as u8_0
            }
            29 => {
                (*globalCtx).nextEntranceIndex = 0x6b as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                gSaveContext.chamberCutsceneNum = 0 as libc::c_int as u8_0;
                (*globalCtx).fadeTransition = 3 as libc::c_int as u8_0
            }
            30 => {
                (*globalCtx).nextEntranceIndex = 0x6b as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 3 as libc::c_int as u8_0;
                Item_Give(globalCtx,
                          ITEM_MEDALLION_FIRE as libc::c_int as u8_0);
                gSaveContext.chamberCutsceneNum = 1 as libc::c_int as u8_0
            }
            31 => {
                (*globalCtx).nextEntranceIndex = 0x6b as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 3 as libc::c_int as u8_0;
                gSaveContext.chamberCutsceneNum = 2 as libc::c_int as u8_0
            }
            32 => {
                (*globalCtx).linkAgeOnLoad = 1 as libc::c_int as u8_0;
                (*globalCtx).nextEntranceIndex = 0xcd as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                gSaveContext.cutsceneIndex = 0xfff2 as libc::c_int;
                (*globalCtx).fadeTransition = 11 as libc::c_int as u8_0
            }
            33 => {
                (*globalCtx).nextEntranceIndex = 0xcd as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 3 as libc::c_int as u8_0
            }
            34 => {
                (*globalCtx).nextEntranceIndex = 0xa0 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                gSaveContext.cutsceneIndex = 0xfff3 as libc::c_int;
                (*globalCtx).fadeTransition = 3 as libc::c_int as u8_0
            }
            35 => {
                (*globalCtx).nextEntranceIndex = 0xcd as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                gSaveContext.cutsceneIndex = 0xfff0 as libc::c_int;
                (*globalCtx).fadeTransition = 4 as libc::c_int as u8_0
            }
            38 => {
                (*globalCtx).nextEntranceIndex = 0xa0 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                gSaveContext.cutsceneIndex = 0xfff4 as libc::c_int;
                (*globalCtx).fadeTransition = 4 as libc::c_int as u8_0
            }
            39 => {
                (*globalCtx).nextEntranceIndex = 0x53 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                gSaveContext.cutsceneIndex = 0xfff9 as libc::c_int;
                (*globalCtx).fadeTransition = 4 as libc::c_int as u8_0
            }
            40 => {
                (*globalCtx).linkAgeOnLoad = 0 as libc::c_int as u8_0;
                (*globalCtx).nextEntranceIndex = 0x53 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                gSaveContext.cutsceneIndex = 0xfffa as libc::c_int;
                (*globalCtx).fadeTransition = 4 as libc::c_int as u8_0
            }
            41 => {
                (*globalCtx).nextEntranceIndex = 0x4e6 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0
            }
            42 => {
                (*globalCtx).nextEntranceIndex = 0xdb as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                gSaveContext.cutsceneIndex = 0xfff2 as libc::c_int;
                (*globalCtx).fadeTransition = 4 as libc::c_int as u8_0
            }
            43 => {
                (*globalCtx).nextEntranceIndex = 0x503 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 4 as libc::c_int as u8_0
            }
            44 => {
                (*globalCtx).nextEntranceIndex = 0x320 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 17 as libc::c_int as u8_0
            }
            46 => {
                gSaveContext.eventChkInf[4 as libc::c_int as usize] =
                    (gSaveContext.eventChkInf[4 as libc::c_int as usize] as
                         libc::c_int | 0x8000 as libc::c_int) as u16_0;
                (*globalCtx).nextEntranceIndex = 0x324 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 4 as libc::c_int as u8_0
            }
            47 => {
                Item_Give(globalCtx,
                          ITEM_SONG_NOCTURNE as libc::c_int as u8_0);
                gSaveContext.eventChkInf[5 as libc::c_int as usize] =
                    (gSaveContext.eventChkInf[5 as libc::c_int as usize] as
                         libc::c_int | 0x10 as libc::c_int) as u16_0;
                (*globalCtx).nextEntranceIndex = 0xdb as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                gSaveContext.cutsceneIndex = 0xfff1 as libc::c_int;
                (*globalCtx).fadeTransition = 4 as libc::c_int as u8_0
            }
            48 => {
                (*globalCtx).nextEntranceIndex = 0x1ed as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 15 as libc::c_int as u8_0;
                gSaveContext.nextTransition = 15 as libc::c_int as u8_0
            }
            49 => {
                (*globalCtx).nextEntranceIndex = 0x58c as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 4 as libc::c_int as u8_0
            }
            50 => {
                (*globalCtx).nextEntranceIndex = 0x513 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 17 as libc::c_int as u8_0
            }
            51 => {
                (*globalCtx).nextEntranceIndex = 0xcd as libc::c_int as s16;
                gSaveContext.cutsceneIndex = 0xfff8 as libc::c_int;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 41 as libc::c_int as u8_0
            }
            52 => {
                (*globalCtx).nextEntranceIndex = 0x53 as libc::c_int as s16;
                gSaveContext.cutsceneIndex = 0xfff7 as libc::c_int;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 11 as libc::c_int as u8_0
            }
            53 => {
                (*globalCtx).nextEntranceIndex = 0x50f as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 3 as libc::c_int as u8_0
            }
            54 => {
                gSaveContext.gameMode = 3 as libc::c_int;
                Audio_SetSoundBanksMute(0x6f as libc::c_int as u16_0);
                (*globalCtx).linkAgeOnLoad = 1 as libc::c_int as u8_0;
                (*globalCtx).nextEntranceIndex = 0x117 as libc::c_int as s16;
                gSaveContext.cutsceneIndex = 0xfff2 as libc::c_int;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0
            }
            55 => {
                (*globalCtx).nextEntranceIndex = 0x129 as libc::c_int as s16;
                gSaveContext.cutsceneIndex = 0xfff1 as libc::c_int;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0
            }
            56 => {
                (*globalCtx).nextEntranceIndex = 0xdb as libc::c_int as s16;
                gSaveContext.cutsceneIndex = 0xfff4 as libc::c_int;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0
            }
            57 => {
                (*globalCtx).nextEntranceIndex = 0x13d as libc::c_int as s16;
                gSaveContext.cutsceneIndex = 0xfff3 as libc::c_int;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0
            }
            58 => {
                (*globalCtx).nextEntranceIndex = 0x14d as libc::c_int as s16;
                gSaveContext.cutsceneIndex = 0xfff1 as libc::c_int;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0
            }
            59 => {
                (*globalCtx).nextEntranceIndex = 0x102 as libc::c_int as s16;
                gSaveContext.cutsceneIndex = 0xfff1 as libc::c_int;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0
            }
            60 => {
                (*globalCtx).nextEntranceIndex = 0x10e as libc::c_int as s16;
                gSaveContext.cutsceneIndex = 0xfff2 as libc::c_int;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0
            }
            61 => {
                (*globalCtx).nextEntranceIndex = 0x108 as libc::c_int as s16;
                gSaveContext.cutsceneIndex = 0xfff0 as libc::c_int;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0
            }
            62 => {
                (*globalCtx).linkAgeOnLoad = 0 as libc::c_int as u8_0;
                (*globalCtx).nextEntranceIndex = 0xee as libc::c_int as s16;
                gSaveContext.cutsceneIndex = 0xfff6 as libc::c_int;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0
            }
            63 => {
                (*globalCtx).nextEntranceIndex = 0xee as libc::c_int as s16;
                gSaveContext.cutsceneIndex = 0xfff7 as libc::c_int;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0
            }
            64 => {
                (*globalCtx).nextEntranceIndex = 0xcd as libc::c_int as s16;
                gSaveContext.cutsceneIndex = 0xfff5 as libc::c_int;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0
            }
            65 => {
                (*globalCtx).linkAgeOnLoad = 1 as libc::c_int as u8_0;
                (*globalCtx).nextEntranceIndex = 0x157 as libc::c_int as s16;
                gSaveContext.cutsceneIndex = 0xfff2 as libc::c_int;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0
            }
            66 => {
                (*globalCtx).nextEntranceIndex = 0x554 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0
            }
            67 => {
                (*globalCtx).nextEntranceIndex = 0x27e as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0
            }
            68 => {
                (*globalCtx).nextEntranceIndex = 0xa0 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                gSaveContext.cutsceneIndex = 0xfff5 as libc::c_int;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0
            }
            69 => {
                (*globalCtx).nextEntranceIndex = 0x5e8 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0
            }
            70 => {
                (*globalCtx).nextEntranceIndex = 0x13d as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                gSaveContext.cutsceneIndex = 0xfff4 as libc::c_int;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0;
                gSaveContext.nextTransition = 2 as libc::c_int as u8_0
            }
            71 => {
                gSaveContext.equips.equipment =
                    (gSaveContext.equips.equipment as libc::c_int |
                         0x100 as libc::c_int) as u16_0;
                Player_SetEquipmentData(globalCtx, player);
                gSaveContext.equips.equipment =
                    (gSaveContext.equips.equipment as libc::c_int |
                         0x1000 as libc::c_int) as u16_0;
                Player_SetEquipmentData(globalCtx, player);
                (*globalCtx).linkAgeOnLoad = 1 as libc::c_int as u8_0;
                (*globalCtx).nextEntranceIndex = 0x53 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                gSaveContext.cutsceneIndex = 0xfff1 as libc::c_int;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0
            }
            72 => {
                (*globalCtx).nextEntranceIndex = 0x400 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                gSaveContext.cutsceneIndex = 0xfff0 as libc::c_int;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0;
                gSaveContext.nextTransition = 2 as libc::c_int as u8_0
            }
            73 => {
                (*globalCtx).linkAgeOnLoad = 1 as libc::c_int as u8_0;
                (*globalCtx).nextEntranceIndex = 0x157 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                gSaveContext.cutsceneIndex = 0xfff2 as libc::c_int;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0
            }
            74 => {
                (*globalCtx).nextEntranceIndex = 0x157 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                gSaveContext.cutsceneIndex = 0xfff3 as libc::c_int;
                (*globalCtx).fadeTransition = 3 as libc::c_int as u8_0;
                gSaveContext.nextTransition = 3 as libc::c_int as u8_0
            }
            75 => {
                (*globalCtx).linkAgeOnLoad = 1 as libc::c_int as u8_0;
                (*globalCtx).nextEntranceIndex = 0x157 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                gSaveContext.cutsceneIndex = 0xfff4 as libc::c_int;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0
            }
            76 => {
                (*globalCtx).linkAgeOnLoad = 0 as libc::c_int as u8_0;
                (*globalCtx).nextEntranceIndex = 0x157 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                gSaveContext.cutsceneIndex = 0xfff5 as libc::c_int;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0
            }
            77 => {
                (*globalCtx).linkAgeOnLoad = 1 as libc::c_int as u8_0;
                (*globalCtx).nextEntranceIndex = 0x157 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                gSaveContext.cutsceneIndex = 0xfff6 as libc::c_int;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0
            }
            78 => {
                (*globalCtx).nextEntranceIndex = 0x157 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                gSaveContext.cutsceneIndex = 0xfff7 as libc::c_int;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0
            }
            79 | 80 | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 | 91 |
            92 | 93 => {
                (*globalCtx).nextEntranceIndex = 0x157 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0
            }
            94 => {
                (*globalCtx).nextEntranceIndex = 0x2ae as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 3 as libc::c_int as u8_0
            }
            95 => {
                if gSaveContext.eventChkInf[4 as libc::c_int as usize] as
                       libc::c_int & 0x100 as libc::c_int != 0 &&
                       gSaveContext.eventChkInf[4 as libc::c_int as usize] as
                           libc::c_int & 0x200 as libc::c_int != 0 &&
                       gSaveContext.eventChkInf[4 as libc::c_int as usize] as
                           libc::c_int & 0x400 as libc::c_int != 0 {
                    (*globalCtx).nextEntranceIndex =
                        0x53 as libc::c_int as s16;
                    (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                    gSaveContext.cutsceneIndex = 0xfff3 as libc::c_int;
                    (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0
                } else {
                    match gSaveContext.sceneSetupIndex {
                        8 => {
                            (*globalCtx).nextEntranceIndex =
                                0xfc as libc::c_int as s16;
                            (*globalCtx).sceneLoadFlag =
                                0x14 as libc::c_int as s8;
                            (*globalCtx).fadeTransition =
                                2 as libc::c_int as u8_0
                        }
                        9 => {
                            (*globalCtx).nextEntranceIndex =
                                0x147 as libc::c_int as s16;
                            (*globalCtx).sceneLoadFlag =
                                0x14 as libc::c_int as s8;
                            (*globalCtx).fadeTransition =
                                2 as libc::c_int as u8_0
                        }
                        10 => {
                            (*globalCtx).nextEntranceIndex =
                                0x102 as libc::c_int as s16;
                            (*globalCtx).sceneLoadFlag =
                                0x14 as libc::c_int as s8;
                            gSaveContext.cutsceneIndex =
                                0xfff0 as libc::c_int;
                            (*globalCtx).fadeTransition =
                                3 as libc::c_int as u8_0
                        }
                        _ => { }
                    }
                }
            }
            96 => {
                if gBitFlags[QUEST_MEDALLION_SHADOW as libc::c_int as usize] &
                       gSaveContext.inventory.questItems != 0 {
                    (*globalCtx).nextEntranceIndex =
                        0x6b as libc::c_int as s16;
                    (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                    gSaveContext.cutsceneIndex = 0xfff1 as libc::c_int;
                    (*globalCtx).fadeTransition = 5 as libc::c_int as u8_0
                } else {
                    gSaveContext.eventChkInf[12 as libc::c_int as usize] =
                        (gSaveContext.eventChkInf[12 as libc::c_int as usize]
                             as libc::c_int | 0x100 as libc::c_int) as u16_0;
                    (*globalCtx).nextEntranceIndex =
                        0x610 as libc::c_int as s16;
                    (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                    (*globalCtx).fadeTransition = 3 as libc::c_int as u8_0;
                    gSaveContext.nextTransition = 3 as libc::c_int as u8_0
                }
            }
            97 => {
                if gBitFlags[QUEST_MEDALLION_SPIRIT as libc::c_int as usize] &
                       gSaveContext.inventory.questItems != 0 {
                    (*globalCtx).nextEntranceIndex =
                        0x6b as libc::c_int as s16;
                    (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                    gSaveContext.cutsceneIndex = 0xfff1 as libc::c_int;
                    (*globalCtx).fadeTransition = 5 as libc::c_int as u8_0
                } else {
                    (*globalCtx).nextEntranceIndex =
                        0x580 as libc::c_int as s16;
                    (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                    (*globalCtx).fadeTransition = 3 as libc::c_int as u8_0;
                    gSaveContext.nextTransition = 3 as libc::c_int as u8_0
                }
            }
            98 => {
                (*globalCtx).nextEntranceIndex = 0x564 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 3 as libc::c_int as u8_0;
                gSaveContext.nextTransition = 3 as libc::c_int as u8_0
            }
            99 => {
                (*globalCtx).nextEntranceIndex = 0x608 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0;
                gSaveContext.nextTransition = 2 as libc::c_int as u8_0
            }
            100 => {
                (*globalCtx).nextEntranceIndex = 0xee as libc::c_int as s16;
                gSaveContext.cutsceneIndex = 0xfff8 as libc::c_int;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 3 as libc::c_int as u8_0;
                gSaveContext.nextTransition = 3 as libc::c_int as u8_0
            }
            101 => {
                (*globalCtx).nextEntranceIndex = 0x1f5 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 15 as libc::c_int as u8_0
            }
            102 => {
                (*globalCtx).nextEntranceIndex = 0x590 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0
            }
            103 => {
                (*globalCtx).nextEntranceIndex = 0xcd as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                gSaveContext.cutsceneIndex = 0xfff3 as libc::c_int;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0
            }
            104 => {
                match sTitleCsState as libc::c_int {
                    0 => {
                        (*globalCtx).nextEntranceIndex =
                            0x8d as libc::c_int as s16;
                        (*globalCtx).sceneLoadFlag =
                            0x14 as libc::c_int as s8;
                        gSaveContext.cutsceneIndex = 0xfff2 as libc::c_int;
                        (*globalCtx).fadeTransition =
                            2 as libc::c_int as u8_0;
                        sTitleCsState = sTitleCsState.wrapping_add(1)
                    }
                    1 => {
                        (*globalCtx).nextEntranceIndex =
                            0x147 as libc::c_int as s16;
                        (*globalCtx).sceneLoadFlag =
                            0x14 as libc::c_int as s8;
                        gSaveContext.cutsceneIndex = 0xfff1 as libc::c_int;
                        (*globalCtx).fadeTransition =
                            2 as libc::c_int as u8_0;
                        sTitleCsState = sTitleCsState.wrapping_add(1)
                    }
                    2 => {
                        (*globalCtx).nextEntranceIndex =
                            0xa0 as libc::c_int as s16;
                        (*globalCtx).sceneLoadFlag =
                            0x14 as libc::c_int as s8;
                        gSaveContext.cutsceneIndex = 0xfff6 as libc::c_int;
                        (*globalCtx).fadeTransition =
                            2 as libc::c_int as u8_0;
                        sTitleCsState = 0 as libc::c_int as u8_0
                    }
                    _ => { }
                }
            }
            105 => {
                (*globalCtx).nextEntranceIndex = 0xe4 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                gSaveContext.cutsceneIndex = 0xfff1 as libc::c_int;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0
            }
            106 => {
                (*globalCtx).nextEntranceIndex = 0x574 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0
            }
            107 => {
                (*globalCtx).nextEntranceIndex = 0x538 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0
            }
            108 => {
                (*globalCtx).nextEntranceIndex = 0x53c as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0
            }
            109 => {
                (*globalCtx).nextEntranceIndex = 0x540 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0
            }
            110 => {
                (*globalCtx).nextEntranceIndex = 0x544 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0
            }
            111 => {
                (*globalCtx).nextEntranceIndex = 0x548 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0
            }
            112 => {
                (*globalCtx).nextEntranceIndex = 0x54c as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0
            }
            113 => {
                if Flags_GetEventChkInf(0xbb as libc::c_int) != 0 &&
                       Flags_GetEventChkInf(0xbc as libc::c_int) != 0 &&
                       Flags_GetEventChkInf(0xbd as libc::c_int) != 0 &&
                       Flags_GetEventChkInf(0xbe as libc::c_int) != 0 &&
                       Flags_GetEventChkInf(0xbf as libc::c_int) != 0 &&
                       Flags_GetEventChkInf(0xad as libc::c_int) != 0 {
                    (*globalCtx).csCtx.segment =
                        gSegments[((gTowerBarrierCs.as_mut_ptr() as u32_0) <<
                                       4 as libc::c_int >> 28 as libc::c_int)
                                      as
                                      usize].wrapping_add(gTowerBarrierCs.as_mut_ptr()
                                                              as u32_0 &
                                                              0xffffff as
                                                                  libc::c_int
                                                                  as
                                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                                 as
                                                                                                 libc::c_uint)
                            as *mut libc::c_void;
                    (*globalCtx).csCtx.frames = 0 as libc::c_int as u16_0;
                    gSaveContext.cutsceneTrigger = 1 as libc::c_int as u8_0;
                    gSaveContext.cutsceneIndex = 0xffff as libc::c_int;
                    (*csCtx).state =
                        CS_STATE_UNSKIPPABLE_INIT as libc::c_int as u8_0
                } else {
                    gSaveContext.cutsceneIndex = 0xffff as libc::c_int;
                    (*csCtx).state =
                        CS_STATE_UNSKIPPABLE_INIT as libc::c_int as u8_0
                }
            }
            114 => {
                (*globalCtx).nextEntranceIndex = 0x185 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0
            }
            115 => {
                (*globalCtx).nextEntranceIndex = 0x594 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 2 as libc::c_int as u8_0;
                gSaveContext.nextTransition = 2 as libc::c_int as u8_0
            }
            116 => {
                if gSaveContext.eventChkInf[12 as libc::c_int as usize] as
                       libc::c_int & 0x100 as libc::c_int != 0 {
                    (*globalCtx).nextEntranceIndex =
                        0x580 as libc::c_int as s16;
                    (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                    (*globalCtx).fadeTransition = 3 as libc::c_int as u8_0
                } else {
                    (*globalCtx).nextEntranceIndex =
                        0x610 as libc::c_int as s16;
                    (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                    (*globalCtx).fadeTransition = 3 as libc::c_int as u8_0
                }
                gSaveContext.nextTransition = 3 as libc::c_int as u8_0
            }
            117 => {
                gSaveContext.gameMode = 3 as libc::c_int;
                Audio_SetSoundBanksMute(0x6f as libc::c_int as u16_0);
                (*globalCtx).linkAgeOnLoad = 0 as libc::c_int as u8_0;
                (*globalCtx).nextEntranceIndex = 0xcd as libc::c_int as s16;
                gSaveContext.cutsceneIndex = 0xfff7 as libc::c_int;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 3 as libc::c_int as u8_0
            }
            118 => {
                gSaveContext.respawn[RESPAWN_MODE_DOWN as libc::c_int as
                                         usize].entranceIndex =
                    0x517 as libc::c_int as s16;
                Gameplay_TriggerVoidOut(globalCtx);
                gSaveContext.respawnFlag = -(2 as libc::c_int);
                gSaveContext.nextTransition = 2 as libc::c_int as u8_0
            }
            119 => {
                gSaveContext.dayTime = 0x8000 as libc::c_int as u16_0;
                gSaveContext.skyboxTime = 0x8000 as libc::c_int as u16_0;
                (*globalCtx).nextEntranceIndex = 0x5f0 as libc::c_int as s16;
                (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
                (*globalCtx).fadeTransition = 3 as libc::c_int as u8_0
            }
            _ => { }
        }
    };
}
// Command 0x2D: Transition Effects
#[no_mangle]
pub unsafe extern "C" fn Cutscene_Command_TransitionFX(mut globalCtx:
                                                           *mut GlobalContext,
                                                       mut csCtx:
                                                           *mut CutsceneContext,
                                                       mut cmd:
                                                           *mut CsCmdBase) {
    let mut temp: f32_0 = 0.;
    if (*csCtx).frames as libc::c_int >= (*cmd).startFrame as libc::c_int &&
           (*csCtx).frames as libc::c_int <= (*cmd).endFrame as libc::c_int {
        (*globalCtx).envCtx.fillScreen = 1 as libc::c_int as u8_0;
        temp =
            Environment_LerpWeight((*cmd).endFrame, (*cmd).startFrame,
                                   (*csCtx).frames);
        match (*cmd).base as libc::c_int {
            1 | 5 => {
                (*globalCtx).envCtx.screenFillColor[0 as libc::c_int as usize]
                    = 160 as libc::c_int as u8_0;
                (*globalCtx).envCtx.screenFillColor[1 as libc::c_int as usize]
                    = 160 as libc::c_int as u8_0;
                (*globalCtx).envCtx.screenFillColor[2 as libc::c_int as usize]
                    = 160 as libc::c_int as u8_0;
                if (*cmd).base as libc::c_int == 1 as libc::c_int {
                    (*globalCtx).envCtx.screenFillColor[3 as libc::c_int as
                                                            usize] =
                        (255.0f32 * temp) as u8_0;
                    if temp == 0.0f32 &&
                           gSaveContext.entranceIndex == 0x6b as libc::c_int {
                        Audio_PlaySoundGeneral(0x4833 as libc::c_int as u16_0,
                                               &mut D_801333D4,
                                               4 as libc::c_int as u8_0,
                                               &mut D_801333E0,
                                               &mut D_801333E0,
                                               &mut D_801333E8);
                    } else if temp == 0.0f32 &&
                                  (gSaveContext.entranceIndex ==
                                       0x53 as libc::c_int ||
                                       gSaveContext.entranceIndex ==
                                           0x138 as libc::c_int ||
                                       gSaveContext.entranceIndex ==
                                           0x371 as libc::c_int) {
                        Audio_PlaySoundGeneral(0x2846 as libc::c_int as u16_0,
                                               &mut D_801333D4,
                                               4 as libc::c_int as u8_0,
                                               &mut D_801333E0,
                                               &mut D_801333E0,
                                               &mut D_801333E8);
                    } else if temp == 0.0f32 &&
                                  (*globalCtx).sceneNum as libc::c_int ==
                                      SCENE_GANONTIKA as libc::c_int {
                        func_800788CC(0x2846 as libc::c_int as u16_0);
                    }
                } else {
                    (*globalCtx).envCtx.screenFillColor[3 as libc::c_int as
                                                            usize] =
                        ((1.0f32 - temp) * 255.0f32) as u8_0
                }
            }
            2 | 6 => {
                (*globalCtx).envCtx.screenFillColor[0 as libc::c_int as usize]
                    = 0 as libc::c_int as u8_0;
                (*globalCtx).envCtx.screenFillColor[1 as libc::c_int as usize]
                    = 0 as libc::c_int as u8_0;
                (*globalCtx).envCtx.screenFillColor[2 as libc::c_int as usize]
                    = 255 as libc::c_int as u8_0;
                if (*cmd).base as libc::c_int == 2 as libc::c_int {
                    (*globalCtx).envCtx.screenFillColor[3 as libc::c_int as
                                                            usize] =
                        (255.0f32 * temp) as u8_0
                } else {
                    (*globalCtx).envCtx.screenFillColor[3 as libc::c_int as
                                                            usize] =
                        ((1.0f32 - temp) * 255.0f32) as u8_0
                }
            }
            3 | 7 => {
                (*globalCtx).envCtx.screenFillColor[0 as libc::c_int as usize]
                    = 255 as libc::c_int as u8_0;
                (*globalCtx).envCtx.screenFillColor[1 as libc::c_int as usize]
                    = 0 as libc::c_int as u8_0;
                (*globalCtx).envCtx.screenFillColor[2 as libc::c_int as usize]
                    = 0 as libc::c_int as u8_0;
                if (*cmd).base as libc::c_int == 3 as libc::c_int {
                    (*globalCtx).envCtx.screenFillColor[3 as libc::c_int as
                                                            usize] =
                        ((1.0f32 - temp) * 255.0f32) as u8_0
                } else {
                    (*globalCtx).envCtx.screenFillColor[3 as libc::c_int as
                                                            usize] =
                        (255.0f32 * temp) as u8_0
                }
            }
            4 | 8 => {
                (*globalCtx).envCtx.screenFillColor[0 as libc::c_int as usize]
                    = 0 as libc::c_int as u8_0;
                (*globalCtx).envCtx.screenFillColor[1 as libc::c_int as usize]
                    = 255 as libc::c_int as u8_0;
                (*globalCtx).envCtx.screenFillColor[2 as libc::c_int as usize]
                    = 0 as libc::c_int as u8_0;
                if (*cmd).base as libc::c_int == 4 as libc::c_int {
                    (*globalCtx).envCtx.screenFillColor[3 as libc::c_int as
                                                            usize] =
                        ((1.0f32 - temp) * 255.0f32) as u8_0
                } else {
                    (*globalCtx).envCtx.screenFillColor[3 as libc::c_int as
                                                            usize] =
                        (255.0f32 * temp) as u8_0
                }
            }
            9 => { gSaveContext.unk_1410 = 1 as libc::c_int as u8_0 }
            10 | 11 => {
                (*globalCtx).envCtx.screenFillColor[0 as libc::c_int as usize]
                    = 0 as libc::c_int as u8_0;
                (*globalCtx).envCtx.screenFillColor[1 as libc::c_int as usize]
                    = 0 as libc::c_int as u8_0;
                (*globalCtx).envCtx.screenFillColor[2 as libc::c_int as usize]
                    = 0 as libc::c_int as u8_0;
                if (*cmd).base as libc::c_int == 10 as libc::c_int {
                    (*globalCtx).envCtx.screenFillColor[3 as libc::c_int as
                                                            usize] =
                        ((1.0f32 - temp) * 255.0f32) as u8_0
                } else {
                    (*globalCtx).envCtx.screenFillColor[3 as libc::c_int as
                                                            usize] =
                        (255.0f32 * temp) as u8_0
                }
            }
            12 => {
                gSaveContext.unk_1410 = (255.0f32 - 155.0f32 * temp) as u8_0
            }
            13 => {
                (*globalCtx).envCtx.screenFillColor[0 as libc::c_int as usize]
                    = 0 as libc::c_int as u8_0;
                (*globalCtx).envCtx.screenFillColor[1 as libc::c_int as usize]
                    = 0 as libc::c_int as u8_0;
                (*globalCtx).envCtx.screenFillColor[2 as libc::c_int as usize]
                    = 0 as libc::c_int as u8_0;
                (*globalCtx).envCtx.screenFillColor[3 as libc::c_int as usize]
                    = (255.0f32 - (1.0f32 - temp) * 155.0f32) as u8_0
            }
            _ => { }
        }
    };
}
// Command 0x1 & 0x5: Camera Positions
#[no_mangle]
pub unsafe extern "C" fn Cutscene_Command_CameraPositions(mut globalCtx:
                                                              *mut GlobalContext,
                                                          mut csCtx:
                                                              *mut CutsceneContext,
                                                          mut cmd: *mut u8_0,
                                                          mut relativeToLink:
                                                              u8_0) -> s32 {
    let mut shouldContinue: s32 = 1 as libc::c_int;
    let mut cmdBase: *mut CsCmdBase = cmd as *mut CsCmdBase;
    let mut size: s32 = 0;
    cmd = cmd.offset(8 as libc::c_int as isize);
    size = 8 as libc::c_int;
    if ((*cmdBase).startFrame as libc::c_int) < (*csCtx).frames as libc::c_int
           &&
           ((*csCtx).frames as libc::c_int) <
               (*cmdBase).endFrame as libc::c_int &&
           (((*csCtx).unk_18 as libc::c_int) <
                (*cmdBase).startFrame as libc::c_int ||
                (*csCtx).unk_18 as libc::c_int >= 0xf000 as libc::c_int) {
        (*csCtx).unk_1B = 1 as libc::c_int as u8_0;
        (*csCtx).cameraPosition = cmd as *mut CutsceneCameraPoint;
        if (*csCtx).unk_1A as libc::c_int != 0 as libc::c_int {
            (*csCtx).unk_18 = (*cmdBase).startFrame;
            if D_8015FCC8 as libc::c_int != 0 as libc::c_int {
                Gameplay_CameraChangeSetting(globalCtx,
                                             (*csCtx).unk_14 as s16,
                                             CAM_SET_CS_0 as libc::c_int as
                                                 s16);
                Gameplay_ChangeCameraStatus(globalCtx, D_8015FCC6,
                                            1 as libc::c_int as s16);
                Gameplay_ChangeCameraStatus(globalCtx, (*csCtx).unk_14 as s16,
                                            7 as libc::c_int as s16);
                Camera_ResetAnim(Gameplay_GetCamera(globalCtx,
                                                    (*csCtx).unk_14 as s16));
                Camera_SetCSParams(Gameplay_GetCamera(globalCtx,
                                                      (*csCtx).unk_14 as s16),
                                   (*csCtx).cameraFocus,
                                   (*csCtx).cameraPosition,
                                   (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER
                                                                        as
                                                                        libc::c_int
                                                                        as
                                                                        usize].head
                                       as *mut Player, relativeToLink as s16);
            }
        }
    }
    while shouldContinue != 0 {
        if (*(cmd as *mut CutsceneCameraPoint)).continueFlag as libc::c_int ==
               -(1 as libc::c_int) {
            shouldContinue = 0 as libc::c_int
        }
        cmd = cmd.offset(0x10 as libc::c_int as isize);
        size += 0x10 as libc::c_int
    }
    return size;
}
// Command 0x2 & 0x6: Camera Focus Points
#[no_mangle]
pub unsafe extern "C" fn Cutscene_Command_CameraFocus(mut globalCtx:
                                                          *mut GlobalContext,
                                                      mut csCtx:
                                                          *mut CutsceneContext,
                                                      mut cmd: *mut u8_0,
                                                      mut relativeToLink:
                                                          u8_0) -> s32 {
    let mut shouldContinue: s32 = 1 as libc::c_int;
    let mut cmdBase: *mut CsCmdBase = cmd as *mut CsCmdBase;
    let mut size: s32 = 0;
    cmd = cmd.offset(8 as libc::c_int as isize);
    size = 8 as libc::c_int;
    if ((*cmdBase).startFrame as libc::c_int) < (*csCtx).frames as libc::c_int
           &&
           ((*csCtx).frames as libc::c_int) <
               (*cmdBase).endFrame as libc::c_int &&
           ((D_8015FCC0 as libc::c_int) < (*cmdBase).startFrame as libc::c_int
                || D_8015FCC0 as libc::c_int >= 0xf000 as libc::c_int) {
        (*csCtx).unk_1A = 1 as libc::c_int as u8_0;
        (*csCtx).cameraFocus = cmd as *mut CutsceneCameraPoint;
        if (*csCtx).unk_1B as libc::c_int != 0 as libc::c_int {
            D_8015FCC0 = (*cmdBase).startFrame;
            if D_8015FCC8 as libc::c_int != 0 as libc::c_int {
                Gameplay_CameraChangeSetting(globalCtx,
                                             (*csCtx).unk_14 as s16,
                                             CAM_SET_CS_0 as libc::c_int as
                                                 s16);
                Gameplay_ChangeCameraStatus(globalCtx, D_8015FCC6,
                                            1 as libc::c_int as s16);
                Gameplay_ChangeCameraStatus(globalCtx, (*csCtx).unk_14 as s16,
                                            7 as libc::c_int as s16);
                Camera_ResetAnim(Gameplay_GetCamera(globalCtx,
                                                    (*csCtx).unk_14 as s16));
                Camera_SetCSParams(Gameplay_GetCamera(globalCtx,
                                                      (*csCtx).unk_14 as s16),
                                   (*csCtx).cameraFocus,
                                   (*csCtx).cameraPosition,
                                   (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER
                                                                        as
                                                                        libc::c_int
                                                                        as
                                                                        usize].head
                                       as *mut Player, relativeToLink as s16);
            }
        }
    }
    while shouldContinue != 0 {
        if (*(cmd as *mut CutsceneCameraPoint)).continueFlag as libc::c_int ==
               -(1 as libc::c_int) {
            shouldContinue = 0 as libc::c_int
        }
        cmd = cmd.offset(0x10 as libc::c_int as isize);
        size += 0x10 as libc::c_int
    }
    return size;
}
// Command 0x7: ? (Related to camera positons)
#[no_mangle]
pub unsafe extern "C" fn Cutscene_Command_07(mut globalCtx:
                                                 *mut GlobalContext,
                                             mut csCtx: *mut CutsceneContext,
                                             mut cmd: *mut u8_0,
                                             mut unused: u8_0) -> s32 {
    let mut cmdBase: *mut CsCmdBase = cmd as *mut CsCmdBase;
    let mut size: s32 = 0;
    let mut sp3C: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp30: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp2C: *mut Camera = 0 as *mut Camera;
    let mut sp28: f32_0 = 0.;
    cmd = cmd.offset(8 as libc::c_int as isize);
    size = 8 as libc::c_int;
    if ((*cmdBase).startFrame as libc::c_int) < (*csCtx).frames as libc::c_int
           &&
           ((*csCtx).frames as libc::c_int) <
               (*cmdBase).endFrame as libc::c_int &&
           ((D_8015FCC2 as libc::c_int) < (*cmdBase).startFrame as libc::c_int
                || D_8015FCC2 as libc::c_int >= 0xf000 as libc::c_int) {
        (*csCtx).unk_1B = 1 as libc::c_int as u8_0;
        (*csCtx).cameraPosition = cmd as *mut CutsceneCameraPoint;
        if (*csCtx).unk_1A as libc::c_int != 0 as libc::c_int {
            D_8015FCC2 = (*cmdBase).startFrame;
            if D_8015FCC8 as libc::c_int != 0 as libc::c_int {
                sp2C = Gameplay_GetCamera(globalCtx, (*csCtx).unk_14 as s16);
                (*sp2C).player = 0 as *mut Player;
                Gameplay_ChangeCameraStatus(globalCtx,
                                            0 as libc::c_int as s16,
                                            1 as libc::c_int as s16);
                Gameplay_ChangeCameraStatus(globalCtx, (*csCtx).unk_14 as s16,
                                            7 as libc::c_int as s16);
                Gameplay_CameraChangeSetting(globalCtx,
                                             (*csCtx).unk_14 as s16,
                                             CAM_SET_FREE0 as libc::c_int as
                                                 s16);
                sp28 =
                    (*(*csCtx).cameraFocus).cameraRoll as libc::c_int as
                        libc::c_float * 1.40625f32;
                Camera_SetParam(sp2C, 64 as libc::c_int,
                                &mut sp28 as *mut f32_0 as *mut libc::c_void);
                sp3C.x = (*(*csCtx).cameraFocus).pos.x as f32_0;
                sp3C.y = (*(*csCtx).cameraFocus).pos.y as f32_0;
                sp3C.z = (*(*csCtx).cameraFocus).pos.z as f32_0;
                sp30.x = (*(*csCtx).cameraPosition).pos.x as f32_0;
                sp30.y = (*(*csCtx).cameraPosition).pos.y as f32_0;
                sp30.z = (*(*csCtx).cameraPosition).pos.z as f32_0;
                Gameplay_CameraSetAtEye(globalCtx, (*csCtx).unk_14 as s16,
                                        &mut sp3C, &mut sp30);
                Gameplay_CameraSetFov(globalCtx, (*csCtx).unk_14 as s16,
                                      (*(*csCtx).cameraPosition).viewAngle);
            }
        }
    }
    size += 0x10 as libc::c_int;
    return size;
}
// Command 0x8: ? (Related to camera focus points)
#[no_mangle]
pub unsafe extern "C" fn Cutscene_Command_08(mut globalCtx:
                                                 *mut GlobalContext,
                                             mut csCtx: *mut CutsceneContext,
                                             mut cmd: *mut u8_0,
                                             mut unused: u8_0) -> s32 {
    let mut cmdBase: *mut CsCmdBase = cmd as *mut CsCmdBase;
    let mut size: s32 = 0;
    let mut sp3C: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp30: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp2C: *mut Camera = 0 as *mut Camera;
    let mut sp28: f32_0 = 0.;
    cmd = cmd.offset(8 as libc::c_int as isize);
    size = 8 as libc::c_int;
    if ((*cmdBase).startFrame as libc::c_int) < (*csCtx).frames as libc::c_int
           &&
           ((*csCtx).frames as libc::c_int) <
               (*cmdBase).endFrame as libc::c_int &&
           ((D_8015FCC4 as libc::c_int) < (*cmdBase).startFrame as libc::c_int
                || D_8015FCC4 as libc::c_int >= 0xf000 as libc::c_int) {
        (*csCtx).unk_1A = 1 as libc::c_int as u8_0;
        (*csCtx).cameraFocus = cmd as *mut CutsceneCameraPoint;
        if (*csCtx).unk_1B as libc::c_int != 0 as libc::c_int {
            D_8015FCC4 = (*cmdBase).startFrame;
            if D_8015FCC8 as libc::c_int != 0 as libc::c_int {
                sp2C = Gameplay_GetCamera(globalCtx, (*csCtx).unk_14 as s16);
                (*sp2C).player = 0 as *mut Player;
                Gameplay_ChangeCameraStatus(globalCtx,
                                            0 as libc::c_int as s16,
                                            1 as libc::c_int as s16);
                Gameplay_ChangeCameraStatus(globalCtx, (*csCtx).unk_14 as s16,
                                            7 as libc::c_int as s16);
                Gameplay_CameraChangeSetting(globalCtx,
                                             (*csCtx).unk_14 as s16,
                                             CAM_SET_FREE0 as libc::c_int as
                                                 s16);
                sp3C.x = (*(*csCtx).cameraFocus).pos.x as f32_0;
                sp3C.y = (*(*csCtx).cameraFocus).pos.y as f32_0;
                sp3C.z = (*(*csCtx).cameraFocus).pos.z as f32_0;
                sp30.x = (*(*csCtx).cameraPosition).pos.x as f32_0;
                sp30.y = (*(*csCtx).cameraPosition).pos.y as f32_0;
                sp30.z = (*(*csCtx).cameraPosition).pos.z as f32_0;
                Gameplay_CameraSetAtEye(globalCtx, (*csCtx).unk_14 as s16,
                                        &mut sp3C, &mut sp30);
                Gameplay_CameraSetFov(globalCtx, (*csCtx).unk_14 as s16,
                                      (*(*csCtx).cameraPosition).viewAngle);
            }
        }
    }
    size += 0x10 as libc::c_int;
    return size;
}
// Command 0x13: Textbox
#[no_mangle]
pub unsafe extern "C" fn Cutscene_Command_Textbox(mut globalCtx:
                                                      *mut GlobalContext,
                                                  mut csCtx:
                                                      *mut CutsceneContext,
                                                  mut cmd:
                                                      *mut CsCmdTextbox) {
    let mut dialogState: u8_0 = 0;
    let mut originalCsFrames: s16 = 0;
    if ((*cmd).startFrame as libc::c_int) < (*csCtx).frames as libc::c_int &&
           (*csCtx).frames as libc::c_int <= (*cmd).endFrame as libc::c_int {
        if (*cmd).type_0 as libc::c_int != 2 as libc::c_int {
            if D_8011E1C0 as libc::c_int != (*cmd).base as libc::c_int {
                D_8011E1C0 = (*cmd).base;
                if (*cmd).type_0 as libc::c_int == 3 as libc::c_int &&
                       gBitFlags[QUEST_ZORA_SAPPHIRE as libc::c_int as usize]
                           & gSaveContext.inventory.questItems != 0 {
                    Message_StartTextbox(globalCtx, (*cmd).textId1,
                                         0 as *mut Actor);
                } else if (*cmd).type_0 as libc::c_int == 4 as libc::c_int &&
                              gBitFlags[QUEST_GORON_RUBY as libc::c_int as
                                            usize] &
                                  gSaveContext.inventory.questItems != 0 {
                    Message_StartTextbox(globalCtx, (*cmd).textId1,
                                         0 as *mut Actor);
                } else {
                    Message_StartTextbox(globalCtx, (*cmd).base,
                                         0 as *mut Actor);
                }
                return
            }
        } else if D_8011E1C4 as libc::c_int != (*cmd).base as libc::c_int {
            D_8011E1C4 = (*cmd).base;
            func_8010BD58(globalCtx, (*cmd).base);
            return
        }
        if (*csCtx).frames as libc::c_int >= (*cmd).endFrame as libc::c_int {
            originalCsFrames = (*csCtx).frames as s16;
            dialogState = Message_GetState(&mut (*globalCtx).msgCtx);
            if dialogState as libc::c_int != TEXT_STATE_CLOSING as libc::c_int
                   &&
                   dialogState as libc::c_int !=
                       TEXT_STATE_NONE as libc::c_int &&
                   dialogState as libc::c_int !=
                       TEXT_STATE_SONG_DEMO_DONE as libc::c_int &&
                   dialogState as libc::c_int != TEXT_STATE_8 as libc::c_int {
                (*csCtx).frames = (*csCtx).frames.wrapping_sub(1);
                if dialogState as libc::c_int ==
                       TEXT_STATE_CHOICE as libc::c_int &&
                       Message_ShouldAdvance(globalCtx) as libc::c_int != 0 {
                    if (*globalCtx).msgCtx.choiceIndex as libc::c_int ==
                           0 as libc::c_int {
                        if (*cmd).textId1 as libc::c_int !=
                               0xffff as libc::c_int {
                            Message_ContinueTextbox(globalCtx,
                                                    (*cmd).textId1);
                        } else {
                            (*csCtx).frames = (*csCtx).frames.wrapping_add(1)
                        }
                    } else if (*cmd).textId2 as libc::c_int !=
                                  0xffff as libc::c_int {
                        Message_ContinueTextbox(globalCtx, (*cmd).textId2);
                    } else {
                        (*csCtx).frames = (*csCtx).frames.wrapping_add(1)
                    }
                }
                if dialogState as libc::c_int == TEXT_STATE_9 as libc::c_int {
                    if (*cmd).textId1 as libc::c_int != 0xffff as libc::c_int
                       {
                        Message_ContinueTextbox(globalCtx, (*cmd).textId1);
                    } else {
                        (*csCtx).frames = (*csCtx).frames.wrapping_add(1)
                    }
                }
                if dialogState as libc::c_int ==
                       TEXT_STATE_EVENT as libc::c_int {
                    if Message_ShouldAdvance(globalCtx) != 0 {
                        func_8010BD58(globalCtx, (*cmd).base);
                    }
                }
            }
            if (*csCtx).frames as libc::c_int ==
                   originalCsFrames as libc::c_int {
                Interface_ChangeAlpha(1 as libc::c_int as u16_0);
                D_8011E1C0 = 0 as libc::c_int as u16_0;
                D_8011E1C4 = 0 as libc::c_int as u16_0
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Cutscene_ProcessCommands(mut globalCtx:
                                                      *mut GlobalContext,
                                                  mut csCtx:
                                                      *mut CutsceneContext,
                                                  mut cutscenePtr:
                                                      *mut u8_0) {
    let mut i: s16 = 0;
    let mut totalEntries: s32 = 0;
    let mut cmdType: s32 = 0;
    let mut cmdEntries: s32 = 0;
    let mut cmd: *mut CsCmdBase = 0 as *mut CsCmdBase;
    let mut cutsceneEndFrame: s32 = 0;
    let mut j: s16 = 0;
    MemCopy(&mut totalEntries as *mut s32 as *mut libc::c_void,
            cutscenePtr as *mut libc::c_void, 4 as libc::c_int);
    cutscenePtr = cutscenePtr.offset(4 as libc::c_int as isize);
    MemCopy(&mut cutsceneEndFrame as *mut s32 as *mut libc::c_void,
            cutscenePtr as *mut libc::c_void, 4 as libc::c_int);
    cutscenePtr = cutscenePtr.offset(4 as libc::c_int as isize);
    if cutsceneEndFrame < (*csCtx).frames as libc::c_int &&
           (*csCtx).state as libc::c_int !=
               CS_STATE_UNSKIPPABLE_EXEC as libc::c_int {
        (*csCtx).state = CS_STATE_UNSKIPPABLE_INIT as libc::c_int as u8_0;
        return
    }
    if !((*globalCtx).state.input[0 as libc::c_int as usize].press.button as
             libc::c_int | !(0x100 as libc::c_int)) == 0 as libc::c_int {
        (*csCtx).state = CS_STATE_UNSKIPPABLE_INIT as libc::c_int as u8_0;
        return
    }
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < totalEntries {
        MemCopy(&mut cmdType as *mut s32 as *mut libc::c_void,
                cutscenePtr as *mut libc::c_void, 4 as libc::c_int);
        cutscenePtr = cutscenePtr.offset(4 as libc::c_int as isize);
        if cmdType == -(1 as libc::c_int) { return }
        match cmdType {
            3 => {
                MemCopy(&mut cmdEntries as *mut s32 as *mut libc::c_void,
                        cutscenePtr as *mut libc::c_void, 4 as libc::c_int);
                cutscenePtr = cutscenePtr.offset(4 as libc::c_int as isize);
                j = 0 as libc::c_int as s16;
                while (j as libc::c_int) < cmdEntries {
                    func_80064824(globalCtx, csCtx,
                                  cutscenePtr as *mut libc::c_void as
                                      *mut CsCmdBase);
                    cutscenePtr =
                        cutscenePtr.offset(0x30 as libc::c_int as isize);
                    j += 1
                }
            }
            4 => {
                MemCopy(&mut cmdEntries as *mut s32 as *mut libc::c_void,
                        cutscenePtr as *mut libc::c_void, 4 as libc::c_int);
                cutscenePtr = cutscenePtr.offset(4 as libc::c_int as isize);
                j = 0 as libc::c_int as s16;
                while (j as libc::c_int) < cmdEntries {
                    Cutscene_Command_SetLighting(globalCtx, csCtx,
                                                 cutscenePtr as
                                                     *mut libc::c_void as
                                                     *mut CsCmdEnvLighting);
                    cutscenePtr =
                        cutscenePtr.offset(0x30 as libc::c_int as isize);
                    j += 1
                }
            }
            86 => {
                MemCopy(&mut cmdEntries as *mut s32 as *mut libc::c_void,
                        cutscenePtr as *mut libc::c_void, 4 as libc::c_int);
                cutscenePtr = cutscenePtr.offset(4 as libc::c_int as isize);
                j = 0 as libc::c_int as s16;
                while (j as libc::c_int) < cmdEntries {
                    Cutscene_Command_PlayBGM(globalCtx, csCtx,
                                             cutscenePtr as *mut libc::c_void
                                                 as *mut CsCmdMusicChange);
                    cutscenePtr =
                        cutscenePtr.offset(0x30 as libc::c_int as isize);
                    j += 1
                }
            }
            87 => {
                MemCopy(&mut cmdEntries as *mut s32 as *mut libc::c_void,
                        cutscenePtr as *mut libc::c_void, 4 as libc::c_int);
                cutscenePtr = cutscenePtr.offset(4 as libc::c_int as isize);
                j = 0 as libc::c_int as s16;
                while (j as libc::c_int) < cmdEntries {
                    Cutscene_Command_StopBGM(globalCtx, csCtx,
                                             cutscenePtr as *mut libc::c_void
                                                 as *mut CsCmdMusicChange);
                    cutscenePtr =
                        cutscenePtr.offset(0x30 as libc::c_int as isize);
                    j += 1
                }
            }
            124 => {
                MemCopy(&mut cmdEntries as *mut s32 as *mut libc::c_void,
                        cutscenePtr as *mut libc::c_void, 4 as libc::c_int);
                cutscenePtr = cutscenePtr.offset(4 as libc::c_int as isize);
                j = 0 as libc::c_int as s16;
                while (j as libc::c_int) < cmdEntries {
                    Cutscene_Command_FadeBGM(globalCtx, csCtx,
                                             cutscenePtr as *mut libc::c_void
                                                 as *mut CsCmdMusicFade);
                    cutscenePtr =
                        cutscenePtr.offset(0x30 as libc::c_int as isize);
                    j += 1
                }
            }
            9 => {
                MemCopy(&mut cmdEntries as *mut s32 as *mut libc::c_void,
                        cutscenePtr as *mut libc::c_void, 4 as libc::c_int);
                cutscenePtr = cutscenePtr.offset(4 as libc::c_int as isize);
                j = 0 as libc::c_int as s16;
                while (j as libc::c_int) < cmdEntries {
                    Cutscene_Command_09(globalCtx, csCtx,
                                        cutscenePtr as *mut libc::c_void as
                                            *mut CsCmdUnknown9);
                    cutscenePtr =
                        cutscenePtr.offset(0xc as libc::c_int as isize);
                    j += 1
                }
            }
            140 => {
                MemCopy(&mut cmdEntries as *mut s32 as *mut libc::c_void,
                        cutscenePtr as *mut libc::c_void, 4 as libc::c_int);
                cutscenePtr = cutscenePtr.offset(4 as libc::c_int as isize);
                j = 0 as libc::c_int as s16;
                while (j as libc::c_int) < cmdEntries {
                    func_80065134(globalCtx, csCtx,
                                  cutscenePtr as *mut libc::c_void as
                                      *mut CsCmdDayTime);
                    cutscenePtr =
                        cutscenePtr.offset(0xc as libc::c_int as isize);
                    j += 1
                }
            }
            10 => {
                MemCopy(&mut cmdEntries as *mut s32 as *mut libc::c_void,
                        cutscenePtr as *mut libc::c_void, 4 as libc::c_int);
                cutscenePtr = cutscenePtr.offset(4 as libc::c_int as isize);
                j = 0 as libc::c_int as s16;
                while (j as libc::c_int) < cmdEntries {
                    cmd = cutscenePtr as *mut CsCmdBase;
                    if ((*cmd).startFrame as libc::c_int) <
                           (*csCtx).frames as libc::c_int &&
                           (*csCtx).frames as libc::c_int <=
                               (*cmd).endFrame as libc::c_int {
                        (*csCtx).linkAction =
                            cutscenePtr as *mut libc::c_void as
                                *mut CsCmdActorAction
                    }
                    cutscenePtr =
                        cutscenePtr.offset(0x30 as libc::c_int as isize);
                    j += 1
                }
            }
            15 | 17 | 18 | 23 | 34 | 39 | 46 | 76 | 85 | 93 | 105 | 107 | 110
            | 119 | 123 | 138 | 139 | 144 => {
                MemCopy(&mut cmdEntries as *mut s32 as *mut libc::c_void,
                        cutscenePtr as *mut libc::c_void, 4 as libc::c_int);
                cutscenePtr = cutscenePtr.offset(4 as libc::c_int as isize);
                j = 0 as libc::c_int as s16;
                while (j as libc::c_int) < cmdEntries {
                    cmd = cutscenePtr as *mut CsCmdBase;
                    if ((*cmd).startFrame as libc::c_int) <
                           (*csCtx).frames as libc::c_int &&
                           (*csCtx).frames as libc::c_int <=
                               (*cmd).endFrame as libc::c_int {
                        (*csCtx).npcActions[0 as libc::c_int as usize] =
                            cutscenePtr as *mut libc::c_void as
                                *mut CsCmdActorAction
                    }
                    cutscenePtr =
                        cutscenePtr.offset(0x30 as libc::c_int as isize);
                    j += 1
                }
            }
            14 | 16 | 24 | 35 | 40 | 48 | 64 | 68 | 70 | 78 | 80 | 94 | 116 |
            118 | 120 | 125 | 131 | 141 => {
                MemCopy(&mut cmdEntries as *mut s32 as *mut libc::c_void,
                        cutscenePtr as *mut libc::c_void, 4 as libc::c_int);
                cutscenePtr = cutscenePtr.offset(4 as libc::c_int as isize);
                j = 0 as libc::c_int as s16;
                while (j as libc::c_int) < cmdEntries {
                    cmd = cutscenePtr as *mut CsCmdBase;
                    if ((*cmd).startFrame as libc::c_int) <
                           (*csCtx).frames as libc::c_int &&
                           (*csCtx).frames as libc::c_int <=
                               (*cmd).endFrame as libc::c_int {
                        (*csCtx).npcActions[1 as libc::c_int as usize] =
                            cutscenePtr as *mut libc::c_void as
                                *mut CsCmdActorAction
                    }
                    cutscenePtr =
                        cutscenePtr.offset(0x30 as libc::c_int as isize);
                    j += 1
                }
            }
            25 | 36 | 41 | 50 | 67 | 69 | 72 | 74 | 81 | 106 | 117 | 121 | 126
            | 132 => {
                MemCopy(&mut cmdEntries as *mut s32 as *mut libc::c_void,
                        cutscenePtr as *mut libc::c_void, 4 as libc::c_int);
                cutscenePtr = cutscenePtr.offset(4 as libc::c_int as isize);
                j = 0 as libc::c_int as s16;
                while (j as libc::c_int) < cmdEntries {
                    cmd = cutscenePtr as *mut CsCmdBase;
                    if ((*cmd).startFrame as libc::c_int) <
                           (*csCtx).frames as libc::c_int &&
                           (*csCtx).frames as libc::c_int <=
                               (*cmd).endFrame as libc::c_int {
                        (*csCtx).npcActions[2 as libc::c_int as usize] =
                            cutscenePtr as *mut libc::c_void as
                                *mut CsCmdActorAction
                    }
                    cutscenePtr =
                        cutscenePtr.offset(0x30 as libc::c_int as isize);
                    j += 1
                }
            }
            29 | 37 | 42 | 51 | 53 | 63 | 65 | 66 | 75 | 82 | 108 | 127 | 133
            => {
                MemCopy(&mut cmdEntries as *mut s32 as *mut libc::c_void,
                        cutscenePtr as *mut libc::c_void, 4 as libc::c_int);
                cutscenePtr = cutscenePtr.offset(4 as libc::c_int as isize);
                j = 0 as libc::c_int as s16;
                while (j as libc::c_int) < cmdEntries {
                    cmd = cutscenePtr as *mut CsCmdBase;
                    if ((*cmd).startFrame as libc::c_int) <
                           (*csCtx).frames as libc::c_int &&
                           (*csCtx).frames as libc::c_int <=
                               (*cmd).endFrame as libc::c_int {
                        (*csCtx).npcActions[3 as libc::c_int as usize] =
                            cutscenePtr as *mut libc::c_void as
                                *mut CsCmdActorAction
                    }
                    cutscenePtr =
                        cutscenePtr.offset(0x30 as libc::c_int as isize);
                    j += 1
                }
            }
            30 | 38 | 43 | 47 | 54 | 79 | 83 | 128 | 135 => {
                MemCopy(&mut cmdEntries as *mut s32 as *mut libc::c_void,
                        cutscenePtr as *mut libc::c_void, 4 as libc::c_int);
                cutscenePtr = cutscenePtr.offset(4 as libc::c_int as isize);
                j = 0 as libc::c_int as s16;
                while (j as libc::c_int) < cmdEntries {
                    cmd = cutscenePtr as *mut CsCmdBase;
                    if ((*cmd).startFrame as libc::c_int) <
                           (*csCtx).frames as libc::c_int &&
                           (*csCtx).frames as libc::c_int <=
                               (*cmd).endFrame as libc::c_int {
                        (*csCtx).npcActions[4 as libc::c_int as usize] =
                            cutscenePtr as *mut libc::c_void as
                                *mut CsCmdActorAction
                    }
                    cutscenePtr =
                        cutscenePtr.offset(0x30 as libc::c_int as isize);
                    j += 1
                }
            }
            44 | 55 | 77 | 84 | 90 | 129 | 136 => {
                MemCopy(&mut cmdEntries as *mut s32 as *mut libc::c_void,
                        cutscenePtr as *mut libc::c_void, 4 as libc::c_int);
                cutscenePtr = cutscenePtr.offset(4 as libc::c_int as isize);
                j = 0 as libc::c_int as s16;
                while (j as libc::c_int) < cmdEntries {
                    cmd = cutscenePtr as *mut CsCmdBase;
                    if ((*cmd).startFrame as libc::c_int) <
                           (*csCtx).frames as libc::c_int &&
                           (*csCtx).frames as libc::c_int <=
                               (*cmd).endFrame as libc::c_int {
                        (*csCtx).npcActions[5 as libc::c_int as usize] =
                            cutscenePtr as *mut libc::c_void as
                                *mut CsCmdActorAction
                    }
                    cutscenePtr =
                        cutscenePtr.offset(0x30 as libc::c_int as isize);
                    j += 1
                }
            }
            31 | 52 | 57 | 58 | 88 | 115 | 130 | 137 => {
                MemCopy(&mut cmdEntries as *mut s32 as *mut libc::c_void,
                        cutscenePtr as *mut libc::c_void, 4 as libc::c_int);
                cutscenePtr = cutscenePtr.offset(4 as libc::c_int as isize);
                j = 0 as libc::c_int as s16;
                while (j as libc::c_int) < cmdEntries {
                    cmd = cutscenePtr as *mut CsCmdBase;
                    if ((*cmd).startFrame as libc::c_int) <
                           (*csCtx).frames as libc::c_int &&
                           (*csCtx).frames as libc::c_int <=
                               (*cmd).endFrame as libc::c_int {
                        (*csCtx).npcActions[6 as libc::c_int as usize] =
                            cutscenePtr as *mut libc::c_void as
                                *mut CsCmdActorAction
                    }
                    cutscenePtr =
                        cutscenePtr.offset(0x30 as libc::c_int as isize);
                    j += 1
                }
            }
            49 | 60 | 89 | 111 | 114 | 134 | 142 => {
                MemCopy(&mut cmdEntries as *mut s32 as *mut libc::c_void,
                        cutscenePtr as *mut libc::c_void, 4 as libc::c_int);
                cutscenePtr = cutscenePtr.offset(4 as libc::c_int as isize);
                j = 0 as libc::c_int as s16;
                while (j as libc::c_int) < cmdEntries {
                    cmd = cutscenePtr as *mut CsCmdBase;
                    if ((*cmd).startFrame as libc::c_int) <
                           (*csCtx).frames as libc::c_int &&
                           (*csCtx).frames as libc::c_int <=
                               (*cmd).endFrame as libc::c_int {
                        (*csCtx).npcActions[7 as libc::c_int as usize] =
                            cutscenePtr as *mut libc::c_void as
                                *mut CsCmdActorAction
                    }
                    cutscenePtr =
                        cutscenePtr.offset(0x30 as libc::c_int as isize);
                    j += 1
                }
            }
            62 => {
                MemCopy(&mut cmdEntries as *mut s32 as *mut libc::c_void,
                        cutscenePtr as *mut libc::c_void, 4 as libc::c_int);
                cutscenePtr = cutscenePtr.offset(4 as libc::c_int as isize);
                j = 0 as libc::c_int as s16;
                while (j as libc::c_int) < cmdEntries {
                    cmd = cutscenePtr as *mut CsCmdBase;
                    if ((*cmd).startFrame as libc::c_int) <
                           (*csCtx).frames as libc::c_int &&
                           (*csCtx).frames as libc::c_int <=
                               (*cmd).endFrame as libc::c_int {
                        (*csCtx).npcActions[8 as libc::c_int as usize] =
                            cutscenePtr as *mut libc::c_void as
                                *mut CsCmdActorAction
                    }
                    cutscenePtr =
                        cutscenePtr.offset(0x30 as libc::c_int as isize);
                    j += 1
                }
            }
            143 => {
                MemCopy(&mut cmdEntries as *mut s32 as *mut libc::c_void,
                        cutscenePtr as *mut libc::c_void, 4 as libc::c_int);
                cutscenePtr = cutscenePtr.offset(4 as libc::c_int as isize);
                j = 0 as libc::c_int as s16;
                while (j as libc::c_int) < cmdEntries {
                    cmd = cutscenePtr as *mut CsCmdBase;
                    if ((*cmd).startFrame as libc::c_int) <
                           (*csCtx).frames as libc::c_int &&
                           (*csCtx).frames as libc::c_int <=
                               (*cmd).endFrame as libc::c_int {
                        (*csCtx).npcActions[9 as libc::c_int as usize] =
                            cutscenePtr as *mut libc::c_void as
                                *mut CsCmdActorAction
                    }
                    cutscenePtr =
                        cutscenePtr.offset(0x30 as libc::c_int as isize);
                    j += 1
                }
            }
            1 => {
                cutscenePtr =
                    cutscenePtr.offset(Cutscene_Command_CameraPositions(globalCtx,
                                                                        csCtx,
                                                                        cutscenePtr
                                                                            as
                                                                            *mut libc::c_void
                                                                            as
                                                                            *mut u8_0,
                                                                        0 as
                                                                            libc::c_int
                                                                            as
                                                                            u8_0)
                                           as isize)
            }
            5 => {
                cutscenePtr =
                    cutscenePtr.offset(Cutscene_Command_CameraPositions(globalCtx,
                                                                        csCtx,
                                                                        cutscenePtr
                                                                            as
                                                                            *mut libc::c_void
                                                                            as
                                                                            *mut u8_0,
                                                                        1 as
                                                                            libc::c_int
                                                                            as
                                                                            u8_0)
                                           as isize)
            }
            2 => {
                cutscenePtr =
                    cutscenePtr.offset(Cutscene_Command_CameraFocus(globalCtx,
                                                                    csCtx,
                                                                    cutscenePtr
                                                                        as
                                                                        *mut libc::c_void
                                                                        as
                                                                        *mut u8_0,
                                                                    0 as
                                                                        libc::c_int
                                                                        as
                                                                        u8_0)
                                           as isize)
            }
            6 => {
                cutscenePtr =
                    cutscenePtr.offset(Cutscene_Command_CameraFocus(globalCtx,
                                                                    csCtx,
                                                                    cutscenePtr
                                                                        as
                                                                        *mut libc::c_void
                                                                        as
                                                                        *mut u8_0,
                                                                    1 as
                                                                        libc::c_int
                                                                        as
                                                                        u8_0)
                                           as isize)
            }
            7 => {
                cutscenePtr =
                    cutscenePtr.offset(Cutscene_Command_07(globalCtx, csCtx,
                                                           cutscenePtr as
                                                               *mut libc::c_void
                                                               as *mut u8_0,
                                                           0 as libc::c_int as
                                                               u8_0) as isize)
            }
            8 => {
                cutscenePtr =
                    cutscenePtr.offset(Cutscene_Command_08(globalCtx, csCtx,
                                                           cutscenePtr as
                                                               *mut libc::c_void
                                                               as *mut u8_0,
                                                           0 as libc::c_int as
                                                               u8_0) as isize)
            }
            1000 => {
                cutscenePtr = cutscenePtr.offset(4 as libc::c_int as isize);
                Cutscene_Command_Terminator(globalCtx, csCtx,
                                            cutscenePtr as *mut libc::c_void
                                                as *mut CsCmdBase);
                cutscenePtr = cutscenePtr.offset(8 as libc::c_int as isize)
            }
            19 => {
                MemCopy(&mut cmdEntries as *mut s32 as *mut libc::c_void,
                        cutscenePtr as *mut libc::c_void, 4 as libc::c_int);
                cutscenePtr = cutscenePtr.offset(4 as libc::c_int as isize);
                j = 0 as libc::c_int as s16;
                while (j as libc::c_int) < cmdEntries {
                    cmd = cutscenePtr as *mut CsCmdBase;
                    if (*cmd).base as libc::c_int != 0xffff as libc::c_int {
                        Cutscene_Command_Textbox(globalCtx, csCtx,
                                                 cutscenePtr as
                                                     *mut libc::c_void as
                                                     *mut CsCmdTextbox);
                    }
                    cutscenePtr =
                        cutscenePtr.offset(0xc as libc::c_int as isize);
                    j += 1
                }
            }
            45 => {
                cutscenePtr = cutscenePtr.offset(4 as libc::c_int as isize);
                Cutscene_Command_TransitionFX(globalCtx, csCtx,
                                              cutscenePtr as *mut libc::c_void
                                                  as *mut CsCmdBase);
                cutscenePtr = cutscenePtr.offset(8 as libc::c_int as isize)
            }
            _ => {
                MemCopy(&mut cmdEntries as *mut s32 as *mut libc::c_void,
                        cutscenePtr as *mut libc::c_void, 4 as libc::c_int);
                cutscenePtr = cutscenePtr.offset(4 as libc::c_int as isize);
                j = 0 as libc::c_int as s16;
                while (j as libc::c_int) < cmdEntries {
                    cutscenePtr =
                        cutscenePtr.offset(0x30 as libc::c_int as isize);
                    j += 1
                }
            }
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_80068C3C(mut globalCtx: *mut GlobalContext,
                                       mut csCtx: *mut CutsceneContext) {
    let mut displayList: *mut Gfx = 0 as *mut Gfx;
    let mut prevDisplayList: *mut Gfx = 0 as *mut Gfx;
    // Necessary to match
    if gSaveContext.cutsceneIndex >= 0xfff0 as libc::c_int {
        // Also necessary to match
        if (*gGameInfo).data[(25 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 0 as libc::c_int) as
                                 usize] as libc::c_int != 0 as libc::c_int {
            let mut __gfxCtx: *mut GraphicsContext =
                0 as *mut GraphicsContext; // "Right here, huh"
            let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
            __gfxCtx = (*globalCtx).state.gfxCtx;
            Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                            b"../z_demo.c\x00" as *const u8 as
                                *const libc::c_char, 4101 as libc::c_int);
            prevDisplayList = (*__gfxCtx).polyOpa.p;
            displayList = Graph_GfxPlusOne((*__gfxCtx).polyOpa.p);
            let fresh0 = (*__gfxCtx).overlay.p;
            (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
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
            (*_g).words.w1 = displayList as libc::c_uint;
            Cutscene_DrawDebugInfo(globalCtx, &mut displayList, csCtx);
            let fresh1 = displayList;
            displayList = displayList.offset(1);
            let mut _g_0: *mut Gfx = fresh1;
            (*_g_0).words.w0 =
                (0xdf as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_0).words.w1 = 0 as libc::c_int as libc::c_uint;
            Graph_BranchDlist(prevDisplayList, displayList);
            (*__gfxCtx).polyOpa.p = displayList;
            Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                             b"../z_demo.c\x00" as *const u8 as
                                 *const libc::c_char, 4108 as libc::c_int);
        }
        (*csCtx).frames = (*csCtx).frames.wrapping_add(1);
        if (*gGameInfo).data[(26 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 95 as libc::c_int) as
                                 usize] as libc::c_int != 0 as libc::c_int {
            Cutscene_ProcessCommands(globalCtx, csCtx,
                                     D_8012D1F0 as *mut u8_0);
        } else {
            Cutscene_ProcessCommands(globalCtx, csCtx,
                                     (*globalCtx).csCtx.segment as *mut u8_0);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_80068D84(mut globalCtx: *mut GlobalContext,
                                       mut csCtx: *mut CutsceneContext) {
    if func_8006472C(globalCtx, csCtx, 0.0f32) != 0 {
        Audio_SetCutsceneFlag(0 as libc::c_int as s8);
        (*csCtx).state = CS_STATE_IDLE as libc::c_int as u8_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_80068DC0(mut globalCtx: *mut GlobalContext,
                                       mut csCtx: *mut CutsceneContext) {
    let mut i: s16 = 0;
    if func_8006472C(globalCtx, csCtx, 0.0f32) != 0 {
        (*csCtx).linkAction = 0 as *mut CsCmdActorAction;
        i = 0 as libc::c_int as s16;
        while (i as libc::c_int) < 10 as libc::c_int {
            (*csCtx).npcActions[i as usize] = 0 as *mut CsCmdActorAction;
            i += 1
        }
        osSyncPrintf(b"\n\n\n\n\n\xe3\x82\x84\xe3\x81\xa3\xe3\x81\xb1\xe3\x82\x8a\xe3\x81\x93\xe3\x81\x93\xe3\x81\x8b\xe3\x81\x84\xe3\x81\xaa\x00"
                         as *const u8 as *const libc::c_char);
        gSaveContext.cutsceneIndex = 0 as libc::c_int;
        gSaveContext.gameMode = 0 as libc::c_int;
        if D_8015FCC8 as libc::c_int != 0 as libc::c_int {
            match gSaveContext.entranceIndex {
                650 | 654 | 658 | 1142 => {
                    Gameplay_CopyCamera(globalCtx, D_8015FCC6,
                                        (*csCtx).unk_14 as s16);
                }
                _ => { }
            }
            Gameplay_ChangeCameraStatus(globalCtx, D_8015FCC6,
                                        7 as libc::c_int as s16);
            Gameplay_ClearCamera(globalCtx, (*csCtx).unk_14 as s16);
            func_8005B1A4((*globalCtx).cameraPtrs[D_8015FCC6 as usize]);
        }
        Audio_SetCutsceneFlag(0 as libc::c_int as s8);
        (*csCtx).state = CS_STATE_IDLE as libc::c_int as u8_0
    };
}
// only written to, never read
#[no_mangle]
pub unsafe extern "C" fn func_80068ECC(mut globalCtx: *mut GlobalContext,
                                       mut csCtx: *mut CutsceneContext) {
    let mut i: u8_0 = 0;
    if gSaveContext.cutsceneTrigger as libc::c_int != 0 as libc::c_int &&
           (*csCtx).state as libc::c_int == CS_STATE_IDLE as libc::c_int &&
           Player_InCsMode(globalCtx) == 0 {
        gSaveContext.cutsceneIndex = 0xfffd as libc::c_int
    }
    if gSaveContext.cutsceneIndex >= 0xfff0 as libc::c_int &&
           (*csCtx).state as libc::c_int == CS_STATE_IDLE as libc::c_int {
        Flags_UnsetEnv(globalCtx, 0 as libc::c_int as s16);
        D_8011E1C0 = 0 as libc::c_int as u16_0;
        D_8011E1C4 = 0 as libc::c_int as u16_0;
        (*csCtx).unk_12 = 0 as libc::c_int as u16_0;
        (*csCtx).linkAction = 0 as *mut CsCmdActorAction;
        i = 0 as libc::c_int as u8_0;
        while (i as libc::c_int) < 10 as libc::c_int {
            (*csCtx).npcActions[i as usize] = 0 as *mut CsCmdActorAction;
            i = i.wrapping_add(1)
        }
        (*csCtx).state = (*csCtx).state.wrapping_add(1);
        if (*csCtx).state as libc::c_int ==
               CS_STATE_SKIPPABLE_INIT as libc::c_int {
            Audio_SetCutsceneFlag(1 as libc::c_int as s8);
            (*csCtx).frames = 0xffff as libc::c_int as u16_0;
            (*csCtx).unk_18 = 0xffff as libc::c_int as u16_0;
            D_8015FCC0 = 0xffff as libc::c_int as u16_0;
            D_8015FCC2 = 0xffff as libc::c_int as u16_0;
            D_8015FCC4 = 0xffff as libc::c_int as u16_0;
            (*csCtx).unk_1A = 0 as libc::c_int as u8_0;
            (*csCtx).unk_1B = 0 as libc::c_int as u8_0;
            D_8015FCC6 = (*globalCtx).activeCamera;
            if D_8015FCC8 as libc::c_int != 0 as libc::c_int {
                (*csCtx).unk_14 = Gameplay_CreateSubCamera(globalCtx) as s32
            }
            if gSaveContext.cutsceneTrigger as libc::c_int == 0 as libc::c_int
               {
                Interface_ChangeAlpha(1 as libc::c_int as u16_0);
                ShrinkWindow_SetVal(0x20 as libc::c_int);
                ShrinkWindow_SetCurrentVal(0x20 as libc::c_int);
                (*csCtx).state = (*csCtx).state.wrapping_add(1)
            }
            func_80068C3C(globalCtx, csCtx);
        }
        gSaveContext.cutsceneTrigger = 0 as libc::c_int as u8_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_80069048(mut globalCtx: *mut GlobalContext) {
    let mut i: s16 = 0;
    D_8015FCCC = 0 as libc::c_int as u16_0;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) < 20 as libc::c_int {
        // Empty Loop
        i += 1
    }
    D_8015FCE4 = 0 as libc::c_int as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn func_8006907C(mut globalCtx: *mut GlobalContext) {
    if D_8015FCCC as libc::c_int != 0 as libc::c_int {
        D_8015FCCC = 0 as libc::c_int as u16_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn Cutscene_HandleEntranceTriggers(mut globalCtx:
                                                             *mut GlobalContext) {
    let mut entranceCutscene: *mut EntranceCutscene =
        0 as *mut EntranceCutscene;
    let mut requiredAge: u8_0 = 0;
    let mut i: s16 = 0;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[EntranceCutscene; 34]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<EntranceCutscene>()
                                                   as libc::c_ulong) as s32 {
        entranceCutscene =
            &mut *sEntranceCutsceneTable.as_mut_ptr().offset(i as isize) as
                *mut EntranceCutscene;
        requiredAge = (*entranceCutscene).ageRestriction;
        if requiredAge as libc::c_int == 2 as libc::c_int {
            requiredAge = gSaveContext.linkAge as u8_0
        }
        if gSaveContext.entranceIndex ==
               (*entranceCutscene).entrance as libc::c_int &&
               (Flags_GetEventChkInf((*entranceCutscene).flag as s32) == 0 ||
                    (*entranceCutscene).flag as libc::c_int ==
                        0x18 as libc::c_int) &&
               gSaveContext.cutsceneIndex < 0xfff0 as libc::c_int &&
               gSaveContext.linkAge as u8_0 as libc::c_int ==
                   requiredAge as libc::c_int &&
               gSaveContext.respawnFlag <= 0 as libc::c_int {
            Flags_SetEventChkInf((*entranceCutscene).flag as s32);
            Cutscene_SetSegment(globalCtx, (*entranceCutscene).segAddr);
            gSaveContext.cutsceneTrigger = 2 as libc::c_int as u8_0;
            gSaveContext.showTitleCard = 0 as libc::c_int as u8_0;
            break ;
        } else { i += 1 }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Cutscene_HandleConditionalTriggers(mut globalCtx:
                                                                *mut GlobalContext) {
    osSyncPrintf(b"\ngame_info.mode=[%d] restart_flag\x00" as *const u8 as
                     *const libc::c_char, gSaveContext.respawnFlag);
    if gSaveContext.gameMode == 0 as libc::c_int &&
           gSaveContext.respawnFlag <= 0 as libc::c_int &&
           gSaveContext.cutsceneIndex < 0xfff0 as libc::c_int {
        if gSaveContext.entranceIndex == 0x1e1 as libc::c_int &&
               Flags_GetEventChkInf(0xac as libc::c_int) == 0 {
            Flags_SetEventChkInf(0xac as libc::c_int);
            gSaveContext.entranceIndex = 0x123 as libc::c_int;
            gSaveContext.cutsceneIndex = 0xfff0 as libc::c_int
        } else if gSaveContext.entranceIndex == 0xdb as libc::c_int &&
                      gSaveContext.linkAge == 0 as libc::c_int &&
                      gSaveContext.eventChkInf[4 as libc::c_int as usize] as
                          libc::c_int & 0x100 as libc::c_int != 0 &&
                      gSaveContext.eventChkInf[4 as libc::c_int as usize] as
                          libc::c_int & 0x200 as libc::c_int != 0 &&
                      gSaveContext.eventChkInf[4 as libc::c_int as usize] as
                          libc::c_int & 0x400 as libc::c_int != 0 &&
                      Flags_GetEventChkInf(0xaa as libc::c_int) == 0 {
            Flags_SetEventChkInf(0xaa as libc::c_int);
            gSaveContext.cutsceneIndex = 0xfff0 as libc::c_int
        } else if gSaveContext.entranceIndex == 0x5e0 as libc::c_int &&
                      Flags_GetEventChkInf(0xc1 as libc::c_int) == 0 {
            Flags_SetEventChkInf(0xc1 as libc::c_int);
            Item_Give(globalCtx, ITEM_OCARINA_FAIRY as libc::c_int as u8_0);
            gSaveContext.entranceIndex = 0x11e as libc::c_int;
            gSaveContext.cutsceneIndex = 0xfff0 as libc::c_int
        } else if gBitFlags[QUEST_MEDALLION_SPIRIT as libc::c_int as usize] &
                      gSaveContext.inventory.questItems != 0 &&
                      gBitFlags[QUEST_MEDALLION_SHADOW as libc::c_int as
                                    usize] & gSaveContext.inventory.questItems
                          != 0 && gSaveContext.linkAge == 0 as libc::c_int &&
                      Flags_GetEventChkInf(0xc4 as libc::c_int) == 0 &&
                      gEntranceTable[gSaveContext.entranceIndex as
                                         usize].scene as libc::c_int ==
                          SCENE_TOKINOMA as libc::c_int {
            Flags_SetEventChkInf(0xc4 as libc::c_int);
            gSaveContext.entranceIndex = 0x53 as libc::c_int;
            gSaveContext.cutsceneIndex = 0xfff8 as libc::c_int
        } else if Flags_GetEventChkInf(0xc7 as libc::c_int) == 0 &&
                      gEntranceTable[gSaveContext.entranceIndex as
                                         usize].scene as libc::c_int ==
                          SCENE_GANON_DEMO as libc::c_int {
            Flags_SetEventChkInf(0xc7 as libc::c_int);
            gSaveContext.entranceIndex = 0x517 as libc::c_int;
            gSaveContext.cutsceneIndex = 0xfff0 as libc::c_int
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Cutscene_SetSegment(mut globalCtx:
                                                 *mut GlobalContext,
                                             mut segment: *mut libc::c_void) {
    if (segment as u32_0) << 4 as libc::c_int >> 28 as libc::c_int !=
           0 as libc::c_int as libc::c_uint {
        (*globalCtx).csCtx.segment =
            gSegments[((segment as u32_0) << 4 as libc::c_int >>
                           28 as libc::c_int) as
                          usize].wrapping_add(segment as u32_0 &
                                                  0xffffff as libc::c_int as
                                                      libc::c_uint).wrapping_add(0x80000000
                                                                                     as
                                                                                     libc::c_uint)
                as *mut libc::c_void
    } else { (*globalCtx).csCtx.segment = segment };
}
