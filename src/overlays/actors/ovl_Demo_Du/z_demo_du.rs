#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, register_tool)]
extern "C" {
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn func_8002829C(globalCtx: *mut GlobalContext, pos: *mut Vec3f,
                     velocity: *mut Vec3f, accel: *mut Vec3f,
                     primColor: *mut Color_RGBA8, envColor: *mut Color_RGBA8,
                     scale: s16, scaleStep: s16);
    #[no_mangle]
    fn ActorShape_Init(shape: *mut ActorShape, yOffset: f32_0,
                       shadowDraw: ActorShadowFunc, shadowScale: f32_0);
    #[no_mangle]
    fn ActorShadow_DrawCircle(actor: *mut Actor, lights: *mut Lights,
                              globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn Actor_UpdateBgCheckInfo(globalCtx: *mut GlobalContext,
                               actor: *mut Actor, wallCheckHeight: f32_0,
                               wallCheckRadius: f32_0,
                               ceilingCheckHeight: f32_0, flags: s32);
    #[no_mangle]
    fn Actor_SpawnAsChild(actorCtx: *mut ActorContext, parent: *mut Actor,
                          globalCtx: *mut GlobalContext, actorId: s16,
                          posX: f32_0, posY: f32_0, posZ: f32_0, rotX: s16,
                          rotY: s16, rotZ: s16, params: s16) -> *mut Actor;
    #[no_mangle]
    fn func_80033480(globalCtx: *mut GlobalContext, posBase: *mut Vec3f,
                     randRangeDiameter: f32_0, amountMinusOne: s32,
                     scaleBase: s16, scaleStep: s16, arg6: u8_0);
    #[no_mangle]
    fn Environment_LerpWeight(max: u16_0, min: u16_0, val: u16_0) -> f32_0;
    #[no_mangle]
    fn Rand_S16Offset(base: s16, range: s16) -> s16;
    #[no_mangle]
    fn func_800788CC(sfxId: u16_0);
    #[no_mangle]
    fn func_80078914(arg0: *mut Vec3f, sfxId: u16_0);
    #[no_mangle]
    fn Item_Give(globalCtx: *mut GlobalContext, item: u8_0) -> u8_0;
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
    fn Animation_GetLastFrame(animation: *mut libc::c_void) -> s16;
    #[no_mangle]
    fn SkelAnime_DrawFlex(globalCtx: *mut GlobalContext,
                          skeleton: *mut *mut libc::c_void,
                          jointTable: *mut Vec3s, dListCount: s32,
                          overrideLimbDraw: OverrideLimbDraw,
                          postLimbDraw: PostLimbDraw, arg: *mut libc::c_void,
                          gfx: *mut Gfx) -> *mut Gfx;
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
    fn Animation_OnFrame(skelAnime: *mut SkelAnime, frame: f32_0) -> s32;
    #[no_mangle]
    fn SkelAnime_Free(skelAnime: *mut SkelAnime,
                      globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn Graph_OpenDisps(dispRefs: *mut *mut Gfx, gfxCtx: *mut GraphicsContext,
                       file: *const libc::c_char, line: s32);
    #[no_mangle]
    fn Graph_CloseDisps(dispRefs: *mut *mut Gfx, gfxCtx: *mut GraphicsContext,
                        file: *const libc::c_char, line: s32);
    #[no_mangle]
    fn Audio_PlaySoundGeneral(sfxId: u16_0, pos: *mut Vec3f, token: u8_0,
                              freqScale: *mut f32_0, a4: *mut f32_0,
                              reverbAdd: *mut s8);
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
    static mut D_80116280: [Gfx; 0];
    #[no_mangle]
    static mut gDaruniaItemGiveAnim: AnimationHeader;
    #[no_mangle]
    static mut gDaruniaItemGiveIdleAnim: AnimationHeader;
    #[no_mangle]
    static mut gDaruniaHitLinkAnim: AnimationHeader;
    #[no_mangle]
    static mut gDaruniaHitBreastAnim: AnimationHeader;
    #[no_mangle]
    static mut gDaruniaStandUpAfterFallingAnim: AnimationHeader;
    #[no_mangle]
    static mut gDaruniaLookingUpToSariaAnim: AnimationHeader;
    #[no_mangle]
    static mut gDaruniaCreditsHitBreastAnim: AnimationHeader;
    #[no_mangle]
    static mut gDaruniaCreditsIdleAnim: AnimationHeader;
    #[no_mangle]
    static mut gDaruniaIdleAnim: AnimationHeader;
    #[no_mangle]
    static mut gDaruniaNoseSeriousTex: [u64_0; 0];
    #[no_mangle]
    static mut gDaruniaEyeOpenTex: [u64_0; 0];
    #[no_mangle]
    static mut gDaruniaEyeOpeningTex: [u64_0; 0];
    #[no_mangle]
    static mut gDaruniaEyeShutTex: [u64_0; 0];
    #[no_mangle]
    static mut gDaruniaMouthSeriousTex: [u64_0; 0];
    #[no_mangle]
    static mut gDaruniaMouthGrinningTex: [u64_0; 0];
    #[no_mangle]
    static mut gDaruniaEyeClosingTex: [u64_0; 0];
    #[no_mangle]
    static mut gDaruniaMouthOpenTex: [u64_0; 0];
    #[no_mangle]
    static mut gDaruniaMouthHappyTex: [u64_0; 0];
    #[no_mangle]
    static mut gDaruniaSkel: FlexSkeletonHeader;
    #[no_mangle]
    static mut gDaruniaSageFormationAnim: AnimationHeader;
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
pub const ACTOR_ID_MAX: C2RustUnnamed_16 = 471;
pub const ACTOR_OBJ_WARP2BLOCK: C2RustUnnamed_16 = 470;
pub const ACTOR_BG_JYA_BLOCK: C2RustUnnamed_16 = 469;
pub const ACTOR_EN_MM2: C2RustUnnamed_16 = 468;
pub const ACTOR_EN_ZL4: C2RustUnnamed_16 = 467;
pub const ACTOR_OBJ_HAMISHI: C2RustUnnamed_16 = 466;
pub const ACTOR_OBJ_TIMEBLOCK: C2RustUnnamed_16 = 465;
pub const ACTOR_EN_GE3: C2RustUnnamed_16 = 464;
pub const ACTOR_OBJ_MAKEKINSUTA: C2RustUnnamed_16 = 463;
pub const ACTOR_EN_ZO: C2RustUnnamed_16 = 462;
pub const ACTOR_BG_MENKURI_NISEKABE: C2RustUnnamed_16 = 461;
pub const ACTOR_EN_EG: C2RustUnnamed_16 = 460;
pub const ACTOR_OCEFF_WIPE4: C2RustUnnamed_16 = 459;
pub const ACTOR_EN_KAKASI3: C2RustUnnamed_16 = 458;
pub const ACTOR_EN_KAKASI2: C2RustUnnamed_16 = 457;
pub const ACTOR_BG_ICE_SHUTTER: C2RustUnnamed_16 = 456;
pub const ACTOR_BG_ICE_TURARA: C2RustUnnamed_16 = 455;
pub const ACTOR_EN_COW: C2RustUnnamed_16 = 454;
pub const ACTOR_EN_MA3: C2RustUnnamed_16 = 453;
pub const ACTOR_BG_SPOT18_SHUTTER: C2RustUnnamed_16 = 452;
pub const ACTOR_BG_SPOT18_FUTA: C2RustUnnamed_16 = 451;
pub const ACTOR_BG_SPOT11_OASIS: C2RustUnnamed_16 = 450;
pub const ACTOR_DOOR_KILLER: C2RustUnnamed_16 = 449;
pub const ACTOR_EN_CROW: C2RustUnnamed_16 = 448;
pub const ACTOR_EN_PO_DESERT: C2RustUnnamed_16 = 447;
pub const ACTOR_EN_WALL_TUBO: C2RustUnnamed_16 = 446;
pub const ACTOR_BG_BOWL_WALL: C2RustUnnamed_16 = 445;
pub const ACTOR_EN_DAIKU_KAKARIKO: C2RustUnnamed_16 = 444;
pub const ACTOR_BG_MIZU_SHUTTER: C2RustUnnamed_16 = 443;
pub const ACTOR_BG_MIZU_BWALL: C2RustUnnamed_16 = 442;
pub const ACTOR_EN_GS: C2RustUnnamed_16 = 441;
pub const ACTOR_EN_GB: C2RustUnnamed_16 = 440;
pub const ACTOR_BG_GND_ICEBLOCK: C2RustUnnamed_16 = 439;
pub const ACTOR_BG_GND_NISEKABE: C2RustUnnamed_16 = 438;
pub const ACTOR_BG_GND_SOULMEIRO: C2RustUnnamed_16 = 437;
pub const ACTOR_BG_GND_DARKMEIRO: C2RustUnnamed_16 = 436;
pub const ACTOR_BG_GND_FIREMEIRO: C2RustUnnamed_16 = 435;
pub const ACTOR_DEMO_GEFF: C2RustUnnamed_16 = 434;
pub const ACTOR_DEMO_GJ: C2RustUnnamed_16 = 433;
pub const ACTOR_EN_SKB: C2RustUnnamed_16 = 432;
pub const ACTOR_EN_WF: C2RustUnnamed_16 = 431;
pub const ACTOR_EN_GO2: C2RustUnnamed_16 = 430;
pub const ACTOR_EN_MU: C2RustUnnamed_16 = 429;
pub const ACTOR_EN_TG: C2RustUnnamed_16 = 428;
pub const ACTOR_OBJ_MURE3: C2RustUnnamed_16 = 427;
pub const ACTOR_UNSET_1AA: C2RustUnnamed_16 = 426;
pub const ACTOR_BG_SPOT17_BAKUDANKABE: C2RustUnnamed_16 = 425;
pub const ACTOR_BG_SPOT08_BAKUDANKABE: C2RustUnnamed_16 = 424;
pub const ACTOR_DEMO_KEKKAI: C2RustUnnamed_16 = 423;
pub const ACTOR_EN_HS2: C2RustUnnamed_16 = 422;
pub const ACTOR_BG_BOM_GUARD: C2RustUnnamed_16 = 421;
pub const ACTOR_EN_GUEST: C2RustUnnamed_16 = 420;
pub const ACTOR_EN_DNT_NOMAL: C2RustUnnamed_16 = 419;
pub const ACTOR_EN_DNT_JIJI: C2RustUnnamed_16 = 418;
pub const ACTOR_EN_DNT_DEMO: C2RustUnnamed_16 = 417;
pub const ACTOR_OBJ_KIBAKO2: C2RustUnnamed_16 = 416;
pub const ACTOR_BG_SPOT11_BAKUDANKABE: C2RustUnnamed_16 = 415;
pub const ACTOR_OBJ_COMB: C2RustUnnamed_16 = 414;
pub const ACTOR_BG_SPOT01_OBJECTS2: C2RustUnnamed_16 = 413;
pub const ACTOR_EN_SI: C2RustUnnamed_16 = 412;
pub const ACTOR_EN_DOG: C2RustUnnamed_16 = 411;
pub const ACTOR_EN_NIW_GIRL: C2RustUnnamed_16 = 410;
pub const ACTOR_OCEFF_WIPE3: C2RustUnnamed_16 = 409;
pub const ACTOR_OCEFF_WIPE2: C2RustUnnamed_16 = 408;
pub const ACTOR_EN_GELDB: C2RustUnnamed_16 = 407;
pub const ACTOR_EN_IT: C2RustUnnamed_16 = 406;
pub const ACTOR_EN_SHOPNUTS: C2RustUnnamed_16 = 405;
pub const ACTOR_BG_SPOT00_BREAK: C2RustUnnamed_16 = 404;
pub const ACTOR_EN_NUTSBALL: C2RustUnnamed_16 = 403;
pub const ACTOR_EN_HINTNUTS: C2RustUnnamed_16 = 402;
pub const ACTOR_BG_SPOT12_SAKU: C2RustUnnamed_16 = 401;
pub const ACTOR_BG_SPOT12_GATE: C2RustUnnamed_16 = 400;
pub const ACTOR_BG_JYA_HAHENIRON: C2RustUnnamed_16 = 399;
pub const ACTOR_BG_JYA_1FLIFT: C2RustUnnamed_16 = 398;
pub const ACTOR_BG_SPOT05_SOKO: C2RustUnnamed_16 = 397;
pub const ACTOR_EN_WEIYER: C2RustUnnamed_16 = 396;
pub const ACTOR_OCEFF_STORM: C2RustUnnamed_16 = 395;
pub const ACTOR_OCEFF_WIPE: C2RustUnnamed_16 = 394;
pub const ACTOR_EN_STH: C2RustUnnamed_16 = 393;
pub const ACTOR_EN_SSH: C2RustUnnamed_16 = 392;
pub const ACTOR_OBJ_ROOMTIMER: C2RustUnnamed_16 = 391;
pub const ACTOR_EN_GE2: C2RustUnnamed_16 = 390;
pub const ACTOR_EN_WONDER_TALK2: C2RustUnnamed_16 = 389;
pub const ACTOR_EN_DY_EXTRA: C2RustUnnamed_16 = 388;
pub const ACTOR_SHOT_SUN: C2RustUnnamed_16 = 387;
pub const ACTOR_DEMO_EC: C2RustUnnamed_16 = 386;
pub const ACTOR_EN_TORCH: C2RustUnnamed_16 = 385;
pub const ACTOR_UNSET_180: C2RustUnnamed_16 = 384;
pub const ACTOR_END_TITLE: C2RustUnnamed_16 = 383;
pub const ACTOR_OCEFF_SPOT: C2RustUnnamed_16 = 382;
pub const ACTOR_OBJ_MAKEOSHIHIKI: C2RustUnnamed_16 = 381;
pub const ACTOR_EN_TAKARA_MAN: C2RustUnnamed_16 = 380;
pub const ACTOR_EN_KAKASI: C2RustUnnamed_16 = 379;
pub const ACTOR_BOSS_GANON2: C2RustUnnamed_16 = 378;
pub const ACTOR_EN_ZL3: C2RustUnnamed_16 = 377;
pub const ACTOR_EN_HEISHI4: C2RustUnnamed_16 = 376;
pub const ACTOR_BG_ZG: C2RustUnnamed_16 = 375;
pub const ACTOR_EFC_ERUPC: C2RustUnnamed_16 = 374;
pub const ACTOR_EN_PO_FIELD: C2RustUnnamed_16 = 373;
pub const ACTOR_DEMO_GT: C2RustUnnamed_16 = 372;
pub const ACTOR_ELF_MSG2: C2RustUnnamed_16 = 371;
pub const ACTOR_DOOR_GERUDO: C2RustUnnamed_16 = 370;
pub const ACTOR_EN_MAG: C2RustUnnamed_16 = 369;
pub const ACTOR_EN_OKARINA_EFFECT: C2RustUnnamed_16 = 368;
pub const ACTOR_EN_GANON_MANT: C2RustUnnamed_16 = 367;
pub const ACTOR_EN_HY: C2RustUnnamed_16 = 366;
pub const ACTOR_EN_MD: C2RustUnnamed_16 = 365;
pub const ACTOR_EN_CS: C2RustUnnamed_16 = 364;
pub const ACTOR_EN_JSJUTAN: C2RustUnnamed_16 = 363;
pub const ACTOR_EN_JS: C2RustUnnamed_16 = 362;
pub const ACTOR_BG_JYA_IRONOBJ: C2RustUnnamed_16 = 361;
pub const ACTOR_EN_EX_ITEM: C2RustUnnamed_16 = 360;
pub const ACTOR_EN_ANI: C2RustUnnamed_16 = 359;
pub const ACTOR_BG_SST_FLOOR: C2RustUnnamed_16 = 358;
pub const ACTOR_EN_WEATHER_TAG: C2RustUnnamed_16 = 357;
pub const ACTOR_EN_KZ: C2RustUnnamed_16 = 356;
pub const ACTOR_EN_KO: C2RustUnnamed_16 = 355;
pub const ACTOR_EN_MM: C2RustUnnamed_16 = 354;
pub const ACTOR_UNSET_161: C2RustUnnamed_16 = 353;
pub const ACTOR_EN_STREAM: C2RustUnnamed_16 = 352;
pub const ACTOR_EN_SIOFUKI: C2RustUnnamed_16 = 351;
pub const ACTOR_EN_GANON_ORGAN: C2RustUnnamed_16 = 350;
pub const ACTOR_UNSET_15D: C2RustUnnamed_16 = 349;
pub const ACTOR_BG_SPOT18_BASKET: C2RustUnnamed_16 = 348;
pub const ACTOR_BG_JYA_BOMBIWA: C2RustUnnamed_16 = 347;
pub const ACTOR_BG_JYA_AMISHUTTER: C2RustUnnamed_16 = 346;
pub const ACTOR_BG_JYA_BOMBCHUIWA: C2RustUnnamed_16 = 345;
pub const ACTOR_BG_JYA_BIGMIRROR: C2RustUnnamed_16 = 344;
pub const ACTOR_BG_JYA_LIFT: C2RustUnnamed_16 = 343;
pub const ACTOR_BG_JYA_MEGAMI: C2RustUnnamed_16 = 342;
pub const ACTOR_EN_CHANGER: C2RustUnnamed_16 = 341;
pub const ACTOR_UNSET_154: C2RustUnnamed_16 = 340;
pub const ACTOR_EN_FU: C2RustUnnamed_16 = 339;
pub const ACTOR_EN_GO: C2RustUnnamed_16 = 338;
pub const ACTOR_OBJ_MURE2: C2RustUnnamed_16 = 337;
pub const ACTOR_OBJ_LIGHTSWITCH: C2RustUnnamed_16 = 336;
pub const ACTOR_OBJ_HANA: C2RustUnnamed_16 = 335;
pub const ACTOR_EN_ISHI: C2RustUnnamed_16 = 334;
pub const ACTOR_EN_OWL: C2RustUnnamed_16 = 333;
pub const ACTOR_EN_BOM_BOWL_PIT: C2RustUnnamed_16 = 332;
pub const ACTOR_EN_BOM_BOWL_MAN: C2RustUnnamed_16 = 331;
pub const ACTOR_EN_MK: C2RustUnnamed_16 = 330;
pub const ACTOR_EN_DS: C2RustUnnamed_16 = 329;
pub const ACTOR_BG_GJYO_BRIDGE: C2RustUnnamed_16 = 328;
pub const ACTOR_EN_WONDER_TALK: C2RustUnnamed_16 = 327;
pub const ACTOR_EN_SA: C2RustUnnamed_16 = 326;
pub const ACTOR_BG_SPOT01_IDOSOKO: C2RustUnnamed_16 = 325;
pub const ACTOR_EN_ATTACK_NIW: C2RustUnnamed_16 = 324;
pub const ACTOR_EN_SYATEKI_NIW: C2RustUnnamed_16 = 323;
pub const ACTOR_EN_HEISHI3: C2RustUnnamed_16 = 322;
pub const ACTOR_EN_KANBAN: C2RustUnnamed_16 = 321;
pub const ACTOR_BG_INGATE: C2RustUnnamed_16 = 320;
pub const ACTOR_EN_HS: C2RustUnnamed_16 = 319;
pub const ACTOR_EN_MS: C2RustUnnamed_16 = 318;
pub const ACTOR_EN_GM: C2RustUnnamed_16 = 317;
pub const ACTOR_EN_NIW_LADY: C2RustUnnamed_16 = 316;
pub const ACTOR_EN_CLEAR_TAG: C2RustUnnamed_16 = 315;
pub const ACTOR_EN_SDA: C2RustUnnamed_16 = 314;
pub const ACTOR_OBJ_BLOCKSTOP: C2RustUnnamed_16 = 313;
pub const ACTOR_EN_GE1: C2RustUnnamed_16 = 312;
pub const ACTOR_ITEM_INBOX: C2RustUnnamed_16 = 311;
pub const ACTOR_EN_BLKOBJ: C2RustUnnamed_16 = 310;
pub const ACTOR_EN_NWC: C2RustUnnamed_16 = 309;
pub const ACTOR_UNSET_134: C2RustUnnamed_16 = 308;
pub const ACTOR_EN_DAIKU: C2RustUnnamed_16 = 307;
pub const ACTOR_EN_TORYO: C2RustUnnamed_16 = 306;
pub const ACTOR_EN_EX_RUPPY: C2RustUnnamed_16 = 305;
pub const ACTOR_EN_GOROIWA: C2RustUnnamed_16 = 304;
pub const ACTOR_EN_YABUSAME_MARK: C2RustUnnamed_16 = 303;
pub const ACTOR_EN_OKARINA_TAG: C2RustUnnamed_16 = 302;
pub const ACTOR_OBJ_HSBLOCK: C2RustUnnamed_16 = 301;
pub const ACTOR_OBJ_LIFT: C2RustUnnamed_16 = 300;
pub const ACTOR_OBJ_ELEVATOR: C2RustUnnamed_16 = 299;
pub const ACTOR_OBJ_SWITCH: C2RustUnnamed_16 = 298;
pub const ACTOR_UNSET_129: C2RustUnnamed_16 = 297;
pub const ACTOR_UNSET_128: C2RustUnnamed_16 = 296;
pub const ACTOR_OBJ_BOMBIWA: C2RustUnnamed_16 = 295;
pub const ACTOR_OBJ_BEAN: C2RustUnnamed_16 = 294;
pub const ACTOR_EN_KUSA: C2RustUnnamed_16 = 293;
pub const ACTOR_EN_DIVING_GAME: C2RustUnnamed_16 = 292;
pub const ACTOR_BG_RELAY_OBJECTS: C2RustUnnamed_16 = 291;
pub const ACTOR_EN_PO_RELAY: C2RustUnnamed_16 = 290;
pub const ACTOR_EN_FZ: C2RustUnnamed_16 = 289;
pub const ACTOR_BG_SPOT07_TAKI: C2RustUnnamed_16 = 288;
pub const ACTOR_BG_SPOT03_TAKI: C2RustUnnamed_16 = 287;
pub const ACTOR_OBJ_ICE_POLY: C2RustUnnamed_16 = 286;
pub const ACTOR_EN_TUBO_TRAP: C2RustUnnamed_16 = 285;
pub const ACTOR_EN_HONOTRAP: C2RustUnnamed_16 = 284;
pub const ACTOR_ELF_MSG: C2RustUnnamed_16 = 283;
pub const ACTOR_EN_DNS: C2RustUnnamed_16 = 282;
pub const ACTOR_DEMO_SHD: C2RustUnnamed_16 = 281;
pub const ACTOR_DEMO_EXT: C2RustUnnamed_16 = 280;
pub const ACTOR_EN_G_SWITCH: C2RustUnnamed_16 = 279;
pub const ACTOR_EN_SKJNEEDLE: C2RustUnnamed_16 = 278;
pub const ACTOR_EN_SKJ: C2RustUnnamed_16 = 277;
pub const ACTOR_DEMO_IK: C2RustUnnamed_16 = 276;
pub const ACTOR_EN_IK: C2RustUnnamed_16 = 275;
pub const ACTOR_EN_WONDER_ITEM: C2RustUnnamed_16 = 274;
pub const ACTOR_OBJ_TSUBO: C2RustUnnamed_16 = 273;
pub const ACTOR_OBJ_KIBAKO: C2RustUnnamed_16 = 272;
pub const ACTOR_ITEM_ETCETERA: C2RustUnnamed_16 = 271;
pub const ACTOR_UNSET_10E: C2RustUnnamed_16 = 270;
pub const ACTOR_UNSET_10D: C2RustUnnamed_16 = 269;
pub const ACTOR_ARROW_LIGHT: C2RustUnnamed_16 = 268;
pub const ACTOR_ARROW_ICE: C2RustUnnamed_16 = 267;
pub const ACTOR_ARROW_FIRE: C2RustUnnamed_16 = 266;
pub const ACTOR_UNSET_109: C2RustUnnamed_16 = 265;
pub const ACTOR_BG_UMAJUMP: C2RustUnnamed_16 = 264;
pub const ACTOR_BG_SPOT15_RRBOX: C2RustUnnamed_16 = 263;
pub const ACTOR_BG_GANON_OTYUKA: C2RustUnnamed_16 = 262;
pub const ACTOR_BG_PO_SYOKUDAI: C2RustUnnamed_16 = 261;
pub const ACTOR_BG_SPOT01_IDOMIZU: C2RustUnnamed_16 = 260;
pub const ACTOR_BG_SPOT01_IDOHASHIRA: C2RustUnnamed_16 = 259;
pub const ACTOR_BG_SPOT01_FUSYA: C2RustUnnamed_16 = 258;
pub const ACTOR_EFF_DUST: C2RustUnnamed_16 = 257;
pub const ACTOR_BG_GATE_SHUTTER: C2RustUnnamed_16 = 256;
pub const ACTOR_OBJ_OSHIHIKI: C2RustUnnamed_16 = 255;
pub const ACTOR_FISHING: C2RustUnnamed_16 = 254;
pub const ACTOR_BG_JYA_KANAAMI: C2RustUnnamed_16 = 253;
pub const ACTOR_BG_JYA_COBRA: C2RustUnnamed_16 = 252;
pub const ACTOR_UNSET_FB: C2RustUnnamed_16 = 251;
pub const ACTOR_BG_JYA_ZURERUKABE: C2RustUnnamed_16 = 250;
pub const ACTOR_BG_JYA_GOROIWA: C2RustUnnamed_16 = 249;
pub const ACTOR_BG_SPOT15_SAKU: C2RustUnnamed_16 = 248;
pub const ACTOR_BG_HAKA_GATE: C2RustUnnamed_16 = 247;
pub const ACTOR_EN_ANUBICE_TAG: C2RustUnnamed_16 = 246;
pub const ACTOR_DEMO_6K: C2RustUnnamed_16 = 245;
pub const ACTOR_MAGIC_DARK: C2RustUnnamed_16 = 244;
pub const ACTOR_UNSET_F3: C2RustUnnamed_16 = 243;
pub const ACTOR_UNSET_F2: C2RustUnnamed_16 = 242;
pub const ACTOR_ITEM_OCARINA: C2RustUnnamed_16 = 241;
pub const ACTOR_EN_ICE_HONO: C2RustUnnamed_16 = 240;
pub const ACTOR_BG_ICE_SHELTER: C2RustUnnamed_16 = 239;
pub const ACTOR_ITEM_SHIELD: C2RustUnnamed_16 = 238;
pub const ACTOR_EN_FR: C2RustUnnamed_16 = 237;
pub const ACTOR_EN_NY: C2RustUnnamed_16 = 236;
pub const ACTOR_UNSET_EB: C2RustUnnamed_16 = 235;
pub const ACTOR_UNSET_EA: C2RustUnnamed_16 = 234;
pub const ACTOR_BOSS_SST: C2RustUnnamed_16 = 233;
pub const ACTOR_BOSS_GANON: C2RustUnnamed_16 = 232;
pub const ACTOR_EN_MA1: C2RustUnnamed_16 = 231;
pub const ACTOR_BG_BDAN_SWITCH: C2RustUnnamed_16 = 230;
pub const ACTOR_BG_SPOT16_DOUGHNUT: C2RustUnnamed_16 = 229;
pub const ACTOR_BG_MORI_IDOMIZU: C2RustUnnamed_16 = 228;
pub const ACTOR_BG_MORI_HASHIRA4: C2RustUnnamed_16 = 227;
pub const ACTOR_BG_MORI_HASHIGO: C2RustUnnamed_16 = 226;
pub const ACTOR_EN_ANUBICE_FIRE: C2RustUnnamed_16 = 225;
pub const ACTOR_EN_ANUBICE: C2RustUnnamed_16 = 224;
pub const ACTOR_EN_BX: C2RustUnnamed_16 = 223;
pub const ACTOR_EN_BA: C2RustUnnamed_16 = 222;
pub const ACTOR_EN_RR: C2RustUnnamed_16 = 221;
pub const ACTOR_BOSS_TW: C2RustUnnamed_16 = 220;
pub const ACTOR_EN_HORSE_GAME_CHECK: C2RustUnnamed_16 = 219;
pub const ACTOR_EN_BOM_CHU: C2RustUnnamed_16 = 218;
pub const ACTOR_EN_MA2: C2RustUnnamed_16 = 217;
pub const ACTOR_UNSET_D8: C2RustUnnamed_16 = 216;
pub const ACTOR_BG_HAKA_WATER: C2RustUnnamed_16 = 215;
pub const ACTOR_BG_ICE_OBJECTS: C2RustUnnamed_16 = 214;
pub const ACTOR_BG_SPOT06_OBJECTS: C2RustUnnamed_16 = 213;
pub const ACTOR_BG_MIZU_UZU: C2RustUnnamed_16 = 212;
pub const ACTOR_OBJ_DEKUJR: C2RustUnnamed_16 = 211;
pub const ACTOR_EN_RU2: C2RustUnnamed_16 = 210;
pub const ACTOR_BG_SPOT08_ICEBLOCK: C2RustUnnamed_16 = 209;
pub const ACTOR_BG_BOMBWALL: C2RustUnnamed_16 = 208;
pub const ACTOR_BG_HIDAN_KOWARERUKABE: C2RustUnnamed_16 = 207;
pub const ACTOR_UNSET_CE: C2RustUnnamed_16 = 206;
pub const ACTOR_BG_SPOT16_BOMBSTONE: C2RustUnnamed_16 = 205;
pub const ACTOR_EN_TR: C2RustUnnamed_16 = 204;
pub const ACTOR_EN_IN: C2RustUnnamed_16 = 203;
pub const ACTOR_DEMO_GO: C2RustUnnamed_16 = 202;
pub const ACTOR_DEMO_SA: C2RustUnnamed_16 = 201;
pub const ACTOR_BG_BDAN_OBJECTS: C2RustUnnamed_16 = 200;
pub const ACTOR_EN_KAREBABA: C2RustUnnamed_16 = 199;
pub const ACTOR_EN_BIGOKUTA: C2RustUnnamed_16 = 198;
pub const ACTOR_EN_SB: C2RustUnnamed_16 = 197;
pub const ACTOR_BOSS_MO: C2RustUnnamed_16 = 196;
pub const ACTOR_EN_NB: C2RustUnnamed_16 = 195;
pub const ACTOR_EN_TANA: C2RustUnnamed_16 = 194;
pub const ACTOR_EN_SYATEKI_MAN: C2RustUnnamed_16 = 193;
pub const ACTOR_EN_SYATEKI_ITM: C2RustUnnamed_16 = 192;
pub const ACTOR_BG_SPOT17_FUNEN: C2RustUnnamed_16 = 191;
pub const ACTOR_BG_HAKA_ZOU: C2RustUnnamed_16 = 190;
pub const ACTOR_BG_HAKA_HUTA: C2RustUnnamed_16 = 189;
pub const ACTOR_BG_HAKA_TRAP: C2RustUnnamed_16 = 188;
pub const ACTOR_BG_HAKA_TUBO: C2RustUnnamed_16 = 187;
pub const ACTOR_BOSS_VA: C2RustUnnamed_16 = 186;
pub const ACTOR_BG_SPOT18_OBJ: C2RustUnnamed_16 = 185;
pub const ACTOR_BG_SPOT09_OBJ: C2RustUnnamed_16 = 184;
pub const ACTOR_MIR_RAY: C2RustUnnamed_16 = 183;
pub const ACTOR_EN_BROB: C2RustUnnamed_16 = 182;
pub const ACTOR_EN_FIRE_ROCK: C2RustUnnamed_16 = 181;
pub const ACTOR_EN_ENCOUNT2: C2RustUnnamed_16 = 180;
pub const ACTOR_EN_HEISHI2: C2RustUnnamed_16 = 179;
pub const ACTOR_UNSET_B2: C2RustUnnamed_16 = 178;
pub const ACTOR_BG_HAKA_SGAMI: C2RustUnnamed_16 = 177;
pub const ACTOR_BG_HAKA_SHIP: C2RustUnnamed_16 = 176;
pub const ACTOR_BG_HAKA_MEGANEBG: C2RustUnnamed_16 = 175;
pub const ACTOR_BG_HAKA_MEGANE: C2RustUnnamed_16 = 174;
pub const ACTOR_EN_VB_BALL: C2RustUnnamed_16 = 173;
pub const ACTOR_BG_VB_SIMA: C2RustUnnamed_16 = 172;
pub const ACTOR_EN_FW: C2RustUnnamed_16 = 171;
pub const ACTOR_DEMO_TRE_LGT: C2RustUnnamed_16 = 170;
pub const ACTOR_DEMO_IM: C2RustUnnamed_16 = 169;
pub const ACTOR_DEMO_DU: C2RustUnnamed_16 = 168;
pub const ACTOR_EN_ENCOUNT1: C2RustUnnamed_16 = 167;
pub const ACTOR_EN_RL: C2RustUnnamed_16 = 166;
pub const ACTOR_EN_DHA: C2RustUnnamed_16 = 165;
pub const ACTOR_EN_DH: C2RustUnnamed_16 = 164;
pub const ACTOR_EN_FD_FIRE: C2RustUnnamed_16 = 163;
pub const ACTOR_BOSS_FD2: C2RustUnnamed_16 = 162;
pub const ACTOR_EN_RU1: C2RustUnnamed_16 = 161;
pub const ACTOR_UNSET_A0: C2RustUnnamed_16 = 160;
pub const ACTOR_MAGIC_FIRE: C2RustUnnamed_16 = 159;
pub const ACTOR_MAGIC_WIND: C2RustUnnamed_16 = 158;
pub const ACTOR_BG_HAKA: C2RustUnnamed_16 = 157;
pub const ACTOR_BG_SPOT02_OBJECTS: C2RustUnnamed_16 = 156;
pub const ACTOR_DOOR_ANA: C2RustUnnamed_16 = 155;
pub const ACTOR_EN_HORSE_LINK_CHILD: C2RustUnnamed_16 = 154;
pub const ACTOR_EN_FD: C2RustUnnamed_16 = 153;
pub const ACTOR_EN_DU: C2RustUnnamed_16 = 152;
pub const ACTOR_OBJECT_KANKYO: C2RustUnnamed_16 = 151;
pub const ACTOR_BOSS_FD: C2RustUnnamed_16 = 150;
pub const ACTOR_EN_SW: C2RustUnnamed_16 = 149;
pub const ACTOR_OBJ_MURE: C2RustUnnamed_16 = 148;
pub const ACTOR_BG_PO_EVENT: C2RustUnnamed_16 = 147;
pub const ACTOR_BG_HEAVY_BLOCK: C2RustUnnamed_16 = 146;
pub const ACTOR_EN_PO_SISTERS: C2RustUnnamed_16 = 145;
pub const ACTOR_EN_RD: C2RustUnnamed_16 = 144;
pub const ACTOR_EN_HEISHI1: C2RustUnnamed_16 = 143;
pub const ACTOR_EN_FLOORMAS: C2RustUnnamed_16 = 142;
pub const ACTOR_BG_HIDAN_FWBIG: C2RustUnnamed_16 = 141;
pub const ACTOR_DEMO_KANKYO: C2RustUnnamed_16 = 140;
pub const ACTOR_DEMO_EFFECT: C2RustUnnamed_16 = 139;
pub const ACTOR_EN_VM: C2RustUnnamed_16 = 138;
pub const ACTOR_BG_MORI_RAKKATENJO: C2RustUnnamed_16 = 137;
pub const ACTOR_BG_MORI_KAITENKABE: C2RustUnnamed_16 = 136;
pub const ACTOR_BG_MORI_ELEVATOR: C2RustUnnamed_16 = 135;
pub const ACTOR_BG_MORI_BIGST: C2RustUnnamed_16 = 134;
pub const ACTOR_EN_TK: C2RustUnnamed_16 = 133;
pub const ACTOR_EN_TA: C2RustUnnamed_16 = 132;
pub const ACTOR_UNSET_83: C2RustUnnamed_16 = 131;
pub const ACTOR_EN_VASE: C2RustUnnamed_16 = 130;
pub const ACTOR_EN_AROW_TRAP: C2RustUnnamed_16 = 129;
pub const ACTOR_EN_TRAP: C2RustUnnamed_16 = 128;
pub const ACTOR_UNSET_7F: C2RustUnnamed_16 = 127;
pub const ACTOR_UNSET_7E: C2RustUnnamed_16 = 126;
pub const ACTOR_EN_PU_BOX: C2RustUnnamed_16 = 125;
pub const ACTOR_EN_LIGHTBOX: C2RustUnnamed_16 = 124;
pub const ACTOR_UNSET_7B: C2RustUnnamed_16 = 123;
pub const ACTOR_UNSET_7A: C2RustUnnamed_16 = 122;
pub const ACTOR_UNSET_79: C2RustUnnamed_16 = 121;
pub const ACTOR_UNSET_78: C2RustUnnamed_16 = 120;
pub const ACTOR_EN_WOOD02: C2RustUnnamed_16 = 119;
pub const ACTOR_UNSET_76: C2RustUnnamed_16 = 118;
pub const ACTOR_UNSET_75: C2RustUnnamed_16 = 117;
pub const ACTOR_UNSET_74: C2RustUnnamed_16 = 116;
pub const ACTOR_UNSET_73: C2RustUnnamed_16 = 115;
pub const ACTOR_EN_BIRD: C2RustUnnamed_16 = 114;
pub const ACTOR_BG_HIDAN_HAMSTEP: C2RustUnnamed_16 = 113;
pub const ACTOR_DOOR_TOKI: C2RustUnnamed_16 = 112;
pub const ACTOR_BG_HIDAN_KOUSI: C2RustUnnamed_16 = 111;
pub const ACTOR_BG_MJIN: C2RustUnnamed_16 = 110;
pub const ACTOR_EN_FHG_FIRE: C2RustUnnamed_16 = 109;
pub const ACTOR_BG_TOKI_SWD: C2RustUnnamed_16 = 108;
pub const ACTOR_EN_YUKABYUN: C2RustUnnamed_16 = 107;
pub const ACTOR_BG_TOKI_HIKARI: C2RustUnnamed_16 = 106;
pub const ACTOR_EN_BB: C2RustUnnamed_16 = 105;
pub const ACTOR_BG_MORI_HINERI: C2RustUnnamed_16 = 104;
pub const ACTOR_EN_FHG: C2RustUnnamed_16 = 103;
pub const ACTOR_ARMS_HOOK: C2RustUnnamed_16 = 102;
pub const ACTOR_BG_MIZU_WATER: C2RustUnnamed_16 = 101;
pub const ACTOR_BG_MIZU_MOVEBG: C2RustUnnamed_16 = 100;
pub const ACTOR_EN_VALI: C2RustUnnamed_16 = 99;
pub const ACTOR_BG_MENKURI_EYE: C2RustUnnamed_16 = 98;
pub const ACTOR_BG_MENKURI_KAITEN: C2RustUnnamed_16 = 97;
pub const ACTOR_EN_DEKUNUTS: C2RustUnnamed_16 = 96;
pub const ACTOR_ITEM_B_HEART: C2RustUnnamed_16 = 95;
pub const ACTOR_OBJ_SYOKUDAI: C2RustUnnamed_16 = 94;
pub const ACTOR_DOOR_WARP1: C2RustUnnamed_16 = 93;
pub const ACTOR_BG_DDAN_KD: C2RustUnnamed_16 = 92;
pub const ACTOR_EN_HORSE_ZELDA: C2RustUnnamed_16 = 91;
pub const ACTOR_EN_JJ: C2RustUnnamed_16 = 90;
pub const ACTOR_BG_BREAKWALL: C2RustUnnamed_16 = 89;
pub const ACTOR_BG_DDAN_JD: C2RustUnnamed_16 = 88;
pub const ACTOR_EN_M_THUNDER: C2RustUnnamed_16 = 87;
pub const ACTOR_EN_M_FIRE1: C2RustUnnamed_16 = 86;
pub const ACTOR_EN_DEKUBABA: C2RustUnnamed_16 = 85;
pub const ACTOR_EN_AM: C2RustUnnamed_16 = 84;
pub const ACTOR_UNSET_53: C2RustUnnamed_16 = 83;
pub const ACTOR_BOSS_GANONDROF: C2RustUnnamed_16 = 82;
pub const ACTOR_BG_YDAN_MARUTA: C2RustUnnamed_16 = 81;
pub const ACTOR_BG_YDAN_HASI: C2RustUnnamed_16 = 80;
pub const ACTOR_EN_OE2: C2RustUnnamed_16 = 79;
pub const ACTOR_BG_HIDAN_FSLIFT: C2RustUnnamed_16 = 78;
pub const ACTOR_EN_ZL2: C2RustUnnamed_16 = 77;
pub const ACTOR_EN_BOMBF: C2RustUnnamed_16 = 76;
pub const ACTOR_EN_MB: C2RustUnnamed_16 = 75;
pub const ACTOR_BG_SPOT00_HANEBASI: C2RustUnnamed_16 = 74;
pub const ACTOR_BG_HIDAN_CURTAIN: C2RustUnnamed_16 = 73;
pub const ACTOR_EN_XC: C2RustUnnamed_16 = 72;
pub const ACTOR_BG_HIDAN_SYOKU: C2RustUnnamed_16 = 71;
pub const ACTOR_BG_HIDAN_SIMA: C2RustUnnamed_16 = 70;
pub const ACTOR_BG_HIDAN_SEKIZOU: C2RustUnnamed_16 = 69;
pub const ACTOR_BG_HIDAN_RSEKIZOU: C2RustUnnamed_16 = 68;
pub const ACTOR_BG_HIDAN_ROCK: C2RustUnnamed_16 = 67;
pub const ACTOR_EN_HORSE_GANON: C2RustUnnamed_16 = 66;
pub const ACTOR_BG_HIDAN_HROCK: C2RustUnnamed_16 = 65;
pub const ACTOR_BG_HIDAN_DALM: C2RustUnnamed_16 = 64;
pub const ACTOR_BG_DODOAGO: C2RustUnnamed_16 = 63;
pub const ACTOR_BG_TREEMOUTH: C2RustUnnamed_16 = 62;
pub const ACTOR_EN_OSSAN: C2RustUnnamed_16 = 61;
pub const ACTOR_EN_HORSE_NORMAL: C2RustUnnamed_16 = 60;
pub const ACTOR_EN_RIVER_SOUND: C2RustUnnamed_16 = 59;
pub const ACTOR_EN_EIYER: C2RustUnnamed_16 = 58;
pub const ACTOR_EN_A_OBJ: C2RustUnnamed_16 = 57;
pub const ACTOR_EN_BW: C2RustUnnamed_16 = 56;
pub const ACTOR_EN_ST: C2RustUnnamed_16 = 55;
pub const ACTOR_UNSET_36: C2RustUnnamed_16 = 54;
pub const ACTOR_EN_TP: C2RustUnnamed_16 = 53;
pub const ACTOR_EN_BILI: C2RustUnnamed_16 = 52;
pub const ACTOR_EN_TORCH2: C2RustUnnamed_16 = 51;
pub const ACTOR_EN_BOOM: C2RustUnnamed_16 = 50;
pub const ACTOR_UNSET_31: C2RustUnnamed_16 = 49;
pub const ACTOR_EN_BDFIRE: C2RustUnnamed_16 = 48;
pub const ACTOR_EN_DODOJR: C2RustUnnamed_16 = 47;
pub const ACTOR_DOOR_SHUTTER: C2RustUnnamed_16 = 46;
pub const ACTOR_EN_BUBBLE: C2RustUnnamed_16 = 45;
pub const ACTOR_BG_PUSHBOX: C2RustUnnamed_16 = 44;
pub const ACTOR_EN_GOMA: C2RustUnnamed_16 = 43;
pub const ACTOR_EN_VIEWER: C2RustUnnamed_16 = 42;
pub const ACTOR_EN_ZL1: C2RustUnnamed_16 = 41;
pub const ACTOR_BOSS_GOMA: C2RustUnnamed_16 = 40;
pub const ACTOR_BOSS_DODONGO: C2RustUnnamed_16 = 39;
pub const ACTOR_EN_HATA: C2RustUnnamed_16 = 38;
pub const ACTOR_EN_ZF: C2RustUnnamed_16 = 37;
pub const ACTOR_EN_SCENE_CHANGE: C2RustUnnamed_16 = 36;
pub const ACTOR_EN_HOLL: C2RustUnnamed_16 = 35;
pub const ACTOR_UNSET_22: C2RustUnnamed_16 = 34;
pub const ACTOR_EN_FISH: C2RustUnnamed_16 = 33;
pub const ACTOR_EN_INSECT: C2RustUnnamed_16 = 32;
pub const ACTOR_UNSET_1F: C2RustUnnamed_16 = 31;
pub const ACTOR_EN_BUTTE: C2RustUnnamed_16 = 30;
pub const ACTOR_EN_PEEHAT: C2RustUnnamed_16 = 29;
pub const ACTOR_EN_REEBA: C2RustUnnamed_16 = 28;
pub const ACTOR_EN_TITE: C2RustUnnamed_16 = 27;
pub const ACTOR_UNSET_1A: C2RustUnnamed_16 = 26;
pub const ACTOR_EN_NIW: C2RustUnnamed_16 = 25;
pub const ACTOR_EN_ELF: C2RustUnnamed_16 = 24;
pub const ACTOR_UNSET_17: C2RustUnnamed_16 = 23;
pub const ACTOR_EN_ARROW: C2RustUnnamed_16 = 22;
pub const ACTOR_EN_ITEM00: C2RustUnnamed_16 = 21;
pub const ACTOR_EN_HORSE: C2RustUnnamed_16 = 20;
pub const ACTOR_EN_FIREFLY: C2RustUnnamed_16 = 19;
pub const ACTOR_EN_DODONGO: C2RustUnnamed_16 = 18;
pub const ACTOR_EN_WALLMAS: C2RustUnnamed_16 = 17;
pub const ACTOR_EN_BOM: C2RustUnnamed_16 = 16;
pub const ACTOR_BG_YDAN_SP: C2RustUnnamed_16 = 15;
pub const ACTOR_EN_OKUTA: C2RustUnnamed_16 = 14;
pub const ACTOR_EN_POH: C2RustUnnamed_16 = 13;
pub const ACTOR_BG_HIDAN_FIREWALL: C2RustUnnamed_16 = 12;
pub const ACTOR_BG_DY_YOSEIZO: C2RustUnnamed_16 = 11;
pub const ACTOR_EN_BOX: C2RustUnnamed_16 = 10;
pub const ACTOR_EN_DOOR: C2RustUnnamed_16 = 9;
pub const ACTOR_EN_LIGHT: C2RustUnnamed_16 = 8;
pub const ACTOR_EN_PART: C2RustUnnamed_16 = 7;
pub const ACTOR_UNSET_6: C2RustUnnamed_16 = 6;
pub const ACTOR_UNSET_5: C2RustUnnamed_16 = 5;
pub const ACTOR_EN_GIRLA: C2RustUnnamed_16 = 4;
pub const ACTOR_UNSET_3: C2RustUnnamed_16 = 3;
pub const ACTOR_EN_TEST: C2RustUnnamed_16 = 2;
pub const ACTOR_UNSET_1: C2RustUnnamed_16 = 1;
pub const ACTOR_PLAYER: C2RustUnnamed_16 = 0;
pub type C2RustUnnamed_17 = libc::c_uint;
pub const PLAYER_LIMB_MAX: C2RustUnnamed_17 = 22;
pub const PLAYER_LIMB_TORSO: C2RustUnnamed_17 = 21;
pub const PLAYER_LIMB_SHEATH: C2RustUnnamed_17 = 20;
pub const PLAYER_LIMB_R_HAND: C2RustUnnamed_17 = 19;
pub const PLAYER_LIMB_R_FOREARM: C2RustUnnamed_17 = 18;
pub const PLAYER_LIMB_R_SHOULDER: C2RustUnnamed_17 = 17;
pub const PLAYER_LIMB_L_HAND: C2RustUnnamed_17 = 16;
pub const PLAYER_LIMB_L_FOREARM: C2RustUnnamed_17 = 15;
pub const PLAYER_LIMB_L_SHOULDER: C2RustUnnamed_17 = 14;
pub const PLAYER_LIMB_COLLAR: C2RustUnnamed_17 = 13;
pub const PLAYER_LIMB_HAT: C2RustUnnamed_17 = 12;
pub const PLAYER_LIMB_HEAD: C2RustUnnamed_17 = 11;
pub const PLAYER_LIMB_UPPER: C2RustUnnamed_17 = 10;
pub const PLAYER_LIMB_L_FOOT: C2RustUnnamed_17 = 9;
pub const PLAYER_LIMB_L_SHIN: C2RustUnnamed_17 = 8;
pub const PLAYER_LIMB_L_THIGH: C2RustUnnamed_17 = 7;
pub const PLAYER_LIMB_R_FOOT: C2RustUnnamed_17 = 6;
pub const PLAYER_LIMB_R_SHIN: C2RustUnnamed_17 = 5;
pub const PLAYER_LIMB_R_THIGH: C2RustUnnamed_17 = 4;
pub const PLAYER_LIMB_LOWER: C2RustUnnamed_17 = 3;
pub const PLAYER_LIMB_WAIST: C2RustUnnamed_17 = 2;
pub const PLAYER_LIMB_ROOT: C2RustUnnamed_17 = 1;
pub const PLAYER_LIMB_NONE: C2RustUnnamed_17 = 0;
pub type C2RustUnnamed_18 = libc::c_uint;
pub const OBJECT_ID_MAX: C2RustUnnamed_18 = 402;
pub const OBJECT_ZL4: C2RustUnnamed_18 = 401;
pub const OBJECT_TIMEBLOCK: C2RustUnnamed_18 = 400;
pub const OBJECT_OUKE_HAKA: C2RustUnnamed_18 = 399;
pub const OBJECT_DOOR_KILLER: C2RustUnnamed_18 = 398;
pub const OBJECT_GI_SWORD_1: C2RustUnnamed_18 = 397;
pub const OBJECT_COB: C2RustUnnamed_18 = 396;
pub const OBJECT_COW: C2RustUnnamed_18 = 395;
pub const OBJECT_BWALL: C2RustUnnamed_18 = 394;
pub const OBJECT_PS: C2RustUnnamed_18 = 393;
pub const OBJECT_GS: C2RustUnnamed_18 = 392;
pub const OBJECT_HAKA_DOOR: C2RustUnnamed_18 = 391;
pub const OBJECT_GEFF: C2RustUnnamed_18 = 390;
pub const OBJECT_GJ: C2RustUnnamed_18 = 389;
pub const OBJECT_SKB: C2RustUnnamed_18 = 388;
pub const OBJECT_WF: C2RustUnnamed_18 = 387;
pub const OBJECT_MU: C2RustUnnamed_18 = 386;
pub const OBJECT_SPOT01_MATOYAB: C2RustUnnamed_18 = 385;
pub const OBJECT_SPOT01_MATOYA: C2RustUnnamed_18 = 384;
pub const OBJECT_GI_RUPY: C2RustUnnamed_18 = 383;
pub const OBJECT_GANON_ANIME3: C2RustUnnamed_18 = 382;
pub const OBJECT_GANON_ANIME2: C2RustUnnamed_18 = 381;
pub const OBJECT_GANON_ANIME1: C2RustUnnamed_18 = 380;
pub const OBJECT_GI_DEKUPOUCH: C2RustUnnamed_18 = 379;
pub const OBJECT_EFC_DOUGHNUT: C2RustUnnamed_18 = 378;
pub const OBJECT_DEMO_KEKKAI: C2RustUnnamed_18 = 377;
pub const OBJECT_BOWL: C2RustUnnamed_18 = 376;
pub const OBJECT_GI_SOUL: C2RustUnnamed_18 = 375;
pub const OBJECT_GI_GHOST: C2RustUnnamed_18 = 374;
pub const OBJECT_GI_BUTTERFLY: C2RustUnnamed_18 = 373;
pub const OBJECT_GI_INSECT: C2RustUnnamed_18 = 372;
pub const OBJECT_GI_FIRE: C2RustUnnamed_18 = 371;
pub const OBJECT_DNK: C2RustUnnamed_18 = 370;
pub const OBJECT_DNS: C2RustUnnamed_18 = 369;
pub const OBJECT_KIBAKO2: C2RustUnnamed_18 = 368;
pub const OBJECT_SPOT11_OBJ: C2RustUnnamed_18 = 367;
pub const OBJECT_UNSET_16E: C2RustUnnamed_18 = 366;
pub const OBJECT_JYA_DOOR: C2RustUnnamed_18 = 365;
pub const OBJECT_JYA_IRON: C2RustUnnamed_18 = 364;
pub const OBJECT_DOG: C2RustUnnamed_18 = 363;
pub const OBJECT_GR: C2RustUnnamed_18 = 362;
pub const OBJECT_GELDB: C2RustUnnamed_18 = 361;
pub const OBJECT_SHOPNUTS: C2RustUnnamed_18 = 360;
pub const OBJECT_GLA: C2RustUnnamed_18 = 359;
pub const OBJECT_SPOT00_BREAK: C2RustUnnamed_18 = 358;
pub const OBJECT_RS: C2RustUnnamed_18 = 357;
pub const OBJECT_HINTNUTS: C2RustUnnamed_18 = 356;
pub const OBJECT_BOMBIWA: C2RustUnnamed_18 = 355;
pub const OBJECT_SPOT12_OBJ: C2RustUnnamed_18 = 354;
pub const OBJECT_SPOT05_OBJECTS: C2RustUnnamed_18 = 353;
pub const OBJECT_BG: C2RustUnnamed_18 = 352;
pub const OBJECT_BIGOKUTA: C2RustUnnamed_18 = 351;
pub const OBJECT_SSH: C2RustUnnamed_18 = 350;
pub const OBJECT_GI_GODDESS: C2RustUnnamed_18 = 349;
pub const OBJECT_GI_SUTARU: C2RustUnnamed_18 = 348;
pub const OBJECT_FISH: C2RustUnnamed_18 = 347;
pub const OBJECT_EC: C2RustUnnamed_18 = 346;
pub const OBJECT_DS2: C2RustUnnamed_18 = 345;
pub const OBJECT_GI_M_ARROW: C2RustUnnamed_18 = 344;
pub const OBJECT_GI_HOVERBOOTS: C2RustUnnamed_18 = 343;
pub const OBJECT_ZG: C2RustUnnamed_18 = 342;
pub const OBJECT_TS: C2RustUnnamed_18 = 341;
pub const OBJECT_KA: C2RustUnnamed_18 = 340;
pub const OBJECT_GANON2: C2RustUnnamed_18 = 339;
pub const OBJECT_GI_GERUDOMASK: C2RustUnnamed_18 = 338;
pub const OBJECT_GI_ZORAMASK: C2RustUnnamed_18 = 337;
pub const OBJECT_GI_GOLONMASK: C2RustUnnamed_18 = 336;
pub const OBJECT_ZL2_ANIME2: C2RustUnnamed_18 = 335;
pub const OBJECT_ZL2_ANIME1: C2RustUnnamed_18 = 334;
pub const OBJECT_EFC_ERUPC: C2RustUnnamed_18 = 333;
pub const OBJECT_GT: C2RustUnnamed_18 = 332;
pub const OBJECT_DOOR_GERUDO: C2RustUnnamed_18 = 331;
pub const OBJECT_MAG: C2RustUnnamed_18 = 330;
pub const OBJECT_GI_FROG: C2RustUnnamed_18 = 329;
pub const OBJECT_GI_SOLDOUT: C2RustUnnamed_18 = 328;
pub const OBJECT_GI_BRACELET: C2RustUnnamed_18 = 327;
pub const OBJECT_GI_PRESCRIPTION: C2RustUnnamed_18 = 326;
pub const OBJECT_CS: C2RustUnnamed_18 = 325;
pub const OBJECT_JS: C2RustUnnamed_18 = 324;
pub const OBJECT_GI_BROKENSWORD: C2RustUnnamed_18 = 323;
pub const OBJECT_GI_TICKETSTONE: C2RustUnnamed_18 = 322;
pub const OBJECT_GI_MUSHROOM: C2RustUnnamed_18 = 321;
pub const OBJECT_GI_POWDER: C2RustUnnamed_18 = 320;
pub const OBJECT_GI_EYE_LOTION: C2RustUnnamed_18 = 319;
pub const OBJECT_OS: C2RustUnnamed_18 = 318;
pub const OBJECT_FA: C2RustUnnamed_18 = 317;
pub const OBJECT_MM: C2RustUnnamed_18 = 316;
pub const OBJECT_STREAM: C2RustUnnamed_18 = 315;
pub const OBJECT_SIOFUKI: C2RustUnnamed_18 = 314;
pub const OBJECT_GANON_OBJECTS: C2RustUnnamed_18 = 313;
pub const OBJECT_GI_TRUTH_MASK: C2RustUnnamed_18 = 312;
pub const OBJECT_GI_RABIT_MASK: C2RustUnnamed_18 = 311;
pub const OBJECT_GI_SKJ_MASK: C2RustUnnamed_18 = 310;
pub const OBJECT_GI_REDEAD_MASK: C2RustUnnamed_18 = 309;
pub const OBJECT_GI_KI_TAN_MASK: C2RustUnnamed_18 = 308;
pub const OBJECT_FU: C2RustUnnamed_18 = 307;
pub const OBJECT_MK: C2RustUnnamed_18 = 306;
pub const OBJECT_OWL: C2RustUnnamed_18 = 305;
pub const OBJECT_GJYO_OBJECTS: C2RustUnnamed_18 = 304;
pub const OBJECT_KANBAN: C2RustUnnamed_18 = 303;
pub const OBJECT_GI_COIN: C2RustUnnamed_18 = 302;
pub const OBJECT_GI_GLOVES: C2RustUnnamed_18 = 301;
pub const OBJECT_TSUBO: C2RustUnnamed_18 = 300;
pub const OBJECT_KUSA: C2RustUnnamed_18 = 299;
pub const OBJECT_LIGHTSWITCH: C2RustUnnamed_18 = 298;
pub const OBJECT_INGATE: C2RustUnnamed_18 = 297;
pub const OBJECT_HS: C2RustUnnamed_18 = 296;
pub const OBJECT_MS: C2RustUnnamed_18 = 295;
pub const OBJECT_GM: C2RustUnnamed_18 = 294;
pub const OBJECT_BLKOBJ: C2RustUnnamed_18 = 293;
pub const OBJECT_NWC: C2RustUnnamed_18 = 292;
pub const OBJECT_UNSET_123: C2RustUnnamed_18 = 291;
pub const OBJECT_DAIKU: C2RustUnnamed_18 = 290;
pub const OBJECT_TORYO: C2RustUnnamed_18 = 289;
pub const OBJECT_UNSET_120: C2RustUnnamed_18 = 288;
pub const OBJECT_GOROIWA: C2RustUnnamed_18 = 287;
pub const OBJECT_MAMENOKI: C2RustUnnamed_18 = 286;
pub const OBJECT_D_LIFT: C2RustUnnamed_18 = 285;
pub const OBJECT_D_HSBLOCK: C2RustUnnamed_18 = 284;
pub const OBJECT_D_ELEVATOR: C2RustUnnamed_18 = 283;
pub const OBJECT_GND_MAGIC: C2RustUnnamed_18 = 282;
pub const OBJECT_GI_SEED: C2RustUnnamed_18 = 281;
pub const OBJECT_GI_BOOTS_2: C2RustUnnamed_18 = 280;
pub const OBJECT_YABUSAME_POINT: C2RustUnnamed_18 = 279;
pub const OBJECT_GE1: C2RustUnnamed_18 = 278;
pub const OBJECT_BOB: C2RustUnnamed_18 = 277;
pub const OBJECT_FZ: C2RustUnnamed_18 = 276;
pub const OBJECT_SPOT07_OBJECT: C2RustUnnamed_18 = 275;
pub const OBJECT_SPOT03_OBJECT: C2RustUnnamed_18 = 274;
pub const OBJECT_BOJ: C2RustUnnamed_18 = 273;
pub const OBJECT_ANE: C2RustUnnamed_18 = 272;
pub const OBJECT_DS: C2RustUnnamed_18 = 271;
pub const OBJECT_GI_OCARINA_0: C2RustUnnamed_18 = 270;
pub const OBJECT_BBA: C2RustUnnamed_18 = 269;
pub const OBJECT_BJI: C2RustUnnamed_18 = 268;
pub const OBJECT_GI_BOTTLE_LETTER: C2RustUnnamed_18 = 267;
pub const OBJECT_SKJ: C2RustUnnamed_18 = 266;
pub const OBJECT_GI_NIWATORI: C2RustUnnamed_18 = 265;
pub const OBJECT_CNE: C2RustUnnamed_18 = 264;
pub const OBJECT_AHG: C2RustUnnamed_18 = 263;
pub const OBJECT_IK: C2RustUnnamed_18 = 262;
pub const OBJECT_AOB: C2RustUnnamed_18 = 261;
pub const OBJECT_MASTERZOORA: C2RustUnnamed_18 = 260;
pub const OBJECT_MASTERGOLON: C2RustUnnamed_18 = 259;
pub const OBJECT_MASTERKOKIRIHEAD: C2RustUnnamed_18 = 258;
pub const OBJECT_MASTERKOKIRI: C2RustUnnamed_18 = 257;
pub const OBJECT_UMAJUMP: C2RustUnnamed_18 = 256;
pub const OBJECT_KZ: C2RustUnnamed_18 = 255;
pub const OBJECT_ZO: C2RustUnnamed_18 = 254;
pub const OBJECT_KW1: C2RustUnnamed_18 = 253;
pub const OBJECT_KM1: C2RustUnnamed_18 = 252;
pub const OBJECT_MD: C2RustUnnamed_18 = 251;
pub const OBJECT_MD_UNUSED: C2RustUnnamed_18 = 250;
pub const OBJECT_SPOT01_OBJECTS: C2RustUnnamed_18 = 249;
pub const OBJECT_GI_LONGSWORD: C2RustUnnamed_18 = 248;
pub const OBJECT_GI_GRASS: C2RustUnnamed_18 = 247;
pub const OBJECT_GI_HAMMER: C2RustUnnamed_18 = 246;
pub const OBJECT_GI_SAW: C2RustUnnamed_18 = 245;
pub const OBJECT_GI_FISH: C2RustUnnamed_18 = 244;
pub const OBJECT_GI_BEAN: C2RustUnnamed_18 = 243;
pub const OBJECT_GI_CLOTHES: C2RustUnnamed_18 = 242;
pub const OBJECT_JYA_OBJ: C2RustUnnamed_18 = 241;
pub const OBJECT_SPOT15_OBJ: C2RustUnnamed_18 = 240;
pub const OBJECT_GI_LETTER: C2RustUnnamed_18 = 239;
pub const OBJECT_GI_SHIELD_3: C2RustUnnamed_18 = 238;
pub const OBJECT_DEMO_6K: C2RustUnnamed_18 = 237;
pub const OBJECT_ANI: C2RustUnnamed_18 = 236;
pub const OBJECT_GI_LIQUID: C2RustUnnamed_18 = 235;
pub const OBJECT_GI_GLASSES: C2RustUnnamed_18 = 234;
pub const OBJECT_GI_BOW: C2RustUnnamed_18 = 233;
pub const OBJECT_GI_BOOMERANG: C2RustUnnamed_18 = 232;
pub const OBJECT_GI_PACHINKO: C2RustUnnamed_18 = 231;
pub const OBJECT_FR: C2RustUnnamed_18 = 230;
pub const OBJECT_NY: C2RustUnnamed_18 = 229;
pub const OBJECT_UNSET_E4: C2RustUnnamed_18 = 228;
pub const OBJECT_NY_UNUSED: C2RustUnnamed_18 = 227;
pub const OBJECT_SST: C2RustUnnamed_18 = 226;
pub const OBJECT_GANON: C2RustUnnamed_18 = 225;
pub const OBJECT_MA1: C2RustUnnamed_18 = 224;
pub const OBJECT_GI_MILK: C2RustUnnamed_18 = 223;
pub const OBJECT_GI_OCARINA: C2RustUnnamed_18 = 222;
pub const OBJECT_GI_HOOKSHOT: C2RustUnnamed_18 = 221;
pub const OBJECT_GI_SHIELD_2: C2RustUnnamed_18 = 220;
pub const OBJECT_GI_SCALE: C2RustUnnamed_18 = 219;
pub const OBJECT_GI_EGG: C2RustUnnamed_18 = 218;
pub const OBJECT_GI_BOMB_2: C2RustUnnamed_18 = 217;
pub const OBJECT_GI_ARROW: C2RustUnnamed_18 = 216;
pub const OBJECT_GI_GERUDO: C2RustUnnamed_18 = 215;
pub const OBJECT_ANUBICE: C2RustUnnamed_18 = 214;
pub const OBJECT_BXA: C2RustUnnamed_18 = 213;
pub const OBJECT_RR: C2RustUnnamed_18 = 212;
pub const OBJECT_TW: C2RustUnnamed_18 = 211;
pub const OBJECT_HNI: C2RustUnnamed_18 = 210;
pub const OBJECT_GI_PURSE: C2RustUnnamed_18 = 209;
pub const OBJECT_MA2: C2RustUnnamed_18 = 208;
pub const OBJECT_OF1S: C2RustUnnamed_18 = 207;
pub const OBJECT_GI_BOMB_1: C2RustUnnamed_18 = 206;
pub const OBJECT_GI_MAGICPOT: C2RustUnnamed_18 = 205;
pub const OBJECT_DEKUJR: C2RustUnnamed_18 = 204;
pub const OBJECT_GI_SHIELD_1: C2RustUnnamed_18 = 203;
pub const OBJECT_RU2: C2RustUnnamed_18 = 202;
pub const OBJECT_OF1D_MAP: C2RustUnnamed_18 = 201;
pub const OBJECT_GI_MAP: C2RustUnnamed_18 = 200;
pub const OBJECT_GI_STICK: C2RustUnnamed_18 = 199;
pub const OBJECT_GI_BOTTLE: C2RustUnnamed_18 = 198;
pub const OBJECT_OS_ANIME: C2RustUnnamed_18 = 197;
pub const OBJECT_OE4S: C2RustUnnamed_18 = 196;
pub const OBJECT_OE1S: C2RustUnnamed_18 = 195;
pub const OBJECT_SPOT16_OBJ: C2RustUnnamed_18 = 194;
pub const OBJECT_TR: C2RustUnnamed_18 = 193;
pub const OBJECT_IN: C2RustUnnamed_18 = 192;
pub const OBJECT_GI_BOMBPOUCH: C2RustUnnamed_18 = 191;
pub const OBJECT_GI_ARROWCASE: C2RustUnnamed_18 = 190;
pub const OBJECT_GI_HEARTS: C2RustUnnamed_18 = 189;
pub const OBJECT_SA: C2RustUnnamed_18 = 188;
pub const OBJECT_GI_NUTS: C2RustUnnamed_18 = 187;
pub const OBJECT_GI_MEDAL: C2RustUnnamed_18 = 186;
pub const OBJECT_GI_BOSSKEY: C2RustUnnamed_18 = 185;
pub const OBJECT_GI_COMPASS: C2RustUnnamed_18 = 184;
pub const OBJECT_GI_HEART: C2RustUnnamed_18 = 183;
pub const OBJECT_GI_MELODY: C2RustUnnamed_18 = 182;
pub const OBJECT_SB: C2RustUnnamed_18 = 181;
pub const OBJECT_MO: C2RustUnnamed_18 = 180;
pub const OBJECT_NB: C2RustUnnamed_18 = 179;
pub const OBJECT_SHOP_DUNGEN: C2RustUnnamed_18 = 178;
pub const OBJECT_SPOT17_OBJ: C2RustUnnamed_18 = 177;
pub const OBJECT_BDOOR: C2RustUnnamed_18 = 176;
pub const OBJECT_SPOT18_OBJ: C2RustUnnamed_18 = 175;
pub const OBJECT_SPOT09_OBJ: C2RustUnnamed_18 = 174;
pub const OBJECT_GI_JEWEL: C2RustUnnamed_18 = 173;
pub const OBJECT_BROB: C2RustUnnamed_18 = 172;
pub const OBJECT_MIR_RAY: C2RustUnnamed_18 = 171;
pub const OBJECT_GI_KEY: C2RustUnnamed_18 = 170;
pub const OBJECT_DEMO_TRE_LGT: C2RustUnnamed_18 = 169;
pub const OBJECT_EFC_TW: C2RustUnnamed_18 = 168;
pub const OBJECT_RL: C2RustUnnamed_18 = 167;
pub const OBJECT_DH: C2RustUnnamed_18 = 166;
pub const OBJECT_FD2: C2RustUnnamed_18 = 165;
pub const OBJECT_SYOKUDAI: C2RustUnnamed_18 = 164;
pub const OBJECT_RU1: C2RustUnnamed_18 = 163;
pub const OBJECT_HAKA: C2RustUnnamed_18 = 162;
pub const OBJECT_SPOT02_OBJECTS: C2RustUnnamed_18 = 161;
pub const OBJECT_HORSE_LINK_CHILD: C2RustUnnamed_18 = 160;
pub const OBJECT_MEDAL: C2RustUnnamed_18 = 159;
pub const OBJECT_FW: C2RustUnnamed_18 = 158;
pub const OBJECT_DU: C2RustUnnamed_18 = 157;
pub const OBJECT_FD: C2RustUnnamed_18 = 156;
pub const OBJECT_GNDD: C2RustUnnamed_18 = 155;
pub const OBJECT_HEAVY_OBJECT: C2RustUnnamed_18 = 154;
pub const OBJECT_PO_SISTERS: C2RustUnnamed_18 = 153;
pub const OBJECT_RD: C2RustUnnamed_18 = 152;
pub const OBJECT_SD: C2RustUnnamed_18 = 151;
pub const OBJECT_BDAN_OBJECTS: C2RustUnnamed_18 = 150;
pub const OBJECT_TRIFORCE_SPOT: C2RustUnnamed_18 = 149;
pub const OBJECT_LIGHT_RING: C2RustUnnamed_18 = 148;
pub const OBJECT_GOD_LGT: C2RustUnnamed_18 = 147;
pub const OBJECT_EFC_STAR_FIELD: C2RustUnnamed_18 = 146;
pub const OBJECT_EFC_LGT_SHOWER: C2RustUnnamed_18 = 145;
pub const OBJECT_EFC_FLASH: C2RustUnnamed_18 = 144;
pub const OBJECT_EFC_FIRE_BALL: C2RustUnnamed_18 = 143;
pub const OBJECT_EFC_CRYSTAL_LIGHT: C2RustUnnamed_18 = 142;
pub const OBJECT_HAKACH_OBJECTS: C2RustUnnamed_18 = 141;
pub const OBJECT_BV: C2RustUnnamed_18 = 140;
pub const OBJECT_VM: C2RustUnnamed_18 = 139;
pub const OBJECT_XC: C2RustUnnamed_18 = 138;
pub const OBJECT_TK: C2RustUnnamed_18 = 137;
pub const OBJECT_TA: C2RustUnnamed_18 = 136;
pub const OBJECT_IM: C2RustUnnamed_18 = 135;
pub const OBJECT_VASE: C2RustUnnamed_18 = 134;
pub const OBJECT_TRAP: C2RustUnnamed_18 = 133;
pub const OBJECT_UNSET_84: C2RustUnnamed_18 = 132;
pub const OBJECT_UNSET_83: C2RustUnnamed_18 = 131;
pub const OBJECT_PU_BOX: C2RustUnnamed_18 = 130;
pub const OBJECT_LIGHTBOX: C2RustUnnamed_18 = 129;
pub const OBJECT_UNSET_80: C2RustUnnamed_18 = 128;
pub const OBJECT_UNSET_7F: C2RustUnnamed_18 = 127;
pub const OBJECT_UNSET_7E: C2RustUnnamed_18 = 126;
pub const OBJECT_UNSET_7D: C2RustUnnamed_18 = 125;
pub const OBJECT_WOOD02: C2RustUnnamed_18 = 124;
pub const OBJECT_UNSET_7B: C2RustUnnamed_18 = 123;
pub const OBJECT_UNSET_7A: C2RustUnnamed_18 = 122;
pub const OBJECT_UNSET_79: C2RustUnnamed_18 = 121;
pub const OBJECT_UNSET_78: C2RustUnnamed_18 = 120;
pub const OBJECT_BIRD: C2RustUnnamed_18 = 119;
pub const OBJECT_HATA: C2RustUnnamed_18 = 118;
pub const OBJECT_WARP2: C2RustUnnamed_18 = 117;
pub const OBJECT_SPOT08_OBJ: C2RustUnnamed_18 = 116;
pub const OBJECT_MORI_TEX: C2RustUnnamed_18 = 115;
pub const OBJECT_MORI_OBJECTS: C2RustUnnamed_18 = 114;
pub const OBJECT_MORI_HINERI2A: C2RustUnnamed_18 = 113;
pub const OBJECT_MORI_HINERI2: C2RustUnnamed_18 = 112;
pub const OBJECT_MORI_HINERI1A: C2RustUnnamed_18 = 111;
pub const OBJECT_PO_COMPOSER: C2RustUnnamed_18 = 110;
pub const OBJECT_PO_FIELD: C2RustUnnamed_18 = 109;
pub const OBJECT_RELAY_OBJECTS: C2RustUnnamed_18 = 108;
pub const OBJECT_ICE_OBJECTS: C2RustUnnamed_18 = 107;
pub const OBJECT_SPOT06_OBJECTS: C2RustUnnamed_18 = 106;
pub const OBJECT_HAKA_OBJECTS: C2RustUnnamed_18 = 105;
pub const OBJECT_MJIN_OKA: C2RustUnnamed_18 = 104;
pub const OBJECT_MJIN_WIND: C2RustUnnamed_18 = 103;
pub const OBJECT_MJIN_SOUL: C2RustUnnamed_18 = 102;
pub const OBJECT_MJIN_ICE: C2RustUnnamed_18 = 101;
pub const OBJECT_MJIN_FLAME: C2RustUnnamed_18 = 100;
pub const OBJECT_MJIN_DARK: C2RustUnnamed_18 = 99;
pub const OBJECT_MJIN_FLASH: C2RustUnnamed_18 = 98;
pub const OBJECT_MJIN: C2RustUnnamed_18 = 97;
pub const OBJECT_ZL2: C2RustUnnamed_18 = 96;
pub const OBJECT_YUKABYUN: C2RustUnnamed_18 = 95;
pub const OBJECT_TOKI_OBJECTS: C2RustUnnamed_18 = 94;
pub const OBJECT_BB: C2RustUnnamed_18 = 93;
pub const OBJECT_MORI_HINERI1: C2RustUnnamed_18 = 92;
pub const OBJECT_OSSAN: C2RustUnnamed_18 = 91;
pub const OBJECT_FHG: C2RustUnnamed_18 = 90;
pub const OBJECT_MIZU_OBJECTS: C2RustUnnamed_18 = 89;
pub const OBJECT_OA11: C2RustUnnamed_18 = 88;
pub const OBJECT_OA10: C2RustUnnamed_18 = 87;
pub const OBJECT_VALI: C2RustUnnamed_18 = 86;
pub const OBJECT_OE12: C2RustUnnamed_18 = 85;
pub const OBJECT_OE11: C2RustUnnamed_18 = 84;
pub const OBJECT_OE10: C2RustUnnamed_18 = 83;
pub const OBJECT_OE9: C2RustUnnamed_18 = 82;
pub const OBJECT_OE8: C2RustUnnamed_18 = 81;
pub const OBJECT_OE7: C2RustUnnamed_18 = 80;
pub const OBJECT_OE6: C2RustUnnamed_18 = 79;
pub const OBJECT_OE5: C2RustUnnamed_18 = 78;
pub const OBJECT_MENKURI_OBJECTS: C2RustUnnamed_18 = 77;
pub const OBJECT_OE4: C2RustUnnamed_18 = 76;
pub const OBJECT_OE3: C2RustUnnamed_18 = 75;
pub const OBJECT_DEKUNUTS: C2RustUnnamed_18 = 74;
pub const OBJECT_B_HEART: C2RustUnnamed_18 = 73;
pub const OBJECT_WARP1: C2RustUnnamed_18 = 72;
pub const OBJECT_OPENING_DEMO1: C2RustUnnamed_18 = 71;
pub const OBJECT_HORSE_ZELDA: C2RustUnnamed_18 = 70;
pub const OBJECT_OB4: C2RustUnnamed_18 = 69;
pub const OBJECT_OB3: C2RustUnnamed_18 = 68;
pub const OBJECT_OB2: C2RustUnnamed_18 = 67;
pub const OBJECT_OA9: C2RustUnnamed_18 = 66;
pub const OBJECT_OA8: C2RustUnnamed_18 = 65;
pub const OBJECT_JJ: C2RustUnnamed_18 = 64;
pub const OBJECT_OA7: C2RustUnnamed_18 = 63;
pub const OBJECT_OA6: C2RustUnnamed_18 = 62;
pub const OBJECT_OA5: C2RustUnnamed_18 = 61;
pub const OBJECT_OA4: C2RustUnnamed_18 = 60;
pub const OBJECT_OA3: C2RustUnnamed_18 = 59;
pub const OBJECT_UNSET_3A: C2RustUnnamed_18 = 58;
pub const OBJECT_DEKUBABA: C2RustUnnamed_18 = 57;
pub const OBJECT_AM: C2RustUnnamed_18 = 56;
pub const OBJECT_GND: C2RustUnnamed_18 = 55;
pub const OBJECT_YDAN_OBJECTS: C2RustUnnamed_18 = 54;
pub const OBJECT_OE2: C2RustUnnamed_18 = 53;
pub const OBJECT_OE_ANIME: C2RustUnnamed_18 = 52;
pub const OBJECT_OE1: C2RustUnnamed_18 = 51;
pub const OBJECT_SK2: C2RustUnnamed_18 = 50;
pub const OBJECT_BOMBF: C2RustUnnamed_18 = 49;
pub const OBJECT_MB: C2RustUnnamed_18 = 48;
pub const OBJECT_SPOT00_OBJECTS: C2RustUnnamed_18 = 47;
pub const OBJECT_OA2: C2RustUnnamed_18 = 46;
pub const OBJECT_HORSE_GANON: C2RustUnnamed_18 = 45;
pub const OBJECT_HIDAN_OBJECTS: C2RustUnnamed_18 = 44;
pub const OBJECT_DDAN_OBJECTS: C2RustUnnamed_18 = 43;
pub const OBJECT_SPOT04_OBJECTS: C2RustUnnamed_18 = 42;
pub const OBJECT_O_ANIME: C2RustUnnamed_18 = 41;
pub const OBJECT_OB1: C2RustUnnamed_18 = 40;
pub const OBJECT_HORSE_NORMAL: C2RustUnnamed_18 = 39;
pub const OBJECT_EI: C2RustUnnamed_18 = 38;
pub const OBJECT_BW: C2RustUnnamed_18 = 37;
pub const OBJECT_ST: C2RustUnnamed_18 = 36;
pub const OBJECT_OA1: C2RustUnnamed_18 = 35;
pub const OBJECT_TP: C2RustUnnamed_18 = 34;
pub const OBJECT_BL: C2RustUnnamed_18 = 33;
pub const OBJECT_TORCH2: C2RustUnnamed_18 = 32;
pub const OBJECT_DODOJR: C2RustUnnamed_18 = 31;
pub const OBJECT_GOL: C2RustUnnamed_18 = 30;
pub const OBJECT_ZL1: C2RustUnnamed_18 = 29;
pub const OBJECT_GOMA: C2RustUnnamed_18 = 28;
pub const OBJECT_ZF: C2RustUnnamed_18 = 27;
pub const OBJECT_HORSE: C2RustUnnamed_18 = 26;
pub const OBJECT_KINGDODONGO: C2RustUnnamed_18 = 25;
pub const OBJECT_PEEHAT: C2RustUnnamed_18 = 24;
pub const OBJECT_REEBA: C2RustUnnamed_18 = 23;
pub const OBJECT_TITE: C2RustUnnamed_18 = 22;
pub const OBJECT_LINK_CHILD: C2RustUnnamed_18 = 21;
pub const OBJECT_LINK_BOY: C2RustUnnamed_18 = 20;
pub const OBJECT_NIW: C2RustUnnamed_18 = 19;
pub const OBJECT_BUBBLE: C2RustUnnamed_18 = 18;
pub const OBJECT_UNSET_11: C2RustUnnamed_18 = 17;
pub const OBJECT_UNSET_10: C2RustUnnamed_18 = 16;
pub const OBJECT_FIRE: C2RustUnnamed_18 = 15;
pub const OBJECT_BOX: C2RustUnnamed_18 = 14;
pub const OBJECT_FIREFLY: C2RustUnnamed_18 = 13;
pub const OBJECT_DODONGO: C2RustUnnamed_18 = 12;
pub const OBJECT_WALLMASTER: C2RustUnnamed_18 = 11;
pub const OBJECT_DY_OBJ: C2RustUnnamed_18 = 10;
pub const OBJECT_POH: C2RustUnnamed_18 = 9;
pub const OBJECT_CROW: C2RustUnnamed_18 = 8;
pub const OBJECT_OKUTA: C2RustUnnamed_18 = 7;
pub const OBJECT_HUMAN: C2RustUnnamed_18 = 6;
pub const OBJECT_UNSET_5: C2RustUnnamed_18 = 5;
pub const OBJECT_UNSET_4: C2RustUnnamed_18 = 4;
pub const OBJECT_GAMEPLAY_DANGEON_KEEP: C2RustUnnamed_18 = 3;
pub const OBJECT_GAMEPLAY_FIELD_KEEP: C2RustUnnamed_18 = 2;
pub const OBJECT_GAMEPLAY_KEEP: C2RustUnnamed_18 = 1;
pub const OBJECT_INVALID: C2RustUnnamed_18 = 0;
pub type C2RustUnnamed_19 = libc::c_uint;
pub const CS_STATE_UNSKIPPABLE_EXEC: C2RustUnnamed_19 = 4;
pub const CS_STATE_UNSKIPPABLE_INIT: C2RustUnnamed_19 = 3;
pub const CS_STATE_SKIPPABLE_EXEC: C2RustUnnamed_19 = 2;
pub const CS_STATE_SKIPPABLE_INIT: C2RustUnnamed_19 = 1;
pub const CS_STATE_IDLE: C2RustUnnamed_19 = 0;
pub type C2RustUnnamed_20 = libc::c_uint;
pub const CS_CMD_END: C2RustUnnamed_20 = 65535;
pub const CS_CMD_TERMINATOR: C2RustUnnamed_20 = 1000;
pub const CS_CMD_SETTIME: C2RustUnnamed_20 = 140;
pub const CS_CMD_FADEBGM: C2RustUnnamed_20 = 124;
pub const CS_CMD_STOPBGM: C2RustUnnamed_20 = 87;
pub const CS_CMD_PLAYBGM: C2RustUnnamed_20 = 86;
pub const CS_CMD_NOP: C2RustUnnamed_20 = 11;
pub const CS_CMD_SCENE_TRANS_FX: C2RustUnnamed_20 = 45;
pub const CS_CMD_SET_ACTOR_ACTION_10: C2RustUnnamed_20 = 143;
pub const CS_CMD_SET_ACTOR_ACTION_9: C2RustUnnamed_20 = 62;
pub const CS_CMD_SET_ACTOR_ACTION_8: C2RustUnnamed_20 = 49;
pub const CS_CMD_SET_ACTOR_ACTION_7: C2RustUnnamed_20 = 31;
pub const CS_CMD_SET_ACTOR_ACTION_6: C2RustUnnamed_20 = 44;
pub const CS_CMD_SET_ACTOR_ACTION_5: C2RustUnnamed_20 = 30;
pub const CS_CMD_SET_ACTOR_ACTION_4: C2RustUnnamed_20 = 29;
pub const CS_CMD_SET_ACTOR_ACTION_3: C2RustUnnamed_20 = 25;
pub const CS_CMD_SET_ACTOR_ACTION_2: C2RustUnnamed_20 = 14;
pub const CS_CMD_SET_ACTOR_ACTION_1: C2RustUnnamed_20 = 15;
pub const CS_CMD_SET_PLAYER_ACTION: C2RustUnnamed_20 = 10;
pub const CS_CMD_TEXTBOX: C2RustUnnamed_20 = 19;
pub const CS_CMD_09: C2RustUnnamed_20 = 9;
pub const CS_CMD_08: C2RustUnnamed_20 = 8;
pub const CS_CMD_07: C2RustUnnamed_20 = 7;
pub const CS_CMD_CAM_AT_REL_TO_PLAYER: C2RustUnnamed_20 = 6;
pub const CS_CMD_CAM_EYE_REL_TO_PLAYER: C2RustUnnamed_20 = 5;
pub const CS_CMD_SET_LIGHTING: C2RustUnnamed_20 = 4;
pub const CS_CMD_MISC: C2RustUnnamed_20 = 3;
pub const CS_CMD_CAM_AT: C2RustUnnamed_20 = 2;
pub const CS_CMD_CAM_EYE: C2RustUnnamed_20 = 1;
pub const CS_CMD_00: C2RustUnnamed_20 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union CutsceneData {
    pub i: s32,
    pub f: f32_0,
    pub s: [s16; 2],
    pub b: [s8; 4],
}
pub type C2RustUnnamed_21 = libc::c_uint;
pub const ZELDAS_COURTYARD_RECEIVE_LETTER: C2RustUnnamed_21 = 119;
pub const GANON_BATTLE_TOWER_COLLAPSE: C2RustUnnamed_21 = 118;
pub const HYRULE_FIELD_SKY: C2RustUnnamed_21 = 117;
pub const DESERT_COLOSSUS_SPIRIT_BLUE_WARP_2: C2RustUnnamed_21 = 116;
pub const HYRULE_FIELD_AFTER_IMPA_ESCORT: C2RustUnnamed_21 = 115;
pub const HYRULE_FIELD_INTRO: C2RustUnnamed_21 = 114;
pub const GANONS_CASTLE_DISPEL_BARRIER_IF_CONDITIONS: C2RustUnnamed_21 = 113;
pub const GANONS_CASTLE_AFTER_SPIRIT_TRIAL: C2RustUnnamed_21 = 112;
pub const GANONS_CASTLE_AFTER_LIGHT_TRIAL: C2RustUnnamed_21 = 111;
pub const GANONS_CASTLE_AFTER_FIRE_TRIAL: C2RustUnnamed_21 = 110;
pub const GANONS_CASTLE_AFTER_SHADOW_TRIAL: C2RustUnnamed_21 = 109;
pub const GANONS_CASTLE_AFTER_WATER_TRIAL: C2RustUnnamed_21 = 108;
pub const GANONS_CASTLE_AFTER_FOREST_TRIAL: C2RustUnnamed_21 = 107;
pub const ROYAL_FAMILYS_TOMB_SUNS_SONG: C2RustUnnamed_21 = 106;
pub const GRAVEYARD_SUNS_SONG: C2RustUnnamed_21 = 105;
pub const SPIRIT_TEMPLE_BOSS_TITLE_SCREEN: C2RustUnnamed_21 = 104;
pub const HYRULE_FIELD_TITLE_SCREEN: C2RustUnnamed_21 = 103;
pub const TEMPLE_OF_TIME_FRONT_OF_PEDESTAL: C2RustUnnamed_21 = 102;
pub const DESERT_COLOSSUS_AFTER_SILVER_GAUNTLETS: C2RustUnnamed_21 = 101;
pub const KOKIRI_FOREST_AFTER_FOREST_BLUE_WARP: C2RustUnnamed_21 = 100;
pub const SACRED_FOREST_MEADOW_AFTER_FOREST_BLUE_WARP: C2RustUnnamed_21 = 99;
pub const DEATH_MOUNTAIN_CRATER_AFTER_FIRE_BLUE_WARP: C2RustUnnamed_21 = 98;
pub const GRAVEYARD_AFTER_SHADOW_BLUE_WARP: C2RustUnnamed_21 = 97;
pub const DESERT_COLOSSUS_SPIRIT_BLUE_WARP: C2RustUnnamed_21 = 96;
pub const CONDITIONAL_DESTINATION: C2RustUnnamed_21 = 95;
pub const LON_LON_RANCH_NO_CS_EPONAS_SONG: C2RustUnnamed_21 = 94;
pub const LON_LON_RANCH_NO_CS_15: C2RustUnnamed_21 = 93;
pub const LON_LON_RANCH_NO_CS_14: C2RustUnnamed_21 = 92;
pub const LON_LON_RANCH_NO_CS_13: C2RustUnnamed_21 = 91;
pub const LON_LON_RANCH_NO_CS_12: C2RustUnnamed_21 = 90;
pub const LON_LON_RANCH_NO_CS_11: C2RustUnnamed_21 = 89;
pub const LON_LON_RANCH_NO_CS_10: C2RustUnnamed_21 = 88;
pub const LON_LON_RANCH_NO_CS_9: C2RustUnnamed_21 = 87;
pub const LON_LON_RANCH_NO_CS_8: C2RustUnnamed_21 = 86;
pub const LON_LON_RANCH_NO_CS_7: C2RustUnnamed_21 = 85;
pub const LON_LON_RANCH_NO_CS_6: C2RustUnnamed_21 = 84;
pub const LON_LON_RANCH_NO_CS_5: C2RustUnnamed_21 = 83;
pub const LON_LON_RANCH_NO_CS_4: C2RustUnnamed_21 = 82;
pub const LON_LON_RANCH_NO_CS_3: C2RustUnnamed_21 = 81;
pub const LON_LON_RANCH_NO_CS_2: C2RustUnnamed_21 = 80;
pub const LON_LON_RANCH_NO_CS_1: C2RustUnnamed_21 = 79;
pub const LON_LON_RANCH_CREDITS_6: C2RustUnnamed_21 = 78;
pub const LON_LON_RANCH_CREDITS_5: C2RustUnnamed_21 = 77;
pub const LON_LON_RANCH_CREDITS_4: C2RustUnnamed_21 = 76;
pub const LON_LON_RANCH_CREDITS_3: C2RustUnnamed_21 = 75;
pub const LON_LON_RANCH_CREDITS_2: C2RustUnnamed_21 = 74;
pub const LON_LON_RANCH_CREDITS_1_2: C2RustUnnamed_21 = 73;
pub const ZELDAS_COURTYARD_CREDITS: C2RustUnnamed_21 = 72;
pub const TEMPLE_OF_TIME_CREDITS: C2RustUnnamed_21 = 71;
pub const DEATH_MOUNTAIN_TRAIL_CREDITS_2: C2RustUnnamed_21 = 70;
pub const KOKIRI_FOREST_POST_FOREST_MEDALLION: C2RustUnnamed_21 = 69;
pub const CUTSCENE_MAP_FIRE: C2RustUnnamed_21 = 68;
pub const HTRULE_FIELD_UNUSED_ENTRANCE: C2RustUnnamed_21 = 67;
pub const KAKARIKO_VILLAGE_AFTER_TRAIL_OWL: C2RustUnnamed_21 = 66;
pub const LON_LON_RANCH_CREDITS_1: C2RustUnnamed_21 = 65;
pub const HYRULE_FIELD_CREDITS: C2RustUnnamed_21 = 64;
pub const KOKIRI_FOREST_CREDITS_2: C2RustUnnamed_21 = 63;
pub const KOKIRI_FOREST_CREDITS_1: C2RustUnnamed_21 = 62;
pub const ZORAS_DOMAIN_CREDITS: C2RustUnnamed_21 = 61;
pub const ZORAS_FOUNTAIN_CREDITS: C2RustUnnamed_21 = 60;
pub const LAKE_HYLIA_CREDITS: C2RustUnnamed_21 = 59;
pub const GORON_CITY_CREDITS: C2RustUnnamed_21 = 58;
pub const DEATH_MOUNTAIN_TRAIL_CREDITS_1: C2RustUnnamed_21 = 57;
pub const KAKARIKO_VILLAGE_CREDITS: C2RustUnnamed_21 = 56;
pub const GERUDO_FORTRESS_CREDITS: C2RustUnnamed_21 = 55;
pub const GERUDO_VALLEY_CREDITS: C2RustUnnamed_21 = 54;
pub const HYRULE_FIELD_AFTER_SONG_OF_TIME: C2RustUnnamed_21 = 53;
pub const TEMPLE_OF_TIME_SONG_OF_TIME: C2RustUnnamed_21 = 52;
pub const HYRULE_FIELD_IMPA_ESCORT_CS: C2RustUnnamed_21 = 51;
pub const KAKARIKO_VILLAGE_AFTER_NOCTURNE: C2RustUnnamed_21 = 50;
pub const TEMPLE_OF_TIME_AFTER_LIGHT_ARROWS: C2RustUnnamed_21 = 49;
pub const DESERT_COLOSSUS_AFTER_REQUIEM: C2RustUnnamed_21 = 48;
pub const KAKARIKO_VILLAGE_NOCTURNE_PART_2: C2RustUnnamed_21 = 47;
pub const TEMPLE_OF_TIME_AFTER_USE_MS_FIRST_2: C2RustUnnamed_21 = 46;
pub const INVALID_DESTINATION_2D: C2RustUnnamed_21 = 45;
pub const TEMPLE_OF_TIME_AFTER_DOOR_OF_TIME_OPENS: C2RustUnnamed_21 = 44;
pub const WINDMILL_AFTER_DRAIN_WELL: C2RustUnnamed_21 = 43;
pub const KAKARIKO_VILLAGE_DRAIN_WELL: C2RustUnnamed_21 = 42;
pub const LAKE_HYLIA_AFTER_BLUE_WARP: C2RustUnnamed_21 = 41;
pub const TEMPLE_OF_TIME_GET_LIGHT_ARROWS: C2RustUnnamed_21 = 40;
pub const TEMPLE_OF_TIME_ZELDA_REVEAL: C2RustUnnamed_21 = 39;
pub const CUTSCENE_MAP_SHEIKAH_LEGEND: C2RustUnnamed_21 = 38;
pub const INVALID_DESTINATION_25: C2RustUnnamed_21 = 37;
pub const INVALID_DESTINATION_24: C2RustUnnamed_21 = 36;
pub const HYRULE_FIELD_INTRO_ZELDA_ESCAPE: C2RustUnnamed_21 = 35;
pub const CUTSCENE_MAP_GANON_AFTER_USE_MS: C2RustUnnamed_21 = 34;
pub const HYRULE_FIELD_AFTER_LAKE_HYLIA_OWL: C2RustUnnamed_21 = 33;
pub const HYRULE_FIELD_FLASHBACK: C2RustUnnamed_21 = 32;
pub const CHAMBER_OF_SAGES_WATER_MEDALLION: C2RustUnnamed_21 = 31;
pub const CHAMBER_OF_SAGES_FIRE_MEDALLION: C2RustUnnamed_21 = 30;
pub const CHAMBER_OF_SAGES_FOREST_MEDALLION: C2RustUnnamed_21 = 29;
pub const TEMPLE_OF_TIME_ZORAS_SAPPHIRE_2: C2RustUnnamed_21 = 28;
pub const TEMPLE_OF_TIME_GORON_RUBY_2: C2RustUnnamed_21 = 27;
pub const TEMPLE_OF_TIME_KOKIRI_EMERALD_2: C2RustUnnamed_21 = 26;
pub const CHAMBER_OF_SAGES_LIGHT_MEDALLION: C2RustUnnamed_21 = 25;
pub const JABU_JABU_INTRO: C2RustUnnamed_21 = 24;
pub const CUTSCENE_MAP_CURSE_YOU: C2RustUnnamed_21 = 23;
pub const DESERT_COLOSSUS_REQUIEM: C2RustUnnamed_21 = 22;
pub const LAKE_HYLIA_WATER_RISES: C2RustUnnamed_21 = 21;
pub const INVALID_DESTINATION_14: C2RustUnnamed_21 = 20;
pub const DEATH_MOUNTAIN_TRAIL_AFTER_INTRO: C2RustUnnamed_21 = 19;
pub const TEMPLE_OF_TIME_AFTER_USE_MS_FIRST: C2RustUnnamed_21 = 18;
pub const TEMPLE_OF_TIME_ZORAS_SAPPHIRE: C2RustUnnamed_21 = 17;
pub const TEMPLE_OF_TIME_GORON_RUBY: C2RustUnnamed_21 = 16;
pub const TEMPLE_OF_TIME_KOKIRI_EMERALD: C2RustUnnamed_21 = 15;
pub const KOKIRI_FOREST_AFTER_KOKIRI_EMERALD: C2RustUnnamed_21 = 14;
pub const ZORAS_FOUNTAIN_AFTER_ZORAS_SAPPHIRE: C2RustUnnamed_21 = 13;
pub const DEATH_MOUNTAIN_TRAIL_AFTER_GORON_RUBY: C2RustUnnamed_21 = 12;
pub const KOKIRI_FOREST_INTRO: C2RustUnnamed_21 = 11;
pub const LINKS_HOUSE_INTRO: C2RustUnnamed_21 = 10;
pub const GERUDO_VALLEY_DIN_2: C2RustUnnamed_21 = 9;
pub const TEMPLE_OF_TIME_AFTER_USE_MS: C2RustUnnamed_21 = 8;
pub const KOKIRI_FOREST_RECEIVE_KOKIRI_EMERALD: C2RustUnnamed_21 = 7;
pub const CUTSCENE_MAP_TRIFORCE_CREATION: C2RustUnnamed_21 = 6;
pub const KOKIRI_FOREST_FARORE: C2RustUnnamed_21 = 5;
pub const DEATH_MOUNTAIN_TRAIL_NAYRU: C2RustUnnamed_21 = 4;
pub const GERUDO_VALLEY_DIN: C2RustUnnamed_21 = 3;
pub const CUTSCENE_MAP_THREE_GODESSES_POST_DEKU_TREE: C2RustUnnamed_21 = 2;
pub const CUTSCENE_MAP_GANON_HORSE: C2RustUnnamed_21 = 1;
pub const INVALID_DESTINATION_0: C2RustUnnamed_21 = 0;
pub type C2RustUnnamed_22 = libc::c_uint;
pub const ITEM_NONE: C2RustUnnamed_22 = 255;
pub const ITEM_NONE_FE: C2RustUnnamed_22 = 254;
pub const ITEM_LAST_USED: C2RustUnnamed_22 = 252;
pub const ITEM_NUT_UPGRADE_40: C2RustUnnamed_22 = 155;
pub const ITEM_NUT_UPGRADE_30: C2RustUnnamed_22 = 154;
pub const ITEM_STICK_UPGRADE_30: C2RustUnnamed_22 = 153;
pub const ITEM_STICK_UPGRADE_20: C2RustUnnamed_22 = 152;
pub const ITEM_BOMBCHUS_20: C2RustUnnamed_22 = 151;
pub const ITEM_BOMBCHUS_5: C2RustUnnamed_22 = 150;
pub const ITEM_SEEDS_30: C2RustUnnamed_22 = 149;
pub const ITEM_ARROWS_LARGE: C2RustUnnamed_22 = 148;
pub const ITEM_ARROWS_MEDIUM: C2RustUnnamed_22 = 147;
pub const ITEM_ARROWS_SMALL: C2RustUnnamed_22 = 146;
pub const ITEM_BOMBS_30: C2RustUnnamed_22 = 145;
pub const ITEM_BOMBS_20: C2RustUnnamed_22 = 144;
pub const ITEM_BOMBS_10: C2RustUnnamed_22 = 143;
pub const ITEM_BOMBS_5: C2RustUnnamed_22 = 142;
pub const ITEM_NUTS_10: C2RustUnnamed_22 = 141;
pub const ITEM_NUTS_5: C2RustUnnamed_22 = 140;
pub const ITEM_STICKS_10: C2RustUnnamed_22 = 139;
pub const ITEM_STICKS_5: C2RustUnnamed_22 = 138;
pub const ITEM_INVALID_8: C2RustUnnamed_22 = 137;
pub const ITEM_RUPEE_GOLD: C2RustUnnamed_22 = 136;
pub const ITEM_RUPEE_PURPLE: C2RustUnnamed_22 = 135;
pub const ITEM_RUPEE_RED: C2RustUnnamed_22 = 134;
pub const ITEM_RUPEE_BLUE: C2RustUnnamed_22 = 133;
pub const ITEM_RUPEE_GREEN: C2RustUnnamed_22 = 132;
pub const ITEM_HEART: C2RustUnnamed_22 = 131;
pub const ITEM_MILK: C2RustUnnamed_22 = 130;
pub const ITEM_INVALID_7: C2RustUnnamed_22 = 129;
pub const ITEM_INVALID_6: C2RustUnnamed_22 = 128;
pub const ITEM_INVALID_5: C2RustUnnamed_22 = 127;
pub const ITEM_INVALID_4: C2RustUnnamed_22 = 126;
pub const ITEM_INVALID_3: C2RustUnnamed_22 = 125;
pub const ITEM_INVALID_2: C2RustUnnamed_22 = 124;
pub const ITEM_INVALID_1: C2RustUnnamed_22 = 123;
pub const ITEM_HEART_PIECE_2: C2RustUnnamed_22 = 122;
pub const ITEM_MAGIC_LARGE: C2RustUnnamed_22 = 121;
pub const ITEM_MAGIC_SMALL: C2RustUnnamed_22 = 120;
pub const ITEM_KEY_SMALL: C2RustUnnamed_22 = 119;
pub const ITEM_DUNGEON_MAP: C2RustUnnamed_22 = 118;
pub const ITEM_COMPASS: C2RustUnnamed_22 = 117;
pub const ITEM_KEY_BOSS: C2RustUnnamed_22 = 116;
pub const ITEM_HEART_PIECE: C2RustUnnamed_22 = 115;
pub const ITEM_HEART_CONTAINER: C2RustUnnamed_22 = 114;
pub const ITEM_SKULL_TOKEN: C2RustUnnamed_22 = 113;
pub const ITEM_GERUDO_CARD: C2RustUnnamed_22 = 112;
pub const ITEM_STONE_OF_AGONY: C2RustUnnamed_22 = 111;
pub const ITEM_ZORA_SAPPHIRE: C2RustUnnamed_22 = 110;
pub const ITEM_GORON_RUBY: C2RustUnnamed_22 = 109;
pub const ITEM_KOKIRI_EMERALD: C2RustUnnamed_22 = 108;
pub const ITEM_MEDALLION_LIGHT: C2RustUnnamed_22 = 107;
pub const ITEM_MEDALLION_SHADOW: C2RustUnnamed_22 = 106;
pub const ITEM_MEDALLION_SPIRIT: C2RustUnnamed_22 = 105;
pub const ITEM_MEDALLION_WATER: C2RustUnnamed_22 = 104;
pub const ITEM_MEDALLION_FIRE: C2RustUnnamed_22 = 103;
pub const ITEM_MEDALLION_FOREST: C2RustUnnamed_22 = 102;
pub const ITEM_SONG_STORMS: C2RustUnnamed_22 = 101;
pub const ITEM_SONG_TIME: C2RustUnnamed_22 = 100;
pub const ITEM_SONG_SUN: C2RustUnnamed_22 = 99;
pub const ITEM_SONG_SARIA: C2RustUnnamed_22 = 98;
pub const ITEM_SONG_EPONA: C2RustUnnamed_22 = 97;
pub const ITEM_SONG_LULLABY: C2RustUnnamed_22 = 96;
pub const ITEM_SONG_PRELUDE: C2RustUnnamed_22 = 95;
pub const ITEM_SONG_NOCTURNE: C2RustUnnamed_22 = 94;
pub const ITEM_SONG_REQUIEM: C2RustUnnamed_22 = 93;
pub const ITEM_SONG_SERENADE: C2RustUnnamed_22 = 92;
pub const ITEM_SONG_BOLERO: C2RustUnnamed_22 = 91;
pub const ITEM_SONG_MINUET: C2RustUnnamed_22 = 90;
pub const ITEM_FISHING_POLE: C2RustUnnamed_22 = 89;
pub const ITEM_SEEDS: C2RustUnnamed_22 = 88;
pub const ITEM_WALLET_GIANT: C2RustUnnamed_22 = 87;
pub const ITEM_WALLET_ADULT: C2RustUnnamed_22 = 86;
pub const ITEM_SWORD_KNIFE: C2RustUnnamed_22 = 85;
pub const ITEM_SCALE_GOLDEN: C2RustUnnamed_22 = 84;
pub const ITEM_SCALE_SILVER: C2RustUnnamed_22 = 83;
pub const ITEM_GAUNTLETS_GOLD: C2RustUnnamed_22 = 82;
pub const ITEM_GAUNTLETS_SILVER: C2RustUnnamed_22 = 81;
pub const ITEM_BRACELET: C2RustUnnamed_22 = 80;
pub const ITEM_BOMB_BAG_40: C2RustUnnamed_22 = 79;
pub const ITEM_BOMB_BAG_30: C2RustUnnamed_22 = 78;
pub const ITEM_BOMB_BAG_20: C2RustUnnamed_22 = 77;
pub const ITEM_QUIVER_50: C2RustUnnamed_22 = 76;
pub const ITEM_QUIVER_40: C2RustUnnamed_22 = 75;
pub const ITEM_QUIVER_30: C2RustUnnamed_22 = 74;
pub const ITEM_BULLET_BAG_50: C2RustUnnamed_22 = 73;
pub const ITEM_BULLET_BAG_40: C2RustUnnamed_22 = 72;
pub const ITEM_BULLET_BAG_30: C2RustUnnamed_22 = 71;
pub const ITEM_BOOTS_HOVER: C2RustUnnamed_22 = 70;
pub const ITEM_BOOTS_IRON: C2RustUnnamed_22 = 69;
pub const ITEM_BOOTS_KOKIRI: C2RustUnnamed_22 = 68;
pub const ITEM_TUNIC_ZORA: C2RustUnnamed_22 = 67;
pub const ITEM_TUNIC_GORON: C2RustUnnamed_22 = 66;
pub const ITEM_TUNIC_KOKIRI: C2RustUnnamed_22 = 65;
pub const ITEM_SHIELD_MIRROR: C2RustUnnamed_22 = 64;
pub const ITEM_SHIELD_HYLIAN: C2RustUnnamed_22 = 63;
pub const ITEM_SHIELD_DEKU: C2RustUnnamed_22 = 62;
pub const ITEM_SWORD_BGS: C2RustUnnamed_22 = 61;
pub const ITEM_SWORD_MASTER: C2RustUnnamed_22 = 60;
pub const ITEM_SWORD_KOKIRI: C2RustUnnamed_22 = 59;
pub const ITEM_BOW_ARROW_LIGHT: C2RustUnnamed_22 = 58;
pub const ITEM_BOW_ARROW_ICE: C2RustUnnamed_22 = 57;
pub const ITEM_BOW_ARROW_FIRE: C2RustUnnamed_22 = 56;
pub const ITEM_CLAIM_CHECK: C2RustUnnamed_22 = 55;
pub const ITEM_EYEDROPS: C2RustUnnamed_22 = 54;
pub const ITEM_FROG: C2RustUnnamed_22 = 53;
pub const ITEM_PRESCRIPTION: C2RustUnnamed_22 = 52;
pub const ITEM_SWORD_BROKEN: C2RustUnnamed_22 = 51;
pub const ITEM_SAW: C2RustUnnamed_22 = 50;
pub const ITEM_ODD_POTION: C2RustUnnamed_22 = 49;
pub const ITEM_ODD_MUSHROOM: C2RustUnnamed_22 = 48;
pub const ITEM_COJIRO: C2RustUnnamed_22 = 47;
pub const ITEM_POCKET_CUCCO: C2RustUnnamed_22 = 46;
pub const ITEM_POCKET_EGG: C2RustUnnamed_22 = 45;
pub const ITEM_SOLD_OUT: C2RustUnnamed_22 = 44;
pub const ITEM_MASK_TRUTH: C2RustUnnamed_22 = 43;
pub const ITEM_MASK_GERUDO: C2RustUnnamed_22 = 42;
pub const ITEM_MASK_ZORA: C2RustUnnamed_22 = 41;
pub const ITEM_MASK_GORON: C2RustUnnamed_22 = 40;
pub const ITEM_MASK_BUNNY: C2RustUnnamed_22 = 39;
pub const ITEM_MASK_SPOOKY: C2RustUnnamed_22 = 38;
pub const ITEM_MASK_SKULL: C2RustUnnamed_22 = 37;
pub const ITEM_MASK_KEATON: C2RustUnnamed_22 = 36;
pub const ITEM_LETTER_ZELDA: C2RustUnnamed_22 = 35;
pub const ITEM_CHICKEN: C2RustUnnamed_22 = 34;
pub const ITEM_WEIRD_EGG: C2RustUnnamed_22 = 33;
pub const ITEM_POE: C2RustUnnamed_22 = 32;
pub const ITEM_MILK_HALF: C2RustUnnamed_22 = 31;
pub const ITEM_BIG_POE: C2RustUnnamed_22 = 30;
pub const ITEM_BUG: C2RustUnnamed_22 = 29;
pub const ITEM_BLUE_FIRE: C2RustUnnamed_22 = 28;
pub const ITEM_LETTER_RUTO: C2RustUnnamed_22 = 27;
pub const ITEM_MILK_BOTTLE: C2RustUnnamed_22 = 26;
pub const ITEM_FISH: C2RustUnnamed_22 = 25;
pub const ITEM_FAIRY: C2RustUnnamed_22 = 24;
pub const ITEM_POTION_BLUE: C2RustUnnamed_22 = 23;
pub const ITEM_POTION_GREEN: C2RustUnnamed_22 = 22;
pub const ITEM_POTION_RED: C2RustUnnamed_22 = 21;
pub const ITEM_BOTTLE: C2RustUnnamed_22 = 20;
pub const ITEM_NAYRUS_LOVE: C2RustUnnamed_22 = 19;
pub const ITEM_ARROW_LIGHT: C2RustUnnamed_22 = 18;
pub const ITEM_HAMMER: C2RustUnnamed_22 = 17;
pub const ITEM_BEAN: C2RustUnnamed_22 = 16;
pub const ITEM_LENS: C2RustUnnamed_22 = 15;
pub const ITEM_BOOMERANG: C2RustUnnamed_22 = 14;
pub const ITEM_FARORES_WIND: C2RustUnnamed_22 = 13;
pub const ITEM_ARROW_ICE: C2RustUnnamed_22 = 12;
pub const ITEM_LONGSHOT: C2RustUnnamed_22 = 11;
pub const ITEM_HOOKSHOT: C2RustUnnamed_22 = 10;
pub const ITEM_BOMBCHU: C2RustUnnamed_22 = 9;
pub const ITEM_OCARINA_TIME: C2RustUnnamed_22 = 8;
pub const ITEM_OCARINA_FAIRY: C2RustUnnamed_22 = 7;
pub const ITEM_SLINGSHOT: C2RustUnnamed_22 = 6;
pub const ITEM_DINS_FIRE: C2RustUnnamed_22 = 5;
pub const ITEM_ARROW_FIRE: C2RustUnnamed_22 = 4;
pub const ITEM_BOW: C2RustUnnamed_22 = 3;
pub const ITEM_BOMB: C2RustUnnamed_22 = 2;
pub const ITEM_NUT: C2RustUnnamed_22 = 1;
pub const ITEM_STICK: C2RustUnnamed_22 = 0;
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
pub struct DemoDu {
    pub actor: Actor,
    pub skelAnime: SkelAnime,
    pub eyeTexIndex: s16,
    pub blinkTimer: s16,
    pub mouthTexIndex: s16,
    pub updateIndex: s32,
    pub drawIndex: s32,
    pub unused: s32,
    pub unk_1A4: f32_0,
    pub shadowAlpha: s32,
    pub demo6KSpawned: s32,
    pub lastAction: s32,
}
pub type DemoDu_Cutscene = libc::c_uint;
pub const DEMO_DU_CS_CREDITS: DemoDu_Cutscene = 3;
pub const DEMO_DU_CS_CHAMBER_AFTER_GANON: DemoDu_Cutscene = 2;
pub const DEMO_DU_CS_GORONS_RUBY: DemoDu_Cutscene = 1;
pub const DEMO_DU_CS_FIREMEDALLION: DemoDu_Cutscene = 0;
pub type C2RustUnnamed_23 = libc::c_uint;
pub const DEMO_EFFECT_MAX_TYPE: C2RustUnnamed_23 = 26;
pub const DEMO_EFFECT_TIMEWARP_TIMEBLOCK_SMALL: C2RustUnnamed_23 = 25;
pub const DEMO_EFFECT_TIMEWARP_TIMEBLOCK_LARGE: C2RustUnnamed_23 = 24;
pub const DEMO_EFFECT_LIGHTARROW: C2RustUnnamed_23 = 23;
pub const DEMO_EFFECT_DUST: C2RustUnnamed_23 = 22;
pub const DEMO_EFFECT_JEWEL_ZORA: C2RustUnnamed_23 = 21;
pub const DEMO_EFFECT_JEWEL_GORON: C2RustUnnamed_23 = 20;
pub const DEMO_EFFECT_JEWEL_KOKIRI: C2RustUnnamed_23 = 19;
pub const DEMO_EFFECT_LIGHT: C2RustUnnamed_23 = 18;
pub const DEMO_EFFECT_LIGHTRING_TRIFORCE: C2RustUnnamed_23 = 17;
pub const DEMO_EFFECT_LIGHTRING_SHRINKING: C2RustUnnamed_23 = 16;
pub const DEMO_EFFECT_TIMEWARP_MASTERSWORD: C2RustUnnamed_23 = 15;
pub const DEMO_EFFECT_MEDAL_LIGHT: C2RustUnnamed_23 = 14;
pub const DEMO_EFFECT_MEDAL_SHADOW: C2RustUnnamed_23 = 13;
pub const DEMO_EFFECT_MEDAL_SPIRIT: C2RustUnnamed_23 = 12;
pub const DEMO_EFFECT_MEDAL_FOREST: C2RustUnnamed_23 = 11;
pub const DEMO_EFFECT_MEDAL_WATER: C2RustUnnamed_23 = 10;
pub const DEMO_EFFECT_MEDAL_FIRE: C2RustUnnamed_23 = 9;
pub const DEMO_EFFECT_TRIFORCE_SPOT: C2RustUnnamed_23 = 8;
pub const DEMO_EFFECT_LIGHTRING_EXPANDING: C2RustUnnamed_23 = 7;
pub const DEMO_EFFECT_GOD_LGT_FARORE: C2RustUnnamed_23 = 6;
pub const DEMO_EFFECT_GOD_LGT_NAYRU: C2RustUnnamed_23 = 5;
pub const DEMO_EFFECT_GOD_LGT_DIN: C2RustUnnamed_23 = 4;
pub const DEMO_EFFECT_LGT_SHOWER: C2RustUnnamed_23 = 3;
pub const DEMO_EFFECT_BLUE_ORB: C2RustUnnamed_23 = 2;
pub const DEMO_EFFECT_FIRE_BALL: C2RustUnnamed_23 = 1;
pub const DEMO_EFFECT_CRYSTAL_LIGHT: C2RustUnnamed_23 = 0;
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
pub type DemoDuActionFunc
    =
    Option<unsafe extern "C" fn(_: *mut DemoDu, _: *mut GlobalContext) -> ()>;
pub type DemoDuDrawFunc
    =
    Option<unsafe extern "C" fn(_: *mut Actor, _: *mut GlobalContext) -> ()>;
static mut D_8096C1A4: [CutsceneData; 820] =
    [CutsceneData{i: 31 as libc::c_int,},
     CutsceneData{i: 3000 as libc::c_int,},
     CutsceneData{i: 0x20 as libc::c_int,},
     CutsceneData{i: 1 as libc::c_int,},
     CutsceneData{i: 0x10000 as libc::c_int,},
     CutsceneData{i: 0xbb80000 as libc::c_int,},
     CutsceneData{i: 0 as libc::c_int,}, CutsceneData{i: 0 as libc::c_int,},
     CutsceneData{i: 0xfffffffc as libc::c_uint as s32,},
     CutsceneData{i: 0x2 as libc::c_int,}, CutsceneData{i: 0 as libc::c_int,},
     CutsceneData{i: 0xfffffffc as libc::c_uint as s32,},
     CutsceneData{i: 0x2 as libc::c_int,}, CutsceneData{i: 0 as libc::c_int,},
     CutsceneData{i: 0 as libc::c_int,}, CutsceneData{i: 0 as libc::c_int,},
     CutsceneData{i: 31 as libc::c_int,}, CutsceneData{i: 5 as libc::c_int,},
     CutsceneData{i:
                      ((0x1 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((546 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: 0 as libc::c_int,}, CutsceneData{i: 216 as libc::c_int,},
     CutsceneData{i: -(10 as libc::c_int),},
     CutsceneData{i: 0 as libc::c_int,}, CutsceneData{i: 216 as libc::c_int,},
     CutsceneData{i: -(10 as libc::c_int),}, CutsceneData{f: 0.0f32,},
     CutsceneData{f: 0.0f32,}, CutsceneData{f: 0.0f32,},
     CutsceneData{i:
                      ((0x2 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (546 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((547 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: 0 as libc::c_int,}, CutsceneData{i: 216 as libc::c_int,},
     CutsceneData{i: -(10 as libc::c_int),},
     CutsceneData{i: 0 as libc::c_int,}, CutsceneData{i: 216 as libc::c_int,},
     CutsceneData{i: -(10 as libc::c_int),}, CutsceneData{f: 0.0f32,},
     CutsceneData{f: 0.0f32,}, CutsceneData{f: 0.0f32,},
     CutsceneData{i:
                      ((0x4 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (547 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((616 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: 0 as libc::c_int,}, CutsceneData{i: 216 as libc::c_int,},
     CutsceneData{i: -(10 as libc::c_int),},
     CutsceneData{i: 0 as libc::c_int,}, CutsceneData{i: 216 as libc::c_int,},
     CutsceneData{i: -(10 as libc::c_int),}, CutsceneData{f: 0.0f32,},
     CutsceneData{f: 0.0f32,}, CutsceneData{f: 0.0f32,},
     CutsceneData{i:
                      ((0x2 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (616 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((667 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: 0 as libc::c_int,}, CutsceneData{i: 216 as libc::c_int,},
     CutsceneData{i: -(10 as libc::c_int),},
     CutsceneData{i: 0 as libc::c_int,}, CutsceneData{i: 82 as libc::c_int,},
     CutsceneData{i: 0 as libc::c_int,}, CutsceneData{f: 0.0f32,},
     CutsceneData{f: -2.627451f32,}, CutsceneData{f: 0.0f32,},
     CutsceneData{i:
                      ((0x3 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (667 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((2834 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: 0 as libc::c_int,}, CutsceneData{i: 82 as libc::c_int,},
     CutsceneData{i: 0 as libc::c_int,}, CutsceneData{i: 0 as libc::c_int,},
     CutsceneData{i: 82 as libc::c_int,}, CutsceneData{i: 0 as libc::c_int,},
     CutsceneData{f: 0.0f32,}, CutsceneData{f: 0.0f32,},
     CutsceneData{f: 0.0f32,},
     CutsceneData{i: CS_CMD_SET_PLAYER_ACTION as libc::c_int,},
     CutsceneData{i: 3 as libc::c_int,},
     CutsceneData{i:
                      ((0xd as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((280 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0x6aaa as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: 0 as libc::c_int,}, CutsceneData{i: 6 as libc::c_int,},
     CutsceneData{i: 0 as libc::c_int,}, CutsceneData{i: 0 as libc::c_int,},
     CutsceneData{i: 6 as libc::c_int,}, CutsceneData{i: 0 as libc::c_int,},
     CutsceneData{f: 0.0f32,}, CutsceneData{f: 0.0f32,},
     CutsceneData{f: 1.401298464324817e-45f32,},
     CutsceneData{i:
                      ((0x5 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (280 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((531 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0x6aaa as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: 0 as libc::c_int,}, CutsceneData{i: 6 as libc::c_int,},
     CutsceneData{i: 0 as libc::c_int,}, CutsceneData{i: 0 as libc::c_int,},
     CutsceneData{i: 6 as libc::c_int,}, CutsceneData{i: 0 as libc::c_int,},
     CutsceneData{f: 0.0f32,}, CutsceneData{f: 0.0f32,},
     CutsceneData{f: 1.401298464324817e-45f32,},
     CutsceneData{i:
                      ((0x13 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (531 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((1716 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0xeaaa as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: 0 as libc::c_int,}, CutsceneData{i: 6 as libc::c_int,},
     CutsceneData{i: 0 as libc::c_int,}, CutsceneData{i: 0 as libc::c_int,},
     CutsceneData{i: 6 as libc::c_int,}, CutsceneData{i: 0 as libc::c_int,},
     CutsceneData{f: 0.0f32,}, CutsceneData{f: 0.0f32,},
     CutsceneData{f: 1.401298464324817e-45f32,},
     CutsceneData{i: 41 as libc::c_int,}, CutsceneData{i: 3 as libc::c_int,},
     CutsceneData{i:
                      ((0x1 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((170 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: 98 as libc::c_int,}, CutsceneData{i: 6 as libc::c_int,},
     CutsceneData{i: -(169 as libc::c_int),},
     CutsceneData{i: 98 as libc::c_int,}, CutsceneData{i: 6 as libc::c_int,},
     CutsceneData{i: -(169 as libc::c_int),}, CutsceneData{f: 0.0f32,},
     CutsceneData{f: 0.0f32,}, CutsceneData{f: 0.0f32,},
     CutsceneData{i:
                      ((0x2 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (170 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((465 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: 98 as libc::c_int,}, CutsceneData{i: 6 as libc::c_int,},
     CutsceneData{i: -(169 as libc::c_int),},
     CutsceneData{i: 98 as libc::c_int,}, CutsceneData{i: 6 as libc::c_int,},
     CutsceneData{i: -(169 as libc::c_int),}, CutsceneData{f: 0.0f32,},
     CutsceneData{f: 0.0f32,}, CutsceneData{f: 0.0f32,},
     CutsceneData{i:
                      ((0x3 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (465 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((2915 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: 98 as libc::c_int,}, CutsceneData{i: 6 as libc::c_int,},
     CutsceneData{i: -(169 as libc::c_int),},
     CutsceneData{i: 98 as libc::c_int,}, CutsceneData{i: 6 as libc::c_int,},
     CutsceneData{i: -(169 as libc::c_int),}, CutsceneData{f: 0.0f32,},
     CutsceneData{f: 0.0f32,}, CutsceneData{f: 0.0f32,},
     CutsceneData{i: 49 as libc::c_int,}, CutsceneData{i: 1 as libc::c_int,},
     CutsceneData{i:
                      ((0x1 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((3000 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: 0 as libc::c_int,},
     CutsceneData{i: -(16 as libc::c_int),},
     CutsceneData{i: -(121 as libc::c_int),},
     CutsceneData{i: 0 as libc::c_int,},
     CutsceneData{i: -(16 as libc::c_int),},
     CutsceneData{i: -(121 as libc::c_int),}, CutsceneData{f: 0.0f32,},
     CutsceneData{f: 0.0f32,}, CutsceneData{f: 0.0f32,},
     CutsceneData{i: CS_CMD_SCENE_TRANS_FX as libc::c_int,},
     CutsceneData{i: 0x1 as libc::c_int,},
     CutsceneData{i:
                      ((0x1 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (530 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((539 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (539 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: CS_CMD_SCENE_TRANS_FX as libc::c_int,},
     CutsceneData{i: 0x1 as libc::c_int,},
     CutsceneData{i:
                      ((0x5 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (540 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((570 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (570 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: CS_CMD_SET_LIGHTING as libc::c_int,},
     CutsceneData{i: 2 as libc::c_int,},
     CutsceneData{i:
                      ((0x1 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((10 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: 0 as libc::c_int,},
     CutsceneData{i: 0xffffffdc as libc::c_uint as s32,},
     CutsceneData{i: 0 as libc::c_int,},
     CutsceneData{i: 0x18 as libc::c_int,},
     CutsceneData{i: 0xffffffdc as libc::c_uint as s32,},
     CutsceneData{i: 0 as libc::c_int,},
     CutsceneData{i: 0x18 as libc::c_int,},
     CutsceneData{i: 0 as libc::c_int,}, CutsceneData{i: 0 as libc::c_int,},
     CutsceneData{i: 0 as libc::c_int,},
     CutsceneData{i:
                      ((0x1 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (10 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((3000 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: 0 as libc::c_int,},
     CutsceneData{i: 0xffffffdc as libc::c_uint as s32,},
     CutsceneData{i: 0 as libc::c_int,},
     CutsceneData{i: 0x18 as libc::c_int,},
     CutsceneData{i: 0xffffffdc as libc::c_uint as s32,},
     CutsceneData{i: 0 as libc::c_int,},
     CutsceneData{i: 0x18 as libc::c_int,},
     CutsceneData{i: 0 as libc::c_int,}, CutsceneData{i: 0 as libc::c_int,},
     CutsceneData{i: 0 as libc::c_int,}, CutsceneData{i: 39 as libc::c_int,},
     CutsceneData{i: 1 as libc::c_int,},
     CutsceneData{i:
                      ((0x1 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((3000 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: 0 as libc::c_int,}, CutsceneData{i: 0 as libc::c_int,},
     CutsceneData{i: -(2 as libc::c_int),},
     CutsceneData{i: 0 as libc::c_int,}, CutsceneData{i: 0 as libc::c_int,},
     CutsceneData{i: -(2 as libc::c_int),}, CutsceneData{f: 0.0f32,},
     CutsceneData{f: 0.0f32,}, CutsceneData{f: 0.0f32,},
     CutsceneData{i: CS_CMD_SCENE_TRANS_FX as libc::c_int,},
     CutsceneData{i: 0x1 as libc::c_int,},
     CutsceneData{i:
                      ((0x1 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (805 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((835 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (835 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: 62 as libc::c_int,}, CutsceneData{i: 1 as libc::c_int,},
     CutsceneData{i:
                      ((0x4 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((3000 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: 32 as libc::c_int,}, CutsceneData{i: 80 as libc::c_int,},
     CutsceneData{i: -(51 as libc::c_int),},
     CutsceneData{i: 32 as libc::c_int,}, CutsceneData{i: 80 as libc::c_int,},
     CutsceneData{i: -(51 as libc::c_int),}, CutsceneData{f: 0.0f32,},
     CutsceneData{f: 0.0f32,}, CutsceneData{f: 0.0f32,},
     CutsceneData{i: CS_CMD_TERMINATOR as libc::c_int,},
     CutsceneData{i: 0x1 as libc::c_int,},
     CutsceneData{i:
                      ((DEATH_MOUNTAIN_CRATER_AFTER_FIRE_BLUE_WARP as
                            libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (905 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((1030 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (1030 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: CS_CMD_PLAYBGM as libc::c_int,},
     CutsceneData{i: 1 as libc::c_int,},
     CutsceneData{i:
                      ((0x44 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (615 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((616 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: 0 as libc::c_int,},
     CutsceneData{i: 0xffffff97 as libc::c_uint as s32,},
     CutsceneData{i: 0 as libc::c_int,},
     CutsceneData{i: 0x30 as libc::c_int,},
     CutsceneData{i: 0xffffff97 as libc::c_uint as s32,},
     CutsceneData{i: 0 as libc::c_int,},
     CutsceneData{i: 0x30 as libc::c_int,},
     CutsceneData{i: 0 as libc::c_int,}, CutsceneData{i: 0 as libc::c_int,},
     CutsceneData{i: 0 as libc::c_int,},
     CutsceneData{i: CS_CMD_FADEBGM as libc::c_int,},
     CutsceneData{i: 1 as libc::c_int,},
     CutsceneData{i:
                      ((0x4 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (500 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((550 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: 0 as libc::c_int,}, CutsceneData{i: 0 as libc::c_int,},
     CutsceneData{i: 0xffffffc3 as libc::c_uint as s32,},
     CutsceneData{i: 0x6e as libc::c_int,},
     CutsceneData{i: 0 as libc::c_int,},
     CutsceneData{i: 0xffffffc3 as libc::c_uint as s32,},
     CutsceneData{i: 0x6e as libc::c_int,},
     CutsceneData{i: 0 as libc::c_int,}, CutsceneData{i: 0 as libc::c_int,},
     CutsceneData{i: 0 as libc::c_int,},
     CutsceneData{i: CS_CMD_TEXTBOX as libc::c_int,},
     CutsceneData{i: 10 as libc::c_int,},
     CutsceneData{i:
                      ((0xffff as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((310 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xffff as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0xffff as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xffff as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0x303c as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (310 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((323 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0xffff as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (323 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((344 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xffff as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0xffff as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xffff as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0x3045 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (344 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((394 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0xffff as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (394 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((415 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xffff as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0xffff as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xffff as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0x3046 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (415 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((465 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0xffff as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (465 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((800 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xffff as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0xffff as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xffff as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0x3c as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (800 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((805 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0xffff as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (805 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((865 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xffff as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0xffff as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xffff as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0x303d as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (865 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((875 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: CS_CMD_CAM_EYE as libc::c_int,},
     CutsceneData{i:
                      ((0x1 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((1361 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.4f32,},
     CutsceneData{i:
                      ((-(85 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (3211 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((795 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xc6 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.4f32,},
     CutsceneData{i:
                      ((-(85 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (3211 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((795 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xc8 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.4f32,},
     CutsceneData{i:
                      ((-(85 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (2925 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((795 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xd7 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.4f32,},
     CutsceneData{i:
                      ((70 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (974 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((497 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xe8 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.4f32,},
     CutsceneData{i:
                      ((320 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (268 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((296 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xea as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.4f32,},
     CutsceneData{i:
                      ((312 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (190 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((150 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x13d as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.4f32,},
     CutsceneData{i:
                      ((261 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (61 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(65 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x13f as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.4f32,},
     CutsceneData{i:
                      ((261 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (61 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(65 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x14e as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.4f32,},
     CutsceneData{i:
                      ((261 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (61 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(65 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x15f as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.4f32,},
     CutsceneData{i:
                      ((261 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (61 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(65 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x161 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: CS_CMD_CAM_EYE as libc::c_int,},
     CutsceneData{i:
                      ((0x1 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (263 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((509 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 29.399885f32,},
     CutsceneData{i:
                      ((89 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (30 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(103 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xc6 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.2f32,},
     CutsceneData{i:
                      ((89 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (30 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(103 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xc8 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.2f32,},
     CutsceneData{i:
                      ((89 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (30 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(103 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xd7 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.2f32,},
     CutsceneData{i:
                      ((89 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (30 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(103 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xe8 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.2f32,},
     CutsceneData{i:
                      ((89 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (30 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(103 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xea as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: CS_CMD_CAM_EYE as libc::c_int,},
     CutsceneData{i:
                      ((0x1 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (333 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((1424 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.999947f32,},
     CutsceneData{i:
                      ((114 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (50 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(116 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xc6 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.999947f32,},
     CutsceneData{i:
                      ((114 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (50 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(116 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xc8 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.999947f32,},
     CutsceneData{i:
                      ((114 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (50 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(116 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xd7 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.999947f32,},
     CutsceneData{i:
                      ((114 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (50 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(116 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xe8 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.999947f32,},
     CutsceneData{i:
                      ((114 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (50 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(116 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xea as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: CS_CMD_CAM_EYE as libc::c_int,},
     CutsceneData{i:
                      ((0x1 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (403 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((1494 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.599945f32,},
     CutsceneData{i:
                      ((26 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (45 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(10 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xc6 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.599945f32,},
     CutsceneData{i:
                      ((26 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (45 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(10 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xc8 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.599945f32,},
     CutsceneData{i:
                      ((26 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (45 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(10 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xd7 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.599945f32,},
     CutsceneData{i:
                      ((26 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (45 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(10 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xe8 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.599945f32,},
     CutsceneData{i:
                      ((26 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (45 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(10 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xea as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: CS_CMD_CAM_EYE as libc::c_int,},
     CutsceneData{i:
                      ((0x1 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (443 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((1624 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.399944f32,},
     CutsceneData{i:
                      ((11 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (23 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(17 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xc6 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.399944f32,},
     CutsceneData{i:
                      ((11 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (23 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(17 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xc8 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.399944f32,},
     CutsceneData{i:
                      ((27 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (31 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(45 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xd7 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.399944f32,},
     CutsceneData{i:
                      ((45 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (40 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(76 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xe8 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.399944f32,},
     CutsceneData{i:
                      ((45 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (40 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(76 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xea as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.399944f32,},
     CutsceneData{i:
                      ((45 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (40 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(76 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x13d as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.399944f32,},
     CutsceneData{i:
                      ((45 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (40 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(76 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x13f as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.399944f32,},
     CutsceneData{i:
                      ((45 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (40 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(76 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x2e as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: CS_CMD_CAM_EYE as libc::c_int,},
     CutsceneData{i:
                      ((0x1 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (473 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((1604 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.199944f32,},
     CutsceneData{i:
                      ((192 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (29 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(246 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xc6 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.199944f32,},
     CutsceneData{i:
                      ((192 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (29 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(246 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xc8 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.199944f32,},
     CutsceneData{i:
                      ((192 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (29 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(246 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xd7 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.199944f32,},
     CutsceneData{i:
                      ((192 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (29 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(246 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xe8 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 70.39992f32,},
     CutsceneData{i:
                      ((192 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (278 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(246 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xea as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 70.39992f32,},
     CutsceneData{i:
                      ((192 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (278 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(246 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x13d as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 70.39992f32,},
     CutsceneData{i:
                      ((192 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (278 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(246 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x13f as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 70.39992f32,},
     CutsceneData{i:
                      ((192 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (278 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(246 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x2e as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 70.39992f32,},
     CutsceneData{i:
                      ((192 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (278 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(246 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x63 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: CS_CMD_CAM_EYE as libc::c_int,},
     CutsceneData{i:
                      ((0x1 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (539 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((881 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.0f32,},
     CutsceneData{i:
                      ((13 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (854 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((2 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xc6 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.0f32,},
     CutsceneData{i:
                      ((9 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (853 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((5 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xc8 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.0f32,},
     CutsceneData{i:
                      ((-(3 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (853 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((5 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xd7 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.0f32,},
     CutsceneData{i:
                      ((-(9 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (853 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(6 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xe8 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.0f32,},
     CutsceneData{i:
                      ((-(2 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (852 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(17 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xea as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.0f32,},
     CutsceneData{i:
                      ((9 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (852 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(17 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x13d as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.0f32,},
     CutsceneData{i:
                      ((16 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (852 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(6 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x13f as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.0f32,},
     CutsceneData{i:
                      ((9 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (852 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((5 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x2e as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.0f32,},
     CutsceneData{i:
                      ((-(3 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (851 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((5 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x63 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: CS_CMD_CAM_EYE_REL_TO_PLAYER as libc::c_int,},
     CutsceneData{i:
                      ((0x1 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (615 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((1796 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 68.599945f32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (33 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(27 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xc6 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 68.599945f32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (33 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(27 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xc8 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 68.599945f32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (68 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(26 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xd7 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 68.599945f32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (103 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(26 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xe8 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 68.599945f32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (103 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(26 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xea as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 68.599945f32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (103 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(26 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x13d as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 68.599945f32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (103 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(26 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x13f as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
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
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 68.599945f32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (103 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(26 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x2e as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: CS_CMD_CAM_AT as libc::c_int,},
     CutsceneData{i:
                      ((0x1 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((1390 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (60 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.4f32,},
     CutsceneData{i:
                      ((-(115 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (3163 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((585 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xc6 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (60 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.4f32,},
     CutsceneData{i:
                      ((-(115 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (3163 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((585 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xc8 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (60 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.4f32,},
     CutsceneData{i:
                      ((-(115 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (2877 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((585 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xd7 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (30 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.4f32,},
     CutsceneData{i:
                      ((27 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (824 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((348 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xe8 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (40 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.4f32,},
     CutsceneData{i:
                      ((197 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (143 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((174 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xea as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (40 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.4f32,},
     CutsceneData{i:
                      ((147 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (100 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((55 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x13d as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (40 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.4f32,},
     CutsceneData{i:
                      ((55 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (35 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(65 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x13f as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (1000 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.4f32,},
     CutsceneData{i:
                      ((55 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (35 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(65 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x14e as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (30 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.4f32,},
     CutsceneData{i:
                      ((55 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (35 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(65 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x15f as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (30 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.4f32,},
     CutsceneData{i:
                      ((56 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (35 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(65 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x161 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: CS_CMD_CAM_AT as libc::c_int,},
     CutsceneData{i:
                      ((0x1 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (263 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((538 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (30 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 78.99979f32,},
     CutsceneData{i:
                      ((87 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (157 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(391 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xc6 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (82 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 78.79979f32,},
     CutsceneData{i:
                      ((87 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (157 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(391 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xc8 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (103 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 70.79991f32,},
     CutsceneData{i:
                      ((87 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (157 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(391 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xd7 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (30 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.2f32,},
     CutsceneData{i:
                      ((87 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (157 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(391 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xe8 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (30 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.2f32,},
     CutsceneData{i:
                      ((87 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (157 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(391 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xea as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: CS_CMD_CAM_AT as libc::c_int,},
     CutsceneData{i:
                      ((0x1 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (333 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((1453 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (30 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.999947f32,},
     CutsceneData{i:
                      ((-(52 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (127 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(309 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xc6 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (30 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.999947f32,},
     CutsceneData{i:
                      ((-(52 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (127 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(309 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xc8 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (1000 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.999947f32,},
     CutsceneData{i:
                      ((-(52 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (127 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(309 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xd7 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (30 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.999947f32,},
     CutsceneData{i:
                      ((-(52 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (127 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(309 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xe8 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (30 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.999947f32,},
     CutsceneData{i:
                      ((-(52 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (127 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(309 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xea as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: CS_CMD_CAM_AT as libc::c_int,},
     CutsceneData{i:
                      ((0x1 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (403 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((1523 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (30 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.599945f32,},
     CutsceneData{i:
                      ((-(269 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (186 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((13 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xc6 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (30 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.599945f32,},
     CutsceneData{i:
                      ((-(269 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (186 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((13 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xc8 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (1000 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.599945f32,},
     CutsceneData{i:
                      ((-(269 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (186 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((13 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xd7 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (30 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.599945f32,},
     CutsceneData{i:
                      ((-(269 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (186 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((13 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xe8 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (30 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.599945f32,},
     CutsceneData{i:
                      ((-(269 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (186 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((13 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xea as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: CS_CMD_CAM_AT as libc::c_int,},
     CutsceneData{i:
                      ((0x1 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (443 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((1653 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (30 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.399944f32,},
     CutsceneData{i:
                      ((163 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (70 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(283 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xc6 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (30 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.399944f32,},
     CutsceneData{i:
                      ((163 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (70 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(283 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xc8 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (30 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.399944f32,},
     CutsceneData{i:
                      ((177 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (74 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(309 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xd7 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (30 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.399944f32,},
     CutsceneData{i:
                      ((193 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (92 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(337 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xe8 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (30 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.399944f32,},
     CutsceneData{i:
                      ((193 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (92 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(337 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xea as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (1000 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.399944f32,},
     CutsceneData{i:
                      ((192 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (91 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(336 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x13d as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (30 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.399944f32,},
     CutsceneData{i:
                      ((192 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (91 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(336 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x13f as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (30 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.399944f32,},
     CutsceneData{i:
                      ((192 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (91 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(336 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x2e as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: CS_CMD_CAM_AT as libc::c_int,},
     CutsceneData{i:
                      ((0x1 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (473 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((1633 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (20 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.199944f32,},
     CutsceneData{i:
                      ((-(9 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (57 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(53 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xc6 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (20 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.199944f32,},
     CutsceneData{i:
                      ((-(9 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (57 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(53 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xc8 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (20 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.199944f32,},
     CutsceneData{i:
                      ((-(9 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (57 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(54 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xd7 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (20 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 45.999947f32,},
     CutsceneData{i:
                      ((-(8 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (57 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(54 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xe8 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (10 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 70.39992f32,},
     CutsceneData{i:
                      ((149 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (547 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(205 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xea as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (10 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 70.39992f32,},
     CutsceneData{i:
                      ((149 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (547 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(205 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x13d as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (1000 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 70.39992f32,},
     CutsceneData{i:
                      ((149 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (547 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(205 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x13f as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (30 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 70.39992f32,},
     CutsceneData{i:
                      ((149 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (547 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(205 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x2e as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (30 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 70.39992f32,},
     CutsceneData{i:
                      ((149 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (547 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(205 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x63 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: CS_CMD_CAM_AT as libc::c_int,},
     CutsceneData{i:
                      ((0x1 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (539 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((930 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (50 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.0f32,},
     CutsceneData{i:
                      ((3 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (6 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(6 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xc6 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (40 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 60.0f32,},
     CutsceneData{i:
                      ((3 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (6 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(6 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xc8 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (30 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 50.999966f32,},
     CutsceneData{i:
                      ((3 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (6 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(6 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xd7 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (20 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 20.59985f32,},
     CutsceneData{i:
                      ((3 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (6 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(6 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xe8 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (51 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 10.799838f32,},
     CutsceneData{i:
                      ((3 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (6 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(6 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xea as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (50 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 10.399838f32,},
     CutsceneData{i:
                      ((3 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (6 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(6 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x13d as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (50 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 10.399838f32,},
     CutsceneData{i:
                      ((3 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (6 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(6 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x13f as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (50 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 10.199839f32,},
     CutsceneData{i:
                      ((3 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (6 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(6 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x2e as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (50 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 10.999838f32,},
     CutsceneData{i:
                      ((3 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (6 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(6 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x63 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: CS_CMD_CAM_AT_REL_TO_PLAYER as libc::c_int,},
     CutsceneData{i:
                      ((0x1 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (615 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((1825 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (30 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 68.599945f32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (100 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((5 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xc6 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (30 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 68.599945f32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (101 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((6 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xc8 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (30 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 68.599945f32,},
     CutsceneData{i:
                      ((1 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (99 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((41 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xd7 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (30 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 68.599945f32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (42 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((16 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xe8 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (30 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 68.599945f32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (42 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((16 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0xea as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (1000 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 68.599945f32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (42 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((16 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x13d as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (30 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 68.599945f32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (42 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((16 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x13f as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((-(1 as libc::c_int) as u32_0 &
                            (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           24 as libc::c_int |
                           (0 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               16 as libc::c_int |
                           (30 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{f: 68.599945f32,},
     CutsceneData{i:
                      ((0 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (42 as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i:
                      ((16 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           16 as libc::c_int |
                           (0x2e as libc::c_int as u32_0 &
                                (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                     1 as libc::c_int) as libc::c_uint) <<
                               0 as libc::c_int) as s32,},
     CutsceneData{i: 0xffffffff as libc::c_uint as s32,},
     CutsceneData{i: 0 as libc::c_int,}];
static mut sEyeTextures: [*mut libc::c_void; 4] =
    unsafe {
        [gDaruniaEyeOpenTex.as_ptr() as *mut _ as *mut libc::c_void,
         gDaruniaEyeOpeningTex.as_ptr() as *mut _ as *mut libc::c_void,
         gDaruniaEyeShutTex.as_ptr() as *mut _ as *mut libc::c_void,
         gDaruniaEyeClosingTex.as_ptr() as *mut _ as *mut libc::c_void]
    };
static mut sMouthTextures: [*mut libc::c_void; 4] =
    unsafe {
        [gDaruniaMouthSeriousTex.as_ptr() as *mut _ as *mut libc::c_void,
         gDaruniaMouthGrinningTex.as_ptr() as *mut _ as *mut libc::c_void,
         gDaruniaMouthOpenTex.as_ptr() as *mut _ as *mut libc::c_void,
         gDaruniaMouthHappyTex.as_ptr() as *mut _ as *mut libc::c_void]
    };
// DEMO_DU_CS_CREDITS
#[no_mangle]
pub unsafe extern "C" fn DemoDu_Destroy(mut thisx: *mut Actor,
                                        mut globalCtx: *mut GlobalContext) {
    let mut this: *mut DemoDu = thisx as *mut DemoDu;
    SkelAnime_Free(&mut (*this).skelAnime, globalCtx);
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_UpdateEyes(mut this: *mut DemoDu) {
    let mut blinkTimer: *mut s16 = &mut (*this).blinkTimer;
    let mut eyeTexIndex: *mut s16 = &mut (*this).eyeTexIndex;
    let mut pad: [s32; 3] = [0; 3];
    if (if *blinkTimer as libc::c_int == 0 as libc::c_int {
            0 as libc::c_int
        } else { *blinkTimer -= 1; *blinkTimer as libc::c_int }) ==
           0 as libc::c_int {
        *blinkTimer =
            Rand_S16Offset(60 as libc::c_int as s16, 60 as libc::c_int as s16)
    }
    *eyeTexIndex = *blinkTimer;
    if *eyeTexIndex as libc::c_int >= 3 as libc::c_int {
        *eyeTexIndex = 0 as libc::c_int as s16
    };
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_SetEyeTexIndex(mut this: *mut DemoDu,
                                               mut eyeTexIndex: s16) {
    (*this).eyeTexIndex = eyeTexIndex;
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_SetMouthTexIndex(mut this: *mut DemoDu,
                                                 mut mouthTexIndex: s16) {
    (*this).mouthTexIndex = mouthTexIndex;
}
// Resets all the values used in this cutscene.
#[no_mangle]
pub unsafe extern "C" fn DemoDu_CsAfterGanon_Reset(mut this: *mut DemoDu) {
    (*this).updateIndex = 21 as libc::c_int + 0 as libc::c_int;
    (*this).drawIndex = 0 as libc::c_int;
    (*this).shadowAlpha = 0 as libc::c_int;
    (*this).demo6KSpawned = 0 as libc::c_int;
    (*this).actor.shape.shadowAlpha = 0 as libc::c_int as u8_0;
    (*this).unk_1A4 = 0.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_CsAfterGanon_CheckIfShouldReset(mut this:
                                                                    *mut DemoDu,
                                                                mut globalCtx:
                                                                    *mut GlobalContext) {
    static mut D_8096CE94: s32 = 0 as libc::c_int;
    if (*globalCtx).csCtx.state as libc::c_int == CS_STATE_IDLE as libc::c_int
       {
        if D_8096CE94 != 0 {
            if (*this).actor.params as libc::c_int ==
                   DEMO_DU_CS_CHAMBER_AFTER_GANON as libc::c_int {
                DemoDu_CsAfterGanon_Reset(this);
            }
            D_8096CE94 = 0 as libc::c_int;
            return
        }
    } else if D_8096CE94 == 0 { D_8096CE94 = 1 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_UpdateSkelAnime(mut this: *mut DemoDu)
 -> s32 {
    return SkelAnime_Update(&mut (*this).skelAnime);
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_UpdateBgCheckInfo(mut this: *mut DemoDu,
                                                  mut globalCtx:
                                                      *mut GlobalContext) {
    Actor_UpdateBgCheckInfo(globalCtx, &mut (*this).actor, 75.0f32, 30.0f32,
                            30.0f32, 5 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_GetNpcAction(mut globalCtx:
                                                 *mut GlobalContext,
                                             mut idx: s32)
 -> *mut CsCmdActorAction {
    if (*globalCtx).csCtx.state as libc::c_int != CS_STATE_IDLE as libc::c_int
       {
        return (*globalCtx).csCtx.npcActions[idx as usize]
    }
    return 0 as *mut CsCmdActorAction;
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_IsNpcDoingThisAction(mut this: *mut DemoDu,
                                                     mut globalCtx:
                                                         *mut GlobalContext,
                                                     mut action: u16_0,
                                                     mut idx: s32) -> s32 {
    let mut npcAction: *mut CsCmdActorAction =
        DemoDu_GetNpcAction(globalCtx, idx);
    if !npcAction.is_null() &&
           (*npcAction).action as libc::c_int == action as libc::c_int {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_IsNpcNotDoingThisAction(mut this: *mut DemoDu,
                                                        mut globalCtx:
                                                            *mut GlobalContext,
                                                        mut action: u16_0,
                                                        mut idx: s32) -> s32 {
    let mut npcAction: *mut CsCmdActorAction =
        DemoDu_GetNpcAction(globalCtx, idx);
    if !npcAction.is_null() &&
           (*npcAction).action as libc::c_int != action as libc::c_int {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_MoveToNpcPos(mut this: *mut DemoDu,
                                             mut globalCtx:
                                                 *mut GlobalContext,
                                             mut idx: s32) {
    let mut npcAction: *mut CsCmdActorAction =
        DemoDu_GetNpcAction(globalCtx, idx);
    let mut pad: s32 = 0;
    if !npcAction.is_null() {
        (*this).actor.world.pos.x = (*npcAction).startPos.x as f32_0;
        (*this).actor.world.pos.y = (*npcAction).startPos.y as f32_0;
        (*this).actor.world.pos.z = (*npcAction).startPos.z as f32_0;
        (*this).actor.shape.rot.y = (*npcAction).c2rust_unnamed.rot.y;
        (*this).actor.world.rot.y = (*this).actor.shape.rot.y
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_80969DDC(mut this: *mut DemoDu,
                                       mut animation: *mut AnimationHeader,
                                       mut mode: u8_0, mut morphFrames: f32_0,
                                       mut arg4: s32) {
    let mut startFrame: f32_0 = 0.;
    let mut lastFrame: s16 =
        Animation_GetLastFrame(animation as *mut libc::c_void);
    let mut endFrame: f32_0 = 0.;
    let mut playSpeed: f32_0 = 0.;
    if arg4 == 0 as libc::c_int {
        startFrame = 0.0f32;
        endFrame = lastFrame as f32_0;
        playSpeed = 1.0f32
    } else {
        endFrame = 0.0f32;
        playSpeed = -1.0f32;
        startFrame = lastFrame as f32_0
    }
    Animation_Change(&mut (*this).skelAnime, animation, playSpeed, startFrame,
                     endFrame, mode, morphFrames);
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_InitCs_FireMedallion(mut this: *mut DemoDu,
                                                     mut globalCtx:
                                                         *mut GlobalContext) {
    SkelAnime_InitFlex(globalCtx, &mut (*this).skelAnime, &mut gDaruniaSkel,
                       &mut gDaruniaIdleAnim, 0 as *mut Vec3s,
                       0 as *mut Vec3s, 0 as libc::c_int);
    (*this).actor.shape.yOffset = -10000.0f32;
    DemoDu_SetEyeTexIndex(this, 1 as libc::c_int as s16);
    DemoDu_SetMouthTexIndex(this, 3 as libc::c_int as s16);
}
// A.k.a Warp portal
#[no_mangle]
pub unsafe extern "C" fn DemoDu_CsFireMedallion_SpawnDoorWarp(mut this:
                                                                  *mut DemoDu,
                                                              mut globalCtx:
                                                                  *mut GlobalContext) {
    let mut posX: f32_0 = (*this).actor.world.pos.x;
    let mut posY: f32_0 = (*this).actor.world.pos.y;
    let mut posZ: f32_0 = (*this).actor.world.pos.z;
    Actor_SpawnAsChild(&mut (*globalCtx).actorCtx, &mut (*this).actor,
                       globalCtx, ACTOR_DOOR_WARP1 as libc::c_int as s16,
                       posX, posY, posZ, 0 as libc::c_int as s16,
                       0 as libc::c_int as s16, 0 as libc::c_int as s16,
                       WARP_SAGES as libc::c_int as s16);
}
// Gives the Fire Medallion to Link.
#[no_mangle]
pub unsafe extern "C" fn func_80969F38(mut this: *mut DemoDu,
                                       mut globalCtx: *mut GlobalContext) {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut posX: f32_0 = (*player).actor.world.pos.x;
    let mut posY: f32_0 = (*player).actor.world.pos.y + 80.0f32;
    let mut posZ: f32_0 = (*player).actor.world.pos.z;
    Actor_SpawnAsChild(&mut (*globalCtx).actorCtx, &mut (*this).actor,
                       globalCtx, ACTOR_DEMO_EFFECT as libc::c_int as s16,
                       posX, posY, posZ, 0 as libc::c_int as s16,
                       0 as libc::c_int as s16, 0 as libc::c_int as s16,
                       DEMO_EFFECT_MEDAL_FIRE as libc::c_int as s16);
    Item_Give(globalCtx, ITEM_MEDALLION_FIRE as libc::c_int as u8_0);
}
#[no_mangle]
pub unsafe extern "C" fn func_80969FB4(mut this: *mut DemoDu,
                                       mut globalCtx: *mut GlobalContext) {
    (*this).actor.shape.yOffset += 250.0f32 / 3.0f32;
}
// Gives the Fire Medallion to Link too.
#[no_mangle]
pub unsafe extern "C" fn DemoDu_CsFireMedallion_AdvanceTo01(mut this:
                                                                *mut DemoDu,
                                                            mut globalCtx:
                                                                *mut GlobalContext) {
    let mut pad: [s32; 2] = [0; 2];
    if gSaveContext.chamberCutsceneNum as libc::c_int == 1 as libc::c_int &&
           gSaveContext.sceneSetupIndex < 4 as libc::c_int {
        let mut player: *mut Player =
            (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                                 usize].head as *mut Player;
        (*this).updateIndex = 0 as libc::c_int + 1 as libc::c_int;
        (*globalCtx).csCtx.segment =
            D_8096C1A4.as_mut_ptr() as *mut libc::c_void;
        gSaveContext.cutsceneTrigger = 2 as libc::c_int as u8_0;
        Item_Give(globalCtx, ITEM_MEDALLION_FIRE as libc::c_int as u8_0);
        (*player).actor.shape.rot.y =
            ((*this).actor.world.rot.y as libc::c_int + 0x8000 as libc::c_int)
                as s16;
        (*player).actor.world.rot.y = (*player).actor.shape.rot.y
    };
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_CsFireMedallion_AdvanceTo02(mut this:
                                                                *mut DemoDu,
                                                            mut globalCtx:
                                                                *mut GlobalContext) {
    if (*globalCtx).csCtx.state as libc::c_int != CS_STATE_IDLE as libc::c_int
       {
        let mut npcAction: *mut CsCmdActorAction =
            (*globalCtx).csCtx.npcActions[2 as libc::c_int as usize];
        if !npcAction.is_null() &&
               (*npcAction).action as libc::c_int != 1 as libc::c_int {
            (*this).updateIndex = 0 as libc::c_int + 2 as libc::c_int;
            (*this).drawIndex = 1 as libc::c_int;
            DemoDu_CsFireMedallion_SpawnDoorWarp(this, globalCtx);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_CsFireMedallion_AdvanceTo03(mut this:
                                                                *mut DemoDu) {
    if (*this).actor.shape.yOffset >= 0.0f32 {
        (*this).updateIndex = 0 as libc::c_int + 3 as libc::c_int;
        (*this).actor.shape.yOffset = 0.0f32
    };
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_CsFireMedallion_AdvanceTo04(mut this:
                                                                *mut DemoDu,
                                                            mut globalCtx:
                                                                *mut GlobalContext) {
    if (*globalCtx).csCtx.state as libc::c_int != CS_STATE_IDLE as libc::c_int
       {
        let mut npcAction: *mut CsCmdActorAction =
            (*globalCtx).csCtx.npcActions[2 as libc::c_int as usize];
        if !npcAction.is_null() &&
               (*npcAction).action as libc::c_int != 2 as libc::c_int {
            Animation_Change(&mut (*this).skelAnime,
                             &mut gDaruniaItemGiveAnim, 1.0f32, 0.0f32,
                             Animation_GetLastFrame(&mut gDaruniaItemGiveAnim
                                                        as
                                                        *mut AnimationHeader
                                                        as *mut libc::c_void)
                                 as f32_0, 2 as libc::c_int as u8_0, 0.0f32);
            (*this).updateIndex = 0 as libc::c_int + 4 as libc::c_int
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_CsFireMedallion_AdvanceTo05(mut this:
                                                                *mut DemoDu,
                                                            mut animFinished:
                                                                s32) {
    if animFinished != 0 {
        Animation_Change(&mut (*this).skelAnime,
                         &mut gDaruniaItemGiveIdleAnim, 1.0f32, 0.0f32,
                         Animation_GetLastFrame(&mut gDaruniaItemGiveIdleAnim
                                                    as *mut AnimationHeader as
                                                    *mut libc::c_void) as
                             f32_0, 0 as libc::c_int as u8_0, 0.0f32);
        (*this).updateIndex = 0 as libc::c_int + 5 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_CsFireMedallion_AdvanceTo06(mut this:
                                                                *mut DemoDu,
                                                            mut globalCtx:
                                                                *mut GlobalContext) {
    if (*globalCtx).csCtx.state as libc::c_int != CS_STATE_IDLE as libc::c_int
       {
        let mut npcAction: *mut CsCmdActorAction =
            (*globalCtx).csCtx.npcActions[6 as libc::c_int as usize];
        if !npcAction.is_null() &&
               (*npcAction).action as libc::c_int == 2 as libc::c_int {
            (*this).updateIndex = 0 as libc::c_int + 6 as libc::c_int;
            func_80969F38(this, globalCtx);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_UpdateCs_FM_00(mut this: *mut DemoDu,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    DemoDu_CsFireMedallion_AdvanceTo01(this, globalCtx);
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_UpdateCs_FM_01(mut this: *mut DemoDu,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    DemoDu_CsFireMedallion_AdvanceTo02(this, globalCtx);
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_UpdateCs_FM_02(mut this: *mut DemoDu,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    func_80969FB4(this, globalCtx);
    DemoDu_UpdateSkelAnime(this);
    DemoDu_CsFireMedallion_AdvanceTo03(this);
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_UpdateCs_FM_03(mut this: *mut DemoDu,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    DemoDu_UpdateBgCheckInfo(this, globalCtx);
    DemoDu_UpdateSkelAnime(this);
    DemoDu_CsFireMedallion_AdvanceTo04(this, globalCtx);
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_UpdateCs_FM_04(mut this: *mut DemoDu,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    let mut animFinished: s32 = 0;
    DemoDu_UpdateBgCheckInfo(this, globalCtx);
    animFinished = DemoDu_UpdateSkelAnime(this);
    DemoDu_CsFireMedallion_AdvanceTo05(this, animFinished);
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_UpdateCs_FM_05(mut this: *mut DemoDu,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    DemoDu_UpdateBgCheckInfo(this, globalCtx);
    DemoDu_UpdateSkelAnime(this);
    DemoDu_CsFireMedallion_AdvanceTo06(this, globalCtx);
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_UpdateCs_FM_06(mut this: *mut DemoDu,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    DemoDu_UpdateBgCheckInfo(this, globalCtx);
    DemoDu_UpdateSkelAnime(this);
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_InitCs_GoronsRuby(mut this: *mut DemoDu,
                                                  mut globalCtx:
                                                      *mut GlobalContext) {
    SkelAnime_InitFlex(globalCtx, &mut (*this).skelAnime, &mut gDaruniaSkel,
                       0 as *mut AnimationHeader, 0 as *mut Vec3s,
                       0 as *mut Vec3s, 0 as libc::c_int);
    (*this).updateIndex = 7 as libc::c_int + 0 as libc::c_int;
}
// Cutscene: Darunia gives Link the Goron's Ruby.
// Sfx played when Darunia lands at the floor at the start of the cutscene.
#[no_mangle]
pub unsafe extern "C" fn DemoDu_CsPlaySfx_GoronLanding(mut this:
                                                           *mut DemoDu) {
    func_80078914(&mut (*this).actor.projectedPos,
                  0x3879 as libc::c_int as u16_0);
}
// Cutscene: Darunia gives Link the Goron's Ruby.
// Sfx played when Darunia is falling at the start of the cutscene.
#[no_mangle]
pub unsafe extern "C" fn DemoDu_CsPlaySfx_DaruniaFalling(mut globalCtx:
                                                             *mut GlobalContext) {
    if (*globalCtx).csCtx.frames as libc::c_int == 160 as libc::c_int {
        func_800788CC(0x28a0 as libc::c_int as u16_0);
    };
}
// Cutscene: Darunia gives Link the Goron's Ruby.
#[no_mangle]
pub unsafe extern "C" fn DemoDu_CsPlaySfx_DaruniaHitsLink(mut globalCtx:
                                                              *mut GlobalContext) {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut pad: s32 = 0;
    func_80078914(&mut (*player).actor.projectedPos,
                  0x3925 as libc::c_int as u16_0);
    Audio_PlaySoundGeneral(0x6825 as libc::c_int as u16_0,
                           &mut (*player).actor.projectedPos,
                           4 as libc::c_int as u8_0, &mut D_801333E0,
                           &mut D_801333E0, &mut D_801333E8);
}
// Cutscene: Darunia gives Link the Goron's Ruby.
#[no_mangle]
pub unsafe extern "C" fn DemoDu_CsPlaySfx_HitBreast(mut this: *mut DemoDu) {
    func_80078914(&mut (*this).actor.projectedPos,
                  (0x3924 as libc::c_int - 0x800 as libc::c_int) as u16_0);
}
// Cutscene: Darunia gives Link the Goron's Ruby.
// Sfx played when Link is escaping from the gorons at the end of the scene.
#[no_mangle]
pub unsafe extern "C" fn DemoDu_CsPlaySfx_LinkEscapeFromGorons(mut globalCtx:
                                                                   *mut GlobalContext) {
    if (*globalCtx).csCtx.frames as libc::c_int == 1400 as libc::c_int {
        let mut player: *mut Player =
            (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                                 usize].head as *mut Player;
        Audio_PlaySoundGeneral(0x6828 as libc::c_int as u16_0,
                               &mut (*player).actor.projectedPos,
                               4 as libc::c_int as u8_0, &mut D_801333E0,
                               &mut D_801333E0, &mut D_801333E8);
    };
}
// Cutscene: Darunia gives Link the Goron's Ruby.
// Sfx played when Link is surprised by Darunia falling from the sky.
#[no_mangle]
pub unsafe extern "C" fn DemoDu_CsPlaySfx_LinkSurprised(mut globalCtx:
                                                            *mut GlobalContext) {
    if (*globalCtx).csCtx.frames as libc::c_int == 174 as libc::c_int {
        let mut player: *mut Player =
            (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                                 usize].head as *mut Player;
        Audio_PlaySoundGeneral(0x6836 as libc::c_int as u16_0,
                               &mut (*player).actor.projectedPos,
                               4 as libc::c_uint as u8_0, &mut D_801333E0,
                               &mut D_801333E0, &mut D_801333E8);
    };
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_CsGoronsRuby_UpdateFaceTextures(mut this:
                                                                    *mut DemoDu,
                                                                mut globalCtx:
                                                                    *mut GlobalContext) {
    let mut frames: *mut u16_0 = &mut (*globalCtx).csCtx.frames;
    if (*frames as libc::c_int) < 260 as libc::c_int {
        DemoDu_UpdateEyes(this);
        DemoDu_SetMouthTexIndex(this, 0 as libc::c_int as s16);
    } else if (*frames as libc::c_int) < 335 as libc::c_int {
        DemoDu_UpdateEyes(this);
        DemoDu_SetMouthTexIndex(this, 3 as libc::c_int as s16);
    } else if (*frames as libc::c_int) < 365 as libc::c_int {
        DemoDu_SetEyeTexIndex(this, 3 as libc::c_int as s16);
        DemoDu_SetMouthTexIndex(this, 1 as libc::c_int as s16);
    } else if (*frames as libc::c_int) < 395 as libc::c_int {
        DemoDu_SetEyeTexIndex(this, 0 as libc::c_int as s16);
        DemoDu_SetMouthTexIndex(this, 3 as libc::c_int as s16);
    } else if (*frames as libc::c_int) < 410 as libc::c_int {
        DemoDu_UpdateEyes(this);
        DemoDu_SetMouthTexIndex(this, 0 as libc::c_int as s16);
    } else {
        DemoDu_UpdateEyes(this);
        DemoDu_SetMouthTexIndex(this, 3 as libc::c_int as s16);
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_8096A630(mut this: *mut DemoDu,
                                       mut globalCtx: *mut GlobalContext) {
    let mut pad: s32 = 0;
    let mut pos: Vec3f = (*this).actor.world.pos;
    pos.y +=
        (*gGameInfo).data[(27 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 5 as libc::c_int) as usize]
            as libc::c_int as libc::c_float;
    func_80033480(globalCtx, &mut pos,
                  (*gGameInfo).data[(27 as libc::c_int * 6 as libc::c_int *
                                         16 as libc::c_int + 1 as libc::c_int)
                                        as usize] as libc::c_int as
                      libc::c_float + 100.0f32,
                  (*gGameInfo).data[(27 as libc::c_int * 6 as libc::c_int *
                                         16 as libc::c_int + 2 as libc::c_int)
                                        as usize] as libc::c_int +
                      10 as libc::c_int,
                  ((*gGameInfo).data[(27 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          3 as libc::c_int) as usize] as
                       libc::c_int + 300 as libc::c_int) as s16,
                  (*gGameInfo).data[(27 as libc::c_int * 6 as libc::c_int *
                                         16 as libc::c_int + 4 as libc::c_int)
                                        as usize], 0 as libc::c_int as u8_0);
    DemoDu_CsPlaySfx_GoronLanding(this);
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_CsGoronsRuby_SpawnDustWhenHittingLink(mut this:
                                                                          *mut DemoDu,
                                                                      mut globalCtx:
                                                                          *mut GlobalContext) {
    static mut dustPosOffsets: [Vec3f; 10] =
        [{ let mut init = Vec3f{x: 11.0f32, y: -11.0f32, z: -6.0f32,}; init },
         { let mut init = Vec3f{x: 0.0f32, y: 14.0f32, z: -13.0f32,}; init },
         { let mut init = Vec3f{x: 14.0f32, y: -2.0f32, z: -10.0f32,}; init },
         { let mut init = Vec3f{x: 10.0f32, y: -6.0f32, z: -8.0f32,}; init },
         { let mut init = Vec3f{x: 8.0f32, y: 6.0f32, z: 8.0f32,}; init },
         { let mut init = Vec3f{x: 13.0f32, y: 8.0f32, z: -10.0f32,}; init },
         { let mut init = Vec3f{x: -14.0f32, y: 1.0f32, z: -14.0f32,}; init },
         { let mut init = Vec3f{x: 5.0f32, y: 12.0f32, z: -9.0f32,}; init },
         { let mut init = Vec3f{x: 11.0f32, y: 6.0f32, z: -7.0f32,}; init },
         {
             let mut init = Vec3f{x: 14.0f32, y: 14.0f32, z: -14.0f32,};
             init
         }];
    if Animation_OnFrame(&mut (*this).skelAnime, 31.0f32) != 0 ||
           Animation_OnFrame(&mut (*this).skelAnime, 41.0f32) != 0 {
        let mut pad: [s32; 2] = [0; 2];
        let mut i: s32 = 0;
        let mut player: *mut Player =
            (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                                 usize].head as *mut Player;
        let mut headPos: *mut Vec3f =
            &mut *(*player).bodyPartsPos.as_mut_ptr().offset(PLAYER_LIMB_HEAD
                                                                 as
                                                                 libc::c_int
                                                                 as isize) as
                *mut Vec3f;
        let mut velocity: Vec3f =
            { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
        let mut accel: Vec3f =
            { let mut init = Vec3f{x: 0.0f32, y: 0.3f32, z: 0.0f32,}; init };
        let mut pad2: s32 = 0;
        i = 4 as libc::c_int;
        while i >= 0 as libc::c_int {
            let mut primColor: Color_RGBA8 =
                {
                    let mut init =
                        Color_RGBA8{r: 190 as libc::c_int as u8_0,
                                    g: 150 as libc::c_int as u8_0,
                                    b: 110 as libc::c_int as u8_0,
                                    a: 255 as libc::c_int as u8_0,};
                    init
                };
            let mut envColor: Color_RGBA8 =
                {
                    let mut init =
                        Color_RGBA8{r: 120 as libc::c_int as u8_0,
                                    g: 80 as libc::c_int as u8_0,
                                    b: 40 as libc::c_int as u8_0,
                                    a: 255 as libc::c_int as u8_0,};
                    init
                };
            let mut colorDelta: s32 = 0;
            let mut position: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
            if Animation_OnFrame(&mut (*this).skelAnime, 31.0f32) != 0 {
                position.x =
                    dustPosOffsets[(i + 5 as libc::c_int) as usize].x +
                        (*headPos).x;
                position.y =
                    dustPosOffsets[(i + 5 as libc::c_int) as usize].y +
                        (*headPos).y;
                position.z =
                    dustPosOffsets[(i + 5 as libc::c_int) as usize].z +
                        (*headPos).z
            } else {
                position.x =
                    dustPosOffsets[(i + 0 as libc::c_int) as usize].x +
                        (*headPos).x;
                position.y =
                    dustPosOffsets[(i + 0 as libc::c_int) as usize].y +
                        (*headPos).y;
                position.z =
                    dustPosOffsets[(i + 0 as libc::c_int) as usize].z +
                        (*headPos).z
            }
            colorDelta = (Rand_ZeroOne() * 20.0f32 - 10.0f32) as s32;
            primColor.r = (primColor.r as libc::c_int + colorDelta) as u8_0;
            primColor.g = (primColor.g as libc::c_int + colorDelta) as u8_0;
            primColor.b = (primColor.b as libc::c_int + colorDelta) as u8_0;
            envColor.r = (envColor.r as libc::c_int + colorDelta) as u8_0;
            envColor.g = (envColor.g as libc::c_int + colorDelta) as u8_0;
            envColor.b = (envColor.b as libc::c_int + colorDelta) as u8_0;
            func_8002829C(globalCtx, &mut position, &mut velocity, &mut accel,
                          &mut primColor, &mut envColor,
                          (Rand_ZeroOne() * 40.0f32 + 200.0f32) as s16,
                          0 as libc::c_int as s16);
            i -= 1
        }
        DemoDu_CsPlaySfx_DaruniaHitsLink(globalCtx);
    };
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_CsGoronsRuby_DaruniaFalling(mut this:
                                                                *mut DemoDu,
                                                            mut globalCtx:
                                                                *mut GlobalContext) {
    let mut pad: s32 = 0;
    let mut csCtx: *mut CutsceneContext = &mut (*globalCtx).csCtx;
    if (*csCtx).state as libc::c_int != CS_STATE_IDLE as libc::c_int {
        let mut npcAction: *mut CsCmdActorAction =
            (*csCtx).npcActions[2 as libc::c_int as usize];
        let mut startPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
        let mut endPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
        let mut pos: *mut Vec3f = &mut (*this).actor.world.pos;
        if !npcAction.is_null() {
            let mut traveledPercent: f32_0 =
                Environment_LerpWeight((*npcAction).endFrame,
                                       (*npcAction).startFrame,
                                       (*csCtx).frames);
            startPos.x = (*npcAction).startPos.x as f32_0;
            startPos.y = (*npcAction).startPos.y as f32_0;
            startPos.z = (*npcAction).startPos.z as f32_0;
            endPos.x = (*npcAction).endPos.x as f32_0;
            endPos.y = (*npcAction).endPos.y as f32_0;
            endPos.z = (*npcAction).endPos.z as f32_0;
            (*pos).x = (endPos.x - startPos.x) * traveledPercent + startPos.x;
            (*pos).y = (endPos.y - startPos.y) * traveledPercent + startPos.y;
            (*pos).z = (endPos.z - startPos.z) * traveledPercent + startPos.z
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_CsGoronsRuby_AdvanceTo01(mut this:
                                                             *mut DemoDu,
                                                         mut globalCtx:
                                                             *mut GlobalContext) {
    (*this).updateIndex = 7 as libc::c_int + 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_CsGoronsRuby_AdvanceTo02(mut this:
                                                             *mut DemoDu,
                                                         mut globalCtx:
                                                             *mut GlobalContext) {
    if (*globalCtx).csCtx.state as libc::c_int != CS_STATE_IDLE as libc::c_int
       {
        let mut npcAction: *mut CsCmdActorAction =
            (*globalCtx).csCtx.npcActions[2 as libc::c_int as usize];
        if !npcAction.is_null() &&
               (*npcAction).action as libc::c_int != 1 as libc::c_int {
            Animation_Change(&mut (*this).skelAnime,
                             &mut gDaruniaStandUpAfterFallingAnim, 1.0f32,
                             0.0f32,
                             Animation_GetLastFrame(&mut gDaruniaStandUpAfterFallingAnim
                                                        as
                                                        *mut AnimationHeader
                                                        as *mut libc::c_void)
                                 as f32_0, 2 as libc::c_int as u8_0, 0.0f32);
            (*this).updateIndex = 7 as libc::c_int + 2 as libc::c_int;
            (*this).drawIndex = 1 as libc::c_int;
            DemoDu_CsGoronsRuby_DaruniaFalling(this, globalCtx);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_CsGoronsRuby_AdvanceTo03(mut this:
                                                             *mut DemoDu,
                                                         mut globalCtx:
                                                             *mut GlobalContext) {
    let mut csCtx: *mut CutsceneContext = &mut (*globalCtx).csCtx;
    if (*csCtx).state as libc::c_int != CS_STATE_IDLE as libc::c_int {
        let mut npcAction: *mut CsCmdActorAction =
            (*csCtx).npcActions[2 as libc::c_int as usize];
        if !npcAction.is_null() &&
               (*csCtx).frames as libc::c_int >=
                   (*npcAction).endFrame as libc::c_int {
            (*this).updateIndex = 7 as libc::c_int + 3 as libc::c_int;
            func_8096A630(this, globalCtx);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_CsGoronsRuby_AdvanceTo04(mut this:
                                                             *mut DemoDu,
                                                         mut globalCtx:
                                                             *mut GlobalContext) {
    if (*globalCtx).csCtx.state as libc::c_int != CS_STATE_IDLE as libc::c_int
       {
        let mut npcAction: *mut CsCmdActorAction =
            (*globalCtx).csCtx.npcActions[2 as libc::c_int as usize];
        if !npcAction.is_null() &&
               (*npcAction).action as libc::c_int != 2 as libc::c_int {
            (*this).updateIndex = 7 as libc::c_int + 4 as libc::c_int
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_CsGoronsRuby_AdvanceTo05(mut this:
                                                             *mut DemoDu,
                                                         mut animFinished:
                                                             s32) {
    if animFinished != 0 {
        Animation_Change(&mut (*this).skelAnime, &mut gDaruniaIdleAnim,
                         1.0f32, 0.0f32,
                         Animation_GetLastFrame(&mut gDaruniaIdleAnim as
                                                    *mut AnimationHeader as
                                                    *mut libc::c_void) as
                             f32_0, ANIMMODE_LOOP as libc::c_int as u8_0,
                         0.0f32);
        (*this).updateIndex = 7 as libc::c_int + 5 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_CsGoronsRuby_AdvanceTo06(mut this:
                                                             *mut DemoDu,
                                                         mut globalCtx:
                                                             *mut GlobalContext) {
    if (*globalCtx).csCtx.state as libc::c_int != CS_STATE_IDLE as libc::c_int
       {
        let mut npcAction: *mut CsCmdActorAction =
            (*globalCtx).csCtx.npcActions[2 as libc::c_int as usize];
        if !npcAction.is_null() &&
               (*npcAction).action as libc::c_int != 3 as libc::c_int {
            Animation_Change(&mut (*this).skelAnime,
                             &mut gDaruniaHitBreastAnim, 1.0f32, 0.0f32,
                             Animation_GetLastFrame(&mut gDaruniaHitBreastAnim
                                                        as
                                                        *mut AnimationHeader
                                                        as *mut libc::c_void)
                                 as f32_0, 2 as libc::c_int as u8_0, -4.0f32);
            (*this).updateIndex = 7 as libc::c_int + 6 as libc::c_int
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_CsGoronsRuby_AdvanceTo07(mut this:
                                                             *mut DemoDu,
                                                         mut animFinished:
                                                             s32) {
    if animFinished != 0 {
        Animation_Change(&mut (*this).skelAnime, &mut gDaruniaIdleAnim,
                         1.0f32, 0.0f32,
                         Animation_GetLastFrame(&mut gDaruniaIdleAnim as
                                                    *mut AnimationHeader as
                                                    *mut libc::c_void) as
                             f32_0, ANIMMODE_LOOP as libc::c_int as u8_0,
                         0.0f32);
        (*this).updateIndex = 7 as libc::c_int + 7 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_CsGoronsRuby_AdvanceTo08(mut this:
                                                             *mut DemoDu,
                                                         mut globalCtx:
                                                             *mut GlobalContext) {
    if (*globalCtx).csCtx.state as libc::c_int != CS_STATE_IDLE as libc::c_int
       {
        let mut npcAction: *mut CsCmdActorAction =
            (*globalCtx).csCtx.npcActions[2 as libc::c_int as usize];
        if !npcAction.is_null() &&
               (*npcAction).action as libc::c_int != 4 as libc::c_int {
            Animation_Change(&mut (*this).skelAnime, &mut gDaruniaHitLinkAnim,
                             1.0f32, 0.0f32,
                             Animation_GetLastFrame(&mut gDaruniaHitLinkAnim
                                                        as
                                                        *mut AnimationHeader
                                                        as *mut libc::c_void)
                                 as f32_0, 2 as libc::c_int as u8_0, 0.0f32);
            (*this).updateIndex = 7 as libc::c_int + 8 as libc::c_int
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_CsGoronsRuby_AdvanceTo09(mut this:
                                                             *mut DemoDu,
                                                         mut animFinished:
                                                             s32) {
    if animFinished != 0 {
        Animation_Change(&mut (*this).skelAnime, &mut gDaruniaHitBreastAnim,
                         1.0f32, 0.0f32,
                         Animation_GetLastFrame(&mut gDaruniaHitBreastAnim as
                                                    *mut AnimationHeader as
                                                    *mut libc::c_void) as
                             f32_0, 2 as libc::c_int as u8_0, 0.0f32);
        (*this).updateIndex = 7 as libc::c_int + 9 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_CsGoronsRuby_AdvanceTo10(mut this:
                                                             *mut DemoDu,
                                                         mut animFinished:
                                                             s32) {
    if animFinished != 0 {
        Animation_Change(&mut (*this).skelAnime, &mut gDaruniaIdleAnim,
                         1.0f32, 0.0f32,
                         Animation_GetLastFrame(&mut gDaruniaIdleAnim as
                                                    *mut AnimationHeader as
                                                    *mut libc::c_void) as
                             f32_0, ANIMMODE_LOOP as libc::c_int as u8_0,
                         0.0f32);
        (*this).updateIndex = 7 as libc::c_int + 10 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_CsGoronsRuby_AdvanceTo11(mut this:
                                                             *mut DemoDu,
                                                         mut globalCtx:
                                                             *mut GlobalContext) {
    if (*globalCtx).csCtx.state as libc::c_int != CS_STATE_IDLE as libc::c_int
       {
        let mut npcAction: *mut CsCmdActorAction =
            (*globalCtx).csCtx.npcActions[2 as libc::c_int as usize];
        if !npcAction.is_null() &&
               (*npcAction).action as libc::c_int != 5 as libc::c_int {
            Animation_Change(&mut (*this).skelAnime,
                             &mut gDaruniaItemGiveAnim, 1.0f32, 0.0f32,
                             Animation_GetLastFrame(&mut gDaruniaItemGiveAnim
                                                        as
                                                        *mut AnimationHeader
                                                        as *mut libc::c_void)
                                 as f32_0, 2 as libc::c_int as u8_0, 0.0f32);
            (*this).updateIndex = 7 as libc::c_int + 11 as libc::c_int
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_CsGoronsRuby_AdvanceTo12(mut this:
                                                             *mut DemoDu,
                                                         mut animFinished:
                                                             s32) {
    if animFinished != 0 {
        Animation_Change(&mut (*this).skelAnime,
                         &mut gDaruniaItemGiveIdleAnim, 1.0f32, 0.0f32,
                         Animation_GetLastFrame(&mut gDaruniaItemGiveIdleAnim
                                                    as *mut AnimationHeader as
                                                    *mut libc::c_void) as
                             f32_0, 0 as libc::c_int as u8_0, 0.0f32);
        (*this).updateIndex = 7 as libc::c_int + 12 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_CsGoronsRuby_AdvanceTo13(mut this:
                                                             *mut DemoDu,
                                                         mut globalCtx:
                                                             *mut GlobalContext) {
    if (*globalCtx).csCtx.state as libc::c_int != CS_STATE_IDLE as libc::c_int
       {
        let mut npcAction: *mut CsCmdActorAction =
            (*globalCtx).csCtx.npcActions[2 as libc::c_int as usize];
        if !npcAction.is_null() &&
               (*npcAction).action as libc::c_int != 6 as libc::c_int {
            Animation_Change(&mut (*this).skelAnime, &mut gDaruniaIdleAnim,
                             1.0f32, 0.0f32,
                             Animation_GetLastFrame(&mut gDaruniaIdleAnim as
                                                        *mut AnimationHeader
                                                        as *mut libc::c_void)
                                 as f32_0,
                             ANIMMODE_LOOP as libc::c_int as u8_0, 0.0f32);
            (*this).updateIndex = 7 as libc::c_int + 13 as libc::c_int
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_UpdateCs_GR_00(mut this: *mut DemoDu,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    DemoDu_CsPlaySfx_DaruniaFalling(globalCtx);
    DemoDu_CsGoronsRuby_AdvanceTo01(this, globalCtx);
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_UpdateCs_GR_01(mut this: *mut DemoDu,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    DemoDu_CsPlaySfx_DaruniaFalling(globalCtx);
    DemoDu_CsPlaySfx_LinkSurprised(globalCtx);
    DemoDu_CsGoronsRuby_AdvanceTo02(this, globalCtx);
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_UpdateCs_GR_02(mut this: *mut DemoDu,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    DemoDu_CsGoronsRuby_DaruniaFalling(this, globalCtx);
    DemoDu_UpdateBgCheckInfo(this, globalCtx);
    DemoDu_CsPlaySfx_DaruniaFalling(globalCtx);
    DemoDu_CsPlaySfx_LinkSurprised(globalCtx);
    DemoDu_CsGoronsRuby_AdvanceTo03(this, globalCtx);
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_UpdateCs_GR_03(mut this: *mut DemoDu,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    DemoDu_UpdateBgCheckInfo(this, globalCtx);
    DemoDu_CsPlaySfx_LinkSurprised(globalCtx);
    DemoDu_CsGoronsRuby_AdvanceTo04(this, globalCtx);
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_UpdateCs_GR_04(mut this: *mut DemoDu,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    let mut animFinished: s32 = 0;
    DemoDu_UpdateBgCheckInfo(this, globalCtx);
    animFinished = DemoDu_UpdateSkelAnime(this);
    DemoDu_CsGoronsRuby_UpdateFaceTextures(this, globalCtx);
    DemoDu_CsGoronsRuby_AdvanceTo05(this, animFinished);
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_UpdateCs_GR_05(mut this: *mut DemoDu,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    DemoDu_UpdateBgCheckInfo(this, globalCtx);
    DemoDu_UpdateSkelAnime(this);
    DemoDu_CsGoronsRuby_UpdateFaceTextures(this, globalCtx);
    DemoDu_CsGoronsRuby_AdvanceTo06(this, globalCtx);
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_UpdateCs_GR_06(mut this: *mut DemoDu,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    let mut animFinished: s32 = 0;
    DemoDu_UpdateBgCheckInfo(this, globalCtx);
    animFinished = DemoDu_UpdateSkelAnime(this);
    DemoDu_CsPlaySfx_HitBreast(this);
    DemoDu_CsGoronsRuby_UpdateFaceTextures(this, globalCtx);
    DemoDu_CsGoronsRuby_AdvanceTo07(this, animFinished);
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_UpdateCs_GR_07(mut this: *mut DemoDu,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    DemoDu_UpdateBgCheckInfo(this, globalCtx);
    DemoDu_UpdateSkelAnime(this);
    DemoDu_CsGoronsRuby_UpdateFaceTextures(this, globalCtx);
    DemoDu_CsGoronsRuby_AdvanceTo08(this, globalCtx);
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_UpdateCs_GR_08(mut this: *mut DemoDu,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    let mut animFinished: s32 = 0;
    DemoDu_UpdateBgCheckInfo(this, globalCtx);
    animFinished = DemoDu_UpdateSkelAnime(this);
    DemoDu_CsGoronsRuby_UpdateFaceTextures(this, globalCtx);
    DemoDu_CsGoronsRuby_SpawnDustWhenHittingLink(this, globalCtx);
    DemoDu_CsGoronsRuby_AdvanceTo09(this, animFinished);
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_UpdateCs_GR_09(mut this: *mut DemoDu,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    let mut animFinished: s32 = 0;
    DemoDu_UpdateBgCheckInfo(this, globalCtx);
    animFinished = DemoDu_UpdateSkelAnime(this);
    DemoDu_CsPlaySfx_HitBreast(this);
    DemoDu_CsGoronsRuby_UpdateFaceTextures(this, globalCtx);
    DemoDu_CsGoronsRuby_AdvanceTo10(this, animFinished);
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_UpdateCs_GR_10(mut this: *mut DemoDu,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    DemoDu_UpdateBgCheckInfo(this, globalCtx);
    DemoDu_UpdateSkelAnime(this);
    DemoDu_CsGoronsRuby_UpdateFaceTextures(this, globalCtx);
    DemoDu_CsGoronsRuby_AdvanceTo11(this, globalCtx);
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_UpdateCs_GR_11(mut this: *mut DemoDu,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    let mut animFinished: s32 = 0;
    DemoDu_UpdateBgCheckInfo(this, globalCtx);
    animFinished = DemoDu_UpdateSkelAnime(this);
    DemoDu_CsGoronsRuby_UpdateFaceTextures(this, globalCtx);
    DemoDu_CsGoronsRuby_AdvanceTo12(this, animFinished);
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_UpdateCs_GR_12(mut this: *mut DemoDu,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    DemoDu_UpdateBgCheckInfo(this, globalCtx);
    DemoDu_UpdateSkelAnime(this);
    DemoDu_CsGoronsRuby_UpdateFaceTextures(this, globalCtx);
    DemoDu_CsGoronsRuby_AdvanceTo13(this, globalCtx);
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_UpdateCs_GR_13(mut this: *mut DemoDu,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    DemoDu_UpdateBgCheckInfo(this, globalCtx);
    DemoDu_UpdateSkelAnime(this);
    DemoDu_CsGoronsRuby_UpdateFaceTextures(this, globalCtx);
    DemoDu_CsPlaySfx_LinkEscapeFromGorons(globalCtx);
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_InitCs_AfterGanon(mut this: *mut DemoDu,
                                                  mut globalCtx:
                                                      *mut GlobalContext) {
    let mut pad: [s32; 3] = [0; 3];
    let mut lastFrame: f32_0 =
        Animation_GetLastFrame(&mut gDaruniaSageFormationAnim as
                                   *mut AnimationHeader as *mut libc::c_void)
            as f32_0;
    SkelAnime_InitFlex(globalCtx, &mut (*this).skelAnime, &mut gDaruniaSkel,
                       0 as *mut AnimationHeader, 0 as *mut Vec3s,
                       0 as *mut Vec3s, 0 as libc::c_int);
    Animation_Change(&mut (*this).skelAnime, &mut gDaruniaSageFormationAnim,
                     1.0f32, 0.0f32, lastFrame,
                     ANIMMODE_ONCE as libc::c_int as u8_0, 0.0f32);
    (*this).updateIndex = 21 as libc::c_int + 0 as libc::c_int;
    (*this).actor.shape.shadowAlpha = 0 as libc::c_int as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_CsPlaySfx_WhiteOut() {
    func_800788CC(0x4834 as libc::c_int as u16_0);
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_CsAfterGanon_SpawnDemo6K(mut this:
                                                             *mut DemoDu,
                                                         mut globalCtx:
                                                             *mut GlobalContext) {
    Actor_SpawnAsChild(&mut (*globalCtx).actorCtx, &mut (*this).actor,
                       globalCtx, ACTOR_DEMO_6K as libc::c_int as s16,
                       (*this).actor.world.pos.x,
                       (*gGameInfo).data[(27 as libc::c_int * 6 as libc::c_int
                                              * 16 as libc::c_int +
                                              16 as libc::c_int) as usize] as
                           libc::c_int as libc::c_float + 22.0f32 +
                           (*this).actor.world.pos.y,
                       (*this).actor.world.pos.z, 0 as libc::c_int as s16,
                       0 as libc::c_int as s16, 0 as libc::c_int as s16,
                       3 as libc::c_int as s16);
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_CsAfterGanon_AdvanceTo01(mut this:
                                                             *mut DemoDu,
                                                         mut globalCtx:
                                                             *mut GlobalContext) {
    if DemoDu_IsNpcDoingThisAction(this, globalCtx, 4 as libc::c_int as u16_0,
                                   2 as libc::c_int) != 0 {
        (*this).updateIndex = 21 as libc::c_int + 1 as libc::c_int;
        (*this).drawIndex = 2 as libc::c_int;
        (*this).shadowAlpha = 0 as libc::c_int;
        (*this).actor.shape.shadowAlpha = 0 as libc::c_int as u8_0;
        (*this).unk_1A4 = 0.0f32;
        DemoDu_CsPlaySfx_WhiteOut();
    };
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_CsAfterGanon_AdvanceTo02(mut this:
                                                             *mut DemoDu,
                                                         mut globalCtx:
                                                             *mut GlobalContext) {
    let mut unk_1A4: *mut f32_0 = &mut (*this).unk_1A4;
    let mut shadowAlpha: s32 = 255 as libc::c_int;
    if DemoDu_IsNpcDoingThisAction(this, globalCtx, 4 as libc::c_int as u16_0,
                                   2 as libc::c_int) != 0 {
        *unk_1A4 += 1.0f32;
        if *unk_1A4 >=
               (*gGameInfo).data[(27 as libc::c_int * 6 as libc::c_int *
                                      16 as libc::c_int + 5 as libc::c_int) as
                                     usize] as libc::c_int as libc::c_float +
                   10.0f32 {
            (*this).updateIndex = 21 as libc::c_int + 2 as libc::c_int;
            (*this).drawIndex = 1 as libc::c_int;
            *unk_1A4 =
                (*gGameInfo).data[(27 as libc::c_int * 6 as libc::c_int *
                                       16 as libc::c_int + 5 as libc::c_int)
                                      as usize] as libc::c_int as
                    libc::c_float + 10.0f32;
            (*this).shadowAlpha = shadowAlpha;
            (*this).actor.shape.shadowAlpha = shadowAlpha as u8_0;
            return
        }
    } else {
        *unk_1A4 -= 1.0f32;
        if *unk_1A4 <= 0.0f32 {
            (*this).updateIndex = 21 as libc::c_int + 0 as libc::c_int;
            (*this).drawIndex = 0 as libc::c_int;
            *unk_1A4 = 0.0f32;
            (*this).shadowAlpha = 0 as libc::c_int;
            (*this).actor.shape.shadowAlpha = 0 as libc::c_int as u8_0;
            return
        }
    }
    shadowAlpha =
        (*unk_1A4 /
             ((*gGameInfo).data[(27 as libc::c_int * 6 as libc::c_int *
                                     16 as libc::c_int + 5 as libc::c_int) as
                                    usize] as libc::c_int as libc::c_float +
                  10.0f32) * 255.0f32) as s32;
    (*this).shadowAlpha = shadowAlpha;
    (*this).actor.shape.shadowAlpha = shadowAlpha as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_CsAfterGanon_BackTo01(mut this: *mut DemoDu,
                                                      mut globalCtx:
                                                          *mut GlobalContext) {
    if DemoDu_IsNpcNotDoingThisAction(this, globalCtx,
                                      4 as libc::c_int as u16_0,
                                      2 as libc::c_int) != 0 {
        (*this).updateIndex = 21 as libc::c_int + 1 as libc::c_int;
        (*this).drawIndex = 2 as libc::c_int;
        (*this).unk_1A4 =
            (*gGameInfo).data[(27 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 5 as libc::c_int) as
                                  usize] as libc::c_int as libc::c_float +
                10.0f32;
        (*this).shadowAlpha = 255 as libc::c_int;
        if (*this).demo6KSpawned == 0 {
            DemoDu_CsAfterGanon_SpawnDemo6K(this, globalCtx);
            (*this).demo6KSpawned = 1 as libc::c_int
        }
        (*this).actor.shape.shadowAlpha = 255 as libc::c_int as u8_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_UpdateCs_AG_00(mut this: *mut DemoDu,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    DemoDu_CsAfterGanon_AdvanceTo01(this, globalCtx);
    DemoDu_CsAfterGanon_CheckIfShouldReset(this, globalCtx);
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_UpdateCs_AG_01(mut this: *mut DemoDu,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    DemoDu_UpdateBgCheckInfo(this, globalCtx);
    DemoDu_UpdateSkelAnime(this);
    DemoDu_UpdateEyes(this);
    DemoDu_CsAfterGanon_AdvanceTo02(this, globalCtx);
    DemoDu_CsAfterGanon_CheckIfShouldReset(this, globalCtx);
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_UpdateCs_AG_02(mut this: *mut DemoDu,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    DemoDu_UpdateBgCheckInfo(this, globalCtx);
    DemoDu_UpdateSkelAnime(this);
    DemoDu_UpdateEyes(this);
    DemoDu_CsAfterGanon_BackTo01(this, globalCtx);
    DemoDu_CsAfterGanon_CheckIfShouldReset(this, globalCtx);
}
// Similar to DemoDu_Draw_01, but this uses POLY_XLU_DISP. Also uses this->shadowAlpha for setting the env color.
#[no_mangle]
pub unsafe extern "C" fn DemoDu_Draw_02(mut thisx: *mut Actor,
                                        mut globalCtx2: *mut GlobalContext) {
    let mut globalCtx: *mut GlobalContext = globalCtx2;
    let mut this: *mut DemoDu = thisx as *mut DemoDu;
    let mut eyeTexIndex: s16 = (*this).eyeTexIndex;
    let mut eyeTexture: *mut libc::c_void =
        sEyeTextures[eyeTexIndex as usize];
    let mut pad: s32 = 0;
    let mut mouthTexIndex: s16 = (*this).mouthTexIndex;
    let mut mouthTexture: *mut libc::c_void =
        sMouthTextures[mouthTexIndex as usize];
    let mut skelAnime: *mut SkelAnime = &mut (*this).skelAnime;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_demo_du_inKenjyanomaDemo02.c\x00" as *const u8 as
                        *const libc::c_char, 275 as libc::c_int);
    func_80093D84((*globalCtx).state.gfxCtx);
    let fresh0 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g: *mut Gfx = fresh0;
    (*_g).words.w0 =
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
    (*_g).words.w1 =
        gSegments[((eyeTexture as u32_0) << 4 as libc::c_int >>
                       28 as libc::c_int) as
                      usize].wrapping_add(eyeTexture as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as libc::c_uint;
    let fresh1 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_0: *mut Gfx = fresh1;
    (*_g_0).words.w0 =
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
    (*_g_0).words.w1 =
        gSegments[((mouthTexture as u32_0) << 4 as libc::c_int >>
                       28 as libc::c_int) as
                      usize].wrapping_add(mouthTexture as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as libc::c_uint;
    let fresh2 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_1: *mut Gfx = fresh2;
    (*_g_1).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((0xa as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_1).words.w1 =
        gSegments[((gDaruniaNoseSeriousTex.as_mut_ptr() as u32_0) <<
                       4 as libc::c_int >> 28 as libc::c_int) as
                      usize].wrapping_add(gDaruniaNoseSeriousTex.as_mut_ptr()
                                              as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as libc::c_uint;
    let fresh3 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_2: *mut Gfx = fresh3;
    (*_g_2).words.w0 =
        (0xfb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
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
            ((*this).shadowAlpha as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh4 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_3: *mut Gfx = fresh4;
    (*_g_3).words.w0 =
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
    (*_g_3).words.w1 =
        &mut *D_80116280.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut Gfx as libc::c_uint;
    (*__gfxCtx).polyXlu.p =
        SkelAnime_DrawFlex(globalCtx, (*skelAnime).skeleton,
                           (*skelAnime).jointTable,
                           (*skelAnime).dListCount as s32, None, None,
                           0 as *mut libc::c_void, (*__gfxCtx).polyXlu.p);
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_demo_du_inKenjyanomaDemo02.c\x00" as *const u8 as
                         *const libc::c_char, 304 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_InitCs_Credits(mut this: *mut DemoDu,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    SkelAnime_InitFlex(globalCtx, &mut (*this).skelAnime, &mut gDaruniaSkel,
                       &mut gDaruniaCreditsIdleAnim, 0 as *mut Vec3s,
                       0 as *mut Vec3s, 0 as libc::c_int);
    (*this).updateIndex = 24 as libc::c_int + 0 as libc::c_int;
    (*this).drawIndex = 0 as libc::c_int;
    (*this).actor.shape.shadowAlpha = 0 as libc::c_int as u8_0;
    DemoDu_SetMouthTexIndex(this, 3 as libc::c_int as s16);
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_CsCredits_UpdateShadowAlpha(mut this:
                                                                *mut DemoDu) {
    let mut shadowAlpha: s32 = 255 as libc::c_int;
    let mut temp_f0: f32_0 = 0.;
    let mut unk_1A4: *mut f32_0 = 0 as *mut f32_0;
    (*this).unk_1A4 += 1.0f32;
    temp_f0 =
        (*gGameInfo).data[(27 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 17 as libc::c_int) as
                              usize] as libc::c_int as libc::c_float +
            10.0f32;
    unk_1A4 = &mut (*this).unk_1A4;
    if temp_f0 <= *unk_1A4 {
        (*this).shadowAlpha = shadowAlpha;
        (*this).actor.shape.shadowAlpha = shadowAlpha as u8_0
    } else {
        shadowAlpha = (*unk_1A4 / temp_f0 * 255.0f32) as s32;
        (*this).shadowAlpha = shadowAlpha;
        (*this).actor.shape.shadowAlpha = shadowAlpha as u8_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_CsCredits_AdvanceTo01(mut this: *mut DemoDu,
                                                      mut globalCtx:
                                                          *mut GlobalContext) {
    DemoDu_MoveToNpcPos(this, globalCtx, 2 as libc::c_int);
    (*this).updateIndex = 24 as libc::c_int + 1 as libc::c_int;
    (*this).drawIndex = 2 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_CsCredits_AdvanceTo02(mut this: *mut DemoDu) {
    if (*this).unk_1A4 >=
           (*gGameInfo).data[(27 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 17 as libc::c_int) as
                                 usize] as libc::c_int as libc::c_float +
               10.0f32 {
        (*this).updateIndex = 24 as libc::c_int + 2 as libc::c_int;
        (*this).drawIndex = 1 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_CsCredits_AdvanceTo03(mut this: *mut DemoDu) {
    func_80969DDC(this, &mut gDaruniaLookingUpToSariaAnim,
                  ANIMMODE_ONCE as libc::c_int as u8_0, -8.0f32,
                  0 as libc::c_int);
    (*this).updateIndex = 24 as libc::c_int + 3 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_CsCredits_AdvanceTo04(mut this: *mut DemoDu) {
    func_80969DDC(this, &mut gDaruniaCreditsHitBreastAnim,
                  ANIMMODE_ONCE as libc::c_int as u8_0, 0.0f32,
                  0 as libc::c_int);
    (*this).updateIndex = 24 as libc::c_int + 4 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_CsCredits_BackTo02(mut this: *mut DemoDu,
                                                   mut animFinished: s32) {
    if animFinished != 0 {
        func_80969DDC(this, &mut gDaruniaCreditsIdleAnim,
                      ANIMMODE_LOOP as libc::c_int as u8_0, 0.0f32,
                      0 as libc::c_int);
        (*this).updateIndex = 24 as libc::c_int + 2 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_CsCredits_HandleSubscenesByNpcAction(mut this:
                                                                         *mut DemoDu,
                                                                     mut globalCtx:
                                                                         *mut GlobalContext) {
    let mut npcAction: *mut CsCmdActorAction =
        DemoDu_GetNpcAction(globalCtx, 2 as libc::c_int);
    if !npcAction.is_null() {
        let mut action: s32 = (*npcAction).action as s32;
        let mut lastAction: s32 = (*this).lastAction;
        if action != lastAction {
            match action {
                9 => { DemoDu_CsCredits_AdvanceTo01(this, globalCtx); }
                10 => { DemoDu_CsCredits_AdvanceTo03(this); }
                11 => { DemoDu_CsCredits_AdvanceTo04(this); }
                _ => {
                    // "Demo_Du_inEnding_Check_DemoMode:There is no such operation!!!!!!!!"
                    osSyncPrintf(b"Demo_Du_inEnding_Check_DemoMode:\xe3\x81\x9d\xe3\x82\x93\xe3\x81\xaa\xe5\x8b\x95\xe4\xbd\x9c\xe3\x81\xaf\xe7\x84\xa1\xe3\x81\x84!!!!!!!!\n\x00"
                                     as *const u8 as *const libc::c_char);
                }
            }
            (*this).lastAction = action
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_UpdateCs_CR_00(mut this: *mut DemoDu,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    DemoDu_CsCredits_HandleSubscenesByNpcAction(this, globalCtx);
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_UpdateCs_CR_01(mut this: *mut DemoDu,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    DemoDu_UpdateBgCheckInfo(this, globalCtx);
    DemoDu_UpdateSkelAnime(this);
    DemoDu_UpdateEyes(this);
    DemoDu_CsCredits_UpdateShadowAlpha(this);
    DemoDu_CsCredits_AdvanceTo02(this);
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_UpdateCs_CR_02(mut this: *mut DemoDu,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    DemoDu_UpdateBgCheckInfo(this, globalCtx);
    DemoDu_UpdateSkelAnime(this);
    DemoDu_UpdateEyes(this);
    DemoDu_CsCredits_HandleSubscenesByNpcAction(this, globalCtx);
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_UpdateCs_CR_03(mut this: *mut DemoDu,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    DemoDu_UpdateBgCheckInfo(this, globalCtx);
    DemoDu_UpdateSkelAnime(this);
    DemoDu_UpdateEyes(this);
    DemoDu_CsCredits_HandleSubscenesByNpcAction(this, globalCtx);
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_UpdateCs_CR_04(mut this: *mut DemoDu,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    let mut animFinished: s32 = 0;
    DemoDu_UpdateBgCheckInfo(this, globalCtx);
    animFinished = DemoDu_UpdateSkelAnime(this);
    DemoDu_UpdateEyes(this);
    DemoDu_CsCredits_BackTo02(this, animFinished);
}
static mut sUpdateFuncs: [DemoDuActionFunc; 29] =
    unsafe {
        [Some(DemoDu_UpdateCs_FM_00 as
                  unsafe extern "C" fn(_: *mut DemoDu, _: *mut GlobalContext)
                      -> ()),
         Some(DemoDu_UpdateCs_FM_01 as
                  unsafe extern "C" fn(_: *mut DemoDu, _: *mut GlobalContext)
                      -> ()),
         Some(DemoDu_UpdateCs_FM_02 as
                  unsafe extern "C" fn(_: *mut DemoDu, _: *mut GlobalContext)
                      -> ()),
         Some(DemoDu_UpdateCs_FM_03 as
                  unsafe extern "C" fn(_: *mut DemoDu, _: *mut GlobalContext)
                      -> ()),
         Some(DemoDu_UpdateCs_FM_04 as
                  unsafe extern "C" fn(_: *mut DemoDu, _: *mut GlobalContext)
                      -> ()),
         Some(DemoDu_UpdateCs_FM_05 as
                  unsafe extern "C" fn(_: *mut DemoDu, _: *mut GlobalContext)
                      -> ()),
         Some(DemoDu_UpdateCs_FM_06 as
                  unsafe extern "C" fn(_: *mut DemoDu, _: *mut GlobalContext)
                      -> ()),
         Some(DemoDu_UpdateCs_GR_00 as
                  unsafe extern "C" fn(_: *mut DemoDu, _: *mut GlobalContext)
                      -> ()),
         Some(DemoDu_UpdateCs_GR_01 as
                  unsafe extern "C" fn(_: *mut DemoDu, _: *mut GlobalContext)
                      -> ()),
         Some(DemoDu_UpdateCs_GR_02 as
                  unsafe extern "C" fn(_: *mut DemoDu, _: *mut GlobalContext)
                      -> ()),
         Some(DemoDu_UpdateCs_GR_03 as
                  unsafe extern "C" fn(_: *mut DemoDu, _: *mut GlobalContext)
                      -> ()),
         Some(DemoDu_UpdateCs_GR_04 as
                  unsafe extern "C" fn(_: *mut DemoDu, _: *mut GlobalContext)
                      -> ()),
         Some(DemoDu_UpdateCs_GR_05 as
                  unsafe extern "C" fn(_: *mut DemoDu, _: *mut GlobalContext)
                      -> ()),
         Some(DemoDu_UpdateCs_GR_06 as
                  unsafe extern "C" fn(_: *mut DemoDu, _: *mut GlobalContext)
                      -> ()),
         Some(DemoDu_UpdateCs_GR_07 as
                  unsafe extern "C" fn(_: *mut DemoDu, _: *mut GlobalContext)
                      -> ()),
         Some(DemoDu_UpdateCs_GR_08 as
                  unsafe extern "C" fn(_: *mut DemoDu, _: *mut GlobalContext)
                      -> ()),
         Some(DemoDu_UpdateCs_GR_09 as
                  unsafe extern "C" fn(_: *mut DemoDu, _: *mut GlobalContext)
                      -> ()),
         Some(DemoDu_UpdateCs_GR_10 as
                  unsafe extern "C" fn(_: *mut DemoDu, _: *mut GlobalContext)
                      -> ()),
         Some(DemoDu_UpdateCs_GR_11 as
                  unsafe extern "C" fn(_: *mut DemoDu, _: *mut GlobalContext)
                      -> ()),
         Some(DemoDu_UpdateCs_GR_12 as
                  unsafe extern "C" fn(_: *mut DemoDu, _: *mut GlobalContext)
                      -> ()),
         Some(DemoDu_UpdateCs_GR_13 as
                  unsafe extern "C" fn(_: *mut DemoDu, _: *mut GlobalContext)
                      -> ()),
         Some(DemoDu_UpdateCs_AG_00 as
                  unsafe extern "C" fn(_: *mut DemoDu, _: *mut GlobalContext)
                      -> ()),
         Some(DemoDu_UpdateCs_AG_01 as
                  unsafe extern "C" fn(_: *mut DemoDu, _: *mut GlobalContext)
                      -> ()),
         Some(DemoDu_UpdateCs_AG_02 as
                  unsafe extern "C" fn(_: *mut DemoDu, _: *mut GlobalContext)
                      -> ()),
         Some(DemoDu_UpdateCs_CR_00 as
                  unsafe extern "C" fn(_: *mut DemoDu, _: *mut GlobalContext)
                      -> ()),
         Some(DemoDu_UpdateCs_CR_01 as
                  unsafe extern "C" fn(_: *mut DemoDu, _: *mut GlobalContext)
                      -> ()),
         Some(DemoDu_UpdateCs_CR_02 as
                  unsafe extern "C" fn(_: *mut DemoDu, _: *mut GlobalContext)
                      -> ()),
         Some(DemoDu_UpdateCs_CR_03 as
                  unsafe extern "C" fn(_: *mut DemoDu, _: *mut GlobalContext)
                      -> ()),
         Some(DemoDu_UpdateCs_CR_04 as
                  unsafe extern "C" fn(_: *mut DemoDu, _: *mut GlobalContext)
                      -> ())]
    };
#[no_mangle]
pub unsafe extern "C" fn DemoDu_Update(mut thisx: *mut Actor,
                                       mut globalCtx: *mut GlobalContext) {
    let mut this: *mut DemoDu = thisx as *mut DemoDu;
    if (*this).updateIndex < 0 as libc::c_int ||
           (*this).updateIndex >= 29 as libc::c_int ||
           sUpdateFuncs[(*this).updateIndex as usize].is_none() {
        // "The main mode is abnormal!!!!!!!!!!!!!!!!!!!!!!!!!"
        osSyncPrintf(b"\x1b[31m\xe3\x83\xa1\xe3\x82\xa4\xe3\x83\xb3\xe3\x83\xa2\xe3\x83\xbc\xe3\x83\x89\xe3\x81\x8c\xe3\x81\x8a\xe3\x81\x8b\xe3\x81\x97\xe3\x81\x84!!!!!!!!!!!!!!!!!!!!!!!!!\n\x1b[m\x00"
                         as *const u8 as *const libc::c_char);
        return
    }
    sUpdateFuncs[(*this).updateIndex as
                     usize].expect("non-null function pointer")(this,
                                                                globalCtx);
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_Init(mut thisx: *mut Actor,
                                     mut globalCtx: *mut GlobalContext) {
    let mut this: *mut DemoDu = thisx as *mut DemoDu;
    ActorShape_Init(&mut (*this).actor.shape, 0.0f32,
                    Some(ActorShadow_DrawCircle as
                             unsafe extern "C" fn(_: *mut Actor,
                                                  _: *mut Lights,
                                                  _: *mut GlobalContext)
                                 -> ()), 30.0f32);
    match (*this).actor.params as libc::c_int {
        1 => { DemoDu_InitCs_GoronsRuby(this, globalCtx); }
        2 => { DemoDu_InitCs_AfterGanon(this, globalCtx); }
        3 => { DemoDu_InitCs_Credits(this, globalCtx); }
        _ => { DemoDu_InitCs_FireMedallion(this, globalCtx); }
    };
}
#[no_mangle]
pub unsafe extern "C" fn DemoDu_Draw_NoDraw(mut thisx: *mut Actor,
                                            mut globalCtx2:
                                                *mut GlobalContext) {
}
// Similar to DemoDu_Draw_02, but this uses POLY_OPA_DISP. Sets the env color to 255.
#[no_mangle]
pub unsafe extern "C" fn DemoDu_Draw_01(mut thisx: *mut Actor,
                                        mut globalCtx2: *mut GlobalContext) {
    let mut globalCtx: *mut GlobalContext = globalCtx2;
    let mut this: *mut DemoDu = thisx as *mut DemoDu;
    let mut eyeTexIndex: s16 = (*this).eyeTexIndex;
    let mut eyeTexture: *mut libc::c_void =
        sEyeTextures[eyeTexIndex as usize];
    let mut pad: s32 = 0;
    let mut mouthTexIndex: s16 = (*this).mouthTexIndex;
    let mut mouthTexture: *mut libc::c_void =
        sMouthTextures[mouthTexIndex as usize];
    let mut skelAnime: *mut SkelAnime = &mut (*this).skelAnime;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_demo_du.c\x00" as *const u8 as *const libc::c_char,
                    615 as libc::c_int);
    func_80093D18((*globalCtx).state.gfxCtx);
    let fresh5 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g: *mut Gfx = fresh5;
    (*_g).words.w0 =
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
    (*_g).words.w1 =
        gSegments[((eyeTexture as u32_0) << 4 as libc::c_int >>
                       28 as libc::c_int) as
                      usize].wrapping_add(eyeTexture as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as libc::c_uint;
    let fresh6 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_0: *mut Gfx = fresh6;
    (*_g_0).words.w0 =
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
    (*_g_0).words.w1 =
        gSegments[((mouthTexture as u32_0) << 4 as libc::c_int >>
                       28 as libc::c_int) as
                      usize].wrapping_add(mouthTexture as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as libc::c_uint;
    let fresh7 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_1: *mut Gfx = fresh7;
    (*_g_1).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((0xa as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_1).words.w1 =
        gSegments[((gDaruniaNoseSeriousTex.as_mut_ptr() as u32_0) <<
                       4 as libc::c_int >> 28 as libc::c_int) as
                      usize].wrapping_add(gDaruniaNoseSeriousTex.as_mut_ptr()
                                              as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as libc::c_uint;
    let fresh8 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_2: *mut Gfx = fresh8;
    (*_g_2).words.w0 =
        (0xfb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
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
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh9 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_3: *mut Gfx = fresh9;
    (*_g_3).words.w0 =
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
    (*_g_3).words.w1 =
        &mut *D_80116280.as_mut_ptr().offset(2 as libc::c_int as isize) as
            *mut Gfx as libc::c_uint;
    SkelAnime_DrawFlexOpa(globalCtx, (*skelAnime).skeleton,
                          (*skelAnime).jointTable,
                          (*skelAnime).dListCount as s32, None, None,
                          this as *mut libc::c_void);
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_demo_du.c\x00" as *const u8 as
                         *const libc::c_char, 638 as libc::c_int);
}
static mut sDrawFuncs: [DemoDuDrawFunc; 3] =
    unsafe {
        [Some(DemoDu_Draw_NoDraw as
                  unsafe extern "C" fn(_: *mut Actor, _: *mut GlobalContext)
                      -> ()),
         Some(DemoDu_Draw_01 as
                  unsafe extern "C" fn(_: *mut Actor, _: *mut GlobalContext)
                      -> ()),
         Some(DemoDu_Draw_02 as
                  unsafe extern "C" fn(_: *mut Actor, _: *mut GlobalContext)
                      -> ())]
    };
#[no_mangle]
pub unsafe extern "C" fn DemoDu_Draw(mut thisx: *mut Actor,
                                     mut globalCtx: *mut GlobalContext) {
    let mut this: *mut DemoDu = thisx as *mut DemoDu;
    if (*this).drawIndex < 0 as libc::c_int ||
           (*this).drawIndex >= 3 as libc::c_int ||
           sDrawFuncs[(*this).drawIndex as usize].is_none() {
        // "The drawing mode is abnormal!!!!!!!!!!!!!!!!!!!!!!!!!"
        osSyncPrintf(b"\x1b[31m\xe6\x8f\x8f\xe7\x94\xbb\xe3\x83\xa2\xe3\x83\xbc\xe3\x83\x89\xe3\x81\x8c\xe3\x81\x8a\xe3\x81\x8b\xe3\x81\x97\xe3\x81\x84!!!!!!!!!!!!!!!!!!!!!!!!!\n\x1b[m\x00"
                         as *const u8 as *const libc::c_char);
        return
    }
    sDrawFuncs[(*this).drawIndex as
                   usize].expect("non-null function pointer")(thisx,
                                                              globalCtx);
}
#[no_mangle]
pub static mut Demo_Du_InitVars: ActorInit =
    unsafe {
        {
            let mut init =
                ActorInit{id: ACTOR_DEMO_DU as libc::c_int as s16,
                          category: ACTORCAT_NPC as libc::c_int as u8_0,
                          flags:
                              ((1 as libc::c_int) << 4 as libc::c_int) as
                                  u32_0,
                          objectId: OBJECT_DU as libc::c_int as s16,
                          instanceSize:
                              ::std::mem::size_of::<DemoDu>() as
                                  libc::c_ulong,
                          init:
                              ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                      *mut Actor,
                                                                                  _:
                                                                                      *mut GlobalContext)
                                                                 -> ()>,
                                                      ActorFunc>(Some(DemoDu_Init
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
                                                      ActorFunc>(Some(DemoDu_Destroy
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
                                                      ActorFunc>(Some(DemoDu_Update
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
                                                      ActorFunc>(Some(DemoDu_Draw
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
