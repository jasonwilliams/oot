#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute,
           ptr_wrapping_offset_from, register_tool)]
extern "C" {
    #[no_mangle]
    fn sqrtf(f: f32_0) -> f32_0;
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn osGetTime() -> OSTime;
    #[no_mangle]
    fn AudioLoad_Init(heap: *mut libc::c_void, heapSize: u32_0);
    #[no_mangle]
    fn Audio_QueueCmdF32(arg0: u32_0, arg1: f32_0);
    #[no_mangle]
    fn Audio_QueueCmdS32(arg0: u32_0, arg1: s32);
    #[no_mangle]
    fn Audio_QueueCmdS8(arg0: u32_0, arg1: s8);
    #[no_mangle]
    fn Audio_QueueCmdU16(arg0: u32_0, arg1: u16_0);
    #[no_mangle]
    fn Audio_ScheduleProcessCmds() -> s32;
    #[no_mangle]
    fn func_800E5E84(arg0: s32, arg1: *mut u32_0) -> *mut u8_0;
    #[no_mangle]
    fn Audio_PreNMIInternal();
    #[no_mangle]
    fn func_800E6680() -> s32;
    #[no_mangle]
    fn Audio_NextRandom() -> u32_0;
    #[no_mangle]
    fn Audio_QueueSeqCmdMute(channelIdx: u8_0);
    #[no_mangle]
    static mut gPadMgr: PadMgr;
    #[no_mangle]
    fn Audio_ClearBGMMute(channelIdx: u8_0);
    #[no_mangle]
    fn Audio_SetSoundBanksMute(muteMask: u16_0);
    #[no_mangle]
    fn Audio_StopSfxById(sfxId: u32_0);
    #[no_mangle]
    static mut D_801333E8: s8;
    #[no_mangle]
    static mut D_801333D4: Vec3f;
    #[no_mangle]
    fn Audio_PlaySoundGeneral(sfxId: u16_0, pos: *mut Vec3f, token: u8_0,
                              freqScale: *mut f32_0, a4: *mut f32_0,
                              reverbAdd: *mut s8);
    #[no_mangle]
    static mut gBendPitchTwoSemitonesFrequencies: [f32_0; 256];
    #[no_mangle]
    fn Audio_QueueSeqCmd(bgmID: u32_0);
    #[no_mangle]
    fn GfxPrint_Printf(this: *mut GfxPrint, fmt: *const libc::c_char, _: ...)
     -> s32;
    #[no_mangle]
    fn GfxPrint_SetPos(this: *mut GfxPrint, x: s32, y: s32);
    #[no_mangle]
    static mut gAudioSpecId: u8_0;
    #[no_mangle]
    fn GfxPrint_SetColor(this: *mut GfxPrint, r: u32_0, g: u32_0, b: u32_0,
                         a: u32_0);
    #[no_mangle]
    static mut gSoundParams: [*mut SoundParams; 7];
    #[no_mangle]
    static mut gAudioContext: AudioContext;
    #[no_mangle]
    static D_8014A6C4: AudioContextInitSizes;
    #[no_mangle]
    static mut gAudioSfxSwapMode: [u8_0; 10];
    #[no_mangle]
    static mut gAudioSfxSwapTarget: [u16_0; 10];
    #[no_mangle]
    static mut gAudioSfxSwapSource: [u16_0; 10];
    #[no_mangle]
    static mut gAudioSfxSwapOff: u8_0;
    #[no_mangle]
    static mut gActiveSounds: [[ActiveSound; 3]; 7];
    #[no_mangle]
    static mut gSoundBanks: [*mut SoundBankEntry; 7];
    #[no_mangle]
    static mut gSfxChannelLayout: u8_0;
    #[no_mangle]
    static mut gAudioSpecs: [AudioSpec; 18];
    #[no_mangle]
    static mut D_8013340C: u8_0;
    #[no_mangle]
    fn Audio_StopSfxByBank(bankId: u8_0);
    #[no_mangle]
    static mut D_801333E0: f32_0;
    #[no_mangle]
    fn func_800FA0B4(a0: u8_0) -> u16_0;
    #[no_mangle]
    fn Audio_SetVolScale(playerIdx: u8_0, scaleIdx: u8_0, targetVol: u8_0,
                         volFadeTimer: u8_0);
    #[no_mangle]
    static mut D_8016E750: [unk_D_8016E750; 4];
    #[no_mangle]
    static mut D_801333F0: u8_0;
    #[no_mangle]
    static mut D_80133408: u8_0;
    #[no_mangle]
    static mut gSoundBankMuted: [u8_0; 0];
    #[no_mangle]
    fn func_800FA3DC();
    #[no_mangle]
    fn func_800F8F88();
    #[no_mangle]
    fn Audio_ProcessSeqCmds();
    #[no_mangle]
    fn Audio_ProcessSoundRequests();
    #[no_mangle]
    fn func_800FAD34() -> u8_0;
    #[no_mangle]
    fn Audio_IsSfxPlaying(sfxId: u32_0) -> u8_0;
    #[no_mangle]
    static mut gNoteFrequencies: [f32_0; 0];
    #[no_mangle]
    fn func_800F9474(_: u8_0, _: u16_0);
    #[no_mangle]
    fn func_800FA11C(arg0: u32_0, arg1: u32_0) -> s32;
    #[no_mangle]
    fn func_800F9280(playerIdx: u8_0, seqId: u8_0, arg2: u8_0,
                     fadeTimer: u16_0);
    #[no_mangle]
    fn Audio_ResetSounds();
    #[no_mangle]
    fn func_800FAEB4();
    #[no_mangle]
    fn func_800FADF8();
    #[no_mangle]
    static mut D_80133418: u8_0;
    #[no_mangle]
    fn PadMgr_RequestPadData(padmgr: *mut PadMgr, inputs: *mut Input,
                             mode: s32);
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
pub struct __OSBlockInfo {
    pub errStatus: u32_0,
    pub dramAddr: *mut libc::c_void,
    pub C2Addr: *mut libc::c_void,
    pub sectorSize: u32_0,
    pub C1ErrNum: u32_0,
    pub C1ErrSector: [u32_0; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __OSTranxInfo {
    pub cmdType: u32_0,
    pub transferMode: u16_0,
    pub blockNum: u16_0,
    pub sectorNum: s32,
    pub devAddr: u32_0,
    pub bmCtlShadow: u32_0,
    pub seqCtlShadow: u32_0,
    pub block: [__OSBlockInfo; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSPiHandle {
    pub next: *mut OSPiHandle,
    pub type_0: u8_0,
    pub latency: u8_0,
    pub pageSize: u8_0,
    pub relDuration: u8_0,
    pub pulse: u8_0,
    pub domain: u8_0,
    pub baseAddress: u32_0,
    pub speed: u32_0,
    pub transferInfo: __OSTranxInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSIoMesgHdr {
    pub type_0: u16_0,
    pub pri: u8_0,
    pub status: u8_0,
    pub retQueue: *mut OSMesgQueue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OSIoMesg {
    pub hdr: OSIoMesgHdr,
    pub dramAddr: *mut libc::c_void,
    pub devAddr: u32_0,
    pub size: size_t,
    pub piHandle: *mut OSPiHandle,
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Aadpcm {
    #[bitfield(name = "cmd", ty = "u32_0", bits = "0..=7")]
    #[bitfield(name = "flags", ty = "u32_0", bits = "8..=15")]
    #[bitfield(name = "gain", ty = "u32_0", bits = "16..=31")]
    pub cmd_flags_gain: [u8; 4],
    pub addr: u32_0,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Apolef {
    #[bitfield(name = "cmd", ty = "u32_0", bits = "0..=7")]
    #[bitfield(name = "flags", ty = "u32_0", bits = "8..=15")]
    #[bitfield(name = "gain", ty = "u32_0", bits = "16..=31")]
    pub cmd_flags_gain: [u8; 4],
    pub addr: u32_0,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Aenvelope {
    #[bitfield(name = "cmd", ty = "u32_0", bits = "0..=7")]
    #[bitfield(name = "flags", ty = "u32_0", bits = "8..=15")]
    #[bitfield(name = "pad1", ty = "u32_0", bits = "16..=31")]
    pub cmd_flags_pad1: [u8; 4],
    pub addr: u32_0,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Aclearbuff {
    #[bitfield(name = "cmd", ty = "u32_0", bits = "0..=7")]
    #[bitfield(name = "pad1", ty = "u32_0", bits = "8..=15")]
    #[bitfield(name = "dmem", ty = "u32_0", bits = "16..=31")]
    #[bitfield(name = "pad2", ty = "u32_0", bits = "32..=47")]
    #[bitfield(name = "count", ty = "u32_0", bits = "48..=63")]
    pub cmd_pad1_dmem_pad2_count: [u8; 8],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Ainterleave {
    #[bitfield(name = "cmd", ty = "u32_0", bits = "0..=7")]
    #[bitfield(name = "pad1", ty = "u32_0", bits = "8..=15")]
    #[bitfield(name = "pad2", ty = "u32_0", bits = "16..=31")]
    #[bitfield(name = "inL", ty = "u32_0", bits = "32..=47")]
    #[bitfield(name = "inR", ty = "u32_0", bits = "48..=63")]
    pub cmd_pad1_pad2_inL_inR: [u8; 8],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Aloadbuff {
    #[bitfield(name = "cmd", ty = "u32_0", bits = "0..=7")]
    #[bitfield(name = "pad1", ty = "u32_0", bits = "8..=31")]
    pub cmd_pad1: [u8; 4],
    pub addr: u32_0,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Aenvmixer {
    #[bitfield(name = "cmd", ty = "u32_0", bits = "0..=7")]
    #[bitfield(name = "flags", ty = "u32_0", bits = "8..=15")]
    #[bitfield(name = "pad1", ty = "u32_0", bits = "16..=31")]
    pub cmd_flags_pad1: [u8; 4],
    pub addr: u32_0,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Amixer {
    #[bitfield(name = "cmd", ty = "u32_0", bits = "0..=7")]
    #[bitfield(name = "flags", ty = "u32_0", bits = "8..=15")]
    #[bitfield(name = "gain", ty = "u32_0", bits = "16..=31")]
    #[bitfield(name = "dmemi", ty = "u32_0", bits = "32..=47")]
    #[bitfield(name = "dmemo", ty = "u32_0", bits = "48..=63")]
    pub cmd_flags_gain_dmemi_dmemo: [u8; 8],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Aresample {
    #[bitfield(name = "cmd", ty = "u32_0", bits = "0..=7")]
    #[bitfield(name = "flags", ty = "u32_0", bits = "8..=15")]
    #[bitfield(name = "pitch", ty = "u32_0", bits = "16..=31")]
    pub cmd_flags_pitch: [u8; 4],
    pub addr: u32_0,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Areverb {
    #[bitfield(name = "cmd", ty = "u32_0", bits = "0..=7")]
    #[bitfield(name = "flags", ty = "u32_0", bits = "8..=15")]
    #[bitfield(name = "pad1", ty = "u32_0", bits = "16..=31")]
    pub cmd_flags_pad1: [u8; 4],
    pub addr: u32_0,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Asavebuff {
    #[bitfield(name = "cmd", ty = "u32_0", bits = "0..=7")]
    #[bitfield(name = "pad1", ty = "u32_0", bits = "8..=31")]
    pub cmd_pad1: [u8; 4],
    pub addr: u32_0,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Asegment {
    #[bitfield(name = "cmd", ty = "u32_0", bits = "0..=7")]
    #[bitfield(name = "pad1", ty = "u32_0", bits = "8..=31")]
    #[bitfield(name = "pad2", ty = "u32_0", bits = "32..=33")]
    #[bitfield(name = "number", ty = "u32_0", bits = "34..=37")]
    #[bitfield(name = "base", ty = "u32_0", bits = "38..=61")]
    pub cmd_pad1_pad2_number_base: [u8; 8],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Asetbuff {
    #[bitfield(name = "cmd", ty = "u32_0", bits = "0..=7")]
    #[bitfield(name = "flags", ty = "u32_0", bits = "8..=15")]
    #[bitfield(name = "dmemin", ty = "u32_0", bits = "16..=31")]
    #[bitfield(name = "dmemout", ty = "u32_0", bits = "32..=47")]
    #[bitfield(name = "count", ty = "u32_0", bits = "48..=63")]
    pub cmd_flags_dmemin_dmemout_count: [u8; 8],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Asetvol {
    #[bitfield(name = "cmd", ty = "u32_0", bits = "0..=7")]
    #[bitfield(name = "flags", ty = "u32_0", bits = "8..=15")]
    #[bitfield(name = "vol", ty = "u32_0", bits = "16..=31")]
    #[bitfield(name = "voltgt", ty = "u32_0", bits = "32..=47")]
    #[bitfield(name = "volrate", ty = "u32_0", bits = "48..=63")]
    pub cmd_flags_vol_voltgt_volrate: [u8; 8],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Admemmove {
    #[bitfield(name = "cmd", ty = "u32_0", bits = "0..=7")]
    #[bitfield(name = "pad1", ty = "u32_0", bits = "8..=15")]
    #[bitfield(name = "dmemin", ty = "u32_0", bits = "16..=31")]
    #[bitfield(name = "dmemout", ty = "u32_0", bits = "32..=47")]
    #[bitfield(name = "count", ty = "u32_0", bits = "48..=63")]
    pub cmd_pad1_dmemin_dmemout_count: [u8; 8],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Aloadadpcm {
    #[bitfield(name = "cmd", ty = "u32_0", bits = "0..=7")]
    #[bitfield(name = "pad1", ty = "u32_0", bits = "8..=15")]
    #[bitfield(name = "count", ty = "u32_0", bits = "16..=31")]
    pub cmd_pad1_count: [u8; 4],
    pub addr: u32_0,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Asetloop {
    #[bitfield(name = "cmd", ty = "u32_0", bits = "0..=7")]
    #[bitfield(name = "pad1", ty = "u32_0", bits = "8..=15")]
    #[bitfield(name = "pad2", ty = "u32_0", bits = "16..=31")]
    pub cmd_pad1_pad2: [u8; 4],
    pub addr: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Awords {
    pub w0: u32_0,
    pub w1: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Acmd {
    pub words: Awords,
    pub adpcm: Aadpcm,
    pub polef: Apolef,
    pub clearbuff: Aclearbuff,
    pub envelope: Aenvelope,
    pub interleave: Ainterleave,
    pub loadbuff: Aloadbuff,
    pub envmixer: Aenvmixer,
    pub resample: Aresample,
    pub reverb: Areverb,
    pub savebuff: Asavebuff,
    pub segment: Asegment,
    pub setbuff: Asetbuff,
    pub setvol: Asetvol,
    pub dmemmove: Admemmove,
    pub loadadpcm: Aloadadpcm,
    pub mixer: Amixer,
    pub setloop: Asetloop,
    pub force_union_align: libc::c_longlong,
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
pub struct Vec3f {
    pub x: f32_0,
    pub y: f32_0,
    pub z: f32_0,
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
pub struct OcarinaStaff {
    pub noteIdx: u8_0,
    pub state: u8_0,
    pub pos: u8_0,
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const SAMPLE_TABLE: C2RustUnnamed_1 = 2;
pub const FONT_TABLE: C2RustUnnamed_1 = 1;
pub const SEQUENCE_TABLE: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Note {
    pub listItem: AudioListItem,
    pub synthesisState: NoteSynthesisState,
    pub playbackState: NotePlaybackState,
    pub portamento: Portamento,
    pub vibratoState: VibratoState,
    pub unk_B8: [libc::c_char; 4],
    pub unk_BC: u32_0,
    pub noteSubEu: NoteSubEu,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NoteSubEu {
    pub bitField0: C2RustUnnamed_4,
    pub bitField1: C2RustUnnamed_3,
    pub unk_2: u8_0,
    pub headsetPanRight: u8_0,
    pub headsetPanLeft: u8_0,
    pub reverbVol: u8_0,
    pub unk_06: u8_0,
    pub unk_07: u8_0,
    pub targetVolLeft: u16_0,
    pub targetVolRight: u16_0,
    pub resamplingRateFixedPoint: u16_0,
    pub unk_0E: u16_0,
    pub sound: C2RustUnnamed_2,
    pub filter: *mut s16,
    pub pad_18: [libc::c_char; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub soundFontSound: *mut SoundFontSound,
    pub samples: *mut s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SoundFontSound {
    pub sample: *mut SoundFontSample,
    pub tuning: f32_0,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct SoundFontSample {
    #[bitfield(name = "codec", ty = "u32_0", bits = "0..=3")]
    #[bitfield(name = "medium", ty = "u32_0", bits = "4..=5")]
    #[bitfield(name = "unk_bit26", ty = "u32_0", bits = "6..=6")]
    #[bitfield(name = "unk_bit25", ty = "u32_0", bits = "7..=7")]
    #[bitfield(name = "size", ty = "u32_0", bits = "8..=31")]
    pub codec_medium_unk_bit26_unk_bit25_size: [u8; 4],
    pub sampleAddr: *mut u8_0,
    pub loop_0: *mut AdpcmLoop,
    pub book: *mut AdpcmBook,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AdpcmBook {
    pub order: s32,
    pub npredictors: s32,
    pub book: [s16; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AdpcmLoop {
    pub start: u32_0,
    pub end: u32_0,
    pub count: u32_0,
    pub unk_0C: [libc::c_char; 4],
    pub state: [s16; 16],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    #[bitfield(name = "reverbIndex", ty = "u8_0", bits = "0..=2")]
    #[bitfield(name = "bookOffset", ty = "u8_0", bits = "3..=4")]
    #[bitfield(name = "isSyntheticWave", ty = "u8_0", bits = "5..=5")]
    #[bitfield(name = "hasTwoParts", ty = "u8_0", bits = "6..=6")]
    #[bitfield(name = "usesHeadsetPanEffects2", ty = "u8_0", bits = "7..=7")]
    pub reverbIndex_bookOffset_isSyntheticWave_hasTwoParts_usesHeadsetPanEffects2: [u8; 1],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    #[bitfield(name = "enabled", ty = "u8_0", bits = "0..=0")]
    #[bitfield(name = "needsInit", ty = "u8_0", bits = "1..=1")]
    #[bitfield(name = "finished", ty = "u8_0", bits = "2..=2")]
    #[bitfield(name = "unused", ty = "u8_0", bits = "3..=3")]
    #[bitfield(name = "stereoStrongRight", ty = "u8_0", bits = "4..=4")]
    #[bitfield(name = "stereoStrongLeft", ty = "u8_0", bits = "5..=5")]
    #[bitfield(name = "stereoHeadsetEffects", ty = "u8_0", bits = "6..=6")]
    #[bitfield(name = "usesHeadsetPanEffects", ty = "u8_0", bits = "7..=7")]
    pub enabled_needsInit_finished_unused_stereoStrongRight_stereoStrongLeft_stereoHeadsetEffects_usesHeadsetPanEffects: [u8; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VibratoState {
    pub channel: *mut SequenceChannel,
    pub time: u32_0,
    pub curve: *mut s16,
    pub extent: f32_0,
    pub rate: f32_0,
    pub active: u8_0,
    pub rateChangeTimer: u16_0,
    pub extentChangeTimer: u16_0,
    pub delay: u16_0,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct SequenceChannel {
    #[bitfield(name = "enabled", ty = "u8_0", bits = "0..=0")]
    #[bitfield(name = "finished", ty = "u8_0", bits = "1..=1")]
    #[bitfield(name = "stopScript", ty = "u8_0", bits = "2..=2")]
    #[bitfield(name = "stopSomething2", ty = "u8_0", bits = "3..=3")]
    #[bitfield(name = "hasInstrument", ty = "u8_0", bits = "4..=4")]
    #[bitfield(name = "stereoHeadsetEffects", ty = "u8_0", bits = "5..=5")]
    #[bitfield(name = "largeNotes", ty = "u8_0", bits = "6..=6")]
    #[bitfield(name = "unused", ty = "u8_0", bits = "7..=7")]
    pub enabled_finished_stopScript_stopSomething2_hasInstrument_stereoHeadsetEffects_largeNotes_unused: [u8; 1],
    pub changes: C2RustUnnamed_6,
    pub noteAllocPolicy: u8_0,
    pub muteBehavior: u8_0,
    pub reverb: u8_0,
    pub notePriority: u8_0,
    pub someOtherPriority: u8_0,
    pub fontId: u8_0,
    pub reverbIndex: u8_0,
    pub bookOffset: u8_0,
    pub newPan: u8_0,
    pub panChannelWeight: u8_0,
    pub unk_0C: u8_0,
    pub velocityRandomVariance: u8_0,
    pub gateTimeRandomVariance: u8_0,
    pub unk_0F: u8_0,
    pub vibratoRateStart: u16_0,
    pub vibratoExtentStart: u16_0,
    pub vibratoRateTarget: u16_0,
    pub vibratoExtentTarget: u16_0,
    pub vibratoRateChangeDelay: u16_0,
    pub vibratoExtentChangeDelay: u16_0,
    pub vibratoDelay: u16_0,
    pub delay: u16_0,
    pub unk_20: u16_0,
    pub unk_22: u16_0,
    pub instOrWave: s16,
    pub transposition: s16,
    pub volumeScale: f32_0,
    pub volume: f32_0,
    pub pan: s32,
    pub appliedVolume: f32_0,
    pub freqScale: f32_0,
    pub dynTable: *mut [[u8_0; 2]; 0],
    pub noteUnused: *mut Note,
    pub layerUnused: *mut SequenceLayer,
    pub instrument: *mut Instrument,
    pub seqPlayer: *mut SequencePlayer,
    pub layers: [*mut SequenceLayer; 4],
    pub scriptState: SeqScriptState,
    pub adsr: AdsrSettings,
    pub notePool: NotePool,
    pub soundScriptIO: [s8; 8],
    pub filter: *mut s16,
    pub stereo: Stereo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Stereo {
    pub s: StereoData,
    pub asByte: u8_0,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct StereoData {
    #[bitfield(name = "unused", ty = "u8_0", bits = "0..=1")]
    #[bitfield(name = "bit2", ty = "u8_0", bits = "2..=3")]
    #[bitfield(name = "strongRight", ty = "u8_0", bits = "4..=4")]
    #[bitfield(name = "strongLeft", ty = "u8_0", bits = "5..=5")]
    #[bitfield(name = "stereoHeadsetEffects", ty = "u8_0", bits = "6..=6")]
    #[bitfield(name = "usesHeadsetPanEffects", ty = "u8_0", bits = "7..=7")]
    pub unused_bit2_strongRight_strongLeft_stereoHeadsetEffects_usesHeadsetPanEffects: [u8; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NotePool {
    pub disabled: AudioListItem,
    pub decaying: AudioListItem,
    pub releasing: AudioListItem,
    pub active: AudioListItem,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioListItem {
    pub prev: *mut AudioListItem,
    pub next: *mut AudioListItem,
    pub u: C2RustUnnamed_5,
    pub pool: *mut NotePool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub value: *mut libc::c_void,
    pub count: s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AdsrSettings {
    pub releaseRate: u8_0,
    pub sustain: u8_0,
    pub envelope: *mut AdsrEnvelope,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AdsrEnvelope {
    pub delay: s16,
    pub arg: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SeqScriptState {
    pub pc: *mut u8_0,
    pub stack: [*mut u8_0; 4],
    pub remLoopIters: [u8_0; 4],
    pub depth: u8_0,
    pub value: s8,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct SequenceLayer {
    #[bitfield(name = "enabled", ty = "u8_0", bits = "0..=0")]
    #[bitfield(name = "finished", ty = "u8_0", bits = "1..=1")]
    #[bitfield(name = "stopSomething", ty = "u8_0", bits = "2..=2")]
    #[bitfield(name = "continuousNotes", ty = "u8_0", bits = "3..=3")]
    #[bitfield(name = "bit3", ty = "u8_0", bits = "4..=4")]
    #[bitfield(name = "ignoreDrumPan", ty = "u8_0", bits = "5..=5")]
    #[bitfield(name = "bit1", ty = "u8_0", bits = "6..=6")]
    #[bitfield(name = "notePropertiesNeedInit", ty = "u8_0", bits = "7..=7")]
    pub enabled_finished_stopSomething_continuousNotes_bit3_ignoreDrumPan_bit1_notePropertiesNeedInit: [u8; 1],
    pub stereo: Stereo,
    pub instOrWave: u8_0,
    pub gateTime: u8_0,
    pub semitone: u8_0,
    pub portamentoTargetNote: u8_0,
    pub pan: u8_0,
    pub notePan: u8_0,
    pub delay: s16,
    pub gateDelay: s16,
    pub delay2: s16,
    pub portamentoTime: u16_0,
    pub transposition: s16,
    pub shortNoteDefaultDelay: s16,
    pub lastDelay: s16,
    pub adsr: AdsrSettings,
    pub portamento: Portamento,
    pub note: *mut Note,
    pub freqScale: f32_0,
    pub unk_34: f32_0,
    pub velocitySquare2: f32_0,
    pub velocitySquare: f32_0,
    pub noteVelocity: f32_0,
    pub noteFreqScale: f32_0,
    pub instrument: *mut Instrument,
    pub sound: *mut SoundFontSound,
    pub channel: *mut SequenceChannel,
    pub scriptState: SeqScriptState,
    pub listItem: AudioListItem,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Instrument {
    pub loaded: u8_0,
    pub normalRangeLo: u8_0,
    pub normalRangeHi: u8_0,
    pub releaseRate: u8_0,
    pub envelope: *mut AdsrEnvelope,
    pub lowNotesSound: SoundFontSound,
    pub normalNotesSound: SoundFontSound,
    pub highNotesSound: SoundFontSound,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Portamento {
    pub mode: u8_0,
    pub cur: u16_0,
    pub speed: u16_0,
    pub extent: f32_0,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct SequencePlayer {
    #[bitfield(name = "enabled", ty = "u8_0", bits = "0..=0")]
    #[bitfield(name = "finished", ty = "u8_0", bits = "1..=1")]
    #[bitfield(name = "muted", ty = "u8_0", bits = "2..=2")]
    #[bitfield(name = "seqDmaInProgress", ty = "u8_0", bits = "3..=3")]
    #[bitfield(name = "fontDmaInProgress", ty = "u8_0", bits = "4..=4")]
    #[bitfield(name = "recalculateVolume", ty = "u8_0", bits = "5..=5")]
    #[bitfield(name = "stopScript", ty = "u8_0", bits = "6..=6")]
    #[bitfield(name = "unk_0b1", ty = "u8_0", bits = "7..=7")]
    pub enabled_finished_muted_seqDmaInProgress_fontDmaInProgress_recalculateVolume_stopScript_unk_0b1: [u8; 1],
    pub state: u8_0,
    pub noteAllocPolicy: u8_0,
    pub muteBehavior: u8_0,
    pub seqId: u8_0,
    pub defaultFont: u8_0,
    pub unk_06: [u8_0; 1],
    pub playerIdx: s8,
    pub tempo: u16_0,
    pub tempoAcc: u16_0,
    pub unk_0C: u16_0,
    pub transposition: s16,
    pub delay: u16_0,
    pub fadeTimer: u16_0,
    pub fadeTimerUnkEu: u16_0,
    pub seqData: *mut u8_0,
    pub fadeVolume: f32_0,
    pub fadeVelocity: f32_0,
    pub volume: f32_0,
    pub muteVolumeScale: f32_0,
    pub fadeVolumeScale: f32_0,
    pub appliedFadeVolume: f32_0,
    pub unk_34: f32_0,
    pub channels: [*mut SequenceChannel; 16],
    pub scriptState: SeqScriptState,
    pub shortNoteVelocityTable: *mut u8_0,
    pub shortNoteGateTimeTable: *mut u8_0,
    pub notePool: NotePool,
    pub skipTicks: s32,
    pub scriptCounter: u32_0,
    pub unk_E4: [libc::c_char; 116],
    pub soundScriptIO: [s8; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub s: C2RustUnnamed_7,
    pub asByte: u8_0,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    #[bitfield(name = "freqScale", ty = "u8_0", bits = "0..=0")]
    #[bitfield(name = "volume", ty = "u8_0", bits = "1..=1")]
    #[bitfield(name = "pan", ty = "u8_0", bits = "2..=2")]
    pub freqScale_volume_pan: [u8; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NotePlaybackState {
    pub priority: u8_0,
    pub waveId: u8_0,
    pub sampleCountIndex: u8_0,
    pub fontId: u8_0,
    pub unk_04: u8_0,
    pub stereoHeadsetEffects: u8_0,
    pub adsrVolScaleUnused: s16,
    pub portamentoFreqScale: f32_0,
    pub vibratoFreqScale: f32_0,
    pub prevParentLayer: *mut SequenceLayer,
    pub parentLayer: *mut SequenceLayer,
    pub wantedParentLayer: *mut SequenceLayer,
    pub attributes: NoteAttributes,
    pub adsr: AdsrState,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AdsrState {
    pub action: C2RustUnnamed_8,
    pub envIndex: u8_0,
    pub delay: s16,
    pub sustain: f32_0,
    pub velocity: f32_0,
    pub fadeOutVel: f32_0,
    pub current: f32_0,
    pub target: f32_0,
    pub unk_18: [libc::c_char; 4],
    pub envelope: *mut AdsrEnvelope,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub s: A,
    pub asByte: u8_0,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct A {
    #[bitfield(name = "unk_0b80", ty = "u8_0", bits = "0..=0")]
    #[bitfield(name = "hang", ty = "u8_0", bits = "1..=1")]
    #[bitfield(name = "decay", ty = "u8_0", bits = "2..=2")]
    #[bitfield(name = "release", ty = "u8_0", bits = "3..=3")]
    #[bitfield(name = "state", ty = "u8_0", bits = "4..=7")]
    pub unk_0b80_hang_decay_release_state: [u8; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NoteAttributes {
    pub reverb: u8_0,
    pub unk_1: u8_0,
    pub pan: u8_0,
    pub stereo: Stereo,
    pub unk_4: u8_0,
    pub unk_6: u16_0,
    pub freqScale: f32_0,
    pub velocity: f32_0,
    pub filter: *mut s16,
    pub filterBuf: [s16; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NoteSynthesisState {
    pub restart: u8_0,
    pub sampleDmaIndex: u8_0,
    pub prevHeadsetPanRight: u8_0,
    pub prevHeadsetPanLeft: u8_0,
    pub reverbVol: u8_0,
    pub numParts: u8_0,
    pub samplePosFrac: u16_0,
    pub samplePosInt: s32,
    pub synthesisBuffers: *mut NoteSynthesisBuffers,
    pub curVolLeft: s16,
    pub curVolRight: s16,
    pub unk_14: u16_0,
    pub unk_16: u16_0,
    pub unk_18: u16_0,
    pub unk_1A: u8_0,
    pub unk_1C: u16_0,
    pub unk_1E: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NoteSynthesisBuffers {
    pub adpcmdecState: [s16; 16],
    pub finalResampleState: [s16; 16],
    pub mixEnvelopeState: [s16; 40],
    pub panResampleState: [s16; 16],
    pub panSamplesBuffer: [s16; 32],
    pub dummyResampleState: [s16; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ReverbRingBufferItem {
    pub numSamplesAfterDownsampling: s16,
    pub chunkLen: s16,
    pub toDownsampleLeft: *mut s16,
    pub toDownsampleRight: *mut s16,
    pub startPos: s32,
    pub lengthA: s16,
    pub lengthB: s16,
    pub unk_14: u16_0,
    pub unk_16: u16_0,
    pub unk_18: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SynthesisReverb {
    pub resampleFlags: u8_0,
    pub useReverb: u8_0,
    pub framesToIgnore: u8_0,
    pub curFrame: u8_0,
    pub downsampleRate: u8_0,
    pub unk_05: s8,
    pub windowSize: u16_0,
    pub unk_08: s16,
    pub unk_0A: s16,
    pub unk_0C: u16_0,
    pub unk_0E: u16_0,
    pub leakRtl: s16,
    pub leakLtr: s16,
    pub unk_14: u16_0,
    pub unk_16: s16,
    pub unk_18: u8_0,
    pub unk_19: u8_0,
    pub unk_1A: u8_0,
    pub unk_1B: u8_0,
    pub nextRingBufPos: s32,
    pub unk_20: s32,
    pub bufSizePerChan: s32,
    pub leftRingBuf: *mut s16,
    pub rightRingBuf: *mut s16,
    pub unk_30: *mut libc::c_void,
    pub unk_34: *mut libc::c_void,
    pub unk_38: *mut libc::c_void,
    pub unk_3C: *mut libc::c_void,
    pub items: [[ReverbRingBufferItem; 5]; 2],
    pub items2: [[ReverbRingBufferItem; 5]; 2],
    pub filterLeft: *mut s16,
    pub filterRight: *mut s16,
    pub filterLeftState: *mut s16,
    pub filterRightState: *mut s16,
    pub sound: SoundFontSound,
    pub sample: SoundFontSample,
    pub loop_0: AdpcmLoop,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Drum {
    pub releaseRate: u8_0,
    pub pan: u8_0,
    pub loaded: u8_0,
    pub sound: SoundFontSound,
    pub envelope: *mut AdsrEnvelope,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SoundFont {
    pub numInstruments: u8_0,
    pub numDrums: u8_0,
    pub sampleBankId1: u8_0,
    pub sampleBankId2: u8_0,
    pub numSfx: u16_0,
    pub instruments: *mut *mut Instrument,
    pub drums: *mut *mut Drum,
    pub soundEffects: *mut SoundFontSound,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ReverbSettings {
    pub downsampleRate: u8_0,
    pub windowSize: u16_0,
    pub unk_4: u16_0,
    pub unk_6: u16_0,
    pub unk_8: u16_0,
    pub unk_A: u16_0,
    pub leakRtl: u16_0,
    pub leakLtr: u16_0,
    pub unk_10: s8,
    pub unk_12: u16_0,
    pub lowPassFilterCutoffLeft: s16,
    pub lowPassFilterCutoffRight: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioSpec {
    pub frequency: u32_0,
    pub unk_04: u8_0,
    pub numNotes: u8_0,
    pub numSequencePlayers: u8_0,
    pub unk_07: u8_0,
    pub unk_08: u8_0,
    pub numReverbs: u8_0,
    pub reverbSettings: *mut ReverbSettings,
    pub sampleDmaBufSize1: u16_0,
    pub sampleDmaBufSize2: u16_0,
    pub unk_14: u16_0,
    pub persistentSeqMem: u32_0,
    pub persistentFontMem: u32_0,
    pub persistentSampleMem: u32_0,
    pub temporarySeqMem: u32_0,
    pub temporaryFontMem: u32_0,
    pub temporarySampleMem: u32_0,
    pub persistentSampleCacheMem: s32,
    pub temporarySampleCacheMem: s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioBufferParameters {
    pub specUnk4: s16,
    pub frequency: u16_0,
    pub aiFrequency: u16_0,
    pub samplesPerFrameTarget: s16,
    pub maxAiBufferLength: s16,
    pub minAiBufferLength: s16,
    pub updatesPerFrame: s16,
    pub samplesPerUpdate: s16,
    pub samplesPerUpdateMax: s16,
    pub samplesPerUpdateMin: s16,
    pub numSequencePlayers: s16,
    pub resampleRate: f32_0,
    pub updatesPerFrameInv: f32_0,
    pub unkUpdatesPerFrameScaled: f32_0,
    pub unk_24: f32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioAllocPool {
    pub start: *mut u8_0,
    pub cur: *mut u8_0,
    pub size: s32,
    pub count: s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioCacheEntry {
    pub ptr: *mut u8_0,
    pub size: u32_0,
    pub tableType: s16,
    pub id: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SampleCacheEntry {
    pub inUse: s8,
    pub origMedium: s8,
    pub sampleBankId: s8,
    pub unk_03: [libc::c_char; 5],
    pub allocatedAddr: *mut u8_0,
    pub sampleAddr: *mut libc::c_void,
    pub size: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioSampleCache {
    pub pool: AudioAllocPool,
    pub entries: [SampleCacheEntry; 32],
    pub size: s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioPersistentCache {
    pub numEntries: u32_0,
    pub pool: AudioAllocPool,
    pub entries: [AudioCacheEntry; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioTemporaryCache {
    pub nextSide: u32_0,
    pub pool: AudioAllocPool,
    pub entries: [AudioCacheEntry; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioCache {
    pub persistent: AudioPersistentCache,
    pub temporary: AudioTemporaryCache,
    pub unk_100: [u8_0; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioPoolSplit2 {
    pub wantPersistent: u32_0,
    pub wantTemporary: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioPoolSplit3 {
    pub wantSeq: u32_0,
    pub wantFont: u32_0,
    pub wantSample: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioPoolSplit4 {
    pub wantSeq: u32_0,
    pub wantFont: u32_0,
    pub wantSample: u32_0,
    pub wantCustom: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioPreloadReq {
    pub endAndMediumKey: u32_0,
    pub sample: *mut SoundFontSample,
    pub ramAddr: *mut u8_0,
    pub encodedInfo: u32_0,
    pub isFree: s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioCmd {
    pub c2rust_unnamed: C2RustUnnamed_10,
    pub c2rust_unnamed_0: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub data: *mut libc::c_void,
    pub asFloat: f32_0,
    pub asInt: s32,
    pub asUShort: u16_0,
    pub asSbyte: s8,
    pub asUbyte: u8_0,
    pub asUInt: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub opArgs: u32_0,
    pub c2rust_unnamed: C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub op: u8_0,
    pub arg0: u8_0,
    pub arg1: u8_0,
    pub arg2: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioAsyncLoad {
    pub status: s8,
    pub delay: s8,
    pub medium: s8,
    pub ramAddr: *mut u8_0,
    pub curDevAddr: u32_0,
    pub curRamAddr: *mut u8_0,
    pub bytesRemaining: u32_0,
    pub chunkSize: u32_0,
    pub unkMediumParam: s32,
    pub retMsg: u32_0,
    pub retQueue: *mut OSMesgQueue,
    pub msgQueue: OSMesgQueue,
    pub msg: OSMesg,
    pub ioMesg: OSIoMesg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioSlowLoad {
    pub medium: u8_0,
    pub seqOrFontId: u8_0,
    pub instId: u16_0,
    pub unkMediumParam: s32,
    pub curDevAddr: s32,
    pub curRamAddr: *mut u8_0,
    pub ramAddr: *mut u8_0,
    pub status: s32,
    pub bytesRemaining: s32,
    pub isDone: *mut s8,
    pub sample: SoundFontSample,
    pub msgqueue: OSMesgQueue,
    pub msg: OSMesg,
    pub ioMesg: OSIoMesg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioTableEntry {
    pub romAddr: u32_0,
    pub size: u32_0,
    pub medium: s8,
    pub cachePolicy: s8,
    pub shortData1: s16,
    pub shortData2: s16,
    pub shortData3: s16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioTable {
    pub numEntries: s16,
    pub unkMediumParam: s16,
    pub romAddr: u32_0,
    pub pad: [libc::c_char; 8],
    pub entries: [AudioTableEntry; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioTask {
    pub task: OSTask,
    pub taskQueue: *mut OSMesgQueue,
    pub unk_44: *mut libc::c_void,
    pub unk_48: [libc::c_char; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SampleDma {
    pub ramAddr: *mut u8_0,
    pub devAddr: u32_0,
    pub sizeUnused: u16_0,
    pub size: u16_0,
    pub unused: u8_0,
    pub reuseIndex: u8_0,
    pub ttl: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioContext {
    pub unk_0000: libc::c_char,
    pub numSynthesisReverbs: s8,
    pub unk_2: u16_0,
    pub unk_4: u16_0,
    pub unk_0006: [libc::c_char; 10],
    pub curLoadedBook: *mut s16,
    pub noteSubsEu: *mut NoteSubEu,
    pub synthesisReverbs: [SynthesisReverb; 4],
    pub unk_0B38: [libc::c_char; 48],
    pub usedSamples: [*mut SoundFontSample; 128],
    pub preloadSampleStack: [AudioPreloadReq; 128],
    pub numUsedSamples: s32,
    pub preloadSampleStackTop: s32,
    pub asyncLoads: [AudioAsyncLoad; 16],
    pub asyncLoadUnkMediumQueue: OSMesgQueue,
    pub unk_1D08: [libc::c_char; 64],
    pub curUnkMediumLoad: *mut AudioAsyncLoad,
    pub slowLoadPos: u32_0,
    pub slowLoads: [AudioSlowLoad; 2],
    pub cartHandle: *mut OSPiHandle,
    pub driveHandle: *mut OSPiHandle,
    pub externalLoadQueue: OSMesgQueue,
    pub externalLoadMesgBuf: [OSMesg; 16],
    pub preloadSampleQueue: OSMesgQueue,
    pub preloadSampleMesgBuf: [OSMesg; 16],
    pub currAudioFrameDmaQueue: OSMesgQueue,
    pub currAudioFrameDmaMesgBuf: [OSMesg; 64],
    pub currAudioFrameDmaIoMesgBuf: [OSIoMesg; 64],
    pub syncDmaQueue: OSMesgQueue,
    pub syncDmaMesg: OSMesg,
    pub syncDmaIoMesg: OSIoMesg,
    pub sampleDmas: *mut SampleDma,
    pub sampleDmaCount: u32_0,
    pub sampleDmaListSize1: u32_0,
    pub unused2628: s32,
    pub sampleDmaReuseQueue1: [u8_0; 256],
    pub sampleDmaReuseQueue2: [u8_0; 256],
    pub sampleDmaReuseQueue1RdPos: u8_0,
    pub sampleDmaReuseQueue2RdPos: u8_0,
    pub sampleDmaReuseQueue1WrPos: u8_0,
    pub sampleDmaReuseQueue2WrPos: u8_0,
    pub sequenceTable: *mut AudioTable,
    pub soundFontTable: *mut AudioTable,
    pub sampleBankTable: *mut AudioTable,
    pub sequenceFontTable: *mut u8_0,
    pub numSequences: u16_0,
    pub soundFonts: *mut SoundFont,
    pub audioBufferParameters: AudioBufferParameters,
    pub unk_2870: f32_0,
    pub sampleDmaBufSize1: s32,
    pub sampleDmaBufSize2: s32,
    pub unk_287C: [libc::c_char; 16],
    pub sampleDmaBufSize: s32,
    pub maxAudioCmds: s32,
    pub numNotes: s32,
    pub tempoInternalToExternal: s16,
    pub soundMode: s8,
    pub totalTaskCnt: s32,
    pub curAudioFrameDmaCount: s32,
    pub rspTaskIdx: s32,
    pub curAIBufIdx: s32,
    pub abiCmdBufs: [*mut Acmd; 2],
    pub curAbiCmdBuf: *mut Acmd,
    pub currTask: *mut AudioTask,
    pub unk_28BC: [libc::c_char; 4],
    pub rspTask: [AudioTask; 2],
    pub unk_2960: f32_0,
    pub refreshRate: s32,
    pub aiBuffers: [*mut s16; 3],
    pub aiBufLengths: [s16; 3],
    pub audioRandom: u32_0,
    pub audioErrorFlags: s32,
    pub resetTimer: u32_0,
    pub unk_2988: [libc::c_char; 8],
    pub audioSessionPool: AudioAllocPool,
    pub externalPool: AudioAllocPool,
    pub audioInitPool: AudioAllocPool,
    pub notesAndBuffersPool: AudioAllocPool,
    pub unk_29D0: [libc::c_char; 32],
    pub cachePool: AudioAllocPool,
    pub persistentCommonPool: AudioAllocPool,
    pub temporaryCommonPool: AudioAllocPool,
    pub seqCache: AudioCache,
    pub fontCache: AudioCache,
    pub sampleBankCache: AudioCache,
    pub permanentPool: AudioAllocPool,
    pub permanentCache: [AudioCacheEntry; 32],
    pub persistentSampleCache: AudioSampleCache,
    pub temporarySampleCache: AudioSampleCache,
    pub sessionPoolSplit: AudioPoolSplit4,
    pub cachePoolSplit: AudioPoolSplit2,
    pub persistentCommonPoolSplit: AudioPoolSplit3,
    pub temporaryCommonPoolSplit: AudioPoolSplit3,
    pub sampleFontLoadStatus: [u8_0; 48],
    pub fontLoadStatus: [u8_0; 48],
    pub seqLoadStatus: [u8_0; 128],
    pub resetStatus: u8_0,
    pub audioResetSpecIdToLoad: u8_0,
    pub audioResetFadeOutFramesLeft: s32,
    pub unk_3520: *mut f32_0,
    pub audioHeap: *mut u8_0,
    pub audioHeapSize: u32_0,
    pub notes: *mut Note,
    pub seqPlayers: [SequencePlayer; 4],
    pub sequenceLayers: [SequenceLayer; 64],
    pub sequenceChannelNone: SequenceChannel,
    pub noteSubEuOffset: s32,
    pub layerFreeList: AudioListItem,
    pub noteFreeLists: NotePool,
    pub cmdWrPos: u8_0,
    pub cmdRdPos: u8_0,
    pub cmdQueueFinished: u8_0,
    pub unk_5BDC: [u16_0; 4],
    pub audioResetQueueP: *mut OSMesgQueue,
    pub taskStartQueueP: *mut OSMesgQueue,
    pub cmdProcQueueP: *mut OSMesgQueue,
    pub taskStartQueue: OSMesgQueue,
    pub cmdProcQueue: OSMesgQueue,
    pub audioResetQueue: OSMesgQueue,
    pub taskStartMsgs: [OSMesg; 1],
    pub audioResetMesgs: [OSMesg; 1],
    pub cmdProcMsgs: [OSMesg; 4],
    pub cmdBuf: [AudioCmd; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioContextInitSizes {
    pub heapSize: u32_0,
    pub initPoolSize: u32_0,
    pub permanentPoolSize: u32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unk_50_s {
    pub unk_00: f32_0,
    pub unk_04: f32_0,
    pub unk_08: f32_0,
    pub unk_0C: u16_0,
    pub unk_10: f32_0,
    pub unk_14: f32_0,
    pub unk_18: f32_0,
    pub unk_1C: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unk_D_8016E750 {
    pub volCur: f32_0,
    pub volTarget: f32_0,
    pub unk_08: f32_0,
    pub unk_0C: u16_0,
    pub volScales: [u8_0; 4],
    pub volFadeTimer: u8_0,
    pub fadeVolUpdate: u8_0,
    pub unk_14: u32_0,
    pub unk_18: u16_0,
    pub unk_1C: f32_0,
    pub unk_20: f32_0,
    pub unk_24: f32_0,
    pub unk_28: u16_0,
    pub unk_2C: [u32_0; 8],
    pub unk_4C: u8_0,
    pub unk_4D: u8_0,
    pub unk_4E: u8_0,
    pub unk_50: [unk_50_s; 16],
    pub unk_250: u16_0,
    pub unk_252: u16_0,
    pub unk_254: u16_0,
    pub unk_256: u16_0,
    pub unk_258: u16_0,
    pub unk_25C: u32_0,
    pub unk_260: u8_0,
}
pub type C2RustUnnamed_12 = libc::c_uint;
pub const BANK_VOICE: C2RustUnnamed_12 = 6;
pub const BANK_OCARINA: C2RustUnnamed_12 = 5;
pub const BANK_SYSTEM: C2RustUnnamed_12 = 4;
pub const BANK_ENEMY: C2RustUnnamed_12 = 3;
pub const BANK_ENV: C2RustUnnamed_12 = 2;
pub const BANK_ITEM: C2RustUnnamed_12 = 1;
pub const BANK_PLAYER: C2RustUnnamed_12 = 0;
pub type C2RustUnnamed_13 = libc::c_uint;
pub const SFX_STATE_PLAYING_2: C2RustUnnamed_13 = 5;
pub const SFX_STATE_PLAYING_1: C2RustUnnamed_13 = 4;
pub const SFX_STATE_PLAYING_REFRESH: C2RustUnnamed_13 = 3;
pub const SFX_STATE_READY: C2RustUnnamed_13 = 2;
pub const SFX_STATE_QUEUED: C2RustUnnamed_13 = 1;
pub const SFX_STATE_EMPTY: C2RustUnnamed_13 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SoundBankEntry {
    pub posX: *mut f32_0,
    pub posY: *mut f32_0,
    pub posZ: *mut f32_0,
    pub token: u8_0,
    pub freqScale: *mut f32_0,
    pub vol: *mut f32_0,
    pub reverbAdd: *mut s8,
    pub dist: f32_0,
    pub priority: u32_0,
    pub sfxImportance: u8_0,
    pub sfxParams: u16_0,
    pub sfxId: u16_0,
    pub state: u8_0,
    pub freshness: u8_0,
    pub prev: u8_0,
    pub next: u8_0,
    pub channelIdx: u8_0,
    pub unk_2F: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ActiveSound {
    pub priority: u32_0,
    pub entryIndex: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SoundParams {
    pub importance: u8_0,
    pub params: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OcarinaNote {
    pub noteIdx: u8_0,
    pub unk_01: u8_0,
    pub unk_02: u16_0,
    pub volume: u8_0,
    pub vibrato: u8_0,
    pub tone: s8,
    pub semitone: u8_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OcarinaSongInfo {
    pub len: u8_0,
    pub notesIdx: [u8_0; 8],
}
pub type C2RustUnnamed_14 = libc::c_uint;
pub const OCARINA_NOTE_INVALID: C2RustUnnamed_14 = 255;
pub const OCARINA_NOTE_C_UP: C2RustUnnamed_14 = 4;
pub const OCARINA_NOTE_C_LEFT: C2RustUnnamed_14 = 3;
pub const OCARINA_NOTE_C_RIGHT: C2RustUnnamed_14 = 2;
pub const OCARINA_NOTE_C_DOWN: C2RustUnnamed_14 = 1;
pub const OCARINA_NOTE_A: C2RustUnnamed_14 = 0;
pub type C2RustUnnamed_15 = libc::c_uint;
pub const SEQ_PLAYER_BGM_SUB: C2RustUnnamed_15 = 3;
pub const SEQ_PLAYER_SFX: C2RustUnnamed_15 = 2;
pub const SEQ_PLAYER_FANFARE: C2RustUnnamed_15 = 1;
pub const SEQ_PLAYER_BGM_MAIN: C2RustUnnamed_15 = 0;
pub type C2RustUnnamed_16 = libc::c_uint;
pub const SEQ_MODE_IGNORE: C2RustUnnamed_16 = 3;
pub const SEQ_MODE_STILL: C2RustUnnamed_16 = 2;
pub const SEQ_MODE_ENEMY: C2RustUnnamed_16 = 1;
pub const SEQ_MODE_DEFAULT: C2RustUnnamed_16 = 0;
pub type C2RustUnnamed_17 = libc::c_uint;
pub const OCARINA_SONG_SCARECROW_LONG: C2RustUnnamed_17 = 14;
pub const OCARINA_SONG_MAX: C2RustUnnamed_17 = 14;
pub const OCARINA_SONG_MEMORY_GAME: C2RustUnnamed_17 = 13;
pub const OCARINA_SONG_SCARECROW: C2RustUnnamed_17 = 12;
pub const OCARINA_SONG_STORMS: C2RustUnnamed_17 = 11;
pub const OCARINA_SONG_TIME: C2RustUnnamed_17 = 10;
pub const OCARINA_SONG_SUNS: C2RustUnnamed_17 = 9;
pub const OCARINA_SONG_LULLABY: C2RustUnnamed_17 = 8;
pub const OCARINA_SONG_EPONAS: C2RustUnnamed_17 = 7;
pub const OCARINA_SONG_SARIAS: C2RustUnnamed_17 = 6;
pub const OCARINA_SONG_PRELUDE: C2RustUnnamed_17 = 5;
pub const OCARINA_SONG_NOCTURNE: C2RustUnnamed_17 = 4;
pub const OCARINA_SONG_REQUIEM: C2RustUnnamed_17 = 3;
pub const OCARINA_SONG_SERENADE: C2RustUnnamed_17 = 2;
pub const OCARINA_SONG_BOLERO: C2RustUnnamed_17 = 1;
pub const OCARINA_SONG_MINUET: C2RustUnnamed_17 = 0;
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
pub struct OcarinaStick {
    pub x: s8,
    pub y: s8,
}
pub const PAGE_FREE_AREA: C2RustUnnamed_19 = 14;
pub const PAGE_SFX_PARAMETER_CHANGE: C2RustUnnamed_19 = 12;
pub const PAGE_OCARINA_TEST: C2RustUnnamed_19 = 11;
pub const PAGE_BLOCK_CHANGE_BGM: C2RustUnnamed_19 = 9;
pub const PAGE_HEAP_INFO: C2RustUnnamed_19 = 3;
pub const PAGE_SUB_TRACK_INFO: C2RustUnnamed_19 = 5;
pub const PAGE_SFX_SWAP: C2RustUnnamed_19 = 8;
pub const PAGE_SCROLL_PRINT: C2RustUnnamed_19 = 13;
pub const PAGE_INTERFACE_INFO: C2RustUnnamed_19 = 7;
pub const PAGE_SOUND_CONTROL: C2RustUnnamed_19 = 1;
pub const PAGE_NON: C2RustUnnamed_19 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub str_0: [s8; 5],
    pub num: u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct D_801306DC_s {
    pub unk_00: u16_0,
    pub unk_02: u16_0,
    pub unk_04: [u8_0; 100],
}
pub const PAGE_MAX: C2RustUnnamed_19 = 15;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FreqLerp {
    pub value: f32_0,
    pub target: f32_0,
    pub step: f32_0,
    pub remainingFrames: s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SfxPlayerState {
    pub vol: f32_0,
    pub freqScale: f32_0,
    pub reverb: s8,
    pub panSigned: s8,
    pub stereoBits: s8,
    pub filter: u8_0,
    pub unk_0C: u8_0,
}
pub type C2RustUnnamed_19 = libc::c_uint;
pub const PAGE_NATURAL_SOUND_CONTROL: C2RustUnnamed_19 = 10;
pub const PAGE_CHANNEL_INFO: C2RustUnnamed_19 = 6;
pub const PAGE_GROUP_TRACK_INFO: C2RustUnnamed_19 = 4;
pub const PAGE_SPEC_INFO: C2RustUnnamed_19 = 2;
// from audio_synthesis
#[no_mangle]
pub static mut gIsLargeSoundBank: [u8_0; 7] =
    [0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0];
// Only the first row of these is supported by sequence 0. (gSfxChannelLayout is always 0.)
#[no_mangle]
pub static mut gChannelsPerBank: [[u8_0; 7]; 4] =
    [[3 as libc::c_int as u8_0, 2 as libc::c_int as u8_0,
      3 as libc::c_int as u8_0, 3 as libc::c_int as u8_0,
      2 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
      2 as libc::c_int as u8_0],
     [3 as libc::c_int as u8_0, 2 as libc::c_int as u8_0,
      2 as libc::c_int as u8_0, 2 as libc::c_int as u8_0,
      2 as libc::c_int as u8_0, 2 as libc::c_int as u8_0,
      2 as libc::c_int as u8_0],
     [3 as libc::c_int as u8_0, 2 as libc::c_int as u8_0,
      2 as libc::c_int as u8_0, 2 as libc::c_int as u8_0,
      2 as libc::c_int as u8_0, 2 as libc::c_int as u8_0,
      2 as libc::c_int as u8_0],
     [4 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
      0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
      2 as libc::c_int as u8_0, 2 as libc::c_int as u8_0,
      2 as libc::c_int as u8_0]];
#[no_mangle]
pub static mut gUsedChannelsPerBank: [[u8_0; 7]; 4] =
    [[3 as libc::c_int as u8_0, 2 as libc::c_int as u8_0,
      3 as libc::c_int as u8_0, 2 as libc::c_int as u8_0,
      2 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
      1 as libc::c_int as u8_0],
     [3 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
      1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
      2 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
      1 as libc::c_int as u8_0],
     [3 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
      1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
      2 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
      1 as libc::c_int as u8_0],
     [2 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
      0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
      1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
      1 as libc::c_int as u8_0]];
#[no_mangle]
pub static mut D_801305B0: f32_0 = 0.7950898f32;
#[no_mangle]
pub static mut D_801305B4: s8 = 35 as libc::c_int as s8;
#[no_mangle]
pub static mut D_801305B8: s8 = 20 as libc::c_int as s8;
#[no_mangle]
pub static mut D_801305BC: s8 = 30 as libc::c_int as s8;
#[no_mangle]
pub static mut D_801305C0: s8 = 20 as libc::c_int as s8;
#[no_mangle]
pub static mut sBehindScreenZ: [f32_0; 2] = [-15.0f32, -65.0f32];
#[no_mangle]
pub static mut sAudioIncreasingTranspose: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut gMorphaTransposeTable: [u8_0; 16] =
    [0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 2 as libc::c_int as u8_0,
     4 as libc::c_int as u8_0, 6 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 8 as libc::c_int as u8_0];
#[no_mangle]
pub static mut sPrevChargeLevel: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut D_801305E4: [f32_0; 4] =
    [1.0f32, 1.12246f32, 1.33484f32, 1.33484f32];
// 2**({0, 2, 5, 5}/12)
#[no_mangle]
pub static mut D_801305F4: f32_0 = 1.0f32;
#[no_mangle]
pub static mut D_801305F8: [u8_0; 8] =
    [127 as libc::c_int as u8_0, 80 as libc::c_int as u8_0,
     75 as libc::c_int as u8_0, 73 as libc::c_int as u8_0,
     70 as libc::c_int as u8_0, 68 as libc::c_int as u8_0,
     65 as libc::c_int as u8_0, 60 as libc::c_int as u8_0];
#[no_mangle]
pub static mut D_80130600: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut D_80130604: s8 = 2 as libc::c_int as s8;
#[no_mangle]
pub static mut D_80130608: s8 = 0 as libc::c_int as s8;
#[no_mangle]
pub static mut sAudioCutsceneFlag: s8 = 0 as libc::c_int as s8;
#[no_mangle]
pub static mut sSpecReverb: s8 = 0 as libc::c_int as s8;
#[no_mangle]
pub static mut sAudioEnvReverb: s8 = 0 as libc::c_int as s8;
#[no_mangle]
pub static mut sAudioCodeReverb: s8 = 0 as libc::c_int as s8;
#[no_mangle]
pub static mut sPrevSeqMode: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut sAudioEnemyDist: f32_0 = 0.0f32;
#[no_mangle]
pub static mut sAudioEnemyVol: s8 = 127 as libc::c_int as s8;
#[no_mangle]
pub static mut D_80130628: u16_0 = 0xffff as libc::c_int as u16_0;
#[no_mangle]
pub static mut D_8013062C: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut D_80130630: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut sNumFramesStill: u32_0 = 0 as libc::c_int as u32_0;
#[no_mangle]
pub static mut sNumFramesMoving: u32_0 = 0 as libc::c_int as u32_0;
#[no_mangle]
pub static mut sAudioBaseFilter: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut sAudioExtraFilter: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut sAudioBaseFilter2: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut sAudioExtraFilter2: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut sSariaBgmPtr: *mut Vec3f = 0 as *const Vec3f as *mut Vec3f;
#[no_mangle]
pub static mut D_80130650: f32_0 = 2000.0f32;
#[no_mangle]
pub static mut sSeqModeInput: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut sSeqFlags: [u8_0; 110] =
    [0x2 as libc::c_int as u8_0, 0x1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x21 as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x88 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x20 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x2 as libc::c_int as u8_0, 0x4 as libc::c_int as u8_0,
     0x2 as libc::c_int as u8_0, 0x2 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x10 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x2 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x80 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x80 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x2 as libc::c_int as u8_0, 0x2 as libc::c_int as u8_0,
     0x2 as libc::c_int as u8_0, 0x2 as libc::c_int as u8_0,
     0x2 as libc::c_int as u8_0, 0x2 as libc::c_int as u8_0,
     0x88 as libc::c_int as u8_0, 0x2 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x2 as libc::c_int as u8_0,
     0x11 as libc::c_int as u8_0, 0x2 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x1 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x2 as libc::c_int as u8_0,
     0x2 as libc::c_int as u8_0, 0x2 as libc::c_int as u8_0,
     0x2 as libc::c_int as u8_0, 0x2 as libc::c_int as u8_0,
     0x2 as libc::c_int as u8_0, 0x2 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x20 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x10 as libc::c_int as u8_0, 0x2 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x4 as libc::c_int as u8_0, 0x20 as libc::c_int as u8_0,
     0x40 as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x2 as libc::c_int as u8_0,
     0x2 as libc::c_int as u8_0, 0x1 as libc::c_int as u8_0,
     0x1 as libc::c_int as u8_0, 0x2 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0x11 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x80 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x80 as libc::c_int as u8_0, 0x80 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0x8 as libc::c_int as u8_0, 0 as libc::c_int as u8_0];
#[no_mangle]
pub static mut sSpecReverbs: [s8; 20] =
    [0 as libc::c_int as s8, 0 as libc::c_int as s8, 0 as libc::c_int as s8,
     0 as libc::c_int as s8, 0 as libc::c_int as s8, 0 as libc::c_int as s8,
     0 as libc::c_int as s8, 40 as libc::c_int as s8, 0 as libc::c_int as s8,
     15 as libc::c_int as s8, 0 as libc::c_int as s8, 0 as libc::c_int as s8,
     0 as libc::c_int as s8, 0 as libc::c_int as s8, 0 as libc::c_int as s8,
     0 as libc::c_int as s8, 0 as libc::c_int as s8, 0 as libc::c_int as s8,
     0 as libc::c_int as s8, 0 as libc::c_int as s8];
#[no_mangle]
pub static mut D_801306DC: [D_801306DC_s; 20] =
    [{
         let mut init =
             D_801306DC_s{unk_00: 0xc0ff as libc::c_int as u16_0,
                          unk_02: 0xc0fe as libc::c_int as u16_0,
                          unk_04:
                              [0 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               9 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               64 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               32 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               16 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               10 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               112 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               48 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               14 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               127 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               16 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               127 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               16 as libc::c_int as u8_0,
                               6 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               6 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               127 as libc::c_int as u8_0,
                               6 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               6 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               16 as libc::c_int as u8_0,
                               7 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               17 as libc::c_int as u8_0,
                               7 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               127 as libc::c_int as u8_0,
                               7 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               7 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               16 as libc::c_int as u8_0,
                               0xff as libc::c_int as u8_0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0],};
         init
     },
     {
         let mut init =
             D_801306DC_s{unk_00: 0xc0fb as libc::c_int as u16_0,
                          unk_02: 0xc0fa as libc::c_int as u16_0,
                          unk_04:
                              [0 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               16 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               11 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               112 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               48 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               14 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               127 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               16 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               127 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               16 as libc::c_int as u8_0,
                               6 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               6 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               127 as libc::c_int as u8_0,
                               6 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               6 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               16 as libc::c_int as u8_0,
                               7 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               17 as libc::c_int as u8_0,
                               7 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               127 as libc::c_int as u8_0,
                               7 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               7 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               16 as libc::c_int as u8_0,
                               0xff as libc::c_int as u8_0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
         init
     },
     {
         let mut init =
             D_801306DC_s{unk_00: 0xc001 as libc::c_int as u16_0,
                          unk_02: 0x4000 as libc::c_int as u16_0,
                          unk_04:
                              [0 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               11 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               48 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               32 as libc::c_int as u8_0,
                               0xff as libc::c_int as u8_0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
         init
     },
     {
         let mut init =
             D_801306DC_s{unk_00: 0xc005 as libc::c_int as u16_0,
                          unk_02: 0x4000 as libc::c_int as u16_0,
                          unk_04:
                              [0 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               32 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               11 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               48 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               32 as libc::c_int as u8_0,
                               0xff as libc::c_int as u8_0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
         init
     },
     {
         let mut init =
             D_801306DC_s{unk_00: 0xc01f as libc::c_int as u16_0,
                          unk_02: 0xc000 as libc::c_int as u16_0,
                          unk_04:
                              [0 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               47 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               13 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               16 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               16 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               32 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               14 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               44 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               11 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               63 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               44 as libc::c_int as u8_0,
                               0xff as libc::c_int as u8_0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0],};
         init
     },
     {
         let mut init =
             D_801306DC_s{unk_00: 0xc003 as libc::c_int as u16_0,
                          unk_02: 0xc000 as libc::c_int as u16_0,
                          unk_04:
                              [0 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               16 as libc::c_int as u8_0,
                               0xff as libc::c_int as u8_0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
         init
     },
     {
         let mut init =
             D_801306DC_s{unk_00: 0xc0fb as libc::c_int as u16_0,
                          unk_02: 0xc0fa as libc::c_int as u16_0,
                          unk_04:
                              [0 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               16 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               11 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               112 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               48 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               14 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               127 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               16 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               127 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               16 as libc::c_int as u8_0,
                               6 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               6 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               127 as libc::c_int as u8_0,
                               6 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               6 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               16 as libc::c_int as u8_0,
                               7 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               17 as libc::c_int as u8_0,
                               7 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               127 as libc::c_int as u8_0,
                               7 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               7 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               16 as libc::c_int as u8_0,
                               0xff as libc::c_int as u8_0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
         init
     },
     {
         let mut init =
             D_801306DC_s{unk_00: 0x8001 as libc::c_int as u16_0,
                          unk_02: 0 as libc::c_int as u16_0,
                          unk_04:
                              [0 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               32 as libc::c_int as u8_0,
                               0xff as libc::c_int as u8_0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0],};
         init
     },
     {
         let mut init =
             D_801306DC_s{unk_00: 0xc003 as libc::c_int as u16_0,
                          unk_02: 0xc000 as libc::c_int as u16_0,
                          unk_04:
                              [0 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               16 as libc::c_int as u8_0,
                               0xff as libc::c_int as u8_0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
         init
     },
     {
         let mut init =
             D_801306DC_s{unk_00: 0xc003 as libc::c_int as u16_0,
                          unk_02: 0xc000 as libc::c_int as u16_0,
                          unk_04:
                              [0 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               16 as libc::c_int as u8_0,
                               0xff as libc::c_int as u8_0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
         init
     },
     {
         let mut init =
             D_801306DC_s{unk_00: 0xc001 as libc::c_int as u16_0,
                          unk_02: 0xc000 as libc::c_int as u16_0,
                          unk_04:
                              [0 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               0xff as libc::c_int as u8_0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0],};
         init
     },
     {
         let mut init =
             D_801306DC_s{unk_00: 0xc02f as libc::c_int as u16_0,
                          unk_02: 0xc02e as libc::c_int as u16_0,
                          unk_04:
                              [0 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               10 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               64 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               32 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               15 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               112 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               48 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               14 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               127 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               16 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               127 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               16 as libc::c_int as u8_0,
                               0xff as libc::c_int as u8_0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0],};
         init
     },
     {
         let mut init =
             D_801306DC_s{unk_00: 0xc07f as libc::c_int as u16_0,
                          unk_02: 0xc07e as libc::c_int as u16_0,
                          unk_04:
                              [0 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               10 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               64 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               32 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               11 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               112 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               48 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               12 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               127 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               16 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               6 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               16 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               16 as libc::c_int as u8_0,
                               6 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               6 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               6 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               6 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               16 as libc::c_int as u8_0,
                               0xff as libc::c_int as u8_0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
         init
     },
     {
         let mut init =
             D_801306DC_s{unk_00: 0xc003 as libc::c_int as u16_0,
                          unk_02: 0xc000 as libc::c_int as u16_0,
                          unk_04:
                              [0 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               16 as libc::c_int as u8_0,
                               0xff as libc::c_int as u8_0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
         init
     },
     {
         let mut init =
             D_801306DC_s{unk_00: 0xc003 as libc::c_int as u16_0,
                          unk_02: 0xc000 as libc::c_int as u16_0,
                          unk_04:
                              [0 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               16 as libc::c_int as u8_0,
                               0xff as libc::c_int as u8_0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
         init
     },
     {
         let mut init =
             D_801306DC_s{unk_00: 0xc01f as libc::c_int as u16_0,
                          unk_02: 0xc000 as libc::c_int as u16_0,
                          unk_04:
                              [0 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               80 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               8 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               10 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               80 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               48 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               6 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               11 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               96 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               32 as libc::c_int as u8_0,
                               0xff as libc::c_int as u8_0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0],};
         init
     },
     {
         let mut init =
             D_801306DC_s{unk_00: 0xc003 as libc::c_int as u16_0,
                          unk_02: 0xc000 as libc::c_int as u16_0,
                          unk_04:
                              [0 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               16 as libc::c_int as u8_0,
                               0xff as libc::c_int as u8_0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
         init
     },
     {
         let mut init =
             D_801306DC_s{unk_00: 0xc003 as libc::c_int as u16_0,
                          unk_02: 0xc000 as libc::c_int as u16_0,
                          unk_04:
                              [0 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               16 as libc::c_int as u8_0,
                               0xff as libc::c_int as u8_0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
         init
     },
     {
         let mut init =
             D_801306DC_s{unk_00: 0xc003 as libc::c_int as u16_0,
                          unk_02: 0xc000 as libc::c_int as u16_0,
                          unk_04:
                              [0 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               16 as libc::c_int as u8_0,
                               0xff as libc::c_int as u8_0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
         init
     },
     {
         let mut init =
             D_801306DC_s{unk_00: 0xc003 as libc::c_int as u16_0,
                          unk_02: 0xc000 as libc::c_int as u16_0,
                          unk_04:
                              [0 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               2 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               3 as libc::c_int as u8_0,
                               0 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               4 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               1 as libc::c_int as u8_0,
                               5 as libc::c_int as u8_0,
                               16 as libc::c_int as u8_0,
                               0xff as libc::c_int as u8_0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                               0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],};
         init
     }];
#[no_mangle]
pub static mut sOcarinaAllowedBtnMask: u32_0 = 0x800f as libc::c_int as u32_0;
#[no_mangle]
pub static mut sOcarinaABtnMap: s32 = 0x8000 as libc::c_int;
#[no_mangle]
pub static mut sOcarinaCUPBtnMap: s32 = 8 as libc::c_int;
#[no_mangle]
pub static mut sOcarinaCDownBtnMap: s32 = 4 as libc::c_int;
#[no_mangle]
pub static mut sOcarinaInpEnabled: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut D_80130F10: s8 = 0 as libc::c_int as s8;
// "OCA", ocarina active?
#[no_mangle]
pub static mut sCurOcarinaBtnVal: u8_0 = 0xff as libc::c_int as u8_0;
#[no_mangle]
pub static mut sPrevOcarinaNoteVal: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut sCurOcarinaBtnIdx: u8_0 = 0 as libc::c_int as u8_0;
// note index?
#[no_mangle]
pub static mut sLearnSongLastBtn: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut D_80130F24: f32_0 = 1.0f32;
#[no_mangle]
pub static mut D_80130F28: f32_0 = 87.0f32 / 127.0f32;
#[no_mangle]
pub static mut D_80130F2C: s8 = 0 as libc::c_int as s8;
// pitch?
#[no_mangle]
pub static mut D_80130F30: s8 = 0x57 as libc::c_int as s8;
#[no_mangle]
pub static mut D_80130F34: s8 = 0 as libc::c_int as s8;
#[no_mangle]
pub static mut sPlaybackState: u8_0 = 0 as libc::c_int as u8_0;
// 80130F38
#[no_mangle]
pub static mut D_80130F3C: u32_0 = 0 as libc::c_int as u32_0;
// "SEQ"
#[no_mangle]
pub static mut sNotePlaybackTimer: u32_0 = 0 as libc::c_int as u32_0;
#[no_mangle]
pub static mut sPlaybackNotePos: u16_0 = 0 as libc::c_int as u16_0;
#[no_mangle]
pub static mut sStaffPlaybackPos: u16_0 = 0 as libc::c_int as u16_0;
#[no_mangle]
pub static mut D_80130F4C: u16_0 = 0 as libc::c_int as u16_0;
#[no_mangle]
pub static mut sDisplayedNoteValue: u8_0 = 0xff as libc::c_int as u8_0;
// Note to display on screen?
#[no_mangle]
pub static mut sNotePlaybackVolume: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut sNotePlaybackVibrato: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut sNotePlaybackTone: s8 = 0 as libc::c_int as s8;
#[no_mangle]
pub static mut sNormalizedNotePlaybackTone: f32_0 = 1.0f32;
#[no_mangle]
pub static mut sNormalizedNotePlaybackVolume: f32_0 = 1.0f32;
#[no_mangle]
pub static mut D_80130F68: s32 = 0 as libc::c_int;
#[no_mangle]
pub static mut sOcarinaNoteValues: [u8_0; 5] =
    [2 as libc::c_int as u8_0, 5 as libc::c_int as u8_0,
     9 as libc::c_int as u8_0, 11 as libc::c_int as u8_0,
     14 as libc::c_int as u8_0];
#[no_mangle]
pub static mut sOcaMinigameAppendPos: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut sOcaMinigameEndPos: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut sOcaMinigameNoteCnts: [u8_0; 3] =
    [5 as libc::c_int as u8_0, 6 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0];
#[no_mangle]
pub static mut sOcarinaSongs: [[OcarinaNote; 20]; 14] =
    [[{
          let mut init =
              OcarinaNote{noteIdx: 2 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 18 as libc::c_int as u16_0,
                          volume: 86 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 14 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 18 as libc::c_int as u16_0,
                          volume: 92 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 11 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 72 as libc::c_int as u16_0,
                          volume: 86 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 9 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 18 as libc::c_int as u16_0,
                          volume: 80 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 11 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 18 as libc::c_int as u16_0,
                          volume: 88 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 9 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 144 as libc::c_int as u16_0,
                          volume: 86 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 0xff as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 0 as libc::c_int as u16_0,
                          volume: 86 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,}],
     [{
          let mut init =
              OcarinaNote{noteIdx: 5 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 15 as libc::c_int as u16_0,
                          volume: 80 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 2 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 15 as libc::c_int as u16_0,
                          volume: 72 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 5 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 15 as libc::c_int as u16_0,
                          volume: 84 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 2 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 15 as libc::c_int as u16_0,
                          volume: 76 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 9 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 15 as libc::c_int as u16_0,
                          volume: 84 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 5 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 15 as libc::c_int as u16_0,
                          volume: 74 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 9 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 15 as libc::c_int as u16_0,
                          volume: 78 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 5 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 135 as libc::c_int as u16_0,
                          volume: 66 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 0xff as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 0 as libc::c_int as u16_0,
                          volume: 66 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,}],
     [{
          let mut init =
              OcarinaNote{noteIdx: 2 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 36 as libc::c_int as u16_0,
                          volume: 60 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 5 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 36 as libc::c_int as u16_0,
                          volume: 78 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 9 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 33 as libc::c_int as u16_0,
                          volume: 82 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 0xff as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 3 as libc::c_int as u16_0,
                          volume: 82 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 9 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 36 as libc::c_int as u16_0,
                          volume: 84 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 11 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 144 as libc::c_int as u16_0,
                          volume: 90 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 0xff as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 0 as libc::c_int as u16_0,
                          volume: 90 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,}],
     [{
          let mut init =
              OcarinaNote{noteIdx: 2 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 45 as libc::c_int as u16_0,
                          volume: 88 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 5 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 23 as libc::c_int as u16_0,
                          volume: 86 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 2 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 22 as libc::c_int as u16_0,
                          volume: 84 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 9 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 45 as libc::c_int as u16_0,
                          volume: 86 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 5 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 45 as libc::c_int as u16_0,
                          volume: 94 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 2 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 180 as libc::c_int as u16_0,
                          volume: 94 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 0xff as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 0 as libc::c_int as u16_0,
                          volume: 94 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,}],
     [{
          let mut init =
              OcarinaNote{noteIdx: 11 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 36 as libc::c_int as u16_0,
                          volume: 88 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 9 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 33 as libc::c_int as u16_0,
                          volume: 84 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 0xff as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 3 as libc::c_int as u16_0,
                          volume: 84 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 9 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 18 as libc::c_int as u16_0,
                          volume: 82 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 2 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 18 as libc::c_int as u16_0,
                          volume: 60 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 11 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 18 as libc::c_int as u16_0,
                          volume: 90 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 9 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 18 as libc::c_int as u16_0,
                          volume: 88 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 5 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 144 as libc::c_int as u16_0,
                          volume: 96 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 0xff as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 0 as libc::c_int as u16_0,
                          volume: 96 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,}],
     [{
          let mut init =
              OcarinaNote{noteIdx: 14 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 15 as libc::c_int as u16_0,
                          volume: 84 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 9 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 45 as libc::c_int as u16_0,
                          volume: 88 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 14 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 15 as libc::c_int as u16_0,
                          volume: 88 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 9 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 15 as libc::c_int as u16_0,
                          volume: 82 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 11 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 15 as libc::c_int as u16_0,
                          volume: 86 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 14 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 60 as libc::c_int as u16_0,
                          volume: 90 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 0xff as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 75 as libc::c_int as u16_0,
                          volume: 90 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 0xff as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 0 as libc::c_int as u16_0,
                          volume: 90 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,}],
     [{
          let mut init =
              OcarinaNote{noteIdx: 5 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 17 as libc::c_int as u16_0,
                          volume: 84 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 9 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 17 as libc::c_int as u16_0,
                          volume: 88 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 11 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 34 as libc::c_int as u16_0,
                          volume: 80 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 5 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 17 as libc::c_int as u16_0,
                          volume: 84 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 9 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 17 as libc::c_int as u16_0,
                          volume: 88 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 11 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 136 as libc::c_int as u16_0,
                          volume: 80 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 0xff as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 0 as libc::c_int as u16_0,
                          volume: 90 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,}],
     [{
          let mut init =
              OcarinaNote{noteIdx: 14 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 18 as libc::c_int as u16_0,
                          volume: 84 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 11 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 18 as libc::c_int as u16_0,
                          volume: 88 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 9 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 72 as libc::c_int as u16_0,
                          volume: 80 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 14 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 18 as libc::c_int as u16_0,
                          volume: 84 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 11 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 18 as libc::c_int as u16_0,
                          volume: 88 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 9 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 144 as libc::c_int as u16_0,
                          volume: 80 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 0xff as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 0 as libc::c_int as u16_0,
                          volume: 90 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,}],
     [{
          let mut init =
              OcarinaNote{noteIdx: 11 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 51 as libc::c_int as u16_0,
                          volume: 84 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 14 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 25 as libc::c_int as u16_0,
                          volume: 88 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 9 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 78 as libc::c_int as u16_0,
                          volume: 80 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 11 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 51 as libc::c_int as u16_0,
                          volume: 84 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 14 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 25 as libc::c_int as u16_0,
                          volume: 88 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 9 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 100 as libc::c_int as u16_0,
                          volume: 80 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 0xff as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 0 as libc::c_int as u16_0,
                          volume: 90 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,}],
     [{
          let mut init =
              OcarinaNote{noteIdx: 9 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 12 as libc::c_int as u16_0,
                          volume: 84 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 5 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 13 as libc::c_int as u16_0,
                          volume: 88 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 14 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 29 as libc::c_int as u16_0,
                          volume: 80 as libc::c_int as u8_0,
                          vibrato: 2 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 0xff as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 9 as libc::c_int as u16_0,
                          volume: 84 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 9 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 12 as libc::c_int as u16_0,
                          volume: 84 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 5 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 13 as libc::c_int as u16_0,
                          volume: 88 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 14 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 120 as libc::c_int as u16_0,
                          volume: 80 as libc::c_int as u8_0,
                          vibrato: 3 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 0xff as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 0 as libc::c_int as u16_0,
                          volume: 90 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,}],
     [{
          let mut init =
              OcarinaNote{noteIdx: 9 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 32 as libc::c_int as u16_0,
                          volume: 84 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 2 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 65 as libc::c_int as u16_0,
                          volume: 88 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 5 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 33 as libc::c_int as u16_0,
                          volume: 80 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 9 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 32 as libc::c_int as u16_0,
                          volume: 84 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 2 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 65 as libc::c_int as u16_0,
                          volume: 88 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 5 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 99 as libc::c_int as u16_0,
                          volume: 80 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 0xff as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 0 as libc::c_int as u16_0,
                          volume: 90 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,}],
     [{
          let mut init =
              OcarinaNote{noteIdx: 2 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 11 as libc::c_int as u16_0,
                          volume: 84 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 5 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 11 as libc::c_int as u16_0,
                          volume: 88 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 14 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 45 as libc::c_int as u16_0,
                          volume: 80 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 2 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 11 as libc::c_int as u16_0,
                          volume: 84 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 5 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 11 as libc::c_int as u16_0,
                          volume: 88 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 14 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 90 as libc::c_int as u16_0,
                          volume: 80 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 0xff as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 0 as libc::c_int as u16_0,
                          volume: 90 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,}],
     [{
          let mut init =
              OcarinaNote{noteIdx: 2 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 3 as libc::c_int as u16_0,
                          volume: 0 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 0xff as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 0 as libc::c_int as u16_0,
                          volume: 255 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,}],
     [{
          let mut init =
              OcarinaNote{noteIdx: 2 as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 3 as libc::c_int as u16_0,
                          volume: 0 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      {
          let mut init =
              OcarinaNote{noteIdx: 0xff as libc::c_int as u8_0,
                          unk_01: 0 as libc::c_int as u8_0,
                          unk_02: 0 as libc::c_int as u16_0,
                          volume: 0 as libc::c_int as u8_0,
                          vibrato: 0 as libc::c_int as u8_0,
                          tone: 0 as libc::c_int as s8,
                          semitone: 0 as libc::c_int as u8_0,};
          init
      },
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,},
      OcarinaNote{noteIdx: 0,
                  unk_01: 0,
                  unk_02: 0,
                  volume: 0,
                  vibrato: 0,
                  tone: 0,
                  semitone: 0,}]];
// Initialized in run_static_initializers
#[no_mangle]
pub static mut sPlaybackSong: *mut OcarinaNote =
    0 as *const OcarinaNote as *mut OcarinaNote;
#[no_mangle]
pub static mut sFrogsSongNotes: [u8_0; 14] =
    [OCARINA_NOTE_A as libc::c_int as u8_0,
     OCARINA_NOTE_C_LEFT as libc::c_int as u8_0,
     OCARINA_NOTE_C_RIGHT as libc::c_int as u8_0,
     OCARINA_NOTE_C_DOWN as libc::c_int as u8_0,
     OCARINA_NOTE_C_LEFT as libc::c_int as u8_0,
     OCARINA_NOTE_C_RIGHT as libc::c_int as u8_0,
     OCARINA_NOTE_C_DOWN as libc::c_int as u8_0,
     OCARINA_NOTE_A as libc::c_int as u8_0,
     OCARINA_NOTE_C_DOWN as libc::c_int as u8_0,
     OCARINA_NOTE_A as libc::c_int as u8_0,
     OCARINA_NOTE_C_DOWN as libc::c_int as u8_0,
     OCARINA_NOTE_C_RIGHT as libc::c_int as u8_0,
     OCARINA_NOTE_C_LEFT as libc::c_int as u8_0,
     OCARINA_NOTE_A as libc::c_int as u8_0];
#[no_mangle]
pub static mut gFrogsSongPtr: *mut u8_0 =
    unsafe { sFrogsSongNotes.as_ptr() as *mut _ };
#[no_mangle]
pub static mut sRecordingState: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut sRecordSongPos: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut D_80131860: u32_0 = 0 as libc::c_int as u32_0;
#[no_mangle]
pub static mut D_80131864: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut D_80131868: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut D_8013186C: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut D_80131870: s8 = 0 as libc::c_int as s8;
#[no_mangle]
pub static mut D_80131874: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut D_80131878: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut D_8013187C: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut D_80131880: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut sPierresSong: [OcarinaNote; 108] =
    [{
         let mut init =
             OcarinaNote{noteIdx: 0xff as libc::c_int as u8_0,
                         unk_01: 0 as libc::c_int as u8_0,
                         unk_02: 0 as libc::c_int as u16_0,
                         volume: 0 as libc::c_int as u8_0,
                         vibrato: 0 as libc::c_int as u8_0,
                         tone: 0 as libc::c_int as s8,
                         semitone: 0 as libc::c_int as u8_0,};
         init
     },
     {
         let mut init =
             OcarinaNote{noteIdx: 0xff as libc::c_int as u8_0,
                         unk_01: 0 as libc::c_int as u8_0,
                         unk_02: 0 as libc::c_int as u16_0,
                         volume: 0 as libc::c_int as u8_0,
                         vibrato: 0 as libc::c_int as u8_0,
                         tone: 0 as libc::c_int as s8,
                         semitone: 0 as libc::c_int as u8_0,};
         init
     },
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,},
     OcarinaNote{noteIdx: 0,
                 unk_01: 0,
                 unk_02: 0,
                 volume: 0,
                 vibrato: 0,
                 tone: 0,
                 semitone: 0,}];
#[no_mangle]
pub static mut gScarecrowCustomSongPtr: *mut OcarinaNote =
    unsafe { sPierresSong.as_ptr() as *mut _ };
// Initialized in run_static_initializers
#[no_mangle]
pub static mut gScarecrowSpawnSongPtr: *mut u8_0 =
    0 as *const u8_0 as *mut u8_0;
// Initialized in run_static_initializers
#[no_mangle]
pub static mut D_80131BEC: *mut OcarinaNote =
    0 as *const OcarinaNote as *mut OcarinaNote;
#[no_mangle]
pub static mut sNoteValueIndexMap: [u8_0; 16] =
    [0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 1 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0, 2 as libc::c_int as u8_0,
     2 as libc::c_int as u8_0, 2 as libc::c_int as u8_0,
     5 as libc::c_int as u8_0, 3 as libc::c_int as u8_0,
     3 as libc::c_int as u8_0, 4 as libc::c_int as u8_0,
     4 as libc::c_int as u8_0, 4 as libc::c_int as u8_0];
#[no_mangle]
pub static mut gOcarinaSongNotes: [OcarinaSongInfo; 14] =
    [{
         let mut init =
             OcarinaSongInfo{len: 6 as libc::c_int as u8_0,
                             notesIdx:
                                 [OCARINA_NOTE_A as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_UP as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_LEFT as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_RIGHT as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_LEFT as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_RIGHT as libc::c_int as u8_0,
                                  0, 0],};
         init
     },
     {
         let mut init =
             OcarinaSongInfo{len: 8 as libc::c_int as u8_0,
                             notesIdx:
                                 [OCARINA_NOTE_C_DOWN as libc::c_int as u8_0,
                                  OCARINA_NOTE_A as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_DOWN as libc::c_int as u8_0,
                                  OCARINA_NOTE_A as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_RIGHT as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_DOWN as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_RIGHT as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_DOWN as libc::c_int as
                                      u8_0],};
         init
     },
     {
         let mut init =
             OcarinaSongInfo{len: 5 as libc::c_int as u8_0,
                             notesIdx:
                                 [OCARINA_NOTE_A as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_DOWN as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_RIGHT as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_RIGHT as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_LEFT as libc::c_int as u8_0,
                                  0, 0, 0],};
         init
     },
     {
         let mut init =
             OcarinaSongInfo{len: 6 as libc::c_int as u8_0,
                             notesIdx:
                                 [OCARINA_NOTE_A as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_DOWN as libc::c_int as u8_0,
                                  OCARINA_NOTE_A as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_RIGHT as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_DOWN as libc::c_int as u8_0,
                                  OCARINA_NOTE_A as libc::c_int as u8_0, 0,
                                  0],};
         init
     },
     {
         let mut init =
             OcarinaSongInfo{len: 7 as libc::c_int as u8_0,
                             notesIdx:
                                 [OCARINA_NOTE_C_LEFT as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_RIGHT as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_RIGHT as libc::c_int as u8_0,
                                  OCARINA_NOTE_A as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_LEFT as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_RIGHT as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_DOWN as libc::c_int as u8_0,
                                  0],};
         init
     },
     {
         let mut init =
             OcarinaSongInfo{len: 6 as libc::c_int as u8_0,
                             notesIdx:
                                 [OCARINA_NOTE_C_UP as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_RIGHT as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_UP as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_RIGHT as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_LEFT as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_UP as libc::c_int as u8_0, 0,
                                  0],};
         init
     },
     {
         let mut init =
             OcarinaSongInfo{len: 6 as libc::c_int as u8_0,
                             notesIdx:
                                 [OCARINA_NOTE_C_DOWN as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_RIGHT as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_LEFT as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_DOWN as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_RIGHT as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_LEFT as libc::c_int as u8_0,
                                  0, 0],};
         init
     },
     {
         let mut init =
             OcarinaSongInfo{len: 6 as libc::c_int as u8_0,
                             notesIdx:
                                 [OCARINA_NOTE_C_UP as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_LEFT as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_RIGHT as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_UP as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_LEFT as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_RIGHT as libc::c_int as u8_0,
                                  0, 0],};
         init
     },
     {
         let mut init =
             OcarinaSongInfo{len: 6 as libc::c_int as u8_0,
                             notesIdx:
                                 [OCARINA_NOTE_C_LEFT as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_UP as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_RIGHT as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_LEFT as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_UP as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_RIGHT as libc::c_int as u8_0,
                                  0, 0],};
         init
     },
     {
         let mut init =
             OcarinaSongInfo{len: 6 as libc::c_int as u8_0,
                             notesIdx:
                                 [OCARINA_NOTE_C_RIGHT as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_DOWN as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_UP as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_RIGHT as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_DOWN as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_UP as libc::c_int as u8_0, 0,
                                  0],};
         init
     },
     {
         let mut init =
             OcarinaSongInfo{len: 6 as libc::c_int as u8_0,
                             notesIdx:
                                 [OCARINA_NOTE_C_RIGHT as libc::c_int as u8_0,
                                  OCARINA_NOTE_A as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_DOWN as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_RIGHT as libc::c_int as u8_0,
                                  OCARINA_NOTE_A as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_DOWN as libc::c_int as u8_0,
                                  0, 0],};
         init
     },
     {
         let mut init =
             OcarinaSongInfo{len: 6 as libc::c_int as u8_0,
                             notesIdx:
                                 [OCARINA_NOTE_A as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_DOWN as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_UP as libc::c_int as u8_0,
                                  OCARINA_NOTE_A as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_DOWN as libc::c_int as u8_0,
                                  OCARINA_NOTE_C_UP as libc::c_int as u8_0, 0,
                                  0],};
         init
     },
     {
         let mut init =
             OcarinaSongInfo{len: 8 as libc::c_int as u8_0,
                             notesIdx:
                                 [0 as libc::c_int as u8_0,
                                  0 as libc::c_int as u8_0,
                                  0 as libc::c_int as u8_0,
                                  0 as libc::c_int as u8_0,
                                  0 as libc::c_int as u8_0,
                                  0 as libc::c_int as u8_0,
                                  0 as libc::c_int as u8_0,
                                  0 as libc::c_int as u8_0],};
         init
     },
     {
         let mut init =
             OcarinaSongInfo{len: 0 as libc::c_int as u8_0,
                             notesIdx:
                                 [0 as libc::c_int as u8_0,
                                  0 as libc::c_int as u8_0,
                                  0 as libc::c_int as u8_0,
                                  0 as libc::c_int as u8_0,
                                  0 as libc::c_int as u8_0,
                                  0 as libc::c_int as u8_0,
                                  0 as libc::c_int as u8_0,
                                  0 as libc::c_int as u8_0],};
         init
     }];
// clang-format on
/* *
 * BSS
 */
#[no_mangle]
pub static mut sAudioUpdateStartTime: u32_0 = 0;
// 8016B7A0
#[no_mangle]
pub static mut sAudioUpdateEndTime: u32_0 = 0;
#[no_mangle]
pub static mut D_8016B7A8: f32_0 = 0.;
#[no_mangle]
pub static mut D_8016B7AC: f32_0 = 0.;
#[no_mangle]
pub static mut D_8016B7B0: f32_0 = 0.;
#[no_mangle]
pub static mut D_8016B7B4: f32_0 = 0.;
#[no_mangle]
pub static mut sRiverFreqScaleLerp: FreqLerp =
    FreqLerp{value: 0., target: 0., step: 0., remainingFrames: 0,};
#[no_mangle]
pub static mut sWaterfallFreqScaleLerp: FreqLerp =
    FreqLerp{value: 0., target: 0., step: 0., remainingFrames: 0,};
#[no_mangle]
pub static mut D_8016B7D8: f32_0 = 0.;
#[no_mangle]
pub static mut D_8016B7DC: s8 = 0;
#[no_mangle]
pub static mut D_8016B7E0: f32_0 = 0.;
#[no_mangle]
pub static mut D_8016B7E4: u16_0 = 0;
#[no_mangle]
pub static mut sAudioScrPrtBuf: [C2RustUnnamed_18; 25] =
    [C2RustUnnamed_18{str_0: [0; 5], num: 0,}; 25];
#[no_mangle]
pub static mut D_8016B8B0: u8_0 = 0;
#[no_mangle]
pub static mut D_8016B8B1: u8_0 = 0;
#[no_mangle]
pub static mut D_8016B8B2: u8_0 = 0;
#[no_mangle]
pub static mut D_8016B8B3: u8_0 = 0;
#[no_mangle]
pub static mut sAudioGanonDistVol: u8_0 = 0;
#[no_mangle]
pub static mut sSfxChannelState: [SfxPlayerState; 16] =
    [SfxPlayerState{vol: 0.,
                    freqScale: 0.,
                    reverb: 0,
                    panSigned: 0,
                    stereoBits: 0,
                    filter: 0,
                    unk_0C: 0,}; 16];
#[no_mangle]
pub static mut sBinToStrBuf: [libc::c_char; 32] = [0; 32];
#[no_mangle]
pub static mut D_8016B9D8: u8_0 = 0;
#[no_mangle]
pub static mut sAudioSpecPeakNumNotes: [u8_0; 18] = [0; 18];
#[no_mangle]
pub static mut D_8016B9F2: u8_0 = 0;
#[no_mangle]
pub static mut D_8016B9F3: u8_0 = 0;
#[no_mangle]
pub static mut D_8016B9F4: u8_0 = 0;
#[no_mangle]
pub static mut D_8016B9F6: u16_0 = 0;
#[no_mangle]
pub static mut sPlayingStaff: OcarinaStaff =
    OcarinaStaff{noteIdx: 0, state: 0, pos: 0,};
#[no_mangle]
pub static mut sDisplayedStaff: OcarinaStaff =
    OcarinaStaff{noteIdx: 0, state: 0, pos: 0,};
#[no_mangle]
pub static mut sRecordingStaff: OcarinaStaff =
    OcarinaStaff{noteIdx: 0, state: 0, pos: 0,};
#[no_mangle]
pub static mut D_8016BA04: u32_0 = 0;
#[no_mangle]
pub static mut sCurOcaStick: OcarinaStick = OcarinaStick{x: 0, y: 0,};
#[no_mangle]
pub static mut sCurOcarinaBtnPress: u32_0 = 0;
#[no_mangle]
pub static mut D_8016BA10: u32_0 = 0;
#[no_mangle]
pub static mut sPrevOcarinaBtnPress: u32_0 = 0;
#[no_mangle]
pub static mut D_8016BA18: s32 = 0;
#[no_mangle]
pub static mut D_8016BA1C: s32 = 0;
#[no_mangle]
pub static mut sCurOcarinaSong: [u8_0; 8] = [0; 8];
#[no_mangle]
pub static mut sOcarinaSongAppendPos: u8_0 = 0;
#[no_mangle]
pub static mut sOcarinaHasStartedSong: u8_0 = 0;
#[no_mangle]
pub static mut sOcarinaSongNoteStartIdx: u8_0 = 0;
#[no_mangle]
pub static mut sOcarinaSongCnt: u8_0 = 0;
#[no_mangle]
pub static mut sOcarinaAvailSongs: u16_0 = 0;
#[no_mangle]
pub static mut sStaffPlayingPos: u8_0 = 0;
#[no_mangle]
pub static mut sLearnSongPos: [u16_0; 16] = [0; 16];
#[no_mangle]
pub static mut D_8016BA50: [u16_0; 16] = [0; 16];
#[no_mangle]
pub static mut D_8016BA70: [u16_0; 16] = [0; 16];
#[no_mangle]
pub static mut sLearnSongExpectedNote: [u8_0; 16] = [0; 16];
#[no_mangle]
pub static mut D_8016BAA0: OcarinaNote =
    OcarinaNote{noteIdx: 0,
                unk_01: 0,
                unk_02: 0,
                volume: 0,
                vibrato: 0,
                tone: 0,
                semitone: 0,};
#[no_mangle]
pub static mut sAudioHasMalonBgm: u8_0 = 0;
#[no_mangle]
pub static mut sAudioMalonBgmDist: f32_0 = 0.;
// Start debug bss
#[no_mangle]
pub static mut sDebugPadHold: u32_0 = 0;
#[no_mangle]
pub static mut sDebugPadBtnLast: u32_0 = 0;
#[no_mangle]
pub static mut sDebugPadPress: u32_0 = 0;
#[no_mangle]
pub static mut sAudioUpdateTaskStart: s32 = 0;
#[no_mangle]
pub static mut sAudioUpdateTaskEnd: s32 = 0;
#[no_mangle]
pub unsafe extern "C" fn func_800EC960(mut custom: u8_0) {
    if custom == 0 {
        osSyncPrintf(b"AUDIO : Ocarina Control Assign Normal\n\x00" as
                         *const u8 as *const libc::c_char);
        sOcarinaAllowedBtnMask =
            (0x8000 as libc::c_int | 0x8 as libc::c_int | 0x4 as libc::c_int |
                 0x2 as libc::c_int | 0x1 as libc::c_int) as u32_0;
        sOcarinaABtnMap = 0x8000 as libc::c_int;
        sOcarinaCUPBtnMap = 0x8 as libc::c_int;
        sOcarinaCDownBtnMap = 0x4 as libc::c_int
    } else {
        osSyncPrintf(b"AUDIO : Ocarina Control Assign Custom\n\x00" as
                         *const u8 as *const libc::c_char);
        sOcarinaAllowedBtnMask =
            (0x8000 as libc::c_int | 0x4000 as libc::c_int |
                 0x4 as libc::c_int | 0x2 as libc::c_int | 0x1 as libc::c_int)
                as u32_0;
        sOcarinaABtnMap = 0x4000 as libc::c_int;
        sOcarinaCUPBtnMap = 0x4 as libc::c_int;
        sOcarinaCDownBtnMap = 0x8000 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn Audio_GetOcaInput() {
    let mut inputs: [Input; 4] =
        [Input{cur: OSContPad{button: 0, stick_x: 0, stick_y: 0, errno: 0,},
               prev: OSContPad{button: 0, stick_x: 0, stick_y: 0, errno: 0,},
               press: OSContPad{button: 0, stick_x: 0, stick_y: 0, errno: 0,},
               rel: OSContPad{button: 0, stick_x: 0, stick_y: 0, errno: 0,},};
            4];
    let mut input: *mut Input =
        &mut *inputs.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut Input;
    let mut sp18: u32_0 = 0;
    sp18 = sCurOcarinaBtnPress;
    PadMgr_RequestPadData(&mut gPadMgr, inputs.as_mut_ptr(),
                          0 as libc::c_int);
    sCurOcarinaBtnPress = (*input).cur.button as u32_0;
    sPrevOcarinaBtnPress = sp18;
    sCurOcaStick.x = (*input).rel.stick_x;
    sCurOcaStick.y = (*input).rel.stick_y;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_OcaAdjStick(mut inp: s8) -> f32_0 {
    let mut inpAdj: s8 = 0;
    let mut ret: f32_0 = 0.;
    if inp as libc::c_int > 0x40 as libc::c_int {
        inpAdj = 127 as libc::c_int as s8
    } else if (inp as libc::c_int) < -(0x40 as libc::c_int) {
        inpAdj = -(128 as libc::c_int) as s8
    } else if inp as libc::c_int >= 0 as libc::c_int {
        inpAdj =
            (inp as libc::c_int * 127 as libc::c_int / 64 as libc::c_int) as
                s8
    } else {
        inpAdj =
            (inp as libc::c_int * 128 as libc::c_int / 64 as libc::c_int) as
                s8
    }
    ret =
        gBendPitchTwoSemitonesFrequencies[(inpAdj as libc::c_int +
                                               128 as libc::c_int) as usize];
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_OcaGetPlayingState() -> u8_0 {
    let mut ret: u8_0 = 0;
    if D_80131878 as libc::c_int != 0 as libc::c_int {
        ret = (D_80131878 as libc::c_int - 1 as libc::c_int) as u8_0;
        D_80131878 = 0 as libc::c_int as u8_0
    } else if D_80130F3C != 0 as libc::c_int as libc::c_uint {
        ret = 0xfe as libc::c_int as u8_0
    } else { ret = 0xff as libc::c_int as u8_0 }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_OcaMapNoteValue(mut arg0: u8_0) -> u8_0 {
    let mut temp_v1: u8_0 = 0;
    temp_v1 =
        sNoteValueIndexMap[(arg0 as libc::c_int & 0x3f as libc::c_int) as
                               usize];
    if temp_v1 as libc::c_int == 5 as libc::c_int {
        if arg0 as libc::c_int & 0x80 as libc::c_int != 0 {
            return 2 as libc::c_int as u8_0
        }
        return 3 as libc::c_int as u8_0
    }
    return temp_v1;
}
#[no_mangle]
pub unsafe extern "C" fn func_800ECB7C(mut songIdx: u8_0) {
    let mut savedSongIdx: u8_0 = 0;
    let mut scarecrowSongIdx: u8_0 = 0;
    let mut noteIdx: u8_0 = 0;
    savedSongIdx = 0 as libc::c_int as u8_0;
    scarecrowSongIdx = 0 as libc::c_int as u8_0;
    while (savedSongIdx as libc::c_int) < 8 as libc::c_int &&
              (scarecrowSongIdx as libc::c_int) < 0x10 as libc::c_int {
        let fresh0 = scarecrowSongIdx;
        scarecrowSongIdx = scarecrowSongIdx.wrapping_add(1);
        noteIdx = sOcarinaSongs[songIdx as usize][fresh0 as usize].noteIdx;
        if noteIdx as libc::c_int != 0xff as libc::c_int {
            let fresh1 = savedSongIdx;
            savedSongIdx = savedSongIdx.wrapping_add(1);
            gOcarinaSongNotes[OCARINA_SONG_SCARECROW as libc::c_int as
                                  usize].notesIdx[fresh1 as usize] =
                sNoteValueIndexMap[noteIdx as usize]
        }
    };
}
// start ocarina.
#[no_mangle]
pub unsafe extern "C" fn func_800ECC04(mut flg: u16_0) {
    let mut i: u8_0 = 0;
    if sOcarinaSongs[OCARINA_SONG_SCARECROW as libc::c_int as
                         usize][1 as libc::c_int as usize].volume as
           libc::c_int != 0xff as libc::c_int &&
           flg as libc::c_int & 0xfff as libc::c_int == 0xfff as libc::c_int {
        flg = (flg as libc::c_int | 0x1000 as libc::c_int) as u16_0
    }
    if flg as libc::c_int == 0xcfff as libc::c_int &&
           sOcarinaSongs[OCARINA_SONG_SCARECROW as libc::c_int as
                             usize][1 as libc::c_int as usize].volume as
               libc::c_int != 0xff as libc::c_int {
        flg = 0xdfff as libc::c_int as u16_0
    }
    if flg as libc::c_int == 0xfff as libc::c_int &&
           sOcarinaSongs[OCARINA_SONG_SCARECROW as libc::c_int as
                             usize][1 as libc::c_int as usize].volume as
               libc::c_int != 0xff as libc::c_int {
        flg = 0x1fff as libc::c_int as u16_0
    }
    if flg as libc::c_int != 0xffff as libc::c_int {
        D_80130F3C = (0x80000000 as libc::c_uint).wrapping_add(flg as u32_0);
        sOcarinaSongNoteStartIdx = 0 as libc::c_int as u8_0;
        sOcarinaSongCnt = 0xe as libc::c_int as u8_0;
        if flg as libc::c_int != 0xa000 as libc::c_int {
            sOcarinaSongCnt = sOcarinaSongCnt.wrapping_sub(1)
        }
        sOcarinaAvailSongs =
            (flg as libc::c_int & 0x3fff as libc::c_int) as u16_0;
        D_8013187C = 8 as libc::c_int as u8_0;
        sOcarinaHasStartedSong = 0 as libc::c_int as u8_0;
        D_80131878 = 0 as libc::c_int as u8_0;
        sStaffPlayingPos = 0 as libc::c_int as u8_0;
        sPlayingStaff.state = Audio_OcaGetPlayingState();
        sOcarinaInpEnabled = 1 as libc::c_int as u8_0;
        D_80130F4C = 0 as libc::c_int as u16_0;
        i = 0 as libc::c_int as u8_0;
        while (i as libc::c_int) < 0xe as libc::c_int {
            sLearnSongPos[i as usize] = 0 as libc::c_int as u16_0;
            D_8016BA50[i as usize] = 0 as libc::c_int as u16_0;
            D_8016BA70[i as usize] = 0 as libc::c_int as u16_0;
            sLearnSongExpectedNote[i as usize] = 0 as libc::c_int as u8_0;
            i = i.wrapping_add(1)
        }
        if flg as libc::c_int & 0x8000 as libc::c_int != 0 {
            D_8013187C = 0 as libc::c_int as u8_0
        }
        if flg as libc::c_int & 0x4000 as libc::c_int != 0 {
            sOcarinaSongAppendPos = 0 as libc::c_int as u8_0
        }
        if flg as libc::c_int & 0xd000 as libc::c_int != 0 {
            func_800ECB7C(OCARINA_SONG_SCARECROW as libc::c_int as u8_0);
        }
    } else {
        D_80130F3C = 0 as libc::c_int as u32_0;
        sOcarinaInpEnabled = 0 as libc::c_int as u8_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800ECDBC() {
    if sCurOcarinaBtnVal as libc::c_int != 0xff as libc::c_int &&
           sOcarinaHasStartedSong as libc::c_int == 0 as libc::c_int {
        sOcarinaHasStartedSong = 1 as libc::c_int as u8_0;
        sLearnSongLastBtn = 0xff as libc::c_int as u8_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800ECDF8() {
    let mut sh: u16_0 = 0;
    let mut pad: u16_0 = 0;
    let mut inputChanged: u8_0 = 0 as libc::c_int as u8_0;
    let mut pad2: u16_0 = 0;
    let mut sp57: s8 = 0 as libc::c_int as s8;
    let mut i: u8_0 = 0;
    let mut prevNote: *mut OcarinaNote = 0 as *mut OcarinaNote;
    let mut note: *mut OcarinaNote = 0 as *mut OcarinaNote;
    func_800ECDBC();
    if sOcarinaHasStartedSong != 0 {
        if (if (D_80130F2C as libc::c_int) < 0 as libc::c_int {
                -(D_80130F2C as libc::c_int)
            } else { D_80130F2C as libc::c_int }) >= 0x15 as libc::c_int {
            D_80130F3C = 0 as libc::c_int as u32_0;
            return
        }
        // clang-format off
        if sPrevOcarinaNoteVal as libc::c_int ==
               sCurOcarinaBtnVal as libc::c_int ||
               sCurOcarinaBtnVal as libc::c_int == 0xff as libc::c_int {
            inputChanged = 1 as libc::c_int as u8_0
        }
        // clang-format on
        i = sOcarinaSongNoteStartIdx;
        while (i as libc::c_int) < sOcarinaSongCnt as libc::c_int {
            sh = ((1 as libc::c_int) << i as libc::c_int) as u16_0;
            if sOcarinaAvailSongs as libc::c_int & sh as libc::c_int != 0 {
                D_8016BA50[i as usize] =
                    (D_8016BA70[i as usize] as libc::c_int +
                         0x12 as libc::c_int) as u16_0;
                if inputChanged != 0 {
                    // (pointless if check, this is always true)
                    if D_8016BA50[i as usize] as libc::c_int >=
                           D_8016BA70[i as usize] as libc::c_int -
                               0x12 as libc::c_int &&
                           D_8016BA50[i as usize] as libc::c_int >=
                               D_8016BA70[i as usize] as libc::c_int +
                                   0x12 as libc::c_int &&
                           sOcarinaSongs[i as
                                             usize][sLearnSongPos[i as usize]
                                                        as usize].unk_02 as
                               libc::c_int == 0 as libc::c_int &&
                           sLearnSongLastBtn as libc::c_int ==
                               sLearnSongExpectedNote[i as usize] as
                                   libc::c_int {
                        D_80131878 =
                            (i as libc::c_int + 1 as libc::c_int) as u8_0;
                        sOcarinaInpEnabled = 0 as libc::c_int as u8_0;
                        D_80130F3C = 0 as libc::c_int as u32_0
                    }
                } else if D_8016BA50[i as usize] as libc::c_int >=
                              D_8016BA70[i as usize] as libc::c_int -
                                  0x12 as libc::c_int {
                    if sLearnSongLastBtn as libc::c_int != 0xff as libc::c_int
                       {
                        if sLearnSongLastBtn as libc::c_int ==
                               sLearnSongExpectedNote[i as usize] as
                                   libc::c_int {
                            if i as libc::c_int == 12 as libc::c_int {
                                D_8016BA50[i as usize] =
                                    0 as libc::c_int as u16_0
                            }
                        } else {
                            sOcarinaAvailSongs =
                                (sOcarinaAvailSongs as libc::c_int ^
                                     sh as libc::c_int) as u16_0
                        }
                    }
                    prevNote =
                        &mut *(*sOcarinaSongs.as_mut_ptr().offset(i as
                                                                      isize)).as_mut_ptr().offset(*sLearnSongPos.as_mut_ptr().offset(i
                                                                                                                                         as
                                                                                                                                         isize)
                                                                                                      as
                                                                                                      isize)
                            as *mut OcarinaNote;
                    let ref mut fresh2 =
                        *sLearnSongPos.as_mut_ptr().offset(i as isize);
                    *fresh2 = (*fresh2).wrapping_add(1);
                    note =
                        &mut *(*sOcarinaSongs.as_mut_ptr().offset(i as
                                                                      isize)).as_mut_ptr().offset(*fresh2
                                                                                                      as
                                                                                                      isize)
                            as *mut OcarinaNote;
                    D_8016BA70[i as usize] = (*prevNote).unk_02;
                    sLearnSongExpectedNote[i as usize] = (*prevNote).noteIdx;
                    if sCurOcarinaBtnVal as libc::c_int !=
                           sLearnSongExpectedNote[i as usize] as libc::c_int {
                        sOcarinaAvailSongs =
                            (sOcarinaAvailSongs as libc::c_int ^
                                 sh as libc::c_int) as u16_0
                    }
                    while (*prevNote).noteIdx as libc::c_int ==
                              (*note).noteIdx as libc::c_int ||
                              (*note).noteIdx as libc::c_int ==
                                  OCARINA_NOTE_INVALID as libc::c_int &&
                                  (*note).unk_02 as libc::c_int !=
                                      0 as libc::c_int {
                        D_8016BA70[i as usize] =
                            (D_8016BA70[i as usize] as libc::c_int +
                                 (*note).unk_02 as libc::c_int) as u16_0;
                        prevNote =
                            &mut *(*sOcarinaSongs.as_mut_ptr().offset(i as
                                                                          isize)).as_mut_ptr().offset(*sLearnSongPos.as_mut_ptr().offset(i
                                                                                                                                             as
                                                                                                                                             isize)
                                                                                                          as
                                                                                                          isize)
                                as *mut OcarinaNote;
                        note =
                            &mut *(*sOcarinaSongs.as_mut_ptr().offset(i as
                                                                          isize)).as_mut_ptr().offset((*sLearnSongPos.as_mut_ptr().offset(i
                                                                                                                                              as
                                                                                                                                              isize)
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           +
                                                                                                           1
                                                                                                               as
                                                                                                               libc::c_int)
                                                                                                          as
                                                                                                          isize)
                                as *mut OcarinaNote;
                        sLearnSongPos[i as usize] =
                            sLearnSongPos[i as usize].wrapping_add(1)
                    }
                } else if (D_8016BA50[i as usize] as libc::c_int) <
                              0xa as libc::c_int {
                    sp57 = -(1 as libc::c_int) as s8;
                    D_8016BA50[i as usize] = 0 as libc::c_int as u16_0;
                    sLearnSongLastBtn = sCurOcarinaBtnVal
                } else {
                    sOcarinaAvailSongs =
                        (sOcarinaAvailSongs as libc::c_int ^
                             sh as libc::c_int) as u16_0
                }
            }
            if sOcarinaAvailSongs as libc::c_int == 0 as libc::c_int &&
                   sStaffPlayingPos as libc::c_int >=
                       D_8013187C as libc::c_int {
                sOcarinaInpEnabled = 0 as libc::c_int as u8_0;
                if D_80130F3C & 0x4000 as libc::c_int as libc::c_uint !=
                       0 as libc::c_int as libc::c_uint &&
                       sCurOcarinaBtnVal as libc::c_int ==
                           sOcarinaSongs[i as
                                             usize][0 as libc::c_int as
                                                        usize].noteIdx as
                               libc::c_int {
                    D_80130F4C = D_80130F3C as u16_0
                }
                D_80130F3C = 0 as libc::c_int as u32_0;
                return
            }
            i = i.wrapping_add(1)
        }
        if inputChanged == 0 {
            sLearnSongLastBtn = sCurOcarinaBtnVal;
            sStaffPlayingPos =
                (sStaffPlayingPos as libc::c_int +
                     (sp57 as libc::c_int + 1 as libc::c_int)) as u8_0
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800ED200() {
    let mut temp_v0: u32_0 = 0;
    let mut i: u8_0 = 0;
    let mut j: u8_0 = 0;
    let mut k: u8_0 = 0;
    if sCurOcarinaBtnPress & 0x20 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint &&
           sCurOcarinaBtnPress & sOcarinaAllowedBtnMask !=
               0 as libc::c_int as libc::c_uint {
        func_800ECC04(D_80130F3C as u16_0);
        return
    }
    func_800ECDBC();
    if sOcarinaHasStartedSong != 0 {
        if sPrevOcarinaNoteVal as libc::c_int !=
               sCurOcarinaBtnVal as libc::c_int &&
               sCurOcarinaBtnVal as libc::c_int != 0xff as libc::c_int {
            sStaffPlayingPos = sStaffPlayingPos.wrapping_add(1);
            if sStaffPlayingPos as libc::c_int >= 9 as libc::c_int {
                sStaffPlayingPos = 1 as libc::c_int as u8_0
            }
            if sOcarinaSongAppendPos as libc::c_int == 8 as libc::c_int {
                i = 0 as libc::c_int as u8_0;
                while (i as libc::c_int) < 7 as libc::c_int {
                    sCurOcarinaSong[i as usize] =
                        sCurOcarinaSong[(i as libc::c_int + 1 as libc::c_int)
                                            as usize];
                    i = i.wrapping_add(1)
                }
            } else {
                sOcarinaSongAppendPos = sOcarinaSongAppendPos.wrapping_add(1)
            }
            if (if (D_80130F2C as libc::c_int) < 0 as libc::c_int {
                    -(D_80130F2C as libc::c_int)
                } else { D_80130F2C as libc::c_int }) >= 0x15 as libc::c_int {
                sCurOcarinaSong[(sOcarinaSongAppendPos as libc::c_int -
                                     1 as libc::c_int) as usize] =
                    0xff as libc::c_int as u8_0
            } else {
                sCurOcarinaSong[(sOcarinaSongAppendPos as libc::c_int -
                                     1 as libc::c_int) as usize] =
                    sCurOcarinaBtnVal
            }
            i = sOcarinaSongNoteStartIdx;
            while (i as libc::c_int) < sOcarinaSongCnt as libc::c_int {
                if sOcarinaAvailSongs as libc::c_int &
                       ((1 as libc::c_int) << i as libc::c_int) as u16_0 as
                           libc::c_int != 0 {
                    j = 0 as libc::c_int as u8_0;
                    k = 0 as libc::c_int as u8_0;
                    while (j as libc::c_int) <
                              gOcarinaSongNotes[i as usize].len as libc::c_int
                              && k as libc::c_int == 0 as libc::c_int &&
                              sOcarinaSongAppendPos as libc::c_int >=
                                  gOcarinaSongNotes[i as usize].len as
                                      libc::c_int {
                        temp_v0 =
                            sCurOcarinaSong[(sOcarinaSongAppendPos as
                                                 libc::c_int -
                                                 gOcarinaSongNotes[i as
                                                                       usize].len
                                                     as libc::c_int +
                                                 j as libc::c_int) as usize]
                                as u32_0;
                        if temp_v0 ==
                               sOcarinaNoteValues[gOcarinaSongNotes[i as
                                                                        usize].notesIdx[j
                                                                                            as
                                                                                            usize]
                                                      as usize] as
                                   libc::c_uint {
                            j = j.wrapping_add(1)
                        } else { k = k.wrapping_add(1) }
                    }
                    if j as libc::c_int ==
                           gOcarinaSongNotes[i as usize].len as libc::c_int {
                        D_80131878 =
                            (i as libc::c_int + 1 as libc::c_int) as u8_0;
                        sOcarinaInpEnabled = 0 as libc::c_int as u8_0;
                        D_80130F3C = 0 as libc::c_int as u32_0
                    }
                }
                i = i.wrapping_add(1)
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800ED458(mut arg0: s32) {
    let mut phi_v1_2: u32_0 = 0;
    if D_80130F3C != 0 as libc::c_int as libc::c_uint &&
           D_80131880 as libc::c_int != 0 as libc::c_int {
        D_80131880 = D_80131880.wrapping_sub(1);
        return
    }
    if D_8016BA10 == 0 as libc::c_int as libc::c_uint ||
           D_8016BA10 & sOcarinaAllowedBtnMask !=
               sCurOcarinaBtnPress & sOcarinaAllowedBtnMask {
        D_8016BA10 = 0 as libc::c_int as u32_0;
        sCurOcarinaBtnVal = 0xff as libc::c_int as u8_0;
        sCurOcarinaBtnIdx = 0xff as libc::c_int as u8_0;
        phi_v1_2 =
            sCurOcarinaBtnPress & sOcarinaAllowedBtnMask &
                (sPrevOcarinaBtnPress & sOcarinaAllowedBtnMask);
        if D_8016BA18 as libc::c_uint & phi_v1_2 == 0 &&
               sCurOcarinaBtnPress != 0 as libc::c_int as libc::c_uint {
            D_8016BA18 = sCurOcarinaBtnPress as s32
        } else { D_8016BA18 = (D_8016BA18 as libc::c_uint & phi_v1_2) as s32 }
        if D_8016BA18 & sOcarinaABtnMap != 0 {
            osSyncPrintf(b"Presss NA_KEY_D4 %08x\n\x00" as *const u8 as
                             *const libc::c_char, sOcarinaABtnMap);
            sCurOcarinaBtnVal = 2 as libc::c_int as u8_0;
            sCurOcarinaBtnIdx = 0 as libc::c_int as u8_0
        } else if D_8016BA18 & sOcarinaCDownBtnMap != 0 {
            osSyncPrintf(b"Presss NA_KEY_F4 %08x\n\x00" as *const u8 as
                             *const libc::c_char, sOcarinaCDownBtnMap);
            sCurOcarinaBtnVal = 5 as libc::c_int as u8_0;
            sCurOcarinaBtnIdx = 1 as libc::c_int as u8_0
        } else if D_8016BA18 & 1 as libc::c_int != 0 {
            osSyncPrintf(b"Presss NA_KEY_A4 %08x\n\x00" as *const u8 as
                             *const libc::c_char, 1 as libc::c_int);
            sCurOcarinaBtnVal = 9 as libc::c_int as u8_0;
            sCurOcarinaBtnIdx = 2 as libc::c_int as u8_0
        } else if D_8016BA18 & 2 as libc::c_int != 0 {
            osSyncPrintf(b"Presss NA_KEY_B4 %08x\n\x00" as *const u8 as
                             *const libc::c_char, 2 as libc::c_int);
            sCurOcarinaBtnVal = 0xb as libc::c_int as u8_0;
            sCurOcarinaBtnIdx = 3 as libc::c_int as u8_0
        } else if D_8016BA18 & sOcarinaCUPBtnMap != 0 {
            osSyncPrintf(b"Presss NA_KEY_D5 %08x\n\x00" as *const u8 as
                             *const libc::c_char, sOcarinaCUPBtnMap);
            sCurOcarinaBtnVal = 0xe as libc::c_int as u8_0;
            sCurOcarinaBtnIdx = 4 as libc::c_int as u8_0
        }
        if sCurOcarinaBtnVal as libc::c_int != 0xff as libc::c_int &&
               sCurOcarinaBtnPress & 0x10 as libc::c_int as libc::c_uint != 0
               && sRecordingState as libc::c_int != 2 as libc::c_int {
            sCurOcarinaBtnIdx =
                (sCurOcarinaBtnIdx as libc::c_int + 0x80 as libc::c_int) as
                    u8_0;
            sCurOcarinaBtnVal = sCurOcarinaBtnVal.wrapping_add(1)
        }
        if sCurOcarinaBtnVal as libc::c_int != 0xff as libc::c_int &&
               sCurOcarinaBtnPress & 0x2000 as libc::c_int as libc::c_uint !=
                   0 && sRecordingState as libc::c_int != 2 as libc::c_int {
            sCurOcarinaBtnIdx =
                (sCurOcarinaBtnIdx as libc::c_int + 0x40 as libc::c_int) as
                    u8_0;
            sCurOcarinaBtnVal = sCurOcarinaBtnVal.wrapping_sub(1)
        }
        if sRecordingState as libc::c_int != 2 as libc::c_int {
            D_80130F2C = sCurOcaStick.y;
            D_80130F24 = Audio_OcaAdjStick(D_80130F2C);
            D_80130F34 =
                ((if (sCurOcaStick.x as libc::c_int) < 0 as libc::c_int {
                      -(sCurOcaStick.x as libc::c_int)
                  } else { sCurOcaStick.x as libc::c_int }) >>
                     2 as libc::c_int) as s8;
            Audio_QueueCmdS8(((0x6 as libc::c_int) << 24 as libc::c_int |
                                  (SEQ_PLAYER_SFX as libc::c_int) <<
                                      16 as libc::c_int |
                                  0xd06 as libc::c_int) as u32_0, D_80130F34);
        } else { D_80130F2C = 0 as libc::c_int as s8; D_80130F24 = 1.0f32 }
        if sCurOcarinaBtnVal as libc::c_int != 0xff as libc::c_int &&
               sPrevOcarinaNoteVal as libc::c_int !=
                   sCurOcarinaBtnVal as libc::c_int {
            Audio_QueueCmdS8(((0x6 as libc::c_int) << 24 as libc::c_int |
                                  (SEQ_PLAYER_SFX as libc::c_int) <<
                                      16 as libc::c_int |
                                  0xd07 as libc::c_int) as u32_0,
                             (D_80130F10 as libc::c_int - 1 as libc::c_int) as
                                 s8);
            Audio_QueueCmdS8(((0x6 as libc::c_int) << 24 as libc::c_int |
                                  (SEQ_PLAYER_SFX as libc::c_int) <<
                                      16 as libc::c_int |
                                  0xd05 as libc::c_int) as u32_0,
                             sCurOcarinaBtnVal as s8);
            Audio_PlaySoundGeneral(0x5800 as libc::c_int as u16_0,
                                   &mut D_801333D4, 4 as libc::c_int as u8_0,
                                   &mut D_80130F24, &mut D_80130F28,
                                   &mut D_801333E8);
        } else if sPrevOcarinaNoteVal as libc::c_int != 0xff as libc::c_int &&
                      sCurOcarinaBtnVal as libc::c_int == 0xff as libc::c_int
         {
            Audio_StopSfxById(0x5800 as libc::c_int as u32_0);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800ED848(mut inputEnabled: u8_0) {
    sOcarinaInpEnabled = inputEnabled;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_OcaSetInstrument(mut arg0: u8_0) {
    if D_80130F10 as libc::c_int == arg0 as libc::c_int { return }
    Audio_QueueSeqCmd(0x80000000 as libc::c_uint |
                          ((SEQ_PLAYER_SFX as libc::c_int as u8_0 as
                                libc::c_int) << 24 as libc::c_int) as
                              libc::c_uint |
                          ((1 as libc::c_int as u8_0 as libc::c_int) <<
                               16 as libc::c_int) as libc::c_uint |
                          ((13 as libc::c_int as u8_0 as libc::c_int) <<
                               8 as libc::c_int) as libc::c_uint |
                          arg0 as libc::c_uint);
    D_80130F10 = arg0 as s8;
    if arg0 as libc::c_int == 0 as libc::c_int {
        sCurOcarinaBtnPress = 0 as libc::c_int as u32_0;
        sPrevOcarinaBtnPress = 0 as libc::c_int as u32_0;
        D_8016BA18 = 0 as libc::c_int;
        D_8016BA10 = 0xffff as libc::c_int as u32_0;
        func_800ED458(0 as libc::c_int);
        Audio_StopSfxById(0x5800 as libc::c_int as u32_0);
        Audio_SetSoundBanksMute(0 as libc::c_int as u16_0);
        sPlaybackState = 0 as libc::c_int as u8_0;
        sStaffPlaybackPos = 0 as libc::c_int as u16_0;
        sOcarinaInpEnabled = 0 as libc::c_int as u8_0;
        D_80130F3C = 0 as libc::c_int as u32_0;
        Audio_ClearBGMMute(13 as libc::c_int as u8_0);
    } else {
        sCurOcarinaBtnPress = 0 as libc::c_int as u32_0;
        Audio_GetOcaInput();
        D_8016BA10 = sCurOcarinaBtnPress;
        Audio_QueueSeqCmdMute(13 as libc::c_int as u8_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Audio_OcaSetSongPlayback(mut songIdxPlusOne: s8,
                                                  mut playbackState: s8) {
    if songIdxPlusOne as libc::c_int == 0 as libc::c_int {
        sPlaybackState = 0 as libc::c_int as u8_0;
        Audio_StopSfxById(0x5800 as libc::c_int as u32_0);
        return
    }
    if (songIdxPlusOne as libc::c_int) < 0xf as libc::c_int {
        sPlaybackSong =
            sOcarinaSongs[(songIdxPlusOne as libc::c_int - 1 as libc::c_int)
                              as usize].as_mut_ptr()
    } else { sPlaybackSong = sPierresSong.as_mut_ptr() }
    sPlaybackState = playbackState as u8_0;
    sNotePlaybackTimer = 0 as libc::c_int as u32_0;
    sDisplayedNoteValue = 0xff as libc::c_int as u8_0;
    sPlaybackNotePos = 0 as libc::c_int as u16_0;
    sStaffPlaybackPos = 0 as libc::c_int as u16_0;
    while (*sPlaybackSong.offset(sPlaybackNotePos as isize)).noteIdx as
              libc::c_int == OCARINA_NOTE_INVALID as libc::c_int {
        sPlaybackNotePos = sPlaybackNotePos.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn Audio_OcaPlayback() {
    let mut noteTimerStep: u32_0 = 0;
    let mut nextNoteTimerStep: u32_0 = 0;
    if sPlaybackState as libc::c_int != 0 as libc::c_int {
        if sStaffPlaybackPos as libc::c_int == 0 as libc::c_int {
            noteTimerStep = 3 as libc::c_int as u32_0
        } else {
            noteTimerStep =
                D_8016BA04.wrapping_sub(D_80130F68 as libc::c_uint)
        }
        if noteTimerStep < sNotePlaybackTimer {
            sNotePlaybackTimer =
                (sNotePlaybackTimer as
                     libc::c_uint).wrapping_sub(noteTimerStep) as u32_0 as
                    u32_0
        } else {
            nextNoteTimerStep =
                noteTimerStep.wrapping_sub(sNotePlaybackTimer);
            sNotePlaybackTimer = 0 as libc::c_int as u32_0
        }
        if sNotePlaybackTimer == 0 as libc::c_int as libc::c_uint {
            sNotePlaybackTimer =
                (*sPlaybackSong.offset(sPlaybackNotePos as isize)).unk_02 as
                    u32_0;
            if sPlaybackNotePos as libc::c_int == 1 as libc::c_int {
                sNotePlaybackTimer = sNotePlaybackTimer.wrapping_add(1)
            }
            if sNotePlaybackTimer == 0 as libc::c_int as libc::c_uint {
                sPlaybackState = sPlaybackState.wrapping_sub(1);
                if sPlaybackState as libc::c_int != 0 as libc::c_int {
                    sPlaybackNotePos = 0 as libc::c_int as u16_0;
                    sStaffPlaybackPos = 0 as libc::c_int as u16_0;
                    sDisplayedNoteValue = 0xff as libc::c_int as u8_0
                } else { Audio_StopSfxById(0x5800 as libc::c_int as u32_0); }
                return
            } else {
                sNotePlaybackTimer =
                    (sNotePlaybackTimer as
                         libc::c_uint).wrapping_sub(nextNoteTimerStep) as
                        u32_0 as u32_0
            }
            if sNotePlaybackVolume as libc::c_int !=
                   (*sPlaybackSong.offset(sPlaybackNotePos as isize)).volume
                       as libc::c_int {
                sNotePlaybackVolume =
                    (*sPlaybackSong.offset(sPlaybackNotePos as isize)).volume;
                sNormalizedNotePlaybackVolume =
                    sNotePlaybackVolume as libc::c_int as libc::c_float /
                        127.0f32
            }
            if sNotePlaybackVibrato as libc::c_int !=
                   (*sPlaybackSong.offset(sPlaybackNotePos as isize)).vibrato
                       as libc::c_int {
                sNotePlaybackVibrato =
                    (*sPlaybackSong.offset(sPlaybackNotePos as
                                               isize)).vibrato;
                Audio_QueueCmdS8(((0x6 as libc::c_int) << 24 as libc::c_int |
                                      (SEQ_PLAYER_SFX as libc::c_int) <<
                                          16 as libc::c_int |
                                      0xd06 as libc::c_int) as u32_0,
                                 sNotePlaybackVibrato as s8);
            }
            if sNotePlaybackTone as libc::c_int !=
                   (*sPlaybackSong.offset(sPlaybackNotePos as isize)).tone as
                       libc::c_int {
                sNotePlaybackTone =
                    (*sPlaybackSong.offset(sPlaybackNotePos as isize)).tone;
                sNormalizedNotePlaybackTone =
                    Audio_OcaAdjStick(sNotePlaybackTone)
            }
            if (*sPlaybackSong.offset(sPlaybackNotePos as isize)).volume as
                   libc::c_int ==
                   (*sPlaybackSong.offset((sPlaybackNotePos as libc::c_int -
                                               1 as libc::c_int) as
                                              isize)).volume as libc::c_int &&
                   (*sPlaybackSong.offset(sPlaybackNotePos as isize)).vibrato
                       as libc::c_int ==
                       (*sPlaybackSong.offset((sPlaybackNotePos as libc::c_int
                                                   - 1 as libc::c_int) as
                                                  isize)).vibrato as
                           libc::c_int &&
                   (*sPlaybackSong.offset(sPlaybackNotePos as isize)).tone as
                       libc::c_int ==
                       (*sPlaybackSong.offset((sPlaybackNotePos as libc::c_int
                                                   - 1 as libc::c_int) as
                                                  isize)).tone as libc::c_int
               {
                sDisplayedNoteValue = 0xfe as libc::c_int as u8_0
            }
            if sDisplayedNoteValue as libc::c_int !=
                   (*sPlaybackSong.offset(sPlaybackNotePos as isize)).noteIdx
                       as libc::c_int {
                let mut tmp: u8_0 =
                    (*sPlaybackSong.offset(sPlaybackNotePos as
                                               isize)).noteIdx;
                if tmp as libc::c_int == 0xa as libc::c_int {
                    sDisplayedNoteValue =
                        (tmp as libc::c_int +
                             (*sPlaybackSong.offset(sPlaybackNotePos as
                                                        isize)).semitone as
                                 libc::c_int) as u8_0
                } else { sDisplayedNoteValue = tmp }
                if sDisplayedNoteValue as libc::c_int != 0xff as libc::c_int {
                    sStaffPlaybackPos = sStaffPlaybackPos.wrapping_add(1);
                    Audio_QueueCmdS8(((0x6 as libc::c_int) <<
                                          24 as libc::c_int |
                                          (SEQ_PLAYER_SFX as libc::c_int) <<
                                              16 as libc::c_int |
                                          0xd07 as libc::c_int) as u32_0,
                                     (D_80130F10 as libc::c_int -
                                          1 as libc::c_int) as s8);
                    Audio_QueueCmdS8(((0x6 as libc::c_int) <<
                                          24 as libc::c_int |
                                          (SEQ_PLAYER_SFX as libc::c_int) <<
                                              16 as libc::c_int |
                                          0xd05 as libc::c_int) as u32_0,
                                     (sDisplayedNoteValue as libc::c_int &
                                          0x3f as libc::c_int) as s8);
                    Audio_PlaySoundGeneral(0x5800 as libc::c_int as u16_0,
                                           &mut D_801333D4,
                                           4 as libc::c_int as u8_0,
                                           &mut sNormalizedNotePlaybackTone,
                                           &mut sNormalizedNotePlaybackVolume,
                                           &mut D_801333E8);
                } else { Audio_StopSfxById(0x5800 as libc::c_int as u32_0); }
            }
            sPlaybackNotePos = sPlaybackNotePos.wrapping_add(1)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800EDD68(mut arg0: u8_0) {
    let mut i: u16_0 = 0;
    let mut i2: u16_0 = 0;
    let mut pad: u16_0 = 0;
    let mut lastNote: u8_0 = 0;
    let mut note: *mut OcarinaNote = 0 as *mut OcarinaNote;
    let mut j: u8_0 = 0;
    let mut k: u8_0 = 0;
    let mut t: s32 = 0;
    let mut song: *mut OcarinaNote = 0 as *mut OcarinaNote;
    if sRecordingState as libc::c_int == 1 as libc::c_int {
        song = gScarecrowCustomSongPtr
    } else { song = D_80131BEC }
    (*song.offset(sRecordSongPos as isize)).noteIdx = D_80131864;
    (*song.offset(sRecordSongPos as isize)).unk_02 =
        D_8016BA04.wrapping_sub(D_80131860) as u16_0;
    (*song.offset(sRecordSongPos as isize)).volume = D_80131868;
    (*song.offset(sRecordSongPos as isize)).vibrato = D_8013186C;
    (*song.offset(sRecordSongPos as isize)).tone = D_80131870;
    (*song.offset(sRecordSongPos as isize)).semitone =
        (D_80131874 as libc::c_int & 0xc0 as libc::c_int) as u8_0;
    D_80131864 = sCurOcarinaBtnVal;
    D_80131868 = D_80130F30 as u8_0;
    D_8013186C = D_80130F34 as u8_0;
    D_80131870 = D_80130F2C;
    D_80131874 = sCurOcarinaBtnIdx;
    sRecordSongPos = sRecordSongPos.wrapping_add(1);
    if sRecordSongPos as libc::c_int != 107 as libc::c_int &&
           arg0 as libc::c_int == 0 as libc::c_int {
        return
    }
    i = sRecordSongPos as u16_0;
    lastNote = 0xff as libc::c_int as u8_0;
    while i as libc::c_int != 0 as libc::c_int &&
              lastNote as libc::c_int == 0xff as libc::c_int {
        i = i.wrapping_sub(1);
        lastNote = (*song.offset(i as isize)).noteIdx
    }
    if sRecordSongPos as libc::c_int != i as libc::c_int + 1 as libc::c_int {
        sRecordSongPos = (i as libc::c_int + 2 as libc::c_int) as u8_0;
        (*song.offset((sRecordSongPos as libc::c_int - 1 as libc::c_int) as
                          isize)).unk_02 = 0 as libc::c_int as u16_0
    }
    (*song.offset(sRecordSongPos as isize)).unk_02 =
        0 as libc::c_int as u16_0;
    if sRecordingState as libc::c_int == 2 as libc::c_int {
        if sStaffPlayingPos as libc::c_int >= 8 as libc::c_int {
            i = 0 as libc::c_int as u16_0;
            while (i as libc::c_int) < sRecordSongPos as libc::c_int {
                *song.offset(i as isize) =
                    *song.offset((i as libc::c_int + 1 as libc::c_int) as
                                     isize);
                i = i.wrapping_add(1)
            }
            func_800ECB7C(OCARINA_SONG_MEMORY_GAME as libc::c_int as u8_0);
            i = 0 as libc::c_int as u16_0;
            while (i as libc::c_int) < OCARINA_SONG_SCARECROW as libc::c_int {
                j = 0 as libc::c_int as u8_0;
                while (j as libc::c_int) <
                          9 as libc::c_int -
                              gOcarinaSongNotes[i as usize].len as libc::c_int
                      {
                    k = 0 as libc::c_int as u8_0;
                    while (k as libc::c_int) <
                              gOcarinaSongNotes[i as usize].len as libc::c_int
                              &&
                              (k as libc::c_int + j as libc::c_int) <
                                  8 as libc::c_int &&
                              gOcarinaSongNotes[i as
                                                    usize].notesIdx[k as
                                                                        usize]
                                  as libc::c_int ==
                                  gOcarinaSongNotes[OCARINA_SONG_SCARECROW as
                                                        libc::c_int as
                                                        usize].notesIdx[(k as
                                                                             libc::c_int
                                                                             +
                                                                             j
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            usize]
                                      as libc::c_int {
                        k = k.wrapping_add(1)
                    }
                    if k as libc::c_int ==
                           gOcarinaSongNotes[i as usize].len as libc::c_int {
                        sRecordingState = 0xff as libc::c_int as u8_0;
                        sOcarinaSongs[OCARINA_SONG_SCARECROW as libc::c_int as
                                          usize][1 as libc::c_int as
                                                     usize].volume =
                            0xff as libc::c_int as u8_0;
                        return
                    }
                    j = j.wrapping_add(1)
                }
                i = i.wrapping_add(1)
            }
            i = 1 as libc::c_int as u16_0;
            while (i as libc::c_int) < 8 as libc::c_int {
                if gOcarinaSongNotes[OCARINA_SONG_SCARECROW as libc::c_int as
                                         usize].notesIdx[0 as libc::c_int as
                                                             usize] as
                       libc::c_int !=
                       gOcarinaSongNotes[OCARINA_SONG_SCARECROW as libc::c_int
                                             as usize].notesIdx[i as usize] as
                           libc::c_int {
                    i = 9 as libc::c_int as u16_0
                } else { i = i.wrapping_add(1) }
            }
            if i as libc::c_int == 8 as libc::c_int {
                sRecordingState = 0xff as libc::c_int as u8_0;
                sOcarinaSongs[OCARINA_SONG_SCARECROW as libc::c_int as
                                  usize][1 as libc::c_int as usize].volume =
                    0xff as libc::c_int as u8_0;
                return
            }
            i = 0 as libc::c_int as u16_0;
            while (i as libc::c_int) < sRecordSongPos as libc::c_int {
                sOcarinaSongs[OCARINA_SONG_SCARECROW as libc::c_int as
                                  usize][i as usize] =
                    sOcarinaSongs[OCARINA_SONG_MEMORY_GAME as libc::c_int as
                                      usize][i as usize];
                i = i.wrapping_add(1)
            }
            sOcarinaInpEnabled = 0 as libc::c_int as u8_0
        } else {
            sOcarinaSongs[OCARINA_SONG_SCARECROW as libc::c_int as
                              usize][1 as libc::c_int as usize].volume =
                0xff as libc::c_int as u8_0
        }
    }
    sRecordingState = 0 as libc::c_int as u8_0;
}
// start custom song?
/* *
 * recordingState = 1, start long scarecrows song
 * recordingState = 0, end
 * recordingState = 2, also scarecrows song
 */
#[no_mangle]
pub unsafe extern "C" fn Audio_OcaSetRecordingState(mut recordingState:
                                                        u8_0) {
    if recordingState as u32_0 == sRecordingState as libc::c_uint { return }
    if recordingState as libc::c_int != 0 as libc::c_int {
        D_80131860 = D_8016BA04;
        D_80131864 = 0xff as libc::c_int as u8_0;
        D_80131868 = 0x57 as libc::c_int as u8_0;
        D_8013186C = 0 as libc::c_int as u8_0;
        D_80131870 = 0 as libc::c_int as s8;
        D_80131874 = 0 as libc::c_int as u8_0;
        sRecordSongPos = 0 as libc::c_int as u8_0;
        sOcarinaInpEnabled = 1 as libc::c_int as u8_0;
        sStaffPlayingPos = 0 as libc::c_int as u8_0;
        D_8016BAA0 = sPierresSong[1 as libc::c_int as usize]
    } else {
        if sRecordSongPos as libc::c_int == 0 as libc::c_int {
            sPierresSong[1 as libc::c_int as usize] = D_8016BAA0
        } else {
            if sRecordingState as libc::c_int == 2 as libc::c_int {
                sStaffPlayingPos = 1 as libc::c_int as u8_0
            }
            func_800EDD68(1 as libc::c_int as u8_0);
        }
        sOcarinaInpEnabled = 0 as libc::c_int as u8_0;
        sStaffPlayingPos = 0 as libc::c_int as u8_0
    }
    sRecordingState = recordingState;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_OcaUpdateRecordingStaff() {
    sRecordingStaff.state = sRecordingState;
    sRecordingStaff.pos = sStaffPlayingPos;
    if sRecordingState as libc::c_int == 0xff as libc::c_int {
        sRecordingState = 0 as libc::c_int as u8_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn Audio_OcaUpdatePlayingStaff() {
    sPlayingStaff.noteIdx =
        (sCurOcarinaBtnIdx as libc::c_int & 0x3f as libc::c_int) as u8_0;
    sPlayingStaff.state = Audio_OcaGetPlayingState();
    sPlayingStaff.pos = sStaffPlayingPos;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_OcaUpdateDisplayedStaff() {
    if (sDisplayedNoteValue as libc::c_int & 0x3f as libc::c_int) <
           0x10 as libc::c_int {
        sDisplayedStaff.noteIdx = Audio_OcaMapNoteValue(sDisplayedNoteValue)
    }
    sDisplayedStaff.state = sPlaybackState;
    if sPlaybackSong != sPierresSong.as_mut_ptr() {
        sDisplayedStaff.pos = sStaffPlaybackPos as u8_0
    } else if sStaffPlaybackPos as libc::c_int == 0 as libc::c_int {
        sDisplayedStaff.pos = 0 as libc::c_int as u8_0
    } else {
        sDisplayedStaff.pos =
            ((sStaffPlaybackPos as libc::c_int - 1 as libc::c_int) %
                 8 as libc::c_int + 1 as libc::c_int) as u8_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn Audio_OcaGetRecordingStaff() -> *mut OcarinaStaff {
    return &mut sRecordingStaff;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_OcaGetPlayingStaff() -> *mut OcarinaStaff {
    if (sPlayingStaff.state as libc::c_int) < 0xfe as libc::c_int {
        D_80130F3C = 0 as libc::c_int as u32_0
    }
    return &mut sPlayingStaff;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_OcaGetDisplayingStaff() -> *mut OcarinaStaff {
    return &mut sDisplayedStaff;
}
#[no_mangle]
pub unsafe extern "C" fn func_800EE404() {
    let mut noteChanged: s32 = 0;
    if sRecordingState as libc::c_int != 0 as libc::c_int &&
           D_8016BA04.wrapping_sub(D_80131860) >=
               3 as libc::c_int as libc::c_uint {
        noteChanged = 0 as libc::c_int;
        if D_80131864 as libc::c_int != sCurOcarinaBtnVal as libc::c_int {
            if sCurOcarinaBtnVal as libc::c_int != 0xff as libc::c_int {
                sRecordingStaff.noteIdx =
                    (sCurOcarinaBtnIdx as libc::c_int & 0x3f as libc::c_int)
                        as u8_0;
                sStaffPlayingPos = sStaffPlayingPos.wrapping_add(1)
            } else if sRecordingState as libc::c_int == 2 as libc::c_int &&
                          sStaffPlayingPos as libc::c_int == 8 as libc::c_int
             {
                func_800EDD68(1 as libc::c_int as u8_0);
                return
            }
            if sStaffPlayingPos as libc::c_int > 8 as libc::c_int {
                if sRecordingState as libc::c_int == 2 as libc::c_int {
                    // notes played are over 8 and in recording mode.
                    func_800EDD68(1 as libc::c_int as u8_0);
                    return
                }
                sStaffPlayingPos = 1 as libc::c_int as u8_0
            }
            noteChanged = 1 as libc::c_int
        } else if D_80131868 as libc::c_int != D_80130F30 as libc::c_int {
            noteChanged = 1 as libc::c_int
        } else if D_8013186C as libc::c_int != D_80130F34 as libc::c_int {
            noteChanged = 1 as libc::c_int
        } else if D_80131870 as libc::c_int != D_80130F2C as libc::c_int {
            noteChanged = 1 as libc::c_int
        }
        if noteChanged != 0 {
            func_800EDD68(0 as libc::c_int as u8_0);
            D_80131860 = D_8016BA04
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Audio_OcaMemoryGameStart(mut minigameRound: u8_0) {
    let mut i: u8_0 = 0;
    if minigameRound as libc::c_int > 2 as libc::c_int {
        minigameRound = 2 as libc::c_int as u8_0
    }
    sOcaMinigameAppendPos = 0 as libc::c_int as u8_0;
    sOcaMinigameEndPos = sOcaMinigameNoteCnts[minigameRound as usize];
    i = 0 as libc::c_int as u8_0;
    while (i as libc::c_int) < 3 as libc::c_int {
        Audio_OcaMemoryGameGenNote();
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn Audio_OcaMemoryGameGenNote() -> s32 {
    let mut rnd: u32_0 = 0;
    let mut rndNote: u8_0 = 0;
    if sOcaMinigameAppendPos as libc::c_int ==
           sOcaMinigameEndPos as libc::c_int {
        return 1 as libc::c_int
    }
    rnd = Audio_NextRandom();
    rndNote =
        sOcarinaNoteValues[rnd.wrapping_rem(5 as libc::c_int as libc::c_uint)
                               as usize];
    if sOcarinaSongs[OCARINA_SONG_MEMORY_GAME as libc::c_int as
                         usize][(sOcaMinigameAppendPos as libc::c_int -
                                     1 as libc::c_int) as usize].noteIdx as
           libc::c_int == rndNote as libc::c_int {
        rndNote =
            sOcarinaNoteValues[rnd.wrapping_add(1 as libc::c_int as
                                                    libc::c_uint).wrapping_rem(5
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint)
                                   as usize]
    }
    sOcarinaSongs[OCARINA_SONG_MEMORY_GAME as libc::c_int as
                      usize][sOcaMinigameAppendPos as usize].noteIdx =
        rndNote;
    sOcarinaSongs[OCARINA_SONG_MEMORY_GAME as libc::c_int as
                      usize][sOcaMinigameAppendPos as usize].unk_02 =
        0x2d as libc::c_int as u16_0;
    sOcarinaSongs[OCARINA_SONG_MEMORY_GAME as libc::c_int as
                      usize][sOcaMinigameAppendPos as usize].volume =
        0x50 as libc::c_int as u8_0;
    sOcarinaSongs[OCARINA_SONG_MEMORY_GAME as libc::c_int as
                      usize][sOcaMinigameAppendPos as usize].vibrato =
        0 as libc::c_int as u8_0;
    sOcarinaSongs[OCARINA_SONG_MEMORY_GAME as libc::c_int as
                      usize][sOcaMinigameAppendPos as usize].tone =
        0 as libc::c_int as s8;
    sOcaMinigameAppendPos = sOcaMinigameAppendPos.wrapping_add(1);
    sOcarinaSongs[OCARINA_SONG_MEMORY_GAME as libc::c_int as
                      usize][sOcaMinigameAppendPos as usize].noteIdx =
        0xff as libc::c_int as u8_0;
    sOcarinaSongs[OCARINA_SONG_MEMORY_GAME as libc::c_int as
                      usize][sOcaMinigameAppendPos as usize].unk_02 =
        0 as libc::c_int as u16_0;
    sOcarinaSongs[OCARINA_SONG_MEMORY_GAME as libc::c_int as
                      usize][(sOcaMinigameAppendPos as libc::c_int +
                                  1 as libc::c_int) as usize].noteIdx =
        0xff as libc::c_int as u8_0;
    sOcarinaSongs[OCARINA_SONG_MEMORY_GAME as libc::c_int as
                      usize][(sOcaMinigameAppendPos as libc::c_int +
                                  1 as libc::c_int) as usize].unk_02 =
        0 as libc::c_int as u16_0;
    return 0 as libc::c_int;
}
// input update?
#[no_mangle]
pub unsafe extern "C" fn func_800EE6F4() {
    D_8016BA04 = gAudioContext.totalTaskCnt as u32_0;
    if D_80130F10 as libc::c_int != 0 as libc::c_int {
        if sOcarinaInpEnabled as libc::c_int == 1 as libc::c_int {
            Audio_GetOcaInput();
        }
        if sPlaybackState as libc::c_int == 0 as libc::c_int &&
               sOcarinaInpEnabled as libc::c_int == 1 as libc::c_int {
            func_800ED458(0 as libc::c_int);
        }
        if D_80130F3C != 0 as libc::c_int as libc::c_uint {
            if D_80130F3C & 0x4000 as libc::c_int as libc::c_uint != 0 {
                func_800ED200();
            } else { func_800ECDF8(); }
        }
        Audio_OcaPlayback();
        D_80130F68 = D_8016BA04 as s32;
        if sPlaybackState as libc::c_int == 0 as libc::c_int {
            func_800EE404();
        }
        if D_80130F3C != 0 as libc::c_int as libc::c_uint &&
               sPrevOcarinaNoteVal as libc::c_int !=
                   sCurOcarinaBtnVal as libc::c_int {
            D_80131880 = 1 as libc::c_int as u8_0
        }
        sPrevOcarinaNoteVal = sCurOcarinaBtnVal
    }
    Audio_OcaUpdatePlayingStaff();
    Audio_OcaUpdateDisplayedStaff();
    Audio_OcaUpdateRecordingStaff();
}
#[no_mangle]
pub unsafe extern "C" fn func_800EE824() {
    static mut D_80131C80: u8_0 = 0 as libc::c_int as u8_0;
    static mut D_80131C84: u8_0 = 1 as libc::c_int as u8_0;
    static mut D_80131C88: u16_0 = 1200 as libc::c_int as u16_0;
    match D_80131C80 as libc::c_int {
        0 => {
            let fresh3 = D_80131C88;
            D_80131C88 = D_80131C88.wrapping_sub(1);
            if fresh3 as libc::c_int == 0 as libc::c_int {
                if (D_80131C84 as libc::c_int) < 7 as libc::c_int {
                    D_80131C80 = D_80131C80.wrapping_add(1)
                } else {
                    D_80131C80 = 3 as libc::c_int as u8_0;
                    Audio_OcaSetInstrument(0 as libc::c_int as u8_0);
                }
                D_80131C88 = 1200 as libc::c_int as u16_0
            }
        }
        1 => {
            Audio_SetSoundBanksMute(0 as libc::c_int as u16_0);
            Audio_OcaSetInstrument(D_80131C84);
            Audio_OcaSetSongPlayback((OCARINA_SONG_SCARECROW_LONG as
                                          libc::c_int + 1 as libc::c_int) as
                                         s8, 1 as libc::c_int as s8);
            D_80131C84 = D_80131C84.wrapping_add(1);
            D_80131C80 = D_80131C80.wrapping_add(1)
        }
        2 => {
            if (*Audio_OcaGetDisplayingStaff()).state as libc::c_int ==
                   0 as libc::c_int {
                D_80131C80 = 0 as libc::c_int as u8_0
            }
        }
        _ => { }
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800EE930() {
    sPlayingStaff.noteIdx = OCARINA_NOTE_INVALID as libc::c_int as u8_0;
    sPlayingStaff.state = 0xff as libc::c_int as u8_0;
    sPlayingStaff.pos = 0 as libc::c_int as u8_0;
    sDisplayedStaff.noteIdx = OCARINA_NOTE_INVALID as libc::c_int as u8_0;
    sDisplayedStaff.state = 0 as libc::c_int as u8_0;
    sDisplayedStaff.pos = 0 as libc::c_int as u8_0;
    sRecordingStaff.noteIdx = OCARINA_NOTE_INVALID as libc::c_int as u8_0;
    sRecordingStaff.state = 0xff as libc::c_int as u8_0;
    sRecordingStaff.pos = 0 as libc::c_int as u8_0;
    D_80131880 = 0 as libc::c_int as u8_0;
}
#[no_mangle]
pub static mut D_80131C8C: f32_0 = 0.0f32;
// === Audio Debugging ===
// These variables come between in-function statics in func_800EE824 and Audio_SplitBgmChannels
#[no_mangle]
pub static mut sAudioUpdateDuration: f32_0 = 0.0f32;
#[no_mangle]
pub static mut sAudioUpdateDurationMax: f32_0 = 0.0f32;
#[no_mangle]
pub static mut sAudioDebugEverOpened: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut sAudioSfxMuted: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut sAudioDebugPage: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut sAudioSndContSel: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut sAudioDebugTextColor: u8_0 = 7 as libc::c_int as u8_0;
#[no_mangle]
pub static mut sAudioDebugPageNames: [[libc::c_char; 23]; 15] =
    unsafe {
        [*::std::mem::transmute::<&[u8; 23],
                                  &mut [libc::c_char; 23]>(b"Non\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 23],
                                  &mut [libc::c_char; 23]>(b"Sound Control\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 23],
                                  &mut [libc::c_char; 23]>(b"Spec Info\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 23],
                                  &mut [libc::c_char; 23]>(b"Heap Info\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 23],
                                  &mut [libc::c_char; 23]>(b"Grp Track Info\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 23],
                                  &mut [libc::c_char; 23]>(b"Sub Track Info\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 23],
                                  &mut [libc::c_char; 23]>(b"Channel Info\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 23],
                                  &mut [libc::c_char; 23]>(b"Interface Info\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 23],
                                  &mut [libc::c_char; 23]>(b"SE Flag Swap\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 23],
                                  &mut [libc::c_char; 23]>(b"Block Change BGM\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 23],
                                  &mut [libc::c_char; 23]>(b"Natural Sound Control\x00\x00"),
         *::std::mem::transmute::<&[u8; 23],
                                  &mut [libc::c_char; 23]>(b"Ocarina Test\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 23],
                                  &mut [libc::c_char; 23]>(b"SE Parameter Change\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 23],
                                  &mut [libc::c_char; 23]>(b"Scroll Print\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 23],
                                  &mut [libc::c_char; 23]>(b"Free Area\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")]
    };
#[no_mangle]
pub static mut sAudioSndContWork: [u16_0; 11] =
    [0 as libc::c_int as u16_0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
#[no_mangle]
pub static mut sAudioSndContWorkLims: [u16_0; 11] =
    [128 as libc::c_int as u16_0, 128 as libc::c_int as u16_0,
     7 as libc::c_int as u16_0, 512 as libc::c_int as u16_0,
     4 as libc::c_int as u16_0, 2 as libc::c_int as u16_0,
     16 as libc::c_int as u16_0, 32 as libc::c_int as u16_0,
     2 as libc::c_int as u16_0, 2 as libc::c_int as u16_0,
     2 as libc::c_int as u16_0];
#[no_mangle]
pub static mut sSoundBankNames: [[libc::c_char; 11]; 7] =
    unsafe {
        [*::std::mem::transmute::<&[u8; 11],
                                  &mut [libc::c_char; 11]>(b"PLAYER\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 11],
                                  &mut [libc::c_char; 11]>(b"ITEM\x00\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 11],
                                  &mut [libc::c_char; 11]>(b"ENVIROMENT\x00"),
         *::std::mem::transmute::<&[u8; 11],
                                  &mut [libc::c_char; 11]>(b"ENEMY\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 11],
                                  &mut [libc::c_char; 11]>(b"SYSTEM\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 11],
                                  &mut [libc::c_char; 11]>(b"OCARINA\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 11],
                                  &mut [libc::c_char; 11]>(b"VOICE\x00\x00\x00\x00\x00\x00")]
    };
#[no_mangle]
pub static mut sSoundModeNames: [[libc::c_char; 10]; 5] =
    unsafe {
        [*::std::mem::transmute::<&[u8; 10],
                                  &mut [libc::c_char; 10]>(b"W-STEREO\x00\x00"),
         *::std::mem::transmute::<&[u8; 10],
                                  &mut [libc::c_char; 10]>(b"HEADPHONE\x00"),
         *::std::mem::transmute::<&[u8; 10],
                                  &mut [libc::c_char; 10]>(b"3D SOUND\x00\x00"),
         *::std::mem::transmute::<&[u8; 10],
                                  &mut [libc::c_char; 10]>(b"MONO\x00\x00\x00\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 10],
                                  &mut [libc::c_char; 10]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")]
    };
#[no_mangle]
pub static mut sAudioIntInfoX: s8 = 0 as libc::c_int as s8;
#[no_mangle]
pub static mut sAudioIntInfoY: s8 = 0 as libc::c_int as s8;
#[no_mangle]
pub static mut sAudioIntInfoSel: s8 = 0 as libc::c_int as s8;
#[no_mangle]
pub static mut sAudioIntInfoBankPage: [s8; 7] =
    [0 as libc::c_int as s8, 0 as libc::c_int as s8, 2 as libc::c_int as s8,
     2 as libc::c_int as s8, 0 as libc::c_int as s8, 0 as libc::c_int as s8,
     0 as libc::c_int as s8];
#[no_mangle]
pub static mut sAudioScrPrtSel: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut sAudioScrPrtInd: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut sAudioScrPrtOverflow: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut sAudioScrPrtX: s8 = 26 as libc::c_int as s8;
#[no_mangle]
pub static mut sAudioScrPrtY: s8 = 1 as libc::c_int as s8;
#[no_mangle]
pub static mut sAudioScrPrtWork: [u8_0; 11] =
    [1 as libc::c_int as u8_0, 19 as libc::c_int as u8_0,
     6 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     0 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
     1 as libc::c_int as u8_0];
#[no_mangle]
pub static mut sAudioScrPrtWorkLims: [u8_0; 11] =
    [2 as libc::c_int as u8_0, 25 as libc::c_int as u8_0,
     8 as libc::c_int as u8_0, 2 as libc::c_int as u8_0,
     2 as libc::c_int as u8_0, 2 as libc::c_int as u8_0,
     2 as libc::c_int as u8_0, 2 as libc::c_int as u8_0,
     2 as libc::c_int as u8_0, 2 as libc::c_int as u8_0,
     2 as libc::c_int as u8_0];
#[no_mangle]
pub static mut sAudioSubTrackInfoSpec: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut sAudioSfxSwapIsEditing: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut sAudioSfxSwapSel: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut sAudioSfxSwapNibbleSel: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut sAudioSfxSwapModeNames: [[libc::c_char; 5]; 2] =
    unsafe {
        [*::std::mem::transmute::<&[u8; 5],
                                  &mut [libc::c_char; 5]>(b"SWAP\x00"),
         *::std::mem::transmute::<&[u8; 5],
                                  &mut [libc::c_char; 5]>(b"ADD\x00\x00")]
    };
#[no_mangle]
pub static mut sAudioSfxParamChgSel: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut sAudioSfxParamChgBitSel: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut sAudioSfxParamChgWork: [u16_0; 4] =
    [0 as libc::c_int as u16_0, 0, 0, 0];
#[no_mangle]
pub static mut sAudioSubTrackInfoPlayerSel: u8_0 =
    SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0;
#[no_mangle]
pub static mut sAudioSubTrackInfoChannelSel: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut sSeqPlayerPeakNumLayers: [u8_0; 20] =
    [0 as libc::c_int as u8_0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0];
#[no_mangle]
pub static mut sAudioSceneNames: [[libc::c_char; 2]; 3] =
    unsafe {
        [*::std::mem::transmute::<&[u8; 2], &mut [libc::c_char; 2]>(b"A\x00"),
         *::std::mem::transmute::<&[u8; 2], &mut [libc::c_char; 2]>(b"S\x00"),
         *::std::mem::transmute::<&[u8; 2], &mut [libc::c_char; 2]>(b"X\x00")]
    };
#[no_mangle]
pub static mut sAudioBlkChgBgmWork: [u8_0; 2] = [0 as libc::c_int as u8_0, 0];
#[no_mangle]
pub static mut sAudioBlkChgBgmSel: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut sBoolStrs: [[libc::c_char; 5]; 3] =
    unsafe {
        [*::std::mem::transmute::<&[u8; 5],
                                  &mut [libc::c_char; 5]>(b"OFF\x00\x00"),
         *::std::mem::transmute::<&[u8; 5],
                                  &mut [libc::c_char; 5]>(b"ON\x00\x00\x00"),
         *::std::mem::transmute::<&[u8; 5],
                                  &mut [libc::c_char; 5]>(b"STBY\x00")]
    };
#[no_mangle]
pub static mut sAudioNatureFailed: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut sPeakNumNotes: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub unsafe extern "C" fn AudioDebug_SetInput() {
    let mut inputs: [Input; 4] =
        [Input{cur: OSContPad{button: 0, stick_x: 0, stick_y: 0, errno: 0,},
               prev: OSContPad{button: 0, stick_x: 0, stick_y: 0, errno: 0,},
               press: OSContPad{button: 0, stick_x: 0, stick_y: 0, errno: 0,},
               rel: OSContPad{button: 0, stick_x: 0, stick_y: 0, errno: 0,},};
            4];
    let mut btn: u32_0 = 0;
    PadMgr_RequestPadData(&mut gPadMgr, inputs.as_mut_ptr(),
                          0 as libc::c_int);
    btn = inputs[3 as libc::c_int as usize].cur.button as u32_0;
    sDebugPadHold = btn & 0xffff as libc::c_int as libc::c_uint;
    sDebugPadPress = (btn ^ sDebugPadBtnLast) & btn;
    sDebugPadBtnLast = btn;
}
#[no_mangle]
pub unsafe extern "C" fn AudioDebug_ToStringBinary(mut num: u32_0,
                                                   mut bits: u8_0)
 -> *mut libc::c_char {
    let mut i: u8_0 = 0;
    let mut flg: u32_0 = 1 as libc::c_int as u32_0;
    i = 0 as libc::c_int as u8_0;
    while (i as libc::c_int) < bits as libc::c_int {
        if num & flg != 0 {
            sBinToStrBuf[(bits as libc::c_int - i as libc::c_int -
                              1 as libc::c_int) as usize] =
                '1' as i32 as libc::c_char
        } else {
            sBinToStrBuf[(bits as libc::c_int - i as libc::c_int -
                              1 as libc::c_int) as usize] =
                '0' as i32 as libc::c_char
        }
        flg =
            (flg as
                 libc::c_uint).wrapping_mul(2 as libc::c_int as libc::c_uint)
                as u32_0 as u32_0;
        i = i.wrapping_add(1)
    }
    sBinToStrBuf[bits as usize] = '\u{0}' as i32 as libc::c_char;
    return sBinToStrBuf.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn AudioDebug_Draw(mut printer: *mut GfxPrint) {
    let mut pad: [s32; 3] = [0; 3];
    let mut i: u8_0 = 0;
    let mut j: u8_0 = 0;
    let mut ctr: u8_0 = 0;
    let mut ctr2: u8_0 = 0;
    let mut k: s8 = 0;
    let mut k2: s8 = 0;
    let mut ind: s8 = 0;
    let mut numEnabledNotes: u8_0 = 0 as libc::c_int as u8_0;
    let mut digitStr: [libc::c_char; 2] =
        *::std::mem::transmute::<&[u8; 2], &mut [libc::c_char; 2]>(b"1\x00");
    sAudioDebugEverOpened = 1 as libc::c_int as u8_0;
    GfxPrint_SetPos(printer, 3 as libc::c_int, 2 as libc::c_int);
    GfxPrint_SetColor(printer,
                      (((sAudioDebugTextColor as libc::c_int &
                             4 as libc::c_int) >> 2 as libc::c_int) *
                           255 as libc::c_int) as u32_0,
                      (((sAudioDebugTextColor as libc::c_int &
                             2 as libc::c_int) >> 1 as libc::c_int) *
                           255 as libc::c_int) as u32_0,
                      ((sAudioDebugTextColor as libc::c_int &
                            1 as libc::c_int) * 255 as libc::c_int) as u32_0,
                      255 as libc::c_int as u32_0);
    GfxPrint_Printf(printer,
                    b"Audio Debug Mode\x00" as *const u8 as
                        *const libc::c_char);
    GfxPrint_SetPos(printer, 3 as libc::c_int, 3 as libc::c_int);
    GfxPrint_Printf(printer,
                    b"- %s -\x00" as *const u8 as *const libc::c_char,
                    sAudioDebugPageNames[sAudioDebugPage as
                                             usize].as_mut_ptr());
    i = 0 as libc::c_int as u8_0;
    while (i as libc::c_int) <
              gAudioSpecs[gAudioSpecId as usize].numNotes as libc::c_int {
        if (*gAudioContext.notes.offset(i as
                                            isize)).noteSubEu.bitField0.enabled()
               as libc::c_int == 1 as libc::c_int {
            numEnabledNotes = numEnabledNotes.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    if (sPeakNumNotes as libc::c_int) < numEnabledNotes as libc::c_int {
        sPeakNumNotes = numEnabledNotes
    }
    if (sAudioSpecPeakNumNotes[gAudioSpecId as usize] as libc::c_int) <
           numEnabledNotes as libc::c_int {
        sAudioSpecPeakNumNotes[gAudioSpecId as usize] = numEnabledNotes
    }
    if sAudioScrPrtWork[0 as libc::c_int as usize] as libc::c_int !=
           0 as libc::c_int {
        GfxPrint_SetPos(printer, sAudioScrPrtX as s32, sAudioScrPrtY as s32);
        GfxPrint_SetColor(printer,
                          (((sAudioScrPrtWork[2 as libc::c_int as usize] as
                                 libc::c_int & 4 as libc::c_int) >>
                                2 as libc::c_int) * 200 as libc::c_int) as
                              u32_0,
                          (((sAudioScrPrtWork[2 as libc::c_int as usize] as
                                 libc::c_int & 2 as libc::c_int) >>
                                1 as libc::c_int) * 200 as libc::c_int) as
                              u32_0,
                          ((sAudioScrPrtWork[2 as libc::c_int as usize] as
                                libc::c_int & 1 as libc::c_int) *
                               200 as libc::c_int) as u32_0,
                          255 as libc::c_int as u32_0);
        GfxPrint_Printf(printer,
                        b"Audio ScrPrt\x00" as *const u8 as
                            *const libc::c_char);
        ind = sAudioScrPrtInd as s8;
        k = 0 as libc::c_int as s8;
        while (k as libc::c_int) <
                  sAudioScrPrtWork[1 as libc::c_int as usize] as libc::c_int +
                      1 as libc::c_int {
            if ind as libc::c_int == 0 as libc::c_int {
                if sAudioScrPrtOverflow as libc::c_int == 1 as libc::c_int {
                    ind = (25 as libc::c_int - 1 as libc::c_int) as s8
                } else {
                    k =
                        (sAudioScrPrtWork[1 as libc::c_int as usize] as
                             libc::c_int + 1 as libc::c_int) as s8
                    // "break;"
                }
            } else { ind -= 1 }
            if k as libc::c_int !=
                   sAudioScrPrtWork[1 as libc::c_int as usize] as libc::c_int
                       + 1 as libc::c_int {
                if ind as libc::c_int % 5 as libc::c_int != 0 as libc::c_int {
                    GfxPrint_SetColor(printer,
                                      (((sAudioScrPrtWork[2 as libc::c_int as
                                                              usize] as
                                             libc::c_int & 4 as libc::c_int)
                                            >> 2 as libc::c_int) *
                                           180 as libc::c_int) as u32_0,
                                      (((sAudioScrPrtWork[2 as libc::c_int as
                                                              usize] as
                                             libc::c_int & 2 as libc::c_int)
                                            >> 1 as libc::c_int) *
                                           180 as libc::c_int) as u32_0,
                                      ((sAudioScrPrtWork[2 as libc::c_int as
                                                             usize] as
                                            libc::c_int & 1 as libc::c_int) *
                                           180 as libc::c_int) as u32_0,
                                      255 as libc::c_int as u32_0);
                } else {
                    GfxPrint_SetColor(printer,
                                      (((sAudioScrPrtWork[2 as libc::c_int as
                                                              usize] as
                                             libc::c_int & 4 as libc::c_int)
                                            >> 2 as libc::c_int) *
                                           120 as libc::c_int) as u32_0,
                                      (((sAudioScrPrtWork[2 as libc::c_int as
                                                              usize] as
                                             libc::c_int & 2 as libc::c_int)
                                            >> 1 as libc::c_int) *
                                           120 as libc::c_int) as u32_0,
                                      ((sAudioScrPrtWork[2 as libc::c_int as
                                                             usize] as
                                            libc::c_int & 1 as libc::c_int) *
                                           120 as libc::c_int) as u32_0,
                                      255 as libc::c_int as u32_0);
                }
                GfxPrint_SetPos(printer,
                                2 as libc::c_int +
                                    sAudioScrPrtX as libc::c_int,
                                sAudioScrPrtY as libc::c_int +
                                    sAudioScrPrtWork[1 as libc::c_int as
                                                         usize] as libc::c_int
                                    + 1 as libc::c_int - k as libc::c_int);
                GfxPrint_Printf(printer,
                                b"%s\x00" as *const u8 as *const libc::c_char,
                                sAudioScrPrtBuf[ind as
                                                    usize].str_0.as_mut_ptr());
                GfxPrint_SetPos(printer,
                                7 as libc::c_int +
                                    sAudioScrPrtX as libc::c_int,
                                sAudioScrPrtY as libc::c_int +
                                    sAudioScrPrtWork[1 as libc::c_int as
                                                         usize] as libc::c_int
                                    + 1 as libc::c_int - k as libc::c_int);
                GfxPrint_Printf(printer,
                                b"%04X\x00" as *const u8 as
                                    *const libc::c_char,
                                sAudioScrPrtBuf[ind as usize].num as
                                    libc::c_int);
            }
            k += 1
        }
    }
    match sAudioDebugPage as libc::c_int {
        0 => {
            GfxPrint_SetPos(printer, 3 as libc::c_int, 4 as libc::c_int);
            GfxPrint_SetColor(printer,
                              (((sAudioDebugTextColor as libc::c_int &
                                     4 as libc::c_int) >> 2 as libc::c_int) *
                                   255 as libc::c_int) as u32_0,
                              (((sAudioDebugTextColor as libc::c_int &
                                     2 as libc::c_int) >> 1 as libc::c_int) *
                                   64 as libc::c_int) as u32_0,
                              ((sAudioDebugTextColor as libc::c_int &
                                    1 as libc::c_int) * 64 as libc::c_int) as
                                  u32_0, 255 as libc::c_int as u32_0);
            GfxPrint_Printf(printer,
                            b"BGM CANCEL:%s\x00" as *const u8 as
                                *const libc::c_char,
                            sBoolStrs[sAudioSndContWork[5 as libc::c_int as
                                                            usize] as
                                          usize].as_mut_ptr());
            GfxPrint_SetPos(printer, 3 as libc::c_int, 5 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"SE MUTE:%s\x00" as *const u8 as
                                *const libc::c_char,
                            sBoolStrs[sAudioSfxMuted as usize].as_mut_ptr());
            GfxPrint_SetPos(printer, 18 as libc::c_int, 4 as libc::c_int);
            GfxPrint_SetColor(printer,
                              (((sAudioDebugTextColor as libc::c_int &
                                     4 as libc::c_int) >> 2 as libc::c_int) *
                                   255 as libc::c_int) as u32_0,
                              (((sAudioDebugTextColor as libc::c_int &
                                     2 as libc::c_int) >> 1 as libc::c_int) *
                                   255 as libc::c_int) as u32_0,
                              ((sAudioDebugTextColor as libc::c_int &
                                    1 as libc::c_int) * 255 as libc::c_int) as
                                  u32_0, 255 as libc::c_int as u32_0);
            GfxPrint_Printf(printer,
                            b"PUSH CONT-4 A-BTN\x00" as *const u8 as
                                *const libc::c_char);
            ind = sAudioSndContWork[2 as libc::c_int as usize] as s8;
            i =
                (*gSoundBanks[ind as
                                  usize].offset(0 as libc::c_int as
                                                    isize)).next;
            j = 0 as libc::c_int as u8_0;
            GfxPrint_SetColor(printer,
                              (((sAudioDebugTextColor as libc::c_int &
                                     4 as libc::c_int) >> 2 as libc::c_int) *
                                   255 as libc::c_int) as u32_0,
                              (((sAudioDebugTextColor as libc::c_int &
                                     2 as libc::c_int) >> 1 as libc::c_int) *
                                   255 as libc::c_int) as u32_0,
                              ((sAudioDebugTextColor as libc::c_int &
                                    1 as libc::c_int) * 255 as libc::c_int) as
                                  u32_0, 255 as libc::c_int as u32_0);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 6 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"SE HANDLE:%s\x00" as *const u8 as
                                *const libc::c_char,
                            sSoundBankNames[ind as usize].as_mut_ptr());
            while i as libc::c_int != 0xff as libc::c_int {
                let fresh4 = j;
                j = j.wrapping_add(1);
                GfxPrint_SetPos(printer, 3 as libc::c_int,
                                7 as libc::c_int + fresh4 as libc::c_int);
                GfxPrint_Printf(printer,
                                b"%02x %04x %02x %08x\x00" as *const u8 as
                                    *const libc::c_char, i as libc::c_int,
                                (*gSoundBanks[ind as
                                                  usize].offset(i as
                                                                    isize)).sfxId
                                    as libc::c_int,
                                (*gSoundBanks[ind as
                                                  usize].offset(i as
                                                                    isize)).state
                                    as libc::c_int,
                                (*gSoundBanks[ind as
                                                  usize].offset(i as
                                                                    isize)).priority);
                i = (*gSoundBanks[ind as usize].offset(i as isize)).next
            }
        }
        1 => {
            GfxPrint_SetPos(printer, 2 as libc::c_int,
                            4 as libc::c_int +
                                sAudioSndContSel as libc::c_int);
            GfxPrint_SetColor(printer,
                              (((sAudioDebugTextColor as libc::c_int &
                                     4 as libc::c_int) >> 2 as libc::c_int) *
                                   127 as libc::c_int) as u32_0,
                              (((sAudioDebugTextColor as libc::c_int &
                                     2 as libc::c_int) >> 1 as libc::c_int) *
                                   255 as libc::c_int) as u32_0,
                              ((sAudioDebugTextColor as libc::c_int &
                                    1 as libc::c_int) * 127 as libc::c_int) as
                                  u32_0, 255 as libc::c_int as u32_0);
            GfxPrint_Printf(printer,
                            b"*\x00" as *const u8 as *const libc::c_char);
            GfxPrint_SetColor(printer,
                              (((sAudioDebugTextColor as libc::c_int &
                                     4 as libc::c_int) >> 2 as libc::c_int) *
                                   255 as libc::c_int) as u32_0,
                              (((sAudioDebugTextColor as libc::c_int &
                                     2 as libc::c_int) >> 1 as libc::c_int) *
                                   255 as libc::c_int) as u32_0,
                              ((sAudioDebugTextColor as libc::c_int &
                                    1 as libc::c_int) * 255 as libc::c_int) as
                                  u32_0, 255 as libc::c_int as u32_0);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 4 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"Seq 0  : %2x\x00" as *const u8 as
                                *const libc::c_char,
                            sAudioSndContWork[0 as libc::c_int as usize] as
                                libc::c_int);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 5 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"Seq 1  : %2x\x00" as *const u8 as
                                *const libc::c_char,
                            sAudioSndContWork[1 as libc::c_int as usize] as
                                libc::c_int);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 6 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"SE HD  : %2x %s\x00" as *const u8 as
                                *const libc::c_char,
                            sAudioSndContWork[2 as libc::c_int as usize] as
                                libc::c_int,
                            sSoundBankNames[sAudioSndContWork[2 as libc::c_int
                                                                  as usize] as
                                                usize].as_mut_ptr());
            GfxPrint_SetPos(printer, 3 as libc::c_int, 7 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"SE No. :%3x\x00" as *const u8 as
                                *const libc::c_char,
                            sAudioSndContWork[3 as libc::c_int as usize] as
                                libc::c_int);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 8 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"S-Out  : %2x %s\x00" as *const u8 as
                                *const libc::c_char,
                            sAudioSndContWork[4 as libc::c_int as usize] as
                                libc::c_int,
                            sSoundModeNames[sAudioSndContWork[4 as libc::c_int
                                                                  as usize] as
                                                usize].as_mut_ptr());
            GfxPrint_SetPos(printer, 3 as libc::c_int, 9 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"BGM Ent: %2x\x00" as *const u8 as
                                *const libc::c_char,
                            sAudioSndContWork[5 as libc::c_int as usize] as
                                libc::c_int);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 10 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"Spec   : %2x\x00" as *const u8 as
                                *const libc::c_char,
                            sAudioSndContWork[6 as libc::c_int as usize] as
                                libc::c_int);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 11 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"Na Snd : %2x\x00" as *const u8 as
                                *const libc::c_char,
                            sAudioSndContWork[7 as libc::c_int as usize] as
                                libc::c_int);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 12 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"Cam Wt : %s\x00" as *const u8 as
                                *const libc::c_char,
                            sBoolStrs[sAudioSndContWork[8 as libc::c_int as
                                                            usize] as
                                          usize].as_mut_ptr());
            GfxPrint_SetPos(printer, 3 as libc::c_int, 13 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"Lnk Wt : %s\x00" as *const u8 as
                                *const libc::c_char,
                            sBoolStrs[sAudioSndContWork[9 as libc::c_int as
                                                            usize] as
                                          usize].as_mut_ptr());
            GfxPrint_SetPos(printer, 3 as libc::c_int, 14 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"SE Ent : %2x\x00" as *const u8 as
                                *const libc::c_char,
                            sAudioSndContWork[10 as libc::c_int as usize] as
                                libc::c_int);
        }
        7 => {
            ind = 0 as libc::c_int as s8;
            k = 0 as libc::c_int as s8;
            while (k as libc::c_int) < 7 as libc::c_int {
                if k as libc::c_int == sAudioIntInfoSel as libc::c_int {
                    GfxPrint_SetColor(printer,
                                      (((sAudioDebugTextColor as libc::c_int &
                                             4 as libc::c_int) >>
                                            2 as libc::c_int) *
                                           255 as libc::c_int) as u32_0,
                                      (((sAudioDebugTextColor as libc::c_int &
                                             2 as libc::c_int) >>
                                            1 as libc::c_int) *
                                           127 as libc::c_int) as u32_0,
                                      ((sAudioDebugTextColor as libc::c_int &
                                            1 as libc::c_int) *
                                           127 as libc::c_int) as u32_0,
                                      255 as libc::c_int as u32_0);
                } else {
                    GfxPrint_SetColor(printer,
                                      (((sAudioDebugTextColor as libc::c_int &
                                             4 as libc::c_int) >>
                                            2 as libc::c_int) *
                                           255 as libc::c_int) as u32_0,
                                      (((sAudioDebugTextColor as libc::c_int &
                                             2 as libc::c_int) >>
                                            1 as libc::c_int) *
                                           255 as libc::c_int) as u32_0,
                                      ((sAudioDebugTextColor as libc::c_int &
                                            1 as libc::c_int) *
                                           255 as libc::c_int) as u32_0,
                                      255 as libc::c_int as u32_0);
                }
                GfxPrint_SetPos(printer,
                                2 as libc::c_int +
                                    sAudioIntInfoX as libc::c_int,
                                4 as libc::c_int + ind as libc::c_int +
                                    sAudioIntInfoY as libc::c_int);
                GfxPrint_Printf(printer,
                                b"%s <%d>\x00" as *const u8 as
                                    *const libc::c_char,
                                sSoundBankNames[k as usize].as_mut_ptr(),
                                sAudioIntInfoBankPage[k as usize] as
                                    libc::c_int);
                k2 = 0 as libc::c_int as s8;
                while (k2 as libc::c_int) <
                          gChannelsPerBank[gSfxChannelLayout as
                                               usize][k as usize] as
                              libc::c_int {
                    GfxPrint_SetPos(printer,
                                    2 as libc::c_int +
                                        sAudioIntInfoX as libc::c_int,
                                    5 as libc::c_int + ind as libc::c_int +
                                        sAudioIntInfoY as libc::c_int);
                    if sAudioIntInfoBankPage[k as usize] as libc::c_int ==
                           1 as libc::c_int {
                        if gActiveSounds[k as usize][k2 as usize].entryIndex
                               as libc::c_int != 0xff as libc::c_int &&
                               ((*gSoundBanks[k as
                                                  usize].offset(gActiveSounds[k
                                                                                  as
                                                                                  usize][k2
                                                                                             as
                                                                                             usize].entryIndex
                                                                    as
                                                                    isize)).state
                                    as libc::c_int ==
                                    SFX_STATE_PLAYING_1 as libc::c_int ||
                                    (*gSoundBanks[k as
                                                      usize].offset(gActiveSounds[k
                                                                                      as
                                                                                      usize][k2
                                                                                                 as
                                                                                                 usize].entryIndex
                                                                        as
                                                                        isize)).state
                                        as libc::c_int ==
                                        SFX_STATE_PLAYING_2 as libc::c_int) {
                            GfxPrint_Printf(printer,
                                            b"%2X %5d %5d %5d %02X %04X %04X\x00"
                                                as *const u8 as
                                                *const libc::c_char,
                                            gActiveSounds[k as
                                                              usize][k2 as
                                                                         usize].entryIndex
                                                as libc::c_int,
                                            *(*gSoundBanks[k as
                                                               usize].offset(gActiveSounds[k
                                                                                               as
                                                                                               usize][k2
                                                                                                          as
                                                                                                          usize].entryIndex
                                                                                 as
                                                                                 isize)).posX
                                                as s32,
                                            *(*gSoundBanks[k as
                                                               usize].offset(gActiveSounds[k
                                                                                               as
                                                                                               usize][k2
                                                                                                          as
                                                                                                          usize].entryIndex
                                                                                 as
                                                                                 isize)).posY
                                                as s32,
                                            *(*gSoundBanks[k as
                                                               usize].offset(gActiveSounds[k
                                                                                               as
                                                                                               usize][k2
                                                                                                          as
                                                                                                          usize].entryIndex
                                                                                 as
                                                                                 isize)).posZ
                                                as s32,
                                            (*gSoundBanks[k as
                                                              usize].offset(gActiveSounds[k
                                                                                              as
                                                                                              usize][k2
                                                                                                         as
                                                                                                         usize].entryIndex
                                                                                as
                                                                                isize)).sfxImportance
                                                as libc::c_int,
                                            (*gSoundBanks[k as
                                                              usize].offset(gActiveSounds[k
                                                                                              as
                                                                                              usize][k2
                                                                                                         as
                                                                                                         usize].entryIndex
                                                                                as
                                                                                isize)).sfxParams
                                                as libc::c_int,
                                            (*gSoundBanks[k as
                                                              usize].offset(gActiveSounds[k
                                                                                              as
                                                                                              usize][k2
                                                                                                         as
                                                                                                         usize].entryIndex
                                                                                as
                                                                                isize)).sfxId
                                                as libc::c_int);
                        } else {
                            GfxPrint_Printf(printer,
                                            b"FF ----- ----- ----- -- ---- ----\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                        }
                    } else if sAudioIntInfoBankPage[k as usize] as libc::c_int
                                  == 2 as libc::c_int {
                        if gActiveSounds[k as usize][k2 as usize].entryIndex
                               as libc::c_int != 0xff as libc::c_int &&
                               ((*gSoundBanks[k as
                                                  usize].offset(gActiveSounds[k
                                                                                  as
                                                                                  usize][k2
                                                                                             as
                                                                                             usize].entryIndex
                                                                    as
                                                                    isize)).state
                                    as libc::c_int ==
                                    SFX_STATE_PLAYING_1 as libc::c_int ||
                                    (*gSoundBanks[k as
                                                      usize].offset(gActiveSounds[k
                                                                                      as
                                                                                      usize][k2
                                                                                                 as
                                                                                                 usize].entryIndex
                                                                        as
                                                                        isize)).state
                                        as libc::c_int ==
                                        SFX_STATE_PLAYING_2 as libc::c_int) {
                            GfxPrint_Printf(printer,
                                            b"%2X %5d %5d %5d %3d %3d %04X\x00"
                                                as *const u8 as
                                                *const libc::c_char,
                                            gActiveSounds[k as
                                                              usize][k2 as
                                                                         usize].entryIndex
                                                as libc::c_int,
                                            *(*gSoundBanks[k as
                                                               usize].offset(gActiveSounds[k
                                                                                               as
                                                                                               usize][k2
                                                                                                          as
                                                                                                          usize].entryIndex
                                                                                 as
                                                                                 isize)).posX
                                                as s32,
                                            *(*gSoundBanks[k as
                                                               usize].offset(gActiveSounds[k
                                                                                               as
                                                                                               usize][k2
                                                                                                          as
                                                                                                          usize].entryIndex
                                                                                 as
                                                                                 isize)).posY
                                                as s32,
                                            *(*gSoundBanks[k as
                                                               usize].offset(gActiveSounds[k
                                                                                               as
                                                                                               usize][k2
                                                                                                          as
                                                                                                          usize].entryIndex
                                                                                 as
                                                                                 isize)).posZ
                                                as s32,
                                            ((*gAudioContext.seqPlayers[SEQ_PLAYER_SFX
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            usize].channels[(*gSoundBanks[k
                                                                                                              as
                                                                                                              usize].offset(gActiveSounds[k
                                                                                                                                              as
                                                                                                                                              usize][k2
                                                                                                                                                         as
                                                                                                                                                         usize].entryIndex
                                                                                                                                as
                                                                                                                                isize)).channelIdx
                                                                                                as
                                                                                                usize]).volume
                                                 * 127.1f32) as s32,
                                            (*gAudioContext.seqPlayers[SEQ_PLAYER_SFX
                                                                           as
                                                                           libc::c_int
                                                                           as
                                                                           usize].channels[(*gSoundBanks[k
                                                                                                             as
                                                                                                             usize].offset(gActiveSounds[k
                                                                                                                                             as
                                                                                                                                             usize][k2
                                                                                                                                                        as
                                                                                                                                                        usize].entryIndex
                                                                                                                               as
                                                                                                                               isize)).channelIdx
                                                                                               as
                                                                                               usize]).newPan
                                                as libc::c_int,
                                            (*gSoundBanks[k as
                                                              usize].offset(gActiveSounds[k
                                                                                              as
                                                                                              usize][k2
                                                                                                         as
                                                                                                         usize].entryIndex
                                                                                as
                                                                                isize)).sfxId
                                                as libc::c_int);
                        } else {
                            GfxPrint_Printf(printer,
                                            b"FF ----- ----- ----- --- --- ----\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                        }
                    } else if sAudioIntInfoBankPage[k as usize] as libc::c_int
                                  == 3 as libc::c_int {
                        if gActiveSounds[k as usize][k2 as usize].entryIndex
                               as libc::c_int != 0xff as libc::c_int &&
                               ((*gSoundBanks[k as
                                                  usize].offset(gActiveSounds[k
                                                                                  as
                                                                                  usize][k2
                                                                                             as
                                                                                             usize].entryIndex
                                                                    as
                                                                    isize)).state
                                    as libc::c_int ==
                                    SFX_STATE_PLAYING_1 as libc::c_int ||
                                    (*gSoundBanks[k as
                                                      usize].offset(gActiveSounds[k
                                                                                      as
                                                                                      usize][k2
                                                                                                 as
                                                                                                 usize].entryIndex
                                                                        as
                                                                        isize)).state
                                        as libc::c_int ==
                                        SFX_STATE_PLAYING_2 as libc::c_int) {
                            GfxPrint_Printf(printer,
                                            b"%2X %5d %5d %5d %3d %3d %04X\x00"
                                                as *const u8 as
                                                *const libc::c_char,
                                            gActiveSounds[k as
                                                              usize][k2 as
                                                                         usize].entryIndex
                                                as libc::c_int,
                                            *(*gSoundBanks[k as
                                                               usize].offset(gActiveSounds[k
                                                                                               as
                                                                                               usize][k2
                                                                                                          as
                                                                                                          usize].entryIndex
                                                                                 as
                                                                                 isize)).posX
                                                as s32,
                                            *(*gSoundBanks[k as
                                                               usize].offset(gActiveSounds[k
                                                                                               as
                                                                                               usize][k2
                                                                                                          as
                                                                                                          usize].entryIndex
                                                                                 as
                                                                                 isize)).posY
                                                as s32,
                                            *(*gSoundBanks[k as
                                                               usize].offset(gActiveSounds[k
                                                                                               as
                                                                                               usize][k2
                                                                                                          as
                                                                                                          usize].entryIndex
                                                                                 as
                                                                                 isize)).posZ
                                                as s32,
                                            ((*gAudioContext.seqPlayers[SEQ_PLAYER_SFX
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            usize].channels[(*gSoundBanks[k
                                                                                                              as
                                                                                                              usize].offset(gActiveSounds[k
                                                                                                                                              as
                                                                                                                                              usize][k2
                                                                                                                                                         as
                                                                                                                                                         usize].entryIndex
                                                                                                                                as
                                                                                                                                isize)).channelIdx
                                                                                                as
                                                                                                usize]).freqScale
                                                 * 100.0f32) as s32,
                                            (*gAudioContext.seqPlayers[SEQ_PLAYER_SFX
                                                                           as
                                                                           libc::c_int
                                                                           as
                                                                           usize].channels[(*gSoundBanks[k
                                                                                                             as
                                                                                                             usize].offset(gActiveSounds[k
                                                                                                                                             as
                                                                                                                                             usize][k2
                                                                                                                                                        as
                                                                                                                                                        usize].entryIndex
                                                                                                                               as
                                                                                                                               isize)).channelIdx
                                                                                               as
                                                                                               usize]).reverb
                                                as libc::c_int,
                                            (*gSoundBanks[k as
                                                              usize].offset(gActiveSounds[k
                                                                                              as
                                                                                              usize][k2
                                                                                                         as
                                                                                                         usize].entryIndex
                                                                                as
                                                                                isize)).sfxId
                                                as libc::c_int);
                        } else {
                            GfxPrint_Printf(printer,
                                            b"FF ----- ----- ----- --- --- ----\x00"
                                                as *const u8 as
                                                *const libc::c_char);
                        }
                    } else if sAudioIntInfoBankPage[k as usize] as libc::c_int
                                  == 4 as libc::c_int {
                        if gActiveSounds[k as usize][k2 as usize].entryIndex
                               as libc::c_int != 0xff as libc::c_int &&
                               ((*gSoundBanks[k as
                                                  usize].offset(gActiveSounds[k
                                                                                  as
                                                                                  usize][k2
                                                                                             as
                                                                                             usize].entryIndex
                                                                    as
                                                                    isize)).state
                                    as libc::c_int ==
                                    SFX_STATE_PLAYING_1 as libc::c_int ||
                                    (*gSoundBanks[k as
                                                      usize].offset(gActiveSounds[k
                                                                                      as
                                                                                      usize][k2
                                                                                                 as
                                                                                                 usize].entryIndex
                                                                        as
                                                                        isize)).state
                                        as libc::c_int ==
                                        SFX_STATE_PLAYING_2 as libc::c_int) {
                            GfxPrint_Printf(printer,
                                            b"%2X %04X\x00" as *const u8 as
                                                *const libc::c_char,
                                            gActiveSounds[k as
                                                              usize][k2 as
                                                                         usize].entryIndex
                                                as libc::c_int,
                                            (*gSoundBanks[k as
                                                              usize].offset(gActiveSounds[k
                                                                                              as
                                                                                              usize][k2
                                                                                                         as
                                                                                                         usize].entryIndex
                                                                                as
                                                                                isize)).sfxId
                                                as libc::c_int);
                        } else {
                            GfxPrint_Printf(printer,
                                            b"FF ----\x00" as *const u8 as
                                                *const libc::c_char);
                        }
                    }
                    if sAudioIntInfoBankPage[k as usize] as libc::c_int !=
                           0 as libc::c_int {
                        ind += 1
                    }
                    k2 += 1
                }
                ind += 1;
                k += 1
            }
        }
        13 => {
            GfxPrint_SetPos(printer, 2 as libc::c_int,
                            4 as libc::c_int +
                                sAudioScrPrtSel as libc::c_int);
            GfxPrint_SetColor(printer,
                              (((sAudioDebugTextColor as libc::c_int &
                                     4 as libc::c_int) >> 2 as libc::c_int) *
                                   255 as libc::c_int) as u32_0,
                              (((sAudioDebugTextColor as libc::c_int &
                                     2 as libc::c_int) >> 1 as libc::c_int) *
                                   255 as libc::c_int) as u32_0,
                              ((sAudioDebugTextColor as libc::c_int &
                                    1 as libc::c_int) * 255 as libc::c_int) as
                                  u32_0, 255 as libc::c_int as u32_0);
            GfxPrint_Printf(printer,
                            b"*\x00" as *const u8 as *const libc::c_char);
            GfxPrint_SetColor(printer,
                              (((sAudioDebugTextColor as libc::c_int &
                                     4 as libc::c_int) >> 2 as libc::c_int) *
                                   255 as libc::c_int) as u32_0,
                              (((sAudioDebugTextColor as libc::c_int &
                                     2 as libc::c_int) >> 1 as libc::c_int) *
                                   255 as libc::c_int) as u32_0,
                              ((sAudioDebugTextColor as libc::c_int &
                                    1 as libc::c_int) * 255 as libc::c_int) as
                                  u32_0, 255 as libc::c_int as u32_0);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 4 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"Swicth  : %d\x00" as *const u8 as
                                *const libc::c_char,
                            sAudioScrPrtWork[0 as libc::c_int as usize] as
                                libc::c_int);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 5 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"Lines   : %d\x00" as *const u8 as
                                *const libc::c_char,
                            sAudioScrPrtWork[1 as libc::c_int as usize] as
                                libc::c_int + 1 as libc::c_int);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 6 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"Color   : %d\x00" as *const u8 as
                                *const libc::c_char,
                            sAudioScrPrtWork[2 as libc::c_int as usize] as
                                libc::c_int);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 7 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"%s  : %d\x00" as *const u8 as
                                *const libc::c_char,
                            sSoundBankNames[0 as libc::c_int as
                                                usize].as_mut_ptr(),
                            sAudioScrPrtWork[3 as libc::c_int as usize] as
                                libc::c_int);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 8 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"%s    : %d\x00" as *const u8 as
                                *const libc::c_char,
                            sSoundBankNames[1 as libc::c_int as
                                                usize].as_mut_ptr(),
                            sAudioScrPrtWork[4 as libc::c_int as usize] as
                                libc::c_int);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 9 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"ENVRONM : %d\x00" as *const u8 as
                                *const libc::c_char,
                            sAudioScrPrtWork[5 as libc::c_int as usize] as
                                libc::c_int);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 10 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"%s   : %d\x00" as *const u8 as
                                *const libc::c_char,
                            sSoundBankNames[3 as libc::c_int as
                                                usize].as_mut_ptr(),
                            sAudioScrPrtWork[6 as libc::c_int as usize] as
                                libc::c_int);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 11 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"%s  : %d\x00" as *const u8 as
                                *const libc::c_char,
                            sSoundBankNames[4 as libc::c_int as
                                                usize].as_mut_ptr(),
                            sAudioScrPrtWork[7 as libc::c_int as usize] as
                                libc::c_int);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 12 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"%s : %d\x00" as *const u8 as
                                *const libc::c_char,
                            sSoundBankNames[5 as libc::c_int as
                                                usize].as_mut_ptr(),
                            sAudioScrPrtWork[8 as libc::c_int as usize] as
                                libc::c_int);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 13 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"%s    : %d\x00" as *const u8 as
                                *const libc::c_char,
                            sSoundBankNames[6 as libc::c_int as
                                                usize].as_mut_ptr(),
                            sAudioScrPrtWork[9 as libc::c_int as usize] as
                                libc::c_int);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 14 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"SEQ ENT : %d\x00" as *const u8 as
                                *const libc::c_char,
                            sAudioScrPrtWork[10 as libc::c_int as usize] as
                                libc::c_int);
        }
        8 => {
            GfxPrint_SetPos(printer, 3 as libc::c_int, 4 as libc::c_int);
            GfxPrint_SetColor(printer,
                              (((sAudioDebugTextColor as libc::c_int &
                                     4 as libc::c_int) >> 2 as libc::c_int) *
                                   255 as libc::c_int) as u32_0,
                              (((sAudioDebugTextColor as libc::c_int &
                                     2 as libc::c_int) >> 1 as libc::c_int) *
                                   255 as libc::c_int) as u32_0,
                              ((sAudioDebugTextColor as libc::c_int &
                                    1 as libc::c_int) * 255 as libc::c_int) as
                                  u32_0, 255 as libc::c_int as u32_0);
            if gAudioSfxSwapOff != 0 {
                GfxPrint_Printf(printer,
                                b"SWAP OFF\x00" as *const u8 as
                                    *const libc::c_char);
            }
            if sAudioSfxSwapIsEditing as libc::c_int == 0 as libc::c_int {
                GfxPrint_SetColor(printer,
                                  (((sAudioDebugTextColor as libc::c_int &
                                         4 as libc::c_int) >>
                                        2 as libc::c_int) *
                                       255 as libc::c_int) as u32_0,
                                  (((sAudioDebugTextColor as libc::c_int &
                                         2 as libc::c_int) >>
                                        1 as libc::c_int) *
                                       255 as libc::c_int) as u32_0,
                                  ((sAudioDebugTextColor as libc::c_int &
                                        1 as libc::c_int) *
                                       255 as libc::c_int) as u32_0,
                                  255 as libc::c_int as u32_0);
            } else {
                GfxPrint_SetColor(printer,
                                  (((sAudioDebugTextColor as libc::c_int &
                                         4 as libc::c_int) >>
                                        2 as libc::c_int) *
                                       127 as libc::c_int) as u32_0,
                                  (((sAudioDebugTextColor as libc::c_int &
                                         2 as libc::c_int) >>
                                        1 as libc::c_int) *
                                       127 as libc::c_int) as u32_0,
                                  ((sAudioDebugTextColor as libc::c_int &
                                        1 as libc::c_int) *
                                       127 as libc::c_int) as u32_0,
                                  255 as libc::c_int as u32_0);
            }
            GfxPrint_SetPos(printer, 2 as libc::c_int,
                            6 as libc::c_int +
                                sAudioSfxSwapSel as libc::c_int);
            GfxPrint_Printf(printer,
                            b"*\x00" as *const u8 as *const libc::c_char);
            ctr = sAudioSfxSwapNibbleSel;
            if sAudioSfxSwapNibbleSel as libc::c_int >= 4 as libc::c_int {
                ctr = ctr.wrapping_add(1)
            }
            if sAudioSfxSwapIsEditing as libc::c_int == 1 as libc::c_int {
                GfxPrint_SetColor(printer,
                                  (((sAudioDebugTextColor as libc::c_int &
                                         4 as libc::c_int) >>
                                        2 as libc::c_int) *
                                       255 as libc::c_int) as u32_0,
                                  (((sAudioDebugTextColor as libc::c_int &
                                         2 as libc::c_int) >>
                                        1 as libc::c_int) *
                                       255 as libc::c_int) as u32_0,
                                  ((sAudioDebugTextColor as libc::c_int &
                                        1 as libc::c_int) *
                                       255 as libc::c_int) as u32_0,
                                  255 as libc::c_int as u32_0);
                GfxPrint_SetPos(printer,
                                3 as libc::c_int + ctr as libc::c_int,
                                5 as libc::c_int);
                GfxPrint_Printf(printer,
                                b"V\x00" as *const u8 as *const libc::c_char);
            }
            i = 0 as libc::c_int as u8_0;
            while (i as libc::c_int) < 10 as libc::c_int {
                if i as libc::c_int == sAudioSfxSwapSel as libc::c_int {
                    if sAudioSfxSwapIsEditing as libc::c_int ==
                           0 as libc::c_int {
                        GfxPrint_SetColor(printer,
                                          (((sAudioDebugTextColor as
                                                 libc::c_int &
                                                 4 as libc::c_int) >>
                                                2 as libc::c_int) *
                                               192 as libc::c_int) as u32_0,
                                          (((sAudioDebugTextColor as
                                                 libc::c_int &
                                                 2 as libc::c_int) >>
                                                1 as libc::c_int) *
                                               192 as libc::c_int) as u32_0,
                                          ((sAudioDebugTextColor as
                                                libc::c_int &
                                                1 as libc::c_int) *
                                               192 as libc::c_int) as u32_0,
                                          255 as libc::c_int as u32_0);
                    } else {
                        GfxPrint_SetColor(printer,
                                          (((sAudioDebugTextColor as
                                                 libc::c_int &
                                                 4 as libc::c_int) >>
                                                2 as libc::c_int) *
                                               255 as libc::c_int) as u32_0,
                                          (((sAudioDebugTextColor as
                                                 libc::c_int &
                                                 2 as libc::c_int) >>
                                                1 as libc::c_int) *
                                               255 as libc::c_int) as u32_0,
                                          ((sAudioDebugTextColor as
                                                libc::c_int &
                                                1 as libc::c_int) *
                                               255 as libc::c_int) as u32_0,
                                          255 as libc::c_int as u32_0);
                    }
                } else if sAudioSfxSwapIsEditing as libc::c_int ==
                              0 as libc::c_int {
                    GfxPrint_SetColor(printer,
                                      (((sAudioDebugTextColor as libc::c_int &
                                             4 as libc::c_int) >>
                                            2 as libc::c_int) *
                                           144 as libc::c_int) as u32_0,
                                      (((sAudioDebugTextColor as libc::c_int &
                                             2 as libc::c_int) >>
                                            1 as libc::c_int) *
                                           144 as libc::c_int) as u32_0,
                                      ((sAudioDebugTextColor as libc::c_int &
                                            1 as libc::c_int) *
                                           144 as libc::c_int) as u32_0,
                                      255 as libc::c_int as u32_0);
                } else {
                    GfxPrint_SetColor(printer,
                                      (((sAudioDebugTextColor as libc::c_int &
                                             4 as libc::c_int) >>
                                            2 as libc::c_int) *
                                           96 as libc::c_int) as u32_0,
                                      (((sAudioDebugTextColor as libc::c_int &
                                             2 as libc::c_int) >>
                                            1 as libc::c_int) *
                                           96 as libc::c_int) as u32_0,
                                      ((sAudioDebugTextColor as libc::c_int &
                                            1 as libc::c_int) *
                                           96 as libc::c_int) as u32_0,
                                      255 as libc::c_int as u32_0);
                }
                GfxPrint_SetPos(printer, 3 as libc::c_int,
                                6 as libc::c_int + i as libc::c_int);
                GfxPrint_Printf(printer,
                                b"%04x %04x %s\x00" as *const u8 as
                                    *const libc::c_char,
                                gAudioSfxSwapSource[i as usize] as
                                    libc::c_int,
                                gAudioSfxSwapTarget[i as usize] as
                                    libc::c_int,
                                sAudioSfxSwapModeNames[gAudioSfxSwapMode[i as
                                                                             usize]
                                                           as
                                                           usize].as_mut_ptr());
                i = i.wrapping_add(1)
            }
        }
        5 => {
            GfxPrint_SetPos(printer, 3 as libc::c_int, 4 as libc::c_int);
            GfxPrint_SetColor(printer,
                              (((sAudioDebugTextColor as libc::c_int &
                                     4 as libc::c_int) >> 2 as libc::c_int) *
                                   255 as libc::c_int) as u32_0,
                              (((sAudioDebugTextColor as libc::c_int &
                                     2 as libc::c_int) >> 1 as libc::c_int) *
                                   255 as libc::c_int) as u32_0,
                              ((sAudioDebugTextColor as libc::c_int &
                                    1 as libc::c_int) * 255 as libc::c_int) as
                                  u32_0, 255 as libc::c_int as u32_0);
            GfxPrint_Printf(printer,
                            b"Group Track:%d\x00" as *const u8 as
                                *const libc::c_char,
                            sAudioSubTrackInfoPlayerSel as libc::c_int);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 5 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"Sub Track  :%d\x00" as *const u8 as
                                *const libc::c_char,
                            sAudioSubTrackInfoChannelSel as libc::c_int);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 6 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"TRK NO. \x00" as *const u8 as
                                *const libc::c_char);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 7 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"ENTRY   \x00" as *const u8 as
                                *const libc::c_char);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 8 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"MUTE    \x00" as *const u8 as
                                *const libc::c_char);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 9 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"OPENNOTE\x00" as *const u8 as
                                *const libc::c_char);
            ctr2 = 0 as libc::c_int as u8_0;
            i = 0 as libc::c_int as u8_0;
            while (i as libc::c_int) < 16 as libc::c_int {
                if i as libc::c_int ==
                       sAudioSubTrackInfoChannelSel as libc::c_int {
                    GfxPrint_SetColor(printer,
                                      (((sAudioDebugTextColor as libc::c_int &
                                             4 as libc::c_int) >>
                                            2 as libc::c_int) *
                                           255 as libc::c_int) as u32_0,
                                      (((sAudioDebugTextColor as libc::c_int &
                                             2 as libc::c_int) >>
                                            1 as libc::c_int) *
                                           255 as libc::c_int) as u32_0,
                                      ((sAudioDebugTextColor as libc::c_int &
                                            1 as libc::c_int) *
                                           255 as libc::c_int) as u32_0,
                                      255 as libc::c_int as u32_0);
                } else {
                    GfxPrint_SetColor(printer,
                                      (((sAudioDebugTextColor as libc::c_int &
                                             4 as libc::c_int) >>
                                            2 as libc::c_int) *
                                           200 as libc::c_int) as u32_0,
                                      (((sAudioDebugTextColor as libc::c_int &
                                             2 as libc::c_int) >>
                                            1 as libc::c_int) *
                                           200 as libc::c_int) as u32_0,
                                      ((sAudioDebugTextColor as libc::c_int &
                                            1 as libc::c_int) *
                                           200 as libc::c_int) as u32_0,
                                      255 as libc::c_int as u32_0);
                }
                GfxPrint_SetPos(printer, 15 as libc::c_int + i as libc::c_int,
                                6 as libc::c_int);
                GfxPrint_Printf(printer,
                                b"%1X\x00" as *const u8 as
                                    *const libc::c_char, i as libc::c_int);
                GfxPrint_SetPos(printer, 15 as libc::c_int + i as libc::c_int,
                                7 as libc::c_int);
                if (*gAudioContext.seqPlayers[sAudioSubTrackInfoPlayerSel as
                                                  usize].channels[i as
                                                                      usize]).enabled()
                       != 0 {
                    GfxPrint_Printf(printer,
                                    b"O\x00" as *const u8 as
                                        *const libc::c_char);
                } else {
                    GfxPrint_Printf(printer,
                                    b"X\x00" as *const u8 as
                                        *const libc::c_char);
                }
                GfxPrint_SetPos(printer, 15 as libc::c_int + i as libc::c_int,
                                8 as libc::c_int);
                if (*gAudioContext.seqPlayers[sAudioSubTrackInfoPlayerSel as
                                                  usize].channels[i as
                                                                      usize]).stopSomething2()
                       != 0 {
                    GfxPrint_Printf(printer,
                                    b"O\x00" as *const u8 as
                                        *const libc::c_char);
                } else {
                    GfxPrint_Printf(printer,
                                    b"X\x00" as *const u8 as
                                        *const libc::c_char);
                }
                GfxPrint_SetPos(printer, 15 as libc::c_int + i as libc::c_int,
                                9 as libc::c_int);
                ctr = 0 as libc::c_int as u8_0;
                j = 0 as libc::c_int as u8_0;
                while (j as libc::c_int) < 4 as libc::c_int {
                    if !(*gAudioContext.seqPlayers[sAudioSubTrackInfoPlayerSel
                                                       as
                                                       usize].channels[i as
                                                                           usize]).layers[j
                                                                                              as
                                                                                              usize].is_null()
                       {
                        ctr = ctr.wrapping_add(1)
                    }
                    j = j.wrapping_add(1)
                }
                GfxPrint_Printf(printer,
                                b"%1X\x00" as *const u8 as
                                    *const libc::c_char, ctr as libc::c_int);
                ctr2 = (ctr2 as libc::c_int + ctr as libc::c_int) as u8_0;
                i = i.wrapping_add(1)
            }
            GfxPrint_SetColor(printer,
                              (((sAudioDebugTextColor as libc::c_int &
                                     4 as libc::c_int) >> 2 as libc::c_int) *
                                   255 as libc::c_int) as u32_0,
                              (((sAudioDebugTextColor as libc::c_int &
                                     2 as libc::c_int) >> 1 as libc::c_int) *
                                   255 as libc::c_int) as u32_0,
                              ((sAudioDebugTextColor as libc::c_int &
                                    1 as libc::c_int) * 255 as libc::c_int) as
                                  u32_0, 255 as libc::c_int as u32_0);
            if (sSeqPlayerPeakNumLayers[sAudioSubTrackInfoPlayerSel as usize]
                    as libc::c_int) < ctr2 as libc::c_int {
                sSeqPlayerPeakNumLayers[sAudioSubTrackInfoPlayerSel as usize]
                    = ctr2
            }
            GfxPrint_SetPos(printer, 16 as libc::c_int + i as libc::c_int,
                            9 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"%2d,%2d\x00" as *const u8 as
                                *const libc::c_char, ctr2 as libc::c_int,
                            sSeqPlayerPeakNumLayers[sAudioSubTrackInfoPlayerSel
                                                        as usize] as
                                libc::c_int);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 11 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"VOL     \x00" as *const u8 as
                                *const libc::c_char);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 12 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"E VOL   \x00" as *const u8 as
                                *const libc::c_char);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 13 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"BANK ID \x00" as *const u8 as
                                *const libc::c_char);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 14 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"PROG    \x00" as *const u8 as
                                *const libc::c_char);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 15 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"PAN    \x00" as *const u8 as
                                *const libc::c_char);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 16 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"PANPOW  \x00" as *const u8 as
                                *const libc::c_char);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 17 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"FXMIX   \x00" as *const u8 as
                                *const libc::c_char);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 18 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"PRIO    \x00" as *const u8 as
                                *const libc::c_char);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 19 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"VIB PIT \x00" as *const u8 as
                                *const libc::c_char);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 20 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"VIB DEP \x00" as *const u8 as
                                *const libc::c_char);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 21 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"TUNE    \x00" as *const u8 as
                                *const libc::c_char);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 22 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"TUNE    \x00" as *const u8 as
                                *const libc::c_char);
            i = 0 as libc::c_int as u8_0;
            while (i as libc::c_int) < 8 as libc::c_int {
                GfxPrint_SetPos(printer,
                                15 as libc::c_int +
                                    3 as libc::c_int * i as libc::c_int,
                                22 as libc::c_int);
                GfxPrint_Printf(printer,
                                b"%02X \x00" as *const u8 as
                                    *const libc::c_char,
                                (*gAudioContext.seqPlayers[sAudioSubTrackInfoPlayerSel
                                                               as
                                                               usize].channels[sAudioSubTrackInfoChannelSel
                                                                                   as
                                                                                   usize]).soundScriptIO[i
                                                                                                             as
                                                                                                             usize]
                                    as u8_0 as libc::c_int);
                i = i.wrapping_add(1)
            }
            if (*gAudioContext.seqPlayers[sAudioSubTrackInfoPlayerSel as
                                              usize].channels[sAudioSubTrackInfoChannelSel
                                                                  as
                                                                  usize]).enabled()
                   != 0 {
                GfxPrint_SetPos(printer, 15 as libc::c_int,
                                11 as libc::c_int);
                GfxPrint_Printf(printer,
                                b"%d\x00" as *const u8 as *const libc::c_char,
                                ((*gAudioContext.seqPlayers[sAudioSubTrackInfoPlayerSel
                                                                as
                                                                usize].channels[sAudioSubTrackInfoChannelSel
                                                                                    as
                                                                                    usize]).volume
                                     as libc::c_double * 127.1f64) as u8_0 as
                                    libc::c_int);
                GfxPrint_SetPos(printer, 15 as libc::c_int,
                                12 as libc::c_int);
                GfxPrint_Printf(printer,
                                b"%d\x00" as *const u8 as *const libc::c_char,
                                ((*gAudioContext.seqPlayers[sAudioSubTrackInfoPlayerSel
                                                                as
                                                                usize].channels[sAudioSubTrackInfoChannelSel
                                                                                    as
                                                                                    usize]).volumeScale
                                     as libc::c_double * 127.1f64) as u8_0 as
                                    libc::c_int);
                GfxPrint_SetPos(printer, 15 as libc::c_int,
                                13 as libc::c_int);
                GfxPrint_Printf(printer,
                                b"%X\x00" as *const u8 as *const libc::c_char,
                                (*gAudioContext.seqPlayers[sAudioSubTrackInfoPlayerSel
                                                               as
                                                               usize].channels[sAudioSubTrackInfoChannelSel
                                                                                   as
                                                                                   usize]).fontId
                                    as libc::c_int);
                ctr =
                    (*gAudioContext.seqPlayers[sAudioSubTrackInfoPlayerSel as
                                                   usize].channels[sAudioSubTrackInfoChannelSel
                                                                       as
                                                                       usize]).instOrWave
                        as u8_0;
                if ctr as libc::c_int == 0 as libc::c_int {
                    ctr2 = 0x7f as libc::c_int as u8_0
                } else if (ctr as libc::c_int) < 0x80 as libc::c_int {
                    ctr2 = (ctr as libc::c_int - 1 as libc::c_int) as u8_0
                } else { ctr2 = ctr }
                GfxPrint_SetPos(printer, 15 as libc::c_int,
                                14 as libc::c_int);
                GfxPrint_Printf(printer,
                                b"%d\x00" as *const u8 as *const libc::c_char,
                                ctr2 as libc::c_int);
                GfxPrint_SetPos(printer, 15 as libc::c_int,
                                15 as libc::c_int);
                GfxPrint_Printf(printer,
                                b"%d\x00" as *const u8 as *const libc::c_char,
                                (*gAudioContext.seqPlayers[sAudioSubTrackInfoPlayerSel
                                                               as
                                                               usize].channels[sAudioSubTrackInfoChannelSel
                                                                                   as
                                                                                   usize]).newPan
                                    as libc::c_int);
                GfxPrint_SetPos(printer, 15 as libc::c_int,
                                16 as libc::c_int);
                GfxPrint_Printf(printer,
                                b"%d\x00" as *const u8 as *const libc::c_char,
                                (*gAudioContext.seqPlayers[sAudioSubTrackInfoPlayerSel
                                                               as
                                                               usize].channels[sAudioSubTrackInfoChannelSel
                                                                                   as
                                                                                   usize]).panChannelWeight
                                    as libc::c_int);
                GfxPrint_SetPos(printer, 15 as libc::c_int,
                                17 as libc::c_int);
                GfxPrint_Printf(printer,
                                b"%d\x00" as *const u8 as *const libc::c_char,
                                (*gAudioContext.seqPlayers[sAudioSubTrackInfoPlayerSel
                                                               as
                                                               usize].channels[sAudioSubTrackInfoChannelSel
                                                                                   as
                                                                                   usize]).reverb
                                    as libc::c_int);
                GfxPrint_SetPos(printer, 15 as libc::c_int,
                                18 as libc::c_int);
                GfxPrint_Printf(printer,
                                b"%d\x00" as *const u8 as *const libc::c_char,
                                (*gAudioContext.seqPlayers[sAudioSubTrackInfoPlayerSel
                                                               as
                                                               usize].channels[sAudioSubTrackInfoChannelSel
                                                                                   as
                                                                                   usize]).notePriority
                                    as libc::c_int);
                GfxPrint_SetPos(printer, 15 as libc::c_int,
                                19 as libc::c_int);
                GfxPrint_Printf(printer,
                                b"%d\x00" as *const u8 as *const libc::c_char,
                                ((*gAudioContext.seqPlayers[sAudioSubTrackInfoPlayerSel
                                                                as
                                                                usize].channels[sAudioSubTrackInfoChannelSel
                                                                                    as
                                                                                    usize]).vibratoRateTarget
                                     as libc::c_int / 32 as libc::c_int) as
                                    u8_0 as libc::c_int);
                GfxPrint_SetPos(printer, 15 as libc::c_int,
                                20 as libc::c_int);
                GfxPrint_Printf(printer,
                                b"%d\x00" as *const u8 as *const libc::c_char,
                                ((*gAudioContext.seqPlayers[sAudioSubTrackInfoPlayerSel
                                                                as
                                                                usize].channels[sAudioSubTrackInfoChannelSel
                                                                                    as
                                                                                    usize]).vibratoExtentTarget
                                     as libc::c_int / 8 as libc::c_int) as
                                    u8_0 as libc::c_int);
                GfxPrint_SetPos(printer, 15 as libc::c_int,
                                21 as libc::c_int);
                GfxPrint_Printf(printer,
                                b"%d\x00" as *const u8 as *const libc::c_char,
                                ((*gAudioContext.seqPlayers[sAudioSubTrackInfoPlayerSel
                                                                as
                                                                usize].channels[sAudioSubTrackInfoChannelSel
                                                                                    as
                                                                                    usize]).freqScale
                                     * 100 as libc::c_int as libc::c_float) as
                                    u16_0 as libc::c_int);
            }
        }
        3 => {
            GfxPrint_SetColor(printer,
                              (((sAudioDebugTextColor as libc::c_int &
                                     4 as libc::c_int) >> 2 as libc::c_int) *
                                   255 as libc::c_int) as u32_0,
                              (((sAudioDebugTextColor as libc::c_int &
                                     2 as libc::c_int) >> 1 as libc::c_int) *
                                   255 as libc::c_int) as u32_0,
                              ((sAudioDebugTextColor as libc::c_int &
                                    1 as libc::c_int) * 255 as libc::c_int) as
                                  u32_0, 255 as libc::c_int as u32_0);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 4 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"TOTAL  %d\x00" as *const u8 as
                                *const libc::c_char, D_8014A6C4.heapSize);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 5 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"DRIVER %05X / %05X\x00" as *const u8 as
                                *const libc::c_char,
                            gAudioContext.notesAndBuffersPool.cur.wrapping_offset_from(gAudioContext.notesAndBuffersPool.start)
                                as libc::c_int,
                            gAudioContext.notesAndBuffersPool.size);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 6 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"AT-SEQ %02X-%02X (%05X-%05X / %05X)\x00" as
                                *const u8 as *const libc::c_char,
                            gAudioContext.seqCache.temporary.entries[0 as
                                                                         libc::c_int
                                                                         as
                                                                         usize].id
                                as u8_0 as libc::c_int,
                            gAudioContext.seqCache.temporary.entries[1 as
                                                                         libc::c_int
                                                                         as
                                                                         usize].id
                                as u8_0 as libc::c_int,
                            gAudioContext.seqCache.temporary.entries[0 as
                                                                         libc::c_int
                                                                         as
                                                                         usize].size,
                            gAudioContext.seqCache.temporary.entries[1 as
                                                                         libc::c_int
                                                                         as
                                                                         usize].size,
                            gAudioContext.seqCache.temporary.pool.size);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 7 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"AT-BNK %02X-%02X (%05X-%05X / %05X)\x00" as
                                *const u8 as *const libc::c_char,
                            gAudioContext.fontCache.temporary.entries[0 as
                                                                          libc::c_int
                                                                          as
                                                                          usize].id
                                as u8_0 as libc::c_int,
                            gAudioContext.fontCache.temporary.entries[1 as
                                                                          libc::c_int
                                                                          as
                                                                          usize].id
                                as u8_0 as libc::c_int,
                            gAudioContext.fontCache.temporary.entries[0 as
                                                                          libc::c_int
                                                                          as
                                                                          usize].size,
                            gAudioContext.fontCache.temporary.entries[1 as
                                                                          libc::c_int
                                                                          as
                                                                          usize].size,
                            gAudioContext.fontCache.temporary.pool.size);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 8 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"ST-SEQ %02Xseqs  (%05X / %06X)\x00" as *const u8
                                as *const libc::c_char,
                            gAudioContext.seqCache.persistent.numEntries,
                            gAudioContext.seqCache.persistent.pool.cur.wrapping_offset_from(gAudioContext.seqCache.persistent.pool.start)
                                as libc::c_int,
                            gAudioContext.seqCache.persistent.pool.size);
            k = 0 as libc::c_int as s8;
            while (k as u32_0) < gAudioContext.seqCache.persistent.numEntries
                  {
                GfxPrint_SetPos(printer,
                                3 as libc::c_int +
                                    3 as libc::c_int * k as libc::c_int,
                                9 as libc::c_int);
                GfxPrint_Printf(printer,
                                b"%02x\x00" as *const u8 as
                                    *const libc::c_char,
                                gAudioContext.seqCache.persistent.entries[k as
                                                                              usize].id
                                    as libc::c_int);
                k += 1
            }
            GfxPrint_SetPos(printer, 3 as libc::c_int, 10 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"ST-BNK %02Xbanks (%05X / %06X)\x00" as *const u8
                                as *const libc::c_char,
                            gAudioContext.fontCache.persistent.numEntries,
                            gAudioContext.fontCache.persistent.pool.cur.wrapping_offset_from(gAudioContext.fontCache.persistent.pool.start)
                                as libc::c_int,
                            gAudioContext.fontCache.persistent.pool.size);
            k = 0 as libc::c_int as s8;
            while (k as u32_0) < gAudioContext.fontCache.persistent.numEntries
                  {
                GfxPrint_SetPos(printer,
                                3 as libc::c_int +
                                    3 as libc::c_int * k as libc::c_int,
                                11 as libc::c_int);
                GfxPrint_Printf(printer,
                                b"%02x\x00" as *const u8 as
                                    *const libc::c_char,
                                gAudioContext.fontCache.persistent.entries[k
                                                                               as
                                                                               usize].id
                                    as libc::c_int);
                k += 1
            }
            GfxPrint_SetPos(printer, 3 as libc::c_int, 12 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"E-MEM  %05X / %05X\x00" as *const u8 as
                                *const libc::c_char,
                            gAudioContext.permanentPool.cur.wrapping_offset_from(gAudioContext.permanentPool.start)
                                as libc::c_int,
                            gAudioContext.permanentPool.size);
        }
        9 => {
            GfxPrint_SetColor(printer,
                              (((sAudioDebugTextColor as libc::c_int &
                                     4 as libc::c_int) >> 2 as libc::c_int) *
                                   255 as libc::c_int) as u32_0,
                              (((sAudioDebugTextColor as libc::c_int &
                                     2 as libc::c_int) >> 1 as libc::c_int) *
                                   255 as libc::c_int) as u32_0,
                              ((sAudioDebugTextColor as libc::c_int &
                                    1 as libc::c_int) * 255 as libc::c_int) as
                                  u32_0, 255 as libc::c_int as u32_0);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 4 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"BGM No.    %02X\x00" as *const u8 as
                                *const libc::c_char,
                            sAudioBlkChgBgmWork[0 as libc::c_int as usize] as
                                libc::c_int);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 5 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"SCENE SET  %02X %s\x00" as *const u8 as
                                *const libc::c_char,
                            sAudioBlkChgBgmWork[1 as libc::c_int as usize] as
                                libc::c_int,
                            sAudioSceneNames[sAudioBlkChgBgmWork[1 as
                                                                     libc::c_int
                                                                     as usize]
                                                 as usize].as_mut_ptr());
            GfxPrint_SetColor(printer,
                              (((sAudioDebugTextColor as libc::c_int &
                                     4 as libc::c_int) >> 2 as libc::c_int) *
                                   0x64 as libc::c_int) as u32_0,
                              (((sAudioDebugTextColor as libc::c_int &
                                     2 as libc::c_int) >> 1 as libc::c_int) *
                                   255 as libc::c_int) as u32_0,
                              ((sAudioDebugTextColor as libc::c_int &
                                    1 as libc::c_int) * 0x64 as libc::c_int)
                                  as u32_0, 255 as libc::c_int as u32_0);
            GfxPrint_SetPos(printer, 2 as libc::c_int,
                            4 as libc::c_int +
                                sAudioBlkChgBgmSel as libc::c_int);
            GfxPrint_Printf(printer,
                            b"*\x00" as *const u8 as *const libc::c_char);
            GfxPrint_SetColor(printer,
                              (((sAudioDebugTextColor as libc::c_int &
                                     4 as libc::c_int) >> 2 as libc::c_int) *
                                   255 as libc::c_int) as u32_0,
                              (((sAudioDebugTextColor as libc::c_int &
                                     2 as libc::c_int) >> 1 as libc::c_int) *
                                   255 as libc::c_int) as u32_0,
                              ((sAudioDebugTextColor as libc::c_int &
                                    1 as libc::c_int) * 255 as libc::c_int) as
                                  u32_0, 255 as libc::c_int as u32_0);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 7 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"NEXT SCENE %02X %s\x00" as *const u8 as
                                *const libc::c_char,
                            gAudioContext.seqPlayers[SEQ_PLAYER_BGM_MAIN as
                                                         libc::c_int as
                                                         usize].soundScriptIO[2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  usize]
                                as u8_0 as libc::c_int,
                            sAudioSceneNames[gAudioContext.seqPlayers[SEQ_PLAYER_BGM_MAIN
                                                                          as
                                                                          libc::c_int
                                                                          as
                                                                          usize].soundScriptIO[2
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   usize]
                                                 as u8_0 as
                                                 usize].as_mut_ptr());
            GfxPrint_SetPos(printer, 3 as libc::c_int, 8 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"NOW SCENE  %02X %s\x00" as *const u8 as
                                *const libc::c_char,
                            gAudioContext.seqPlayers[SEQ_PLAYER_BGM_MAIN as
                                                         libc::c_int as
                                                         usize].soundScriptIO[4
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  usize]
                                as u8_0 as libc::c_int,
                            sAudioSceneNames[gAudioContext.seqPlayers[SEQ_PLAYER_BGM_MAIN
                                                                          as
                                                                          libc::c_int
                                                                          as
                                                                          usize].soundScriptIO[4
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   usize]
                                                 as u8_0 as
                                                 usize].as_mut_ptr());
            GfxPrint_SetPos(printer, 3 as libc::c_int, 9 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"NOW BLOCK  %02X\x00" as *const u8 as
                                *const libc::c_char,
                            gAudioContext.seqPlayers[SEQ_PLAYER_BGM_MAIN as
                                                         libc::c_int as
                                                         usize].soundScriptIO[5
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  usize]
                                as libc::c_int + 1 as libc::c_int &
                                0xff as libc::c_int);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 11 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"PORT\x00" as *const u8 as *const libc::c_char);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 12 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"%02X %02X %02X %02X\x00" as *const u8 as
                                *const libc::c_char,
                            gAudioContext.seqPlayers[SEQ_PLAYER_BGM_MAIN as
                                                         libc::c_int as
                                                         usize].soundScriptIO[0
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  usize]
                                as u8_0 as libc::c_int,
                            gAudioContext.seqPlayers[SEQ_PLAYER_BGM_MAIN as
                                                         libc::c_int as
                                                         usize].soundScriptIO[1
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  usize]
                                as u8_0 as libc::c_int,
                            gAudioContext.seqPlayers[SEQ_PLAYER_BGM_MAIN as
                                                         libc::c_int as
                                                         usize].soundScriptIO[2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  usize]
                                as u8_0 as libc::c_int,
                            gAudioContext.seqPlayers[SEQ_PLAYER_BGM_MAIN as
                                                         libc::c_int as
                                                         usize].soundScriptIO[3
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  usize]
                                as u8_0 as libc::c_int);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 13 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"%02X %02X %02X %02X\x00" as *const u8 as
                                *const libc::c_char,
                            gAudioContext.seqPlayers[SEQ_PLAYER_BGM_MAIN as
                                                         libc::c_int as
                                                         usize].soundScriptIO[4
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  usize]
                                as u8_0 as libc::c_int,
                            gAudioContext.seqPlayers[SEQ_PLAYER_BGM_MAIN as
                                                         libc::c_int as
                                                         usize].soundScriptIO[5
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  usize]
                                as u8_0 as libc::c_int,
                            gAudioContext.seqPlayers[SEQ_PLAYER_BGM_MAIN as
                                                         libc::c_int as
                                                         usize].soundScriptIO[6
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  usize]
                                as u8_0 as libc::c_int,
                            gAudioContext.seqPlayers[SEQ_PLAYER_BGM_MAIN as
                                                         libc::c_int as
                                                         usize].soundScriptIO[7
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  usize]
                                as u8_0 as libc::c_int);
        }
        11 => {
            GfxPrint_SetColor(printer,
                              (((sAudioDebugTextColor as libc::c_int &
                                     4 as libc::c_int) >> 2 as libc::c_int) *
                                   255 as libc::c_int) as u32_0,
                              (((sAudioDebugTextColor as libc::c_int &
                                     2 as libc::c_int) >> 1 as libc::c_int) *
                                   255 as libc::c_int) as u32_0,
                              ((sAudioDebugTextColor as libc::c_int &
                                    1 as libc::c_int) * 255 as libc::c_int) as
                                  u32_0, 255 as libc::c_int as u32_0);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 4 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"SEQ INFO  : %2d %02x %d\x00" as *const u8 as
                                *const libc::c_char,
                            sDisplayedStaff.noteIdx as libc::c_int,
                            sDisplayedStaff.state as libc::c_int,
                            sDisplayedStaff.pos as libc::c_int);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 5 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"PLAY INFO : %2d %02x %d\x00" as *const u8 as
                                *const libc::c_char,
                            sPlayingStaff.noteIdx as libc::c_int,
                            sPlayingStaff.state as libc::c_int,
                            sPlayingStaff.pos as libc::c_int);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 6 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"8note REC POINTER : %08x\x00" as *const u8 as
                                *const libc::c_char, gScarecrowSpawnSongPtr);
            ctr = 0 as libc::c_int as u8_0;
            j = 0 as libc::c_int as u8_0;
            while (j as libc::c_int) < 4 as libc::c_int {
                i = 0 as libc::c_int as u8_0;
                while (i as libc::c_int) < 8 as libc::c_int {
                    GfxPrint_SetPos(printer,
                                    3 as libc::c_int +
                                        3 as libc::c_int * i as libc::c_int,
                                    7 as libc::c_int + j as libc::c_int);
                    let fresh5 = ctr;
                    ctr = ctr.wrapping_add(1);
                    GfxPrint_Printf(printer,
                                    b"%02x\x00" as *const u8 as
                                        *const libc::c_char,
                                    *gScarecrowSpawnSongPtr.offset(fresh5 as
                                                                       isize)
                                        as libc::c_int);
                    i = i.wrapping_add(1)
                }
                j = j.wrapping_add(1)
            }
            GfxPrint_SetPos(printer, 3 as libc::c_int, 24 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"OCA:%02x SEQ:%04x PLAY:%02x REC:%02x\x00" as
                                *const u8 as *const libc::c_char,
                            D_80130F10 as libc::c_int, D_80130F3C,
                            sPlaybackState as libc::c_int,
                            sRecordingState as libc::c_int);
        }
        12 => {
            GfxPrint_SetPos(printer, 2 as libc::c_int,
                            4 as libc::c_int +
                                sAudioSfxParamChgSel as libc::c_int);
            GfxPrint_SetColor(printer,
                              (((sAudioDebugTextColor as libc::c_int &
                                     4 as libc::c_int) >> 2 as libc::c_int) *
                                   127 as libc::c_int) as u32_0,
                              (((sAudioDebugTextColor as libc::c_int &
                                     2 as libc::c_int) >> 1 as libc::c_int) *
                                   255 as libc::c_int) as u32_0,
                              ((sAudioDebugTextColor as libc::c_int &
                                    1 as libc::c_int) * 127 as libc::c_int) as
                                  u32_0, 255 as libc::c_int as u32_0);
            GfxPrint_Printf(printer,
                            b"*\x00" as *const u8 as *const libc::c_char);
            GfxPrint_SetColor(printer,
                              (((sAudioDebugTextColor as libc::c_int &
                                     4 as libc::c_int) >> 2 as libc::c_int) *
                                   255 as libc::c_int) as u32_0,
                              (((sAudioDebugTextColor as libc::c_int &
                                     2 as libc::c_int) >> 1 as libc::c_int) *
                                   255 as libc::c_int) as u32_0,
                              ((sAudioDebugTextColor as libc::c_int &
                                    1 as libc::c_int) * 255 as libc::c_int) as
                                  u32_0, 255 as libc::c_int as u32_0);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 4 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"SE HD  : %02x %s\x00" as *const u8 as
                                *const libc::c_char,
                            sAudioSfxParamChgWork[0 as libc::c_int as usize]
                                as libc::c_int,
                            sSoundBankNames[sAudioSfxParamChgWork[0 as
                                                                      libc::c_int
                                                                      as
                                                                      usize]
                                                as usize].as_mut_ptr());
            GfxPrint_SetPos(printer, 3 as libc::c_int, 5 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"SE No. : %02x\x00" as *const u8 as
                                *const libc::c_char,
                            sAudioSfxParamChgWork[1 as libc::c_int as usize]
                                as libc::c_int);
            GfxPrint_SetPos(printer, 20 as libc::c_int, 6 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"       : %04x\x00" as *const u8 as
                                *const libc::c_char,
                            (*gSoundParams[sAudioSfxParamChgWork[0 as
                                                                     libc::c_int
                                                                     as usize]
                                               as
                                               usize].offset(sAudioSfxParamChgWork[1
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       usize]
                                                                 as
                                                                 isize)).params
                                as libc::c_int);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 6 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"SE SW    %s\x00" as *const u8 as
                                *const libc::c_char,
                            AudioDebug_ToStringBinary((*gSoundParams[sAudioSfxParamChgWork[0
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               usize]
                                                                         as
                                                                         usize].offset(sAudioSfxParamChgWork[1
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 as
                                                                                                                 usize]
                                                                                           as
                                                                                           isize)).params
                                                          as u32_0,
                                                      16 as libc::c_int as
                                                          u8_0));
            GfxPrint_SetColor(printer,
                              (((sAudioDebugTextColor as libc::c_int &
                                     4 as libc::c_int) >> 2 as libc::c_int) *
                                   127 as libc::c_int) as u32_0,
                              (((sAudioDebugTextColor as libc::c_int &
                                     2 as libc::c_int) >> 1 as libc::c_int) *
                                   255 as libc::c_int) as u32_0,
                              ((sAudioDebugTextColor as libc::c_int &
                                    1 as libc::c_int) * 127 as libc::c_int) as
                                  u32_0, 255 as libc::c_int as u32_0);
            digitStr[0 as libc::c_int as usize] =
                ('0' as i32 +
                     ((*gSoundParams[sAudioSfxParamChgWork[0 as libc::c_int as
                                                               usize] as
                                         usize].offset(sAudioSfxParamChgWork[1
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 usize]
                                                           as isize)).params
                          as libc::c_int >>
                          15 as libc::c_int -
                              sAudioSfxParamChgBitSel as libc::c_int &
                          1 as libc::c_int)) as libc::c_char;
            GfxPrint_SetPos(printer,
                            12 as libc::c_int +
                                sAudioSfxParamChgBitSel as libc::c_int,
                            6 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"%s\x00" as *const u8 as *const libc::c_char,
                            digitStr.as_mut_ptr());
            GfxPrint_SetColor(printer,
                              (((sAudioDebugTextColor as libc::c_int &
                                     4 as libc::c_int) >> 2 as libc::c_int) *
                                   255 as libc::c_int) as u32_0,
                              (((sAudioDebugTextColor as libc::c_int &
                                     2 as libc::c_int) >> 1 as libc::c_int) *
                                   255 as libc::c_int) as u32_0,
                              ((sAudioDebugTextColor as libc::c_int &
                                    1 as libc::c_int) * 255 as libc::c_int) as
                                  u32_0, 255 as libc::c_int as u32_0);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 7 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"SE PR  : %02x\x00" as *const u8 as
                                *const libc::c_char,
                            (*gSoundParams[sAudioSfxParamChgWork[0 as
                                                                     libc::c_int
                                                                     as usize]
                                               as
                                               usize].offset(sAudioSfxParamChgWork[1
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       usize]
                                                                 as
                                                                 isize)).importance
                                as libc::c_int);
        }
        14 => {
            GfxPrint_SetPos(printer, 3 as libc::c_int, 4 as libc::c_int);
            GfxPrint_SetColor(printer,
                              (((sAudioDebugTextColor as libc::c_int &
                                     4 as libc::c_int) >> 2 as libc::c_int) *
                                   255 as libc::c_int) as u32_0,
                              (((sAudioDebugTextColor as libc::c_int &
                                     2 as libc::c_int) >> 1 as libc::c_int) *
                                   255 as libc::c_int) as u32_0,
                              ((sAudioDebugTextColor as libc::c_int &
                                    1 as libc::c_int) * 255 as libc::c_int) as
                                  u32_0, 255 as libc::c_int as u32_0);
            GfxPrint_Printf(printer,
                            b"env_fx %d code_fx %d SPEC %d\x00" as *const u8
                                as *const libc::c_char,
                            sAudioEnvReverb as libc::c_int,
                            sAudioCodeReverb as libc::c_int,
                            gAudioSpecId as libc::c_int);
            if sAudioUpdateTaskStart == sAudioUpdateTaskEnd {
                sAudioUpdateDuration =
                    (sAudioUpdateEndTime.wrapping_sub(sAudioUpdateStartTime)
                         as
                         u64_0).wrapping_mul((1000000000 as libc::c_longlong /
                                                  15625000 as
                                                      libc::c_longlong) as
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
                                                                                      15625000
                                                                                          as
                                                                                          libc::c_longlong)
                                                                                     as
                                                                                     libc::c_ulonglong)
                        as libc::c_float /
                        (1e9f32 / 20 as libc::c_int as libc::c_float);
                if sAudioUpdateDurationMax < sAudioUpdateDuration {
                    sAudioUpdateDurationMax = sAudioUpdateDuration
                }
            }
            GfxPrint_SetPos(printer, 3 as libc::c_int, 6 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"SOUND GAME FRAME NOW %f\x00" as *const u8 as
                                *const libc::c_char,
                            sAudioUpdateDuration as libc::c_double);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 7 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"SOUND GAME FRAME MAX %f\x00" as *const u8 as
                                *const libc::c_char,
                            sAudioUpdateDurationMax as libc::c_double);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 9 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"SWITCH BGM MODE %d %d %d (FLAG %d)\x00" as
                                *const u8 as *const libc::c_char,
                            sPrevSeqMode as libc::c_int, sNumFramesStill,
                            sNumFramesMoving, sSeqModeInput as libc::c_int);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 10 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"ENEMY DIST %f VOL %3d\x00" as *const u8 as
                                *const libc::c_char,
                            sAudioEnemyDist as libc::c_double,
                            sAudioEnemyVol as libc::c_int);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 11 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"GANON DIST VOL %3d\x00" as *const u8 as
                                *const libc::c_char,
                            sAudioGanonDistVol as libc::c_int);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 12 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"DEMO FLAG %d\x00" as *const u8 as
                                *const libc::c_char,
                            sAudioCutsceneFlag as libc::c_int);
            GfxPrint_SetPos(printer, 3 as libc::c_int, 12 as libc::c_int);
            if sAudioHasMalonBgm as libc::c_int == 1 as libc::c_int {
                GfxPrint_Printf(printer,
                                b"MARON BGM DIST %f\x00" as *const u8 as
                                    *const libc::c_char,
                                sAudioMalonBgmDist as libc::c_double);
                sAudioHasMalonBgm = 0 as libc::c_int as u8_0
            }
            GfxPrint_SetPos(printer, 3 as libc::c_int, 23 as libc::c_int);
            if sAudioNatureFailed as libc::c_int != 0 as libc::c_int {
                GfxPrint_Printf(printer,
                                b"NATURE FAILED %01x\x00" as *const u8 as
                                    *const libc::c_char,
                                sAudioNatureFailed as libc::c_int);
            }
            GfxPrint_SetPos(printer, 3 as libc::c_int, 24 as libc::c_int);
            if !sSariaBgmPtr.is_null() {
                GfxPrint_Printf(printer,
                                b"SARIA BGM PTR %08x\x00" as *const u8 as
                                    *const libc::c_char, sSariaBgmPtr);
            }
            GfxPrint_SetPos(printer, 3 as libc::c_int, 25 as libc::c_int);
            GfxPrint_Printf(printer,
                            b"POLI %d(%d)\x00" as *const u8 as
                                *const libc::c_char,
                            sPeakNumNotes as libc::c_int,
                            numEnabledNotes as libc::c_int);
            i = 0 as libc::c_int as u8_0;
            while (i as libc::c_int) < 11 as libc::c_int {
                GfxPrint_SetPos(printer,
                                3 as libc::c_int +
                                    3 as libc::c_int * i as libc::c_int,
                                26 as libc::c_int);
                GfxPrint_Printf(printer,
                                b"%d\x00" as *const u8 as *const libc::c_char,
                                sAudioSpecPeakNumNotes[i as usize] as
                                    libc::c_int);
                i = i.wrapping_add(1)
            }
        }
        _ => { }
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioDebug_ProcessInput_SndCont() {
    let mut step: u16_0 = 1 as libc::c_int as u16_0;
    if sDebugPadHold & 0x4 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        if sAudioSndContWorkLims[sAudioSndContSel as usize] as libc::c_int >=
               16 as libc::c_int {
            step = 16 as libc::c_int as u16_0
        }
    } else if sDebugPadHold & 0x2 as libc::c_int as libc::c_uint !=
                  0 as libc::c_int as libc::c_uint {
        if sAudioSndContWorkLims[sAudioSndContSel as usize] as libc::c_int >=
               16 as libc::c_int {
            step = 8 as libc::c_int as u16_0
        }
    } else if sDebugPadHold & 0x8 as libc::c_int as libc::c_uint !=
                  0 as libc::c_int as libc::c_uint {
        sAudioSndContWork[sAudioSndContSel as usize] =
            0 as libc::c_int as u16_0
    }
    if sDebugPadPress & 0x800 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        if sAudioSndContSel as libc::c_int > 0 as libc::c_int {
            sAudioSndContSel = sAudioSndContSel.wrapping_sub(1)
        } else { sAudioSndContSel = 10 as libc::c_int as u8_0 }
    }
    if sDebugPadPress & 0x400 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        if (sAudioSndContSel as libc::c_int) < 10 as libc::c_int {
            sAudioSndContSel = sAudioSndContSel.wrapping_add(1)
        } else { sAudioSndContSel = 0 as libc::c_int as u8_0 }
    }
    if sDebugPadPress & 0x200 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        if sAudioSndContWork[sAudioSndContSel as usize] as libc::c_int >=
               step as libc::c_int {
            sAudioSndContWork[sAudioSndContSel as usize] =
                (sAudioSndContWork[sAudioSndContSel as usize] as libc::c_int -
                     step as libc::c_int) as u16_0
        } else {
            sAudioSndContWork[sAudioSndContSel as usize] =
                (sAudioSndContWork[sAudioSndContSel as usize] as libc::c_int +
                     (sAudioSndContWorkLims[sAudioSndContSel as usize] as
                          libc::c_int - step as libc::c_int)) as u16_0
        }
    }
    if sDebugPadPress & 0x100 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        if (sAudioSndContWork[sAudioSndContSel as usize] as libc::c_int +
                step as libc::c_int) <
               sAudioSndContWorkLims[sAudioSndContSel as usize] as libc::c_int
           {
            sAudioSndContWork[sAudioSndContSel as usize] =
                (sAudioSndContWork[sAudioSndContSel as usize] as libc::c_int +
                     step as libc::c_int) as u16_0
        } else {
            sAudioSndContWork[sAudioSndContSel as usize] =
                (sAudioSndContWork[sAudioSndContSel as usize] as libc::c_int +
                     (step as libc::c_int -
                          sAudioSndContWorkLims[sAudioSndContSel as usize] as
                              libc::c_int)) as u16_0
        }
    }
    if sAudioSndContSel as libc::c_int == 8 as libc::c_int {
        if sAudioSndContWork[sAudioSndContSel as usize] as libc::c_int !=
               0 as libc::c_int {
            Audio_SetExtraFilter(0x20 as libc::c_int as u8_0);
        } else { Audio_SetExtraFilter(0 as libc::c_int as u8_0); }
    }
    if sAudioSndContSel as libc::c_int == 9 as libc::c_int {
        if sAudioSndContWork[sAudioSndContSel as usize] as libc::c_int !=
               0 as libc::c_int {
            Audio_SetBaseFilter(0x20 as libc::c_int as u8_0);
        } else { Audio_SetBaseFilter(0 as libc::c_int as u8_0); }
    }
    if sDebugPadPress & 0x8000 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        match sAudioSndContSel as libc::c_int {
            0 | 1 => {
                Audio_QueueSeqCmd((0 as libc::c_int |
                                       (sAudioSndContSel as libc::c_int) <<
                                           24 as libc::c_int |
                                       (0 as libc::c_int as u8_0 as
                                            libc::c_int) <<
                                           0x10 as libc::c_int |
                                       sAudioSndContWork[sAudioSndContSel as
                                                             usize] as
                                           libc::c_int) as u32_0);
            }
            2 | 3 => {
                Audio_PlaySoundGeneral((((sAudioSndContWork[2 as libc::c_int
                                                                as usize] as
                                              libc::c_int) <<
                                             12 as libc::c_int &
                                             0xffff as libc::c_int) +
                                            sAudioSndContWork[3 as libc::c_int
                                                                  as usize] as
                                                libc::c_int +
                                            0x800 as libc::c_int) as u16_0,
                                       &mut D_801333D4,
                                       4 as libc::c_int as u8_0,
                                       &mut D_801333E0, &mut D_801333E0,
                                       &mut D_801333E8);
            }
            4 => {
                func_800F6700(sAudioSndContWork[sAudioSndContSel as usize] as
                                  s8);
            }
            5 => {
                Audio_QueueSeqCmd(0xe0000100 as libc::c_uint |
                                      ((SEQ_PLAYER_BGM_MAIN as libc::c_int as
                                            u8_0 as libc::c_int) <<
                                           24 as libc::c_int) as libc::c_uint
                                      |
                                      sAudioSndContWork[sAudioSndContSel as
                                                            usize] as
                                          libc::c_uint);
            }
            6 => {
                Audio_QueueSeqCmd(0xf0000000 as libc::c_uint |
                                      ((SEQ_PLAYER_BGM_MAIN as libc::c_int as
                                            u8_0 as libc::c_int) <<
                                           24 as libc::c_int) as libc::c_uint
                                      |
                                      sAudioSndContWork[sAudioSndContSel as
                                                            usize] as u8_0 as
                                          libc::c_uint);
                sAudioSubTrackInfoSpec =
                    sAudioSndContWork[6 as libc::c_int as usize] as u8_0;
                if sAudioSubTrackInfoPlayerSel as libc::c_int >
                       gAudioSpecs[sAudioSubTrackInfoSpec as
                                       usize].numSequencePlayers as
                           libc::c_int - 1 as libc::c_int {
                    sAudioSubTrackInfoPlayerSel =
                        (gAudioSpecs[sAudioSubTrackInfoSpec as
                                         usize].numSequencePlayers as
                             libc::c_int - 1 as libc::c_int) as u8_0
                }
            }
            7 => {
                func_800F6FB4(sAudioSndContWork[sAudioSndContSel as usize] as
                                  u8_0);
            }
            10 => {
                Audio_SetSoundBanksMute((sAudioSndContWork[sAudioSndContSel as
                                                               usize] as
                                             libc::c_int *
                                             0x7f as libc::c_int) as u16_0);
            }
            8 | 9 | _ => { }
        }
    }
    if sDebugPadPress & 0x4000 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        match sAudioSndContSel as libc::c_int {
            0 | 1 => {
                Audio_QueueSeqCmd((0x100000ff as libc::c_int |
                                       (sAudioSndContSel as libc::c_int) <<
                                           24 as libc::c_int |
                                       (0 as libc::c_int as u8_0 as
                                            libc::c_int) << 16 as libc::c_int)
                                      as u32_0);
            }
            7 => {
                Audio_QueueSeqCmd((0x100000ff as libc::c_int |
                                       (SEQ_PLAYER_BGM_MAIN as libc::c_int as
                                            u8_0 as libc::c_int) <<
                                           24 as libc::c_int |
                                       (0 as libc::c_int as u8_0 as
                                            libc::c_int) << 16 as libc::c_int)
                                      as u32_0);
            }
            2 | 3 => {
                Audio_StopSfxByBank(sAudioSndContWork[2 as libc::c_int as
                                                          usize] as u8_0);
            }
            _ => { }
        }
    }
    if sDebugPadPress & 0x4 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        if sAudioSndContSel as libc::c_int == 0 as libc::c_int {
            func_800F595C(sAudioSndContWork[sAudioSndContSel as usize]);
        }
    }
    if sDebugPadPress & 0x1 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        if sAudioSndContSel as libc::c_int == 0 as libc::c_int {
            func_800F5ACC(sAudioSndContWork[sAudioSndContSel as usize]);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioDebug_ProcessInput_IntInfo() {
    if sDebugPadPress & 0x8 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        sAudioIntInfoY -= 1
    }
    if sDebugPadPress & 0x4 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        sAudioIntInfoY += 1
    }
    if sDebugPadPress & 0x2 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        sAudioIntInfoX -= 1
    }
    if sDebugPadPress & 0x1 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        sAudioIntInfoX += 1
    }
    if sDebugPadPress & 0x4000 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        sAudioIntInfoX = 0 as libc::c_int as s8;
        sAudioIntInfoY = 0 as libc::c_int as s8
    }
    if sDebugPadPress & 0x800 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint &&
           sAudioIntInfoSel as libc::c_int > 0 as libc::c_int {
        sAudioIntInfoSel -= 1
    }
    if sDebugPadPress & 0x400 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint &&
           (sAudioIntInfoSel as libc::c_int) < 6 as libc::c_int {
        sAudioIntInfoSel += 1
    }
    if sDebugPadPress & 0x200 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint &&
           sAudioIntInfoBankPage[sAudioIntInfoSel as usize] as libc::c_int >
               0 as libc::c_int {
        sAudioIntInfoBankPage[sAudioIntInfoSel as usize] -= 1
    }
    if sDebugPadPress & 0x100 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint &&
           (sAudioIntInfoBankPage[sAudioIntInfoSel as usize] as libc::c_int) <
               4 as libc::c_int {
        sAudioIntInfoBankPage[sAudioIntInfoSel as usize] += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioDebug_ProcessInput_ScrPrt() {
    if sAudioScrPrtWork[0 as libc::c_int as usize] as libc::c_int !=
           0 as libc::c_int {
        if sDebugPadPress & 0x8 as libc::c_int as libc::c_uint !=
               0 as libc::c_int as libc::c_uint {
            sAudioScrPrtY -= 1
        }
        if sDebugPadPress & 0x4 as libc::c_int as libc::c_uint !=
               0 as libc::c_int as libc::c_uint {
            sAudioScrPrtY += 1
        }
        if sDebugPadPress & 0x2 as libc::c_int as libc::c_uint !=
               0 as libc::c_int as libc::c_uint {
            sAudioScrPrtX -= 1
        }
        if sDebugPadPress & 0x1 as libc::c_int as libc::c_uint !=
               0 as libc::c_int as libc::c_uint {
            sAudioScrPrtX += 1
        }
        if sDebugPadPress & 0x8000 as libc::c_int as libc::c_uint !=
               0 as libc::c_int as libc::c_uint {
            sAudioScrPrtX = 26 as libc::c_int as s8;
            sAudioScrPrtY = 1 as libc::c_int as s8;
            sAudioScrPrtWork[2 as libc::c_int as usize] =
                6 as libc::c_int as u8_0
        }
        if sDebugPadPress & 0x4000 as libc::c_int as libc::c_uint !=
               0 as libc::c_int as libc::c_uint {
            sAudioScrPrtInd = 0 as libc::c_int as u8_0;
            sAudioScrPrtOverflow = 0 as libc::c_int as u8_0
        }
    }
    if sDebugPadPress & 0x800 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        if sAudioScrPrtSel as libc::c_int > 0 as libc::c_int {
            sAudioScrPrtSel = sAudioScrPrtSel.wrapping_sub(1)
        } else { sAudioScrPrtSel = 10 as libc::c_int as u8_0 }
    }
    if sDebugPadPress & 0x400 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        if (sAudioScrPrtSel as libc::c_int) < 10 as libc::c_int {
            sAudioScrPrtSel = sAudioScrPrtSel.wrapping_add(1)
        } else { sAudioScrPrtSel = 0 as libc::c_int as u8_0 }
    }
    if sDebugPadPress & 0x200 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        if sAudioScrPrtWork[sAudioScrPrtSel as usize] as libc::c_int >
               0 as libc::c_int {
            sAudioScrPrtWork[sAudioScrPrtSel as usize] =
                sAudioScrPrtWork[sAudioScrPrtSel as usize].wrapping_sub(1)
        } else {
            sAudioScrPrtWork[sAudioScrPrtSel as usize] =
                (sAudioScrPrtWorkLims[sAudioScrPrtSel as usize] as libc::c_int
                     - 1 as libc::c_int) as u8_0
        }
    }
    if sDebugPadPress & 0x100 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        if (sAudioScrPrtWork[sAudioScrPrtSel as usize] as libc::c_int) <
               sAudioScrPrtWorkLims[sAudioScrPrtSel as usize] as libc::c_int -
                   1 as libc::c_int {
            sAudioScrPrtWork[sAudioScrPrtSel as usize] =
                sAudioScrPrtWork[sAudioScrPrtSel as usize].wrapping_add(1)
        } else {
            sAudioScrPrtWork[sAudioScrPrtSel as usize] =
                0 as libc::c_int as u8_0
        }
    }
    D_801333F0 =
        (sAudioScrPrtWork[3 as libc::c_int as usize] as libc::c_int +
             sAudioScrPrtWork[4 as libc::c_int as usize] as libc::c_int *
                 2 as libc::c_int +
             sAudioScrPrtWork[5 as libc::c_int as usize] as libc::c_int *
                 4 as libc::c_int +
             sAudioScrPrtWork[6 as libc::c_int as usize] as libc::c_int *
                 8 as libc::c_int +
             sAudioScrPrtWork[7 as libc::c_int as usize] as libc::c_int *
                 0x10 as libc::c_int +
             sAudioScrPrtWork[8 as libc::c_int as usize] as libc::c_int *
                 0x20 as libc::c_int) as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn AudioDebug_ProcessInput_SfxSwap() {
    let mut step: s16 = 0;
    let mut val: u16_0 = 0;
    let mut prev: u8_0 = 0;
    if sAudioSfxSwapIsEditing == 0 {
        if sDebugPadPress & 0x800 as libc::c_int as libc::c_uint !=
               0 as libc::c_int as libc::c_uint {
            if sAudioSfxSwapSel as libc::c_int > 0 as libc::c_int {
                sAudioSfxSwapSel = sAudioSfxSwapSel.wrapping_sub(1)
            } else { sAudioSfxSwapSel = 9 as libc::c_int as u8_0 }
        }
        if sDebugPadPress & 0x400 as libc::c_int as libc::c_uint !=
               0 as libc::c_int as libc::c_uint {
            if (sAudioSfxSwapSel as libc::c_int) < 9 as libc::c_int {
                sAudioSfxSwapSel = sAudioSfxSwapSel.wrapping_add(1)
            } else { sAudioSfxSwapSel = 0 as libc::c_int as u8_0 }
        }
        if sDebugPadPress & 0x8000 as libc::c_int as libc::c_uint !=
               0 as libc::c_int as libc::c_uint {
            sAudioSfxSwapIsEditing = 1 as libc::c_int as u8_0
        }
        if sDebugPadPress & 0x4000 as libc::c_int as libc::c_uint !=
               0 as libc::c_int as libc::c_uint {
            gAudioSfxSwapSource[sAudioSfxSwapSel as usize] =
                0 as libc::c_int as u16_0;
            gAudioSfxSwapTarget[sAudioSfxSwapSel as usize] =
                0 as libc::c_int as u16_0
        }
        if sDebugPadPress & 0x1000 as libc::c_int as libc::c_uint !=
               0 as libc::c_int as libc::c_uint {
            if sAudioSfxSwapSel as libc::c_int != 0 as libc::c_int {
                prev =
                    (sAudioSfxSwapSel as libc::c_int - 1 as libc::c_int) as
                        u8_0
            } else { prev = 9 as libc::c_int as u8_0 }
            gAudioSfxSwapSource[sAudioSfxSwapSel as usize] =
                gAudioSfxSwapSource[prev as usize];
            gAudioSfxSwapTarget[sAudioSfxSwapSel as usize] =
                gAudioSfxSwapTarget[prev as usize]
        }
    } else {
        if sDebugPadPress & 0x200 as libc::c_int as libc::c_uint !=
               0 as libc::c_int as libc::c_uint {
            if sAudioSfxSwapNibbleSel as libc::c_int > 0 as libc::c_int {
                sAudioSfxSwapNibbleSel =
                    sAudioSfxSwapNibbleSel.wrapping_sub(1)
            } else { sAudioSfxSwapNibbleSel = 7 as libc::c_int as u8_0 }
        }
        if sDebugPadPress & 0x100 as libc::c_int as libc::c_uint !=
               0 as libc::c_int as libc::c_uint {
            if (sAudioSfxSwapNibbleSel as libc::c_int) < 7 as libc::c_int {
                sAudioSfxSwapNibbleSel =
                    sAudioSfxSwapNibbleSel.wrapping_add(1)
            } else { sAudioSfxSwapNibbleSel = 0 as libc::c_int as u8_0 }
        }
        if sDebugPadPress & 0x800 as libc::c_int as libc::c_uint !=
               0 as libc::c_int as libc::c_uint ||
               sDebugPadPress & 0x400 as libc::c_int as libc::c_uint !=
                   0 as libc::c_int as libc::c_uint {
            if sDebugPadPress & 0x800 as libc::c_int as libc::c_uint !=
                   0 as libc::c_int as libc::c_uint {
                step =
                    if sDebugPadHold & 0x8 as libc::c_int as libc::c_uint !=
                           0 as libc::c_int as libc::c_uint {
                        8 as libc::c_int
                    } else { 1 as libc::c_int } as s16
            }
            if sDebugPadPress & 0x400 as libc::c_int as libc::c_uint !=
                   0 as libc::c_int as libc::c_uint {
                step =
                    if sDebugPadHold & 0x8 as libc::c_int as libc::c_uint !=
                           0 as libc::c_int as libc::c_uint {
                        -(8 as libc::c_int)
                    } else { -(1 as libc::c_int) } as s16
            }
            if (sAudioSfxSwapNibbleSel as libc::c_int) < 4 as libc::c_int {
                val =
                    (gAudioSfxSwapSource[sAudioSfxSwapSel as usize] as
                         libc::c_int >>
                         (3 as libc::c_int -
                              sAudioSfxSwapNibbleSel as libc::c_int) *
                             4 as libc::c_int) as u16_0;
                val =
                    (val as libc::c_int + step as libc::c_int &
                         0xf as libc::c_int) as u16_0;
                gAudioSfxSwapSource[sAudioSfxSwapSel as usize] =
                    ((gAudioSfxSwapSource[sAudioSfxSwapSel as usize] as
                          libc::c_int &
                          ((0xf as libc::c_int) <<
                               (3 as libc::c_int -
                                    sAudioSfxSwapNibbleSel as libc::c_int) *
                                   4 as libc::c_int ^ 0xffff as libc::c_int))
                         +
                         ((val as libc::c_int) <<
                              (3 as libc::c_int -
                                   sAudioSfxSwapNibbleSel as libc::c_int) *
                                  4 as libc::c_int)) as u16_0
            } else {
                val =
                    (gAudioSfxSwapTarget[sAudioSfxSwapSel as usize] as
                         libc::c_int >>
                         (7 as libc::c_int -
                              sAudioSfxSwapNibbleSel as libc::c_int) *
                             4 as libc::c_int) as u16_0;
                val =
                    (val as libc::c_int + step as libc::c_int &
                         0xf as libc::c_int) as u16_0;
                gAudioSfxSwapTarget[sAudioSfxSwapSel as usize] =
                    ((gAudioSfxSwapTarget[sAudioSfxSwapSel as usize] as
                          libc::c_int &
                          ((0xf as libc::c_int) <<
                               (7 as libc::c_int -
                                    sAudioSfxSwapNibbleSel as libc::c_int) *
                                   4 as libc::c_int ^ 0xffff as libc::c_int))
                         +
                         ((val as libc::c_int) <<
                              (7 as libc::c_int -
                                   sAudioSfxSwapNibbleSel as libc::c_int) *
                                  4 as libc::c_int)) as u16_0
            }
        }
        if sDebugPadPress & 0x8000 as libc::c_int as libc::c_uint !=
               0 as libc::c_int as libc::c_uint {
            sAudioSfxSwapIsEditing = 0 as libc::c_int as u8_0
        }
        if sDebugPadPress & 0x4000 as libc::c_int as libc::c_uint !=
               0 as libc::c_int as libc::c_uint {
            if (sAudioSfxSwapNibbleSel as libc::c_int) < 4 as libc::c_int {
                gAudioSfxSwapSource[sAudioSfxSwapSel as usize] =
                    0 as libc::c_int as u16_0
            } else {
                gAudioSfxSwapTarget[sAudioSfxSwapSel as usize] =
                    0 as libc::c_int as u16_0
            }
        }
    }
    if sDebugPadPress & 0x2 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        gAudioSfxSwapOff =
            (gAudioSfxSwapOff as libc::c_int ^ 1 as libc::c_int) as u8_0
    }
    if sDebugPadPress & 0x4 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        gAudioSfxSwapMode[sAudioSfxSwapSel as usize] =
            (gAudioSfxSwapMode[sAudioSfxSwapSel as usize] as libc::c_int ^
                 1 as libc::c_int) as u8_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioDebug_ProcessInput_SubTrackInfo() {
    if sDebugPadPress & 0x400 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        if sAudioSubTrackInfoPlayerSel as libc::c_int != 0 as libc::c_int {
            sAudioSubTrackInfoPlayerSel =
                sAudioSubTrackInfoPlayerSel.wrapping_sub(1)
        } else {
            sAudioSubTrackInfoPlayerSel =
                (gAudioSpecs[sAudioSubTrackInfoSpec as
                                 usize].numSequencePlayers as libc::c_int -
                     1 as libc::c_int) as u8_0
        }
    }
    if sDebugPadPress & 0x800 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        if (sAudioSubTrackInfoPlayerSel as libc::c_int) <
               gAudioSpecs[sAudioSubTrackInfoSpec as usize].numSequencePlayers
                   as libc::c_int - 1 as libc::c_int {
            sAudioSubTrackInfoPlayerSel =
                sAudioSubTrackInfoPlayerSel.wrapping_add(1)
        } else {
            sAudioSubTrackInfoPlayerSel =
                SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0
        }
    }
    if sDebugPadPress & 0x200 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        sAudioSubTrackInfoChannelSel =
            (sAudioSubTrackInfoChannelSel as libc::c_int - 1 as libc::c_int &
                 0xf as libc::c_int) as u8_0
    }
    if sDebugPadPress & 0x100 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        sAudioSubTrackInfoChannelSel =
            (sAudioSubTrackInfoChannelSel as libc::c_int + 1 as libc::c_int &
                 0xf as libc::c_int) as u8_0
    }
    if sDebugPadPress & 0x1000 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        sSeqPlayerPeakNumLayers[sAudioSubTrackInfoPlayerSel as usize] =
            SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioDebug_ProcessInput_HeapInfo() { }
#[no_mangle]
pub unsafe extern "C" fn AudioDebug_ProcessInput_BlkChgBgm() {
    if sDebugPadPress & 0x800 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        if sAudioBlkChgBgmSel as libc::c_int > 0 as libc::c_int {
            sAudioBlkChgBgmSel = sAudioBlkChgBgmSel.wrapping_sub(1)
        } else { sAudioBlkChgBgmSel = 1 as libc::c_int as u8_0 }
    }
    if sDebugPadPress & 0x400 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        if sAudioBlkChgBgmSel as libc::c_int <= 0 as libc::c_int {
            sAudioBlkChgBgmSel = sAudioBlkChgBgmSel.wrapping_add(1)
        } else { sAudioBlkChgBgmSel = 0 as libc::c_int as u8_0 }
    }
    if sDebugPadPress & 0x200 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        sAudioBlkChgBgmWork[sAudioBlkChgBgmSel as usize] =
            sAudioBlkChgBgmWork[sAudioBlkChgBgmSel as usize].wrapping_sub(1);
        if sAudioBlkChgBgmSel as libc::c_int == 1 as libc::c_int {
            Audio_SetSequenceMode(sAudioBlkChgBgmWork[1 as libc::c_int as
                                                          usize]);
            // might be a fake match?
        }
    }
    if sDebugPadPress & 0x100 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        sAudioBlkChgBgmWork[sAudioBlkChgBgmSel as usize] =
            sAudioBlkChgBgmWork[sAudioBlkChgBgmSel as usize].wrapping_add(1);
        if sAudioBlkChgBgmSel as libc::c_int == 1 as libc::c_int {
            Audio_SetSequenceMode(sAudioBlkChgBgmWork[1 as libc::c_int as
                                                          usize]);
        }
    }
    if sDebugPadPress & 0x8000 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        Audio_QueueCmdS8(((0x46 as libc::c_int & 0xff as libc::c_int) <<
                              0x18 as libc::c_int |
                              (SEQ_PLAYER_BGM_MAIN as libc::c_int &
                                   0xff as libc::c_int) << 0x10 as libc::c_int
                              |
                              (0 as libc::c_int & 0xff as libc::c_int) <<
                                  0x8 as libc::c_int |
                              (0 as libc::c_int & 0xff as libc::c_int) <<
                                  0 as libc::c_int) as u32_0,
                         sAudioBlkChgBgmWork[1 as libc::c_int as usize] as
                             s8);
        Audio_QueueSeqCmd((sAudioBlkChgBgmWork[0 as libc::c_int as usize] as
                               libc::c_int | 0x10000 as libc::c_int) as
                              u32_0);
    }
    if sDebugPadPress & 0x4000 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        Audio_QueueSeqCmd(0x100100ff as libc::c_int as u32_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioDebug_ProcessInput_OcaTest() { }
#[no_mangle]
pub unsafe extern "C" fn AudioDebug_ProcessInput_SfxParamChg() {
    let mut step: s32 = 0;
    let mut sfx: u16_0 = 0;
    if sDebugPadHold & 0x2 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        step = 8 as libc::c_int
    } else { step = 1 as libc::c_int }
    if sDebugPadPress & 0x800 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        if sAudioSfxParamChgSel as libc::c_int > 0 as libc::c_int {
            sAudioSfxParamChgSel = sAudioSfxParamChgSel.wrapping_sub(1)
        } else { sAudioSfxParamChgSel = 3 as libc::c_int as u8_0 }
    }
    if sDebugPadPress & 0x400 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        if (sAudioSfxParamChgSel as libc::c_int) < 3 as libc::c_int {
            sAudioSfxParamChgSel = sAudioSfxParamChgSel.wrapping_add(1)
        } else { sAudioSfxParamChgSel = 0 as libc::c_int as u8_0 }
    }
    if sDebugPadPress & 0x200 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        if (sAudioSfxParamChgSel as libc::c_int) < 2 as libc::c_int {
            if sAudioSfxParamChgSel as libc::c_int == 0 as libc::c_int {
                if sAudioSfxParamChgWork[sAudioSfxParamChgSel as usize] as
                       libc::c_int > 0 as libc::c_int {
                    sAudioSfxParamChgWork[sAudioSfxParamChgSel as usize] =
                        sAudioSfxParamChgWork[sAudioSfxParamChgSel as
                                                  usize].wrapping_sub(1)
                } else {
                    sAudioSfxParamChgWork[sAudioSfxParamChgSel as usize] =
                        (sAudioSndContWorkLims[2 as libc::c_int as usize] as
                             libc::c_int - 1 as libc::c_int) as u16_0
                }
            } else {
                sAudioSfxParamChgWork[sAudioSfxParamChgSel as usize] =
                    (sAudioSfxParamChgWork[sAudioSfxParamChgSel as usize] as
                         libc::c_int - step) as u16_0;
                sAudioSfxParamChgWork[sAudioSfxParamChgSel as usize] =
                    (sAudioSfxParamChgWork[sAudioSfxParamChgSel as usize] as
                         libc::c_int & 0x1ff as libc::c_int) as u16_0
            }
        } else if sAudioSfxParamChgSel as libc::c_int == 3 as libc::c_int {
            let ref mut fresh6 =
                (*gSoundParams[sAudioSfxParamChgWork[0 as libc::c_int as
                                                         usize] as
                                   usize].offset(sAudioSfxParamChgWork[1 as
                                                                           libc::c_int
                                                                           as
                                                                           usize]
                                                     as isize)).importance;
            *fresh6 = (*fresh6 as libc::c_int - step) as u8_0
        } else {
            sAudioSfxParamChgBitSel =
                (sAudioSfxParamChgBitSel as libc::c_int - 1 as libc::c_int &
                     0xf as libc::c_int) as u8_0
        }
    }
    if sDebugPadPress & 0x100 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        if (sAudioSfxParamChgSel as libc::c_int) < 2 as libc::c_int {
            if sAudioSfxParamChgSel as libc::c_int == 0 as libc::c_int {
                if (sAudioSfxParamChgWork[sAudioSfxParamChgSel as usize] as
                        libc::c_int) <
                       sAudioSndContWorkLims[2 as libc::c_int as usize] as
                           libc::c_int - 1 as libc::c_int {
                    sAudioSfxParamChgWork[sAudioSfxParamChgSel as usize] =
                        sAudioSfxParamChgWork[sAudioSfxParamChgSel as
                                                  usize].wrapping_add(1)
                } else {
                    sAudioSfxParamChgWork[sAudioSfxParamChgSel as usize] =
                        0 as libc::c_int as u16_0
                }
            } else {
                sAudioSfxParamChgWork[sAudioSfxParamChgSel as usize] =
                    (sAudioSfxParamChgWork[sAudioSfxParamChgSel as usize] as
                         libc::c_int + step) as u16_0;
                sAudioSfxParamChgWork[sAudioSfxParamChgSel as usize] =
                    (sAudioSfxParamChgWork[sAudioSfxParamChgSel as usize] as
                         libc::c_int & 0x1ff as libc::c_int) as u16_0
            }
        } else if sAudioSfxParamChgSel as libc::c_int == 3 as libc::c_int {
            let ref mut fresh7 =
                (*gSoundParams[sAudioSfxParamChgWork[0 as libc::c_int as
                                                         usize] as
                                   usize].offset(sAudioSfxParamChgWork[1 as
                                                                           libc::c_int
                                                                           as
                                                                           usize]
                                                     as isize)).importance;
            *fresh7 = (*fresh7 as libc::c_int + step) as u8_0
        } else {
            sAudioSfxParamChgBitSel =
                (sAudioSfxParamChgBitSel as libc::c_int + 1 as libc::c_int &
                     0xf as libc::c_int) as u8_0
        }
    }
    if sDebugPadPress & 0x8000 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        sfx =
            (((sAudioSfxParamChgWork[0 as libc::c_int as usize] as
                   libc::c_int) << 12 as libc::c_int) as u16_0 as libc::c_int
                 +
                 sAudioSfxParamChgWork[1 as libc::c_int as usize] as
                     libc::c_int + 0x800 as libc::c_int) as u16_0;
        Audio_PlaySoundGeneral(sfx, &mut D_801333D4, 4 as libc::c_int as u8_0,
                               &mut D_801333E0, &mut D_801333E0,
                               &mut D_801333E8);
    }
    if sDebugPadPress & 0x4000 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        Audio_StopSfxByBank(sAudioSfxParamChgWork[0 as libc::c_int as usize]
                                as u8_0);
    }
    if sDebugPadPress & 0x4 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        if sAudioSfxParamChgSel as libc::c_int == 2 as libc::c_int {
            let ref mut fresh8 =
                (*gSoundParams[sAudioSfxParamChgWork[0 as libc::c_int as
                                                         usize] as
                                   usize].offset(sAudioSfxParamChgWork[1 as
                                                                           libc::c_int
                                                                           as
                                                                           usize]
                                                     as isize)).params;
            *fresh8 =
                (*fresh8 as libc::c_int ^
                     (1 as libc::c_int) <<
                         0xf as libc::c_int -
                             sAudioSfxParamChgBitSel as libc::c_int) as u16_0
        }
    }
    if sDebugPadPress & 0x8 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        if (sAudioSfxParamChgSel as libc::c_int) < 2 as libc::c_int {
            sAudioSfxParamChgWork[sAudioSfxParamChgSel as usize] =
                0 as libc::c_int as u16_0
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioDebug_ScrPrt(mut str: *const s8,
                                           mut num: u16_0) {
    let mut i: u8_0 = 0 as libc::c_int as u8_0;
    sAudioScrPrtBuf[sAudioScrPrtInd as usize].num = num;
    while *str.offset(i as isize) as libc::c_int != 0 as libc::c_int {
        sAudioScrPrtBuf[sAudioScrPrtInd as usize].str_0[i as usize] =
            *str.offset(i as isize);
        i = i.wrapping_add(1)
    }
    while (i as libc::c_int) < 5 as libc::c_int {
        sAudioScrPrtBuf[sAudioScrPrtInd as usize].str_0[i as usize] =
            0 as libc::c_int as s8;
        i = i.wrapping_add(1)
    }
    if (sAudioScrPrtInd as libc::c_int) < 25 as libc::c_int - 1 as libc::c_int
       {
        sAudioScrPrtInd = sAudioScrPrtInd.wrapping_add(1)
    } else {
        sAudioScrPrtInd = 0 as libc::c_int as u8_0;
        sAudioScrPrtOverflow = 1 as libc::c_int as u8_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioDebug_ProcessInput() {
    if sAudioDebugEverOpened == 0 { return }
    if sAudioSfxMuted != 0 {
        Audio_SetSoundBanksMute(0x6f as libc::c_int as u16_0);
    }
    if sDebugPadPress & 0x20 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        if (sAudioDebugPage as libc::c_int) <
               PAGE_MAX as libc::c_int - 1 as libc::c_int {
            sAudioDebugPage = sAudioDebugPage.wrapping_add(1)
        } else { sAudioDebugPage = 0 as libc::c_int as u8_0 }
    }
    if sDebugPadPress & 0x10 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        if sAudioDebugPage as libc::c_int > 0 as libc::c_int {
            sAudioDebugPage = sAudioDebugPage.wrapping_sub(1)
        } else {
            sAudioDebugPage =
                (PAGE_MAX as libc::c_int - 1 as libc::c_int) as u8_0
        }
    }
    if sDebugPadPress & 0x2000 as libc::c_int as libc::c_uint !=
           0 as libc::c_int as libc::c_uint {
        sAudioDebugTextColor = sAudioDebugTextColor.wrapping_add(1);
        sAudioDebugTextColor =
            (sAudioDebugTextColor as libc::c_int & 7 as libc::c_int) as u8_0
    }
    match sAudioDebugPage as libc::c_int {
        0 => {
            if sDebugPadPress & 0x8000 as libc::c_int as libc::c_uint !=
                   0 as libc::c_int as libc::c_uint {
                sAudioSndContWork[5 as libc::c_int as usize] =
                    (sAudioSndContWork[5 as libc::c_int as usize] as
                         libc::c_int ^ 1 as libc::c_int) as u16_0;
                Audio_QueueSeqCmd(0xe0000100 as libc::c_uint |
                                      ((SEQ_PLAYER_BGM_MAIN as libc::c_int as
                                            u8_0 as libc::c_int) <<
                                           24 as libc::c_int) as libc::c_uint
                                      |
                                      sAudioSndContWork[5 as libc::c_int as
                                                            usize] as
                                          libc::c_uint);
                if func_800FA0B4(SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0)
                       as libc::c_int != 0x1 as libc::c_int {
                    Audio_QueueSeqCmd((0x100000ff as libc::c_int |
                                           (SEQ_PLAYER_BGM_MAIN as libc::c_int
                                                as u8_0 as libc::c_int) <<
                                               24 as libc::c_int |
                                           (0 as libc::c_int as u8_0 as
                                                libc::c_int) <<
                                               16 as libc::c_int) as u32_0);
                }
                Audio_QueueSeqCmd((0x100000ff as libc::c_int |
                                       (SEQ_PLAYER_FANFARE as libc::c_int as
                                            u8_0 as libc::c_int) <<
                                           24 as libc::c_int |
                                       (0 as libc::c_int as u8_0 as
                                            libc::c_int) << 16 as libc::c_int)
                                      as u32_0);
                Audio_QueueSeqCmd((0x100000ff as libc::c_int |
                                       (SEQ_PLAYER_BGM_SUB as libc::c_int as
                                            u8_0 as libc::c_int) <<
                                           24 as libc::c_int |
                                       (0 as libc::c_int as u8_0 as
                                            libc::c_int) << 16 as libc::c_int)
                                      as u32_0);
            }
            if sDebugPadPress & 0x4000 as libc::c_int as libc::c_uint !=
                   0 as libc::c_int as libc::c_uint {
                sAudioSfxMuted =
                    (sAudioSfxMuted as libc::c_int ^ 1 as libc::c_int) as
                        u8_0;
                if sAudioSfxMuted == 0 {
                    Audio_SetSoundBanksMute(0 as libc::c_int as u16_0);
                }
            }
        }
        1 => { AudioDebug_ProcessInput_SndCont(); }
        7 => { AudioDebug_ProcessInput_IntInfo(); }
        13 => { AudioDebug_ProcessInput_ScrPrt(); }
        8 => { AudioDebug_ProcessInput_SfxSwap(); }
        5 => { AudioDebug_ProcessInput_SubTrackInfo(); }
        3 => { AudioDebug_ProcessInput_HeapInfo(); }
        9 => { AudioDebug_ProcessInput_BlkChgBgm(); }
        11 => { AudioDebug_ProcessInput_OcaTest(); }
        12 => { AudioDebug_ProcessInput_SfxParamChg(); }
        14 | _ => { }
    }
    D_8013340C = sAudioScrPrtWork[10 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn func_800F3054() {
    if func_800FAD34() as libc::c_int == 0 as libc::c_int {
        sAudioUpdateTaskStart = gAudioContext.totalTaskCnt;
        sAudioUpdateStartTime = osGetTime() as u32_0;
        func_800EE6F4();
        Audio_StepFreqLerp(&mut sRiverFreqScaleLerp);
        Audio_StepFreqLerp(&mut sWaterfallFreqScaleLerp);
        func_800F4A70();
        func_800F56A8();
        func_800F5CF8();
        if gAudioSpecId as libc::c_int == 7 as libc::c_int {
            Audio_ClearSariaBgm();
        }
        Audio_ProcessSoundRequests();
        Audio_ProcessSeqCmds();
        func_800F8F88();
        func_800FA3DC();
        AudioDebug_SetInput();
        AudioDebug_ProcessInput();
        Audio_ScheduleProcessCmds();
        sAudioUpdateTaskEnd = gAudioContext.totalTaskCnt;
        sAudioUpdateEndTime = osGetTime() as u32_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800F3138(mut arg0: s32) { }
#[no_mangle]
pub unsafe extern "C" fn func_800F3140(mut arg0: s32, mut arg1: s32) { }
#[no_mangle]
pub unsafe extern "C" fn func_800F314C(mut arg0: s8) {
    Audio_QueueCmdS32(((0x82 as libc::c_int) << 24 as libc::c_int |
                           (SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                               16 as libc::c_int |
                           (arg0 as u8_0 as libc::c_int & 0xff as libc::c_int)
                               << 8 as libc::c_int) as u32_0,
                      1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Audio_ComputeSoundVolume(mut bankId: u8_0,
                                                  mut entryIdx: u8_0)
 -> f32_0 {
    let mut bankEntry: *mut SoundBankEntry =
        &mut *(*gSoundBanks.as_mut_ptr().offset(bankId as
                                                    isize)).offset(entryIdx as
                                                                       isize)
            as *mut SoundBankEntry;
    let mut minDist: f32_0 = 0.;
    let mut baseDist: f32_0 = 0.;
    let mut ret: f32_0 = 0.;
    if (*bankEntry).sfxParams as libc::c_int & 0x2000 as libc::c_int != 0 {
        return 1.0f32
    }
    if (*bankEntry).dist > 10000.0f32 {
        ret = 0.0f32
    } else {
        match (*bankEntry).sfxParams as libc::c_int & 3 as libc::c_int {
            1 => { baseDist = 10000.0f32 / 15.0f32 }
            2 => { baseDist = 10000.0f32 / 10.5f32 }
            3 => { baseDist = 10000.0f32 / 2.6f32 }
            _ => { baseDist = 10000.0f32 / 20.0f32 }
        }
        minDist = baseDist / 5.0f32;
        // Volume grows as inverse square of distance. Linearly approximate
        // the inverse part, then square.
        if (*bankEntry).dist < minDist {
            ret = 1.0f32
        } else if (*bankEntry).dist < baseDist {
            ret =
                (baseDist - minDist - ((*bankEntry).dist - minDist)) /
                    (baseDist - minDist) * 0.19f32 + 0.81f32
        } else {
            ret =
                (1.0f32 -
                     ((*bankEntry).dist - baseDist) / (10000.0f32 - baseDist))
                    * 0.81f32
        }
        ret = ret * ret
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_ComputeSoundReverb(mut bankId: u8_0,
                                                  mut entryIdx: u8_0,
                                                  mut channelIdx: u8_0)
 -> s8 {
    let mut distAdd: s8 = 0 as libc::c_int as s8;
    let mut scriptAdd: s32 = 0 as libc::c_int;
    let mut entry_0: *mut SoundBankEntry =
        &mut *(*gSoundBanks.as_mut_ptr().offset(bankId as
                                                    isize)).offset(entryIdx as
                                                                       isize)
            as *mut SoundBankEntry;
    let mut reverb: s32 = 0;
    if (*entry_0).sfxParams as libc::c_int & 0x1000 as libc::c_int == 0 {
        if (*entry_0).dist < 2500.0f32 {
            distAdd =
                if *(*entry_0).posZ > 0.0f32 {
                    ((*entry_0).dist / 2500.0f32) * 70.0f32
                } else { ((*entry_0).dist / 2500.0f32) * 91.0f32 } as s8
        } else { distAdd = 70 as libc::c_int as s8 }
    }
    if gAudioContext.seqPlayers[SEQ_PLAYER_SFX as libc::c_int as
                                    usize].channels[channelIdx as usize] as
           u32_0 !=
           &mut gAudioContext.sequenceChannelNone as *mut SequenceChannel as
               u32_0 {
        scriptAdd =
            (*gAudioContext.seqPlayers[SEQ_PLAYER_SFX as libc::c_int as
                                           usize].channels[channelIdx as
                                                               usize]).soundScriptIO[1
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         usize]
                as s32;
        if ((*gAudioContext.seqPlayers[SEQ_PLAYER_SFX as libc::c_int as
                                           usize].channels[channelIdx as
                                                               usize]).soundScriptIO[1
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         usize]
                as libc::c_int) < 0 as libc::c_int {
            scriptAdd = 0 as libc::c_int
        }
    }
    reverb =
        *(*entry_0).reverbAdd as libc::c_int + distAdd as libc::c_int +
            scriptAdd;
    if bankId as libc::c_int != BANK_OCARINA as libc::c_int ||
           !(((*entry_0).sfxId as libc::c_int & 0x1ff as libc::c_int) <
                 2 as libc::c_int) {
        reverb +=
            sAudioEnvReverb as libc::c_int + sAudioCodeReverb as libc::c_int +
                sSpecReverb as libc::c_int
    }
    if reverb > 0x7f as libc::c_int { reverb = 0x7f as libc::c_int }
    return reverb as s8;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_ComputeSoundPanSigned(mut x: f32_0,
                                                     mut z: f32_0,
                                                     mut token: u8_0) -> s8 {
    let mut absX: f32_0 = 0.;
    let mut absZ: f32_0 = 0.;
    let mut pan: f32_0 = 0.;
    if x < 0 as libc::c_int as libc::c_float { absX = -x } else { absX = x }
    if z < 0 as libc::c_int as libc::c_float { absZ = -z } else { absZ = z }
    if absX > 8000.0f32 { absX = 8000.0f32 }
    if absZ > 8000.0f32 { absZ = 8000.0f32 }
    if x == 0.0f32 && z == 0.0f32 {
        pan = 0.5f32
    } else if absZ <= absX {
        pan = (16000.0f32 - absX) / (3.3f32 * (16000.0f32 - absZ));
        if x >= 0.0f32 { pan = 1.0f32 - pan }
    } else {
        pan = x / (5.0769234f32 * absZ) + 0.5f32
        // about 66 / 13
    }
    if absZ < 50.0f32 {
        if absX < 50.0f32 {
            pan =
                (pan - 0.5f32) * (absX / 50.0f32 * (absX / 50.0f32)) + 0.5f32
        }
    }
    return (pan * 127.0f32 + 0.5f32) as s8;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_ComputeSoundFreqScale(mut bankId: u8_0,
                                                     mut entryIdx: u8_0)
 -> f32_0 {
    let mut phi_v0: s32 = 0 as libc::c_int;
    let mut entry_0: *mut SoundBankEntry =
        &mut *(*gSoundBanks.as_mut_ptr().offset(bankId as
                                                    isize)).offset(entryIdx as
                                                                       isize)
            as *mut SoundBankEntry;
    let mut unk1C: f32_0 = 0.;
    let mut freq: f32_0 = 1.0f32;
    if (*entry_0).sfxParams as libc::c_int & 0x4000 as libc::c_int != 0 {
        freq =
            1.0f32 -
                (gAudioContext.audioRandom &
                     0xf as libc::c_int as libc::c_uint) as libc::c_float /
                    192.0f32
    }
    match bankId as libc::c_int {
        0 | 1 | 6 => {
            if sAudioBaseFilter2 as libc::c_int != 0 as libc::c_int {
                phi_v0 = 1 as libc::c_int
            }
        }
        2 | 3 => {
            if sAudioExtraFilter2 as libc::c_int != 0 as libc::c_int {
                phi_v0 = 1 as libc::c_int
            }
        }
        4 | 5 | _ => { }
    }
    if phi_v0 == 1 as libc::c_int {
        if (*entry_0).sfxParams as libc::c_int & 0x800 as libc::c_int == 0 {
            freq =
                (freq as libc::c_double *
                     (1.0293f64 -
                          ((gAudioContext.audioRandom &
                                0xf as libc::c_int as libc::c_uint) as
                               libc::c_float / 144.0f32) as libc::c_double))
                    as f32_0
        }
    }
    unk1C = (*entry_0).dist;
    if (*entry_0).sfxParams as libc::c_int & 0x2000 as libc::c_int == 0 {
        if (*entry_0).sfxParams as libc::c_int & 0x8000 as libc::c_int == 0 {
            if unk1C >= 10000.0f32 {
                freq += 0.2f32
            } else { freq += 0.2f32 * (unk1C / 10000.0f32) }
        }
    }
    if (*entry_0).sfxParams as libc::c_int & 0xc0 as libc::c_int != 0 {
        freq += (*entry_0).unk_2F as libc::c_int as libc::c_float / 192.0f32
    }
    return freq;
}
#[no_mangle]
pub unsafe extern "C" fn func_800F37B8(mut behindScreenZ: f32_0,
                                       mut arg1: *mut SoundBankEntry,
                                       mut arg2: s8) -> u8_0 {
    let mut phi_v0: s8 = 0;
    let mut phi_v1: u8_0 = 0;
    let mut phi_f0: f32_0 = 0.;
    let mut phi_f12: f32_0 = 0.;
    if *(*arg1).posZ < behindScreenZ {
        phi_v0 =
            if (arg2 as libc::c_int) < 65 as libc::c_int {
                arg2 as libc::c_int
            } else { (0x7f as libc::c_int) - arg2 as libc::c_int } as s8;
        if (phi_v0 as libc::c_int) < 30 as libc::c_int {
            phi_v1 = 0 as libc::c_int as u8_0
        } else {
            phi_v1 =
                (((phi_v0 as libc::c_int & 0xffff as libc::c_int) *
                      10 as libc::c_int - 300 as libc::c_int) /
                     34 as libc::c_int) as u8_0;
            if phi_v1 as libc::c_int != 0 as libc::c_int {
                phi_v1 = (0x10 as libc::c_int - phi_v1 as libc::c_int) as u8_0
            }
        }
    } else { phi_v1 = 0 as libc::c_int as u8_0 }
    if phi_v1 as libc::c_int == 0 as libc::c_int {
        if (*arg1).sfxParams as libc::c_int & 0x200 as libc::c_int != 0 {
            phi_v1 = 0xf as libc::c_int as u8_0
        }
    }
    match (*arg1).sfxParams as libc::c_int & 3 as libc::c_int {
        1 => { phi_f0 = 12.0f32 }
        2 => { phi_f0 = 9.0f32 }
        3 => { phi_f0 = 6.0f32 }
        _ => { phi_f0 = 15.0f32 }
    }
    phi_f12 =
        if (*arg1).dist > 10000.0f32 / 5.2f32 {
            (10000.0f32) / 5.2f32
        } else { (*arg1).dist };
    return (phi_v1 as libc::c_int * 0x10 as libc::c_int +
                (phi_f0 * phi_f12 / (10000.0f32 / 5.2f32)) as u8_0 as
                    libc::c_int) as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn func_800F3990(mut arg0: f32_0, mut sfxParams: u16_0)
 -> s8 {
    let mut ret: s8 = 0 as libc::c_int as s8;
    if arg0 >= 0.0f32 {
        if arg0 > 625.0f32 {
            ret = 127 as libc::c_int as s8
        } else { ret = (arg0 / 625.0f32 * 126.0f32) as s8 }
    }
    return (ret as libc::c_int | 1 as libc::c_int) as s8;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_SetSoundProperties(mut bankId: u8_0,
                                                  mut entryIdx: u8_0,
                                                  mut channelIdx: u8_0) {
    let mut vol: f32_0 = 1.0f32;
    let mut volS8: s8 = 0;
    let mut reverb: s8 = 0 as libc::c_int as s8;
    let mut freqScale: f32_0 = 1.0f32;
    let mut panSigned: s8 = 0x40 as libc::c_int as s8;
    let mut stereoBits: u8_0 = 0 as libc::c_int as u8_0;
    let mut filter: u8_0 = 0 as libc::c_int as u8_0;
    let mut sp38: s8 = 0 as libc::c_int as s8;
    let mut behindScreenZ: f32_0 = 0.;
    let mut baseFilter: u8_0 = 0 as libc::c_int as u8_0;
    let mut entry_0: *mut SoundBankEntry =
        &mut *(*gSoundBanks.as_mut_ptr().offset(bankId as
                                                    isize)).offset(entryIdx as
                                                                       isize)
            as *mut SoundBankEntry;
    let mut current_block_35: u64;
    match bankId as libc::c_int {
        0 | 1 | 2 | 3 | 6 => {
            if D_80130604 as libc::c_int == 2 as libc::c_int {
                sp38 = func_800F3990(*(*entry_0).posY, (*entry_0).sfxParams)
            }
            current_block_35 = 13394931369644283952;
        }
        5 => { current_block_35 = 13394931369644283952; }
        4 | _ => { current_block_35 = 3938820862080741272; }
    }
    match current_block_35 {
        13394931369644283952 =>
        // fallthrough
        {
            (*entry_0).dist = sqrtf((*entry_0).dist);
            vol =
                Audio_ComputeSoundVolume(bankId, entryIdx) * *(*entry_0).vol;
            reverb = Audio_ComputeSoundReverb(bankId, entryIdx, channelIdx);
            panSigned =
                Audio_ComputeSoundPanSigned(*(*entry_0).posX,
                                            *(*entry_0).posZ,
                                            (*entry_0).token);
            freqScale =
                Audio_ComputeSoundFreqScale(bankId, entryIdx) *
                    *(*entry_0).freqScale;
            if D_80130604 as libc::c_int == 2 as libc::c_int {
                behindScreenZ =
                    sBehindScreenZ[(((*entry_0).sfxParams as libc::c_int &
                                         0x400 as libc::c_int) >>
                                        10 as libc::c_int) as usize];
                if (*entry_0).sfxParams as libc::c_int & 0x800 as libc::c_int
                       == 0 {
                    if *(*entry_0).posZ < behindScreenZ {
                        stereoBits = 0x10 as libc::c_int as u8_0
                    }
                    if (sSfxChannelState[channelIdx as usize].stereoBits as
                            libc::c_int ^ stereoBits as libc::c_int) &
                           0x10 as libc::c_int != 0 {
                        if (panSigned as libc::c_int) < 0x40 as libc::c_int {
                            stereoBits =
                                (sSfxChannelState[channelIdx as
                                                      usize].stereoBits as
                                     libc::c_int ^ 0x14 as libc::c_int) as
                                    u8_0
                        } else {
                            stereoBits =
                                (sSfxChannelState[channelIdx as
                                                      usize].stereoBits as
                                     libc::c_int ^ 0x18 as libc::c_int) as
                                    u8_0
                        }
                    } else {
                        stereoBits =
                            sSfxChannelState[channelIdx as usize].stereoBits
                                as u8_0
                    }
                }
            }
            if sAudioBaseFilter as libc::c_int != 0 as libc::c_int {
                if bankId as libc::c_int == BANK_ITEM as libc::c_int ||
                       bankId as libc::c_int == BANK_PLAYER as libc::c_int ||
                       bankId as libc::c_int == BANK_VOICE as libc::c_int {
                    baseFilter = sAudioBaseFilter
                }
            }
            if baseFilter as libc::c_int | sAudioExtraFilter as libc::c_int !=
                   0 as libc::c_int {
                filter =
                    (baseFilter as libc::c_int |
                         sAudioExtraFilter as libc::c_int) as u8_0
            } else if D_80130604 as libc::c_int == 2 as libc::c_int &&
                          (*entry_0).sfxParams as libc::c_int &
                              0x2000 as libc::c_int == 0 as libc::c_int {
                filter = func_800F37B8(behindScreenZ, entry_0, panSigned)
            }
        }
        _ => { }
    }
    if sSfxChannelState[channelIdx as usize].vol != vol {
        volS8 = (vol * 127.0f32) as u8_0 as s8;
        sSfxChannelState[channelIdx as usize].vol = vol
    } else { volS8 = -(1 as libc::c_int) as s8 }
    // CHAN_UPD_SCRIPT_IO (slot 2, sets volume)
    Audio_QueueCmdS8(((0x6 as libc::c_int) << 24 as libc::c_int |
                          (SEQ_PLAYER_SFX as libc::c_int) << 16 as libc::c_int
                          | (channelIdx as libc::c_int) << 8 as libc::c_int |
                          2 as libc::c_int) as u32_0, volS8);
    if reverb as libc::c_int !=
           sSfxChannelState[channelIdx as usize].reverb as libc::c_int {
        Audio_QueueCmdS8(((0x5 as libc::c_int) << 24 as libc::c_int |
                              (SEQ_PLAYER_SFX as libc::c_int) <<
                                  16 as libc::c_int |
                              (channelIdx as libc::c_int) << 8 as libc::c_int)
                             as u32_0, reverb);
        sSfxChannelState[channelIdx as usize].reverb = reverb
    }
    if freqScale != sSfxChannelState[channelIdx as usize].freqScale {
        Audio_QueueCmdF32(((0x4 as libc::c_int) << 24 as libc::c_int |
                               (SEQ_PLAYER_SFX as libc::c_int) <<
                                   16 as libc::c_int |
                               (channelIdx as libc::c_int) <<
                                   8 as libc::c_int) as u32_0, freqScale);
        sSfxChannelState[channelIdx as usize].freqScale = freqScale
    }
    if stereoBits as libc::c_int !=
           sSfxChannelState[channelIdx as usize].stereoBits as libc::c_int {
        Audio_QueueCmdS8(((0xe as libc::c_int) << 24 as libc::c_int |
                              (SEQ_PLAYER_SFX as libc::c_int) <<
                                  16 as libc::c_int |
                              (channelIdx as libc::c_int) << 8 as libc::c_int)
                             as u32_0,
                         (stereoBits as libc::c_int | 0x10 as libc::c_int) as
                             s8);
        sSfxChannelState[channelIdx as usize].stereoBits = stereoBits as s8
    }
    if filter as libc::c_int !=
           sSfxChannelState[channelIdx as usize].filter as libc::c_int {
        // CHAN_UPD_SCRIPT_IO (slot 3, sets filter)
        Audio_QueueCmdS8(((0x6 as libc::c_int) << 24 as libc::c_int |
                              (SEQ_PLAYER_SFX as libc::c_int) <<
                                  16 as libc::c_int |
                              (channelIdx as libc::c_int) << 8 as libc::c_int
                              | 3 as libc::c_int) as u32_0, filter as s8);
        sSfxChannelState[channelIdx as usize].filter = filter
    }
    if sp38 as libc::c_int !=
           sSfxChannelState[channelIdx as usize].unk_0C as libc::c_int {
        // CHAN_UPD_UNK_0F
        Audio_QueueCmdS8(((0xc as libc::c_int) << 24 as libc::c_int |
                              (SEQ_PLAYER_SFX as libc::c_int) <<
                                  16 as libc::c_int |
                              (channelIdx as libc::c_int) << 8 as libc::c_int)
                             as u32_0, 0x10 as libc::c_int as s8);
        // CHAN_UPD_UNK_20
        Audio_QueueCmdU16(((0xd as libc::c_int) << 24 as libc::c_int |
                               (SEQ_PLAYER_SFX as libc::c_int) <<
                                   16 as libc::c_int |
                               (channelIdx as libc::c_int) <<
                                   8 as libc::c_int) as u32_0,
                          (((sp38 as u16_0 as libc::c_int) <<
                                8 as libc::c_int) + 0xff as libc::c_int) as
                              u16_0);
        sSfxChannelState[channelIdx as usize].unk_0C = sp38 as u8_0
    }
    if panSigned as libc::c_int !=
           sSfxChannelState[channelIdx as usize].panSigned as libc::c_int {
        Audio_QueueCmdS8(((0x3 as libc::c_int) << 24 as libc::c_int |
                              (SEQ_PLAYER_SFX as libc::c_int) <<
                                  16 as libc::c_int |
                              (channelIdx as libc::c_int) << 8 as libc::c_int)
                             as u32_0, panSigned);
        sSfxChannelState[channelIdx as usize].panSigned = panSigned
    };
}
#[no_mangle]
pub unsafe extern "C" fn Audio_ResetSfxChannelState() {
    let mut i: u8_0 = 0;
    let mut state: *mut SfxPlayerState = 0 as *mut SfxPlayerState;
    i = 0 as libc::c_int as u8_0;
    while (i as libc::c_int) < 16 as libc::c_int {
        state =
            &mut *sSfxChannelState.as_mut_ptr().offset(i as isize) as
                *mut SfxPlayerState;
        (*state).vol = 1.0f32;
        (*state).freqScale = 1.0f32;
        (*state).reverb = 0 as libc::c_int as s8;
        (*state).panSigned = 0x40 as libc::c_int as s8;
        (*state).stereoBits = 0 as libc::c_int as s8;
        (*state).filter = 0xff as libc::c_int as u8_0;
        (*state).unk_0C = 0xff as libc::c_int as u8_0;
        i = i.wrapping_add(1)
    }
    sSfxChannelState[13 as libc::c_int as usize].unk_0C =
        0 as libc::c_int as u8_0;
    sPrevSeqMode = 0 as libc::c_int as u8_0;
    sAudioCodeReverb = 0 as libc::c_int as s8;
}
#[no_mangle]
pub unsafe extern "C" fn func_800F3F3C(mut arg0: u8_0) {
    if *gSoundBankMuted.as_mut_ptr().offset(0 as libc::c_int as isize) as
           libc::c_int != 1 as libc::c_int {
        Audio_QueueSeqCmd((0 as libc::c_int |
                               (SEQ_PLAYER_BGM_SUB as libc::c_int as u8_0 as
                                    libc::c_int) << 24 as libc::c_int |
                               (0 as libc::c_int as u8_0 as libc::c_int) <<
                                   0x10 as libc::c_int |
                               0x6d as libc::c_int as u16_0 as libc::c_int) as
                              u32_0);
        Audio_QueueSeqCmd(0x80000000 as libc::c_uint |
                              ((SEQ_PLAYER_BGM_SUB as libc::c_int as u8_0 as
                                    libc::c_int) << 24 as libc::c_int) as
                                  libc::c_uint |
                              ((0 as libc::c_int as u8_0 as libc::c_int) <<
                                   16 as libc::c_int) as libc::c_uint |
                              ((0 as libc::c_int as u8_0 as libc::c_int) <<
                                   8 as libc::c_int) as libc::c_uint |
                              arg0 as libc::c_uint);
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800F3F84(mut arg0: f32_0) -> f32_0 {
    let mut ret: f32_0 = 1.0f32;
    if arg0 > 6.0f32 {
        D_8016B7A8 = 1.0f32;
        D_8016B7B0 = 1.1f32
    } else {
        ret = arg0 / 6.0f32;
        D_8016B7A8 = ret * 0.22500002f32 + 0.775f32;
        D_8016B7B0 = ret * 0.2f32 + 0.9f32
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn func_800F4010(mut pos: *mut Vec3f, mut sfxId: u16_0,
                                       mut arg2: f32_0) {
    let mut sp24: f32_0 = 0.;
    let mut phi_f0: f32_0 = 0.;
    let mut phi_v0: u8_0 = 0;
    let mut sfxId2: u16_0 = 0;
    D_80131C8C = arg2;
    sp24 = func_800F3F84(arg2);
    Audio_PlaySoundGeneral(sfxId, pos, 4 as libc::c_int as u8_0,
                           &mut D_8016B7B0, &mut D_8016B7A8, &mut D_801333E8);
    if sfxId as libc::c_int & 0xf0 as libc::c_int == 0xb0 as libc::c_int {
        phi_f0 = 0.3f32;
        phi_v0 = 1 as libc::c_int as u8_0;
        sp24 = 1.0f32
    } else {
        phi_f0 = 1.1f32;
        phi_v0 =
            gAudioContext.audioRandom.wrapping_rem(2 as libc::c_int as
                                                       libc::c_uint) as u8_0
    }
    if phi_f0 < arg2 && phi_v0 as libc::c_int != 0 as libc::c_int {
        if sfxId as libc::c_int & 0x80 as libc::c_int != 0 as libc::c_int {
            sfxId2 = 0x867 as libc::c_int as u16_0
        } else { sfxId2 = 0x866 as libc::c_int as u16_0 }
        D_8016B7AC = (sp24 as libc::c_double * 0.7f64 + 0.3f64) as f32_0;
        Audio_PlaySoundGeneral(sfxId2, pos, 4 as libc::c_int as u8_0,
                               &mut D_8016B7B0, &mut D_8016B7AC,
                               &mut D_801333E8);
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800F4138(mut pos: *mut Vec3f, mut sfxId: u16_0,
                                       mut arg2: f32_0) {
    func_800F3F84(arg2);
    Audio_PlaySoundGeneral(sfxId, pos, 4 as libc::c_int as u8_0,
                           &mut D_8016B7B0, &mut D_8016B7A8, &mut D_801333E8);
}
#[no_mangle]
pub unsafe extern "C" fn func_800F4190(mut pos: *mut Vec3f,
                                       mut sfxId: u16_0) {
    Audio_PlaySoundGeneral(sfxId, pos, 4 as libc::c_int as u8_0,
                           &mut D_801305B0, &mut D_801333E0, &mut D_801305B4);
}
#[no_mangle]
pub unsafe extern "C" fn Audio_PlaySoundRandom(mut pos: *mut Vec3f,
                                               mut baseSfxId: u16_0,
                                               mut randLim: u8_0) {
    let mut offset: u8_0 =
        Audio_NextRandom().wrapping_rem(randLim as libc::c_uint) as u8_0;
    Audio_PlaySoundGeneral((baseSfxId as libc::c_int + offset as libc::c_int)
                               as u16_0, pos, 4 as libc::c_int as u8_0,
                           &mut D_801333E0, &mut D_801333E0, &mut D_801333E8);
}
#[no_mangle]
pub unsafe extern "C" fn func_800F4254(mut pos: *mut Vec3f, mut level: u8_0) {
    level = (level as libc::c_int & 3 as libc::c_int) as u8_0;
    if level as libc::c_int != sPrevChargeLevel as libc::c_int {
        D_801305F4 = D_801305E4[level as usize];
        match level as libc::c_int {
            1 => {
                Audio_PlaySoundGeneral(0x86d as libc::c_int as u16_0, pos,
                                       4 as libc::c_int as u8_0,
                                       &mut D_801305F4, &mut D_801333E0,
                                       &mut D_801333E8);
            }
            2 => {
                Audio_PlaySoundGeneral(0x86d as libc::c_int as u16_0, pos,
                                       4 as libc::c_int as u8_0,
                                       &mut D_801305F4, &mut D_801333E0,
                                       &mut D_801333E8);
            }
            _ => { }
        }
        sPrevChargeLevel = level
    }
    if level as libc::c_int != 0 as libc::c_int {
        Audio_PlaySoundGeneral((0x1822 as libc::c_int - 0x800 as libc::c_int)
                                   as u16_0, pos, 4 as libc::c_int as u8_0,
                               &mut D_801305F4, &mut D_801333E0,
                               &mut D_801333E8);
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800F436C(mut pos: *mut Vec3f, mut sfxId: u16_0,
                                       mut arg2: f32_0) {
    if arg2 < 0.75f32 {
        D_8016B7D8 = arg2 / 0.75f32 * 0.25f32 + 0.5f32
    } else { D_8016B7D8 = arg2 }
    if D_8016B7D8 > 0.5f32 {
        Audio_PlaySoundGeneral(sfxId, pos, 4 as libc::c_int as u8_0,
                               &mut D_8016B7D8, &mut D_801333E0,
                               &mut D_801333E8);
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800F4414(mut pos: *mut Vec3f, mut sfxId: u16_0,
                                       mut arg2: f32_0) {
    D_801305B8 -= 1;
    if D_801305B8 as libc::c_int == 0 as libc::c_int {
        Audio_PlaySoundGeneral(sfxId, pos, 4 as libc::c_int as u8_0,
                               &mut D_8016B7D8, &mut D_801333E0,
                               &mut D_801333E8);
        if arg2 > 2.0f32 { arg2 = 2.0f32 }
        D_801305B8 =
            (((D_801305C0 as libc::c_int - D_801305BC as libc::c_int) as
                  libc::c_float * (1.0f32 - arg2)) as s8 as libc::c_int +
                 D_801305C0 as libc::c_int) as s8
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800F44EC(mut arg0: s8, mut arg1: s8) {
    D_801305B8 = 1 as libc::c_int as s8;
    D_801305BC = arg1;
    D_801305C0 = arg0;
}
#[no_mangle]
pub unsafe extern "C" fn func_800F4524(mut pos: *mut Vec3f, mut sfxId: u16_0,
                                       mut arg2: s8) {
    D_8016B7DC = arg2;
    Audio_PlaySoundGeneral(sfxId, pos, 4 as libc::c_int as u8_0,
                           &mut D_801333E0, &mut D_801333E0, &mut D_8016B7DC);
}
#[no_mangle]
pub unsafe extern "C" fn func_800F4578(mut pos: *mut Vec3f, mut sfxId: u16_0,
                                       mut arg2: f32_0) {
    D_8016B7E0 = arg2;
    Audio_PlaySoundGeneral(sfxId, pos, 4 as libc::c_int as u8_0,
                           &mut D_801333E0, &mut D_8016B7E0, &mut D_801333E8);
}
#[no_mangle]
pub unsafe extern "C" fn func_800F45D0(mut arg0: f32_0) {
    func_800F4414(&mut D_801333D4,
                  (0x183d as libc::c_int - 0x800 as libc::c_int) as u16_0,
                  arg0);
    func_800F436C(&mut D_801333D4, 0 as libc::c_int as u16_0,
                  0.15f32 * arg0 + 1.4f32);
}
#[no_mangle]
pub unsafe extern "C" fn Audio_PlaySoundRiver(mut pos: *mut Vec3f,
                                              mut freqScale: f32_0) {
    if Audio_IsSfxPlaying((0x2806 as libc::c_int - 0x800 as libc::c_int) as
                              u32_0) == 0 {
        sRiverFreqScaleLerp.value = freqScale
    } else if freqScale != sRiverFreqScaleLerp.value {
        sRiverFreqScaleLerp.target = freqScale;
        sRiverFreqScaleLerp.remainingFrames = 40 as libc::c_int;
        sRiverFreqScaleLerp.step =
            (sRiverFreqScaleLerp.target - sRiverFreqScaleLerp.value) /
                40 as libc::c_int as libc::c_float
    }
    Audio_PlaySoundGeneral((0x2806 as libc::c_int - 0x800 as libc::c_int) as
                               u16_0, pos, 4 as libc::c_int as u8_0,
                           &mut sRiverFreqScaleLerp.value, &mut D_801333E0,
                           &mut D_801333E8);
}
#[no_mangle]
pub unsafe extern "C" fn Audio_PlaySoundWaterfall(mut pos: *mut Vec3f,
                                                  mut freqScale: f32_0) {
    if Audio_IsSfxPlaying((0x2807 as libc::c_int - 0x800 as libc::c_int) as
                              u32_0) == 0 {
        sWaterfallFreqScaleLerp.value = freqScale
    } else if freqScale != sWaterfallFreqScaleLerp.value {
        sWaterfallFreqScaleLerp.target = freqScale;
        sWaterfallFreqScaleLerp.remainingFrames = 40 as libc::c_int;
        sWaterfallFreqScaleLerp.step =
            (sWaterfallFreqScaleLerp.target - sWaterfallFreqScaleLerp.value) /
                40 as libc::c_int as libc::c_float
    }
    Audio_PlaySoundGeneral((0x2807 as libc::c_int - 0x800 as libc::c_int) as
                               u16_0, pos, 4 as libc::c_int as u8_0,
                           &mut sWaterfallFreqScaleLerp.value,
                           &mut sWaterfallFreqScaleLerp.value,
                           &mut D_801333E8);
}
#[no_mangle]
pub unsafe extern "C" fn Audio_StepFreqLerp(mut lerp: *mut FreqLerp) {
    if (*lerp).remainingFrames != 0 as libc::c_int {
        (*lerp).remainingFrames -= 1;
        if (*lerp).remainingFrames != 0 as libc::c_int {
            (*lerp).value += (*lerp).step
        } else { (*lerp).value = (*lerp).target }
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800F47BC() {
    Audio_SetVolScale(SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0,
                      1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
                      10 as libc::c_int as u8_0);
    Audio_SetVolScale(SEQ_PLAYER_BGM_SUB as libc::c_int as u8_0,
                      1 as libc::c_int as u8_0, 0 as libc::c_int as u8_0,
                      10 as libc::c_int as u8_0);
}
#[no_mangle]
pub unsafe extern "C" fn func_800F47FC() {
    Audio_SetVolScale(SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0,
                      1 as libc::c_int as u8_0, 0x7f as libc::c_int as u8_0,
                      3 as libc::c_int as u8_0);
    Audio_SetVolScale(SEQ_PLAYER_BGM_SUB as libc::c_int as u8_0,
                      1 as libc::c_int as u8_0, 0x7f as libc::c_int as u8_0,
                      3 as libc::c_int as u8_0);
}
#[no_mangle]
pub unsafe extern "C" fn func_800F483C(mut targetVol: u8_0,
                                       mut volFadeTimer: u8_0) {
    Audio_SetVolScale(SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0,
                      0 as libc::c_int as u8_0, targetVol, volFadeTimer);
}
#[no_mangle]
pub unsafe extern "C" fn func_800F4870(mut arg0: u8_0) {
    let mut pan: s8 = 0;
    let mut i: u8_0 = 0;
    pan = 0 as libc::c_int as s8;
    if arg0 as libc::c_int == 0 as libc::c_int {
        pan = 0x7f as libc::c_int as s8
    }
    i = 0 as libc::c_int as u8_0;
    while (i as libc::c_int) < 16 as libc::c_int {
        // CHAN_UPD_PAN_UNSIGNED
        Audio_QueueCmdS8((0x7 as libc::c_int as u32_0 &
                              (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                   1 as libc::c_int) as libc::c_uint) <<
                             24 as libc::c_int |
                             (SEQ_PLAYER_BGM_MAIN as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 16 as libc::c_int |
                             (i as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 8 as libc::c_int |
                             (0 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 0 as libc::c_int, pan);
        i = i.wrapping_add(1)
    }
    if arg0 as libc::c_int == 7 as libc::c_int {
        D_80130600 = 2 as libc::c_int as u8_0
    } else {
        Audio_SetGanonDistVol(D_801305F8[(arg0 as libc::c_int &
                                              7 as libc::c_int) as usize]);
    };
}
// (name derived from debug strings, should probably update. used in ganon/ganon_boss scenes)
#[no_mangle]
pub unsafe extern "C" fn Audio_SetGanonDistVol(mut targetVol: u8_0) -> s32 {
    let mut phi_v0: u8_0 = 0;
    let mut phi_v0_2: u16_0 = 0;
    let mut i: u8_0 = 0;
    if sAudioGanonDistVol as libc::c_int != targetVol as libc::c_int {
        Audio_SetVolScale(SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0,
                          0 as libc::c_int as u8_0, targetVol,
                          2 as libc::c_int as u8_0);
        if (targetVol as libc::c_int) < 0x40 as libc::c_int {
            phi_v0 = 0x10 as libc::c_int as u8_0
        } else {
            phi_v0 =
                (((targetVol as libc::c_int - 0x40 as libc::c_int >>
                       2 as libc::c_int) + 1 as libc::c_int) <<
                     4 as libc::c_int) as u8_0
        }
        Audio_QueueSeqCmd(0x80000000 as libc::c_uint |
                              ((SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0 as
                                    libc::c_int) << 24 as libc::c_int) as
                                  libc::c_uint |
                              ((4 as libc::c_int as u8_0 as libc::c_int) <<
                                   16 as libc::c_int) as libc::c_uint |
                              ((15 as libc::c_int as u8_0 as libc::c_int) <<
                                   8 as libc::c_int) as libc::c_uint |
                              phi_v0 as libc::c_uint);
        i = 0 as libc::c_int as u8_0;
        while (i as libc::c_int) < 0x10 as libc::c_int {
            if gAudioContext.seqPlayers[SEQ_PLAYER_BGM_MAIN as libc::c_int as
                                            usize].channels[i as usize] !=
                   &mut gAudioContext.sequenceChannelNone as
                       *mut SequenceChannel {
                if (*gAudioContext.seqPlayers[SEQ_PLAYER_BGM_MAIN as
                                                  libc::c_int as
                                                  usize].channels[i as
                                                                      usize]).soundScriptIO[5
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                usize]
                       as u8_0 as libc::c_int != 0xff as libc::c_int {
                    // this looks like some kind of macro?
                    phi_v0_2 =
                        ((*gAudioContext.seqPlayers[SEQ_PLAYER_BGM_MAIN as
                                                        libc::c_int as
                                                        usize].channels[i as
                                                                            usize]).soundScriptIO[5
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      usize]
                             as u16_0 as libc::c_int -
                             targetVol as libc::c_int + 0x7f as libc::c_int)
                            as u16_0;
                    if phi_v0_2 as libc::c_int >= 0x80 as libc::c_int {
                        phi_v0_2 = 0x7f as libc::c_int as u16_0
                    }
                    // CHAN_UPD_REVERB
                    Audio_QueueCmdS8((0x5 as libc::c_int as u32_0 &
                                          (((0x1 as libc::c_int) <<
                                                8 as libc::c_int) -
                                               1 as libc::c_int) as
                                              libc::c_uint) <<
                                         24 as libc::c_int |
                                         (SEQ_PLAYER_BGM_MAIN as libc::c_int
                                              as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             16 as libc::c_int |
                                         (i as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             8 as libc::c_int |
                                         (0 as libc::c_int as u32_0 &
                                              (((0x1 as libc::c_int) <<
                                                    8 as libc::c_int) -
                                                   1 as libc::c_int) as
                                                  libc::c_uint) <<
                                             0 as libc::c_int,
                                     phi_v0_2 as u8_0 as s8);
                }
            }
            i = i.wrapping_add(1)
        }
        sAudioGanonDistVol = targetVol
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn func_800F4A54(mut arg0: u8_0) {
    D_8016B8B0 = arg0;
    D_8016B8B2 = 1 as libc::c_int as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn func_800F4A70() {
    if D_8016B8B2 as libc::c_int == 1 as libc::c_int {
        if D_8016B8B1 as libc::c_int != D_8016B8B0 as libc::c_int {
            Audio_SetVolScale(SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0,
                              0 as libc::c_int as u8_0, D_8016B8B0,
                              0xa as libc::c_int as u8_0);
            D_8016B8B1 = D_8016B8B0;
            D_8016B8B3 = 1 as libc::c_int as u8_0
        }
        D_8016B8B2 = 0 as libc::c_int as u8_0
    } else if D_8016B8B3 as libc::c_int == 1 as libc::c_int &&
                  D_80130608 as libc::c_int == 0 as libc::c_int {
        Audio_SetVolScale(SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0,
                          0 as libc::c_int as u8_0,
                          0x7f as libc::c_int as u8_0,
                          0xa as libc::c_int as u8_0);
        D_8016B8B1 = 0x7f as libc::c_int as u8_0;
        D_8016B8B3 = 0 as libc::c_int as u8_0
    }
    if D_80130600 as libc::c_int != 0 as libc::c_int {
        D_80130600 = D_80130600.wrapping_sub(1);
        if D_80130600 as libc::c_int == 0 as libc::c_int {
            Audio_SetGanonDistVol(D_801305F8[7 as libc::c_int as usize]);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Audio_PlaySoundIncreasinglyTransposed(mut pos:
                                                                   *mut Vec3f,
                                                               mut sfxId: s16,
                                                               mut semitones:
                                                                   *mut u8_0) {
    Audio_PlaySoundGeneral(sfxId as u16_0, pos, 4 as libc::c_int as u8_0,
                           &mut *gNoteFrequencies.as_mut_ptr().offset((*semitones.offset(sAudioIncreasingTranspose
                                                                                             as
                                                                                             isize)
                                                                           as
                                                                           libc::c_int
                                                                           +
                                                                           39
                                                                               as
                                                                               libc::c_int)
                                                                          as
                                                                          isize),
                           &mut D_801333E0, &mut D_801333E8);
    if (sAudioIncreasingTranspose as libc::c_int) < 15 as libc::c_int {
        sAudioIncreasingTranspose = sAudioIncreasingTranspose.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn Audio_ResetIncreasingTranspose() {
    sAudioIncreasingTranspose = 0 as libc::c_int as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_PlaySoundTransposed(mut pos: *mut Vec3f,
                                                   mut sfxId: u16_0,
                                                   mut semitone: s8) {
    Audio_PlaySoundGeneral(sfxId, pos, 4 as libc::c_int as u8_0,
                           &mut *gNoteFrequencies.as_mut_ptr().offset((semitone
                                                                           as
                                                                           libc::c_int
                                                                           +
                                                                           39
                                                                               as
                                                                               libc::c_int)
                                                                          as
                                                                          isize),
                           &mut D_801333E0, &mut D_801333E8);
}
#[no_mangle]
pub unsafe extern "C" fn func_800F4C58(mut pos: *mut Vec3f, mut sfxId: u16_0,
                                       mut arg2: u8_0) {
    let mut phi_s1: u8_0 = 0 as libc::c_int as u8_0;
    let mut i: u8_0 = 0;
    let mut bankId: u8_0 = 0;
    bankId =
        (sfxId as libc::c_int >> 12 as libc::c_int & 0xff as libc::c_int) as
            u8_0;
    i = 0 as libc::c_int as u8_0;
    while (i as libc::c_int) < bankId as libc::c_int {
        phi_s1 =
            (phi_s1 as libc::c_int +
                 gChannelsPerBank[gSfxChannelLayout as usize][i as usize] as
                     libc::c_int) as u8_0;
        i = i.wrapping_add(1)
    }
    i = 0 as libc::c_int as u8_0;
    while (i as libc::c_int) <
              gChannelsPerBank[gSfxChannelLayout as usize][bankId as usize] as
                  libc::c_int {
        if gActiveSounds[bankId as usize][i as usize].entryIndex as
               libc::c_int != 0xff as libc::c_int &&
               sfxId as libc::c_int ==
                   (*gSoundBanks[bankId as
                                     usize].offset(gActiveSounds[bankId as
                                                                     usize][i
                                                                                as
                                                                                usize].entryIndex
                                                       as isize)).sfxId as
                       libc::c_int {
            Audio_QueueCmdS8((0x6 as libc::c_int as u32_0 &
                                  (((0x1 as libc::c_int) << 8 as libc::c_int)
                                       - 1 as libc::c_int) as libc::c_uint) <<
                                 24 as libc::c_int |
                                 (SEQ_PLAYER_SFX as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            8 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 16 as libc::c_int |
                                 (phi_s1 as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            8 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 8 as libc::c_int |
                                 (6 as libc::c_int as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            8 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int, arg2 as s8);
        }
        phi_s1 = phi_s1.wrapping_add(1);
        i = i.wrapping_add(1)
    }
    Audio_PlaySoundGeneral(sfxId, pos, 4 as libc::c_int as u8_0,
                           &mut D_801333E0, &mut D_801333E0, &mut D_801333E8);
}
#[no_mangle]
pub unsafe extern "C" fn func_800F4E30(mut pos: *mut Vec3f, mut arg1: f32_0) {
    let mut phi_f22: f32_0 = 0.;
    let mut phi_s4: s8 = 0;
    let mut i: u8_0 = 0;
    if sSariaBgmPtr.is_null() {
        sSariaBgmPtr = pos;
        D_80130650 = arg1
    } else if pos != sSariaBgmPtr {
        if arg1 < D_80130650 { sSariaBgmPtr = pos; D_80130650 = arg1 }
    } else { D_80130650 = arg1 }
    if (*sSariaBgmPtr).x > 100.0f32 {
        phi_s4 = 0x7f as libc::c_int as s8
    } else if (*sSariaBgmPtr).x < -100.0f32 {
        phi_s4 = 0 as libc::c_int as s8
    } else {
        phi_s4 = ((*sSariaBgmPtr).x / 100.0f32 * 64.0f32 + 64.0f32) as s8
    }
    if D_80130650 > 400.0f32 {
        phi_f22 = 0.1f32
    } else if D_80130650 < 120.0f32 {
        phi_f22 = 1.0f32
    } else {
        phi_f22 =
            (1.0f32 - (D_80130650 - 120.0f32) / 280.0f32) * 0.9f32 + 0.1f32
    }
    i = 0 as libc::c_int as u8_0;
    while (i as libc::c_int) < 0x10 as libc::c_int {
        if i as libc::c_int != 9 as libc::c_int {
            Audio_QueueSeqCmd((0x60000000 as libc::c_int |
                                   (SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0
                                        as libc::c_int) << 24 as libc::c_int |
                                   (2 as libc::c_int as u8_0 as libc::c_int)
                                       << 16 as libc::c_int |
                                   (i as libc::c_int) << 8 as libc::c_int |
                                   (127.0f32 * phi_f22) as u8_0 as
                                       libc::c_int) as u32_0);
            Audio_QueueCmdS8(((0x3 as libc::c_int) << 24 as libc::c_int |
                                  (SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                                      16 as libc::c_int |
                                  (i as u32_0 as u8_0 as libc::c_int) <<
                                      8 as libc::c_int) as u32_0, phi_s4);
        }
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn Audio_ClearSariaBgm() {
    if !sSariaBgmPtr.is_null() { sSariaBgmPtr = 0 as *mut Vec3f };
}
#[no_mangle]
pub unsafe extern "C" fn Audio_ClearSariaBgmAtPos(mut pos: *mut Vec3f) {
    if sSariaBgmPtr == pos { sSariaBgmPtr = 0 as *mut Vec3f };
}
/* *
 * Turns on and off channels from both bgm players in a way that splits
 * equally between the two bgm channels. Split based on note priority
 */
#[no_mangle]
pub unsafe extern "C" fn Audio_SplitBgmChannels(mut volSplit: s8) {
    let mut volume: u8_0 = 0;
    let mut notePriority: u8_0 = 0;
    let mut channelBits: u16_0 = 0;
    let mut bgmPlayers: [u8_0; 2] =
        [SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0,
         SEQ_PLAYER_BGM_SUB as libc::c_int as u8_0];
    let mut channelIdx: u8_0 = 0;
    let mut i: u8_0 = 0;
    if func_800FA0B4(SEQ_PLAYER_FANFARE as libc::c_int as u8_0) as libc::c_int
           == 0xffff as libc::c_int &&
           func_800FA0B4(SEQ_PLAYER_BGM_SUB as libc::c_int as u8_0) as
               libc::c_int != 0x2f as libc::c_int {
        i = 0 as libc::c_int as u8_0;
        while (i as libc::c_int) <
                  (::std::mem::size_of::<[u8_0; 2]>() as
                       libc::c_ulong).wrapping_div(::std::mem::size_of::<u8_0>()
                                                       as libc::c_ulong) as
                      s32 {
            if i as libc::c_int == 0 as libc::c_int {
                // Main Bgm SeqPlayer
                volume = volSplit as u8_0
            } else {
                // Sub Bgm SeqPlayer
                volume =
                    (0x7f as libc::c_int - volSplit as libc::c_int) as u8_0
            }
            if volume as libc::c_int > 100 as libc::c_int {
                notePriority = 11 as libc::c_int as u8_0
            } else if (volume as libc::c_int) < 20 as libc::c_int {
                notePriority = 2 as libc::c_int as u8_0
            } else {
                notePriority =
                    ((volume as libc::c_int - 20 as libc::c_int) /
                         10 as libc::c_int + 2 as libc::c_int) as u8_0
            }
            channelBits = 0 as libc::c_int as u16_0;
            channelIdx = 0 as libc::c_int as u8_0;
            while (channelIdx as libc::c_int) < 16 as libc::c_int {
                if notePriority as libc::c_int >
                       (*gAudioContext.seqPlayers[bgmPlayers[i as usize] as
                                                      usize].channels[channelIdx
                                                                          as
                                                                          usize]).notePriority
                           as libc::c_int {
                    // If the note currently playing in the channel is a high enough priority,
                    // then keep the channel on by setting a channelBit
                    // If this condition fails, then the channel will be shut off
                    channelBits =
                        (channelBits as libc::c_int +
                             ((1 as libc::c_int) <<
                                  channelIdx as libc::c_int)) as u16_0
                }
                channelIdx = channelIdx.wrapping_add(1)
            }
            Audio_QueueSeqCmd(0xa0000000 as libc::c_uint |
                                  ((bgmPlayers[i as usize] as libc::c_int) <<
                                       24 as libc::c_int) as libc::c_uint |
                                  channelBits as libc::c_uint);
            i = i.wrapping_add(1)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Audio_PlaySariaBgm(mut pos: *mut Vec3f,
                                            mut seqId: u16_0,
                                            mut distMax: u16_0) {
    let mut absY: f32_0 = 0.;
    let mut dist: f32_0 = 0.;
    let mut vol: u8_0 = 0;
    let mut prevDist: f32_0 = 0.;
    if D_8016B9F3 as libc::c_int != 0 as libc::c_int {
        D_8016B9F3 = D_8016B9F3.wrapping_sub(1);
        return
    }
    dist = sqrtf((*pos).z * (*pos).z + (*pos).x * (*pos).x);
    if sSariaBgmPtr.is_null() {
        sSariaBgmPtr = pos;
        func_800F5E18(SEQ_PLAYER_BGM_SUB as libc::c_int as u8_0, seqId,
                      0 as libc::c_int as u8_0, 7 as libc::c_int as s8,
                      2 as libc::c_int as s8);
    } else {
        prevDist =
            sqrtf((*sSariaBgmPtr).z * (*sSariaBgmPtr).z +
                      (*sSariaBgmPtr).x * (*sSariaBgmPtr).x);
        if dist < prevDist { sSariaBgmPtr = pos } else { dist = prevDist }
    }
    if (*pos).y < 0.0f32 { absY = -(*pos).y } else { absY = (*pos).y }
    if distMax as libc::c_int as libc::c_float / 15.0f32 < absY {
        vol = 0 as libc::c_int as u8_0
    } else if dist < distMax as libc::c_int as libc::c_float {
        vol =
            ((1.0f32 - dist / distMax as libc::c_int as libc::c_float) *
                 127.0f32) as u8_0
    } else { vol = 0 as libc::c_int as u8_0 }
    if seqId as libc::c_int != 0x28 as libc::c_int {
        Audio_SplitBgmChannels(vol as s8);
    }
    Audio_SetVolScale(SEQ_PLAYER_BGM_SUB as libc::c_int as u8_0,
                      3 as libc::c_int as u8_0, vol,
                      0 as libc::c_int as u8_0);
    Audio_SetVolScale(SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0,
                      3 as libc::c_int as u8_0,
                      (0x7f as libc::c_int - vol as libc::c_int) as u8_0,
                      0 as libc::c_int as u8_0);
}
#[no_mangle]
pub unsafe extern "C" fn Audio_ClearSariaBgm2() {
    sSariaBgmPtr = 0 as *mut Vec3f;
}
#[no_mangle]
pub unsafe extern "C" fn func_800F5510(mut seqId: u16_0) {
    func_800F5550(seqId);
    func_800F5E18(SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0, seqId,
                  0 as libc::c_int as u8_0, 0 as libc::c_int as s8,
                  1 as libc::c_int as s8);
}
#[no_mangle]
pub unsafe extern "C" fn func_800F5550(mut seqId: u16_0) {
    let mut sp27: u8_0 = 0 as libc::c_int as u8_0;
    let mut nv: u16_0 = 0;
    if func_800FA0B4(SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0) as
           libc::c_int != 0x4c as libc::c_int {
        if func_800FA0B4(SEQ_PLAYER_BGM_SUB as libc::c_int as u8_0) as
               libc::c_int == 0x2f as libc::c_int {
            func_800F9474(SEQ_PLAYER_BGM_SUB as libc::c_int as u8_0,
                          0 as libc::c_int as u16_0);
            Audio_QueueCmdS32(0xf8000000 as libc::c_uint, 0 as libc::c_int);
        }
        if sSeqFlags[D_80130630 as usize] as libc::c_int & 0x20 as libc::c_int
               != 0 &&
               sSeqFlags[(seqId as libc::c_int & 0xff as libc::c_int &
                              0xff as libc::c_int) as usize] as libc::c_int &
                   0x10 as libc::c_int != 0 {
            if D_8013062C as libc::c_int & 0x3f as libc::c_int !=
                   0 as libc::c_int {
                sp27 = 0x1e as libc::c_int as u8_0
            }
            func_800F5E18(SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0, seqId,
                          sp27, 7 as libc::c_int as s8, D_8013062C as s8);
            D_8013062C = 0 as libc::c_int as u8_0
        } else {
            nv =
                if sSeqFlags[(seqId as libc::c_int & 0xff as libc::c_int &
                                  0xff as libc::c_int) as usize] as
                       libc::c_int & 0x40 as libc::c_int != 0 {
                    1 as libc::c_int
                } else { 0xff as libc::c_int } as u16_0;
            func_800F5E18(SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0, seqId,
                          0 as libc::c_int as u8_0, 7 as libc::c_int as s8,
                          nv as s8);
            if sSeqFlags[seqId as usize] as libc::c_int & 0x20 as libc::c_int
                   == 0 {
                D_8013062C = 0xc0 as libc::c_int as u8_0
            }
        }
        D_80130630 = (seqId as libc::c_int & 0xff as libc::c_int) as u8_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800F56A8() {
    let mut temp_v0: u16_0 = 0;
    let mut bvar: u8_0 = 0;
    temp_v0 = func_800FA0B4(SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0);
    bvar = (temp_v0 as libc::c_int & 0xff as libc::c_int) as u8_0;
    if temp_v0 as libc::c_int != 0xffff as libc::c_int &&
           sSeqFlags[bvar as usize] as libc::c_int & 0x10 as libc::c_int != 0
       {
        if D_8013062C as libc::c_int != 0xc0 as libc::c_int {
            D_8013062C =
                gAudioContext.seqPlayers[SEQ_PLAYER_BGM_MAIN as libc::c_int as
                                             usize].soundScriptIO[3 as
                                                                      libc::c_int
                                                                      as
                                                                      usize]
                    as u8_0
        } else { D_8013062C = 0 as libc::c_int as u8_0 }
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800F5718() {
    if func_800FA0B4(SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0) as
           libc::c_int != 0x4c as libc::c_int {
        Audio_QueueSeqCmd((0 as libc::c_int |
                               (SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0 as
                                    libc::c_int) << 24 as libc::c_int |
                               (0 as libc::c_int as u8_0 as libc::c_int) <<
                                   0x10 as libc::c_int |
                               0x4c as libc::c_int as u16_0 as libc::c_int) as
                              u32_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800F574C(mut arg0: f32_0, mut arg2: u8_0) {
    if arg0 == 1.0f32 {
        Audio_QueueSeqCmd(0xb0004000 as libc::c_uint |
                              ((SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0 as
                                    libc::c_int) << 24 as libc::c_int) as
                                  libc::c_uint |
                              ((arg2 as libc::c_int) << 16 as libc::c_int) as
                                  libc::c_uint |
                              0 as libc::c_int as u8_0 as libc::c_uint);
    } else {
        Audio_QueueSeqCmd(0xc0000000 as libc::c_uint |
                              ((SEQ_PLAYER_FANFARE as libc::c_int as u8_0 as
                                    libc::c_int) << 24 as libc::c_int) as
                                  libc::c_uint |
                              ((0x30 as libc::c_int as u8_0 as libc::c_int) <<
                                   16 as libc::c_int) as libc::c_uint |
                              ((arg2 as libc::c_int) << 8 as libc::c_int) as
                                  libc::c_uint |
                              (arg0 * 100.0f32) as u8_0 as libc::c_uint);
    }
    Audio_QueueSeqCmd(0xc0000000 as libc::c_uint |
                          ((SEQ_PLAYER_FANFARE as libc::c_int as u8_0 as
                                libc::c_int) << 24 as libc::c_int) as
                              libc::c_uint |
                          ((0xa0 as libc::c_int as u8_0 as libc::c_int) <<
                               16 as libc::c_int) as libc::c_uint |
                          ((arg2 as libc::c_int) << 8 as libc::c_int) as
                              libc::c_uint |
                          (arg0 * 100.0f32) as u8_0 as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn func_800F5918() {
    if func_800FA0B4(SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0) as
           libc::c_int == 0x6c as libc::c_int &&
           func_800FA11C(0 as libc::c_int as u32_0,
                         0xf0000000 as libc::c_uint) != 0 {
        Audio_QueueSeqCmd(0xb0000000 as libc::c_uint |
                              ((SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0 as
                                    libc::c_int) << 24 as libc::c_int) as
                                  libc::c_uint |
                              ((5 as libc::c_int as u8_0 as libc::c_int) <<
                                   16 as libc::c_int) as libc::c_uint |
                              ((0 as libc::c_int as u8_0 as libc::c_int) <<
                                   8 as libc::c_int) as libc::c_uint |
                              0xd2 as libc::c_int as u8_0 as libc::c_uint);
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800F595C(mut arg0: u16_0) {
    let mut arg0b: u8_0 = (arg0 as libc::c_int & 0xff as libc::c_int) as u8_0;
    if sSeqFlags[arg0b as usize] as libc::c_int & 2 as libc::c_int != 0 {
        Audio_PlayFanfare(arg0);
    } else if sSeqFlags[arg0b as usize] as libc::c_int & 4 as libc::c_int != 0
     {
        Audio_QueueSeqCmd((0 as libc::c_int |
                               (SEQ_PLAYER_FANFARE as libc::c_int as u8_0 as
                                    libc::c_int) << 24 as libc::c_int |
                               (0 as libc::c_int as u8_0 as libc::c_int) <<
                                   0x10 as libc::c_int | arg0 as libc::c_int)
                              as u32_0);
    } else {
        func_800F5E18(SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0, arg0,
                      0 as libc::c_int as u8_0, 7 as libc::c_int as s8,
                      -(1 as libc::c_int) as s8);
        Audio_QueueSeqCmd((0x100000ff as libc::c_int |
                               (SEQ_PLAYER_FANFARE as libc::c_int as u8_0 as
                                    libc::c_int) << 24 as libc::c_int |
                               (0 as libc::c_int as u8_0 as libc::c_int) <<
                                   16 as libc::c_int) as u32_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800F59E8(mut arg0: u16_0) {
    let mut arg0b: u8_0 = (arg0 as libc::c_int & 0xff as libc::c_int) as u8_0;
    if sSeqFlags[arg0b as usize] as libc::c_int & 2 as libc::c_int != 0 {
        Audio_QueueSeqCmd((0x100000ff as libc::c_int |
                               (SEQ_PLAYER_FANFARE as libc::c_int as u8_0 as
                                    libc::c_int) << 24 as libc::c_int |
                               (0 as libc::c_int as u8_0 as libc::c_int) <<
                                   16 as libc::c_int) as u32_0);
    } else if sSeqFlags[arg0b as usize] as libc::c_int & 4 as libc::c_int != 0
     {
        Audio_QueueSeqCmd((0x100000ff as libc::c_int |
                               (SEQ_PLAYER_FANFARE as libc::c_int as u8_0 as
                                    libc::c_int) << 24 as libc::c_int |
                               (0 as libc::c_int as u8_0 as libc::c_int) <<
                                   16 as libc::c_int) as u32_0);
    } else {
        Audio_QueueSeqCmd((0x100000ff as libc::c_int |
                               (SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0 as
                                    libc::c_int) << 24 as libc::c_int |
                               (0 as libc::c_int as u8_0 as libc::c_int) <<
                                   16 as libc::c_int) as u32_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800F5A58(mut arg0: u8_0) -> s32 {
    let mut phi_a1: u8_0 = 0 as libc::c_int as u8_0;
    if sSeqFlags[(arg0 as libc::c_int & 0xff as libc::c_int) as usize] as
           libc::c_int & 2 as libc::c_int != 0 {
        phi_a1 = 1 as libc::c_int as u8_0
    } else if sSeqFlags[(arg0 as libc::c_int & 0xff as libc::c_int) as usize]
                  as libc::c_int & 4 as libc::c_int != 0 {
        phi_a1 = 1 as libc::c_int as u8_0
    }
    if arg0 as libc::c_int == func_800FA0B4(phi_a1) as u8_0 as libc::c_int {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn func_800F5ACC(mut seqId: u16_0) {
    let mut temp_v0: u16_0 = 0;
    temp_v0 = func_800FA0B4(SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0);
    if temp_v0 as libc::c_int & 0xff as libc::c_int != 0x2e as libc::c_int &&
           temp_v0 as libc::c_int & 0xff as libc::c_int != 0x62 as libc::c_int
           && temp_v0 as libc::c_int != seqId as libc::c_int {
        Audio_SetSequenceMode(SEQ_MODE_IGNORE as libc::c_int as u8_0);
        if temp_v0 as libc::c_int != 0xffff as libc::c_int {
            D_80130628 = temp_v0
        } else {
            osSyncPrintf(b"Middle Boss BGM Start not stack \n\x00" as
                             *const u8 as *const libc::c_char);
        }
        Audio_QueueSeqCmd((0 as libc::c_int |
                               (SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0 as
                                    libc::c_int) << 24 as libc::c_int |
                               (0 as libc::c_int as u8_0 as libc::c_int) <<
                                   0x10 as libc::c_int | seqId as libc::c_int)
                              as u32_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800F5B58() {
    if func_800FA0B4(SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0) as
           libc::c_int != 0xffff as libc::c_int &&
           D_80130628 as libc::c_int != 0xffff as libc::c_int &&
           sSeqFlags[(func_800FA0B4(SEQ_PLAYER_BGM_MAIN as libc::c_int as
                                        u8_0) as libc::c_int &
                          0xff as libc::c_int) as usize] as libc::c_int &
               8 as libc::c_int != 0 {
        if D_80130628 as libc::c_int == 0xffff as libc::c_int {
            Audio_QueueSeqCmd((0x100000ff as libc::c_int |
                                   (SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0
                                        as libc::c_int) << 24 as libc::c_int |
                                   (0 as libc::c_int as u8_0 as libc::c_int)
                                       << 16 as libc::c_int) as u32_0);
        } else {
            Audio_QueueSeqCmd((0 as libc::c_int |
                                   (SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0
                                        as libc::c_int) << 24 as libc::c_int |
                                   (0 as libc::c_int as u8_0 as libc::c_int)
                                       << 0x10 as libc::c_int |
                                   D_80130628 as libc::c_int) as u32_0);
        }
        D_80130628 = 0xffff as libc::c_int as u16_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800F5BF0(mut arg0: u8_0) {
    let mut temp_v0: u16_0 = 0;
    temp_v0 = func_800FA0B4(SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0);
    if temp_v0 as libc::c_int != 0x1 as libc::c_int { D_80130628 = temp_v0 }
    func_800F6FB4(arg0);
}
#[no_mangle]
pub unsafe extern "C" fn func_800F5C2C() {
    if D_80130628 as libc::c_int != 0xffff as libc::c_int {
        Audio_QueueSeqCmd((0 as libc::c_int |
                               (SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0 as
                                    libc::c_int) << 24 as libc::c_int |
                               (0 as libc::c_int as u8_0 as libc::c_int) <<
                                   0x10 as libc::c_int |
                               D_80130628 as libc::c_int) as u32_0);
    }
    D_80130628 = 0xffff as libc::c_int as u16_0;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_PlayFanfare(mut seqId: u16_0) {
    let mut sp26: u16_0 = 0;
    let mut sp20: u32_0 = 0;
    let mut sp1C: *mut u8_0 = 0 as *mut u8_0;
    let mut sp18: *mut u8_0 = 0 as *mut u8_0;
    sp26 = func_800FA0B4(SEQ_PLAYER_FANFARE as libc::c_int as u8_0);
    sp1C =
        func_800E5E84(sp26 as libc::c_int & 0xff as libc::c_int, &mut sp20);
    sp18 =
        func_800E5E84(seqId as libc::c_int & 0xff as libc::c_int, &mut sp20);
    if sp26 as libc::c_int == 0xffff as libc::c_int ||
           *sp1C as libc::c_int == *sp18 as libc::c_int {
        D_8016B9F4 = 1 as libc::c_int as u8_0
    } else {
        D_8016B9F4 = 5 as libc::c_int as u8_0;
        Audio_QueueSeqCmd((0x100000ff as libc::c_int |
                               (SEQ_PLAYER_FANFARE as libc::c_int as u8_0 as
                                    libc::c_int) << 24 as libc::c_int |
                               (0 as libc::c_int as u8_0 as libc::c_int) <<
                                   16 as libc::c_int) as u32_0);
    }
    D_8016B9F6 = seqId;
}
#[no_mangle]
pub unsafe extern "C" fn func_800F5CF8() {
    let mut sp26: u16_0 = 0;
    let mut pad: u16_0 = 0;
    let mut sp22: u16_0 = 0;
    if D_8016B9F4 as libc::c_int != 0 as libc::c_int {
        D_8016B9F4 = D_8016B9F4.wrapping_sub(1);
        if D_8016B9F4 as libc::c_int == 0 as libc::c_int {
            Audio_QueueCmdS32(0xe3000000 as libc::c_uint,
                              SEQUENCE_TABLE as libc::c_int);
            Audio_QueueCmdS32(0xe3000000 as libc::c_uint,
                              FONT_TABLE as libc::c_int);
            func_800FA0B4(SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0);
            sp26 = func_800FA0B4(SEQ_PLAYER_FANFARE as libc::c_int as u8_0);
            sp22 = func_800FA0B4(SEQ_PLAYER_BGM_SUB as libc::c_int as u8_0);
            if sp26 as libc::c_int == 0xffff as libc::c_int {
                Audio_SetVolScale(SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0,
                                  1 as libc::c_int as u8_0,
                                  0 as libc::c_int as u8_0,
                                  5 as libc::c_int as u8_0);
                Audio_SetVolScale(SEQ_PLAYER_BGM_SUB as libc::c_int as u8_0,
                                  1 as libc::c_int as u8_0,
                                  0 as libc::c_int as u8_0,
                                  5 as libc::c_int as u8_0);
                Audio_QueueSeqCmd(0xc0000000 as libc::c_uint |
                                      ((SEQ_PLAYER_FANFARE as libc::c_int as
                                            u8_0 as libc::c_int) <<
                                           24 as libc::c_int) as libc::c_uint
                                      |
                                      ((0x80 as libc::c_int as u8_0 as
                                            libc::c_int) << 16 as libc::c_int)
                                          as libc::c_uint |
                                      ((1 as libc::c_int as u8_0 as
                                            libc::c_int) << 8 as libc::c_int)
                                          as libc::c_uint |
                                      0xa as libc::c_int as u8_0 as
                                          libc::c_uint);
                Audio_QueueSeqCmd(0xc0000000 as libc::c_uint |
                                      ((SEQ_PLAYER_FANFARE as libc::c_int as
                                            u8_0 as libc::c_int) <<
                                           24 as libc::c_int) as libc::c_uint
                                      |
                                      ((0x83 as libc::c_int as u8_0 as
                                            libc::c_int) << 16 as libc::c_int)
                                          as libc::c_uint |
                                      ((1 as libc::c_int as u8_0 as
                                            libc::c_int) << 8 as libc::c_int)
                                          as libc::c_uint |
                                      0xa as libc::c_int as u8_0 as
                                          libc::c_uint);
                Audio_QueueSeqCmd(0xc0000000 as libc::c_uint |
                                      ((SEQ_PLAYER_FANFARE as libc::c_int as
                                            u8_0 as libc::c_int) <<
                                           24 as libc::c_int) as libc::c_uint
                                      |
                                      ((0x90 as libc::c_int as u8_0 as
                                            libc::c_int) << 16 as libc::c_int)
                                          as libc::c_uint |
                                      ((0 as libc::c_int as u8_0 as
                                            libc::c_int) << 8 as libc::c_int)
                                          as libc::c_uint |
                                      0 as libc::c_int as u8_0 as
                                          libc::c_uint);
                if sp22 as libc::c_int != 0x2f as libc::c_int {
                    Audio_QueueSeqCmd(0xc0000000 as libc::c_uint |
                                          ((SEQ_PLAYER_FANFARE as libc::c_int
                                                as u8_0 as libc::c_int) <<
                                               24 as libc::c_int) as
                                              libc::c_uint |
                                          ((0x93 as libc::c_int as u8_0 as
                                                libc::c_int) <<
                                               16 as libc::c_int) as
                                              libc::c_uint |
                                          ((0 as libc::c_int as u8_0 as
                                                libc::c_int) <<
                                               8 as libc::c_int) as
                                              libc::c_uint |
                                          0 as libc::c_int as u8_0 as
                                              libc::c_uint);
                }
            }
            Audio_QueueSeqCmd((0 as libc::c_int |
                                   (SEQ_PLAYER_FANFARE as libc::c_int as u8_0
                                        as libc::c_int) << 24 as libc::c_int |
                                   (1 as libc::c_int as u8_0 as libc::c_int)
                                       << 0x10 as libc::c_int |
                                   D_8016B9F6 as libc::c_int) as u32_0);
            Audio_QueueSeqCmd(0xa0000000 as libc::c_uint |
                                  ((0 as libc::c_int as u8_0 as libc::c_int)
                                       << 24 as libc::c_int) as libc::c_uint |
                                  0xffff as libc::c_int as u16_0 as
                                      libc::c_uint);
            if sp22 as libc::c_int != 0x2f as libc::c_int {
                Audio_QueueSeqCmd(0xa0000000 as libc::c_uint |
                                      ((SEQ_PLAYER_BGM_SUB as libc::c_int as
                                            u8_0 as libc::c_int) <<
                                           24 as libc::c_int) as libc::c_uint
                                      |
                                      0xffff as libc::c_int as u16_0 as
                                          libc::c_uint);
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800F5E18(mut playerIdx: u8_0, mut seqId: u16_0,
                                       mut fadeTimer: u8_0, mut arg3: s8,
                                       mut arg4: s8) {
    Audio_QueueSeqCmd((0x70000000 as libc::c_int |
                           (playerIdx as libc::c_int) << 0x18 as libc::c_int |
                           (arg3 as u8_0 as libc::c_int) <<
                               0x10 as libc::c_int |
                           arg4 as u8_0 as libc::c_int) as u32_0);
    Audio_QueueSeqCmd((0 as libc::c_int |
                           (playerIdx as libc::c_int) << 24 as libc::c_int |
                           (fadeTimer as libc::c_int) << 0x10 as libc::c_int |
                           seqId as libc::c_int) as u32_0);
}
#[no_mangle]
pub unsafe extern "C" fn Audio_SetSequenceMode(mut seqMode: u8_0) {
    let mut volumeFadeInTimer: s32 = 0;
    let mut seqId: u16_0 = 0;
    let mut volumeFadeOutTimer: u8_0 = 0;
    sSeqModeInput = seqMode;
    if D_80130628 as libc::c_int == 0xffff as libc::c_int {
        if sAudioCutsceneFlag != 0 {
            seqMode = SEQ_MODE_IGNORE as libc::c_int as u8_0
        }
        seqId =
            D_8016E750[SEQ_PLAYER_BGM_MAIN as libc::c_int as usize].unk_254;
        if seqId as libc::c_int == 0x2 as libc::c_int &&
               func_800FA0B4(SEQ_PLAYER_BGM_SUB as libc::c_int as u8_0) as
                   libc::c_int == 0x1a as libc::c_int | 0x800 as libc::c_int {
            seqMode = SEQ_MODE_IGNORE as libc::c_int as u8_0
        }
        if seqId as libc::c_int == 0xffff as libc::c_int ||
               sSeqFlags[(seqId as libc::c_int & 0xff as libc::c_int) as u8_0
                             as usize] as libc::c_int & 1 as libc::c_int != 0
               ||
               sPrevSeqMode as libc::c_int & 0x7f as libc::c_int ==
                   SEQ_MODE_ENEMY as libc::c_int {
            if seqMode as libc::c_int !=
                   sPrevSeqMode as libc::c_int & 0x7f as libc::c_int {
                if seqMode as libc::c_int == SEQ_MODE_ENEMY as libc::c_int {
                    // Start playing enemy bgm
                    if (D_8016E750[SEQ_PLAYER_BGM_SUB as libc::c_int as
                                       usize].volScales[1 as libc::c_int as
                                                            usize] as
                            libc::c_int - sAudioEnemyVol as libc::c_int) <
                           0 as libc::c_int {
                        volumeFadeInTimer =
                            -(D_8016E750[SEQ_PLAYER_BGM_SUB as libc::c_int as
                                             usize].volScales[1 as libc::c_int
                                                                  as usize] as
                                  libc::c_int - sAudioEnemyVol as libc::c_int)
                    } else {
                        volumeFadeInTimer =
                            D_8016E750[SEQ_PLAYER_BGM_SUB as libc::c_int as
                                           usize].volScales[1 as libc::c_int
                                                                as usize] as
                                libc::c_int - sAudioEnemyVol as libc::c_int
                    }
                    Audio_SetVolScale(SEQ_PLAYER_BGM_SUB as libc::c_int as
                                          u8_0, 3 as libc::c_int as u8_0,
                                      sAudioEnemyVol as u8_0,
                                      volumeFadeInTimer as u8_0);
                    Audio_QueueSeqCmd((0 as libc::c_int |
                                           (SEQ_PLAYER_BGM_SUB as libc::c_int
                                                as u8_0 as libc::c_int) <<
                                               24 as libc::c_int |
                                           (10 as libc::c_int as u8_0 as
                                                libc::c_int) <<
                                               0x10 as libc::c_int |
                                           0x1a as libc::c_int as u16_0 as
                                               libc::c_int |
                                           0x800 as libc::c_int) as u32_0);
                    if seqId as libc::c_int != 0x1 as libc::c_int {
                        Audio_SetVolScale(SEQ_PLAYER_BGM_MAIN as libc::c_int
                                              as u8_0,
                                          3 as libc::c_int as u8_0,
                                          (0x7f as libc::c_int -
                                               sAudioEnemyVol as libc::c_int &
                                               0xff as libc::c_int) as u8_0,
                                          0xa as libc::c_int as u8_0);
                        Audio_SplitBgmChannels(sAudioEnemyVol);
                    }
                } else if sPrevSeqMode as libc::c_int & 0x7f as libc::c_int ==
                              SEQ_MODE_ENEMY as libc::c_int {
                    // Stop playing enemy bgm
                    Audio_QueueSeqCmd((0x100000ff as libc::c_int |
                                           (SEQ_PLAYER_BGM_SUB as libc::c_int
                                                as u8_0 as libc::c_int) <<
                                               24 as libc::c_int |
                                           (10 as libc::c_int as u8_0 as
                                                libc::c_int) <<
                                               16 as libc::c_int) as u32_0);
                    if seqMode as libc::c_int ==
                           SEQ_MODE_IGNORE as libc::c_int {
                        volumeFadeOutTimer = 0 as libc::c_int as u8_0
                    } else { volumeFadeOutTimer = 10 as libc::c_int as u8_0 }
                    Audio_SetVolScale(SEQ_PLAYER_BGM_MAIN as libc::c_int as
                                          u8_0, 3 as libc::c_int as u8_0,
                                      0x7f as libc::c_int as u8_0,
                                      volumeFadeOutTimer);
                    Audio_SplitBgmChannels(0 as libc::c_int as s8);
                }
                sPrevSeqMode =
                    (seqMode as libc::c_int + 0x80 as libc::c_int) as u8_0
            }
        } else {
            // Hyrule Field will play slightly different bgm music depending on whether player is standing
            // still or moving. This is the logic to determine the transition between those two states
            if seqMode as libc::c_int == SEQ_MODE_DEFAULT as libc::c_int {
                if sPrevSeqMode as libc::c_int ==
                       SEQ_MODE_STILL as libc::c_int {
                    sNumFramesMoving = 0 as libc::c_int as u32_0
                }
                sNumFramesStill = 0 as libc::c_int as u32_0;
                sNumFramesMoving = sNumFramesMoving.wrapping_add(1)
            } else { sNumFramesStill = sNumFramesStill.wrapping_add(1) }
            if seqMode as libc::c_int == SEQ_MODE_STILL as libc::c_int &&
                   sNumFramesStill < 30 as libc::c_int as libc::c_uint &&
                   sNumFramesMoving > 20 as libc::c_int as libc::c_uint {
                seqMode = SEQ_MODE_DEFAULT as libc::c_int as u8_0
            }
            sPrevSeqMode = seqMode;
            Audio_QueueSeqCmd((0x70000000 as libc::c_int |
                                   (SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0
                                        as libc::c_int) << 0x18 as libc::c_int
                                   |
                                   (2 as libc::c_int as u8_0 as libc::c_int)
                                       << 0x10 as libc::c_int |
                                   seqMode as libc::c_int) as u32_0);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Audio_SetBgmEnemyVolume(mut dist: f32_0) {
    let mut adjDist: f32_0 = 0.;
    if sPrevSeqMode as libc::c_int ==
           0x80 as libc::c_int | SEQ_MODE_ENEMY as libc::c_int {
        if dist != sAudioEnemyDist {
            if dist < 150.0f32 {
                adjDist = 0.0f32
            } else if dist > 500.0f32 {
                adjDist = 350.0f32
            } else { adjDist = dist - 150.0f32 }
            sAudioEnemyVol =
                ((350.0f32 - adjDist) * 127.0f32 / 350.0f32) as s8;
            Audio_SetVolScale(SEQ_PLAYER_BGM_SUB as libc::c_int as u8_0,
                              3 as libc::c_int as u8_0,
                              sAudioEnemyVol as u8_0,
                              10 as libc::c_int as u8_0);
            if D_8016E750[SEQ_PLAYER_BGM_MAIN as libc::c_int as usize].unk_254
                   as libc::c_int != 0x1 as libc::c_int {
                Audio_SetVolScale(SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0,
                                  3 as libc::c_int as u8_0,
                                  (0x7f as libc::c_int -
                                       sAudioEnemyVol as libc::c_int) as u8_0,
                                  10 as libc::c_int as u8_0);
            }
        }
        if D_8016E750[SEQ_PLAYER_BGM_MAIN as libc::c_int as usize].unk_254 as
               libc::c_int != 0x1 as libc::c_int {
            Audio_SplitBgmChannels(sAudioEnemyVol);
        }
    }
    sAudioEnemyDist = dist;
}
#[no_mangle]
pub unsafe extern "C" fn func_800F6268(mut dist: f32_0, mut arg1: u16_0) {
    let mut pad: s8 = 0;
    let mut phi_v1: s8 = 0;
    let mut temp_a0: s16 = 0;
    sAudioHasMalonBgm = 1 as libc::c_int as u8_0;
    sAudioMalonBgmDist = dist;
    if D_8016B9F2 as libc::c_int == 0 as libc::c_int {
        temp_a0 =
            (func_800FA0B4(SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0) as
                 libc::c_int & 0xff as libc::c_int) as s8 as s16;
        if temp_a0 as libc::c_int == arg1 as libc::c_int & 0xff as libc::c_int
           {
            if arg1 as libc::c_int & 0xff as libc::c_int ==
                   0x2f as libc::c_int {
                if dist > 2000.0f32 {
                    phi_v1 = 127 as libc::c_int as s8
                } else if dist < 200.0f32 {
                    phi_v1 = 0 as libc::c_int as s8
                } else {
                    phi_v1 = ((dist - 200.0f32) * 127.0f32 / 1800.0f32) as s8
                }
                // Transition volume of channels 0, 1 and 13 on seq player 0 over 3 frames
                Audio_QueueSeqCmd((0x60000000 as libc::c_int |
                                       (SEQ_PLAYER_BGM_MAIN as libc::c_int as
                                            u8_0 as libc::c_int) <<
                                           24 as libc::c_int |
                                       (3 as libc::c_int as u8_0 as
                                            libc::c_int) << 16 as libc::c_int
                                       |
                                       (0 as libc::c_int as u8_0 as
                                            libc::c_int) << 8 as libc::c_int |
                                       127 as libc::c_int as u8_0 as
                                           libc::c_int -
                                           phi_v1 as libc::c_int) as u32_0);
                Audio_QueueSeqCmd((0x60000000 as libc::c_int |
                                       (SEQ_PLAYER_BGM_MAIN as libc::c_int as
                                            u8_0 as libc::c_int) <<
                                           24 as libc::c_int |
                                       (3 as libc::c_int as u8_0 as
                                            libc::c_int) << 16 as libc::c_int
                                       |
                                       (1 as libc::c_int as u8_0 as
                                            libc::c_int) << 8 as libc::c_int |
                                       127 as libc::c_int as u8_0 as
                                           libc::c_int -
                                           phi_v1 as libc::c_int) as u32_0);
                Audio_QueueSeqCmd((0x60000000 as libc::c_int |
                                       (SEQ_PLAYER_BGM_MAIN as libc::c_int as
                                            u8_0 as libc::c_int) <<
                                           24 as libc::c_int |
                                       (3 as libc::c_int as u8_0 as
                                            libc::c_int) << 16 as libc::c_int
                                       |
                                       (13 as libc::c_int as u8_0 as
                                            libc::c_int) << 8 as libc::c_int |
                                       phi_v1 as u8_0 as libc::c_int) as
                                      u32_0);
                if D_8016B9D8 as libc::c_int == 0 as libc::c_int {
                    D_8016B9D8 = D_8016B9D8.wrapping_add(1)
                }
            }
        } else if temp_a0 as libc::c_int == 0x1 as libc::c_int &&
                      arg1 as libc::c_int & 0xff as libc::c_int ==
                          0x2f as libc::c_int {
            temp_a0 =
                (func_800FA0B4(SEQ_PLAYER_BGM_SUB as libc::c_int as u8_0) as
                     libc::c_int & 0xff as libc::c_int) as s8 as s16;
            if temp_a0 as libc::c_int !=
                   arg1 as libc::c_int & 0xff as libc::c_int &&
                   (D_8016B9D8 as libc::c_int) < 10 as libc::c_int {
                func_800F5E18(SEQ_PLAYER_BGM_SUB as libc::c_int as u8_0,
                              0x2f as libc::c_int as u16_0,
                              0 as libc::c_int as u8_0,
                              0 as libc::c_int as s8, 0 as libc::c_int as s8);
                Audio_QueueSeqCmd(0xa0000000 as libc::c_uint |
                                      ((SEQ_PLAYER_BGM_SUB as libc::c_int as
                                            u8_0 as libc::c_int) <<
                                           24 as libc::c_int) as libc::c_uint
                                      |
                                      0xfffc as libc::c_int as u16_0 as
                                          libc::c_uint);
                D_8016B9D8 = 10 as libc::c_int as u8_0
            }
            if dist > 2000.0f32 {
                phi_v1 = 127 as libc::c_int as s8
            } else if dist < 200.0f32 {
                phi_v1 = 0 as libc::c_int as s8
            } else {
                phi_v1 = ((dist - 200.0f32) * 127.0f32 / 1800.0f32) as s8
            }
            // Transition volume of channels 0 and 1 on seq player 0 over 3 frames
            Audio_QueueSeqCmd((0x60000000 as libc::c_int |
                                   (SEQ_PLAYER_BGM_SUB as libc::c_int as u8_0
                                        as libc::c_int) << 24 as libc::c_int |
                                   (3 as libc::c_int as u8_0 as libc::c_int)
                                       << 16 as libc::c_int |
                                   (0 as libc::c_int as u8_0 as libc::c_int)
                                       << 8 as libc::c_int |
                                   127 as libc::c_int as u8_0 as libc::c_int -
                                       phi_v1 as libc::c_int) as u32_0);
            Audio_QueueSeqCmd((0x60000000 as libc::c_int |
                                   (SEQ_PLAYER_BGM_SUB as libc::c_int as u8_0
                                        as libc::c_int) << 24 as libc::c_int |
                                   (3 as libc::c_int as u8_0 as libc::c_int)
                                       << 16 as libc::c_int |
                                   (1 as libc::c_int as u8_0 as libc::c_int)
                                       << 8 as libc::c_int |
                                   127 as libc::c_int as u8_0 as libc::c_int -
                                       phi_v1 as libc::c_int) as u32_0);
        }
        if (D_8016B9D8 as libc::c_int) < 10 as libc::c_int {
            D_8016B9D8 = D_8016B9D8.wrapping_add(1)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800F64E0(mut arg0: u8_0) {
    D_80130608 = arg0 as s8;
    if arg0 as libc::c_int != 0 as libc::c_int {
        Audio_PlaySoundGeneral(0x4800 as libc::c_int as u16_0,
                               &mut D_801333D4, 4 as libc::c_int as u8_0,
                               &mut D_801333E0, &mut D_801333E0,
                               &mut D_801333E8);
        Audio_QueueCmdS32(0xf1000000 as libc::c_uint, 0 as libc::c_int);
    } else {
        Audio_PlaySoundGeneral(0x4801 as libc::c_int as u16_0,
                               &mut D_801333D4, 4 as libc::c_int as u8_0,
                               &mut D_801333E0, &mut D_801333E0,
                               &mut D_801333E8);
        Audio_QueueCmdS32(0xf2000000 as libc::c_uint, 0 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800F6584(mut arg0: u8_0) {
    let mut playerIdx: u8_0 = 0;
    let mut sp34: u16_0 = 0;
    D_8016B9F2 = arg0;
    if func_800FA0B4(SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0) as
           libc::c_int & 0xff as libc::c_int == 0x2f as libc::c_int {
        playerIdx = SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0;
        sp34 = 0 as libc::c_int as u16_0
    } else if func_800FA0B4(SEQ_PLAYER_BGM_SUB as libc::c_int as u8_0) as
                  libc::c_int & 0xff as libc::c_int == 0x2f as libc::c_int {
        playerIdx = SEQ_PLAYER_BGM_SUB as libc::c_int as u8_0;
        sp34 = 0xfffc as libc::c_int as u16_0
    } else { return }
    if arg0 as libc::c_int != 0 as libc::c_int {
        Audio_QueueSeqCmd((0x60000000 as libc::c_int |
                               (playerIdx as libc::c_int) << 24 as libc::c_int
                               |
                               (1 as libc::c_int as u8_0 as libc::c_int) <<
                                   16 as libc::c_int |
                               (0 as libc::c_int as u8_0 as libc::c_int) <<
                                   8 as libc::c_int |
                               0 as libc::c_int as u8_0 as libc::c_int) as
                              u32_0);
        Audio_QueueSeqCmd((0x60000000 as libc::c_int |
                               (playerIdx as libc::c_int) << 24 as libc::c_int
                               |
                               (1 as libc::c_int as u8_0 as libc::c_int) <<
                                   16 as libc::c_int |
                               (1 as libc::c_int as u8_0 as libc::c_int) <<
                                   8 as libc::c_int |
                               0 as libc::c_int as u8_0 as libc::c_int) as
                              u32_0);
        if playerIdx as libc::c_int == SEQ_PLAYER_BGM_SUB as libc::c_int {
            Audio_QueueSeqCmd(0xa0000000 as libc::c_uint |
                                  ((playerIdx as libc::c_int) <<
                                       24 as libc::c_int) as libc::c_uint |
                                  (sp34 as libc::c_int | 3 as libc::c_int) as
                                      u16_0 as libc::c_uint);
        }
    } else {
        if playerIdx as libc::c_int == SEQ_PLAYER_BGM_SUB as libc::c_int {
            func_800F5E18(SEQ_PLAYER_BGM_SUB as libc::c_int as u8_0,
                          0x2f as libc::c_int as u16_0,
                          0 as libc::c_int as u8_0, 0 as libc::c_int as s8,
                          0 as libc::c_int as s8);
        }
        Audio_QueueSeqCmd((0x60000000 as libc::c_int |
                               (playerIdx as libc::c_int) << 24 as libc::c_int
                               |
                               (1 as libc::c_int as u8_0 as libc::c_int) <<
                                   16 as libc::c_int |
                               (0 as libc::c_int as u8_0 as libc::c_int) <<
                                   8 as libc::c_int |
                               0x7f as libc::c_int as u8_0 as libc::c_int) as
                              u32_0);
        Audio_QueueSeqCmd((0x60000000 as libc::c_int |
                               (playerIdx as libc::c_int) << 24 as libc::c_int
                               |
                               (1 as libc::c_int as u8_0 as libc::c_int) <<
                                   16 as libc::c_int |
                               (1 as libc::c_int as u8_0 as libc::c_int) <<
                                   8 as libc::c_int |
                               0x7f as libc::c_int as u8_0 as libc::c_int) as
                              u32_0);
        if playerIdx as libc::c_int == SEQ_PLAYER_BGM_SUB as libc::c_int {
            Audio_QueueSeqCmd(0xa0000000 as libc::c_uint |
                                  ((playerIdx as libc::c_int) <<
                                       24 as libc::c_int) as libc::c_uint |
                                  sp34 as libc::c_uint);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Audio_SetEnvReverb(mut reverb: s8) {
    sAudioEnvReverb = (reverb as libc::c_int & 0x7f as libc::c_int) as s8;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_SetCodeReverb(mut reverb: s8) {
    if reverb as libc::c_int != 0 as libc::c_int {
        sAudioCodeReverb = (reverb as libc::c_int & 0x7f as libc::c_int) as s8
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800F6700(mut arg0: s8) {
    let mut sp1F: s8 = 0;
    match arg0 as libc::c_int {
        0 => {
            sp1F = 0 as libc::c_int as s8;
            D_80130604 = 0 as libc::c_int as s8
        }
        1 => {
            sp1F = 3 as libc::c_int as s8;
            D_80130604 = 3 as libc::c_int as s8
        }
        2 => {
            sp1F = 1 as libc::c_int as s8;
            D_80130604 = 1 as libc::c_int as s8
        }
        3 => {
            sp1F = 0 as libc::c_int as s8;
            D_80130604 = 2 as libc::c_int as s8
        }
        _ => { }
    }
    Audio_QueueSeqCmd(0xe0000000 as libc::c_uint |
                          ((SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0 as
                                libc::c_int) << 24 as libc::c_int) as
                              libc::c_uint | sp1F as u8_0 as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn Audio_SetBaseFilter(mut filter: u8_0) {
    if sAudioBaseFilter as libc::c_int != filter as libc::c_int {
        if filter as libc::c_int == 0 as libc::c_int {
            Audio_StopSfxById(0x86b as libc::c_int as u32_0);
        } else if sAudioBaseFilter as libc::c_int == 0 as libc::c_int {
            Audio_PlaySoundGeneral(0x86b as libc::c_int as u16_0,
                                   &mut D_801333D4, 4 as libc::c_int as u8_0,
                                   &mut D_801333E0, &mut D_801333E0,
                                   &mut D_801333E8);
        }
    }
    sAudioBaseFilter = filter;
    sAudioBaseFilter2 = filter;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_SetExtraFilter(mut filter: u8_0) {
    let mut t: u32_0 = 0;
    let mut i: u8_0 = 0;
    sAudioExtraFilter2 = filter;
    sAudioExtraFilter = filter;
    if D_8016E750[SEQ_PLAYER_BGM_MAIN as libc::c_int as usize].unk_254 as
           libc::c_int == 0x1 as libc::c_int {
        i = 0 as libc::c_int as u8_0;
        while (i as libc::c_int) < 16 as libc::c_int {
            t = i as u32_0;
            // CHAN_UPD_SCRIPT_IO (seq player 0, all channels, slot 6)
            Audio_QueueCmdS8(((0x6 as libc::c_int) << 24 as libc::c_int |
                                  (SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                                      16 as libc::c_int) as libc::c_uint |
                                 (t & 0xff as libc::c_int as libc::c_uint) <<
                                     8 as libc::c_int |
                                 6 as libc::c_int as libc::c_uint,
                             filter as s8);
            i = i.wrapping_add(1)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Audio_SetCutsceneFlag(mut flag: s8) {
    sAudioCutsceneFlag = flag;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_PlaySoundGeneralIfNotInCutscene(mut sfxId:
                                                                   u16_0,
                                                               mut pos:
                                                                   *mut Vec3f,
                                                               mut arg2: u8_0,
                                                               mut freqScale:
                                                                   *mut f32_0,
                                                               mut arg4:
                                                                   *mut f32_0,
                                                               mut reverbAdd:
                                                                   *mut s8) {
    if sAudioCutsceneFlag == 0 {
        Audio_PlaySoundGeneral(sfxId, pos, arg2, freqScale, arg4, reverbAdd);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Audio_PlaySoundIfNotInCutscene(mut sfxId: u16_0) {
    Audio_PlaySoundGeneralIfNotInCutscene(sfxId, &mut D_801333D4,
                                          4 as libc::c_int as u8_0,
                                          &mut D_801333E0, &mut D_801333E0,
                                          &mut D_801333E8);
}
#[no_mangle]
pub unsafe extern "C" fn func_800F6964(mut arg0: u16_0) {
    let mut skip: s32 = 0;
    let mut i: u8_0 = 0;
    Audio_QueueSeqCmd((0x100000ff as libc::c_int |
                           (SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0 as
                                libc::c_int) << 24 as libc::c_int |
                           ((arg0 as libc::c_int * 3 as libc::c_int /
                                 2 as libc::c_int) as u8_0 as libc::c_int) <<
                               16 as libc::c_int) as u32_0);
    Audio_QueueSeqCmd((0x100000ff as libc::c_int |
                           (SEQ_PLAYER_FANFARE as libc::c_int as u8_0 as
                                libc::c_int) << 24 as libc::c_int |
                           ((arg0 as libc::c_int * 3 as libc::c_int /
                                 2 as libc::c_int) as u8_0 as libc::c_int) <<
                               16 as libc::c_int) as u32_0);
    i = 0 as libc::c_int as u8_0;
    while (i as libc::c_int) < 0x10 as libc::c_int {
        skip = 0 as libc::c_int;
        match i as libc::c_int {
            11 | 12 => {
                if gAudioSpecId as libc::c_int == 10 as libc::c_int {
                    skip = 1 as libc::c_int
                }
            }
            13 => { skip = 1 as libc::c_int }
            _ => { }
        }
        if skip == 0 {
            Audio_QueueSeqCmd((0x60000000 as libc::c_int |
                                   (SEQ_PLAYER_SFX as libc::c_int as u8_0 as
                                        libc::c_int) << 24 as libc::c_int |
                                   ((arg0 as libc::c_int >> 1 as libc::c_int)
                                        as u8_0 as libc::c_int) <<
                                       16 as libc::c_int |
                                   (i as libc::c_int) << 8 as libc::c_int |
                                   0 as libc::c_int as u8_0 as libc::c_int) as
                                  u32_0);
        }
        i = i.wrapping_add(1)
    }
    Audio_QueueSeqCmd((0x100000ff as libc::c_int |
                           (SEQ_PLAYER_BGM_SUB as libc::c_int as u8_0 as
                                libc::c_int) << 24 as libc::c_int |
                           ((arg0 as libc::c_int * 3 as libc::c_int /
                                 2 as libc::c_int) as u8_0 as libc::c_int) <<
                               16 as libc::c_int) as u32_0);
}
#[no_mangle]
pub unsafe extern "C" fn func_800F6AB0(mut arg0: u16_0) {
    Audio_QueueSeqCmd((0x100000ff as libc::c_int |
                           (SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0 as
                                libc::c_int) << 24 as libc::c_int |
                           (arg0 as u8_0 as libc::c_int) << 16 as libc::c_int)
                          as u32_0);
    Audio_QueueSeqCmd((0x100000ff as libc::c_int |
                           (SEQ_PLAYER_FANFARE as libc::c_int as u8_0 as
                                libc::c_int) << 24 as libc::c_int |
                           (arg0 as u8_0 as libc::c_int) << 16 as libc::c_int)
                          as u32_0);
    Audio_QueueSeqCmd((0x100000ff as libc::c_int |
                           (SEQ_PLAYER_BGM_SUB as libc::c_int as u8_0 as
                                libc::c_int) << 24 as libc::c_int |
                           (arg0 as u8_0 as libc::c_int) << 16 as libc::c_int)
                          as u32_0);
    Audio_SetVolScale(SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0,
                      3 as libc::c_int as u8_0, 0x7f as libc::c_int as u8_0,
                      0 as libc::c_int as u8_0);
    Audio_SetVolScale(SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0,
                      1 as libc::c_int as u8_0, 0x7f as libc::c_int as u8_0,
                      0 as libc::c_int as u8_0);
}
#[no_mangle]
pub unsafe extern "C" fn func_800F6B3C() {
    func_800F9280(SEQ_PLAYER_SFX as libc::c_int as u8_0,
                  0 as libc::c_int as u8_0, 0xff as libc::c_int as u8_0,
                  5 as libc::c_int as u16_0);
}
#[no_mangle]
pub unsafe extern "C" fn Audio_DisableAllSeq() {
    Audio_QueueCmdS32(0x83000000 as libc::c_uint |
                          ((SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0 as
                                libc::c_int) << 16 as libc::c_int) as
                              libc::c_uint, 0 as libc::c_int);
    Audio_QueueCmdS32(0x83000000 as libc::c_uint |
                          ((SEQ_PLAYER_FANFARE as libc::c_int as u8_0 as
                                libc::c_int) << 16 as libc::c_int) as
                              libc::c_uint, 0 as libc::c_int);
    Audio_QueueCmdS32(0x83000000 as libc::c_uint |
                          ((SEQ_PLAYER_SFX as libc::c_int as u8_0 as
                                libc::c_int) << 16 as libc::c_int) as
                              libc::c_uint, 0 as libc::c_int);
    Audio_QueueCmdS32(0x83000000 as libc::c_uint |
                          ((SEQ_PLAYER_BGM_SUB as libc::c_int as u8_0 as
                                libc::c_int) << 16 as libc::c_int) as
                              libc::c_uint, 0 as libc::c_int);
    Audio_ScheduleProcessCmds();
}
#[no_mangle]
pub unsafe extern "C" fn func_800F6BB8() -> s8 {
    return func_800E6680() as s8;
}
#[no_mangle]
pub unsafe extern "C" fn func_800F6BDC() {
    Audio_DisableAllSeq();
    Audio_ScheduleProcessCmds();
    loop  { if func_800F6BB8() == 0 { return } };
}
#[no_mangle]
pub unsafe extern "C" fn Audio_PreNMI() { Audio_PreNMIInternal(); }
#[no_mangle]
pub unsafe extern "C" fn func_800F6C34() {
    sPrevSeqMode = 0 as libc::c_int as u8_0;
    D_8016B7A8 = 1.0f32;
    D_8016B7B0 = 1.0f32;
    sAudioBaseFilter = 0 as libc::c_int as u8_0;
    sAudioExtraFilter = 0 as libc::c_int as u8_0;
    sAudioBaseFilter2 = 0 as libc::c_int as u8_0;
    sAudioExtraFilter2 = 0 as libc::c_int as u8_0;
    Audio_OcaSetInstrument(0 as libc::c_int as u8_0);
    sRiverFreqScaleLerp.remainingFrames = 0 as libc::c_int;
    sWaterfallFreqScaleLerp.remainingFrames = 0 as libc::c_int;
    sRiverFreqScaleLerp.value = 1.0f32;
    sWaterfallFreqScaleLerp.value = 1.0f32;
    D_8016B7D8 = 1.0f32;
    D_8016B8B0 = 0x7f as libc::c_int as u8_0;
    D_8016B8B1 = 0x7f as libc::c_int as u8_0;
    D_8016B8B2 = 0 as libc::c_int as u8_0;
    D_8016B8B3 = 0 as libc::c_int as u8_0;
    sAudioGanonDistVol = 0xff as libc::c_int as u8_0;
    D_8016B9D8 = 0 as libc::c_int as u8_0;
    sSpecReverb = sSpecReverbs[gAudioSpecId as usize];
    D_80130608 = 0 as libc::c_int as s8;
    D_80130628 = 0xffff as libc::c_int as u16_0;
    Audio_QueueCmdS8(((0x46 as libc::c_int) << 24 as libc::c_int |
                          (SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                              16 as libc::c_int) as u32_0,
                     -(1 as libc::c_int) as s8);
    sSariaBgmPtr = 0 as *mut Vec3f;
    D_8016B9F4 = 0 as libc::c_int as u8_0;
    D_8016B9F3 = 1 as libc::c_int as u8_0;
    D_8016B9F2 = 0 as libc::c_int as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn func_800F6D58(mut arg0: u8_0, mut arg1: u8_0,
                                       mut arg2: u8_0) {
    let mut t: u8_0 = 0;
    let mut temp_a0: u8_0 = 0;
    let mut i: u8_0 = 0;
    if D_8016E750[SEQ_PLAYER_BGM_MAIN as libc::c_int as usize].unk_254 as
           libc::c_int != 0x1 as libc::c_int &&
           func_800FA11C(1 as libc::c_int as u32_0,
                         0xf00000ff as libc::c_uint) != 0 {
        sAudioNatureFailed = 1 as libc::c_int as u8_0;
        return
    }
    if ((arg0 as libc::c_int) << 8 as libc::c_int) + arg1 as libc::c_int ==
           0x101 as libc::c_int {
        if func_800FA0B4(SEQ_PLAYER_BGM_SUB as libc::c_int as u8_0) as
               libc::c_int != 0x2f as libc::c_int {
            D_8016B9D8 = 0 as libc::c_int as u8_0
        }
    }
    t = (arg0 as libc::c_int >> 4 as libc::c_int) as u8_0;
    temp_a0 = (arg0 as libc::c_int & 0xf as libc::c_int) as u8_0;
    if t as libc::c_int == 0 as libc::c_int {
        t = (arg0 as libc::c_int & 0xf as libc::c_int) as u8_0
    }
    i = t;
    while i as libc::c_int <= temp_a0 as libc::c_int {
        Audio_QueueSeqCmd(0x80000000 as libc::c_uint |
                              ((SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0 as
                                    libc::c_int) << 24 as libc::c_int) as
                                  libc::c_uint |
                              ((arg1 as libc::c_int) << 16 as libc::c_int) as
                                  libc::c_uint |
                              ((i as libc::c_int) << 8 as libc::c_int) as
                                  libc::c_uint | arg2 as libc::c_uint);
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800F6E7C(mut arg0: u16_0, mut arg1: u16_0) {
    let mut i: u8_0 = 0;
    let mut t: u32_0 = 0;
    if func_800FA0B4(SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0) as
           libc::c_int == 0x4c as libc::c_int {
        func_800F3F3C(0xf as libc::c_int as u8_0);
        return
    }
    Audio_QueueSeqCmd((0x70000000 as libc::c_int |
                           (SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0 as
                                libc::c_int) << 0x18 as libc::c_int |
                           (0 as libc::c_int as u8_0 as libc::c_int) <<
                               0x10 as libc::c_int |
                           1 as libc::c_int as u8_0 as libc::c_int) as u32_0);
    Audio_QueueSeqCmd((0x70000000 as libc::c_int |
                           (SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0 as
                                libc::c_int) << 0x18 as libc::c_int |
                           (4 as libc::c_int as u8_0 as libc::c_int) <<
                               0x10 as libc::c_int |
                           (arg0 as libc::c_int >> 8 as libc::c_int) as u8_0
                               as libc::c_int) as u32_0);
    Audio_QueueSeqCmd((0x70000000 as libc::c_int |
                           (SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0 as
                                libc::c_int) << 0x18 as libc::c_int |
                           (5 as libc::c_int as u8_0 as libc::c_int) <<
                               0x10 as libc::c_int |
                           (arg0 as libc::c_int & 0xff as libc::c_int) as u8_0
                               as libc::c_int) as u32_0);
    Audio_SetVolScale(SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0,
                      0 as libc::c_int as u8_0, 0x7f as libc::c_int as u8_0,
                      1 as libc::c_int as u8_0);
    i = 0 as libc::c_int as u8_0;
    if D_80133408 as libc::c_int != 0 as libc::c_int {
        i = 1 as libc::c_int as u8_0;
        Audio_QueueSeqCmd(0xe0000100 as libc::c_uint |
                              ((SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0 as
                                    libc::c_int) << 24 as libc::c_int) as
                                  libc::c_uint |
                              0 as libc::c_int as u16_0 as libc::c_uint);
    }
    Audio_QueueSeqCmd((0 as libc::c_int |
                           (SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0 as
                                libc::c_int) << 24 as libc::c_int |
                           (0 as libc::c_int as u8_0 as libc::c_int) <<
                               0x10 as libc::c_int |
                           0x1 as libc::c_int as u16_0 as libc::c_int) as
                          u32_0);
    if i as libc::c_int != 0 as libc::c_int {
        Audio_QueueSeqCmd(0xe0000100 as libc::c_uint |
                              ((SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0 as
                                    libc::c_int) << 24 as libc::c_int) as
                                  libc::c_uint |
                              1 as libc::c_int as u16_0 as libc::c_uint);
    }
    i = 0 as libc::c_int as u8_0;
    while (i as libc::c_int) < 0x10 as libc::c_int {
        if arg1 as libc::c_int & (1 as libc::c_int) << i as libc::c_int == 0
               &&
               arg0 as libc::c_int & (1 as libc::c_int) << i as libc::c_int !=
                   0 {
            Audio_QueueSeqCmd(0x80000000 as libc::c_uint |
                                  ((SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0
                                        as libc::c_int) << 24 as libc::c_int)
                                      as libc::c_uint |
                                  ((1 as libc::c_int as u8_0 as libc::c_int)
                                       << 16 as libc::c_int) as libc::c_uint |
                                  ((i as libc::c_int) << 8 as libc::c_int) as
                                      libc::c_uint |
                                  1 as libc::c_int as u8_0 as libc::c_uint);
        }
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800F6FB4(mut arg0: u8_0) {
    let mut i: u8_0 = 0 as libc::c_int as u8_0;
    let mut b0: u8_0 = 0;
    let mut b1: u8_0 = 0;
    let mut b2: u8_0 = 0;
    if D_8016E750[SEQ_PLAYER_BGM_MAIN as libc::c_int as usize].unk_254 as
           libc::c_int == 0xffff as libc::c_int ||
           sSeqFlags[(D_8016E750[SEQ_PLAYER_BGM_MAIN as libc::c_int as
                                     usize].unk_254 as u8_0 as libc::c_int &
                          0xff as libc::c_int) as usize] as libc::c_int &
               0x80 as libc::c_int == 0 {
        func_800F6E7C(D_801306DC[arg0 as usize].unk_00,
                      D_801306DC[arg0 as usize].unk_02);
        while D_801306DC[arg0 as usize].unk_04[i as usize] as libc::c_int !=
                  0xff as libc::c_int &&
                  (i as libc::c_int) < 100 as libc::c_int {
            // Probably a fake match, using Audio_SeqCmd8 doesn't work.
            let fresh9 = i;
            i = i.wrapping_add(1);
            b0 = D_801306DC[arg0 as usize].unk_04[fresh9 as usize];
            let fresh10 = i;
            i = i.wrapping_add(1);
            b1 = D_801306DC[arg0 as usize].unk_04[fresh10 as usize];
            let fresh11 = i;
            i = i.wrapping_add(1);
            b2 = D_801306DC[arg0 as usize].unk_04[fresh11 as usize];
            Audio_QueueSeqCmd(0x80000000 as libc::c_uint |
                                  ((SEQ_PLAYER_BGM_MAIN as libc::c_int) <<
                                       24 as libc::c_int) as libc::c_uint |
                                  ((b1 as libc::c_int) << 0x10 as libc::c_int)
                                      as libc::c_uint |
                                  ((b0 as libc::c_int) << 8 as libc::c_int) as
                                      libc::c_uint | b2 as libc::c_uint);
        }
        Audio_QueueSeqCmd(0x80000000 as libc::c_uint |
                              ((SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0 as
                                    libc::c_int) << 24 as libc::c_int) as
                                  libc::c_uint |
                              ((0x7 as libc::c_int as u8_0 as libc::c_int) <<
                                   16 as libc::c_int) as libc::c_uint |
                              ((13 as libc::c_int as u8_0 as libc::c_int) <<
                                   8 as libc::c_int) as libc::c_uint |
                              D_80130604 as u8_0 as libc::c_uint);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Audio_Init() {
    AudioLoad_Init(0 as *mut libc::c_void, 0 as libc::c_int as u32_0);
}
#[no_mangle]
pub unsafe extern "C" fn Audio_InitSound() {
    func_800F6C34();
    func_800EE930();
    Audio_ResetSfxChannelState();
    func_800FAEB4();
    Audio_ResetSounds();
    func_800F9280(SEQ_PLAYER_SFX as libc::c_int as u8_0,
                  0 as libc::c_int as u8_0, 0x70 as libc::c_int as u8_0,
                  10 as libc::c_int as u16_0);
}
#[no_mangle]
pub unsafe extern "C" fn func_800F7170() {
    func_800F9280(SEQ_PLAYER_SFX as libc::c_int as u8_0,
                  0 as libc::c_int as u8_0, 0x70 as libc::c_int as u8_0,
                  1 as libc::c_int as u16_0);
    Audio_QueueCmdS32(0xf2000000 as libc::c_uint, 1 as libc::c_int);
    Audio_ScheduleProcessCmds();
    Audio_QueueCmdS32(0xf8000000 as libc::c_uint, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn func_800F71BC(mut arg0: s32) {
    D_80133418 = 1 as libc::c_int as u8_0;
    func_800F6C34();
    func_800EE930();
    Audio_ResetSfxChannelState();
    func_800FADF8();
    Audio_ResetSounds();
}
#[no_mangle]
pub unsafe extern "C" fn func_800F7208() {
    func_800FADF8();
    Audio_QueueCmdS32(0xf2000000 as libc::c_uint, 1 as libc::c_int);
    func_800F6C34();
    Audio_ResetSfxChannelState();
    func_800F9280(SEQ_PLAYER_SFX as libc::c_int as u8_0,
                  0 as libc::c_int as u8_0, 0x70 as libc::c_int as u8_0,
                  1 as libc::c_int as u16_0);
}
unsafe extern "C" fn run_static_initializers() {
    sPlaybackSong = sOcarinaSongs[0 as libc::c_int as usize].as_mut_ptr();
    gScarecrowSpawnSongPtr =
        &mut *sOcarinaSongs.as_mut_ptr().offset(OCARINA_SONG_SCARECROW as
                                                    libc::c_int as isize) as
            *mut [OcarinaNote; 20] as *mut u8_0;
    D_80131BEC =
        sOcarinaSongs[OCARINA_SONG_MEMORY_GAME as libc::c_int as
                          usize].as_mut_ptr()
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
