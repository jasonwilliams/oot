#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, register_tool)]
extern "C" {
    #[no_mangle]
    fn osSyncPrintf(fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    static mut gAudioSfxSwapMode: [u8_0; 10];
    #[no_mangle]
    fn Audio_QueueCmdS8(arg0: u32_0, arg1: s8);
    #[no_mangle]
    fn Audio_NextRandom() -> u32_0;
    #[no_mangle]
    fn AudioDebug_ScrPrt(str: *const s8, num: u16_0);
    #[no_mangle]
    fn Audio_SetSoundProperties(bankId: u8_0, entryIdx: u8_0,
                                channelIdx: u8_0);
    #[no_mangle]
    static mut gSoundBankMuted: [u8_0; 0];
    #[no_mangle]
    fn Audio_SetVolScale(playerIdx: u8_0, scaleIdx: u8_0, targetVol: u8_0,
                         volFadeTimer: u8_0);
    #[no_mangle]
    static mut gAudioSfxSwapTarget: [u16_0; 10];
    #[no_mangle]
    static mut gAudioSfxSwapSource: [u16_0; 10];
    #[no_mangle]
    static mut gSoundParams: [*mut SoundParams; 7];
    #[no_mangle]
    static mut gUsedChannelsPerBank: [[u8_0; 7]; 4];
    #[no_mangle]
    static mut gActiveSounds: [[ActiveSound; 3]; 7];
    #[no_mangle]
    static mut gChannelsPerBank: [[u8_0; 7]; 4];
    #[no_mangle]
    static mut gIsLargeSoundBank: [u8_0; 7];
    #[no_mangle]
    static mut gAudioContext: AudioContext;
    // bss
    #[no_mangle]
    static mut sSoundRequests: [SoundRequest; 256];
    #[no_mangle]
    static mut D_8016BAD0: [SoundBankEntry; 9];
    #[no_mangle]
    static mut D_8016BC80: [SoundBankEntry; 12];
    #[no_mangle]
    static mut D_8016BEC0: [SoundBankEntry; 22];
    #[no_mangle]
    static mut D_8016C2E0: [SoundBankEntry; 20];
    #[no_mangle]
    static mut D_8016C6A0: [SoundBankEntry; 8];
    #[no_mangle]
    static mut D_8016C820: [SoundBankEntry; 3];
    #[no_mangle]
    static mut D_8016C8B0: [SoundBankEntry; 5];
    #[no_mangle]
    static mut sSoundBankListEnd: [u8_0; 7];
    #[no_mangle]
    static mut sSoundBankFreeListStart: [u8_0; 7];
    #[no_mangle]
    static mut sSoundBankUnused: [u8_0; 7];
    #[no_mangle]
    static mut sCurSfxPlayerChannelIdx: u8_0;
    #[no_mangle]
    static mut sUnusedBankLerp: [UnusedBankLerp; 7];
}
pub type s8 = libc::c_schar;
pub type u8_0 = libc::c_uchar;
pub type s16 = libc::c_short;
pub type u16_0 = libc::c_ushort;
pub type s32 = libc::c_int;
pub type u32_0 = libc::c_uint;
pub type u64_0 = libc::c_ulonglong;
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
pub struct Vec3f {
    pub x: f32_0,
    pub y: f32_0,
    pub z: f32_0,
}
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
    pub bitField0: C2RustUnnamed_2,
    pub bitField1: C2RustUnnamed_1,
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
    pub sound: C2RustUnnamed_0,
    pub filter: *mut s16,
    pub pad_18: [libc::c_char; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
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
pub struct C2RustUnnamed_1 {
    #[bitfield(name = "reverbIndex", ty = "u8_0", bits = "0..=2")]
    #[bitfield(name = "bookOffset", ty = "u8_0", bits = "3..=4")]
    #[bitfield(name = "isSyntheticWave", ty = "u8_0", bits = "5..=5")]
    #[bitfield(name = "hasTwoParts", ty = "u8_0", bits = "6..=6")]
    #[bitfield(name = "usesHeadsetPanEffects2", ty = "u8_0", bits = "7..=7")]
    pub reverbIndex_bookOffset_isSyntheticWave_hasTwoParts_usesHeadsetPanEffects2: [u8; 1],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
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
    pub changes: C2RustUnnamed_4,
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
    pub u: C2RustUnnamed_3,
    pub pool: *mut NotePool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
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
pub union C2RustUnnamed_4 {
    pub s: C2RustUnnamed_5,
    pub asByte: u8_0,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
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
    pub action: C2RustUnnamed_6,
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
pub union C2RustUnnamed_6 {
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
    pub c2rust_unnamed: C2RustUnnamed_8,
    pub c2rust_unnamed_0: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
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
pub union C2RustUnnamed_8 {
    pub opArgs: u32_0,
    pub c2rust_unnamed: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
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
pub type C2RustUnnamed_10 = libc::c_uint;
pub const SFX_STATE_PLAYING_2: C2RustUnnamed_10 = 5;
pub const SFX_STATE_PLAYING_1: C2RustUnnamed_10 = 4;
pub const SFX_STATE_PLAYING_REFRESH: C2RustUnnamed_10 = 3;
pub const SFX_STATE_READY: C2RustUnnamed_10 = 2;
pub const SFX_STATE_QUEUED: C2RustUnnamed_10 = 1;
pub const SFX_STATE_EMPTY: C2RustUnnamed_10 = 0;
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
pub type C2RustUnnamed_11 = libc::c_uint;
pub const SEQ_PLAYER_BGM_SUB: C2RustUnnamed_11 = 3;
pub const SEQ_PLAYER_SFX: C2RustUnnamed_11 = 2;
pub const SEQ_PLAYER_FANFARE: C2RustUnnamed_11 = 1;
pub const SEQ_PLAYER_BGM_MAIN: C2RustUnnamed_11 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SoundRequest {
    pub sfxId: u16_0,
    pub pos: *mut Vec3f,
    pub token: u8_0,
    pub freqScale: *mut f32_0,
    pub vol: *mut f32_0,
    pub reverbAdd: *mut s8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UnusedBankLerp {
    pub value: f32_0,
    pub target: f32_0,
    pub step: f32_0,
    pub remainingFrames: u16_0,
}
// size = 0x10
// rodata for Audio_ProcessSoundRequest (this file)
// (probably moved to .data due to -use_readwrite_const)
#[no_mangle]
pub static mut D_80133340: [libc::c_char; 3] =
    unsafe {
        *::std::mem::transmute::<&[u8; 3], &mut [libc::c_char; 3]>(b"SE\x00")
    };
// rodata for Audio_ChooseActiveSounds (this file)
#[no_mangle]
pub static mut D_80133344: [libc::c_char; 71] =
    unsafe {
        *::std::mem::transmute::<&[u8; 71],
                                 &mut [libc::c_char; 71]>(b"\x1b[41;37m<INAGAKI CHECK> dist over! flag:%04X ptr:%08X pos:%f-%f-%f\x1b[m\n\x00")
    };
// file padding
#[no_mangle]
pub static mut D_8013338C: s32 = 0 as libc::c_int;
// rodata for Audio_ProcessSeqCmd (code_800F9280.c)
#[no_mangle]
pub static mut D_80133390: [libc::c_char; 6] =
    unsafe {
        *::std::mem::transmute::<&[u8; 6],
                                 &mut [libc::c_char; 6]>(b"SEQ H\x00")
    };
#[no_mangle]
pub static mut D_80133398: [libc::c_char; 6] =
    unsafe {
        *::std::mem::transmute::<&[u8; 6],
                                 &mut [libc::c_char; 6]>(b"    L\x00")
    };
// data
// sSoundRequests ring buffer endpoints. read index <= write index, wrapping around mod 256.
#[no_mangle]
pub static mut sSoundRequestWriteIndex: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut sSoundRequestReadIndex: u8_0 = 0 as libc::c_int as u8_0;
/* *
 * Array of pointers to arrays of SoundBankEntry of sizes: 9, 12, 22, 20, 8, 3, 5
 *
 * 0 : Player Bank          size 9
 * 1 : Item Bank            size 12
 * 2 : Environment Bank     size 22
 * 3 : Enemy Bank           size 20
 * 4 : System Bank          size 8
 * 5 : Ocarina Bank         size 3
 * 6 : Voice Bank           size 5
 */
#[no_mangle]
pub static mut gSoundBanks: [*mut SoundBankEntry; 7] =
    unsafe {
        [D_8016BAD0.as_ptr() as *mut _, D_8016BC80.as_ptr() as *mut _,
         D_8016BEC0.as_ptr() as *mut _, D_8016C2E0.as_ptr() as *mut _,
         D_8016C6A0.as_ptr() as *mut _, D_8016C820.as_ptr() as *mut _,
         D_8016C8B0.as_ptr() as *mut _]
    };
// Initialized in run_static_initializers
#[no_mangle]
pub static mut sBankSizes: [u8_0; 7] = [0; 7];
#[no_mangle]
pub static mut gSfxChannelLayout: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut D_801333D0: u16_0 = 0 as libc::c_int as u16_0;
#[no_mangle]
pub static mut D_801333D4: Vec3f =
    { let mut init = Vec3f{x: 0.0f32, y: 0.0f32, z: 0.0f32,}; init };
// default pos
#[no_mangle]
pub static mut D_801333E0: f32_0 = 1.0f32;
// default freqScale
#[no_mangle]
pub static mut D_801333E4: s32 = 0 as libc::c_int;
// unused
#[no_mangle]
pub static mut D_801333E8: s8 = 0 as libc::c_int as s8;
// default reverbAdd
#[no_mangle]
pub static mut D_801333EC: s32 = 0 as libc::c_int;
// unused
#[no_mangle]
pub static mut D_801333F0: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut gAudioSfxSwapOff: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub static mut D_801333F8: u8_0 = 0 as libc::c_int as u8_0;
#[no_mangle]
pub unsafe extern "C" fn Audio_SetSoundBanksMute(mut muteMask: u16_0) {
    let mut bankId: u8_0 = 0; // "ADD"
    bankId = 0 as libc::c_int as u8_0;
    while (bankId as libc::c_int) <
              (::std::mem::size_of::<[*mut SoundBankEntry; 7]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<*mut SoundBankEntry>()
                                                   as libc::c_ulong) as s32 {
        if muteMask as libc::c_int & 1 as libc::c_int != 0 {
            *gSoundBankMuted.as_mut_ptr().offset(bankId as isize) =
                1 as libc::c_int as u8_0
        } else {
            *gSoundBankMuted.as_mut_ptr().offset(bankId as isize) =
                0 as libc::c_int as u8_0
        }
        muteMask = (muteMask as libc::c_int >> 1 as libc::c_int) as u16_0;
        bankId = bankId.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn Audio_QueueSeqCmdMute(mut channelIdx: u8_0) {
    D_801333D0 =
        (D_801333D0 as libc::c_int |
             (1 as libc::c_int) << channelIdx as libc::c_int) as u16_0;
    Audio_SetVolScale(SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0,
                      2 as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
                      0xf as libc::c_int as u8_0);
    Audio_SetVolScale(SEQ_PLAYER_BGM_SUB as libc::c_int as u8_0,
                      2 as libc::c_int as u8_0, 0x40 as libc::c_int as u8_0,
                      0xf as libc::c_int as u8_0);
}
#[no_mangle]
pub unsafe extern "C" fn Audio_ClearBGMMute(mut channelIdx: u8_0) {
    D_801333D0 =
        (D_801333D0 as libc::c_int &
             ((1 as libc::c_int) << channelIdx as libc::c_int ^
                  0xffff as libc::c_int)) as u16_0;
    if D_801333D0 as libc::c_int == 0 as libc::c_int {
        Audio_SetVolScale(SEQ_PLAYER_BGM_MAIN as libc::c_int as u8_0,
                          2 as libc::c_int as u8_0,
                          0x7f as libc::c_int as u8_0,
                          0xf as libc::c_int as u8_0);
        Audio_SetVolScale(SEQ_PLAYER_BGM_SUB as libc::c_int as u8_0,
                          2 as libc::c_int as u8_0,
                          0x7f as libc::c_int as u8_0,
                          0xf as libc::c_int as u8_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Audio_PlaySoundGeneral(mut sfxId: u16_0,
                                                mut pos: *mut Vec3f,
                                                mut token: u8_0,
                                                mut freqScale: *mut f32_0,
                                                mut vol: *mut f32_0,
                                                mut reverbAdd: *mut s8) {
    let mut i: u8_0 = 0;
    let mut req: *mut SoundRequest = 0 as *mut SoundRequest;
    if *gSoundBankMuted.as_mut_ptr().offset((sfxId as libc::c_int >>
                                                 12 as libc::c_int &
                                                 0xff as libc::c_int) as
                                                isize) == 0 {
        req =
            &mut *sSoundRequests.as_mut_ptr().offset(sSoundRequestWriteIndex
                                                         as isize) as
                *mut SoundRequest;
        if gAudioSfxSwapOff == 0 {
            i = 0 as libc::c_int as u8_0;
            while (i as libc::c_int) < 10 as libc::c_int {
                if sfxId as libc::c_int ==
                       gAudioSfxSwapSource[i as usize] as libc::c_int {
                    if gAudioSfxSwapMode[i as usize] as libc::c_int ==
                           0 as libc::c_int {
                        // "SWAP"
                        sfxId = gAudioSfxSwapTarget[i as usize]
                    } else {
                        (*req).sfxId = gAudioSfxSwapTarget[i as usize];
                        (*req).pos = pos;
                        (*req).token = token;
                        (*req).freqScale = freqScale;
                        (*req).vol = vol;
                        (*req).reverbAdd = reverbAdd;
                        sSoundRequestWriteIndex =
                            sSoundRequestWriteIndex.wrapping_add(1);
                        req =
                            &mut *sSoundRequests.as_mut_ptr().offset(sSoundRequestWriteIndex
                                                                         as
                                                                         isize)
                                as *mut SoundRequest
                    }
                    i = 10 as libc::c_int as u8_0
                    // "break;"
                } // fake
                i = i.wrapping_add(1)
            }
        }
        (*req).sfxId = sfxId;
        (*req).pos = pos;
        (*req).token = token;
        (*req).freqScale = freqScale;
        (*req).vol = vol;
        (*req).reverbAdd = reverbAdd;
        sSoundRequestWriteIndex = sSoundRequestWriteIndex.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn Audio_RemoveMatchingSoundRequests(mut aspect: u8_0,
                                                           mut cmp:
                                                               *mut SoundBankEntry) {
    let mut req: *mut SoundRequest = 0 as *mut SoundRequest;
    let mut remove: s32 = 0;
    let mut i: u8_0 = sSoundRequestReadIndex;
    while i as libc::c_int != sSoundRequestWriteIndex as libc::c_int {
        remove = 0 as libc::c_int;
        req =
            &mut *sSoundRequests.as_mut_ptr().offset(i as isize) as
                *mut SoundRequest;
        match aspect as libc::c_int {
            0 => {
                if (*req).sfxId as libc::c_int & 0xf000 as libc::c_int ==
                       (*cmp).sfxId as libc::c_int & 0xf000 as libc::c_int {
                    remove = 1 as libc::c_int
                }
            }
            1 => {
                if (*req).sfxId as libc::c_int & 0xf000 as libc::c_int ==
                       (*cmp).sfxId as libc::c_int & 0xf000 as libc::c_int &&
                       &mut (*(*req).pos).x as *mut f32_0 == (*cmp).posX {
                    remove = 1 as libc::c_int
                }
            }
            2 => {
                if &mut (*(*req).pos).x as *mut f32_0 == (*cmp).posX {
                    remove = 1 as libc::c_int
                }
            }
            3 => {
                if &mut (*(*req).pos).x as *mut f32_0 == (*cmp).posX &&
                       (*req).sfxId as libc::c_int ==
                           (*cmp).sfxId as libc::c_int {
                    remove = 1 as libc::c_int
                }
            }
            4 => {
                if (*req).token as libc::c_int == (*cmp).token as libc::c_int
                       &&
                       (*req).sfxId as libc::c_int ==
                           (*cmp).sfxId as libc::c_int {
                    remove = 1 as libc::c_int
                }
            }
            5 => {
                if (*req).sfxId as libc::c_int == (*cmp).sfxId as libc::c_int
                   {
                    remove = 1 as libc::c_int
                }
            }
            _ => { }
        }
        if remove != 0 { (*req).sfxId = 0 as libc::c_int as u16_0 }
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn Audio_ProcessSoundRequest() {
    let mut sfxId: u16_0 = 0;
    let mut count: u8_0 = 0;
    let mut index: u8_0 = 0;
    let mut req: *mut SoundRequest = 0 as *mut SoundRequest;
    let mut entry: *mut SoundBankEntry = 0 as *mut SoundBankEntry;
    let mut soundParams: *mut SoundParams = 0 as *mut SoundParams;
    let mut bankId: s32 = 0;
    let mut evictImportance: u8_0 = 0;
    let mut evictIndex: u8_0 = 0;
    req =
        &mut *sSoundRequests.as_mut_ptr().offset(sSoundRequestReadIndex as
                                                     isize) as
            *mut SoundRequest;
    evictIndex = 0x80 as libc::c_int as u8_0;
    if (*req).sfxId as libc::c_int == 0 as libc::c_int { return }
    bankId =
        ((*req).sfxId as libc::c_int & 0xf000 as libc::c_int) >>
            12 as libc::c_int & 0xff as libc::c_int;
    if (1 as libc::c_int) << bankId & D_801333F0 as libc::c_int != 0 {
        AudioDebug_ScrPrt(D_80133340.as_mut_ptr() as *const s8, (*req).sfxId);
        bankId =
            ((*req).sfxId as libc::c_int & 0xf000 as libc::c_int) >>
                12 as libc::c_int & 0xff as libc::c_int
    }
    count = 0 as libc::c_int as u8_0;
    index =
        (*gSoundBanks[bankId as
                          usize].offset(0 as libc::c_int as isize)).next;
    while index as libc::c_int != 0xff as libc::c_int &&
              index as libc::c_int != 0 as libc::c_int {
        if (*gSoundBanks[bankId as usize].offset(index as isize)).posX ==
               &mut (*(*req).pos).x as *mut f32_0 {
            if (*gSoundParams[((*req).sfxId as libc::c_int >>
                                   12 as libc::c_int & 0xff as libc::c_int) as
                                  usize].offset(((*req).sfxId as libc::c_int &
                                                     0x1ff as libc::c_int) as
                                                    isize)).params as
                   libc::c_int & 0x20 as libc::c_int != 0 &&
                   (*gSoundParams[((*req).sfxId as libc::c_int >>
                                       12 as libc::c_int &
                                       0xff as libc::c_int) as
                                      usize].offset(((*req).sfxId as
                                                         libc::c_int &
                                                         0x1ff as libc::c_int)
                                                        as isize)).importance
                       as libc::c_int ==
                       (*gSoundBanks[bankId as
                                         usize].offset(index as
                                                           isize)).sfxImportance
                           as libc::c_int {
                return
            }
            if (*gSoundBanks[bankId as usize].offset(index as isize)).sfxId as
                   libc::c_int == (*req).sfxId as libc::c_int {
                count =
                    gUsedChannelsPerBank[gSfxChannelLayout as
                                             usize][bankId as usize]
            } else {
                if count as libc::c_int == 0 as libc::c_int {
                    evictIndex = index;
                    sfxId =
                        ((*gSoundBanks[bankId as
                                           usize].offset(index as
                                                             isize)).sfxId as
                             libc::c_int & 0xffff as libc::c_int) as u16_0;
                    evictImportance =
                        (*gSoundParams[(sfxId as libc::c_int >>
                                            12 as libc::c_int &
                                            0xff as libc::c_int) as
                                           usize].offset((sfxId as libc::c_int
                                                              &
                                                              0x1ff as
                                                                  libc::c_int)
                                                             as
                                                             isize)).importance
                } else if ((*gSoundBanks[bankId as
                                             usize].offset(index as
                                                               isize)).sfxImportance
                               as libc::c_int) <
                              evictImportance as libc::c_int {
                    evictIndex = index;
                    sfxId =
                        ((*gSoundBanks[bankId as
                                           usize].offset(index as
                                                             isize)).sfxId as
                             libc::c_int & 0xffff as libc::c_int) as u16_0;
                    evictImportance =
                        (*gSoundParams[(sfxId as libc::c_int >>
                                            12 as libc::c_int &
                                            0xff as libc::c_int) as
                                           usize].offset((sfxId as libc::c_int
                                                              &
                                                              0x1ff as
                                                                  libc::c_int)
                                                             as
                                                             isize)).importance
                }
                count = count.wrapping_add(1);
                if count as libc::c_int ==
                       gUsedChannelsPerBank[gSfxChannelLayout as
                                                usize][bankId as usize] as
                           libc::c_int {
                    if (*gSoundParams[((*req).sfxId as libc::c_int >>
                                           12 as libc::c_int &
                                           0xff as libc::c_int) as
                                          usize].offset(((*req).sfxId as
                                                             libc::c_int &
                                                             0x1ff as
                                                                 libc::c_int)
                                                            as
                                                            isize)).importance
                           as libc::c_int >= evictImportance as libc::c_int {
                        index = evictIndex
                    } else { index = 0 as libc::c_int as u8_0 }
                }
            }
            if count as libc::c_int ==
                   gUsedChannelsPerBank[gSfxChannelLayout as
                                            usize][bankId as usize] as
                       libc::c_int {
                soundParams =
                    &mut *(*gSoundParams.as_mut_ptr().offset(((*req).sfxId as
                                                                  libc::c_int
                                                                  >>
                                                                  12 as
                                                                      libc::c_int
                                                                  &
                                                                  0xff as
                                                                      libc::c_int)
                                                                 as
                                                                 isize)).offset(((*req).sfxId
                                                                                     as
                                                                                     libc::c_int
                                                                                     &
                                                                                     0x1ff
                                                                                         as
                                                                                         libc::c_int)
                                                                                    as
                                                                                    isize)
                        as *mut SoundParams;
                if (*req).sfxId as libc::c_int & 0xc00 as libc::c_int != 0 ||
                       (*soundParams).params as libc::c_int & 4 as libc::c_int
                           != 0 ||
                       index as libc::c_int == evictIndex as libc::c_int {
                    if (*gSoundBanks[bankId as
                                         usize].offset(index as
                                                           isize)).sfxParams
                           as libc::c_int & 8 as libc::c_int != 0 &&
                           (*gSoundBanks[bankId as
                                             usize].offset(index as
                                                               isize)).state
                               as libc::c_int !=
                               SFX_STATE_QUEUED as libc::c_int {
                        Audio_ClearBGMMute((*gSoundBanks[bankId as
                                                             usize].offset(index
                                                                               as
                                                                               isize)).channelIdx);
                    }
                    (*gSoundBanks[bankId as
                                      usize].offset(index as isize)).token =
                        (*req).token;
                    (*gSoundBanks[bankId as
                                      usize].offset(index as isize)).sfxId =
                        (*req).sfxId;
                    (*gSoundBanks[bankId as
                                      usize].offset(index as isize)).state =
                        SFX_STATE_QUEUED as libc::c_int as u8_0;
                    (*gSoundBanks[bankId as
                                      usize].offset(index as isize)).freshness
                        = 2 as libc::c_int as u8_0;
                    let ref mut fresh0 =
                        (*gSoundBanks[bankId as
                                          usize].offset(index as
                                                            isize)).freqScale;
                    *fresh0 = (*req).freqScale;
                    let ref mut fresh1 =
                        (*gSoundBanks[bankId as
                                          usize].offset(index as isize)).vol;
                    *fresh1 = (*req).vol;
                    let ref mut fresh2 =
                        (*gSoundBanks[bankId as
                                          usize].offset(index as
                                                            isize)).reverbAdd;
                    *fresh2 = (*req).reverbAdd;
                    (*gSoundBanks[bankId as
                                      usize].offset(index as isize)).sfxParams
                        = (*soundParams).params;
                    (*gSoundBanks[bankId as
                                      usize].offset(index as
                                                        isize)).sfxImportance
                        = (*soundParams).importance
                } else if (*gSoundBanks[bankId as
                                            usize].offset(index as
                                                              isize)).state as
                              libc::c_int ==
                              SFX_STATE_PLAYING_2 as libc::c_int {
                    (*gSoundBanks[bankId as
                                      usize].offset(index as isize)).state =
                        SFX_STATE_PLAYING_1 as libc::c_int as u8_0
                }
                index = 0 as libc::c_int as u8_0
            }
        }
        if index as libc::c_int != 0 as libc::c_int {
            index =
                (*gSoundBanks[bankId as usize].offset(index as isize)).next
        }
    }
    if (*gSoundBanks[bankId as
                         usize].offset(sSoundBankFreeListStart[bankId as
                                                                   usize] as
                                           isize)).next as libc::c_int !=
           0xff as libc::c_int && index as libc::c_int != 0 as libc::c_int {
        index = sSoundBankFreeListStart[bankId as usize];
        entry =
            &mut *(*gSoundBanks.as_mut_ptr().offset(bankId as
                                                        isize)).offset(index
                                                                           as
                                                                           isize)
                as *mut SoundBankEntry;
        (*entry).posX = &mut (*(*req).pos).x;
        (*entry).posY = &mut (*(*req).pos).y;
        (*entry).posZ = &mut (*(*req).pos).z;
        (*entry).token = (*req).token;
        (*entry).freqScale = (*req).freqScale;
        (*entry).vol = (*req).vol;
        (*entry).reverbAdd = (*req).reverbAdd;
        soundParams =
            &mut *(*gSoundParams.as_mut_ptr().offset(((*req).sfxId as
                                                          libc::c_int >>
                                                          12 as libc::c_int &
                                                          0xff as libc::c_int)
                                                         as
                                                         isize)).offset(((*req).sfxId
                                                                             as
                                                                             libc::c_int
                                                                             &
                                                                             0x1ff
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            isize)
                as *mut SoundParams;
        (*entry).sfxParams = (*soundParams).params;
        (*entry).sfxImportance = (*soundParams).importance;
        (*entry).sfxId = (*req).sfxId;
        (*entry).state = SFX_STATE_QUEUED as libc::c_int as u8_0;
        (*entry).freshness = 2 as libc::c_int as u8_0;
        (*entry).prev = sSoundBankListEnd[bankId as usize];
        (*gSoundBanks[bankId as
                          usize].offset(sSoundBankListEnd[bankId as usize] as
                                            isize)).next =
            sSoundBankFreeListStart[bankId as usize];
        sSoundBankListEnd[bankId as usize] =
            sSoundBankFreeListStart[bankId as usize];
        sSoundBankFreeListStart[bankId as usize] =
            (*gSoundBanks[bankId as
                              usize].offset(sSoundBankFreeListStart[bankId as
                                                                        usize]
                                                as isize)).next;
        (*gSoundBanks[bankId as
                          usize].offset(sSoundBankFreeListStart[bankId as
                                                                    usize] as
                                            isize)).prev =
            0xff as libc::c_int as u8_0;
        (*entry).next = 0xff as libc::c_int as u8_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn Audio_RemoveSoundBankEntry(mut bankId: u8_0,
                                                    mut entryIndex: u8_0) {
    let mut entry: *mut SoundBankEntry =
        &mut *(*gSoundBanks.as_mut_ptr().offset(bankId as
                                                    isize)).offset(entryIndex
                                                                       as
                                                                       isize)
            as *mut SoundBankEntry;
    let mut i: u8_0 = 0;
    if (*entry).sfxParams as libc::c_int & 8 as libc::c_int != 0 {
        Audio_ClearBGMMute((*entry).channelIdx);
    }
    if entryIndex as libc::c_int ==
           sSoundBankListEnd[bankId as usize] as libc::c_int {
        sSoundBankListEnd[bankId as usize] = (*entry).prev
    } else {
        (*gSoundBanks[bankId as usize].offset((*entry).next as isize)).prev =
            (*entry).prev
    }
    (*gSoundBanks[bankId as usize].offset((*entry).prev as isize)).next =
        (*entry).next;
    (*entry).next = sSoundBankFreeListStart[bankId as usize];
    (*entry).prev = 0xff as libc::c_int as u8_0;
    (*gSoundBanks[bankId as
                      usize].offset(sSoundBankFreeListStart[bankId as usize]
                                        as isize)).prev = entryIndex;
    sSoundBankFreeListStart[bankId as usize] = entryIndex;
    (*entry).state = SFX_STATE_EMPTY as libc::c_int as u8_0;
    i = 0 as libc::c_int as u8_0;
    while (i as libc::c_int) <
              gChannelsPerBank[gSfxChannelLayout as usize][bankId as usize] as
                  libc::c_int {
        if gActiveSounds[bankId as usize][i as usize].entryIndex as
               libc::c_int == entryIndex as libc::c_int {
            gActiveSounds[bankId as usize][i as usize].entryIndex =
                0xff as libc::c_int as u8_0;
            i = gChannelsPerBank[gSfxChannelLayout as usize][bankId as usize]
        }
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn Audio_ChooseActiveSounds(mut bankId: u8_0) {
    let mut numChosenSounds: u8_0 = 0;
    let mut numChannels: u8_0 = 0;
    let mut entryIndex: u8_0 = 0;
    let mut i: u8_0 = 0;
    let mut j: u8_0 = 0;
    let mut k: u8_0 = 0;
    let mut sfxImportance: u8_0 = 0;
    let mut needNewSound: u8_0 = 0;
    let mut chosenEntryIndex: u8_0 = 0;
    let mut temp3: u16_0 = 0;
    let mut tempf1: f32_0 = 0.;
    let mut entry: *mut SoundBankEntry = 0 as *mut SoundBankEntry;
    let mut chosenSounds: [ActiveSound; 3] =
        [ActiveSound{priority: 0, entryIndex: 0,}; 3];
    let mut activeSound: *mut ActiveSound = 0 as *mut ActiveSound;
    let mut pad: s32 = 0;
    numChosenSounds = 0 as libc::c_int as u8_0;
    i = 0 as libc::c_int as u8_0;
    while (i as libc::c_int) < 3 as libc::c_int {
        chosenSounds[i as usize].priority =
            0x7fffffff as libc::c_int as u32_0;
        chosenSounds[i as usize].entryIndex = 0xff as libc::c_int as u8_0;
        i = i.wrapping_add(1)
    }
    entryIndex =
        (*gSoundBanks[bankId as
                          usize].offset(0 as libc::c_int as isize)).next;
    k = 0 as libc::c_int as u8_0;
    while entryIndex as libc::c_int != 0xff as libc::c_int {
        if (*gSoundBanks[bankId as usize].offset(entryIndex as isize)).state
               as libc::c_int == SFX_STATE_QUEUED as libc::c_int &&
               (*gSoundBanks[bankId as
                                 usize].offset(entryIndex as isize)).sfxId as
                   libc::c_int & 0xc00 as libc::c_int != 0 {
            let ref mut fresh3 =
                (*gSoundBanks[bankId as
                                  usize].offset(entryIndex as
                                                    isize)).freshness;
            *fresh3 = (*fresh3).wrapping_sub(1)
        } else if (*gSoundBanks[bankId as
                                    usize].offset(entryIndex as isize)).sfxId
                      as libc::c_int & 0xc00 as libc::c_int == 0 &&
                      (*gSoundBanks[bankId as
                                        usize].offset(entryIndex as
                                                          isize)).state as
                          libc::c_int == SFX_STATE_PLAYING_2 as libc::c_int {
            Audio_QueueCmdS8((((*gSoundBanks[bankId as
                                                 usize].offset(entryIndex as
                                                                   isize)).channelIdx
                                   as libc::c_int) << 8 as libc::c_int |
                                  0x6020000 as libc::c_int) as u32_0,
                             0 as libc::c_int as s8);
            Audio_RemoveSoundBankEntry(bankId, entryIndex);
        }
        if (*gSoundBanks[bankId as
                             usize].offset(entryIndex as isize)).freshness as
               libc::c_int == 0 as libc::c_int {
            Audio_RemoveSoundBankEntry(bankId, entryIndex);
        } else if (*gSoundBanks[bankId as
                                    usize].offset(entryIndex as isize)).state
                      as libc::c_int != SFX_STATE_EMPTY as libc::c_int {
            entry =
                &mut *(*gSoundBanks.as_mut_ptr().offset(bankId as
                                                            isize)).offset(entryIndex
                                                                               as
                                                                               isize)
                    as *mut SoundBankEntry;
            if &mut D_801333D4.x as *mut f32_0 ==
                   (*entry.offset(0 as libc::c_int as isize)).posX {
                (*entry).dist = 0.0f32
            } else {
                tempf1 = *(*entry).posY * 1 as libc::c_int as libc::c_float;
                (*entry).dist =
                    (*(*entry).posX * *(*entry).posX + tempf1 * tempf1 +
                         *(*entry).posZ * *(*entry).posZ) *
                        1 as libc::c_int as libc::c_float
            }
            sfxImportance = (*entry).sfxImportance;
            if (*entry).sfxParams as libc::c_int & 0x10 as libc::c_int != 0 {
                (*entry).priority =
                    ((0xff as libc::c_int - sfxImportance as libc::c_int) *
                         (0xff as libc::c_int - sfxImportance as libc::c_int)
                         * (76 as libc::c_int * 76 as libc::c_int)) as u32_0
            } else {
                if (*entry).dist > 0x7fffffd0 as libc::c_int as libc::c_float
                   {
                    (*entry).dist = 0x70000008 as libc::c_int as f32_0;
                    osSyncPrintf(D_80133344.as_mut_ptr(),
                                 (*entry).sfxId as libc::c_int, (*entry).posX,
                                 (*entry).posZ,
                                 *(*entry).posX as libc::c_double,
                                 *(*entry).posY as libc::c_double,
                                 *(*entry).posZ as libc::c_double);
                }
                temp3 = (*entry).sfxId;
                (*entry).priority =
                    ((*entry).dist as
                         u32_0).wrapping_add(((0xff as libc::c_int -
                                                   sfxImportance as
                                                       libc::c_int) *
                                                  (0xff as libc::c_int -
                                                       sfxImportance as
                                                           libc::c_int) *
                                                  (76 as libc::c_int *
                                                       76 as libc::c_int)) as
                                                 libc::c_uint).wrapping_add(temp3
                                                                                as
                                                                                libc::c_uint).wrapping_sub(temp3
                                                                                                               as
                                                                                                               libc::c_uint);
                if *(*entry).posZ < 0.0f32 {
                    (*entry).priority =
                        ((*entry).priority as
                             libc::c_uint).wrapping_add((-*(*entry).posZ *
                                                             6.0f32) as s32 as
                                                            libc::c_uint) as
                            u32_0 as u32_0
                }
            }
            if (*entry).dist > 1e5f32 * 1e5f32 {
                if (*entry).state as libc::c_int ==
                       SFX_STATE_PLAYING_1 as libc::c_int {
                    Audio_QueueCmdS8((((*entry).channelIdx as libc::c_int) <<
                                          8 as libc::c_int |
                                          0x6020000 as libc::c_int) as u32_0,
                                     0 as libc::c_int as s8);
                    if (*entry).sfxId as libc::c_int & 0xc00 as libc::c_int !=
                           0 {
                        Audio_RemoveSoundBankEntry(bankId, entryIndex);
                        entryIndex = k
                    }
                }
            } else {
                numChannels =
                    gChannelsPerBank[gSfxChannelLayout as
                                         usize][bankId as usize];
                i = 0 as libc::c_int as u8_0;
                while (i as libc::c_int) < numChannels as libc::c_int {
                    if chosenSounds[i as usize].priority >= (*entry).priority
                       {
                        if (numChosenSounds as libc::c_int) <
                               gChannelsPerBank[gSfxChannelLayout as
                                                    usize][bankId as usize] as
                                   libc::c_int {
                            numChosenSounds = numChosenSounds.wrapping_add(1)
                        }
                        j =
                            (numChannels as libc::c_int - 1 as libc::c_int) as
                                u8_0;
                        while j as libc::c_int > i as libc::c_int {
                            chosenSounds[j as usize].priority =
                                chosenSounds[(j as libc::c_int -
                                                  1 as libc::c_int) as
                                                 usize].priority;
                            chosenSounds[j as usize].entryIndex =
                                chosenSounds[(j as libc::c_int -
                                                  1 as libc::c_int) as
                                                 usize].entryIndex;
                            j = j.wrapping_sub(1)
                        }
                        chosenSounds[i as usize].priority = (*entry).priority;
                        chosenSounds[i as usize].entryIndex = entryIndex;
                        i = numChannels
                        // "break;"
                    }
                    i = i.wrapping_add(1)
                }
            }
            k = entryIndex
        }
        entryIndex = (*gSoundBanks[bankId as usize].offset(k as isize)).next
    }
    i = 0 as libc::c_int as u8_0;
    while (i as libc::c_int) < numChosenSounds as libc::c_int {
        entry =
            &mut *(*gSoundBanks.as_mut_ptr().offset(bankId as
                                                        isize)).offset((*chosenSounds.as_mut_ptr().offset(i
                                                                                                              as
                                                                                                              isize)).entryIndex
                                                                           as
                                                                           isize)
                as *mut SoundBankEntry;
        if (*entry).state as libc::c_int == SFX_STATE_QUEUED as libc::c_int {
            (*entry).state = SFX_STATE_READY as libc::c_int as u8_0
        } else if (*entry).state as libc::c_int ==
                      SFX_STATE_PLAYING_1 as libc::c_int {
            (*entry).state = SFX_STATE_PLAYING_REFRESH as libc::c_int as u8_0
        }
        i = i.wrapping_add(1)
    }
    // Pick something to play for all channels.
    numChannels =
        gChannelsPerBank[gSfxChannelLayout as usize][bankId as usize];
    i = 0 as libc::c_int as u8_0;
    while (i as libc::c_int) < numChannels as libc::c_int {
        needNewSound = 0 as libc::c_int as u8_0;
        activeSound =
            &mut *(*gActiveSounds.as_mut_ptr().offset(bankId as
                                                          isize)).as_mut_ptr().offset(i
                                                                                          as
                                                                                          isize)
                as *mut ActiveSound;
        if (*activeSound).entryIndex as libc::c_int == 0xff as libc::c_int {
            needNewSound = 1 as libc::c_int as u8_0
        } else {
            entry =
                &mut *(*gSoundBanks.as_mut_ptr().offset(bankId as
                                                            isize)).offset((*activeSound.offset(0
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    isize)).entryIndex
                                                                               as
                                                                               isize)
                    as *mut SoundBankEntry;
            if (*entry).state as libc::c_int ==
                   SFX_STATE_PLAYING_1 as libc::c_int {
                if (*entry).sfxId as libc::c_int & 0xc00 as libc::c_int != 0 {
                    Audio_RemoveSoundBankEntry(bankId,
                                               (*activeSound).entryIndex);
                } else {
                    (*entry).state = SFX_STATE_QUEUED as libc::c_int as u8_0
                }
                needNewSound = 1 as libc::c_int as u8_0
            } else if (*entry).state as libc::c_int ==
                          SFX_STATE_EMPTY as libc::c_int {
                (*activeSound).entryIndex = 0xff as libc::c_int as u8_0;
                needNewSound = 1 as libc::c_int as u8_0
            } else {
                // Sound is already playing as it should, nothing to do.
                j = 0 as libc::c_int as u8_0;
                while (j as libc::c_int) < numChannels as libc::c_int {
                    if (*activeSound).entryIndex as libc::c_int ==
                           chosenSounds[j as usize].entryIndex as libc::c_int
                       {
                        chosenSounds[j as usize].entryIndex =
                            0xff as libc::c_int as u8_0;
                        j = numChannels
                    }
                    j = j.wrapping_add(1)
                }
                numChosenSounds = numChosenSounds.wrapping_sub(1)
            }
        }
        if needNewSound as libc::c_int == 1 as libc::c_int {
            j = 0 as libc::c_int as u8_0;
            while (j as libc::c_int) < numChannels as libc::c_int {
                chosenEntryIndex = chosenSounds[j as usize].entryIndex;
                if chosenEntryIndex as libc::c_int != 0xff as libc::c_int &&
                       (*gSoundBanks[bankId as
                                         usize].offset(chosenEntryIndex as
                                                           isize)).state as
                           libc::c_int !=
                           SFX_STATE_PLAYING_REFRESH as libc::c_int {
                    k = 0 as libc::c_int as u8_0;
                    while (k as libc::c_int) < numChannels as libc::c_int {
                        if chosenEntryIndex as libc::c_int ==
                               gActiveSounds[bankId as
                                                 usize][k as usize].entryIndex
                                   as libc::c_int {
                            needNewSound = 0 as libc::c_int as u8_0;
                            k = numChannels
                            // "break;"
                        }
                        k = k.wrapping_add(1)
                    }
                    if needNewSound as libc::c_int == 1 as libc::c_int {
                        (*activeSound).entryIndex = chosenEntryIndex;
                        chosenSounds[j as usize].entryIndex =
                            0xff as libc::c_int as u8_0;
                        j =
                            (numChannels as libc::c_int + 1 as libc::c_int) as
                                u8_0;
                        numChosenSounds = numChosenSounds.wrapping_sub(1)
                    }
                }
                j = j.wrapping_add(1)
            }
            if j as libc::c_int == numChannels as libc::c_int {
                // nothing found
                (*activeSound).entryIndex = 0xff as libc::c_int as u8_0
            }
        }
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn Audio_PlayActiveSounds(mut bankId: u8_0) {
    let mut entryIndex: u8_0 = 0;
    let mut channel: *mut SequenceChannel = 0 as *mut SequenceChannel;
    let mut entry: *mut SoundBankEntry = 0 as *mut SoundBankEntry;
    let mut i: u8_0 = 0;
    i = 0 as libc::c_int as u8_0;
    while (i as libc::c_int) <
              gChannelsPerBank[gSfxChannelLayout as usize][bankId as usize] as
                  libc::c_int {
        entryIndex = gActiveSounds[bankId as usize][i as usize].entryIndex;
        if entryIndex as libc::c_int != 0xff as libc::c_int {
            entry =
                &mut *(*gSoundBanks.as_mut_ptr().offset(bankId as
                                                            isize)).offset(entryIndex
                                                                               as
                                                                               isize)
                    as *mut SoundBankEntry;
            channel =
                gAudioContext.seqPlayers[SEQ_PLAYER_SFX as libc::c_int as
                                             usize].channels[sCurSfxPlayerChannelIdx
                                                                 as usize];
            if (*entry).state as libc::c_int == SFX_STATE_READY as libc::c_int
               {
                (*entry).channelIdx = sCurSfxPlayerChannelIdx;
                if (*entry).sfxParams as libc::c_int & 8 as libc::c_int != 0 {
                    Audio_QueueSeqCmdMute(sCurSfxPlayerChannelIdx);
                }
                if (*entry).sfxParams as libc::c_int & 0xc0 as libc::c_int !=
                       0 {
                    match (*entry).sfxParams as libc::c_int &
                              0xc0 as libc::c_int {
                        64 => {
                            (*entry).unk_2F =
                                (Audio_NextRandom() &
                                     0xf as libc::c_int as libc::c_uint) as
                                    u8_0
                        }
                        128 => {
                            (*entry).unk_2F =
                                (Audio_NextRandom() &
                                     0x1f as libc::c_int as libc::c_uint) as
                                    u8_0
                        }
                        192 => {
                            (*entry).unk_2F =
                                (Audio_NextRandom() &
                                     0x3f as libc::c_int as libc::c_uint) as
                                    u8_0
                        }
                        _ => { (*entry).unk_2F = 0 as libc::c_int as u8_0 }
                    }
                }
                Audio_SetSoundProperties(bankId, entryIndex,
                                         sCurSfxPlayerChannelIdx);
                Audio_QueueCmdS8(((0x6 as libc::c_int) << 24 as libc::c_int |
                                      (SEQ_PLAYER_SFX as libc::c_int) <<
                                          16 as libc::c_int |
                                      (sCurSfxPlayerChannelIdx as libc::c_int
                                           & 0xff as libc::c_int) <<
                                          8 as libc::c_int) as u32_0,
                                 1 as libc::c_int as s8);
                Audio_QueueCmdS8(((0x6 as libc::c_int) << 24 as libc::c_int |
                                      (SEQ_PLAYER_SFX as libc::c_int) <<
                                          16 as libc::c_int |
                                      (sCurSfxPlayerChannelIdx as libc::c_int
                                           & 0xff as libc::c_int) <<
                                          8 as libc::c_int | 4 as libc::c_int)
                                     as u32_0,
                                 ((*entry).sfxId as libc::c_int &
                                      0xff as libc::c_int) as s8);
                if gIsLargeSoundBank[bankId as usize] != 0 {
                    Audio_QueueCmdS8(((0x6 as libc::c_int) <<
                                          24 as libc::c_int |
                                          (SEQ_PLAYER_SFX as libc::c_int) <<
                                              16 as libc::c_int |
                                          (sCurSfxPlayerChannelIdx as
                                               libc::c_int &
                                               0xff as libc::c_int) <<
                                              8 as libc::c_int |
                                          5 as libc::c_int) as u32_0,
                                     (((*entry).sfxId as libc::c_int &
                                           0x100 as libc::c_int) >>
                                          8 as libc::c_int) as s8);
                }
                if (*entry).sfxId as libc::c_int & 0xc00 as libc::c_int != 0 {
                    (*entry).state =
                        SFX_STATE_PLAYING_1 as libc::c_int as u8_0
                } else {
                    (*entry).state =
                        SFX_STATE_PLAYING_2 as libc::c_int as u8_0
                }
            } else if (*channel).soundScriptIO[1 as libc::c_int as usize] as
                          u8_0 as libc::c_int == 0xff as libc::c_int {
                Audio_RemoveSoundBankEntry(bankId, entryIndex);
            } else if (*entry).state as libc::c_int ==
                          SFX_STATE_PLAYING_REFRESH as libc::c_int {
                Audio_SetSoundProperties(bankId, entryIndex,
                                         sCurSfxPlayerChannelIdx);
                if (*entry).sfxId as libc::c_int & 0xc00 as libc::c_int != 0 {
                    (*entry).state =
                        SFX_STATE_PLAYING_1 as libc::c_int as u8_0
                } else {
                    (*entry).state =
                        SFX_STATE_PLAYING_2 as libc::c_int as u8_0
                }
            }
        }
        sCurSfxPlayerChannelIdx = sCurSfxPlayerChannelIdx.wrapping_add(1);
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn Audio_StopSfxByBank(mut bankId: u8_0) {
    let mut entry: *mut SoundBankEntry = 0 as *mut SoundBankEntry;
    let mut pad: s32 = 0;
    let mut cmp: SoundBankEntry =
        SoundBankEntry{posX: 0 as *const f32_0 as *mut f32_0,
                       posY: 0 as *const f32_0 as *mut f32_0,
                       posZ: 0 as *const f32_0 as *mut f32_0,
                       token: 0,
                       freqScale: 0 as *const f32_0 as *mut f32_0,
                       vol: 0 as *const f32_0 as *mut f32_0,
                       reverbAdd: 0 as *const s8 as *mut s8,
                       dist: 0.,
                       priority: 0,
                       sfxImportance: 0,
                       sfxParams: 0,
                       sfxId: 0,
                       state: 0,
                       freshness: 0,
                       prev: 0,
                       next: 0,
                       channelIdx: 0,
                       unk_2F: 0,};
    let mut entryIndex: u8_0 =
        (*gSoundBanks[bankId as
                          usize].offset(0 as libc::c_int as isize)).next;
    while entryIndex as libc::c_int != 0xff as libc::c_int {
        entry =
            &mut *(*gSoundBanks.as_mut_ptr().offset(bankId as
                                                        isize)).offset(entryIndex
                                                                           as
                                                                           isize)
                as *mut SoundBankEntry;
        if (*entry).state as libc::c_int >=
               SFX_STATE_PLAYING_REFRESH as libc::c_int {
            Audio_QueueCmdS8(((0x6 as libc::c_int) << 24 as libc::c_int |
                                  (SEQ_PLAYER_SFX as libc::c_int) <<
                                      16 as libc::c_int |
                                  ((*entry).channelIdx as libc::c_int &
                                       0xff as libc::c_int) <<
                                      8 as libc::c_int) as u32_0,
                             0 as libc::c_int as s8);
        }
        if (*entry).state as libc::c_int != SFX_STATE_EMPTY as libc::c_int {
            Audio_RemoveSoundBankEntry(bankId, entryIndex);
        }
        entryIndex =
            (*gSoundBanks[bankId as
                              usize].offset(0 as libc::c_int as isize)).next
    }
    cmp.sfxId = ((bankId as libc::c_int) << 12 as libc::c_int) as u16_0;
    Audio_RemoveMatchingSoundRequests(0 as libc::c_int as u8_0, &mut cmp);
}
#[no_mangle]
pub unsafe extern "C" fn func_800F8884(mut bankId: u8_0,
                                       mut pos: *mut Vec3f) {
    let mut entry: *mut SoundBankEntry = 0 as *mut SoundBankEntry;
    let mut entryIndex: u8_0 =
        (*gSoundBanks[bankId as
                          usize].offset(0 as libc::c_int as isize)).next;
    let mut prevEntryIndex: u8_0 = 0 as libc::c_int as u8_0;
    while entryIndex as libc::c_int != 0xff as libc::c_int {
        entry =
            &mut *(*gSoundBanks.as_mut_ptr().offset(bankId as
                                                        isize)).offset(entryIndex
                                                                           as
                                                                           isize)
                as *mut SoundBankEntry;
        if (*entry).posX == &mut (*pos).x as *mut f32_0 {
            if (*entry).state as libc::c_int >=
                   SFX_STATE_PLAYING_REFRESH as libc::c_int {
                Audio_QueueCmdS8(((0x6 as libc::c_int) << 24 as libc::c_int |
                                      (SEQ_PLAYER_SFX as libc::c_int) <<
                                          16 as libc::c_int |
                                      ((*entry).channelIdx as libc::c_int &
                                           0xff as libc::c_int) <<
                                          8 as libc::c_int) as u32_0,
                                 0 as libc::c_int as s8);
            }
            if (*entry).state as libc::c_int != SFX_STATE_EMPTY as libc::c_int
               {
                Audio_RemoveSoundBankEntry(bankId, entryIndex);
            }
        } else { prevEntryIndex = entryIndex }
        entryIndex =
            (*gSoundBanks[bankId as
                              usize].offset(prevEntryIndex as isize)).next
    };
}
#[no_mangle]
pub unsafe extern "C" fn Audio_StopSfxByPosAndBank(mut bankId: u8_0,
                                                   mut pos: *mut Vec3f) {
    let mut cmp: SoundBankEntry =
        SoundBankEntry{posX: 0 as *const f32_0 as *mut f32_0,
                       posY: 0 as *const f32_0 as *mut f32_0,
                       posZ: 0 as *const f32_0 as *mut f32_0,
                       token: 0,
                       freqScale: 0 as *const f32_0 as *mut f32_0,
                       vol: 0 as *const f32_0 as *mut f32_0,
                       reverbAdd: 0 as *const s8 as *mut s8,
                       dist: 0.,
                       priority: 0,
                       sfxImportance: 0,
                       sfxParams: 0,
                       sfxId: 0,
                       state: 0,
                       freshness: 0,
                       prev: 0,
                       next: 0,
                       channelIdx: 0,
                       unk_2F: 0,};
    func_800F8884(bankId, pos);
    cmp.sfxId = ((bankId as libc::c_int) << 12 as libc::c_int) as u16_0;
    cmp.posX = &mut (*pos).x;
    Audio_RemoveMatchingSoundRequests(1 as libc::c_int as u8_0, &mut cmp);
}
#[no_mangle]
pub unsafe extern "C" fn Audio_StopSfxByPos(mut pos: *mut Vec3f) {
    let mut i: u8_0 = 0;
    let mut cmp: SoundBankEntry =
        SoundBankEntry{posX: 0 as *const f32_0 as *mut f32_0,
                       posY: 0 as *const f32_0 as *mut f32_0,
                       posZ: 0 as *const f32_0 as *mut f32_0,
                       token: 0,
                       freqScale: 0 as *const f32_0 as *mut f32_0,
                       vol: 0 as *const f32_0 as *mut f32_0,
                       reverbAdd: 0 as *const s8 as *mut s8,
                       dist: 0.,
                       priority: 0,
                       sfxImportance: 0,
                       sfxParams: 0,
                       sfxId: 0,
                       state: 0,
                       freshness: 0,
                       prev: 0,
                       next: 0,
                       channelIdx: 0,
                       unk_2F: 0,};
    i = 0 as libc::c_int as u8_0;
    while (i as libc::c_int) <
              (::std::mem::size_of::<[*mut SoundBankEntry; 7]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<*mut SoundBankEntry>()
                                                   as libc::c_ulong) as s32 {
        func_800F8884(i, pos);
        i = i.wrapping_add(1)
    }
    cmp.posX = &mut (*pos).x;
    Audio_RemoveMatchingSoundRequests(2 as libc::c_int as u8_0, &mut cmp);
}
#[no_mangle]
pub unsafe extern "C" fn Audio_StopSfxByPosAndId(mut pos: *mut Vec3f,
                                                 mut sfxId: u16_0) {
    let mut entry: *mut SoundBankEntry = 0 as *mut SoundBankEntry;
    let mut entryIndex: u8_0 =
        (*gSoundBanks[((sfxId as libc::c_int & 0xf000 as libc::c_int) >>
                           12 as libc::c_int & 0xff as libc::c_int) as
                          usize].offset(0 as libc::c_int as isize)).next;
    let mut prevEntryIndex: u8_0 = 0 as libc::c_int as u8_0;
    let mut cmp: SoundBankEntry =
        SoundBankEntry{posX: 0 as *const f32_0 as *mut f32_0,
                       posY: 0 as *const f32_0 as *mut f32_0,
                       posZ: 0 as *const f32_0 as *mut f32_0,
                       token: 0,
                       freqScale: 0 as *const f32_0 as *mut f32_0,
                       vol: 0 as *const f32_0 as *mut f32_0,
                       reverbAdd: 0 as *const s8 as *mut s8,
                       dist: 0.,
                       priority: 0,
                       sfxImportance: 0,
                       sfxParams: 0,
                       sfxId: 0,
                       state: 0,
                       freshness: 0,
                       prev: 0,
                       next: 0,
                       channelIdx: 0,
                       unk_2F: 0,};
    while entryIndex as libc::c_int != 0xff as libc::c_int {
        entry =
            &mut *(*gSoundBanks.as_mut_ptr().offset(((sfxId as libc::c_int &
                                                          0xf000 as
                                                              libc::c_int) >>
                                                         12 as libc::c_int &
                                                         0xff as libc::c_int)
                                                        as
                                                        isize)).offset(entryIndex
                                                                           as
                                                                           isize)
                as *mut SoundBankEntry;
        if (*entry).posX == &mut (*pos).x as *mut f32_0 &&
               (*entry).sfxId as libc::c_int == sfxId as libc::c_int {
            if (*entry).state as libc::c_int >=
                   SFX_STATE_PLAYING_REFRESH as libc::c_int {
                Audio_QueueCmdS8(((0x6 as libc::c_int) << 24 as libc::c_int |
                                      (SEQ_PLAYER_SFX as libc::c_int) <<
                                          16 as libc::c_int |
                                      ((*entry).channelIdx as libc::c_int &
                                           0xff as libc::c_int) <<
                                          8 as libc::c_int) as u32_0,
                                 0 as libc::c_int as s8);
            }
            if (*entry).state as libc::c_int != SFX_STATE_EMPTY as libc::c_int
               {
                Audio_RemoveSoundBankEntry(((sfxId as libc::c_int &
                                                 0xf000 as libc::c_int) >>
                                                12 as libc::c_int &
                                                0xff as libc::c_int) as u8_0,
                                           entryIndex);
            }
            entryIndex = 0xff as libc::c_int as u8_0
        } else { prevEntryIndex = entryIndex }
        if entryIndex as libc::c_int != 0xff as libc::c_int {
            entryIndex =
                (*gSoundBanks[((sfxId as libc::c_int & 0xf000 as libc::c_int)
                                   >> 12 as libc::c_int & 0xff as libc::c_int)
                                  as
                                  usize].offset(prevEntryIndex as isize)).next
        }
    }
    cmp.posX = &mut (*pos).x;
    cmp.sfxId = sfxId;
    Audio_RemoveMatchingSoundRequests(3 as libc::c_int as u8_0, &mut cmp);
}
#[no_mangle]
pub unsafe extern "C" fn Audio_StopSfxByTokenAndId(mut token: u8_0,
                                                   mut sfxId: u16_0) {
    let mut entry: *mut SoundBankEntry = 0 as *mut SoundBankEntry;
    let mut entryIndex: u8_0 =
        (*gSoundBanks[((sfxId as libc::c_int & 0xf000 as libc::c_int) >>
                           12 as libc::c_int & 0xff as libc::c_int) as
                          usize].offset(0 as libc::c_int as isize)).next;
    let mut prevEntryIndex: u8_0 = 0 as libc::c_int as u8_0;
    let mut cmp: SoundBankEntry =
        SoundBankEntry{posX: 0 as *const f32_0 as *mut f32_0,
                       posY: 0 as *const f32_0 as *mut f32_0,
                       posZ: 0 as *const f32_0 as *mut f32_0,
                       token: 0,
                       freqScale: 0 as *const f32_0 as *mut f32_0,
                       vol: 0 as *const f32_0 as *mut f32_0,
                       reverbAdd: 0 as *const s8 as *mut s8,
                       dist: 0.,
                       priority: 0,
                       sfxImportance: 0,
                       sfxParams: 0,
                       sfxId: 0,
                       state: 0,
                       freshness: 0,
                       prev: 0,
                       next: 0,
                       channelIdx: 0,
                       unk_2F: 0,};
    while entryIndex as libc::c_int != 0xff as libc::c_int {
        entry =
            &mut *(*gSoundBanks.as_mut_ptr().offset(((sfxId as libc::c_int &
                                                          0xf000 as
                                                              libc::c_int) >>
                                                         12 as libc::c_int &
                                                         0xff as libc::c_int)
                                                        as
                                                        isize)).offset(entryIndex
                                                                           as
                                                                           isize)
                as *mut SoundBankEntry;
        if (*entry).token as libc::c_int == token as libc::c_int &&
               (*entry).sfxId as libc::c_int == sfxId as libc::c_int {
            if (*entry).state as libc::c_int >=
                   SFX_STATE_PLAYING_REFRESH as libc::c_int {
                Audio_QueueCmdS8(((0x6 as libc::c_int) << 24 as libc::c_int |
                                      (SEQ_PLAYER_SFX as libc::c_int) <<
                                          16 as libc::c_int |
                                      ((*entry).channelIdx as libc::c_int &
                                           0xff as libc::c_int) <<
                                          8 as libc::c_int) as u32_0,
                                 0 as libc::c_int as s8);
            }
            if (*entry).state as libc::c_int != SFX_STATE_EMPTY as libc::c_int
               {
                Audio_RemoveSoundBankEntry(((sfxId as libc::c_int &
                                                 0xf000 as libc::c_int) >>
                                                12 as libc::c_int &
                                                0xff as libc::c_int) as u8_0,
                                           entryIndex);
            }
        } else { prevEntryIndex = entryIndex }
        if entryIndex as libc::c_int != 0xff as libc::c_int {
            entryIndex =
                (*gSoundBanks[((sfxId as libc::c_int & 0xf000 as libc::c_int)
                                   >> 12 as libc::c_int & 0xff as libc::c_int)
                                  as
                                  usize].offset(prevEntryIndex as isize)).next
        }
    }
    cmp.token = token;
    cmp.sfxId = sfxId;
    Audio_RemoveMatchingSoundRequests(4 as libc::c_int as u8_0, &mut cmp);
}
#[no_mangle]
pub unsafe extern "C" fn Audio_StopSfxById(mut sfxId: u32_0) {
    let mut entry: *mut SoundBankEntry = 0 as *mut SoundBankEntry;
    let mut entryIndex: u8_0 =
        (*gSoundBanks[((sfxId & 0xf000 as libc::c_int as libc::c_uint) >>
                           12 as libc::c_int &
                           0xff as libc::c_int as libc::c_uint) as
                          usize].offset(0 as libc::c_int as isize)).next;
    let mut prevEntryIndex: u8_0 = 0 as libc::c_int as u8_0;
    let mut cmp: SoundBankEntry =
        SoundBankEntry{posX: 0 as *const f32_0 as *mut f32_0,
                       posY: 0 as *const f32_0 as *mut f32_0,
                       posZ: 0 as *const f32_0 as *mut f32_0,
                       token: 0,
                       freqScale: 0 as *const f32_0 as *mut f32_0,
                       vol: 0 as *const f32_0 as *mut f32_0,
                       reverbAdd: 0 as *const s8 as *mut s8,
                       dist: 0.,
                       priority: 0,
                       sfxImportance: 0,
                       sfxParams: 0,
                       sfxId: 0,
                       state: 0,
                       freshness: 0,
                       prev: 0,
                       next: 0,
                       channelIdx: 0,
                       unk_2F: 0,};
    while entryIndex as libc::c_int != 0xff as libc::c_int {
        entry =
            &mut *(*gSoundBanks.as_mut_ptr().offset(((sfxId &
                                                          0xf000 as
                                                              libc::c_int as
                                                              libc::c_uint) >>
                                                         12 as libc::c_int &
                                                         0xff as libc::c_int
                                                             as libc::c_uint)
                                                        as
                                                        isize)).offset(entryIndex
                                                                           as
                                                                           isize)
                as *mut SoundBankEntry;
        if (*entry).sfxId as libc::c_uint == sfxId {
            if (*entry).state as libc::c_int >=
                   SFX_STATE_PLAYING_REFRESH as libc::c_int {
                Audio_QueueCmdS8(((0x6 as libc::c_int) << 24 as libc::c_int |
                                      (SEQ_PLAYER_SFX as libc::c_int) <<
                                          16 as libc::c_int |
                                      ((*entry).channelIdx as libc::c_int &
                                           0xff as libc::c_int) <<
                                          8 as libc::c_int) as u32_0,
                                 0 as libc::c_int as s8);
            }
            if (*entry).state as libc::c_int != SFX_STATE_EMPTY as libc::c_int
               {
                Audio_RemoveSoundBankEntry(((sfxId &
                                                 0xf000 as libc::c_int as
                                                     libc::c_uint) >>
                                                12 as libc::c_int &
                                                0xff as libc::c_int as
                                                    libc::c_uint) as u8_0,
                                           entryIndex);
            }
        } else { prevEntryIndex = entryIndex }
        entryIndex =
            (*gSoundBanks[((sfxId & 0xf000 as libc::c_int as libc::c_uint) >>
                               12 as libc::c_int &
                               0xff as libc::c_int as libc::c_uint) as
                              usize].offset(prevEntryIndex as isize)).next
    }
    cmp.sfxId = sfxId as u16_0;
    Audio_RemoveMatchingSoundRequests(5 as libc::c_int as u8_0, &mut cmp);
}
#[no_mangle]
pub unsafe extern "C" fn Audio_ProcessSoundRequests() {
    while sSoundRequestWriteIndex as libc::c_int !=
              sSoundRequestReadIndex as libc::c_int {
        Audio_ProcessSoundRequest();
        sSoundRequestReadIndex = sSoundRequestReadIndex.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn Audio_SetUnusedBankLerp(mut bankId: u8_0,
                                                 mut target: u8_0,
                                                 mut delay: u16_0) {
    if delay as libc::c_int == 0 as libc::c_int {
        delay = delay.wrapping_add(1)
    }
    sUnusedBankLerp[bankId as usize].target =
        target as libc::c_int as libc::c_float / 127.0f32;
    sUnusedBankLerp[bankId as usize].remainingFrames = delay;
    sUnusedBankLerp[bankId as usize].step =
        (sUnusedBankLerp[bankId as usize].value -
             sUnusedBankLerp[bankId as usize].target) /
            delay as libc::c_int as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_StepUnusedBankLerp(mut bankId: u8_0) {
    if sUnusedBankLerp[bankId as usize].remainingFrames as libc::c_int !=
           0 as libc::c_int {
        sUnusedBankLerp[bankId as usize].remainingFrames =
            sUnusedBankLerp[bankId as usize].remainingFrames.wrapping_sub(1);
        if sUnusedBankLerp[bankId as usize].remainingFrames as libc::c_int !=
               0 as libc::c_int {
            sUnusedBankLerp[bankId as usize].value -=
                sUnusedBankLerp[bankId as usize].step
        } else {
            sUnusedBankLerp[bankId as usize].value =
                sUnusedBankLerp[bankId as usize].target
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800F8F88() {
    let mut bankId: u8_0 = 0;
    if gAudioContext.seqPlayers[SEQ_PLAYER_SFX as libc::c_int as
                                    usize].channels[0 as libc::c_int as usize]
           as u32_0 !=
           &mut gAudioContext.sequenceChannelNone as *mut SequenceChannel as
               u32_0 {
        sCurSfxPlayerChannelIdx = 0 as libc::c_int as u8_0;
        bankId = 0 as libc::c_int as u8_0;
        while (bankId as libc::c_int) <
                  (::std::mem::size_of::<[*mut SoundBankEntry; 7]>() as
                       libc::c_ulong).wrapping_div(::std::mem::size_of::<*mut SoundBankEntry>()
                                                       as libc::c_ulong) as
                      s32 {
            Audio_ChooseActiveSounds(bankId);
            Audio_PlayActiveSounds(bankId);
            Audio_StepUnusedBankLerp(bankId);
            bankId = bankId.wrapping_add(1)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Audio_IsSfxPlaying(mut sfxId: u32_0) -> u8_0 {
    let mut entry: *mut SoundBankEntry = 0 as *mut SoundBankEntry;
    let mut entryIndex: u8_0 =
        (*gSoundBanks[((sfxId & 0xf000 as libc::c_int as libc::c_uint) >>
                           12 as libc::c_int &
                           0xff as libc::c_int as libc::c_uint) as
                          usize].offset(0 as libc::c_int as isize)).next;
    while entryIndex as libc::c_int != 0xff as libc::c_int {
        entry =
            &mut *(*gSoundBanks.as_mut_ptr().offset(((sfxId &
                                                          0xf000 as
                                                              libc::c_int as
                                                              libc::c_uint) >>
                                                         12 as libc::c_int &
                                                         0xff as libc::c_int
                                                             as libc::c_uint)
                                                        as
                                                        isize)).offset(entryIndex
                                                                           as
                                                                           isize)
                as *mut SoundBankEntry;
        if (*entry).sfxId as libc::c_uint == sfxId {
            return 1 as libc::c_int as u8_0
        }
        entryIndex = (*entry).next
    }
    return 0 as libc::c_int as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_ResetSounds() {
    let mut bankId: u8_0 = 0;
    let mut i: u8_0 = 0;
    let mut entryIndex: u8_0 = 0;
    sSoundRequestWriteIndex = 0 as libc::c_int as u8_0;
    sSoundRequestReadIndex = 0 as libc::c_int as u8_0;
    D_801333D0 = 0 as libc::c_int as u16_0;
    bankId = 0 as libc::c_int as u8_0;
    while (bankId as libc::c_int) <
              (::std::mem::size_of::<[*mut SoundBankEntry; 7]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<*mut SoundBankEntry>()
                                                   as libc::c_ulong) as s32 {
        sSoundBankListEnd[bankId as usize] = 0 as libc::c_int as u8_0;
        sSoundBankFreeListStart[bankId as usize] = 1 as libc::c_int as u8_0;
        sSoundBankUnused[bankId as usize] = 0 as libc::c_int as u8_0;
        *gSoundBankMuted.as_mut_ptr().offset(bankId as isize) =
            0 as libc::c_int as u8_0;
        sUnusedBankLerp[bankId as usize].value = 1.0f32;
        sUnusedBankLerp[bankId as usize].remainingFrames =
            0 as libc::c_int as u16_0;
        bankId = bankId.wrapping_add(1)
    }
    bankId = 0 as libc::c_int as u8_0;
    while (bankId as libc::c_int) <
              (::std::mem::size_of::<[*mut SoundBankEntry; 7]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<*mut SoundBankEntry>()
                                                   as libc::c_ulong) as s32 {
        i = 0 as libc::c_int as u8_0;
        while (i as libc::c_int) < 3 as libc::c_int {
            gActiveSounds[bankId as usize][i as usize].entryIndex =
                0xff as libc::c_int as u8_0;
            i = i.wrapping_add(1)
        }
        bankId = bankId.wrapping_add(1)
    }
    bankId = 0 as libc::c_int as u8_0;
    while (bankId as libc::c_int) <
              (::std::mem::size_of::<[*mut SoundBankEntry; 7]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<*mut SoundBankEntry>()
                                                   as libc::c_ulong) as s32 {
        (*gSoundBanks[bankId as usize].offset(0 as libc::c_int as isize)).prev
            = 0xff as libc::c_int as u8_0;
        (*gSoundBanks[bankId as usize].offset(0 as libc::c_int as isize)).next
            = 0xff as libc::c_int as u8_0;
        i = 1 as libc::c_int as u8_0;
        while (i as libc::c_int) <
                  sBankSizes[bankId as usize] as libc::c_int -
                      1 as libc::c_int {
            (*gSoundBanks[bankId as usize].offset(i as isize)).prev =
                (i as libc::c_int - 1 as libc::c_int) as u8_0;
            (*gSoundBanks[bankId as usize].offset(i as isize)).next =
                (i as libc::c_int + 1 as libc::c_int) as u8_0;
            i = i.wrapping_add(1)
        }
        (*gSoundBanks[bankId as usize].offset(i as isize)).prev =
            (i as libc::c_int - 1 as libc::c_int) as u8_0;
        (*gSoundBanks[bankId as usize].offset(i as isize)).next =
            0xff as libc::c_int as u8_0;
        bankId = bankId.wrapping_add(1)
    }
    if D_801333F8 as libc::c_int == 0 as libc::c_int {
        bankId = 0 as libc::c_int as u8_0;
        while (bankId as libc::c_int) < 10 as libc::c_int {
            gAudioSfxSwapSource[bankId as usize] = 0 as libc::c_int as u16_0;
            gAudioSfxSwapTarget[bankId as usize] = 0 as libc::c_int as u16_0;
            gAudioSfxSwapMode[bankId as usize] = 0 as libc::c_int as u8_0;
            bankId = bankId.wrapping_add(1)
        }
        D_801333F8 = D_801333F8.wrapping_add(1)
    };
}
unsafe extern "C" fn run_static_initializers() {
    sBankSizes =
        [(::std::mem::size_of::<[SoundBankEntry; 9]>() as
              libc::c_ulong).wrapping_div(::std::mem::size_of::<SoundBankEntry>()
                                              as libc::c_ulong) as s32 as
             u8_0,
         (::std::mem::size_of::<[SoundBankEntry; 12]>() as
              libc::c_ulong).wrapping_div(::std::mem::size_of::<SoundBankEntry>()
                                              as libc::c_ulong) as s32 as
             u8_0,
         (::std::mem::size_of::<[SoundBankEntry; 22]>() as
              libc::c_ulong).wrapping_div(::std::mem::size_of::<SoundBankEntry>()
                                              as libc::c_ulong) as s32 as
             u8_0,
         (::std::mem::size_of::<[SoundBankEntry; 20]>() as
              libc::c_ulong).wrapping_div(::std::mem::size_of::<SoundBankEntry>()
                                              as libc::c_ulong) as s32 as
             u8_0,
         (::std::mem::size_of::<[SoundBankEntry; 8]>() as
              libc::c_ulong).wrapping_div(::std::mem::size_of::<SoundBankEntry>()
                                              as libc::c_ulong) as s32 as
             u8_0,
         (::std::mem::size_of::<[SoundBankEntry; 3]>() as
              libc::c_ulong).wrapping_div(::std::mem::size_of::<SoundBankEntry>()
                                              as libc::c_ulong) as s32 as
             u8_0,
         (::std::mem::size_of::<[SoundBankEntry; 5]>() as
              libc::c_ulong).wrapping_div(::std::mem::size_of::<SoundBankEntry>()
                                              as libc::c_ulong) as s32 as
             u8_0]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
