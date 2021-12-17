#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn sqrtf(f: f32_0) -> f32_0;
    #[no_mangle]
    fn DmaMgr_SendRequest2(req: *mut DmaRequest, ram: u32_0, vrom: u32_0,
                           size: u32_0, unk5: u32_0, queue: *mut OSMesgQueue,
                           msg: OSMesg, file: *const libc::c_char, line: s32)
     -> s32;
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn osRecvMesg(mq: *mut OSMesgQueue, msg: *mut OSMesg, flag: s32) -> s32;
    #[no_mangle]
    fn osCreateMesgQueue(mq: *mut OSMesgQueue, msg: *mut OSMesg, count: s32);
    #[no_mangle]
    fn Flags_SetEventChkInf(flag: s32);
    #[no_mangle]
    fn Math_CosS(angle: s16) -> f32_0;
    #[no_mangle]
    fn Math_SinS(angle: s16) -> f32_0;
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
    fn Lights_PointNoGlowSetInfo(info: *mut LightInfo, x: s16, y: s16, z: s16,
                                 r: u8_0, g: u8_0, b: u8_0, radius: s16);
    #[no_mangle]
    fn Lights_DirectionalSetInfo(info: *mut LightInfo, x: s8, y: s8, z: s8,
                                 r: u8_0, g: u8_0, b: u8_0);
    #[no_mangle]
    fn LightContext_InsertLight(globalCtx: *mut GlobalContext,
                                lightCtx: *mut LightContext,
                                info: *mut LightInfo) -> *mut LightNode;
    #[no_mangle]
    fn LightContext_RemoveLight(globalCtx: *mut GlobalContext,
                                lightCtx: *mut LightContext,
                                node: *mut LightNode);
    #[no_mangle]
    fn Lights_GlowCheck(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn Inventory_ReplaceItem(globalCtx: *mut GlobalContext, oldItem: u16_0,
                             newItem: u16_0) -> s32;
    #[no_mangle]
    fn Player_InCsMode(globalCtx: *mut GlobalContext) -> s32;
    #[no_mangle]
    fn Gfx_CallSetupDL(gfx: *mut Gfx, i: u32_0) -> *mut Gfx;
    #[no_mangle]
    fn func_800937C0(gfx: *mut Gfx) -> *mut Gfx;
    #[no_mangle]
    fn func_800938B4(gfxCtx: *mut GraphicsContext);
    #[no_mangle]
    fn func_8009398C(gfxCtx: *mut GraphicsContext);
    #[no_mangle]
    fn func_80093AD0(gfxCtx: *mut GraphicsContext);
    #[no_mangle]
    fn func_80093D84(gfxCtx: *mut GraphicsContext);
    #[no_mangle]
    fn func_80093F34(gfx: *mut Gfx) -> *mut Gfx;
    #[no_mangle]
    fn func_800947AC(gfx: *mut Gfx) -> *mut Gfx;
    #[no_mangle]
    fn func_80094C50(gfxCtx: *mut GraphicsContext);
    #[no_mangle]
    fn Gfx_TwoTexScroll(gfxCtx: *mut GraphicsContext, tile1: s32, x1: u32_0,
                        y1: u32_0, width1: s32, height1: s32, tile2: s32,
                        x2: u32_0, y2: u32_0, width2: s32, height2: s32)
     -> *mut Gfx;
    #[no_mangle]
    fn Object_GetIndex(objectCtx: *mut ObjectContext, objectId: s16) -> s32;
    #[no_mangle]
    fn func_800AA15C();
    #[no_mangle]
    fn func_800AA16C();
    #[no_mangle]
    fn func_800C016C(globalCtx: *mut GlobalContext, src: *mut Vec3f,
                     dest: *mut Vec3f);
    #[no_mangle]
    fn func_800C0CB8(globalCtx: *mut GlobalContext) -> s32;
    #[no_mangle]
    fn FrameAdvance_IsEnabled(globalCtx: *mut GlobalContext) -> s32;
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
    fn Math_Atan2F(x: f32_0, y: f32_0) -> f32_0;
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
    static mut gZBuffer: [[u16_0; 320]; 240];
    #[no_mangle]
    static mut D_8015FCC8: u8_0;
    #[no_mangle]
    static mut gGameInfo: *mut GameInfo;
    #[no_mangle]
    static mut gSaveContext: SaveContext;
    #[no_mangle]
    static mut _vr_holy0_pal_staticSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _vr_holy0_pal_staticSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _vr_holy0_staticSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _vr_holy0_staticSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _vr_cloud3_pal_staticSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _vr_cloud3_pal_staticSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _vr_cloud3_staticSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _vr_cloud3_staticSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _vr_cloud2_pal_staticSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _vr_cloud2_pal_staticSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _vr_cloud2_staticSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _vr_cloud2_staticSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _vr_cloud1_pal_staticSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _vr_cloud1_pal_staticSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _vr_cloud1_staticSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _vr_cloud1_staticSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _vr_cloud0_pal_staticSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _vr_cloud0_pal_staticSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _vr_cloud0_staticSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _vr_cloud0_staticSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _vr_fine3_pal_staticSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _vr_fine3_pal_staticSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _vr_fine3_staticSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _vr_fine3_staticSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _vr_fine2_pal_staticSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _vr_fine2_pal_staticSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _vr_fine2_staticSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _vr_fine2_staticSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _vr_fine1_pal_staticSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _vr_fine1_pal_staticSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _vr_fine1_staticSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _vr_fine1_staticSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _vr_fine0_pal_staticSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _vr_fine0_pal_staticSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    static mut _vr_fine0_staticSegmentRomEnd: [u8_0; 0];
    #[no_mangle]
    static mut _vr_fine0_staticSegmentRomStart: [u8_0; 0];
    #[no_mangle]
    fn GfxPrint_Destroy(this: *mut GfxPrint);
    #[no_mangle]
    fn GfxPrint_Close(this: *mut GfxPrint) -> *mut Gfx;
    #[no_mangle]
    fn GfxPrint_Printf(this: *mut GfxPrint, fmt: *const libc::c_char, _: ...)
     -> s32;
    #[no_mangle]
    fn GfxPrint_SetPos(this: *mut GfxPrint, x: s32, y: s32);
    #[no_mangle]
    fn GfxPrint_SetColor(this: *mut GfxPrint, r: u32_0, g: u32_0, b: u32_0,
                         a: u32_0);
    #[no_mangle]
    fn GfxPrint_Open(this: *mut GfxPrint, dList: *mut Gfx);
    #[no_mangle]
    fn GfxPrint_Init(this: *mut GfxPrint);
    #[no_mangle]
    fn func_800F6D58(_: u8_0, _: u8_0, _: u8_0);
    #[no_mangle]
    fn Message_StartTextbox(globalCtx: *mut GlobalContext, textId: u16_0,
                            actor: *mut Actor);
    #[no_mangle]
    fn func_800F6FB4(_: u8_0);
    #[no_mangle]
    fn Audio_QueueSeqCmd(bgmID: u32_0);
    #[no_mangle]
    fn func_800F5510(seqId: u16_0);
    #[no_mangle]
    static mut D_01000000: Mtx;
    #[no_mangle]
    fn Rand_ZeroOne() -> f32_0;
    #[no_mangle]
    static mut gSegments: [u32_0; 16];
    #[no_mangle]
    fn Audio_SetEnvReverb(reverb: s8);
    #[no_mangle]
    fn func_800F5550(seqId: u16_0);
    #[no_mangle]
    fn func_800FA0B4(a0: u8_0) -> u16_0;
    #[no_mangle]
    static mut gEffShockwaveDL: [Gfx; 0];
    #[no_mangle]
    static mut gEffLightning1Tex: [u64_0; 0];
    #[no_mangle]
    static mut gEffLightning2Tex: [u64_0; 0];
    #[no_mangle]
    static mut gEffLightning3Tex: [u64_0; 0];
    #[no_mangle]
    static mut gEffLightning4Tex: [u64_0; 0];
    #[no_mangle]
    static mut gEffLightning5Tex: [u64_0; 0];
    #[no_mangle]
    static mut gEffLightning6Tex: [u64_0; 0];
    #[no_mangle]
    static mut gEffLightning7Tex: [u64_0; 0];
    #[no_mangle]
    static mut gEffLightning8Tex: [u64_0; 0];
    #[no_mangle]
    static mut gEffLightningDL: [Gfx; 0];
    #[no_mangle]
    static mut gLensFlareCircleDL: [Gfx; 0];
    #[no_mangle]
    static mut gLensFlareRingDL: [Gfx; 0];
    #[no_mangle]
    static mut gMoonDL: [Gfx; 0];
    #[no_mangle]
    static mut gRaindropDL: [Gfx; 0];
    #[no_mangle]
    static mut gSunDL: [Gfx; 0];
    #[no_mangle]
    static mut gFieldSandstormDL: [Gfx; 0];
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
pub struct Color_RGBA8 {
    pub r: u8_0,
    pub g: u8_0,
    pub b: u8_0,
    pub a: u8_0,
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
pub type C2RustUnnamed_17 = libc::c_uint;
pub const OBJECT_ID_MAX: C2RustUnnamed_17 = 402;
pub const OBJECT_ZL4: C2RustUnnamed_17 = 401;
pub const OBJECT_TIMEBLOCK: C2RustUnnamed_17 = 400;
pub const OBJECT_OUKE_HAKA: C2RustUnnamed_17 = 399;
pub const OBJECT_DOOR_KILLER: C2RustUnnamed_17 = 398;
pub const OBJECT_GI_SWORD_1: C2RustUnnamed_17 = 397;
pub const OBJECT_COB: C2RustUnnamed_17 = 396;
pub const OBJECT_COW: C2RustUnnamed_17 = 395;
pub const OBJECT_BWALL: C2RustUnnamed_17 = 394;
pub const OBJECT_PS: C2RustUnnamed_17 = 393;
pub const OBJECT_GS: C2RustUnnamed_17 = 392;
pub const OBJECT_HAKA_DOOR: C2RustUnnamed_17 = 391;
pub const OBJECT_GEFF: C2RustUnnamed_17 = 390;
pub const OBJECT_GJ: C2RustUnnamed_17 = 389;
pub const OBJECT_SKB: C2RustUnnamed_17 = 388;
pub const OBJECT_WF: C2RustUnnamed_17 = 387;
pub const OBJECT_MU: C2RustUnnamed_17 = 386;
pub const OBJECT_SPOT01_MATOYAB: C2RustUnnamed_17 = 385;
pub const OBJECT_SPOT01_MATOYA: C2RustUnnamed_17 = 384;
pub const OBJECT_GI_RUPY: C2RustUnnamed_17 = 383;
pub const OBJECT_GANON_ANIME3: C2RustUnnamed_17 = 382;
pub const OBJECT_GANON_ANIME2: C2RustUnnamed_17 = 381;
pub const OBJECT_GANON_ANIME1: C2RustUnnamed_17 = 380;
pub const OBJECT_GI_DEKUPOUCH: C2RustUnnamed_17 = 379;
pub const OBJECT_EFC_DOUGHNUT: C2RustUnnamed_17 = 378;
pub const OBJECT_DEMO_KEKKAI: C2RustUnnamed_17 = 377;
pub const OBJECT_BOWL: C2RustUnnamed_17 = 376;
pub const OBJECT_GI_SOUL: C2RustUnnamed_17 = 375;
pub const OBJECT_GI_GHOST: C2RustUnnamed_17 = 374;
pub const OBJECT_GI_BUTTERFLY: C2RustUnnamed_17 = 373;
pub const OBJECT_GI_INSECT: C2RustUnnamed_17 = 372;
pub const OBJECT_GI_FIRE: C2RustUnnamed_17 = 371;
pub const OBJECT_DNK: C2RustUnnamed_17 = 370;
pub const OBJECT_DNS: C2RustUnnamed_17 = 369;
pub const OBJECT_KIBAKO2: C2RustUnnamed_17 = 368;
pub const OBJECT_SPOT11_OBJ: C2RustUnnamed_17 = 367;
pub const OBJECT_UNSET_16E: C2RustUnnamed_17 = 366;
pub const OBJECT_JYA_DOOR: C2RustUnnamed_17 = 365;
pub const OBJECT_JYA_IRON: C2RustUnnamed_17 = 364;
pub const OBJECT_DOG: C2RustUnnamed_17 = 363;
pub const OBJECT_GR: C2RustUnnamed_17 = 362;
pub const OBJECT_GELDB: C2RustUnnamed_17 = 361;
pub const OBJECT_SHOPNUTS: C2RustUnnamed_17 = 360;
pub const OBJECT_GLA: C2RustUnnamed_17 = 359;
pub const OBJECT_SPOT00_BREAK: C2RustUnnamed_17 = 358;
pub const OBJECT_RS: C2RustUnnamed_17 = 357;
pub const OBJECT_HINTNUTS: C2RustUnnamed_17 = 356;
pub const OBJECT_BOMBIWA: C2RustUnnamed_17 = 355;
pub const OBJECT_SPOT12_OBJ: C2RustUnnamed_17 = 354;
pub const OBJECT_SPOT05_OBJECTS: C2RustUnnamed_17 = 353;
pub const OBJECT_BG: C2RustUnnamed_17 = 352;
pub const OBJECT_BIGOKUTA: C2RustUnnamed_17 = 351;
pub const OBJECT_SSH: C2RustUnnamed_17 = 350;
pub const OBJECT_GI_GODDESS: C2RustUnnamed_17 = 349;
pub const OBJECT_GI_SUTARU: C2RustUnnamed_17 = 348;
pub const OBJECT_FISH: C2RustUnnamed_17 = 347;
pub const OBJECT_EC: C2RustUnnamed_17 = 346;
pub const OBJECT_DS2: C2RustUnnamed_17 = 345;
pub const OBJECT_GI_M_ARROW: C2RustUnnamed_17 = 344;
pub const OBJECT_GI_HOVERBOOTS: C2RustUnnamed_17 = 343;
pub const OBJECT_ZG: C2RustUnnamed_17 = 342;
pub const OBJECT_TS: C2RustUnnamed_17 = 341;
pub const OBJECT_KA: C2RustUnnamed_17 = 340;
pub const OBJECT_GANON2: C2RustUnnamed_17 = 339;
pub const OBJECT_GI_GERUDOMASK: C2RustUnnamed_17 = 338;
pub const OBJECT_GI_ZORAMASK: C2RustUnnamed_17 = 337;
pub const OBJECT_GI_GOLONMASK: C2RustUnnamed_17 = 336;
pub const OBJECT_ZL2_ANIME2: C2RustUnnamed_17 = 335;
pub const OBJECT_ZL2_ANIME1: C2RustUnnamed_17 = 334;
pub const OBJECT_EFC_ERUPC: C2RustUnnamed_17 = 333;
pub const OBJECT_GT: C2RustUnnamed_17 = 332;
pub const OBJECT_DOOR_GERUDO: C2RustUnnamed_17 = 331;
pub const OBJECT_MAG: C2RustUnnamed_17 = 330;
pub const OBJECT_GI_FROG: C2RustUnnamed_17 = 329;
pub const OBJECT_GI_SOLDOUT: C2RustUnnamed_17 = 328;
pub const OBJECT_GI_BRACELET: C2RustUnnamed_17 = 327;
pub const OBJECT_GI_PRESCRIPTION: C2RustUnnamed_17 = 326;
pub const OBJECT_CS: C2RustUnnamed_17 = 325;
pub const OBJECT_JS: C2RustUnnamed_17 = 324;
pub const OBJECT_GI_BROKENSWORD: C2RustUnnamed_17 = 323;
pub const OBJECT_GI_TICKETSTONE: C2RustUnnamed_17 = 322;
pub const OBJECT_GI_MUSHROOM: C2RustUnnamed_17 = 321;
pub const OBJECT_GI_POWDER: C2RustUnnamed_17 = 320;
pub const OBJECT_GI_EYE_LOTION: C2RustUnnamed_17 = 319;
pub const OBJECT_OS: C2RustUnnamed_17 = 318;
pub const OBJECT_FA: C2RustUnnamed_17 = 317;
pub const OBJECT_MM: C2RustUnnamed_17 = 316;
pub const OBJECT_STREAM: C2RustUnnamed_17 = 315;
pub const OBJECT_SIOFUKI: C2RustUnnamed_17 = 314;
pub const OBJECT_GANON_OBJECTS: C2RustUnnamed_17 = 313;
pub const OBJECT_GI_TRUTH_MASK: C2RustUnnamed_17 = 312;
pub const OBJECT_GI_RABIT_MASK: C2RustUnnamed_17 = 311;
pub const OBJECT_GI_SKJ_MASK: C2RustUnnamed_17 = 310;
pub const OBJECT_GI_REDEAD_MASK: C2RustUnnamed_17 = 309;
pub const OBJECT_GI_KI_TAN_MASK: C2RustUnnamed_17 = 308;
pub const OBJECT_FU: C2RustUnnamed_17 = 307;
pub const OBJECT_MK: C2RustUnnamed_17 = 306;
pub const OBJECT_OWL: C2RustUnnamed_17 = 305;
pub const OBJECT_GJYO_OBJECTS: C2RustUnnamed_17 = 304;
pub const OBJECT_KANBAN: C2RustUnnamed_17 = 303;
pub const OBJECT_GI_COIN: C2RustUnnamed_17 = 302;
pub const OBJECT_GI_GLOVES: C2RustUnnamed_17 = 301;
pub const OBJECT_TSUBO: C2RustUnnamed_17 = 300;
pub const OBJECT_KUSA: C2RustUnnamed_17 = 299;
pub const OBJECT_LIGHTSWITCH: C2RustUnnamed_17 = 298;
pub const OBJECT_INGATE: C2RustUnnamed_17 = 297;
pub const OBJECT_HS: C2RustUnnamed_17 = 296;
pub const OBJECT_MS: C2RustUnnamed_17 = 295;
pub const OBJECT_GM: C2RustUnnamed_17 = 294;
pub const OBJECT_BLKOBJ: C2RustUnnamed_17 = 293;
pub const OBJECT_NWC: C2RustUnnamed_17 = 292;
pub const OBJECT_UNSET_123: C2RustUnnamed_17 = 291;
pub const OBJECT_DAIKU: C2RustUnnamed_17 = 290;
pub const OBJECT_TORYO: C2RustUnnamed_17 = 289;
pub const OBJECT_UNSET_120: C2RustUnnamed_17 = 288;
pub const OBJECT_GOROIWA: C2RustUnnamed_17 = 287;
pub const OBJECT_MAMENOKI: C2RustUnnamed_17 = 286;
pub const OBJECT_D_LIFT: C2RustUnnamed_17 = 285;
pub const OBJECT_D_HSBLOCK: C2RustUnnamed_17 = 284;
pub const OBJECT_D_ELEVATOR: C2RustUnnamed_17 = 283;
pub const OBJECT_GND_MAGIC: C2RustUnnamed_17 = 282;
pub const OBJECT_GI_SEED: C2RustUnnamed_17 = 281;
pub const OBJECT_GI_BOOTS_2: C2RustUnnamed_17 = 280;
pub const OBJECT_YABUSAME_POINT: C2RustUnnamed_17 = 279;
pub const OBJECT_GE1: C2RustUnnamed_17 = 278;
pub const OBJECT_BOB: C2RustUnnamed_17 = 277;
pub const OBJECT_FZ: C2RustUnnamed_17 = 276;
pub const OBJECT_SPOT07_OBJECT: C2RustUnnamed_17 = 275;
pub const OBJECT_SPOT03_OBJECT: C2RustUnnamed_17 = 274;
pub const OBJECT_BOJ: C2RustUnnamed_17 = 273;
pub const OBJECT_ANE: C2RustUnnamed_17 = 272;
pub const OBJECT_DS: C2RustUnnamed_17 = 271;
pub const OBJECT_GI_OCARINA_0: C2RustUnnamed_17 = 270;
pub const OBJECT_BBA: C2RustUnnamed_17 = 269;
pub const OBJECT_BJI: C2RustUnnamed_17 = 268;
pub const OBJECT_GI_BOTTLE_LETTER: C2RustUnnamed_17 = 267;
pub const OBJECT_SKJ: C2RustUnnamed_17 = 266;
pub const OBJECT_GI_NIWATORI: C2RustUnnamed_17 = 265;
pub const OBJECT_CNE: C2RustUnnamed_17 = 264;
pub const OBJECT_AHG: C2RustUnnamed_17 = 263;
pub const OBJECT_IK: C2RustUnnamed_17 = 262;
pub const OBJECT_AOB: C2RustUnnamed_17 = 261;
pub const OBJECT_MASTERZOORA: C2RustUnnamed_17 = 260;
pub const OBJECT_MASTERGOLON: C2RustUnnamed_17 = 259;
pub const OBJECT_MASTERKOKIRIHEAD: C2RustUnnamed_17 = 258;
pub const OBJECT_MASTERKOKIRI: C2RustUnnamed_17 = 257;
pub const OBJECT_UMAJUMP: C2RustUnnamed_17 = 256;
pub const OBJECT_KZ: C2RustUnnamed_17 = 255;
pub const OBJECT_ZO: C2RustUnnamed_17 = 254;
pub const OBJECT_KW1: C2RustUnnamed_17 = 253;
pub const OBJECT_KM1: C2RustUnnamed_17 = 252;
pub const OBJECT_MD: C2RustUnnamed_17 = 251;
pub const OBJECT_MD_UNUSED: C2RustUnnamed_17 = 250;
pub const OBJECT_SPOT01_OBJECTS: C2RustUnnamed_17 = 249;
pub const OBJECT_GI_LONGSWORD: C2RustUnnamed_17 = 248;
pub const OBJECT_GI_GRASS: C2RustUnnamed_17 = 247;
pub const OBJECT_GI_HAMMER: C2RustUnnamed_17 = 246;
pub const OBJECT_GI_SAW: C2RustUnnamed_17 = 245;
pub const OBJECT_GI_FISH: C2RustUnnamed_17 = 244;
pub const OBJECT_GI_BEAN: C2RustUnnamed_17 = 243;
pub const OBJECT_GI_CLOTHES: C2RustUnnamed_17 = 242;
pub const OBJECT_JYA_OBJ: C2RustUnnamed_17 = 241;
pub const OBJECT_SPOT15_OBJ: C2RustUnnamed_17 = 240;
pub const OBJECT_GI_LETTER: C2RustUnnamed_17 = 239;
pub const OBJECT_GI_SHIELD_3: C2RustUnnamed_17 = 238;
pub const OBJECT_DEMO_6K: C2RustUnnamed_17 = 237;
pub const OBJECT_ANI: C2RustUnnamed_17 = 236;
pub const OBJECT_GI_LIQUID: C2RustUnnamed_17 = 235;
pub const OBJECT_GI_GLASSES: C2RustUnnamed_17 = 234;
pub const OBJECT_GI_BOW: C2RustUnnamed_17 = 233;
pub const OBJECT_GI_BOOMERANG: C2RustUnnamed_17 = 232;
pub const OBJECT_GI_PACHINKO: C2RustUnnamed_17 = 231;
pub const OBJECT_FR: C2RustUnnamed_17 = 230;
pub const OBJECT_NY: C2RustUnnamed_17 = 229;
pub const OBJECT_UNSET_E4: C2RustUnnamed_17 = 228;
pub const OBJECT_NY_UNUSED: C2RustUnnamed_17 = 227;
pub const OBJECT_SST: C2RustUnnamed_17 = 226;
pub const OBJECT_GANON: C2RustUnnamed_17 = 225;
pub const OBJECT_MA1: C2RustUnnamed_17 = 224;
pub const OBJECT_GI_MILK: C2RustUnnamed_17 = 223;
pub const OBJECT_GI_OCARINA: C2RustUnnamed_17 = 222;
pub const OBJECT_GI_HOOKSHOT: C2RustUnnamed_17 = 221;
pub const OBJECT_GI_SHIELD_2: C2RustUnnamed_17 = 220;
pub const OBJECT_GI_SCALE: C2RustUnnamed_17 = 219;
pub const OBJECT_GI_EGG: C2RustUnnamed_17 = 218;
pub const OBJECT_GI_BOMB_2: C2RustUnnamed_17 = 217;
pub const OBJECT_GI_ARROW: C2RustUnnamed_17 = 216;
pub const OBJECT_GI_GERUDO: C2RustUnnamed_17 = 215;
pub const OBJECT_ANUBICE: C2RustUnnamed_17 = 214;
pub const OBJECT_BXA: C2RustUnnamed_17 = 213;
pub const OBJECT_RR: C2RustUnnamed_17 = 212;
pub const OBJECT_TW: C2RustUnnamed_17 = 211;
pub const OBJECT_HNI: C2RustUnnamed_17 = 210;
pub const OBJECT_GI_PURSE: C2RustUnnamed_17 = 209;
pub const OBJECT_MA2: C2RustUnnamed_17 = 208;
pub const OBJECT_OF1S: C2RustUnnamed_17 = 207;
pub const OBJECT_GI_BOMB_1: C2RustUnnamed_17 = 206;
pub const OBJECT_GI_MAGICPOT: C2RustUnnamed_17 = 205;
pub const OBJECT_DEKUJR: C2RustUnnamed_17 = 204;
pub const OBJECT_GI_SHIELD_1: C2RustUnnamed_17 = 203;
pub const OBJECT_RU2: C2RustUnnamed_17 = 202;
pub const OBJECT_OF1D_MAP: C2RustUnnamed_17 = 201;
pub const OBJECT_GI_MAP: C2RustUnnamed_17 = 200;
pub const OBJECT_GI_STICK: C2RustUnnamed_17 = 199;
pub const OBJECT_GI_BOTTLE: C2RustUnnamed_17 = 198;
pub const OBJECT_OS_ANIME: C2RustUnnamed_17 = 197;
pub const OBJECT_OE4S: C2RustUnnamed_17 = 196;
pub const OBJECT_OE1S: C2RustUnnamed_17 = 195;
pub const OBJECT_SPOT16_OBJ: C2RustUnnamed_17 = 194;
pub const OBJECT_TR: C2RustUnnamed_17 = 193;
pub const OBJECT_IN: C2RustUnnamed_17 = 192;
pub const OBJECT_GI_BOMBPOUCH: C2RustUnnamed_17 = 191;
pub const OBJECT_GI_ARROWCASE: C2RustUnnamed_17 = 190;
pub const OBJECT_GI_HEARTS: C2RustUnnamed_17 = 189;
pub const OBJECT_SA: C2RustUnnamed_17 = 188;
pub const OBJECT_GI_NUTS: C2RustUnnamed_17 = 187;
pub const OBJECT_GI_MEDAL: C2RustUnnamed_17 = 186;
pub const OBJECT_GI_BOSSKEY: C2RustUnnamed_17 = 185;
pub const OBJECT_GI_COMPASS: C2RustUnnamed_17 = 184;
pub const OBJECT_GI_HEART: C2RustUnnamed_17 = 183;
pub const OBJECT_GI_MELODY: C2RustUnnamed_17 = 182;
pub const OBJECT_SB: C2RustUnnamed_17 = 181;
pub const OBJECT_MO: C2RustUnnamed_17 = 180;
pub const OBJECT_NB: C2RustUnnamed_17 = 179;
pub const OBJECT_SHOP_DUNGEN: C2RustUnnamed_17 = 178;
pub const OBJECT_SPOT17_OBJ: C2RustUnnamed_17 = 177;
pub const OBJECT_BDOOR: C2RustUnnamed_17 = 176;
pub const OBJECT_SPOT18_OBJ: C2RustUnnamed_17 = 175;
pub const OBJECT_SPOT09_OBJ: C2RustUnnamed_17 = 174;
pub const OBJECT_GI_JEWEL: C2RustUnnamed_17 = 173;
pub const OBJECT_BROB: C2RustUnnamed_17 = 172;
pub const OBJECT_MIR_RAY: C2RustUnnamed_17 = 171;
pub const OBJECT_GI_KEY: C2RustUnnamed_17 = 170;
pub const OBJECT_DEMO_TRE_LGT: C2RustUnnamed_17 = 169;
pub const OBJECT_EFC_TW: C2RustUnnamed_17 = 168;
pub const OBJECT_RL: C2RustUnnamed_17 = 167;
pub const OBJECT_DH: C2RustUnnamed_17 = 166;
pub const OBJECT_FD2: C2RustUnnamed_17 = 165;
pub const OBJECT_SYOKUDAI: C2RustUnnamed_17 = 164;
pub const OBJECT_RU1: C2RustUnnamed_17 = 163;
pub const OBJECT_HAKA: C2RustUnnamed_17 = 162;
pub const OBJECT_SPOT02_OBJECTS: C2RustUnnamed_17 = 161;
pub const OBJECT_HORSE_LINK_CHILD: C2RustUnnamed_17 = 160;
pub const OBJECT_MEDAL: C2RustUnnamed_17 = 159;
pub const OBJECT_FW: C2RustUnnamed_17 = 158;
pub const OBJECT_DU: C2RustUnnamed_17 = 157;
pub const OBJECT_FD: C2RustUnnamed_17 = 156;
pub const OBJECT_GNDD: C2RustUnnamed_17 = 155;
pub const OBJECT_HEAVY_OBJECT: C2RustUnnamed_17 = 154;
pub const OBJECT_PO_SISTERS: C2RustUnnamed_17 = 153;
pub const OBJECT_RD: C2RustUnnamed_17 = 152;
pub const OBJECT_SD: C2RustUnnamed_17 = 151;
pub const OBJECT_BDAN_OBJECTS: C2RustUnnamed_17 = 150;
pub const OBJECT_TRIFORCE_SPOT: C2RustUnnamed_17 = 149;
pub const OBJECT_LIGHT_RING: C2RustUnnamed_17 = 148;
pub const OBJECT_GOD_LGT: C2RustUnnamed_17 = 147;
pub const OBJECT_EFC_STAR_FIELD: C2RustUnnamed_17 = 146;
pub const OBJECT_EFC_LGT_SHOWER: C2RustUnnamed_17 = 145;
pub const OBJECT_EFC_FLASH: C2RustUnnamed_17 = 144;
pub const OBJECT_EFC_FIRE_BALL: C2RustUnnamed_17 = 143;
pub const OBJECT_EFC_CRYSTAL_LIGHT: C2RustUnnamed_17 = 142;
pub const OBJECT_HAKACH_OBJECTS: C2RustUnnamed_17 = 141;
pub const OBJECT_BV: C2RustUnnamed_17 = 140;
pub const OBJECT_VM: C2RustUnnamed_17 = 139;
pub const OBJECT_XC: C2RustUnnamed_17 = 138;
pub const OBJECT_TK: C2RustUnnamed_17 = 137;
pub const OBJECT_TA: C2RustUnnamed_17 = 136;
pub const OBJECT_IM: C2RustUnnamed_17 = 135;
pub const OBJECT_VASE: C2RustUnnamed_17 = 134;
pub const OBJECT_TRAP: C2RustUnnamed_17 = 133;
pub const OBJECT_UNSET_84: C2RustUnnamed_17 = 132;
pub const OBJECT_UNSET_83: C2RustUnnamed_17 = 131;
pub const OBJECT_PU_BOX: C2RustUnnamed_17 = 130;
pub const OBJECT_LIGHTBOX: C2RustUnnamed_17 = 129;
pub const OBJECT_UNSET_80: C2RustUnnamed_17 = 128;
pub const OBJECT_UNSET_7F: C2RustUnnamed_17 = 127;
pub const OBJECT_UNSET_7E: C2RustUnnamed_17 = 126;
pub const OBJECT_UNSET_7D: C2RustUnnamed_17 = 125;
pub const OBJECT_WOOD02: C2RustUnnamed_17 = 124;
pub const OBJECT_UNSET_7B: C2RustUnnamed_17 = 123;
pub const OBJECT_UNSET_7A: C2RustUnnamed_17 = 122;
pub const OBJECT_UNSET_79: C2RustUnnamed_17 = 121;
pub const OBJECT_UNSET_78: C2RustUnnamed_17 = 120;
pub const OBJECT_BIRD: C2RustUnnamed_17 = 119;
pub const OBJECT_HATA: C2RustUnnamed_17 = 118;
pub const OBJECT_WARP2: C2RustUnnamed_17 = 117;
pub const OBJECT_SPOT08_OBJ: C2RustUnnamed_17 = 116;
pub const OBJECT_MORI_TEX: C2RustUnnamed_17 = 115;
pub const OBJECT_MORI_OBJECTS: C2RustUnnamed_17 = 114;
pub const OBJECT_MORI_HINERI2A: C2RustUnnamed_17 = 113;
pub const OBJECT_MORI_HINERI2: C2RustUnnamed_17 = 112;
pub const OBJECT_MORI_HINERI1A: C2RustUnnamed_17 = 111;
pub const OBJECT_PO_COMPOSER: C2RustUnnamed_17 = 110;
pub const OBJECT_PO_FIELD: C2RustUnnamed_17 = 109;
pub const OBJECT_RELAY_OBJECTS: C2RustUnnamed_17 = 108;
pub const OBJECT_ICE_OBJECTS: C2RustUnnamed_17 = 107;
pub const OBJECT_SPOT06_OBJECTS: C2RustUnnamed_17 = 106;
pub const OBJECT_HAKA_OBJECTS: C2RustUnnamed_17 = 105;
pub const OBJECT_MJIN_OKA: C2RustUnnamed_17 = 104;
pub const OBJECT_MJIN_WIND: C2RustUnnamed_17 = 103;
pub const OBJECT_MJIN_SOUL: C2RustUnnamed_17 = 102;
pub const OBJECT_MJIN_ICE: C2RustUnnamed_17 = 101;
pub const OBJECT_MJIN_FLAME: C2RustUnnamed_17 = 100;
pub const OBJECT_MJIN_DARK: C2RustUnnamed_17 = 99;
pub const OBJECT_MJIN_FLASH: C2RustUnnamed_17 = 98;
pub const OBJECT_MJIN: C2RustUnnamed_17 = 97;
pub const OBJECT_ZL2: C2RustUnnamed_17 = 96;
pub const OBJECT_YUKABYUN: C2RustUnnamed_17 = 95;
pub const OBJECT_TOKI_OBJECTS: C2RustUnnamed_17 = 94;
pub const OBJECT_BB: C2RustUnnamed_17 = 93;
pub const OBJECT_MORI_HINERI1: C2RustUnnamed_17 = 92;
pub const OBJECT_OSSAN: C2RustUnnamed_17 = 91;
pub const OBJECT_FHG: C2RustUnnamed_17 = 90;
pub const OBJECT_MIZU_OBJECTS: C2RustUnnamed_17 = 89;
pub const OBJECT_OA11: C2RustUnnamed_17 = 88;
pub const OBJECT_OA10: C2RustUnnamed_17 = 87;
pub const OBJECT_VALI: C2RustUnnamed_17 = 86;
pub const OBJECT_OE12: C2RustUnnamed_17 = 85;
pub const OBJECT_OE11: C2RustUnnamed_17 = 84;
pub const OBJECT_OE10: C2RustUnnamed_17 = 83;
pub const OBJECT_OE9: C2RustUnnamed_17 = 82;
pub const OBJECT_OE8: C2RustUnnamed_17 = 81;
pub const OBJECT_OE7: C2RustUnnamed_17 = 80;
pub const OBJECT_OE6: C2RustUnnamed_17 = 79;
pub const OBJECT_OE5: C2RustUnnamed_17 = 78;
pub const OBJECT_MENKURI_OBJECTS: C2RustUnnamed_17 = 77;
pub const OBJECT_OE4: C2RustUnnamed_17 = 76;
pub const OBJECT_OE3: C2RustUnnamed_17 = 75;
pub const OBJECT_DEKUNUTS: C2RustUnnamed_17 = 74;
pub const OBJECT_B_HEART: C2RustUnnamed_17 = 73;
pub const OBJECT_WARP1: C2RustUnnamed_17 = 72;
pub const OBJECT_OPENING_DEMO1: C2RustUnnamed_17 = 71;
pub const OBJECT_HORSE_ZELDA: C2RustUnnamed_17 = 70;
pub const OBJECT_OB4: C2RustUnnamed_17 = 69;
pub const OBJECT_OB3: C2RustUnnamed_17 = 68;
pub const OBJECT_OB2: C2RustUnnamed_17 = 67;
pub const OBJECT_OA9: C2RustUnnamed_17 = 66;
pub const OBJECT_OA8: C2RustUnnamed_17 = 65;
pub const OBJECT_JJ: C2RustUnnamed_17 = 64;
pub const OBJECT_OA7: C2RustUnnamed_17 = 63;
pub const OBJECT_OA6: C2RustUnnamed_17 = 62;
pub const OBJECT_OA5: C2RustUnnamed_17 = 61;
pub const OBJECT_OA4: C2RustUnnamed_17 = 60;
pub const OBJECT_OA3: C2RustUnnamed_17 = 59;
pub const OBJECT_UNSET_3A: C2RustUnnamed_17 = 58;
pub const OBJECT_DEKUBABA: C2RustUnnamed_17 = 57;
pub const OBJECT_AM: C2RustUnnamed_17 = 56;
pub const OBJECT_GND: C2RustUnnamed_17 = 55;
pub const OBJECT_YDAN_OBJECTS: C2RustUnnamed_17 = 54;
pub const OBJECT_OE2: C2RustUnnamed_17 = 53;
pub const OBJECT_OE_ANIME: C2RustUnnamed_17 = 52;
pub const OBJECT_OE1: C2RustUnnamed_17 = 51;
pub const OBJECT_SK2: C2RustUnnamed_17 = 50;
pub const OBJECT_BOMBF: C2RustUnnamed_17 = 49;
pub const OBJECT_MB: C2RustUnnamed_17 = 48;
pub const OBJECT_SPOT00_OBJECTS: C2RustUnnamed_17 = 47;
pub const OBJECT_OA2: C2RustUnnamed_17 = 46;
pub const OBJECT_HORSE_GANON: C2RustUnnamed_17 = 45;
pub const OBJECT_HIDAN_OBJECTS: C2RustUnnamed_17 = 44;
pub const OBJECT_DDAN_OBJECTS: C2RustUnnamed_17 = 43;
pub const OBJECT_SPOT04_OBJECTS: C2RustUnnamed_17 = 42;
pub const OBJECT_O_ANIME: C2RustUnnamed_17 = 41;
pub const OBJECT_OB1: C2RustUnnamed_17 = 40;
pub const OBJECT_HORSE_NORMAL: C2RustUnnamed_17 = 39;
pub const OBJECT_EI: C2RustUnnamed_17 = 38;
pub const OBJECT_BW: C2RustUnnamed_17 = 37;
pub const OBJECT_ST: C2RustUnnamed_17 = 36;
pub const OBJECT_OA1: C2RustUnnamed_17 = 35;
pub const OBJECT_TP: C2RustUnnamed_17 = 34;
pub const OBJECT_BL: C2RustUnnamed_17 = 33;
pub const OBJECT_TORCH2: C2RustUnnamed_17 = 32;
pub const OBJECT_DODOJR: C2RustUnnamed_17 = 31;
pub const OBJECT_GOL: C2RustUnnamed_17 = 30;
pub const OBJECT_ZL1: C2RustUnnamed_17 = 29;
pub const OBJECT_GOMA: C2RustUnnamed_17 = 28;
pub const OBJECT_ZF: C2RustUnnamed_17 = 27;
pub const OBJECT_HORSE: C2RustUnnamed_17 = 26;
pub const OBJECT_KINGDODONGO: C2RustUnnamed_17 = 25;
pub const OBJECT_PEEHAT: C2RustUnnamed_17 = 24;
pub const OBJECT_REEBA: C2RustUnnamed_17 = 23;
pub const OBJECT_TITE: C2RustUnnamed_17 = 22;
pub const OBJECT_LINK_CHILD: C2RustUnnamed_17 = 21;
pub const OBJECT_LINK_BOY: C2RustUnnamed_17 = 20;
pub const OBJECT_NIW: C2RustUnnamed_17 = 19;
pub const OBJECT_BUBBLE: C2RustUnnamed_17 = 18;
pub const OBJECT_UNSET_11: C2RustUnnamed_17 = 17;
pub const OBJECT_UNSET_10: C2RustUnnamed_17 = 16;
pub const OBJECT_FIRE: C2RustUnnamed_17 = 15;
pub const OBJECT_BOX: C2RustUnnamed_17 = 14;
pub const OBJECT_FIREFLY: C2RustUnnamed_17 = 13;
pub const OBJECT_DODONGO: C2RustUnnamed_17 = 12;
pub const OBJECT_WALLMASTER: C2RustUnnamed_17 = 11;
pub const OBJECT_DY_OBJ: C2RustUnnamed_17 = 10;
pub const OBJECT_POH: C2RustUnnamed_17 = 9;
pub const OBJECT_CROW: C2RustUnnamed_17 = 8;
pub const OBJECT_OKUTA: C2RustUnnamed_17 = 7;
pub const OBJECT_HUMAN: C2RustUnnamed_17 = 6;
pub const OBJECT_UNSET_5: C2RustUnnamed_17 = 5;
pub const OBJECT_UNSET_4: C2RustUnnamed_17 = 4;
pub const OBJECT_GAMEPLAY_DANGEON_KEEP: C2RustUnnamed_17 = 3;
pub const OBJECT_GAMEPLAY_FIELD_KEEP: C2RustUnnamed_17 = 2;
pub const OBJECT_GAMEPLAY_KEEP: C2RustUnnamed_17 = 1;
pub const OBJECT_INVALID: C2RustUnnamed_17 = 0;
pub type C2RustUnnamed_18 = libc::c_uint;
pub const LIGHTNING_MODE_LAST: C2RustUnnamed_18 = 2;
pub const LIGHTNING_MODE_ON: C2RustUnnamed_18 = 1;
pub const LIGHTNING_MODE_OFF: C2RustUnnamed_18 = 0;
pub type C2RustUnnamed_19 = libc::c_uint;
pub const LIGHTNING_STRIKE_END: C2RustUnnamed_19 = 2;
pub const LIGHTNING_STRIKE_START: C2RustUnnamed_19 = 1;
pub const LIGHTNING_STRIKE_WAIT: C2RustUnnamed_19 = 0;
pub type C2RustUnnamed_20 = libc::c_uint;
pub const SKYBOX_DMA_PAL2_START: C2RustUnnamed_20 = 13;
pub const SKYBOX_DMA_FILE2_DONE: C2RustUnnamed_20 = 12;
pub const SKYBOX_DMA_FILE2_START: C2RustUnnamed_20 = 11;
pub const SKYBOX_DMA_PAL1_START: C2RustUnnamed_20 = 3;
pub const SKYBOX_DMA_FILE1_DONE: C2RustUnnamed_20 = 2;
pub const SKYBOX_DMA_FILE1_START: C2RustUnnamed_20 = 1;
pub const SKYBOX_DMA_INACTIVE: C2RustUnnamed_20 = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct struct_8011FC1C {
    pub startTime: u16_0,
    pub endTime: u16_0,
    pub blend: u8_0,
    pub skybox1Index: u8_0,
    pub skybox2Index: u8_0,
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
pub type C2RustUnnamed_23 = libc::c_uint;
pub const SEQ_PLAYER_BGM_SUB: C2RustUnnamed_23 = 3;
pub const SEQ_PLAYER_SFX: C2RustUnnamed_23 = 2;
pub const SEQ_PLAYER_FANFARE: C2RustUnnamed_23 = 1;
pub const SEQ_PLAYER_BGM_MAIN: C2RustUnnamed_23 = 0;
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
pub const SKYBOX_UNSET_27: C2RustUnnamed_24 = 39;
pub const SKYBOX_HOUSE_ALLEY: C2RustUnnamed_24 = 34;
pub const SKYBOX_HOUSE_SARIA: C2RustUnnamed_24 = 33;
pub const SKYBOX_HOUSE_MIDO: C2RustUnnamed_24 = 32;
pub const SKYBOX_UNSET_1D: C2RustUnnamed_24 = 29;
pub const SKYBOX_TENT: C2RustUnnamed_24 = 28;
pub const SKYBOX_HOUSE_IMPA: C2RustUnnamed_24 = 27;
pub const SKYBOX_HOUSE_RICHARD: C2RustUnnamed_24 = 26;
pub const SKYBOX_BOMBCHU_SHOP: C2RustUnnamed_24 = 24;
pub const SKYBOX_POTION_SHOP_MARKET: C2RustUnnamed_24 = 23;
pub const SKYBOX_POTION_SHOP_KAKARIKO: C2RustUnnamed_24 = 22;
pub const SKYBOX_ZORA_SHOP: C2RustUnnamed_24 = 20;
pub const SKYBOX_GORON_SHOP: C2RustUnnamed_24 = 19;
pub const SKYBOX_KOKIRI_SHOP: C2RustUnnamed_24 = 17;
pub const SKYBOX_HOUSE_KAKARIKO: C2RustUnnamed_24 = 16;
pub const SKYBOX_STABLES: C2RustUnnamed_24 = 15;
pub const SKYBOX_HOUSE_OF_TWINS: C2RustUnnamed_24 = 14;
pub const SKYBOX_HOUSE_KNOW_IT_ALL_BROTHERS: C2RustUnnamed_24 = 12;
pub const SKYBOX_HAPPY_MASK_SHOP: C2RustUnnamed_24 = 11;
pub const SKYBOX_MARKET_CHILD_NIGHT: C2RustUnnamed_24 = 10;
pub const SKYBOX_MARKET_CHILD_DAY: C2RustUnnamed_24 = 9;
pub const SKYBOX_HOUSE_LINK: C2RustUnnamed_24 = 7;
pub const SKYBOX_CUTSCENE_MAP: C2RustUnnamed_24 = 5;
pub const SKYBOX_MARKET_ADULT: C2RustUnnamed_24 = 4;
pub const SKYBOX_OVERCAST_SUNSET: C2RustUnnamed_24 = 3;
pub const SKYBOX_BAZAAR: C2RustUnnamed_24 = 2;
pub const SKYBOX_NORMAL_SKY: C2RustUnnamed_24 = 1;
pub const SKYBOX_NONE: C2RustUnnamed_24 = 0;
pub type C2RustUnnamed_25 = libc::c_uint;
pub const GAMEOVER_REVIVE_FADE_OUT: C2RustUnnamed_25 = 24;
pub const GAMEOVER_REVIVE_WAIT_FAIRY: C2RustUnnamed_25 = 23;
pub const GAMEOVER_REVIVE_WAIT_GROUND: C2RustUnnamed_25 = 22;
pub const GAMEOVER_REVIVE_RUMBLE: C2RustUnnamed_25 = 21;
pub const GAMEOVER_REVIVE_START: C2RustUnnamed_25 = 20;
pub const GAMEOVER_DEATH_MENU: C2RustUnnamed_25 = 4;
pub const GAMEOVER_DEATH_DELAY_MENU: C2RustUnnamed_25 = 3;
pub const GAMEOVER_DEATH_WAIT_GROUND: C2RustUnnamed_25 = 2;
pub const GAMEOVER_DEATH_START: C2RustUnnamed_25 = 1;
pub const GAMEOVER_INACTIVE: C2RustUnnamed_25 = 0;
pub type C2RustUnnamed_26 = libc::c_uint;
pub const MTXMODE_APPLY: C2RustUnnamed_26 = 1;
pub const MTXMODE_NEW: C2RustUnnamed_26 = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SkyboxFile {
    pub file: RomFile,
    pub palette: RomFile,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Struct_8011FAF0 {
    pub unk0: s32,
    pub unk1: s32,
}
pub const LIGHTNING_BOLT_INACTIVE: C2RustUnnamed_28 = 255;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LightningBolt {
    pub state: u8_0,
    pub offset: Vec3f,
    pub pos: Vec3f,
    pub pitch: s8,
    pub roll: s8,
    pub textureIndex: u8_0,
    pub delayTimer: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct struct_8011FB48 {
    pub startTime: u16_0,
    pub endTime: u16_0,
    pub unk_04: u8_0,
    pub unk_05: u8_0,
}
pub const LENS_FLARE_RING: C2RustUnnamed_27 = 2;
pub const LENS_FLARE_CIRCLE1: C2RustUnnamed_27 = 1;
pub const LENS_FLARE_CIRCLE0: C2RustUnnamed_27 = 0;
pub const LIGHTNING_BOLT_START: C2RustUnnamed_28 = 0;
pub const LIGHTNING_BOLT_DRAW: C2RustUnnamed_28 = 2;
pub const LIGHTNING_BOLT_WAIT: C2RustUnnamed_28 = 1;
pub type C2RustUnnamed_27 = libc::c_uint;
pub type C2RustUnnamed_28 = libc::c_uint;
// size = 0x8
#[no_mangle]
pub static mut D_8011FAF0: [Struct_8011FAF0; 8] =
    [{
         let mut init =
             Struct_8011FAF0{unk0: 6 as libc::c_int, unk1: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             Struct_8011FAF0{unk0: 5 as libc::c_int,
                             unk1: 0x20000 as libc::c_int,};
         init
     },
     {
         let mut init =
             Struct_8011FAF0{unk0: 4 as libc::c_int,
                             unk1: 0x30000 as libc::c_int,};
         init
     },
     {
         let mut init =
             Struct_8011FAF0{unk0: 3 as libc::c_int,
                             unk1: 0x38000 as libc::c_int,};
         init
     },
     {
         let mut init =
             Struct_8011FAF0{unk0: 2 as libc::c_int,
                             unk1: 0x3c000 as libc::c_int,};
         init
     },
     {
         let mut init =
             Struct_8011FAF0{unk0: 1 as libc::c_int,
                             unk1: 0x3e000 as libc::c_int,};
         init
     },
     {
         let mut init =
             Struct_8011FAF0{unk0: 0 as libc::c_int,
                             unk1: 0x3f000 as libc::c_int,};
         init
     },
     {
         let mut init =
             Struct_8011FAF0{unk0: 0 as libc::c_int,
                             unk1: 0x3f800 as libc::c_int,};
         init
     }];
#[no_mangle]
pub static mut gWeatherMode: u8_0 = 0 as libc::c_int as u8_0;
// "E_wether_flg"
#[no_mangle]
pub static mut D_8011FB34: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut D_8011FB38: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut gSkyboxBlendingEnabled: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut gTimeIncrement: u16_0 = 0 as libc::c_int as u16_0;
#[no_mangle]
pub static mut D_8011FB44: u16_0 = 0xfffc as libc::c_int as u16_0;
#[no_mangle]
pub static mut D_8011FB48: [[struct_8011FB48; 7]; 5] =
    [[{
          let mut init =
              struct_8011FB48{startTime: 0 as libc::c_int as u16_0,
                              endTime: 0x2aac as libc::c_int as u16_0,
                              unk_04: 3 as libc::c_int as u8_0,
                              unk_05: 3 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FB48{startTime: 0x2aac as libc::c_int as u16_0,
                              endTime: 0x4000 as libc::c_int as u16_0,
                              unk_04: 3 as libc::c_int as u8_0,
                              unk_05: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FB48{startTime: 0x4000 as libc::c_int as u16_0,
                              endTime: 0x5556 as libc::c_int as u16_0,
                              unk_04: 0 as libc::c_int as u8_0,
                              unk_05: 1 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FB48{startTime: 0x5556 as libc::c_int as u16_0,
                              endTime: 0xaaab as libc::c_int as u16_0,
                              unk_04: 1 as libc::c_int as u8_0,
                              unk_05: 1 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FB48{startTime: 0xaaab as libc::c_int as u16_0,
                              endTime: 0xb556 as libc::c_int as u16_0,
                              unk_04: 1 as libc::c_int as u8_0,
                              unk_05: 2 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FB48{startTime: 0xb556 as libc::c_int as u16_0,
                              endTime: 0xcaac as libc::c_int as u16_0,
                              unk_04: 2 as libc::c_int as u8_0,
                              unk_05: 3 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FB48{startTime: 0xcaac as libc::c_int as u16_0,
                              endTime: 0xffff as libc::c_int as u16_0,
                              unk_04: 3 as libc::c_int as u8_0,
                              unk_05: 3 as libc::c_int as u8_0,};
          init
      }],
     [{
          let mut init =
              struct_8011FB48{startTime: 0 as libc::c_int as u16_0,
                              endTime: 0x2aac as libc::c_int as u16_0,
                              unk_04: 7 as libc::c_int as u8_0,
                              unk_05: 7 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FB48{startTime: 0x2aac as libc::c_int as u16_0,
                              endTime: 0x4000 as libc::c_int as u16_0,
                              unk_04: 7 as libc::c_int as u8_0,
                              unk_05: 4 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FB48{startTime: 0x4000 as libc::c_int as u16_0,
                              endTime: 0x5556 as libc::c_int as u16_0,
                              unk_04: 4 as libc::c_int as u8_0,
                              unk_05: 5 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FB48{startTime: 0x5556 as libc::c_int as u16_0,
                              endTime: 0xaaab as libc::c_int as u16_0,
                              unk_04: 5 as libc::c_int as u8_0,
                              unk_05: 5 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FB48{startTime: 0xaaab as libc::c_int as u16_0,
                              endTime: 0xb556 as libc::c_int as u16_0,
                              unk_04: 5 as libc::c_int as u8_0,
                              unk_05: 6 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FB48{startTime: 0xb556 as libc::c_int as u16_0,
                              endTime: 0xcaac as libc::c_int as u16_0,
                              unk_04: 6 as libc::c_int as u8_0,
                              unk_05: 7 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FB48{startTime: 0xcaac as libc::c_int as u16_0,
                              endTime: 0xffff as libc::c_int as u16_0,
                              unk_04: 7 as libc::c_int as u8_0,
                              unk_05: 7 as libc::c_int as u8_0,};
          init
      }],
     [{
          let mut init =
              struct_8011FB48{startTime: 0 as libc::c_int as u16_0,
                              endTime: 0x2aac as libc::c_int as u16_0,
                              unk_04: 11 as libc::c_int as u8_0,
                              unk_05: 11 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FB48{startTime: 0x2aac as libc::c_int as u16_0,
                              endTime: 0x4000 as libc::c_int as u16_0,
                              unk_04: 11 as libc::c_int as u8_0,
                              unk_05: 8 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FB48{startTime: 0x4000 as libc::c_int as u16_0,
                              endTime: 0x5556 as libc::c_int as u16_0,
                              unk_04: 8 as libc::c_int as u8_0,
                              unk_05: 9 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FB48{startTime: 0x5556 as libc::c_int as u16_0,
                              endTime: 0xaaab as libc::c_int as u16_0,
                              unk_04: 9 as libc::c_int as u8_0,
                              unk_05: 9 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FB48{startTime: 0xaaab as libc::c_int as u16_0,
                              endTime: 0xb556 as libc::c_int as u16_0,
                              unk_04: 9 as libc::c_int as u8_0,
                              unk_05: 10 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FB48{startTime: 0xb556 as libc::c_int as u16_0,
                              endTime: 0xcaac as libc::c_int as u16_0,
                              unk_04: 10 as libc::c_int as u8_0,
                              unk_05: 11 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FB48{startTime: 0xcaac as libc::c_int as u16_0,
                              endTime: 0xffff as libc::c_int as u16_0,
                              unk_04: 11 as libc::c_int as u8_0,
                              unk_05: 11 as libc::c_int as u8_0,};
          init
      }],
     [{
          let mut init =
              struct_8011FB48{startTime: 0 as libc::c_int as u16_0,
                              endTime: 0x2aac as libc::c_int as u16_0,
                              unk_04: 15 as libc::c_int as u8_0,
                              unk_05: 15 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FB48{startTime: 0x2aac as libc::c_int as u16_0,
                              endTime: 0x4000 as libc::c_int as u16_0,
                              unk_04: 15 as libc::c_int as u8_0,
                              unk_05: 12 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FB48{startTime: 0x4000 as libc::c_int as u16_0,
                              endTime: 0x5556 as libc::c_int as u16_0,
                              unk_04: 12 as libc::c_int as u8_0,
                              unk_05: 13 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FB48{startTime: 0x5556 as libc::c_int as u16_0,
                              endTime: 0xaaab as libc::c_int as u16_0,
                              unk_04: 13 as libc::c_int as u8_0,
                              unk_05: 13 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FB48{startTime: 0xaaab as libc::c_int as u16_0,
                              endTime: 0xb556 as libc::c_int as u16_0,
                              unk_04: 13 as libc::c_int as u8_0,
                              unk_05: 14 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FB48{startTime: 0xb556 as libc::c_int as u16_0,
                              endTime: 0xcaac as libc::c_int as u16_0,
                              unk_04: 14 as libc::c_int as u8_0,
                              unk_05: 15 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FB48{startTime: 0xcaac as libc::c_int as u16_0,
                              endTime: 0xffff as libc::c_int as u16_0,
                              unk_04: 15 as libc::c_int as u8_0,
                              unk_05: 15 as libc::c_int as u8_0,};
          init
      }],
     [{
          let mut init =
              struct_8011FB48{startTime: 0 as libc::c_int as u16_0,
                              endTime: 0x2aac as libc::c_int as u16_0,
                              unk_04: 23 as libc::c_int as u8_0,
                              unk_05: 23 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FB48{startTime: 0x2aac as libc::c_int as u16_0,
                              endTime: 0x4000 as libc::c_int as u16_0,
                              unk_04: 23 as libc::c_int as u8_0,
                              unk_05: 20 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FB48{startTime: 0x4000 as libc::c_int as u16_0,
                              endTime: 0x5556 as libc::c_int as u16_0,
                              unk_04: 20 as libc::c_int as u8_0,
                              unk_05: 21 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FB48{startTime: 0x5556 as libc::c_int as u16_0,
                              endTime: 0xaaab as libc::c_int as u16_0,
                              unk_04: 21 as libc::c_int as u8_0,
                              unk_05: 21 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FB48{startTime: 0xaaab as libc::c_int as u16_0,
                              endTime: 0xb556 as libc::c_int as u16_0,
                              unk_04: 21 as libc::c_int as u8_0,
                              unk_05: 22 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FB48{startTime: 0xb556 as libc::c_int as u16_0,
                              endTime: 0xcaac as libc::c_int as u16_0,
                              unk_04: 22 as libc::c_int as u8_0,
                              unk_05: 23 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FB48{startTime: 0xcaac as libc::c_int as u16_0,
                              endTime: 0xffff as libc::c_int as u16_0,
                              unk_04: 23 as libc::c_int as u8_0,
                              unk_05: 23 as libc::c_int as u8_0,};
          init
      }]];
#[no_mangle]
pub static mut D_8011FC1C: [[struct_8011FC1C; 9]; 4] =
    [[{
          let mut init =
              struct_8011FC1C{startTime: 0 as libc::c_int as u16_0,
                              endTime: 0x2aac as libc::c_int as u16_0,
                              blend: 0 as libc::c_int as u8_0,
                              skybox1Index: 3 as libc::c_int as u8_0,
                              skybox2Index: 3 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FC1C{startTime: 0x2aac as libc::c_int as u16_0,
                              endTime: 0x3556 as libc::c_int as u16_0,
                              blend: 1 as libc::c_int as u8_0,
                              skybox1Index: 3 as libc::c_int as u8_0,
                              skybox2Index: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FC1C{startTime: 0x3556 as libc::c_int as u16_0,
                              endTime: 0x4000 as libc::c_int as u16_0,
                              blend: 0 as libc::c_int as u8_0,
                              skybox1Index: 0 as libc::c_int as u8_0,
                              skybox2Index: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FC1C{startTime: 0x4000 as libc::c_int as u16_0,
                              endTime: 0x5556 as libc::c_int as u16_0,
                              blend: 1 as libc::c_int as u8_0,
                              skybox1Index: 0 as libc::c_int as u8_0,
                              skybox2Index: 1 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FC1C{startTime: 0x5556 as libc::c_int as u16_0,
                              endTime: 0xaaab as libc::c_int as u16_0,
                              blend: 0 as libc::c_int as u8_0,
                              skybox1Index: 1 as libc::c_int as u8_0,
                              skybox2Index: 1 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FC1C{startTime: 0xaaab as libc::c_int as u16_0,
                              endTime: 0xb556 as libc::c_int as u16_0,
                              blend: 1 as libc::c_int as u8_0,
                              skybox1Index: 1 as libc::c_int as u8_0,
                              skybox2Index: 2 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FC1C{startTime: 0xb556 as libc::c_int as u16_0,
                              endTime: 0xc001 as libc::c_int as u16_0,
                              blend: 0 as libc::c_int as u8_0,
                              skybox1Index: 2 as libc::c_int as u8_0,
                              skybox2Index: 2 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FC1C{startTime: 0xc001 as libc::c_int as u16_0,
                              endTime: 0xcaac as libc::c_int as u16_0,
                              blend: 1 as libc::c_int as u8_0,
                              skybox1Index: 2 as libc::c_int as u8_0,
                              skybox2Index: 3 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FC1C{startTime: 0xcaac as libc::c_int as u16_0,
                              endTime: 0xffff as libc::c_int as u16_0,
                              blend: 0 as libc::c_int as u8_0,
                              skybox1Index: 3 as libc::c_int as u8_0,
                              skybox2Index: 3 as libc::c_int as u8_0,};
          init
      }],
     [{
          let mut init =
              struct_8011FC1C{startTime: 0 as libc::c_int as u16_0,
                              endTime: 0x2aac as libc::c_int as u16_0,
                              blend: 0 as libc::c_int as u8_0,
                              skybox1Index: 7 as libc::c_int as u8_0,
                              skybox2Index: 7 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FC1C{startTime: 0x2aac as libc::c_int as u16_0,
                              endTime: 0x3556 as libc::c_int as u16_0,
                              blend: 1 as libc::c_int as u8_0,
                              skybox1Index: 7 as libc::c_int as u8_0,
                              skybox2Index: 4 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FC1C{startTime: 0x3556 as libc::c_int as u16_0,
                              endTime: 0x4000 as libc::c_int as u16_0,
                              blend: 0 as libc::c_int as u8_0,
                              skybox1Index: 4 as libc::c_int as u8_0,
                              skybox2Index: 4 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FC1C{startTime: 0x4000 as libc::c_int as u16_0,
                              endTime: 0x5556 as libc::c_int as u16_0,
                              blend: 1 as libc::c_int as u8_0,
                              skybox1Index: 4 as libc::c_int as u8_0,
                              skybox2Index: 5 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FC1C{startTime: 0x5556 as libc::c_int as u16_0,
                              endTime: 0xaaab as libc::c_int as u16_0,
                              blend: 0 as libc::c_int as u8_0,
                              skybox1Index: 5 as libc::c_int as u8_0,
                              skybox2Index: 5 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FC1C{startTime: 0xaaab as libc::c_int as u16_0,
                              endTime: 0xb556 as libc::c_int as u16_0,
                              blend: 1 as libc::c_int as u8_0,
                              skybox1Index: 5 as libc::c_int as u8_0,
                              skybox2Index: 6 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FC1C{startTime: 0xb556 as libc::c_int as u16_0,
                              endTime: 0xc001 as libc::c_int as u16_0,
                              blend: 0 as libc::c_int as u8_0,
                              skybox1Index: 6 as libc::c_int as u8_0,
                              skybox2Index: 6 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FC1C{startTime: 0xc001 as libc::c_int as u16_0,
                              endTime: 0xcaac as libc::c_int as u16_0,
                              blend: 1 as libc::c_int as u8_0,
                              skybox1Index: 6 as libc::c_int as u8_0,
                              skybox2Index: 7 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FC1C{startTime: 0xcaac as libc::c_int as u16_0,
                              endTime: 0xffff as libc::c_int as u16_0,
                              blend: 0 as libc::c_int as u8_0,
                              skybox1Index: 7 as libc::c_int as u8_0,
                              skybox2Index: 7 as libc::c_int as u8_0,};
          init
      }],
     [{
          let mut init =
              struct_8011FC1C{startTime: 0 as libc::c_int as u16_0,
                              endTime: 0x1556 as libc::c_int as u16_0,
                              blend: 0 as libc::c_int as u8_0,
                              skybox1Index: 3 as libc::c_int as u8_0,
                              skybox2Index: 3 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FC1C{startTime: 0x1556 as libc::c_int as u16_0,
                              endTime: 0x2aac as libc::c_int as u16_0,
                              blend: 1 as libc::c_int as u8_0,
                              skybox1Index: 3 as libc::c_int as u8_0,
                              skybox2Index: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FC1C{startTime: 0x2aac as libc::c_int as u16_0,
                              endTime: 0x5556 as libc::c_int as u16_0,
                              blend: 0 as libc::c_int as u8_0,
                              skybox1Index: 0 as libc::c_int as u8_0,
                              skybox2Index: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FC1C{startTime: 0x5556 as libc::c_int as u16_0,
                              endTime: 0x6aab as libc::c_int as u16_0,
                              blend: 1 as libc::c_int as u8_0,
                              skybox1Index: 0 as libc::c_int as u8_0,
                              skybox2Index: 1 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FC1C{startTime: 0x6aab as libc::c_int as u16_0,
                              endTime: 0x9556 as libc::c_int as u16_0,
                              blend: 0 as libc::c_int as u8_0,
                              skybox1Index: 1 as libc::c_int as u8_0,
                              skybox2Index: 1 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FC1C{startTime: 0x9556 as libc::c_int as u16_0,
                              endTime: 0xaaab as libc::c_int as u16_0,
                              blend: 1 as libc::c_int as u8_0,
                              skybox1Index: 1 as libc::c_int as u8_0,
                              skybox2Index: 2 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FC1C{startTime: 0xaaab as libc::c_int as u16_0,
                              endTime: 0xd556 as libc::c_int as u16_0,
                              blend: 0 as libc::c_int as u8_0,
                              skybox1Index: 2 as libc::c_int as u8_0,
                              skybox2Index: 2 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FC1C{startTime: 0xd556 as libc::c_int as u16_0,
                              endTime: 0xeaab as libc::c_int as u16_0,
                              blend: 1 as libc::c_int as u8_0,
                              skybox1Index: 2 as libc::c_int as u8_0,
                              skybox2Index: 3 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FC1C{startTime: 0xeaab as libc::c_int as u16_0,
                              endTime: 0xffff as libc::c_int as u16_0,
                              blend: 0 as libc::c_int as u8_0,
                              skybox1Index: 3 as libc::c_int as u8_0,
                              skybox2Index: 3 as libc::c_int as u8_0,};
          init
      }],
     [{
          let mut init =
              struct_8011FC1C{startTime: 0 as libc::c_int as u16_0,
                              endTime: 0x3556 as libc::c_int as u16_0,
                              blend: 0 as libc::c_int as u8_0,
                              skybox1Index: 11 as libc::c_int as u8_0,
                              skybox2Index: 11 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FC1C{startTime: 0x3556 as libc::c_int as u16_0,
                              endTime: 0x4000 as libc::c_int as u16_0,
                              blend: 1 as libc::c_int as u8_0,
                              skybox1Index: 11 as libc::c_int as u8_0,
                              skybox2Index: 8 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FC1C{startTime: 0x4000 as libc::c_int as u16_0,
                              endTime: 0x4aab as libc::c_int as u16_0,
                              blend: 0 as libc::c_int as u8_0,
                              skybox1Index: 8 as libc::c_int as u8_0,
                              skybox2Index: 8 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FC1C{startTime: 0x4aab as libc::c_int as u16_0,
                              endTime: 0x5556 as libc::c_int as u16_0,
                              blend: 1 as libc::c_int as u8_0,
                              skybox1Index: 8 as libc::c_int as u8_0,
                              skybox2Index: 9 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FC1C{startTime: 0x5556 as libc::c_int as u16_0,
                              endTime: 0xaaab as libc::c_int as u16_0,
                              blend: 0 as libc::c_int as u8_0,
                              skybox1Index: 9 as libc::c_int as u8_0,
                              skybox2Index: 9 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FC1C{startTime: 0xaaab as libc::c_int as u16_0,
                              endTime: 0xb556 as libc::c_int as u16_0,
                              blend: 1 as libc::c_int as u8_0,
                              skybox1Index: 9 as libc::c_int as u8_0,
                              skybox2Index: 10 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FC1C{startTime: 0xb556 as libc::c_int as u16_0,
                              endTime: 0xc001 as libc::c_int as u16_0,
                              blend: 0 as libc::c_int as u8_0,
                              skybox1Index: 10 as libc::c_int as u8_0,
                              skybox2Index: 10 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FC1C{startTime: 0xc001 as libc::c_int as u16_0,
                              endTime: 0xcaac as libc::c_int as u16_0,
                              blend: 1 as libc::c_int as u8_0,
                              skybox1Index: 10 as libc::c_int as u8_0,
                              skybox2Index: 11 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              struct_8011FC1C{startTime: 0xcaac as libc::c_int as u16_0,
                              endTime: 0xffff as libc::c_int as u16_0,
                              blend: 0 as libc::c_int as u8_0,
                              skybox1Index: 11 as libc::c_int as u8_0,
                              skybox2Index: 11 as libc::c_int as u8_0,};
          init
      }]];
// Initialized in run_static_initializers
#[no_mangle]
pub static mut gSkyboxFiles: [SkyboxFile; 9] =
    [SkyboxFile{file: RomFile{vromStart: 0, vromEnd: 0,},
                palette: RomFile{vromStart: 0, vromEnd: 0,},}; 9];
#[no_mangle]
pub static mut D_8011FDCC: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut D_8011FDD0: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut D_8011FDD4: f32_0 = 0.0f32;
#[no_mangle]
pub static mut gCustomLensFlareOn: u8_0 = 0;
#[no_mangle]
pub static mut gCustomLensFlarePos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
#[no_mangle]
pub static mut gLensFlareUnused: s16 = 0;
#[no_mangle]
pub static mut gLensFlareScale: s16 = 0;
#[no_mangle]
pub static mut gLensFlareColorIntensity: f32_0 = 0.;
#[no_mangle]
pub static mut gLensFlareScreenFillAlpha: s16 = 0;
#[no_mangle]
pub static mut sLightningBolts: [LightningBolt; 3] =
    [LightningBolt{state: 0,
                   offset: Vec3f{x: 0., y: 0., z: 0.,},
                   pos: Vec3f{x: 0., y: 0., z: 0.,},
                   pitch: 0,
                   roll: 0,
                   textureIndex: 0,
                   delayTimer: 0,}; 3];
#[no_mangle]
pub static mut gLightningStrike: LightningStrike =
    LightningStrike{state: 0,
                    flashRed: 0,
                    flashGreen: 0,
                    flashBlue: 0,
                    flashAlphaTarget: 0,
                    delayTimer: 0.,};
#[no_mangle]
pub static mut sLightningFlashAlpha: s16 = 0;
#[no_mangle]
pub static mut D_8015FD7E: s16 = 0;
#[no_mangle]
pub static mut D_8015FD80: s16 = 0;
#[no_mangle]
pub static mut sNGameOverLightNode: *mut LightNode =
    0 as *const LightNode as *mut LightNode;
#[no_mangle]
pub static mut sNGameOverLightInfo: LightInfo =
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
pub static mut sSGameOverLightNode: *mut LightNode =
    0 as *const LightNode as *mut LightNode;
#[no_mangle]
pub static mut sSGameOverLightInfo: LightInfo =
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
pub static mut sGameOverLightsIntensity: u8_0 = 0;
#[no_mangle]
pub static mut D_8015FDB0: u16_0 = 0;
#[no_mangle]
pub unsafe extern "C" fn func_8006F0A0(mut a0: s32) -> s32 {
    let mut ret: s32 =
        ((a0 >> 4 as libc::c_int & 0x7ff as libc::c_int) <<
             D_8011FAF0[(a0 >> 15 as libc::c_int & 7 as libc::c_int) as
                            usize].unk0) +
            D_8011FAF0[(a0 >> 15 as libc::c_int & 7 as libc::c_int) as
                           usize].unk1;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn Environment_GetPixelDepth(mut x: s32, mut y: s32)
 -> u16_0 {
    let mut pixelDepth: s32 = gZBuffer[y as usize][x as usize] as s32;
    return pixelDepth as u16_0;
}
#[no_mangle]
pub unsafe extern "C" fn Environment_GraphCallback(mut gfxCtx:
                                                       *mut GraphicsContext,
                                                   mut param:
                                                       *mut libc::c_void) {
    let mut globalCtx: *mut GlobalContext = param as *mut GlobalContext;
    D_8011FB44 =
        Environment_GetPixelDepth(D_8015FD7E as s32, D_8015FD80 as s32);
    Lights_GlowCheck(globalCtx);
}
#[no_mangle]
pub unsafe extern "C" fn Environment_Init(mut globalCtx2: *mut GlobalContext,
                                          mut envCtx: *mut EnvironmentContext,
                                          mut unused: s32) {
    let mut i: u8_0 = 0;
    let mut globalCtx: *mut GlobalContext = globalCtx2;
    gSaveContext.sunsSongState = SUNSSONG_INACTIVE as libc::c_int as s16;
    if gSaveContext.dayTime as libc::c_int > 0xc000 as libc::c_int ||
           (gSaveContext.dayTime as libc::c_int) < 0x4555 as libc::c_int {
        gSaveContext.nightFlag = 1 as libc::c_int
    } else { gSaveContext.nightFlag = 0 as libc::c_int }
    (*(*globalCtx).state.gfxCtx).callback =
        Some(Environment_GraphCallback as
                 unsafe extern "C" fn(_: *mut GraphicsContext,
                                      _: *mut libc::c_void) -> ());
    (*(*globalCtx).state.gfxCtx).callbackParam =
        globalCtx as *mut libc::c_void;
    Lights_DirectionalSetInfo(&mut (*envCtx).dirLight1,
                              80 as libc::c_int as s8,
                              80 as libc::c_int as s8,
                              80 as libc::c_int as s8,
                              80 as libc::c_int as u8_0,
                              80 as libc::c_int as u8_0,
                              80 as libc::c_int as u8_0);
    LightContext_InsertLight(globalCtx, &mut (*globalCtx).lightCtx,
                             &mut (*envCtx).dirLight1);
    Lights_DirectionalSetInfo(&mut (*envCtx).dirLight2,
                              80 as libc::c_int as s8,
                              80 as libc::c_int as s8,
                              80 as libc::c_int as s8,
                              80 as libc::c_int as u8_0,
                              80 as libc::c_int as u8_0,
                              80 as libc::c_int as u8_0);
    LightContext_InsertLight(globalCtx, &mut (*globalCtx).lightCtx,
                             &mut (*envCtx).dirLight2);
    (*envCtx).skybox1Index = 99 as libc::c_int as u8_0;
    (*envCtx).skybox2Index = 99 as libc::c_int as u8_0;
    (*envCtx).unk_19 = 0 as libc::c_int as u8_0;
    (*envCtx).unk_1A = 0 as libc::c_int as u16_0;
    (*envCtx).unk_21 = 0 as libc::c_int as u8_0;
    (*envCtx).unk_22 = 0 as libc::c_int as u16_0;
    (*envCtx).skyboxDmaState = SKYBOX_DMA_INACTIVE as libc::c_int as s8;
    (*envCtx).unk_1F = 0 as libc::c_int as u8_0;
    (*envCtx).unk_20 = 0 as libc::c_int as u8_0;
    (*envCtx).unk_84 = 0.0f32;
    (*envCtx).unk_88 = 0.0f32;
    (*envCtx).unk_BD = 0 as libc::c_int as u8_0;
    (*envCtx).unk_BE = 0 as libc::c_int as u8_0;
    (*envCtx).unk_D8 = 1.0f32;
    (*envCtx).unk_DC = 0 as libc::c_int as u8_0;
    (*envCtx).gloomySkyMode = 0 as libc::c_int as u8_0;
    (*envCtx).unk_DE = 0 as libc::c_int as u8_0;
    (*envCtx).lightningMode = LIGHTNING_MODE_OFF as libc::c_int as u8_0;
    (*envCtx).unk_E0 = 0 as libc::c_int as u8_0;
    (*envCtx).fillScreen = 0 as libc::c_int as u8_0;
    (*envCtx).screenFillColor[0 as libc::c_int as usize] =
        0 as libc::c_int as u8_0;
    (*envCtx).screenFillColor[1 as libc::c_int as usize] =
        0 as libc::c_int as u8_0;
    (*envCtx).screenFillColor[2 as libc::c_int as usize] =
        0 as libc::c_int as u8_0;
    (*envCtx).screenFillColor[3 as libc::c_int as usize] =
        0 as libc::c_int as u8_0;
    (*envCtx).customSkyboxFilter = 0 as libc::c_int as u8_0;
    (*envCtx).skyboxFilterColor[0 as libc::c_int as usize] =
        0 as libc::c_int as u8_0;
    (*envCtx).skyboxFilterColor[1 as libc::c_int as usize] =
        0 as libc::c_int as u8_0;
    (*envCtx).skyboxFilterColor[2 as libc::c_int as usize] =
        0 as libc::c_int as u8_0;
    (*envCtx).skyboxFilterColor[3 as libc::c_int as usize] =
        0 as libc::c_int as u8_0;
    (*envCtx).sandstormState = 0 as libc::c_int as u8_0;
    (*envCtx).sandstormPrimA = 0 as libc::c_int as u8_0;
    (*envCtx).sandstormEnvA = 0 as libc::c_int as u8_0;
    gLightningStrike.state = LIGHTNING_STRIKE_WAIT as libc::c_int as u8_0;
    gLightningStrike.flashRed = 0 as libc::c_int as u8_0;
    gLightningStrike.flashGreen = 0 as libc::c_int as u8_0;
    gLightningStrike.flashBlue = 0 as libc::c_int as u8_0;
    sLightningFlashAlpha = 0 as libc::c_int as s16;
    gSaveContext.unk_1410 = 0 as libc::c_int as u8_0;
    (*envCtx).adjFogFar = 0 as libc::c_int as s16;
    (*envCtx).adjFogNear = (*envCtx).adjFogFar;
    (*envCtx).adjFogColor[2 as libc::c_int as usize] = (*envCtx).adjFogNear;
    (*envCtx).adjFogColor[1 as libc::c_int as usize] =
        (*envCtx).adjFogColor[2 as libc::c_int as usize];
    (*envCtx).adjFogColor[0 as libc::c_int as usize] =
        (*envCtx).adjFogColor[1 as libc::c_int as usize];
    (*envCtx).adjLight1Color[2 as libc::c_int as usize] =
        (*envCtx).adjFogColor[0 as libc::c_int as usize];
    (*envCtx).adjLight1Color[1 as libc::c_int as usize] =
        (*envCtx).adjLight1Color[2 as libc::c_int as usize];
    (*envCtx).adjLight1Color[0 as libc::c_int as usize] =
        (*envCtx).adjLight1Color[1 as libc::c_int as usize];
    (*envCtx).adjAmbientColor[2 as libc::c_int as usize] =
        (*envCtx).adjLight1Color[0 as libc::c_int as usize];
    (*envCtx).adjAmbientColor[1 as libc::c_int as usize] =
        (*envCtx).adjAmbientColor[2 as libc::c_int as usize];
    (*envCtx).adjAmbientColor[0 as libc::c_int as usize] =
        (*envCtx).adjAmbientColor[1 as libc::c_int as usize];
    (*envCtx).sunPos.x =
        -(Math_SinS((gSaveContext.dayTime as libc::c_int -
                         0x8000 as libc::c_int) as s16) * 120.0f32) * 25.0f32;
    (*envCtx).sunPos.y =
        Math_CosS((gSaveContext.dayTime as libc::c_int -
                       0x8000 as libc::c_int) as s16) * 120.0f32 * 25.0f32;
    (*envCtx).sunPos.z =
        Math_CosS((gSaveContext.dayTime as libc::c_int -
                       0x8000 as libc::c_int) as s16) * 20.0f32 * 25.0f32;
    (*envCtx).windDirection.x = 80 as libc::c_int as s16;
    (*envCtx).windDirection.y = 80 as libc::c_int as s16;
    (*envCtx).windDirection.z = 80 as libc::c_int as s16;
    (*envCtx).blendIndoorLights = 0 as libc::c_int as u8_0;
    (*envCtx).unk_BF = 0xff as libc::c_int as u8_0;
    (*envCtx).unk_D6 = 0xffff as libc::c_int as u16_0;
    (*envCtx).timeIncrement = 0 as libc::c_int as u16_0;
    gTimeIncrement = (*envCtx).timeIncrement;
    (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 15 as libc::c_int) as usize] =
        gTimeIncrement as s16;
    (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 9 as libc::c_int) as usize] =
        1 as libc::c_int as s16;
    if (*gGameInfo).data[(11 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 3 as libc::c_int) as usize]
           as libc::c_int != 0 as libc::c_int {
        gSaveContext.chamberCutsceneNum =
            ((*gGameInfo).data[(11 as libc::c_int * 6 as libc::c_int *
                                    16 as libc::c_int + 3 as libc::c_int) as
                                   usize] as libc::c_int - 1 as libc::c_int)
                as u8_0
    }
    (*globalCtx).envCtx.unk_EE[0 as libc::c_int as usize] =
        0 as libc::c_int as u8_0;
    (*globalCtx).envCtx.unk_EE[1 as libc::c_int as usize] =
        0 as libc::c_int as u8_0;
    (*globalCtx).envCtx.unk_EE[2 as libc::c_int as usize] =
        0 as libc::c_int as u8_0;
    (*globalCtx).envCtx.unk_EE[3 as libc::c_int as usize] =
        0 as libc::c_int as u8_0;
    (*globalCtx).envCtx.unk_F2[0 as libc::c_int as usize] =
        0 as libc::c_int as u8_0;
    if gSaveContext.unk_13C3 as libc::c_int != 0 as libc::c_int {
        if gSaveContext.sceneSetupIndex < 4 as libc::c_int {
            match gWeatherMode as libc::c_int {
                1 => {
                    (*envCtx).unk_17 = 1 as libc::c_int as u8_0;
                    (*envCtx).unk_18 = 1 as libc::c_int as u8_0;
                    (*envCtx).unk_1F = 3 as libc::c_int as u8_0;
                    (*envCtx).unk_20 = 3 as libc::c_int as u8_0;
                    (*globalCtx).envCtx.unk_EE[3 as libc::c_int as usize] =
                        0 as libc::c_int as u8_0;
                    (*globalCtx).envCtx.unk_EE[2 as libc::c_int as usize] =
                        0 as libc::c_int as u8_0
                }
                2 | 3 | 4 => {
                    (*envCtx).unk_17 = 1 as libc::c_int as u8_0;
                    (*envCtx).unk_18 = 1 as libc::c_int as u8_0;
                    (*envCtx).unk_1F = 2 as libc::c_int as u8_0;
                    (*envCtx).unk_20 = 2 as libc::c_int as u8_0;
                    (*globalCtx).envCtx.unk_EE[3 as libc::c_int as usize] =
                        0 as libc::c_int as u8_0;
                    (*globalCtx).envCtx.unk_EE[2 as libc::c_int as usize] =
                        0 as libc::c_int as u8_0
                }
                5 => {
                    (*envCtx).unk_17 = 1 as libc::c_int as u8_0;
                    (*envCtx).unk_18 = 1 as libc::c_int as u8_0;
                    (*envCtx).unk_1F = 4 as libc::c_int as u8_0;
                    (*envCtx).unk_20 = 4 as libc::c_int as u8_0;
                    (*globalCtx).envCtx.unk_EE[3 as libc::c_int as usize] =
                        0 as libc::c_int as u8_0;
                    (*globalCtx).envCtx.unk_EE[2 as libc::c_int as usize] =
                        0 as libc::c_int as u8_0
                }
                _ => { }
            }
            if (*globalCtx).skyboxId as libc::c_int ==
                   SKYBOX_NORMAL_SKY as libc::c_int {
                if gWeatherMode as libc::c_int == 3 as libc::c_int {
                    (*globalCtx).envCtx.unk_EE[3 as libc::c_int as usize] =
                        0x40 as libc::c_int as u8_0;
                    (*globalCtx).envCtx.unk_EE[2 as libc::c_int as usize] =
                        (*globalCtx).envCtx.unk_EE[3 as libc::c_int as usize]
                } else if gWeatherMode as libc::c_int == 4 as libc::c_int {
                    (*globalCtx).envCtx.unk_EE[0 as libc::c_int as usize] =
                        0x14 as libc::c_int as u8_0;
                    (*globalCtx).envCtx.unk_EE[1 as libc::c_int as usize] =
                        0x14 as libc::c_int as u8_0
                } else if gWeatherMode as libc::c_int == 5 as libc::c_int {
                    (*globalCtx).envCtx.unk_EE[0 as libc::c_int as usize] =
                        0x1e as libc::c_int as u8_0;
                    (*globalCtx).envCtx.unk_EE[1 as libc::c_int as usize] =
                        0x1e as libc::c_int as u8_0
                }
            }
        }
    } else { gWeatherMode = 0 as libc::c_int as u8_0 }
    D_8011FB38 = 0 as libc::c_int as u8_0;
    D_8011FB34 = 0 as libc::c_int as u8_0;
    gSkyboxBlendingEnabled = 0 as libc::c_int as u8_0;
    gSaveContext.unk_13C3 = 0 as libc::c_int as u8_0;
    (*gGameInfo).data[(15 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 3 as libc::c_int +
                           0 as libc::c_int) as usize] =
        80 as libc::c_int as s16;
    (*gGameInfo).data[(15 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 3 as libc::c_int +
                           1 as libc::c_int) as usize] =
        80 as libc::c_int as s16;
    (*gGameInfo).data[(15 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 3 as libc::c_int +
                           2 as libc::c_int) as usize] =
        80 as libc::c_int as s16;
    (*gGameInfo).data[(15 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 6 as libc::c_int +
                           0 as libc::c_int) as usize] =
        -(80 as libc::c_int) as s16;
    (*gGameInfo).data[(15 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 6 as libc::c_int +
                           1 as libc::c_int) as usize] =
        -(80 as libc::c_int) as s16;
    (*gGameInfo).data[(15 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 6 as libc::c_int +
                           2 as libc::c_int) as usize] =
        -(80 as libc::c_int) as s16;
    (*gGameInfo).data[(15 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 9 as libc::c_int) as usize] =
        10 as libc::c_int as s16;
    (*gGameInfo).data[(15 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 10 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(15 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 11 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(15 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 12 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(15 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 13 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    (*gGameInfo).data[(15 as libc::c_int * 6 as libc::c_int *
                           16 as libc::c_int + 14 as libc::c_int) as usize] =
        0 as libc::c_int as s16;
    D_8015FCC8 = 1 as libc::c_int as u8_0;
    i = 0 as libc::c_int as u8_0;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[LightningBolt; 3]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<LightningBolt>()
                                                   as libc::c_ulong) as s32 {
        sLightningBolts[i as usize].state =
            LIGHTNING_BOLT_INACTIVE as libc::c_int as u8_0;
        i = i.wrapping_add(1)
    }
    (*globalCtx).roomCtx.unk_74[0 as libc::c_int as usize] =
        0 as libc::c_int as s16;
    (*globalCtx).roomCtx.unk_74[1 as libc::c_int as usize] =
        0 as libc::c_int as s16;
    i = 0 as libc::c_int as u8_0;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[*mut CsCmdActorAction; 10]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<*mut CsCmdActorAction>()
                                                   as libc::c_ulong) as s32 {
        (*globalCtx).csCtx.npcActions[i as usize] =
            0 as *mut CsCmdActorAction;
        i = i.wrapping_add(1)
    }
    if Object_GetIndex(&mut (*globalCtx).objectCtx,
                       OBJECT_GAMEPLAY_FIELD_KEEP as libc::c_int as s16) <
           0 as libc::c_int && (*globalCtx).envCtx.sunMoonDisabled == 0 {
        (*globalCtx).envCtx.sunMoonDisabled = 1 as libc::c_int as u8_0;
        // "Sun setting other than field keep! So forced release!"
        osSyncPrintf(b"\x1b[43;30m\n\n\xe3\x83\x95\xe3\x82\xa3\xe3\x83\xbc\xe3\x83\xab\xe3\x83\x89\xe5\xb8\xb8\xe9\xa7\x90\xe4\xbb\xa5\xe5\xa4\x96\xe3\x80\x81\xe5\xa4\xaa\xe9\x99\xbd\xe8\xa8\xad\xe5\xae\x9a\xef\xbc\x81\xe3\x82\x88\xe3\x81\xa3\xe3\x81\xa6\xe5\xbc\xb7\xe5\x88\xb6\xe8\xa7\xa3\xe9\x99\xa4\xef\xbc\x81\n\x1b[m\x00"
                         as *const u8 as *const libc::c_char);
    }
    gCustomLensFlareOn = 0 as libc::c_int as u8_0;
    func_800AA15C();
}
#[no_mangle]
pub unsafe extern "C" fn Environment_SmoothStepToU8(mut pvalue: *mut u8_0,
                                                    mut target: u8_0,
                                                    mut scale: u8_0,
                                                    mut step: u8_0,
                                                    mut minStep: u8_0)
 -> u8_0 {
    let mut stepSize: s16 = 0 as libc::c_int as s16;
    let mut diff: s16 =
        (target as libc::c_int - *pvalue as libc::c_int) as s16;
    if target as libc::c_int != *pvalue as libc::c_int {
        stepSize = (diff as libc::c_int / scale as libc::c_int) as s16;
        if stepSize as libc::c_int >= minStep as s16 as libc::c_int ||
               -(minStep as libc::c_int) as s16 as libc::c_int >=
                   stepSize as libc::c_int {
            if (step as s16 as libc::c_int) < stepSize as libc::c_int {
                stepSize = step as s16
            }
            if -(step as libc::c_int) as s16 as libc::c_int >
                   stepSize as libc::c_int {
                stepSize = -(step as libc::c_int) as s16
            }
            *pvalue =
                (*pvalue as libc::c_int + stepSize as u8_0 as libc::c_int) as
                    u8_0
        } else {
            if (stepSize as libc::c_int) < minStep as s16 as libc::c_int {
                stepSize = minStep as s16;
                *pvalue =
                    (*pvalue as libc::c_int + stepSize as u8_0 as libc::c_int)
                        as u8_0;
                if (target as libc::c_int) < *pvalue as libc::c_int {
                    *pvalue = target
                }
            }
            if (-(minStep as libc::c_int) as s16 as libc::c_int) <
                   stepSize as libc::c_int {
                stepSize = -(minStep as libc::c_int) as s16;
                *pvalue =
                    (*pvalue as libc::c_int + stepSize as u8_0 as libc::c_int)
                        as u8_0;
                if (*pvalue as libc::c_int) < target as libc::c_int {
                    *pvalue = target
                }
            }
        }
    }
    return diff as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn Environment_SmoothStepToS8(mut pvalue: *mut s8,
                                                    mut target: s8,
                                                    mut scale: u8_0,
                                                    mut step: u8_0,
                                                    mut minStep: u8_0)
 -> u8_0 {
    let mut stepSize: s16 = 0 as libc::c_int as s16;
    let mut diff: s16 =
        (target as libc::c_int - *pvalue as libc::c_int) as s16;
    if target as libc::c_int != *pvalue as libc::c_int {
        stepSize = (diff as libc::c_int / scale as libc::c_int) as s16;
        if stepSize as libc::c_int >= minStep as s16 as libc::c_int ||
               -(minStep as libc::c_int) as s16 as libc::c_int >=
                   stepSize as libc::c_int {
            if (step as s16 as libc::c_int) < stepSize as libc::c_int {
                stepSize = step as s16
            }
            if -(step as libc::c_int) as s16 as libc::c_int >
                   stepSize as libc::c_int {
                stepSize = -(step as libc::c_int) as s16
            }
            *pvalue =
                (*pvalue as libc::c_int + stepSize as s8 as libc::c_int) as s8
        } else {
            if (stepSize as libc::c_int) < minStep as s16 as libc::c_int {
                stepSize = minStep as s16;
                *pvalue =
                    (*pvalue as libc::c_int + stepSize as s8 as libc::c_int)
                        as s8;
                if (target as libc::c_int) < *pvalue as libc::c_int {
                    *pvalue = target
                }
            }
            if (-(minStep as libc::c_int) as s16 as libc::c_int) <
                   stepSize as libc::c_int {
                stepSize = -(minStep as libc::c_int) as s16;
                *pvalue =
                    (*pvalue as libc::c_int + stepSize as s8 as libc::c_int)
                        as s8;
                if (*pvalue as libc::c_int) < target as libc::c_int {
                    *pvalue = target
                }
            }
        }
    }
    return diff as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn Environment_LerpWeight(mut max: u16_0,
                                                mut min: u16_0,
                                                mut val: u16_0) -> f32_0 {
    let mut diff: f32_0 = (max as libc::c_int - min as libc::c_int) as f32_0;
    let mut ret: f32_0 = 0.;
    if diff != 0.0f32 {
        ret =
            1.0f32 -
                (max as libc::c_int - val as libc::c_int) as libc::c_float /
                    diff;
        if !(ret >= 1.0f32) { return ret }
    }
    return 1.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn Environment_LerpWeightAccelDecel(mut endFrame: u16_0,
                                                          mut startFrame:
                                                              u16_0,
                                                          mut curFrame: u16_0,
                                                          mut accelDuration:
                                                              u16_0,
                                                          mut decelDuration:
                                                              u16_0)
 -> f32_0 {
    let mut endFrameF: f32_0 = 0.;
    let mut startFrameF: f32_0 = 0.;
    let mut curFrameF: f32_0 = 0.;
    let mut accelDurationF: f32_0 = 0.;
    let mut decelDurationF: f32_0 = 0.;
    let mut totalFrames: f32_0 = 0.;
    let mut temp: f32_0 = 0.;
    let mut framesElapsed: f32_0 = 0.;
    let mut ret: f32_0 = 0.;
    if curFrame as libc::c_int <= startFrame as libc::c_int { return 0.0f32 }
    if curFrame as libc::c_int >= endFrame as libc::c_int { return 1.0f32 }
    endFrameF = endFrame as s32 as f32_0;
    startFrameF = startFrame as s32 as f32_0;
    curFrameF = curFrame as s32 as f32_0;
    totalFrames = endFrameF - startFrameF;
    framesElapsed = curFrameF - startFrameF;
    accelDurationF = accelDuration as s32 as f32_0;
    decelDurationF = decelDuration as s32 as f32_0;
    if startFrameF >= endFrameF ||
           accelDurationF + decelDurationF > totalFrames {
        // "The frame relation between end_frame and start_frame is wrong!!!"
        osSyncPrintf(b"\x1b[41;37m\nend_frame\xe3\x81\xa8start_frame\xe3\x81\xae\xe3\x83\x95\xe3\x83\xac\xe3\x83\xbc\xe3\x83\xa0\xe9\x96\xa2\xe4\xbf\x82\xe3\x81\x8c\xe3\x81\x8a\xe3\x81\x8b\xe3\x81\x97\xe3\x81\x84!!!\x1b[m\x00"
                         as *const u8 as *const libc::c_char);
        osSyncPrintf(b"\x1b[41;37m\nby get_parcent_forAccelBrake!!!!!!!!!\x1b[m\x00"
                         as *const u8 as *const libc::c_char);
        return 0.0f32
    }
    temp = 1.0f32 / (totalFrames * 2.0f32 - accelDurationF - decelDurationF);
    if accelDurationF != 0.0f32 {
        if framesElapsed <= accelDurationF {
            return temp * framesElapsed * framesElapsed / accelDurationF
        }
        ret = temp * accelDurationF
    } else { ret = 0.0f32 }
    if framesElapsed <= totalFrames - decelDurationF {
        ret += 2.0f32 * temp * (framesElapsed - accelDurationF);
        return ret
    }
    ret += 2.0f32 * temp * (totalFrames - accelDurationF - decelDurationF);
    if decelDurationF != 0.0f32 {
        ret += temp * decelDurationF;
        if framesElapsed < totalFrames {
            ret -=
                temp * (totalFrames - framesElapsed) *
                    (totalFrames - framesElapsed) / decelDurationF
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn func_8006FB94(mut envCtx: *mut EnvironmentContext,
                                       mut unused: u8_0) {
    if (*envCtx).gloomySkyMode as libc::c_int != 0 as libc::c_int {
        match (*envCtx).unk_DE as libc::c_int {
            0 => {
                if (*envCtx).gloomySkyMode as libc::c_int == 1 as libc::c_int
                       && gSkyboxBlendingEnabled == 0 {
                    (*envCtx).unk_19 = 1 as libc::c_int as u8_0;
                    (*envCtx).unk_17 = 0 as libc::c_int as u8_0;
                    (*envCtx).unk_18 = 1 as libc::c_int as u8_0;
                    (*envCtx).unk_1A = 100 as libc::c_int as u16_0;
                    (*envCtx).unk_21 = 1 as libc::c_int as u8_0;
                    (*envCtx).unk_1F = 0 as libc::c_int as u8_0;
                    (*envCtx).unk_20 = 2 as libc::c_int as u8_0;
                    D_8011FB34 = 2 as libc::c_int as u8_0;
                    (*envCtx).unk_24 = 100 as libc::c_int as u16_0;
                    (*envCtx).unk_22 = (*envCtx).unk_24;
                    (*envCtx).unk_DE = (*envCtx).unk_DE.wrapping_add(1)
                }
            }
            1 => {
                if gSkyboxBlendingEnabled == 0 &&
                       (*envCtx).gloomySkyMode as libc::c_int ==
                           2 as libc::c_int {
                    gWeatherMode = 0 as libc::c_int as u8_0;
                    (*envCtx).unk_19 = 1 as libc::c_int as u8_0;
                    (*envCtx).unk_17 = 1 as libc::c_int as u8_0;
                    (*envCtx).unk_18 = 0 as libc::c_int as u8_0;
                    (*envCtx).unk_1A = 100 as libc::c_int as u16_0;
                    (*envCtx).unk_21 = 1 as libc::c_int as u8_0;
                    (*envCtx).unk_1F = 2 as libc::c_int as u8_0;
                    (*envCtx).unk_20 = 0 as libc::c_int as u8_0;
                    D_8011FB34 = 0 as libc::c_int as u8_0;
                    (*envCtx).unk_24 = 100 as libc::c_int as u16_0;
                    (*envCtx).unk_22 = (*envCtx).unk_24;
                    (*envCtx).unk_EE[0 as libc::c_int as usize] =
                        0 as libc::c_int as u8_0;
                    (*envCtx).gloomySkyMode = 0 as libc::c_int as u8_0;
                    (*envCtx).unk_DE = 0 as libc::c_int as u8_0
                }
            }
            _ => { }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Environment_UpdateSkybox(mut skyboxId: u8_0,
                                                  mut envCtx:
                                                      *mut EnvironmentContext,
                                                  mut skyboxCtx:
                                                      *mut SkyboxContext) {
    let mut size: u32_0 = 0;
    let mut i: u8_0 = 0;
    let mut newSkybox1Index: u8_0 = 0xff as libc::c_int as u8_0;
    let mut newSkybox2Index: u8_0 = 0xff as libc::c_int as u8_0;
    let mut skyboxBlend: u8_0 = 0 as libc::c_int as u8_0;
    if skyboxId as libc::c_int == SKYBOX_CUTSCENE_MAP as libc::c_int {
        (*envCtx).unk_17 = 3 as libc::c_int as u8_0;
        i = 0 as libc::c_int as u8_0;
        while (i as libc::c_int) <
                  (::std::mem::size_of::<[struct_8011FC1C; 9]>() as
                       libc::c_ulong).wrapping_div(::std::mem::size_of::<struct_8011FC1C>()
                                                       as libc::c_ulong) as
                      s32 {
            if gSaveContext.skyboxTime as libc::c_int >=
                   D_8011FC1C[(*envCtx).unk_17 as usize][i as usize].startTime
                       as libc::c_int &&
                   ((gSaveContext.skyboxTime as libc::c_int) <
                        D_8011FC1C[(*envCtx).unk_17 as
                                       usize][i as usize].endTime as
                            libc::c_int ||
                        D_8011FC1C[(*envCtx).unk_17 as
                                       usize][i as usize].endTime as
                            libc::c_int == 0xffff as libc::c_int) {
                if D_8011FC1C[(*envCtx).unk_17 as usize][i as usize].blend !=
                       0 {
                    (*envCtx).skyboxBlend =
                        (Environment_LerpWeight(D_8011FC1C[(*envCtx).unk_17 as
                                                               usize][i as
                                                                          usize].endTime,
                                                D_8011FC1C[(*envCtx).unk_17 as
                                                               usize][i as
                                                                          usize].startTime,
                                                gSaveContext.skyboxTime) *
                             255 as libc::c_int as libc::c_float) as u8_0
                } else { (*envCtx).skyboxBlend = 0 as libc::c_int as u8_0 }
                break ;
            } else { i = i.wrapping_add(1) }
        }
    } else if skyboxId as libc::c_int == SKYBOX_NORMAL_SKY as libc::c_int &&
                  (*envCtx).skyboxDisabled == 0 {
        i = 0 as libc::c_int as u8_0;
        while (i as libc::c_int) <
                  (::std::mem::size_of::<[struct_8011FC1C; 9]>() as
                       libc::c_ulong).wrapping_div(::std::mem::size_of::<struct_8011FC1C>()
                                                       as libc::c_ulong) as
                      s32 {
            if gSaveContext.skyboxTime as libc::c_int >=
                   D_8011FC1C[(*envCtx).unk_17 as usize][i as usize].startTime
                       as libc::c_int &&
                   ((gSaveContext.skyboxTime as libc::c_int) <
                        D_8011FC1C[(*envCtx).unk_17 as
                                       usize][i as usize].endTime as
                            libc::c_int ||
                        D_8011FC1C[(*envCtx).unk_17 as
                                       usize][i as usize].endTime as
                            libc::c_int == 0xffff as libc::c_int) {
                newSkybox1Index =
                    D_8011FC1C[(*envCtx).unk_17 as
                                   usize][i as usize].skybox1Index;
                newSkybox2Index =
                    D_8011FC1C[(*envCtx).unk_17 as
                                   usize][i as usize].skybox2Index;
                gSkyboxBlendingEnabled =
                    D_8011FC1C[(*envCtx).unk_17 as usize][i as usize].blend;
                if gSkyboxBlendingEnabled != 0 {
                    skyboxBlend =
                        (Environment_LerpWeight(D_8011FC1C[(*envCtx).unk_17 as
                                                               usize][i as
                                                                          usize].endTime,
                                                D_8011FC1C[(*envCtx).unk_17 as
                                                               usize][i as
                                                                          usize].startTime,
                                                gSaveContext.skyboxTime) *
                             255 as libc::c_int as libc::c_float) as u8_0
                } else {
                    skyboxBlend =
                        (Environment_LerpWeight(D_8011FC1C[(*envCtx).unk_17 as
                                                               usize][i as
                                                                          usize].endTime,
                                                D_8011FC1C[(*envCtx).unk_17 as
                                                               usize][i as
                                                                          usize].startTime,
                                                gSaveContext.skyboxTime) *
                             255 as libc::c_int as libc::c_float) as u8_0;
                    skyboxBlend =
                        if (skyboxBlend as libc::c_int) < 0x80 as libc::c_int
                           {
                            0xff as libc::c_int
                        } else { 0 as libc::c_int } as u8_0;
                    if (*envCtx).unk_19 as libc::c_int != 0 as libc::c_int &&
                           ((*envCtx).unk_19 as libc::c_int) <
                               3 as libc::c_int {
                        (*envCtx).unk_19 = (*envCtx).unk_19.wrapping_add(1);
                        skyboxBlend = 0 as libc::c_int as u8_0
                    }
                }
                break ;
            } else { i = i.wrapping_add(1) }
        }
        func_8006FB94(envCtx, skyboxBlend);
        if (*envCtx).unk_19 as libc::c_int >= 3 as libc::c_int {
            newSkybox1Index =
                D_8011FC1C[(*envCtx).unk_17 as
                               usize][i as usize].skybox1Index;
            newSkybox2Index =
                D_8011FC1C[(*envCtx).unk_18 as
                               usize][i as usize].skybox2Index;
            let fresh0 = (*envCtx).unk_1A;
            (*envCtx).unk_1A = (*envCtx).unk_1A.wrapping_sub(1);
            skyboxBlend =
                (((*envCtx).unk_24 as f32_0 -
                      fresh0 as libc::c_int as libc::c_float) /
                     (*envCtx).unk_24 as f32_0 *
                     255 as libc::c_int as libc::c_float) as u8_0;
            if (*envCtx).unk_1A as libc::c_int <= 0 as libc::c_int {
                (*envCtx).unk_19 = 0 as libc::c_int as u8_0;
                (*envCtx).unk_17 = (*envCtx).unk_18
            }
        }
        if newSkybox1Index as libc::c_int == 0xff as libc::c_int {
            // "Environment VR data acquisition failed! Report to Sasaki!"
            osSyncPrintf(b"\x1b[41;37m\n\xe7\x92\xb0\xe5\xa2\x83\xef\xbc\xb6\xef\xbc\xb2\xe3\x83\x87\xe3\x83\xbc\xe3\x82\xbf\xe5\x8f\x96\xe5\xbe\x97\xe5\xa4\xb1\xe6\x95\x97\xef\xbc\x81 \xe3\x81\x95\xe3\x81\x95\xe3\x81\x8d\xe3\x81\xbe\xe3\x81\xa7\xe3\x81\x94\xe5\xa0\xb1\xe5\x91\x8a\xe3\x82\x92\xef\xbc\x81\x1b[m\x00"
                             as *const u8 as *const libc::c_char);
        }
        if (*envCtx).skybox1Index as libc::c_int !=
               newSkybox1Index as libc::c_int &&
               (*envCtx).skyboxDmaState as libc::c_int ==
                   SKYBOX_DMA_INACTIVE as libc::c_int {
            (*envCtx).skyboxDmaState =
                SKYBOX_DMA_FILE1_START as libc::c_int as s8;
            size =
                gSkyboxFiles[newSkybox1Index as
                                 usize].file.vromEnd.wrapping_sub(gSkyboxFiles[newSkybox1Index
                                                                                   as
                                                                                   usize].file.vromStart);
            osCreateMesgQueue(&mut (*envCtx).loadQueue,
                              &mut (*envCtx).loadMsg, 1 as libc::c_int);
            DmaMgr_SendRequest2(&mut (*envCtx).dmaRequest,
                                (*skyboxCtx).staticSegments[0 as libc::c_int
                                                                as usize] as
                                    u32_0,
                                gSkyboxFiles[newSkybox1Index as
                                                 usize].file.vromStart, size,
                                0 as libc::c_int as u32_0,
                                &mut (*envCtx).loadQueue,
                                0 as *mut libc::c_void,
                                b"../z_kankyo.c\x00" as *const u8 as
                                    *const libc::c_char, 1264 as libc::c_int);
            (*envCtx).skybox1Index = newSkybox1Index
        }
        if (*envCtx).skybox2Index as libc::c_int !=
               newSkybox2Index as libc::c_int &&
               (*envCtx).skyboxDmaState as libc::c_int ==
                   SKYBOX_DMA_INACTIVE as libc::c_int {
            (*envCtx).skyboxDmaState =
                SKYBOX_DMA_FILE2_START as libc::c_int as s8;
            size =
                gSkyboxFiles[newSkybox2Index as
                                 usize].file.vromEnd.wrapping_sub(gSkyboxFiles[newSkybox2Index
                                                                                   as
                                                                                   usize].file.vromStart);
            osCreateMesgQueue(&mut (*envCtx).loadQueue,
                              &mut (*envCtx).loadMsg, 1 as libc::c_int);
            DmaMgr_SendRequest2(&mut (*envCtx).dmaRequest,
                                (*skyboxCtx).staticSegments[1 as libc::c_int
                                                                as usize] as
                                    u32_0,
                                gSkyboxFiles[newSkybox2Index as
                                                 usize].file.vromStart, size,
                                0 as libc::c_int as u32_0,
                                &mut (*envCtx).loadQueue,
                                0 as *mut libc::c_void,
                                b"../z_kankyo.c\x00" as *const u8 as
                                    *const libc::c_char, 1281 as libc::c_int);
            (*envCtx).skybox2Index = newSkybox2Index
        }
        if (*envCtx).skyboxDmaState as libc::c_int ==
               SKYBOX_DMA_FILE1_DONE as libc::c_int {
            (*envCtx).skyboxDmaState =
                SKYBOX_DMA_PAL1_START as libc::c_int as s8;
            if newSkybox1Index as libc::c_int & 1 as libc::c_int ^
                   (newSkybox1Index as libc::c_int & 4 as libc::c_int) >>
                       2 as libc::c_int != 0 {
                size =
                    gSkyboxFiles[newSkybox1Index as
                                     usize].palette.vromEnd.wrapping_sub(gSkyboxFiles[newSkybox1Index
                                                                                          as
                                                                                          usize].palette.vromStart);
                osCreateMesgQueue(&mut (*envCtx).loadQueue,
                                  &mut (*envCtx).loadMsg, 1 as libc::c_int);
                DmaMgr_SendRequest2(&mut (*envCtx).dmaRequest,
                                    (*skyboxCtx).palettes as u32_0,
                                    gSkyboxFiles[newSkybox1Index as
                                                     usize].palette.vromStart,
                                    size, 0 as libc::c_int as u32_0,
                                    &mut (*envCtx).loadQueue,
                                    0 as *mut libc::c_void,
                                    b"../z_kankyo.c\x00" as *const u8 as
                                        *const libc::c_char,
                                    1307 as libc::c_int);
            } else {
                size =
                    gSkyboxFiles[newSkybox1Index as
                                     usize].palette.vromEnd.wrapping_sub(gSkyboxFiles[newSkybox1Index
                                                                                          as
                                                                                          usize].palette.vromStart);
                osCreateMesgQueue(&mut (*envCtx).loadQueue,
                                  &mut (*envCtx).loadMsg, 1 as libc::c_int);
                DmaMgr_SendRequest2(&mut (*envCtx).dmaRequest,
                                    ((*skyboxCtx).palettes as
                                         u32_0).wrapping_add(size),
                                    gSkyboxFiles[newSkybox1Index as
                                                     usize].palette.vromStart,
                                    size, 0 as libc::c_int as u32_0,
                                    &mut (*envCtx).loadQueue,
                                    0 as *mut libc::c_void,
                                    b"../z_kankyo.c\x00" as *const u8 as
                                        *const libc::c_char,
                                    1320 as libc::c_int);
            }
        }
        if (*envCtx).skyboxDmaState as libc::c_int ==
               SKYBOX_DMA_FILE2_DONE as libc::c_int {
            (*envCtx).skyboxDmaState =
                SKYBOX_DMA_PAL2_START as libc::c_int as s8;
            if newSkybox2Index as libc::c_int & 1 as libc::c_int ^
                   (newSkybox2Index as libc::c_int & 4 as libc::c_int) >>
                       2 as libc::c_int != 0 {
                size =
                    gSkyboxFiles[newSkybox2Index as
                                     usize].palette.vromEnd.wrapping_sub(gSkyboxFiles[newSkybox2Index
                                                                                          as
                                                                                          usize].palette.vromStart);
                osCreateMesgQueue(&mut (*envCtx).loadQueue,
                                  &mut (*envCtx).loadMsg, 1 as libc::c_int);
                DmaMgr_SendRequest2(&mut (*envCtx).dmaRequest,
                                    (*skyboxCtx).palettes as u32_0,
                                    gSkyboxFiles[newSkybox2Index as
                                                     usize].palette.vromStart,
                                    size, 0 as libc::c_int as u32_0,
                                    &mut (*envCtx).loadQueue,
                                    0 as *mut libc::c_void,
                                    b"../z_kankyo.c\x00" as *const u8 as
                                        *const libc::c_char,
                                    1342 as libc::c_int);
            } else {
                size =
                    gSkyboxFiles[newSkybox2Index as
                                     usize].palette.vromEnd.wrapping_sub(gSkyboxFiles[newSkybox2Index
                                                                                          as
                                                                                          usize].palette.vromStart);
                osCreateMesgQueue(&mut (*envCtx).loadQueue,
                                  &mut (*envCtx).loadMsg, 1 as libc::c_int);
                DmaMgr_SendRequest2(&mut (*envCtx).dmaRequest,
                                    ((*skyboxCtx).palettes as
                                         u32_0).wrapping_add(size),
                                    gSkyboxFiles[newSkybox2Index as
                                                     usize].palette.vromStart,
                                    size, 0 as libc::c_int as u32_0,
                                    &mut (*envCtx).loadQueue,
                                    0 as *mut libc::c_void,
                                    b"../z_kankyo.c\x00" as *const u8 as
                                        *const libc::c_char,
                                    1355 as libc::c_int);
            }
        }
        if (*envCtx).skyboxDmaState as libc::c_int ==
               SKYBOX_DMA_FILE1_START as libc::c_int ||
               (*envCtx).skyboxDmaState as libc::c_int ==
                   SKYBOX_DMA_FILE2_START as libc::c_int {
            if osRecvMesg(&mut (*envCtx).loadQueue, 0 as *mut OSMesg,
                          0 as libc::c_int) == 0 as libc::c_int {
                (*envCtx).skyboxDmaState += 1
            }
        } else if (*envCtx).skyboxDmaState as libc::c_int >=
                      SKYBOX_DMA_FILE1_DONE as libc::c_int {
            if osRecvMesg(&mut (*envCtx).loadQueue, 0 as *mut OSMesg,
                          0 as libc::c_int) == 0 as libc::c_int {
                (*envCtx).skyboxDmaState =
                    SKYBOX_DMA_INACTIVE as libc::c_int as s8
            }
        }
        (*envCtx).skyboxBlend = skyboxBlend
    };
}
#[no_mangle]
pub unsafe extern "C" fn Environment_EnableUnderwaterLights(mut globalCtx:
                                                                *mut GlobalContext,
                                                            mut waterLightsIndex:
                                                                s32) {
    if waterLightsIndex == 0x1f as libc::c_int {
        waterLightsIndex = 0 as libc::c_int;
        // "Underwater color is not set in the water poly data!"
        osSyncPrintf(b"\x1b[43;30m\n\xe6\xb0\xb4\xe3\x83\x9d\xe3\x83\xaa\xe3\x82\xb4\xe3\x83\xb3\xe3\x83\x87\xe3\x83\xbc\xe3\x82\xbf\xe3\x81\xab\xe6\xb0\xb4\xe4\xb8\xad\xe3\x82\xab\xe3\x83\xa9\xe3\x83\xbc\xe3\x81\x8c\xe8\xa8\xad\xe5\xae\x9a\xe3\x81\x95\xe3\x82\x8c\xe3\x81\xa6\xe3\x81\x8a\xe3\x82\x8a\xe3\x81\xbe\xe3\x81\x9b\xe3\x82\x93!\x1b[m\x00"
                         as *const u8 as
                         *const libc::c_char); // instantly switch to water lights
    } // instantly switch to previous lights
    if (*globalCtx).envCtx.indoors == 0 {
        D_8011FB34 = (*globalCtx).envCtx.unk_20;
        if (*globalCtx).envCtx.unk_1F as libc::c_int != waterLightsIndex {
            (*globalCtx).envCtx.unk_1F = waterLightsIndex as u8_0;
            (*globalCtx).envCtx.unk_20 = waterLightsIndex as u8_0
        }
    } else {
        (*globalCtx).envCtx.blendIndoorLights = 0 as libc::c_int as u8_0;
        (*globalCtx).envCtx.unk_BF = waterLightsIndex as u8_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn Environment_DisableUnderwaterLights(mut globalCtx:
                                                                 *mut GlobalContext) {
    if (*globalCtx).envCtx.indoors == 0 {
        (*globalCtx).envCtx.unk_1F = D_8011FB34;
        (*globalCtx).envCtx.unk_20 = D_8011FB34
    } else {
        (*globalCtx).envCtx.blendIndoorLights = 0 as libc::c_int as u8_0;
        (*globalCtx).envCtx.unk_BF = 0xff as libc::c_int as u8_0;
        (*globalCtx).envCtx.unk_D8 = 1.0f32
    };
}
#[no_mangle]
pub unsafe extern "C" fn Environment_PrintDebugInfo(mut globalCtx:
                                                        *mut GlobalContext,
                                                    mut gfx: *mut *mut Gfx) {
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
                 unk_14: [0; 28],};
    let mut pad: [s32; 2] = [0; 2];
    GfxPrint_Init(&mut printer);
    GfxPrint_Open(&mut printer, *gfx);
    GfxPrint_SetPos(&mut printer, 22 as libc::c_int, 7 as libc::c_int);
    GfxPrint_SetColor(&mut printer, 155 as libc::c_int as u32_0,
                      155 as libc::c_int as u32_0,
                      255 as libc::c_int as u32_0,
                      64 as libc::c_int as u32_0);
    GfxPrint_Printf(&mut printer as *mut GfxPrint,
                    b"T%03d \x00" as *const u8 as *const libc::c_char,
                    gSaveContext.totalDays);
    GfxPrint_Printf(&mut printer as *mut GfxPrint,
                    b"E%03d\x00" as *const u8 as *const libc::c_char,
                    gSaveContext.bgsDayCount);
    GfxPrint_SetColor(&mut printer, 255 as libc::c_int as u32_0,
                      255 as libc::c_int as u32_0, 55 as libc::c_int as u32_0,
                      64 as libc::c_int as u32_0);
    GfxPrint_SetPos(&mut printer, 22 as libc::c_int, 8 as libc::c_int);
    GfxPrint_Printf(&mut printer as *mut GfxPrint,
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    b"ZELDATIME \x00" as *const u8 as *const libc::c_char);
    GfxPrint_SetColor(&mut printer, 255 as libc::c_int as u32_0,
                      255 as libc::c_int as u32_0,
                      255 as libc::c_int as u32_0,
                      64 as libc::c_int as u32_0);
    GfxPrint_Printf(&mut printer as *mut GfxPrint,
                    b"%02d\x00" as *const u8 as *const libc::c_char,
                    ((24 as libc::c_int * 60 as libc::c_int) as libc::c_float
                         / 0x10000 as libc::c_int as f32_0 *
                         gSaveContext.dayTime as libc::c_int as libc::c_float
                         / 60.0f32) as u8_0 as libc::c_int);
    if gSaveContext.dayTime as libc::c_int & 0x1f as libc::c_int >=
           0x10 as libc::c_int ||
           gTimeIncrement as libc::c_int >= 6 as libc::c_int {
        GfxPrint_Printf(&mut printer as *mut GfxPrint,
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        b":\x00" as *const u8 as *const libc::c_char);
    } else {
        GfxPrint_Printf(&mut printer as *mut GfxPrint,
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        b" \x00" as *const u8 as *const libc::c_char);
    }
    GfxPrint_Printf(&mut printer as *mut GfxPrint,
                    b"%02d\x00" as *const u8 as *const libc::c_char,
                    ((24 as libc::c_int * 60 as libc::c_int) as libc::c_float
                         / 0x10000 as libc::c_int as f32_0 *
                         gSaveContext.dayTime as libc::c_int as libc::c_float)
                        as s16 as libc::c_int % 60 as libc::c_int);
    GfxPrint_SetColor(&mut printer, 255 as libc::c_int as u32_0,
                      255 as libc::c_int as u32_0, 55 as libc::c_int as u32_0,
                      64 as libc::c_int as u32_0);
    GfxPrint_SetPos(&mut printer, 22 as libc::c_int, 9 as libc::c_int);
    GfxPrint_Printf(&mut printer as *mut GfxPrint,
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    b"VRBOXTIME \x00" as *const u8 as *const libc::c_char);
    GfxPrint_SetColor(&mut printer, 255 as libc::c_int as u32_0,
                      255 as libc::c_int as u32_0,
                      255 as libc::c_int as u32_0,
                      64 as libc::c_int as u32_0);
    GfxPrint_Printf(&mut printer as *mut GfxPrint,
                    b"%02d\x00" as *const u8 as *const libc::c_char,
                    ((24 as libc::c_int * 60 as libc::c_int) as libc::c_float
                         / 0x10000 as libc::c_int as f32_0 *
                         gSaveContext.skyboxTime as libc::c_int as
                             libc::c_float / 60.0f32) as u8_0 as libc::c_int);
    if gSaveContext.skyboxTime as libc::c_int & 0x1f as libc::c_int >=
           0x10 as libc::c_int ||
           gTimeIncrement as libc::c_int >= 6 as libc::c_int {
        GfxPrint_Printf(&mut printer as *mut GfxPrint,
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        b":\x00" as *const u8 as *const libc::c_char);
    } else {
        GfxPrint_Printf(&mut printer as *mut GfxPrint,
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        b" \x00" as *const u8 as *const libc::c_char);
    }
    GfxPrint_Printf(&mut printer as *mut GfxPrint,
                    b"%02d\x00" as *const u8 as *const libc::c_char,
                    ((24 as libc::c_int * 60 as libc::c_int) as libc::c_float
                         / 0x10000 as libc::c_int as f32_0 *
                         gSaveContext.skyboxTime as libc::c_int as
                             libc::c_float) as s16 as libc::c_int %
                        60 as libc::c_int);
    GfxPrint_SetColor(&mut printer, 55 as libc::c_int as u32_0,
                      255 as libc::c_int as u32_0,
                      255 as libc::c_int as u32_0,
                      64 as libc::c_int as u32_0);
    GfxPrint_SetPos(&mut printer, 22 as libc::c_int, 6 as libc::c_int);
    if gSaveContext.nightFlag != 0 {
        GfxPrint_Printf(&mut printer as *mut GfxPrint,
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        b"YORU\x00" as *const u8 as *const libc::c_char);
        // "night"
    } else {
        GfxPrint_Printf(&mut printer as *mut GfxPrint,
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        b"HIRU\x00" as *const u8 as *const libc::c_char);
        // "day"
    } // increments or decrements unk_EE[1] depending on some condition
    *gfx =
        GfxPrint_Close(&mut printer); // updates bgm/sfx and other things as the day progresses
    GfxPrint_Destroy(&mut printer);
}
#[no_mangle]
pub unsafe extern "C" fn Environment_Update(mut globalCtx: *mut GlobalContext,
                                            mut envCtx:
                                                *mut EnvironmentContext,
                                            mut lightCtx: *mut LightContext,
                                            mut pauseCtx: *mut PauseContext,
                                            mut msgCtx: *mut MessageContext,
                                            mut gameOverCtx:
                                                *mut GameOverContext,
                                            mut gfxCtx:
                                                *mut GraphicsContext) {
    let mut sp8C: f32_0 = 0.;
    let mut sp88: f32_0 = 0.0f32;
    let mut i: u16_0 = 0;
    let mut j: u16_0 = 0;
    let mut time: u16_0 = 0;
    let mut lightSettingsList: *mut EnvLightSettings =
        (*globalCtx).envCtx.lightSettingsList;
    let mut adjustment: s32 = 0;
    if gSaveContext.gameMode != 0 as libc::c_int &&
           gSaveContext.gameMode != 3 as libc::c_int {
        func_800AA16C(globalCtx);
    }
    if (*pauseCtx).state as libc::c_int == 0 as libc::c_int {
        if (*globalCtx).pauseCtx.state as libc::c_int == 0 as libc::c_int &&
               (*globalCtx).pauseCtx.debugState as libc::c_int ==
                   0 as libc::c_int {
            if (*globalCtx).skyboxId as libc::c_int ==
                   SKYBOX_NORMAL_SKY as libc::c_int {
                (*globalCtx).skyboxCtx.rot.y -= 0.001f32
            } else if (*globalCtx).skyboxId as libc::c_int ==
                          SKYBOX_CUTSCENE_MAP as libc::c_int {
                (*globalCtx).skyboxCtx.rot.y -= 0.005f32
            }
        }
        func_800766C4(globalCtx);
        func_80075B44(globalCtx);
        if gSaveContext.nextDayTime as libc::c_int >= 0xff00 as libc::c_int &&
               gSaveContext.nextDayTime as libc::c_int !=
                   0xffff as libc::c_int {
            gSaveContext.nextDayTime =
                (gSaveContext.nextDayTime as libc::c_int -
                     0x10 as libc::c_int) as u16_0;
            osSyncPrintf(b"\nnext_zelda_time=[%x]\x00" as *const u8 as
                             *const libc::c_char,
                         gSaveContext.nextDayTime as libc::c_int);
            if gSaveContext.nextDayTime as libc::c_int ==
                   0xff0e as libc::c_int {
                func_80078884(0x2813 as libc::c_int as u16_0);
                gSaveContext.nextDayTime = 0xffff as libc::c_int as u16_0
            } else if gSaveContext.nextDayTime as libc::c_int ==
                          0xff0d as libc::c_int {
                func_800788CC(0x28ae as libc::c_int as u16_0);
                gSaveContext.nextDayTime = 0xffff as libc::c_int as u16_0
            }
        }
        if (*pauseCtx).state as libc::c_int == 0 as libc::c_int &&
               (*gameOverCtx).state as libc::c_int ==
                   GAMEOVER_INACTIVE as libc::c_int {
            if (*msgCtx).msgLength == 0 as libc::c_int &&
                   (*msgCtx).msgMode as libc::c_int == 0 as libc::c_int ||
                   gSaveContext.gameMode == 3 as libc::c_int {
                if (*envCtx).unk_1A as libc::c_int == 0 as libc::c_int &&
                       FrameAdvance_IsEnabled(globalCtx) == 0 &&
                       ((*globalCtx).transitionMode as libc::c_int ==
                            0 as libc::c_int ||
                            gSaveContext.gameMode != 0 as libc::c_int) {
                    if gSaveContext.nightFlag == 0 ||
                           gTimeIncrement as libc::c_int >=
                               0x190 as libc::c_int {
                        gSaveContext.dayTime =
                            (gSaveContext.dayTime as libc::c_int +
                                 gTimeIncrement as libc::c_int) as u16_0
                    } else {
                        gSaveContext.dayTime =
                            (gSaveContext.dayTime as libc::c_int +
                                 gTimeIncrement as libc::c_int *
                                     2 as libc::c_int) as u16_0
                        // time moves twice as fast at night
                    }
                }
            }
        }
        // ! @bug `gTimeIncrement` is unsigned, it can't be negative
        if (gSaveContext.sceneSetupIndex >= 5 as libc::c_int ||
                gTimeIncrement as libc::c_int != 0 as libc::c_int) &&
               gSaveContext.dayTime as libc::c_int >
                   gSaveContext.skyboxTime as libc::c_int ||
               ((gSaveContext.dayTime as libc::c_int) < 0xaab as libc::c_int
                    || (gTimeIncrement as libc::c_int) < 0 as libc::c_int) {
            gSaveContext.skyboxTime = gSaveContext.dayTime
        }
        time = gSaveContext.dayTime;
        if time as libc::c_int > 0xc000 as libc::c_int ||
               (time as libc::c_int) < 0x4555 as libc::c_int {
            gSaveContext.nightFlag = 1 as libc::c_int
        } else { gSaveContext.nightFlag = 0 as libc::c_int }
        if (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 0 as libc::c_int) as
                                 usize] as libc::c_int != 0 as libc::c_int ||
               (*gGameInfo).data[(11 as libc::c_int * 6 as libc::c_int *
                                      16 as libc::c_int + 2 as libc::c_int) as
                                     usize] as libc::c_int != 0 as libc::c_int
           {
            let mut displayList: *mut Gfx = 0 as *mut Gfx;
            let mut prevDisplayList: *mut Gfx = 0 as *mut Gfx;
            let mut __gfxCtx: *mut GraphicsContext =
                0 as *mut GraphicsContext;
            let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
            __gfxCtx = (*globalCtx).state.gfxCtx;
            Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                            b"../z_kankyo.c\x00" as *const u8 as
                                *const libc::c_char, 1682 as libc::c_int);
            prevDisplayList = (*__gfxCtx).polyOpa.p;
            displayList = Graph_GfxPlusOne((*__gfxCtx).polyOpa.p);
            let fresh1 = (*__gfxCtx).overlay.p;
            (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
            let mut _g: *mut Gfx = fresh1;
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
            Environment_PrintDebugInfo(globalCtx, &mut displayList);
            let fresh2 = displayList;
            displayList = displayList.offset(1);
            let mut _g_0: *mut Gfx = fresh2;
            (*_g_0).words.w0 =
                (0xdf as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_0).words.w1 = 0 as libc::c_int as libc::c_uint;
            Graph_BranchDlist(prevDisplayList, displayList);
            (*__gfxCtx).polyOpa.p = displayList;
            Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                             b"../z_kankyo.c\x00" as *const u8 as
                                 *const libc::c_char, 1690 as libc::c_int);
        }
        if (*envCtx).unk_BF as libc::c_int != 0xff as libc::c_int &&
               (*envCtx).unk_DC as libc::c_int != 2 as libc::c_int &&
               (*envCtx).unk_BD as libc::c_int !=
                   (*envCtx).unk_BF as libc::c_int &&
               (*envCtx).unk_D8 >= 1.0f32 &&
               ((*envCtx).unk_BF as libc::c_int) < 0x20 as libc::c_int {
            (*envCtx).unk_BE = (*envCtx).unk_BD;
            (*envCtx).unk_BD = (*envCtx).unk_BF;
            (*envCtx).unk_D8 = 0.0f32
        }
        if (*envCtx).unk_BF as libc::c_int != 0xfe as libc::c_int {
            if (*envCtx).indoors == 0 &&
                   (*envCtx).unk_BF as libc::c_int == 0xff as libc::c_int {
                i = 0 as libc::c_int as u16_0;
                while (i as libc::c_int) <
                          (::std::mem::size_of::<[struct_8011FB48; 7]>() as
                               libc::c_ulong).wrapping_div(::std::mem::size_of::<struct_8011FB48>()
                                                               as
                                                               libc::c_ulong)
                              as s32 {
                    if gSaveContext.skyboxTime as libc::c_int >=
                           D_8011FB48[(*envCtx).unk_1F as
                                          usize][i as usize].startTime as
                               libc::c_int &&
                           ((gSaveContext.skyboxTime as libc::c_int) <
                                D_8011FB48[(*envCtx).unk_1F as
                                               usize][i as usize].endTime as
                                    libc::c_int ||
                                D_8011FB48[(*envCtx).unk_1F as
                                               usize][i as usize].endTime as
                                    libc::c_int == 0xffff as libc::c_int) {
                        let mut blend8: [u8_0; 2] = [0; 2];
                        let mut blend16: [s16; 2] = [0; 2];
                        sp8C =
                            Environment_LerpWeight(D_8011FB48[(*envCtx).unk_1F
                                                                  as
                                                                  usize][i as
                                                                             usize].endTime,
                                                   D_8011FB48[(*envCtx).unk_1F
                                                                  as
                                                                  usize][i as
                                                                             usize].startTime,
                                                   gSaveContext.skyboxTime);
                        D_8011FDCC =
                            (D_8011FB48[(*envCtx).unk_1F as
                                            usize][i as usize].unk_04 as
                                 libc::c_int & 3 as libc::c_int) as u8_0;
                        D_8011FDD0 =
                            (D_8011FB48[(*envCtx).unk_1F as
                                            usize][i as usize].unk_05 as
                                 libc::c_int & 3 as libc::c_int) as u8_0;
                        D_8011FDD4 = sp8C;
                        if (*envCtx).unk_21 != 0 {
                            sp88 =
                                ((*envCtx).unk_24 as f32_0 -
                                     (*envCtx).unk_22 as libc::c_int as
                                         libc::c_float) /
                                    (*envCtx).unk_24 as libc::c_int as
                                        libc::c_float;
                            (*envCtx).unk_22 =
                                (*envCtx).unk_22.wrapping_sub(1);
                            if (*envCtx).unk_22 as libc::c_int <=
                                   0 as libc::c_int {
                                (*envCtx).unk_21 = 0 as libc::c_int as u8_0;
                                (*envCtx).unk_1F = (*envCtx).unk_20
                            }
                        }
                        j = 0 as libc::c_int as u16_0;
                        while (j as libc::c_int) < 3 as libc::c_int {
                            // blend ambient color
                            blend8[0 as libc::c_int as usize] =
                                (((*lightSettingsList.offset(D_8011FB48[(*envCtx).unk_1F
                                                                            as
                                                                            usize][i
                                                                                       as
                                                                                       usize].unk_05
                                                                 as
                                                                 isize)).ambientColor[j
                                                                                          as
                                                                                          usize]
                                      as libc::c_int -
                                      (*lightSettingsList.offset(D_8011FB48[(*envCtx).unk_1F
                                                                                as
                                                                                usize][i
                                                                                           as
                                                                                           usize].unk_04
                                                                     as
                                                                     isize)).ambientColor[j
                                                                                              as
                                                                                              usize]
                                          as libc::c_int) as libc::c_float *
                                     sp8C +
                                     (*lightSettingsList.offset(D_8011FB48[(*envCtx).unk_1F
                                                                               as
                                                                               usize][i
                                                                                          as
                                                                                          usize].unk_04
                                                                    as
                                                                    isize)).ambientColor[j
                                                                                             as
                                                                                             usize]
                                         as libc::c_int as libc::c_float) as
                                    u8_0;
                            blend8[1 as libc::c_int as usize] =
                                (((*lightSettingsList.offset(D_8011FB48[(*envCtx).unk_20
                                                                            as
                                                                            usize][i
                                                                                       as
                                                                                       usize].unk_05
                                                                 as
                                                                 isize)).ambientColor[j
                                                                                          as
                                                                                          usize]
                                      as libc::c_int -
                                      (*lightSettingsList.offset(D_8011FB48[(*envCtx).unk_20
                                                                                as
                                                                                usize][i
                                                                                           as
                                                                                           usize].unk_04
                                                                     as
                                                                     isize)).ambientColor[j
                                                                                              as
                                                                                              usize]
                                          as libc::c_int) as libc::c_float *
                                     sp8C +
                                     (*lightSettingsList.offset(D_8011FB48[(*envCtx).unk_20
                                                                               as
                                                                               usize][i
                                                                                          as
                                                                                          usize].unk_04
                                                                    as
                                                                    isize)).ambientColor[j
                                                                                             as
                                                                                             usize]
                                         as libc::c_int as libc::c_float) as
                                    u8_0;
                            *(*envCtx).lightSettings.ambientColor.as_mut_ptr().offset(j
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          isize)
                                =
                                ((blend8[1 as libc::c_int as usize] as
                                      libc::c_int -
                                      blend8[0 as libc::c_int as usize] as
                                          libc::c_int) as libc::c_float * sp88
                                     +
                                     blend8[0 as libc::c_int as usize] as
                                         libc::c_int as libc::c_float) as
                                    u8_0;
                            j = j.wrapping_add(1)
                        }
                        // set light1 direction for the sun
                        (*envCtx).lightSettings.light1Dir[0 as libc::c_int as
                                                              usize] =
                            -(Math_SinS((gSaveContext.dayTime as libc::c_int -
                                             0x8000 as libc::c_int) as s16) *
                                  120.0f32) as s8;
                        (*envCtx).lightSettings.light1Dir[1 as libc::c_int as
                                                              usize] =
                            (Math_CosS((gSaveContext.dayTime as libc::c_int -
                                            0x8000 as libc::c_int) as s16) *
                                 120.0f32) as s8;
                        (*envCtx).lightSettings.light1Dir[2 as libc::c_int as
                                                              usize] =
                            (Math_CosS((gSaveContext.dayTime as libc::c_int -
                                            0x8000 as libc::c_int) as s16) *
                                 20.0f32) as s8;
                        // set light2 direction for the moon
                        (*envCtx).lightSettings.light2Dir[0 as libc::c_int as
                                                              usize] =
                            -((*envCtx).lightSettings.light1Dir[0 as
                                                                    libc::c_int
                                                                    as usize]
                                  as libc::c_int) as s8;
                        (*envCtx).lightSettings.light2Dir[1 as libc::c_int as
                                                              usize] =
                            -((*envCtx).lightSettings.light1Dir[1 as
                                                                    libc::c_int
                                                                    as usize]
                                  as libc::c_int) as s8;
                        (*envCtx).lightSettings.light2Dir[2 as libc::c_int as
                                                              usize] =
                            -((*envCtx).lightSettings.light1Dir[2 as
                                                                    libc::c_int
                                                                    as usize]
                                  as libc::c_int) as s8;
                        j = 0 as libc::c_int as u16_0;
                        while (j as libc::c_int) < 3 as libc::c_int {
                            // blend light1Color
                            blend8[0 as libc::c_int as usize] =
                                (((*lightSettingsList.offset(D_8011FB48[(*envCtx).unk_1F
                                                                            as
                                                                            usize][i
                                                                                       as
                                                                                       usize].unk_05
                                                                 as
                                                                 isize)).light1Color[j
                                                                                         as
                                                                                         usize]
                                      as libc::c_int -
                                      (*lightSettingsList.offset(D_8011FB48[(*envCtx).unk_1F
                                                                                as
                                                                                usize][i
                                                                                           as
                                                                                           usize].unk_04
                                                                     as
                                                                     isize)).light1Color[j
                                                                                             as
                                                                                             usize]
                                          as libc::c_int) as libc::c_float *
                                     sp8C +
                                     (*lightSettingsList.offset(D_8011FB48[(*envCtx).unk_1F
                                                                               as
                                                                               usize][i
                                                                                          as
                                                                                          usize].unk_04
                                                                    as
                                                                    isize)).light1Color[j
                                                                                            as
                                                                                            usize]
                                         as libc::c_int as libc::c_float) as
                                    u8_0;
                            blend8[1 as libc::c_int as usize] =
                                (((*lightSettingsList.offset(D_8011FB48[(*envCtx).unk_20
                                                                            as
                                                                            usize][i
                                                                                       as
                                                                                       usize].unk_05
                                                                 as
                                                                 isize)).light1Color[j
                                                                                         as
                                                                                         usize]
                                      as libc::c_int -
                                      (*lightSettingsList.offset(D_8011FB48[(*envCtx).unk_20
                                                                                as
                                                                                usize][i
                                                                                           as
                                                                                           usize].unk_04
                                                                     as
                                                                     isize)).light1Color[j
                                                                                             as
                                                                                             usize]
                                          as libc::c_int) as libc::c_float *
                                     sp8C +
                                     (*lightSettingsList.offset(D_8011FB48[(*envCtx).unk_20
                                                                               as
                                                                               usize][i
                                                                                          as
                                                                                          usize].unk_04
                                                                    as
                                                                    isize)).light1Color[j
                                                                                            as
                                                                                            usize]
                                         as libc::c_int as libc::c_float) as
                                    u8_0;
                            *(*envCtx).lightSettings.light1Color.as_mut_ptr().offset(j
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         isize)
                                =
                                ((blend8[1 as libc::c_int as usize] as
                                      libc::c_int -
                                      blend8[0 as libc::c_int as usize] as
                                          libc::c_int) as libc::c_float * sp88
                                     +
                                     blend8[0 as libc::c_int as usize] as
                                         libc::c_int as libc::c_float) as
                                    u8_0;
                            // blend light2Color
                            blend8[0 as libc::c_int as usize] =
                                (((*lightSettingsList.offset(D_8011FB48[(*envCtx).unk_1F
                                                                            as
                                                                            usize][i
                                                                                       as
                                                                                       usize].unk_05
                                                                 as
                                                                 isize)).light2Color[j
                                                                                         as
                                                                                         usize]
                                      as libc::c_int -
                                      (*lightSettingsList.offset(D_8011FB48[(*envCtx).unk_1F
                                                                                as
                                                                                usize][i
                                                                                           as
                                                                                           usize].unk_04
                                                                     as
                                                                     isize)).light2Color[j
                                                                                             as
                                                                                             usize]
                                          as libc::c_int) as libc::c_float *
                                     sp8C +
                                     (*lightSettingsList.offset(D_8011FB48[(*envCtx).unk_1F
                                                                               as
                                                                               usize][i
                                                                                          as
                                                                                          usize].unk_04
                                                                    as
                                                                    isize)).light2Color[j
                                                                                            as
                                                                                            usize]
                                         as libc::c_int as libc::c_float) as
                                    u8_0;
                            blend8[1 as libc::c_int as usize] =
                                (((*lightSettingsList.offset(D_8011FB48[(*envCtx).unk_20
                                                                            as
                                                                            usize][i
                                                                                       as
                                                                                       usize].unk_05
                                                                 as
                                                                 isize)).light2Color[j
                                                                                         as
                                                                                         usize]
                                      as libc::c_int -
                                      (*lightSettingsList.offset(D_8011FB48[(*envCtx).unk_20
                                                                                as
                                                                                usize][i
                                                                                           as
                                                                                           usize].unk_04
                                                                     as
                                                                     isize)).light2Color[j
                                                                                             as
                                                                                             usize]
                                          as libc::c_int) as libc::c_float *
                                     sp8C +
                                     (*lightSettingsList.offset(D_8011FB48[(*envCtx).unk_20
                                                                               as
                                                                               usize][i
                                                                                          as
                                                                                          usize].unk_04
                                                                    as
                                                                    isize)).light2Color[j
                                                                                            as
                                                                                            usize]
                                         as libc::c_int as libc::c_float) as
                                    u8_0;
                            *(*envCtx).lightSettings.light2Color.as_mut_ptr().offset(j
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         isize)
                                =
                                ((blend8[1 as libc::c_int as usize] as
                                      libc::c_int -
                                      blend8[0 as libc::c_int as usize] as
                                          libc::c_int) as libc::c_float * sp88
                                     +
                                     blend8[0 as libc::c_int as usize] as
                                         libc::c_int as libc::c_float) as
                                    u8_0;
                            j = j.wrapping_add(1)
                        }
                        // blend fogColor
                        j = 0 as libc::c_int as u16_0;
                        while (j as libc::c_int) < 3 as libc::c_int {
                            blend8[0 as libc::c_int as usize] =
                                (((*lightSettingsList.offset(D_8011FB48[(*envCtx).unk_1F
                                                                            as
                                                                            usize][i
                                                                                       as
                                                                                       usize].unk_05
                                                                 as
                                                                 isize)).fogColor[j
                                                                                      as
                                                                                      usize]
                                      as libc::c_int -
                                      (*lightSettingsList.offset(D_8011FB48[(*envCtx).unk_1F
                                                                                as
                                                                                usize][i
                                                                                           as
                                                                                           usize].unk_04
                                                                     as
                                                                     isize)).fogColor[j
                                                                                          as
                                                                                          usize]
                                          as libc::c_int) as libc::c_float *
                                     sp8C +
                                     (*lightSettingsList.offset(D_8011FB48[(*envCtx).unk_1F
                                                                               as
                                                                               usize][i
                                                                                          as
                                                                                          usize].unk_04
                                                                    as
                                                                    isize)).fogColor[j
                                                                                         as
                                                                                         usize]
                                         as libc::c_int as libc::c_float) as
                                    u8_0;
                            blend8[1 as libc::c_int as usize] =
                                (((*lightSettingsList.offset(D_8011FB48[(*envCtx).unk_20
                                                                            as
                                                                            usize][i
                                                                                       as
                                                                                       usize].unk_05
                                                                 as
                                                                 isize)).fogColor[j
                                                                                      as
                                                                                      usize]
                                      as libc::c_int -
                                      (*lightSettingsList.offset(D_8011FB48[(*envCtx).unk_20
                                                                                as
                                                                                usize][i
                                                                                           as
                                                                                           usize].unk_04
                                                                     as
                                                                     isize)).fogColor[j
                                                                                          as
                                                                                          usize]
                                          as libc::c_int) as libc::c_float *
                                     sp8C +
                                     (*lightSettingsList.offset(D_8011FB48[(*envCtx).unk_20
                                                                               as
                                                                               usize][i
                                                                                          as
                                                                                          usize].unk_04
                                                                    as
                                                                    isize)).fogColor[j
                                                                                         as
                                                                                         usize]
                                         as libc::c_int as libc::c_float) as
                                    u8_0;
                            *(*envCtx).lightSettings.fogColor.as_mut_ptr().offset(j
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      isize)
                                =
                                ((blend8[1 as libc::c_int as usize] as
                                      libc::c_int -
                                      blend8[0 as libc::c_int as usize] as
                                          libc::c_int) as libc::c_float * sp88
                                     +
                                     blend8[0 as libc::c_int as usize] as
                                         libc::c_int as libc::c_float) as
                                    u8_0;
                            j = j.wrapping_add(1)
                        }
                        blend16[0 as libc::c_int as usize] =
                            (((((*lightSettingsList.offset(D_8011FB48[(*envCtx).unk_1F
                                                                          as
                                                                          usize][i
                                                                                     as
                                                                                     usize].unk_05
                                                               as
                                                               isize)).fogNear
                                    as libc::c_int & 0x3ff as libc::c_int) -
                                   ((*lightSettingsList.offset(D_8011FB48[(*envCtx).unk_1F
                                                                              as
                                                                              usize][i
                                                                                         as
                                                                                         usize].unk_04
                                                                   as
                                                                   isize)).fogNear
                                        as libc::c_int &
                                        0x3ff as libc::c_int)) as
                                  libc::c_float * sp8C) as s16 as libc::c_int
                                 +
                                 ((*lightSettingsList.offset(D_8011FB48[(*envCtx).unk_1F
                                                                            as
                                                                            usize][i
                                                                                       as
                                                                                       usize].unk_04
                                                                 as
                                                                 isize)).fogNear
                                      as libc::c_int & 0x3ff as libc::c_int))
                                as s16;
                        blend16[1 as libc::c_int as usize] =
                            (((((*lightSettingsList.offset(D_8011FB48[(*envCtx).unk_20
                                                                          as
                                                                          usize][i
                                                                                     as
                                                                                     usize].unk_05
                                                               as
                                                               isize)).fogNear
                                    as libc::c_int & 0x3ff as libc::c_int) -
                                   ((*lightSettingsList.offset(D_8011FB48[(*envCtx).unk_20
                                                                              as
                                                                              usize][i
                                                                                         as
                                                                                         usize].unk_04
                                                                   as
                                                                   isize)).fogNear
                                        as libc::c_int &
                                        0x3ff as libc::c_int)) as
                                  libc::c_float * sp8C) as s16 as libc::c_int
                                 +
                                 ((*lightSettingsList.offset(D_8011FB48[(*envCtx).unk_20
                                                                            as
                                                                            usize][i
                                                                                       as
                                                                                       usize].unk_04
                                                                 as
                                                                 isize)).fogNear
                                      as libc::c_int & 0x3ff as libc::c_int))
                                as s16;
                        (*envCtx).lightSettings.fogNear =
                            (((blend16[1 as libc::c_int as usize] as
                                   libc::c_int -
                                   blend16[0 as libc::c_int as usize] as
                                       libc::c_int) as libc::c_float * sp88)
                                 as s16 as libc::c_int +
                                 blend16[0 as libc::c_int as usize] as
                                     libc::c_int) as s16;
                        blend16[0 as libc::c_int as usize] =
                            ((((*lightSettingsList.offset(D_8011FB48[(*envCtx).unk_1F
                                                                         as
                                                                         usize][i
                                                                                    as
                                                                                    usize].unk_05
                                                              as
                                                              isize)).fogFar
                                   as libc::c_int -
                                   (*lightSettingsList.offset(D_8011FB48[(*envCtx).unk_1F
                                                                             as
                                                                             usize][i
                                                                                        as
                                                                                        usize].unk_04
                                                                  as
                                                                  isize)).fogFar
                                       as libc::c_int) as libc::c_float *
                                  sp8C) as s16 as libc::c_int +
                                 (*lightSettingsList.offset(D_8011FB48[(*envCtx).unk_1F
                                                                           as
                                                                           usize][i
                                                                                      as
                                                                                      usize].unk_04
                                                                as
                                                                isize)).fogFar
                                     as libc::c_int) as s16;
                        blend16[1 as libc::c_int as usize] =
                            ((((*lightSettingsList.offset(D_8011FB48[(*envCtx).unk_20
                                                                         as
                                                                         usize][i
                                                                                    as
                                                                                    usize].unk_05
                                                              as
                                                              isize)).fogFar
                                   as libc::c_int -
                                   (*lightSettingsList.offset(D_8011FB48[(*envCtx).unk_20
                                                                             as
                                                                             usize][i
                                                                                        as
                                                                                        usize].unk_04
                                                                  as
                                                                  isize)).fogFar
                                       as libc::c_int) as libc::c_float *
                                  sp8C) as s16 as libc::c_int +
                                 (*lightSettingsList.offset(D_8011FB48[(*envCtx).unk_20
                                                                           as
                                                                           usize][i
                                                                                      as
                                                                                      usize].unk_04
                                                                as
                                                                isize)).fogFar
                                     as libc::c_int) as s16;
                        (*envCtx).lightSettings.fogFar =
                            (((blend16[1 as libc::c_int as usize] as
                                   libc::c_int -
                                   blend16[0 as libc::c_int as usize] as
                                       libc::c_int) as libc::c_float * sp88)
                                 as s16 as libc::c_int +
                                 blend16[0 as libc::c_int as usize] as
                                     libc::c_int) as s16;
                        if D_8011FB48[(*envCtx).unk_20 as
                                          usize][i as usize].unk_05 as
                               libc::c_int >=
                               (*envCtx).numLightSettings as libc::c_int {
                            // "The color palette setting seems to be wrong!"
                            osSyncPrintf(b"\x1b[41;37m\n\xe3\x82\xab\xe3\x83\xa9\xe3\x83\xbc\xe3\x83\x91\xe3\x83\xac\xe3\x83\x83\xe3\x83\x88\xe3\x81\xae\xe8\xa8\xad\xe5\xae\x9a\xe3\x81\x8c\xe3\x81\x8a\xe3\x81\x8b\xe3\x81\x97\xe3\x81\x84\xe3\x82\x88\xe3\x81\x86\xe3\x81\xa7\xe3\x81\x99\xef\xbc\x81\x1b[m\x00"
                                             as *const u8 as
                                             *const libc::c_char);
                            // "Palette setting = [] Last palette number = []"
                            osSyncPrintf(b"\x1b[41;37m\n\xe8\xa8\xad\xe5\xae\x9a\xe3\x83\x91\xe3\x83\xac\xe3\x83\x83\xe3\x83\x88\xef\xbc\x9d[%d] \xe6\x9c\x80\xe5\xbe\x8c\xe3\x83\x91\xe3\x83\xac\xe3\x83\x83\xe3\x83\x88\xe7\x95\xaa\xe5\x8f\xb7\xef\xbc\x9d[%d]\n\x1b[m\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                         D_8011FB48[(*envCtx).unk_20 as
                                                        usize][i as
                                                                   usize].unk_05
                                             as libc::c_int,
                                         (*envCtx).numLightSettings as
                                             libc::c_int - 1 as libc::c_int);
                        }
                        break ;
                    } else { i = i.wrapping_add(1) }
                }
            } else {
                if (*envCtx).blendIndoorLights == 0 {
                    i = 0 as libc::c_int as u16_0;
                    while (i as libc::c_int) < 3 as libc::c_int {
                        (*envCtx).lightSettings.ambientColor[i as usize] =
                            (*lightSettingsList.offset((*envCtx).unk_BD as
                                                           isize)).ambientColor[i
                                                                                    as
                                                                                    usize];
                        (*envCtx).lightSettings.light1Dir[i as usize] =
                            (*lightSettingsList.offset((*envCtx).unk_BD as
                                                           isize)).light1Dir[i
                                                                                 as
                                                                                 usize];
                        (*envCtx).lightSettings.light1Color[i as usize] =
                            (*lightSettingsList.offset((*envCtx).unk_BD as
                                                           isize)).light1Color[i
                                                                                   as
                                                                                   usize];
                        (*envCtx).lightSettings.light2Dir[i as usize] =
                            (*lightSettingsList.offset((*envCtx).unk_BD as
                                                           isize)).light2Dir[i
                                                                                 as
                                                                                 usize];
                        (*envCtx).lightSettings.light2Color[i as usize] =
                            (*lightSettingsList.offset((*envCtx).unk_BD as
                                                           isize)).light2Color[i
                                                                                   as
                                                                                   usize];
                        (*envCtx).lightSettings.fogColor[i as usize] =
                            (*lightSettingsList.offset((*envCtx).unk_BD as
                                                           isize)).fogColor[i
                                                                                as
                                                                                usize];
                        i = i.wrapping_add(1)
                    }
                    (*envCtx).lightSettings.fogNear =
                        ((*lightSettingsList.offset((*envCtx).unk_BD as
                                                        isize)).fogNear as
                             libc::c_int & 0x3ff as libc::c_int) as s16;
                    (*envCtx).lightSettings.fogFar =
                        (*lightSettingsList.offset((*envCtx).unk_BD as
                                                       isize)).fogFar;
                    (*envCtx).unk_D8 = 1.0f32
                } else {
                    let mut blendRate: u8_0 =
                        (((*lightSettingsList.offset((*envCtx).unk_BD as
                                                         isize)).fogNear as
                              libc::c_int >> 0xa as libc::c_int) *
                             4 as libc::c_int) as u8_0;
                    if blendRate as libc::c_int == 0 as libc::c_int {
                        blendRate = blendRate.wrapping_add(1)
                    }
                    if (*envCtx).unk_D6 as libc::c_int !=
                           0xffff as libc::c_int {
                        blendRate = (*envCtx).unk_D6 as u8_0
                    }
                    if (*envCtx).unk_DC as libc::c_int == 0 as libc::c_int {
                        (*envCtx).unk_D8 +=
                            blendRate as libc::c_int as libc::c_float /
                                255.0f32
                    }
                    if (*envCtx).unk_D8 > 1.0f32 { (*envCtx).unk_D8 = 1.0f32 }
                    i = 0 as libc::c_int as u16_0;
                    while (i as libc::c_int) < 3 as libc::c_int {
                        (*envCtx).lightSettings.ambientColor[i as usize] =
                            (((*lightSettingsList.offset((*envCtx).unk_BD as
                                                             isize)).ambientColor[i
                                                                                      as
                                                                                      usize]
                                  as libc::c_int -
                                  (*lightSettingsList.offset((*envCtx).unk_BE
                                                                 as
                                                                 isize)).ambientColor[i
                                                                                          as
                                                                                          usize]
                                      as libc::c_int) as libc::c_float *
                                 (*envCtx).unk_D8 +
                                 (*lightSettingsList.offset((*envCtx).unk_BE
                                                                as
                                                                isize)).ambientColor[i
                                                                                         as
                                                                                         usize]
                                     as libc::c_int as libc::c_float) as u8_0;
                        (*envCtx).lightSettings.light1Dir[i as usize] =
                            ((((*lightSettingsList.offset((*envCtx).unk_BD as
                                                              isize)).light1Dir[i
                                                                                    as
                                                                                    usize]
                                   as libc::c_int -
                                   (*lightSettingsList.offset((*envCtx).unk_BE
                                                                  as
                                                                  isize)).light1Dir[i
                                                                                        as
                                                                                        usize]
                                       as libc::c_int) as libc::c_float *
                                  (*envCtx).unk_D8) as s16 as libc::c_int +
                                 (*lightSettingsList.offset((*envCtx).unk_BE
                                                                as
                                                                isize)).light1Dir[i
                                                                                      as
                                                                                      usize]
                                     as libc::c_int) as s8;
                        (*envCtx).lightSettings.light1Color[i as usize] =
                            (((*lightSettingsList.offset((*envCtx).unk_BD as
                                                             isize)).light1Color[i
                                                                                     as
                                                                                     usize]
                                  as libc::c_int -
                                  (*lightSettingsList.offset((*envCtx).unk_BE
                                                                 as
                                                                 isize)).light1Color[i
                                                                                         as
                                                                                         usize]
                                      as libc::c_int) as libc::c_float *
                                 (*envCtx).unk_D8 +
                                 (*lightSettingsList.offset((*envCtx).unk_BE
                                                                as
                                                                isize)).light1Color[i
                                                                                        as
                                                                                        usize]
                                     as libc::c_int as libc::c_float) as u8_0;
                        (*envCtx).lightSettings.light2Dir[i as usize] =
                            ((((*lightSettingsList.offset((*envCtx).unk_BD as
                                                              isize)).light2Dir[i
                                                                                    as
                                                                                    usize]
                                   as libc::c_int -
                                   (*lightSettingsList.offset((*envCtx).unk_BE
                                                                  as
                                                                  isize)).light2Dir[i
                                                                                        as
                                                                                        usize]
                                       as libc::c_int) as libc::c_float *
                                  (*envCtx).unk_D8) as s16 as libc::c_int +
                                 (*lightSettingsList.offset((*envCtx).unk_BE
                                                                as
                                                                isize)).light2Dir[i
                                                                                      as
                                                                                      usize]
                                     as libc::c_int) as s8;
                        (*envCtx).lightSettings.light2Color[i as usize] =
                            (((*lightSettingsList.offset((*envCtx).unk_BD as
                                                             isize)).light2Color[i
                                                                                     as
                                                                                     usize]
                                  as libc::c_int -
                                  (*lightSettingsList.offset((*envCtx).unk_BE
                                                                 as
                                                                 isize)).light2Color[i
                                                                                         as
                                                                                         usize]
                                      as libc::c_int) as libc::c_float *
                                 (*envCtx).unk_D8 +
                                 (*lightSettingsList.offset((*envCtx).unk_BE
                                                                as
                                                                isize)).light2Color[i
                                                                                        as
                                                                                        usize]
                                     as libc::c_int as libc::c_float) as u8_0;
                        (*envCtx).lightSettings.fogColor[i as usize] =
                            (((*lightSettingsList.offset((*envCtx).unk_BD as
                                                             isize)).fogColor[i
                                                                                  as
                                                                                  usize]
                                  as libc::c_int -
                                  (*lightSettingsList.offset((*envCtx).unk_BE
                                                                 as
                                                                 isize)).fogColor[i
                                                                                      as
                                                                                      usize]
                                      as libc::c_int) as libc::c_float *
                                 (*envCtx).unk_D8 +
                                 (*lightSettingsList.offset((*envCtx).unk_BE
                                                                as
                                                                isize)).fogColor[i
                                                                                     as
                                                                                     usize]
                                     as libc::c_int as libc::c_float) as u8_0;
                        i = i.wrapping_add(1)
                    }
                    (*envCtx).lightSettings.fogNear =
                        (((((*lightSettingsList.offset((*envCtx).unk_BD as
                                                           isize)).fogNear as
                                libc::c_int & 0x3ff as libc::c_int) -
                               ((*lightSettingsList.offset((*envCtx).unk_BE as
                                                               isize)).fogNear
                                    as libc::c_int & 0x3ff as libc::c_int)) as
                              libc::c_float * (*envCtx).unk_D8) as s16 as
                             libc::c_int +
                             ((*lightSettingsList.offset((*envCtx).unk_BE as
                                                             isize)).fogNear
                                  as libc::c_int & 0x3ff as libc::c_int)) as
                            s16;
                    (*envCtx).lightSettings.fogFar =
                        ((((*lightSettingsList.offset((*envCtx).unk_BD as
                                                          isize)).fogFar as
                               libc::c_int -
                               (*lightSettingsList.offset((*envCtx).unk_BE as
                                                              isize)).fogFar
                                   as libc::c_int) as libc::c_float *
                              (*envCtx).unk_D8) as s16 as libc::c_int +
                             (*lightSettingsList.offset((*envCtx).unk_BE as
                                                            isize)).fogFar as
                                 libc::c_int) as s16
                }
                if (*envCtx).unk_BD as libc::c_int >=
                       (*envCtx).numLightSettings as libc::c_int {
                    // "The color palette seems to be wrong!"
                    osSyncPrintf(b"\n\x1b[31m\xe3\x82\xab\xe3\x83\xa9\xe3\x83\xbc\xe3\x83\x91\xe3\x83\xac\xe3\x83\x83\xe3\x83\x88\xe3\x81\x8c\xe3\x81\x8a\xe3\x81\x8b\xe3\x81\x97\xe3\x81\x84\xe3\x82\x88\xe3\x81\x86\xe3\x81\xa7\xe3\x81\x99\xef\xbc\x81\x00"
                                     as *const u8 as *const libc::c_char);
                    // "Palette setting = [] Last palette number = []"
                    osSyncPrintf(b"\n\x1b[33m\xe8\xa8\xad\xe5\xae\x9a\xe3\x83\x91\xe3\x83\xac\xe3\x83\x83\xe3\x83\x88\xef\xbc\x9d[%d] \xe3\x83\x91\xe3\x83\xac\xe3\x83\x83\xe3\x83\x88\xe6\x95\xb0\xef\xbc\x9d[%d]\n\x1b[m\x00"
                                     as *const u8 as *const libc::c_char,
                                 (*envCtx).unk_BD as libc::c_int,
                                 (*envCtx).numLightSettings as libc::c_int);
                }
            }
        }
        (*envCtx).blendIndoorLights = 1 as libc::c_int as u8_0;
        // Apply lighting adjustments
        i = 0 as libc::c_int as u16_0;
        while (i as libc::c_int) < 3 as libc::c_int {
            if ((*envCtx).lightSettings.ambientColor[i as usize] as
                    libc::c_int +
                    (*envCtx).adjAmbientColor[i as usize] as libc::c_int) as
                   s16 as libc::c_int > 255 as libc::c_int {
                (*lightCtx).ambientColor[i as usize] =
                    255 as libc::c_int as u8_0
            } else if (((*envCtx).lightSettings.ambientColor[i as usize] as
                            libc::c_int +
                            (*envCtx).adjAmbientColor[i as usize] as
                                libc::c_int) as s16 as libc::c_int) <
                          0 as libc::c_int {
                (*lightCtx).ambientColor[i as usize] =
                    0 as libc::c_int as u8_0
            } else {
                (*lightCtx).ambientColor[i as usize] =
                    ((*envCtx).lightSettings.ambientColor[i as usize] as
                         libc::c_int +
                         (*envCtx).adjAmbientColor[i as usize] as libc::c_int)
                        as s16 as u8_0
            }
            if ((*envCtx).lightSettings.light1Color[i as usize] as libc::c_int
                    + (*envCtx).adjLight1Color[i as usize] as libc::c_int) as
                   s16 as libc::c_int > 255 as libc::c_int {
                (*envCtx).dirLight1.params.dir.color[i as usize] =
                    255 as libc::c_int as u8_0
            } else if (((*envCtx).lightSettings.light1Color[i as usize] as
                            libc::c_int +
                            (*envCtx).adjLight1Color[i as usize] as
                                libc::c_int) as s16 as libc::c_int) <
                          0 as libc::c_int {
                (*envCtx).dirLight1.params.dir.color[i as usize] =
                    0 as libc::c_int as u8_0
            } else {
                (*envCtx).dirLight1.params.dir.color[i as usize] =
                    ((*envCtx).lightSettings.light1Color[i as usize] as
                         libc::c_int +
                         (*envCtx).adjLight1Color[i as usize] as libc::c_int)
                        as s16 as u8_0
            }
            if ((*envCtx).lightSettings.light2Color[i as usize] as libc::c_int
                    + (*envCtx).adjLight1Color[i as usize] as libc::c_int) as
                   s16 as libc::c_int > 255 as libc::c_int {
                (*envCtx).dirLight2.params.dir.color[i as usize] =
                    255 as libc::c_int as u8_0
            } else if (((*envCtx).lightSettings.light2Color[i as usize] as
                            libc::c_int +
                            (*envCtx).adjLight1Color[i as usize] as
                                libc::c_int) as s16 as libc::c_int) <
                          0 as libc::c_int {
                (*envCtx).dirLight2.params.dir.color[i as usize] =
                    0 as libc::c_int as u8_0
            } else {
                (*envCtx).dirLight2.params.dir.color[i as usize] =
                    ((*envCtx).lightSettings.light2Color[i as usize] as
                         libc::c_int +
                         (*envCtx).adjLight1Color[i as usize] as libc::c_int)
                        as s16 as u8_0
            }
            if ((*envCtx).lightSettings.fogColor[i as usize] as libc::c_int +
                    (*envCtx).adjFogColor[i as usize] as libc::c_int) as s16
                   as libc::c_int > 255 as libc::c_int {
                (*lightCtx).fogColor[i as usize] = 255 as libc::c_int as u8_0
            } else if (((*envCtx).lightSettings.fogColor[i as usize] as
                            libc::c_int +
                            (*envCtx).adjFogColor[i as usize] as libc::c_int)
                           as s16 as libc::c_int) < 0 as libc::c_int {
                (*lightCtx).fogColor[i as usize] = 0 as libc::c_int as u8_0
            } else {
                (*lightCtx).fogColor[i as usize] =
                    ((*envCtx).lightSettings.fogColor[i as usize] as
                         libc::c_int +
                         (*envCtx).adjFogColor[i as usize] as libc::c_int) as
                        s16 as u8_0
            }
            i = i.wrapping_add(1)
        }
        // Set both directional light directions
        (*envCtx).dirLight1.params.dir.x =
            (*envCtx).lightSettings.light1Dir[0 as libc::c_int as usize];
        (*envCtx).dirLight1.params.dir.y =
            (*envCtx).lightSettings.light1Dir[1 as libc::c_int as usize];
        (*envCtx).dirLight1.params.dir.z =
            (*envCtx).lightSettings.light1Dir[2 as libc::c_int as usize];
        (*envCtx).dirLight2.params.dir.x =
            (*envCtx).lightSettings.light2Dir[0 as libc::c_int as usize];
        (*envCtx).dirLight2.params.dir.y =
            (*envCtx).lightSettings.light2Dir[1 as libc::c_int as usize];
        (*envCtx).dirLight2.params.dir.z =
            (*envCtx).lightSettings.light2Dir[2 as libc::c_int as usize];
        // Adjust fog near and far if necessary
        adjustment =
            (*envCtx).lightSettings.fogNear as libc::c_int +
                (*envCtx).adjFogNear as libc::c_int;
        if adjustment <= 996 as libc::c_int {
            (*lightCtx).fogNear = adjustment as s16
        } else { (*lightCtx).fogNear = 996 as libc::c_int as s16 }
        adjustment =
            (*envCtx).lightSettings.fogFar as libc::c_int +
                (*envCtx).adjFogFar as libc::c_int;
        if adjustment <= 12800 as libc::c_int {
            (*lightCtx).fogFar = adjustment as s16
        } else { (*lightCtx).fogFar = 12800 as libc::c_int as s16 }
        // When environment debug is enabled, various environment related variables can be configured via the reg editor
        if (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 9 as libc::c_int) as
                                 usize] != 0 {
            (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 0 as libc::c_int +
                                   0 as libc::c_int) as usize] =
                (*lightCtx).ambientColor[0 as libc::c_int as usize] as s16;
            (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 0 as libc::c_int +
                                   1 as libc::c_int) as usize] =
                (*lightCtx).ambientColor[1 as libc::c_int as usize] as s16;
            (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 0 as libc::c_int +
                                   2 as libc::c_int) as usize] =
                (*lightCtx).ambientColor[2 as libc::c_int as usize] as s16;
            (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 3 as libc::c_int +
                                   0 as libc::c_int) as usize] =
                (*envCtx).dirLight1.params.dir.color[0 as libc::c_int as
                                                         usize] as s16;
            (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 3 as libc::c_int +
                                   1 as libc::c_int) as usize] =
                (*envCtx).dirLight1.params.dir.color[1 as libc::c_int as
                                                         usize] as s16;
            (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 3 as libc::c_int +
                                   2 as libc::c_int) as usize] =
                (*envCtx).dirLight1.params.dir.color[2 as libc::c_int as
                                                         usize] as s16;
            (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 6 as libc::c_int +
                                   0 as libc::c_int) as usize] =
                (*envCtx).dirLight2.params.dir.color[0 as libc::c_int as
                                                         usize] as s16;
            (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 6 as libc::c_int +
                                   1 as libc::c_int) as usize] =
                (*envCtx).dirLight2.params.dir.color[1 as libc::c_int as
                                                         usize] as s16;
            (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 6 as libc::c_int +
                                   2 as libc::c_int) as usize] =
                (*envCtx).dirLight2.params.dir.color[2 as libc::c_int as
                                                         usize] as s16;
            (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 10 as libc::c_int +
                                   0 as libc::c_int) as usize] =
                (*lightCtx).fogColor[0 as libc::c_int as usize] as s16;
            (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 10 as libc::c_int +
                                   1 as libc::c_int) as usize] =
                (*lightCtx).fogColor[1 as libc::c_int as usize] as s16;
            (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 10 as libc::c_int +
                                   2 as libc::c_int) as usize] =
                (*lightCtx).fogColor[2 as libc::c_int as usize] as s16;
            (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 13 as libc::c_int) as
                                  usize] = (*lightCtx).fogFar;
            (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 14 as libc::c_int) as
                                  usize] = (*lightCtx).fogNear;
            (*gGameInfo).data[(15 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 3 as libc::c_int +
                                   0 as libc::c_int) as usize] =
                (*envCtx).dirLight1.params.dir.x as s16;
            (*gGameInfo).data[(15 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 3 as libc::c_int +
                                   1 as libc::c_int) as usize] =
                (*envCtx).dirLight1.params.dir.y as s16;
            (*gGameInfo).data[(15 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 3 as libc::c_int +
                                   2 as libc::c_int) as usize] =
                (*envCtx).dirLight1.params.dir.z as s16;
            (*gGameInfo).data[(15 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 6 as libc::c_int +
                                   0 as libc::c_int) as usize] =
                (*envCtx).dirLight2.params.dir.x as s16;
            (*gGameInfo).data[(15 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 6 as libc::c_int +
                                   1 as libc::c_int) as usize] =
                (*envCtx).dirLight2.params.dir.y as s16;
            (*gGameInfo).data[(15 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 6 as libc::c_int +
                                   2 as libc::c_int) as usize] =
                (*envCtx).dirLight2.params.dir.z as s16;
            (*gGameInfo).data[(11 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 16 as libc::c_int +
                                   0 as libc::c_int) as usize] =
                (*envCtx).windDirection.x;
            (*gGameInfo).data[(11 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 16 as libc::c_int +
                                   1 as libc::c_int) as usize] =
                (*envCtx).windDirection.y;
            (*gGameInfo).data[(11 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 16 as libc::c_int +
                                   2 as libc::c_int) as usize] =
                (*envCtx).windDirection.z;
            (*gGameInfo).data[(11 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 19 as libc::c_int) as
                                  usize] = (*envCtx).windSpeed as s16
        } else {
            (*lightCtx).ambientColor[0 as libc::c_int as usize] =
                (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int *
                                       16 as libc::c_int + 0 as libc::c_int +
                                       0 as libc::c_int) as usize] as u8_0;
            (*lightCtx).ambientColor[1 as libc::c_int as usize] =
                (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int *
                                       16 as libc::c_int + 0 as libc::c_int +
                                       1 as libc::c_int) as usize] as u8_0;
            (*lightCtx).ambientColor[2 as libc::c_int as usize] =
                (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int *
                                       16 as libc::c_int + 0 as libc::c_int +
                                       2 as libc::c_int) as usize] as u8_0;
            (*envCtx).dirLight1.params.dir.color[0 as libc::c_int as usize] =
                (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int *
                                       16 as libc::c_int + 3 as libc::c_int +
                                       0 as libc::c_int) as usize] as u8_0;
            (*envCtx).dirLight1.params.dir.color[1 as libc::c_int as usize] =
                (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int *
                                       16 as libc::c_int + 3 as libc::c_int +
                                       1 as libc::c_int) as usize] as u8_0;
            (*envCtx).dirLight1.params.dir.color[2 as libc::c_int as usize] =
                (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int *
                                       16 as libc::c_int + 3 as libc::c_int +
                                       2 as libc::c_int) as usize] as u8_0;
            (*envCtx).dirLight2.params.dir.color[0 as libc::c_int as usize] =
                (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int *
                                       16 as libc::c_int + 6 as libc::c_int +
                                       0 as libc::c_int) as usize] as u8_0;
            (*envCtx).dirLight2.params.dir.color[1 as libc::c_int as usize] =
                (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int *
                                       16 as libc::c_int + 6 as libc::c_int +
                                       1 as libc::c_int) as usize] as u8_0;
            (*envCtx).dirLight2.params.dir.color[2 as libc::c_int as usize] =
                (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int *
                                       16 as libc::c_int + 6 as libc::c_int +
                                       2 as libc::c_int) as usize] as u8_0;
            (*lightCtx).fogColor[0 as libc::c_int as usize] =
                (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int *
                                       16 as libc::c_int + 10 as libc::c_int +
                                       0 as libc::c_int) as usize] as u8_0;
            (*lightCtx).fogColor[1 as libc::c_int as usize] =
                (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int *
                                       16 as libc::c_int + 10 as libc::c_int +
                                       1 as libc::c_int) as usize] as u8_0;
            (*lightCtx).fogColor[2 as libc::c_int as usize] =
                (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int *
                                       16 as libc::c_int + 10 as libc::c_int +
                                       2 as libc::c_int) as usize] as u8_0;
            (*lightCtx).fogNear =
                (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int *
                                       16 as libc::c_int + 14 as libc::c_int)
                                      as usize];
            (*lightCtx).fogFar =
                (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int *
                                       16 as libc::c_int + 13 as libc::c_int)
                                      as usize];
            if (*gGameInfo).data[(15 as libc::c_int * 6 as libc::c_int *
                                      16 as libc::c_int + 14 as libc::c_int)
                                     as usize] != 0 {
                (*gGameInfo).data[(15 as libc::c_int * 6 as libc::c_int *
                                       16 as libc::c_int + 3 as libc::c_int +
                                       0 as libc::c_int) as usize] =
                    (Math_CosS((*gGameInfo).data[(15 as libc::c_int *
                                                      6 as libc::c_int *
                                                      16 as libc::c_int +
                                                      10 as libc::c_int) as
                                                     usize]) *
                         Math_CosS((*gGameInfo).data[(15 as libc::c_int *
                                                          6 as libc::c_int *
                                                          16 as libc::c_int +
                                                          11 as libc::c_int)
                                                         as usize]) *
                         120.0f32) as s16;
                (*envCtx).dirLight1.params.dir.x =
                    (*gGameInfo).data[(15 as libc::c_int * 6 as libc::c_int *
                                           16 as libc::c_int +
                                           3 as libc::c_int +
                                           0 as libc::c_int) as usize] as s8;
                (*gGameInfo).data[(15 as libc::c_int * 6 as libc::c_int *
                                       16 as libc::c_int + 3 as libc::c_int +
                                       1 as libc::c_int) as usize] =
                    (Math_SinS((*gGameInfo).data[(15 as libc::c_int *
                                                      6 as libc::c_int *
                                                      16 as libc::c_int +
                                                      10 as libc::c_int) as
                                                     usize]) *
                         Math_CosS((*gGameInfo).data[(15 as libc::c_int *
                                                          6 as libc::c_int *
                                                          16 as libc::c_int +
                                                          11 as libc::c_int)
                                                         as usize]) *
                         120.0f32) as s16;
                (*envCtx).dirLight1.params.dir.y =
                    (*gGameInfo).data[(15 as libc::c_int * 6 as libc::c_int *
                                           16 as libc::c_int +
                                           3 as libc::c_int +
                                           1 as libc::c_int) as usize] as s8;
                (*gGameInfo).data[(15 as libc::c_int * 6 as libc::c_int *
                                       16 as libc::c_int + 3 as libc::c_int +
                                       2 as libc::c_int) as usize] =
                    (Math_SinS((*gGameInfo).data[(15 as libc::c_int *
                                                      6 as libc::c_int *
                                                      16 as libc::c_int +
                                                      11 as libc::c_int) as
                                                     usize]) * 120.0f32) as
                        s16;
                (*envCtx).dirLight1.params.dir.z =
                    (*gGameInfo).data[(15 as libc::c_int * 6 as libc::c_int *
                                           16 as libc::c_int +
                                           3 as libc::c_int +
                                           2 as libc::c_int) as usize] as s8;
                (*gGameInfo).data[(15 as libc::c_int * 6 as libc::c_int *
                                       16 as libc::c_int + 6 as libc::c_int +
                                       0 as libc::c_int) as usize] =
                    (Math_CosS((*gGameInfo).data[(15 as libc::c_int *
                                                      6 as libc::c_int *
                                                      16 as libc::c_int +
                                                      12 as libc::c_int) as
                                                     usize]) *
                         Math_CosS((*gGameInfo).data[(15 as libc::c_int *
                                                          6 as libc::c_int *
                                                          16 as libc::c_int +
                                                          13 as libc::c_int)
                                                         as usize]) *
                         120.0f32) as s16;
                (*envCtx).dirLight2.params.dir.x =
                    (*gGameInfo).data[(15 as libc::c_int * 6 as libc::c_int *
                                           16 as libc::c_int +
                                           6 as libc::c_int +
                                           0 as libc::c_int) as usize] as s8;
                (*gGameInfo).data[(15 as libc::c_int * 6 as libc::c_int *
                                       16 as libc::c_int + 6 as libc::c_int +
                                       1 as libc::c_int) as usize] =
                    (Math_SinS((*gGameInfo).data[(15 as libc::c_int *
                                                      6 as libc::c_int *
                                                      16 as libc::c_int +
                                                      12 as libc::c_int) as
                                                     usize]) *
                         Math_CosS((*gGameInfo).data[(15 as libc::c_int *
                                                          6 as libc::c_int *
                                                          16 as libc::c_int +
                                                          13 as libc::c_int)
                                                         as usize]) *
                         120.0f32) as s16;
                (*envCtx).dirLight2.params.dir.y =
                    (*gGameInfo).data[(15 as libc::c_int * 6 as libc::c_int *
                                           16 as libc::c_int +
                                           6 as libc::c_int +
                                           1 as libc::c_int) as usize] as s8;
                (*gGameInfo).data[(15 as libc::c_int * 6 as libc::c_int *
                                       16 as libc::c_int + 6 as libc::c_int +
                                       2 as libc::c_int) as usize] =
                    (Math_SinS((*gGameInfo).data[(15 as libc::c_int *
                                                      6 as libc::c_int *
                                                      16 as libc::c_int +
                                                      13 as libc::c_int) as
                                                     usize]) * 120.0f32) as
                        s16;
                (*envCtx).dirLight2.params.dir.z =
                    (*gGameInfo).data[(15 as libc::c_int * 6 as libc::c_int *
                                           16 as libc::c_int +
                                           6 as libc::c_int +
                                           2 as libc::c_int) as usize] as s8
            } else {
                (*envCtx).dirLight1.params.dir.x =
                    (*gGameInfo).data[(15 as libc::c_int * 6 as libc::c_int *
                                           16 as libc::c_int +
                                           3 as libc::c_int +
                                           0 as libc::c_int) as usize] as s8;
                (*envCtx).dirLight1.params.dir.y =
                    (*gGameInfo).data[(15 as libc::c_int * 6 as libc::c_int *
                                           16 as libc::c_int +
                                           3 as libc::c_int +
                                           1 as libc::c_int) as usize] as s8;
                (*envCtx).dirLight1.params.dir.z =
                    (*gGameInfo).data[(15 as libc::c_int * 6 as libc::c_int *
                                           16 as libc::c_int +
                                           3 as libc::c_int +
                                           2 as libc::c_int) as usize] as s8;
                (*envCtx).dirLight2.params.dir.x =
                    (*gGameInfo).data[(15 as libc::c_int * 6 as libc::c_int *
                                           16 as libc::c_int +
                                           6 as libc::c_int +
                                           0 as libc::c_int) as usize] as s8;
                (*envCtx).dirLight2.params.dir.y =
                    (*gGameInfo).data[(15 as libc::c_int * 6 as libc::c_int *
                                           16 as libc::c_int +
                                           6 as libc::c_int +
                                           1 as libc::c_int) as usize] as s8;
                (*envCtx).dirLight2.params.dir.z =
                    (*gGameInfo).data[(15 as libc::c_int * 6 as libc::c_int *
                                           16 as libc::c_int +
                                           6 as libc::c_int +
                                           2 as libc::c_int) as usize] as s8
            }
            (*envCtx).windDirection.x =
                (*gGameInfo).data[(11 as libc::c_int * 6 as libc::c_int *
                                       16 as libc::c_int + 16 as libc::c_int +
                                       0 as libc::c_int) as usize];
            (*envCtx).windDirection.y =
                (*gGameInfo).data[(11 as libc::c_int * 6 as libc::c_int *
                                       16 as libc::c_int + 16 as libc::c_int +
                                       1 as libc::c_int) as usize];
            (*envCtx).windDirection.z =
                (*gGameInfo).data[(11 as libc::c_int * 6 as libc::c_int *
                                       16 as libc::c_int + 16 as libc::c_int +
                                       2 as libc::c_int) as usize];
            (*envCtx).windSpeed =
                (*gGameInfo).data[(11 as libc::c_int * 6 as libc::c_int *
                                       16 as libc::c_int + 19 as libc::c_int)
                                      as usize] as f32_0
        }
        if (*envCtx).dirLight1.params.dir.x as libc::c_int == 0 as libc::c_int
               &&
               (*envCtx).dirLight1.params.dir.y as libc::c_int ==
                   0 as libc::c_int &&
               (*envCtx).dirLight1.params.dir.z as libc::c_int ==
                   0 as libc::c_int {
            (*envCtx).dirLight1.params.dir.x = 1 as libc::c_int as s8
        }
        if (*envCtx).dirLight2.params.dir.x as libc::c_int == 0 as libc::c_int
               &&
               (*envCtx).dirLight2.params.dir.y as libc::c_int ==
                   0 as libc::c_int &&
               (*envCtx).dirLight2.params.dir.z as libc::c_int ==
                   0 as libc::c_int {
            (*envCtx).dirLight2.params.dir.x = 1 as libc::c_int as s8
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Environment_DrawSunAndMoon(mut globalCtx:
                                                        *mut GlobalContext) {
    let mut alpha: f32_0 = 0.;
    let mut color: f32_0 = 0.;
    let mut y: f32_0 = 0.;
    let mut scale: f32_0 = 0.;
    let mut temp: f32_0 = 0.;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_kankyo.c\x00" as *const u8 as *const libc::c_char,
                    2266 as libc::c_int);
    if (*globalCtx).csCtx.state as libc::c_int != 0 as libc::c_int {
        Math_SmoothStepToF(&mut (*globalCtx).envCtx.sunPos.x,
                           -(Math_SinS((gSaveContext.dayTime as libc::c_int -
                                            0x8000 as libc::c_int) as s16) *
                                 120.0f32) * 25.0f32, 1.0f32, 0.8f32, 0.8f32);
        Math_SmoothStepToF(&mut (*globalCtx).envCtx.sunPos.y,
                           Math_CosS((gSaveContext.dayTime as libc::c_int -
                                          0x8000 as libc::c_int) as s16) *
                               120.0f32 * 25.0f32, 1.0f32, 0.8f32, 0.8f32);
        // ! @bug This should be z.
        Math_SmoothStepToF(&mut (*globalCtx).envCtx.sunPos.y,
                           Math_CosS((gSaveContext.dayTime as libc::c_int -
                                          0x8000 as libc::c_int) as s16) *
                               20.0f32 * 25.0f32, 1.0f32, 0.8f32, 0.8f32);
    } else {
        (*globalCtx).envCtx.sunPos.x =
            -(Math_SinS((gSaveContext.dayTime as libc::c_int -
                             0x8000 as libc::c_int) as s16) * 120.0f32) *
                25.0f32;
        (*globalCtx).envCtx.sunPos.y =
            Math_CosS((gSaveContext.dayTime as libc::c_int -
                           0x8000 as libc::c_int) as s16) * 120.0f32 *
                25.0f32;
        (*globalCtx).envCtx.sunPos.z =
            Math_CosS((gSaveContext.dayTime as libc::c_int -
                           0x8000 as libc::c_int) as s16) * 20.0f32 * 25.0f32
    }
    if gSaveContext.entranceIndex != 0xcd as libc::c_int ||
           gSaveContext.sceneSetupIndex != 5 as libc::c_int {
        Matrix_Translate((*globalCtx).view.eye.x +
                             (*globalCtx).envCtx.sunPos.x,
                         (*globalCtx).view.eye.y +
                             (*globalCtx).envCtx.sunPos.y,
                         (*globalCtx).view.eye.z +
                             (*globalCtx).envCtx.sunPos.z,
                         MTXMODE_NEW as libc::c_int as u8_0);
        y = (*globalCtx).envCtx.sunPos.y / 25.0f32;
        temp = y / 80.0f32;
        alpha = temp * 255.0f32;
        if alpha < 0.0f32 { alpha = 0.0f32 }
        if alpha > 255.0f32 { alpha = 255.0f32 }
        alpha = 255.0f32 - alpha;
        color = temp;
        if color < 0.0f32 { color = 0.0f32 }
        if color > 1.0f32 { color = 1.0f32 }
        let fresh3 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g: *mut Gfx = fresh3;
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
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (((color * 75.0f32) as u8_0 as libc::c_int +
                      180 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                (((color * 155.0f32) as u8_0 as libc::c_int +
                      100 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (255 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh4 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_0: *mut Gfx = fresh4;
        (*_g_0).words.w0 =
            (0xfb as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_g_0).words.w1 =
            (255 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((color * 255.0f32) as u8_0 as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                ((color * 255.0f32) as u8_0 as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                (alpha as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        scale = color * 2.0f32 + 10.0f32;
        Matrix_Scale(scale, scale, scale,
                     MTXMODE_APPLY as libc::c_int as u8_0);
        let fresh5 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
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
                ((0x2 as libc::c_int ^ 0x1 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_1).words.w1 =
            Matrix_NewMtx((*globalCtx).state.gfxCtx,
                          b"../z_kankyo.c\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                          2364 as libc::c_int) as libc::c_uint;
        func_80093AD0((*globalCtx).state.gfxCtx);
        let fresh6 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_2: *mut Gfx = fresh6;
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
        (*_g_2).words.w1 = gSunDL.as_mut_ptr() as libc::c_uint;
        Matrix_Translate((*globalCtx).view.eye.x -
                             (*globalCtx).envCtx.sunPos.x,
                         (*globalCtx).view.eye.y -
                             (*globalCtx).envCtx.sunPos.y,
                         (*globalCtx).view.eye.z -
                             (*globalCtx).envCtx.sunPos.z,
                         MTXMODE_NEW as libc::c_int as u8_0);
        color = -y / 120.0f32;
        color = if color < 0.0f32 { 0.0f32 } else { color };
        scale = -15.0f32 * color + 25.0f32;
        Matrix_Scale(scale, scale, scale,
                     MTXMODE_APPLY as libc::c_int as u8_0);
        temp = -y / 80.0f32;
        temp = if temp > 1.0f32 { 1.0f32 } else { temp };
        alpha = temp * 255.0f32;
        if alpha > 0.0f32 {
            let fresh7 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_3: *mut Gfx = fresh7;
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
                    ((0x2 as libc::c_int ^ 0x1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_3).words.w1 =
                Matrix_NewMtx((*globalCtx).state.gfxCtx,
                              b"../z_kankyo.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              2406 as libc::c_int) as libc::c_uint;
            func_8009398C((*globalCtx).state.gfxCtx);
            let fresh8 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_4: *mut Gfx = fresh8;
            (*_g_4).words.w0 =
                (0xe7 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_4).words.w1 = 0 as libc::c_int as libc::c_uint;
            let fresh9 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_5: *mut Gfx = fresh9;
            (*_g_5).words.w0 =
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
            (*_g_5).words.w1 =
                (240 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (255 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (180 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    (alpha as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh10 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_6: *mut Gfx = fresh10;
            (*_g_6).words.w0 =
                (0xfb as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_6).words.w1 =
                (80 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (70 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (20 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    (alpha as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh11 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_7: *mut Gfx = fresh11;
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
            (*_g_7).words.w1 = gMoonDL.as_mut_ptr() as libc::c_uint
        }
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_kankyo.c\x00" as *const u8 as *const libc::c_char,
                     2429 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Environment_DrawSunLensFlare(mut globalCtx:
                                                          *mut GlobalContext,
                                                      mut envCtx:
                                                          *mut EnvironmentContext,
                                                      mut view: *mut View,
                                                      mut gfxCtx:
                                                          *mut GraphicsContext,
                                                      mut pos: Vec3f,
                                                      mut unused: s32) {
    if (*globalCtx).envCtx.unk_EE[1 as libc::c_int as usize] as libc::c_int ==
           0 as libc::c_int &&
           (*globalCtx).envCtx.unk_17 as libc::c_int == 0 as libc::c_int {
        Environment_DrawLensFlare(globalCtx, &mut (*globalCtx).envCtx,
                                  &mut (*globalCtx).view,
                                  (*globalCtx).state.gfxCtx, pos,
                                  2000 as libc::c_int,
                                  370 as libc::c_int as s16,
                                  Math_CosS((gSaveContext.dayTime as
                                                 libc::c_int -
                                                 0x8000 as libc::c_int) as
                                                s16) * 120.0f32,
                                  400 as libc::c_int as s16,
                                  1 as libc::c_int as u8_0);
    };
}
#[no_mangle]
pub static mut sLensFlareScales: [f32_0; 10] =
    [23.0f32, 12.0f32, 7.0f32, 5.0f32, 3.0f32, 10.0f32, 6.0f32, 2.0f32,
     3.0f32, 1.0f32];
#[no_mangle]
pub unsafe extern "C" fn Environment_DrawLensFlare(mut globalCtx:
                                                       *mut GlobalContext,
                                                   mut envCtx:
                                                       *mut EnvironmentContext,
                                                   mut view: *mut View,
                                                   mut gfxCtx:
                                                       *mut GraphicsContext,
                                                   mut pos: Vec3f,
                                                   mut unused: s32,
                                                   mut scale: s16,
                                                   mut colorIntensity: f32_0,
                                                   mut screenFillAlpha: s16,
                                                   mut arg9: u8_0) {
    let mut i: s16 = 0;
    let mut tempX: f32_0 = 0.;
    let mut tempY: f32_0 = 0.;
    let mut tempZ: f32_0 = 0.;
    let mut lookDirX: f32_0 = 0.;
    let mut lookDirY: f32_0 = 0.;
    let mut lookDirZ: f32_0 = 0.;
    let mut tempX2: f32_0 = 0.;
    let mut tempY2: f32_0 = 0.;
    let mut tempZ2: f32_0 = 0.;
    let mut posDirX: f32_0 = 0.;
    let mut posDirY: f32_0 = 0.;
    let mut posDirZ: f32_0 = 0.;
    let mut length: f32_0 = 0.;
    let mut dist: f32_0 = 0.;
    let mut halfPosX: f32_0 = 0.;
    let mut halfPosY: f32_0 = 0.;
    let mut halfPosZ: f32_0 = 0.;
    let mut cosAngle: f32_0 = 0.;
    let mut pad160: f32_0 = 0.;
    let mut unk88Target: f32_0 = 0.;
    let mut isOffScreen: u32_0 = 0 as libc::c_int as u32_0;
    let mut alpha: f32_0 = 0.;
    let mut adjScale: f32_0 = 0.;
    let mut screenPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut fogInfluence: f32_0 = 0.;
    let mut temp: f32_0 = 0.;
    let mut alphaScale: f32_0 = 0.;
    let mut lensFlareColors: [Color_RGB8; 10] =
        [{
             let mut init =
                 Color_RGB8{r: 155 as libc::c_int as u8_0,
                            g: 205 as libc::c_int as u8_0,
                            b: 255 as libc::c_int as u8_0,};
             init
         },
         {
             let mut init =
                 Color_RGB8{r: 255 as libc::c_int as u8_0,
                            g: 255 as libc::c_int as u8_0,
                            b: 205 as libc::c_int as u8_0,};
             init
         },
         {
             let mut init =
                 Color_RGB8{r: 255 as libc::c_int as u8_0,
                            g: 255 as libc::c_int as u8_0,
                            b: 205 as libc::c_int as u8_0,};
             init
         },
         {
             let mut init =
                 Color_RGB8{r: 255 as libc::c_int as u8_0,
                            g: 255 as libc::c_int as u8_0,
                            b: 205 as libc::c_int as u8_0,};
             init
         },
         {
             let mut init =
                 Color_RGB8{r: 155 as libc::c_int as u8_0,
                            g: 255 as libc::c_int as u8_0,
                            b: 205 as libc::c_int as u8_0,};
             init
         },
         {
             let mut init =
                 Color_RGB8{r: 205 as libc::c_int as u8_0,
                            g: 255 as libc::c_int as u8_0,
                            b: 255 as libc::c_int as u8_0,};
             init
         },
         {
             let mut init =
                 Color_RGB8{r: 155 as libc::c_int as u8_0,
                            g: 155 as libc::c_int as u8_0,
                            b: 255 as libc::c_int as u8_0,};
             init
         },
         {
             let mut init =
                 Color_RGB8{r: 205 as libc::c_int as u8_0,
                            g: 175 as libc::c_int as u8_0,
                            b: 255 as libc::c_int as u8_0,};
             init
         },
         {
             let mut init =
                 Color_RGB8{r: 175 as libc::c_int as u8_0,
                            g: 255 as libc::c_int as u8_0,
                            b: 205 as libc::c_int as u8_0,};
             init
         },
         {
             let mut init =
                 Color_RGB8{r: 255 as libc::c_int as u8_0,
                            g: 155 as libc::c_int as u8_0,
                            b: 235 as libc::c_int as u8_0,};
             init
         }];
    let mut lensFlareAlphas: [u32_0; 10] =
        [50 as libc::c_int as u32_0, 10 as libc::c_int as u32_0,
         25 as libc::c_int as u32_0, 40 as libc::c_int as u32_0,
         70 as libc::c_int as u32_0, 30 as libc::c_int as u32_0,
         50 as libc::c_int as u32_0, 70 as libc::c_int as u32_0,
         50 as libc::c_int as u32_0, 40 as libc::c_int as u32_0];
    let mut lensFlareTypes: [u32_0; 10] =
        [LENS_FLARE_RING as libc::c_int as u32_0,
         LENS_FLARE_CIRCLE1 as libc::c_int as u32_0,
         LENS_FLARE_CIRCLE1 as libc::c_int as u32_0,
         LENS_FLARE_CIRCLE1 as libc::c_int as u32_0,
         LENS_FLARE_CIRCLE1 as libc::c_int as u32_0,
         LENS_FLARE_CIRCLE1 as libc::c_int as u32_0,
         LENS_FLARE_CIRCLE1 as libc::c_int as u32_0,
         LENS_FLARE_CIRCLE1 as libc::c_int as u32_0,
         LENS_FLARE_CIRCLE1 as libc::c_int as u32_0,
         LENS_FLARE_CIRCLE1 as libc::c_int as u32_0];
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), gfxCtx,
                    b"../z_kankyo.c\x00" as *const u8 as *const libc::c_char,
                    2516 as libc::c_int);
    dist = Math3D_Vec3f_DistXYZ(&mut pos, &mut (*view).eye) / 12.0f32;
    // compute a unit vector in the look direction
    tempX = (*view).lookAt.x - (*view).eye.x;
    tempY = (*view).lookAt.y - (*view).eye.y;
    tempZ = (*view).lookAt.z - (*view).eye.z;
    length = sqrtf(tempX * tempX + tempY * tempY + tempZ * tempZ);
    lookDirX = tempX / length;
    lookDirY = tempY / length;
    lookDirZ = tempZ / length;
    // compute a position along the look vector half as far as pos
    halfPosX = (*view).eye.x + lookDirX * (dist * 6.0f32);
    halfPosY = (*view).eye.y + lookDirY * (dist * 6.0f32);
    halfPosZ = (*view).eye.z + lookDirZ * (dist * 6.0f32);
    // compute a unit vector in the direction from halfPos to pos
    tempX2 = pos.x - halfPosX;
    tempY2 = pos.y - halfPosY;
    tempZ2 = pos.z - halfPosZ;
    length = sqrtf(tempX2 * tempX2 + tempY2 * tempY2 + tempZ2 * tempZ2);
    posDirX = tempX2 / length;
    posDirY = tempY2 / length;
    posDirZ = tempZ2 / length;
    // compute the cosine of the angle between lookDir and posDir
    cosAngle =
        (lookDirX * posDirX + lookDirY * posDirY + lookDirZ * posDirZ) /
            sqrtf((lookDirX * lookDirX + lookDirY * lookDirY +
                       lookDirZ * lookDirZ) *
                      (posDirX * posDirX + posDirY * posDirY +
                           posDirZ * posDirZ));
    unk88Target = cosAngle * 3.5f32;
    unk88Target = if unk88Target > 1.0f32 { 1.0f32 } else { unk88Target };
    if arg9 as libc::c_int == 0 as libc::c_int { unk88Target = cosAngle }
    if !(cosAngle < 0.0f32) {
        if arg9 != 0 {
            func_800C016C(globalCtx, &mut pos, &mut screenPos);
            D_8015FD7E = screenPos.x as s16;
            D_8015FD80 =
                (screenPos.y as s16 as libc::c_int as libc::c_float - 5.0f32)
                    as s16;
            if D_8011FB44 as libc::c_int != 0xfffc as libc::c_int ||
                   screenPos.x < 0.0f32 || screenPos.y < 0.0f32 ||
                   screenPos.x > 320 as libc::c_int as libc::c_float ||
                   screenPos.y > 240 as libc::c_int as libc::c_float {
                isOffScreen = 1 as libc::c_int as u32_0
            }
        }
        i = 0 as libc::c_int as s16;
        while (i as libc::c_int) <
                  (::std::mem::size_of::<[u32_0; 10]>() as
                       libc::c_ulong).wrapping_div(::std::mem::size_of::<u32_0>()
                                                       as libc::c_ulong) as
                      s32 {
            Matrix_Translate(pos.x, pos.y, pos.z,
                             MTXMODE_NEW as libc::c_int as u8_0);
            if arg9 != 0 {
                temp =
                    Environment_LerpWeight(60 as libc::c_int as u16_0,
                                           15 as libc::c_int as u16_0,
                                           (*globalCtx).view.fovy as u16_0)
            }
            Matrix_Translate(-posDirX * i as libc::c_int as libc::c_float *
                                 dist,
                             -posDirY * i as libc::c_int as libc::c_float *
                                 dist,
                             -posDirZ * i as libc::c_int as libc::c_float *
                                 dist, MTXMODE_APPLY as libc::c_int as u8_0);
            adjScale = sLensFlareScales[i as usize] * cosAngle;
            if arg9 != 0 {
                adjScale =
                    (adjScale as libc::c_double *
                         (0.001f64 *
                              (scale as libc::c_int as libc::c_float +
                                   630.0f32 * temp) as libc::c_double)) as
                        f32_0
            } else {
                adjScale *=
                    0.0001f32 * scale as libc::c_int as libc::c_float *
                        (2.0f32 * dist)
            }
            Matrix_Scale(adjScale, adjScale, adjScale,
                         MTXMODE_APPLY as libc::c_int as u8_0);
            alpha = colorIntensity / 10.0f32;
            alpha = if alpha > 1.0f32 { 1.0f32 } else { alpha };
            alpha = alpha * lensFlareAlphas[i as usize] as libc::c_float;
            alpha = if alpha < 0.0f32 { 0.0f32 } else { alpha };
            fogInfluence =
                (996 as libc::c_int -
                     (*globalCtx).lightCtx.fogNear as libc::c_int) as
                    libc::c_float / 50.0f32;
            fogInfluence =
                if fogInfluence > 1.0f32 { 1.0f32 } else { fogInfluence };
            alpha *= 1.0f32 - fogInfluence;
            if isOffScreen ^ 0 as libc::c_int as libc::c_uint == 0 {
                Math_SmoothStepToF(&mut (*envCtx).unk_88, unk88Target, 0.5f32,
                                   0.05f32, 0.001f32);
            } else {
                Math_SmoothStepToF(&mut (*envCtx).unk_88, 0.0f32, 0.5f32,
                                   0.05f32, 0.001f32);
            }
            let fresh12 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            (*__gfxCtx).polyXlu.p = func_800947AC(fresh12);
            let fresh13 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g: *mut Gfx = fresh13;
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
                (lensFlareColors[i as usize].r as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (lensFlareColors[i as usize].g as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (lensFlareColors[i as usize].b as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    ((alpha * (*envCtx).unk_88) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh14 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_0: *mut Gfx = fresh14;
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
                           0 as libc::c_int) ^ 0x1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_0).words.w1 =
                Matrix_NewMtx(gfxCtx,
                              b"../z_kankyo.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              2662 as libc::c_int) as libc::c_uint;
            let fresh15 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_1: *mut Gfx = fresh15;
            (*_g_1).words.w0 =
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
            (*_g_1).words.w1 =
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
                         (1 as libc::c_int as u32_0 &
                              (((0x1 as libc::c_int) << 3 as libc::c_int) -
                                   1 as libc::c_int) as libc::c_uint) <<
                             21 as libc::c_int |
                         (3 as libc::c_int as u32_0 &
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
                         (7 as libc::c_int as u32_0 &
                              (((0x1 as libc::c_int) << 3 as libc::c_int) -
                                   1 as libc::c_int) as libc::c_uint) <<
                             0 as libc::c_int);
            let fresh16 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_2: *mut Gfx = fresh16;
            (*_g_2).words.w0 =
                (0xe3 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((32 as libc::c_int - 4 as libc::c_int - 2 as libc::c_int)
                         as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    ((2 as libc::c_int - 1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_2).words.w1 =
                ((3 as libc::c_int) << 4 as libc::c_int) as libc::c_uint;
            let fresh17 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_3: *mut Gfx = fresh17;
            (*_g_3).words.w0 =
                (0xe3 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((32 as libc::c_int - 6 as libc::c_int - 2 as libc::c_int)
                         as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    ((2 as libc::c_int - 1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_3).words.w1 =
                ((3 as libc::c_int) << 6 as libc::c_int) as libc::c_uint;
            let fresh18 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_4: *mut Gfx = fresh18;
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
                    (((0 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int)
                          ^ 0x1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_4).words.w1 = &mut D_01000000 as *mut Mtx as libc::c_uint;
            match lensFlareTypes[i as usize] {
                0 | 1 => {
                    let fresh19 = (*__gfxCtx).polyXlu.p;
                    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                    let mut _g_5: *mut Gfx = fresh19;
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
                                 (((0x1 as libc::c_int) << 16 as libc::c_int)
                                      - 1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int;
                    (*_g_5).words.w1 =
                        gLensFlareCircleDL.as_mut_ptr() as libc::c_uint
                }
                2 => {
                    let fresh20 = (*__gfxCtx).polyXlu.p;
                    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                    let mut _g_6: *mut Gfx = fresh20;
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
                                 (((0x1 as libc::c_int) << 16 as libc::c_int)
                                      - 1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int;
                    (*_g_6).words.w1 =
                        gLensFlareRingDL.as_mut_ptr() as libc::c_uint
                }
                _ => { }
            }
            i += 1
        }
        alphaScale = cosAngle - (1.5f32 - cosAngle);
        if screenFillAlpha as libc::c_int != 0 as libc::c_int {
            if alphaScale > 0.0f32 {
                (*__gfxCtx).polyXlu.p = func_800937C0((*__gfxCtx).polyXlu.p);
                alpha = colorIntensity / 10.0f32;
                alpha = if alpha > 1.0f32 { 1.0f32 } else { alpha };
                alpha =
                    alpha * screenFillAlpha as libc::c_int as libc::c_float;
                alpha = if alpha < 0.0f32 { 0.0f32 } else { alpha };
                fogInfluence =
                    (996 as libc::c_int -
                         (*globalCtx).lightCtx.fogNear as libc::c_int) as
                        libc::c_float / 50.0f32;
                fogInfluence =
                    if fogInfluence > 1.0f32 { 1.0f32 } else { fogInfluence };
                alpha *= 1.0f32 - fogInfluence;
                let fresh21 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_7: *mut Gfx = fresh21;
                (*_g_7).words.w0 =
                    (0xe3 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        ((32 as libc::c_int - 4 as libc::c_int -
                              2 as libc::c_int) as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            8 as libc::c_int |
                        ((2 as libc::c_int - 1 as libc::c_int) as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                (*_g_7).words.w1 =
                    ((3 as libc::c_int) << 4 as libc::c_int) as libc::c_uint;
                let fresh22 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_8: *mut Gfx = fresh22;
                (*_g_8).words.w0 =
                    (0xe3 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        ((32 as libc::c_int - 6 as libc::c_int -
                              2 as libc::c_int) as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            8 as libc::c_int |
                        ((2 as libc::c_int - 1 as libc::c_int) as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                (*_g_8).words.w1 =
                    ((3 as libc::c_int) << 6 as libc::c_int) as libc::c_uint;
                if isOffScreen ^ 0 as libc::c_int as libc::c_uint == 0 {
                    Math_SmoothStepToF(&mut (*envCtx).unk_84,
                                       alpha * alphaScale, 0.5f32, 50.0f32,
                                       0.1f32);
                } else {
                    Math_SmoothStepToF(&mut (*envCtx).unk_84, 0.0f32, 0.5f32,
                                       50.0f32, 0.1f32);
                }
                temp = colorIntensity / 120.0f32;
                temp = if temp < 0.0f32 { 0.0f32 } else { temp };
                let fresh23 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_9: *mut Gfx = fresh23;
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
                    (255 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (((temp * 75.0f32) as u8_0 as libc::c_int +
                              180 as libc::c_int) as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            16 as libc::c_int |
                        (((temp * 155.0f32) as u8_0 as libc::c_int +
                              100 as libc::c_int) as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            8 as libc::c_int |
                        ((*envCtx).unk_84 as u8_0 as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                let fresh24 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_10: *mut Gfx = fresh24;
                (*_g_10).words.w0 =
                    (0xf6 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        ((320 as libc::c_int - 1 as libc::c_int) as u32_0 &
                             (((0x1 as libc::c_int) << 10 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            14 as libc::c_int |
                        ((240 as libc::c_int - 1 as libc::c_int) as u32_0 &
                             (((0x1 as libc::c_int) << 10 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            2 as libc::c_int;
                (*_g_10).words.w1 =
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 10 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        14 as libc::c_int |
                        (0 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 10 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            2 as libc::c_int
            } else { (*envCtx).unk_84 = 0.0f32 }
        }
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), gfxCtx,
                     b"../z_kankyo.c\x00" as *const u8 as *const libc::c_char,
                     2750 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn func_800746DC() -> f32_0 {
    return Rand_ZeroOne() - 0.5f32;
}
#[no_mangle]
pub unsafe extern "C" fn Environment_DrawRain(mut globalCtx:
                                                  *mut GlobalContext,
                                              mut view: *mut View,
                                              mut gfxCtx:
                                                  *mut GraphicsContext) {
    let mut i: s16 = 0;
    let mut pad: s32 = 0;
    let mut vec: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut temp1: f32_0 = 0.;
    let mut temp2: f32_0 = 0.;
    let mut temp3: f32_0 = 0.;
    let mut length: f32_0 = 0.;
    let mut rotX: f32_0 = 0.;
    let mut rotY: f32_0 = 0.;
    let mut x50: f32_0 = 0.;
    let mut y50: f32_0 = 0.;
    let mut z50: f32_0 = 0.;
    let mut x280: f32_0 = 0.;
    let mut z280: f32_0 = 0.;
    let mut unused: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
    let mut windDirection: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    if (*(*globalCtx).cameraPtrs[0 as libc::c_int as usize]).unk_14C as
           libc::c_int & 0x100 as libc::c_int == 0 &&
           (*globalCtx).envCtx.unk_EE[2 as libc::c_int as usize] as
               libc::c_int == 0 as libc::c_int {
        let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
        let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
        __gfxCtx = gfxCtx;
        Graph_OpenDisps(dispRefs.as_mut_ptr(), gfxCtx,
                        b"../z_kankyo.c\x00" as *const u8 as
                            *const libc::c_char, 2799 as libc::c_int);
        vec.x = (*view).lookAt.x - (*view).eye.x;
        vec.y = (*view).lookAt.y - (*view).eye.y;
        vec.z = (*view).lookAt.z - (*view).eye.z;
        length = sqrtf(vec.x * vec.x + vec.y * vec.y + vec.z * vec.z);
        temp1 = vec.x / length;
        temp2 = vec.y / length;
        temp3 = vec.z / length;
        x50 = (*view).eye.x + temp1 * 50.0f32;
        y50 = (*view).eye.y + temp2 * 50.0f32;
        z50 = (*view).eye.z + temp3 * 50.0f32;
        x280 = (*view).eye.x + temp1 * 280.0f32;
        z280 = (*view).eye.z + temp3 * 280.0f32;
        if (*globalCtx).envCtx.unk_EE[1 as libc::c_int as usize] != 0 {
            let fresh25 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g: *mut Gfx = fresh25;
            (*_g).words.w0 =
                (0xe7 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g).words.w1 = 0 as libc::c_int as libc::c_uint;
            let fresh26 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_0: *mut Gfx = fresh26;
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
            (*__gfxCtx).polyXlu.p =
                Gfx_CallSetupDL((*__gfxCtx).polyXlu.p,
                                20 as libc::c_int as u32_0)
        }
        // draw rain drops
        i = 0 as libc::c_int as s16;
        while (i as libc::c_int) <
                  (*globalCtx).envCtx.unk_EE[1 as libc::c_int as usize] as
                      libc::c_int {
            temp2 = Rand_ZeroOne();
            temp1 = Rand_ZeroOne();
            temp3 = Rand_ZeroOne();
            Matrix_Translate((temp2 - 0.7f32) * 100.0f32 + x50,
                             (temp1 - 0.7f32) * 100.0f32 + y50,
                             (temp3 - 0.7f32) * 100.0f32 + z50,
                             MTXMODE_NEW as libc::c_int as u8_0);
            windDirection.x = (*globalCtx).envCtx.windDirection.x as f32_0;
            windDirection.y = (*globalCtx).envCtx.windDirection.y as f32_0;
            windDirection.z = (*globalCtx).envCtx.windDirection.z as f32_0;
            vec.x = windDirection.x;
            vec.y = windDirection.y + 500.0f32 + Rand_ZeroOne() * 200.0f32;
            vec.z = windDirection.z;
            length = sqrtf(vec.x * vec.x + vec.z * vec.z);
            let fresh27 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_1: *mut Gfx = fresh27;
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
                    (((0 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int)
                          ^ 0x1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_1).words.w1 = &mut D_01000000 as *mut Mtx as libc::c_uint;
            rotX = Math_Atan2F(length, -vec.y);
            rotY = Math_Atan2F(vec.z, vec.x);
            Matrix_RotateY(-rotY, MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_RotateX(3.14159265358979323846f32 /
                               2 as libc::c_int as libc::c_float - rotX,
                           MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_Scale(0.4f32, 1.2f32, 0.4f32,
                         MTXMODE_APPLY as libc::c_int as u8_0);
            let fresh28 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_2: *mut Gfx = fresh28;
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
                              b"../z_kankyo.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              2887 as libc::c_int) as libc::c_uint;
            let fresh29 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_3: *mut Gfx = fresh29;
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
            (*_g_3).words.w1 = gRaindropDL.as_mut_ptr() as libc::c_uint;
            i += 1
        }
        // draw droplet rings on the ground
        if (*player).actor.world.pos.y < (*view).eye.y {
            let mut firstDone: u8_0 = 0 as libc::c_int as u8_0;
            i = 0 as libc::c_int as s16;
            while (i as libc::c_int) <
                      (*globalCtx).envCtx.unk_EE[1 as libc::c_int as usize] as
                          libc::c_int {
                if firstDone == 0 {
                    func_80093D84(gfxCtx);
                    let fresh30 = (*__gfxCtx).polyXlu.p;
                    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                    let mut _g_4: *mut Gfx = fresh30;
                    (*_g_4).words.w0 =
                        (0xfb as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            24 as libc::c_int;
                    (*_g_4).words.w1 =
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
                    let fresh31 = (*__gfxCtx).polyXlu.p;
                    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                    let mut _g_5: *mut Gfx = fresh31;
                    (*_g_5).words.w0 =
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
                    (*_g_5).words.w1 =
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
                            (120 as libc::c_int as u32_0 &
                                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                      1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int;
                    firstDone = firstDone.wrapping_add(1)
                }
                Matrix_Translate(func_800746DC() * 280.0f32 + x280,
                                 (*player).actor.world.pos.y + 2.0f32,
                                 func_800746DC() * 280.0f32 + z280,
                                 MTXMODE_NEW as libc::c_int as u8_0);
                if gSaveContext.linkAge == 0 as libc::c_int &&
                       (*player).actor.world.pos.y + 2.0f32 - (*view).eye.y >
                           -48.0f32 ||
                       !(gSaveContext.linkAge == 0 as libc::c_int) &&
                           (*player).actor.world.pos.y + 2.0f32 -
                               (*view).eye.y > -30.0f32 {
                    Matrix_Scale(0.02f32, 0.02f32, 0.02f32,
                                 MTXMODE_APPLY as libc::c_int as u8_0);
                } else {
                    Matrix_Scale(0.1f32, 0.1f32, 0.1f32,
                                 MTXMODE_APPLY as libc::c_int as u8_0);
                }
                let fresh32 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_6: *mut Gfx = fresh32;
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
                    Matrix_NewMtx(gfxCtx,
                                  b"../z_kankyo.c\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char, 2940 as libc::c_int)
                        as libc::c_uint;
                let fresh33 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_7: *mut Gfx = fresh33;
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
                    gEffShockwaveDL.as_mut_ptr() as libc::c_uint;
                i += 1
            }
        }
        Graph_CloseDisps(dispRefs.as_mut_ptr(), gfxCtx,
                         b"../z_kankyo.c\x00" as *const u8 as
                             *const libc::c_char, 2946 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_80074CE8(mut globalCtx: *mut GlobalContext,
                                       mut arg1: u32_0) {
    if (*globalCtx).envCtx.unk_BD as libc::c_uint != arg1 &&
           (*globalCtx).envCtx.unk_D8 >= 1.0f32 &&
           (*globalCtx).envCtx.unk_BF as libc::c_int == 0xff as libc::c_int {
        if arg1 > 30 as libc::c_int as libc::c_uint {
            arg1 = 0 as libc::c_int as u32_0
        }
        (*globalCtx).envCtx.unk_D8 = 0.0f32;
        (*globalCtx).envCtx.unk_BE = (*globalCtx).envCtx.unk_BD;
        (*globalCtx).envCtx.unk_BD = arg1 as u8_0
    };
}
/* *
 * Draw color filters over the skybox. There are two filters.
 * The first uses the global fog color, and an alpha calculated with `fogNear`.
 * This filter draws unconditionally for skybox 29 at full alpha.
 * (note: skybox 29 is unused in the original game)
 * For the rest of the skyboxes it will draw if fogNear is less than 980.
 *
 * The second filter uses a custom color specified in `skyboxFilterColor`
 * and can be enabled with `customSkyboxFilter`.
 *
 * An example usage of a filter is to dim the skybox in cloudy conditions.
 */
#[no_mangle]
pub unsafe extern "C" fn Environment_DrawSkyboxFilters(mut globalCtx:
                                                           *mut GlobalContext) {
    if (*globalCtx).skyboxId as libc::c_int != SKYBOX_NONE as libc::c_int &&
           ((*globalCtx).lightCtx.fogNear as libc::c_int) < 980 as libc::c_int
           ||
           (*globalCtx).skyboxId as libc::c_int ==
               SKYBOX_UNSET_1D as libc::c_int {
        let mut alpha: f32_0 = 0.;
        let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
        let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
        __gfxCtx = (*globalCtx).state.gfxCtx;
        Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                        b"../z_kankyo.c\x00" as *const u8 as
                            *const libc::c_char, 3032 as libc::c_int);
        func_800938B4((*globalCtx).state.gfxCtx);
        alpha =
            (1000 as libc::c_int -
                 (*globalCtx).lightCtx.fogNear as libc::c_int) as
                libc::c_float * 0.02f32;
        if (*globalCtx).skyboxId as libc::c_int ==
               SKYBOX_UNSET_1D as libc::c_int {
            alpha = 1.0f32
        }
        if alpha > 1.0f32 { alpha = 1.0f32 }
        let fresh34 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g: *mut Gfx = fresh34;
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
            ((*globalCtx).lightCtx.fogColor[0 as libc::c_int as usize] as
                 u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((*globalCtx).lightCtx.fogColor[1 as libc::c_int as usize] as
                     u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                ((*globalCtx).lightCtx.fogColor[2 as libc::c_int as usize] as
                     u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                ((255.0f32 * alpha) as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh35 = (*__gfxCtx).polyOpa.p;
        (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
        let mut _g_0: *mut Gfx = fresh35;
        (*_g_0).words.w0 =
            (0xf6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((320 as libc::c_int - 1 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 10 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    14 as libc::c_int |
                ((240 as libc::c_int - 1 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 10 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    2 as libc::c_int;
        (*_g_0).words.w1 =
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 10 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 14 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 10 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    2 as libc::c_int;
        Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                         b"../z_kankyo.c\x00" as *const u8 as
                             *const libc::c_char, 3043 as libc::c_int);
    }
    if (*globalCtx).envCtx.customSkyboxFilter != 0 {
        let mut __gfxCtx_0: *mut GraphicsContext = 0 as *mut GraphicsContext;
        let mut dispRefs_0: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
        __gfxCtx_0 = (*globalCtx).state.gfxCtx;
        Graph_OpenDisps(dispRefs_0.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                        b"../z_kankyo.c\x00" as *const u8 as
                            *const libc::c_char, 3048 as libc::c_int);
        func_800938B4((*globalCtx).state.gfxCtx);
        let fresh36 = (*__gfxCtx_0).polyOpa.p;
        (*__gfxCtx_0).polyOpa.p = (*__gfxCtx_0).polyOpa.p.offset(1);
        let mut _g_1: *mut Gfx = fresh36;
        (*_g_1).words.w0 =
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
        (*_g_1).words.w1 =
            ((*globalCtx).envCtx.skyboxFilterColor[0 as libc::c_int as usize]
                 as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((*globalCtx).envCtx.skyboxFilterColor[1 as libc::c_int as
                                                           usize] as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                ((*globalCtx).envCtx.skyboxFilterColor[2 as libc::c_int as
                                                           usize] as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    8 as libc::c_int |
                ((*globalCtx).envCtx.skyboxFilterColor[3 as libc::c_int as
                                                           usize] as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh37 = (*__gfxCtx_0).polyOpa.p;
        (*__gfxCtx_0).polyOpa.p = (*__gfxCtx_0).polyOpa.p.offset(1);
        let mut _g_2: *mut Gfx = fresh37;
        (*_g_2).words.w0 =
            (0xf6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((320 as libc::c_int - 1 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 10 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    14 as libc::c_int |
                ((240 as libc::c_int - 1 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 10 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    2 as libc::c_int;
        (*_g_2).words.w1 =
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 10 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 14 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 10 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    2 as libc::c_int;
        Graph_CloseDisps(dispRefs_0.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                         b"../z_kankyo.c\x00" as *const u8 as
                             *const libc::c_char, 3056 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Environment_DrawLightningFlash(mut globalCtx:
                                                            *mut GlobalContext,
                                                        mut red: u8_0,
                                                        mut green: u8_0,
                                                        mut blue: u8_0,
                                                        mut alpha: u8_0) {
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_kankyo.c\x00" as *const u8 as *const libc::c_char,
                    3069 as libc::c_int);
    func_800938B4((*globalCtx).state.gfxCtx);
    let fresh38 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g: *mut Gfx = fresh38;
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
        (red as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (green as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (blue as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (alpha as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh39 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_0: *mut Gfx = fresh39;
    (*_g_0).words.w0 =
        (0xf6 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((320 as libc::c_int - 1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 10 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 14 as libc::c_int
            |
            ((240 as libc::c_int - 1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 10 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 2 as libc::c_int;
    (*_g_0).words.w1 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 10 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 14 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 10 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 2 as libc::c_int;
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_kankyo.c\x00" as *const u8 as *const libc::c_char,
                     3079 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Environment_UpdateLightningStrike(mut globalCtx:
                                                               *mut GlobalContext) {
    if (*globalCtx).envCtx.lightningMode as libc::c_int !=
           LIGHTNING_MODE_OFF as libc::c_int {
        match gLightningStrike.state as libc::c_int {
            0 => {
                // every frame theres a 10% chance of the timer advancing 50 units
                if Rand_ZeroOne() < 0.1f32 {
                    gLightningStrike.delayTimer += 50.0f32
                }
                gLightningStrike.delayTimer += Rand_ZeroOne();
                if gLightningStrike.delayTimer > 500.0f32 {
                    gLightningStrike.flashRed = 200 as libc::c_int as u8_0;
                    gLightningStrike.flashGreen = 200 as libc::c_int as u8_0;
                    gLightningStrike.flashBlue = 255 as libc::c_int as u8_0;
                    gLightningStrike.flashAlphaTarget =
                        200 as libc::c_int as u8_0;
                    gLightningStrike.delayTimer = 0.0f32;
                    Environment_AddLightningBolts(globalCtx,
                                                  ((Rand_ZeroOne() *
                                                        ((::std::mem::size_of::<[LightningBolt; 3]>()
                                                              as
                                                              libc::c_ulong).wrapping_div(::std::mem::size_of::<LightningBolt>()
                                                                                              as
                                                                                              libc::c_ulong)
                                                             as s32 as
                                                             libc::c_float -
                                                             0.1f32)) as u8_0
                                                       as libc::c_int +
                                                       1 as libc::c_int) as
                                                      u8_0);
                    sLightningFlashAlpha = 0 as libc::c_int as s16;
                    gLightningStrike.state =
                        gLightningStrike.state.wrapping_add(1)
                }
            }
            1 => {
                gLightningStrike.flashRed = 200 as libc::c_int as u8_0;
                gLightningStrike.flashGreen = 200 as libc::c_int as u8_0;
                gLightningStrike.flashBlue = 255 as libc::c_int as u8_0;
                (*globalCtx).envCtx.adjAmbientColor[0 as libc::c_int as usize]
                    =
                    ((*globalCtx).envCtx.adjAmbientColor[0 as libc::c_int as
                                                             usize] as
                         libc::c_int + 80 as libc::c_int) as s16;
                (*globalCtx).envCtx.adjAmbientColor[1 as libc::c_int as usize]
                    =
                    ((*globalCtx).envCtx.adjAmbientColor[1 as libc::c_int as
                                                             usize] as
                         libc::c_int + 80 as libc::c_int) as s16;
                (*globalCtx).envCtx.adjAmbientColor[2 as libc::c_int as usize]
                    =
                    ((*globalCtx).envCtx.adjAmbientColor[2 as libc::c_int as
                                                             usize] as
                         libc::c_int + 100 as libc::c_int) as s16;
                sLightningFlashAlpha =
                    (sLightningFlashAlpha as libc::c_int + 100 as libc::c_int)
                        as s16;
                if sLightningFlashAlpha as libc::c_int >=
                       gLightningStrike.flashAlphaTarget as libc::c_int {
                    func_800F6D58(15 as libc::c_int as u8_0,
                                  0 as libc::c_int as u8_0,
                                  0 as libc::c_int as u8_0);
                    gLightningStrike.state =
                        gLightningStrike.state.wrapping_add(1);
                    gLightningStrike.flashAlphaTarget =
                        0 as libc::c_int as u8_0
                }
            }
            2 => {
                if (*globalCtx).envCtx.adjAmbientColor[0 as libc::c_int as
                                                           usize] as
                       libc::c_int > 0 as libc::c_int {
                    (*globalCtx).envCtx.adjAmbientColor[0 as libc::c_int as
                                                            usize] =
                        ((*globalCtx).envCtx.adjAmbientColor[0 as libc::c_int
                                                                 as usize] as
                             libc::c_int - 10 as libc::c_int) as s16;
                    (*globalCtx).envCtx.adjAmbientColor[1 as libc::c_int as
                                                            usize] =
                        ((*globalCtx).envCtx.adjAmbientColor[1 as libc::c_int
                                                                 as usize] as
                             libc::c_int - 10 as libc::c_int) as s16
                }
                if (*globalCtx).envCtx.adjAmbientColor[2 as libc::c_int as
                                                           usize] as
                       libc::c_int > 0 as libc::c_int {
                    (*globalCtx).envCtx.adjAmbientColor[2 as libc::c_int as
                                                            usize] =
                        ((*globalCtx).envCtx.adjAmbientColor[2 as libc::c_int
                                                                 as usize] as
                             libc::c_int - 10 as libc::c_int) as s16
                }
                sLightningFlashAlpha =
                    (sLightningFlashAlpha as libc::c_int - 10 as libc::c_int)
                        as s16;
                if sLightningFlashAlpha as libc::c_int <=
                       gLightningStrike.flashAlphaTarget as libc::c_int {
                    (*globalCtx).envCtx.adjAmbientColor[0 as libc::c_int as
                                                            usize] =
                        0 as libc::c_int as s16;
                    (*globalCtx).envCtx.adjAmbientColor[1 as libc::c_int as
                                                            usize] =
                        0 as libc::c_int as s16;
                    (*globalCtx).envCtx.adjAmbientColor[2 as libc::c_int as
                                                            usize] =
                        0 as libc::c_int as s16;
                    gLightningStrike.state =
                        LIGHTNING_STRIKE_WAIT as libc::c_int as u8_0;
                    if (*globalCtx).envCtx.lightningMode as libc::c_int ==
                           LIGHTNING_MODE_LAST as libc::c_int {
                        (*globalCtx).envCtx.lightningMode =
                            LIGHTNING_MODE_OFF as libc::c_int as u8_0
                    }
                }
            }
            _ => { }
        }
    }
    if gLightningStrike.state as libc::c_int !=
           LIGHTNING_STRIKE_WAIT as libc::c_int {
        Environment_DrawLightningFlash(globalCtx, gLightningStrike.flashRed,
                                       gLightningStrike.flashGreen,
                                       gLightningStrike.flashBlue,
                                       sLightningFlashAlpha as u8_0);
    };
}
/* *
 * Request the number of lightning bolts specified by `num`
 * Note: only 3 lightning bolts can be active at the same time.
 */
#[no_mangle]
pub unsafe extern "C" fn Environment_AddLightningBolts(mut globalCtx:
                                                           *mut GlobalContext,
                                                       mut num: u8_0) {
    let mut boltsAdded: s16 = 0 as libc::c_int as s16;
    let mut i: s16 = 0;
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[LightningBolt; 3]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<LightningBolt>()
                                                   as libc::c_ulong) as s32 {
        if sLightningBolts[i as usize].state as libc::c_int ==
               LIGHTNING_BOLT_INACTIVE as libc::c_int {
            sLightningBolts[i as usize].state =
                LIGHTNING_BOLT_START as libc::c_int as u8_0;
            boltsAdded += 1;
            if boltsAdded as libc::c_int >= num as libc::c_int { break ; }
        }
        i += 1
    };
}
/* *
 * Draw any active lightning bolt entries contained in `sLightningBolts`
 */
#[no_mangle]
pub unsafe extern "C" fn Environment_DrawLightning(mut globalCtx:
                                                       *mut GlobalContext,
                                                   mut unused: s32) {
    static mut lightningTextures: [*mut libc::c_void; 9] =
        unsafe {
            [gEffLightning1Tex.as_ptr() as *mut _ as *mut libc::c_void,
             gEffLightning2Tex.as_ptr() as *mut _ as *mut libc::c_void,
             gEffLightning3Tex.as_ptr() as *mut _ as *mut libc::c_void,
             gEffLightning4Tex.as_ptr() as *mut _ as *mut libc::c_void,
             gEffLightning5Tex.as_ptr() as *mut _ as *mut libc::c_void,
             gEffLightning6Tex.as_ptr() as *mut _ as *mut libc::c_void,
             gEffLightning7Tex.as_ptr() as *mut _ as *mut libc::c_void,
             gEffLightning8Tex.as_ptr() as *mut _ as *mut libc::c_void,
             0 as *const libc::c_void as *mut libc::c_void]
        };
    let mut i: s16 = 0;
    let mut dx: f32_0 = 0.;
    let mut dz: f32_0 = 0.;
    let mut x: f32_0 = 0.;
    let mut z: f32_0 = 0.;
    let mut pad: [s32; 2] = [0; 2];
    let mut unused1: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
    let mut unused2: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_kankyo.c\x00" as *const u8 as *const libc::c_char,
                    3253 as libc::c_int);
    i = 0 as libc::c_int as s16;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[LightningBolt; 3]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<LightningBolt>()
                                                   as libc::c_ulong) as s32 {
        match sLightningBolts[i as usize].state as libc::c_int {
            0 => {
                dx = (*globalCtx).view.lookAt.x - (*globalCtx).view.eye.x;
                dz = (*globalCtx).view.lookAt.z - (*globalCtx).view.eye.z;
                x = dx / sqrtf(dx * dx + dz * dz);
                z = dz / sqrtf(dx * dx + dz * dz);
                sLightningBolts[i as usize].pos.x =
                    (*globalCtx).view.eye.x + x * 9500.0f32;
                sLightningBolts[i as usize].pos.y =
                    Rand_ZeroOne() * 1000.0f32 + 4000.0f32;
                sLightningBolts[i as usize].pos.z =
                    (*globalCtx).view.eye.z + z * 9500.0f32;
                sLightningBolts[i as usize].offset.x =
                    (Rand_ZeroOne() - 0.5f32) * 5000.0f32;
                sLightningBolts[i as usize].offset.y = 0.0f32;
                sLightningBolts[i as usize].offset.z =
                    (Rand_ZeroOne() - 0.5f32) * 5000.0f32;
                sLightningBolts[i as usize].textureIndex =
                    0 as libc::c_int as u8_0;
                sLightningBolts[i as usize].pitch =
                    ((Rand_ZeroOne() - 0.5f32) * 40.0f32) as s8;
                sLightningBolts[i as usize].roll =
                    ((Rand_ZeroOne() - 0.5f32) * 40.0f32) as s8;
                sLightningBolts[i as usize].delayTimer =
                    (3 as libc::c_int * (i as libc::c_int + 1 as libc::c_int))
                        as u8_0;
                sLightningBolts[i as usize].state =
                    sLightningBolts[i as usize].state.wrapping_add(1)
            }
            1 => {
                sLightningBolts[i as usize].delayTimer =
                    sLightningBolts[i as usize].delayTimer.wrapping_sub(1);
                if sLightningBolts[i as usize].delayTimer as libc::c_int <=
                       0 as libc::c_int {
                    sLightningBolts[i as usize].state =
                        sLightningBolts[i as usize].state.wrapping_add(1)
                }
            }
            2 => {
                if (sLightningBolts[i as usize].textureIndex as libc::c_int) <
                       7 as libc::c_int {
                    sLightningBolts[i as usize].textureIndex =
                        sLightningBolts[i as
                                            usize].textureIndex.wrapping_add(1)
                } else {
                    sLightningBolts[i as usize].state =
                        LIGHTNING_BOLT_INACTIVE as libc::c_int as u8_0
                }
            }
            _ => { }
        }
        if sLightningBolts[i as usize].state as libc::c_int ==
               LIGHTNING_BOLT_DRAW as libc::c_int {
            Matrix_Translate(sLightningBolts[i as usize].pos.x +
                                 sLightningBolts[i as usize].offset.x,
                             sLightningBolts[i as usize].pos.y +
                                 sLightningBolts[i as usize].offset.y,
                             sLightningBolts[i as usize].pos.z +
                                 sLightningBolts[i as usize].offset.z,
                             MTXMODE_NEW as libc::c_int as u8_0);
            Matrix_RotateX(sLightningBolts[i as usize].pitch as libc::c_int as
                               libc::c_float *
                               (3.14159265358979323846f32 / 180.0f32),
                           MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_RotateZ(sLightningBolts[i as usize].roll as libc::c_int as
                               libc::c_float *
                               (3.14159265358979323846f32 / 180.0f32),
                           MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_Scale(22.0f32, 100.0f32, 22.0f32,
                         MTXMODE_APPLY as libc::c_int as u8_0);
            let fresh40 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g: *mut Gfx = fresh40;
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
                    (128 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh41 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_0: *mut Gfx = fresh41;
            (*_g_0).words.w0 =
                (0xfb as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_0).words.w1 =
                (0 as libc::c_int as u32_0 &
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
                    (128 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
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
                Matrix_NewMtx((*globalCtx).state.gfxCtx,
                              b"../z_kankyo.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              3333 as libc::c_int) as libc::c_uint;
            let fresh43 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_2: *mut Gfx = fresh43;
            (*_g_2).words.w0 =
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
            (*_g_2).words.w1 =
                gSegments[((lightningTextures[sLightningBolts[i as
                                                                  usize].textureIndex
                                                  as usize] as u32_0) <<
                               4 as libc::c_int >> 28 as libc::c_int) as
                              usize].wrapping_add(lightningTextures[sLightningBolts[i
                                                                                        as
                                                                                        usize].textureIndex
                                                                        as
                                                                        usize]
                                                      as u32_0 &
                                                      0xffffff as libc::c_int
                                                          as
                                                          libc::c_uint).wrapping_add(0x80000000
                                                                                         as
                                                                                         libc::c_uint)
                    as *mut libc::c_void as libc::c_uint;
            func_80094C50((*globalCtx).state.gfxCtx);
            let fresh44 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_3: *mut Gfx = fresh44;
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
                    (((0 as libc::c_int | 0 as libc::c_int | 0 as libc::c_int)
                          ^ 0x1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_3).words.w1 = &mut D_01000000 as *mut Mtx as libc::c_uint;
            let fresh45 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_4: *mut Gfx = fresh45;
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
            (*_g_4).words.w1 = gEffLightningDL.as_mut_ptr() as libc::c_uint
        }
        i += 1
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_kankyo.c\x00" as *const u8 as *const libc::c_char,
                     3353 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn func_800758AC(mut globalCtx: *mut GlobalContext) {
    (*globalCtx).envCtx.unk_E0 = 0xff as libc::c_int as u8_0;
    // both lost woods exits on the bridge from kokiri to hyrule field
    if gSaveContext.entranceIndex == 0x4de as libc::c_int ||
           gSaveContext.entranceIndex == 0x5e0 as libc::c_int {
        func_800F6FB4(4 as libc::c_int as u8_0);
    } else if gSaveContext.forcedSeqId as libc::c_int != 0 as libc::c_int {
        if Environment_IsForcedSequenceDisabled() == 0 {
            Audio_QueueSeqCmd(((SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                                   24 as libc::c_int |
                                   gSaveContext.forcedSeqId as s32) as u32_0);
        }
        gSaveContext.forcedSeqId = 0 as libc::c_int as u16_0
    } else if (*globalCtx).sequenceCtx.seqId as libc::c_int ==
                  0x7f as libc::c_int {
        if (*globalCtx).sequenceCtx.natureAmbienceId as libc::c_int ==
               0x13 as libc::c_int {
            return
        }
        if gSaveContext.natureAmbienceId as libc::c_int !=
               (*globalCtx).sequenceCtx.natureAmbienceId as libc::c_int {
            func_800F6FB4((*globalCtx).sequenceCtx.natureAmbienceId);
        }
    } else if (*globalCtx).sequenceCtx.natureAmbienceId as libc::c_int ==
                  0x13 as libc::c_int {
        // "BGM Configuration"
        osSyncPrintf(b"\n\n\nBGM\xe8\xa8\xad\xe5\xae\x9agame_play->sound_info.BGM=[%d] old_bgm=[%d]\n\n\x00"
                         as *const u8 as *const libc::c_char,
                     (*globalCtx).sequenceCtx.seqId as libc::c_int,
                     gSaveContext.seqId as libc::c_int); // "Forced BGM"
        if gSaveContext.seqId as libc::c_int !=
               (*globalCtx).sequenceCtx.seqId as libc::c_int {
            func_800F5550((*globalCtx).sequenceCtx.seqId as u16_0);
        }
    } else if gSaveContext.dayTime as libc::c_int > 0x4aaa as libc::c_int &&
                  (gSaveContext.dayTime as libc::c_int) <
                      0xb71d as libc::c_int {
        if gSaveContext.seqId as libc::c_int !=
               (*globalCtx).sequenceCtx.seqId as libc::c_int {
            func_800F5550((*globalCtx).sequenceCtx.seqId as u16_0);
        }
        (*globalCtx).envCtx.unk_E0 = 1 as libc::c_int as u8_0
    } else {
        if gSaveContext.natureAmbienceId as libc::c_int !=
               (*globalCtx).sequenceCtx.natureAmbienceId as libc::c_int {
            func_800F6FB4((*globalCtx).sequenceCtx.natureAmbienceId);
        }
        if gSaveContext.dayTime as libc::c_int > 0xb71c as libc::c_int &&
               (gSaveContext.dayTime as libc::c_int) < 0xcaac as libc::c_int {
            (*globalCtx).envCtx.unk_E0 = 3 as libc::c_int as u8_0
        } else if gSaveContext.dayTime as libc::c_int > 0xcaac as libc::c_int
                      ||
                      (gSaveContext.dayTime as libc::c_int) <
                          0x4555 as libc::c_int {
            (*globalCtx).envCtx.unk_E0 = 5 as libc::c_int as u8_0
        } else { (*globalCtx).envCtx.unk_E0 = 7 as libc::c_int as u8_0 }
    }
    osSyncPrintf(b"\n-----------------\n\x00" as *const u8 as
                     *const libc::c_char,
                 gSaveContext.forcedSeqId as libc::c_int);
    osSyncPrintf(b"\n \xe5\xbc\xb7\xe5\x88\xb6\xef\xbc\xa2\xef\xbc\xa7\xef\xbc\xad=[%d]\x00"
                     as *const u8 as *const libc::c_char,
                 gSaveContext.forcedSeqId as libc::c_int);
    osSyncPrintf(b"\n     \xef\xbc\xa2\xef\xbc\xa7\xef\xbc\xad=[%d]\x00" as
                     *const u8 as *const libc::c_char,
                 (*globalCtx).sequenceCtx.seqId as libc::c_int);
    osSyncPrintf(b"\n     \xe3\x82\xa8\xe3\x83\xb3\xe3\x83\x96=[%d]\x00" as
                     *const u8 as *const libc::c_char,
                 (*globalCtx).sequenceCtx.natureAmbienceId as libc::c_int);
    osSyncPrintf(b"\n     status=[%d]\x00" as *const u8 as
                     *const libc::c_char,
                 (*globalCtx).envCtx.unk_E0 as libc::c_int);
    Audio_SetEnvReverb((*globalCtx).roomCtx.curRoom.echo);
}
// updates bgm/sfx and other things as the day progresses
#[no_mangle]
pub unsafe extern "C" fn func_80075B44(mut globalCtx: *mut GlobalContext) {
    match (*globalCtx).envCtx.unk_E0 as libc::c_int {
        0 => {
            func_800F6D58(86 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
                          0 as libc::c_int as u8_0);
            if (*globalCtx).envCtx.unk_EE[0 as libc::c_int as usize] as
                   libc::c_int == 0 as libc::c_int &&
                   (*globalCtx).envCtx.unk_F2[0 as libc::c_int as usize] as
                       libc::c_int == 0 as libc::c_int {
                osSyncPrintf(b"\n\n\nNa_StartMorinigBgm\n\n\x00" as *const u8
                                 as *const libc::c_char);
                func_800F5510((*globalCtx).sequenceCtx.seqId as u16_0);
            }
            (*globalCtx).envCtx.unk_E0 =
                (*globalCtx).envCtx.unk_E0.wrapping_add(1)
        }
        1 => {
            if gSaveContext.dayTime as libc::c_int > 0xb71c as libc::c_int {
                if (*globalCtx).envCtx.unk_EE[0 as libc::c_int as usize] as
                       libc::c_int == 0 as libc::c_int &&
                       (*globalCtx).envCtx.unk_F2[0 as libc::c_int as usize]
                           as libc::c_int == 0 as libc::c_int {
                    Audio_QueueSeqCmd(((0x1 as libc::c_int) <<
                                           28 as libc::c_int |
                                           (SEQ_PLAYER_BGM_MAIN as
                                                libc::c_int) <<
                                               24 as libc::c_int |
                                           0xf000ff as libc::c_int) as u32_0);
                }
                (*globalCtx).envCtx.unk_E0 =
                    (*globalCtx).envCtx.unk_E0.wrapping_add(1)
            }
        }
        2 => {
            if gSaveContext.dayTime as libc::c_int > 0xc000 as libc::c_int {
                func_800788CC(0x28ae as libc::c_int as u16_0);
                (*globalCtx).envCtx.unk_E0 =
                    (*globalCtx).envCtx.unk_E0.wrapping_add(1)
            }
        }
        3 => {
            if (*globalCtx).envCtx.unk_EE[0 as libc::c_int as usize] as
                   libc::c_int == 0 as libc::c_int &&
                   (*globalCtx).envCtx.unk_F2[0 as libc::c_int as usize] as
                       libc::c_int == 0 as libc::c_int {
                func_800F6FB4((*globalCtx).sequenceCtx.natureAmbienceId);
                func_800F6D58(1 as libc::c_int as u8_0,
                              1 as libc::c_int as u8_0,
                              1 as libc::c_int as u8_0);
            }
            (*globalCtx).envCtx.unk_E0 =
                (*globalCtx).envCtx.unk_E0.wrapping_add(1)
        }
        4 => {
            if gSaveContext.dayTime as libc::c_int > 0xcaab as libc::c_int {
                (*globalCtx).envCtx.unk_E0 =
                    (*globalCtx).envCtx.unk_E0.wrapping_add(1)
            }
        }
        5 => {
            func_800F6D58(1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
                          0 as libc::c_int as u8_0);
            if (*globalCtx).envCtx.unk_EE[0 as libc::c_int as usize] as
                   libc::c_int == 0 as libc::c_int &&
                   (*globalCtx).envCtx.unk_F2[0 as libc::c_int as usize] as
                       libc::c_int == 0 as libc::c_int {
                func_800F6D58(36 as libc::c_int as u8_0,
                              1 as libc::c_int as u8_0,
                              1 as libc::c_int as u8_0);
            }
            (*globalCtx).envCtx.unk_E0 =
                (*globalCtx).envCtx.unk_E0.wrapping_add(1)
        }
        6 => {
            if (gSaveContext.dayTime as libc::c_int) < 0xcaac as libc::c_int
                   &&
                   gSaveContext.dayTime as libc::c_int > 0x4555 as libc::c_int
               {
                gSaveContext.totalDays += 1;
                gSaveContext.bgsDayCount += 1;
                gSaveContext.dogIsLost = 1 as libc::c_int as u8_0;
                func_80078884(0x2813 as libc::c_int as u16_0);
                if (Inventory_ReplaceItem(globalCtx,
                                          ITEM_WEIRD_EGG as libc::c_int as
                                              u16_0,
                                          ITEM_CHICKEN as libc::c_int as
                                              u16_0) != 0 ||
                        Inventory_ReplaceItem(globalCtx,
                                              ITEM_POCKET_EGG as libc::c_int
                                                  as u16_0,
                                              ITEM_POCKET_CUCCO as libc::c_int
                                                  as u16_0) != 0) &&
                       (*globalCtx).csCtx.state as libc::c_int ==
                           0 as libc::c_int && Player_InCsMode(globalCtx) == 0
                   {
                    Message_StartTextbox(globalCtx,
                                         0x3066 as libc::c_int as u16_0,
                                         0 as *mut Actor);
                }
                (*globalCtx).envCtx.unk_E0 =
                    (*globalCtx).envCtx.unk_E0.wrapping_add(1)
            }
        }
        7 => {
            func_800F6D58(36 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
                          0 as libc::c_int as u8_0);
            if (*globalCtx).envCtx.unk_EE[0 as libc::c_int as usize] as
                   libc::c_int == 0 as libc::c_int &&
                   (*globalCtx).envCtx.unk_F2[0 as libc::c_int as usize] as
                       libc::c_int == 0 as libc::c_int {
                func_800F6D58(86 as libc::c_int as u8_0,
                              1 as libc::c_int as u8_0,
                              1 as libc::c_int as u8_0);
            }
            (*globalCtx).envCtx.unk_E0 =
                (*globalCtx).envCtx.unk_E0.wrapping_add(1)
        }
        8 => {
            if gSaveContext.dayTime as libc::c_int > 0x4aab as libc::c_int {
                (*globalCtx).envCtx.unk_E0 = 0 as libc::c_int as u8_0
            }
        }
        _ => { }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Environment_DrawCustomLensFlare(mut globalCtx:
                                                             *mut GlobalContext) {
    let mut pos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    if gCustomLensFlareOn != 0 {
        pos.x = gCustomLensFlarePos.x;
        pos.y = gCustomLensFlarePos.y;
        pos.z = gCustomLensFlarePos.z;
        Environment_DrawLensFlare(globalCtx, &mut (*globalCtx).envCtx,
                                  &mut (*globalCtx).view,
                                  (*globalCtx).state.gfxCtx, pos,
                                  gLensFlareUnused as s32, gLensFlareScale,
                                  gLensFlareColorIntensity,
                                  gLensFlareScreenFillAlpha,
                                  0 as libc::c_int as u8_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Environment_InitGameOverLights(mut globalCtx:
                                                            *mut GlobalContext) {
    let mut pad: s32 = 0;
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    sGameOverLightsIntensity = 0 as libc::c_int as u8_0;
    Lights_PointNoGlowSetInfo(&mut sNGameOverLightInfo,
                              ((*player).actor.world.pos.x as s16 as
                                   libc::c_int as libc::c_float - 10.0f32) as
                                  s16,
                              ((*player).actor.world.pos.y as s16 as
                                   libc::c_int as libc::c_float + 10.0f32) as
                                  s16,
                              ((*player).actor.world.pos.z as s16 as
                                   libc::c_int as libc::c_float - 10.0f32) as
                                  s16, 0 as libc::c_int as u8_0,
                              0 as libc::c_int as u8_0,
                              0 as libc::c_int as u8_0,
                              255 as libc::c_int as s16);
    sNGameOverLightNode =
        LightContext_InsertLight(globalCtx, &mut (*globalCtx).lightCtx,
                                 &mut sNGameOverLightInfo);
    Lights_PointNoGlowSetInfo(&mut sSGameOverLightInfo,
                              ((*player).actor.world.pos.x as s16 as
                                   libc::c_int as libc::c_float + 10.0f32) as
                                  s16,
                              ((*player).actor.world.pos.y as s16 as
                                   libc::c_int as libc::c_float + 10.0f32) as
                                  s16,
                              ((*player).actor.world.pos.z as s16 as
                                   libc::c_int as libc::c_float + 10.0f32) as
                                  s16, 0 as libc::c_int as u8_0,
                              0 as libc::c_int as u8_0,
                              0 as libc::c_int as u8_0,
                              255 as libc::c_int as s16);
    sSGameOverLightNode =
        LightContext_InsertLight(globalCtx, &mut (*globalCtx).lightCtx,
                                 &mut sSGameOverLightInfo);
}
#[no_mangle]
pub unsafe extern "C" fn Environment_FadeInGameOverLights(mut globalCtx:
                                                              *mut GlobalContext) {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut i: s16 = 0;
    Lights_PointNoGlowSetInfo(&mut sNGameOverLightInfo,
                              ((*player).actor.world.pos.x as s16 as
                                   libc::c_int as libc::c_float - 10.0f32) as
                                  s16,
                              ((*player).actor.world.pos.y as s16 as
                                   libc::c_int as libc::c_float + 10.0f32) as
                                  s16,
                              ((*player).actor.world.pos.z as s16 as
                                   libc::c_int as libc::c_float - 10.0f32) as
                                  s16, sGameOverLightsIntensity,
                              sGameOverLightsIntensity,
                              sGameOverLightsIntensity,
                              255 as libc::c_int as s16);
    Lights_PointNoGlowSetInfo(&mut sSGameOverLightInfo,
                              ((*player).actor.world.pos.x as s16 as
                                   libc::c_int as libc::c_float + 10.0f32) as
                                  s16,
                              ((*player).actor.world.pos.y as s16 as
                                   libc::c_int as libc::c_float + 10.0f32) as
                                  s16,
                              ((*player).actor.world.pos.z as s16 as
                                   libc::c_int as libc::c_float + 10.0f32) as
                                  s16, sGameOverLightsIntensity,
                              sGameOverLightsIntensity,
                              sGameOverLightsIntensity,
                              255 as libc::c_int as s16);
    if (sGameOverLightsIntensity as libc::c_int) < 254 as libc::c_int {
        sGameOverLightsIntensity =
            (sGameOverLightsIntensity as libc::c_int + 2 as libc::c_int) as
                u8_0
    }
    if func_800C0CB8(globalCtx) != 0 {
        i = 0 as libc::c_int as s16;
        while (i as libc::c_int) < 3 as libc::c_int {
            if (*globalCtx).envCtx.adjAmbientColor[i as usize] as libc::c_int
                   > -(255 as libc::c_int) {
                (*globalCtx).envCtx.adjAmbientColor[i as usize] =
                    ((*globalCtx).envCtx.adjAmbientColor[i as usize] as
                         libc::c_int - 12 as libc::c_int) as s16;
                (*globalCtx).envCtx.adjLight1Color[i as usize] =
                    ((*globalCtx).envCtx.adjLight1Color[i as usize] as
                         libc::c_int - 12 as libc::c_int) as s16
            }
            (*globalCtx).envCtx.adjFogColor[i as usize] =
                -(255 as libc::c_int) as s16;
            i += 1
        }
        if (*globalCtx).envCtx.lightSettings.fogFar as libc::c_int +
               (*globalCtx).envCtx.adjFogFar as libc::c_int >
               900 as libc::c_int {
            (*globalCtx).envCtx.adjFogFar =
                ((*globalCtx).envCtx.adjFogFar as libc::c_int -
                     100 as libc::c_int) as s16
        }
        if (*globalCtx).envCtx.lightSettings.fogNear as libc::c_int +
               (*globalCtx).envCtx.adjFogNear as libc::c_int >
               950 as libc::c_int {
            (*globalCtx).envCtx.adjFogNear =
                ((*globalCtx).envCtx.adjFogNear as libc::c_int -
                     10 as libc::c_int) as s16
        }
    } else {
        (*globalCtx).envCtx.fillScreen = 1 as libc::c_int as u8_0;
        (*globalCtx).envCtx.screenFillColor[0 as libc::c_int as usize] =
            0 as libc::c_int as u8_0;
        (*globalCtx).envCtx.screenFillColor[1 as libc::c_int as usize] =
            0 as libc::c_int as u8_0;
        (*globalCtx).envCtx.screenFillColor[2 as libc::c_int as usize] =
            0 as libc::c_int as u8_0;
        (*globalCtx).envCtx.screenFillColor[3 as libc::c_int as usize] =
            sGameOverLightsIntensity
    };
}
#[no_mangle]
pub unsafe extern "C" fn Environment_FadeOutGameOverLights(mut globalCtx:
                                                               *mut GlobalContext) {
    let mut player: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut i: s16 = 0;
    if sGameOverLightsIntensity as libc::c_int >= 3 as libc::c_int {
        sGameOverLightsIntensity =
            (sGameOverLightsIntensity as libc::c_int - 3 as libc::c_int) as
                u8_0
    } else { sGameOverLightsIntensity = 0 as libc::c_int as u8_0 }
    if sGameOverLightsIntensity as libc::c_int == 1 as libc::c_int {
        LightContext_RemoveLight(globalCtx, &mut (*globalCtx).lightCtx,
                                 sNGameOverLightNode);
        LightContext_RemoveLight(globalCtx, &mut (*globalCtx).lightCtx,
                                 sSGameOverLightNode);
    } else if sGameOverLightsIntensity as libc::c_int >= 2 as libc::c_int {
        Lights_PointNoGlowSetInfo(&mut sNGameOverLightInfo,
                                  ((*player).actor.world.pos.x as s16 as
                                       libc::c_int as libc::c_float - 10.0f32)
                                      as s16,
                                  ((*player).actor.world.pos.y as s16 as
                                       libc::c_int as libc::c_float + 10.0f32)
                                      as s16,
                                  ((*player).actor.world.pos.z as s16 as
                                       libc::c_int as libc::c_float - 10.0f32)
                                      as s16, sGameOverLightsIntensity,
                                  sGameOverLightsIntensity,
                                  sGameOverLightsIntensity,
                                  255 as libc::c_int as s16);
        Lights_PointNoGlowSetInfo(&mut sSGameOverLightInfo,
                                  ((*player).actor.world.pos.x as s16 as
                                       libc::c_int as libc::c_float + 10.0f32)
                                      as s16,
                                  ((*player).actor.world.pos.y as s16 as
                                       libc::c_int as libc::c_float + 10.0f32)
                                      as s16,
                                  ((*player).actor.world.pos.z as s16 as
                                       libc::c_int as libc::c_float + 10.0f32)
                                      as s16, sGameOverLightsIntensity,
                                  sGameOverLightsIntensity,
                                  sGameOverLightsIntensity,
                                  255 as libc::c_int as s16);
    }
    if func_800C0CB8(globalCtx) != 0 {
        i = 0 as libc::c_int as s16;
        while (i as libc::c_int) < 3 as libc::c_int {
            Math_SmoothStepToS(&mut *(*globalCtx).envCtx.adjAmbientColor.as_mut_ptr().offset(i
                                                                                                 as
                                                                                                 isize),
                               0 as libc::c_int as s16,
                               5 as libc::c_int as s16,
                               12 as libc::c_int as s16,
                               1 as libc::c_int as s16);
            Math_SmoothStepToS(&mut *(*globalCtx).envCtx.adjLight1Color.as_mut_ptr().offset(i
                                                                                                as
                                                                                                isize),
                               0 as libc::c_int as s16,
                               5 as libc::c_int as s16,
                               12 as libc::c_int as s16,
                               1 as libc::c_int as s16);
            (*globalCtx).envCtx.adjFogColor[i as usize] =
                0 as libc::c_int as s16;
            i += 1
        }
        (*globalCtx).envCtx.adjFogFar = 0 as libc::c_int as s16;
        (*globalCtx).envCtx.adjFogNear = 0 as libc::c_int as s16
    } else {
        (*globalCtx).envCtx.fillScreen = 1 as libc::c_int as u8_0;
        (*globalCtx).envCtx.screenFillColor[0 as libc::c_int as usize] =
            0 as libc::c_int as u8_0;
        (*globalCtx).envCtx.screenFillColor[1 as libc::c_int as usize] =
            0 as libc::c_int as u8_0;
        (*globalCtx).envCtx.screenFillColor[2 as libc::c_int as usize] =
            0 as libc::c_int as u8_0;
        (*globalCtx).envCtx.screenFillColor[3 as libc::c_int as usize] =
            sGameOverLightsIntensity;
        if sGameOverLightsIntensity as libc::c_int == 0 as libc::c_int {
            (*globalCtx).envCtx.fillScreen = 0 as libc::c_int as u8_0
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800766C4(mut globalCtx: *mut GlobalContext) {
    let mut max: u8_0 =
        if (*globalCtx).envCtx.unk_EE[0 as libc::c_int as usize] as
               libc::c_int >
               (*globalCtx).envCtx.unk_F2[0 as libc::c_int as usize] as
                   libc::c_int {
            (*globalCtx).envCtx.unk_EE[0 as libc::c_int as usize] as
                libc::c_int
        } else {
            (*globalCtx).envCtx.unk_F2[0 as libc::c_int as usize] as
                libc::c_int
        } as u8_0;
    if (*globalCtx).envCtx.unk_EE[1 as libc::c_int as usize] as libc::c_int !=
           max as libc::c_int &&
           (*globalCtx).state.frames.wrapping_rem(8 as libc::c_int as
                                                      libc::c_uint) ==
               0 as libc::c_int as libc::c_uint {
        if ((*globalCtx).envCtx.unk_EE[1 as libc::c_int as usize] as
                libc::c_int) < max as libc::c_int {
            (*globalCtx).envCtx.unk_EE[1 as libc::c_int as usize] =
                ((*globalCtx).envCtx.unk_EE[1 as libc::c_int as usize] as
                     libc::c_int + 2 as libc::c_int) as u8_0
        } else {
            (*globalCtx).envCtx.unk_EE[1 as libc::c_int as usize] =
                ((*globalCtx).envCtx.unk_EE[1 as libc::c_int as usize] as
                     libc::c_int - 2 as libc::c_int) as u8_0
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Environment_FillScreen(mut gfxCtx:
                                                    *mut GraphicsContext,
                                                mut red: u8_0,
                                                mut green: u8_0,
                                                mut blue: u8_0,
                                                mut alpha: u8_0,
                                                mut drawFlags: u8_0) {
    if alpha as libc::c_int != 0 as libc::c_int {
        let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
        let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
        __gfxCtx = gfxCtx;
        Graph_OpenDisps(dispRefs.as_mut_ptr(), gfxCtx,
                        b"../z_kankyo.c\x00" as *const u8 as
                            *const libc::c_char, 3835 as libc::c_int);
        if drawFlags as libc::c_int & (1 as libc::c_int) << 0 as libc::c_int
               != 0 {
            (*__gfxCtx).polyOpa.p = func_800937C0((*__gfxCtx).polyOpa.p);
            let fresh46 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g: *mut Gfx = fresh46;
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
                (red as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (green as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (blue as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    (alpha as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh47 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_0: *mut Gfx = fresh47;
            (*_g_0).words.w0 =
                (0xe3 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((32 as libc::c_int - 4 as libc::c_int - 2 as libc::c_int)
                         as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    ((2 as libc::c_int - 1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_0).words.w1 =
                ((3 as libc::c_int) << 4 as libc::c_int) as libc::c_uint;
            let fresh48 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_1: *mut Gfx = fresh48;
            (*_g_1).words.w0 =
                (0xe3 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((32 as libc::c_int - 6 as libc::c_int - 2 as libc::c_int)
                         as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    ((2 as libc::c_int - 1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_1).words.w1 =
                ((3 as libc::c_int) << 6 as libc::c_int) as libc::c_uint;
            let fresh49 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_2: *mut Gfx = fresh49;
            (*_g_2).words.w0 =
                (0xf6 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((320 as libc::c_int - 1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 10 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        14 as libc::c_int |
                    ((240 as libc::c_int - 1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 10 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        2 as libc::c_int;
            (*_g_2).words.w1 =
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 10 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    14 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 10 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        2 as libc::c_int
        }
        if drawFlags as libc::c_int & (1 as libc::c_int) << 1 as libc::c_int
               != 0 {
            (*__gfxCtx).polyXlu.p = func_800937C0((*__gfxCtx).polyXlu.p);
            let fresh50 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_3: *mut Gfx = fresh50;
            (*_g_3).words.w0 =
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
            (*_g_3).words.w1 =
                (red as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (green as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (blue as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    (alpha as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            if alpha as u32_0 == 255 as libc::c_int as libc::c_uint {
                let fresh51 = (*__gfxCtx).polyXlu.p;
                (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
                let mut _g_4: *mut Gfx = fresh51;
                (*_g_4).words.w0 =
                    (0xe2 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        ((32 as libc::c_int - 3 as libc::c_int -
                              29 as libc::c_int) as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            8 as libc::c_int |
                        ((29 as libc::c_int - 1 as libc::c_int) as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                (*_g_4).words.w1 =
                    (0 as libc::c_int | 0x4000 as libc::c_int |
                         0 as libc::c_int |
                         (0 as libc::c_int) << 30 as libc::c_int |
                         (3 as libc::c_int) << 26 as libc::c_int |
                         (0 as libc::c_int) << 22 as libc::c_int |
                         (2 as libc::c_int) << 18 as libc::c_int |
                         (0 as libc::c_int | 0x4000 as libc::c_int |
                              0 as libc::c_int |
                              (0 as libc::c_int) << 28 as libc::c_int |
                              (3 as libc::c_int) << 24 as libc::c_int |
                              (0 as libc::c_int) << 20 as libc::c_int |
                              (2 as libc::c_int) << 16 as libc::c_int)) as
                        libc::c_uint
            }
            let fresh52 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_5: *mut Gfx = fresh52;
            (*_g_5).words.w0 =
                (0xe3 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((32 as libc::c_int - 4 as libc::c_int - 2 as libc::c_int)
                         as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    ((2 as libc::c_int - 1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_5).words.w1 =
                ((3 as libc::c_int) << 4 as libc::c_int) as libc::c_uint;
            let fresh53 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_6: *mut Gfx = fresh53;
            (*_g_6).words.w0 =
                (0xe3 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((32 as libc::c_int - 6 as libc::c_int - 2 as libc::c_int)
                         as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    ((2 as libc::c_int - 1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_6).words.w1 =
                ((3 as libc::c_int) << 6 as libc::c_int) as libc::c_uint;
            let fresh54 = (*__gfxCtx).polyXlu.p;
            (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
            let mut _g_7: *mut Gfx = fresh54;
            (*_g_7).words.w0 =
                (0xf6 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((320 as libc::c_int - 1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 10 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        14 as libc::c_int |
                    ((240 as libc::c_int - 1 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 10 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        2 as libc::c_int;
            (*_g_7).words.w1 =
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 10 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    14 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 10 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        2 as libc::c_int
        }
        Graph_CloseDisps(dispRefs.as_mut_ptr(), gfxCtx,
                         b"../z_kankyo.c\x00" as *const u8 as
                             *const libc::c_char, 3863 as libc::c_int);
    };
}
#[no_mangle]
pub static mut sSandstormPrimColors: [Color_RGB8; 4] =
    [{
         let mut init =
             Color_RGB8{r: 210 as libc::c_int as u8_0,
                        g: 156 as libc::c_int as u8_0,
                        b: 85 as libc::c_int as u8_0,};
         init
     },
     {
         let mut init =
             Color_RGB8{r: 255 as libc::c_int as u8_0,
                        g: 200 as libc::c_int as u8_0,
                        b: 100 as libc::c_int as u8_0,};
         init
     },
     {
         let mut init =
             Color_RGB8{r: 225 as libc::c_int as u8_0,
                        g: 160 as libc::c_int as u8_0,
                        b: 50 as libc::c_int as u8_0,};
         init
     },
     {
         let mut init =
             Color_RGB8{r: 105 as libc::c_int as u8_0,
                        g: 90 as libc::c_int as u8_0,
                        b: 40 as libc::c_int as u8_0,};
         init
     }];
#[no_mangle]
pub static mut sSandstormEnvColors: [Color_RGB8; 4] =
    [{
         let mut init =
             Color_RGB8{r: 155 as libc::c_int as u8_0,
                        g: 106 as libc::c_int as u8_0,
                        b: 35 as libc::c_int as u8_0,};
         init
     },
     {
         let mut init =
             Color_RGB8{r: 200 as libc::c_int as u8_0,
                        g: 150 as libc::c_int as u8_0,
                        b: 50 as libc::c_int as u8_0,};
         init
     },
     {
         let mut init =
             Color_RGB8{r: 170 as libc::c_int as u8_0,
                        g: 110 as libc::c_int as u8_0,
                        b: 0 as libc::c_int as u8_0,};
         init
     },
     {
         let mut init =
             Color_RGB8{r: 50 as libc::c_int as u8_0,
                        g: 40 as libc::c_int as u8_0,
                        b: 0 as libc::c_int as u8_0,};
         init
     }];
#[no_mangle]
pub unsafe extern "C" fn Environment_DrawSandstorm(mut globalCtx:
                                                       *mut GlobalContext,
                                                   mut sandstormState: u8_0) {
    let mut primA1: s32 = 0;
    let mut envA1: s32 = 0;
    let mut primA: s32 = (*globalCtx).envCtx.sandstormPrimA as s32;
    let mut envA: s32 = (*globalCtx).envCtx.sandstormEnvA as s32;
    let mut primColor: Color_RGBA8 = Color_RGBA8{r: 0, g: 0, b: 0, a: 0,};
    let mut envColor: Color_RGBA8 = Color_RGBA8{r: 0, g: 0, b: 0, a: 0,};
    let mut pad: s32 = 0;
    let mut sp98: f32_0 = 0.;
    let mut sp96: u16_0 = 0;
    let mut sp94: u16_0 = 0;
    let mut sp92: u16_0 = 0;
    match sandstormState as libc::c_int {
        3 => {
            if (*globalCtx).sceneNum as libc::c_int ==
                   SCENE_SPOT13 as libc::c_int &&
                   (*globalCtx).roomCtx.curRoom.num as libc::c_int ==
                       0 as libc::c_int {
                envA1 = 0 as libc::c_int;
                primA1 =
                    if (*globalCtx).envCtx.sandstormEnvA as libc::c_int >
                           128 as libc::c_int {
                        255 as libc::c_int
                    } else {
                        ((*globalCtx).envCtx.sandstormEnvA as libc::c_int) >>
                            1 as libc::c_int
                    }
            } else {
                primA1 =
                    (*globalCtx).state.frames.wrapping_rem(128 as libc::c_int
                                                               as
                                                               libc::c_uint)
                        as s32;
                if primA1 > 64 as libc::c_int {
                    primA1 = 128 as libc::c_int - primA1
                }
                primA1 += 73 as libc::c_int;
                envA1 = 128 as libc::c_int
            }
        }
        1 => {
            primA1 = 255 as libc::c_int;
            envA1 =
                if (*globalCtx).envCtx.sandstormPrimA as libc::c_int >=
                       255 as libc::c_int {
                    255 as libc::c_int
                } else { 128 as libc::c_int }
        }
        2 => {
            envA1 = 128 as libc::c_int;
            if (*globalCtx).envCtx.sandstormEnvA as libc::c_int >
                   128 as libc::c_int {
                primA1 = 0xff as libc::c_int
            } else {
                primA1 =
                    (*globalCtx).state.frames.wrapping_rem(128 as libc::c_int
                                                               as
                                                               libc::c_uint)
                        as s32;
                if primA1 > 64 as libc::c_int {
                    primA1 = 128 as libc::c_int - primA1
                }
                primA1 += 73 as libc::c_int
            }
            if primA1 >= primA && primA1 != 255 as libc::c_int {
                (*globalCtx).envCtx.sandstormState = 3 as libc::c_int as u8_0
            }
        }
        4 => {
            envA1 = 0 as libc::c_int;
            primA1 =
                if (*globalCtx).envCtx.sandstormEnvA as libc::c_int >
                       128 as libc::c_int {
                    255 as libc::c_int
                } else {
                    ((*globalCtx).envCtx.sandstormEnvA as libc::c_int) >>
                        1 as libc::c_int
                };
            if primA == 0 as libc::c_int {
                (*globalCtx).envCtx.sandstormState = 0 as libc::c_int as u8_0
            }
        }
        _ => { }
    }
    if (if primA - primA1 >= 0 as libc::c_int {
            (primA) - primA1
        } else { -(primA - primA1) }) < 9 as libc::c_int {
        primA = primA1
    } else if primA1 < primA {
        primA = primA - 9 as libc::c_int
    } else { primA = primA + 9 as libc::c_int }
    if (if envA - envA1 >= 0 as libc::c_int {
            (envA) - envA1
        } else { -(envA - envA1) }) < 9 as libc::c_int {
        envA = envA1
    } else if envA1 < envA {
        envA = envA - 9 as libc::c_int
    } else { envA = envA + 9 as libc::c_int }
    (*globalCtx).envCtx.sandstormPrimA = primA as u8_0;
    (*globalCtx).envCtx.sandstormEnvA = envA as u8_0;
    sp98 = (512.0f32 - (primA + envA) as libc::c_float) * (3.0f32 / 128.0f32);
    if sp98 > 6.0f32 { sp98 = 6.0f32 }
    if (*globalCtx).envCtx.indoors as libc::c_int != 0 ||
           (*globalCtx).envCtx.unk_BF as libc::c_int != 0xff as libc::c_int {
        primColor.r = sSandstormPrimColors[1 as libc::c_int as usize].r;
        primColor.g = sSandstormPrimColors[1 as libc::c_int as usize].g;
        primColor.b = sSandstormPrimColors[1 as libc::c_int as usize].b;
        envColor.r = sSandstormEnvColors[1 as libc::c_int as usize].r;
        envColor.g = sSandstormEnvColors[1 as libc::c_int as usize].g;
        envColor.b = sSandstormEnvColors[1 as libc::c_int as usize].b
    } else if D_8011FDCC as libc::c_int == D_8011FDD0 as libc::c_int {
        primColor.r = sSandstormPrimColors[D_8011FDCC as usize].r;
        primColor.g = sSandstormPrimColors[D_8011FDCC as usize].g;
        primColor.b = sSandstormPrimColors[D_8011FDCC as usize].b;
        envColor.r = sSandstormEnvColors[D_8011FDCC as usize].r;
        envColor.g = sSandstormEnvColors[D_8011FDCC as usize].g;
        envColor.b = sSandstormEnvColors[D_8011FDCC as usize].b
    } else {
        primColor.r =
            (sSandstormPrimColors[D_8011FDCC as usize].r as libc::c_int as
                 libc::c_float * (1.0f32 - D_8011FDD4) +
                 sSandstormPrimColors[D_8011FDD0 as usize].r as libc::c_int as
                     libc::c_float * D_8011FDD4) as s32 as u8_0;
        primColor.g =
            (sSandstormPrimColors[D_8011FDCC as usize].g as libc::c_int as
                 libc::c_float * (1.0f32 - D_8011FDD4) +
                 sSandstormPrimColors[D_8011FDD0 as usize].g as libc::c_int as
                     libc::c_float * D_8011FDD4) as s32 as u8_0;
        primColor.b =
            (sSandstormPrimColors[D_8011FDCC as usize].b as libc::c_int as
                 libc::c_float * (1.0f32 - D_8011FDD4) +
                 sSandstormPrimColors[D_8011FDD0 as usize].b as libc::c_int as
                     libc::c_float * D_8011FDD4) as s32 as u8_0;
        envColor.r =
            (sSandstormEnvColors[D_8011FDCC as usize].r as libc::c_int as
                 libc::c_float * (1.0f32 - D_8011FDD4) +
                 sSandstormEnvColors[D_8011FDD0 as usize].r as libc::c_int as
                     libc::c_float * D_8011FDD4) as s32 as u8_0;
        envColor.g =
            (sSandstormEnvColors[D_8011FDCC as usize].g as libc::c_int as
                 libc::c_float * (1.0f32 - D_8011FDD4) +
                 sSandstormEnvColors[D_8011FDD0 as usize].g as libc::c_int as
                     libc::c_float * D_8011FDD4) as s32 as u8_0;
        envColor.b =
            (sSandstormEnvColors[D_8011FDCC as usize].b as libc::c_int as
                 libc::c_float * (1.0f32 - D_8011FDD4) +
                 sSandstormEnvColors[D_8011FDD0 as usize].b as libc::c_int as
                     libc::c_float * D_8011FDD4) as s32 as u8_0
    }
    envColor.r =
        ((envColor.r as libc::c_int as libc::c_float * sp98 +
              (6.0f32 - sp98) * primColor.r as libc::c_int as libc::c_float) *
             (1.0f32 / 6.0f32)) as u8_0;
    envColor.g =
        ((envColor.g as libc::c_int as libc::c_float * sp98 +
              (6.0f32 - sp98) * primColor.g as libc::c_int as libc::c_float) *
             (1.0f32 / 6.0f32)) as u8_0;
    envColor.b =
        ((envColor.b as libc::c_int as libc::c_float * sp98 +
              (6.0f32 - sp98) * primColor.b as libc::c_int as libc::c_float) *
             (1.0f32 / 6.0f32)) as u8_0;
    sp96 =
        (D_8015FDB0 as libc::c_int as libc::c_float * (11.0f32 / 6.0f32)) as
            s32 as u16_0;
    sp94 =
        (D_8015FDB0 as libc::c_int as libc::c_float * (9.0f32 / 6.0f32)) as
            s32 as u16_0;
    sp92 =
        (D_8015FDB0 as libc::c_int as libc::c_float * (6.0f32 / 6.0f32)) as
            s32 as u16_0;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_kankyo.c\x00" as *const u8 as *const libc::c_char,
                    4044 as libc::c_int);
    (*__gfxCtx).polyXlu.p = func_80093F34((*__gfxCtx).polyXlu.p);
    let fresh55 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g: *mut Gfx = fresh55;
    (*_g).words.w0 =
        (0xe3 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((32 as libc::c_int - 4 as libc::c_int - 2 as libc::c_int) as
                 u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            ((2 as libc::c_int - 1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g).words.w1 = ((2 as libc::c_int) << 4 as libc::c_int) as libc::c_uint;
    let fresh56 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_0: *mut Gfx = fresh56;
    (*_g_0).words.w0 =
        (0xe3 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((32 as libc::c_int - 6 as libc::c_int - 2 as libc::c_int) as
                 u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            ((2 as libc::c_int - 1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_0).words.w1 =
        ((2 as libc::c_int) << 6 as libc::c_int) as libc::c_uint;
    let fresh57 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_1: *mut Gfx = fresh57;
    (*_g_1).words.w0 =
        (0xfa as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (0x80 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_1).words.w1 =
        (primColor.r as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (primColor.g as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (primColor.b as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            ((*globalCtx).envCtx.sandstormPrimA as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh58 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_2: *mut Gfx = fresh58;
    (*_g_2).words.w0 =
        (0xfb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_2).words.w1 =
        (envColor.r as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (envColor.g as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (envColor.b as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            ((*globalCtx).envCtx.sandstormEnvA as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh59 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_3: *mut Gfx = fresh59;
    (*_g_3).words.w0 =
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
    (*_g_3).words.w1 =
        Gfx_TwoTexScroll((*globalCtx).state.gfxCtx, 0 as libc::c_int,
                         (sp96 as
                              u32_0).wrapping_rem(0x1000 as libc::c_int as
                                                      libc::c_uint),
                         0 as libc::c_int as u32_0, 0x200 as libc::c_int,
                         0x20 as libc::c_int, 1 as libc::c_int,
                         (sp94 as
                              u32_0).wrapping_rem(0x1000 as libc::c_int as
                                                      libc::c_uint),
                         (0xfff as libc::c_int as
                              libc::c_uint).wrapping_sub((sp92 as
                                                              u32_0).wrapping_rem(0x1000
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_uint)),
                         0x100 as libc::c_int, 0x40 as libc::c_int) as
            libc::c_uint;
    let fresh60 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_4: *mut Gfx = fresh60;
    (*_g_4).words.w0 =
        (0xe3 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((32 as libc::c_int - 14 as libc::c_int - 2 as libc::c_int) as
                 u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            ((2 as libc::c_int - 1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_4).words.w1 =
        ((0 as libc::c_int) << 14 as libc::c_int) as libc::c_uint;
    let fresh61 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_5: *mut Gfx = fresh61;
    (*_g_5).words.w0 =
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
    (*_g_5).words.w1 = gFieldSandstormDL.as_mut_ptr() as libc::c_uint;
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_kankyo.c\x00" as *const u8 as *const libc::c_char,
                     4068 as libc::c_int);
    D_8015FDB0 = (D_8015FDB0 as libc::c_int + sp98 as s32) as u16_0;
}
#[no_mangle]
pub unsafe extern "C" fn Environment_AdjustLights(mut globalCtx:
                                                      *mut GlobalContext,
                                                  mut arg1: f32_0,
                                                  mut arg2: f32_0,
                                                  mut arg3: f32_0,
                                                  mut arg4: f32_0) {
    let mut temp: f32_0 = 0.;
    let mut i: s32 = 0;
    if (*globalCtx).roomCtx.curRoom.unk_03 as libc::c_int != 5 as libc::c_int
           && func_800C0CB8(globalCtx) != 0 {
        arg1 = if arg1 < 0.0f32 { 0.0f32 } else { arg1 };
        arg1 = if arg1 > 1.0f32 { 1.0f32 } else { arg1 };
        temp = arg1 - arg3;
        if arg1 < arg3 { temp = 0.0f32 }
        (*globalCtx).envCtx.adjFogNear =
            ((arg2 -
                  (*globalCtx).envCtx.lightSettings.fogNear as libc::c_int as
                      libc::c_float) * temp) as s16;
        if arg1 == 0.0f32 {
            i = 0 as libc::c_int;
            while i < 3 as libc::c_int {
                (*globalCtx).envCtx.adjFogColor[i as usize] =
                    0 as libc::c_int as s16;
                i += 1
            }
        } else {
            temp = arg1 * 5.0f32;
            temp = if temp > 1.0f32 { 1.0f32 } else { temp };
            i = 0 as libc::c_int;
            while i < 3 as libc::c_int {
                (*globalCtx).envCtx.adjFogColor[i as usize] =
                    -(((*globalCtx).envCtx.lightSettings.fogColor[i as usize]
                           as libc::c_int as libc::c_float * temp) as s16 as
                          libc::c_int) as s16;
                i += 1
            }
        }
        if arg4 <= 0.0f32 { return }
        arg1 *= arg4;
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            (*globalCtx).envCtx.adjAmbientColor[i as usize] =
                -(((*globalCtx).envCtx.lightSettings.ambientColor[i as usize]
                       as libc::c_int as libc::c_float * arg1) as s16 as
                      libc::c_int) as s16;
            (*globalCtx).envCtx.adjLight1Color[i as usize] =
                -(((*globalCtx).envCtx.lightSettings.light1Color[i as usize]
                       as libc::c_int as libc::c_float * arg1) as s16 as
                      libc::c_int) as s16;
            i += 1
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Environment_GetBgsDayCount() -> s32 {
    return gSaveContext.bgsDayCount;
}
#[no_mangle]
pub unsafe extern "C" fn Environment_ClearBgsDayCount() {
    gSaveContext.bgsDayCount = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Environment_GetTotalDays() -> s32 {
    return gSaveContext.totalDays;
}
#[no_mangle]
pub unsafe extern "C" fn Environment_ForcePlaySequence(mut seqId: u16_0) {
    gSaveContext.forcedSeqId = seqId;
}
#[no_mangle]
pub unsafe extern "C" fn Environment_IsForcedSequenceDisabled() -> s32 {
    let mut isDisabled: s32 = 0 as libc::c_int;
    if gSaveContext.forcedSeqId as libc::c_int == 0xffff as libc::c_int {
        isDisabled = 1 as libc::c_int
    }
    return isDisabled;
}
#[no_mangle]
pub unsafe extern "C" fn func_80077624(mut globalCtx: *mut GlobalContext) {
    if (*globalCtx).sequenceCtx.natureAmbienceId as libc::c_int ==
           19 as libc::c_int {
        func_800F6FB4(5 as libc::c_int as u8_0);
    } else { func_800F6FB4((*globalCtx).sequenceCtx.natureAmbienceId); }
    func_800F6D58(14 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
                  1 as libc::c_int as u8_0);
    func_800F6D58(15 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
                  1 as libc::c_int as u8_0);
}
#[no_mangle]
pub unsafe extern "C" fn func_80077684(mut globalCtx: *mut GlobalContext) {
    func_800F6D58(14 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
                  0 as libc::c_int as u8_0);
    func_800F6D58(15 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
                  0 as libc::c_int as u8_0);
    if func_800FA0B4(SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0) as
           libc::c_int == 0x1 as libc::c_int {
        gSaveContext.seqId = 0x80 as libc::c_int as u8_0;
        func_800758AC(globalCtx);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Environment_WarpSongLeave(mut globalCtx:
                                                       *mut GlobalContext) {
    gWeatherMode = 0 as libc::c_int as u8_0;
    gSaveContext.cutsceneIndex = 0 as libc::c_int;
    gSaveContext.respawnFlag = -(3 as libc::c_int);
    (*globalCtx).nextEntranceIndex =
        gSaveContext.respawn[RESPAWN_MODE_RETURN as libc::c_int as
                                 usize].entranceIndex;
    (*globalCtx).sceneLoadFlag = 0x14 as libc::c_int as s8;
    (*globalCtx).fadeTransition = 3 as libc::c_int as u8_0;
    gSaveContext.nextTransition = 3 as libc::c_int as u8_0;
    match (*globalCtx).nextEntranceIndex as libc::c_int {
        327 => { Flags_SetEventChkInf(0xb9 as libc::c_int); }
        258 => { Flags_SetEventChkInf(0xb1 as libc::c_int); }
        291 => { Flags_SetEventChkInf(0xb8 as libc::c_int); }
        228 => { Flags_SetEventChkInf(0xb6 as libc::c_int); }
        83 => { Flags_SetEventChkInf(0xa7 as libc::c_int); }
        252 | _ => { }
    };
}
unsafe extern "C" fn run_static_initializers() {
    gSkyboxFiles =
        [{
             let mut init =
                 SkyboxFile{file:
                                {
                                    let mut init =
                                        RomFile{vromStart:
                                                    _vr_fine0_staticSegmentRomStart.as_mut_ptr()
                                                        as u32_0,
                                                vromEnd:
                                                    _vr_fine0_staticSegmentRomEnd.as_mut_ptr()
                                                        as u32_0,};
                                    init
                                },
                            palette:
                                {
                                    let mut init =
                                        RomFile{vromStart:
                                                    _vr_fine0_pal_staticSegmentRomStart.as_mut_ptr()
                                                        as u32_0,
                                                vromEnd:
                                                    _vr_fine0_pal_staticSegmentRomEnd.as_mut_ptr()
                                                        as u32_0,};
                                    init
                                },};
             init
         },
         {
             let mut init =
                 SkyboxFile{file:
                                {
                                    let mut init =
                                        RomFile{vromStart:
                                                    _vr_fine1_staticSegmentRomStart.as_mut_ptr()
                                                        as u32_0,
                                                vromEnd:
                                                    _vr_fine1_staticSegmentRomEnd.as_mut_ptr()
                                                        as u32_0,};
                                    init
                                },
                            palette:
                                {
                                    let mut init =
                                        RomFile{vromStart:
                                                    _vr_fine1_pal_staticSegmentRomStart.as_mut_ptr()
                                                        as u32_0,
                                                vromEnd:
                                                    _vr_fine1_pal_staticSegmentRomEnd.as_mut_ptr()
                                                        as u32_0,};
                                    init
                                },};
             init
         },
         {
             let mut init =
                 SkyboxFile{file:
                                {
                                    let mut init =
                                        RomFile{vromStart:
                                                    _vr_fine2_staticSegmentRomStart.as_mut_ptr()
                                                        as u32_0,
                                                vromEnd:
                                                    _vr_fine2_staticSegmentRomEnd.as_mut_ptr()
                                                        as u32_0,};
                                    init
                                },
                            palette:
                                {
                                    let mut init =
                                        RomFile{vromStart:
                                                    _vr_fine2_pal_staticSegmentRomStart.as_mut_ptr()
                                                        as u32_0,
                                                vromEnd:
                                                    _vr_fine2_pal_staticSegmentRomEnd.as_mut_ptr()
                                                        as u32_0,};
                                    init
                                },};
             init
         },
         {
             let mut init =
                 SkyboxFile{file:
                                {
                                    let mut init =
                                        RomFile{vromStart:
                                                    _vr_fine3_staticSegmentRomStart.as_mut_ptr()
                                                        as u32_0,
                                                vromEnd:
                                                    _vr_fine3_staticSegmentRomEnd.as_mut_ptr()
                                                        as u32_0,};
                                    init
                                },
                            palette:
                                {
                                    let mut init =
                                        RomFile{vromStart:
                                                    _vr_fine3_pal_staticSegmentRomStart.as_mut_ptr()
                                                        as u32_0,
                                                vromEnd:
                                                    _vr_fine3_pal_staticSegmentRomEnd.as_mut_ptr()
                                                        as u32_0,};
                                    init
                                },};
             init
         },
         {
             let mut init =
                 SkyboxFile{file:
                                {
                                    let mut init =
                                        RomFile{vromStart:
                                                    _vr_cloud0_staticSegmentRomStart.as_mut_ptr()
                                                        as u32_0,
                                                vromEnd:
                                                    _vr_cloud0_staticSegmentRomEnd.as_mut_ptr()
                                                        as u32_0,};
                                    init
                                },
                            palette:
                                {
                                    let mut init =
                                        RomFile{vromStart:
                                                    _vr_cloud0_pal_staticSegmentRomStart.as_mut_ptr()
                                                        as u32_0,
                                                vromEnd:
                                                    _vr_cloud0_pal_staticSegmentRomEnd.as_mut_ptr()
                                                        as u32_0,};
                                    init
                                },};
             init
         },
         {
             let mut init =
                 SkyboxFile{file:
                                {
                                    let mut init =
                                        RomFile{vromStart:
                                                    _vr_cloud1_staticSegmentRomStart.as_mut_ptr()
                                                        as u32_0,
                                                vromEnd:
                                                    _vr_cloud1_staticSegmentRomEnd.as_mut_ptr()
                                                        as u32_0,};
                                    init
                                },
                            palette:
                                {
                                    let mut init =
                                        RomFile{vromStart:
                                                    _vr_cloud1_pal_staticSegmentRomStart.as_mut_ptr()
                                                        as u32_0,
                                                vromEnd:
                                                    _vr_cloud1_pal_staticSegmentRomEnd.as_mut_ptr()
                                                        as u32_0,};
                                    init
                                },};
             init
         },
         {
             let mut init =
                 SkyboxFile{file:
                                {
                                    let mut init =
                                        RomFile{vromStart:
                                                    _vr_cloud2_staticSegmentRomStart.as_mut_ptr()
                                                        as u32_0,
                                                vromEnd:
                                                    _vr_cloud2_staticSegmentRomEnd.as_mut_ptr()
                                                        as u32_0,};
                                    init
                                },
                            palette:
                                {
                                    let mut init =
                                        RomFile{vromStart:
                                                    _vr_cloud2_pal_staticSegmentRomStart.as_mut_ptr()
                                                        as u32_0,
                                                vromEnd:
                                                    _vr_cloud2_pal_staticSegmentRomEnd.as_mut_ptr()
                                                        as u32_0,};
                                    init
                                },};
             init
         },
         {
             let mut init =
                 SkyboxFile{file:
                                {
                                    let mut init =
                                        RomFile{vromStart:
                                                    _vr_cloud3_staticSegmentRomStart.as_mut_ptr()
                                                        as u32_0,
                                                vromEnd:
                                                    _vr_cloud3_staticSegmentRomEnd.as_mut_ptr()
                                                        as u32_0,};
                                    init
                                },
                            palette:
                                {
                                    let mut init =
                                        RomFile{vromStart:
                                                    _vr_cloud3_pal_staticSegmentRomStart.as_mut_ptr()
                                                        as u32_0,
                                                vromEnd:
                                                    _vr_cloud3_pal_staticSegmentRomEnd.as_mut_ptr()
                                                        as u32_0,};
                                    init
                                },};
             init
         },
         {
             let mut init =
                 SkyboxFile{file:
                                {
                                    let mut init =
                                        RomFile{vromStart:
                                                    _vr_holy0_staticSegmentRomStart.as_mut_ptr()
                                                        as u32_0,
                                                vromEnd:
                                                    _vr_holy0_staticSegmentRomEnd.as_mut_ptr()
                                                        as u32_0,};
                                    init
                                },
                            palette:
                                {
                                    let mut init =
                                        RomFile{vromStart:
                                                    _vr_holy0_pal_staticSegmentRomStart.as_mut_ptr()
                                                        as u32_0,
                                                vromEnd:
                                                    _vr_holy0_pal_staticSegmentRomEnd.as_mut_ptr()
                                                        as u32_0,};
                                    init
                                },};
             init
         }]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
