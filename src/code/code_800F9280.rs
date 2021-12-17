#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn Audio_QueueCmdF32(arg0: u32_0, arg1: f32_0);
    #[no_mangle]
    fn Audio_QueueCmdS32(arg0: u32_0, arg1: s32);
    #[no_mangle]
    fn Audio_QueueCmdS8(arg0: u32_0, arg1: s8);
    #[no_mangle]
    fn Audio_QueueCmdU16(arg0: u32_0, arg1: u16_0);
    #[no_mangle]
    fn func_800E5E20(arg0: *mut u32_0) -> u32_0;
    #[no_mangle]
    fn func_800E5EDC() -> s32;
    #[no_mangle]
    fn func_800E5F88(arg0: s32) -> s32;
    #[no_mangle]
    fn AudioDebug_ScrPrt(str: *const s8, num: u16_0);
    #[no_mangle]
    fn func_800F7170();
    #[no_mangle]
    fn func_800F71BC(arg0: s32);
    #[no_mangle]
    static mut D_8016E750: [unk_D_8016E750; 4];
    #[no_mangle]
    static mut gAudioContext: AudioContext;
    #[no_mangle]
    static mut D_80133408: u8_0;
    #[no_mangle]
    static mut sAudioSeqCmds: [u32_0; 256];
    #[no_mangle]
    static mut gAudioSpecId: u8_0;
    #[no_mangle]
    static mut gSfxChannelLayout: u8_0;
    #[no_mangle]
    static mut D_8016E348: [u8_0; 4];
    #[no_mangle]
    static mut D_80133398: [libc::c_char; 0];
    #[no_mangle]
    static mut D_80133390: [libc::c_char; 0];
    #[no_mangle]
    static mut D_8013340C: u8_0;
    #[no_mangle]
    static mut D_80133418: u8_0;
    #[no_mangle]
    static mut sSeqCmdRdPos: u8_0;
    #[no_mangle]
    static mut sSeqCmdWrPos: u8_0;
    #[no_mangle]
    static mut D_80133410: [u8_0; 0];
    #[no_mangle]
    static mut D_8016E320: [[Struct_8016E320; 5]; 0];
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const SAMPLE_TABLE: C2RustUnnamed_0 = 2;
pub const FONT_TABLE: C2RustUnnamed_0 = 1;
pub const SEQUENCE_TABLE: C2RustUnnamed_0 = 0;
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
    pub bitField0: C2RustUnnamed_3,
    pub bitField1: C2RustUnnamed_2,
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
    pub sound: C2RustUnnamed_1,
    pub filter: *mut s16,
    pub pad_18: [libc::c_char; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
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
pub struct C2RustUnnamed_2 {
    #[bitfield(name = "reverbIndex", ty = "u8_0", bits = "0..=2")]
    #[bitfield(name = "bookOffset", ty = "u8_0", bits = "3..=4")]
    #[bitfield(name = "isSyntheticWave", ty = "u8_0", bits = "5..=5")]
    #[bitfield(name = "hasTwoParts", ty = "u8_0", bits = "6..=6")]
    #[bitfield(name = "usesHeadsetPanEffects2", ty = "u8_0", bits = "7..=7")]
    pub reverbIndex_bookOffset_isSyntheticWave_hasTwoParts_usesHeadsetPanEffects2: [u8; 1],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
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
    pub changes: C2RustUnnamed_5,
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
    pub u: C2RustUnnamed_4,
    pub pool: *mut NotePool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
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
pub union C2RustUnnamed_5 {
    pub s: C2RustUnnamed_6,
    pub asByte: u8_0,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
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
    pub action: C2RustUnnamed_7,
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
pub union C2RustUnnamed_7 {
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
    pub c2rust_unnamed: C2RustUnnamed_9,
    pub c2rust_unnamed_0: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
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
pub union C2RustUnnamed_9 {
    pub opArgs: u32_0,
    pub c2rust_unnamed: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
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
pub type C2RustUnnamed_11 = libc::c_uint;
pub const SEQ_PLAYER_BGM_SUB: C2RustUnnamed_11 = 3;
pub const SEQ_PLAYER_SFX: C2RustUnnamed_11 = 2;
pub const SEQ_PLAYER_FANFARE: C2RustUnnamed_11 = 1;
pub const SEQ_PLAYER_BGM_MAIN: C2RustUnnamed_11 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Struct_8016E320 {
    pub unk_0: u8_0,
    pub unk_1: u8_0,
}
// TODO: clean up these macros. They are similar to ones in code_800EC960.c but without casts.
#[no_mangle]
pub unsafe extern "C" fn func_800F9280(mut playerIdx: u8_0, mut seqId: u8_0,
                                       mut arg2: u8_0, mut fadeTimer: u16_0) {
    let mut i: u8_0 = 0;
    let mut dur: u16_0 = 0;
    let mut pad: s32 = 0;
    if D_80133408 as libc::c_int == 0 as libc::c_int ||
           playerIdx as libc::c_int == SEQ_PLAYER_SFX as libc::c_int {
        arg2 = (arg2 as libc::c_int & 0x7f as libc::c_int) as u8_0;
        if arg2 as libc::c_int == 0x7f as libc::c_int {
            dur =
                ((fadeTimer as libc::c_int >> 3 as libc::c_int) *
                     60 as libc::c_int *
                     gAudioContext.audioBufferParameters.updatesPerFrame as
                         libc::c_int) as u16_0;
            Audio_QueueCmdS32(0x85000000 as libc::c_uint |
                                  (playerIdx as u32_0 &
                                       (((0x1 as libc::c_int) <<
                                             8 as libc::c_int) -
                                            1 as libc::c_int) as libc::c_uint)
                                      << 16 as libc::c_int |
                                  (seqId as u32_0 &
                                       (((0x1 as libc::c_int) <<
                                             8 as libc::c_int) -
                                            1 as libc::c_int) as libc::c_uint)
                                      << 8 as libc::c_int, dur as s32);
        } else {
            Audio_QueueCmdS32(0x82000000 as libc::c_uint |
                                  (playerIdx as u32_0 &
                                       (((0x1 as libc::c_int) <<
                                             8 as libc::c_int) -
                                            1 as libc::c_int) as libc::c_uint)
                                      << 16 as libc::c_int |
                                  (seqId as u32_0 &
                                       (((0x1 as libc::c_int) <<
                                             8 as libc::c_int) -
                                            1 as libc::c_int) as libc::c_uint)
                                      << 8 as libc::c_int,
                              fadeTimer as libc::c_int *
                                  gAudioContext.audioBufferParameters.updatesPerFrame
                                      as u16_0 as libc::c_int /
                                  4 as libc::c_int);
        }
        D_8016E750[playerIdx as usize].unk_254 =
            (seqId as libc::c_int | (arg2 as libc::c_int) << 8 as libc::c_int)
                as u16_0;
        D_8016E750[playerIdx as usize].unk_256 =
            (seqId as libc::c_int | (arg2 as libc::c_int) << 8 as libc::c_int)
                as u16_0;
        if D_8016E750[playerIdx as usize].volCur != 1.0f32 {
            Audio_QueueCmdF32(0x41000000 as libc::c_int as libc::c_uint |
                                  (playerIdx as u32_0 &
                                       (((0x1 as libc::c_int) <<
                                             8 as libc::c_int) -
                                            1 as libc::c_int) as libc::c_uint)
                                      << 16 as libc::c_int,
                              D_8016E750[playerIdx as usize].volCur);
        }
        D_8016E750[playerIdx as usize].unk_28 = 0 as libc::c_int as u16_0;
        D_8016E750[playerIdx as usize].unk_18 = 0 as libc::c_int as u16_0;
        D_8016E750[playerIdx as usize].unk_14 = 0 as libc::c_int as u32_0;
        i = 0 as libc::c_int as u8_0;
        while (i as libc::c_int) < 0x10 as libc::c_int {
            D_8016E750[playerIdx as usize].unk_50[i as usize].unk_00 = 1.0f32;
            D_8016E750[playerIdx as usize].unk_50[i as usize].unk_0C =
                0 as libc::c_int as u16_0;
            D_8016E750[playerIdx as usize].unk_50[i as usize].unk_10 = 1.0f32;
            D_8016E750[playerIdx as usize].unk_50[i as usize].unk_1C =
                0 as libc::c_int as u16_0;
            i = i.wrapping_add(1)
        }
        D_8016E750[playerIdx as usize].unk_250 = 0 as libc::c_int as u16_0;
        D_8016E750[playerIdx as usize].unk_252 = 0 as libc::c_int as u16_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800F9474(mut playerIdx: u8_0, mut arg1: u16_0) {
    Audio_QueueCmdS32(0x83000000 as libc::c_uint |
                          ((playerIdx as libc::c_int) << 16 as libc::c_int) as
                              libc::c_uint,
                      arg1 as libc::c_int *
                          gAudioContext.audioBufferParameters.updatesPerFrame
                              as u16_0 as libc::c_int / 4 as libc::c_int);
    D_8016E750[playerIdx as usize].unk_254 = 0xffff as libc::c_int as u16_0;
}
// sorted by unk_1, descending
#[no_mangle]
pub unsafe extern "C" fn Audio_ProcessSeqCmd(mut cmd: u32_0) {
    let mut pad: [s32; 2] = [0; 2]; // "SEQ H"
    let mut fadeTimer: u16_0 = 0;
    let mut channelMask: u16_0 = 0;
    let mut val: u16_0 = 0;
    let mut oldSpec: u8_0 = 0;
    let mut spec: u8_0 = 0;
    let mut op: u8_0 = 0;
    let mut subOp: u8_0 = 0;
    let mut playerIdx: u8_0 = 0;
    let mut seqId: u8_0 = 0;
    let mut seqArgs: u8_0 = 0;
    let mut found: u8_0 = 0;
    let mut port: u8_0 = 0;
    let mut duration: u8_0 = 0;
    let mut chanIdx: u8_0 = 0;
    let mut i: u8_0 = 0;
    let mut new_var: s32 = 0;
    let mut freqScale: f32_0 = 0.;
    if D_8013340C as libc::c_int != 0 &&
           cmd & 0xf0000000 as libc::c_uint !=
               0x70000000 as libc::c_int as libc::c_uint {
        AudioDebug_ScrPrt(D_80133390.as_mut_ptr() as *const s8,
                          (cmd >> 16 as libc::c_int &
                               0xffff as libc::c_int as libc::c_uint) as
                              u16_0);
        AudioDebug_ScrPrt(D_80133398.as_mut_ptr() as *const s8,
                          (cmd & 0xffff as libc::c_int as libc::c_uint) as
                              u16_0);
        // "    L"
    }
    op = (cmd >> 28 as libc::c_int) as u8_0;
    playerIdx =
        ((cmd & 0xf000000 as libc::c_int as libc::c_uint) >>
             24 as libc::c_int) as u8_0;
    match op as libc::c_int {
        0 => {
            // play sequence immediately
            seqId = (cmd & 0xff as libc::c_int as libc::c_uint) as u8_0;
            seqArgs =
                ((cmd & 0xff00 as libc::c_int as libc::c_uint) >>
                     8 as libc::c_int) as u8_0;
            fadeTimer =
                ((cmd & 0xff0000 as libc::c_int as libc::c_uint) >>
                     13 as libc::c_int) as u16_0;
            if D_8016E750[playerIdx as usize].unk_260 as libc::c_int ==
                   0 as libc::c_int &&
                   (seqArgs as libc::c_int) < 0x80 as libc::c_int {
                func_800F9280(playerIdx, seqId, seqArgs, fadeTimer);
            }
        }
        1 => {
            // disable seq player
            fadeTimer =
                ((cmd & 0xff0000 as libc::c_int as libc::c_uint) >>
                     13 as libc::c_int) as u16_0;
            func_800F9474(playerIdx, fadeTimer);
        }
        2 => {
            // queue sequence
            seqId = (cmd & 0xff as libc::c_int as libc::c_uint) as u8_0;
            seqArgs =
                ((cmd & 0xff00 as libc::c_int as libc::c_uint) >>
                     8 as libc::c_int) as u8_0;
            fadeTimer =
                ((cmd & 0xff0000 as libc::c_int as libc::c_uint) >>
                     13 as libc::c_int) as u16_0;
            new_var = seqArgs as s32;
            i = 0 as libc::c_int as u8_0;
            while (i as libc::c_int) <
                      D_8016E348[playerIdx as usize] as libc::c_int {
                if (*D_8016E320.as_mut_ptr().offset(playerIdx as
                                                        isize))[i as
                                                                    usize].unk_0
                       as libc::c_int == seqId as libc::c_int {
                    if i as libc::c_int == 0 as libc::c_int {
                        func_800F9280(playerIdx, seqId, seqArgs, fadeTimer);
                    }
                    return
                }
                i = i.wrapping_add(1)
            }
            found = D_8016E348[playerIdx as usize];
            i = 0 as libc::c_int as u8_0;
            while (i as libc::c_int) <
                      D_8016E348[playerIdx as usize] as libc::c_int {
                if (*D_8016E320.as_mut_ptr().offset(playerIdx as
                                                        isize))[i as
                                                                    usize].unk_1
                       as libc::c_int <= new_var {
                    found = i;
                    i = D_8016E348[playerIdx as usize]
                    // "break;"
                }
                i = i.wrapping_add(1)
            }
            if (D_8016E348[playerIdx as usize] as libc::c_int) <
                   5 as libc::c_int {
                D_8016E348[playerIdx as usize] =
                    D_8016E348[playerIdx as usize].wrapping_add(1)
            }
            i =
                (D_8016E348[playerIdx as usize] as libc::c_int -
                     1 as libc::c_int) as u8_0;
            while i as libc::c_int != found as libc::c_int {
                (*D_8016E320.as_mut_ptr().offset(playerIdx as
                                                     isize))[i as usize].unk_1
                    =
                    (*D_8016E320.as_mut_ptr().offset(playerIdx as
                                                         isize))[(i as
                                                                      libc::c_int
                                                                      -
                                                                      1 as
                                                                          libc::c_int)
                                                                     as
                                                                     usize].unk_1;
                (*D_8016E320.as_mut_ptr().offset(playerIdx as
                                                     isize))[i as usize].unk_0
                    =
                    (*D_8016E320.as_mut_ptr().offset(playerIdx as
                                                         isize))[(i as
                                                                      libc::c_int
                                                                      -
                                                                      1 as
                                                                          libc::c_int)
                                                                     as
                                                                     usize].unk_0;
                i = i.wrapping_sub(1)
            }
            (*D_8016E320.as_mut_ptr().offset(playerIdx as
                                                 isize))[found as usize].unk_1
                = seqArgs;
            (*D_8016E320.as_mut_ptr().offset(playerIdx as
                                                 isize))[found as usize].unk_0
                = seqId;
            if found as libc::c_int == 0 as libc::c_int {
                func_800F9280(playerIdx, seqId, seqArgs, fadeTimer);
            }
        }
        3 => {
            // unqueue/stop sequence
            seqId = (cmd & 0xff as libc::c_int as libc::c_uint) as u8_0;
            fadeTimer =
                ((cmd & 0xff0000 as libc::c_int as libc::c_uint) >>
                     13 as libc::c_int) as u16_0;
            found = D_8016E348[playerIdx as usize];
            i = 0 as libc::c_int as u8_0;
            while (i as libc::c_int) <
                      D_8016E348[playerIdx as usize] as libc::c_int {
                if (*D_8016E320.as_mut_ptr().offset(playerIdx as
                                                        isize))[i as
                                                                    usize].unk_0
                       as libc::c_int == seqId as libc::c_int {
                    found = i;
                    i = D_8016E348[playerIdx as usize]
                    // "break;"
                }
                i = i.wrapping_add(1)
            }
            if found as libc::c_int !=
                   D_8016E348[playerIdx as usize] as libc::c_int {
                i = found;
                while (i as libc::c_int) <
                          D_8016E348[playerIdx as usize] as libc::c_int -
                              1 as libc::c_int {
                    (*D_8016E320.as_mut_ptr().offset(playerIdx as
                                                         isize))[i as
                                                                     usize].unk_1
                        =
                        (*D_8016E320.as_mut_ptr().offset(playerIdx as
                                                             isize))[(i as
                                                                          libc::c_int
                                                                          +
                                                                          1 as
                                                                              libc::c_int)
                                                                         as
                                                                         usize].unk_1;
                    (*D_8016E320.as_mut_ptr().offset(playerIdx as
                                                         isize))[i as
                                                                     usize].unk_0
                        =
                        (*D_8016E320.as_mut_ptr().offset(playerIdx as
                                                             isize))[(i as
                                                                          libc::c_int
                                                                          +
                                                                          1 as
                                                                              libc::c_int)
                                                                         as
                                                                         usize].unk_0;
                    i = i.wrapping_add(1)
                }
                D_8016E348[playerIdx as usize] =
                    D_8016E348[playerIdx as usize].wrapping_sub(1)
            }
            if found as libc::c_int == 0 as libc::c_int {
                func_800F9474(playerIdx, fadeTimer);
                if D_8016E348[playerIdx as usize] as libc::c_int !=
                       0 as libc::c_int {
                    func_800F9280(playerIdx,
                                  (*D_8016E320.as_mut_ptr().offset(playerIdx
                                                                       as
                                                                       isize))[0
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   usize].unk_0,
                                  (*D_8016E320.as_mut_ptr().offset(playerIdx
                                                                       as
                                                                       isize))[0
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   usize].unk_1,
                                  fadeTimer);
                }
            }
        }
        4 => {
            // transition seq volume
            duration =
                ((cmd & 0xff0000 as libc::c_int as libc::c_uint) >>
                     15 as libc::c_int) as u8_0;
            val = (cmd & 0xff as libc::c_int as libc::c_uint) as u16_0;
            if duration as libc::c_int == 0 as libc::c_int {
                duration = duration.wrapping_add(1)
            }
            D_8016E750[playerIdx as usize].volTarget =
                val as f32_0 / 127.0f32;
            if D_8016E750[playerIdx as usize].volCur !=
                   D_8016E750[playerIdx as usize].volTarget {
                D_8016E750[playerIdx as usize].unk_08 =
                    (D_8016E750[playerIdx as usize].volCur -
                         D_8016E750[playerIdx as usize].volTarget) /
                        duration as f32_0;
                D_8016E750[playerIdx as usize].unk_0C = duration as u16_0
            }
        }
        5 => {
            // transition freq scale for all channels
            duration =
                ((cmd & 0xff0000 as libc::c_int as libc::c_uint) >>
                     15 as libc::c_int) as u8_0;
            val = (cmd & 0xffff as libc::c_int as libc::c_uint) as u16_0;
            if duration as libc::c_int == 0 as libc::c_int {
                duration = duration.wrapping_add(1)
            }
            freqScale = val as f32_0 / 1000.0f32;
            i = 0 as libc::c_int as u8_0;
            while (i as libc::c_int) < 16 as libc::c_int {
                D_8016E750[playerIdx as usize].unk_50[i as usize].unk_14 =
                    freqScale;
                D_8016E750[playerIdx as usize].unk_50[i as usize].unk_1C =
                    duration as u16_0;
                D_8016E750[playerIdx as usize].unk_50[i as usize].unk_18 =
                    (D_8016E750[playerIdx as usize].unk_50[i as usize].unk_10
                         - freqScale) / duration as f32_0;
                i = i.wrapping_add(1)
            }
            D_8016E750[playerIdx as usize].unk_250 =
                0xffff as libc::c_int as u16_0
        }
        13 => {
            // transition freq scale
            duration =
                ((cmd & 0xff0000 as libc::c_int as libc::c_uint) >>
                     15 as libc::c_int) as u8_0;
            chanIdx =
                ((cmd & 0xf000 as libc::c_int as libc::c_uint) >>
                     12 as libc::c_int) as u8_0;
            val = (cmd & 0xfff as libc::c_int as libc::c_uint) as u16_0;
            if duration as libc::c_int == 0 as libc::c_int {
                duration = duration.wrapping_add(1)
            }
            freqScale = val as f32_0 / 1000.0f32;
            D_8016E750[playerIdx as usize].unk_50[chanIdx as usize].unk_14 =
                freqScale;
            D_8016E750[playerIdx as usize].unk_50[chanIdx as usize].unk_18 =
                (D_8016E750[playerIdx as
                                usize].unk_50[chanIdx as usize].unk_10 -
                     freqScale) / duration as f32_0;
            D_8016E750[playerIdx as usize].unk_50[chanIdx as usize].unk_1C =
                duration as u16_0;
            D_8016E750[playerIdx as usize].unk_250 =
                (D_8016E750[playerIdx as usize].unk_250 as libc::c_int |
                     (1 as libc::c_int) << chanIdx as libc::c_int) as u16_0
        }
        6 => {
            // transition vol scale
            duration =
                ((cmd & 0xff0000 as libc::c_int as libc::c_uint) >>
                     15 as libc::c_int) as u8_0;
            chanIdx =
                ((cmd & 0xf00 as libc::c_int as libc::c_uint) >>
                     8 as libc::c_int) as u8_0;
            val = (cmd & 0xff as libc::c_int as libc::c_uint) as u16_0;
            if duration as libc::c_int == 0 as libc::c_int {
                duration = duration.wrapping_add(1)
            }
            D_8016E750[playerIdx as usize].unk_50[chanIdx as usize].unk_04 =
                val as f32_0 / 127.0f32;
            if D_8016E750[playerIdx as usize].unk_50[chanIdx as usize].unk_00
                   !=
                   D_8016E750[playerIdx as
                                  usize].unk_50[chanIdx as usize].unk_04 {
                D_8016E750[playerIdx as usize].unk_50[chanIdx as usize].unk_08
                    =
                    (D_8016E750[playerIdx as
                                    usize].unk_50[chanIdx as usize].unk_00 -
                         D_8016E750[playerIdx as
                                        usize].unk_50[chanIdx as
                                                          usize].unk_04) /
                        duration as f32_0;
                D_8016E750[playerIdx as usize].unk_50[chanIdx as usize].unk_0C
                    = duration as u16_0;
                D_8016E750[playerIdx as usize].unk_252 =
                    (D_8016E750[playerIdx as usize].unk_252 as libc::c_int |
                         (1 as libc::c_int) << chanIdx as libc::c_int) as
                        u16_0
            }
        }
        7 => {
            // set global io port
            port =
                ((cmd & 0xff0000 as libc::c_int as libc::c_uint) >>
                     16 as libc::c_int) as u8_0;
            val = (cmd & 0xff as libc::c_int as libc::c_uint) as u16_0;
            Audio_QueueCmdS8(0x46000000 as libc::c_int as libc::c_uint |
                                 (playerIdx as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            8 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 16 as libc::c_int |
                                 (port as u32_0 &
                                      (((0x1 as libc::c_int) <<
                                            8 as libc::c_int) -
                                           1 as libc::c_int) as libc::c_uint)
                                     << 0 as libc::c_int, val as s8);
        }
        8 => {
            // set io port if channel masked
            chanIdx =
                ((cmd & 0xf00 as libc::c_int as libc::c_uint) >>
                     8 as libc::c_int) as u8_0;
            port =
                ((cmd & 0xff0000 as libc::c_int as libc::c_uint) >>
                     16 as libc::c_int) as u8_0;
            val = (cmd & 0xff as libc::c_int as libc::c_uint) as u16_0;
            if D_8016E750[playerIdx as usize].unk_258 as libc::c_int &
                   (1 as libc::c_int) << chanIdx as libc::c_int ==
                   0 as libc::c_int {
                Audio_QueueCmdS8(0x6000000 as libc::c_int as libc::c_uint |
                                     (playerIdx as u32_0 &
                                          (((0x1 as libc::c_int) <<
                                                8 as libc::c_int) -
                                               1 as libc::c_int) as
                                              libc::c_uint) <<
                                         16 as libc::c_int |
                                     (chanIdx as u32_0 &
                                          (((0x1 as libc::c_int) <<
                                                8 as libc::c_int) -
                                               1 as libc::c_int) as
                                              libc::c_uint) <<
                                         8 as libc::c_int |
                                     (port as u32_0 &
                                          (((0x1 as libc::c_int) <<
                                                8 as libc::c_int) -
                                               1 as libc::c_int) as
                                              libc::c_uint) <<
                                         0 as libc::c_int, val as s8);
            }
        }
        9 => {
            // set channel mask for command 0x8
            D_8016E750[playerIdx as usize].unk_258 =
                (cmd & 0xffff as libc::c_int as libc::c_uint) as u16_0
        }
        10 => {
            // set channel stop mask
            channelMask =
                (cmd & 0xffff as libc::c_int as libc::c_uint) as u16_0;
            if channelMask as libc::c_int != 0 as libc::c_int {
                // with channel mask channelMask...
                Audio_QueueCmdU16(0x90000000 as libc::c_uint |
                                      (playerIdx as u32_0 &
                                           (((0x1 as libc::c_int) <<
                                                 8 as libc::c_int) -
                                                1 as libc::c_int) as
                                               libc::c_uint) <<
                                          16 as libc::c_int, channelMask);
                // stop channels
                Audio_QueueCmdS8(0x8000000 as libc::c_int as libc::c_uint |
                                     (playerIdx as u32_0 &
                                          (((0x1 as libc::c_int) <<
                                                8 as libc::c_int) -
                                               1 as libc::c_int) as
                                              libc::c_uint) <<
                                         16 as libc::c_int |
                                     0xff00 as libc::c_int as libc::c_uint,
                                 1 as libc::c_int as s8);
            }
            if channelMask as libc::c_int ^ 0xffff as libc::c_int !=
                   0 as libc::c_int {
                // with channel mask ~channelMask...
                Audio_QueueCmdU16(0x90000000 as libc::c_uint |
                                      (playerIdx as u32_0 &
                                           (((0x1 as libc::c_int) <<
                                                 8 as libc::c_int) -
                                                1 as libc::c_int) as
                                               libc::c_uint) <<
                                          16 as libc::c_int,
                                  (channelMask as libc::c_int ^
                                       0xffff as libc::c_int) as u16_0);
                // unstop channels
                Audio_QueueCmdS8(0x8000000 as libc::c_int as libc::c_uint |
                                     (playerIdx as u32_0 &
                                          (((0x1 as libc::c_int) <<
                                                8 as libc::c_int) -
                                               1 as libc::c_int) as
                                              libc::c_uint) <<
                                         16 as libc::c_int |
                                     0xff00 as libc::c_int as libc::c_uint,
                                 0 as libc::c_int as s8);
            }
        }
        11 => {
            // update tempo
            D_8016E750[playerIdx as usize].unk_14 = cmd
        }
        12 => {
            // start sequence with setup commands
            subOp =
                ((cmd & 0xf00000 as libc::c_int as libc::c_uint) >>
                     20 as libc::c_int) as u8_0;
            if subOp as libc::c_int != 0xf as libc::c_int {
                if (D_8016E750[playerIdx as usize].unk_4D as libc::c_int) <
                       7 as libc::c_int {
                    let fresh0 = D_8016E750[playerIdx as usize].unk_4D;
                    D_8016E750[playerIdx as usize].unk_4D =
                        D_8016E750[playerIdx as usize].unk_4D.wrapping_add(1);
                    found = fresh0;
                    if (found as libc::c_int) < 8 as libc::c_int {
                        D_8016E750[playerIdx as usize].unk_2C[found as usize]
                            = cmd;
                        D_8016E750[playerIdx as usize].unk_4C =
                            2 as libc::c_int as u8_0
                    }
                }
            } else {
                D_8016E750[playerIdx as usize].unk_4D =
                    0 as libc::c_int as u8_0
            }
        }
        14 => {
            subOp =
                ((cmd & 0xf00 as libc::c_int as libc::c_uint) >>
                     8 as libc::c_int) as u8_0;
            val = (cmd & 0xff as libc::c_int as libc::c_uint) as u16_0;
            match subOp as libc::c_int {
                0 => {
                    // set sound mode
                    Audio_QueueCmdS32(0xf0000000 as libc::c_uint,
                                      *D_80133410.as_mut_ptr().offset(val as
                                                                          isize)
                                          as s32);
                }
                1 => {
                    // set sequence starting disabled?
                    D_80133408 =
                        (val as libc::c_int & 1 as libc::c_int) as u8_0
                }
                _ => { }
            }
        }
        15 => {
            // change spec
            spec = (cmd & 0xff as libc::c_int as libc::c_uint) as u8_0;
            gSfxChannelLayout =
                ((cmd & 0xff00 as libc::c_int as libc::c_uint) >>
                     8 as libc::c_int) as u8_0;
            oldSpec = gAudioSpecId;
            gAudioSpecId = spec;
            func_800E5F88(spec as s32);
            func_800F71BC(oldSpec as s32);
            Audio_QueueCmdS32(0xf8000000 as libc::c_uint, 0 as libc::c_int);
        }
        _ => { }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Audio_QueueSeqCmd(mut cmd: u32_0) {
    let fresh1 = sSeqCmdWrPos;
    sSeqCmdWrPos = sSeqCmdWrPos.wrapping_add(1);
    sAudioSeqCmds[fresh1 as usize] = cmd;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_ProcessSeqCmds() {
    while sSeqCmdWrPos as libc::c_int != sSeqCmdRdPos as libc::c_int {
        let fresh2 = sSeqCmdRdPos;
        sSeqCmdRdPos = sSeqCmdRdPos.wrapping_add(1);
        Audio_ProcessSeqCmd(sAudioSeqCmds[fresh2 as usize]);
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800FA0B4(mut playerIdx: u8_0) -> u16_0 {
    if gAudioContext.seqPlayers[playerIdx as usize].enabled() == 0 {
        return 0xffff as libc::c_int as u16_0
    }
    return D_8016E750[playerIdx as usize].unk_254;
}
#[no_mangle]
pub unsafe extern "C" fn func_800FA11C(mut arg0: u32_0, mut arg1: u32_0)
 -> s32 {
    let mut i: u8_0 = 0;
    i = sSeqCmdRdPos;
    while i as libc::c_int != sSeqCmdWrPos as libc::c_int {
        if arg0 == sAudioSeqCmds[i as usize] & arg1 {
            return 0 as libc::c_int
        }
        i = i.wrapping_add(1)
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_800FA174(mut playerIdx: u8_0) {
    D_8016E348[playerIdx as usize] = 0 as libc::c_int as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn func_800FA18C(mut playerIdx: u8_0, mut arg1: u8_0) {
    let mut i: u8_0 = 0;
    i = 0 as libc::c_int as u8_0;
    while (i as libc::c_int) <
              D_8016E750[playerIdx as usize].unk_4D as libc::c_int {
        let mut unkb: u8_0 =
            ((D_8016E750[playerIdx as usize].unk_2C[i as usize] &
                  0xf00000 as libc::c_int as libc::c_uint) >>
                 20 as libc::c_int) as u8_0;
        if unkb as libc::c_int == arg1 as libc::c_int {
            D_8016E750[playerIdx as usize].unk_2C[i as usize] =
                0xff000000 as libc::c_uint
        }
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn Audio_SetVolScale(mut playerIdx: u8_0,
                                           mut scaleIdx: u8_0,
                                           mut targetVol: u8_0,
                                           mut volFadeTimer: u8_0) {
    let mut volScale: f32_0 = 0.;
    let mut i: u8_0 = 0;
    D_8016E750[playerIdx as usize].volScales[scaleIdx as usize] =
        (targetVol as libc::c_int & 0x7f as libc::c_int) as u8_0;
    if volFadeTimer as libc::c_int != 0 as libc::c_int {
        D_8016E750[playerIdx as usize].fadeVolUpdate =
            1 as libc::c_int as u8_0;
        D_8016E750[playerIdx as usize].volFadeTimer = volFadeTimer
    } else {
        i = 0 as libc::c_int as u8_0;
        volScale = 1.0f32;
        while (i as libc::c_int) < 4 as libc::c_int {
            volScale *=
                D_8016E750[playerIdx as usize].volScales[i as usize] as
                    libc::c_int as libc::c_float / 127.0f32;
            i = i.wrapping_add(1)
        }
        Audio_ProcessSeqCmd((0x40000000 as libc::c_int |
                                 (playerIdx as libc::c_int) <<
                                     24 as libc::c_int |
                                 (volFadeTimer as libc::c_int) <<
                                     16 as libc::c_int |
                                 (volScale * 127.0f32) as u8_0 as libc::c_int)
                                as u32_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800FA3DC() {
    let mut temp_a1: u32_0 = 0;
    let mut temp_lo: u16_0 = 0;
    let mut temp_v1: u16_0 = 0;
    let mut phi_a2: u16_0 = 0;
    let mut temp_v0_4: u8_0 = 0;
    let mut temp_a0: u8_0 = 0;
    let mut temp_s1: u8_0 = 0;
    let mut temp_s0_3: u8_0 = 0;
    let mut temp_a3_3: u8_0 = 0;
    let mut pad: [s32; 3] = [0; 3];
    let mut dummy: u32_0 = 0;
    let mut phi_f0: f32_0 = 0.;
    let mut phi_t0: u8_0 = 0;
    let mut playerIdx: u8_0 = 0;
    let mut j: u8_0 = 0;
    let mut k: u8_0 = 0;
    playerIdx = 0 as libc::c_int as u8_0;
    while (playerIdx as libc::c_int) < 4 as libc::c_int {
        if D_8016E750[playerIdx as usize].unk_260 as libc::c_int !=
               0 as libc::c_int {
            match func_800E5E20(&mut dummy) {
                1 | 2 | 3 | 4 => {
                    D_8016E750[playerIdx as usize].unk_260 =
                        0 as libc::c_int as u8_0;
                    Audio_ProcessSeqCmd(D_8016E750[playerIdx as
                                                       usize].unk_25C);
                }
                _ => { }
            }
        }
        if D_8016E750[playerIdx as usize].fadeVolUpdate != 0 {
            phi_f0 = 1.0f32;
            j = 0 as libc::c_int as u8_0;
            while (j as libc::c_int) < 4 as libc::c_int {
                phi_f0 *=
                    D_8016E750[playerIdx as usize].volScales[j as usize] as
                        libc::c_int as libc::c_float / 127.0f32;
                j = j.wrapping_add(1)
            }
            Audio_QueueSeqCmd((0x40000000 as libc::c_int |
                                   (playerIdx as libc::c_int) <<
                                       24 as libc::c_int |
                                   (D_8016E750[playerIdx as
                                                   usize].volFadeTimer as
                                        libc::c_int) << 16 as libc::c_int |
                                   (phi_f0 * 127.0f32) as u8_0 as libc::c_int)
                                  as u32_0);
            D_8016E750[playerIdx as usize].fadeVolUpdate =
                0 as libc::c_int as u8_0
        }
        if D_8016E750[playerIdx as usize].unk_0C as libc::c_int !=
               0 as libc::c_int {
            D_8016E750[playerIdx as usize].unk_0C =
                D_8016E750[playerIdx as usize].unk_0C.wrapping_sub(1);
            if D_8016E750[playerIdx as usize].unk_0C as libc::c_int !=
                   0 as libc::c_int {
                D_8016E750[playerIdx as usize].volCur =
                    D_8016E750[playerIdx as usize].volCur -
                        D_8016E750[playerIdx as usize].unk_08
            } else {
                D_8016E750[playerIdx as usize].volCur =
                    D_8016E750[playerIdx as usize].volTarget
            }
            Audio_QueueCmdF32(0x41000000 as libc::c_int as libc::c_uint |
                                  (playerIdx as u32_0 &
                                       (((0x1 as libc::c_int) <<
                                             8 as libc::c_int) -
                                            1 as libc::c_int) as libc::c_uint)
                                      << 16 as libc::c_int,
                              D_8016E750[playerIdx as usize].volCur);
        }
        if D_8016E750[playerIdx as usize].unk_14 !=
               0 as libc::c_int as libc::c_uint {
            temp_a1 = D_8016E750[playerIdx as usize].unk_14;
            phi_t0 =
                ((temp_a1 & 0xff0000 as libc::c_int as libc::c_uint) >>
                     15 as libc::c_int) as u8_0;
            phi_a2 =
                (temp_a1 & 0xfff as libc::c_int as libc::c_uint) as u16_0;
            if phi_t0 as libc::c_int == 0 as libc::c_int {
                phi_t0 = phi_t0.wrapping_add(1)
            }
            if gAudioContext.seqPlayers[playerIdx as usize].enabled() != 0 {
                temp_lo =
                    (gAudioContext.seqPlayers[playerIdx as usize].tempo as
                         libc::c_int / 0x30 as libc::c_int) as u16_0;
                temp_v0_4 =
                    ((temp_a1 & 0xf000 as libc::c_int as libc::c_uint) >>
                         12 as libc::c_int) as u8_0;
                match temp_v0_4 as libc::c_int {
                    1 => {
                        phi_a2 =
                            (phi_a2 as libc::c_int + temp_lo as libc::c_int)
                                as u16_0
                    }
                    2 => {
                        if (phi_a2 as libc::c_int) < temp_lo as libc::c_int {
                            phi_a2 =
                                (temp_lo as libc::c_int -
                                     phi_a2 as libc::c_int) as u16_0
                        }
                    }
                    3 => {
                        phi_a2 =
                            (temp_lo as libc::c_int as libc::c_float *
                                 (phi_a2 as libc::c_int as libc::c_float /
                                      100.0f32)) as u16_0
                    }
                    4 => {
                        if D_8016E750[playerIdx as usize].unk_18 != 0 {
                            phi_a2 = D_8016E750[playerIdx as usize].unk_18
                        } else { phi_a2 = temp_lo }
                    }
                    _ => { }
                }
                if phi_a2 as libc::c_int > 300 as libc::c_int {
                    phi_a2 = 300 as libc::c_int as u16_0
                }
                if D_8016E750[playerIdx as usize].unk_18 as libc::c_int ==
                       0 as libc::c_int {
                    D_8016E750[playerIdx as usize].unk_18 = temp_lo
                }
                D_8016E750[playerIdx as usize].unk_20 = phi_a2 as f32_0;
                D_8016E750[playerIdx as usize].unk_1C =
                    (gAudioContext.seqPlayers[playerIdx as usize].tempo as
                         libc::c_int / 0x30 as libc::c_int) as f32_0;
                D_8016E750[playerIdx as usize].unk_24 =
                    (D_8016E750[playerIdx as usize].unk_1C -
                         D_8016E750[playerIdx as usize].unk_20) /
                        phi_t0 as libc::c_int as libc::c_float;
                D_8016E750[playerIdx as usize].unk_28 = phi_t0 as u16_0;
                D_8016E750[playerIdx as usize].unk_14 =
                    0 as libc::c_int as u32_0
            }
        }
        if D_8016E750[playerIdx as usize].unk_28 as libc::c_int !=
               0 as libc::c_int {
            D_8016E750[playerIdx as usize].unk_28 =
                D_8016E750[playerIdx as usize].unk_28.wrapping_sub(1);
            if D_8016E750[playerIdx as usize].unk_28 as libc::c_int !=
                   0 as libc::c_int {
                D_8016E750[playerIdx as usize].unk_1C =
                    D_8016E750[playerIdx as usize].unk_1C -
                        D_8016E750[playerIdx as usize].unk_24
            } else {
                D_8016E750[playerIdx as usize].unk_1C =
                    D_8016E750[playerIdx as usize].unk_20
            }
            // set tempo
            Audio_QueueCmdS32(0x47000000 as libc::c_int as libc::c_uint |
                                  (playerIdx as u32_0 &
                                       (((0x1 as libc::c_int) <<
                                             8 as libc::c_int) -
                                            1 as libc::c_int) as libc::c_uint)
                                      << 16 as libc::c_int,
                              D_8016E750[playerIdx as usize].unk_1C as s32);
        }
        if D_8016E750[playerIdx as usize].unk_252 as libc::c_int !=
               0 as libc::c_int {
            k = 0 as libc::c_int as u8_0;
            while (k as libc::c_int) < 0x10 as libc::c_int {
                if D_8016E750[playerIdx as usize].unk_50[k as usize].unk_0C as
                       libc::c_int != 0 as libc::c_int {
                    D_8016E750[playerIdx as usize].unk_50[k as usize].unk_0C =
                        D_8016E750[playerIdx as
                                       usize].unk_50[k as
                                                         usize].unk_0C.wrapping_sub(1);
                    if D_8016E750[playerIdx as
                                      usize].unk_50[k as usize].unk_0C as
                           libc::c_int != 0 as libc::c_int {
                        D_8016E750[playerIdx as
                                       usize].unk_50[k as usize].unk_00 -=
                            D_8016E750[playerIdx as
                                           usize].unk_50[k as usize].unk_08
                    } else {
                        D_8016E750[playerIdx as
                                       usize].unk_50[k as usize].unk_00 =
                            D_8016E750[playerIdx as
                                           usize].unk_50[k as usize].unk_04;
                        D_8016E750[playerIdx as usize].unk_252 =
                            (D_8016E750[playerIdx as usize].unk_252 as
                                 libc::c_int ^
                                 (1 as libc::c_int) << k as libc::c_int) as
                                u16_0
                    }
                    // CHAN_UPD_VOL_SCALE (playerIdx = seq, k = chan)
                    Audio_QueueCmdF32(0x1000000 as libc::c_int as libc::c_uint
                                          |
                                          (playerIdx as u32_0 &
                                               (((0x1 as libc::c_int) <<
                                                     8 as libc::c_int) -
                                                    1 as libc::c_int) as
                                                   libc::c_uint) <<
                                              16 as libc::c_int |
                                          (k as u32_0 &
                                               (((0x1 as libc::c_int) <<
                                                     8 as libc::c_int) -
                                                    1 as libc::c_int) as
                                                   libc::c_uint) <<
                                              8 as libc::c_int,
                                      D_8016E750[playerIdx as
                                                     usize].unk_50[k as
                                                                       usize].unk_00);
                }
                k = k.wrapping_add(1)
            }
        }
        if D_8016E750[playerIdx as usize].unk_250 as libc::c_int !=
               0 as libc::c_int {
            k = 0 as libc::c_int as u8_0;
            while (k as libc::c_int) < 0x10 as libc::c_int {
                if D_8016E750[playerIdx as usize].unk_50[k as usize].unk_1C as
                       libc::c_int != 0 as libc::c_int {
                    D_8016E750[playerIdx as usize].unk_50[k as usize].unk_1C =
                        D_8016E750[playerIdx as
                                       usize].unk_50[k as
                                                         usize].unk_1C.wrapping_sub(1);
                    if D_8016E750[playerIdx as
                                      usize].unk_50[k as usize].unk_1C as
                           libc::c_int != 0 as libc::c_int {
                        D_8016E750[playerIdx as
                                       usize].unk_50[k as usize].unk_10 -=
                            D_8016E750[playerIdx as
                                           usize].unk_50[k as usize].unk_18
                    } else {
                        D_8016E750[playerIdx as
                                       usize].unk_50[k as usize].unk_10 =
                            D_8016E750[playerIdx as
                                           usize].unk_50[k as usize].unk_14;
                        D_8016E750[playerIdx as usize].unk_250 =
                            (D_8016E750[playerIdx as usize].unk_250 as
                                 libc::c_int ^
                                 (1 as libc::c_int) << k as libc::c_int) as
                                u16_0
                    }
                    // CHAN_UPD_FREQ_SCALE
                    Audio_QueueCmdF32(0x4000000 as libc::c_int as libc::c_uint
                                          |
                                          (playerIdx as u32_0 &
                                               (((0x1 as libc::c_int) <<
                                                     8 as libc::c_int) -
                                                    1 as libc::c_int) as
                                                   libc::c_uint) <<
                                              16 as libc::c_int |
                                          (k as u32_0 &
                                               (((0x1 as libc::c_int) <<
                                                     8 as libc::c_int) -
                                                    1 as libc::c_int) as
                                                   libc::c_uint) <<
                                              8 as libc::c_int,
                                      D_8016E750[playerIdx as
                                                     usize].unk_50[k as
                                                                       usize].unk_10);
                }
                k = k.wrapping_add(1)
            }
        }
        if D_8016E750[playerIdx as usize].unk_4D as libc::c_int !=
               0 as libc::c_int {
            if func_800FA11C(0xf0000000 as libc::c_uint,
                             0xf0000000 as libc::c_uint) == 0 as libc::c_int {
                D_8016E750[playerIdx as usize].unk_4D =
                    0 as libc::c_int as u8_0;
                return
            }
            if D_8016E750[playerIdx as usize].unk_4C as libc::c_int !=
                   0 as libc::c_int {
                D_8016E750[playerIdx as usize].unk_4C =
                    D_8016E750[playerIdx as usize].unk_4C.wrapping_sub(1)
            } else if !(gAudioContext.seqPlayers[playerIdx as usize].enabled()
                            != 0) {
                j = 0 as libc::c_int as u8_0;
                while (j as libc::c_int) <
                          D_8016E750[playerIdx as usize].unk_4D as libc::c_int
                      {
                    temp_a0 =
                        ((D_8016E750[playerIdx as usize].unk_2C[j as usize] &
                              0xf00000 as libc::c_int as libc::c_uint) >>
                             20 as libc::c_int) as u8_0;
                    temp_s1 =
                        ((D_8016E750[playerIdx as usize].unk_2C[j as usize] &
                              0xf0000 as libc::c_int as libc::c_uint) >>
                             16 as libc::c_int) as u8_0;
                    temp_s0_3 =
                        ((D_8016E750[playerIdx as usize].unk_2C[j as usize] &
                              0xff00 as libc::c_int as libc::c_uint) >>
                             8 as libc::c_int) as u8_0;
                    temp_a3_3 =
                        (D_8016E750[playerIdx as usize].unk_2C[j as usize] &
                             0xff as libc::c_int as libc::c_uint) as u8_0;
                    match temp_a0 as libc::c_int {
                        0 => {
                            Audio_SetVolScale(temp_s1,
                                              1 as libc::c_int as u8_0,
                                              0x7f as libc::c_int as u8_0,
                                              temp_a3_3);
                        }
                        7 => {
                            if D_8016E348[playerIdx as usize] as libc::c_int
                                   == temp_a3_3 as libc::c_int {
                                Audio_SetVolScale(temp_s1,
                                                  1 as libc::c_int as u8_0,
                                                  0x7f as libc::c_int as u8_0,
                                                  temp_s0_3);
                            }
                        }
                        1 => {
                            Audio_QueueSeqCmd((0x30000000 as libc::c_int |
                                                   (playerIdx as libc::c_int)
                                                       << 24 as libc::c_int |
                                                   D_8016E750[playerIdx as
                                                                  usize].unk_254
                                                       as libc::c_int) as
                                                  u32_0);
                        }
                        2 => {
                            Audio_QueueSeqCmd((0 as libc::c_int |
                                                   (temp_s1 as libc::c_int) <<
                                                       24 as libc::c_int |
                                                   (1 as libc::c_int) <<
                                                       16 as libc::c_int |
                                                   D_8016E750[temp_s1 as
                                                                  usize].unk_254
                                                       as libc::c_int) as
                                                  u32_0);
                            D_8016E750[temp_s1 as usize].fadeVolUpdate =
                                1 as libc::c_int as u8_0;
                            D_8016E750[temp_s1 as
                                           usize].volScales[1 as libc::c_int
                                                                as usize] =
                                0x7f as libc::c_int as u8_0
                        }
                        3 => {
                            Audio_QueueSeqCmd(0xb0003000 as libc::c_uint |
                                                  ((temp_s1 as libc::c_int) <<
                                                       24 as libc::c_int) as
                                                      libc::c_uint |
                                                  ((temp_s0_3 as libc::c_int)
                                                       << 16 as libc::c_int)
                                                      as libc::c_uint |
                                                  temp_a3_3 as libc::c_uint);
                        }
                        4 => {
                            Audio_QueueSeqCmd(0xb0004000 as libc::c_uint |
                                                  ((temp_s1 as libc::c_int) <<
                                                       24 as libc::c_int) as
                                                      libc::c_uint |
                                                  ((temp_a3_3 as libc::c_int)
                                                       << 16 as libc::c_int)
                                                      as libc::c_uint |
                                                  0 as libc::c_int as
                                                      libc::c_uint);
                        }
                        5 => {
                            temp_v1 =
                                (D_8016E750[playerIdx as
                                                usize].unk_2C[j as usize] &
                                     0xffff as libc::c_int as libc::c_uint) as
                                    u16_0;
                            Audio_QueueSeqCmd((0 as libc::c_int |
                                                   (temp_s1 as libc::c_int) <<
                                                       24 as libc::c_int |
                                                   (D_8016E750[temp_s1 as
                                                                   usize].unk_4E
                                                        as libc::c_int) <<
                                                       16 as libc::c_int |
                                                   temp_v1 as libc::c_int) as
                                                  u32_0);
                            Audio_SetVolScale(temp_s1,
                                              1 as libc::c_int as u8_0,
                                              0x7f as libc::c_int as u8_0,
                                              0 as libc::c_int as u8_0);
                            D_8016E750[temp_s1 as usize].unk_4E =
                                0 as libc::c_int as u8_0
                        }
                        6 => {
                            D_8016E750[playerIdx as usize].unk_4E = temp_s0_3
                        }
                        8 => {
                            Audio_SetVolScale(temp_s1, temp_s0_3,
                                              0x7f as libc::c_int as u8_0,
                                              temp_a3_3);
                        }
                        14 => {
                            if temp_a3_3 as libc::c_int & 1 as libc::c_int !=
                                   0 {
                                Audio_QueueCmdS32(0xe3000000 as libc::c_uint,
                                                  SEQUENCE_TABLE as
                                                      libc::c_int);
                            }
                            if temp_a3_3 as libc::c_int & 2 as libc::c_int !=
                                   0 {
                                Audio_QueueCmdS32(0xe3000000 as libc::c_uint,
                                                  FONT_TABLE as libc::c_int);
                            }
                            if temp_a3_3 as libc::c_int & 4 as libc::c_int !=
                                   0 {
                                Audio_QueueCmdS32(0xe3000000 as libc::c_uint,
                                                  SAMPLE_TABLE as
                                                      libc::c_int);
                            }
                        }
                        9 => {
                            temp_v1 =
                                (D_8016E750[playerIdx as
                                                usize].unk_2C[j as usize] &
                                     0xffff as libc::c_int as libc::c_uint) as
                                    u16_0;
                            Audio_QueueSeqCmd(0xa0000000 as libc::c_uint |
                                                  ((temp_s1 as libc::c_int) <<
                                                       24 as libc::c_int) as
                                                      libc::c_uint |
                                                  temp_v1 as libc::c_uint);
                        }
                        10 => {
                            Audio_QueueSeqCmd((0x50000000 as libc::c_int |
                                                   (temp_s1 as libc::c_int) <<
                                                       24 as libc::c_int |
                                                   (temp_s0_3 as libc::c_int)
                                                       << 16 as libc::c_int |
                                                   temp_a3_3 as libc::c_int *
                                                       10 as libc::c_int &
                                                       0xffff as libc::c_int)
                                                  as u32_0);
                        }
                        _ => { }
                    }
                    j = j.wrapping_add(1)
                }
                D_8016E750[playerIdx as usize].unk_4D =
                    0 as libc::c_int as u8_0
            }
        }
        playerIdx = playerIdx.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800FAD34() -> u8_0 {
    if D_80133418 as libc::c_int != 0 as libc::c_int {
        if D_80133418 as libc::c_int == 1 as libc::c_int {
            if func_800E5EDC() == 1 as libc::c_int {
                D_80133418 = 0 as libc::c_int as u8_0;
                Audio_QueueCmdS8(0x46020000 as libc::c_int as u32_0,
                                 gSfxChannelLayout as s8);
                func_800F7170();
            }
        } else if D_80133418 as libc::c_int == 2 as libc::c_int {
            while func_800E5EDC() != 1 as libc::c_int { }
            D_80133418 = 0 as libc::c_int as u8_0;
            Audio_QueueCmdS8(0x46020000 as libc::c_int as u32_0,
                             gSfxChannelLayout as s8);
            func_800F7170();
        }
    }
    return D_80133418;
}
#[no_mangle]
pub unsafe extern "C" fn func_800FADF8() {
    let mut playerIdx: u8_0 = 0;
    let mut j: u8_0 = 0;
    playerIdx = 0 as libc::c_int as u8_0;
    while (playerIdx as libc::c_int) < 4 as libc::c_int {
        D_8016E348[playerIdx as usize] = 0 as libc::c_int as u8_0;
        D_8016E750[playerIdx as usize].unk_254 =
            0xffff as libc::c_int as u16_0;
        D_8016E750[playerIdx as usize].unk_256 =
            0xffff as libc::c_int as u16_0;
        D_8016E750[playerIdx as usize].unk_28 = 0 as libc::c_int as u16_0;
        D_8016E750[playerIdx as usize].unk_18 = 0 as libc::c_int as u16_0;
        D_8016E750[playerIdx as usize].unk_14 = 0 as libc::c_int as u32_0;
        D_8016E750[playerIdx as usize].unk_258 = 0 as libc::c_int as u16_0;
        D_8016E750[playerIdx as usize].unk_4D = 0 as libc::c_int as u8_0;
        D_8016E750[playerIdx as usize].unk_4E = 0 as libc::c_int as u8_0;
        D_8016E750[playerIdx as usize].unk_250 = 0 as libc::c_int as u16_0;
        D_8016E750[playerIdx as usize].unk_252 = 0 as libc::c_int as u16_0;
        j = 0 as libc::c_int as u8_0;
        while (j as libc::c_int) < 4 as libc::c_int {
            D_8016E750[playerIdx as usize].volScales[j as usize] =
                0x7f as libc::c_int as u8_0;
            j = j.wrapping_add(1)
        }
        D_8016E750[playerIdx as usize].volFadeTimer =
            1 as libc::c_int as u8_0;
        D_8016E750[playerIdx as usize].fadeVolUpdate =
            1 as libc::c_int as u8_0;
        playerIdx = playerIdx.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800FAEB4() {
    let mut playerIdx: u8_0 = 0;
    let mut j: u8_0 = 0;
    playerIdx = 0 as libc::c_int as u8_0;
    while (playerIdx as libc::c_int) < 4 as libc::c_int {
        D_8016E750[playerIdx as usize].volCur = 1.0f32;
        D_8016E750[playerIdx as usize].unk_0C = 0 as libc::c_int as u16_0;
        D_8016E750[playerIdx as usize].fadeVolUpdate =
            0 as libc::c_int as u8_0;
        j = 0 as libc::c_int as u8_0;
        while (j as libc::c_int) < 4 as libc::c_int {
            D_8016E750[playerIdx as usize].volScales[j as usize] =
                0x7f as libc::c_int as u8_0;
            j = j.wrapping_add(1)
        }
        playerIdx = playerIdx.wrapping_add(1)
    }
    func_800FADF8();
}
