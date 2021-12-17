#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn LogUtils_LogHexDump(ptr: *mut libc::c_void, size0: s32);
    #[no_mangle]
    fn LogUtils_LogThreadId(name: *const libc::c_char, line: s32);
    #[no_mangle]
    fn sprintf(dst: *mut libc::c_char, fmt: *const libc::c_char, _: ...)
     -> s32;
    #[no_mangle]
    fn osSendMesg(mq: *mut OSMesgQueue, mesg: OSMesg, flag: s32) -> s32;
    #[no_mangle]
    fn osRecvMesg(mq: *mut OSMesgQueue, msg: *mut OSMesg, flag: s32) -> s32;
    #[no_mangle]
    fn bzero(__s: *mut libc::c_void, __n: u32_0);
    #[no_mangle]
    fn osViGetNextFramebuffer() -> *mut libc::c_void;
    #[no_mangle]
    fn osCreateMesgQueue(mq: *mut OSMesgQueue, msg: *mut OSMesg, count: s32);
    #[no_mangle]
    fn osViSwapBuffer(vaddr: *mut libc::c_void);
    #[no_mangle]
    fn osGetTime() -> OSTime;
    #[no_mangle]
    fn Overlay_LoadGameState(overlayEntry: *mut GameStateOverlay);
    #[no_mangle]
    fn Overlay_FreeGameState(overlayEntry: *mut GameStateOverlay);
    #[no_mangle]
    fn PreNmiBuff_IsResetting(this: *mut PreNmiBuff) -> u32_0;
    #[no_mangle]
    fn PreNMI_Init(thisx: *mut GameState);
    #[no_mangle]
    fn Gameplay_Init(thisx: *mut GameState);
    #[no_mangle]
    fn THGA_Ct(thga: *mut TwoHeadGfxArena, start: *mut Gfx, size: u32_0);
    #[no_mangle]
    fn THGA_IsCrash(thga: *mut TwoHeadGfxArena) -> u32_0;
    #[no_mangle]
    fn THGA_AllocEnd(thga: *mut TwoHeadGfxArena, size: u32_0) -> *mut Gfx;
    #[no_mangle]
    fn TitleSetup_Init(gameState: *mut GameState);
    #[no_mangle]
    fn GameState_ReqPadData(gameState: *mut GameState);
    #[no_mangle]
    fn GameState_Update(gameState: *mut GameState);
    #[no_mangle]
    fn GameState_Init(gameState: *mut GameState, init: GameStateFunc,
                      gfxCtx: *mut GraphicsContext);
    #[no_mangle]
    fn GameState_Destroy(gameState: *mut GameState);
    #[no_mangle]
    fn GameState_GetInit(gameState: *mut GameState) -> GameStateFunc;
    #[no_mangle]
    fn GameState_IsRunning(gameState: *mut GameState) -> u32_0;
    #[no_mangle]
    fn Sched_SendEntryMsg(sc: *mut SchedContext);
    #[no_mangle]
    fn SysCfb_GetFbPtr(idx: s32) -> u32_0;
    #[no_mangle]
    fn SysUcode_GetUCodeBoot() -> u32_0;
    #[no_mangle]
    fn SysUcode_GetUCodeBootSize() -> u32_0;
    #[no_mangle]
    fn SysUcode_GetUCode() -> u32_0;
    #[no_mangle]
    fn SysUcode_GetUCodeData() -> u32_0;
    #[no_mangle]
    fn func_800D31F0();
    #[no_mangle]
    fn func_800D3210();
    #[no_mangle]
    fn Fault_AddClient(_: *mut FaultClient, _: *mut libc::c_void,
                       _: *mut libc::c_void, _: *mut libc::c_void);
    #[no_mangle]
    fn Fault_RemoveClient(_: *mut FaultClient);
    #[no_mangle]
    fn Fault_WaitForInput();
    #[no_mangle]
    fn Fault_AddHungupAndCrashImpl(_: *const libc::c_char,
                                   _: *const libc::c_char);
    #[no_mangle]
    fn Fault_AddHungupAndCrash(_: *const libc::c_char, _: u32_0);
    #[no_mangle]
    fn UCodeDisas_Init(_: *mut UCodeDisas);
    #[no_mangle]
    fn UCodeDisas_Destroy(_: *mut UCodeDisas);
    #[no_mangle]
    fn UCodeDisas_RegisterUCode(_: *mut UCodeDisas, _: s32,
                                _: *mut UCodeInfo);
    #[no_mangle]
    fn UCodeDisas_SetCurUCode(_: *mut UCodeDisas, _: *mut libc::c_void);
    #[no_mangle]
    static mut gGameInfo: *mut GameInfo;
    #[no_mangle]
    static mut D_80155F50: [u8_0; 0];
    #[no_mangle]
    static mut D_80113070: [u8_0; 0];
    #[no_mangle]
    static mut gGfxPools: [GfxPool; 2];
    #[no_mangle]
    static mut gGameStateOverlayTable: [GameStateOverlay; 6];
    #[no_mangle]
    fn FileChoose_Init(thisx: *mut GameState);
    #[no_mangle]
    fn Opening_Init(thisx: *mut GameState);
    #[no_mangle]
    fn Title_Init(thisx: *mut GameState);
    #[no_mangle]
    fn Select_Init(thisx: *mut GameState);
    #[no_mangle]
    static mut gViConfigYScale: f32_0;
    #[no_mangle]
    static mut gViConfigXScale: f32_0;
    #[no_mangle]
    static mut gViConfigFeatures: u32_0;
    #[no_mangle]
    static mut gSchedContext: SchedContext;
    #[no_mangle]
    static mut gGfxSPTaskYieldBuffer: [u8_0; 3072];
    #[no_mangle]
    static mut gGfxSPTaskOutputBuffer: [u64_0; 12288];
    #[no_mangle]
    static mut gGfxSPTaskStack: [u8_0; 1024];
    #[no_mangle]
    static mut D_8016A558: OSTime;
    #[no_mangle]
    static mut D_8016A520: OSTime;
    #[no_mangle]
    static mut D_8016A550: OSTime;
    #[no_mangle]
    fn osStopTimer(timer: *mut OSTimer) -> s32;
    #[no_mangle]
    fn osSetTimer(timer: *mut OSTimer, countdown: OSTime, interval: OSTime,
                  mq: *mut OSMesgQueue, msg: OSMesg) -> s32;
    #[no_mangle]
    static mut D_8016A528: OSTime;
    #[no_mangle]
    static mut gAppNmiBufferPtr: *mut PreNmiBuff;
    #[no_mangle]
    static mut gIsCtrlr2Valid: u32_0;
    #[no_mangle]
    static mut gSaveContext: SaveContext;
    #[no_mangle]
    static mut D_8016A548: OSTime;
    #[no_mangle]
    static mut gRDPTotalTime: OSTime;
    #[no_mangle]
    static mut gRSPAudioTotalTime: OSTime;
    #[no_mangle]
    static mut gRSPGFXTotalTime: OSTime;
    #[no_mangle]
    static mut D_8016A540: OSTime;
    #[no_mangle]
    static mut D_8016A530: OSTime;
    #[no_mangle]
    static mut D_8016A538: OSTime;
    #[no_mangle]
    fn func_800F3054();
    #[no_mangle]
    fn SystemArena_FreeDebug(ptr: *mut libc::c_void,
                             file: *const libc::c_char, line: s32);
    #[no_mangle]
    fn SystemArena_MallocDebug(size: u32_0, file: *const libc::c_char,
                               line: s32) -> *mut libc::c_void;
}
pub type s8 = libc::c_schar;
pub type u8_0 = libc::c_uchar;
pub type s16 = libc::c_short;
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
pub type u64_0 = libc::c_ulonglong;
pub type vu32 = u32_0;
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
pub type size_t = libc::c_ulong;
pub type OSPri = s32;
pub type OSId = s32;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __OSfp {
    pub f: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
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
pub struct OSTimer {
    pub next: *mut OSTimer,
    pub prev: *mut OSTimer,
    pub interval: OSTime,
    pub value: OSTime,
    pub mq: *mut OSMesgQueue,
    pub msg: OSMesg,
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
pub struct Input {
    pub cur: OSContPad,
    pub prev: OSContPad,
    pub press: OSContPad,
    pub rel: OSContPad,
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
pub struct GfxPool {
    pub headMagic: u16_0,
    pub polyOpaBuffer: [Gfx; 6112],
    pub polyXluBuffer: [Gfx; 2048],
    pub overlayBuffer: [Gfx; 1024],
    pub workBuffer: [Gfx; 128],
    pub unusedBuffer: [Gfx; 32],
    pub tailMagic: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SelectContext {
    pub state: GameState,
    pub view: View,
    pub count: s32,
    pub scenes: *mut SceneSelectEntry,
    pub currentScene: s32,
    pub pageDownIndex: s32,
    pub pageDownStops: [s32; 7],
    pub unk_1FC: [libc::c_char; 12],
    pub opt: s32,
    pub topDisplayedScene: s32,
    pub unk_210: [libc::c_char; 12],
    pub verticalInputAccumulator: s32,
    pub verticalInput: s32,
    pub timerUp: s32,
    pub timerDown: s32,
    pub lockUp: s32,
    pub lockDown: s32,
    pub unk_234: s32,
    pub staticSegment: *mut u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SceneSelectEntry {
    pub name: *mut libc::c_char,
    pub loadFunc: Option<unsafe extern "C" fn(_: *mut SelectContext, _: s32)
                             -> ()>,
    pub entranceIndex: s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GameStateOverlay {
    pub loadedRamAddr: *mut libc::c_void,
    pub vromStart: u32_0,
    pub vromEnd: u32_0,
    pub vramStart: *mut libc::c_void,
    pub vramEnd: *mut libc::c_void,
    pub unk_14: *mut libc::c_void,
    pub init: *mut libc::c_void,
    pub destroy: *mut libc::c_void,
    pub unk_20: *mut libc::c_void,
    pub unk_24: *mut libc::c_void,
    pub unk_28: s32,
    pub instanceSize: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PreNMIContext {
    pub state: GameState,
    pub timer: u32_0,
    pub unk_A8: s32,
}
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
pub struct IrqMgrClient {
    pub prev: *mut IrqMgrClient,
    pub queue: *mut OSMesgQueue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SchedContext {
    pub interruptQ: OSMesgQueue,
    pub intBuf: [OSMesg; 8],
    pub cmdQ: OSMesgQueue,
    pub cmdMsgBuf: [OSMesg; 8],
    pub thread: OSThread,
    pub audioListHead: *mut OSScTask,
    pub gfxListHead: *mut OSScTask,
    pub audioListTail: *mut OSScTask,
    pub gfxListTail: *mut OSScTask,
    pub curRSPTask: *mut OSScTask,
    pub curRDPTask: *mut OSScTask,
    pub retraceCnt: s32,
    pub doAudio: s32,
    pub curBuf: *mut CfbInfo,
    pub pendingSwapBuf1: *mut CfbInfo,
    pub pendingSwapBuf2: *mut CfbInfo,
    pub unk_24C: s32,
    pub irqClient: IrqMgrClient,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PreNmiBuff {
    pub resetting: u32_0,
    pub resetCount: u32_0,
    pub duration: OSTime,
    pub resetTime: OSTime,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UCodeInfo {
    pub type_0: u32_0,
    pub ptr: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UCodeDisas {
    pub segments: [u32_0; 16],
    pub dlStack: [*mut Gfx; 18],
    pub dlDepth: s32,
    pub dlCnt: u32_0,
    pub vtxCnt: u32_0,
    pub spvtxCnt: u32_0,
    pub tri1Cnt: u32_0,
    pub tri2Cnt: u32_0,
    pub quadCnt: u32_0,
    pub lineCnt: u32_0,
    pub loaducodeCnt: u32_0,
    pub pipeSyncRequired: u32_0,
    pub tileSyncRequired: u32_0,
    pub loadSyncRequired: u32_0,
    pub syncErr: u32_0,
    pub enableLog: s32,
    pub ucodeType: s32,
    pub ucodeInfoCount: s32,
    pub ucodeInfo: *mut UCodeInfo,
    pub modeH: u32_0,
    pub modeL: u32_0,
    pub geometryMode: u32_0,
}
#[no_mangle]
pub static mut sGraphUpdateTime: OSTime = 0;
#[no_mangle]
pub static mut sGraphSetTaskTime: OSTime = 0;
#[no_mangle]
pub static mut sGraphFaultClient: FaultClient =
    FaultClient{next: 0 as *const FaultClient as *mut FaultClient,
                callback: 0,
                param1: 0,
                param2: 0,};
#[no_mangle]
pub static mut sGraphCfbInfos: [CfbInfo; 3] =
    [CfbInfo{fb1: 0 as *const u16_0 as *mut u16_0,
             swapBuffer: 0 as *const u16_0 as *mut u16_0,
             viMode: 0 as *const OSViMode as *mut OSViMode,
             features: 0,
             unk_10: 0,
             updateRate: 0,
             updateRate2: 0,
             unk_13: 0,
             xScale: 0.,
             yScale: 0.,}; 3];
#[no_mangle]
pub static mut sGraphUcodeFaultClient: FaultClient =
    FaultClient{next: 0 as *const FaultClient as *mut FaultClient,
                callback: 0,
                param1: 0,
                param2: 0,};
// clang-format off
#[no_mangle]
pub static mut D_8012D230: [UCodeInfo; 3] =
    unsafe {
        [{
             let mut init =
                 UCodeInfo{type_0: 1 as libc::c_int as u32_0,
                           ptr:
                               D_80155F50.as_ptr() as *mut _ as
                                   *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 UCodeInfo{type_0: 2 as libc::c_int as u32_0,
                           ptr:
                               0 as *const libc::c_void as
                                   *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 UCodeInfo{type_0: 3 as libc::c_int as u32_0,
                           ptr:
                               D_80113070.as_ptr() as *mut _ as
                                   *mut libc::c_void,};
             init
         }]
    };
#[no_mangle]
pub static mut D_8012D248: [UCodeInfo; 3] =
    unsafe {
        [{
             let mut init =
                 UCodeInfo{type_0: 1 as libc::c_int as u32_0,
                           ptr:
                               D_80155F50.as_ptr() as *mut _ as
                                   *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 UCodeInfo{type_0: 2 as libc::c_int as u32_0,
                           ptr:
                               0 as *const libc::c_void as
                                   *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 UCodeInfo{type_0: 3 as libc::c_int as u32_0,
                           ptr:
                               D_80113070.as_ptr() as *mut _ as
                                   *mut libc::c_void,};
             init
         }]
    };
// clang-format on
#[no_mangle]
pub unsafe extern "C" fn Graph_FaultClient() {
    let mut nextFb: *mut libc::c_void =
        osViGetNextFramebuffer(); // "RCP did not return."
    let mut newFb: *mut libc::c_void =
        if SysCfb_GetFbPtr(0 as libc::c_int) != nextFb as u32_0 {
            SysCfb_GetFbPtr(0 as libc::c_int)
        } else { SysCfb_GetFbPtr(1 as libc::c_int) } as
            *mut libc::c_void; // Necessary to match stack usage
    osViSwapBuffer(newFb);
    Fault_WaitForInput();
    osViSwapBuffer(nextFb);
}
#[no_mangle]
pub unsafe extern "C" fn Graph_DisassembleUCode(mut workBuf: *mut Gfx) {
    let mut disassembler: UCodeDisas =
        UCodeDisas{segments: [0; 16],
                   dlStack: [0 as *mut Gfx; 18],
                   dlDepth: 0,
                   dlCnt: 0,
                   vtxCnt: 0,
                   spvtxCnt: 0,
                   tri1Cnt: 0,
                   tri2Cnt: 0,
                   quadCnt: 0,
                   lineCnt: 0,
                   loaducodeCnt: 0,
                   pipeSyncRequired: 0,
                   tileSyncRequired: 0,
                   loadSyncRequired: 0,
                   syncErr: 0,
                   enableLog: 0,
                   ucodeType: 0,
                   ucodeInfoCount: 0,
                   ucodeInfo: 0 as *mut UCodeInfo,
                   modeH: 0,
                   modeL: 0,
                   geometryMode: 0,};
    if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 80 as libc::c_int) as usize]
           as libc::c_int == 7 as libc::c_int &&
           (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 81 as libc::c_int) as
                                 usize] as libc::c_int != 0 as libc::c_int {
        UCodeDisas_Init(&mut disassembler);
        disassembler.enableLog =
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 83 as libc::c_int) as
                                  usize] as s32;
        UCodeDisas_RegisterUCode(&mut disassembler,
                                 (::std::mem::size_of::<[UCodeInfo; 3]>() as
                                      libc::c_ulong).wrapping_div(::std::mem::size_of::<UCodeInfo>()
                                                                      as
                                                                      libc::c_ulong)
                                     as s32, D_8012D230.as_mut_ptr());
        UCodeDisas_SetCurUCode(&mut disassembler,
                               D_80155F50.as_mut_ptr() as *mut libc::c_void);
        UCodeDisas_Disassemble(&mut disassembler, workBuf);
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 93 as libc::c_int) as
                              usize] = disassembler.dlCnt as s16;
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 84 as libc::c_int) as
                              usize] =
            disassembler.tri2Cnt.wrapping_mul(2 as libc::c_int as
                                                  libc::c_uint).wrapping_add(disassembler.tri1Cnt).wrapping_add(disassembler.quadCnt.wrapping_mul(2
                                                                                                                                                      as
                                                                                                                                                      libc::c_int
                                                                                                                                                      as
                                                                                                                                                      libc::c_uint)).wrapping_add(disassembler.lineCnt)
                as s16;
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 85 as libc::c_int) as
                              usize] = disassembler.vtxCnt as s16;
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 86 as libc::c_int) as
                              usize] = disassembler.spvtxCnt as s16;
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 87 as libc::c_int) as
                              usize] = disassembler.tri1Cnt as s16;
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 88 as libc::c_int) as
                              usize] = disassembler.tri2Cnt as s16;
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 89 as libc::c_int) as
                              usize] = disassembler.quadCnt as s16;
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 90 as libc::c_int) as
                              usize] = disassembler.lineCnt as s16;
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 91 as libc::c_int) as
                              usize] = disassembler.syncErr as s16;
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 92 as libc::c_int) as
                              usize] = disassembler.loaducodeCnt as s16;
        if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 82 as libc::c_int) as
                                 usize] as libc::c_int == 1 as libc::c_int ||
               (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                      16 as libc::c_int + 82 as libc::c_int)
                                     as usize] as libc::c_int ==
                   2 as libc::c_int {
            osSyncPrintf(b"vtx_cnt=%d\n\x00" as *const u8 as
                             *const libc::c_char, disassembler.vtxCnt);
            osSyncPrintf(b"spvtx_cnt=%d\n\x00" as *const u8 as
                             *const libc::c_char, disassembler.spvtxCnt);
            osSyncPrintf(b"tri1_cnt=%d\n\x00" as *const u8 as
                             *const libc::c_char, disassembler.tri1Cnt);
            osSyncPrintf(b"tri2_cnt=%d\n\x00" as *const u8 as
                             *const libc::c_char, disassembler.tri2Cnt);
            osSyncPrintf(b"quad_cnt=%d\n\x00" as *const u8 as
                             *const libc::c_char, disassembler.quadCnt);
            osSyncPrintf(b"line_cnt=%d\n\x00" as *const u8 as
                             *const libc::c_char, disassembler.lineCnt);
            osSyncPrintf(b"sync_err=%d\n\x00" as *const u8 as
                             *const libc::c_char, disassembler.syncErr);
            osSyncPrintf(b"loaducode_cnt=%d\n\x00" as *const u8 as
                             *const libc::c_char, disassembler.loaducodeCnt);
            osSyncPrintf(b"dl_depth=%d\n\x00" as *const u8 as
                             *const libc::c_char, disassembler.dlDepth);
            osSyncPrintf(b"dl_cnt=%d\n\x00" as *const u8 as
                             *const libc::c_char, disassembler.dlCnt);
        }
        UCodeDisas_Destroy(&mut disassembler);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Graph_UCodeFaultClient(mut workBuf: *mut Gfx) {
    let mut disassembler: UCodeDisas =
        UCodeDisas{segments: [0; 16],
                   dlStack: [0 as *mut Gfx; 18],
                   dlDepth: 0,
                   dlCnt: 0,
                   vtxCnt: 0,
                   spvtxCnt: 0,
                   tri1Cnt: 0,
                   tri2Cnt: 0,
                   quadCnt: 0,
                   lineCnt: 0,
                   loaducodeCnt: 0,
                   pipeSyncRequired: 0,
                   tileSyncRequired: 0,
                   loadSyncRequired: 0,
                   syncErr: 0,
                   enableLog: 0,
                   ucodeType: 0,
                   ucodeInfoCount: 0,
                   ucodeInfo: 0 as *mut UCodeInfo,
                   modeH: 0,
                   modeL: 0,
                   geometryMode: 0,};
    UCodeDisas_Init(&mut disassembler);
    disassembler.enableLog = 1 as libc::c_int;
    UCodeDisas_RegisterUCode(&mut disassembler,
                             (::std::mem::size_of::<[UCodeInfo; 3]>() as
                                  libc::c_ulong).wrapping_div(::std::mem::size_of::<UCodeInfo>()
                                                                  as
                                                                  libc::c_ulong)
                                 as s32, D_8012D248.as_mut_ptr());
    UCodeDisas_SetCurUCode(&mut disassembler,
                           D_80155F50.as_mut_ptr() as *mut libc::c_void);
    UCodeDisas_Disassemble(&mut disassembler, workBuf);
    UCodeDisas_Destroy(&mut disassembler);
}
#[no_mangle]
pub unsafe extern "C" fn Graph_InitTHGA(mut gfxCtx: *mut GraphicsContext) {
    let mut pool: *mut GfxPool =
        &mut *gGfxPools.as_mut_ptr().offset(((*gfxCtx).gfxPoolIdx &
                                                 1 as libc::c_int as
                                                     libc::c_uint) as isize)
            as *mut GfxPool;
    (*pool).headMagic = 0x1234 as libc::c_int as u16_0;
    (*pool).tailMagic = 0x5678 as libc::c_int as u16_0;
    THGA_Ct(&mut (*gfxCtx).polyOpa, (*pool).polyOpaBuffer.as_mut_ptr(),
            ::std::mem::size_of::<[Gfx; 6112]>() as libc::c_ulong);
    THGA_Ct(&mut (*gfxCtx).polyXlu, (*pool).polyXluBuffer.as_mut_ptr(),
            ::std::mem::size_of::<[Gfx; 2048]>() as libc::c_ulong);
    THGA_Ct(&mut (*gfxCtx).overlay, (*pool).overlayBuffer.as_mut_ptr(),
            ::std::mem::size_of::<[Gfx; 1024]>() as libc::c_ulong);
    THGA_Ct(&mut (*gfxCtx).work, (*pool).workBuffer.as_mut_ptr(),
            ::std::mem::size_of::<[Gfx; 128]>() as libc::c_ulong);
    (*gfxCtx).polyOpaBuffer = (*pool).polyOpaBuffer.as_mut_ptr();
    (*gfxCtx).polyXluBuffer = (*pool).polyXluBuffer.as_mut_ptr();
    (*gfxCtx).overlayBuffer = (*pool).overlayBuffer.as_mut_ptr();
    (*gfxCtx).workBuffer = (*pool).workBuffer.as_mut_ptr();
    (*gfxCtx).curFrameBuffer =
        SysCfb_GetFbPtr((*gfxCtx).fbIdx % 2 as libc::c_int) as *mut u16_0;
    (*gfxCtx).unk_014 = 0 as libc::c_int as u32_0;
}
#[no_mangle]
pub unsafe extern "C" fn Graph_GetNextGameState(mut gameState: *mut GameState)
 -> *mut GameStateOverlay {
    let mut gameStateInitFunc: *mut libc::c_void =
        ::std::mem::transmute::<GameStateFunc,
                                *mut libc::c_void>(GameState_GetInit(gameState));
    if gameStateInitFunc ==
           ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                   *mut GameState)
                                              -> ()>,
                                   *mut libc::c_void>(Some(TitleSetup_Init as
                                                               unsafe extern "C" fn(_:
                                                                                        *mut GameState)
                                                                   -> ())) {
        return &mut *gGameStateOverlayTable.as_mut_ptr().offset(0 as
                                                                    libc::c_int
                                                                    as isize)
                   as *mut GameStateOverlay
    }
    if gameStateInitFunc ==
           ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                   *mut GameState)
                                              -> ()>,
                                   *mut libc::c_void>(Some(Select_Init as
                                                               unsafe extern "C" fn(_:
                                                                                        *mut GameState)
                                                                   -> ())) {
        return &mut *gGameStateOverlayTable.as_mut_ptr().offset(1 as
                                                                    libc::c_int
                                                                    as isize)
                   as *mut GameStateOverlay
    }
    if gameStateInitFunc ==
           ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                   *mut GameState)
                                              -> ()>,
                                   *mut libc::c_void>(Some(Title_Init as
                                                               unsafe extern "C" fn(_:
                                                                                        *mut GameState)
                                                                   -> ())) {
        return &mut *gGameStateOverlayTable.as_mut_ptr().offset(2 as
                                                                    libc::c_int
                                                                    as isize)
                   as *mut GameStateOverlay
    }
    if gameStateInitFunc ==
           ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                   *mut GameState)
                                              -> ()>,
                                   *mut libc::c_void>(Some(Gameplay_Init as
                                                               unsafe extern "C" fn(_:
                                                                                        *mut GameState)
                                                                   -> ())) {
        return &mut *gGameStateOverlayTable.as_mut_ptr().offset(3 as
                                                                    libc::c_int
                                                                    as isize)
                   as *mut GameStateOverlay
    }
    if gameStateInitFunc ==
           ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                   *mut GameState)
                                              -> ()>,
                                   *mut libc::c_void>(Some(Opening_Init as
                                                               unsafe extern "C" fn(_:
                                                                                        *mut GameState)
                                                                   -> ())) {
        return &mut *gGameStateOverlayTable.as_mut_ptr().offset(4 as
                                                                    libc::c_int
                                                                    as isize)
                   as *mut GameStateOverlay
    }
    if gameStateInitFunc ==
           ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                   *mut GameState)
                                              -> ()>,
                                   *mut libc::c_void>(Some(FileChoose_Init as
                                                               unsafe extern "C" fn(_:
                                                                                        *mut GameState)
                                                                   -> ())) {
        return &mut *gGameStateOverlayTable.as_mut_ptr().offset(5 as
                                                                    libc::c_int
                                                                    as isize)
                   as *mut GameStateOverlay
    }
    LogUtils_LogThreadId(b"../graph.c\x00" as *const u8 as
                             *const libc::c_char, 696 as libc::c_int);
    osSyncPrintf(b"game_init_func = %08x\n\x00" as *const u8 as
                     *const libc::c_char, gameStateInitFunc);
    return 0 as *mut GameStateOverlay;
}
#[no_mangle]
pub unsafe extern "C" fn Graph_Init(mut gfxCtx: *mut GraphicsContext) {
    bzero(gfxCtx as *mut libc::c_void,
          ::std::mem::size_of::<GraphicsContext>() as libc::c_ulong);
    (*gfxCtx).gfxPoolIdx = 0 as libc::c_int as u32_0;
    (*gfxCtx).fbIdx = 0 as libc::c_int;
    (*gfxCtx).viMode = 0 as *mut OSViMode;
    (*gfxCtx).viFeatures = gViConfigFeatures;
    (*gfxCtx).xScale = gViConfigXScale;
    (*gfxCtx).yScale = gViConfigYScale;
    osCreateMesgQueue(&mut (*gfxCtx).queue, (*gfxCtx).msgBuff.as_mut_ptr(),
                      (::std::mem::size_of::<[OSMesg; 8]>() as
                           libc::c_ulong).wrapping_div(::std::mem::size_of::<OSMesg>()
                                                           as libc::c_ulong)
                          as s32);
    func_800D31F0();
    Fault_AddClient(&mut sGraphFaultClient,
                    ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                       -> ()>,
                                            *mut libc::c_void>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                ->
                                                                                                    (),
                                                                                            unsafe extern "C" fn()
                                                                                                ->
                                                                                                    ()>(Graph_FaultClient))),
                    0 as *mut libc::c_void, 0 as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn Graph_Destroy(mut gfxCtx: *mut GraphicsContext) {
    func_800D3210();
    Fault_RemoveClient(&mut sGraphFaultClient);
}
#[no_mangle]
pub unsafe extern "C" fn Graph_TaskSet00(mut gfxCtx: *mut GraphicsContext) {
    static mut D_8012D260: *mut Gfx = 0 as *const Gfx as *mut Gfx;
    static mut sGraphCfbInfoIdx: s32 = 0 as libc::c_int;
    let mut time: OSTime = 0;
    let mut timer: OSTimer =
        OSTimer{next: 0 as *mut OSTimer,
                prev: 0 as *mut OSTimer,
                interval: 0,
                value: 0,
                mq: 0 as *mut OSMesgQueue,
                msg: 0 as *mut libc::c_void,};
    let mut msg: OSMesg = 0 as *mut libc::c_void;
    let mut task: *mut OSTask_t = &mut (*gfxCtx).task.list.t;
    let mut scTask: *mut OSScTask = &mut (*gfxCtx).task;
    let mut cfb: *mut CfbInfo = 0 as *mut CfbInfo;
    let mut pad1: s32 = 0;
    ::std::ptr::write_volatile(&mut D_8016A528 as *mut OSTime,
                               osGetTime().wrapping_sub(sGraphSetTaskTime).wrapping_sub(D_8016A558));
    osSetTimer(&mut timer,
               (3000000 as libc::c_int as
                    u64_0).wrapping_mul((62500000 as libc::c_longlong *
                                             3 as libc::c_int as
                                                 libc::c_longlong /
                                             4 as libc::c_int as
                                                 libc::c_longlong /
                                             15625 as libc::c_longlong) as
                                            libc::c_ulonglong).wrapping_div((1000000
                                                                                 as
                                                                                 libc::c_longlong
                                                                                 /
                                                                                 15625
                                                                                     as
                                                                                     libc::c_longlong)
                                                                                as
                                                                                libc::c_ulonglong),
               0 as libc::c_int as OSTime, &mut (*gfxCtx).queue,
               666 as libc::c_int as OSMesg);
    osRecvMesg(&mut (*gfxCtx).queue, &mut msg, 1 as libc::c_int);
    osStopTimer(&mut timer);
    if msg == 666 as libc::c_int as OSMesg {
        osSyncPrintf(b"\x1b[31m\x00" as *const u8 as *const libc::c_char);
        osSyncPrintf(b"RCP\xe3\x81\x8c\xe5\xb8\xb0\xe3\x81\xa3\xe3\x81\xa6\xe3\x81\x8d\xe3\x81\xbe\xe3\x81\x9b\xe3\x82\x93\xe3\x81\xa7\xe3\x81\x97\xe3\x81\x9f\xe3\x80\x82\x00"
                         as *const u8 as *const libc::c_char);
        osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
        LogUtils_LogHexDump((0x4040000 as libc::c_int as libc::c_uint |
                                 0xa0000000 as libc::c_uint) as *mut u32_0 as
                                *mut libc::c_void, 0x20 as libc::c_int);
        LogUtils_LogHexDump((0xa4100000 as
                                 libc::c_uint).wrapping_add(0 as libc::c_int
                                                                as
                                                                libc::c_uint)
                                as *mut vu32 as *mut libc::c_void,
                            0x20 as libc::c_int);
        LogUtils_LogHexDump(gGfxSPTaskYieldBuffer.as_mut_ptr() as
                                *mut libc::c_void,
                            ::std::mem::size_of::<[u8_0; 3072]>() as
                                libc::c_ulong as s32);
        (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 6 as libc::c_int) as usize]
            = -(1 as libc::c_int) as s16;
        if !D_8012D260.is_null() {
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 80 as libc::c_int) as
                                  usize] = 7 as libc::c_int as s16;
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 81 as libc::c_int) as
                                  usize] = 1 as libc::c_int as s16;
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 83 as libc::c_int) as
                                  usize] = 2 as libc::c_int as s16;
            D_8012D260 = D_8012D260;
            Graph_DisassembleUCode(D_8012D260);
        }
        Fault_AddHungupAndCrashImpl(b"RCP is HUNG UP!!\x00" as *const u8 as
                                        *const libc::c_char,
                                    b"Oh! MY GOD!!\x00" as *const u8 as
                                        *const libc::c_char);
    }
    osRecvMesg(&mut (*gfxCtx).queue, &mut msg, 0 as libc::c_int);
    D_8012D260 = (*gfxCtx).workBuffer;
    if (*gfxCtx).callback.is_some() {
        (*gfxCtx).callback.expect("non-null function pointer")(gfxCtx,
                                                               (*gfxCtx).callbackParam);
    }
    time = osGetTime();
    if D_8016A550 != 0 as libc::c_int as libc::c_ulonglong {
        ::std::ptr::write_volatile(&mut D_8016A558 as *mut OSTime,
                                   D_8016A558.wrapping_add(time).wrapping_sub(D_8016A550));
        ::std::ptr::write_volatile(&mut D_8016A550 as *mut OSTime, time)
    }
    ::std::ptr::write_volatile(&mut D_8016A520 as *mut OSTime, D_8016A558);
    ::std::ptr::write_volatile(&mut D_8016A558 as *mut OSTime,
                               0 as libc::c_int as OSTime);
    sGraphSetTaskTime = osGetTime();
    (*task).type_0 = 1 as libc::c_int as u32_0;
    (*task).flags = 0x4 as libc::c_int as u32_0;
    (*task).ucode_boot = SysUcode_GetUCodeBoot() as *mut u64_0;
    (*task).ucode_boot_size = SysUcode_GetUCodeBootSize();
    (*task).ucode = SysUcode_GetUCode() as *mut u64_0;
    (*task).ucode_data = SysUcode_GetUCodeData() as *mut u64_0;
    (*task).ucode_size = 0x1000 as libc::c_int as u32_0;
    (*task).ucode_data_size = 0x800 as libc::c_int as u32_0;
    (*task).dram_stack = gGfxSPTaskStack.as_mut_ptr() as *mut u64_0;
    (*task).dram_stack_size =
        ::std::mem::size_of::<[u8_0; 1024]>() as libc::c_ulong;
    (*task).output_buff = gGfxSPTaskOutputBuffer.as_mut_ptr();
    (*task).output_buff_size =
        (gGfxSPTaskOutputBuffer.as_mut_ptr() as
             *mut u8_0).offset(::std::mem::size_of::<[u64_0; 12288]>() as
                                   libc::c_ulong as isize) as *mut u64_0;
    (*task).data_ptr = (*gfxCtx).workBuffer as *mut u64_0;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), gfxCtx,
                    b"../graph.c\x00" as *const u8 as *const libc::c_char,
                    828 as libc::c_int);
    (*task).data_size =
        ((*__gfxCtx).work.p as
             u32_0).wrapping_sub((*gfxCtx).workBuffer as u32_0);
    Graph_CloseDisps(dispRefs.as_mut_ptr(), gfxCtx,
                     b"../graph.c\x00" as *const u8 as *const libc::c_char,
                     830 as libc::c_int);
    let mut pad2: s32 = 0;
    (*task).yield_data_ptr = gGfxSPTaskYieldBuffer.as_mut_ptr() as *mut u64_0;
    (*task).yield_data_size =
        ::std::mem::size_of::<[u8_0; 3072]>() as libc::c_ulong;
    (*scTask).next = 0 as *mut OSScTask;
    (*scTask).flags =
        (0x3 as libc::c_int | 0x40 as libc::c_int | 0x20 as libc::c_int) as
            u32_0;
    if (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 33 as libc::c_int) as usize]
           as libc::c_int & 1 as libc::c_int != 0 {
        (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 33 as libc::c_int) as
                              usize] =
            ((*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                    16 as libc::c_int + 33 as libc::c_int) as
                                   usize] as libc::c_int &
                 !(1 as libc::c_int)) as s16;
        (*scTask).flags &= !(0x40 as libc::c_int) as libc::c_uint;
        (*gfxCtx).fbIdx -= 1
    }
    (*scTask).msgQ = &mut (*gfxCtx).queue;
    (*scTask).msg = 0 as *mut libc::c_void;
    let fresh0 = sGraphCfbInfoIdx;
    sGraphCfbInfoIdx = sGraphCfbInfoIdx + 1;
    cfb =
        &mut *sGraphCfbInfos.as_mut_ptr().offset(fresh0 as isize) as
            *mut CfbInfo;
    (*cfb).fb1 = (*gfxCtx).curFrameBuffer;
    (*cfb).swapBuffer = (*gfxCtx).curFrameBuffer;
    (*cfb).viMode = (*gfxCtx).viMode;
    (*cfb).features = (*gfxCtx).viFeatures;
    (*cfb).xScale = (*gfxCtx).xScale;
    (*cfb).yScale = (*gfxCtx).yScale;
    (*cfb).unk_10 = 0 as libc::c_int as u8_0;
    (*cfb).updateRate =
        (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 30 as libc::c_int) as
                              usize] as s8;
    (*scTask).framebuffer = cfb;
    sGraphCfbInfoIdx =
        sGraphCfbInfoIdx %
            (::std::mem::size_of::<[CfbInfo; 3]>() as
                 libc::c_ulong).wrapping_div(::std::mem::size_of::<CfbInfo>()
                                                 as libc::c_ulong) as s32;
    (*gfxCtx).schedMsgQ = &mut gSchedContext.cmdQ;
    osSendMesg(&mut gSchedContext.cmdQ, scTask as OSMesg, 1 as libc::c_int);
    Sched_SendEntryMsg(&mut gSchedContext);
}
#[no_mangle]
pub unsafe extern "C" fn Graph_Update(mut gfxCtx: *mut GraphicsContext,
                                      mut gameState: *mut GameState) {
    let mut problem: u32_0 = 0;
    (*gameState).unk_A0 = 0 as libc::c_int as u32_0;
    Graph_InitTHGA(gfxCtx);
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), gfxCtx,
                    b"../graph.c\x00" as *const u8 as *const libc::c_char,
                    966 as libc::c_int);
    let fresh1 = (*__gfxCtx).work.p;
    (*__gfxCtx).work.p = (*__gfxCtx).work.p.offset(1);
    let mut _g: *mut Gfx = fresh1;
    (*_g).words.w0 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g).words.w1 =
        b"WORK_DISP \xe9\x96\x8b\xe5\xa7\x8b\x00" as *const u8 as
            *const libc::c_char as libc::c_uint;
    let fresh2 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_0: *mut Gfx = fresh2;
    (*_g_0).words.w0 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_0).words.w1 =
        b"POLY_OPA_DISP \xe9\x96\x8b\xe5\xa7\x8b\x00" as *const u8 as
            *const libc::c_char as libc::c_uint;
    let fresh3 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_1: *mut Gfx = fresh3;
    (*_g_1).words.w0 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_1).words.w1 =
        b"POLY_XLU_DISP \xe9\x96\x8b\xe5\xa7\x8b\x00" as *const u8 as
            *const libc::c_char as libc::c_uint;
    let fresh4 = (*__gfxCtx).overlay.p;
    (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
    let mut _g_2: *mut Gfx = fresh4;
    (*_g_2).words.w0 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_2).words.w1 =
        b"OVERLAY_DISP \xe9\x96\x8b\xe5\xa7\x8b\x00" as *const u8 as
            *const libc::c_char as libc::c_uint;
    Graph_CloseDisps(dispRefs.as_mut_ptr(), gfxCtx,
                     b"../graph.c\x00" as *const u8 as *const libc::c_char,
                     975 as libc::c_int);
    GameState_ReqPadData(gameState);
    GameState_Update(gameState);
    let mut __gfxCtx_0: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs_0: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx_0 = gfxCtx;
    Graph_OpenDisps(dispRefs_0.as_mut_ptr(), gfxCtx,
                    b"../graph.c\x00" as *const u8 as *const libc::c_char,
                    987 as libc::c_int);
    let fresh5 = (*__gfxCtx_0).work.p;
    (*__gfxCtx_0).work.p = (*__gfxCtx_0).work.p.offset(1);
    let mut _g_3: *mut Gfx = fresh5;
    (*_g_3).words.w0 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_3).words.w1 =
        b"WORK_DISP \xe7\xb5\x82\xe4\xba\x86\x00" as *const u8 as
            *const libc::c_char as libc::c_uint;
    let fresh6 = (*__gfxCtx_0).polyOpa.p;
    (*__gfxCtx_0).polyOpa.p = (*__gfxCtx_0).polyOpa.p.offset(1);
    let mut _g_4: *mut Gfx = fresh6;
    (*_g_4).words.w0 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_4).words.w1 =
        b"POLY_OPA_DISP \xe7\xb5\x82\xe4\xba\x86\x00" as *const u8 as
            *const libc::c_char as libc::c_uint;
    let fresh7 = (*__gfxCtx_0).polyXlu.p;
    (*__gfxCtx_0).polyXlu.p = (*__gfxCtx_0).polyXlu.p.offset(1);
    let mut _g_5: *mut Gfx = fresh7;
    (*_g_5).words.w0 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_5).words.w1 =
        b"POLY_XLU_DISP \xe7\xb5\x82\xe4\xba\x86\x00" as *const u8 as
            *const libc::c_char as libc::c_uint;
    let fresh8 = (*__gfxCtx_0).overlay.p;
    (*__gfxCtx_0).overlay.p = (*__gfxCtx_0).overlay.p.offset(1);
    let mut _g_6: *mut Gfx = fresh8;
    (*_g_6).words.w0 =
        (0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_6).words.w1 =
        b"OVERLAY_DISP \xe7\xb5\x82\xe4\xba\x86\x00" as *const u8 as
            *const libc::c_char as libc::c_uint;
    Graph_CloseDisps(dispRefs_0.as_mut_ptr(), gfxCtx,
                     b"../graph.c\x00" as *const u8 as *const libc::c_char,
                     996 as libc::c_int);
    let mut __gfxCtx_1: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs_1: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx_1 = gfxCtx;
    Graph_OpenDisps(dispRefs_1.as_mut_ptr(), gfxCtx,
                    b"../graph.c\x00" as *const u8 as *const libc::c_char,
                    999 as libc::c_int);
    let fresh9 = (*__gfxCtx_1).work.p;
    (*__gfxCtx_1).work.p = (*__gfxCtx_1).work.p.offset(1);
    let mut _g_7: *mut Gfx = fresh9;
    (*_g_7).words.w0 =
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
    (*_g_7).words.w1 = (*gfxCtx).polyOpaBuffer as libc::c_uint;
    let fresh10 = (*__gfxCtx_1).polyOpa.p;
    (*__gfxCtx_1).polyOpa.p = (*__gfxCtx_1).polyOpa.p.offset(1);
    let mut _g_8: *mut Gfx = fresh10;
    (*_g_8).words.w0 =
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
    (*_g_8).words.w1 = (*gfxCtx).polyXluBuffer as libc::c_uint;
    let fresh11 = (*__gfxCtx_1).polyXlu.p;
    (*__gfxCtx_1).polyXlu.p = (*__gfxCtx_1).polyXlu.p.offset(1);
    let mut _g_9: *mut Gfx = fresh11;
    (*_g_9).words.w0 =
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
    (*_g_9).words.w1 = (*gfxCtx).overlayBuffer as libc::c_uint;
    let fresh12 = (*__gfxCtx_1).overlay.p;
    (*__gfxCtx_1).overlay.p = (*__gfxCtx_1).overlay.p.offset(1);
    let mut _g_10: *mut Gfx = fresh12;
    (*_g_10).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_10).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh13 = (*__gfxCtx_1).overlay.p;
    (*__gfxCtx_1).overlay.p = (*__gfxCtx_1).overlay.p.offset(1);
    let mut _g_11: *mut Gfx = fresh13;
    (*_g_11).words.w0 =
        (0xe9 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_11).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh14 = (*__gfxCtx_1).overlay.p;
    (*__gfxCtx_1).overlay.p = (*__gfxCtx_1).overlay.p.offset(1);
    let mut _g_12: *mut Gfx = fresh14;
    (*_g_12).words.w0 =
        (0xdf as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_12).words.w1 = 0 as libc::c_int as libc::c_uint;
    Graph_CloseDisps(dispRefs_1.as_mut_ptr(), gfxCtx,
                     b"../graph.c\x00" as *const u8 as *const libc::c_char,
                     1028 as libc::c_int);
    if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 80 as libc::c_int) as usize]
           as libc::c_int == 10 as libc::c_int &&
           (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 93 as libc::c_int) as
                                 usize] as libc::c_int == 2 as libc::c_int {
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 80 as libc::c_int) as
                              usize] = 7 as libc::c_int as s16;
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 81 as libc::c_int) as
                              usize] = -(1 as libc::c_int) as s16;
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 83 as libc::c_int) as
                              usize] =
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 92 as libc::c_int) as
                                  usize]
    }
    if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 80 as libc::c_int) as usize]
           as libc::c_int == 7 as libc::c_int &&
           (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 81 as libc::c_int) as
                                 usize] as libc::c_int != 0 as libc::c_int {
        if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 82 as libc::c_int) as
                                 usize] as libc::c_int == 3 as libc::c_int {
            Fault_AddClient(&mut sGraphUcodeFaultClient,
                            ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                    *mut Gfx)
                                                               -> ()>,
                                                    *mut libc::c_void>(Some(Graph_UCodeFaultClient
                                                                                as
                                                                                unsafe extern "C" fn(_:
                                                                                                         *mut Gfx)
                                                                                    ->
                                                                                        ())),
                            (*gfxCtx).workBuffer as *mut libc::c_void,
                            b"do_count_fault\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_void);
        }
        Graph_DisassembleUCode((*gfxCtx).workBuffer);
        if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 82 as libc::c_int) as
                                 usize] as libc::c_int == 3 as libc::c_int {
            Fault_RemoveClient(&mut sGraphUcodeFaultClient);
        }
        if ((*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 81 as libc::c_int) as
                                  usize] as libc::c_int) < 0 as libc::c_int {
            LogUtils_LogHexDump((0x4040000 as libc::c_int as libc::c_uint |
                                     0xa0000000 as libc::c_uint) as *mut u32_0
                                    as *mut libc::c_void,
                                0x20 as libc::c_int);
            LogUtils_LogHexDump((0xa4100000 as
                                     libc::c_uint).wrapping_add(0 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint)
                                    as *mut vu32 as *mut libc::c_void,
                                0x20 as libc::c_int);
        }
        if ((*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 81 as libc::c_int) as
                                  usize] as libc::c_int) < 0 as libc::c_int {
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 81 as libc::c_int) as
                                  usize] = 0 as libc::c_int as s16
        }
    }
    problem = 0 as libc::c_int as u32_0;
    let mut pool: *mut GfxPool =
        &mut *gGfxPools.as_mut_ptr().offset(((*gfxCtx).gfxPoolIdx &
                                                 1 as libc::c_int as
                                                     libc::c_uint) as isize)
            as *mut GfxPool;
    if (*pool).headMagic as libc::c_int != 0x1234 as libc::c_int {
        // ! @bug (?) : "problem = true;" may be missing
        osSyncPrintf(b"%c\x00" as *const u8 as *const libc::c_char,
                     7 as libc::c_int);
        // "Dynamic area head is destroyed"
        osSyncPrintf(b"\x1b[41;37m\xe3\x83\x80\xe3\x82\xa4\xe3\x83\x8a\xe3\x83\x9f\xe3\x83\x83\xe3\x82\xaf\xe9\xa0\x98\xe5\x9f\x9f\xe5\x85\x88\xe9\xa0\xad\xe3\x81\x8c\xe7\xa0\xb4\xe5\xa3\x8a\xe3\x81\x95\xe3\x82\x8c\xe3\x81\xa6\xe3\x81\x84\xe3\x81\xbe\xe3\x81\x99\n\x1b[m\x00"
                         as *const u8 as *const libc::c_char);
        Fault_AddHungupAndCrash(b"../graph.c\x00" as *const u8 as
                                    *const libc::c_char,
                                1070 as libc::c_int as u32_0);
    }
    if (*pool).tailMagic as libc::c_int != 0x5678 as libc::c_int {
        problem = 1 as libc::c_int as u32_0;
        osSyncPrintf(b"%c\x00" as *const u8 as *const libc::c_char,
                     7 as libc::c_int);
        // "Dynamic region tail is destroyed"
        osSyncPrintf(b"\x1b[41;37m\xe3\x83\x80\xe3\x82\xa4\xe3\x83\x8a\xe3\x83\x9f\xe3\x83\x83\xe3\x82\xaf\xe9\xa0\x98\xe5\x9f\x9f\xe6\x9c\xab\xe5\xb0\xbe\xe3\x81\x8c\xe7\xa0\xb4\xe5\xa3\x8a\xe3\x81\x95\xe3\x82\x8c\xe3\x81\xa6\xe3\x81\x84\xe3\x81\xbe\xe3\x81\x99\n\x1b[m\x00"
                         as *const u8 as *const libc::c_char);
        Fault_AddHungupAndCrash(b"../graph.c\x00" as *const u8 as
                                    *const libc::c_char,
                                1076 as libc::c_int as u32_0);
    }
    if THGA_IsCrash(&mut (*gfxCtx).polyOpa) != 0 {
        problem = 1 as libc::c_int as u32_0;
        osSyncPrintf(b"%c\x00" as *const u8 as *const libc::c_char,
                     7 as libc::c_int);
        // "Zelda 0 is dead"
        osSyncPrintf(b"\x1b[41;37m\xe3\x82\xbc\xe3\x83\xab\xe3\x83\x800\xe3\x81\xaf\xe6\xad\xbb\xe3\x82\x93\xe3\x81\xa7\xe3\x81\x97\xe3\x81\xbe\xe3\x81\xa3\xe3\x81\x9f(graph_alloc is empty)\n\x1b[m\x00"
                         as *const u8 as *const libc::c_char);
    }
    if THGA_IsCrash(&mut (*gfxCtx).polyXlu) != 0 {
        problem = 1 as libc::c_int as u32_0;
        osSyncPrintf(b"%c\x00" as *const u8 as *const libc::c_char,
                     7 as libc::c_int);
        // "Zelda 1 is dead"
        osSyncPrintf(b"\x1b[41;37m\xe3\x82\xbc\xe3\x83\xab\xe3\x83\x801\xe3\x81\xaf\xe6\xad\xbb\xe3\x82\x93\xe3\x81\xa7\xe3\x81\x97\xe3\x81\xbe\xe3\x81\xa3\xe3\x81\x9f(graph_alloc is empty)\n\x1b[m\x00"
                         as *const u8 as *const libc::c_char);
    }
    if THGA_IsCrash(&mut (*gfxCtx).overlay) != 0 {
        problem = 1 as libc::c_int as u32_0;
        osSyncPrintf(b"%c\x00" as *const u8 as *const libc::c_char,
                     7 as libc::c_int);
        // "Zelda 4 is dead"
        osSyncPrintf(b"\x1b[41;37m\xe3\x82\xbc\xe3\x83\xab\xe3\x83\x804\xe3\x81\xaf\xe6\xad\xbb\xe3\x82\x93\xe3\x81\xa7\xe3\x81\x97\xe3\x81\xbe\xe3\x81\xa3\xe3\x81\x9f(graph_alloc is empty)\n\x1b[m\x00"
                         as *const u8 as *const libc::c_char);
    }
    if problem == 0 {
        Graph_TaskSet00(gfxCtx);
        (*gfxCtx).gfxPoolIdx = (*gfxCtx).gfxPoolIdx.wrapping_add(1);
        (*gfxCtx).fbIdx += 1
    }
    func_800F3054();
    let mut time: OSTime = osGetTime();
    let mut pad: [s32; 4] = [0; 4];
    ::std::ptr::write_volatile(&mut D_8016A538 as *mut OSTime,
                               gRSPGFXTotalTime);
    ::std::ptr::write_volatile(&mut D_8016A530 as *mut OSTime,
                               gRSPAudioTotalTime);
    ::std::ptr::write_volatile(&mut D_8016A540 as *mut OSTime, gRDPTotalTime);
    ::std::ptr::write_volatile(&mut gRSPGFXTotalTime as *mut OSTime,
                               0 as libc::c_int as OSTime);
    ::std::ptr::write_volatile(&mut gRSPAudioTotalTime as *mut OSTime,
                               0 as libc::c_int as OSTime);
    ::std::ptr::write_volatile(&mut gRDPTotalTime as *mut OSTime,
                               0 as libc::c_int as OSTime);
    if sGraphUpdateTime != 0 as libc::c_int as libc::c_ulonglong {
        ::std::ptr::write_volatile(&mut D_8016A548 as *mut OSTime,
                                   time.wrapping_sub(sGraphUpdateTime))
    }
    sGraphUpdateTime = time;
    if gIsCtrlr2Valid != 0 &&
           !((*gameState).input[0 as libc::c_int as usize].press.button as
                 libc::c_int | !(0x2000 as libc::c_int)) == 0 as libc::c_int
           &&
           !((*gameState).input[0 as libc::c_int as usize].cur.button as
                 libc::c_int | !(0x20 as libc::c_int | 0x10 as libc::c_int))
               == 0 as libc::c_int {
        gSaveContext.gameMode = 0 as libc::c_int;
        (*gameState).init =
            Some(Select_Init as
                     unsafe extern "C" fn(_: *mut GameState) -> ());
        (*gameState).size =
            ::std::mem::size_of::<SelectContext>() as libc::c_ulong;
        (*gameState).running = 0 as libc::c_int as u32_0
    }
    if gIsCtrlr2Valid != 0 && PreNmiBuff_IsResetting(gAppNmiBufferPtr) != 0 &&
           (*gameState).unk_A0 == 0 {
        // "To reset mode"
        osSyncPrintf(b"\x1b[43;30mPRE-NMI\xe3\x81\xab\xe3\x82\x88\xe3\x82\x8a\xe3\x83\xaa\xe3\x82\xbb\xe3\x83\x83\xe3\x83\x88\xe3\x83\xa2\xe3\x83\xbc\xe3\x83\x89\xe3\x81\xab\xe7\xa7\xbb\xe8\xa1\x8c\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x99\n\x1b[m\x00"
                         as *const u8 as
                         *const libc::c_char); // "Start graphic thread execution"
        (*gameState).init =
            Some(PreNMI_Init as
                     unsafe extern "C" fn(_: *mut GameState)
                         -> ()); // "Class size = %d bytes"
        (*gameState).size =
            ::std::mem::size_of::<PreNMIContext>() as
                libc::c_ulong; // "Failure to secure"
        (*gameState).running = 0 as libc::c_int as u32_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn Graph_ThreadEntry(mut arg0: *mut libc::c_void) {
    let mut gfxCtx: GraphicsContext =
        GraphicsContext{polyOpaBuffer: 0 as *mut Gfx,
                        polyXluBuffer: 0 as *mut Gfx,
                        unk_008: [0; 8],
                        overlayBuffer: 0 as *mut Gfx,
                        unk_014: 0,
                        unk_018: [0; 32],
                        msgBuff: [0 as *mut libc::c_void; 8],
                        schedMsgQ: 0 as *mut OSMesgQueue,
                        queue:
                            OSMesgQueue{mtqueue:
                                            0 as *const OSThread as
                                                *mut OSThread,
                                        fullqueue:
                                            0 as *const OSThread as
                                                *mut OSThread,
                                        validCount: 0,
                                        first: 0,
                                        msgCount: 0,
                                        msg:
                                            0 as *const OSMesg as
                                                *mut OSMesg,},
                        unk_074: [0; 4],
                        task:
                            OSScTask{next: 0 as *mut OSScTask,
                                     state: 0,
                                     flags: 0,
                                     framebuffer: 0 as *mut CfbInfo,
                                     list:
                                         OSTask{t:
                                                    OSTask_t{type_0: 0,
                                                             flags: 0,
                                                             ucode_boot:
                                                                 0 as
                                                                     *mut u64_0,
                                                             ucode_boot_size:
                                                                 0,
                                                             ucode:
                                                                 0 as
                                                                     *mut u64_0,
                                                             ucode_size: 0,
                                                             ucode_data:
                                                                 0 as
                                                                     *mut u64_0,
                                                             ucode_data_size:
                                                                 0,
                                                             dram_stack:
                                                                 0 as
                                                                     *mut u64_0,
                                                             dram_stack_size:
                                                                 0,
                                                             output_buff:
                                                                 0 as
                                                                     *mut u64_0,
                                                             output_buff_size:
                                                                 0 as
                                                                     *mut u64_0,
                                                             data_ptr:
                                                                 0 as
                                                                     *mut u64_0,
                                                             data_size: 0,
                                                             yield_data_ptr:
                                                                 0 as
                                                                     *mut u64_0,
                                                             yield_data_size:
                                                                 0,},},
                                     msgQ: 0 as *mut OSMesgQueue,
                                     msg: 0 as *mut libc::c_void,},
                        unk_0D0: [0; 224],
                        workBuffer: 0 as *mut Gfx,
                        work:
                            TwoHeadGfxArena{size: 0,
                                            bufp: 0 as *mut Gfx,
                                            p: 0 as *mut Gfx,
                                            d: 0 as *mut Gfx,},
                        unk_01C4: [0; 192],
                        viMode: 0 as *mut OSViMode,
                        unk_0288: [0; 32],
                        overlay:
                            TwoHeadGfxArena{size: 0,
                                            bufp: 0 as *mut Gfx,
                                            p: 0 as *mut Gfx,
                                            d: 0 as *mut Gfx,},
                        polyOpa:
                            TwoHeadGfxArena{size: 0,
                                            bufp: 0 as *mut Gfx,
                                            p: 0 as *mut Gfx,
                                            d: 0 as *mut Gfx,},
                        polyXlu:
                            TwoHeadGfxArena{size: 0,
                                            bufp: 0 as *mut Gfx,
                                            p: 0 as *mut Gfx,
                                            d: 0 as *mut Gfx,},
                        gfxPoolIdx: 0,
                        curFrameBuffer: 0 as *mut u16_0,
                        unk_2E0: [0; 4],
                        viFeatures: 0,
                        fbIdx: 0,
                        callback: None,
                        callbackParam: 0 as *mut libc::c_void,
                        xScale: 0.,
                        yScale: 0.,
                        unk_2FC: [0; 4],};
    let mut gameState: *mut GameState = 0 as *mut GameState;
    let mut size: u32_0 = 0;
    let mut nextOvl: *mut GameStateOverlay = 0 as *mut GameStateOverlay;
    let mut ovl: *mut GameStateOverlay = 0 as *mut GameStateOverlay;
    let mut faultMsg: [libc::c_char; 80] = [0; 80];
    nextOvl =
        &mut *gGameStateOverlayTable.as_mut_ptr().offset(0 as libc::c_int as
                                                             isize) as
            *mut GameStateOverlay;
    osSyncPrintf(b"\xe3\x82\xb0\xe3\x83\xa9\xe3\x83\x95\xe3\x82\xa3\xe3\x83\x83\xe3\x82\xaf\xe3\x82\xb9\xe3\x83\xac\xe3\x83\x83\xe3\x83\x89\xe5\xae\x9f\xe8\xa1\x8c\xe9\x96\x8b\xe5\xa7\x8b\n\x00"
                     as *const u8 as *const libc::c_char);
    Graph_Init(&mut gfxCtx);
    while !nextOvl.is_null() {
        ovl = nextOvl;
        Overlay_LoadGameState(ovl);
        size = (*ovl).instanceSize;
        osSyncPrintf(b"\xe3\x82\xaf\xe3\x83\xa9\xe3\x82\xb9\xe3\x82\xb5\xe3\x82\xa4\xe3\x82\xba\xef\xbc\x9d%d\xe3\x83\x90\xe3\x82\xa4\xe3\x83\x88\n\x00"
                         as *const u8 as *const libc::c_char, size);
        gameState =
            SystemArena_MallocDebug(size,
                                    b"../graph.c\x00" as *const u8 as
                                        *const libc::c_char,
                                    1196 as libc::c_int) as *mut GameState;
        if gameState.is_null() {
            osSyncPrintf(b"\xe7\xa2\xba\xe4\xbf\x9d\xe5\xa4\xb1\xe6\x95\x97\n\x00"
                             as *const u8 as *const libc::c_char);
            sprintf(faultMsg.as_mut_ptr(),
                    b"CLASS SIZE= %d bytes\x00" as *const u8 as
                        *const libc::c_char, size);
            Fault_AddHungupAndCrashImpl(b"GAME CLASS MALLOC FAILED\x00" as
                                            *const u8 as *const libc::c_char,
                                        faultMsg.as_mut_ptr());
        }
        GameState_Init(gameState,
                       ::std::mem::transmute::<*mut libc::c_void,
                                               GameStateFunc>((*ovl).init),
                       &mut gfxCtx);
        while GameState_IsRunning(gameState) != 0 {
            Graph_Update(&mut gfxCtx, gameState);
        }
        nextOvl = Graph_GetNextGameState(gameState);
        GameState_Destroy(gameState);
        SystemArena_FreeDebug(gameState as *mut libc::c_void,
                              b"../graph.c\x00" as *const u8 as
                                  *const libc::c_char, 1227 as libc::c_int);
        Overlay_FreeGameState(ovl);
    }
    Graph_Destroy(&mut gfxCtx);
    osSyncPrintf(b"\xe3\x82\xb0\xe3\x83\xa9\xe3\x83\x95\xe3\x82\xa3\xe3\x83\x83\xe3\x82\xaf\xe3\x82\xb9\xe3\x83\xac\xe3\x83\x83\xe3\x83\x89\xe5\xae\x9f\xe8\xa1\x8c\xe7\xb5\x82\xe4\xba\x86\n\x00"
                     as *const u8 as *const libc::c_char);
    // "End of graphic thread execution"
}
#[no_mangle]
pub unsafe extern "C" fn Graph_Alloc(mut gfxCtx: *mut GraphicsContext,
                                     mut size: size_t) -> *mut libc::c_void {
    let mut thga: *mut TwoHeadGfxArena = &mut (*gfxCtx).polyOpa;
    if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 59 as libc::c_int) as usize]
           as libc::c_int == 1 as libc::c_int {
        osSyncPrintf(b"graph_alloc siz=%d thga size=%08x bufp=%08x head=%08x tail=%08x\n\x00"
                         as *const u8 as *const libc::c_char, size,
                     (*thga).size, (*thga).bufp, (*thga).p, (*thga).d);
    }
    return THGA_AllocEnd(&mut (*gfxCtx).polyOpa,
                         (size.wrapping_add(0xf as libc::c_int as
                                                libc::c_ulong) &
                              !(0xf as libc::c_int) as libc::c_ulong) as
                             u32_0) as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn Graph_Alloc2(mut gfxCtx: *mut GraphicsContext,
                                      mut size: size_t) -> *mut libc::c_void {
    let mut thga: *mut TwoHeadGfxArena = &mut (*gfxCtx).polyOpa;
    if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 59 as libc::c_int) as usize]
           as libc::c_int == 1 as libc::c_int {
        osSyncPrintf(b"graph_alloc siz=%d thga size=%08x bufp=%08x head=%08x tail=%08x\n\x00"
                         as *const u8 as *const libc::c_char, size,
                     (*thga).size, (*thga).bufp, (*thga).p, (*thga).d);
    }
    return THGA_AllocEnd(&mut (*gfxCtx).polyOpa,
                         (size.wrapping_add(0xf as libc::c_int as
                                                libc::c_ulong) &
                              !(0xf as libc::c_int) as libc::c_ulong) as
                             u32_0) as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn Graph_OpenDisps(mut dispRefs: *mut *mut Gfx,
                                         mut gfxCtx: *mut GraphicsContext,
                                         mut file: *const libc::c_char,
                                         mut line: s32) {
    if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 80 as libc::c_int) as usize]
           as libc::c_int == 7 as libc::c_int &&
           (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 82 as libc::c_int) as
                                 usize] as libc::c_int != 4 as libc::c_int {
        let ref mut fresh15 = *dispRefs.offset(0 as libc::c_int as isize);
        *fresh15 = (*gfxCtx).polyOpa.p;
        let ref mut fresh16 = *dispRefs.offset(1 as libc::c_int as isize);
        *fresh16 = (*gfxCtx).polyXlu.p;
        let ref mut fresh17 = *dispRefs.offset(2 as libc::c_int as isize);
        *fresh17 = (*gfxCtx).overlay.p;
        let fresh18 = (*gfxCtx).polyOpa.p;
        (*gfxCtx).polyOpa.p = (*gfxCtx).polyOpa.p.offset(1);
        let mut _g: *mut Gfx = fresh18;
        (*_g).words.w0 =
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (7 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                (line as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g).words.w1 = file as libc::c_uint;
        let fresh19 = (*gfxCtx).polyXlu.p;
        (*gfxCtx).polyXlu.p = (*gfxCtx).polyXlu.p.offset(1);
        let mut _g_0: *mut Gfx = fresh19;
        (*_g_0).words.w0 =
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (7 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                (line as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_0).words.w1 = file as libc::c_uint;
        let fresh20 = (*gfxCtx).overlay.p;
        (*gfxCtx).overlay.p = (*gfxCtx).overlay.p.offset(1);
        let mut _g_1: *mut Gfx = fresh20;
        (*_g_1).words.w0 =
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (7 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                (line as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_g_1).words.w1 = file as libc::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C" fn Graph_CloseDisps(mut dispRefs: *mut *mut Gfx,
                                          mut gfxCtx: *mut GraphicsContext,
                                          mut file: *const libc::c_char,
                                          mut line: s32) {
    if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 80 as libc::c_int) as usize]
           as libc::c_int == 7 as libc::c_int &&
           (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 82 as libc::c_int) as
                                 usize] as libc::c_int != 4 as libc::c_int {
        if (*dispRefs.offset(0 as libc::c_int as
                                 isize)).offset(1 as libc::c_int as isize) ==
               (*gfxCtx).polyOpa.p {
            (*gfxCtx).polyOpa.p = *dispRefs.offset(0 as libc::c_int as isize)
        } else {
            let fresh21 = (*gfxCtx).polyOpa.p;
            (*gfxCtx).polyOpa.p = (*gfxCtx).polyOpa.p.offset(1);
            let mut _g: *mut Gfx = fresh21;
            (*_g).words.w0 =
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (8 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (line as u32_0 &
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g).words.w1 = file as libc::c_uint
        }
        if (*dispRefs.offset(1 as libc::c_int as
                                 isize)).offset(1 as libc::c_int as isize) ==
               (*gfxCtx).polyXlu.p {
            (*gfxCtx).polyXlu.p = *dispRefs.offset(1 as libc::c_int as isize)
        } else {
            let fresh22 = (*gfxCtx).polyXlu.p;
            (*gfxCtx).polyXlu.p = (*gfxCtx).polyXlu.p.offset(1);
            let mut _g_0: *mut Gfx = fresh22;
            (*_g_0).words.w0 =
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (8 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (line as u32_0 &
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_0).words.w1 = file as libc::c_uint
        }
        if (*dispRefs.offset(2 as libc::c_int as
                                 isize)).offset(1 as libc::c_int as isize) ==
               (*gfxCtx).overlay.p {
            (*gfxCtx).overlay.p = *dispRefs.offset(2 as libc::c_int as isize)
        } else {
            let fresh23 = (*gfxCtx).overlay.p;
            (*gfxCtx).overlay.p = (*gfxCtx).overlay.p.offset(1);
            let mut _g_1: *mut Gfx = fresh23;
            (*_g_1).words.w0 =
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (8 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (line as u32_0 &
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_g_1).words.w1 = file as libc::c_uint
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Graph_GfxPlusOne(mut gfx: *mut Gfx) -> *mut Gfx {
    return gfx.offset(1 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn Graph_BranchDlist(mut gfx: *mut Gfx,
                                           mut dst: *mut Gfx) -> *mut Gfx {
    let mut _g: *mut Gfx = gfx;
    (*_g).words.w0 =
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
    (*_g).words.w1 = dst as libc::c_uint;
    return dst;
}
#[no_mangle]
pub unsafe extern "C" fn Graph_DlistAlloc(mut gfx: *mut *mut Gfx,
                                          mut size: u32_0)
 -> *mut libc::c_void {
    let mut ptr: *mut u8_0 = 0 as *mut u8_0;
    let mut dst: *mut Gfx = 0 as *mut Gfx;
    size =
        size.wrapping_add(7 as libc::c_int as libc::c_uint) &
            !(7 as libc::c_int) as libc::c_uint;
    ptr = (*gfx).offset(1 as libc::c_int as isize) as *mut u8_0;
    dst = ptr.offset(size as isize) as *mut Gfx;
    let mut _g: *mut Gfx = *gfx;
    (*_g).words.w0 =
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
    (*_g).words.w1 = dst as libc::c_uint;
    *gfx = dst;
    return ptr as *mut libc::c_void;
}
