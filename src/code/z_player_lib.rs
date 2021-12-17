#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn sqrtf(f: f32_0) -> f32_0;
    #[no_mangle]
    fn DmaMgr_SendRequest1(ram0: *mut libc::c_void, vrom: u32_0, size: u32_0,
                           file: *const libc::c_char, line: s32) -> s32;
    #[no_mangle]
    fn osRecvMesg(mq: *mut OSMesgQueue, msg: *mut OSMesg, flag: s32) -> s32;
    #[no_mangle]
    fn EffectBlure_AddVertex(this: *mut EffectBlure, p1: *mut Vec3f,
                             p2: *mut Vec3f);
    #[no_mangle]
    fn Effect_GetByIndex(index: s32) -> *mut libc::c_void;
    #[no_mangle]
    fn EffectSsGFire_Spawn(globalCtx: *mut GlobalContext, pos: *mut Vec3f);
    #[no_mangle]
    fn Actor_SetFeetPos(actor: *mut Actor, limbIndex: s32, leftFootIndex: s32,
                        leftFootPos: *mut Vec3f, rightFootIndex: s32,
                        rightFootPos: *mut Vec3f);
    #[no_mangle]
    fn func_8002DD6C(player: *mut Player) -> s32;
    #[no_mangle]
    fn func_8002DD78(player: *mut Player) -> s32;
    #[no_mangle]
    fn func_8002EABC(object: *mut Vec3f, eye: *mut Vec3f,
                     lightDir: *mut Vec3f, gfxCtx: *mut GraphicsContext)
     -> *mut Hilite;
    #[no_mangle]
    fn BgCheck_EntityRaycastFloor4(colCtx: *mut CollisionContext,
                                   outPoly: *mut *mut CollisionPoly,
                                   bgId: *mut s32, actor: *mut Actor,
                                   arg4: *mut Vec3f) -> f32_0;
    #[no_mangle]
    fn BgCheck_AnyLineTest3(colCtx: *mut CollisionContext, posA: *mut Vec3f,
                            posB: *mut Vec3f, posResult: *mut Vec3f,
                            outPoly: *mut *mut CollisionPoly, chkWall: s32,
                            chkFloor: s32, chkCeil: s32, chkOneFace: s32,
                            bgId: *mut s32) -> s32;
    #[no_mangle]
    fn func_80041D4C(colCtx: *mut CollisionContext, poly: *mut CollisionPoly,
                     bgId: s32) -> u32_0;
    #[no_mangle]
    fn SurfaceType_IsWallDamage(colCtx: *mut CollisionContext,
                                poly: *mut CollisionPoly, bgId: s32) -> u32_0;
    #[no_mangle]
    fn Camera_ChangeMode(camera: *mut Camera, mode: s16) -> s32;
    #[no_mangle]
    fn Camera_SetParam(camera: *mut Camera, param: s32,
                       value: *mut libc::c_void) -> s32;
    #[no_mangle]
    fn Collider_ResetQuadAT(globalCtx: *mut GlobalContext,
                            collider: *mut Collider) -> s32;
    #[no_mangle]
    fn CollisionCheck_SetAT(globalCtx: *mut GlobalContext,
                            colChkCtx: *mut CollisionCheckContext,
                            collider: *mut Collider) -> s32;
    #[no_mangle]
    fn CollisionCheck_SetAC(globalCtx: *mut GlobalContext,
                            colChkCtx: *mut CollisionCheckContext,
                            collider: *mut Collider) -> s32;
    #[no_mangle]
    fn Collider_SetQuadVertices(collider: *mut ColliderQuad, a: *mut Vec3f,
                                b: *mut Vec3f, c: *mut Vec3f, d: *mut Vec3f);
    #[no_mangle]
    fn GetItem_Draw(globalCtx: *mut GlobalContext, drawId: s16);
    #[no_mangle]
    fn Math_CosS(angle: s16) -> f32_0;
    #[no_mangle]
    fn Math_SinS(angle: s16) -> f32_0;
    #[no_mangle]
    fn Math_Vec3f_Copy(dest: *mut Vec3f, src: *mut Vec3f);
    #[no_mangle]
    fn Math_Vec3f_Diff(a: *mut Vec3f, b: *mut Vec3f, dest: *mut Vec3f);
    #[no_mangle]
    fn Math_Vec3f_DistXYZ(a: *mut Vec3f, b: *mut Vec3f) -> f32_0;
    #[no_mangle]
    fn Inventory_UpdateBottleItem(globalCtx: *mut GlobalContext, item: u8_0,
                                  cButton: u8_0);
    #[no_mangle]
    fn Gfx_SetFog2(gfx: *mut Gfx, r: s32, g: s32, b: s32, a: s32, near: s32,
                   far: s32) -> *mut Gfx;
    #[no_mangle]
    fn Gfx_CallSetupDL(gfx: *mut Gfx, i: u32_0) -> *mut Gfx;
    #[no_mangle]
    fn func_80093C80(globalCtx: *mut GlobalContext);
    #[no_mangle]
    fn SkelAnime_DrawFlexLod(globalCtx: *mut GlobalContext,
                             skeleton: *mut *mut libc::c_void,
                             jointTable: *mut Vec3s, dListCount: s32,
                             overrideLimbDraw: OverrideLimbDrawOpa,
                             postLimbDraw: PostLimbDrawOpa,
                             arg: *mut libc::c_void, dListIndex: s32);
    #[no_mangle]
    fn SkelAnime_InitLink(globalCtx: *mut GlobalContext,
                          skelAnime: *mut SkelAnime,
                          skeletonHeaderSeg: *mut FlexSkeletonHeader,
                          animation: *mut LinkAnimationHeader, initFlags: s32,
                          jointTable: *mut Vec3s, morphTable: *mut Vec3s,
                          limbCount: s32);
    #[no_mangle]
    fn SkinMatrix_Vec3fMtxFMultXYZW(mf: *mut MtxF, src: *mut Vec3f,
                                    xyzDest: *mut Vec3f, wDest: *mut f32_0);
    #[no_mangle]
    fn Gameplay_GetCamera(globalCtx: *mut GlobalContext, camId: s16)
     -> *mut Camera;
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
    fn Matrix_RotateZYX(x: s16, y: s16, z: s16, mode: u8_0);
    #[no_mangle]
    fn Matrix_TranslateRotateZYX(translation: *mut Vec3f,
                                 rotation: *mut Vec3s);
    #[no_mangle]
    fn func_800D1694(x: f32_0, y: f32_0, z: f32_0, vec: *mut Vec3s);
    #[no_mangle]
    fn Matrix_NewMtx(gfxCtx: *mut GraphicsContext, file: *mut libc::c_char,
                     line: s32) -> *mut Mtx;
    #[no_mangle]
    fn Matrix_MultVec3f(src: *mut Vec3f, dest: *mut Vec3f);
    #[no_mangle]
    fn Matrix_MtxFToYXZRotS(mf: *mut MtxF, rotDest: *mut Vec3s, flag: s32);
    #[no_mangle]
    static mut gGameInfo: *mut GameInfo;
    #[no_mangle]
    static mut gSaveContext: SaveContext;
    #[no_mangle]
    static mut gEquipShifts: [u8_0; 4];
    #[no_mangle]
    static mut gEquipMasks: [u16_0; 4];
    #[no_mangle]
    static mut gUpgradeShifts: [u8_0; 8];
    #[no_mangle]
    static mut gUpgradeMasks: [u32_0; 8];
    #[no_mangle]
    fn Message_StartTextbox(globalCtx: *mut GlobalContext, textId: u16_0,
                            actor: *mut Actor);
    #[no_mangle]
    static mut D_060257B8: [Gfx; 0];
    #[no_mangle]
    static mut D_06025658: [Gfx; 0];
    #[no_mangle]
    static mut D_06025438: [Gfx; 0];
    #[no_mangle]
    static mut D_060252D8: [Gfx; 0];
    #[no_mangle]
    static mut D_06025598: [Gfx; 0];
    #[no_mangle]
    static mut D_06025218: [Gfx; 0];
    #[no_mangle]
    fn Math_FAtan2F(y: f32_0, x: f32_0) -> f32_0;
    #[no_mangle]
    static mut D_0602A738: [Gfx; 0];
    #[no_mangle]
    static mut gSegments: [u32_0; 16];
    #[no_mangle]
    static mut D_0602CB48: [Gfx; 0];
    #[no_mangle]
    static mut gLinkObjectIds: [s16; 2];
    #[no_mangle]
    static mut gObjectTable: [RomFile; 402];
    #[no_mangle]
    fn guLookAt(_: *mut Mtx, xEye: f32_0, yEye: f32_0, zEye: f32_0,
                xAt: f32_0, yAt: f32_0, zAt: f32_0, xUp: f32_0, yUp: f32_0,
                zUp: f32_0);
    #[no_mangle]
    fn guPerspective(m: *mut Mtx, perspNorm: *mut u16_0, fovy: f32_0,
                     aspect: f32_0, near: f32_0, far: f32_0, scale: f32_0);
    #[no_mangle]
    static mut D_04002040: [Vec3s; 0];
    #[no_mangle]
    static mut D_040020D0: [Vec3s; 0];
    #[no_mangle]
    static mut D_04002160: [Vec3s; 0];
    #[no_mangle]
    static mut D_040021F0: [Vec3s; 0];
    #[no_mangle]
    static mut D_04002280: [Vec3s; 0];
    #[no_mangle]
    static mut gPlayerAnim_003238: LinkAnimationHeader;
    #[no_mangle]
    static mut gLinkChildLinkDekuStickDL: [Gfx; 0];
    #[no_mangle]
    static mut gLinkChildLeftHandNearDL: [Gfx; 0];
    #[no_mangle]
    static mut gLinkChildLeftFistNearDL: [Gfx; 0];
    #[no_mangle]
    static mut gLinkChildLeftFistAndKokiriSwordNearDL: [Gfx; 0];
    #[no_mangle]
    static mut gLinkChildRightHandNearDL: [Gfx; 0];
    #[no_mangle]
    static mut gLinkChildRightHandClosedNearDL: [Gfx; 0];
    #[no_mangle]
    static mut gLinkChildRightFistAndDekuShieldNearDL: [Gfx; 0];
    #[no_mangle]
    static mut gLinkChildLeftFistAndBoomerangNearDL: [Gfx; 0];
    #[no_mangle]
    static mut gLinkChildHylianShieldSwordAndSheathNearDL: [Gfx; 0];
    #[no_mangle]
    static mut gLinkChildHylianShieldAndSheathNearDL: [Gfx; 0];
    #[no_mangle]
    static mut gLinkChildDekuShieldSwordAndSheathNearDL: [Gfx; 0];
    #[no_mangle]
    static mut gLinkChildDekuShieldAndSheathNearDL: [Gfx; 0];
    #[no_mangle]
    static mut gLinkChildSwordAndSheathNearDL: [Gfx; 0];
    #[no_mangle]
    static mut gLinkChildSheathNearDL: [Gfx; 0];
    #[no_mangle]
    static mut gLinkChildLeftHandHoldingMasterSwordDL: [Gfx; 0];
    #[no_mangle]
    static mut gLinkChildRightHandAndOOTNearDL: [Gfx; 0];
    #[no_mangle]
    static mut gLinkChildRightHandHoldingFairyOcarinaNearDL: [Gfx; 0];
    #[no_mangle]
    static mut gLinkChildRightHandHoldingFairySlingshotNearDL: [Gfx; 0];
    #[no_mangle]
    static mut gLinkChildLeftHandUpNearDL: [Gfx; 0];
    #[no_mangle]
    static mut gLinkChildGoronBraceletDL: [Gfx; 0];
    #[no_mangle]
    static mut gLinkChildLeftHandFarDL: [Gfx; 0];
    #[no_mangle]
    static mut gLinkChildLeftFistFarDL: [Gfx; 0];
    #[no_mangle]
    static mut gLinkChildRightHandFarDL: [Gfx; 0];
    #[no_mangle]
    static mut gLinkChildRightHandClosedFarDL: [Gfx; 0];
    #[no_mangle]
    static mut gLinkChildRightFistAndDekuShieldFarDL: [Gfx; 0];
    #[no_mangle]
    static mut gLinkChildLeftFistAndBoomerangFarDL: [Gfx; 0];
    #[no_mangle]
    static mut gLinkChildHylianShieldSwordAndSheathFarDL: [Gfx; 0];
    #[no_mangle]
    static mut gLinkChildHylianShieldAndSheathFarDL: [Gfx; 0];
    #[no_mangle]
    static mut gLinkChildDekuShieldSwordAndSheathFarDL: [Gfx; 0];
    #[no_mangle]
    static mut gLinkChildDekuShieldAndSheathFarDL: [Gfx; 0];
    #[no_mangle]
    static mut gLinkChildSwordAndSheathFarDL: [Gfx; 0];
    #[no_mangle]
    static mut gLinkChildSheathFarDL: [Gfx; 0];
    #[no_mangle]
    static mut gLinkChildLeftFistAndKokiriSwordFarDL: [Gfx; 0];
    #[no_mangle]
    static mut gLinkChildRightHandHoldingOOTFarDL: [Gfx; 0];
    #[no_mangle]
    static mut gLinkChildRightHandHoldingFairyOcarinaFarDL: [Gfx; 0];
    #[no_mangle]
    static mut gLinkChildRightHandHoldingFairySlingshotFarDL: [Gfx; 0];
    #[no_mangle]
    static mut gLinkChildRightArmStretchedSlingshotDL: [Gfx; 0];
    #[no_mangle]
    static mut gLinkChildBottleDL: [Gfx; 0];
    #[no_mangle]
    static mut gLinkChildWaistFarDL: [Gfx; 0];
    #[no_mangle]
    static mut gLinkChildWaistNearDL: [Gfx; 0];
    #[no_mangle]
    static mut gLinkChildLeftShoulderNearDL: [Gfx; 0];
    #[no_mangle]
    static mut gLinkChildDekuShieldWithMatrixDL: [Gfx; 0];
    #[no_mangle]
    static mut gLinkChildSkel: FlexSkeletonHeader;
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
pub struct Lights1 {
    pub a: Ambient,
    pub l: [Light; 1],
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
pub const PLAYER_SHIELD_MAX: C2RustUnnamed_16 = 4;
pub const PLAYER_SHIELD_MIRROR: C2RustUnnamed_16 = 3;
pub const PLAYER_SHIELD_HYLIAN: C2RustUnnamed_16 = 2;
pub const PLAYER_SHIELD_DEKU: C2RustUnnamed_16 = 1;
pub const PLAYER_SHIELD_NONE: C2RustUnnamed_16 = 0;
pub type C2RustUnnamed_17 = libc::c_uint;
pub const PLAYER_TUNIC_MAX: C2RustUnnamed_17 = 3;
pub const PLAYER_TUNIC_ZORA: C2RustUnnamed_17 = 2;
pub const PLAYER_TUNIC_GORON: C2RustUnnamed_17 = 1;
pub const PLAYER_TUNIC_KOKIRI: C2RustUnnamed_17 = 0;
pub type C2RustUnnamed_18 = libc::c_uint;
pub const PLAYER_BOOTS_MAX: C2RustUnnamed_18 = 6;
pub const PLAYER_BOOTS_NORMAL_CHILD: C2RustUnnamed_18 = 5;
pub const PLAYER_BOOTS_IRON_UNDERWATER: C2RustUnnamed_18 = 4;
pub const PLAYER_BOOTS_INDOOR: C2RustUnnamed_18 = 3;
pub const PLAYER_BOOTS_HOVER: C2RustUnnamed_18 = 2;
pub const PLAYER_BOOTS_IRON: C2RustUnnamed_18 = 1;
pub const PLAYER_BOOTS_NORMAL: C2RustUnnamed_18 = 0;
pub type C2RustUnnamed_19 = libc::c_uint;
pub const PLAYER_STR_MAX: C2RustUnnamed_19 = 4;
pub const PLAYER_STR_GOLD_G: C2RustUnnamed_19 = 3;
pub const PLAYER_STR_SILVER_G: C2RustUnnamed_19 = 2;
pub const PLAYER_STR_BRACELET: C2RustUnnamed_19 = 1;
pub const PLAYER_STR_NONE: C2RustUnnamed_19 = 0;
pub type C2RustUnnamed_20 = libc::c_uint;
pub const PLAYER_MASK_MAX: C2RustUnnamed_20 = 9;
pub const PLAYER_MASK_TRUTH: C2RustUnnamed_20 = 8;
pub const PLAYER_MASK_GERUDO: C2RustUnnamed_20 = 7;
pub const PLAYER_MASK_ZORA: C2RustUnnamed_20 = 6;
pub const PLAYER_MASK_GORON: C2RustUnnamed_20 = 5;
pub const PLAYER_MASK_BUNNY: C2RustUnnamed_20 = 4;
pub const PLAYER_MASK_SPOOKY: C2RustUnnamed_20 = 3;
pub const PLAYER_MASK_SKULL: C2RustUnnamed_20 = 2;
pub const PLAYER_MASK_KEATON: C2RustUnnamed_20 = 1;
pub const PLAYER_MASK_NONE: C2RustUnnamed_20 = 0;
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
pub const PLAYER_LIMB_MAX: C2RustUnnamed_22 = 22;
pub const PLAYER_LIMB_TORSO: C2RustUnnamed_22 = 21;
pub const PLAYER_LIMB_SHEATH: C2RustUnnamed_22 = 20;
pub const PLAYER_LIMB_R_HAND: C2RustUnnamed_22 = 19;
pub const PLAYER_LIMB_R_FOREARM: C2RustUnnamed_22 = 18;
pub const PLAYER_LIMB_R_SHOULDER: C2RustUnnamed_22 = 17;
pub const PLAYER_LIMB_L_HAND: C2RustUnnamed_22 = 16;
pub const PLAYER_LIMB_L_FOREARM: C2RustUnnamed_22 = 15;
pub const PLAYER_LIMB_L_SHOULDER: C2RustUnnamed_22 = 14;
pub const PLAYER_LIMB_COLLAR: C2RustUnnamed_22 = 13;
pub const PLAYER_LIMB_HAT: C2RustUnnamed_22 = 12;
pub const PLAYER_LIMB_HEAD: C2RustUnnamed_22 = 11;
pub const PLAYER_LIMB_UPPER: C2RustUnnamed_22 = 10;
pub const PLAYER_LIMB_L_FOOT: C2RustUnnamed_22 = 9;
pub const PLAYER_LIMB_L_SHIN: C2RustUnnamed_22 = 8;
pub const PLAYER_LIMB_L_THIGH: C2RustUnnamed_22 = 7;
pub const PLAYER_LIMB_R_FOOT: C2RustUnnamed_22 = 6;
pub const PLAYER_LIMB_R_SHIN: C2RustUnnamed_22 = 5;
pub const PLAYER_LIMB_R_THIGH: C2RustUnnamed_22 = 4;
pub const PLAYER_LIMB_LOWER: C2RustUnnamed_22 = 3;
pub const PLAYER_LIMB_WAIST: C2RustUnnamed_22 = 2;
pub const PLAYER_LIMB_ROOT: C2RustUnnamed_22 = 1;
pub const PLAYER_LIMB_NONE: C2RustUnnamed_22 = 0;
pub type C2RustUnnamed_23 = libc::c_uint;
pub const OBJECT_ID_MAX: C2RustUnnamed_23 = 402;
pub const OBJECT_ZL4: C2RustUnnamed_23 = 401;
pub const OBJECT_TIMEBLOCK: C2RustUnnamed_23 = 400;
pub const OBJECT_OUKE_HAKA: C2RustUnnamed_23 = 399;
pub const OBJECT_DOOR_KILLER: C2RustUnnamed_23 = 398;
pub const OBJECT_GI_SWORD_1: C2RustUnnamed_23 = 397;
pub const OBJECT_COB: C2RustUnnamed_23 = 396;
pub const OBJECT_COW: C2RustUnnamed_23 = 395;
pub const OBJECT_BWALL: C2RustUnnamed_23 = 394;
pub const OBJECT_PS: C2RustUnnamed_23 = 393;
pub const OBJECT_GS: C2RustUnnamed_23 = 392;
pub const OBJECT_HAKA_DOOR: C2RustUnnamed_23 = 391;
pub const OBJECT_GEFF: C2RustUnnamed_23 = 390;
pub const OBJECT_GJ: C2RustUnnamed_23 = 389;
pub const OBJECT_SKB: C2RustUnnamed_23 = 388;
pub const OBJECT_WF: C2RustUnnamed_23 = 387;
pub const OBJECT_MU: C2RustUnnamed_23 = 386;
pub const OBJECT_SPOT01_MATOYAB: C2RustUnnamed_23 = 385;
pub const OBJECT_SPOT01_MATOYA: C2RustUnnamed_23 = 384;
pub const OBJECT_GI_RUPY: C2RustUnnamed_23 = 383;
pub const OBJECT_GANON_ANIME3: C2RustUnnamed_23 = 382;
pub const OBJECT_GANON_ANIME2: C2RustUnnamed_23 = 381;
pub const OBJECT_GANON_ANIME1: C2RustUnnamed_23 = 380;
pub const OBJECT_GI_DEKUPOUCH: C2RustUnnamed_23 = 379;
pub const OBJECT_EFC_DOUGHNUT: C2RustUnnamed_23 = 378;
pub const OBJECT_DEMO_KEKKAI: C2RustUnnamed_23 = 377;
pub const OBJECT_BOWL: C2RustUnnamed_23 = 376;
pub const OBJECT_GI_SOUL: C2RustUnnamed_23 = 375;
pub const OBJECT_GI_GHOST: C2RustUnnamed_23 = 374;
pub const OBJECT_GI_BUTTERFLY: C2RustUnnamed_23 = 373;
pub const OBJECT_GI_INSECT: C2RustUnnamed_23 = 372;
pub const OBJECT_GI_FIRE: C2RustUnnamed_23 = 371;
pub const OBJECT_DNK: C2RustUnnamed_23 = 370;
pub const OBJECT_DNS: C2RustUnnamed_23 = 369;
pub const OBJECT_KIBAKO2: C2RustUnnamed_23 = 368;
pub const OBJECT_SPOT11_OBJ: C2RustUnnamed_23 = 367;
pub const OBJECT_UNSET_16E: C2RustUnnamed_23 = 366;
pub const OBJECT_JYA_DOOR: C2RustUnnamed_23 = 365;
pub const OBJECT_JYA_IRON: C2RustUnnamed_23 = 364;
pub const OBJECT_DOG: C2RustUnnamed_23 = 363;
pub const OBJECT_GR: C2RustUnnamed_23 = 362;
pub const OBJECT_GELDB: C2RustUnnamed_23 = 361;
pub const OBJECT_SHOPNUTS: C2RustUnnamed_23 = 360;
pub const OBJECT_GLA: C2RustUnnamed_23 = 359;
pub const OBJECT_SPOT00_BREAK: C2RustUnnamed_23 = 358;
pub const OBJECT_RS: C2RustUnnamed_23 = 357;
pub const OBJECT_HINTNUTS: C2RustUnnamed_23 = 356;
pub const OBJECT_BOMBIWA: C2RustUnnamed_23 = 355;
pub const OBJECT_SPOT12_OBJ: C2RustUnnamed_23 = 354;
pub const OBJECT_SPOT05_OBJECTS: C2RustUnnamed_23 = 353;
pub const OBJECT_BG: C2RustUnnamed_23 = 352;
pub const OBJECT_BIGOKUTA: C2RustUnnamed_23 = 351;
pub const OBJECT_SSH: C2RustUnnamed_23 = 350;
pub const OBJECT_GI_GODDESS: C2RustUnnamed_23 = 349;
pub const OBJECT_GI_SUTARU: C2RustUnnamed_23 = 348;
pub const OBJECT_FISH: C2RustUnnamed_23 = 347;
pub const OBJECT_EC: C2RustUnnamed_23 = 346;
pub const OBJECT_DS2: C2RustUnnamed_23 = 345;
pub const OBJECT_GI_M_ARROW: C2RustUnnamed_23 = 344;
pub const OBJECT_GI_HOVERBOOTS: C2RustUnnamed_23 = 343;
pub const OBJECT_ZG: C2RustUnnamed_23 = 342;
pub const OBJECT_TS: C2RustUnnamed_23 = 341;
pub const OBJECT_KA: C2RustUnnamed_23 = 340;
pub const OBJECT_GANON2: C2RustUnnamed_23 = 339;
pub const OBJECT_GI_GERUDOMASK: C2RustUnnamed_23 = 338;
pub const OBJECT_GI_ZORAMASK: C2RustUnnamed_23 = 337;
pub const OBJECT_GI_GOLONMASK: C2RustUnnamed_23 = 336;
pub const OBJECT_ZL2_ANIME2: C2RustUnnamed_23 = 335;
pub const OBJECT_ZL2_ANIME1: C2RustUnnamed_23 = 334;
pub const OBJECT_EFC_ERUPC: C2RustUnnamed_23 = 333;
pub const OBJECT_GT: C2RustUnnamed_23 = 332;
pub const OBJECT_DOOR_GERUDO: C2RustUnnamed_23 = 331;
pub const OBJECT_MAG: C2RustUnnamed_23 = 330;
pub const OBJECT_GI_FROG: C2RustUnnamed_23 = 329;
pub const OBJECT_GI_SOLDOUT: C2RustUnnamed_23 = 328;
pub const OBJECT_GI_BRACELET: C2RustUnnamed_23 = 327;
pub const OBJECT_GI_PRESCRIPTION: C2RustUnnamed_23 = 326;
pub const OBJECT_CS: C2RustUnnamed_23 = 325;
pub const OBJECT_JS: C2RustUnnamed_23 = 324;
pub const OBJECT_GI_BROKENSWORD: C2RustUnnamed_23 = 323;
pub const OBJECT_GI_TICKETSTONE: C2RustUnnamed_23 = 322;
pub const OBJECT_GI_MUSHROOM: C2RustUnnamed_23 = 321;
pub const OBJECT_GI_POWDER: C2RustUnnamed_23 = 320;
pub const OBJECT_GI_EYE_LOTION: C2RustUnnamed_23 = 319;
pub const OBJECT_OS: C2RustUnnamed_23 = 318;
pub const OBJECT_FA: C2RustUnnamed_23 = 317;
pub const OBJECT_MM: C2RustUnnamed_23 = 316;
pub const OBJECT_STREAM: C2RustUnnamed_23 = 315;
pub const OBJECT_SIOFUKI: C2RustUnnamed_23 = 314;
pub const OBJECT_GANON_OBJECTS: C2RustUnnamed_23 = 313;
pub const OBJECT_GI_TRUTH_MASK: C2RustUnnamed_23 = 312;
pub const OBJECT_GI_RABIT_MASK: C2RustUnnamed_23 = 311;
pub const OBJECT_GI_SKJ_MASK: C2RustUnnamed_23 = 310;
pub const OBJECT_GI_REDEAD_MASK: C2RustUnnamed_23 = 309;
pub const OBJECT_GI_KI_TAN_MASK: C2RustUnnamed_23 = 308;
pub const OBJECT_FU: C2RustUnnamed_23 = 307;
pub const OBJECT_MK: C2RustUnnamed_23 = 306;
pub const OBJECT_OWL: C2RustUnnamed_23 = 305;
pub const OBJECT_GJYO_OBJECTS: C2RustUnnamed_23 = 304;
pub const OBJECT_KANBAN: C2RustUnnamed_23 = 303;
pub const OBJECT_GI_COIN: C2RustUnnamed_23 = 302;
pub const OBJECT_GI_GLOVES: C2RustUnnamed_23 = 301;
pub const OBJECT_TSUBO: C2RustUnnamed_23 = 300;
pub const OBJECT_KUSA: C2RustUnnamed_23 = 299;
pub const OBJECT_LIGHTSWITCH: C2RustUnnamed_23 = 298;
pub const OBJECT_INGATE: C2RustUnnamed_23 = 297;
pub const OBJECT_HS: C2RustUnnamed_23 = 296;
pub const OBJECT_MS: C2RustUnnamed_23 = 295;
pub const OBJECT_GM: C2RustUnnamed_23 = 294;
pub const OBJECT_BLKOBJ: C2RustUnnamed_23 = 293;
pub const OBJECT_NWC: C2RustUnnamed_23 = 292;
pub const OBJECT_UNSET_123: C2RustUnnamed_23 = 291;
pub const OBJECT_DAIKU: C2RustUnnamed_23 = 290;
pub const OBJECT_TORYO: C2RustUnnamed_23 = 289;
pub const OBJECT_UNSET_120: C2RustUnnamed_23 = 288;
pub const OBJECT_GOROIWA: C2RustUnnamed_23 = 287;
pub const OBJECT_MAMENOKI: C2RustUnnamed_23 = 286;
pub const OBJECT_D_LIFT: C2RustUnnamed_23 = 285;
pub const OBJECT_D_HSBLOCK: C2RustUnnamed_23 = 284;
pub const OBJECT_D_ELEVATOR: C2RustUnnamed_23 = 283;
pub const OBJECT_GND_MAGIC: C2RustUnnamed_23 = 282;
pub const OBJECT_GI_SEED: C2RustUnnamed_23 = 281;
pub const OBJECT_GI_BOOTS_2: C2RustUnnamed_23 = 280;
pub const OBJECT_YABUSAME_POINT: C2RustUnnamed_23 = 279;
pub const OBJECT_GE1: C2RustUnnamed_23 = 278;
pub const OBJECT_BOB: C2RustUnnamed_23 = 277;
pub const OBJECT_FZ: C2RustUnnamed_23 = 276;
pub const OBJECT_SPOT07_OBJECT: C2RustUnnamed_23 = 275;
pub const OBJECT_SPOT03_OBJECT: C2RustUnnamed_23 = 274;
pub const OBJECT_BOJ: C2RustUnnamed_23 = 273;
pub const OBJECT_ANE: C2RustUnnamed_23 = 272;
pub const OBJECT_DS: C2RustUnnamed_23 = 271;
pub const OBJECT_GI_OCARINA_0: C2RustUnnamed_23 = 270;
pub const OBJECT_BBA: C2RustUnnamed_23 = 269;
pub const OBJECT_BJI: C2RustUnnamed_23 = 268;
pub const OBJECT_GI_BOTTLE_LETTER: C2RustUnnamed_23 = 267;
pub const OBJECT_SKJ: C2RustUnnamed_23 = 266;
pub const OBJECT_GI_NIWATORI: C2RustUnnamed_23 = 265;
pub const OBJECT_CNE: C2RustUnnamed_23 = 264;
pub const OBJECT_AHG: C2RustUnnamed_23 = 263;
pub const OBJECT_IK: C2RustUnnamed_23 = 262;
pub const OBJECT_AOB: C2RustUnnamed_23 = 261;
pub const OBJECT_MASTERZOORA: C2RustUnnamed_23 = 260;
pub const OBJECT_MASTERGOLON: C2RustUnnamed_23 = 259;
pub const OBJECT_MASTERKOKIRIHEAD: C2RustUnnamed_23 = 258;
pub const OBJECT_MASTERKOKIRI: C2RustUnnamed_23 = 257;
pub const OBJECT_UMAJUMP: C2RustUnnamed_23 = 256;
pub const OBJECT_KZ: C2RustUnnamed_23 = 255;
pub const OBJECT_ZO: C2RustUnnamed_23 = 254;
pub const OBJECT_KW1: C2RustUnnamed_23 = 253;
pub const OBJECT_KM1: C2RustUnnamed_23 = 252;
pub const OBJECT_MD: C2RustUnnamed_23 = 251;
pub const OBJECT_MD_UNUSED: C2RustUnnamed_23 = 250;
pub const OBJECT_SPOT01_OBJECTS: C2RustUnnamed_23 = 249;
pub const OBJECT_GI_LONGSWORD: C2RustUnnamed_23 = 248;
pub const OBJECT_GI_GRASS: C2RustUnnamed_23 = 247;
pub const OBJECT_GI_HAMMER: C2RustUnnamed_23 = 246;
pub const OBJECT_GI_SAW: C2RustUnnamed_23 = 245;
pub const OBJECT_GI_FISH: C2RustUnnamed_23 = 244;
pub const OBJECT_GI_BEAN: C2RustUnnamed_23 = 243;
pub const OBJECT_GI_CLOTHES: C2RustUnnamed_23 = 242;
pub const OBJECT_JYA_OBJ: C2RustUnnamed_23 = 241;
pub const OBJECT_SPOT15_OBJ: C2RustUnnamed_23 = 240;
pub const OBJECT_GI_LETTER: C2RustUnnamed_23 = 239;
pub const OBJECT_GI_SHIELD_3: C2RustUnnamed_23 = 238;
pub const OBJECT_DEMO_6K: C2RustUnnamed_23 = 237;
pub const OBJECT_ANI: C2RustUnnamed_23 = 236;
pub const OBJECT_GI_LIQUID: C2RustUnnamed_23 = 235;
pub const OBJECT_GI_GLASSES: C2RustUnnamed_23 = 234;
pub const OBJECT_GI_BOW: C2RustUnnamed_23 = 233;
pub const OBJECT_GI_BOOMERANG: C2RustUnnamed_23 = 232;
pub const OBJECT_GI_PACHINKO: C2RustUnnamed_23 = 231;
pub const OBJECT_FR: C2RustUnnamed_23 = 230;
pub const OBJECT_NY: C2RustUnnamed_23 = 229;
pub const OBJECT_UNSET_E4: C2RustUnnamed_23 = 228;
pub const OBJECT_NY_UNUSED: C2RustUnnamed_23 = 227;
pub const OBJECT_SST: C2RustUnnamed_23 = 226;
pub const OBJECT_GANON: C2RustUnnamed_23 = 225;
pub const OBJECT_MA1: C2RustUnnamed_23 = 224;
pub const OBJECT_GI_MILK: C2RustUnnamed_23 = 223;
pub const OBJECT_GI_OCARINA: C2RustUnnamed_23 = 222;
pub const OBJECT_GI_HOOKSHOT: C2RustUnnamed_23 = 221;
pub const OBJECT_GI_SHIELD_2: C2RustUnnamed_23 = 220;
pub const OBJECT_GI_SCALE: C2RustUnnamed_23 = 219;
pub const OBJECT_GI_EGG: C2RustUnnamed_23 = 218;
pub const OBJECT_GI_BOMB_2: C2RustUnnamed_23 = 217;
pub const OBJECT_GI_ARROW: C2RustUnnamed_23 = 216;
pub const OBJECT_GI_GERUDO: C2RustUnnamed_23 = 215;
pub const OBJECT_ANUBICE: C2RustUnnamed_23 = 214;
pub const OBJECT_BXA: C2RustUnnamed_23 = 213;
pub const OBJECT_RR: C2RustUnnamed_23 = 212;
pub const OBJECT_TW: C2RustUnnamed_23 = 211;
pub const OBJECT_HNI: C2RustUnnamed_23 = 210;
pub const OBJECT_GI_PURSE: C2RustUnnamed_23 = 209;
pub const OBJECT_MA2: C2RustUnnamed_23 = 208;
pub const OBJECT_OF1S: C2RustUnnamed_23 = 207;
pub const OBJECT_GI_BOMB_1: C2RustUnnamed_23 = 206;
pub const OBJECT_GI_MAGICPOT: C2RustUnnamed_23 = 205;
pub const OBJECT_DEKUJR: C2RustUnnamed_23 = 204;
pub const OBJECT_GI_SHIELD_1: C2RustUnnamed_23 = 203;
pub const OBJECT_RU2: C2RustUnnamed_23 = 202;
pub const OBJECT_OF1D_MAP: C2RustUnnamed_23 = 201;
pub const OBJECT_GI_MAP: C2RustUnnamed_23 = 200;
pub const OBJECT_GI_STICK: C2RustUnnamed_23 = 199;
pub const OBJECT_GI_BOTTLE: C2RustUnnamed_23 = 198;
pub const OBJECT_OS_ANIME: C2RustUnnamed_23 = 197;
pub const OBJECT_OE4S: C2RustUnnamed_23 = 196;
pub const OBJECT_OE1S: C2RustUnnamed_23 = 195;
pub const OBJECT_SPOT16_OBJ: C2RustUnnamed_23 = 194;
pub const OBJECT_TR: C2RustUnnamed_23 = 193;
pub const OBJECT_IN: C2RustUnnamed_23 = 192;
pub const OBJECT_GI_BOMBPOUCH: C2RustUnnamed_23 = 191;
pub const OBJECT_GI_ARROWCASE: C2RustUnnamed_23 = 190;
pub const OBJECT_GI_HEARTS: C2RustUnnamed_23 = 189;
pub const OBJECT_SA: C2RustUnnamed_23 = 188;
pub const OBJECT_GI_NUTS: C2RustUnnamed_23 = 187;
pub const OBJECT_GI_MEDAL: C2RustUnnamed_23 = 186;
pub const OBJECT_GI_BOSSKEY: C2RustUnnamed_23 = 185;
pub const OBJECT_GI_COMPASS: C2RustUnnamed_23 = 184;
pub const OBJECT_GI_HEART: C2RustUnnamed_23 = 183;
pub const OBJECT_GI_MELODY: C2RustUnnamed_23 = 182;
pub const OBJECT_SB: C2RustUnnamed_23 = 181;
pub const OBJECT_MO: C2RustUnnamed_23 = 180;
pub const OBJECT_NB: C2RustUnnamed_23 = 179;
pub const OBJECT_SHOP_DUNGEN: C2RustUnnamed_23 = 178;
pub const OBJECT_SPOT17_OBJ: C2RustUnnamed_23 = 177;
pub const OBJECT_BDOOR: C2RustUnnamed_23 = 176;
pub const OBJECT_SPOT18_OBJ: C2RustUnnamed_23 = 175;
pub const OBJECT_SPOT09_OBJ: C2RustUnnamed_23 = 174;
pub const OBJECT_GI_JEWEL: C2RustUnnamed_23 = 173;
pub const OBJECT_BROB: C2RustUnnamed_23 = 172;
pub const OBJECT_MIR_RAY: C2RustUnnamed_23 = 171;
pub const OBJECT_GI_KEY: C2RustUnnamed_23 = 170;
pub const OBJECT_DEMO_TRE_LGT: C2RustUnnamed_23 = 169;
pub const OBJECT_EFC_TW: C2RustUnnamed_23 = 168;
pub const OBJECT_RL: C2RustUnnamed_23 = 167;
pub const OBJECT_DH: C2RustUnnamed_23 = 166;
pub const OBJECT_FD2: C2RustUnnamed_23 = 165;
pub const OBJECT_SYOKUDAI: C2RustUnnamed_23 = 164;
pub const OBJECT_RU1: C2RustUnnamed_23 = 163;
pub const OBJECT_HAKA: C2RustUnnamed_23 = 162;
pub const OBJECT_SPOT02_OBJECTS: C2RustUnnamed_23 = 161;
pub const OBJECT_HORSE_LINK_CHILD: C2RustUnnamed_23 = 160;
pub const OBJECT_MEDAL: C2RustUnnamed_23 = 159;
pub const OBJECT_FW: C2RustUnnamed_23 = 158;
pub const OBJECT_DU: C2RustUnnamed_23 = 157;
pub const OBJECT_FD: C2RustUnnamed_23 = 156;
pub const OBJECT_GNDD: C2RustUnnamed_23 = 155;
pub const OBJECT_HEAVY_OBJECT: C2RustUnnamed_23 = 154;
pub const OBJECT_PO_SISTERS: C2RustUnnamed_23 = 153;
pub const OBJECT_RD: C2RustUnnamed_23 = 152;
pub const OBJECT_SD: C2RustUnnamed_23 = 151;
pub const OBJECT_BDAN_OBJECTS: C2RustUnnamed_23 = 150;
pub const OBJECT_TRIFORCE_SPOT: C2RustUnnamed_23 = 149;
pub const OBJECT_LIGHT_RING: C2RustUnnamed_23 = 148;
pub const OBJECT_GOD_LGT: C2RustUnnamed_23 = 147;
pub const OBJECT_EFC_STAR_FIELD: C2RustUnnamed_23 = 146;
pub const OBJECT_EFC_LGT_SHOWER: C2RustUnnamed_23 = 145;
pub const OBJECT_EFC_FLASH: C2RustUnnamed_23 = 144;
pub const OBJECT_EFC_FIRE_BALL: C2RustUnnamed_23 = 143;
pub const OBJECT_EFC_CRYSTAL_LIGHT: C2RustUnnamed_23 = 142;
pub const OBJECT_HAKACH_OBJECTS: C2RustUnnamed_23 = 141;
pub const OBJECT_BV: C2RustUnnamed_23 = 140;
pub const OBJECT_VM: C2RustUnnamed_23 = 139;
pub const OBJECT_XC: C2RustUnnamed_23 = 138;
pub const OBJECT_TK: C2RustUnnamed_23 = 137;
pub const OBJECT_TA: C2RustUnnamed_23 = 136;
pub const OBJECT_IM: C2RustUnnamed_23 = 135;
pub const OBJECT_VASE: C2RustUnnamed_23 = 134;
pub const OBJECT_TRAP: C2RustUnnamed_23 = 133;
pub const OBJECT_UNSET_84: C2RustUnnamed_23 = 132;
pub const OBJECT_UNSET_83: C2RustUnnamed_23 = 131;
pub const OBJECT_PU_BOX: C2RustUnnamed_23 = 130;
pub const OBJECT_LIGHTBOX: C2RustUnnamed_23 = 129;
pub const OBJECT_UNSET_80: C2RustUnnamed_23 = 128;
pub const OBJECT_UNSET_7F: C2RustUnnamed_23 = 127;
pub const OBJECT_UNSET_7E: C2RustUnnamed_23 = 126;
pub const OBJECT_UNSET_7D: C2RustUnnamed_23 = 125;
pub const OBJECT_WOOD02: C2RustUnnamed_23 = 124;
pub const OBJECT_UNSET_7B: C2RustUnnamed_23 = 123;
pub const OBJECT_UNSET_7A: C2RustUnnamed_23 = 122;
pub const OBJECT_UNSET_79: C2RustUnnamed_23 = 121;
pub const OBJECT_UNSET_78: C2RustUnnamed_23 = 120;
pub const OBJECT_BIRD: C2RustUnnamed_23 = 119;
pub const OBJECT_HATA: C2RustUnnamed_23 = 118;
pub const OBJECT_WARP2: C2RustUnnamed_23 = 117;
pub const OBJECT_SPOT08_OBJ: C2RustUnnamed_23 = 116;
pub const OBJECT_MORI_TEX: C2RustUnnamed_23 = 115;
pub const OBJECT_MORI_OBJECTS: C2RustUnnamed_23 = 114;
pub const OBJECT_MORI_HINERI2A: C2RustUnnamed_23 = 113;
pub const OBJECT_MORI_HINERI2: C2RustUnnamed_23 = 112;
pub const OBJECT_MORI_HINERI1A: C2RustUnnamed_23 = 111;
pub const OBJECT_PO_COMPOSER: C2RustUnnamed_23 = 110;
pub const OBJECT_PO_FIELD: C2RustUnnamed_23 = 109;
pub const OBJECT_RELAY_OBJECTS: C2RustUnnamed_23 = 108;
pub const OBJECT_ICE_OBJECTS: C2RustUnnamed_23 = 107;
pub const OBJECT_SPOT06_OBJECTS: C2RustUnnamed_23 = 106;
pub const OBJECT_HAKA_OBJECTS: C2RustUnnamed_23 = 105;
pub const OBJECT_MJIN_OKA: C2RustUnnamed_23 = 104;
pub const OBJECT_MJIN_WIND: C2RustUnnamed_23 = 103;
pub const OBJECT_MJIN_SOUL: C2RustUnnamed_23 = 102;
pub const OBJECT_MJIN_ICE: C2RustUnnamed_23 = 101;
pub const OBJECT_MJIN_FLAME: C2RustUnnamed_23 = 100;
pub const OBJECT_MJIN_DARK: C2RustUnnamed_23 = 99;
pub const OBJECT_MJIN_FLASH: C2RustUnnamed_23 = 98;
pub const OBJECT_MJIN: C2RustUnnamed_23 = 97;
pub const OBJECT_ZL2: C2RustUnnamed_23 = 96;
pub const OBJECT_YUKABYUN: C2RustUnnamed_23 = 95;
pub const OBJECT_TOKI_OBJECTS: C2RustUnnamed_23 = 94;
pub const OBJECT_BB: C2RustUnnamed_23 = 93;
pub const OBJECT_MORI_HINERI1: C2RustUnnamed_23 = 92;
pub const OBJECT_OSSAN: C2RustUnnamed_23 = 91;
pub const OBJECT_FHG: C2RustUnnamed_23 = 90;
pub const OBJECT_MIZU_OBJECTS: C2RustUnnamed_23 = 89;
pub const OBJECT_OA11: C2RustUnnamed_23 = 88;
pub const OBJECT_OA10: C2RustUnnamed_23 = 87;
pub const OBJECT_VALI: C2RustUnnamed_23 = 86;
pub const OBJECT_OE12: C2RustUnnamed_23 = 85;
pub const OBJECT_OE11: C2RustUnnamed_23 = 84;
pub const OBJECT_OE10: C2RustUnnamed_23 = 83;
pub const OBJECT_OE9: C2RustUnnamed_23 = 82;
pub const OBJECT_OE8: C2RustUnnamed_23 = 81;
pub const OBJECT_OE7: C2RustUnnamed_23 = 80;
pub const OBJECT_OE6: C2RustUnnamed_23 = 79;
pub const OBJECT_OE5: C2RustUnnamed_23 = 78;
pub const OBJECT_MENKURI_OBJECTS: C2RustUnnamed_23 = 77;
pub const OBJECT_OE4: C2RustUnnamed_23 = 76;
pub const OBJECT_OE3: C2RustUnnamed_23 = 75;
pub const OBJECT_DEKUNUTS: C2RustUnnamed_23 = 74;
pub const OBJECT_B_HEART: C2RustUnnamed_23 = 73;
pub const OBJECT_WARP1: C2RustUnnamed_23 = 72;
pub const OBJECT_OPENING_DEMO1: C2RustUnnamed_23 = 71;
pub const OBJECT_HORSE_ZELDA: C2RustUnnamed_23 = 70;
pub const OBJECT_OB4: C2RustUnnamed_23 = 69;
pub const OBJECT_OB3: C2RustUnnamed_23 = 68;
pub const OBJECT_OB2: C2RustUnnamed_23 = 67;
pub const OBJECT_OA9: C2RustUnnamed_23 = 66;
pub const OBJECT_OA8: C2RustUnnamed_23 = 65;
pub const OBJECT_JJ: C2RustUnnamed_23 = 64;
pub const OBJECT_OA7: C2RustUnnamed_23 = 63;
pub const OBJECT_OA6: C2RustUnnamed_23 = 62;
pub const OBJECT_OA5: C2RustUnnamed_23 = 61;
pub const OBJECT_OA4: C2RustUnnamed_23 = 60;
pub const OBJECT_OA3: C2RustUnnamed_23 = 59;
pub const OBJECT_UNSET_3A: C2RustUnnamed_23 = 58;
pub const OBJECT_DEKUBABA: C2RustUnnamed_23 = 57;
pub const OBJECT_AM: C2RustUnnamed_23 = 56;
pub const OBJECT_GND: C2RustUnnamed_23 = 55;
pub const OBJECT_YDAN_OBJECTS: C2RustUnnamed_23 = 54;
pub const OBJECT_OE2: C2RustUnnamed_23 = 53;
pub const OBJECT_OE_ANIME: C2RustUnnamed_23 = 52;
pub const OBJECT_OE1: C2RustUnnamed_23 = 51;
pub const OBJECT_SK2: C2RustUnnamed_23 = 50;
pub const OBJECT_BOMBF: C2RustUnnamed_23 = 49;
pub const OBJECT_MB: C2RustUnnamed_23 = 48;
pub const OBJECT_SPOT00_OBJECTS: C2RustUnnamed_23 = 47;
pub const OBJECT_OA2: C2RustUnnamed_23 = 46;
pub const OBJECT_HORSE_GANON: C2RustUnnamed_23 = 45;
pub const OBJECT_HIDAN_OBJECTS: C2RustUnnamed_23 = 44;
pub const OBJECT_DDAN_OBJECTS: C2RustUnnamed_23 = 43;
pub const OBJECT_SPOT04_OBJECTS: C2RustUnnamed_23 = 42;
pub const OBJECT_O_ANIME: C2RustUnnamed_23 = 41;
pub const OBJECT_OB1: C2RustUnnamed_23 = 40;
pub const OBJECT_HORSE_NORMAL: C2RustUnnamed_23 = 39;
pub const OBJECT_EI: C2RustUnnamed_23 = 38;
pub const OBJECT_BW: C2RustUnnamed_23 = 37;
pub const OBJECT_ST: C2RustUnnamed_23 = 36;
pub const OBJECT_OA1: C2RustUnnamed_23 = 35;
pub const OBJECT_TP: C2RustUnnamed_23 = 34;
pub const OBJECT_BL: C2RustUnnamed_23 = 33;
pub const OBJECT_TORCH2: C2RustUnnamed_23 = 32;
pub const OBJECT_DODOJR: C2RustUnnamed_23 = 31;
pub const OBJECT_GOL: C2RustUnnamed_23 = 30;
pub const OBJECT_ZL1: C2RustUnnamed_23 = 29;
pub const OBJECT_GOMA: C2RustUnnamed_23 = 28;
pub const OBJECT_ZF: C2RustUnnamed_23 = 27;
pub const OBJECT_HORSE: C2RustUnnamed_23 = 26;
pub const OBJECT_KINGDODONGO: C2RustUnnamed_23 = 25;
pub const OBJECT_PEEHAT: C2RustUnnamed_23 = 24;
pub const OBJECT_REEBA: C2RustUnnamed_23 = 23;
pub const OBJECT_TITE: C2RustUnnamed_23 = 22;
pub const OBJECT_LINK_CHILD: C2RustUnnamed_23 = 21;
pub const OBJECT_LINK_BOY: C2RustUnnamed_23 = 20;
pub const OBJECT_NIW: C2RustUnnamed_23 = 19;
pub const OBJECT_BUBBLE: C2RustUnnamed_23 = 18;
pub const OBJECT_UNSET_11: C2RustUnnamed_23 = 17;
pub const OBJECT_UNSET_10: C2RustUnnamed_23 = 16;
pub const OBJECT_FIRE: C2RustUnnamed_23 = 15;
pub const OBJECT_BOX: C2RustUnnamed_23 = 14;
pub const OBJECT_FIREFLY: C2RustUnnamed_23 = 13;
pub const OBJECT_DODONGO: C2RustUnnamed_23 = 12;
pub const OBJECT_WALLMASTER: C2RustUnnamed_23 = 11;
pub const OBJECT_DY_OBJ: C2RustUnnamed_23 = 10;
pub const OBJECT_POH: C2RustUnnamed_23 = 9;
pub const OBJECT_CROW: C2RustUnnamed_23 = 8;
pub const OBJECT_OKUTA: C2RustUnnamed_23 = 7;
pub const OBJECT_HUMAN: C2RustUnnamed_23 = 6;
pub const OBJECT_UNSET_5: C2RustUnnamed_23 = 5;
pub const OBJECT_UNSET_4: C2RustUnnamed_23 = 4;
pub const OBJECT_GAMEPLAY_DANGEON_KEEP: C2RustUnnamed_23 = 3;
pub const OBJECT_GAMEPLAY_FIELD_KEEP: C2RustUnnamed_23 = 2;
pub const OBJECT_GAMEPLAY_KEEP: C2RustUnnamed_23 = 1;
pub const OBJECT_INVALID: C2RustUnnamed_23 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EffectBlureElement {
    pub state: s32,
    pub timer: s32,
    pub p1: Vec3s,
    pub p2: Vec3s,
    pub flags: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EffectBlure {
    pub elements: [EffectBlureElement; 16],
    pub calcMode: s32,
    pub mode4Param: f32_0,
    pub flags: u16_0,
    pub addAngleChange: s16,
    pub addAngle: s16,
    pub p1StartColor: Color_RGBA8,
    pub p2StartColor: Color_RGBA8,
    pub p1EndColor: Color_RGBA8,
    pub p2EndColor: Color_RGBA8,
    pub numElements: u8_0,
    pub elemDuration: u8_0,
    pub unkFlag: u8_0,
    pub drawMode: u8_0,
    pub altPrimColor: Color_RGBA8,
    pub altEnvColor: Color_RGBA8,
}
pub type C2RustUnnamed_24 = libc::c_uint;
pub const EQUIP_BOOTS: C2RustUnnamed_24 = 3;
pub const EQUIP_TUNIC: C2RustUnnamed_24 = 2;
pub const EQUIP_SHIELD: C2RustUnnamed_24 = 1;
pub const EQUIP_SWORD: C2RustUnnamed_24 = 0;
pub type C2RustUnnamed_25 = libc::c_uint;
pub const UPG_NUTS: C2RustUnnamed_25 = 7;
pub const UPG_STICKS: C2RustUnnamed_25 = 6;
pub const UPG_BULLET_BAG: C2RustUnnamed_25 = 5;
pub const UPG_WALLET: C2RustUnnamed_25 = 4;
pub const UPG_SCALE: C2RustUnnamed_25 = 3;
pub const UPG_STRENGTH: C2RustUnnamed_25 = 2;
pub const UPG_BOMB_BAG: C2RustUnnamed_25 = 1;
pub const UPG_QUIVER: C2RustUnnamed_25 = 0;
pub type C2RustUnnamed_26 = libc::c_uint;
pub const ITEM_NONE: C2RustUnnamed_26 = 255;
pub const ITEM_NONE_FE: C2RustUnnamed_26 = 254;
pub const ITEM_LAST_USED: C2RustUnnamed_26 = 252;
pub const ITEM_NUT_UPGRADE_40: C2RustUnnamed_26 = 155;
pub const ITEM_NUT_UPGRADE_30: C2RustUnnamed_26 = 154;
pub const ITEM_STICK_UPGRADE_30: C2RustUnnamed_26 = 153;
pub const ITEM_STICK_UPGRADE_20: C2RustUnnamed_26 = 152;
pub const ITEM_BOMBCHUS_20: C2RustUnnamed_26 = 151;
pub const ITEM_BOMBCHUS_5: C2RustUnnamed_26 = 150;
pub const ITEM_SEEDS_30: C2RustUnnamed_26 = 149;
pub const ITEM_ARROWS_LARGE: C2RustUnnamed_26 = 148;
pub const ITEM_ARROWS_MEDIUM: C2RustUnnamed_26 = 147;
pub const ITEM_ARROWS_SMALL: C2RustUnnamed_26 = 146;
pub const ITEM_BOMBS_30: C2RustUnnamed_26 = 145;
pub const ITEM_BOMBS_20: C2RustUnnamed_26 = 144;
pub const ITEM_BOMBS_10: C2RustUnnamed_26 = 143;
pub const ITEM_BOMBS_5: C2RustUnnamed_26 = 142;
pub const ITEM_NUTS_10: C2RustUnnamed_26 = 141;
pub const ITEM_NUTS_5: C2RustUnnamed_26 = 140;
pub const ITEM_STICKS_10: C2RustUnnamed_26 = 139;
pub const ITEM_STICKS_5: C2RustUnnamed_26 = 138;
pub const ITEM_INVALID_8: C2RustUnnamed_26 = 137;
pub const ITEM_RUPEE_GOLD: C2RustUnnamed_26 = 136;
pub const ITEM_RUPEE_PURPLE: C2RustUnnamed_26 = 135;
pub const ITEM_RUPEE_RED: C2RustUnnamed_26 = 134;
pub const ITEM_RUPEE_BLUE: C2RustUnnamed_26 = 133;
pub const ITEM_RUPEE_GREEN: C2RustUnnamed_26 = 132;
pub const ITEM_HEART: C2RustUnnamed_26 = 131;
pub const ITEM_MILK: C2RustUnnamed_26 = 130;
pub const ITEM_INVALID_7: C2RustUnnamed_26 = 129;
pub const ITEM_INVALID_6: C2RustUnnamed_26 = 128;
pub const ITEM_INVALID_5: C2RustUnnamed_26 = 127;
pub const ITEM_INVALID_4: C2RustUnnamed_26 = 126;
pub const ITEM_INVALID_3: C2RustUnnamed_26 = 125;
pub const ITEM_INVALID_2: C2RustUnnamed_26 = 124;
pub const ITEM_INVALID_1: C2RustUnnamed_26 = 123;
pub const ITEM_HEART_PIECE_2: C2RustUnnamed_26 = 122;
pub const ITEM_MAGIC_LARGE: C2RustUnnamed_26 = 121;
pub const ITEM_MAGIC_SMALL: C2RustUnnamed_26 = 120;
pub const ITEM_KEY_SMALL: C2RustUnnamed_26 = 119;
pub const ITEM_DUNGEON_MAP: C2RustUnnamed_26 = 118;
pub const ITEM_COMPASS: C2RustUnnamed_26 = 117;
pub const ITEM_KEY_BOSS: C2RustUnnamed_26 = 116;
pub const ITEM_HEART_PIECE: C2RustUnnamed_26 = 115;
pub const ITEM_HEART_CONTAINER: C2RustUnnamed_26 = 114;
pub const ITEM_SKULL_TOKEN: C2RustUnnamed_26 = 113;
pub const ITEM_GERUDO_CARD: C2RustUnnamed_26 = 112;
pub const ITEM_STONE_OF_AGONY: C2RustUnnamed_26 = 111;
pub const ITEM_ZORA_SAPPHIRE: C2RustUnnamed_26 = 110;
pub const ITEM_GORON_RUBY: C2RustUnnamed_26 = 109;
pub const ITEM_KOKIRI_EMERALD: C2RustUnnamed_26 = 108;
pub const ITEM_MEDALLION_LIGHT: C2RustUnnamed_26 = 107;
pub const ITEM_MEDALLION_SHADOW: C2RustUnnamed_26 = 106;
pub const ITEM_MEDALLION_SPIRIT: C2RustUnnamed_26 = 105;
pub const ITEM_MEDALLION_WATER: C2RustUnnamed_26 = 104;
pub const ITEM_MEDALLION_FIRE: C2RustUnnamed_26 = 103;
pub const ITEM_MEDALLION_FOREST: C2RustUnnamed_26 = 102;
pub const ITEM_SONG_STORMS: C2RustUnnamed_26 = 101;
pub const ITEM_SONG_TIME: C2RustUnnamed_26 = 100;
pub const ITEM_SONG_SUN: C2RustUnnamed_26 = 99;
pub const ITEM_SONG_SARIA: C2RustUnnamed_26 = 98;
pub const ITEM_SONG_EPONA: C2RustUnnamed_26 = 97;
pub const ITEM_SONG_LULLABY: C2RustUnnamed_26 = 96;
pub const ITEM_SONG_PRELUDE: C2RustUnnamed_26 = 95;
pub const ITEM_SONG_NOCTURNE: C2RustUnnamed_26 = 94;
pub const ITEM_SONG_REQUIEM: C2RustUnnamed_26 = 93;
pub const ITEM_SONG_SERENADE: C2RustUnnamed_26 = 92;
pub const ITEM_SONG_BOLERO: C2RustUnnamed_26 = 91;
pub const ITEM_SONG_MINUET: C2RustUnnamed_26 = 90;
pub const ITEM_FISHING_POLE: C2RustUnnamed_26 = 89;
pub const ITEM_SEEDS: C2RustUnnamed_26 = 88;
pub const ITEM_WALLET_GIANT: C2RustUnnamed_26 = 87;
pub const ITEM_WALLET_ADULT: C2RustUnnamed_26 = 86;
pub const ITEM_SWORD_KNIFE: C2RustUnnamed_26 = 85;
pub const ITEM_SCALE_GOLDEN: C2RustUnnamed_26 = 84;
pub const ITEM_SCALE_SILVER: C2RustUnnamed_26 = 83;
pub const ITEM_GAUNTLETS_GOLD: C2RustUnnamed_26 = 82;
pub const ITEM_GAUNTLETS_SILVER: C2RustUnnamed_26 = 81;
pub const ITEM_BRACELET: C2RustUnnamed_26 = 80;
pub const ITEM_BOMB_BAG_40: C2RustUnnamed_26 = 79;
pub const ITEM_BOMB_BAG_30: C2RustUnnamed_26 = 78;
pub const ITEM_BOMB_BAG_20: C2RustUnnamed_26 = 77;
pub const ITEM_QUIVER_50: C2RustUnnamed_26 = 76;
pub const ITEM_QUIVER_40: C2RustUnnamed_26 = 75;
pub const ITEM_QUIVER_30: C2RustUnnamed_26 = 74;
pub const ITEM_BULLET_BAG_50: C2RustUnnamed_26 = 73;
pub const ITEM_BULLET_BAG_40: C2RustUnnamed_26 = 72;
pub const ITEM_BULLET_BAG_30: C2RustUnnamed_26 = 71;
pub const ITEM_BOOTS_HOVER: C2RustUnnamed_26 = 70;
pub const ITEM_BOOTS_IRON: C2RustUnnamed_26 = 69;
pub const ITEM_BOOTS_KOKIRI: C2RustUnnamed_26 = 68;
pub const ITEM_TUNIC_ZORA: C2RustUnnamed_26 = 67;
pub const ITEM_TUNIC_GORON: C2RustUnnamed_26 = 66;
pub const ITEM_TUNIC_KOKIRI: C2RustUnnamed_26 = 65;
pub const ITEM_SHIELD_MIRROR: C2RustUnnamed_26 = 64;
pub const ITEM_SHIELD_HYLIAN: C2RustUnnamed_26 = 63;
pub const ITEM_SHIELD_DEKU: C2RustUnnamed_26 = 62;
pub const ITEM_SWORD_BGS: C2RustUnnamed_26 = 61;
pub const ITEM_SWORD_MASTER: C2RustUnnamed_26 = 60;
pub const ITEM_SWORD_KOKIRI: C2RustUnnamed_26 = 59;
pub const ITEM_BOW_ARROW_LIGHT: C2RustUnnamed_26 = 58;
pub const ITEM_BOW_ARROW_ICE: C2RustUnnamed_26 = 57;
pub const ITEM_BOW_ARROW_FIRE: C2RustUnnamed_26 = 56;
pub const ITEM_CLAIM_CHECK: C2RustUnnamed_26 = 55;
pub const ITEM_EYEDROPS: C2RustUnnamed_26 = 54;
pub const ITEM_FROG: C2RustUnnamed_26 = 53;
pub const ITEM_PRESCRIPTION: C2RustUnnamed_26 = 52;
pub const ITEM_SWORD_BROKEN: C2RustUnnamed_26 = 51;
pub const ITEM_SAW: C2RustUnnamed_26 = 50;
pub const ITEM_ODD_POTION: C2RustUnnamed_26 = 49;
pub const ITEM_ODD_MUSHROOM: C2RustUnnamed_26 = 48;
pub const ITEM_COJIRO: C2RustUnnamed_26 = 47;
pub const ITEM_POCKET_CUCCO: C2RustUnnamed_26 = 46;
pub const ITEM_POCKET_EGG: C2RustUnnamed_26 = 45;
pub const ITEM_SOLD_OUT: C2RustUnnamed_26 = 44;
pub const ITEM_MASK_TRUTH: C2RustUnnamed_26 = 43;
pub const ITEM_MASK_GERUDO: C2RustUnnamed_26 = 42;
pub const ITEM_MASK_ZORA: C2RustUnnamed_26 = 41;
pub const ITEM_MASK_GORON: C2RustUnnamed_26 = 40;
pub const ITEM_MASK_BUNNY: C2RustUnnamed_26 = 39;
pub const ITEM_MASK_SPOOKY: C2RustUnnamed_26 = 38;
pub const ITEM_MASK_SKULL: C2RustUnnamed_26 = 37;
pub const ITEM_MASK_KEATON: C2RustUnnamed_26 = 36;
pub const ITEM_LETTER_ZELDA: C2RustUnnamed_26 = 35;
pub const ITEM_CHICKEN: C2RustUnnamed_26 = 34;
pub const ITEM_WEIRD_EGG: C2RustUnnamed_26 = 33;
pub const ITEM_POE: C2RustUnnamed_26 = 32;
pub const ITEM_MILK_HALF: C2RustUnnamed_26 = 31;
pub const ITEM_BIG_POE: C2RustUnnamed_26 = 30;
pub const ITEM_BUG: C2RustUnnamed_26 = 29;
pub const ITEM_BLUE_FIRE: C2RustUnnamed_26 = 28;
pub const ITEM_LETTER_RUTO: C2RustUnnamed_26 = 27;
pub const ITEM_MILK_BOTTLE: C2RustUnnamed_26 = 26;
pub const ITEM_FISH: C2RustUnnamed_26 = 25;
pub const ITEM_FAIRY: C2RustUnnamed_26 = 24;
pub const ITEM_POTION_BLUE: C2RustUnnamed_26 = 23;
pub const ITEM_POTION_GREEN: C2RustUnnamed_26 = 22;
pub const ITEM_POTION_RED: C2RustUnnamed_26 = 21;
pub const ITEM_BOTTLE: C2RustUnnamed_26 = 20;
pub const ITEM_NAYRUS_LOVE: C2RustUnnamed_26 = 19;
pub const ITEM_ARROW_LIGHT: C2RustUnnamed_26 = 18;
pub const ITEM_HAMMER: C2RustUnnamed_26 = 17;
pub const ITEM_BEAN: C2RustUnnamed_26 = 16;
pub const ITEM_LENS: C2RustUnnamed_26 = 15;
pub const ITEM_BOOMERANG: C2RustUnnamed_26 = 14;
pub const ITEM_FARORES_WIND: C2RustUnnamed_26 = 13;
pub const ITEM_ARROW_ICE: C2RustUnnamed_26 = 12;
pub const ITEM_LONGSHOT: C2RustUnnamed_26 = 11;
pub const ITEM_HOOKSHOT: C2RustUnnamed_26 = 10;
pub const ITEM_BOMBCHU: C2RustUnnamed_26 = 9;
pub const ITEM_OCARINA_TIME: C2RustUnnamed_26 = 8;
pub const ITEM_OCARINA_FAIRY: C2RustUnnamed_26 = 7;
pub const ITEM_SLINGSHOT: C2RustUnnamed_26 = 6;
pub const ITEM_DINS_FIRE: C2RustUnnamed_26 = 5;
pub const ITEM_ARROW_FIRE: C2RustUnnamed_26 = 4;
pub const ITEM_BOW: C2RustUnnamed_26 = 3;
pub const ITEM_BOMB: C2RustUnnamed_26 = 2;
pub const ITEM_NUT: C2RustUnnamed_26 = 1;
pub const ITEM_STICK: C2RustUnnamed_26 = 0;
pub type C2RustUnnamed_27 = libc::c_uint;
pub const EXCH_ITEM_MAX: C2RustUnnamed_27 = 30;
pub const EXCH_ITEM_LETTER_RUTO: C2RustUnnamed_27 = 29;
pub const EXCH_ITEM_BIG_POE: C2RustUnnamed_27 = 28;
pub const EXCH_ITEM_POE: C2RustUnnamed_27 = 27;
pub const EXCH_ITEM_BUG: C2RustUnnamed_27 = 26;
pub const EXCH_ITEM_BLUE_FIRE: C2RustUnnamed_27 = 25;
pub const EXCH_ITEM_FISH: C2RustUnnamed_27 = 24;
pub const EXCH_ITEM_MASK_GERUDO: C2RustUnnamed_27 = 23;
pub const EXCH_ITEM_MASK_ZORA: C2RustUnnamed_27 = 22;
pub const EXCH_ITEM_MASK_GORON: C2RustUnnamed_27 = 21;
pub const EXCH_ITEM_MASK_TRUTH: C2RustUnnamed_27 = 20;
pub const EXCH_ITEM_MASK_BUNNY: C2RustUnnamed_27 = 19;
pub const EXCH_ITEM_MASK_KEATON: C2RustUnnamed_27 = 18;
pub const EXCH_ITEM_MASK_SPOOKY: C2RustUnnamed_27 = 17;
pub const EXCH_ITEM_MASK_SKULL: C2RustUnnamed_27 = 16;
pub const EXCH_ITEM_CLAIM_CHECK: C2RustUnnamed_27 = 15;
pub const EXCH_ITEM_EYEDROPS: C2RustUnnamed_27 = 14;
pub const EXCH_ITEM_FROG: C2RustUnnamed_27 = 13;
pub const EXCH_ITEM_PRESCRIPTION: C2RustUnnamed_27 = 12;
pub const EXCH_ITEM_SWORD_BROKEN: C2RustUnnamed_27 = 11;
pub const EXCH_ITEM_SAW: C2RustUnnamed_27 = 10;
pub const EXCH_ITEM_ODD_POTION: C2RustUnnamed_27 = 9;
pub const EXCH_ITEM_ODD_MUSHROOM: C2RustUnnamed_27 = 8;
pub const EXCH_ITEM_COJIRO: C2RustUnnamed_27 = 7;
pub const EXCH_ITEM_POCKET_CUCCO: C2RustUnnamed_27 = 6;
pub const EXCH_ITEM_POCKET_EGG: C2RustUnnamed_27 = 5;
pub const EXCH_ITEM_BEAN: C2RustUnnamed_27 = 4;
pub const EXCH_ITEM_CHICKEN: C2RustUnnamed_27 = 3;
pub const EXCH_ITEM_WEIRD_EGG: C2RustUnnamed_27 = 2;
pub const EXCH_ITEM_LETTER_ZELDA: C2RustUnnamed_27 = 1;
pub const EXCH_ITEM_NONE: C2RustUnnamed_27 = 0;
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
pub type C2RustUnnamed_28 = libc::c_uint;
pub const MTXMODE_APPLY: C2RustUnnamed_28 = 1;
pub const MTXMODE_NEW: C2RustUnnamed_28 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TextTriggerEntry {
    pub flag: u8_0,
    pub textId: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BowStringData {
    pub dList: *mut libc::c_void,
    pub pos: Vec3f,
}
// size = 0x10
#[no_mangle]
pub static mut gPlayerSkelHeaders: [*mut FlexSkeletonHeader; 2] =
    unsafe {
        [0x60377f4 as libc::c_int as *mut FlexSkeletonHeader,
         &gLinkChildSkel as *const FlexSkeletonHeader as
             *mut FlexSkeletonHeader]
    };
#[no_mangle]
pub static mut sBootData: [[s16; 17]; 6] =
    [[200 as libc::c_int as s16, 1000 as libc::c_int as s16,
      300 as libc::c_int as s16, 700 as libc::c_int as s16,
      550 as libc::c_int as s16, 270 as libc::c_int as s16,
      600 as libc::c_int as s16, 350 as libc::c_int as s16,
      800 as libc::c_int as s16, 600 as libc::c_int as s16,
      -(100 as libc::c_int) as s16, 600 as libc::c_int as s16,
      590 as libc::c_int as s16, 750 as libc::c_int as s16,
      125 as libc::c_int as s16, 200 as libc::c_int as s16,
      130 as libc::c_int as s16],
     [200 as libc::c_int as s16, 1000 as libc::c_int as s16,
      300 as libc::c_int as s16, 700 as libc::c_int as s16,
      550 as libc::c_int as s16, 270 as libc::c_int as s16,
      1000 as libc::c_int as s16, 0 as libc::c_int as s16,
      800 as libc::c_int as s16, 300 as libc::c_int as s16,
      -(160 as libc::c_int) as s16, 600 as libc::c_int as s16,
      590 as libc::c_int as s16, 750 as libc::c_int as s16,
      125 as libc::c_int as s16, 200 as libc::c_int as s16,
      130 as libc::c_int as s16],
     [200 as libc::c_int as s16, 1000 as libc::c_int as s16,
      300 as libc::c_int as s16, 700 as libc::c_int as s16,
      550 as libc::c_int as s16, 270 as libc::c_int as s16,
      600 as libc::c_int as s16, 600 as libc::c_int as s16,
      800 as libc::c_int as s16, 550 as libc::c_int as s16,
      -(100 as libc::c_int) as s16, 600 as libc::c_int as s16,
      540 as libc::c_int as s16, 270 as libc::c_int as s16,
      25 as libc::c_int as s16, 0 as libc::c_int as s16,
      130 as libc::c_int as s16],
     [200 as libc::c_int as s16, 1000 as libc::c_int as s16,
      300 as libc::c_int as s16, 700 as libc::c_int as s16,
      380 as libc::c_int as s16, 400 as libc::c_int as s16,
      0 as libc::c_int as s16, 300 as libc::c_int as s16,
      800 as libc::c_int as s16, 500 as libc::c_int as s16,
      -(100 as libc::c_int) as s16, 600 as libc::c_int as s16,
      590 as libc::c_int as s16, 750 as libc::c_int as s16,
      125 as libc::c_int as s16, 200 as libc::c_int as s16,
      130 as libc::c_int as s16],
     [80 as libc::c_int as s16, 800 as libc::c_int as s16,
      150 as libc::c_int as s16, 700 as libc::c_int as s16,
      480 as libc::c_int as s16, 270 as libc::c_int as s16,
      600 as libc::c_int as s16, 50 as libc::c_int as s16,
      800 as libc::c_int as s16, 550 as libc::c_int as s16,
      -(40 as libc::c_int) as s16, 400 as libc::c_int as s16,
      540 as libc::c_int as s16, 270 as libc::c_int as s16,
      25 as libc::c_int as s16, 0 as libc::c_int as s16,
      80 as libc::c_int as s16],
     [200 as libc::c_int as s16, 1000 as libc::c_int as s16,
      300 as libc::c_int as s16, 800 as libc::c_int as s16,
      500 as libc::c_int as s16, 400 as libc::c_int as s16,
      800 as libc::c_int as s16, 400 as libc::c_int as s16,
      800 as libc::c_int as s16, 550 as libc::c_int as s16,
      -(100 as libc::c_int) as s16, 600 as libc::c_int as s16,
      540 as libc::c_int as s16, 750 as libc::c_int as s16,
      125 as libc::c_int as s16, 400 as libc::c_int as s16,
      200 as libc::c_int as s16]];
// Used to map action params to model groups
#[no_mangle]
pub static mut sActionModelGroups: [u8_0; 67] =
    [3 as libc::c_int as u8_0, 15 as libc::c_int as u8_0,
     10 as libc::c_int as u8_0, 2 as libc::c_int as u8_0,
     2 as libc::c_int as u8_0, 5 as libc::c_int as u8_0,
     10 as libc::c_int as u8_0, 11 as libc::c_int as u8_0,
     6 as libc::c_int as u8_0, 6 as libc::c_int as u8_0,
     6 as libc::c_int as u8_0, 6 as libc::c_int as u8_0,
     6 as libc::c_int as u8_0, 6 as libc::c_int as u8_0,
     6 as libc::c_int as u8_0, 6 as libc::c_int as u8_0,
     9 as libc::c_int as u8_0, 9 as libc::c_int as u8_0,
     7 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 3 as libc::c_int as u8_0,
     3 as libc::c_int as u8_0, 6 as libc::c_int as u8_0,
     3 as libc::c_int as u8_0, 3 as libc::c_int as u8_0,
     3 as libc::c_int as u8_0, 3 as libc::c_int as u8_0,
     12 as libc::c_int as u8_0, 13 as libc::c_int as u8_0,
     14 as libc::c_int as u8_0, 14 as libc::c_int as u8_0,
     14 as libc::c_int as u8_0, 14 as libc::c_int as u8_0,
     14 as libc::c_int as u8_0, 14 as libc::c_int as u8_0,
     14 as libc::c_int as u8_0, 14 as libc::c_int as u8_0,
     14 as libc::c_int as u8_0, 14 as libc::c_int as u8_0,
     14 as libc::c_int as u8_0, 14 as libc::c_int as u8_0,
     14 as libc::c_int as u8_0, 3 as libc::c_int as u8_0,
     3 as libc::c_int as u8_0, 3 as libc::c_int as u8_0,
     3 as libc::c_int as u8_0, 3 as libc::c_int as u8_0,
     3 as libc::c_int as u8_0, 3 as libc::c_int as u8_0,
     3 as libc::c_int as u8_0, 3 as libc::c_int as u8_0,
     3 as libc::c_int as u8_0, 3 as libc::c_int as u8_0,
     3 as libc::c_int as u8_0, 3 as libc::c_int as u8_0,
     3 as libc::c_int as u8_0, 3 as libc::c_int as u8_0,
     3 as libc::c_int as u8_0, 3 as libc::c_int as u8_0,
     3 as libc::c_int as u8_0, 3 as libc::c_int as u8_0,
     3 as libc::c_int as u8_0, 3 as libc::c_int as u8_0,
     3 as libc::c_int as u8_0, 3 as libc::c_int as u8_0,
     3 as libc::c_int as u8_0];
#[no_mangle]
pub static mut sTextTriggers: [TextTriggerEntry; 4] =
    [{
         let mut init =
             TextTriggerEntry{flag: 1 as libc::c_int as u8_0,
                              textId: 0x3040 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             TextTriggerEntry{flag: 2 as libc::c_int as u8_0,
                              textId: 0x401d as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             TextTriggerEntry{flag: 0 as libc::c_int as u8_0,
                              textId: 0 as libc::c_int as u16_0,};
         init
     },
     {
         let mut init =
             TextTriggerEntry{flag: 2 as libc::c_int as u8_0,
                              textId: 0x401d as libc::c_int as u16_0,};
         init
     }];
// Used to map model groups to model types for [animation, left hand, right hand, sheath, waist]
#[no_mangle]
pub static mut gPlayerModelTypes: [[u8_0; 5]; 16] =
    [[2 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
      10 as libc::c_int as u8_0, 16 as libc::c_int as u8_0,
      20 as libc::c_int as u8_0],
     [1 as libc::c_int as u8_0, 2 as libc::c_int as u8_0,
      9 as libc::c_int as u8_0, 19 as libc::c_int as u8_0,
      20 as libc::c_int as u8_0],
     [1 as libc::c_int as u8_0, 2 as libc::c_int as u8_0,
      10 as libc::c_int as u8_0, 17 as libc::c_int as u8_0,
      20 as libc::c_int as u8_0],
     [0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
      8 as libc::c_int as u8_0, 18 as libc::c_int as u8_0,
      20 as libc::c_int as u8_0],
     [0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
      8 as libc::c_int as u8_0, 18 as libc::c_int as u8_0,
      20 as libc::c_int as u8_0],
     [3 as libc::c_int as u8_0, 4 as libc::c_int as u8_0,
      9 as libc::c_int as u8_0, 19 as libc::c_int as u8_0,
      20 as libc::c_int as u8_0],
     [4 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
      11 as libc::c_int as u8_0, 18 as libc::c_int as u8_0,
      20 as libc::c_int as u8_0],
     [5 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
      8 as libc::c_int as u8_0, 18 as libc::c_int as u8_0,
      20 as libc::c_int as u8_0],
     [0 as libc::c_int as u8_0, 6 as libc::c_int as u8_0,
      8 as libc::c_int as u8_0, 18 as libc::c_int as u8_0,
      20 as libc::c_int as u8_0],
     [4 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
      15 as libc::c_int as u8_0, 18 as libc::c_int as u8_0,
      20 as libc::c_int as u8_0],
     [3 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
      9 as libc::c_int as u8_0, 18 as libc::c_int as u8_0,
      20 as libc::c_int as u8_0],
     [3 as libc::c_int as u8_0, 5 as libc::c_int as u8_0,
      9 as libc::c_int as u8_0, 18 as libc::c_int as u8_0,
      20 as libc::c_int as u8_0],
     [0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
      13 as libc::c_int as u8_0, 18 as libc::c_int as u8_0,
      20 as libc::c_int as u8_0],
     [0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
      14 as libc::c_int as u8_0, 18 as libc::c_int as u8_0,
      20 as libc::c_int as u8_0],
     [0 as libc::c_int as u8_0, 7 as libc::c_int as u8_0,
      8 as libc::c_int as u8_0, 18 as libc::c_int as u8_0,
      20 as libc::c_int as u8_0],
     [0 as libc::c_int as u8_0, 2 as libc::c_int as u8_0,
      8 as libc::c_int as u8_0, 19 as libc::c_int as u8_0,
      20 as libc::c_int as u8_0]];
#[no_mangle]
pub static mut D_80125CE8: [*mut Gfx; 16] =
    unsafe {
        [0x60226e0 as libc::c_int as *mut Gfx,
         gLinkChildRightHandClosedNearDL.as_ptr() as *mut _,
         0x6027690 as libc::c_int as *mut Gfx,
         gLinkChildRightHandClosedFarDL.as_ptr() as *mut _,
         0x60226e0 as libc::c_int as *mut Gfx,
         gLinkChildRightFistAndDekuShieldNearDL.as_ptr() as *mut _,
         0x6027690 as libc::c_int as *mut Gfx,
         gLinkChildRightFistAndDekuShieldFarDL.as_ptr() as *mut _,
         0x6022970 as libc::c_int as *mut Gfx,
         gLinkChildRightHandClosedNearDL.as_ptr() as *mut _,
         0x6027918 as libc::c_int as *mut Gfx,
         gLinkChildRightHandClosedFarDL.as_ptr() as *mut _,
         0x60241c0 as libc::c_int as *mut Gfx,
         gLinkChildRightHandClosedNearDL.as_ptr() as *mut _,
         0x6028b40 as libc::c_int as *mut Gfx,
         gLinkChildRightHandClosedFarDL.as_ptr() as *mut _]
    };
#[no_mangle]
pub static mut D_80125D28: [*mut Gfx; 16] =
    unsafe {
        [0x6023160 as libc::c_int as *mut Gfx,
         gLinkChildSwordAndSheathNearDL.as_ptr() as *mut _,
         0x6027f00 as libc::c_int as *mut Gfx,
         gLinkChildSwordAndSheathFarDL.as_ptr() as *mut _,
         0x6023160 as libc::c_int as *mut Gfx,
         gLinkChildDekuShieldSwordAndSheathNearDL.as_ptr() as *mut _,
         0x6027f00 as libc::c_int as *mut Gfx,
         gLinkChildDekuShieldSwordAndSheathFarDL.as_ptr() as *mut _,
         0x6020a78 as libc::c_int as *mut Gfx,
         gLinkChildHylianShieldSwordAndSheathNearDL.as_ptr() as *mut _,
         0x6025fb8 as libc::c_int as *mut Gfx,
         gLinkChildHylianShieldSwordAndSheathFarDL.as_ptr() as *mut _,
         0x60211b8 as libc::c_int as *mut Gfx,
         gLinkChildSwordAndSheathNearDL.as_ptr() as *mut _,
         0x60264f0 as libc::c_int as *mut Gfx,
         gLinkChildSwordAndSheathFarDL.as_ptr() as *mut _]
    };
#[no_mangle]
pub static mut D_80125D68: [*mut Gfx; 8] =
    unsafe {
        [0 as *const Gfx as *mut Gfx, 0 as *const Gfx as *mut Gfx,
         0 as *const Gfx as *mut Gfx, 0 as *const Gfx as *mut Gfx,
         0 as *const Gfx as *mut Gfx,
         gLinkChildDekuShieldWithMatrixDL.as_ptr() as *mut _,
         0 as *const Gfx as *mut Gfx,
         gLinkChildDekuShieldWithMatrixDL.as_ptr() as *mut _]
    };
#[no_mangle]
pub static mut D_80125D88: [*mut Gfx; 24] =
    unsafe {
        [0x60249d8 as libc::c_int as *mut Gfx,
         gLinkChildSheathNearDL.as_ptr() as *mut _,
         0x6028150 as libc::c_int as *mut Gfx,
         gLinkChildSheathFarDL.as_ptr() as *mut _,
         0x60249d8 as libc::c_int as *mut Gfx,
         gLinkChildDekuShieldAndSheathNearDL.as_ptr() as *mut _,
         0x6028150 as libc::c_int as *mut Gfx,
         gLinkChildDekuShieldAndSheathFarDL.as_ptr() as *mut _,
         0x6020e70 as libc::c_int as *mut Gfx,
         gLinkChildHylianShieldAndSheathNearDL.as_ptr() as *mut _,
         0x60262b8 as libc::c_int as *mut Gfx,
         gLinkChildHylianShieldAndSheathFarDL.as_ptr() as *mut _,
         0x60216b0 as libc::c_int as *mut Gfx,
         gLinkChildSheathNearDL.as_ptr() as *mut _,
         0x6026910 as libc::c_int as *mut Gfx,
         gLinkChildSheathFarDL.as_ptr() as *mut _,
         0 as *const Gfx as *mut Gfx, 0 as *const Gfx as *mut Gfx,
         0 as *const Gfx as *mut Gfx, 0 as *const Gfx as *mut Gfx,
         0x60249d8 as libc::c_int as *mut Gfx,
         gLinkChildDekuShieldWithMatrixDL.as_ptr() as *mut _,
         0x60249d8 as libc::c_int as *mut Gfx,
         gLinkChildDekuShieldWithMatrixDL.as_ptr() as *mut _]
    };
#[no_mangle]
pub static mut D_80125DE8: [*mut Gfx; 8] =
    unsafe {
        [0x60238c8 as libc::c_int as *mut Gfx,
         gLinkChildLeftHandHoldingMasterSwordDL.as_ptr() as *mut _,
         0x60286b8 as libc::c_int as *mut Gfx,
         gLinkChildLeftHandHoldingMasterSwordDL.as_ptr() as *mut _,
         0x6023d50 as libc::c_int as *mut Gfx,
         gLinkChildLeftHandHoldingMasterSwordDL.as_ptr() as *mut _,
         0x60291e8 as libc::c_int as *mut Gfx,
         gLinkChildLeftHandHoldingMasterSwordDL.as_ptr() as *mut _]
    };
#[no_mangle]
pub static mut D_80125E08: [*mut Gfx; 4] =
    unsafe {
        [0x6021aa8 as libc::c_int as *mut Gfx,
         gLinkChildLeftHandNearDL.as_ptr() as *mut _,
         0x6026c58 as libc::c_int as *mut Gfx,
         gLinkChildLeftHandFarDL.as_ptr() as *mut _]
    };
#[no_mangle]
pub static mut D_80125E18: [*mut Gfx; 4] =
    unsafe {
        [0x6021ce8 as libc::c_int as *mut Gfx,
         gLinkChildLeftFistNearDL.as_ptr() as *mut _,
         0x6026df0 as libc::c_int as *mut Gfx,
         gLinkChildLeftFistFarDL.as_ptr() as *mut _]
    };
#[no_mangle]
pub static mut D_80125E28: [*mut Gfx; 4] =
    unsafe {
        [0x6021f78 as libc::c_int as *mut Gfx,
         gLinkChildLeftFistAndKokiriSwordNearDL.as_ptr() as *mut _,
         0x6027078 as libc::c_int as *mut Gfx,
         gLinkChildLeftFistAndKokiriSwordFarDL.as_ptr() as *mut _]
    };
#[no_mangle]
pub static mut D_80125E38: [*mut Gfx; 4] =
    unsafe {
        [0x6021f78 as libc::c_int as *mut Gfx,
         gLinkChildLeftFistAndKokiriSwordNearDL.as_ptr() as *mut _,
         0x6027078 as libc::c_int as *mut Gfx,
         gLinkChildLeftFistAndKokiriSwordFarDL.as_ptr() as *mut _]
    };
#[no_mangle]
pub static mut D_80125E48: [*mut Gfx; 4] =
    unsafe {
        [0x6022498 as libc::c_int as *mut Gfx,
         gLinkChildRightHandNearDL.as_ptr() as *mut _,
         0x60274f8 as libc::c_int as *mut Gfx,
         gLinkChildRightHandFarDL.as_ptr() as *mut _]
    };
#[no_mangle]
pub static mut D_80125E58: [*mut Gfx; 4] =
    unsafe {
        [0x60226e0 as libc::c_int as *mut Gfx,
         gLinkChildRightHandClosedNearDL.as_ptr() as *mut _,
         0x6027690 as libc::c_int as *mut Gfx,
         gLinkChildRightHandClosedFarDL.as_ptr() as *mut _]
    };
#[no_mangle]
pub static mut D_80125E68: [*mut Gfx; 4] =
    unsafe {
        [0x6022da8 as libc::c_int as *mut Gfx,
         gLinkChildRightHandHoldingFairySlingshotNearDL.as_ptr() as *mut _,
         0x6027b88 as libc::c_int as *mut Gfx,
         gLinkChildRightHandHoldingFairySlingshotFarDL.as_ptr() as *mut _]
    };
#[no_mangle]
pub static mut D_80125E78: [*mut Gfx; 4] =
    unsafe {
        [0x6023160 as libc::c_int as *mut Gfx,
         gLinkChildSwordAndSheathNearDL.as_ptr() as *mut _,
         0x6027f00 as libc::c_int as *mut Gfx,
         gLinkChildSwordAndSheathFarDL.as_ptr() as *mut _]
    };
#[no_mangle]
pub static mut D_80125E88: [*mut Gfx; 4] =
    unsafe {
        [0x60249d8 as libc::c_int as *mut Gfx,
         gLinkChildSheathNearDL.as_ptr() as *mut _,
         0x6028150 as libc::c_int as *mut Gfx,
         gLinkChildSheathFarDL.as_ptr() as *mut _]
    };
#[no_mangle]
pub static mut D_80125E98: [*mut Gfx; 4] =
    unsafe {
        [0x6035330 as libc::c_int as *mut Gfx,
         gLinkChildWaistNearDL.as_ptr() as *mut _,
         0x602f530 as libc::c_int as *mut Gfx,
         gLinkChildWaistFarDL.as_ptr() as *mut _]
    };
#[no_mangle]
pub static mut D_80125EA8: [*mut Gfx; 4] =
    unsafe {
        [0x6022da8 as libc::c_int as *mut Gfx,
         gLinkChildRightHandHoldingFairySlingshotNearDL.as_ptr() as *mut _,
         0x6027b88 as libc::c_int as *mut Gfx,
         gLinkChildRightHandHoldingFairySlingshotFarDL.as_ptr() as *mut _]
    };
#[no_mangle]
pub static mut D_80125EB8: [*mut Gfx; 4] =
    unsafe {
        [0x6024698 as libc::c_int as *mut Gfx,
         gLinkChildRightHandHoldingFairyOcarinaNearDL.as_ptr() as *mut _,
         0x6028f58 as libc::c_int as *mut Gfx,
         gLinkChildRightHandHoldingFairyOcarinaFarDL.as_ptr() as *mut _]
    };
#[no_mangle]
pub static mut D_80125EC8: [*mut Gfx; 4] =
    unsafe {
        [0x6024698 as libc::c_int as *mut Gfx,
         gLinkChildRightHandAndOOTNearDL.as_ptr() as *mut _,
         0x6028f58 as libc::c_int as *mut Gfx,
         gLinkChildRightHandHoldingOOTFarDL.as_ptr() as *mut _]
    };
#[no_mangle]
pub static mut D_80125ED8: [*mut Gfx; 4] =
    unsafe {
        [0x6024d70 as libc::c_int as *mut Gfx,
         gLinkChildRightHandNearDL.as_ptr() as *mut _,
         0x6024d70 as libc::c_int as *mut Gfx,
         gLinkChildRightHandFarDL.as_ptr() as *mut _]
    };
#[no_mangle]
pub static mut D_80125EE8: [*mut Gfx; 4] =
    unsafe {
        [0x60233e0 as libc::c_int as *mut Gfx,
         gLinkChildLeftHandNearDL.as_ptr() as *mut _,
         0x6028288 as libc::c_int as *mut Gfx,
         gLinkChildLeftHandFarDL.as_ptr() as *mut _]
    };
#[no_mangle]
pub static mut D_80125EF8: [*mut Gfx; 4] =
    unsafe {
        [0x6021aa8 as libc::c_int as *mut Gfx,
         gLinkChildLeftFistAndBoomerangNearDL.as_ptr() as *mut _,
         0x6026c58 as libc::c_int as *mut Gfx,
         gLinkChildLeftFistAndBoomerangFarDL.as_ptr() as *mut _]
    };
#[no_mangle]
pub static mut D_80125F08: [*mut Gfx; 4] =
    unsafe {
        [0x6024b58 as libc::c_int as *mut Gfx,
         gLinkChildLeftHandUpNearDL.as_ptr() as *mut _,
         0x6024b58 as libc::c_int as *mut Gfx,
         gLinkChildLeftHandUpNearDL.as_ptr() as *mut _]
    };
#[no_mangle]
pub static mut D_80125F18: [*mut Gfx; 2] =
    unsafe {
        [0x6029918 as libc::c_int as *mut Gfx, 0 as *const Gfx as *mut Gfx]
    };
#[no_mangle]
pub static mut D_80125F20: [*mut Gfx; 2] =
    unsafe {
        [0x6029c20 as libc::c_int as *mut Gfx, 0 as *const Gfx as *mut Gfx]
    };
#[no_mangle]
pub static mut D_80125F28: [*mut Gfx; 2] =
    unsafe {
        [0x6036e58 as libc::c_int as *mut Gfx,
         gLinkChildLeftShoulderNearDL.as_ptr() as *mut _]
    };
#[no_mangle]
pub static mut D_80125F30: [*mut Gfx; 2] =
    unsafe {
        [0x6029fa0 as libc::c_int as *mut Gfx, 0 as *const Gfx as *mut Gfx]
    };
#[no_mangle]
pub static mut D_80125F38: [*mut Gfx; 2] =
    unsafe {
        [0x602a248 as libc::c_int as *mut Gfx,
         gLinkChildRightArmStretchedSlingshotDL.as_ptr() as *mut _]
    };
// Indexed by model types (left hand, right hand, sheath or waist)
#[no_mangle]
pub static mut sPlayerDListGroups: [*mut *mut Gfx; 21] =
    unsafe {
        [D_80125E08.as_ptr() as *mut _, D_80125E18.as_ptr() as *mut _,
         D_80125E38.as_ptr() as *mut _, D_80125E28.as_ptr() as *mut _,
         D_80125DE8.as_ptr() as *mut _, D_80125EE8.as_ptr() as *mut _,
         D_80125EF8.as_ptr() as *mut _, D_80125F08.as_ptr() as *mut _,
         D_80125E48.as_ptr() as *mut _, D_80125E58.as_ptr() as *mut _,
         D_80125CE8.as_ptr() as *mut _, D_80125E68.as_ptr() as *mut _,
         D_80125EA8.as_ptr() as *mut _, D_80125EB8.as_ptr() as *mut _,
         D_80125EC8.as_ptr() as *mut _, D_80125ED8.as_ptr() as *mut _,
         D_80125E78.as_ptr() as *mut _, D_80125E88.as_ptr() as *mut _,
         D_80125D28.as_ptr() as *mut _, D_80125D88.as_ptr() as *mut _,
         D_80125E98.as_ptr() as *mut _]
    };
#[no_mangle]
pub static mut gCullBackDList: [Gfx; 2] =
    [Gfx{words:
             {
                 let mut init =
                     Gwords{w0:
                                (0xd9 as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int |
                                    (!(0 as libc::c_int as u32_0) &
                                         (((0x1 as libc::c_int) <<
                                               24 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        0 as libc::c_int,
                            w1: 0x400 as libc::c_int as u32_0,};
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
pub static mut gCullFrontDList: [Gfx; 2] =
    [Gfx{words:
             {
                 let mut init =
                     Gwords{w0:
                                (0xd9 as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int |
                                    (!(0 as libc::c_int as u32_0) &
                                         (((0x1 as libc::c_int) <<
                                               24 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        0 as libc::c_int,
                            w1: 0x200 as libc::c_int as u32_0,};
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
pub static mut D_80160000: *mut Vec3f = 0 as *const Vec3f as *mut Vec3f;
#[no_mangle]
pub static mut sDListsLodOffset: s32 = 0;
#[no_mangle]
pub static mut sGetItemRefPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
#[no_mangle]
pub static mut D_80160014: s32 = 0;
#[no_mangle]
pub static mut D_80160018: s32 = 0;
#[no_mangle]
pub unsafe extern "C" fn Player_SetBootData(mut globalCtx: *mut GlobalContext,
                                            mut this: *mut Player) {
    let mut currentBoots: s32 = 0;
    let mut bootRegs: *mut s16 = 0 as *mut s16;
    (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 27 as libc::c_int) as usize] =
        2000 as libc::c_int as s16;
    (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 48 as libc::c_int) as usize] =
        370 as libc::c_int as s16;
    currentBoots = (*this).currentBoots as s32;
    if currentBoots == PLAYER_BOOTS_NORMAL as libc::c_int {
        if !(gSaveContext.linkAge == 0 as libc::c_int) {
            currentBoots = PLAYER_BOOTS_NORMAL_CHILD as libc::c_int
        }
    } else if currentBoots == PLAYER_BOOTS_IRON as libc::c_int {
        if (*this).stateFlags1 & 0x8000000 as libc::c_int as libc::c_uint != 0
           {
            currentBoots = PLAYER_BOOTS_IRON_UNDERWATER as libc::c_int
        }
        (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 27 as libc::c_int) as
                              usize] = 500 as libc::c_int as s16;
        (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 48 as libc::c_int) as
                              usize] = 100 as libc::c_int as s16
    }
    bootRegs = sBootData[currentBoots as usize].as_mut_ptr();
    (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 19 as libc::c_int) as usize] =
        *bootRegs.offset(0 as libc::c_int as isize);
    (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 30 as libc::c_int) as usize] =
        *bootRegs.offset(1 as libc::c_int as isize);
    (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 32 as libc::c_int) as usize] =
        *bootRegs.offset(2 as libc::c_int as isize);
    (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 34 as libc::c_int) as usize] =
        *bootRegs.offset(3 as libc::c_int as isize);
    (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 35 as libc::c_int) as usize] =
        *bootRegs.offset(4 as libc::c_int as isize);
    (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 36 as libc::c_int) as usize] =
        *bootRegs.offset(5 as libc::c_int as isize);
    (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 37 as libc::c_int) as usize] =
        *bootRegs.offset(6 as libc::c_int as isize);
    (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 38 as libc::c_int) as usize] =
        *bootRegs.offset(7 as libc::c_int as isize);
    (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 43 as libc::c_int) as usize] =
        *bootRegs.offset(8 as libc::c_int as isize);
    (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 45 as libc::c_int) as usize] =
        *bootRegs.offset(9 as libc::c_int as isize);
    (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 68 as libc::c_int) as usize] =
        *bootRegs.offset(10 as libc::c_int as isize);
    (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 69 as libc::c_int) as usize] =
        *bootRegs.offset(11 as libc::c_int as isize);
    (*gGameInfo).data[(9 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 66 as libc::c_int) as usize] =
        *bootRegs.offset(12 as libc::c_int as isize);
    (*gGameInfo).data[(9 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 67 as libc::c_int) as usize] =
        *bootRegs.offset(13 as libc::c_int as isize);
    (*gGameInfo).data[(9 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 68 as libc::c_int) as usize] =
        *bootRegs.offset(14 as libc::c_int as isize);
    (*gGameInfo).data[(9 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 69 as libc::c_int) as usize] =
        *bootRegs.offset(15 as libc::c_int as isize);
    (*gGameInfo).data[(5 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 95 as libc::c_int) as usize] =
        *bootRegs.offset(16 as libc::c_int as isize);
    if (*globalCtx).roomCtx.curRoom.unk_03 as libc::c_int == 2 as libc::c_int
       {
        (*gGameInfo).data[(0 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 45 as libc::c_int) as
                              usize] = 500 as libc::c_int as s16
    };
}
#[no_mangle]
pub unsafe extern "C" fn Player_InBlockingCsMode(mut globalCtx:
                                                     *mut GlobalContext,
                                                 mut this: *mut Player)
 -> s32 {
    return ((*this).stateFlags1 & 0x20000080 as libc::c_int as libc::c_uint !=
                0 || (*this).csMode as libc::c_int != 0 as libc::c_int ||
                (*globalCtx).sceneLoadFlag as libc::c_int ==
                    0x14 as libc::c_int ||
                (*this).stateFlags1 & 1 as libc::c_int as libc::c_uint != 0 ||
                (*this).stateFlags3 as libc::c_int & 0x80 as libc::c_int != 0
                ||
                gSaveContext.unk_13F0 as libc::c_int != 0 as libc::c_int &&
                    Player_ActionToMagicSpell(this,
                                              (*this).itemActionParam as s32)
                        >= 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Player_InCsMode(mut globalCtx: *mut GlobalContext)
 -> s32 {
    let mut this: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    return (Player_InBlockingCsMode(globalCtx, this) != 0 ||
                (*this).unk_6AD as libc::c_int == 4 as libc::c_int) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_8008E9C4(mut this: *mut Player) -> s32 {
    return ((*this).stateFlags1 & 0x10 as libc::c_int as libc::c_uint) as s32;
}
#[no_mangle]
pub unsafe extern "C" fn Player_IsChildWithHylianShield(mut this: *mut Player)
 -> s32 {
    return (gSaveContext.linkAge != 0 as libc::c_int &&
                (*this).currentShield as libc::c_int ==
                    PLAYER_SHIELD_HYLIAN as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Player_ActionToModelGroup(mut this: *mut Player,
                                                   mut actionParam: s32)
 -> s32 {
    let mut modelGroup: s32 = sActionModelGroups[actionParam as usize] as s32;
    if modelGroup == 2 as libc::c_int &&
           Player_IsChildWithHylianShield(this) != 0 {
        return 1 as libc::c_int
    } else { return modelGroup };
}
#[no_mangle]
pub unsafe extern "C" fn Player_SetModelsForHoldingShield(mut this:
                                                              *mut Player) {
    if (*this).stateFlags1 & 0x400000 as libc::c_int as libc::c_uint != 0 &&
           (((*this).itemActionParam as libc::c_int) < 0 as libc::c_int ||
                (*this).itemActionParam as libc::c_int ==
                    (*this).heldItemActionParam as libc::c_int) {
        if Player_HoldsTwoHandedWeapon(this) == 0 &&
               Player_IsChildWithHylianShield(this) == 0 {
            (*this).rightHandType = 10 as libc::c_int as u8_0;
            (*this).rightHandDLists =
                &mut *(*sPlayerDListGroups.as_mut_ptr().offset(10 as
                                                                   libc::c_int
                                                                   as
                                                                   isize)).offset(gSaveContext.linkAge
                                                                                      as
                                                                                      isize)
                    as *mut *mut Gfx;
            if (*this).sheathType as libc::c_int == 18 as libc::c_int {
                (*this).sheathType = 16 as libc::c_int as u8_0
            } else if (*this).sheathType as libc::c_int == 19 as libc::c_int {
                (*this).sheathType = 17 as libc::c_int as u8_0
            }
            (*this).sheathDLists =
                &mut *(*sPlayerDListGroups.as_mut_ptr().offset((*this).sheathType
                                                                   as
                                                                   isize)).offset(gSaveContext.linkAge
                                                                                      as
                                                                                      isize)
                    as *mut *mut Gfx;
            (*this).modelAnimType = 2 as libc::c_int as u8_0;
            (*this).itemActionParam = -(1 as libc::c_int) as s8
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Player_SetModels(mut this: *mut Player,
                                          mut modelGroup: s32) {
    (*this).leftHandType =
        gPlayerModelTypes[modelGroup as usize][1 as libc::c_int as usize];
    (*this).rightHandType =
        gPlayerModelTypes[modelGroup as usize][2 as libc::c_int as usize];
    (*this).sheathType =
        gPlayerModelTypes[modelGroup as usize][3 as libc::c_int as usize];
    (*this).leftHandDLists =
        &mut *(*sPlayerDListGroups.as_mut_ptr().offset(*(*gPlayerModelTypes.as_mut_ptr().offset(modelGroup
                                                                                                    as
                                                                                                    isize)).as_mut_ptr().offset(1
                                                                                                                                    as
                                                                                                                                    libc::c_int
                                                                                                                                    as
                                                                                                                                    isize)
                                                           as
                                                           isize)).offset(gSaveContext.linkAge
                                                                              as
                                                                              isize)
            as *mut *mut Gfx;
    (*this).rightHandDLists =
        &mut *(*sPlayerDListGroups.as_mut_ptr().offset(*(*gPlayerModelTypes.as_mut_ptr().offset(modelGroup
                                                                                                    as
                                                                                                    isize)).as_mut_ptr().offset(2
                                                                                                                                    as
                                                                                                                                    libc::c_int
                                                                                                                                    as
                                                                                                                                    isize)
                                                           as
                                                           isize)).offset(gSaveContext.linkAge
                                                                              as
                                                                              isize)
            as *mut *mut Gfx;
    (*this).sheathDLists =
        &mut *(*sPlayerDListGroups.as_mut_ptr().offset(*(*gPlayerModelTypes.as_mut_ptr().offset(modelGroup
                                                                                                    as
                                                                                                    isize)).as_mut_ptr().offset(3
                                                                                                                                    as
                                                                                                                                    libc::c_int
                                                                                                                                    as
                                                                                                                                    isize)
                                                           as
                                                           isize)).offset(gSaveContext.linkAge
                                                                              as
                                                                              isize)
            as *mut *mut Gfx;
    (*this).waistDLists =
        &mut *(*sPlayerDListGroups.as_mut_ptr().offset(*(*gPlayerModelTypes.as_mut_ptr().offset(modelGroup
                                                                                                    as
                                                                                                    isize)).as_mut_ptr().offset(4
                                                                                                                                    as
                                                                                                                                    libc::c_int
                                                                                                                                    as
                                                                                                                                    isize)
                                                           as
                                                           isize)).offset(gSaveContext.linkAge
                                                                              as
                                                                              isize)
            as *mut *mut Gfx;
    Player_SetModelsForHoldingShield(this);
}
#[no_mangle]
pub unsafe extern "C" fn Player_SetModelGroup(mut this: *mut Player,
                                              mut modelGroup: s32) {
    (*this).modelGroup = modelGroup as u8_0;
    if modelGroup == 1 as libc::c_int {
        (*this).modelAnimType = 0 as libc::c_int as u8_0
    } else {
        (*this).modelAnimType =
            gPlayerModelTypes[modelGroup as usize][0 as libc::c_int as usize]
    }
    if ((*this).modelAnimType as libc::c_int) < 3 as libc::c_int &&
           (*this).currentShield as libc::c_int ==
               PLAYER_SHIELD_NONE as libc::c_int {
        (*this).modelAnimType = 0 as libc::c_int as u8_0
    }
    Player_SetModels(this, modelGroup);
}
#[no_mangle]
pub unsafe extern "C" fn func_8008EC70(mut this: *mut Player) {
    (*this).itemActionParam = (*this).heldItemActionParam;
    Player_SetModelGroup(this,
                         Player_ActionToModelGroup(this,
                                                   (*this).heldItemActionParam
                                                       as s32));
    (*this).unk_6AD = 0 as libc::c_int as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn Player_SetEquipmentData(mut globalCtx:
                                                     *mut GlobalContext,
                                                 mut this: *mut Player) {
    if (*this).csMode as libc::c_int != 0x56 as libc::c_int {
        (*this).currentShield =
            ((gSaveContext.equips.equipment as libc::c_int &
                  gEquipMasks[EQUIP_SHIELD as libc::c_int as usize] as
                      libc::c_int) >>
                 gEquipShifts[EQUIP_SHIELD as libc::c_int as usize] as
                     libc::c_int) as s8;
        (*this).currentTunic =
            (((gSaveContext.equips.equipment as libc::c_int &
                   gEquipMasks[EQUIP_TUNIC as libc::c_int as usize] as
                       libc::c_int) >>
                  gEquipShifts[EQUIP_TUNIC as libc::c_int as usize] as
                      libc::c_int) - 1 as libc::c_int) as s8;
        (*this).currentBoots =
            (((gSaveContext.equips.equipment as libc::c_int &
                   gEquipMasks[EQUIP_BOOTS as libc::c_int as usize] as
                       libc::c_int) >>
                  gEquipShifts[EQUIP_BOOTS as libc::c_int as usize] as
                      libc::c_int) - 1 as libc::c_int) as s8;
        (*this).currentSword =
            if gSaveContext.buttonStatus[0 as libc::c_int as usize] as
                   libc::c_int == ITEM_NONE as libc::c_int {
                ITEM_NONE as libc::c_int
            } else if gSaveContext.equips.buttonItems[0 as libc::c_int as
                                                          usize] as
                          libc::c_int == ITEM_SWORD_KNIFE as libc::c_int {
                ITEM_SWORD_BGS as libc::c_int
            } else {
                gSaveContext.equips.buttonItems[0 as libc::c_int as usize] as
                    libc::c_int
            } as s8;
        Player_SetModelGroup(this,
                             Player_ActionToModelGroup(this,
                                                       (*this).heldItemActionParam
                                                           as s32));
        Player_SetBootData(globalCtx, this);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Player_UpdateBottleHeld(mut globalCtx:
                                                     *mut GlobalContext,
                                                 mut this: *mut Player,
                                                 mut item: s32,
                                                 mut actionParam: s32) {
    Inventory_UpdateBottleItem(globalCtx, item as u8_0,
                               (*this).heldItemButton as u8_0);
    if item != ITEM_BOTTLE as libc::c_int {
        (*this).heldItemId = item as u8_0;
        (*this).heldItemActionParam = actionParam as s8
    }
    (*this).itemActionParam = actionParam as s8;
}
#[no_mangle]
pub unsafe extern "C" fn func_8008EDF0(mut this: *mut Player) {
    (*this).unk_664 = 0 as *mut Actor;
    (*this).stateFlags2 &= !(0x2000 as libc::c_int) as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn func_8008EE08(mut this: *mut Player) {
    if (*this).actor.bgCheckFlags as libc::c_int & 1 as libc::c_int != 0 ||
           (*this).stateFlags1 & 0x8a00000 as libc::c_int as libc::c_uint != 0
           ||
           (*this).stateFlags1 & 0xc0000 as libc::c_int as libc::c_uint == 0
               &&
               (*this).actor.world.pos.y - (*this).actor.floorHeight <
                   100.0f32 {
        (*this).stateFlags1 &= !(0x400f8000 as libc::c_int) as libc::c_uint
    } else if (*this).stateFlags1 & 0x2c0000 as libc::c_int as libc::c_uint ==
                  0 {
        (*this).stateFlags1 |= 0x80000 as libc::c_int as libc::c_uint
    }
    func_8008EDF0(this);
}
#[no_mangle]
pub unsafe extern "C" fn func_8008EEAC(mut globalCtx: *mut GlobalContext,
                                       mut actor: *mut Actor) {
    let mut this: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    func_8008EE08(this);
    (*this).unk_664 = actor;
    (*this).unk_684 = actor;
    (*this).stateFlags1 |= 0x10000 as libc::c_int as libc::c_uint;
    Camera_SetParam(Gameplay_GetCamera(globalCtx, 0 as libc::c_int as s16),
                    8 as libc::c_int, actor as *mut libc::c_void);
    Camera_ChangeMode(Gameplay_GetCamera(globalCtx, 0 as libc::c_int as s16),
                      2 as libc::c_int as s16);
}
#[no_mangle]
pub unsafe extern "C" fn func_8008EF30(mut globalCtx: *mut GlobalContext)
 -> s32 {
    let mut this: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    return ((*this).stateFlags1 & 0x800000 as libc::c_int as libc::c_uint) as
               s32;
}
#[no_mangle]
pub unsafe extern "C" fn func_8008EF44(mut globalCtx: *mut GlobalContext,
                                       mut ammo: s32) -> s32 {
    (*globalCtx).shootingGalleryStatus = (ammo + 1 as libc::c_int) as s8;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Player_IsBurningStickInRange(mut globalCtx:
                                                          *mut GlobalContext,
                                                      mut pos: *mut Vec3f,
                                                      mut xzRange: f32_0,
                                                      mut yRange: f32_0)
 -> s32 {
    let mut this: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut diff: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut pad: s32 = 0;
    if (*this).heldItemActionParam as libc::c_int ==
           PLAYER_AP_STICK as libc::c_int &&
           (*this).unk_860 as libc::c_int != 0 as libc::c_int {
        Math_Vec3f_Diff(&mut (*(*this).swordInfo.as_mut_ptr().offset(0 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)).tip,
                        pos, &mut diff);
        return (diff.x * diff.x + diff.z * diff.z <= xzRange * xzRange &&
                    0.0f32 <= diff.y && diff.y <= yRange) as libc::c_int
    } else { return 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn Player_GetStrength() -> s32 {
    let mut strengthUpgrade: s32 =
        (gSaveContext.inventory.upgrades &
             gUpgradeMasks[UPG_STRENGTH as libc::c_int as usize]) as s32 >>
            gUpgradeShifts[UPG_STRENGTH as libc::c_int as usize] as
                libc::c_int;
    if gSaveContext.linkAge == 0 as libc::c_int {
        return strengthUpgrade
    } else if strengthUpgrade != 0 as libc::c_int {
        return PLAYER_STR_BRACELET as libc::c_int
    } else { return PLAYER_STR_NONE as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn Player_GetMask(mut globalCtx: *mut GlobalContext)
 -> u8_0 {
    let mut this: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    return (*this).currentMask;
}
#[no_mangle]
pub unsafe extern "C" fn Player_UnsetMask(mut globalCtx: *mut GlobalContext)
 -> *mut Player {
    let mut this: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    (*this).currentMask = PLAYER_MASK_NONE as libc::c_int as u8_0;
    return this;
}
#[no_mangle]
pub unsafe extern "C" fn Player_HasMirrorShieldEquipped(mut globalCtx:
                                                            *mut GlobalContext)
 -> s32 {
    let mut this: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    return ((*this).currentShield as libc::c_int ==
                PLAYER_SHIELD_MIRROR as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Player_HasMirrorShieldSetToDraw(mut globalCtx:
                                                             *mut GlobalContext)
 -> s32 {
    let mut this: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    return ((*this).rightHandType as libc::c_int == 10 as libc::c_int &&
                (*this).currentShield as libc::c_int ==
                    PLAYER_SHIELD_MIRROR as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Player_ActionToMagicSpell(mut this: *mut Player,
                                                   mut actionParam: s32)
 -> s32 {
    let mut magicSpell: s32 =
        actionParam - PLAYER_AP_MAGIC_SPELL_15 as libc::c_int;
    if magicSpell >= 0 as libc::c_int && magicSpell < 6 as libc::c_int {
        return magicSpell
    } else { return -(1 as libc::c_int) };
}
#[no_mangle]
pub unsafe extern "C" fn Player_HoldsHookshot(mut this: *mut Player) -> s32 {
    return ((*this).heldItemActionParam as libc::c_int ==
                PLAYER_AP_HOOKSHOT as libc::c_int ||
                (*this).heldItemActionParam as libc::c_int ==
                    PLAYER_AP_LONGSHOT as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_8008F128(mut this: *mut Player) -> s32 {
    return (Player_HoldsHookshot(this) != 0 && (*this).heldActor.is_null()) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Player_ActionToSword(mut actionParam: s32) -> s32 {
    let mut sword: s32 = actionParam - PLAYER_AP_FISHING_POLE as libc::c_int;
    if sword > 0 as libc::c_int && sword < 6 as libc::c_int {
        return sword
    } else { return 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn Player_GetSwordHeld(mut this: *mut Player) -> s32 {
    return Player_ActionToSword((*this).heldItemActionParam as s32);
}
#[no_mangle]
pub unsafe extern "C" fn Player_HoldsTwoHandedWeapon(mut this: *mut Player)
 -> s32 {
    if (*this).heldItemActionParam as libc::c_int >=
           PLAYER_AP_SWORD_BGS as libc::c_int &&
           (*this).heldItemActionParam as libc::c_int <=
               PLAYER_AP_HAMMER as libc::c_int {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn Player_HoldsBrokenKnife(mut this: *mut Player)
 -> s32 {
    return ((*this).heldItemActionParam as libc::c_int ==
                PLAYER_AP_SWORD_BGS as libc::c_int &&
                gSaveContext.swordHealth as libc::c_int as libc::c_float <=
                    0.0f32) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Player_ActionToBottle(mut this: *mut Player,
                                               mut actionParam: s32) -> s32 {
    let mut bottle: s32 = actionParam - PLAYER_AP_BOTTLE as libc::c_int;
    if bottle >= 0 as libc::c_int && bottle < 13 as libc::c_int {
        return bottle
    } else { return -(1 as libc::c_int) };
}
#[no_mangle]
pub unsafe extern "C" fn Player_GetBottleHeld(mut this: *mut Player) -> s32 {
    return Player_ActionToBottle(this, (*this).heldItemActionParam as s32);
}
#[no_mangle]
pub unsafe extern "C" fn Player_ActionToExplosive(mut this: *mut Player,
                                                  mut actionParam: s32)
 -> s32 {
    let mut explosive: s32 = actionParam - PLAYER_AP_BOMB as libc::c_int;
    if explosive >= 0 as libc::c_int && explosive < 2 as libc::c_int {
        return explosive
    } else { return -(1 as libc::c_int) };
}
#[no_mangle]
pub unsafe extern "C" fn Player_GetExplosiveHeld(mut this: *mut Player)
 -> s32 {
    return Player_ActionToExplosive(this, (*this).heldItemActionParam as s32);
}
#[no_mangle]
pub unsafe extern "C" fn func_8008F2BC(mut this: *mut Player,
                                       mut actionParam: s32) -> s32 {
    let mut sword: s32 = 0 as libc::c_int;
    if actionParam != PLAYER_AP_LAST_USED as libc::c_int {
        sword = actionParam - PLAYER_AP_SWORD_MASTER as libc::c_int;
        if sword < 0 as libc::c_int || sword >= 3 as libc::c_int {
            return -(1 as libc::c_int)
        }
    }
    return sword;
}
#[no_mangle]
pub unsafe extern "C" fn func_8008F2F8(mut globalCtx: *mut GlobalContext)
 -> s32 {
    let mut this: *mut Player =
        (*globalCtx).actorCtx.actorLists[ACTORCAT_PLAYER as libc::c_int as
                                             usize].head as *mut Player;
    let mut triggerEntry: *mut TextTriggerEntry = 0 as *mut TextTriggerEntry;
    let mut var: s32 = 0;
    if (*globalCtx).roomCtx.curRoom.unk_02 as libc::c_int == 3 as libc::c_int
       {
        // Room is hot
        var = 0 as libc::c_int
    } else if (*this).unk_840 as libc::c_int > 80 as libc::c_int &&
                  ((*this).currentBoots as libc::c_int ==
                       PLAYER_BOOTS_IRON as libc::c_int ||
                       (*this).unk_840 as libc::c_int >= 300 as libc::c_int) {
        // Deep underwater
        var =
            if (*this).currentBoots as libc::c_int ==
                   PLAYER_BOOTS_IRON as libc::c_int &&
                   (*this).actor.bgCheckFlags as libc::c_int &
                       1 as libc::c_int != 0 {
                1 as libc::c_int
            } else { 3 as libc::c_int }
    } else if (*this).stateFlags1 & 0x8000000 as libc::c_int as libc::c_uint
                  != 0 {
        // Swimming
        var = 2 as libc::c_int
    } else { return 0 as libc::c_int }
    // Trigger general textboxes under certain conditions, like "It's so hot in here!"
    if Player_InCsMode(globalCtx) == 0 {
        triggerEntry =
            &mut *sTextTriggers.as_mut_ptr().offset(var as isize) as
                *mut TextTriggerEntry;
        if (*triggerEntry).flag as libc::c_int != 0 as libc::c_int &&
               gSaveContext.textTriggerFlags as libc::c_int &
                   (*triggerEntry).flag as libc::c_int == 0 &&
               (var == 0 as libc::c_int &&
                    (*this).currentTunic as libc::c_int !=
                        PLAYER_TUNIC_GORON as libc::c_int ||
                    (var == 1 as libc::c_int || var == 3 as libc::c_int) &&
                        (*this).currentBoots as libc::c_int ==
                            PLAYER_BOOTS_IRON as libc::c_int &&
                        (*this).currentTunic as libc::c_int !=
                            PLAYER_TUNIC_ZORA as libc::c_int) {
            Message_StartTextbox(globalCtx, (*triggerEntry).textId,
                                 0 as *mut Actor);
            gSaveContext.textTriggerFlags =
                (gSaveContext.textTriggerFlags as libc::c_int |
                     (*triggerEntry).flag as libc::c_int) as u8_0
        }
    }
    return var + 1 as libc::c_int;
}
#[no_mangle]
pub static mut sEyeMouthIndexes: [[u8_0; 2]; 16] =
    [[0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0],
     [1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0],
     [2 as libc::c_int as u8_0, 0 as libc::c_int as u8_0],
     [0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0],
     [1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0],
     [2 as libc::c_int as u8_0, 0 as libc::c_int as u8_0],
     [4 as libc::c_int as u8_0, 0 as libc::c_int as u8_0],
     [5 as libc::c_int as u8_0, 1 as libc::c_int as u8_0],
     [7 as libc::c_int as u8_0, 2 as libc::c_int as u8_0],
     [0 as libc::c_int as u8_0, 2 as libc::c_int as u8_0],
     [3 as libc::c_int as u8_0, 0 as libc::c_int as u8_0],
     [4 as libc::c_int as u8_0, 0 as libc::c_int as u8_0],
     [2 as libc::c_int as u8_0, 2 as libc::c_int as u8_0],
     [1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0],
     [0 as libc::c_int as u8_0, 2 as libc::c_int as u8_0],
     [0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0]];
/* *
 * Link's eye and mouth textures are placed at the exact same place in adult and child Link's respective object files.
 * This allows the array to only contain the symbols for one file and have it apply to both. This is a problem for
 * shiftability, and changes will need to be made in the code to account for this in a modding scenario. The symbols
 * from adult Link's object are used here.
 */
#[no_mangle]
pub static mut sEyeTextures: [*mut libc::c_void; 8] =
    unsafe {
        [0x6000000 as libc::c_int as *mut libc::c_void,
         0x6000800 as libc::c_int as *mut libc::c_void,
         0x6001000 as libc::c_int as *mut libc::c_void,
         0x6001800 as libc::c_int as *mut libc::c_void,
         0x6002000 as libc::c_int as *mut libc::c_void,
         0x6002800 as libc::c_int as *mut libc::c_void,
         0x6003000 as libc::c_int as *mut libc::c_void,
         0x6003800 as libc::c_int as *mut libc::c_void]
    };
#[no_mangle]
pub static mut sMouthTextures: [*mut libc::c_void; 4] =
    unsafe {
        [0x6004000 as libc::c_int as *mut libc::c_void,
         0x6004400 as libc::c_int as *mut libc::c_void,
         0x6004800 as libc::c_int as *mut libc::c_void,
         0x6004c00 as libc::c_int as *mut libc::c_void]
    };
#[no_mangle]
pub static mut sTunicColors: [Color_RGB8; 3] =
    [{
         let mut init =
             Color_RGB8{r: 30 as libc::c_int as u8_0,
                        g: 105 as libc::c_int as u8_0,
                        b: 27 as libc::c_int as u8_0,};
         init
     },
     {
         let mut init =
             Color_RGB8{r: 100 as libc::c_int as u8_0,
                        g: 20 as libc::c_int as u8_0,
                        b: 0 as libc::c_int as u8_0,};
         init
     },
     {
         let mut init =
             Color_RGB8{r: 0 as libc::c_int as u8_0,
                        g: 60 as libc::c_int as u8_0,
                        b: 100 as libc::c_int as u8_0,};
         init
     }];
#[no_mangle]
pub static mut sGauntletColors: [Color_RGB8; 2] =
    [{
         let mut init =
             Color_RGB8{r: 255 as libc::c_int as u8_0,
                        g: 255 as libc::c_int as u8_0,
                        b: 255 as libc::c_int as u8_0,};
         init
     },
     {
         let mut init =
             Color_RGB8{r: 254 as libc::c_int as u8_0,
                        g: 207 as libc::c_int as u8_0,
                        b: 15 as libc::c_int as u8_0,};
         init
     }];
#[no_mangle]
pub static mut sBootDListGroups: [[*mut Gfx; 2]; 2] =
    unsafe {
        [[0x6025918 as libc::c_int as *mut Gfx,
          0x6025a60 as libc::c_int as *mut Gfx],
         [0x6025ba8 as libc::c_int as *mut Gfx,
          0x6025db0 as libc::c_int as *mut Gfx]]
    };
#[no_mangle]
pub unsafe extern "C" fn func_8008F470(mut globalCtx: *mut GlobalContext,
                                       mut skeleton: *mut *mut libc::c_void,
                                       mut jointTable: *mut Vec3s,
                                       mut dListCount: s32, mut lod: s32,
                                       mut tunic: s32, mut boots: s32,
                                       mut face: s32,
                                       mut overrideLimbDraw:
                                           OverrideLimbDrawOpa,
                                       mut postLimbDraw: PostLimbDrawOpa,
                                       mut data: *mut libc::c_void) {
    let mut color: *mut Color_RGB8 = 0 as *mut Color_RGB8;
    let mut eyeIndex: s32 =
        ((*jointTable.offset(22 as libc::c_int as isize)).x as libc::c_int &
             0xf as libc::c_int) - 1 as libc::c_int;
    let mut mouthIndex: s32 =
        ((*jointTable.offset(22 as libc::c_int as isize)).x as libc::c_int >>
             4 as libc::c_int) - 1 as libc::c_int;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_player_lib.c\x00" as *const u8 as
                        *const libc::c_char, 1721 as libc::c_int);
    if eyeIndex < 0 as libc::c_int {
        eyeIndex =
            sEyeMouthIndexes[face as usize][0 as libc::c_int as usize] as s32
    }
    let fresh0 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
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
        gSegments[((sEyeTextures[eyeIndex as usize] as u32_0) <<
                       4 as libc::c_int >> 28 as libc::c_int) as
                      usize].wrapping_add(sEyeTextures[eyeIndex as usize] as
                                              u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as libc::c_uint;
    if mouthIndex < 0 as libc::c_int {
        mouthIndex =
            sEyeMouthIndexes[face as usize][1 as libc::c_int as usize] as s32
    }
    let fresh1 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
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
        gSegments[((sMouthTextures[mouthIndex as usize] as u32_0) <<
                       4 as libc::c_int >> 28 as libc::c_int) as
                      usize].wrapping_add(sMouthTextures[mouthIndex as usize]
                                              as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as libc::c_uint;
    color =
        &mut *sTunicColors.as_mut_ptr().offset(tunic as isize) as
            *mut Color_RGB8;
    let fresh2 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_1: *mut Gfx = fresh2;
    (*_g_1).words.w0 =
        (0xfb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_1).words.w1 =
        ((*color).r as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((*color).g as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((*color).b as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    sDListsLodOffset = lod * 2 as libc::c_int;
    SkelAnime_DrawFlexLod(globalCtx, skeleton, jointTable, dListCount,
                          overrideLimbDraw, postLimbDraw, data, lod);
    if overrideLimbDraw !=
           Some(func_800902F0 as
                    unsafe extern "C" fn(_: *mut GlobalContext, _: s32,
                                         _: *mut *mut Gfx, _: *mut Vec3f,
                                         _: *mut Vec3s, _: *mut libc::c_void)
                        -> s32) &&
           overrideLimbDraw !=
               Some(func_80090440 as
                        unsafe extern "C" fn(_: *mut GlobalContext, _: s32,
                                             _: *mut *mut Gfx, _: *mut Vec3f,
                                             _: *mut Vec3s,
                                             _: *mut libc::c_void) -> s32) &&
           gSaveContext.gameMode != 3 as libc::c_int {
        if gSaveContext.linkAge == 0 as libc::c_int {
            let mut strengthUpgrade: s32 =
                (gSaveContext.inventory.upgrades &
                     gUpgradeMasks[UPG_STRENGTH as libc::c_int as usize]) as
                    s32 >>
                    gUpgradeShifts[UPG_STRENGTH as libc::c_int as usize] as
                        libc::c_int;
            if strengthUpgrade >= 2 as libc::c_int {
                // silver or gold gauntlets
                let fresh3 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_2: *mut Gfx = fresh3;
                (*_g_2).words.w0 =
                    (0xe7 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int;
                (*_g_2).words.w1 = 0 as libc::c_int as libc::c_uint;
                color =
                    &mut *sGauntletColors.as_mut_ptr().offset((strengthUpgrade
                                                                   -
                                                                   2 as
                                                                       libc::c_int)
                                                                  as isize) as
                        *mut Color_RGB8;
                let fresh4 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_3: *mut Gfx = fresh4;
                (*_g_3).words.w0 =
                    (0xfb as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int;
                (*_g_3).words.w1 =
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
                        (0 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                let fresh5 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_4: *mut Gfx = fresh5;
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
                (*_g_4).words.w1 = D_06025218.as_mut_ptr() as libc::c_uint;
                let fresh6 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
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
                (*_g_5).words.w1 = D_06025598.as_mut_ptr() as libc::c_uint;
                let fresh7 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_6: *mut Gfx = fresh7;
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
                (*_g_6).words.w1 =
                    if D_80160014 == 0 as libc::c_int {
                        D_060252D8.as_mut_ptr()
                    } else { D_06025438.as_mut_ptr() } as libc::c_uint;
                let fresh8 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_7: *mut Gfx = fresh8;
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
                    if D_80160018 == 8 as libc::c_int {
                        D_06025658.as_mut_ptr()
                    } else { D_060257B8.as_mut_ptr() } as libc::c_uint
            }
            if boots != 0 as libc::c_int {
                let mut bootDLists: *mut *mut Gfx =
                    sBootDListGroups[(boots - 1 as libc::c_int) as
                                         usize].as_mut_ptr();
                let fresh9 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_8: *mut Gfx = fresh9;
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
                    *bootDLists.offset(0 as libc::c_int as isize) as
                        libc::c_uint;
                let fresh10 = (*__gfxCtx).polyOpa.p;
                (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
                let mut _g_9: *mut Gfx = fresh10;
                (*_g_9).words.w0 =
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
                (*_g_9).words.w1 =
                    *bootDLists.offset(1 as libc::c_int as isize) as
                        libc::c_uint
            }
        } else if Player_GetStrength() > PLAYER_STR_NONE as libc::c_int {
            let fresh11 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_10: *mut Gfx = fresh11;
            (*_g_10).words.w0 =
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
            (*_g_10).words.w1 =
                gLinkChildGoronBraceletDL.as_mut_ptr() as libc::c_uint
        }
    }
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_player_lib.c\x00" as *const u8 as
                         *const libc::c_char, 1803 as libc::c_int);
}
#[no_mangle]
pub static mut D_8012602C: Vec3f =
    { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
#[no_mangle]
pub static mut D_80126038: [Vec3f; 2] =
    [{ let mut init = Vec3f{x: 1304.0f32, y: 0.0f32, z: 0.0f32,}; init },
     { let mut init = Vec3f{x: 695.0f32, y: 0.0f32, z: 0.0f32,}; init }];
#[no_mangle]
pub static mut D_80126050: [f32_0; 2] = [1265.0f32, 826.0f32];
#[no_mangle]
pub static mut D_80126058: [f32_0; 2] =
    [13.04f32 * 13.04f32, 6.95f32 * 6.95f32];
#[no_mangle]
pub static mut D_80126060: [f32_0; 2] = [10.019104f32, -19.925102f32];
#[no_mangle]
pub static mut D_80126068: [f32_0; 2] = [5.0f32, 3.0f32];
#[no_mangle]
pub static mut D_80126070: Vec3f =
    { let mut init = Vec3f{x: 0.0f32, y: -300.0f32, z: 0.0f32,}; init };
#[no_mangle]
pub unsafe extern "C" fn func_8008F87C(mut globalCtx: *mut GlobalContext,
                                       mut this: *mut Player,
                                       mut skelAnime: *mut SkelAnime,
                                       mut pos: *mut Vec3f,
                                       mut rot: *mut Vec3s,
                                       mut thighLimbIndex: s32,
                                       mut shinLimbIndex: s32,
                                       mut footLimbIndex: s32) {
    let mut spA4: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp98: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut footprintPos: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp88: *mut CollisionPoly = 0 as *mut CollisionPoly;
    let mut sp84: s32 = 0;
    let mut sp80: f32_0 = 0.;
    let mut sp7C: f32_0 = 0.;
    let mut sp78: f32_0 = 0.;
    let mut sp74: f32_0 = 0.;
    let mut sp70: f32_0 = 0.;
    let mut sp6C: f32_0 = 0.;
    let mut sp68: f32_0 = 0.;
    let mut sp64: f32_0 = 0.;
    let mut sp60: f32_0 = 0.;
    let mut sp5C: f32_0 = 0.;
    let mut sp58: f32_0 = 0.;
    let mut sp54: f32_0 = 0.;
    let mut sp50: f32_0 = 0.;
    let mut temp1: s16 = 0;
    let mut temp2: s16 = 0;
    let mut temp3: s32 = 0;
    if (*this).actor.scale.y >= 0.0f32 &&
           (*this).stateFlags1 & 0x80 as libc::c_int as libc::c_uint == 0 &&
           Player_ActionToMagicSpell(this, (*this).itemActionParam as s32) <
               0 as libc::c_int {
        let mut pad: s32 = 0;
        sp7C = D_80126058[gSaveContext.linkAge as usize];
        sp78 = D_80126060[gSaveContext.linkAge as usize];
        sp74 = D_80126068[gSaveContext.linkAge as usize] - (*this).unk_6C4;
        Matrix_Push();
        Matrix_TranslateRotateZYX(pos, rot);
        Matrix_MultVec3f(&mut D_8012602C, &mut spA4);
        Matrix_TranslateRotateZYX(&mut *D_80126038.as_mut_ptr().offset(gSaveContext.linkAge
                                                                           as
                                                                           isize),
                                  &mut *(*skelAnime).jointTable.offset(shinLimbIndex
                                                                           as
                                                                           isize));
        Matrix_Translate(D_80126050[gSaveContext.linkAge as usize], 0.0f32,
                         0.0f32, MTXMODE_APPLY as libc::c_int as u8_0);
        Matrix_MultVec3f(&mut D_8012602C, &mut sp98);
        Matrix_MultVec3f(&mut D_80126070, &mut footprintPos);
        Matrix_Pop();
        footprintPos.y += 15.0f32;
        sp80 =
            BgCheck_EntityRaycastFloor4(&mut (*globalCtx).colCtx, &mut sp88,
                                        &mut sp84, &mut (*this).actor,
                                        &mut footprintPos) + sp74;
        if sp98.y < sp80 {
            sp70 = sp98.x - spA4.x;
            sp6C = sp98.y - spA4.y;
            sp68 = sp98.z - spA4.z;
            sp64 = sqrtf(sp70 * sp70 + sp6C * sp6C + sp68 * sp68);
            sp60 = (sp64 * sp64 + sp78) / (2.0f32 * sp64);
            sp58 = sp7C - sp60 * sp60;
            sp58 = if sp7C < sp60 * sp60 { 0.0f32 } else { sqrtf(sp58) };
            sp54 = Math_FAtan2F(sp58, sp60);
            sp6C = sp80 - spA4.y;
            sp64 = sqrtf(sp70 * sp70 + sp6C * sp6C + sp68 * sp68);
            sp60 = (sp64 * sp64 + sp78) / (2.0f32 * sp64);
            sp5C = sp64 - sp60;
            sp58 = sp7C - sp60 * sp60;
            sp58 = if sp7C < sp60 * sp60 { 0.0f32 } else { sqrtf(sp58) };
            sp50 = Math_FAtan2F(sp58, sp60);
            temp1 =
                ((3.14159265358979323846f32 -
                      (Math_FAtan2F(sp5C, sp58) +
                           (3.14159265358979323846f32 /
                                2 as libc::c_int as libc::c_float - sp50))) *
                     (0x8000 as libc::c_int as libc::c_float /
                          3.14159265358979323846f32)) as s16;
            temp1 =
                (temp1 as libc::c_int -
                     (*(*skelAnime).jointTable.offset(shinLimbIndex as
                                                          isize)).z as
                         libc::c_int) as s16;
            if (((if (*(*skelAnime).jointTable.offset(shinLimbIndex as
                                                          isize)).x as
                         libc::c_int >= 0 as libc::c_int {
                      (*(*skelAnime).jointTable.offset(shinLimbIndex as
                                                           isize)).x as
                          libc::c_int
                  } else {
                      -((*(*skelAnime).jointTable.offset(shinLimbIndex as
                                                             isize)).x as
                            libc::c_int)
                  }) +
                     (if (*(*skelAnime).jointTable.offset(shinLimbIndex as
                                                              isize)).y as
                             libc::c_int >= 0 as libc::c_int {
                          (*(*skelAnime).jointTable.offset(shinLimbIndex as
                                                               isize)).y as
                              libc::c_int
                      } else {
                          -((*(*skelAnime).jointTable.offset(shinLimbIndex as
                                                                 isize)).y as
                                libc::c_int)
                      })) as s16 as libc::c_int) < 0 as libc::c_int {
                temp1 = (temp1 as libc::c_int + 0x8000 as libc::c_int) as s16
            }
            temp2 =
                ((sp50 - sp54) *
                     (0x8000 as libc::c_int as libc::c_float /
                          3.14159265358979323846f32)) as s16;
            (*rot).z =
                ((*rot).z as libc::c_int - temp2 as libc::c_int) as s16;
            (*(*skelAnime).jointTable.offset(thighLimbIndex as isize)).z =
                ((*(*skelAnime).jointTable.offset(thighLimbIndex as isize)).z
                     as libc::c_int - temp2 as libc::c_int) as s16;
            (*(*skelAnime).jointTable.offset(shinLimbIndex as isize)).z =
                ((*(*skelAnime).jointTable.offset(shinLimbIndex as isize)).z
                     as libc::c_int + temp1 as libc::c_int) as s16;
            (*(*skelAnime).jointTable.offset(footLimbIndex as isize)).z =
                ((*(*skelAnime).jointTable.offset(footLimbIndex as isize)).z
                     as libc::c_int + temp2 as libc::c_int -
                     temp1 as libc::c_int) as s16;
            temp3 =
                func_80041D4C(&mut (*globalCtx).colCtx, sp88, sp84) as s32;
            if temp3 >= 2 as libc::c_int && temp3 < 4 as libc::c_int &&
                   SurfaceType_IsWallDamage(&mut (*globalCtx).colCtx, sp88,
                                            sp84) == 0 {
                footprintPos.y = sp80;
                EffectSsGFire_Spawn(globalCtx, &mut footprintPos);
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_8008FCC8(mut globalCtx: *mut GlobalContext,
                                       mut limbIndex: s32,
                                       mut dList: *mut *mut Gfx,
                                       mut pos: *mut Vec3f,
                                       mut rot: *mut Vec3s,
                                       mut thisx: *mut libc::c_void) -> s32 {
    let mut this: *mut Player = thisx as *mut Player;
    if limbIndex == PLAYER_LIMB_ROOT as libc::c_int {
        D_80160014 = (*this).leftHandType as s32;
        D_80160018 = (*this).rightHandType as s32;
        D_80160000 =
            &mut (*(*this).swordInfo.as_mut_ptr().offset(2 as libc::c_int as
                                                             isize)).base;
        if !(gSaveContext.linkAge == 0 as libc::c_int) {
            if (*this).skelAnime.moveFlags as libc::c_int & 4 as libc::c_int
                   == 0 ||
                   (*this).skelAnime.moveFlags as libc::c_int &
                       1 as libc::c_int != 0 {
                (*pos).x *= 0.64f32;
                (*pos).z *= 0.64f32
            }
            if (*this).skelAnime.moveFlags as libc::c_int & 4 as libc::c_int
                   == 0 ||
                   (*this).skelAnime.moveFlags as libc::c_int &
                       2 as libc::c_int != 0 {
                (*pos).y *= 0.64f32
            }
        }
        (*pos).y -= (*this).unk_6C4;
        if (*this).unk_6C2 as libc::c_int != 0 as libc::c_int {
            Matrix_Translate((*pos).x,
                             (Math_CosS((*this).unk_6C2) - 1.0f32) * 200.0f32
                                 + (*pos).y, (*pos).z,
                             MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_RotateX((*this).unk_6C2 as libc::c_int as libc::c_float *
                               (3.14159265358979323846f32 /
                                    0x8000 as libc::c_int as libc::c_float),
                           MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_RotateZYX((*rot).x, (*rot).y, (*rot).z,
                             MTXMODE_APPLY as libc::c_int as u8_0);
            (*pos).z = 0.0f32;
            (*pos).y = (*pos).z;
            (*pos).x = (*pos).y;
            (*rot).z = 0 as libc::c_int as s16;
            (*rot).y = (*rot).z;
            (*rot).x = (*rot).y
        }
    } else {
        if !(*dList).is_null() { D_80160000 = D_80160000.offset(1) }
        if limbIndex == PLAYER_LIMB_HEAD as libc::c_int {
            (*rot).x =
                ((*rot).x as libc::c_int + (*this).unk_6BA as libc::c_int) as
                    s16;
            (*rot).y =
                ((*rot).y as libc::c_int - (*this).unk_6B8 as libc::c_int) as
                    s16;
            (*rot).z =
                ((*rot).z as libc::c_int + (*this).unk_6B6 as libc::c_int) as
                    s16
        } else if limbIndex == PLAYER_LIMB_UPPER as libc::c_int {
            if (*this).unk_6B0 as libc::c_int != 0 as libc::c_int {
                Matrix_RotateZ(0x44c as libc::c_int as libc::c_float *
                                   (3.14159265358979323846f32 /
                                        0x8000 as libc::c_int as
                                            libc::c_float),
                               MTXMODE_APPLY as libc::c_int as u8_0);
                Matrix_RotateY((*this).unk_6B0 as libc::c_int as libc::c_float
                                   *
                                   (3.14159265358979323846f32 /
                                        0x8000 as libc::c_int as
                                            libc::c_float),
                               MTXMODE_APPLY as libc::c_int as u8_0);
            }
            if (*this).unk_6BE as libc::c_int != 0 as libc::c_int {
                Matrix_RotateY((*this).unk_6BE as libc::c_int as libc::c_float
                                   *
                                   (3.14159265358979323846f32 /
                                        0x8000 as libc::c_int as
                                            libc::c_float),
                               MTXMODE_APPLY as libc::c_int as u8_0);
            }
            if (*this).unk_6BC as libc::c_int != 0 as libc::c_int {
                Matrix_RotateX((*this).unk_6BC as libc::c_int as libc::c_float
                                   *
                                   (3.14159265358979323846f32 /
                                        0x8000 as libc::c_int as
                                            libc::c_float),
                               MTXMODE_APPLY as libc::c_int as u8_0);
            }
            if (*this).unk_6C0 as libc::c_int != 0 as libc::c_int {
                Matrix_RotateZ((*this).unk_6C0 as libc::c_int as libc::c_float
                                   *
                                   (3.14159265358979323846f32 /
                                        0x8000 as libc::c_int as
                                            libc::c_float),
                               MTXMODE_APPLY as libc::c_int as u8_0);
            }
        } else if limbIndex == PLAYER_LIMB_L_THIGH as libc::c_int {
            func_8008F87C(globalCtx, this, &mut (*this).skelAnime, pos, rot,
                          PLAYER_LIMB_L_THIGH as libc::c_int,
                          PLAYER_LIMB_L_SHIN as libc::c_int,
                          PLAYER_LIMB_L_FOOT as libc::c_int);
        } else if limbIndex == PLAYER_LIMB_R_THIGH as libc::c_int {
            func_8008F87C(globalCtx, this, &mut (*this).skelAnime, pos, rot,
                          PLAYER_LIMB_R_THIGH as libc::c_int,
                          PLAYER_LIMB_R_SHIN as libc::c_int,
                          PLAYER_LIMB_R_FOOT as libc::c_int);
            return 0 as libc::c_int
        } else { return 0 as libc::c_int }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_80090014(mut globalCtx: *mut GlobalContext,
                                       mut limbIndex: s32,
                                       mut dList: *mut *mut Gfx,
                                       mut pos: *mut Vec3f,
                                       mut rot: *mut Vec3s,
                                       mut thisx: *mut libc::c_void) -> s32 {
    let mut this: *mut Player = thisx as *mut Player;
    if func_8008FCC8(globalCtx, limbIndex, dList, pos, rot, thisx) == 0 {
        if limbIndex == PLAYER_LIMB_L_HAND as libc::c_int {
            let mut dLists: *mut *mut Gfx = (*this).leftHandDLists;
            if D_80160014 == 4 as libc::c_int &&
                   gSaveContext.swordHealth as libc::c_int as libc::c_float <=
                       0.0f32 {
                dLists = dLists.offset(4 as libc::c_int as isize)
            } else if D_80160014 == 6 as libc::c_int &&
                          (*this).stateFlags1 &
                              0x2000000 as libc::c_int as libc::c_uint != 0 {
                dLists =
                    &mut *D_80125E08.as_mut_ptr().offset(gSaveContext.linkAge
                                                             as isize) as
                        *mut *mut Gfx;
                D_80160014 = 0 as libc::c_int
            } else if (*this).leftHandType as libc::c_int == 0 as libc::c_int
                          && (*this).actor.speedXZ > 2.0f32 &&
                          (*this).stateFlags1 &
                              0x8000000 as libc::c_int as libc::c_uint == 0 {
                dLists =
                    &mut *D_80125E18.as_mut_ptr().offset(gSaveContext.linkAge
                                                             as isize) as
                        *mut *mut Gfx;
                D_80160014 = 1 as libc::c_int
            }
            *dList = *dLists.offset(sDListsLodOffset as isize)
        } else if limbIndex == PLAYER_LIMB_R_HAND as libc::c_int {
            let mut dLists_0: *mut *mut Gfx = (*this).rightHandDLists;
            if D_80160018 == 10 as libc::c_int {
                dLists_0 =
                    dLists_0.offset(((*this).currentShield as libc::c_int *
                                         4 as libc::c_int) as isize)
            } else if (*this).rightHandType as libc::c_int == 8 as libc::c_int
                          && (*this).actor.speedXZ > 2.0f32 &&
                          (*this).stateFlags1 &
                              0x8000000 as libc::c_int as libc::c_uint == 0 {
                dLists_0 =
                    &mut *D_80125E58.as_mut_ptr().offset(gSaveContext.linkAge
                                                             as isize) as
                        *mut *mut Gfx;
                D_80160018 = 9 as libc::c_int
            }
            *dList = *dLists_0.offset(sDListsLodOffset as isize)
        } else if limbIndex == PLAYER_LIMB_SHEATH as libc::c_int {
            let mut dLists_1: *mut *mut Gfx = (*this).sheathDLists;
            if (*this).sheathType as libc::c_int == 18 as libc::c_int ||
                   (*this).sheathType as libc::c_int == 19 as libc::c_int {
                dLists_1 =
                    dLists_1.offset(((*this).currentShield as libc::c_int *
                                         4 as libc::c_int) as isize);
                if !(gSaveContext.linkAge == 0 as libc::c_int) &&
                       ((*this).currentShield as libc::c_int) <
                           PLAYER_SHIELD_HYLIAN as libc::c_int &&
                       gSaveContext.equips.buttonItems[0 as libc::c_int as
                                                           usize] as
                           libc::c_int != ITEM_SWORD_KOKIRI as libc::c_int {
                    dLists_1 = dLists_1.offset(16 as libc::c_int as isize)
                }
            } else if !(gSaveContext.linkAge == 0 as libc::c_int) &&
                          ((*this).sheathType as libc::c_int ==
                               16 as libc::c_int ||
                               (*this).sheathType as libc::c_int ==
                                   17 as libc::c_int) &&
                          gSaveContext.equips.buttonItems[0 as libc::c_int as
                                                              usize] as
                              libc::c_int != ITEM_SWORD_KOKIRI as libc::c_int
             {
                dLists_1 = D_80125D68.as_mut_ptr()
            }
            *dList = *dLists_1.offset(sDListsLodOffset as isize)
        } else if limbIndex == PLAYER_LIMB_WAIST as libc::c_int {
            *dList = *(*this).waistDLists.offset(sDListsLodOffset as isize)
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_800902F0(mut globalCtx: *mut GlobalContext,
                                       mut limbIndex: s32,
                                       mut dList: *mut *mut Gfx,
                                       mut pos: *mut Vec3f,
                                       mut rot: *mut Vec3s,
                                       mut thisx: *mut libc::c_void) -> s32 {
    let mut this: *mut Player = thisx as *mut Player;
    if func_8008FCC8(globalCtx, limbIndex, dList, pos, rot, thisx) == 0 {
        if (*this).unk_6AD as libc::c_int != 2 as libc::c_int {
            *dList = 0 as *mut Gfx
        } else if limbIndex == PLAYER_LIMB_L_FOREARM as libc::c_int {
            *dList = D_80125F18[gSaveContext.linkAge as usize]
        } else if limbIndex == PLAYER_LIMB_L_HAND as libc::c_int {
            *dList = D_80125F20[gSaveContext.linkAge as usize]
        } else if limbIndex == PLAYER_LIMB_R_SHOULDER as libc::c_int {
            *dList = D_80125F28[gSaveContext.linkAge as usize]
        } else if limbIndex == PLAYER_LIMB_R_FOREARM as libc::c_int {
            *dList = D_80125F30[gSaveContext.linkAge as usize]
        } else if limbIndex == PLAYER_LIMB_R_HAND as libc::c_int {
            *dList =
                if Player_HoldsHookshot(this) != 0 {
                    D_0602A738.as_mut_ptr()
                } else { D_80125F38[gSaveContext.linkAge as usize] }
        } else { *dList = 0 as *mut Gfx }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_80090440(mut globalCtx: *mut GlobalContext,
                                       mut limbIndex: s32,
                                       mut dList: *mut *mut Gfx,
                                       mut pos: *mut Vec3f,
                                       mut rot: *mut Vec3s,
                                       mut thisx: *mut libc::c_void) -> s32 {
    if func_8008FCC8(globalCtx, limbIndex, dList, pos, rot, thisx) == 0 {
        *dList = 0 as *mut Gfx
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_80090480(mut globalCtx: *mut GlobalContext,
                                       mut collider: *mut ColliderQuad,
                                       mut weaponInfo: *mut WeaponInfo,
                                       mut newTip: *mut Vec3f,
                                       mut newBase: *mut Vec3f) -> u8_0 {
    if (*weaponInfo).active == 0 as libc::c_int {
        if !collider.is_null() {
            Collider_ResetQuadAT(globalCtx, &mut (*collider).base);
        }
        Math_Vec3f_Copy(&mut (*weaponInfo).tip, newTip);
        Math_Vec3f_Copy(&mut (*weaponInfo).base, newBase);
        (*weaponInfo).active = 1 as libc::c_int;
        return 1 as libc::c_int as u8_0
    } else if (*weaponInfo).tip.x == (*newTip).x &&
                  (*weaponInfo).tip.y == (*newTip).y &&
                  (*weaponInfo).tip.z == (*newTip).z &&
                  (*weaponInfo).base.x == (*newBase).x &&
                  (*weaponInfo).base.y == (*newBase).y &&
                  (*weaponInfo).base.z == (*newBase).z {
        if !collider.is_null() {
            Collider_ResetQuadAT(globalCtx, &mut (*collider).base);
        }
        return 0 as libc::c_int as u8_0
    } else {
        if !collider.is_null() {
            Collider_SetQuadVertices(collider, newBase, newTip,
                                     &mut (*weaponInfo).base,
                                     &mut (*weaponInfo).tip);
            CollisionCheck_SetAT(globalCtx, &mut (*globalCtx).colChkCtx,
                                 &mut (*collider).base);
        }
        Math_Vec3f_Copy(&mut (*weaponInfo).base, newBase);
        Math_Vec3f_Copy(&mut (*weaponInfo).tip, newTip);
        (*weaponInfo).active = 1 as libc::c_int;
        return 1 as libc::c_int as u8_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_80090604(mut globalCtx: *mut GlobalContext,
                                       mut this: *mut Player,
                                       mut collider: *mut ColliderQuad,
                                       mut quadSrc: *mut Vec3f) {
    static mut shieldColTypes: [u8_0; 4] =
        [COLTYPE_METAL as libc::c_int as u8_0,
         COLTYPE_WOOD as libc::c_int as u8_0,
         COLTYPE_METAL as libc::c_int as u8_0,
         COLTYPE_METAL as libc::c_int as u8_0];
    if (*this).stateFlags1 & 0x400000 as libc::c_int as libc::c_uint != 0 {
        let mut quadDest: [Vec3f; 4] = [Vec3f{x: 0., y: 0., z: 0.,}; 4];
        (*this).shieldQuad.base.colType =
            shieldColTypes[(*this).currentShield as usize];
        Matrix_MultVec3f(&mut *quadSrc.offset(0 as libc::c_int as isize),
                         &mut *quadDest.as_mut_ptr().offset(0 as libc::c_int
                                                                as isize));
        Matrix_MultVec3f(&mut *quadSrc.offset(1 as libc::c_int as isize),
                         &mut *quadDest.as_mut_ptr().offset(1 as libc::c_int
                                                                as isize));
        Matrix_MultVec3f(&mut *quadSrc.offset(2 as libc::c_int as isize),
                         &mut *quadDest.as_mut_ptr().offset(2 as libc::c_int
                                                                as isize));
        Matrix_MultVec3f(&mut *quadSrc.offset(3 as libc::c_int as isize),
                         &mut *quadDest.as_mut_ptr().offset(3 as libc::c_int
                                                                as isize));
        Collider_SetQuadVertices(collider,
                                 &mut *quadDest.as_mut_ptr().offset(0 as
                                                                        libc::c_int
                                                                        as
                                                                        isize),
                                 &mut *quadDest.as_mut_ptr().offset(1 as
                                                                        libc::c_int
                                                                        as
                                                                        isize),
                                 &mut *quadDest.as_mut_ptr().offset(2 as
                                                                        libc::c_int
                                                                        as
                                                                        isize),
                                 &mut *quadDest.as_mut_ptr().offset(3 as
                                                                        libc::c_int
                                                                        as
                                                                        isize));
        CollisionCheck_SetAC(globalCtx, &mut (*globalCtx).colChkCtx,
                             &mut (*collider).base);
        CollisionCheck_SetAT(globalCtx, &mut (*globalCtx).colChkCtx,
                             &mut (*collider).base);
    };
}
#[no_mangle]
pub static mut D_80126080: Vec3f =
    { let mut init = Vec3f{x: 5000.0f32, y: 400.0f32, z: 0.0f32,}; init };
#[no_mangle]
pub static mut D_8012608C: Vec3f =
    { let mut init = Vec3f{x: 5000.0f32, y: -400.0f32, z: 1000.0f32,}; init };
#[no_mangle]
pub static mut D_80126098: Vec3f =
    {
        let mut init = Vec3f{x: 5000.0f32, y: 1400.0f32, z: -1000.0f32,};
        init
    };
#[no_mangle]
pub static mut D_801260A4: [Vec3f; 3] =
    [{ let mut init = Vec3f{x: 0.0f32, y: 400.0f32, z: 0.0f32,}; init },
     { let mut init = Vec3f{x: 0.0f32, y: 1400.0f32, z: -1000.0f32,}; init },
     { let mut init = Vec3f{x: 0.0f32, y: -400.0f32, z: 1000.0f32,}; init }];
#[no_mangle]
pub unsafe extern "C" fn func_800906D4(mut globalCtx: *mut GlobalContext,
                                       mut this: *mut Player,
                                       mut newTipPos: *mut Vec3f) {
    let mut newBasePos: [Vec3f; 3] = [Vec3f{x: 0., y: 0., z: 0.,}; 3];
    Matrix_MultVec3f(&mut *D_801260A4.as_mut_ptr().offset(0 as libc::c_int as
                                                              isize),
                     &mut *newBasePos.as_mut_ptr().offset(0 as libc::c_int as
                                                              isize));
    Matrix_MultVec3f(&mut *D_801260A4.as_mut_ptr().offset(1 as libc::c_int as
                                                              isize),
                     &mut *newBasePos.as_mut_ptr().offset(1 as libc::c_int as
                                                              isize));
    Matrix_MultVec3f(&mut *D_801260A4.as_mut_ptr().offset(2 as libc::c_int as
                                                              isize),
                     &mut *newBasePos.as_mut_ptr().offset(2 as libc::c_int as
                                                              isize));
    if func_80090480(globalCtx, 0 as *mut ColliderQuad,
                     &mut *(*this).swordInfo.as_mut_ptr().offset(0 as
                                                                     libc::c_int
                                                                     as
                                                                     isize),
                     &mut *newTipPos.offset(0 as libc::c_int as isize),
                     &mut *newBasePos.as_mut_ptr().offset(0 as libc::c_int as
                                                              isize)) as
           libc::c_int != 0 &&
           (*this).stateFlags1 & 0x400000 as libc::c_int as libc::c_uint == 0
       {
        EffectBlure_AddVertex(Effect_GetByIndex((*this).swordEffectIndex) as
                                  *mut EffectBlure,
                              &mut (*(*this).swordInfo.as_mut_ptr().offset(0
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize)).tip,
                              &mut (*(*this).swordInfo.as_mut_ptr().offset(0
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize)).base);
    }
    if (*this).swordState as libc::c_int > 0 as libc::c_int &&
           (((*this).swordAnimation as libc::c_int) < 0x18 as libc::c_int ||
                (*this).stateFlags2 & 0x20000 as libc::c_int as libc::c_uint
                    != 0) {
        func_80090480(globalCtx,
                      &mut *(*this).swordQuads.as_mut_ptr().offset(0 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                      &mut *(*this).swordInfo.as_mut_ptr().offset(1 as
                                                                      libc::c_int
                                                                      as
                                                                      isize),
                      &mut *newTipPos.offset(1 as libc::c_int as isize),
                      &mut *newBasePos.as_mut_ptr().offset(1 as libc::c_int as
                                                               isize));
        func_80090480(globalCtx,
                      &mut *(*this).swordQuads.as_mut_ptr().offset(1 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                      &mut *(*this).swordInfo.as_mut_ptr().offset(2 as
                                                                      libc::c_int
                                                                      as
                                                                      isize),
                      &mut *newTipPos.offset(2 as libc::c_int as isize),
                      &mut *newBasePos.as_mut_ptr().offset(2 as libc::c_int as
                                                               isize));
    };
}
#[no_mangle]
pub unsafe extern "C" fn Player_DrawGetItemImpl(mut globalCtx:
                                                    *mut GlobalContext,
                                                mut this: *mut Player,
                                                mut refPos: *mut Vec3f,
                                                mut drawIdPlusOne: s32) {
    let mut height: f32_0 =
        if (*this).exchangeItemId as libc::c_int !=
               EXCH_ITEM_NONE as libc::c_int {
            6.0f32
        } else { 14.0f32 };
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_player_lib.c\x00" as *const u8 as
                        *const libc::c_char, 2401 as libc::c_int);
    gSegments[6 as libc::c_int as usize] =
        ((*this).giObjectSegment as
             *mut u8_0).offset(-(0x80000000 as libc::c_uint as isize)) as
            u32_0;
    let fresh12 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g: *mut Gfx = fresh12;
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
    (*_g).words.w1 = (*this).giObjectSegment as libc::c_uint;
    let fresh13 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_0: *mut Gfx = fresh13;
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
    (*_g_0).words.w1 = (*this).giObjectSegment as libc::c_uint;
    Matrix_Translate((*refPos).x +
                         3.3f32 * Math_SinS((*this).actor.shape.rot.y),
                     (*refPos).y + height,
                     (*refPos).z +
                         (3.3f32 +
                              (*gGameInfo).data[(9 as libc::c_int *
                                                     6 as libc::c_int *
                                                     16 as libc::c_int +
                                                     90 as libc::c_int) as
                                                    usize] as libc::c_int as
                                  libc::c_float / 10.0f32) *
                             Math_CosS((*this).actor.shape.rot.y),
                     MTXMODE_NEW as libc::c_int as u8_0);
    Matrix_RotateZYX(0 as libc::c_int as s16,
                     (*globalCtx).gameplayFrames.wrapping_mul(1000 as
                                                                  libc::c_int
                                                                  as
                                                                  libc::c_uint)
                         as s16, 0 as libc::c_int as s16,
                     MTXMODE_APPLY as libc::c_int as u8_0);
    Matrix_Scale(0.2f32, 0.2f32, 0.2f32,
                 MTXMODE_APPLY as libc::c_int as u8_0);
    GetItem_Draw(globalCtx, (drawIdPlusOne - 1 as libc::c_int) as s16);
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_player_lib.c\x00" as *const u8 as
                         *const libc::c_char, 2421 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Player_DrawGetItem(mut globalCtx: *mut GlobalContext,
                                            mut this: *mut Player) {
    if (*this).giObjectLoading == 0 ||
           osRecvMesg(&mut (*this).giObjectLoadQueue, 0 as *mut OSMesg,
                      0 as libc::c_int) == 0 {
        (*this).giObjectLoading = 0 as libc::c_int as u8_0;
        Player_DrawGetItemImpl(globalCtx, this, &mut sGetItemRefPos,
                               if (*this).unk_862 as libc::c_int >=
                                      0 as libc::c_int {
                                   (*this).unk_862 as libc::c_int
                               } else { -((*this).unk_862 as libc::c_int) });
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_80090A28(mut this: *mut Player,
                                       mut vecs: *mut Vec3f) {
    D_8012608C.x = D_80126080.x;
    if (*this).unk_845 as libc::c_int >= 3 as libc::c_int {
        (*this).unk_845 =
            ((*this).unk_845 as libc::c_int + 1 as libc::c_int) as u8_0;
        D_8012608C.x *=
            1.0f32 +
                (9 as libc::c_int - (*this).unk_845 as libc::c_int) as
                    libc::c_float * 0.1f32
    }
    D_8012608C.x += 1200.0f32;
    D_80126098.x = D_8012608C.x;
    Matrix_MultVec3f(&mut D_80126080,
                     &mut *vecs.offset(0 as libc::c_int as isize));
    Matrix_MultVec3f(&mut D_8012608C,
                     &mut *vecs.offset(1 as libc::c_int as isize));
    Matrix_MultVec3f(&mut D_80126098,
                     &mut *vecs.offset(2 as libc::c_int as isize));
}
#[no_mangle]
pub unsafe extern "C" fn func_80090AFC(mut globalCtx: *mut GlobalContext,
                                       mut this: *mut Player,
                                       mut arg2: f32_0) {
    static mut D_801260C8: Vec3f =
        {
            let mut init = Vec3f{x: -500.0f32, y: -100.0f32, z: 0.0f32,};
            init
        };
    let mut sp9C: *mut CollisionPoly = 0 as *mut CollisionPoly;
    let mut bgId: s32 = 0;
    let mut sp8C: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp80: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp74: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp68: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
    let mut sp64: f32_0 = 0.;
    let mut sp60: f32_0 = 0.;
    D_801260C8.z = 0.0f32;
    Matrix_MultVec3f(&mut D_801260C8, &mut sp8C);
    D_801260C8.z = arg2;
    Matrix_MultVec3f(&mut D_801260C8, &mut sp80);
    if BgCheck_AnyLineTest3(&mut (*globalCtx).colCtx, &mut sp8C, &mut sp80,
                            &mut sp74, &mut sp9C, 1 as libc::c_int,
                            1 as libc::c_int, 1 as libc::c_int,
                            1 as libc::c_int, &mut bgId) != 0 {
        let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
        let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
        __gfxCtx = (*globalCtx).state.gfxCtx;
        Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                        b"../z_player_lib.c\x00" as *const u8 as
                            *const libc::c_char, 2572 as libc::c_int);
        (*__gfxCtx).overlay.p =
            Gfx_CallSetupDL((*__gfxCtx).overlay.p,
                            0x7 as libc::c_int as u32_0);
        SkinMatrix_Vec3fMtxFMultXYZW(&mut (*globalCtx).viewProjectionMtxF,
                                     &mut sp74, &mut sp68, &mut sp64);
        sp60 =
            if sp64 < 200.0f32 {
                0.08f32
            } else { (sp64 / 200.0f32) * 0.08f32 };
        Matrix_Translate(sp74.x, sp74.y, sp74.z,
                         MTXMODE_NEW as libc::c_int as u8_0);
        Matrix_Scale(sp60, sp60, sp60, MTXMODE_APPLY as libc::c_int as u8_0);
        let fresh14 = (*__gfxCtx).overlay.p;
        (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
        let mut _g: *mut Gfx = fresh14;
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
                          b"../z_player_lib.c\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                          2587 as libc::c_int) as libc::c_uint;
        let fresh15 = (*__gfxCtx).overlay.p;
        (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
        let mut _g_0: *mut Gfx = fresh15;
        (*_g_0).words.w0 =
            (0xdb as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0x6 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                ((0x6 as libc::c_int * 4 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_0).words.w1 =
            (*globalCtx).objectCtx.status[(*this).actor.objBankIndex as
                                              usize].segment as libc::c_uint;
        let fresh16 = (*__gfxCtx).overlay.p;
        (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
        let mut _g_1: *mut Gfx = fresh16;
        (*_g_1).words.w0 =
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
        (*_g_1).words.w1 = D_0602CB48.as_mut_ptr() as libc::c_uint;
        Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                         b"../z_player_lib.c\x00" as *const u8 as
                             *const libc::c_char, 2592 as libc::c_int);
    };
}
#[no_mangle]
pub static mut D_801260D4: Vec3f =
    { let mut init = Vec3f{x: 1100.0f32, y: -700.0f32, z: 0.0f32,}; init };
#[no_mangle]
pub static mut sSwordLengths: [f32_0; 6] =
    [0.0f32, 4000.0f32, 3000.0f32, 5500.0f32, 0.0f32, 2500.0f32];
#[no_mangle]
pub static mut sBottleDLists: [*mut Gfx; 2] =
    unsafe {
        [0x602ad58 as libc::c_int as *mut Gfx,
         gLinkChildBottleDL.as_ptr() as *mut _]
    };
#[no_mangle]
pub static mut sBottleColors: [Color_RGB8; 13] =
    [{
         let mut init =
             Color_RGB8{r: 255 as libc::c_int as u8_0,
                        g: 255 as libc::c_int as u8_0,
                        b: 255 as libc::c_int as u8_0,};
         init
     },
     {
         let mut init =
             Color_RGB8{r: 80 as libc::c_int as u8_0,
                        g: 80 as libc::c_int as u8_0,
                        b: 255 as libc::c_int as u8_0,};
         init
     },
     {
         let mut init =
             Color_RGB8{r: 255 as libc::c_int as u8_0,
                        g: 100 as libc::c_int as u8_0,
                        b: 255 as libc::c_int as u8_0,};
         init
     },
     {
         let mut init =
             Color_RGB8{r: 0 as libc::c_int as u8_0,
                        g: 0 as libc::c_int as u8_0,
                        b: 255 as libc::c_int as u8_0,};
         init
     },
     {
         let mut init =
             Color_RGB8{r: 255 as libc::c_int as u8_0,
                        g: 0 as libc::c_int as u8_0,
                        b: 255 as libc::c_int as u8_0,};
         init
     },
     {
         let mut init =
             Color_RGB8{r: 255 as libc::c_int as u8_0,
                        g: 0 as libc::c_int as u8_0,
                        b: 255 as libc::c_int as u8_0,};
         init
     },
     {
         let mut init =
             Color_RGB8{r: 200 as libc::c_int as u8_0,
                        g: 200 as libc::c_int as u8_0,
                        b: 100 as libc::c_int as u8_0,};
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
             Color_RGB8{r: 0 as libc::c_int as u8_0,
                        g: 0 as libc::c_int as u8_0,
                        b: 255 as libc::c_int as u8_0,};
         init
     },
     {
         let mut init =
             Color_RGB8{r: 0 as libc::c_int as u8_0,
                        g: 255 as libc::c_int as u8_0,
                        b: 0 as libc::c_int as u8_0,};
         init
     },
     {
         let mut init =
             Color_RGB8{r: 255 as libc::c_int as u8_0,
                        g: 255 as libc::c_int as u8_0,
                        b: 255 as libc::c_int as u8_0,};
         init
     },
     {
         let mut init =
             Color_RGB8{r: 255 as libc::c_int as u8_0,
                        g: 255 as libc::c_int as u8_0,
                        b: 255 as libc::c_int as u8_0,};
         init
     },
     {
         let mut init =
             Color_RGB8{r: 80 as libc::c_int as u8_0,
                        g: 80 as libc::c_int as u8_0,
                        b: 255 as libc::c_int as u8_0,};
         init
     }];
#[no_mangle]
pub static mut D_80126128: Vec3f =
    { let mut init = Vec3f{x: 398.0f32, y: 1419.0f32, z: 244.0f32,}; init };
#[no_mangle]
pub static mut sBowStringData: [BowStringData; 2] =
    unsafe {
        [{
             let mut init =
                 BowStringData{dList:
                                   0x602b108 as libc::c_int as
                                       *mut libc::c_void,
                               pos:
                                   {
                                       let mut init =
                                           Vec3f{x: 0.0f32,
                                                 y: -360.4f32,
                                                 z: 0.0f32,};
                                       init
                                   },};
             init
         },
         {
             let mut init =
                 BowStringData{dList:
                                   0x60221a8 as libc::c_int as
                                       *mut libc::c_void,
                               pos:
                                   {
                                       let mut init =
                                           Vec3f{x: 606.0f32,
                                                 y: 236.0f32,
                                                 z: 0.0f32,};
                                       init
                                   },};
             init
         }]
    };
#[no_mangle]
pub static mut D_80126154: [Vec3f; 4] =
    [{
         let mut init = Vec3f{x: -4500.0f32, y: -3000.0f32, z: -600.0f32,};
         init
     },
     {
         let mut init = Vec3f{x: 1500.0f32, y: -3000.0f32, z: -600.0f32,};
         init
     },
     {
         let mut init = Vec3f{x: -4500.0f32, y: 3000.0f32, z: -600.0f32,};
         init
     },
     {
         let mut init = Vec3f{x: 1500.0f32, y: 3000.0f32, z: -600.0f32,};
         init
     }];
#[no_mangle]
pub static mut D_80126184: Vec3f =
    { let mut init = Vec3f{x: 100.0f32, y: 1500.0f32, z: 0.0f32,}; init };
#[no_mangle]
pub static mut D_80126190: Vec3f =
    { let mut init = Vec3f{x: 100.0f32, y: 1640.0f32, z: 0.0f32,}; init };
#[no_mangle]
pub static mut D_8012619C: [Vec3f; 4] =
    [{
         let mut init = Vec3f{x: -3000.0f32, y: -3000.0f32, z: -900.0f32,};
         init
     },
     {
         let mut init = Vec3f{x: 3000.0f32, y: -3000.0f32, z: -900.0f32,};
         init
     },
     {
         let mut init = Vec3f{x: -3000.0f32, y: 3000.0f32, z: -900.0f32,};
         init
     },
     {
         let mut init = Vec3f{x: 3000.0f32, y: 3000.0f32, z: -900.0f32,};
         init
     }];
#[no_mangle]
pub static mut D_801261CC: Vec3f =
    { let mut init = Vec3f{x: 630.0f32, y: 100.0f32, z: -30.0f32,}; init };
#[no_mangle]
pub static mut D_801261D8: Vec3s =
    {
        let mut init =
            Vec3s{x: 0 as libc::c_int as s16,
                  y: 0 as libc::c_int as s16,
                  z: 0x7fff as libc::c_int as s16,};
        init
    };
#[no_mangle]
pub static mut D_801261E0: [Vec3f; 2] =
    [{ let mut init = Vec3f{x: 200.0f32, y: 300.0f32, z: 0.0f32,}; init },
     { let mut init = Vec3f{x: 200.0f32, y: 200.0f32, z: 0.0f32,}; init }];
#[no_mangle]
pub unsafe extern "C" fn func_80090D20(mut globalCtx: *mut GlobalContext,
                                       mut limbIndex: s32,
                                       mut dList: *mut *mut Gfx,
                                       mut rot: *mut Vec3s,
                                       mut thisx: *mut libc::c_void) {
    let mut this: *mut Player = thisx as *mut Player;
    if !(*dList).is_null() { Matrix_MultVec3f(&mut D_8012602C, D_80160000); }
    if limbIndex == PLAYER_LIMB_L_HAND as libc::c_int {
        let mut sp14C: MtxF = MtxF{mf: [[0.; 4]; 4],};
        let mut hookedActor: *mut Actor = 0 as *mut Actor;
        Math_Vec3f_Copy(&mut (*this).leftHandPos, D_80160000);
        if (*this).itemActionParam as libc::c_int ==
               PLAYER_AP_STICK as libc::c_int {
            let mut sp124: [Vec3f; 3] = [Vec3f{x: 0., y: 0., z: 0.,}; 3];
            let mut __gfxCtx: *mut GraphicsContext =
                0 as *mut GraphicsContext;
            let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
            __gfxCtx = (*globalCtx).state.gfxCtx;
            Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                            b"../z_player_lib.c\x00" as *const u8 as
                                *const libc::c_char, 2633 as libc::c_int);
            if (*this).actor.scale.y >= 0.0f32 {
                D_80126080.x = (*this).unk_85C * 5000.0f32;
                func_80090A28(this, sp124.as_mut_ptr());
                if (*this).swordState as libc::c_int != 0 as libc::c_int {
                    func_800906D4(globalCtx, this, sp124.as_mut_ptr());
                } else {
                    Math_Vec3f_Copy(&mut (*(*this).swordInfo.as_mut_ptr().offset(0
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).tip,
                                    &mut *sp124.as_mut_ptr().offset(0 as
                                                                        libc::c_int
                                                                        as
                                                                        isize));
                }
            }
            Matrix_Translate(-428.26f32, 267.2f32, -33.82f32,
                             MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_RotateZYX(-(0x8000 as libc::c_int) as s16,
                             0 as libc::c_int as s16,
                             0x4000 as libc::c_int as s16,
                             MTXMODE_APPLY as libc::c_int as u8_0);
            Matrix_Scale(1.0f32, (*this).unk_85C, 1.0f32,
                         MTXMODE_APPLY as libc::c_int as u8_0);
            let fresh17 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g: *mut Gfx = fresh17;
            (*_g).words.w0 =
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
            (*_g).words.w1 =
                Matrix_NewMtx((*globalCtx).state.gfxCtx,
                              b"../z_player_lib.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              2653 as libc::c_int) as libc::c_uint;
            let fresh18 = (*__gfxCtx).polyOpa.p;
            (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
            let mut _g_0: *mut Gfx = fresh18;
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
                gLinkChildLinkDekuStickDL.as_mut_ptr() as libc::c_uint;
            Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                             b"../z_player_lib.c\x00" as *const u8 as
                                 *const libc::c_char, 2656 as libc::c_int);
        } else if (*this).actor.scale.y >= 0.0f32 &&
                      (*this).swordState as libc::c_int != 0 as libc::c_int {
            let mut spE4: [Vec3f; 3] = [Vec3f{x: 0., y: 0., z: 0.,}; 3];
            if Player_HoldsBrokenKnife(this) != 0 {
                D_80126080.x = 1500.0f32
            } else {
                D_80126080.x =
                    sSwordLengths[Player_GetSwordHeld(this) as usize]
            }
            func_80090A28(this, spE4.as_mut_ptr());
            func_800906D4(globalCtx, this, spE4.as_mut_ptr());
        } else if !(*dList).is_null() &&
                      (*this).leftHandType as libc::c_int == 7 as libc::c_int
         {
            let mut bottleColor: *mut Color_RGB8 =
                &mut *sBottleColors.as_mut_ptr().offset((Player_ActionToBottle
                                                             as
                                                             unsafe extern "C" fn(_:
                                                                                      *mut Player,
                                                                                  _:
                                                                                      s32)
                                                                 ->
                                                                     s32)(this,
                                                                          (*this).itemActionParam
                                                                              as
                                                                              s32)
                                                            as isize) as
                    *mut Color_RGB8;
            let mut __gfxCtx_0: *mut GraphicsContext =
                0 as *mut GraphicsContext;
            let mut dispRefs_0: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
            __gfxCtx_0 = (*globalCtx).state.gfxCtx;
            Graph_OpenDisps(dispRefs_0.as_mut_ptr(),
                            (*globalCtx).state.gfxCtx,
                            b"../z_player_lib.c\x00" as *const u8 as
                                *const libc::c_char, 2710 as libc::c_int);
            let fresh19 = (*__gfxCtx_0).polyXlu.p;
            (*__gfxCtx_0).polyXlu.p = (*__gfxCtx_0).polyXlu.p.offset(1);
            let mut _g_1: *mut Gfx = fresh19;
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
                              b"../z_player_lib.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              2712 as libc::c_int) as libc::c_uint;
            let fresh20 = (*__gfxCtx_0).polyXlu.p;
            (*__gfxCtx_0).polyXlu.p = (*__gfxCtx_0).polyXlu.p.offset(1);
            let mut _g_2: *mut Gfx = fresh20;
            (*_g_2).words.w0 =
                (0xfb as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_2).words.w1 =
                ((*bottleColor).r as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((*bottleColor).g as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    ((*bottleColor).b as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        8 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh21 = (*__gfxCtx_0).polyXlu.p;
            (*__gfxCtx_0).polyXlu.p = (*__gfxCtx_0).polyXlu.p.offset(1);
            let mut _g_3: *mut Gfx = fresh21;
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
                sBottleDLists[gSaveContext.linkAge as usize] as libc::c_uint;
            Graph_CloseDisps(dispRefs_0.as_mut_ptr(),
                             (*globalCtx).state.gfxCtx,
                             b"../z_player_lib.c\x00" as *const u8 as
                                 *const libc::c_char, 2717 as libc::c_int);
        }
        if (*this).actor.scale.y >= 0.0f32 {
            if Player_HoldsHookshot(this) == 0 &&
                   { hookedActor = (*this).heldActor; !hookedActor.is_null() }
               {
                if (*this).stateFlags1 & 0x200 as libc::c_int as libc::c_uint
                       != 0 {
                    Matrix_MultVec3f(&mut D_80126128,
                                     &mut (*hookedActor).world.pos);
                    Matrix_RotateZYX(0x69e8 as libc::c_int as s16,
                                     -(0x5708 as libc::c_int) as s16,
                                     0x458e as libc::c_int as s16,
                                     MTXMODE_APPLY as libc::c_int as u8_0);
                    Matrix_Get(&mut sp14C);
                    Matrix_MtxFToYXZRotS(&mut sp14C,
                                         &mut (*hookedActor).world.rot,
                                         0 as libc::c_int);
                    (*hookedActor).shape.rot = (*hookedActor).world.rot
                } else if (*this).stateFlags1 &
                              0x800 as libc::c_int as libc::c_uint != 0 {
                    let mut spB8: Vec3s = Vec3s{x: 0, y: 0, z: 0,};
                    Matrix_Get(&mut sp14C);
                    Matrix_MtxFToYXZRotS(&mut sp14C, &mut spB8,
                                         0 as libc::c_int);
                    if (*hookedActor).flags &
                           ((1 as libc::c_int) << 17 as libc::c_int) as
                               libc::c_uint != 0 {
                        (*hookedActor).shape.rot.x =
                            (spB8.x as libc::c_int -
                                 (*this).unk_3BC.x as libc::c_int) as s16;
                        (*hookedActor).world.rot.x =
                            (*hookedActor).shape.rot.x
                    } else {
                        (*hookedActor).shape.rot.y =
                            ((*this).actor.shape.rot.y as libc::c_int +
                                 (*this).unk_3BC.y as libc::c_int) as s16;
                        (*hookedActor).world.rot.y =
                            (*hookedActor).shape.rot.y
                    }
                }
            } else {
                Matrix_Get(&mut (*this).mf_9E0);
                Matrix_MtxFToYXZRotS(&mut (*this).mf_9E0,
                                     &mut (*this).unk_3BC, 0 as libc::c_int);
            }
        }
    } else if limbIndex == PLAYER_LIMB_R_HAND as libc::c_int {
        let mut heldActor: *mut Actor = (*this).heldActor;
        if (*this).rightHandType as libc::c_int == 0xff as libc::c_int {
            Matrix_Get(&mut (*this).shieldMf);
        } else if (*this).rightHandType as libc::c_int == 11 as libc::c_int ||
                      (*this).rightHandType as libc::c_int ==
                          12 as libc::c_int {
            let mut stringData: *mut BowStringData =
                &mut *sBowStringData.as_mut_ptr().offset(gSaveContext.linkAge
                                                             as isize) as
                    *mut BowStringData;
            let mut __gfxCtx_1: *mut GraphicsContext =
                0 as *mut GraphicsContext;
            let mut dispRefs_1: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
            __gfxCtx_1 = (*globalCtx).state.gfxCtx;
            Graph_OpenDisps(dispRefs_1.as_mut_ptr(),
                            (*globalCtx).state.gfxCtx,
                            b"../z_player_lib.c\x00" as *const u8 as
                                *const libc::c_char, 2783 as libc::c_int);
            Matrix_Push();
            Matrix_Translate((*stringData).pos.x, (*stringData).pos.y,
                             (*stringData).pos.z,
                             MTXMODE_APPLY as libc::c_int as u8_0);
            if (*this).stateFlags1 & 0x200 as libc::c_int as libc::c_uint != 0
                   && (*this).unk_860 as libc::c_int >= 0 as libc::c_int &&
                   (*this).unk_834 as libc::c_int <= 10 as libc::c_int {
                let mut sp90: Vec3f = Vec3f{x: 0., y: 0., z: 0.,};
                let mut distXYZ: f32_0 = 0.;
                Matrix_MultVec3f(&mut D_8012602C, &mut sp90);
                distXYZ = Math_Vec3f_DistXYZ(D_80160000, &mut sp90);
                (*this).unk_858 = distXYZ - 3.0f32;
                if distXYZ < 3.0f32 {
                    (*this).unk_858 = 0.0f32
                } else {
                    (*this).unk_858 *= 1.6f32;
                    if (*this).unk_858 > 1.0f32 { (*this).unk_858 = 1.0f32 }
                }
                (*this).unk_85C = -0.5f32
            }
            Matrix_Scale(1.0f32, (*this).unk_858, 1.0f32,
                         MTXMODE_APPLY as libc::c_int as u8_0);
            if !(gSaveContext.linkAge == 0 as libc::c_int) {
                Matrix_RotateZ((*this).unk_858 * -0.2f32,
                               MTXMODE_APPLY as libc::c_int as u8_0);
            }
            let fresh22 = (*__gfxCtx_1).polyXlu.p;
            (*__gfxCtx_1).polyXlu.p = (*__gfxCtx_1).polyXlu.p.offset(1);
            let mut _g_4: *mut Gfx = fresh22;
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
                              b"../z_player_lib.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              2804 as libc::c_int) as libc::c_uint;
            let fresh23 = (*__gfxCtx_1).polyXlu.p;
            (*__gfxCtx_1).polyXlu.p = (*__gfxCtx_1).polyXlu.p.offset(1);
            let mut _g_5: *mut Gfx = fresh23;
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
            (*_g_5).words.w1 = (*stringData).dList as libc::c_uint;
            Matrix_Pop();
            Graph_CloseDisps(dispRefs_1.as_mut_ptr(),
                             (*globalCtx).state.gfxCtx,
                             b"../z_player_lib.c\x00" as *const u8 as
                                 *const libc::c_char, 2809 as libc::c_int);
        } else if (*this).actor.scale.y >= 0.0f32 &&
                      (*this).rightHandType as libc::c_int ==
                          10 as libc::c_int {
            Matrix_Get(&mut (*this).shieldMf);
            func_80090604(globalCtx, this, &mut (*this).shieldQuad,
                          D_80126154.as_mut_ptr());
        }
        if (*this).actor.scale.y >= 0.0f32 {
            if (*this).heldItemActionParam as libc::c_int ==
                   PLAYER_AP_HOOKSHOT as libc::c_int ||
                   (*this).heldItemActionParam as libc::c_int ==
                       PLAYER_AP_LONGSHOT as libc::c_int {
                Matrix_MultVec3f(&mut D_80126184, &mut (*this).unk_3C8);
                if !heldActor.is_null() {
                    let mut sp44: MtxF = MtxF{mf: [[0.; 4]; 4],};
                    let mut pad: s32 = 0;
                    Matrix_MultVec3f(&mut D_80126190,
                                     &mut (*heldActor).world.pos);
                    Matrix_RotateZYX(0 as libc::c_int as s16,
                                     -(0x4000 as libc::c_int) as s16,
                                     -(0x4000 as libc::c_int) as s16,
                                     MTXMODE_APPLY as libc::c_int as u8_0);
                    Matrix_Get(&mut sp44);
                    Matrix_MtxFToYXZRotS(&mut sp44,
                                         &mut (*heldActor).world.rot,
                                         0 as libc::c_int);
                    (*heldActor).shape.rot = (*heldActor).world.rot;
                    if func_8002DD78(this) != 0 as libc::c_int {
                        Matrix_Translate(500.0f32, 300.0f32, 0.0f32,
                                         MTXMODE_APPLY as libc::c_int as
                                             u8_0);
                        func_80090AFC(globalCtx, this,
                                      if (*this).heldItemActionParam as
                                             libc::c_int ==
                                             PLAYER_AP_HOOKSHOT as libc::c_int
                                         {
                                          38600.0f32
                                      } else { 77600.0f32 });
                    }
                }
            }
            if (*this).unk_862 as libc::c_int != 0 as libc::c_int ||
                   func_8002DD6C(this) == 0 as libc::c_int &&
                       !heldActor.is_null() {
                if (*this).stateFlags1 & 0x400 as libc::c_int as libc::c_uint
                       == 0 &&
                       (*this).unk_862 as libc::c_int != 0 as libc::c_int &&
                       (*this).exchangeItemId as libc::c_int !=
                           EXCH_ITEM_NONE as libc::c_int {
                    Math_Vec3f_Copy(&mut sGetItemRefPos,
                                    &mut (*this).leftHandPos);
                } else {
                    sGetItemRefPos.x =
                        ((*this).bodyPartsPos[15 as libc::c_int as usize].x +
                             (*this).leftHandPos.x) * 0.5f32;
                    sGetItemRefPos.y =
                        ((*this).bodyPartsPos[15 as libc::c_int as usize].y +
                             (*this).leftHandPos.y) * 0.5f32;
                    sGetItemRefPos.z =
                        ((*this).bodyPartsPos[15 as libc::c_int as usize].z +
                             (*this).leftHandPos.z) * 0.5f32
                }
                if (*this).unk_862 as libc::c_int == 0 as libc::c_int {
                    Math_Vec3f_Copy(&mut (*heldActor).world.pos,
                                    &mut sGetItemRefPos);
                }
            }
        }
    } else if (*this).actor.scale.y >= 0.0f32 {
        if limbIndex == PLAYER_LIMB_SHEATH as libc::c_int {
            if (*this).rightHandType as libc::c_int != 10 as libc::c_int &&
                   (*this).rightHandType as libc::c_int != 0xff as libc::c_int
               {
                if Player_IsChildWithHylianShield(this) != 0 {
                    func_80090604(globalCtx, this, &mut (*this).shieldQuad,
                                  D_8012619C.as_mut_ptr());
                }
                Matrix_TranslateRotateZYX(&mut D_801261CC, &mut D_801261D8);
                Matrix_Get(&mut (*this).shieldMf);
            }
        } else if limbIndex == PLAYER_LIMB_HEAD as libc::c_int {
            Matrix_MultVec3f(&mut D_801260D4, &mut (*this).actor.focus.pos);
        } else {
            let mut vec: *mut Vec3f =
                &mut *D_801261E0.as_mut_ptr().offset(gSaveContext.linkAge as
                                                         isize) as *mut Vec3f;
            Actor_SetFeetPos(&mut (*this).actor, limbIndex,
                             PLAYER_LIMB_L_FOOT as libc::c_int, vec,
                             PLAYER_LIMB_R_FOOT as libc::c_int, vec);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_80091738(mut globalCtx: *mut GlobalContext,
                                       mut segment: *mut u8_0,
                                       mut skelAnime: *mut SkelAnime)
 -> u32_0 {
    let mut linkObjectId: s16 = gLinkObjectIds[gSaveContext.linkAge as usize];
    let mut size: u32_0 = 0;
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    size =
        gObjectTable[OBJECT_GAMEPLAY_KEEP as libc::c_int as
                         usize].vromEnd.wrapping_sub(gObjectTable[OBJECT_GAMEPLAY_KEEP
                                                                      as
                                                                      libc::c_int
                                                                      as
                                                                      usize].vromStart);
    ptr = segment.offset(0x3800 as libc::c_int as isize) as *mut libc::c_void;
    DmaMgr_SendRequest1(ptr,
                        gObjectTable[OBJECT_GAMEPLAY_KEEP as libc::c_int as
                                         usize].vromStart, size,
                        b"../z_player_lib.c\x00" as *const u8 as
                            *const libc::c_char, 2982 as libc::c_int);
    size =
        gObjectTable[linkObjectId as
                         usize].vromEnd.wrapping_sub(gObjectTable[linkObjectId
                                                                      as
                                                                      usize].vromStart);
    ptr = segment.offset(0x8800 as libc::c_int as isize) as *mut libc::c_void;
    DmaMgr_SendRequest1(ptr, gObjectTable[linkObjectId as usize].vromStart,
                        size,
                        b"../z_player_lib.c\x00" as *const u8 as
                            *const libc::c_char, 2988 as libc::c_int);
    ptr =
        ((ptr as
              u32_0).wrapping_add(size).wrapping_add(0xf as libc::c_int as
                                                         libc::c_uint) &
             !(0xf as libc::c_int) as libc::c_uint) as *mut libc::c_void;
    gSegments[4 as libc::c_int as usize] =
        segment.offset(0x3800 as libc::c_int as
                           isize).offset(-(0x80000000 as libc::c_uint as
                                               isize)) as u32_0;
    gSegments[6 as libc::c_int as usize] =
        segment.offset(0x8800 as libc::c_int as
                           isize).offset(-(0x80000000 as libc::c_uint as
                                               isize)) as u32_0;
    SkelAnime_InitLink(globalCtx, skelAnime,
                       gPlayerSkelHeaders[gSaveContext.linkAge as usize],
                       &mut gPlayerAnim_003238, 9 as libc::c_int,
                       ptr as *mut Vec3s, ptr as *mut Vec3s,
                       PLAYER_LIMB_MAX as libc::c_int);
    return size.wrapping_add(0x8800 as libc::c_int as
                                 libc::c_uint).wrapping_add(0x90 as
                                                                libc::c_int as
                                                                libc::c_uint);
}
#[no_mangle]
pub static mut D_801261F8: [u8_0; 3] =
    [2 as libc::c_int as u8_0, 2 as libc::c_int as u8_0,
     5 as libc::c_int as u8_0];
#[no_mangle]
pub unsafe extern "C" fn func_80091880(mut globalCtx: *mut GlobalContext,
                                       mut limbIndex: s32,
                                       mut dList: *mut *mut Gfx,
                                       mut pos: *mut Vec3f,
                                       mut rot: *mut Vec3s,
                                       mut arg: *mut libc::c_void) -> s32 {
    let mut ptr: *mut u8_0 = arg as *mut u8_0;
    let mut modelGroup: u8_0 =
        D_801261F8[(*ptr.offset(0 as libc::c_int as isize) as libc::c_int -
                        1 as libc::c_int) as usize];
    let mut type_0: s32 = 0;
    let mut dListOffset: s32 = 0 as libc::c_int;
    let mut dLists: *mut *mut Gfx = 0 as *mut *mut Gfx;
    if modelGroup as libc::c_int == 2 as libc::c_int &&
           !(gSaveContext.linkAge == 0 as libc::c_int) &&
           *ptr.offset(1 as libc::c_int as isize) as libc::c_int ==
               2 as libc::c_int {
        modelGroup = 1 as libc::c_int as u8_0
    }
    if limbIndex == PLAYER_LIMB_L_HAND as libc::c_int {
        type_0 =
            gPlayerModelTypes[modelGroup as usize][1 as libc::c_int as usize]
                as s32;
        D_80160014 = type_0;
        if type_0 == 4 as libc::c_int &&
               gSaveContext.swordHealth as libc::c_int as libc::c_float <=
                   0.0f32 {
            dListOffset = 4 as libc::c_int
        }
    } else if limbIndex == PLAYER_LIMB_R_HAND as libc::c_int {
        type_0 =
            gPlayerModelTypes[modelGroup as usize][2 as libc::c_int as usize]
                as s32;
        D_80160018 = type_0;
        if type_0 == 10 as libc::c_int {
            dListOffset =
                *ptr.offset(1 as libc::c_int as isize) as libc::c_int *
                    4 as libc::c_int
        }
    } else if limbIndex == PLAYER_LIMB_SHEATH as libc::c_int {
        type_0 =
            gPlayerModelTypes[modelGroup as usize][3 as libc::c_int as usize]
                as s32;
        if type_0 == 18 as libc::c_int || type_0 == 19 as libc::c_int {
            dListOffset =
                *ptr.offset(1 as libc::c_int as isize) as libc::c_int *
                    4 as libc::c_int
        }
    } else if limbIndex == PLAYER_LIMB_WAIST as libc::c_int {
        type_0 =
            gPlayerModelTypes[modelGroup as usize][4 as libc::c_int as usize]
                as s32
    } else { return 0 as libc::c_int }
    dLists =
        &mut *(*sPlayerDListGroups.as_mut_ptr().offset(type_0 as
                                                           isize)).offset(gSaveContext.linkAge
                                                                              as
                                                                              isize)
            as *mut *mut Gfx;
    *dList = *dLists.offset(dListOffset as isize);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_80091A24(mut globalCtx: *mut GlobalContext,
                                       mut seg04: *mut libc::c_void,
                                       mut seg06: *mut libc::c_void,
                                       mut skelAnime: *mut SkelAnime,
                                       mut pos: *mut Vec3f,
                                       mut rot: *mut Vec3s, mut scale: f32_0,
                                       mut sword: s32, mut tunic: s32,
                                       mut shield: s32, mut boots: s32,
                                       mut width: s32, mut height: s32,
                                       mut eye: *mut Vec3f,
                                       mut at: *mut Vec3f, mut fovy: f32_0,
                                       mut img1: *mut libc::c_void,
                                       mut img2: *mut libc::c_void) {
    static mut viewport: Vp =
        Vp{vp:
               {
                   let mut init =
                       Vp_t{vscale:
                                [128 as libc::c_int as libc::c_short,
                                 224 as libc::c_int as libc::c_short,
                                 511 as libc::c_int as libc::c_short,
                                 0 as libc::c_int as libc::c_short],
                            vtrans:
                                [128 as libc::c_int as libc::c_short,
                                 224 as libc::c_int as libc::c_short,
                                 511 as libc::c_int as libc::c_short,
                                 0 as libc::c_int as libc::c_short],};
                   init
               },};
    static mut lights1: Lights1 =
        {
            let mut init =
                Lights1{a:
                            Ambient{l:
                                        {
                                            let mut init =
                                                Ambient_t{col:
                                                              [80 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uchar,
                                                               80 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uchar,
                                                               80 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uchar],
                                                          pad1:
                                                              0 as libc::c_int
                                                                  as
                                                                  libc::c_char,
                                                          colc:
                                                              [80 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uchar,
                                                               80 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uchar,
                                                               80 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uchar],
                                                          pad2:
                                                              0 as libc::c_int
                                                                  as
                                                                  libc::c_char,};
                                            init
                                        },},
                        l:
                            [Light{l:
                                       {
                                           let mut init =
                                               Light_t{col:
                                                           [255 as libc::c_int
                                                                as
                                                                libc::c_uchar,
                                                            255 as libc::c_int
                                                                as
                                                                libc::c_uchar,
                                                            255 as libc::c_int
                                                                as
                                                                libc::c_uchar],
                                                       pad1:
                                                           0 as libc::c_int as
                                                               libc::c_char,
                                                       colc:
                                                           [255 as libc::c_int
                                                                as
                                                                libc::c_uchar,
                                                            255 as libc::c_int
                                                                as
                                                                libc::c_uchar,
                                                            255 as libc::c_int
                                                                as
                                                                libc::c_uchar],
                                                       pad2:
                                                           0 as libc::c_int as
                                                               libc::c_char,
                                                       dir:
                                                           [84 as libc::c_int
                                                                as
                                                                libc::c_schar,
                                                            84 as libc::c_int
                                                                as
                                                                libc::c_schar,
                                                            172 as libc::c_int
                                                                as
                                                                libc::c_schar],
                                                       pad3:
                                                           0 as libc::c_int as
                                                               libc::c_char,};
                                           init
                                       },}],};
            init
        };
    static mut lightDir: Vec3f =
        { let mut init = Vec3f{x: 89.8f32, y: 0.0f32, z: 89.8f32,}; init };
    let mut sp12C: [u8_0; 2] = [0; 2];
    let mut opaRef: *mut Gfx = 0 as *mut Gfx;
    let mut xluRef: *mut Gfx = 0 as *mut Gfx;
    let mut perspNorm: u16_0 = 0;
    let mut perspMtx: *mut Mtx =
        Graph_Alloc((*globalCtx).state.gfxCtx,
                    ::std::mem::size_of::<Mtx>() as libc::c_ulong as size_t)
            as *mut Mtx;
    let mut lookAtMtx: *mut Mtx =
        Graph_Alloc((*globalCtx).state.gfxCtx,
                    ::std::mem::size_of::<Mtx>() as libc::c_ulong as size_t)
            as *mut Mtx;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = (*globalCtx).state.gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                    b"../z_player_lib.c\x00" as *const u8 as
                        *const libc::c_char, 3129 as libc::c_int);
    let mut pad: [s32; 2] = [0; 2];
    opaRef = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    xluRef = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let fresh24 = (*__gfxCtx).work.p;
    (*__gfxCtx).work.p = (*__gfxCtx).work.p.offset(1);
    let mut _g: *mut Gfx = fresh24;
    (*_g).words.w0 =
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
    (*_g).words.w1 = (*__gfxCtx).polyOpa.p as libc::c_uint;
    let fresh25 = (*__gfxCtx).work.p;
    (*__gfxCtx).work.p = (*__gfxCtx).work.p.offset(1);
    let mut _g_0: *mut Gfx = fresh25;
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
    (*_g_0).words.w1 = (*__gfxCtx).polyXlu.p as libc::c_uint;
    let fresh26 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_1: *mut Gfx = fresh26;
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
    let fresh27 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_2: *mut Gfx = fresh27;
    (*_g_2).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_2).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh28 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_3: *mut Gfx = fresh28;
    (*_g_3).words.w0 =
        (0xd9 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (!(-(1 as libc::c_int) as u32_0) &
                 (((0x1 as libc::c_int) << 24 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_3).words.w1 = 0 as libc::c_int as u32_0;
    let fresh29 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_4: *mut Gfx = fresh29;
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
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 7 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 1 as libc::c_int;
    (*_g_4).words.w1 =
        (0xffff as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 16 as libc::c_int |
            (0xffff as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh30 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_5: *mut Gfx = fresh30;
    (*_g_5).words.w0 =
        (0xfc as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (((31 as libc::c_int as u32_0 &
                   (((0x1 as libc::c_int) << 4 as libc::c_int) -
                        1 as libc::c_int) as libc::c_uint) <<
                  20 as libc::c_int |
                  (31 as libc::c_int as u32_0 &
                       (((0x1 as libc::c_int) << 5 as libc::c_int) -
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
                       5 as libc::c_int |
                       (31 as libc::c_int as u32_0 &
                            (((0x1 as libc::c_int) << 5 as libc::c_int) -
                                 1 as libc::c_int) as libc::c_uint) <<
                           0 as libc::c_int)) &
                 (((0x1 as libc::c_int) << 24 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_5).words.w1 =
        (31 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 28 as libc::c_int |
            (4 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 15 as libc::c_int
            |
            (7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            (4 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 9 as libc::c_int |
            ((31 as libc::c_int as u32_0 &
                  (((0x1 as libc::c_int) << 4 as libc::c_int) -
                       1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                 |
                 (7 as libc::c_int as u32_0 &
                      (((0x1 as libc::c_int) << 3 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     21 as libc::c_int |
                 (7 as libc::c_int as u32_0 &
                      (((0x1 as libc::c_int) << 3 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     18 as libc::c_int |
                 (4 as libc::c_int as u32_0 &
                      (((0x1 as libc::c_int) << 3 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     6 as libc::c_int |
                 (7 as libc::c_int as u32_0 &
                      (((0x1 as libc::c_int) << 3 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     3 as libc::c_int |
                 (4 as libc::c_int as u32_0 &
                      (((0x1 as libc::c_int) << 3 as libc::c_int) -
                           1 as libc::c_int) as libc::c_uint) <<
                     0 as libc::c_int);
    let fresh31 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_6: *mut Gfx = fresh31;
    (*_g_6).words.w0 =
        (0xef as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (((3 as libc::c_int) << 4 as libc::c_int |
                  (0 as libc::c_int) << 6 as libc::c_int |
                  (0 as libc::c_int) << 8 as libc::c_int |
                  (6 as libc::c_int) << 9 as libc::c_int |
                  (2 as libc::c_int) << 12 as libc::c_int |
                  (0 as libc::c_int) << 14 as libc::c_int |
                  (0 as libc::c_int) << 16 as libc::c_int |
                  (0 as libc::c_int) << 17 as libc::c_int |
                  (1 as libc::c_int) << 19 as libc::c_int |
                  (3 as libc::c_int) << 20 as libc::c_int |
                  (0 as libc::c_int) << 23 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 24 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_6).words.w1 =
        ((0 as libc::c_int) << 0 as libc::c_int |
             (0 as libc::c_int) << 2 as libc::c_int |
             (0 as libc::c_int) << 30 as libc::c_int |
             (0 as libc::c_int) << 26 as libc::c_int |
             (0 as libc::c_int) << 22 as libc::c_int |
             (0 as libc::c_int) << 18 as libc::c_int |
             (0 as libc::c_int) << 28 as libc::c_int |
             (0 as libc::c_int) << 24 as libc::c_int |
             (0 as libc::c_int) << 20 as libc::c_int |
             (0 as libc::c_int) << 16 as libc::c_int) as libc::c_uint;
    let fresh32 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_7: *mut Gfx = fresh32;
    (*_g_7).words.w0 =
        (0xd9 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (!(-(1 as libc::c_int) as u32_0) &
                 (((0x1 as libc::c_int) << 24 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_7).words.w1 =
        (0x1 as libc::c_int | 0x4 as libc::c_int | 0x400 as libc::c_int |
             0x20000 as libc::c_int | 0x200000 as libc::c_int) as u32_0;
    let fresh33 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_8: *mut Gfx = fresh33;
    (*_g_8).words.w0 =
        (0xed as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((0 as libc::c_int as libc::c_float * 4.0f32) as libc::c_int as
                 u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            ((0 as libc::c_int as libc::c_float * 4.0f32) as libc::c_int as
                 u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_8).words.w1 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 2 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((width as libc::c_float * 4.0f32) as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 12 as libc::c_int
            |
            ((height as libc::c_float * 4.0f32) as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh34 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_9: *mut Gfx = fresh34;
    (*_g_9).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x4 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0x4 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_9).words.w1 = 0x1 as libc::c_int as libc::c_uint;
    let fresh35 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_10: *mut Gfx = fresh35;
    (*_g_10).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x4 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0xc as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_10).words.w1 = 0x1 as libc::c_int as libc::c_uint;
    let fresh36 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_11: *mut Gfx = fresh36;
    (*_g_11).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x4 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0x14 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_11).words.w1 = 0xffff as libc::c_int as libc::c_uint;
    let fresh37 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_12: *mut Gfx = fresh37;
    (*_g_12).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x4 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0x1c as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_12).words.w1 = 0xffff as libc::c_int as libc::c_uint;
    let fresh38 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_13: *mut Gfx = fresh38;
    (*_g_13).words.w0 =
        (0xff as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 21 as libc::c_int
            |
            (2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 19 as libc::c_int
            |
            ((width - 1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_13).words.w1 = img2 as libc::c_uint;
    let fresh39 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_14: *mut Gfx = fresh39;
    (*_g_14).words.w0 =
        (0xe3 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((32 as libc::c_int - 20 as libc::c_int - 2 as libc::c_int) as
                 u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            ((2 as libc::c_int - 1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_14).words.w1 =
        ((3 as libc::c_int) << 20 as libc::c_int) as libc::c_uint;
    let fresh40 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_15: *mut Gfx = fresh40;
    (*_g_15).words.w0 =
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
    (*_g_15).words.w1 =
        ((0 as libc::c_int) << 30 as libc::c_int |
             (0 as libc::c_int) << 26 as libc::c_int |
             (0 as libc::c_int) << 22 as libc::c_int |
             (0 as libc::c_int) << 18 as libc::c_int |
             ((0 as libc::c_int) << 28 as libc::c_int |
                  (0 as libc::c_int) << 24 as libc::c_int |
                  (0 as libc::c_int) << 20 as libc::c_int |
                  (0 as libc::c_int) << 16 as libc::c_int)) as libc::c_uint;
    let fresh41 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_16: *mut Gfx = fresh41;
    (*_g_16).words.w0 =
        (0xf7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_16).words.w1 =
        (((255 as libc::c_int) << 8 as libc::c_int & 0xf800 as libc::c_int |
              (255 as libc::c_int) << 3 as libc::c_int & 0x7c0 as libc::c_int
              | 240 as libc::c_int >> 2 as libc::c_int & 0x3e as libc::c_int |
              0 as libc::c_int & 0x1 as libc::c_int) << 16 as libc::c_int |
             ((255 as libc::c_int) << 8 as libc::c_int & 0xf800 as libc::c_int
                  |
                  (255 as libc::c_int) << 3 as libc::c_int &
                      0x7c0 as libc::c_int |
                  240 as libc::c_int >> 2 as libc::c_int & 0x3e as libc::c_int
                  | 0 as libc::c_int & 0x1 as libc::c_int)) as libc::c_uint;
    let fresh42 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_17: *mut Gfx = fresh42;
    (*_g_17).words.w0 =
        (0xf6 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((width - 1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 10 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 14 as libc::c_int
            |
            ((height - 1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 10 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 2 as libc::c_int;
    (*_g_17).words.w1 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 10 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 14 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 10 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 2 as libc::c_int;
    let fresh43 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_18: *mut Gfx = fresh43;
    (*_g_18).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_18).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh44 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_19: *mut Gfx = fresh44;
    (*_g_19).words.w0 =
        (0xff as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 21 as libc::c_int
            |
            (2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 19 as libc::c_int
            |
            ((width - 1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_19).words.w1 = img1 as libc::c_uint;
    let fresh45 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_20: *mut Gfx = fresh45;
    (*_g_20).words.w0 =
        (0xe3 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((32 as libc::c_int - 20 as libc::c_int - 2 as libc::c_int) as
                 u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            ((2 as libc::c_int - 1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_20).words.w1 =
        ((3 as libc::c_int) << 20 as libc::c_int) as libc::c_uint;
    let fresh46 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_21: *mut Gfx = fresh46;
    (*_g_21).words.w0 =
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
    (*_g_21).words.w1 =
        ((0 as libc::c_int) << 30 as libc::c_int |
             (0 as libc::c_int) << 26 as libc::c_int |
             (0 as libc::c_int) << 22 as libc::c_int |
             (0 as libc::c_int) << 18 as libc::c_int |
             ((0 as libc::c_int) << 28 as libc::c_int |
                  (0 as libc::c_int) << 24 as libc::c_int |
                  (0 as libc::c_int) << 20 as libc::c_int |
                  (0 as libc::c_int) << 16 as libc::c_int)) as libc::c_uint;
    let fresh47 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_22: *mut Gfx = fresh47;
    (*_g_22).words.w0 =
        (0xf7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_22).words.w1 =
        (((0 as libc::c_int) << 8 as libc::c_int & 0xf800 as libc::c_int |
              (0 as libc::c_int) << 3 as libc::c_int & 0x7c0 as libc::c_int |
              0 as libc::c_int >> 2 as libc::c_int & 0x3e as libc::c_int |
              1 as libc::c_int & 0x1 as libc::c_int) << 16 as libc::c_int |
             ((0 as libc::c_int) << 8 as libc::c_int & 0xf800 as libc::c_int |
                  (0 as libc::c_int) << 3 as libc::c_int &
                      0x7c0 as libc::c_int |
                  0 as libc::c_int >> 2 as libc::c_int & 0x3e as libc::c_int |
                  1 as libc::c_int & 0x1 as libc::c_int)) as libc::c_uint;
    let fresh48 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_23: *mut Gfx = fresh48;
    (*_g_23).words.w0 =
        (0xf6 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((width - 1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 10 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 14 as libc::c_int
            |
            ((height - 1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 10 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 2 as libc::c_int;
    (*_g_23).words.w1 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 10 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 14 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 10 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 2 as libc::c_int;
    let fresh49 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_24: *mut Gfx = fresh49;
    (*_g_24).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_24).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh50 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_25: *mut Gfx = fresh50;
    (*_g_25).words.w0 =
        (0xfe as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 3 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 21 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 2 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 19 as libc::c_int
            |
            ((1 as libc::c_int - 1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 12 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_25).words.w1 = img2 as libc::c_uint;
    viewport.vp.vtrans[0 as libc::c_int as usize] =
        (width * 2 as libc::c_int) as libc::c_short;
    viewport.vp.vscale[0 as libc::c_int as usize] =
        viewport.vp.vtrans[0 as libc::c_int as usize];
    viewport.vp.vtrans[1 as libc::c_int as usize] =
        (height * 2 as libc::c_int) as libc::c_short;
    viewport.vp.vscale[1 as libc::c_int as usize] =
        viewport.vp.vtrans[1 as libc::c_int as usize];
    let fresh51 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_26: *mut Gfx = fresh51;
    (*_g_26).words.w0 =
        (0xdc as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((::std::mem::size_of::<Vp>() as
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
            (8 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_26).words.w1 = &mut viewport as *mut Vp as libc::c_uint;
    guPerspective(perspMtx, &mut perspNorm, fovy,
                  width as f32_0 / height as f32_0, 10.0f32, 4000.0f32,
                  1.0f32);
    let fresh52 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_27: *mut Gfx = fresh52;
    (*_g_27).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0xe as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_27).words.w1 = perspNorm as libc::c_uint;
    let fresh53 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_28: *mut Gfx = fresh53;
    (*_g_28).words.w0 =
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
            (((0 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int) ^
                  0x1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_28).words.w1 = perspMtx as libc::c_uint;
    guLookAt(lookAtMtx, (*eye).x, (*eye).y, (*eye).z, (*at).x, (*at).y,
             (*at).z, 0.0f32, 1.0f32, 0.0f32);
    let fresh54 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_29: *mut Gfx = fresh54;
    (*_g_29).words.w0 =
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
            (((0 as libc::c_int | 0 as libc::c_int | 0x4 as libc::c_int) ^
                  0x1 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_29).words.w1 = lookAtMtx as libc::c_uint;
    sp12C[0 as libc::c_int as usize] = sword as u8_0;
    sp12C[1 as libc::c_int as usize] = shield as u8_0;
    func_800D1694((*pos).x, (*pos).y, (*pos).z, rot);
    Matrix_Scale(scale, scale, scale, MTXMODE_APPLY as libc::c_int as u8_0);
    let fresh55 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_30: *mut Gfx = fresh55;
    (*_g_30).words.w0 =
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
    (*_g_30).words.w1 = seg04 as libc::c_uint;
    let fresh56 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_31: *mut Gfx = fresh56;
    (*_g_31).words.w0 =
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
    (*_g_31).words.w1 = seg06 as libc::c_uint;
    let fresh57 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_32: *mut Gfx = fresh57;
    (*_g_32).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_32).words.w1 =
        (1 as libc::c_int * 24 as libc::c_int) as libc::c_uint;
    let fresh58 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_33: *mut Gfx = fresh58;
    (*_g_33).words.w0 =
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
            (((1 as libc::c_int * 24 as libc::c_int + 24 as libc::c_int) /
                  8 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (10 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_33).words.w1 =
        &mut *lights1.l.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut Light as libc::c_uint;
    let fresh59 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_34: *mut Gfx = fresh59;
    (*_g_34).words.w0 =
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
            (((2 as libc::c_int * 24 as libc::c_int + 24 as libc::c_int) /
                  8 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (10 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_34).words.w1 = &mut lights1.a as *mut Ambient as libc::c_uint;
    func_80093C80(globalCtx);
    let fresh60 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    (*__gfxCtx).polyOpa.p =
        Gfx_SetFog2(fresh60, 0 as libc::c_int, 0 as libc::c_int,
                    0 as libc::c_int, 0 as libc::c_int, 997 as libc::c_int,
                    1000 as libc::c_int);
    func_8002EABC(pos, &mut (*globalCtx).view.eye, &mut lightDir,
                  (*globalCtx).state.gfxCtx);
    let fresh61 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_35: *mut Gfx = fresh61;
    (*_g_35).words.w0 =
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
    (*_g_35).words.w1 = gCullBackDList.as_mut_ptr() as libc::c_uint;
    func_8008F470(globalCtx, (*skelAnime).skeleton, (*skelAnime).jointTable,
                  (*skelAnime).dListCount as s32, 0 as libc::c_int, tunic,
                  boots, 0 as libc::c_int,
                  Some(func_80091880 as
                           unsafe extern "C" fn(_: *mut GlobalContext, _: s32,
                                                _: *mut *mut Gfx,
                                                _: *mut Vec3f, _: *mut Vec3s,
                                                _: *mut libc::c_void) -> s32),
                  None, &mut sp12C as *mut [u8_0; 2] as *mut libc::c_void);
    let fresh62 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_36: *mut Gfx = fresh62;
    (*_g_36).words.w0 =
        (0xdf as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_36).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh63 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_37: *mut Gfx = fresh63;
    (*_g_37).words.w0 =
        (0xdf as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_37).words.w1 = 0 as libc::c_int as libc::c_uint;
    let mut _g_38: *mut Gfx = opaRef;
    (*_g_38).words.w0 =
        (0xde as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x1 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_38).words.w1 = (*__gfxCtx).polyOpa.p as libc::c_uint;
    let mut _g_39: *mut Gfx = xluRef;
    (*_g_39).words.w0 =
        (0xde as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x1 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_39).words.w1 = (*__gfxCtx).polyXlu.p as libc::c_uint;
    Graph_CloseDisps(dispRefs.as_mut_ptr(), (*globalCtx).state.gfxCtx,
                     b"../z_player_lib.c\x00" as *const u8 as
                         *const libc::c_char, 3288 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn func_8009214C(mut globalCtx: *mut GlobalContext,
                                       mut segment: *mut u8_0,
                                       mut skelAnime: *mut SkelAnime,
                                       mut pos: *mut Vec3f,
                                       mut rot: *mut Vec3s, mut scale: f32_0,
                                       mut sword: s32, mut tunic: s32,
                                       mut shield: s32, mut boots: s32) {
    static mut eye: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: -400.0f32,}; init };
    static mut at: Vec3f =
        { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
    let mut destTable: *mut Vec3s = 0 as *mut Vec3s;
    let mut srcTable: *mut Vec3s = 0 as *mut Vec3s;
    let mut i: s32 = 0;
    gSegments[4 as libc::c_int as usize] =
        segment.offset(0x3800 as libc::c_int as
                           isize).offset(-(0x80000000 as libc::c_uint as
                                               isize)) as u32_0;
    gSegments[6 as libc::c_int as usize] =
        segment.offset(0x8800 as libc::c_int as
                           isize).offset(-(0x80000000 as libc::c_uint as
                                               isize)) as u32_0;
    if !(gSaveContext.linkAge == 0 as libc::c_int) {
        if shield == PLAYER_SHIELD_DEKU as libc::c_int {
            srcTable = D_040020D0.as_mut_ptr()
        } else { srcTable = D_04002040.as_mut_ptr() }
    } else if sword == 3 as libc::c_int {
        srcTable = D_04002160.as_mut_ptr()
    } else if shield != PLAYER_SHIELD_NONE as libc::c_int {
        srcTable = D_04002280.as_mut_ptr()
    } else { srcTable = D_040021F0.as_mut_ptr() }
    srcTable =
        gSegments[((srcTable as u32_0) << 4 as libc::c_int >>
                       28 as libc::c_int) as
                      usize].wrapping_add(srcTable as u32_0 &
                                              0xffffff as libc::c_int as
                                                  libc::c_uint).wrapping_add(0x80000000
                                                                                 as
                                                                                 libc::c_uint)
            as *mut libc::c_void as *mut Vec3s;
    destTable = (*skelAnime).jointTable;
    i = 0 as libc::c_int;
    while i < (*skelAnime).limbCount as libc::c_int {
        let fresh64 = srcTable;
        srcTable = srcTable.offset(1);
        let fresh65 = destTable;
        destTable = destTable.offset(1);
        *fresh65 = *fresh64;
        i += 1
    }
    func_80091A24(globalCtx,
                  segment.offset(0x3800 as libc::c_int as isize) as
                      *mut libc::c_void,
                  segment.offset(0x8800 as libc::c_int as isize) as
                      *mut libc::c_void, skelAnime, pos, rot, scale, sword,
                  tunic, shield, boots, 64 as libc::c_int, 112 as libc::c_int,
                  &mut eye, &mut at, 60.0f32,
                  (*(*globalCtx).state.gfxCtx).curFrameBuffer as
                      *mut libc::c_void,
                  (*(*globalCtx).state.gfxCtx).curFrameBuffer.offset(0x1c00 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)
                      as *mut libc::c_void);
}
