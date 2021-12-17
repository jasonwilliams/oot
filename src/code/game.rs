#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, register_tool)]
extern "C" {
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn LogUtils_LogHexDump(ptr: *mut libc::c_void, size0: s32);
    #[no_mangle]
    fn LogUtils_CheckNullPointer(exp: *const libc::c_char,
                                 ptr: *mut libc::c_void,
                                 file: *const libc::c_char, line: s32);
    #[no_mangle]
    fn osSendMesg(mq: *mut OSMesgQueue, mesg: OSMesg, flag: s32) -> s32;
    #[no_mangle]
    fn osRecvMesg(mq: *mut OSMesgQueue, msg: *mut OSMesg, flag: s32) -> s32;
    #[no_mangle]
    fn osGetTime() -> OSTime;
    #[no_mangle]
    fn func_8006390C(input: *mut Input);
    #[no_mangle]
    fn func_80063D7C(gfxCtx: *mut GraphicsContext);
    #[no_mangle]
    fn func_800AA0B4();
    #[no_mangle]
    fn func_800AA0F0();
    #[no_mangle]
    fn ViMode_Init(viMode: *mut ViMode);
    #[no_mangle]
    fn ViMode_Destroy(viMode: *mut ViMode);
    #[no_mangle]
    fn ViMode_Update(viMode: *mut ViMode, input: *mut Input);
    #[no_mangle]
    fn func_800ACE70(this: *mut struct_801664F0);
    #[no_mangle]
    fn func_800ACE90(this: *mut struct_801664F0);
    #[no_mangle]
    fn func_800ACE98(this: *mut struct_801664F0, gfxp: *mut *mut Gfx);
    #[no_mangle]
    fn VisMono_Init(this: *mut VisMono);
    #[no_mangle]
    fn VisMono_Destroy(this: *mut VisMono);
    #[no_mangle]
    fn VisMono_Draw(this: *mut VisMono, gfxp: *mut *mut Gfx);
    #[no_mangle]
    fn func_800AD920(this: *mut struct_80166500);
    #[no_mangle]
    fn func_800AD950(this: *mut struct_80166500);
    #[no_mangle]
    fn func_800AD958(this: *mut struct_80166500, gfxp: *mut *mut Gfx);
    #[no_mangle]
    fn THA_AllocEndAlign16(tha: *mut TwoHeadArena, size: u32_0)
     -> *mut libc::c_void;
    #[no_mangle]
    fn THA_GetSize(tha: *mut TwoHeadArena) -> s32;
    #[no_mangle]
    fn THA_IsCrash(tha: *mut TwoHeadArena) -> u32_0;
    #[no_mangle]
    fn THA_Ct(tha: *mut TwoHeadArena, ptr: *mut libc::c_void, size: u32_0);
    #[no_mangle]
    fn THA_Dt(tha: *mut TwoHeadArena);
    #[no_mangle]
    fn func_800C3C20();
    #[no_mangle]
    fn GameAlloc_MallocDebug(this: *mut GameAlloc, size: u32_0,
                             file: *const libc::c_char, line: s32)
     -> *mut libc::c_void;
    #[no_mangle]
    fn GameAlloc_Free(this: *mut GameAlloc, data: *mut libc::c_void);
    #[no_mangle]
    fn GameAlloc_Cleanup(this: *mut GameAlloc);
    #[no_mangle]
    fn GameAlloc_Init(this: *mut GameAlloc);
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
    fn SpeedMeter_Init(this: *mut SpeedMeter);
    #[no_mangle]
    fn SpeedMeter_Destroy(this: *mut SpeedMeter);
    #[no_mangle]
    fn SpeedMeter_DrawTimeEntries(this: *mut SpeedMeter,
                                  gfxCtx: *mut GraphicsContext);
    #[no_mangle]
    fn SpeedMeter_DrawAllocEntries(meter: *mut SpeedMeter,
                                   gfxCtx: *mut GraphicsContext,
                                   state: *mut GameState);
    #[no_mangle]
    fn DebugArena_Display();
    #[no_mangle]
    fn Fault_AddClient(_: *mut FaultClient, _: *mut libc::c_void,
                       _: *mut libc::c_void, _: *mut libc::c_void);
    #[no_mangle]
    fn Fault_RemoveClient(_: *mut FaultClient);
    #[no_mangle]
    fn Fault_AddHungupAndCrash(_: *const libc::c_char, _: u32_0);
    #[no_mangle]
    fn FaultDrawer_DrawText(_: s32, _: s32, _: *const libc::c_char, _: ...);
    #[no_mangle]
    fn AudioDebug_Draw(printer: *mut GfxPrint);
    #[no_mangle]
    fn func_800F3054();
    #[no_mangle]
    fn GfxPrint_Init(this: *mut GfxPrint);
    #[no_mangle]
    fn GfxPrint_Destroy(this: *mut GfxPrint);
    #[no_mangle]
    fn GfxPrint_Open(this: *mut GfxPrint, dList: *mut Gfx);
    #[no_mangle]
    fn GfxPrint_Close(this: *mut GfxPrint) -> *mut Gfx;
    #[no_mangle]
    static mut gGameInfo: *mut GameInfo;
    #[no_mangle]
    fn SystemArena_Display();
    #[no_mangle]
    static mut gZBuffer: [[u16_0; 320]; 240];
    #[no_mangle]
    static mut gPadMgr: PadMgr;
    #[no_mangle]
    static mut D_80009430: s8;
    #[no_mangle]
    static mut gViConfigAdditionalScanLines: u8_0;
    #[no_mangle]
    static mut gViConfigYScale: f32_0;
    #[no_mangle]
    static mut osViModeFpalLan1: OSViMode;
    #[no_mangle]
    static mut osTvType: u32_0;
    #[no_mangle]
    static mut osViModePalLan1: OSViMode;
    #[no_mangle]
    static mut osViModeMpalLan1: OSViMode;
    #[no_mangle]
    static mut osViModeNtscLan1: OSViMode;
    #[no_mangle]
    static mut gViConfigXScale: f32_0;
    #[no_mangle]
    static mut gViConfigFeatures: u32_0;
    #[no_mangle]
    static mut gViConfigMode: OSViMode;
    #[no_mangle]
    static mut gZeldaArenaLogSeverity: s32;
    #[no_mangle]
    static mut gSystemArenaLogSeverity: s32;
    #[no_mangle]
    static mut gDmaMgrDmaBuffSize: u32_0;
    #[no_mangle]
    static mut D_80009460: u32_0;
    #[no_mangle]
    static mut gIsCtrlr2Valid: u32_0;
    #[no_mangle]
    static mut __osMalloc_FreeBlockTest_Enable: u32_0;
    #[no_mangle]
    fn SystemArena_GetSizes(outMaxFree: *mut u32_0, outFree: *mut u32_0,
                            outAlloc: *mut u32_0);
    #[no_mangle]
    fn PadMgr_RequestPadData(_: *mut PadMgr, _: *mut Input, _: s32);
}
pub type s8 = libc::c_schar;
pub type u8_0 = libc::c_uchar;
pub type s16 = libc::c_short;
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
pub type u64_0 = libc::c_ulonglong;
pub type vu8 = u8_0;
pub type f32_0 = libc::c_float;
pub type size_t = libc::c_ulong;
pub type OSPri = s32;
pub type OSId = s32;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __OSfp {
    pub f: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
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
pub struct OSContStatus {
    pub type_0: u16_0,
    pub status: u8_0,
    pub errno: u8_0,
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
pub struct Tri {
    pub flag: libc::c_uchar,
    pub v: [libc::c_uchar; 3],
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
pub struct OSPfs {
    pub status: s32,
    pub queue: *mut OSMesgQueue,
    pub channel: s32,
    pub id: [u8_0; 32],
    pub label: [u8_0; 32],
    pub version: s32,
    pub dir_size: s32,
    pub inode_table: s32,
    pub minode_table: s32,
    pub dir_table: s32,
    pub inodeStartPage: s32,
    pub banks: u8_0,
    pub activebank: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Color_RGBA8_u32 {
    pub c2rust_unnamed: C2RustUnnamed_0,
    pub rgba: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub r: u8_0,
    pub g: u8_0,
    pub b: u8_0,
    pub a: u8_0,
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
pub struct FaultClient {
    pub next: *mut FaultClient,
    pub callback: u32_0,
    pub param1: u32_0,
    pub param2: u32_0,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSScMsg {
    pub type_0: s16,
    pub misc: [libc::c_char; 30],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IrqMgrClient {
    pub prev: *mut IrqMgrClient,
    pub queue: *mut OSMesgQueue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IrqMgr {
    pub retraceMsg: OSScMsg,
    pub prenmiMsg: OSScMsg,
    pub nmiMsg: OSScMsg,
    pub queue: OSMesgQueue,
    pub msgBuf: [OSMesg; 8],
    pub thread: OSThread,
    pub clients: *mut IrqMgrClient,
    pub resetStatus: u8_0,
    pub resetTime: OSTime,
    pub timer: OSTimer,
    pub retraceTime: OSTime,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PadMgr {
    pub padStatus: [OSContStatus; 4],
    pub serialMsgBuf: [OSMesg; 1],
    pub lockMsgBuf: [OSMesg; 1],
    pub interruptMsgBuf: [OSMesg; 4],
    pub serialMsgQ: OSMesgQueue,
    pub lockMsgQ: OSMesgQueue,
    pub interruptMsgQ: OSMesgQueue,
    pub irqClient: IrqMgrClient,
    pub irqMgr: *mut IrqMgr,
    pub thread: OSThread,
    pub inputs: [Input; 4],
    pub pads: [OSContPad; 4],
    pub validCtrlrsMask: vu8,
    pub nControllers: u8_0,
    pub ctrlrIsConnected: [u8_0; 4],
    pub pakType: [u8_0; 4],
    pub rumbleEnable: [vu8; 4],
    pub rumbleCounter: [u8_0; 4],
    pub pfs: [OSPfs; 4],
    pub rumbleOffFrames: vu8,
    pub rumbleOnFrames: vu8,
    pub preNMIShutdown: u8_0,
    pub retraceCallback: Option<unsafe extern "C" fn(_: *mut PadMgr, _: s32)
                                    -> ()>,
    pub retraceCallbackValue: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ViMode {
    pub customViMode: OSViMode,
    pub viHeight: s32,
    pub viWidth: s32,
    pub unk_58: s32,
    pub unk_5C: s32,
    pub unk_60: s32,
    pub unk_64: s32,
    pub viModeBase: s32,
    pub viTvType: s32,
    pub unk_70: u32_0,
    pub unk_74: u32_0,
    pub unk_78: u32_0,
    pub unk_7C: u32_0,
    pub viFeatures: u32_0,
    pub unk_84: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct struct_801664F0 {
    pub type_0: u32_0,
    pub setScissor: u32_0,
    pub color: Color_RGBA8_u32,
    pub envColor: Color_RGBA8_u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VisMono {
    pub unk_00: u32_0,
    pub setScissor: u32_0,
    pub primColor: Color_RGBA8_u32,
    pub envColor: Color_RGBA8_u32,
    pub tlut: *mut u16_0,
    pub monoDl: *mut Gfx,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct struct_80166500 {
    pub useRgba: u32_0,
    pub setScissor: u32_0,
    pub primColor: Color_RGBA8_u32,
    pub envColor: Color_RGBA8_u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SpeedMeter {
    pub unk_00: [libc::c_char; 24],
    pub unk_18: s32,
    pub y: s32,
}
#[no_mangle]
pub static mut D_801664D0: SpeedMeter =
    SpeedMeter{unk_00: [0; 24], unk_18: 0, y: 0,};
#[no_mangle]
pub static mut D_801664F0: struct_801664F0 =
    struct_801664F0{type_0: 0,
                    setScissor: 0,
                    color:
                        Color_RGBA8_u32{c2rust_unnamed:
                                            C2RustUnnamed_0{r: 0,
                                                            g: 0,
                                                            b: 0,
                                                            a: 0,},},
                    envColor:
                        Color_RGBA8_u32{c2rust_unnamed:
                                            C2RustUnnamed_0{r: 0,
                                                            g: 0,
                                                            b: 0,
                                                            a: 0,},},};
#[no_mangle]
pub static mut D_80166500: struct_80166500 =
    struct_80166500{useRgba: 0,
                    setScissor: 0,
                    primColor:
                        Color_RGBA8_u32{c2rust_unnamed:
                                            C2RustUnnamed_0{r: 0,
                                                            g: 0,
                                                            b: 0,
                                                            a: 0,},},
                    envColor:
                        Color_RGBA8_u32{c2rust_unnamed:
                                            C2RustUnnamed_0{r: 0,
                                                            g: 0,
                                                            b: 0,
                                                            a: 0,},},};
#[no_mangle]
pub static mut sMonoColors: VisMono =
    VisMono{unk_00: 0,
            setScissor: 0,
            primColor:
                Color_RGBA8_u32{c2rust_unnamed:
                                    C2RustUnnamed_0{r: 0,
                                                    g: 0,
                                                    b: 0,
                                                    a: 0,},},
            envColor:
                Color_RGBA8_u32{c2rust_unnamed:
                                    C2RustUnnamed_0{r: 0,
                                                    g: 0,
                                                    b: 0,
                                                    a: 0,},},
            tlut: 0 as *const u16_0 as *mut u16_0,
            monoDl: 0 as *const Gfx as *mut Gfx,};
#[no_mangle]
pub static mut sViMode: ViMode =
    ViMode{customViMode:
               OSViMode{type_0: 0,
                        comRegs:
                            OSViCommonRegs{ctrl: 0,
                                           width: 0,
                                           burst: 0,
                                           vSync: 0,
                                           hSync: 0,
                                           leap: 0,
                                           hStart: 0,
                                           xScale: 0,
                                           vCurrent: 0,},
                        fldRegs:
                            [OSViFieldRegs{origin: 0,
                                           yScale: 0,
                                           vStart: 0,
                                           vBurst: 0,
                                           vIntr: 0,}; 2],},
           viHeight: 0,
           viWidth: 0,
           unk_58: 0,
           unk_5C: 0,
           unk_60: 0,
           unk_64: 0,
           viModeBase: 0,
           viTvType: 0,
           unk_70: 0,
           unk_74: 0,
           unk_78: 0,
           unk_7C: 0,
           viFeatures: 0,
           unk_84: 0,};
#[no_mangle]
pub static mut sGameFaultClient: FaultClient =
    FaultClient{next: 0 as *const FaultClient as *mut FaultClient,
                callback: 0,
                param1: 0,
                param2: 0,};
#[no_mangle]
pub static mut sLastButtonPressed: u16_0 = 0;
#[no_mangle]
pub unsafe extern "C" fn GameState_FaultPrint() {
    static mut sBtnChars: [libc::c_char; 17] =
        unsafe {
            *::std::mem::transmute::<&[u8; 17],
                                     &mut [libc::c_char; 17]>(b"ABZSuldr*+LRudlr\x00")
        };
    let mut i: s32 = 0;
    osSyncPrintf(b"last_button=%04x\n\x00" as *const u8 as
                     *const libc::c_char, sLastButtonPressed as libc::c_int);
    FaultDrawer_DrawText(120 as libc::c_int, 180 as libc::c_int,
                         b"%08x\x00" as *const u8 as *const libc::c_char,
                         sLastButtonPressed as libc::c_int);
    i = 0 as libc::c_int;
    while i <
              (::std::mem::size_of::<[libc::c_char; 17]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_char>()
                                                   as libc::c_ulong) as s32 {
        if sLastButtonPressed as libc::c_int & (1 as libc::c_int) << i != 0 {
            FaultDrawer_DrawText(i * 8 as libc::c_int + 0x78 as libc::c_int,
                                 0xbe as libc::c_int,
                                 b"%c\x00" as *const u8 as
                                     *const libc::c_char,
                                 sBtnChars[i as usize] as libc::c_int);
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn GameState_SetFBFilter(mut gfx: *mut *mut Gfx) {
    let mut gfxP: *mut Gfx = 0 as *mut Gfx;
    gfxP = *gfx;
    if (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 80 as libc::c_int) as usize]
           as libc::c_int > 0 as libc::c_int &&
           ((*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 80 as libc::c_int) as
                                  usize] as libc::c_int) < 5 as libc::c_int {
        D_801664F0.type_0 =
            (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 80 as libc::c_int) as
                                  usize] as u32_0;
        D_801664F0.color.c2rust_unnamed.r =
            (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 81 as libc::c_int +
                                   0 as libc::c_int) as usize] as u8_0;
        D_801664F0.color.c2rust_unnamed.g =
            (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 81 as libc::c_int +
                                   1 as libc::c_int) as usize] as u8_0;
        D_801664F0.color.c2rust_unnamed.b =
            (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 81 as libc::c_int +
                                   2 as libc::c_int) as usize] as u8_0;
        D_801664F0.color.c2rust_unnamed.a =
            (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 84 as libc::c_int) as
                                  usize] as u8_0;
        func_800ACE98(&mut D_801664F0, &mut gfxP);
    } else if (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                     16 as libc::c_int + 80 as libc::c_int) as
                                    usize] as libc::c_int == 5 as libc::c_int
                  ||
                  (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                         16 as libc::c_int +
                                         80 as libc::c_int) as usize] as
                      libc::c_int == 6 as libc::c_int {
        D_80166500.useRgba =
            ((*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                    16 as libc::c_int + 80 as libc::c_int) as
                                   usize] as libc::c_int == 6 as libc::c_int)
                as libc::c_int as u32_0;
        D_80166500.primColor.c2rust_unnamed.r =
            (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 81 as libc::c_int +
                                   0 as libc::c_int) as usize] as u8_0;
        D_80166500.primColor.c2rust_unnamed.g =
            (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 81 as libc::c_int +
                                   1 as libc::c_int) as usize] as u8_0;
        D_80166500.primColor.c2rust_unnamed.b =
            (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 81 as libc::c_int +
                                   2 as libc::c_int) as usize] as u8_0;
        D_80166500.primColor.c2rust_unnamed.a =
            (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 84 as libc::c_int) as
                                  usize] as u8_0;
        D_80166500.envColor.c2rust_unnamed.r =
            (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 85 as libc::c_int +
                                   0 as libc::c_int) as usize] as u8_0;
        D_80166500.envColor.c2rust_unnamed.g =
            (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 85 as libc::c_int +
                                   1 as libc::c_int) as usize] as u8_0;
        D_80166500.envColor.c2rust_unnamed.b =
            (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 85 as libc::c_int +
                                   2 as libc::c_int) as usize] as u8_0;
        D_80166500.envColor.c2rust_unnamed.a =
            (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 84 as libc::c_int) as
                                  usize] as u8_0;
        func_800AD958(&mut D_80166500, &mut gfxP);
    } else if (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                     16 as libc::c_int + 80 as libc::c_int) as
                                    usize] as libc::c_int == 7 as libc::c_int
     {
        sMonoColors.unk_00 = 0 as libc::c_int as u32_0;
        sMonoColors.primColor.c2rust_unnamed.r =
            (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 81 as libc::c_int +
                                   0 as libc::c_int) as usize] as u8_0;
        sMonoColors.primColor.c2rust_unnamed.g =
            (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 81 as libc::c_int +
                                   1 as libc::c_int) as usize] as u8_0;
        sMonoColors.primColor.c2rust_unnamed.b =
            (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 81 as libc::c_int +
                                   2 as libc::c_int) as usize] as u8_0;
        sMonoColors.primColor.c2rust_unnamed.a =
            (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 84 as libc::c_int) as
                                  usize] as u8_0;
        sMonoColors.envColor.c2rust_unnamed.r =
            (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 85 as libc::c_int +
                                   0 as libc::c_int) as usize] as u8_0;
        sMonoColors.envColor.c2rust_unnamed.g =
            (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 85 as libc::c_int +
                                   1 as libc::c_int) as usize] as u8_0;
        sMonoColors.envColor.c2rust_unnamed.b =
            (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 85 as libc::c_int +
                                   2 as libc::c_int) as usize] as u8_0;
        sMonoColors.envColor.c2rust_unnamed.a =
            (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 84 as libc::c_int) as
                                  usize] as u8_0;
        VisMono_Draw(&mut sMonoColors, &mut gfxP);
    }
    *gfx = gfxP;
}
#[no_mangle]
pub unsafe extern "C" fn func_800C4344(mut gameState: *mut GameState) {
    let mut selectedInput: *mut Input = 0 as *mut Input;
    let mut hexDumpSize: s32 = 0;
    let mut hReg82: u16_0 = 0;
    if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 80 as libc::c_int) as usize]
           as libc::c_int == 0x14 as libc::c_int {
        __osMalloc_FreeBlockTest_Enable =
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 82 as libc::c_int) as
                                  usize] as u32_0
    }
    if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 80 as libc::c_int) as usize]
           as libc::c_int == 0xc as libc::c_int {
        selectedInput =
            &mut *(*gameState).input.as_mut_ptr().offset(if (*(*gGameInfo).data.as_mut_ptr().offset((21
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         *
                                                                                                         6
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                         *
                                                                                                         16
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                         +
                                                                                                         81
                                                                                                             as
                                                                                                             libc::c_int)
                                                                                                        as
                                                                                                        isize)
                                                                 as u32_0) <
                                                                4 as
                                                                    libc::c_uint
                                                            {
                                                             *(*gGameInfo).data.as_mut_ptr().offset((21
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         *
                                                                                                         6
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                         *
                                                                                                         16
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                         +
                                                                                                         81
                                                                                                             as
                                                                                                             libc::c_int)
                                                                                                        as
                                                                                                        isize)
                                                                 as
                                                                 libc::c_int
                                                         } else {
                                                             0 as libc::c_int
                                                         } as isize) as
                *mut Input;
        hReg82 =
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 82 as libc::c_int) as
                                  usize] as u16_0;
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 83 as libc::c_int) as
                              usize] = (*selectedInput).cur.button as s16;
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 84 as libc::c_int) as
                              usize] = (*selectedInput).press.button as s16;
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 85 as libc::c_int) as
                              usize] = (*selectedInput).rel.stick_x as s16;
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 86 as libc::c_int) as
                              usize] = (*selectedInput).rel.stick_y as s16;
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 87 as libc::c_int) as
                              usize] = (*selectedInput).rel.stick_x as s16;
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 88 as libc::c_int) as
                              usize] = (*selectedInput).rel.stick_y as s16;
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 89 as libc::c_int) as
                              usize] = (*selectedInput).cur.stick_x as s16;
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 90 as libc::c_int) as
                              usize] = (*selectedInput).cur.stick_y as s16;
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 93 as libc::c_int) as
                              usize] =
            ((*selectedInput).cur.button as libc::c_int ==
                 hReg82 as libc::c_int) as libc::c_int as s16;
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 94 as libc::c_int) as
                              usize] =
            (!((*selectedInput).cur.button as libc::c_int |
                   !(hReg82 as libc::c_int)) == 0 as libc::c_int) as
                libc::c_int as s16;
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 95 as libc::c_int) as
                              usize] =
            (!((*selectedInput).press.button as libc::c_int |
                   !(hReg82 as libc::c_int)) == 0 as libc::c_int) as
                libc::c_int as s16
    }
    if gIsCtrlr2Valid != 0 {
        func_8006390C(&mut *(*gameState).input.as_mut_ptr().offset(1 as
                                                                       libc::c_int
                                                                       as
                                                                       isize));
    }
    D_80009460 =
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 60 as libc::c_int) as
                              usize] as u32_0;
    gDmaMgrDmaBuffSize =
        if (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 21 as libc::c_int) as
                                 usize] as libc::c_int != 0 as libc::c_int {
            ((*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                    16 as libc::c_int + 21 as libc::c_int) as
                                   usize] as libc::c_int + 0xf as libc::c_int)
                & !(0xf as libc::c_int)
        } else { 0x2000 as libc::c_int } as u32_0;
    gSystemArenaLogSeverity =
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 61 as libc::c_int) as
                              usize] as s32;
    gZeldaArenaLogSeverity =
        (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 62 as libc::c_int) as
                              usize] as s32;
    if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 80 as libc::c_int) as usize]
           as libc::c_int == 8 as libc::c_int {
        if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 94 as libc::c_int) as
                                 usize] as libc::c_int != 8 as libc::c_int {
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 94 as libc::c_int) as
                                  usize] = 8 as libc::c_int as s16;
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 81 as libc::c_int) as
                                  usize] = 0 as libc::c_int as s16;
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 82 as libc::c_int) as
                                  usize] = 0 as libc::c_int as s16;
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 83 as libc::c_int) as
                                  usize] = 0 as libc::c_int as s16
        }
        if ((*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 81 as libc::c_int) as
                                  usize] as libc::c_int) < 0 as libc::c_int {
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 81 as libc::c_int) as
                                  usize] = 0 as libc::c_int as s16;
            // & 0xFFFFFFFF necessary for matching.
            hexDumpSize =
                ((if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                            16 as libc::c_int +
                                            83 as libc::c_int) as usize] as
                         libc::c_int == 0 as libc::c_int {
                      0x100 as libc::c_int
                  } else {
                      ((*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int
                                              * 16 as libc::c_int +
                                              83 as libc::c_int) as usize] as
                           libc::c_int) * 0x10 as libc::c_int
                  }) as libc::c_uint & 0xffffffff as libc::c_uint) as s32;
            LogUtils_LogHexDump(((((*gGameInfo).data[(21 as libc::c_int *
                                                          6 as libc::c_int *
                                                          16 as libc::c_int +
                                                          82 as libc::c_int)
                                                         as usize] as
                                       libc::c_int) << 8 as libc::c_int) as
                                     u32_0).wrapping_add(0x80000000 as
                                                             libc::c_uint) as
                                    *mut libc::c_void, hexDumpSize);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn GameState_DrawInputDisplay(mut input: u16_0,
                                                    mut gfx: *mut *mut Gfx) {
    static mut sInpDispBtnColors: [u16_0; 16] =
        [((255 as libc::c_int) << 8 as libc::c_int & 0xf800 as libc::c_int |
              (255 as libc::c_int) << 3 as libc::c_int & 0x7c0 as libc::c_int
              | 0 as libc::c_int >> 2 as libc::c_int & 0x3e as libc::c_int |
              1 as libc::c_int & 0x1 as libc::c_int) as u16_0,
         ((255 as libc::c_int) << 8 as libc::c_int & 0xf800 as libc::c_int |
              (255 as libc::c_int) << 3 as libc::c_int & 0x7c0 as libc::c_int
              | 0 as libc::c_int >> 2 as libc::c_int & 0x3e as libc::c_int |
              1 as libc::c_int & 0x1 as libc::c_int) as u16_0,
         ((255 as libc::c_int) << 8 as libc::c_int & 0xf800 as libc::c_int |
              (255 as libc::c_int) << 3 as libc::c_int & 0x7c0 as libc::c_int
              | 0 as libc::c_int >> 2 as libc::c_int & 0x3e as libc::c_int |
              1 as libc::c_int & 0x1 as libc::c_int) as u16_0,
         ((255 as libc::c_int) << 8 as libc::c_int & 0xf800 as libc::c_int |
              (255 as libc::c_int) << 3 as libc::c_int & 0x7c0 as libc::c_int
              | 0 as libc::c_int >> 2 as libc::c_int & 0x3e as libc::c_int |
              1 as libc::c_int & 0x1 as libc::c_int) as u16_0,
         ((120 as libc::c_int) << 8 as libc::c_int & 0xf800 as libc::c_int |
              (120 as libc::c_int) << 3 as libc::c_int & 0x7c0 as libc::c_int
              | 120 as libc::c_int >> 2 as libc::c_int & 0x3e as libc::c_int |
              1 as libc::c_int & 0x1 as libc::c_int) as u16_0,
         ((120 as libc::c_int) << 8 as libc::c_int & 0xf800 as libc::c_int |
              (120 as libc::c_int) << 3 as libc::c_int & 0x7c0 as libc::c_int
              | 120 as libc::c_int >> 2 as libc::c_int & 0x3e as libc::c_int |
              1 as libc::c_int & 0x1 as libc::c_int) as u16_0,
         ((0 as libc::c_int) << 8 as libc::c_int & 0xf800 as libc::c_int |
              (255 as libc::c_int) << 3 as libc::c_int & 0x7c0 as libc::c_int
              | 255 as libc::c_int >> 2 as libc::c_int & 0x3e as libc::c_int |
              1 as libc::c_int & 0x1 as libc::c_int) as u16_0,
         ((255 as libc::c_int) << 8 as libc::c_int & 0xf800 as libc::c_int |
              (0 as libc::c_int) << 3 as libc::c_int & 0x7c0 as libc::c_int |
              255 as libc::c_int >> 2 as libc::c_int & 0x3e as libc::c_int |
              1 as libc::c_int & 0x1 as libc::c_int) as u16_0,
         ((120 as libc::c_int) << 8 as libc::c_int & 0xf800 as libc::c_int |
              (120 as libc::c_int) << 3 as libc::c_int & 0x7c0 as libc::c_int
              | 120 as libc::c_int >> 2 as libc::c_int & 0x3e as libc::c_int |
              1 as libc::c_int & 0x1 as libc::c_int) as u16_0,
         ((120 as libc::c_int) << 8 as libc::c_int & 0xf800 as libc::c_int |
              (120 as libc::c_int) << 3 as libc::c_int & 0x7c0 as libc::c_int
              | 120 as libc::c_int >> 2 as libc::c_int & 0x3e as libc::c_int |
              1 as libc::c_int & 0x1 as libc::c_int) as u16_0,
         ((120 as libc::c_int) << 8 as libc::c_int & 0xf800 as libc::c_int |
              (120 as libc::c_int) << 3 as libc::c_int & 0x7c0 as libc::c_int
              | 120 as libc::c_int >> 2 as libc::c_int & 0x3e as libc::c_int |
              1 as libc::c_int & 0x1 as libc::c_int) as u16_0,
         ((120 as libc::c_int) << 8 as libc::c_int & 0xf800 as libc::c_int |
              (120 as libc::c_int) << 3 as libc::c_int & 0x7c0 as libc::c_int
              | 120 as libc::c_int >> 2 as libc::c_int & 0x3e as libc::c_int |
              1 as libc::c_int & 0x1 as libc::c_int) as u16_0,
         ((255 as libc::c_int) << 8 as libc::c_int & 0xf800 as libc::c_int |
              (0 as libc::c_int) << 3 as libc::c_int & 0x7c0 as libc::c_int |
              0 as libc::c_int >> 2 as libc::c_int & 0x3e as libc::c_int |
              1 as libc::c_int & 0x1 as libc::c_int) as u16_0,
         ((120 as libc::c_int) << 8 as libc::c_int & 0xf800 as libc::c_int |
              (120 as libc::c_int) << 3 as libc::c_int & 0x7c0 as libc::c_int
              | 120 as libc::c_int >> 2 as libc::c_int & 0x3e as libc::c_int |
              1 as libc::c_int & 0x1 as libc::c_int) as u16_0,
         ((0 as libc::c_int) << 8 as libc::c_int & 0xf800 as libc::c_int |
              (255 as libc::c_int) << 3 as libc::c_int & 0x7c0 as libc::c_int
              | 0 as libc::c_int >> 2 as libc::c_int & 0x3e as libc::c_int |
              1 as libc::c_int & 0x1 as libc::c_int) as u16_0,
         ((0 as libc::c_int) << 8 as libc::c_int & 0xf800 as libc::c_int |
              (0 as libc::c_int) << 3 as libc::c_int & 0x7c0 as libc::c_int |
              255 as libc::c_int >> 2 as libc::c_int & 0x3e as libc::c_int |
              1 as libc::c_int & 0x1 as libc::c_int) as u16_0];
    let mut i: s32 = 0;
    let mut j: s32 = 0;
    let mut k: s32 = 0;
    let mut gfxP: *mut Gfx = *gfx;
    let fresh0 = gfxP;
    gfxP = gfxP.offset(1);
    let mut _g: *mut Gfx = fresh0;
    (*_g).words.w0 =
        (0xe7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh1 = gfxP;
    gfxP = gfxP.offset(1);
    let mut _g_0: *mut Gfx = fresh1;
    (*_g_0).words.w0 =
        (0xef as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (((0 as libc::c_int) << 4 as libc::c_int |
                  (0 as libc::c_int) << 6 as libc::c_int |
                  (0 as libc::c_int) << 8 as libc::c_int |
                  (0 as libc::c_int) << 9 as libc::c_int |
                  (0 as libc::c_int) << 12 as libc::c_int |
                  (0 as libc::c_int) << 14 as libc::c_int |
                  (0 as libc::c_int) << 16 as libc::c_int |
                  (0 as libc::c_int) << 17 as libc::c_int |
                  (0 as libc::c_int) << 19 as libc::c_int |
                  (3 as libc::c_int) << 20 as libc::c_int |
                  (0 as libc::c_int) << 23 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 24 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_0).words.w1 =
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
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        j = i;
        if input as libc::c_int & (1 as libc::c_int) << i != 0 {
            let fresh2 = gfxP;
            gfxP = gfxP.offset(1);
            let mut _g_1: *mut Gfx = fresh2;
            (*_g_1).words.w0 =
                (0xf7 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_1).words.w1 =
                ((sInpDispBtnColors[i as usize] as libc::c_int) <<
                     0x10 as libc::c_int |
                     sInpDispBtnColors[i as usize] as libc::c_int) as
                    libc::c_uint;
            k = i + 1 as libc::c_int;
            let fresh3 = gfxP;
            gfxP = gfxP.offset(1);
            let mut _g_2: *mut Gfx = fresh3;
            (*_g_2).words.w0 =
                (0xf6 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    ((k * 4 as libc::c_int + 225 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 10 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        14 as libc::c_int |
                    (223 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 10 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        2 as libc::c_int;
            (*_g_2).words.w1 =
                ((j * 4 as libc::c_int + 226 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 10 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    14 as libc::c_int |
                    (220 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 10 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        2 as libc::c_int;
            let fresh4 = gfxP;
            gfxP = gfxP.offset(1);
            let mut _g_3: *mut Gfx = fresh4;
            (*_g_3).words.w0 =
                (0xe7 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int;
            (*_g_3).words.w1 = 0 as libc::c_int as libc::c_uint
        }
        i += 1
    }
    *gfx = gfxP;
}
#[no_mangle]
pub unsafe extern "C" fn GameState_Draw(mut gameState: *mut GameState,
                                        mut gfxCtx: *mut GraphicsContext) {
    let mut newDList: *mut Gfx = 0 as *mut Gfx;
    let mut polyOpaP: *mut Gfx = 0 as *mut Gfx;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), gfxCtx,
                    b"../game.c\x00" as *const u8 as *const libc::c_char,
                    746 as libc::c_int);
    polyOpaP = (*__gfxCtx).polyOpa.p;
    newDList = Graph_GfxPlusOne(polyOpaP);
    let fresh5 = (*__gfxCtx).overlay.p;
    (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
    let mut _g: *mut Gfx = fresh5;
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
    (*_g).words.w1 = newDList as libc::c_uint;
    if (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 88 as libc::c_int) as usize]
           as libc::c_int == 1 as libc::c_int {
        GameState_SetFBFilter(&mut newDList);
    }
    sLastButtonPressed =
        ((*gameState).input[0 as libc::c_int as usize].press.button as
             libc::c_int |
             (*gameState).input[0 as libc::c_int as usize].cur.button as
                 libc::c_int) as u16_0;
    if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 47 as libc::c_int) as usize]
           as libc::c_int == 0 as libc::c_int {
        GameState_DrawInputDisplay(sLastButtonPressed, &mut newDList);
    }
    if (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 36 as libc::c_int) as usize]
           as libc::c_int & 1 as libc::c_int != 0 {
        let mut pad: s32 = 0;
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
                                             C2RustUnnamed_0{r: 0,
                                                             g: 0,
                                                             b: 0,
                                                             a: 0,},},
                     unk_14: [0; 28],};
        GfxPrint_Init(&mut printer);
        GfxPrint_Open(&mut printer, newDList);
        AudioDebug_Draw(&mut printer);
        newDList = GfxPrint_Close(&mut printer);
        GfxPrint_Destroy(&mut printer);
    }
    if ((*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 0 as libc::c_int) as usize]
            as libc::c_int) < 0 as libc::c_int {
        let mut pad_0: s32 = 0;
        DebugArena_Display();
        SystemArena_Display();
        // "%08x bytes left until the death of Hyrule (game_alloc)"
        osSyncPrintf(b"\xe3\x83\x8f\xe3\x82\xa4\xe3\x83\xa9\xe3\x83\xab\xe6\xbb\x85\xe4\xba\xa1\xe3\x81\xbe\xe3\x81\xa7\xe3\x81\x82\xe3\x81\xa8 %08x \xe3\x83\x90\xe3\x82\xa4\xe3\x83\x88(game_alloc)\n\x00"
                         as *const u8 as *const libc::c_char,
                     THA_GetSize(&mut (*gameState).tha)); // "Hyrule reserved size = %u bytes"
        (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                               16 as libc::c_int + 0 as libc::c_int) as usize]
            = 0 as libc::c_int as s16
    }
    let fresh6 = newDList;
    newDList = newDList.offset(1);
    let mut _g_0: *mut Gfx = fresh6;
    (*_g_0).words.w0 =
        (0xdf as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_0).words.w1 = 0 as libc::c_int as libc::c_uint;
    Graph_BranchDlist(polyOpaP, newDList);
    (*__gfxCtx).polyOpa.p = newDList;
    Graph_CloseDisps(dispRefs.as_mut_ptr(), gfxCtx,
                     b"../game.c\x00" as *const u8 as *const libc::c_char,
                     800 as libc::c_int);
    func_80063D7C(gfxCtx);
    if (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 0 as libc::c_int) as usize]
           as libc::c_int != 0 as libc::c_int {
        SpeedMeter_DrawTimeEntries(&mut D_801664D0, gfxCtx);
        SpeedMeter_DrawAllocEntries(&mut D_801664D0, gfxCtx, gameState);
    };
}
#[no_mangle]
pub unsafe extern "C" fn GameState_SetFrameBuffer(mut gfxCtx:
                                                      *mut GraphicsContext) {
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), gfxCtx,
                    b"../game.c\x00" as *const u8 as *const libc::c_char,
                    814 as libc::c_int);
    let fresh7 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g: *mut Gfx = fresh7;
    (*_g).words.w0 =
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
    (*_g).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh8 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_0: *mut Gfx = fresh8;
    (*_g_0).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((0xf as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_0).words.w1 = (*gfxCtx).curFrameBuffer as libc::c_uint;
    let fresh9 = (*__gfxCtx).polyOpa.p;
    (*__gfxCtx).polyOpa.p = (*__gfxCtx).polyOpa.p.offset(1);
    let mut _g_1: *mut Gfx = fresh9;
    (*_g_1).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((0xe as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_1).words.w1 = gZBuffer.as_mut_ptr() as libc::c_uint;
    let fresh10 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_2: *mut Gfx = fresh10;
    (*_g_2).words.w0 =
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
    (*_g_2).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh11 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_3: *mut Gfx = fresh11;
    (*_g_3).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((0xf as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_3).words.w1 = (*gfxCtx).curFrameBuffer as libc::c_uint;
    let fresh12 = (*__gfxCtx).polyXlu.p;
    (*__gfxCtx).polyXlu.p = (*__gfxCtx).polyXlu.p.offset(1);
    let mut _g_4: *mut Gfx = fresh12;
    (*_g_4).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((0xe as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_4).words.w1 = gZBuffer.as_mut_ptr() as libc::c_uint;
    let fresh13 = (*__gfxCtx).overlay.p;
    (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
    let mut _g_5: *mut Gfx = fresh13;
    (*_g_5).words.w0 =
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
    (*_g_5).words.w1 = 0 as libc::c_int as libc::c_uint;
    let fresh14 = (*__gfxCtx).overlay.p;
    (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
    let mut _g_6: *mut Gfx = fresh14;
    (*_g_6).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((0xf as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_6).words.w1 = (*gfxCtx).curFrameBuffer as libc::c_uint;
    let fresh15 = (*__gfxCtx).overlay.p;
    (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
    let mut _g_7: *mut Gfx = fresh15;
    (*_g_7).words.w0 =
        (0xdb as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x6 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((0xe as libc::c_int * 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_g_7).words.w1 = gZBuffer.as_mut_ptr() as libc::c_uint;
    Graph_CloseDisps(dispRefs.as_mut_ptr(), gfxCtx,
                     b"../game.c\x00" as *const u8 as *const libc::c_char,
                     838 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn func_800C49F4(mut gfxCtx: *mut GraphicsContext) {
    let mut newDlist: *mut Gfx = 0 as *mut Gfx;
    let mut polyOpaP: *mut Gfx = 0 as *mut Gfx;
    let mut __gfxCtx: *mut GraphicsContext = 0 as *mut GraphicsContext;
    let mut dispRefs: [*mut Gfx; 4] = [0 as *mut Gfx; 4];
    __gfxCtx = gfxCtx;
    Graph_OpenDisps(dispRefs.as_mut_ptr(), gfxCtx,
                    b"../game.c\x00" as *const u8 as *const libc::c_char,
                    846 as libc::c_int);
    polyOpaP = (*__gfxCtx).polyOpa.p;
    newDlist = Graph_GfxPlusOne(polyOpaP);
    let fresh16 = (*__gfxCtx).overlay.p;
    (*__gfxCtx).overlay.p = (*__gfxCtx).overlay.p.offset(1);
    let mut _g: *mut Gfx = fresh16;
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
    (*_g).words.w1 = newDlist as libc::c_uint;
    let fresh17 = newDlist;
    newDlist = newDlist.offset(1);
    let mut _g_0: *mut Gfx = fresh17;
    (*_g_0).words.w0 =
        (0xdf as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*_g_0).words.w1 = 0 as libc::c_int as libc::c_uint;
    Graph_BranchDlist(polyOpaP, newDlist);
    (*__gfxCtx).polyOpa.p = newDlist;
    Graph_CloseDisps(dispRefs.as_mut_ptr(), gfxCtx,
                     b"../game.c\x00" as *const u8 as *const libc::c_char,
                     865 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn GameState_ReqPadData(mut gameState: *mut GameState) {
    PadMgr_RequestPadData(&mut gPadMgr,
                          &mut *(*gameState).input.as_mut_ptr().offset(0 as
                                                                           libc::c_int
                                                                           as
                                                                           isize),
                          1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn GameState_Update(mut gameState: *mut GameState) {
    let mut gfxCtx: *mut GraphicsContext = (*gameState).gfxCtx;
    GameState_SetFrameBuffer(gfxCtx);
    (*gameState).main.expect("non-null function pointer")(gameState);
    func_800C4344(gameState);
    if (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 63 as libc::c_int) as usize]
           as libc::c_uint == 1 as libc::c_uint {
        if ((*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 48 as libc::c_int) as
                                  usize] as libc::c_int) < 0 as libc::c_int {
            (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 48 as libc::c_int) as
                                  usize] = 0 as libc::c_int as s16;
            (*gfxCtx).viMode = &mut gViConfigMode;
            (*gfxCtx).viFeatures = gViConfigFeatures;
            (*gfxCtx).xScale = gViConfigXScale;
            (*gfxCtx).yScale = gViConfigYScale
        } else if (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                         16 as libc::c_int +
                                         48 as libc::c_int) as usize] as
                      libc::c_int > 0 as libc::c_int {
            ViMode_Update(&mut sViMode, (*gameState).input.as_mut_ptr());
            (*gfxCtx).viMode = &mut sViMode.customViMode;
            (*gfxCtx).viFeatures = sViMode.viFeatures;
            (*gfxCtx).xScale = 1.0f32;
            (*gfxCtx).yScale = 1.0f32
        }
    } else if (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                     16 as libc::c_int + 63 as libc::c_int) as
                                    usize] as libc::c_int >= 2 as libc::c_int
     {
        (*gfxCtx).viMode = &mut gViConfigMode;
        (*gfxCtx).viFeatures = gViConfigFeatures;
        (*gfxCtx).xScale = gViConfigXScale;
        (*gfxCtx).yScale = gViConfigYScale;
        if (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 63 as libc::c_int) as
                                 usize] as libc::c_int == 6 as libc::c_int ||
               (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                      16 as libc::c_int + 63 as libc::c_int)
                                     as usize] as libc::c_uint ==
                   2 as libc::c_uint &&
                   osTvType == 1 as libc::c_int as libc::c_uint {
            (*gfxCtx).viMode = &mut osViModeNtscLan1;
            (*gfxCtx).yScale = 1.0f32
        }
        if (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 63 as libc::c_int) as
                                 usize] as libc::c_int == 5 as libc::c_int ||
               (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                      16 as libc::c_int + 63 as libc::c_int)
                                     as usize] as libc::c_uint ==
                   2 as libc::c_uint &&
                   osTvType == 2 as libc::c_int as libc::c_uint {
            (*gfxCtx).viMode = &mut osViModeMpalLan1;
            (*gfxCtx).yScale = 1.0f32
        }
        if (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 63 as libc::c_int) as
                                 usize] as libc::c_int == 4 as libc::c_int ||
               (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                      16 as libc::c_int + 63 as libc::c_int)
                                     as usize] as libc::c_uint ==
                   2 as libc::c_uint &&
                   osTvType == 0 as libc::c_int as libc::c_uint {
            (*gfxCtx).viMode = &mut osViModePalLan1;
            (*gfxCtx).yScale = 1.0f32
        }
        if (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 63 as libc::c_int) as
                                 usize] as libc::c_int == 3 as libc::c_int ||
               (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                                      16 as libc::c_int + 63 as libc::c_int)
                                     as usize] as libc::c_uint ==
                   2 as libc::c_uint &&
                   osTvType == 0 as libc::c_int as libc::c_uint {
            (*gfxCtx).viMode = &mut osViModeFpalLan1;
            (*gfxCtx).yScale = 0.833f32
        }
    } else { (*gfxCtx).viMode = 0 as *mut OSViMode }
    if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 80 as libc::c_int) as usize]
           as libc::c_int == 0x15 as libc::c_int {
        if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 95 as libc::c_int) as
                                 usize] as libc::c_int != 0x15 as libc::c_int
           {
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 95 as libc::c_int) as
                                  usize] = 0x15 as libc::c_int as s16;
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 81 as libc::c_int) as
                                  usize] = 0 as libc::c_int as s16;
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 82 as libc::c_int) as
                                  usize] =
                gViConfigAdditionalScanLines as s16;
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 83 as libc::c_int) as
                                  usize] = 0 as libc::c_int as s16;
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 84 as libc::c_int) as
                                  usize] = 0 as libc::c_int as s16
        }
        if ((*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 82 as libc::c_int) as
                                  usize] as libc::c_int) < 0 as libc::c_int {
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 82 as libc::c_int) as
                                  usize] = 0 as libc::c_int as s16
        }
        if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 82 as libc::c_int) as
                                 usize] as libc::c_int > 0x30 as libc::c_int {
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 82 as libc::c_int) as
                                  usize] = 0x30 as libc::c_int as s16
        }
        if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                  16 as libc::c_int + 83 as libc::c_int) as
                                 usize] as libc::c_int !=
               (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                      16 as libc::c_int + 82 as libc::c_int)
                                     as usize] as libc::c_int ||
               (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                      16 as libc::c_int + 84 as libc::c_int)
                                     as usize] as libc::c_int !=
                   (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          81 as libc::c_int) as usize] as
                       libc::c_int {
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 83 as libc::c_int) as
                                  usize] =
                (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                       16 as libc::c_int + 82 as libc::c_int)
                                      as usize];
            (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                   16 as libc::c_int + 84 as libc::c_int) as
                                  usize] =
                (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                       16 as libc::c_int + 81 as libc::c_int)
                                      as usize];
            gViConfigAdditionalScanLines =
                (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                       16 as libc::c_int + 82 as libc::c_int)
                                      as usize] as u8_0;
            gViConfigYScale =
                if (*gGameInfo).data[(21 as libc::c_int * 6 as libc::c_int *
                                          16 as libc::c_int +
                                          81 as libc::c_int) as usize] as
                       libc::c_int == 0 as libc::c_int {
                    (240.0f32) /
                        (gViConfigAdditionalScanLines as libc::c_int as
                             libc::c_float + 240.0f32)
                } else { 1.0f32 };
            D_80009430 = 1 as libc::c_int as s8
        }
    }
    if (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 94 as libc::c_int) as usize]
           as libc::c_uint != 2 as libc::c_uint {
        GameState_Draw(gameState, gfxCtx);
        func_800C49F4(gfxCtx);
    }
    (*gameState).frames = (*gameState).frames.wrapping_add(1);
}
#[no_mangle]
pub unsafe extern "C" fn GameState_InitArena(mut gameState: *mut GameState,
                                             mut size: size_t) {
    let mut arena: *mut libc::c_void = 0 as *mut libc::c_void;
    osSyncPrintf(b"\xe3\x83\x8f\xe3\x82\xa4\xe3\x83\xa9\xe3\x83\xab\xe7\xa2\xba\xe4\xbf\x9d \xe3\x82\xb5\xe3\x82\xa4\xe3\x82\xba\xef\xbc\x9d%u \xe3\x83\x90\xe3\x82\xa4\xe3\x83\x88\n\x00"
                     as *const u8 as *const libc::c_char);
    arena =
        GameAlloc_MallocDebug(&mut (*gameState).alloc, size as u32_0,
                              b"../game.c\x00" as *const u8 as
                                  *const libc::c_char, 992 as libc::c_int);
    if !arena.is_null() {
        THA_Ct(&mut (*gameState).tha, arena, size as u32_0);
        osSyncPrintf(b"\xe3\x83\x8f\xe3\x82\xa4\xe3\x83\xa9\xe3\x83\xab\xe7\xa2\xba\xe4\xbf\x9d\xe6\x88\x90\xe5\x8a\x9f\n\x00"
                         as *const u8 as *const libc::c_char);
        // "Successful Hyral"
    } else {
        THA_Ct(&mut (*gameState).tha, 0 as *mut libc::c_void,
               0 as libc::c_int as u32_0); // "Failure to secure Hyrule"
        osSyncPrintf(b"\xe3\x83\x8f\xe3\x82\xa4\xe3\x83\xa9\xe3\x83\xab\xe7\xa2\xba\xe4\xbf\x9d\xe5\xa4\xb1\xe6\x95\x97\n\x00"
                         as *const u8 as
                         *const libc::c_char); // "Hyrule temporarily released!!"
        Fault_AddHungupAndCrash(b"../game.c\x00" as *const u8 as
                                    *const libc::c_char,
                                999 as libc::c_int as u32_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn GameState_Realloc(mut gameState: *mut GameState,
                                           mut size: size_t) {
    let mut alloc: *mut GameAlloc = &mut (*gameState).alloc;
    let mut gameArena: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut systemMaxFree: u32_0 = 0;
    let mut systemFree: u32_0 = 0;
    let mut systemAlloc: u32_0 = 0;
    let mut thaBufp: *mut libc::c_void = (*gameState).tha.bufp;
    THA_Dt(&mut (*gameState).tha);
    GameAlloc_Free(alloc, thaBufp);
    osSyncPrintf(b"\xe3\x83\x8f\xe3\x82\xa4\xe3\x83\xa9\xe3\x83\xab\xe4\xb8\x80\xe6\x99\x82\xe8\xa7\xa3\xe6\x94\xbe!!\n\x00"
                     as *const u8 as *const libc::c_char);
    SystemArena_GetSizes(&mut systemMaxFree, &mut systemFree,
                         &mut systemAlloc);
    if (systemMaxFree.wrapping_sub(0x10 as libc::c_int as libc::c_uint) as
            libc::c_ulong) < size {
        osSyncPrintf(b"%c\x00" as *const u8 as *const libc::c_char,
                     7 as libc::c_int);
        osSyncPrintf(b"\x1b[31m\x00" as *const u8 as *const libc::c_char);
        // "Not enough memory. Change the hyral size to the largest possible value"
        osSyncPrintf(b"\xe3\x83\xa1\xe3\x83\xa2\xe3\x83\xaa\xe3\x81\x8c\xe8\xb6\xb3\xe3\x82\x8a\xe3\x81\xbe\xe3\x81\x9b\xe3\x82\x93\xe3\x80\x82\xe3\x83\x8f\xe3\x82\xa4\xe3\x83\xa9\xe3\x83\xab\xe3\x82\xb5\xe3\x82\xa4\xe3\x82\xba\xe3\x82\x92\xe5\x8f\xaf\xe8\x83\xbd\xe3\x81\xaa\xe6\x9c\x80\xe5\xa4\xa7\xe5\x80\xa4\xe3\x81\xab\xe5\xa4\x89\xe6\x9b\xb4\xe3\x81\x97\xe3\x81\xbe\xe3\x81\x99\n\x00"
                         as *const u8 as
                         *const libc::c_char); // "Hyral reallocate size = %u bytes"
        osSyncPrintf(b"(hyral=%08x max=%08x free=%08x alloc=%08x)\n\x00" as
                         *const u8 as *const libc::c_char, size,
                     systemMaxFree, systemFree, systemAlloc);
        osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
        size =
            systemMaxFree.wrapping_sub(0x10 as libc::c_int as libc::c_uint) as
                size_t
    }
    osSyncPrintf(b"\xe3\x83\x8f\xe3\x82\xa4\xe3\x83\xa9\xe3\x83\xab\xe5\x86\x8d\xe7\xa2\xba\xe4\xbf\x9d \xe3\x82\xb5\xe3\x82\xa4\xe3\x82\xba\xef\xbc\x9d%u \xe3\x83\x90\xe3\x82\xa4\xe3\x83\x88\n\x00"
                     as *const u8 as *const libc::c_char, size);
    gameArena =
        GameAlloc_MallocDebug(alloc, size as u32_0,
                              b"../game.c\x00" as *const u8 as
                                  *const libc::c_char, 1033 as libc::c_int);
    if !gameArena.is_null() {
        THA_Ct(&mut (*gameState).tha, gameArena, size as u32_0);
        osSyncPrintf(b"\xe3\x83\x8f\xe3\x82\xa4\xe3\x83\xa9\xe3\x83\xab\xe5\x86\x8d\xe7\xa2\xba\xe4\xbf\x9d\xe6\x88\x90\xe5\x8a\x9f\n\x00"
                         as *const u8 as *const libc::c_char);
        // "Successful reacquisition of Hyrule"
    } else {
        THA_Ct(&mut (*gameState).tha, 0 as *mut libc::c_void,
               0 as libc::c_int as u32_0); // "Failure to secure Hyral"
        osSyncPrintf(b"\xe3\x83\x8f\xe3\x82\xa4\xe3\x83\xa9\xe3\x83\xab\xe5\x86\x8d\xe7\xa2\xba\xe4\xbf\x9d\xe5\xa4\xb1\xe6\x95\x97\n\x00"
                         as *const u8 as
                         *const libc::c_char); // "game constructor start"
        SystemArena_Display();
        Fault_AddHungupAndCrash(b"../game.c\x00" as *const u8 as
                                    *const libc::c_char,
                                1044 as libc::c_int as u32_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn GameState_Init(mut gameState: *mut GameState,
                                        mut init: GameStateFunc,
                                        mut gfxCtx: *mut GraphicsContext) {
    let mut startTime: OSTime = 0;
    let mut endTime: OSTime = 0;
    osSyncPrintf(b"game \xe3\x82\xb3\xe3\x83\xb3\xe3\x82\xb9\xe3\x83\x88\xe3\x83\xa9\xe3\x82\xaf\xe3\x82\xbf\xe9\x96\x8b\xe5\xa7\x8b\n\x00"
                     as *const u8 as *const libc::c_char);
    (*gameState).gfxCtx = gfxCtx;
    (*gameState).frames = 0 as libc::c_int as u32_0;
    (*gameState).main = None;
    (*gameState).destroy = None;
    (*gameState).running = 1 as libc::c_int as u32_0;
    startTime = osGetTime();
    (*gameState).size = 0 as libc::c_int as u32_0;
    (*gameState).init = None;
    endTime = osGetTime();
    // "game_set_next_game_null processing time %d us"
    osSyncPrintf(b"game_set_next_game_null \xe5\x87\xa6\xe7\x90\x86\xe6\x99\x82\xe9\x96\x93 %d us\n\x00"
                     as *const u8 as *const libc::c_char,
                 endTime.wrapping_sub(startTime).wrapping_mul((1000000 as
                                                                   libc::c_longlong
                                                                   /
                                                                   15625 as
                                                                       libc::c_longlong)
                                                                  as
                                                                  libc::c_ulonglong).wrapping_div((62500000
                                                                                                       as
                                                                                                       libc::c_longlong
                                                                                                       *
                                                                                                       3
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_longlong
                                                                                                       /
                                                                                                       4
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_longlong
                                                                                                       /
                                                                                                       15625
                                                                                                           as
                                                                                                           libc::c_longlong)
                                                                                                      as
                                                                                                      libc::c_ulonglong));
    startTime = endTime;
    GameAlloc_Init(&mut (*gameState).alloc);
    endTime = osGetTime();
    // "gamealloc_init processing time %d us"
    osSyncPrintf(b"gamealloc_init \xe5\x87\xa6\xe7\x90\x86\xe6\x99\x82\xe9\x96\x93 %d us\n\x00"
                     as *const u8 as *const libc::c_char,
                 endTime.wrapping_sub(startTime).wrapping_mul((1000000 as
                                                                   libc::c_longlong
                                                                   /
                                                                   15625 as
                                                                       libc::c_longlong)
                                                                  as
                                                                  libc::c_ulonglong).wrapping_div((62500000
                                                                                                       as
                                                                                                       libc::c_longlong
                                                                                                       *
                                                                                                       3
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_longlong
                                                                                                       /
                                                                                                       4
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_longlong
                                                                                                       /
                                                                                                       15625
                                                                                                           as
                                                                                                           libc::c_longlong)
                                                                                                      as
                                                                                                      libc::c_ulonglong));
    startTime = endTime;
    GameState_InitArena(gameState, 0x100000 as libc::c_int as size_t);
    (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int * 16 as libc::c_int
                           + 30 as libc::c_int) as usize] =
        3 as libc::c_int as s16;
    init.expect("non-null function pointer")(gameState);
    endTime = osGetTime();
    // "init processing time %d us"
    osSyncPrintf(b"init \xe5\x87\xa6\xe7\x90\x86\xe6\x99\x82\xe9\x96\x93 %d us\n\x00"
                     as *const u8 as *const libc::c_char,
                 endTime.wrapping_sub(startTime).wrapping_mul((1000000 as
                                                                   libc::c_longlong
                                                                   /
                                                                   15625 as
                                                                       libc::c_longlong)
                                                                  as
                                                                  libc::c_ulonglong).wrapping_div((62500000
                                                                                                       as
                                                                                                       libc::c_longlong
                                                                                                       *
                                                                                                       3
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_longlong
                                                                                                       /
                                                                                                       4
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_longlong
                                                                                                       /
                                                                                                       15625
                                                                                                           as
                                                                                                           libc::c_longlong)
                                                                                                      as
                                                                                                      libc::c_ulonglong));
    startTime = endTime;
    LogUtils_CheckNullPointer(b"this->cleanup\x00" as *const u8 as
                                  *const libc::c_char,
                              ::std::mem::transmute::<GameStateFunc,
                                                      *mut libc::c_void>((*gameState).destroy),
                              b"../game.c\x00" as *const u8 as
                                  *const libc::c_char, 1088 as libc::c_int);
    func_800ACE70(&mut D_801664F0);
    func_800AD920(&mut D_80166500);
    VisMono_Init(&mut sMonoColors);
    if (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 48 as libc::c_int) as usize]
           as libc::c_int == 0 as libc::c_int {
        ViMode_Init(&mut sViMode);
    }
    SpeedMeter_Init(&mut D_801664D0);
    func_800AA0B4();
    osSendMesg(&mut (*(*gameState).gfxCtx).queue, 0 as *mut libc::c_void,
               1 as libc::c_int);
    endTime = osGetTime();
    // "Other initialization processing time %d us"
    osSyncPrintf(b"\xe3\x81\x9d\xe3\x81\xae\xe4\xbb\x96\xe5\x88\x9d\xe6\x9c\x9f\xe5\x8c\x96 \xe5\x87\xa6\xe7\x90\x86\xe6\x99\x82\xe9\x96\x93 %d us\n\x00"
                     as *const u8 as *const libc::c_char,
                 endTime.wrapping_sub(startTime).wrapping_mul((1000000 as
                                                                   libc::c_longlong
                                                                   /
                                                                   15625 as
                                                                       libc::c_longlong)
                                                                  as
                                                                  libc::c_ulonglong).wrapping_div((62500000
                                                                                                       as
                                                                                                       libc::c_longlong
                                                                                                       *
                                                                                                       3
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_longlong
                                                                                                       /
                                                                                                       4
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_longlong
                                                                                                       /
                                                                                                       15625
                                                                                                           as
                                                                                                           libc::c_longlong)
                                                                                                      as
                                                                                                      libc::c_ulonglong));
    Fault_AddClient(&mut sGameFaultClient,
                    ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                       -> ()>,
                                            *mut libc::c_void>(Some(GameState_FaultPrint
                                                                        as
                                                                        unsafe extern "C" fn()
                                                                            ->
                                                                                ())),
                    0 as *mut libc::c_void, 0 as *mut libc::c_void);
    osSyncPrintf(b"game \xe3\x82\xb3\xe3\x83\xb3\xe3\x82\xb9\xe3\x83\x88\xe3\x83\xa9\xe3\x82\xaf\xe3\x82\xbf\xe7\xb5\x82\xe4\xba\x86\n\x00"
                     as *const u8 as *const libc::c_char);
    // "game constructor end"
}
#[no_mangle]
pub unsafe extern "C" fn GameState_Destroy(mut gameState: *mut GameState) {
    osSyncPrintf(b"game \xe3\x83\x87\xe3\x82\xb9\xe3\x83\x88\xe3\x83\xa9\xe3\x82\xaf\xe3\x82\xbf\xe9\x96\x8b\xe5\xa7\x8b\n\x00"
                     as *const u8 as
                     *const libc::c_char); // "game destructor start"
    func_800C3C20();
    func_800F3054();
    osRecvMesg(&mut (*(*gameState).gfxCtx).queue, 0 as *mut OSMesg,
               1 as libc::c_int);
    LogUtils_CheckNullPointer(b"this->cleanup\x00" as *const u8 as
                                  *const libc::c_char,
                              ::std::mem::transmute::<GameStateFunc,
                                                      *mut libc::c_void>((*gameState).destroy),
                              b"../game.c\x00" as *const u8 as
                                  *const libc::c_char, 1139 as libc::c_int);
    if (*gameState).destroy.is_some() {
        (*gameState).destroy.expect("non-null function pointer")(gameState);
    }
    func_800AA0F0();
    SpeedMeter_Destroy(&mut D_801664D0);
    func_800ACE90(&mut D_801664F0);
    func_800AD950(&mut D_80166500);
    VisMono_Destroy(&mut sMonoColors);
    if (*gGameInfo).data[(1 as libc::c_int * 6 as libc::c_int *
                              16 as libc::c_int + 48 as libc::c_int) as usize]
           as libc::c_int == 0 as libc::c_int {
        ViMode_Destroy(&mut sViMode);
    }
    THA_Dt(&mut (*gameState).tha);
    GameAlloc_Cleanup(&mut (*gameState).alloc);
    SystemArena_Display();
    Fault_RemoveClient(&mut sGameFaultClient);
    osSyncPrintf(b"game \xe3\x83\x87\xe3\x82\xb9\xe3\x83\x88\xe3\x83\xa9\xe3\x82\xaf\xe3\x82\xbf\xe7\xb5\x82\xe4\xba\x86\n\x00"
                     as *const u8 as *const libc::c_char);
    // "game destructor end"
}
#[no_mangle]
pub unsafe extern "C" fn GameState_GetInit(mut gameState: *mut GameState)
 -> GameStateFunc {
    return (*gameState).init;
}
#[no_mangle]
pub unsafe extern "C" fn GameState_GetSize(mut gameState: *mut GameState)
 -> size_t {
    return (*gameState).size as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn GameState_IsRunning(mut gameState: *mut GameState)
 -> u32_0 {
    return (*gameState).running;
}
#[no_mangle]
pub unsafe extern "C" fn GameState_Alloc(mut gameState: *mut GameState,
                                         mut size: size_t,
                                         mut file: *mut libc::c_char,
                                         mut line: s32) -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    if THA_IsCrash(&mut (*gameState).tha) != 0 {
        osSyncPrintf(b"\xe3\x83\x8f\xe3\x82\xa4\xe3\x83\xa9\xe3\x83\xab\xe3\x81\xaf\xe6\xbb\x85\xe4\xba\xa1\xe3\x81\x97\xe3\x81\xa6\xe3\x81\x84\xe3\x82\x8b\n\x00"
                         as *const u8 as *const libc::c_char);
        ret = 0 as *mut libc::c_void
    } else if (THA_GetSize(&mut (*gameState).tha) as u32_0 as libc::c_ulong) <
                  size {
        // "Hyral on the verge of extinction does not have %d bytes left (%d bytes until extinction)"
        osSyncPrintf(b"\xe6\xbb\x85\xe4\xba\xa1\xe5\xaf\xb8\xe5\x89\x8d\xe3\x81\xae\xe3\x83\x8f\xe3\x82\xa4\xe3\x83\xa9\xe3\x83\xab\xe3\x81\xab\xe3\x81\xaf %d \xe3\x83\x90\xe3\x82\xa4\xe3\x83\x88\xe3\x81\xae\xe4\xbd\x99\xe5\x8a\x9b\xe3\x82\x82\xe3\x81\xaa\xe3\x81\x84\xef\xbc\x88\xe6\xbb\x85\xe4\xba\xa1\xe3\x81\xbe\xe3\x81\xa7\xe3\x81\x82\xe3\x81\xa8 %d \xe3\x83\x90\xe3\x82\xa4\xe3\x83\x88\xef\xbc\x89\n\x00"
                         as *const u8 as *const libc::c_char, size,
                     THA_GetSize(&mut (*gameState).tha)); // "Hyrule has been destroyed"
        ret = 0 as *mut libc::c_void
    } else {
        ret = THA_AllocEndAlign16(&mut (*gameState).tha, size as u32_0);
        if THA_IsCrash(&mut (*gameState).tha) != 0 {
            osSyncPrintf(b"\xe3\x83\x8f\xe3\x82\xa4\xe3\x83\xa9\xe3\x83\xab\xe3\x81\xaf\xe6\xbb\x85\xe4\xba\xa1\xe3\x81\x97\xe3\x81\xa6\xe3\x81\x97\xe3\x81\xbe\xe3\x81\xa3\xe3\x81\x9f\n\x00"
                             as *const u8 as *const libc::c_char);
            ret = 0 as *mut libc::c_void
        }
    }
    if !ret.is_null() {
        osSyncPrintf(b"\x1b[32m\x00" as *const u8 as *const libc::c_char);
        osSyncPrintf(b"game_alloc(%08x) %08x-%08x [%s:%d]\n\x00" as *const u8
                         as *const libc::c_char, size, ret,
                     (ret as u32_0 as libc::c_ulong).wrapping_add(size), file,
                     line);
        osSyncPrintf(b"\x1b[m\x00" as *const u8 as *const libc::c_char);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn GameState_AllocEndAlign16(mut gameState:
                                                       *mut GameState,
                                                   mut size: size_t)
 -> *mut libc::c_void {
    return THA_AllocEndAlign16(&mut (*gameState).tha, size as u32_0);
}
#[no_mangle]
pub unsafe extern "C" fn GameState_GetArenaSize(mut gameState: *mut GameState)
 -> s32 {
    return THA_GetSize(&mut (*gameState).tha);
}
