#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, register_tool)]
extern "C" {
    #[no_mangle]
    fn ActorShape_Init(shape: *mut ActorShape, yOffset: f32_0,
                       shadowDraw: ActorShadowFunc, shadowScale: f32_0);
    #[no_mangle]
    fn ActorShadow_DrawCircle(actor: *mut Actor, lights: *mut Lights,
                              globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn Actor_SetScale(actor: *mut Actor, scale: f32_0);
    #[no_mangle]
    fn Actor_ProcessTalkRequest(actor: *mut Actor,
                                globalCtx: *mut GlobalContext) -> u32_0;
    #[no_mangle]
    fn func_8002F298(actor: *mut Actor, globalCtx: *mut GlobalContext,
                     arg2: f32_0, arg3: u32_0) -> s32;
    #[no_mangle]
    fn func_8002F368(globalCtx: *mut GlobalContext) -> s8;
    #[no_mangle]
    fn Actor_HasParent(actor: *mut Actor, globalCtx: *mut GlobalContext)
     -> u32_0;
    #[no_mangle]
    fn func_8002F434(actor: *mut Actor, globalCtx: *mut GlobalContext,
                     getItemId: s32, xzRange: f32_0, yRange: f32_0) -> s32;
    #[no_mangle]
    fn Audio_PlayActorSound2(actor: *mut Actor, sfxId: u16_0);
    #[no_mangle]
    fn Rand_ZeroFloat(f: f32_0) -> f32_0;
    #[no_mangle]
    fn DynaPoly_SetBgActor(globalCtx: *mut GlobalContext,
                           dyna: *mut DynaCollisionContext, actor: *mut Actor,
                           colHeader: *mut CollisionHeader) -> s32;
    #[no_mangle]
    fn DynaPoly_DeleteBgActor(globalCtx: *mut GlobalContext,
                              dyna: *mut DynaCollisionContext, bgId: s32);
    #[no_mangle]
    fn CollisionHeader_GetVirtual(colHeader: *mut libc::c_void,
                                  dest: *mut *mut CollisionHeader);
    #[no_mangle]
    fn DynaPolyActor_Init(dynaActor: *mut DynaPolyActor, flags: s32);
    #[no_mangle]
    fn Collider_InitCylinder(globalCtx: *mut GlobalContext,
                             collider: *mut ColliderCylinder) -> s32;
    #[no_mangle]
    fn Collider_DestroyCylinder(globalCtx: *mut GlobalContext,
                                collider: *mut ColliderCylinder) -> s32;
    #[no_mangle]
    fn Collider_SetCylinderType1(globalCtx: *mut GlobalContext,
                                 collider: *mut ColliderCylinder,
                                 actor: *mut Actor,
                                 src: *mut ColliderCylinderInitType1) -> s32;
    #[no_mangle]
    fn CollisionCheck_SetOC(globalCtx: *mut GlobalContext,
                            colChkCtx: *mut CollisionCheckContext,
                            collider: *mut Collider) -> s32;
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
    fn Rupees_ChangeBy(rupeeChange: s16);
    #[no_mangle]
    fn Player_UpdateBottleHeld(globalCtx: *mut GlobalContext,
                               player: *mut Player, item: s32,
                               actionParam: s32);
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
    fn Matrix_RotateZYX(x: s16, y: s16, z: s16, mode: u8_0);
    #[no_mangle]
    fn Matrix_NewMtx(gfxCtx: *mut GraphicsContext, file: *mut libc::c_char,
                     line: s32) -> *mut Mtx;
    #[no_mangle]
    fn Matrix_MultVec3f(src: *mut Vec3f, dest: *mut Vec3f);
    #[no_mangle]
    fn func_800D1FD4(mf: *mut MtxF);
    #[no_mangle]
    fn Rand_ZeroOne() -> f32_0;
    #[no_mangle]
    fn Message_ShouldAdvance(globalCtx: *mut GlobalContext) -> u8_0;
    #[no_mangle]
    fn Message_ContinueTextbox(globalCtx: *mut GlobalContext, textId: u16_0);
    #[no_mangle]
    fn Message_GetState(msgCtx: *mut MessageContext) -> u8_0;
    #[no_mangle]
    static mut gSaveContext: SaveContext;
    #[no_mangle]
    static mut gSegments: [u32_0; 16];
    #[no_mangle]
    static mut gPoeSellerIdleAnim: AnimationHeader;
    #[no_mangle]
    static mut gPoeSellerAngrySoulTex: [u64_0; 0];
    #[no_mangle]
    static mut gPoeSellerHappySoulTex: [u64_0; 0];
    #[no_mangle]
    static mut gPoeSellerSadSoulTex: [u64_0; 0];
    #[no_mangle]
    static mut gPoeSellerCagedSoulDL: [Gfx; 0];
    #[no_mangle]
    static mut gPoeSellerSkel: FlexSkeletonHeader;
    #[no_mangle]
    static mut gPoeSellerCol: CollisionHeader;
    #[no_mangle]
    static mut gPoeSellerSwingStickAnim: AnimationHeader;
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
pub type C2RustUnnamed_15 = libc::c_uint;
pub const ANIMMODE_LOOP_PARTIAL_INTERP: C2RustUnnamed_15 = 5;
pub const ANIMMODE_LOOP_PARTIAL: C2RustUnnamed_15 = 4;
pub const ANIMMODE_ONCE_INTERP: C2RustUnnamed_15 = 3;
pub const ANIMMODE_ONCE: C2RustUnnamed_15 = 2;
pub const ANIMMODE_LOOP_INTERP: C2RustUnnamed_15 = 1;
pub const ANIMMODE_LOOP: C2RustUnnamed_15 = 0;
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
pub struct ColliderInitType1 {
    pub colType: u8_0,
    pub atFlags: u8_0,
    pub acFlags: u8_0,
    pub ocFlags1: u8_0,
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
pub struct ColliderCylinderInitType1 {
    pub base: ColliderInitType1,
    pub info: ColliderInfoInit,
    pub dim: Cylinder16,
}
pub type C2RustUnnamed_16 = libc::c_uint;
pub const COLTYPE_TREE: C2RustUnnamed_16 = 13;
pub const COLTYPE_HARD: C2RustUnnamed_16 = 12;
pub const COLTYPE_WOOD: C2RustUnnamed_16 = 11;
pub const COLTYPE_NONE: C2RustUnnamed_16 = 10;
pub const COLTYPE_METAL: C2RustUnnamed_16 = 9;
pub const COLTYPE_HIT8: C2RustUnnamed_16 = 8;
pub const COLTYPE_HIT7: C2RustUnnamed_16 = 7;
pub const COLTYPE_HIT6: C2RustUnnamed_16 = 6;
pub const COLTYPE_HIT5: C2RustUnnamed_16 = 5;
pub const COLTYPE_HIT4: C2RustUnnamed_16 = 4;
pub const COLTYPE_HIT3: C2RustUnnamed_16 = 3;
pub const COLTYPE_HIT2: C2RustUnnamed_16 = 2;
pub const COLTYPE_HIT1: C2RustUnnamed_16 = 1;
pub const COLTYPE_HIT0: C2RustUnnamed_16 = 0;
pub type C2RustUnnamed_17 = libc::c_uint;
pub const COLSHAPE_INVALID: C2RustUnnamed_17 = 4;
pub const COLSHAPE_QUAD: C2RustUnnamed_17 = 3;
pub const COLSHAPE_TRIS: C2RustUnnamed_17 = 2;
pub const COLSHAPE_CYLINDER: C2RustUnnamed_17 = 1;
pub const COLSHAPE_JNTSPH: C2RustUnnamed_17 = 0;
pub type C2RustUnnamed_18 = libc::c_uint;
pub const ELEMTYPE_UNK7: C2RustUnnamed_18 = 7;
pub const ELEMTYPE_UNK6: C2RustUnnamed_18 = 6;
pub const ELEMTYPE_UNK5: C2RustUnnamed_18 = 5;
pub const ELEMTYPE_UNK4: C2RustUnnamed_18 = 4;
pub const ELEMTYPE_UNK3: C2RustUnnamed_18 = 3;
pub const ELEMTYPE_UNK2: C2RustUnnamed_18 = 2;
pub const ELEMTYPE_UNK1: C2RustUnnamed_18 = 1;
pub const ELEMTYPE_UNK0: C2RustUnnamed_18 = 0;
pub type C2RustUnnamed_19 = libc::c_uint;
pub const ACTORCAT_CHEST: C2RustUnnamed_19 = 11;
pub const ACTORCAT_DOOR: C2RustUnnamed_19 = 10;
pub const ACTORCAT_BOSS: C2RustUnnamed_19 = 9;
pub const ACTORCAT_MISC: C2RustUnnamed_19 = 8;
pub const ACTORCAT_ITEMACTION: C2RustUnnamed_19 = 7;
pub const ACTORCAT_PROP: C2RustUnnamed_19 = 6;
pub const ACTORCAT_ENEMY: C2RustUnnamed_19 = 5;
pub const ACTORCAT_NPC: C2RustUnnamed_19 = 4;
pub const ACTORCAT_EXPLOSIVE: C2RustUnnamed_19 = 3;
pub const ACTORCAT_PLAYER: C2RustUnnamed_19 = 2;
pub const ACTORCAT_BG: C2RustUnnamed_19 = 1;
pub const ACTORCAT_SWITCH: C2RustUnnamed_19 = 0;
pub type C2RustUnnamed_20 = libc::c_uint;
pub const ACTOR_ID_MAX: C2RustUnnamed_20 = 471;
pub const ACTOR_OBJ_WARP2BLOCK: C2RustUnnamed_20 = 470;
pub const ACTOR_BG_JYA_BLOCK: C2RustUnnamed_20 = 469;
pub const ACTOR_EN_MM2: C2RustUnnamed_20 = 468;
pub const ACTOR_EN_ZL4: C2RustUnnamed_20 = 467;
pub const ACTOR_OBJ_HAMISHI: C2RustUnnamed_20 = 466;
pub const ACTOR_OBJ_TIMEBLOCK: C2RustUnnamed_20 = 465;
pub const ACTOR_EN_GE3: C2RustUnnamed_20 = 464;
pub const ACTOR_OBJ_MAKEKINSUTA: C2RustUnnamed_20 = 463;
pub const ACTOR_EN_ZO: C2RustUnnamed_20 = 462;
pub const ACTOR_BG_MENKURI_NISEKABE: C2RustUnnamed_20 = 461;
pub const ACTOR_EN_EG: C2RustUnnamed_20 = 460;
pub const ACTOR_OCEFF_WIPE4: C2RustUnnamed_20 = 459;
pub const ACTOR_EN_KAKASI3: C2RustUnnamed_20 = 458;
pub const ACTOR_EN_KAKASI2: C2RustUnnamed_20 = 457;
pub const ACTOR_BG_ICE_SHUTTER: C2RustUnnamed_20 = 456;
pub const ACTOR_BG_ICE_TURARA: C2RustUnnamed_20 = 455;
pub const ACTOR_EN_COW: C2RustUnnamed_20 = 454;
pub const ACTOR_EN_MA3: C2RustUnnamed_20 = 453;
pub const ACTOR_BG_SPOT18_SHUTTER: C2RustUnnamed_20 = 452;
pub const ACTOR_BG_SPOT18_FUTA: C2RustUnnamed_20 = 451;
pub const ACTOR_BG_SPOT11_OASIS: C2RustUnnamed_20 = 450;
pub const ACTOR_DOOR_KILLER: C2RustUnnamed_20 = 449;
pub const ACTOR_EN_CROW: C2RustUnnamed_20 = 448;
pub const ACTOR_EN_PO_DESERT: C2RustUnnamed_20 = 447;
pub const ACTOR_EN_WALL_TUBO: C2RustUnnamed_20 = 446;
pub const ACTOR_BG_BOWL_WALL: C2RustUnnamed_20 = 445;
pub const ACTOR_EN_DAIKU_KAKARIKO: C2RustUnnamed_20 = 444;
pub const ACTOR_BG_MIZU_SHUTTER: C2RustUnnamed_20 = 443;
pub const ACTOR_BG_MIZU_BWALL: C2RustUnnamed_20 = 442;
pub const ACTOR_EN_GS: C2RustUnnamed_20 = 441;
pub const ACTOR_EN_GB: C2RustUnnamed_20 = 440;
pub const ACTOR_BG_GND_ICEBLOCK: C2RustUnnamed_20 = 439;
pub const ACTOR_BG_GND_NISEKABE: C2RustUnnamed_20 = 438;
pub const ACTOR_BG_GND_SOULMEIRO: C2RustUnnamed_20 = 437;
pub const ACTOR_BG_GND_DARKMEIRO: C2RustUnnamed_20 = 436;
pub const ACTOR_BG_GND_FIREMEIRO: C2RustUnnamed_20 = 435;
pub const ACTOR_DEMO_GEFF: C2RustUnnamed_20 = 434;
pub const ACTOR_DEMO_GJ: C2RustUnnamed_20 = 433;
pub const ACTOR_EN_SKB: C2RustUnnamed_20 = 432;
pub const ACTOR_EN_WF: C2RustUnnamed_20 = 431;
pub const ACTOR_EN_GO2: C2RustUnnamed_20 = 430;
pub const ACTOR_EN_MU: C2RustUnnamed_20 = 429;
pub const ACTOR_EN_TG: C2RustUnnamed_20 = 428;
pub const ACTOR_OBJ_MURE3: C2RustUnnamed_20 = 427;
pub const ACTOR_UNSET_1AA: C2RustUnnamed_20 = 426;
pub const ACTOR_BG_SPOT17_BAKUDANKABE: C2RustUnnamed_20 = 425;
pub const ACTOR_BG_SPOT08_BAKUDANKABE: C2RustUnnamed_20 = 424;
pub const ACTOR_DEMO_KEKKAI: C2RustUnnamed_20 = 423;
pub const ACTOR_EN_HS2: C2RustUnnamed_20 = 422;
pub const ACTOR_BG_BOM_GUARD: C2RustUnnamed_20 = 421;
pub const ACTOR_EN_GUEST: C2RustUnnamed_20 = 420;
pub const ACTOR_EN_DNT_NOMAL: C2RustUnnamed_20 = 419;
pub const ACTOR_EN_DNT_JIJI: C2RustUnnamed_20 = 418;
pub const ACTOR_EN_DNT_DEMO: C2RustUnnamed_20 = 417;
pub const ACTOR_OBJ_KIBAKO2: C2RustUnnamed_20 = 416;
pub const ACTOR_BG_SPOT11_BAKUDANKABE: C2RustUnnamed_20 = 415;
pub const ACTOR_OBJ_COMB: C2RustUnnamed_20 = 414;
pub const ACTOR_BG_SPOT01_OBJECTS2: C2RustUnnamed_20 = 413;
pub const ACTOR_EN_SI: C2RustUnnamed_20 = 412;
pub const ACTOR_EN_DOG: C2RustUnnamed_20 = 411;
pub const ACTOR_EN_NIW_GIRL: C2RustUnnamed_20 = 410;
pub const ACTOR_OCEFF_WIPE3: C2RustUnnamed_20 = 409;
pub const ACTOR_OCEFF_WIPE2: C2RustUnnamed_20 = 408;
pub const ACTOR_EN_GELDB: C2RustUnnamed_20 = 407;
pub const ACTOR_EN_IT: C2RustUnnamed_20 = 406;
pub const ACTOR_EN_SHOPNUTS: C2RustUnnamed_20 = 405;
pub const ACTOR_BG_SPOT00_BREAK: C2RustUnnamed_20 = 404;
pub const ACTOR_EN_NUTSBALL: C2RustUnnamed_20 = 403;
pub const ACTOR_EN_HINTNUTS: C2RustUnnamed_20 = 402;
pub const ACTOR_BG_SPOT12_SAKU: C2RustUnnamed_20 = 401;
pub const ACTOR_BG_SPOT12_GATE: C2RustUnnamed_20 = 400;
pub const ACTOR_BG_JYA_HAHENIRON: C2RustUnnamed_20 = 399;
pub const ACTOR_BG_JYA_1FLIFT: C2RustUnnamed_20 = 398;
pub const ACTOR_BG_SPOT05_SOKO: C2RustUnnamed_20 = 397;
pub const ACTOR_EN_WEIYER: C2RustUnnamed_20 = 396;
pub const ACTOR_OCEFF_STORM: C2RustUnnamed_20 = 395;
pub const ACTOR_OCEFF_WIPE: C2RustUnnamed_20 = 394;
pub const ACTOR_EN_STH: C2RustUnnamed_20 = 393;
pub const ACTOR_EN_SSH: C2RustUnnamed_20 = 392;
pub const ACTOR_OBJ_ROOMTIMER: C2RustUnnamed_20 = 391;
pub const ACTOR_EN_GE2: C2RustUnnamed_20 = 390;
pub const ACTOR_EN_WONDER_TALK2: C2RustUnnamed_20 = 389;
pub const ACTOR_EN_DY_EXTRA: C2RustUnnamed_20 = 388;
pub const ACTOR_SHOT_SUN: C2RustUnnamed_20 = 387;
pub const ACTOR_DEMO_EC: C2RustUnnamed_20 = 386;
pub const ACTOR_EN_TORCH: C2RustUnnamed_20 = 385;
pub const ACTOR_UNSET_180: C2RustUnnamed_20 = 384;
pub const ACTOR_END_TITLE: C2RustUnnamed_20 = 383;
pub const ACTOR_OCEFF_SPOT: C2RustUnnamed_20 = 382;
pub const ACTOR_OBJ_MAKEOSHIHIKI: C2RustUnnamed_20 = 381;
pub const ACTOR_EN_TAKARA_MAN: C2RustUnnamed_20 = 380;
pub const ACTOR_EN_KAKASI: C2RustUnnamed_20 = 379;
pub const ACTOR_BOSS_GANON2: C2RustUnnamed_20 = 378;
pub const ACTOR_EN_ZL3: C2RustUnnamed_20 = 377;
pub const ACTOR_EN_HEISHI4: C2RustUnnamed_20 = 376;
pub const ACTOR_BG_ZG: C2RustUnnamed_20 = 375;
pub const ACTOR_EFC_ERUPC: C2RustUnnamed_20 = 374;
pub const ACTOR_EN_PO_FIELD: C2RustUnnamed_20 = 373;
pub const ACTOR_DEMO_GT: C2RustUnnamed_20 = 372;
pub const ACTOR_ELF_MSG2: C2RustUnnamed_20 = 371;
pub const ACTOR_DOOR_GERUDO: C2RustUnnamed_20 = 370;
pub const ACTOR_EN_MAG: C2RustUnnamed_20 = 369;
pub const ACTOR_EN_OKARINA_EFFECT: C2RustUnnamed_20 = 368;
pub const ACTOR_EN_GANON_MANT: C2RustUnnamed_20 = 367;
pub const ACTOR_EN_HY: C2RustUnnamed_20 = 366;
pub const ACTOR_EN_MD: C2RustUnnamed_20 = 365;
pub const ACTOR_EN_CS: C2RustUnnamed_20 = 364;
pub const ACTOR_EN_JSJUTAN: C2RustUnnamed_20 = 363;
pub const ACTOR_EN_JS: C2RustUnnamed_20 = 362;
pub const ACTOR_BG_JYA_IRONOBJ: C2RustUnnamed_20 = 361;
pub const ACTOR_EN_EX_ITEM: C2RustUnnamed_20 = 360;
pub const ACTOR_EN_ANI: C2RustUnnamed_20 = 359;
pub const ACTOR_BG_SST_FLOOR: C2RustUnnamed_20 = 358;
pub const ACTOR_EN_WEATHER_TAG: C2RustUnnamed_20 = 357;
pub const ACTOR_EN_KZ: C2RustUnnamed_20 = 356;
pub const ACTOR_EN_KO: C2RustUnnamed_20 = 355;
pub const ACTOR_EN_MM: C2RustUnnamed_20 = 354;
pub const ACTOR_UNSET_161: C2RustUnnamed_20 = 353;
pub const ACTOR_EN_STREAM: C2RustUnnamed_20 = 352;
pub const ACTOR_EN_SIOFUKI: C2RustUnnamed_20 = 351;
pub const ACTOR_EN_GANON_ORGAN: C2RustUnnamed_20 = 350;
pub const ACTOR_UNSET_15D: C2RustUnnamed_20 = 349;
pub const ACTOR_BG_SPOT18_BASKET: C2RustUnnamed_20 = 348;
pub const ACTOR_BG_JYA_BOMBIWA: C2RustUnnamed_20 = 347;
pub const ACTOR_BG_JYA_AMISHUTTER: C2RustUnnamed_20 = 346;
pub const ACTOR_BG_JYA_BOMBCHUIWA: C2RustUnnamed_20 = 345;
pub const ACTOR_BG_JYA_BIGMIRROR: C2RustUnnamed_20 = 344;
pub const ACTOR_BG_JYA_LIFT: C2RustUnnamed_20 = 343;
pub const ACTOR_BG_JYA_MEGAMI: C2RustUnnamed_20 = 342;
pub const ACTOR_EN_CHANGER: C2RustUnnamed_20 = 341;
pub const ACTOR_UNSET_154: C2RustUnnamed_20 = 340;
pub const ACTOR_EN_FU: C2RustUnnamed_20 = 339;
pub const ACTOR_EN_GO: C2RustUnnamed_20 = 338;
pub const ACTOR_OBJ_MURE2: C2RustUnnamed_20 = 337;
pub const ACTOR_OBJ_LIGHTSWITCH: C2RustUnnamed_20 = 336;
pub const ACTOR_OBJ_HANA: C2RustUnnamed_20 = 335;
pub const ACTOR_EN_ISHI: C2RustUnnamed_20 = 334;
pub const ACTOR_EN_OWL: C2RustUnnamed_20 = 333;
pub const ACTOR_EN_BOM_BOWL_PIT: C2RustUnnamed_20 = 332;
pub const ACTOR_EN_BOM_BOWL_MAN: C2RustUnnamed_20 = 331;
pub const ACTOR_EN_MK: C2RustUnnamed_20 = 330;
pub const ACTOR_EN_DS: C2RustUnnamed_20 = 329;
pub const ACTOR_BG_GJYO_BRIDGE: C2RustUnnamed_20 = 328;
pub const ACTOR_EN_WONDER_TALK: C2RustUnnamed_20 = 327;
pub const ACTOR_EN_SA: C2RustUnnamed_20 = 326;
pub const ACTOR_BG_SPOT01_IDOSOKO: C2RustUnnamed_20 = 325;
pub const ACTOR_EN_ATTACK_NIW: C2RustUnnamed_20 = 324;
pub const ACTOR_EN_SYATEKI_NIW: C2RustUnnamed_20 = 323;
pub const ACTOR_EN_HEISHI3: C2RustUnnamed_20 = 322;
pub const ACTOR_EN_KANBAN: C2RustUnnamed_20 = 321;
pub const ACTOR_BG_INGATE: C2RustUnnamed_20 = 320;
pub const ACTOR_EN_HS: C2RustUnnamed_20 = 319;
pub const ACTOR_EN_MS: C2RustUnnamed_20 = 318;
pub const ACTOR_EN_GM: C2RustUnnamed_20 = 317;
pub const ACTOR_EN_NIW_LADY: C2RustUnnamed_20 = 316;
pub const ACTOR_EN_CLEAR_TAG: C2RustUnnamed_20 = 315;
pub const ACTOR_EN_SDA: C2RustUnnamed_20 = 314;
pub const ACTOR_OBJ_BLOCKSTOP: C2RustUnnamed_20 = 313;
pub const ACTOR_EN_GE1: C2RustUnnamed_20 = 312;
pub const ACTOR_ITEM_INBOX: C2RustUnnamed_20 = 311;
pub const ACTOR_EN_BLKOBJ: C2RustUnnamed_20 = 310;
pub const ACTOR_EN_NWC: C2RustUnnamed_20 = 309;
pub const ACTOR_UNSET_134: C2RustUnnamed_20 = 308;
pub const ACTOR_EN_DAIKU: C2RustUnnamed_20 = 307;
pub const ACTOR_EN_TORYO: C2RustUnnamed_20 = 306;
pub const ACTOR_EN_EX_RUPPY: C2RustUnnamed_20 = 305;
pub const ACTOR_EN_GOROIWA: C2RustUnnamed_20 = 304;
pub const ACTOR_EN_YABUSAME_MARK: C2RustUnnamed_20 = 303;
pub const ACTOR_EN_OKARINA_TAG: C2RustUnnamed_20 = 302;
pub const ACTOR_OBJ_HSBLOCK: C2RustUnnamed_20 = 301;
pub const ACTOR_OBJ_LIFT: C2RustUnnamed_20 = 300;
pub const ACTOR_OBJ_ELEVATOR: C2RustUnnamed_20 = 299;
pub const ACTOR_OBJ_SWITCH: C2RustUnnamed_20 = 298;
pub const ACTOR_UNSET_129: C2RustUnnamed_20 = 297;
pub const ACTOR_UNSET_128: C2RustUnnamed_20 = 296;
pub const ACTOR_OBJ_BOMBIWA: C2RustUnnamed_20 = 295;
pub const ACTOR_OBJ_BEAN: C2RustUnnamed_20 = 294;
pub const ACTOR_EN_KUSA: C2RustUnnamed_20 = 293;
pub const ACTOR_EN_DIVING_GAME: C2RustUnnamed_20 = 292;
pub const ACTOR_BG_RELAY_OBJECTS: C2RustUnnamed_20 = 291;
pub const ACTOR_EN_PO_RELAY: C2RustUnnamed_20 = 290;
pub const ACTOR_EN_FZ: C2RustUnnamed_20 = 289;
pub const ACTOR_BG_SPOT07_TAKI: C2RustUnnamed_20 = 288;
pub const ACTOR_BG_SPOT03_TAKI: C2RustUnnamed_20 = 287;
pub const ACTOR_OBJ_ICE_POLY: C2RustUnnamed_20 = 286;
pub const ACTOR_EN_TUBO_TRAP: C2RustUnnamed_20 = 285;
pub const ACTOR_EN_HONOTRAP: C2RustUnnamed_20 = 284;
pub const ACTOR_ELF_MSG: C2RustUnnamed_20 = 283;
pub const ACTOR_EN_DNS: C2RustUnnamed_20 = 282;
pub const ACTOR_DEMO_SHD: C2RustUnnamed_20 = 281;
pub const ACTOR_DEMO_EXT: C2RustUnnamed_20 = 280;
pub const ACTOR_EN_G_SWITCH: C2RustUnnamed_20 = 279;
pub const ACTOR_EN_SKJNEEDLE: C2RustUnnamed_20 = 278;
pub const ACTOR_EN_SKJ: C2RustUnnamed_20 = 277;
pub const ACTOR_DEMO_IK: C2RustUnnamed_20 = 276;
pub const ACTOR_EN_IK: C2RustUnnamed_20 = 275;
pub const ACTOR_EN_WONDER_ITEM: C2RustUnnamed_20 = 274;
pub const ACTOR_OBJ_TSUBO: C2RustUnnamed_20 = 273;
pub const ACTOR_OBJ_KIBAKO: C2RustUnnamed_20 = 272;
pub const ACTOR_ITEM_ETCETERA: C2RustUnnamed_20 = 271;
pub const ACTOR_UNSET_10E: C2RustUnnamed_20 = 270;
pub const ACTOR_UNSET_10D: C2RustUnnamed_20 = 269;
pub const ACTOR_ARROW_LIGHT: C2RustUnnamed_20 = 268;
pub const ACTOR_ARROW_ICE: C2RustUnnamed_20 = 267;
pub const ACTOR_ARROW_FIRE: C2RustUnnamed_20 = 266;
pub const ACTOR_UNSET_109: C2RustUnnamed_20 = 265;
pub const ACTOR_BG_UMAJUMP: C2RustUnnamed_20 = 264;
pub const ACTOR_BG_SPOT15_RRBOX: C2RustUnnamed_20 = 263;
pub const ACTOR_BG_GANON_OTYUKA: C2RustUnnamed_20 = 262;
pub const ACTOR_BG_PO_SYOKUDAI: C2RustUnnamed_20 = 261;
pub const ACTOR_BG_SPOT01_IDOMIZU: C2RustUnnamed_20 = 260;
pub const ACTOR_BG_SPOT01_IDOHASHIRA: C2RustUnnamed_20 = 259;
pub const ACTOR_BG_SPOT01_FUSYA: C2RustUnnamed_20 = 258;
pub const ACTOR_EFF_DUST: C2RustUnnamed_20 = 257;
pub const ACTOR_BG_GATE_SHUTTER: C2RustUnnamed_20 = 256;
pub const ACTOR_OBJ_OSHIHIKI: C2RustUnnamed_20 = 255;
pub const ACTOR_FISHING: C2RustUnnamed_20 = 254;
pub const ACTOR_BG_JYA_KANAAMI: C2RustUnnamed_20 = 253;
pub const ACTOR_BG_JYA_COBRA: C2RustUnnamed_20 = 252;
pub const ACTOR_UNSET_FB: C2RustUnnamed_20 = 251;
pub const ACTOR_BG_JYA_ZURERUKABE: C2RustUnnamed_20 = 250;
pub const ACTOR_BG_JYA_GOROIWA: C2RustUnnamed_20 = 249;
pub const ACTOR_BG_SPOT15_SAKU: C2RustUnnamed_20 = 248;
pub const ACTOR_BG_HAKA_GATE: C2RustUnnamed_20 = 247;
pub const ACTOR_EN_ANUBICE_TAG: C2RustUnnamed_20 = 246;
pub const ACTOR_DEMO_6K: C2RustUnnamed_20 = 245;
pub const ACTOR_MAGIC_DARK: C2RustUnnamed_20 = 244;
pub const ACTOR_UNSET_F3: C2RustUnnamed_20 = 243;
pub const ACTOR_UNSET_F2: C2RustUnnamed_20 = 242;
pub const ACTOR_ITEM_OCARINA: C2RustUnnamed_20 = 241;
pub const ACTOR_EN_ICE_HONO: C2RustUnnamed_20 = 240;
pub const ACTOR_BG_ICE_SHELTER: C2RustUnnamed_20 = 239;
pub const ACTOR_ITEM_SHIELD: C2RustUnnamed_20 = 238;
pub const ACTOR_EN_FR: C2RustUnnamed_20 = 237;
pub const ACTOR_EN_NY: C2RustUnnamed_20 = 236;
pub const ACTOR_UNSET_EB: C2RustUnnamed_20 = 235;
pub const ACTOR_UNSET_EA: C2RustUnnamed_20 = 234;
pub const ACTOR_BOSS_SST: C2RustUnnamed_20 = 233;
pub const ACTOR_BOSS_GANON: C2RustUnnamed_20 = 232;
pub const ACTOR_EN_MA1: C2RustUnnamed_20 = 231;
pub const ACTOR_BG_BDAN_SWITCH: C2RustUnnamed_20 = 230;
pub const ACTOR_BG_SPOT16_DOUGHNUT: C2RustUnnamed_20 = 229;
pub const ACTOR_BG_MORI_IDOMIZU: C2RustUnnamed_20 = 228;
pub const ACTOR_BG_MORI_HASHIRA4: C2RustUnnamed_20 = 227;
pub const ACTOR_BG_MORI_HASHIGO: C2RustUnnamed_20 = 226;
pub const ACTOR_EN_ANUBICE_FIRE: C2RustUnnamed_20 = 225;
pub const ACTOR_EN_ANUBICE: C2RustUnnamed_20 = 224;
pub const ACTOR_EN_BX: C2RustUnnamed_20 = 223;
pub const ACTOR_EN_BA: C2RustUnnamed_20 = 222;
pub const ACTOR_EN_RR: C2RustUnnamed_20 = 221;
pub const ACTOR_BOSS_TW: C2RustUnnamed_20 = 220;
pub const ACTOR_EN_HORSE_GAME_CHECK: C2RustUnnamed_20 = 219;
pub const ACTOR_EN_BOM_CHU: C2RustUnnamed_20 = 218;
pub const ACTOR_EN_MA2: C2RustUnnamed_20 = 217;
pub const ACTOR_UNSET_D8: C2RustUnnamed_20 = 216;
pub const ACTOR_BG_HAKA_WATER: C2RustUnnamed_20 = 215;
pub const ACTOR_BG_ICE_OBJECTS: C2RustUnnamed_20 = 214;
pub const ACTOR_BG_SPOT06_OBJECTS: C2RustUnnamed_20 = 213;
pub const ACTOR_BG_MIZU_UZU: C2RustUnnamed_20 = 212;
pub const ACTOR_OBJ_DEKUJR: C2RustUnnamed_20 = 211;
pub const ACTOR_EN_RU2: C2RustUnnamed_20 = 210;
pub const ACTOR_BG_SPOT08_ICEBLOCK: C2RustUnnamed_20 = 209;
pub const ACTOR_BG_BOMBWALL: C2RustUnnamed_20 = 208;
pub const ACTOR_BG_HIDAN_KOWARERUKABE: C2RustUnnamed_20 = 207;
pub const ACTOR_UNSET_CE: C2RustUnnamed_20 = 206;
pub const ACTOR_BG_SPOT16_BOMBSTONE: C2RustUnnamed_20 = 205;
pub const ACTOR_EN_TR: C2RustUnnamed_20 = 204;
pub const ACTOR_EN_IN: C2RustUnnamed_20 = 203;
pub const ACTOR_DEMO_GO: C2RustUnnamed_20 = 202;
pub const ACTOR_DEMO_SA: C2RustUnnamed_20 = 201;
pub const ACTOR_BG_BDAN_OBJECTS: C2RustUnnamed_20 = 200;
pub const ACTOR_EN_KAREBABA: C2RustUnnamed_20 = 199;
pub const ACTOR_EN_BIGOKUTA: C2RustUnnamed_20 = 198;
pub const ACTOR_EN_SB: C2RustUnnamed_20 = 197;
pub const ACTOR_BOSS_MO: C2RustUnnamed_20 = 196;
pub const ACTOR_EN_NB: C2RustUnnamed_20 = 195;
pub const ACTOR_EN_TANA: C2RustUnnamed_20 = 194;
pub const ACTOR_EN_SYATEKI_MAN: C2RustUnnamed_20 = 193;
pub const ACTOR_EN_SYATEKI_ITM: C2RustUnnamed_20 = 192;
pub const ACTOR_BG_SPOT17_FUNEN: C2RustUnnamed_20 = 191;
pub const ACTOR_BG_HAKA_ZOU: C2RustUnnamed_20 = 190;
pub const ACTOR_BG_HAKA_HUTA: C2RustUnnamed_20 = 189;
pub const ACTOR_BG_HAKA_TRAP: C2RustUnnamed_20 = 188;
pub const ACTOR_BG_HAKA_TUBO: C2RustUnnamed_20 = 187;
pub const ACTOR_BOSS_VA: C2RustUnnamed_20 = 186;
pub const ACTOR_BG_SPOT18_OBJ: C2RustUnnamed_20 = 185;
pub const ACTOR_BG_SPOT09_OBJ: C2RustUnnamed_20 = 184;
pub const ACTOR_MIR_RAY: C2RustUnnamed_20 = 183;
pub const ACTOR_EN_BROB: C2RustUnnamed_20 = 182;
pub const ACTOR_EN_FIRE_ROCK: C2RustUnnamed_20 = 181;
pub const ACTOR_EN_ENCOUNT2: C2RustUnnamed_20 = 180;
pub const ACTOR_EN_HEISHI2: C2RustUnnamed_20 = 179;
pub const ACTOR_UNSET_B2: C2RustUnnamed_20 = 178;
pub const ACTOR_BG_HAKA_SGAMI: C2RustUnnamed_20 = 177;
pub const ACTOR_BG_HAKA_SHIP: C2RustUnnamed_20 = 176;
pub const ACTOR_BG_HAKA_MEGANEBG: C2RustUnnamed_20 = 175;
pub const ACTOR_BG_HAKA_MEGANE: C2RustUnnamed_20 = 174;
pub const ACTOR_EN_VB_BALL: C2RustUnnamed_20 = 173;
pub const ACTOR_BG_VB_SIMA: C2RustUnnamed_20 = 172;
pub const ACTOR_EN_FW: C2RustUnnamed_20 = 171;
pub const ACTOR_DEMO_TRE_LGT: C2RustUnnamed_20 = 170;
pub const ACTOR_DEMO_IM: C2RustUnnamed_20 = 169;
pub const ACTOR_DEMO_DU: C2RustUnnamed_20 = 168;
pub const ACTOR_EN_ENCOUNT1: C2RustUnnamed_20 = 167;
pub const ACTOR_EN_RL: C2RustUnnamed_20 = 166;
pub const ACTOR_EN_DHA: C2RustUnnamed_20 = 165;
pub const ACTOR_EN_DH: C2RustUnnamed_20 = 164;
pub const ACTOR_EN_FD_FIRE: C2RustUnnamed_20 = 163;
pub const ACTOR_BOSS_FD2: C2RustUnnamed_20 = 162;
pub const ACTOR_EN_RU1: C2RustUnnamed_20 = 161;
pub const ACTOR_UNSET_A0: C2RustUnnamed_20 = 160;
pub const ACTOR_MAGIC_FIRE: C2RustUnnamed_20 = 159;
pub const ACTOR_MAGIC_WIND: C2RustUnnamed_20 = 158;
pub const ACTOR_BG_HAKA: C2RustUnnamed_20 = 157;
pub const ACTOR_BG_SPOT02_OBJECTS: C2RustUnnamed_20 = 156;
pub const ACTOR_DOOR_ANA: C2RustUnnamed_20 = 155;
pub const ACTOR_EN_HORSE_LINK_CHILD: C2RustUnnamed_20 = 154;
pub const ACTOR_EN_FD: C2RustUnnamed_20 = 153;
pub const ACTOR_EN_DU: C2RustUnnamed_20 = 152;
pub const ACTOR_OBJECT_KANKYO: C2RustUnnamed_20 = 151;
pub const ACTOR_BOSS_FD: C2RustUnnamed_20 = 150;
pub const ACTOR_EN_SW: C2RustUnnamed_20 = 149;
pub const ACTOR_OBJ_MURE: C2RustUnnamed_20 = 148;
pub const ACTOR_BG_PO_EVENT: C2RustUnnamed_20 = 147;
pub const ACTOR_BG_HEAVY_BLOCK: C2RustUnnamed_20 = 146;
pub const ACTOR_EN_PO_SISTERS: C2RustUnnamed_20 = 145;
pub const ACTOR_EN_RD: C2RustUnnamed_20 = 144;
pub const ACTOR_EN_HEISHI1: C2RustUnnamed_20 = 143;
pub const ACTOR_EN_FLOORMAS: C2RustUnnamed_20 = 142;
pub const ACTOR_BG_HIDAN_FWBIG: C2RustUnnamed_20 = 141;
pub const ACTOR_DEMO_KANKYO: C2RustUnnamed_20 = 140;
pub const ACTOR_DEMO_EFFECT: C2RustUnnamed_20 = 139;
pub const ACTOR_EN_VM: C2RustUnnamed_20 = 138;
pub const ACTOR_BG_MORI_RAKKATENJO: C2RustUnnamed_20 = 137;
pub const ACTOR_BG_MORI_KAITENKABE: C2RustUnnamed_20 = 136;
pub const ACTOR_BG_MORI_ELEVATOR: C2RustUnnamed_20 = 135;
pub const ACTOR_BG_MORI_BIGST: C2RustUnnamed_20 = 134;
pub const ACTOR_EN_TK: C2RustUnnamed_20 = 133;
pub const ACTOR_EN_TA: C2RustUnnamed_20 = 132;
pub const ACTOR_UNSET_83: C2RustUnnamed_20 = 131;
pub const ACTOR_EN_VASE: C2RustUnnamed_20 = 130;
pub const ACTOR_EN_AROW_TRAP: C2RustUnnamed_20 = 129;
pub const ACTOR_EN_TRAP: C2RustUnnamed_20 = 128;
pub const ACTOR_UNSET_7F: C2RustUnnamed_20 = 127;
pub const ACTOR_UNSET_7E: C2RustUnnamed_20 = 126;
pub const ACTOR_EN_PU_BOX: C2RustUnnamed_20 = 125;
pub const ACTOR_EN_LIGHTBOX: C2RustUnnamed_20 = 124;
pub const ACTOR_UNSET_7B: C2RustUnnamed_20 = 123;
pub const ACTOR_UNSET_7A: C2RustUnnamed_20 = 122;
pub const ACTOR_UNSET_79: C2RustUnnamed_20 = 121;
pub const ACTOR_UNSET_78: C2RustUnnamed_20 = 120;
pub const ACTOR_EN_WOOD02: C2RustUnnamed_20 = 119;
pub const ACTOR_UNSET_76: C2RustUnnamed_20 = 118;
pub const ACTOR_UNSET_75: C2RustUnnamed_20 = 117;
pub const ACTOR_UNSET_74: C2RustUnnamed_20 = 116;
pub const ACTOR_UNSET_73: C2RustUnnamed_20 = 115;
pub const ACTOR_EN_BIRD: C2RustUnnamed_20 = 114;
pub const ACTOR_BG_HIDAN_HAMSTEP: C2RustUnnamed_20 = 113;
pub const ACTOR_DOOR_TOKI: C2RustUnnamed_20 = 112;
pub const ACTOR_BG_HIDAN_KOUSI: C2RustUnnamed_20 = 111;
pub const ACTOR_BG_MJIN: C2RustUnnamed_20 = 110;
pub const ACTOR_EN_FHG_FIRE: C2RustUnnamed_20 = 109;
pub const ACTOR_BG_TOKI_SWD: C2RustUnnamed_20 = 108;
pub const ACTOR_EN_YUKABYUN: C2RustUnnamed_20 = 107;
pub const ACTOR_BG_TOKI_HIKARI: C2RustUnnamed_20 = 106;
pub const ACTOR_EN_BB: C2RustUnnamed_20 = 105;
pub const ACTOR_BG_MORI_HINERI: C2RustUnnamed_20 = 104;
pub const ACTOR_EN_FHG: C2RustUnnamed_20 = 103;
pub const ACTOR_ARMS_HOOK: C2RustUnnamed_20 = 102;
pub const ACTOR_BG_MIZU_WATER: C2RustUnnamed_20 = 101;
pub const ACTOR_BG_MIZU_MOVEBG: C2RustUnnamed_20 = 100;
pub const ACTOR_EN_VALI: C2RustUnnamed_20 = 99;
pub const ACTOR_BG_MENKURI_EYE: C2RustUnnamed_20 = 98;
pub const ACTOR_BG_MENKURI_KAITEN: C2RustUnnamed_20 = 97;
pub const ACTOR_EN_DEKUNUTS: C2RustUnnamed_20 = 96;
pub const ACTOR_ITEM_B_HEART: C2RustUnnamed_20 = 95;
pub const ACTOR_OBJ_SYOKUDAI: C2RustUnnamed_20 = 94;
pub const ACTOR_DOOR_WARP1: C2RustUnnamed_20 = 93;
pub const ACTOR_BG_DDAN_KD: C2RustUnnamed_20 = 92;
pub const ACTOR_EN_HORSE_ZELDA: C2RustUnnamed_20 = 91;
pub const ACTOR_EN_JJ: C2RustUnnamed_20 = 90;
pub const ACTOR_BG_BREAKWALL: C2RustUnnamed_20 = 89;
pub const ACTOR_BG_DDAN_JD: C2RustUnnamed_20 = 88;
pub const ACTOR_EN_M_THUNDER: C2RustUnnamed_20 = 87;
pub const ACTOR_EN_M_FIRE1: C2RustUnnamed_20 = 86;
pub const ACTOR_EN_DEKUBABA: C2RustUnnamed_20 = 85;
pub const ACTOR_EN_AM: C2RustUnnamed_20 = 84;
pub const ACTOR_UNSET_53: C2RustUnnamed_20 = 83;
pub const ACTOR_BOSS_GANONDROF: C2RustUnnamed_20 = 82;
pub const ACTOR_BG_YDAN_MARUTA: C2RustUnnamed_20 = 81;
pub const ACTOR_BG_YDAN_HASI: C2RustUnnamed_20 = 80;
pub const ACTOR_EN_OE2: C2RustUnnamed_20 = 79;
pub const ACTOR_BG_HIDAN_FSLIFT: C2RustUnnamed_20 = 78;
pub const ACTOR_EN_ZL2: C2RustUnnamed_20 = 77;
pub const ACTOR_EN_BOMBF: C2RustUnnamed_20 = 76;
pub const ACTOR_EN_MB: C2RustUnnamed_20 = 75;
pub const ACTOR_BG_SPOT00_HANEBASI: C2RustUnnamed_20 = 74;
pub const ACTOR_BG_HIDAN_CURTAIN: C2RustUnnamed_20 = 73;
pub const ACTOR_EN_XC: C2RustUnnamed_20 = 72;
pub const ACTOR_BG_HIDAN_SYOKU: C2RustUnnamed_20 = 71;
pub const ACTOR_BG_HIDAN_SIMA: C2RustUnnamed_20 = 70;
pub const ACTOR_BG_HIDAN_SEKIZOU: C2RustUnnamed_20 = 69;
pub const ACTOR_BG_HIDAN_RSEKIZOU: C2RustUnnamed_20 = 68;
pub const ACTOR_BG_HIDAN_ROCK: C2RustUnnamed_20 = 67;
pub const ACTOR_EN_HORSE_GANON: C2RustUnnamed_20 = 66;
pub const ACTOR_BG_HIDAN_HROCK: C2RustUnnamed_20 = 65;
pub const ACTOR_BG_HIDAN_DALM: C2RustUnnamed_20 = 64;
pub const ACTOR_BG_DODOAGO: C2RustUnnamed_20 = 63;
pub const ACTOR_BG_TREEMOUTH: C2RustUnnamed_20 = 62;
pub const ACTOR_EN_OSSAN: C2RustUnnamed_20 = 61;
pub const ACTOR_EN_HORSE_NORMAL: C2RustUnnamed_20 = 60;
pub const ACTOR_EN_RIVER_SOUND: C2RustUnnamed_20 = 59;
pub const ACTOR_EN_EIYER: C2RustUnnamed_20 = 58;
pub const ACTOR_EN_A_OBJ: C2RustUnnamed_20 = 57;
pub const ACTOR_EN_BW: C2RustUnnamed_20 = 56;
pub const ACTOR_EN_ST: C2RustUnnamed_20 = 55;
pub const ACTOR_UNSET_36: C2RustUnnamed_20 = 54;
pub const ACTOR_EN_TP: C2RustUnnamed_20 = 53;
pub const ACTOR_EN_BILI: C2RustUnnamed_20 = 52;
pub const ACTOR_EN_TORCH2: C2RustUnnamed_20 = 51;
pub const ACTOR_EN_BOOM: C2RustUnnamed_20 = 50;
pub const ACTOR_UNSET_31: C2RustUnnamed_20 = 49;
pub const ACTOR_EN_BDFIRE: C2RustUnnamed_20 = 48;
pub const ACTOR_EN_DODOJR: C2RustUnnamed_20 = 47;
pub const ACTOR_DOOR_SHUTTER: C2RustUnnamed_20 = 46;
pub const ACTOR_EN_BUBBLE: C2RustUnnamed_20 = 45;
pub const ACTOR_BG_PUSHBOX: C2RustUnnamed_20 = 44;
pub const ACTOR_EN_GOMA: C2RustUnnamed_20 = 43;
pub const ACTOR_EN_VIEWER: C2RustUnnamed_20 = 42;
pub const ACTOR_EN_ZL1: C2RustUnnamed_20 = 41;
pub const ACTOR_BOSS_GOMA: C2RustUnnamed_20 = 40;
pub const ACTOR_BOSS_DODONGO: C2RustUnnamed_20 = 39;
pub const ACTOR_EN_HATA: C2RustUnnamed_20 = 38;
pub const ACTOR_EN_ZF: C2RustUnnamed_20 = 37;
pub const ACTOR_EN_SCENE_CHANGE: C2RustUnnamed_20 = 36;
pub const ACTOR_EN_HOLL: C2RustUnnamed_20 = 35;
pub const ACTOR_UNSET_22: C2RustUnnamed_20 = 34;
pub const ACTOR_EN_FISH: C2RustUnnamed_20 = 33;
pub const ACTOR_EN_INSECT: C2RustUnnamed_20 = 32;
pub const ACTOR_UNSET_1F: C2RustUnnamed_20 = 31;
pub const ACTOR_EN_BUTTE: C2RustUnnamed_20 = 30;
pub const ACTOR_EN_PEEHAT: C2RustUnnamed_20 = 29;
pub const ACTOR_EN_REEBA: C2RustUnnamed_20 = 28;
pub const ACTOR_EN_TITE: C2RustUnnamed_20 = 27;
pub const ACTOR_UNSET_1A: C2RustUnnamed_20 = 26;
pub const ACTOR_EN_NIW: C2RustUnnamed_20 = 25;
pub const ACTOR_EN_ELF: C2RustUnnamed_20 = 24;
pub const ACTOR_UNSET_17: C2RustUnnamed_20 = 23;
pub const ACTOR_EN_ARROW: C2RustUnnamed_20 = 22;
pub const ACTOR_EN_ITEM00: C2RustUnnamed_20 = 21;
pub const ACTOR_EN_HORSE: C2RustUnnamed_20 = 20;
pub const ACTOR_EN_FIREFLY: C2RustUnnamed_20 = 19;
pub const ACTOR_EN_DODONGO: C2RustUnnamed_20 = 18;
pub const ACTOR_EN_WALLMAS: C2RustUnnamed_20 = 17;
pub const ACTOR_EN_BOM: C2RustUnnamed_20 = 16;
pub const ACTOR_BG_YDAN_SP: C2RustUnnamed_20 = 15;
pub const ACTOR_EN_OKUTA: C2RustUnnamed_20 = 14;
pub const ACTOR_EN_POH: C2RustUnnamed_20 = 13;
pub const ACTOR_BG_HIDAN_FIREWALL: C2RustUnnamed_20 = 12;
pub const ACTOR_BG_DY_YOSEIZO: C2RustUnnamed_20 = 11;
pub const ACTOR_EN_BOX: C2RustUnnamed_20 = 10;
pub const ACTOR_EN_DOOR: C2RustUnnamed_20 = 9;
pub const ACTOR_EN_LIGHT: C2RustUnnamed_20 = 8;
pub const ACTOR_EN_PART: C2RustUnnamed_20 = 7;
pub const ACTOR_UNSET_6: C2RustUnnamed_20 = 6;
pub const ACTOR_UNSET_5: C2RustUnnamed_20 = 5;
pub const ACTOR_EN_GIRLA: C2RustUnnamed_20 = 4;
pub const ACTOR_UNSET_3: C2RustUnnamed_20 = 3;
pub const ACTOR_EN_TEST: C2RustUnnamed_20 = 2;
pub const ACTOR_UNSET_1: C2RustUnnamed_20 = 1;
pub const ACTOR_PLAYER: C2RustUnnamed_20 = 0;
pub type C2RustUnnamed_21 = libc::c_uint;
pub const PLAYER_AP_MAX: C2RustUnnamed_21 = 67;
pub const PLAYER_AP_LENS: C2RustUnnamed_21 = 66;
pub const PLAYER_AP_MASK_TRUTH: C2RustUnnamed_21 = 65;
pub const PLAYER_AP_MASK_GERUDO: C2RustUnnamed_21 = 64;
pub const PLAYER_AP_MASK_ZORA: C2RustUnnamed_21 = 63;
pub const PLAYER_AP_MASK_GORON: C2RustUnnamed_21 = 62;
pub const PLAYER_AP_MASK_BUNNY: C2RustUnnamed_21 = 61;
pub const PLAYER_AP_MASK_SPOOKY: C2RustUnnamed_21 = 60;
pub const PLAYER_AP_MASK_SKULL: C2RustUnnamed_21 = 59;
pub const PLAYER_AP_MASK_KEATON: C2RustUnnamed_21 = 58;
pub const PLAYER_AP_CLAIM_CHECK: C2RustUnnamed_21 = 57;
pub const PLAYER_AP_EYEDROPS: C2RustUnnamed_21 = 56;
pub const PLAYER_AP_FROG: C2RustUnnamed_21 = 55;
pub const PLAYER_AP_PRESCRIPTION: C2RustUnnamed_21 = 54;
pub const PLAYER_AP_SWORD_BROKEN: C2RustUnnamed_21 = 53;
pub const PLAYER_AP_SAW: C2RustUnnamed_21 = 52;
pub const PLAYER_AP_ODD_POTION: C2RustUnnamed_21 = 51;
pub const PLAYER_AP_ODD_MUSHROOM: C2RustUnnamed_21 = 50;
pub const PLAYER_AP_COJIRO: C2RustUnnamed_21 = 49;
pub const PLAYER_AP_POCKET_CUCCO: C2RustUnnamed_21 = 48;
pub const PLAYER_AP_POCKET_EGG: C2RustUnnamed_21 = 47;
pub const PLAYER_AP_BEAN: C2RustUnnamed_21 = 46;
pub const PLAYER_AP_CHICKEN: C2RustUnnamed_21 = 45;
pub const PLAYER_AP_WEIRD_EGG: C2RustUnnamed_21 = 44;
pub const PLAYER_AP_LETTER_ZELDA: C2RustUnnamed_21 = 43;
pub const PLAYER_AP_BOTTLE_FAIRY: C2RustUnnamed_21 = 42;
pub const PLAYER_AP_BOTTLE_MILK_HALF: C2RustUnnamed_21 = 41;
pub const PLAYER_AP_BOTTLE_MILK: C2RustUnnamed_21 = 40;
pub const PLAYER_AP_BOTTLE_POTION_GREEN: C2RustUnnamed_21 = 39;
pub const PLAYER_AP_BOTTLE_POTION_BLUE: C2RustUnnamed_21 = 38;
pub const PLAYER_AP_BOTTLE_POTION_RED: C2RustUnnamed_21 = 37;
pub const PLAYER_AP_BOTTLE_LETTER: C2RustUnnamed_21 = 36;
pub const PLAYER_AP_BOTTLE_BIG_POE: C2RustUnnamed_21 = 35;
pub const PLAYER_AP_BOTTLE_POE: C2RustUnnamed_21 = 34;
pub const PLAYER_AP_BOTTLE_BUG: C2RustUnnamed_21 = 33;
pub const PLAYER_AP_BOTTLE_FIRE: C2RustUnnamed_21 = 32;
pub const PLAYER_AP_BOTTLE_FISH: C2RustUnnamed_21 = 31;
pub const PLAYER_AP_BOTTLE: C2RustUnnamed_21 = 30;
pub const PLAYER_AP_OCARINA_TIME: C2RustUnnamed_21 = 29;
pub const PLAYER_AP_OCARINA_FAIRY: C2RustUnnamed_21 = 28;
pub const PLAYER_AP_NUT: C2RustUnnamed_21 = 27;
pub const PLAYER_AP_DINS_FIRE: C2RustUnnamed_21 = 26;
pub const PLAYER_AP_NAYRUS_LOVE: C2RustUnnamed_21 = 25;
pub const PLAYER_AP_FARORES_WIND: C2RustUnnamed_21 = 24;
pub const PLAYER_AP_MAGIC_SPELL_17: C2RustUnnamed_21 = 23;
pub const PLAYER_AP_MAGIC_SPELL_16: C2RustUnnamed_21 = 22;
pub const PLAYER_AP_MAGIC_SPELL_15: C2RustUnnamed_21 = 21;
pub const PLAYER_AP_BOOMERANG: C2RustUnnamed_21 = 20;
pub const PLAYER_AP_BOMBCHU: C2RustUnnamed_21 = 19;
pub const PLAYER_AP_BOMB: C2RustUnnamed_21 = 18;
pub const PLAYER_AP_LONGSHOT: C2RustUnnamed_21 = 17;
pub const PLAYER_AP_HOOKSHOT: C2RustUnnamed_21 = 16;
pub const PLAYER_AP_SLINGSHOT: C2RustUnnamed_21 = 15;
pub const PLAYER_AP_BOW_0E: C2RustUnnamed_21 = 14;
pub const PLAYER_AP_BOW_0D: C2RustUnnamed_21 = 13;
pub const PLAYER_AP_BOW_0C: C2RustUnnamed_21 = 12;
pub const PLAYER_AP_BOW_LIGHT: C2RustUnnamed_21 = 11;
pub const PLAYER_AP_BOW_ICE: C2RustUnnamed_21 = 10;
pub const PLAYER_AP_BOW_FIRE: C2RustUnnamed_21 = 9;
pub const PLAYER_AP_BOW: C2RustUnnamed_21 = 8;
pub const PLAYER_AP_HAMMER: C2RustUnnamed_21 = 7;
pub const PLAYER_AP_STICK: C2RustUnnamed_21 = 6;
pub const PLAYER_AP_SWORD_BGS: C2RustUnnamed_21 = 5;
pub const PLAYER_AP_SWORD_KOKIRI: C2RustUnnamed_21 = 4;
pub const PLAYER_AP_SWORD_MASTER: C2RustUnnamed_21 = 3;
pub const PLAYER_AP_FISHING_POLE: C2RustUnnamed_21 = 2;
pub const PLAYER_AP_LAST_USED: C2RustUnnamed_21 = 1;
pub const PLAYER_AP_NONE: C2RustUnnamed_21 = 0;
pub type C2RustUnnamed_22 = libc::c_uint;
pub const OBJECT_ID_MAX: C2RustUnnamed_22 = 402;
pub const OBJECT_ZL4: C2RustUnnamed_22 = 401;
pub const OBJECT_TIMEBLOCK: C2RustUnnamed_22 = 400;
pub const OBJECT_OUKE_HAKA: C2RustUnnamed_22 = 399;
pub const OBJECT_DOOR_KILLER: C2RustUnnamed_22 = 398;
pub const OBJECT_GI_SWORD_1: C2RustUnnamed_22 = 397;
pub const OBJECT_COB: C2RustUnnamed_22 = 396;
pub const OBJECT_COW: C2RustUnnamed_22 = 395;
pub const OBJECT_BWALL: C2RustUnnamed_22 = 394;
pub const OBJECT_PS: C2RustUnnamed_22 = 393;
pub const OBJECT_GS: C2RustUnnamed_22 = 392;
pub const OBJECT_HAKA_DOOR: C2RustUnnamed_22 = 391;
pub const OBJECT_GEFF: C2RustUnnamed_22 = 390;
pub const OBJECT_GJ: C2RustUnnamed_22 = 389;
pub const OBJECT_SKB: C2RustUnnamed_22 = 388;
pub const OBJECT_WF: C2RustUnnamed_22 = 387;
pub const OBJECT_MU: C2RustUnnamed_22 = 386;
pub const OBJECT_SPOT01_MATOYAB: C2RustUnnamed_22 = 385;
pub const OBJECT_SPOT01_MATOYA: C2RustUnnamed_22 = 384;
pub const OBJECT_GI_RUPY: C2RustUnnamed_22 = 383;
pub const OBJECT_GANON_ANIME3: C2RustUnnamed_22 = 382;
pub const OBJECT_GANON_ANIME2: C2RustUnnamed_22 = 381;
pub const OBJECT_GANON_ANIME1: C2RustUnnamed_22 = 380;
pub const OBJECT_GI_DEKUPOUCH: C2RustUnnamed_22 = 379;
pub const OBJECT_EFC_DOUGHNUT: C2RustUnnamed_22 = 378;
pub const OBJECT_DEMO_KEKKAI: C2RustUnnamed_22 = 377;
pub const OBJECT_BOWL: C2RustUnnamed_22 = 376;
pub const OBJECT_GI_SOUL: C2RustUnnamed_22 = 375;
pub const OBJECT_GI_GHOST: C2RustUnnamed_22 = 374;
pub const OBJECT_GI_BUTTERFLY: C2RustUnnamed_22 = 373;
pub const OBJECT_GI_INSECT: C2RustUnnamed_22 = 372;
pub const OBJECT_GI_FIRE: C2RustUnnamed_22 = 371;
pub const OBJECT_DNK: C2RustUnnamed_22 = 370;
pub const OBJECT_DNS: C2RustUnnamed_22 = 369;
pub const OBJECT_KIBAKO2: C2RustUnnamed_22 = 368;
pub const OBJECT_SPOT11_OBJ: C2RustUnnamed_22 = 367;
pub const OBJECT_UNSET_16E: C2RustUnnamed_22 = 366;
pub const OBJECT_JYA_DOOR: C2RustUnnamed_22 = 365;
pub const OBJECT_JYA_IRON: C2RustUnnamed_22 = 364;
pub const OBJECT_DOG: C2RustUnnamed_22 = 363;
pub const OBJECT_GR: C2RustUnnamed_22 = 362;
pub const OBJECT_GELDB: C2RustUnnamed_22 = 361;
pub const OBJECT_SHOPNUTS: C2RustUnnamed_22 = 360;
pub const OBJECT_GLA: C2RustUnnamed_22 = 359;
pub const OBJECT_SPOT00_BREAK: C2RustUnnamed_22 = 358;
pub const OBJECT_RS: C2RustUnnamed_22 = 357;
pub const OBJECT_HINTNUTS: C2RustUnnamed_22 = 356;
pub const OBJECT_BOMBIWA: C2RustUnnamed_22 = 355;
pub const OBJECT_SPOT12_OBJ: C2RustUnnamed_22 = 354;
pub const OBJECT_SPOT05_OBJECTS: C2RustUnnamed_22 = 353;
pub const OBJECT_BG: C2RustUnnamed_22 = 352;
pub const OBJECT_BIGOKUTA: C2RustUnnamed_22 = 351;
pub const OBJECT_SSH: C2RustUnnamed_22 = 350;
pub const OBJECT_GI_GODDESS: C2RustUnnamed_22 = 349;
pub const OBJECT_GI_SUTARU: C2RustUnnamed_22 = 348;
pub const OBJECT_FISH: C2RustUnnamed_22 = 347;
pub const OBJECT_EC: C2RustUnnamed_22 = 346;
pub const OBJECT_DS2: C2RustUnnamed_22 = 345;
pub const OBJECT_GI_M_ARROW: C2RustUnnamed_22 = 344;
pub const OBJECT_GI_HOVERBOOTS: C2RustUnnamed_22 = 343;
pub const OBJECT_ZG: C2RustUnnamed_22 = 342;
pub const OBJECT_TS: C2RustUnnamed_22 = 341;
pub const OBJECT_KA: C2RustUnnamed_22 = 340;
pub const OBJECT_GANON2: C2RustUnnamed_22 = 339;
pub const OBJECT_GI_GERUDOMASK: C2RustUnnamed_22 = 338;
pub const OBJECT_GI_ZORAMASK: C2RustUnnamed_22 = 337;
pub const OBJECT_GI_GOLONMASK: C2RustUnnamed_22 = 336;
pub const OBJECT_ZL2_ANIME2: C2RustUnnamed_22 = 335;
pub const OBJECT_ZL2_ANIME1: C2RustUnnamed_22 = 334;
pub const OBJECT_EFC_ERUPC: C2RustUnnamed_22 = 333;
pub const OBJECT_GT: C2RustUnnamed_22 = 332;
pub const OBJECT_DOOR_GERUDO: C2RustUnnamed_22 = 331;
pub const OBJECT_MAG: C2RustUnnamed_22 = 330;
pub const OBJECT_GI_FROG: C2RustUnnamed_22 = 329;
pub const OBJECT_GI_SOLDOUT: C2RustUnnamed_22 = 328;
pub const OBJECT_GI_BRACELET: C2RustUnnamed_22 = 327;
pub const OBJECT_GI_PRESCRIPTION: C2RustUnnamed_22 = 326;
pub const OBJECT_CS: C2RustUnnamed_22 = 325;
pub const OBJECT_JS: C2RustUnnamed_22 = 324;
pub const OBJECT_GI_BROKENSWORD: C2RustUnnamed_22 = 323;
pub const OBJECT_GI_TICKETSTONE: C2RustUnnamed_22 = 322;
pub const OBJECT_GI_MUSHROOM: C2RustUnnamed_22 = 321;
pub const OBJECT_GI_POWDER: C2RustUnnamed_22 = 320;
pub const OBJECT_GI_EYE_LOTION: C2RustUnnamed_22 = 319;
pub const OBJECT_OS: C2RustUnnamed_22 = 318;
pub const OBJECT_FA: C2RustUnnamed_22 = 317;
pub const OBJECT_MM: C2RustUnnamed_22 = 316;
pub const OBJECT_STREAM: C2RustUnnamed_22 = 315;
pub const OBJECT_SIOFUKI: C2RustUnnamed_22 = 314;
pub const OBJECT_GANON_OBJECTS: C2RustUnnamed_22 = 313;
pub const OBJECT_GI_TRUTH_MASK: C2RustUnnamed_22 = 312;
pub const OBJECT_GI_RABIT_MASK: C2RustUnnamed_22 = 311;
pub const OBJECT_GI_SKJ_MASK: C2RustUnnamed_22 = 310;
pub const OBJECT_GI_REDEAD_MASK: C2RustUnnamed_22 = 309;
pub const OBJECT_GI_KI_TAN_MASK: C2RustUnnamed_22 = 308;
pub const OBJECT_FU: C2RustUnnamed_22 = 307;
pub const OBJECT_MK: C2RustUnnamed_22 = 306;
pub const OBJECT_OWL: C2RustUnnamed_22 = 305;
pub const OBJECT_GJYO_OBJECTS: C2RustUnnamed_22 = 304;
pub const OBJECT_KANBAN: C2RustUnnamed_22 = 303;
pub const OBJECT_GI_COIN: C2RustUnnamed_22 = 302;
pub const OBJECT_GI_GLOVES: C2RustUnnamed_22 = 301;
pub const OBJECT_TSUBO: C2RustUnnamed_22 = 300;
pub const OBJECT_KUSA: C2RustUnnamed_22 = 299;
pub const OBJECT_LIGHTSWITCH: C2RustUnnamed_22 = 298;
pub const OBJECT_INGATE: C2RustUnnamed_22 = 297;
pub const OBJECT_HS: C2RustUnnamed_22 = 296;
pub const OBJECT_MS: C2RustUnnamed_22 = 295;
pub const OBJECT_GM: C2RustUnnamed_22 = 294;
pub const OBJECT_BLKOBJ: C2RustUnnamed_22 = 293;
pub const OBJECT_NWC: C2RustUnnamed_22 = 292;
pub const OBJECT_UNSET_123: C2RustUnnamed_22 = 291;
pub const OBJECT_DAIKU: C2RustUnnamed_22 = 290;
pub const OBJECT_TORYO: C2RustUnnamed_22 = 289;
pub const OBJECT_UNSET_120: C2RustUnnamed_22 = 288;
pub const OBJECT_GOROIWA: C2RustUnnamed_22 = 287;
pub const OBJECT_MAMENOKI: C2RustUnnamed_22 = 286;
pub const OBJECT_D_LIFT: C2RustUnnamed_22 = 285;
pub const OBJECT_D_HSBLOCK: C2RustUnnamed_22 = 284;
pub const OBJECT_D_ELEVATOR: C2RustUnnamed_22 = 283;
pub const OBJECT_GND_MAGIC: C2RustUnnamed_22 = 282;
pub const OBJECT_GI_SEED: C2RustUnnamed_22 = 281;
pub const OBJECT_GI_BOOTS_2: C2RustUnnamed_22 = 280;
pub const OBJECT_YABUSAME_POINT: C2RustUnnamed_22 = 279;
pub const OBJECT_GE1: C2RustUnnamed_22 = 278;
pub const OBJECT_BOB: C2RustUnnamed_22 = 277;
pub const OBJECT_FZ: C2RustUnnamed_22 = 276;
pub const OBJECT_SPOT07_OBJECT: C2RustUnnamed_22 = 275;
pub const OBJECT_SPOT03_OBJECT: C2RustUnnamed_22 = 274;
pub const OBJECT_BOJ: C2RustUnnamed_22 = 273;
pub const OBJECT_ANE: C2RustUnnamed_22 = 272;
pub const OBJECT_DS: C2RustUnnamed_22 = 271;
pub const OBJECT_GI_OCARINA_0: C2RustUnnamed_22 = 270;
pub const OBJECT_BBA: C2RustUnnamed_22 = 269;
pub const OBJECT_BJI: C2RustUnnamed_22 = 268;
pub const OBJECT_GI_BOTTLE_LETTER: C2RustUnnamed_22 = 267;
pub const OBJECT_SKJ: C2RustUnnamed_22 = 266;
pub const OBJECT_GI_NIWATORI: C2RustUnnamed_22 = 265;
pub const OBJECT_CNE: C2RustUnnamed_22 = 264;
pub const OBJECT_AHG: C2RustUnnamed_22 = 263;
pub const OBJECT_IK: C2RustUnnamed_22 = 262;
pub const OBJECT_AOB: C2RustUnnamed_22 = 261;
pub const OBJECT_MASTERZOORA: C2RustUnnamed_22 = 260;
pub const OBJECT_MASTERGOLON: C2RustUnnamed_22 = 259;
pub const OBJECT_MASTERKOKIRIHEAD: C2RustUnnamed_22 = 258;
pub const OBJECT_MASTERKOKIRI: C2RustUnnamed_22 = 257;
pub const OBJECT_UMAJUMP: C2RustUnnamed_22 = 256;
pub const OBJECT_KZ: C2RustUnnamed_22 = 255;
pub const OBJECT_ZO: C2RustUnnamed_22 = 254;
pub const OBJECT_KW1: C2RustUnnamed_22 = 253;
pub const OBJECT_KM1: C2RustUnnamed_22 = 252;
pub const OBJECT_MD: C2RustUnnamed_22 = 251;
pub const OBJECT_MD_UNUSED: C2RustUnnamed_22 = 250;
pub const OBJECT_SPOT01_OBJECTS: C2RustUnnamed_22 = 249;
pub const OBJECT_GI_LONGSWORD: C2RustUnnamed_22 = 248;
pub const OBJECT_GI_GRASS: C2RustUnnamed_22 = 247;
pub const OBJECT_GI_HAMMER: C2RustUnnamed_22 = 246;
pub const OBJECT_GI_SAW: C2RustUnnamed_22 = 245;
pub const OBJECT_GI_FISH: C2RustUnnamed_22 = 244;
pub const OBJECT_GI_BEAN: C2RustUnnamed_22 = 243;
pub const OBJECT_GI_CLOTHES: C2RustUnnamed_22 = 242;
pub const OBJECT_JYA_OBJ: C2RustUnnamed_22 = 241;
pub const OBJECT_SPOT15_OBJ: C2RustUnnamed_22 = 240;
pub const OBJECT_GI_LETTER: C2RustUnnamed_22 = 239;
pub const OBJECT_GI_SHIELD_3: C2RustUnnamed_22 = 238;
pub const OBJECT_DEMO_6K: C2RustUnnamed_22 = 237;
pub const OBJECT_ANI: C2RustUnnamed_22 = 236;
pub const OBJECT_GI_LIQUID: C2RustUnnamed_22 = 235;
pub const OBJECT_GI_GLASSES: C2RustUnnamed_22 = 234;
pub const OBJECT_GI_BOW: C2RustUnnamed_22 = 233;
pub const OBJECT_GI_BOOMERANG: C2RustUnnamed_22 = 232;
pub const OBJECT_GI_PACHINKO: C2RustUnnamed_22 = 231;
pub const OBJECT_FR: C2RustUnnamed_22 = 230;
pub const OBJECT_NY: C2RustUnnamed_22 = 229;
pub const OBJECT_UNSET_E4: C2RustUnnamed_22 = 228;
pub const OBJECT_NY_UNUSED: C2RustUnnamed_22 = 227;
pub const OBJECT_SST: C2RustUnnamed_22 = 226;
pub const OBJECT_GANON: C2RustUnnamed_22 = 225;
pub const OBJECT_MA1: C2RustUnnamed_22 = 224;
pub const OBJECT_GI_MILK: C2RustUnnamed_22 = 223;
pub const OBJECT_GI_OCARINA: C2RustUnnamed_22 = 222;
pub const OBJECT_GI_HOOKSHOT: C2RustUnnamed_22 = 221;
pub const OBJECT_GI_SHIELD_2: C2RustUnnamed_22 = 220;
pub const OBJECT_GI_SCALE: C2RustUnnamed_22 = 219;
pub const OBJECT_GI_EGG: C2RustUnnamed_22 = 218;
pub const OBJECT_GI_BOMB_2: C2RustUnnamed_22 = 217;
pub const OBJECT_GI_ARROW: C2RustUnnamed_22 = 216;
pub const OBJECT_GI_GERUDO: C2RustUnnamed_22 = 215;
pub const OBJECT_ANUBICE: C2RustUnnamed_22 = 214;
pub const OBJECT_BXA: C2RustUnnamed_22 = 213;
pub const OBJECT_RR: C2RustUnnamed_22 = 212;
pub const OBJECT_TW: C2RustUnnamed_22 = 211;
pub const OBJECT_HNI: C2RustUnnamed_22 = 210;
pub const OBJECT_GI_PURSE: C2RustUnnamed_22 = 209;
pub const OBJECT_MA2: C2RustUnnamed_22 = 208;
pub const OBJECT_OF1S: C2RustUnnamed_22 = 207;
pub const OBJECT_GI_BOMB_1: C2RustUnnamed_22 = 206;
pub const OBJECT_GI_MAGICPOT: C2RustUnnamed_22 = 205;
pub const OBJECT_DEKUJR: C2RustUnnamed_22 = 204;
pub const OBJECT_GI_SHIELD_1: C2RustUnnamed_22 = 203;
pub const OBJECT_RU2: C2RustUnnamed_22 = 202;
pub const OBJECT_OF1D_MAP: C2RustUnnamed_22 = 201;
pub const OBJECT_GI_MAP: C2RustUnnamed_22 = 200;
pub const OBJECT_GI_STICK: C2RustUnnamed_22 = 199;
pub const OBJECT_GI_BOTTLE: C2RustUnnamed_22 = 198;
pub const OBJECT_OS_ANIME: C2RustUnnamed_22 = 197;
pub const OBJECT_OE4S: C2RustUnnamed_22 = 196;
pub const OBJECT_OE1S: C2RustUnnamed_22 = 195;
pub const OBJECT_SPOT16_OBJ: C2RustUnnamed_22 = 194;
pub const OBJECT_TR: C2RustUnnamed_22 = 193;
pub const OBJECT_IN: C2RustUnnamed_22 = 192;
pub const OBJECT_GI_BOMBPOUCH: C2RustUnnamed_22 = 191;
pub const OBJECT_GI_ARROWCASE: C2RustUnnamed_22 = 190;
pub const OBJECT_GI_HEARTS: C2RustUnnamed_22 = 189;
pub const OBJECT_SA: C2RustUnnamed_22 = 188;
pub const OBJECT_GI_NUTS: C2RustUnnamed_22 = 187;
pub const OBJECT_GI_MEDAL: C2RustUnnamed_22 = 186;
pub const OBJECT_GI_BOSSKEY: C2RustUnnamed_22 = 185;
pub const OBJECT_GI_COMPASS: C2RustUnnamed_22 = 184;
pub const OBJECT_GI_HEART: C2RustUnnamed_22 = 183;
pub const OBJECT_GI_MELODY: C2RustUnnamed_22 = 182;
pub const OBJECT_SB: C2RustUnnamed_22 = 181;
pub const OBJECT_MO: C2RustUnnamed_22 = 180;
pub const OBJECT_NB: C2RustUnnamed_22 = 179;
pub const OBJECT_SHOP_DUNGEN: C2RustUnnamed_22 = 178;
pub const OBJECT_SPOT17_OBJ: C2RustUnnamed_22 = 177;
pub const OBJECT_BDOOR: C2RustUnnamed_22 = 176;
pub const OBJECT_SPOT18_OBJ: C2RustUnnamed_22 = 175;
pub const OBJECT_SPOT09_OBJ: C2RustUnnamed_22 = 174;
pub const OBJECT_GI_JEWEL: C2RustUnnamed_22 = 173;
pub const OBJECT_BROB: C2RustUnnamed_22 = 172;
pub const OBJECT_MIR_RAY: C2RustUnnamed_22 = 171;
pub const OBJECT_GI_KEY: C2RustUnnamed_22 = 170;
pub const OBJECT_DEMO_TRE_LGT: C2RustUnnamed_22 = 169;
pub const OBJECT_EFC_TW: C2RustUnnamed_22 = 168;
pub const OBJECT_RL: C2RustUnnamed_22 = 167;
pub const OBJECT_DH: C2RustUnnamed_22 = 166;
pub const OBJECT_FD2: C2RustUnnamed_22 = 165;
pub const OBJECT_SYOKUDAI: C2RustUnnamed_22 = 164;
pub const OBJECT_RU1: C2RustUnnamed_22 = 163;
pub const OBJECT_HAKA: C2RustUnnamed_22 = 162;
pub const OBJECT_SPOT02_OBJECTS: C2RustUnnamed_22 = 161;
pub const OBJECT_HORSE_LINK_CHILD: C2RustUnnamed_22 = 160;
pub const OBJECT_MEDAL: C2RustUnnamed_22 = 159;
pub const OBJECT_FW: C2RustUnnamed_22 = 158;
pub const OBJECT_DU: C2RustUnnamed_22 = 157;
pub const OBJECT_FD: C2RustUnnamed_22 = 156;
pub const OBJECT_GNDD: C2RustUnnamed_22 = 155;
pub const OBJECT_HEAVY_OBJECT: C2RustUnnamed_22 = 154;
pub const OBJECT_PO_SISTERS: C2RustUnnamed_22 = 153;
pub const OBJECT_RD: C2RustUnnamed_22 = 152;
pub const OBJECT_SD: C2RustUnnamed_22 = 151;
pub const OBJECT_BDAN_OBJECTS: C2RustUnnamed_22 = 150;
pub const OBJECT_TRIFORCE_SPOT: C2RustUnnamed_22 = 149;
pub const OBJECT_LIGHT_RING: C2RustUnnamed_22 = 148;
pub const OBJECT_GOD_LGT: C2RustUnnamed_22 = 147;
pub const OBJECT_EFC_STAR_FIELD: C2RustUnnamed_22 = 146;
pub const OBJECT_EFC_LGT_SHOWER: C2RustUnnamed_22 = 145;
pub const OBJECT_EFC_FLASH: C2RustUnnamed_22 = 144;
pub const OBJECT_EFC_FIRE_BALL: C2RustUnnamed_22 = 143;
pub const OBJECT_EFC_CRYSTAL_LIGHT: C2RustUnnamed_22 = 142;
pub const OBJECT_HAKACH_OBJECTS: C2RustUnnamed_22 = 141;
pub const OBJECT_BV: C2RustUnnamed_22 = 140;
pub const OBJECT_VM: C2RustUnnamed_22 = 139;
pub const OBJECT_XC: C2RustUnnamed_22 = 138;
pub const OBJECT_TK: C2RustUnnamed_22 = 137;
pub const OBJECT_TA: C2RustUnnamed_22 = 136;
pub const OBJECT_IM: C2RustUnnamed_22 = 135;
pub const OBJECT_VASE: C2RustUnnamed_22 = 134;
pub const OBJECT_TRAP: C2RustUnnamed_22 = 133;
pub const OBJECT_UNSET_84: C2RustUnnamed_22 = 132;
pub const OBJECT_UNSET_83: C2RustUnnamed_22 = 131;
pub const OBJECT_PU_BOX: C2RustUnnamed_22 = 130;
pub const OBJECT_LIGHTBOX: C2RustUnnamed_22 = 129;
pub const OBJECT_UNSET_80: C2RustUnnamed_22 = 128;
pub const OBJECT_UNSET_7F: C2RustUnnamed_22 = 127;
pub const OBJECT_UNSET_7E: C2RustUnnamed_22 = 126;
pub const OBJECT_UNSET_7D: C2RustUnnamed_22 = 125;
pub const OBJECT_WOOD02: C2RustUnnamed_22 = 124;
pub const OBJECT_UNSET_7B: C2RustUnnamed_22 = 123;
pub const OBJECT_UNSET_7A: C2RustUnnamed_22 = 122;
pub const OBJECT_UNSET_79: C2RustUnnamed_22 = 121;
pub const OBJECT_UNSET_78: C2RustUnnamed_22 = 120;
pub const OBJECT_BIRD: C2RustUnnamed_22 = 119;
pub const OBJECT_HATA: C2RustUnnamed_22 = 118;
pub const OBJECT_WARP2: C2RustUnnamed_22 = 117;
pub const OBJECT_SPOT08_OBJ: C2RustUnnamed_22 = 116;
pub const OBJECT_MORI_TEX: C2RustUnnamed_22 = 115;
pub const OBJECT_MORI_OBJECTS: C2RustUnnamed_22 = 114;
pub const OBJECT_MORI_HINERI2A: C2RustUnnamed_22 = 113;
pub const OBJECT_MORI_HINERI2: C2RustUnnamed_22 = 112;
pub const OBJECT_MORI_HINERI1A: C2RustUnnamed_22 = 111;
pub const OBJECT_PO_COMPOSER: C2RustUnnamed_22 = 110;
pub const OBJECT_PO_FIELD: C2RustUnnamed_22 = 109;
pub const OBJECT_RELAY_OBJECTS: C2RustUnnamed_22 = 108;
pub const OBJECT_ICE_OBJECTS: C2RustUnnamed_22 = 107;
pub const OBJECT_SPOT06_OBJECTS: C2RustUnnamed_22 = 106;
pub const OBJECT_HAKA_OBJECTS: C2RustUnnamed_22 = 105;
pub const OBJECT_MJIN_OKA: C2RustUnnamed_22 = 104;
pub const OBJECT_MJIN_WIND: C2RustUnnamed_22 = 103;
pub const OBJECT_MJIN_SOUL: C2RustUnnamed_22 = 102;
pub const OBJECT_MJIN_ICE: C2RustUnnamed_22 = 101;
pub const OBJECT_MJIN_FLAME: C2RustUnnamed_22 = 100;
pub const OBJECT_MJIN_DARK: C2RustUnnamed_22 = 99;
pub const OBJECT_MJIN_FLASH: C2RustUnnamed_22 = 98;
pub const OBJECT_MJIN: C2RustUnnamed_22 = 97;
pub const OBJECT_ZL2: C2RustUnnamed_22 = 96;
pub const OBJECT_YUKABYUN: C2RustUnnamed_22 = 95;
pub const OBJECT_TOKI_OBJECTS: C2RustUnnamed_22 = 94;
pub const OBJECT_BB: C2RustUnnamed_22 = 93;
pub const OBJECT_MORI_HINERI1: C2RustUnnamed_22 = 92;
pub const OBJECT_OSSAN: C2RustUnnamed_22 = 91;
pub const OBJECT_FHG: C2RustUnnamed_22 = 90;
pub const OBJECT_MIZU_OBJECTS: C2RustUnnamed_22 = 89;
pub const OBJECT_OA11: C2RustUnnamed_22 = 88;
pub const OBJECT_OA10: C2RustUnnamed_22 = 87;
pub const OBJECT_VALI: C2RustUnnamed_22 = 86;
pub const OBJECT_OE12: C2RustUnnamed_22 = 85;
pub const OBJECT_OE11: C2RustUnnamed_22 = 84;
pub const OBJECT_OE10: C2RustUnnamed_22 = 83;
pub const OBJECT_OE9: C2RustUnnamed_22 = 82;
pub const OBJECT_OE8: C2RustUnnamed_22 = 81;
pub const OBJECT_OE7: C2RustUnnamed_22 = 80;
pub const OBJECT_OE6: C2RustUnnamed_22 = 79;
pub const OBJECT_OE5: C2RustUnnamed_22 = 78;
pub const OBJECT_MENKURI_OBJECTS: C2RustUnnamed_22 = 77;
pub const OBJECT_OE4: C2RustUnnamed_22 = 76;
pub const OBJECT_OE3: C2RustUnnamed_22 = 75;
pub const OBJECT_DEKUNUTS: C2RustUnnamed_22 = 74;
pub const OBJECT_B_HEART: C2RustUnnamed_22 = 73;
pub const OBJECT_WARP1: C2RustUnnamed_22 = 72;
pub const OBJECT_OPENING_DEMO1: C2RustUnnamed_22 = 71;
pub const OBJECT_HORSE_ZELDA: C2RustUnnamed_22 = 70;
pub const OBJECT_OB4: C2RustUnnamed_22 = 69;
pub const OBJECT_OB3: C2RustUnnamed_22 = 68;
pub const OBJECT_OB2: C2RustUnnamed_22 = 67;
pub const OBJECT_OA9: C2RustUnnamed_22 = 66;
pub const OBJECT_OA8: C2RustUnnamed_22 = 65;
pub const OBJECT_JJ: C2RustUnnamed_22 = 64;
pub const OBJECT_OA7: C2RustUnnamed_22 = 63;
pub const OBJECT_OA6: C2RustUnnamed_22 = 62;
pub const OBJECT_OA5: C2RustUnnamed_22 = 61;
pub const OBJECT_OA4: C2RustUnnamed_22 = 60;
pub const OBJECT_OA3: C2RustUnnamed_22 = 59;
pub const OBJECT_UNSET_3A: C2RustUnnamed_22 = 58;
pub const OBJECT_DEKUBABA: C2RustUnnamed_22 = 57;
pub const OBJECT_AM: C2RustUnnamed_22 = 56;
pub const OBJECT_GND: C2RustUnnamed_22 = 55;
pub const OBJECT_YDAN_OBJECTS: C2RustUnnamed_22 = 54;
pub const OBJECT_OE2: C2RustUnnamed_22 = 53;
pub const OBJECT_OE_ANIME: C2RustUnnamed_22 = 52;
pub const OBJECT_OE1: C2RustUnnamed_22 = 51;
pub const OBJECT_SK2: C2RustUnnamed_22 = 50;
pub const OBJECT_BOMBF: C2RustUnnamed_22 = 49;
pub const OBJECT_MB: C2RustUnnamed_22 = 48;
pub const OBJECT_SPOT00_OBJECTS: C2RustUnnamed_22 = 47;
pub const OBJECT_OA2: C2RustUnnamed_22 = 46;
pub const OBJECT_HORSE_GANON: C2RustUnnamed_22 = 45;
pub const OBJECT_HIDAN_OBJECTS: C2RustUnnamed_22 = 44;
pub const OBJECT_DDAN_OBJECTS: C2RustUnnamed_22 = 43;
pub const OBJECT_SPOT04_OBJECTS: C2RustUnnamed_22 = 42;
pub const OBJECT_O_ANIME: C2RustUnnamed_22 = 41;
pub const OBJECT_OB1: C2RustUnnamed_22 = 40;
pub const OBJECT_HORSE_NORMAL: C2RustUnnamed_22 = 39;
pub const OBJECT_EI: C2RustUnnamed_22 = 38;
pub const OBJECT_BW: C2RustUnnamed_22 = 37;
pub const OBJECT_ST: C2RustUnnamed_22 = 36;
pub const OBJECT_OA1: C2RustUnnamed_22 = 35;
pub const OBJECT_TP: C2RustUnnamed_22 = 34;
pub const OBJECT_BL: C2RustUnnamed_22 = 33;
pub const OBJECT_TORCH2: C2RustUnnamed_22 = 32;
pub const OBJECT_DODOJR: C2RustUnnamed_22 = 31;
pub const OBJECT_GOL: C2RustUnnamed_22 = 30;
pub const OBJECT_ZL1: C2RustUnnamed_22 = 29;
pub const OBJECT_GOMA: C2RustUnnamed_22 = 28;
pub const OBJECT_ZF: C2RustUnnamed_22 = 27;
pub const OBJECT_HORSE: C2RustUnnamed_22 = 26;
pub const OBJECT_KINGDODONGO: C2RustUnnamed_22 = 25;
pub const OBJECT_PEEHAT: C2RustUnnamed_22 = 24;
pub const OBJECT_REEBA: C2RustUnnamed_22 = 23;
pub const OBJECT_TITE: C2RustUnnamed_22 = 22;
pub const OBJECT_LINK_CHILD: C2RustUnnamed_22 = 21;
pub const OBJECT_LINK_BOY: C2RustUnnamed_22 = 20;
pub const OBJECT_NIW: C2RustUnnamed_22 = 19;
pub const OBJECT_BUBBLE: C2RustUnnamed_22 = 18;
pub const OBJECT_UNSET_11: C2RustUnnamed_22 = 17;
pub const OBJECT_UNSET_10: C2RustUnnamed_22 = 16;
pub const OBJECT_FIRE: C2RustUnnamed_22 = 15;
pub const OBJECT_BOX: C2RustUnnamed_22 = 14;
pub const OBJECT_FIREFLY: C2RustUnnamed_22 = 13;
pub const OBJECT_DODONGO: C2RustUnnamed_22 = 12;
pub const OBJECT_WALLMASTER: C2RustUnnamed_22 = 11;
pub const OBJECT_DY_OBJ: C2RustUnnamed_22 = 10;
pub const OBJECT_POH: C2RustUnnamed_22 = 9;
pub const OBJECT_CROW: C2RustUnnamed_22 = 8;
pub const OBJECT_OKUTA: C2RustUnnamed_22 = 7;
pub const OBJECT_HUMAN: C2RustUnnamed_22 = 6;
pub const OBJECT_UNSET_5: C2RustUnnamed_22 = 5;
pub const OBJECT_UNSET_4: C2RustUnnamed_22 = 4;
pub const OBJECT_GAMEPLAY_DANGEON_KEEP: C2RustUnnamed_22 = 3;
pub const OBJECT_GAMEPLAY_FIELD_KEEP: C2RustUnnamed_22 = 2;
pub const OBJECT_GAMEPLAY_KEEP: C2RustUnnamed_22 = 1;
pub const OBJECT_INVALID: C2RustUnnamed_22 = 0;
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
pub const GI_MAX: C2RustUnnamed_24 = 126;
pub const GI_TEXT_0: C2RustUnnamed_24 = 125;
pub const GI_ICE_TRAP: C2RustUnnamed_24 = 124;
pub const GI_BULLET_BAG_50: C2RustUnnamed_24 = 123;
pub const GI_NUT_UPGRADE_40: C2RustUnnamed_24 = 122;
pub const GI_NUT_UPGRADE_30: C2RustUnnamed_24 = 121;
pub const GI_STICK_UPGRADE_30: C2RustUnnamed_24 = 120;
pub const GI_STICK_UPGRADE_20: C2RustUnnamed_24 = 119;
pub const GI_HEART_PIECE_WIN: C2RustUnnamed_24 = 118;
pub const GI_RUPEE_PURPLE_LOSE: C2RustUnnamed_24 = 117;
pub const GI_RUPEE_RED_LOSE: C2RustUnnamed_24 = 116;
pub const GI_RUPEE_BLUE_LOSE: C2RustUnnamed_24 = 115;
pub const GI_RUPEE_GREEN_LOSE: C2RustUnnamed_24 = 114;
pub const GI_DOOR_KEY: C2RustUnnamed_24 = 113;
pub const GI_BIG_POE: C2RustUnnamed_24 = 112;
pub const GI_POE: C2RustUnnamed_24 = 111;
pub const GI_BLUE_FIRE: C2RustUnnamed_24 = 110;
pub const GI_BUGS: C2RustUnnamed_24 = 109;
pub const GI_FISH: C2RustUnnamed_24 = 108;
pub const GI_BOMBCHUS_20: C2RustUnnamed_24 = 107;
pub const GI_BOMBCHUS_5: C2RustUnnamed_24 = 106;
pub const GI_SEEDS_30: C2RustUnnamed_24 = 105;
pub const GI_BOMBS_30: C2RustUnnamed_24 = 104;
pub const GI_BOMBS_20: C2RustUnnamed_24 = 103;
pub const GI_BOMBS_10: C2RustUnnamed_24 = 102;
pub const GI_BOMBS_1: C2RustUnnamed_24 = 101;
pub const GI_NUTS_10: C2RustUnnamed_24 = 100;
pub const GI_NUTS_5_2: C2RustUnnamed_24 = 99;
pub const GI_STICKS_10: C2RustUnnamed_24 = 98;
pub const GI_STICKS_5: C2RustUnnamed_24 = 97;
pub const GI_BULLET_BAG_40: C2RustUnnamed_24 = 96;
pub const GI_BULLET_BAG_30: C2RustUnnamed_24 = 95;
pub const GI_NAYRUS_LOVE: C2RustUnnamed_24 = 94;
pub const GI_FARORES_WIND: C2RustUnnamed_24 = 93;
pub const GI_DINS_FIRE: C2RustUnnamed_24 = 92;
pub const GI_SKULL_TOKEN: C2RustUnnamed_24 = 91;
pub const GI_ARROW_LIGHT: C2RustUnnamed_24 = 90;
pub const GI_ARROW_ICE: C2RustUnnamed_24 = 89;
pub const GI_ARROW_FIRE: C2RustUnnamed_24 = 88;
pub const GI_SWORD_BGS: C2RustUnnamed_24 = 87;
pub const GI_RUPEE_GOLD: C2RustUnnamed_24 = 86;
pub const GI_RUPEE_PURPLE: C2RustUnnamed_24 = 85;
pub const GI_BRACELET: C2RustUnnamed_24 = 84;
pub const GI_MASK_GERUDO: C2RustUnnamed_24 = 83;
pub const GI_MASK_ZORA: C2RustUnnamed_24 = 82;
pub const GI_MASK_GORON: C2RustUnnamed_24 = 81;
pub const GI_MILK: C2RustUnnamed_24 = 80;
pub const GI_HEART_CONTAINER_2: C2RustUnnamed_24 = 79;
pub const GI_RUPEE_RED: C2RustUnnamed_24 = 78;
pub const GI_RUPEE_BLUE: C2RustUnnamed_24 = 77;
pub const GI_RUPEE_GREEN: C2RustUnnamed_24 = 76;
pub const GI_ARROWS_LARGE: C2RustUnnamed_24 = 75;
pub const GI_ARROWS_MEDIUM: C2RustUnnamed_24 = 74;
pub const GI_ARROWS_SMALL: C2RustUnnamed_24 = 73;
pub const GI_HEART: C2RustUnnamed_24 = 72;
pub const GI_WEIRD_EGG: C2RustUnnamed_24 = 71;
pub const GI_WALLET_GIANT: C2RustUnnamed_24 = 70;
pub const GI_WALLET_ADULT: C2RustUnnamed_24 = 69;
pub const GI_MAGIC_LARGE: C2RustUnnamed_24 = 68;
pub const GI_MAGIC_SMALL: C2RustUnnamed_24 = 67;
pub const GI_KEY_SMALL: C2RustUnnamed_24 = 66;
pub const GI_MAP: C2RustUnnamed_24 = 65;
pub const GI_COMPASS: C2RustUnnamed_24 = 64;
pub const GI_KEY_BOSS: C2RustUnnamed_24 = 63;
pub const GI_HEART_PIECE: C2RustUnnamed_24 = 62;
pub const GI_HEART_CONTAINER: C2RustUnnamed_24 = 61;
pub const GI_SEEDS_5: C2RustUnnamed_24 = 60;
pub const GI_OCARINA_FAIRY: C2RustUnnamed_24 = 59;
pub const GI_GERUDO_CARD: C2RustUnnamed_24 = 58;
pub const GI_STONE_OF_AGONY: C2RustUnnamed_24 = 57;
pub const GI_SCALE_GOLD: C2RustUnnamed_24 = 56;
pub const GI_SCALE_SILVER: C2RustUnnamed_24 = 55;
pub const GI_GAUNTLETS_GOLD: C2RustUnnamed_24 = 54;
pub const GI_GAUNTLETS_SILVER: C2RustUnnamed_24 = 53;
pub const GI_BOMB_BAG_40: C2RustUnnamed_24 = 52;
pub const GI_BOMB_BAG_30: C2RustUnnamed_24 = 51;
pub const GI_BOMB_BAG_20: C2RustUnnamed_24 = 50;
pub const GI_QUIVER_50: C2RustUnnamed_24 = 49;
pub const GI_QUIVER_40: C2RustUnnamed_24 = 48;
pub const GI_BOOTS_HOVER: C2RustUnnamed_24 = 47;
pub const GI_BOOTS_IRON: C2RustUnnamed_24 = 46;
pub const GI_TUNIC_ZORA: C2RustUnnamed_24 = 45;
pub const GI_TUNIC_GORON: C2RustUnnamed_24 = 44;
pub const GI_SHIELD_MIRROR: C2RustUnnamed_24 = 43;
pub const GI_SHIELD_HYLIAN: C2RustUnnamed_24 = 42;
pub const GI_SHIELD_DEKU: C2RustUnnamed_24 = 41;
pub const GI_SWORD_KNIFE: C2RustUnnamed_24 = 40;
pub const GI_SWORD_KOKIRI: C2RustUnnamed_24 = 39;
pub const GI_CLAIM_CHECK: C2RustUnnamed_24 = 38;
pub const GI_EYEDROPS: C2RustUnnamed_24 = 37;
pub const GI_FROG: C2RustUnnamed_24 = 36;
pub const GI_PRESCRIPTION: C2RustUnnamed_24 = 35;
pub const GI_SWORD_BROKEN: C2RustUnnamed_24 = 34;
pub const GI_SAW: C2RustUnnamed_24 = 33;
pub const GI_ODD_POTION: C2RustUnnamed_24 = 32;
pub const GI_ODD_MUSHROOM: C2RustUnnamed_24 = 31;
pub const GI_POCKET_CUCCO: C2RustUnnamed_24 = 30;
pub const GI_POCKET_EGG: C2RustUnnamed_24 = 29;
pub const GI_MASK_TRUTH: C2RustUnnamed_24 = 28;
pub const GI_MASK_BUNNY: C2RustUnnamed_24 = 27;
pub const GI_MASK_KEATON: C2RustUnnamed_24 = 26;
pub const GI_CHICKEN: C2RustUnnamed_24 = 25;
pub const GI_MASK_SPOOKY: C2RustUnnamed_24 = 24;
pub const GI_MASK_SKULL: C2RustUnnamed_24 = 23;
pub const GI_BEAN: C2RustUnnamed_24 = 22;
pub const GI_LETTER_RUTO: C2RustUnnamed_24 = 21;
pub const GI_MILK_BOTTLE: C2RustUnnamed_24 = 20;
pub const GI_FAIRY: C2RustUnnamed_24 = 19;
pub const GI_POTION_BLUE: C2RustUnnamed_24 = 18;
pub const GI_POTION_GREEN: C2RustUnnamed_24 = 17;
pub const GI_POTION_RED: C2RustUnnamed_24 = 16;
pub const GI_BOTTLE: C2RustUnnamed_24 = 15;
pub const GI_COJIRO: C2RustUnnamed_24 = 14;
pub const GI_HAMMER: C2RustUnnamed_24 = 13;
pub const GI_OCARINA_OOT: C2RustUnnamed_24 = 12;
pub const GI_LETTER_ZELDA: C2RustUnnamed_24 = 11;
pub const GI_LENS: C2RustUnnamed_24 = 10;
pub const GI_LONGSHOT: C2RustUnnamed_24 = 9;
pub const GI_HOOKSHOT: C2RustUnnamed_24 = 8;
pub const GI_STICKS_1: C2RustUnnamed_24 = 7;
pub const GI_BOOMERANG: C2RustUnnamed_24 = 6;
pub const GI_SLINGSHOT: C2RustUnnamed_24 = 5;
pub const GI_BOW: C2RustUnnamed_24 = 4;
pub const GI_BOMBCHUS_10: C2RustUnnamed_24 = 3;
pub const GI_NUTS_5: C2RustUnnamed_24 = 2;
pub const GI_BOMBS_5: C2RustUnnamed_24 = 1;
pub const GI_NONE: C2RustUnnamed_24 = 0;
pub type C2RustUnnamed_25 = libc::c_uint;
pub const EXCH_ITEM_MAX: C2RustUnnamed_25 = 30;
pub const EXCH_ITEM_LETTER_RUTO: C2RustUnnamed_25 = 29;
pub const EXCH_ITEM_BIG_POE: C2RustUnnamed_25 = 28;
pub const EXCH_ITEM_POE: C2RustUnnamed_25 = 27;
pub const EXCH_ITEM_BUG: C2RustUnnamed_25 = 26;
pub const EXCH_ITEM_BLUE_FIRE: C2RustUnnamed_25 = 25;
pub const EXCH_ITEM_FISH: C2RustUnnamed_25 = 24;
pub const EXCH_ITEM_MASK_GERUDO: C2RustUnnamed_25 = 23;
pub const EXCH_ITEM_MASK_ZORA: C2RustUnnamed_25 = 22;
pub const EXCH_ITEM_MASK_GORON: C2RustUnnamed_25 = 21;
pub const EXCH_ITEM_MASK_TRUTH: C2RustUnnamed_25 = 20;
pub const EXCH_ITEM_MASK_BUNNY: C2RustUnnamed_25 = 19;
pub const EXCH_ITEM_MASK_KEATON: C2RustUnnamed_25 = 18;
pub const EXCH_ITEM_MASK_SPOOKY: C2RustUnnamed_25 = 17;
pub const EXCH_ITEM_MASK_SKULL: C2RustUnnamed_25 = 16;
pub const EXCH_ITEM_CLAIM_CHECK: C2RustUnnamed_25 = 15;
pub const EXCH_ITEM_EYEDROPS: C2RustUnnamed_25 = 14;
pub const EXCH_ITEM_FROG: C2RustUnnamed_25 = 13;
pub const EXCH_ITEM_PRESCRIPTION: C2RustUnnamed_25 = 12;
pub const EXCH_ITEM_SWORD_BROKEN: C2RustUnnamed_25 = 11;
pub const EXCH_ITEM_SAW: C2RustUnnamed_25 = 10;
pub const EXCH_ITEM_ODD_POTION: C2RustUnnamed_25 = 9;
pub const EXCH_ITEM_ODD_MUSHROOM: C2RustUnnamed_25 = 8;
pub const EXCH_ITEM_COJIRO: C2RustUnnamed_25 = 7;
pub const EXCH_ITEM_POCKET_CUCCO: C2RustUnnamed_25 = 6;
pub const EXCH_ITEM_POCKET_EGG: C2RustUnnamed_25 = 5;
pub const EXCH_ITEM_BEAN: C2RustUnnamed_25 = 4;
pub const EXCH_ITEM_CHICKEN: C2RustUnnamed_25 = 3;
pub const EXCH_ITEM_WEIRD_EGG: C2RustUnnamed_25 = 2;
pub const EXCH_ITEM_LETTER_ZELDA: C2RustUnnamed_25 = 1;
pub const EXCH_ITEM_NONE: C2RustUnnamed_25 = 0;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct InitChainEntry {
    #[bitfield(name = "cont", ty = "u32_0", bits = "0..=0")]
    #[bitfield(name = "type_0", ty = "u32_0", bits = "1..=4")]
    #[bitfield(name = "offset", ty = "u32_0", bits = "5..=15")]
    #[bitfield(name = "value", ty = "s32", bits = "16..=31")]
    pub cont_type_0_offset_value: [u8; 4],
}
pub type C2RustUnnamed_26 = libc::c_uint;
pub const ICHAINTYPE_VEC3S: C2RustUnnamed_26 = 10;
pub const ICHAINTYPE_VEC3F_DIV1000: C2RustUnnamed_26 = 9;
pub const ICHAINTYPE_VEC3F: C2RustUnnamed_26 = 8;
pub const ICHAINTYPE_F32_DIV1000: C2RustUnnamed_26 = 7;
pub const ICHAINTYPE_F32: C2RustUnnamed_26 = 6;
pub const ICHAINTYPE_S32: C2RustUnnamed_26 = 5;
pub const ICHAINTYPE_U32: C2RustUnnamed_26 = 4;
pub const ICHAINTYPE_S16: C2RustUnnamed_26 = 3;
pub const ICHAINTYPE_U16: C2RustUnnamed_26 = 2;
pub const ICHAINTYPE_S8: C2RustUnnamed_26 = 1;
pub const ICHAINTYPE_U8: C2RustUnnamed_26 = 0;
pub type C2RustUnnamed_27 = libc::c_uint;
pub const TEXT_STATE_AWAITING_NEXT: C2RustUnnamed_27 = 10;
pub const TEXT_STATE_9: C2RustUnnamed_27 = 9;
pub const TEXT_STATE_8: C2RustUnnamed_27 = 8;
pub const TEXT_STATE_SONG_DEMO_DONE: C2RustUnnamed_27 = 7;
pub const TEXT_STATE_DONE: C2RustUnnamed_27 = 6;
pub const TEXT_STATE_EVENT: C2RustUnnamed_27 = 5;
pub const TEXT_STATE_CHOICE: C2RustUnnamed_27 = 4;
pub const TEXT_STATE_DONE_FADING: C2RustUnnamed_27 = 3;
pub const TEXT_STATE_CLOSING: C2RustUnnamed_27 = 2;
pub const TEXT_STATE_DONE_HAS_NEXT: C2RustUnnamed_27 = 1;
pub const TEXT_STATE_NONE: C2RustUnnamed_27 = 0;
pub type C2RustUnnamed_28 = libc::c_uint;
pub const DPM_UNK3: C2RustUnnamed_28 = 3;
pub const DPM_ENEMY: C2RustUnnamed_28 = 2;
pub const DPM_PLAYER: C2RustUnnamed_28 = 1;
pub const DPM_UNK: C2RustUnnamed_28 = 0;
pub type C2RustUnnamed_29 = libc::c_uint;
pub const MTXMODE_APPLY: C2RustUnnamed_29 = 1;
pub const MTXMODE_NEW: C2RustUnnamed_29 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EnGb {
    pub dyna: DynaPolyActor,
    pub skelAnime: SkelAnime,
    pub jointTable: [Vec3s; 12],
    pub morphTable: [Vec3s; 12],
    pub actionFunc: EnGbActionFunc,
    pub collider: ColliderCylinder,
    pub bottlesColliders: [ColliderCylinder; 3],
    pub light: *mut LightNode,
    pub lightInfo: LightInfo,
    pub textId: u16_0,
    pub frameTimer: s16,
    pub actionTimer: s16,
    pub lightColor: Color_RGBA8,
    pub cagedSouls: [EnGbCagedSoul; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EnGbCagedSoul {
    pub infoIdx: u8_0,
    pub unk_1: u8_0,
    pub unk_2: u8_0,
    pub unk_3: u8_0,
    pub rotate180: u8_0,
    pub unk_6: s16,
    pub translation: Vec3f,
    pub unk_14: Vec3f,
    pub unk_20: f32_0,
    pub unk_24: f32_0,
    pub unk_28: f32_0,
}
pub type EnGbActionFunc
    =
    Option<unsafe extern "C" fn(_: *mut EnGb, _: *mut GlobalContext) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EnGbCagedSoulInfo {
    pub prim: Color_RGBA8,
    pub env: Color_RGBA8,
    pub texture: *mut libc::c_void,
    pub timerMultiplier: s16,
}
#[no_mangle]
pub static mut En_Gb_InitVars: ActorInit =
    unsafe {
        {
            let mut init =
                ActorInit{id: ACTOR_EN_GB as libc::c_int as s16,
                          category: ACTORCAT_NPC as libc::c_int as u8_0,
                          flags:
                              ((1 as libc::c_int) << 0 as libc::c_int |
                                   (1 as libc::c_int) << 3 as libc::c_int) as
                                  u32_0,
                          objectId: OBJECT_PS as libc::c_int as s16,
                          instanceSize:
                              ::std::mem::size_of::<EnGb>() as libc::c_ulong,
                          init:
                              ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                      *mut Actor,
                                                                                  _:
                                                                                      *mut GlobalContext)
                                                                 -> ()>,
                                                      ActorFunc>(Some(EnGb_Init
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
                                                      ActorFunc>(Some(EnGb_Destroy
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
                                                      ActorFunc>(Some(EnGb_Update
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
                                                      ActorFunc>(Some(EnGb_Draw
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
static mut sCagedSoulInfo: [EnGbCagedSoulInfo; 3] =
    unsafe {
        [{
             let mut init =
                 EnGbCagedSoulInfo{prim:
                                       {
                                           let mut init =
                                               Color_RGBA8{r:
                                                               255 as
                                                                   libc::c_int
                                                                   as u8_0,
                                                           g:
                                                               255 as
                                                                   libc::c_int
                                                                   as u8_0,
                                                           b:
                                                               170 as
                                                                   libc::c_int
                                                                   as u8_0,
                                                           a:
                                                               255 as
                                                                   libc::c_int
                                                                   as u8_0,};
                                           init
                                       },
                                   env:
                                       {
                                           let mut init =
                                               Color_RGBA8{r:
                                                               255 as
                                                                   libc::c_int
                                                                   as u8_0,
                                                           g:
                                                               200 as
                                                                   libc::c_int
                                                                   as u8_0,
                                                           b:
                                                               0 as
                                                                   libc::c_int
                                                                   as u8_0,
                                                           a:
                                                               255 as
                                                                   libc::c_int
                                                                   as u8_0,};
                                           init
                                       },
                                   texture:
                                       gPoeSellerAngrySoulTex.as_ptr() as
                                           *mut _ as *mut libc::c_void,
                                   timerMultiplier:
                                       -(15 as libc::c_int) as s16,};
             init
         },
         {
             let mut init =
                 EnGbCagedSoulInfo{prim:
                                       {
                                           let mut init =
                                               Color_RGBA8{r:
                                                               255 as
                                                                   libc::c_int
                                                                   as u8_0,
                                                           g:
                                                               255 as
                                                                   libc::c_int
                                                                   as u8_0,
                                                           b:
                                                               170 as
                                                                   libc::c_int
                                                                   as u8_0,
                                                           a:
                                                               255 as
                                                                   libc::c_int
                                                                   as u8_0,};
                                           init
                                       },
                                   env:
                                       {
                                           let mut init =
                                               Color_RGBA8{r:
                                                               0 as
                                                                   libc::c_int
                                                                   as u8_0,
                                                           g:
                                                               150 as
                                                                   libc::c_int
                                                                   as u8_0,
                                                           b:
                                                               0 as
                                                                   libc::c_int
                                                                   as u8_0,
                                                           a:
                                                               255 as
                                                                   libc::c_int
                                                                   as u8_0,};
                                           init
                                       },
                                   texture:
                                       gPoeSellerHappySoulTex.as_ptr() as
                                           *mut _ as *mut libc::c_void,
                                   timerMultiplier:
                                       -(12 as libc::c_int) as s16,};
             init
         },
         {
             let mut init =
                 EnGbCagedSoulInfo{prim:
                                       {
                                           let mut init =
                                               Color_RGBA8{r:
                                                               255 as
                                                                   libc::c_int
                                                                   as u8_0,
                                                           g:
                                                               170 as
                                                                   libc::c_int
                                                                   as u8_0,
                                                           b:
                                                               255 as
                                                                   libc::c_int
                                                                   as u8_0,
                                                           a:
                                                               255 as
                                                                   libc::c_int
                                                                   as u8_0,};
                                           init
                                       },
                                   env:
                                       {
                                           let mut init =
                                               Color_RGBA8{r:
                                                               100 as
                                                                   libc::c_int
                                                                   as u8_0,
                                                           g:
                                                               0 as
                                                                   libc::c_int
                                                                   as u8_0,
                                                           b:
                                                               150 as
                                                                   libc::c_int
                                                                   as u8_0,
                                                           a:
                                                               255 as
                                                                   libc::c_int
                                                                   as u8_0,};
                                           init
                                       },
                                   texture:
                                       gPoeSellerSadSoulTex.as_ptr() as *mut _
                                           as *mut libc::c_void,
                                   timerMultiplier:
                                       -(8 as libc::c_int) as s16,};
             init
         }]
    };
static mut sCylinderInit: ColliderCylinderInitType1 =
    {
        let mut init =
            ColliderCylinderInitType1{base:
                                          {
                                              let mut init =
                                                  ColliderInitType1{colType:
                                                                        COLTYPE_NONE
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            u8_0,
                                                                    atFlags:
                                                                        0 as
                                                                            libc::c_int
                                                                            as
                                                                            u8_0,
                                                                    acFlags:
                                                                        0 as
                                                                            libc::c_int
                                                                            as
                                                                            u8_0,
                                                                    ocFlags1:
                                                                        ((1 as
                                                                              libc::c_int)
                                                                             <<
                                                                             0
                                                                                 as
                                                                                 libc::c_int
                                                                             |
                                                                             ((1
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
                                                                                          libc::c_int))
                                                                            as
                                                                            u8_0,
                                                                    shape:
                                                                        COLSHAPE_CYLINDER
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            u8_0,};
                                              init
                                          },
                                      info:
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
                                                  Cylinder16{radius:
                                                                 40 as
                                                                     libc::c_int
                                                                     as s16,
                                                             height:
                                                                 75 as
                                                                     libc::c_int
                                                                     as s16,
                                                             yShift:
                                                                 0 as
                                                                     libc::c_int
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
static mut sBottlesCylindersInit: [ColliderCylinderInitType1; 3] =
    [{
         let mut init =
             ColliderCylinderInitType1{base:
                                           {
                                               let mut init =
                                                   ColliderInitType1{colType:
                                                                         COLTYPE_NONE
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             u8_0,
                                                                     atFlags:
                                                                         0 as
                                                                             libc::c_int
                                                                             as
                                                                             u8_0,
                                                                     acFlags:
                                                                         0 as
                                                                             libc::c_int
                                                                             as
                                                                             u8_0,
                                                                     ocFlags1:
                                                                         ((1
                                                                               as
                                                                               libc::c_int)
                                                                              <<
                                                                              0
                                                                                  as
                                                                                  libc::c_int
                                                                              |
                                                                              ((1
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
                                                                                           libc::c_int))
                                                                             as
                                                                             u8_0,
                                                                     shape:
                                                                         COLSHAPE_CYLINDER
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             u8_0,};
                                               init
                                           },
                                       info:
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
                                                   Cylinder16{radius:
                                                                  4 as
                                                                      libc::c_int
                                                                      as s16,
                                                              height:
                                                                  20 as
                                                                      libc::c_int
                                                                      as s16,
                                                              yShift:
                                                                  0 as
                                                                      libc::c_int
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
     },
     {
         let mut init =
             ColliderCylinderInitType1{base:
                                           {
                                               let mut init =
                                                   ColliderInitType1{colType:
                                                                         COLTYPE_NONE
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             u8_0,
                                                                     atFlags:
                                                                         0 as
                                                                             libc::c_int
                                                                             as
                                                                             u8_0,
                                                                     acFlags:
                                                                         0 as
                                                                             libc::c_int
                                                                             as
                                                                             u8_0,
                                                                     ocFlags1:
                                                                         ((1
                                                                               as
                                                                               libc::c_int)
                                                                              <<
                                                                              0
                                                                                  as
                                                                                  libc::c_int
                                                                              |
                                                                              ((1
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
                                                                                           libc::c_int))
                                                                             as
                                                                             u8_0,
                                                                     shape:
                                                                         COLSHAPE_CYLINDER
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             u8_0,};
                                               init
                                           },
                                       info:
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
                                                   Cylinder16{radius:
                                                                  4 as
                                                                      libc::c_int
                                                                      as s16,
                                                              height:
                                                                  20 as
                                                                      libc::c_int
                                                                      as s16,
                                                              yShift:
                                                                  0 as
                                                                      libc::c_int
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
     },
     {
         let mut init =
             ColliderCylinderInitType1{base:
                                           {
                                               let mut init =
                                                   ColliderInitType1{colType:
                                                                         COLTYPE_NONE
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             u8_0,
                                                                     atFlags:
                                                                         0 as
                                                                             libc::c_int
                                                                             as
                                                                             u8_0,
                                                                     acFlags:
                                                                         0 as
                                                                             libc::c_int
                                                                             as
                                                                             u8_0,
                                                                     ocFlags1:
                                                                         ((1
                                                                               as
                                                                               libc::c_int)
                                                                              <<
                                                                              0
                                                                                  as
                                                                                  libc::c_int
                                                                              |
                                                                              ((1
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
                                                                                           libc::c_int))
                                                                             as
                                                                             u8_0,
                                                                     shape:
                                                                         COLSHAPE_CYLINDER
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             u8_0,};
                                               init
                                           },
                                       info:
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
                                                   Cylinder16{radius:
                                                                  10 as
                                                                      libc::c_int
                                                                      as s16,
                                                              height:
                                                                  20 as
                                                                      libc::c_int
                                                                      as s16,
                                                              yShift:
                                                                  0 as
                                                                      libc::c_int
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
     }];
// Initialized in run_static_initializers
static mut sInitChain: [InitChainEntry; 2] =
    [InitChainEntry{cont_type_0_offset_value: [0; 4],}; 2];
// relative positions of poe souls
static mut sCagedSoulPositions: [Vec3f; 4] =
    [{ let mut init = Vec3f{x: -8.0f32, y: 112.0f32, z: -8.0f32,}; init },
     { let mut init = Vec3f{x: -3.0f32, y: 112.0f32, z: 29.0f32,}; init },
     { let mut init = Vec3f{x: 31.0f32, y: 112.0f32, z: 29.0f32,}; init },
     { let mut init = Vec3f{x: 31.0f32, y: 112.0f32, z: -8.0f32,}; init }];
// relative positions of bottles
static mut sBottlesPositions: [Vec3f; 3] =
    [{
         let mut init =
             Vec3f{x: -48.0f32,
                   y: 0.0f32,
                   z: 34.0f32,}; // these ifs cannot just contain a constant
         init
     }, { let mut init = Vec3f{x: -55.0f32, y: 0.0f32, z: 49.0f32,}; init },
     { let mut init = Vec3f{x: -48.0f32, y: 0.0f32, z: 60.0f32,}; init }];
#[no_mangle]
pub unsafe extern "C" fn func_80A2F180(mut this: *mut EnGb) {
    if gSaveContext.infTable[0xb as libc::c_int as usize] as libc::c_int &
           0x40 as libc::c_int != 0 {
        (*this).textId = 0x70f5 as libc::c_int as u16_0
    } else { (*this).textId = 0x70f4 as libc::c_int as u16_0 };
}
#[no_mangle]
pub unsafe extern "C" fn EnGb_Init(mut thisx: *mut Actor,
                                   mut globalCtx: *mut GlobalContext) {
    let mut this: *mut EnGb = thisx as *mut EnGb;
    let mut pad: s32 = 0;
    let mut colHeader: *mut CollisionHeader = 0 as *mut CollisionHeader;
    let mut i: s32 = 0;
    let mut rand: f32_0 = 0.;
    let mut focusOffset: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    Actor_ProcessInitChain(&mut (*this).dyna.actor, sInitChain.as_mut_ptr());
    DynaPolyActor_Init(&mut (*this).dyna, DPM_UNK as libc::c_int);
    CollisionHeader_GetVirtual(&mut gPoeSellerCol as *mut CollisionHeader as
                                   *mut libc::c_void, &mut colHeader);
    (*this).dyna.bgId =
        DynaPoly_SetBgActor(globalCtx, &mut (*globalCtx).colCtx.dyna,
                            &mut (*this).dyna.actor, colHeader);
    SkelAnime_InitFlex(globalCtx, &mut (*this).skelAnime, &mut gPoeSellerSkel,
                       &mut gPoeSellerIdleAnim,
                       (*this).jointTable.as_mut_ptr(),
                       (*this).morphTable.as_mut_ptr(), 12 as libc::c_int);
    Collider_InitCylinder(globalCtx, &mut (*this).collider);
    Collider_SetCylinderType1(globalCtx, &mut (*this).collider,
                              &mut (*this).dyna.actor, &mut sCylinderInit);
    i = 0 as libc::c_int;
    while i <
              (::std::mem::size_of::<[ColliderCylinderInitType1; 3]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<ColliderCylinderInitType1>()
                                                   as libc::c_ulong) as s32 {
        Collider_InitCylinder(globalCtx,
                              &mut *(*this).bottlesColliders.as_mut_ptr().offset(i
                                                                                     as
                                                                                     isize));
        Collider_SetCylinderType1(globalCtx,
                                  &mut *(*this).bottlesColliders.as_mut_ptr().offset(i
                                                                                         as
                                                                                         isize),
                                  &mut (*this).dyna.actor,
                                  &mut *sBottlesCylindersInit.as_mut_ptr().offset(i
                                                                                      as
                                                                                      isize));
        i += 1
    }
    (*this).light =
        LightContext_InsertLight(globalCtx, &mut (*globalCtx).lightCtx,
                                 &mut (*this).lightInfo);
    Lights_PointNoGlowSetInfo(&mut (*this).lightInfo,
                              (*this).dyna.actor.home.pos.x as s16,
                              (*this).dyna.actor.home.pos.y as s16,
                              (*this).dyna.actor.home.pos.z as s16,
                              255 as libc::c_int as u8_0,
                              255 as libc::c_int as u8_0,
                              255 as libc::c_int as u8_0,
                              200 as libc::c_int as s16);
    ActorShape_Init(&mut (*this).dyna.actor.shape, 0.0f32,
                    Some(ActorShadow_DrawCircle as
                             unsafe extern "C" fn(_: *mut Actor,
                                                  _: *mut Lights,
                                                  _: *mut GlobalContext)
                                 -> ()), 35.0f32);
    Actor_SetScale(&mut (*this).dyna.actor, 0.01f32);
    (*this).dyna.actor.colChkInfo.mass = 0xff as libc::c_int as u8_0;
    (*this).dyna.actor.speedXZ = 0.0f32;
    (*this).dyna.actor.velocity.y = 0.0f32;
    (*this).dyna.actor.gravity = -1.0f32;
    (*this).actionTimer =
        (Rand_ZeroFloat(100.0f32) as s16 as libc::c_int + 100 as libc::c_int)
            as s16;
    i = 0 as libc::c_int;
    while i <
              (::std::mem::size_of::<[Vec3f; 4]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<Vec3f>()
                                                   as libc::c_ulong) as s32 {
        (*this).cagedSouls[i as usize].infoIdx =
            (Rand_ZeroFloat(30.0f32) as s32 % 3 as libc::c_int) as u8_0;
        (*this).cagedSouls[i as usize].translation.x =
            sCagedSoulPositions[i as usize].x +
                (*this).dyna.actor.world.pos.x;
        (*this).cagedSouls[i as usize].unk_14.x =
            (*this).cagedSouls[i as usize].translation.x;
        (*this).cagedSouls[i as usize].translation.y =
            sCagedSoulPositions[i as usize].y +
                (*this).dyna.actor.world.pos.y;
        (*this).cagedSouls[i as usize].unk_14.y =
            (*this).cagedSouls[i as usize].translation.y;
        (*this).cagedSouls[i as usize].translation.z =
            sCagedSoulPositions[i as usize].z +
                (*this).dyna.actor.world.pos.z;
        (*this).cagedSouls[i as usize].unk_14.z =
            (*this).cagedSouls[i as usize].translation.z;
        (*this).cagedSouls[i as usize].unk_1 = 1 as libc::c_int as u8_0;
        (*this).cagedSouls[i as usize].unk_2 = 0 as libc::c_int as u8_0;
        (*this).cagedSouls[i as usize].unk_3 =
            (*this).cagedSouls[i as usize].unk_2;
        (*this).cagedSouls[i as usize].unk_24 = 0.0f32;
        (*this).cagedSouls[i as usize].unk_20 =
            (*this).cagedSouls[i as usize].unk_24;
        (*this).cagedSouls[i as usize].unk_6 = Rand_ZeroFloat(40.0f32) as s16;
        (*this).cagedSouls[i as usize].rotate180 =
            ((*this).cagedSouls[i as usize].unk_6 as libc::c_int &
                 1 as libc::c_int) as u8_0;
        (*this).cagedSouls[i as usize].unk_28 = 0.2f32;
        i += 1
    }
    rand = Rand_ZeroOne();
    (*this).lightColor.r =
        ((rand * 30.0f32) as s8 as libc::c_int + 225 as libc::c_int) as u8_0;
    (*this).lightColor.g =
        ((rand * 100.0f32) as s8 as libc::c_int + 155 as libc::c_int) as u8_0;
    (*this).lightColor.b =
        ((rand * 160.0f32) as s8 as libc::c_int + 95 as libc::c_int) as u8_0;
    (*this).lightColor.a = 200 as libc::c_int as u8_0;
    Matrix_Translate((*this).dyna.actor.world.pos.x,
                     (*this).dyna.actor.world.pos.y,
                     (*this).dyna.actor.world.pos.z,
                     MTXMODE_NEW as libc::c_int as u8_0);
    Matrix_RotateZYX((*this).dyna.actor.world.rot.x,
                     (*this).dyna.actor.world.rot.y,
                     (*this).dyna.actor.world.rot.z,
                     MTXMODE_APPLY as libc::c_int as u8_0);
    focusOffset.y = 0.0f32;
    focusOffset.x = focusOffset.y;
    focusOffset.z = 44.0f32;
    Matrix_MultVec3f(&mut focusOffset, &mut (*this).dyna.actor.focus.pos);
    (*this).dyna.actor.focus.pos.y += 62.5f32;
    func_80A2F180(this);
    (*this).actionFunc =
        Some(func_80A2F83C as
                 unsafe extern "C" fn(_: *mut EnGb, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn EnGb_Destroy(mut thisx: *mut Actor,
                                      mut globalCtx: *mut GlobalContext) {
    let mut this: *mut EnGb = thisx as *mut EnGb;
    Collider_DestroyCylinder(globalCtx, &mut (*this).collider);
    LightContext_RemoveLight(globalCtx, &mut (*globalCtx).lightCtx,
                             (*this).light);
    DynaPoly_DeleteBgActor(globalCtx, &mut (*globalCtx).colCtx.dyna,
                           (*this).dyna.bgId);
}
#[no_mangle]
pub unsafe extern "C" fn func_80A2F608(mut this: *mut EnGb) {
    let mut i: s32 = 0;
    let mut sp48: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp3C: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    Matrix_Translate((*this).dyna.actor.world.pos.x,
                     (*this).dyna.actor.world.pos.y,
                     (*this).dyna.actor.world.pos.z,
                     MTXMODE_NEW as libc::c_int as u8_0);
    Matrix_RotateZYX((*this).dyna.actor.world.rot.x,
                     (*this).dyna.actor.world.rot.y,
                     (*this).dyna.actor.world.rot.z,
                     MTXMODE_APPLY as libc::c_int as u8_0);
    sp48.y = 0.0f32;
    sp48.x = sp48.y;
    sp48.z = 25.0f32;
    Matrix_MultVec3f(&mut sp48, &mut sp3C);
    (*this).collider.dim.pos.x = sp3C.x as s16;
    (*this).collider.dim.pos.y = sp3C.y as s16;
    (*this).collider.dim.pos.z = sp3C.z as s16;
    i = 0 as libc::c_int;
    while i <
              (::std::mem::size_of::<[Vec3f; 3]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<Vec3f>()
                                                   as libc::c_ulong) as s32 {
        Matrix_Translate((*this).dyna.actor.world.pos.x,
                         (*this).dyna.actor.world.pos.y,
                         (*this).dyna.actor.world.pos.z,
                         MTXMODE_NEW as libc::c_int as u8_0);
        Matrix_RotateZYX((*this).dyna.actor.world.rot.x,
                         (*this).dyna.actor.world.rot.y,
                         (*this).dyna.actor.world.rot.z,
                         MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_MultVec3f(&mut *sBottlesPositions.as_mut_ptr().offset(i as
                                                                         isize),
                         &mut sp3C);
        (*this).bottlesColliders[i as usize].dim.pos.x = sp3C.x as s16;
        (*this).bottlesColliders[i as usize].dim.pos.y = sp3C.y as s16;
        (*this).bottlesColliders[i as usize].dim.pos.z = sp3C.z as s16;
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_80A2F760(mut this: *mut EnGb) -> s32 {
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i <
              (::std::mem::size_of::<[EnGbCagedSoul; 4]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<EnGbCagedSoul>()
                                                   as libc::c_ulong) as s32 {
        if (*this).cagedSouls[i as usize].unk_3 != 0 {
            return 1 as libc::c_int
        }
        i += 1
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_80A2F7C0(mut this: *mut EnGb) {
    Animation_Change(&mut (*this).skelAnime, &mut gPoeSellerSwingStickAnim,
                     1.0f32, 0.0f32,
                     Animation_GetLastFrame(&mut gPoeSellerSwingStickAnim as
                                                *mut AnimationHeader as
                                                *mut libc::c_void) as f32_0,
                     ANIMMODE_ONCE as libc::c_int as u8_0, 0.0f32);
    Audio_PlayActorSound2(&mut (*this).dyna.actor,
                          0x28b0 as libc::c_int as u16_0);
    (*this).actionFunc =
        Some(func_80A2FC70 as
                 unsafe extern "C" fn(_: *mut EnGb, _: *mut GlobalContext)
                     -> ());
}
#[no_mangle]
pub unsafe extern "C" fn func_80A2F83C(mut this: *mut EnGb,
                                       mut globalCtx: *mut GlobalContext) {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    if func_80A2F760(this) == 0 {
        if (*this).actionTimer as libc::c_int != 0 as libc::c_int {
            (*this).actionTimer -= 1
        } else { func_80A2F7C0(this); return }
    }
    if Actor_ProcessTalkRequest(&mut (*this).dyna.actor, globalCtx) != 0 {
        match func_8002F368(globalCtx) as libc::c_int {
            0 => {
                func_80A2F180(this);
                (*this).actionFunc =
                    Some(func_80A2F94C as
                             unsafe extern "C" fn(_: *mut EnGb,
                                                  _: *mut GlobalContext)
                                 -> ())
            }
            27 => {
                (*player).actor.textId = 0x70f6 as libc::c_int as u16_0;
                (*this).actionFunc =
                    Some(func_80A2F9C0 as
                             unsafe extern "C" fn(_: *mut EnGb,
                                                  _: *mut GlobalContext)
                                 -> ())
            }
            28 => {
                (*player).actor.textId = 0x70f7 as libc::c_int as u16_0;
                (*this).actionFunc =
                    Some(func_80A2FA50 as
                             unsafe extern "C" fn(_: *mut EnGb,
                                                  _: *mut GlobalContext)
                                 -> ())
            }
            _ => { }
        }
        return
    }
    if (*this).dyna.actor.xzDistToPlayer < 100.0f32 {
        func_8002F298(&mut (*this).dyna.actor, globalCtx, 100.0f32,
                      EXCH_ITEM_POE as libc::c_int as u32_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_80A2F94C(mut this: *mut EnGb,
                                       mut globalCtx: *mut GlobalContext) {
    if Message_GetState(&mut (*globalCtx).msgCtx) as libc::c_int ==
           TEXT_STATE_DONE as libc::c_int &&
           Message_ShouldAdvance(globalCtx) as libc::c_int != 0 {
        if gSaveContext.infTable[0xb as libc::c_int as usize] as libc::c_int &
               0x40 as libc::c_int == 0 {
            gSaveContext.infTable[0xb as libc::c_int as usize] =
                (gSaveContext.infTable[0xb as libc::c_int as usize] as
                     libc::c_int | 0x40 as libc::c_int) as u16_0
        }
        func_80A2F180(this);
        (*this).actionFunc =
            Some(func_80A2F83C as
                     unsafe extern "C" fn(_: *mut EnGb, _: *mut GlobalContext)
                         -> ())
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_80A2F9C0(mut this: *mut EnGb,
                                       mut globalCtx: *mut GlobalContext) {
    if Message_GetState(&mut (*globalCtx).msgCtx) as libc::c_int ==
           TEXT_STATE_DONE as libc::c_int &&
           Message_ShouldAdvance(globalCtx) as libc::c_int != 0 {
        if gSaveContext.infTable[0xb as libc::c_int as usize] as libc::c_int &
               0x40 as libc::c_int == 0 {
            gSaveContext.infTable[0xb as libc::c_int as usize] =
                (gSaveContext.infTable[0xb as libc::c_int as usize] as
                     libc::c_int | 0x40 as libc::c_int) as u16_0
        }
        func_80A2F180(this);
        Player_UpdateBottleHeld(globalCtx,
                                (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER
                                                                     as
                                                                     libc::c_int
                                                                     as
                                                                     usize].head
                                    as *mut Player,
                                ITEM_BOTTLE as libc::c_int,
                                PLAYER_AP_BOTTLE as libc::c_int);
        Rupees_ChangeBy(10 as libc::c_int as s16);
        (*this).actionFunc =
            Some(func_80A2F83C as
                     unsafe extern "C" fn(_: *mut EnGb, _: *mut GlobalContext)
                         -> ())
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_80A2FA50(mut this: *mut EnGb,
                                       mut globalCtx: *mut GlobalContext) {
    if Message_GetState(&mut (*globalCtx).msgCtx) as libc::c_int ==
           TEXT_STATE_DONE as libc::c_int &&
           Message_ShouldAdvance(globalCtx) as libc::c_int != 0 {
        if gSaveContext.infTable[0xb as libc::c_int as usize] as libc::c_int &
               0x40 as libc::c_int == 0 {
            gSaveContext.infTable[0xb as libc::c_int as usize] =
                (gSaveContext.infTable[0xb as libc::c_int as usize] as
                     libc::c_int | 0x40 as libc::c_int) as u16_0
        }
        func_80A2F180(this);
        Player_UpdateBottleHeld(globalCtx,
                                (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER
                                                                     as
                                                                     libc::c_int
                                                                     as
                                                                     usize].head
                                    as *mut Player,
                                ITEM_BOTTLE as libc::c_int,
                                PLAYER_AP_BOTTLE as libc::c_int);
        Rupees_ChangeBy(50 as libc::c_int as s16);
        gSaveContext.highScores[HS_POE_POINTS as libc::c_int as usize] +=
            100 as libc::c_int;
        if gSaveContext.highScores[HS_POE_POINTS as libc::c_int as usize] !=
               1000 as libc::c_int {
            if gSaveContext.highScores[HS_POE_POINTS as libc::c_int as usize]
                   > 1100 as libc::c_int {
                gSaveContext.highScores[HS_POE_POINTS as libc::c_int as usize]
                    = 1100 as libc::c_int
            }
            (*this).actionFunc =
                Some(func_80A2F83C as
                         unsafe extern "C" fn(_: *mut EnGb,
                                              _: *mut GlobalContext) -> ())
        } else {
            let mut player: *mut Player =
                (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as
                                                     libc::c_int as
                                                     usize].head as
                    *mut Player;
            (*player).exchangeItemId = EXCH_ITEM_NONE as libc::c_int as s8;
            (*this).textId = 0x70f8 as libc::c_int as u16_0;
            Message_ContinueTextbox(globalCtx, (*this).textId);
            (*this).actionFunc =
                Some(func_80A2FB40 as
                         unsafe extern "C" fn(_: *mut EnGb,
                                              _: *mut GlobalContext) -> ())
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_80A2FB40(mut this: *mut EnGb,
                                       mut globalCtx: *mut GlobalContext) {
    if Message_GetState(&mut (*globalCtx).msgCtx) as libc::c_int ==
           TEXT_STATE_DONE as libc::c_int &&
           Message_ShouldAdvance(globalCtx) as libc::c_int != 0 {
        func_8002F434(&mut (*this).dyna.actor, globalCtx,
                      GI_BOTTLE as libc::c_int, 100.0f32, 10.0f32);
        (*this).actionFunc =
            Some(func_80A2FBB0 as
                     unsafe extern "C" fn(_: *mut EnGb, _: *mut GlobalContext)
                         -> ())
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_80A2FBB0(mut this: *mut EnGb,
                                       mut globalCtx: *mut GlobalContext) {
    if Actor_HasParent(&mut (*this).dyna.actor, globalCtx) != 0 {
        (*this).dyna.actor.parent = 0 as *mut Actor;
        (*this).actionFunc =
            Some(func_80A2FC0C as
                     unsafe extern "C" fn(_: *mut EnGb, _: *mut GlobalContext)
                         -> ())
    } else {
        func_8002F434(&mut (*this).dyna.actor, globalCtx,
                      GI_BOTTLE as libc::c_int, 100.0f32, 10.0f32);
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_80A2FC0C(mut this: *mut EnGb,
                                       mut globalCtx: *mut GlobalContext) {
    if Message_GetState(&mut (*globalCtx).msgCtx) as libc::c_int ==
           TEXT_STATE_DONE as libc::c_int &&
           Message_ShouldAdvance(globalCtx) as libc::c_int != 0 {
        Actor_ProcessTalkRequest(&mut (*this).dyna.actor, globalCtx);
        func_80A2F180(this);
        (*this).actionFunc =
            Some(func_80A2F83C as
                     unsafe extern "C" fn(_: *mut EnGb, _: *mut GlobalContext)
                         -> ())
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_80A2FC70(mut this: *mut EnGb,
                                       mut globalCtx: *mut GlobalContext) {
    if (*this).skelAnime.curFrame ==
           Animation_GetLastFrame(&mut gPoeSellerSwingStickAnim as
                                      *mut AnimationHeader as
                                      *mut libc::c_void) as libc::c_int as
               libc::c_float {
        Animation_Change(&mut (*this).skelAnime, &mut gPoeSellerIdleAnim,
                         1.0f32, 0.0f32,
                         Animation_GetLastFrame(&mut gPoeSellerIdleAnim as
                                                    *mut AnimationHeader as
                                                    *mut libc::c_void) as
                             f32_0, ANIMMODE_LOOP as libc::c_int as u8_0,
                         0.0f32);
        (*this).actionFunc =
            Some(func_80A2F83C as
                     unsafe extern "C" fn(_: *mut EnGb, _: *mut GlobalContext)
                         -> ())
    } else if (*this).skelAnime.curFrame == 18.0f32 {
        (*this).cagedSouls[1 as libc::c_int as usize].unk_1 =
            3 as libc::c_int as u8_0;
        (*this).cagedSouls[1 as libc::c_int as usize].unk_3 =
            1 as libc::c_int as u8_0;
        (*this).cagedSouls[2 as libc::c_int as usize].unk_1 =
            3 as libc::c_int as u8_0;
        (*this).cagedSouls[2 as libc::c_int as usize].unk_3 =
            1 as libc::c_int as u8_0;
        (*this).cagedSouls[3 as libc::c_int as usize].unk_1 =
            3 as libc::c_int as u8_0;
        (*this).cagedSouls[3 as libc::c_int as usize].unk_3 =
            1 as libc::c_int as u8_0;
        (*this).actionFunc.is_some();
        (*this).cagedSouls[0 as libc::c_int as usize].unk_1 =
            3 as libc::c_int as u8_0;
        (*this).cagedSouls[0 as libc::c_int as usize].unk_3 =
            1 as libc::c_int as u8_0;
        (*this).actionFunc.is_some();
        (*this).actionTimer =
            (Rand_ZeroFloat(600.0f32) as s16 as libc::c_int +
                 600 as libc::c_int) as s16;
        Audio_PlayActorSound2(&mut (*this).dyna.actor,
                              0x2873 as libc::c_int as u16_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnGb_Update(mut thisx: *mut Actor,
                                     mut globalCtx2: *mut GlobalContext) {
    let mut this: *mut EnGb = thisx as *mut EnGb;
    let mut globalCtx: *mut GlobalContext = globalCtx2;
    let mut i: s32 = 0;
    let mut rand: f32_0 = 0.;
    (*this).frameTimer += 1;
    SkelAnime_Update(&mut (*this).skelAnime);
    (*this).actionFunc.expect("non-null function pointer")(this, globalCtx);
    (*this).dyna.actor.textId = (*this).textId;
    func_80A2F608(this);
    CollisionCheck_SetOC(globalCtx, &mut (*globalCtx).colChkCtx,
                         &mut (*this).collider.base);
    i = 0 as libc::c_int;
    while i <
              (::std::mem::size_of::<[ColliderCylinder; 3]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<ColliderCylinder>()
                                                   as libc::c_ulong) as s32 {
        CollisionCheck_SetOC(globalCtx, &mut (*globalCtx).colChkCtx,
                             &mut (*(*this).bottlesColliders.as_mut_ptr().offset(i
                                                                                     as
                                                                                     isize)).base);
        i += 1
    }
    rand = Rand_ZeroOne();
    (*this).lightColor.r =
        ((rand * 30.0f32) as s8 as libc::c_int + 225 as libc::c_int) as u8_0;
    (*this).lightColor.g =
        ((rand * 100.0f32) as s8 as libc::c_int + 155 as libc::c_int) as u8_0;
    (*this).lightColor.b =
        ((rand * 160.0f32) as s8 as libc::c_int + 95 as libc::c_int) as u8_0;
    (*this).lightColor.a = 200 as libc::c_int as u8_0;
    EnGb_UpdateCagedSouls(this, globalCtx);
}
#[no_mangle]
pub unsafe extern "C" fn EnGb_Draw(mut thisx: *mut Actor,
                                   mut globalCtx: *mut GlobalContext) {
    let mut this: *mut EnGb = thisx as *mut EnGb;
    let mut pad: s32 = 0;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_en_gb.c\x00" as *const u8 as *const libc::c_char,
                    763 as libc::c_int);
    func_80093D18((*globalCtx).state.gfxCtx);
    let fresh0 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g: *mut Gfx = fresh0;
    (*_g).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh1 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_0: *mut Gfx = fresh1;
    (*_g_0).words.w0 =
        (0xfb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_0).words.w1 =
        ((*this).lightColor.r as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((*this).lightColor.g as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((*this).lightColor.b as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    Lights_PointNoGlowSetInfo(&mut (*this).lightInfo,
                              (*this).dyna.actor.world.pos.x as s16,
                              (*this).dyna.actor.world.pos.y as s16,
                              (*this).dyna.actor.world.pos.z as s16,
                              (*this).lightColor.r, (*this).lightColor.g,
                              (*this).lightColor.b,
                              (*this).lightColor.a as s16);
    SkelAnime_DrawFlexOpa(globalCtx, (*this).skelAnime.skeleton,
                          (*this).skelAnime.jointTable,
                          (*this).skelAnime.dListCount as s32, None, None,
                          &mut (*this).dyna.actor as *mut Actor as
                              *mut libc::c_void);
    EnGb_DrawCagedSouls(this, globalCtx);
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_en_gb.c\x00" as *const u8 as *const libc::c_char,
                     796 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn EnGb_UpdateCagedSouls(mut this: *mut EnGb,
                                               mut globalCtx:
                                                   *mut GlobalContext) {
    let mut temp_f20: f32_0 = 0.;
    let mut rot: s16 = 0;
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        match (*this).cagedSouls[i as usize].unk_1 as libc::c_int {
            0 => {
                Math_ApproachF(&mut (*(*this).cagedSouls.as_mut_ptr().offset(i
                                                                                 as
                                                                                 isize)).unk_20,
                               1.0f32, 0.02f32,
                               (*this).cagedSouls[i as usize].unk_24);
                Math_ApproachF(&mut (*(*this).cagedSouls.as_mut_ptr().offset(i
                                                                                 as
                                                                                 isize)).unk_24,
                               1.0f32, 0.001f32, 1.0f32);
                if (*this).cagedSouls[i as usize].unk_28 - 0.01f32 <=
                       (*this).cagedSouls[i as usize].unk_20 {
                    (*this).cagedSouls[i as usize].unk_20 =
                        (*this).cagedSouls[i as usize].unk_28;
                    (*this).cagedSouls[i as usize].unk_24 = 0.0f32;
                    (*this).cagedSouls[i as usize].unk_1 =
                        1 as libc::c_int as u8_0;
                    (*this).cagedSouls[i as usize].unk_2 =
                        2 as libc::c_int as u8_0;
                    (*this).cagedSouls[i as usize].unk_6 =
                        0 as libc::c_int as s16
                }
            }
            1 => {
                if (*this).cagedSouls[i as usize].unk_6 as libc::c_int !=
                       0 as libc::c_int {
                    (*this).cagedSouls[i as usize].unk_6 -= 1
                } else {
                    (*this).cagedSouls[i as usize].unk_3 =
                        0 as libc::c_int as u8_0;
                    (*this).cagedSouls[i as usize].unk_24 = 0.0f32;
                    (*this).cagedSouls[i as usize].unk_1 =
                        (*this).cagedSouls[i as usize].unk_2
                }
            }
            2 => {
                Math_ApproachF(&mut (*(*this).cagedSouls.as_mut_ptr().offset(i
                                                                                 as
                                                                                 isize)).unk_20,
                               0.0f32, 0.02f32,
                               (*this).cagedSouls[i as usize].unk_24);
                Math_ApproachF(&mut (*(*this).cagedSouls.as_mut_ptr().offset(i
                                                                                 as
                                                                                 isize)).unk_24,
                               1.0f32, 0.001f32, 1.0f32);
                if (*this).cagedSouls[i as usize].unk_20 <= 0.01f32 {
                    (*this).cagedSouls[i as usize].unk_28 =
                        (*this).cagedSouls[i as usize].unk_28 + 0.2f32;
                    if (*this).cagedSouls[i as usize].unk_28 > 1.0f32 {
                        (*this).cagedSouls[i as usize].unk_28 = 1.0f32
                    }
                    (*this).cagedSouls[i as usize].unk_20 = 0.0f32;
                    (*this).cagedSouls[i as usize].unk_24 = 0.0f32;
                    (*this).cagedSouls[i as usize].unk_1 =
                        1 as libc::c_int as u8_0;
                    (*this).cagedSouls[i as usize].unk_2 =
                        0 as libc::c_int as u8_0;
                    (*this).cagedSouls[i as usize].unk_6 =
                        0 as libc::c_int as s16
                }
            }
            3 => {
                Math_ApproachF(&mut (*(*this).cagedSouls.as_mut_ptr().offset(i
                                                                                 as
                                                                                 isize)).unk_20,
                               0.0f32, 0.5f32, 1.0f32);
                if (*this).cagedSouls[i as usize].unk_20 <= 0.01f32 {
                    (*this).cagedSouls[i as usize].unk_28 = 0.2f32;
                    (*this).cagedSouls[i as usize].unk_20 = 0.0f32;
                    (*this).cagedSouls[i as usize].unk_24 = 0.0f32;
                    (*this).cagedSouls[i as usize].unk_1 =
                        1 as libc::c_int as u8_0;
                    (*this).cagedSouls[i as usize].unk_2 =
                        0 as libc::c_int as u8_0;
                    (*this).cagedSouls[i as usize].unk_6 =
                        (Rand_ZeroFloat(60.0f32) as s16 as libc::c_int +
                             60 as libc::c_int) as s16
                }
            }
            _ => { }
        }
        temp_f20 = (*this).cagedSouls[i as usize].unk_20 * 60.0f32;
        if i == 0 as libc::c_int || i == 3 as libc::c_int {
            (*this).cagedSouls[i as usize].translation.x =
                (*this).cagedSouls[i as usize].unk_14.x;
            (*this).cagedSouls[i as usize].translation.y =
                (*this).cagedSouls[i as usize].unk_14.y + temp_f20;
            (*this).cagedSouls[i as usize].translation.z =
                (*this).cagedSouls[i as usize].unk_14.z
        } else if i == 1 as libc::c_int {
            rot =
                ((*this).dyna.actor.world.rot.y as libc::c_int -
                     0x4000 as libc::c_int) as s16;
            (*this).cagedSouls[i as usize].translation.x =
                (*this).cagedSouls[i as usize].unk_14.x +
                    Math_SinS(rot) * temp_f20;
            (*this).cagedSouls[i as usize].translation.z =
                (*this).cagedSouls[i as usize].unk_14.z +
                    Math_CosS(rot) * temp_f20;
            (*this).cagedSouls[i as usize].translation.y =
                (*this).cagedSouls[i as usize].unk_14.y
        } else {
            rot =
                ((*this).dyna.actor.world.rot.y as libc::c_int +
                     0x4000 as libc::c_int) as s16;
            (*this).cagedSouls[i as usize].translation.x =
                (*this).cagedSouls[i as usize].unk_14.x +
                    Math_SinS(rot) * temp_f20;
            (*this).cagedSouls[i as usize].translation.z =
                (*this).cagedSouls[i as usize].unk_14.z +
                    Math_CosS(rot) * temp_f20;
            (*this).cagedSouls[i as usize].translation.y =
                (*this).cagedSouls[i as usize].unk_14.y
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnGb_DrawCagedSouls(mut this: *mut EnGb,
                                             mut globalCtx:
                                                 *mut GlobalContext) {
    let mut pad: s32 = 0;
    let mut i: s32 = 0;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_en_gb.c\x00" as *const u8 as *const libc::c_char,
                    914 as libc::c_int);
    func_80093D84((*globalCtx).state.gfxCtx);
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        let mut idx: s32 = (*this).cagedSouls[i as usize].infoIdx as s32;
        let fresh2 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g: *mut Gfx = fresh2;
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
                             0 as libc::c_int as u32_0,
                             0 as libc::c_int as u32_0, 32 as libc::c_int,
                             64 as libc::c_int, 1 as libc::c_int,
                             0 as libc::c_int as u32_0,
                             ((sCagedSoulInfo[idx as usize].timerMultiplier as
                                   libc::c_int *
                                   (*this).frameTimer as libc::c_int) as
                                  u32_0).wrapping_rem(512 as libc::c_int as
                                                          libc::c_uint),
                             32 as libc::c_int, 128 as libc::c_int) as
                libc::c_uint;
        let fresh3 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_0: *mut Gfx = fresh3;
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
            gSegments[((sCagedSoulInfo[idx as usize].texture as u32_0) <<
                           4 as libc::c_int >> 28 as libc::c_int) as
                          usize].wrapping_add(sCagedSoulInfo[idx as
                                                                 usize].texture
                                                  as u32_0 &
                                                  0xffffff as libc::c_int as
                                                      libc::c_uint).wrapping_add(0x80000000
                                                                                     as
                                                                                     libc::c_uint)
                as *mut libc::c_void as libc::c_uint;
        let fresh4 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_1: *mut Gfx = fresh4;
        (*_g_1).words.w0 =
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
        (*_g_1).words.w1 =
            (sCagedSoulInfo[idx as usize].prim.r as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (sCagedSoulInfo[idx as usize].prim.g as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                (sCagedSoulInfo[idx as usize].prim.b as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (sCagedSoulInfo[idx as usize].prim.a as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh5 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_2: *mut Gfx = fresh5;
        (*_g_2).words.w0 =
            (0xfb as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_2).words.w1 =
            (sCagedSoulInfo[idx as usize].env.r as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (sCagedSoulInfo[idx as usize].env.g as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                (sCagedSoulInfo[idx as usize].env.b as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (sCagedSoulInfo[idx as usize].env.a as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        Matrix_Push();
        Matrix_Translate((*this).cagedSouls[i as usize].translation.x,
                         (*this).cagedSouls[i as usize].translation.y,
                         (*this).cagedSouls[i as usize].translation.z,
                         MTXMODE_NEW as libc::c_int as u8_0);
        func_800D1FD4(&mut (*globalCtx).billboardMtxF);
        if (*this).cagedSouls[i as usize].rotate180 != 0 {
            Matrix_RotateZYX(0 as libc::c_int as s16,
                             -(0x8000 as libc::c_int) as s16,
                             0 as libc::c_int as s16,
                             MTXMODE_APPLY as libc::c_int as u8_0);
        }
        Matrix_Scale(0.007f32, 0.007f32, 1.0f32,
                     MTXMODE_APPLY as libc::c_int as u8_0);
        let fresh6 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_3: *mut Gfx = fresh6;
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
                          b"../z_en_gb.c\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                          955 as libc::c_int) as libc::c_uint;
        let fresh7 = (*__gfxCtx).polyXlu.p;
        (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
        let mut _g_4: *mut Gfx = fresh7;
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
        (*_g_4).words.w1 = gPoeSellerCagedSoulDL.as_mut_ptr() as libc::c_uint;
        Matrix_Pop();
        i += 1
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_en_gb.c\x00" as *const u8 as *const libc::c_char,
                     962 as libc::c_int);
}
unsafe extern "C" fn run_static_initializers() {
    sInitChain =
        [{
             let mut init = InitChainEntry{cont_type_0_offset_value: [0; 4],};
             init.set_cont(1 as libc::c_int as u32_0);
             init.set_type_0(ICHAINTYPE_U8 as libc::c_int as u32_0);
             init.set_offset(&mut (*(0 as *mut Actor)).targetMode as *mut s8
                                 as size_t as u32_0);
             init.set_value(6 as libc::c_int);
             init
         },
         {
             let mut init = InitChainEntry{cont_type_0_offset_value: [0; 4],};
             init.set_cont(0 as libc::c_int as u32_0);
             init.set_type_0(ICHAINTYPE_F32 as libc::c_int as u32_0);
             init.set_offset(&mut (*(0 as *mut Actor)).targetArrowOffset as
                                 *mut f32_0 as size_t as u32_0);
             init.set_value(2200 as libc::c_int);
             init
         }]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
