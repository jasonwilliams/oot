#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, ptr_wrapping_offset_from,
           register_tool)]
extern "C" {
    #[no_mangle]
    fn AudioLoad_DmaSampleData(devAddr: u32_0, size: u32_0, arg2: s32,
                               dmaIndexRef: *mut u8_0, medium: s32)
     -> *mut libc::c_void;
    #[no_mangle]
    fn Audio_InvalDCache(buf: *mut libc::c_void, size: s32);
    #[no_mangle]
    fn AudioSeq_ProcessSequences(arg0: s32);
    #[no_mangle]
    static mut gAudioContext: AudioContext;
    #[no_mangle]
    static mut D_8012FBA8: [s16; 0];
    #[no_mangle]
    static mut gWaveSamples: [*mut s16; 9];
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
pub const MEDIUM_DISK_DRIVE: C2RustUnnamed_0 = 3;
pub const MEDIUM_CART: C2RustUnnamed_0 = 2;
pub const MEDIUM_UNK: C2RustUnnamed_0 = 1;
pub const MEDIUM_RAM: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const CODEC_S16: C2RustUnnamed_1 = 5;
pub const CODEC_REVERB: C2RustUnnamed_1 = 4;
pub const CODEC_SMALL_ADPCM: C2RustUnnamed_1 = 3;
pub const CODEC_S16_INMEMORY: C2RustUnnamed_1 = 2;
pub const CODEC_S8: C2RustUnnamed_1 = 1;
pub const CODEC_ADPCM: C2RustUnnamed_1 = 0;
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
#[no_mangle]
pub static mut D_801304A0: u32_0 = 0x13000000 as libc::c_int as u32_0;
#[no_mangle]
pub static mut D_801304A4: u32_0 = 0x5caec8e2 as libc::c_int as u32_0;
#[no_mangle]
pub static mut D_801304A8: u32_0 = 0x945cc8e2 as libc::c_uint;
#[no_mangle]
pub static mut D_801304AC: u32_0 = 0x94aec8e2 as libc::c_uint;
#[no_mangle]
pub static mut D_801304B0: [u16_0; 8] =
    [0x7fff as libc::c_int as u16_0, 0xd001 as libc::c_int as u16_0,
     0x3fff as libc::c_int as u16_0, 0xf001 as libc::c_int as u16_0,
     0x5fff as libc::c_int as u16_0, 0x9001 as libc::c_int as u16_0,
     0x7fff as libc::c_int as u16_0, 0x8001 as libc::c_int as u16_0];
#[no_mangle]
pub static mut D_801304C0: [u8_0; 4] =
    [0x40 as libc::c_int as u8_0, 0x20 as libc::c_int as u8_0,
     0x10 as libc::c_int as u8_0, 0x8 as libc::c_int as u8_0];
#[no_mangle]
pub unsafe extern "C" fn AudioSynth_InitNextRingBuf(mut chunkLen: s32,
                                                    mut bufIndex: s32,
                                                    mut reverbIndex: s32) {
    let mut bufItem: *mut ReverbRingBufferItem =
        0 as *mut ReverbRingBufferItem;
    let mut pad: [s32; 3] = [0; 3];
    let mut reverb: *mut SynthesisReverb =
        &mut *gAudioContext.synthesisReverbs.as_mut_ptr().offset(reverbIndex
                                                                     as isize)
            as *mut SynthesisReverb;
    let mut temp_a0_2: s32 = 0;
    let mut temp_a0_4: s32 = 0;
    let mut sampleCnt: s32 = 0;
    let mut extraSamples: s32 = 0;
    let mut i: s32 = 0;
    let mut j: s32 = 0;
    if (*reverb).downsampleRate as libc::c_int >= 2 as libc::c_int {
        if (*reverb).framesToIgnore as libc::c_int == 0 as libc::c_int {
            bufItem =
                &mut *(*(*reverb).items.as_mut_ptr().offset((*reverb).curFrame
                                                                as
                                                                isize)).as_mut_ptr().offset(bufIndex
                                                                                                as
                                                                                                isize)
                    as *mut ReverbRingBufferItem;
            Audio_InvalDCache((*bufItem).toDownsampleLeft as
                                  *mut libc::c_void, 0x340 as libc::c_int);
            j = 0 as libc::c_int;
            i = 0 as libc::c_int;
            while i < (*bufItem).lengthA as libc::c_int / 2 as libc::c_int {
                *(*reverb).leftRingBuf.offset(((*bufItem).startPos + i) as
                                                  isize) =
                    *(*bufItem).toDownsampleLeft.offset(j as isize);
                *(*reverb).rightRingBuf.offset(((*bufItem).startPos + i) as
                                                   isize) =
                    *(*bufItem).toDownsampleRight.offset(j as isize);
                j += (*reverb).downsampleRate as libc::c_int;
                i += 1
            }
            i = 0 as libc::c_int;
            while i < (*bufItem).lengthB as libc::c_int / 2 as libc::c_int {
                *(*reverb).leftRingBuf.offset(i as isize) =
                    *(*bufItem).toDownsampleLeft.offset(j as isize);
                *(*reverb).rightRingBuf.offset(i as isize) =
                    *(*bufItem).toDownsampleRight.offset(j as isize);
                j += (*reverb).downsampleRate as libc::c_int;
                i += 1
            }
        }
    }
    bufItem =
        &mut *(*(*reverb).items.as_mut_ptr().offset((*reverb).curFrame as
                                                        isize)).as_mut_ptr().offset(bufIndex
                                                                                        as
                                                                                        isize)
            as *mut ReverbRingBufferItem;
    sampleCnt = chunkLen / (*reverb).downsampleRate as libc::c_int;
    extraSamples =
        sampleCnt + (*reverb).nextRingBufPos - (*reverb).bufSizePerChan;
    temp_a0_2 = (*reverb).nextRingBufPos;
    if extraSamples < 0 as libc::c_int {
        (*bufItem).lengthA = (sampleCnt * 2 as libc::c_int) as s16;
        (*bufItem).lengthB = 0 as libc::c_int as s16;
        (*bufItem).startPos = (*reverb).nextRingBufPos;
        (*reverb).nextRingBufPos += sampleCnt
    } else {
        (*bufItem).lengthA =
            ((sampleCnt - extraSamples) * 2 as libc::c_int) as s16;
        (*bufItem).lengthB = (extraSamples * 2 as libc::c_int) as s16;
        (*bufItem).startPos = (*reverb).nextRingBufPos;
        (*reverb).nextRingBufPos = extraSamples
    }
    (*bufItem).numSamplesAfterDownsampling = sampleCnt as s16;
    (*bufItem).chunkLen = chunkLen as s16;
    if (*reverb).unk_14 as libc::c_int != 0 as libc::c_int {
        temp_a0_4 = (*reverb).unk_14 as libc::c_int + temp_a0_2;
        if temp_a0_4 >= (*reverb).bufSizePerChan {
            temp_a0_4 -= (*reverb).bufSizePerChan
        }
        bufItem =
            &mut *(*(*reverb).items2.as_mut_ptr().offset((*reverb).curFrame as
                                                             isize)).as_mut_ptr().offset(bufIndex
                                                                                             as
                                                                                             isize)
                as *mut ReverbRingBufferItem;
        sampleCnt = chunkLen / (*reverb).downsampleRate as libc::c_int;
        extraSamples = temp_a0_4 + sampleCnt - (*reverb).bufSizePerChan;
        if extraSamples < 0 as libc::c_int {
            (*bufItem).lengthA = (sampleCnt * 2 as libc::c_int) as s16;
            (*bufItem).lengthB = 0 as libc::c_int as s16;
            (*bufItem).startPos = temp_a0_4
        } else {
            (*bufItem).lengthA =
                ((sampleCnt - extraSamples) * 2 as libc::c_int) as s16;
            (*bufItem).lengthB = (extraSamples * 2 as libc::c_int) as s16;
            (*bufItem).startPos = temp_a0_4
        }
        (*bufItem).numSamplesAfterDownsampling = sampleCnt as s16;
        (*bufItem).chunkLen = chunkLen as s16
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_800DB03C(mut arg0: s32) {
    let mut subEu: *mut NoteSubEu = 0 as *mut NoteSubEu;
    let mut subEu2: *mut NoteSubEu = 0 as *mut NoteSubEu;
    let mut baseIndex: s32 = 0;
    let mut i: s32 = 0;
    baseIndex = gAudioContext.numNotes * arg0;
    i = 0 as libc::c_int;
    while i < gAudioContext.numNotes {
        subEu = &mut (*gAudioContext.notes.offset(i as isize)).noteSubEu;
        subEu2 =
            &mut *gAudioContext.noteSubsEu.offset((baseIndex + i) as isize) as
                *mut NoteSubEu;
        if (*subEu).bitField0.enabled() != 0 {
            (*subEu).bitField0.set_needsInit(0 as libc::c_int as u8_0)
        } else { (*subEu2).bitField0.set_enabled(0 as libc::c_int as u8_0) }
        (*subEu).unk_06 = 0 as libc::c_int as u8_0;
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioSynth_Update(mut cmdStart: *mut Acmd,
                                           mut cmdCnt: *mut s32,
                                           mut aiStart: *mut s16,
                                           mut aiBufLen: s32) -> *mut Acmd {
    let mut chunkLen: s32 = 0;
    let mut aiBufP: *mut s16 = 0 as *mut s16;
    let mut cmdP: *mut Acmd = 0 as *mut Acmd;
    let mut i: s32 = 0;
    let mut j: s32 = 0;
    let mut reverb: *mut SynthesisReverb = 0 as *mut SynthesisReverb;
    cmdP = cmdStart;
    i = gAudioContext.audioBufferParameters.updatesPerFrame as s32;
    while i > 0 as libc::c_int {
        AudioSeq_ProcessSequences(i - 1 as libc::c_int);
        func_800DB03C(gAudioContext.audioBufferParameters.updatesPerFrame as
                          libc::c_int - i);
        i -= 1
    }
    aiBufP = aiStart;
    gAudioContext.curLoadedBook = 0 as *mut s16;
    i = gAudioContext.audioBufferParameters.updatesPerFrame as s32;
    while i > 0 as libc::c_int {
        if i == 1 as libc::c_int {
            chunkLen = aiBufLen
        } else if aiBufLen / i >=
                      gAudioContext.audioBufferParameters.samplesPerUpdateMax
                          as libc::c_int {
            chunkLen =
                gAudioContext.audioBufferParameters.samplesPerUpdateMax as s32
        } else if gAudioContext.audioBufferParameters.samplesPerUpdateMin as
                      libc::c_int >= aiBufLen / i {
            chunkLen =
                gAudioContext.audioBufferParameters.samplesPerUpdateMin as s32
        } else {
            chunkLen =
                gAudioContext.audioBufferParameters.samplesPerUpdate as s32
        }
        j = 0 as libc::c_int;
        while j < gAudioContext.numSynthesisReverbs as libc::c_int {
            if gAudioContext.synthesisReverbs[j as usize].useReverb != 0 {
                AudioSynth_InitNextRingBuf(chunkLen,
                                           gAudioContext.audioBufferParameters.updatesPerFrame
                                               as libc::c_int - i, j);
            }
            j += 1
        }
        cmdP =
            AudioSynth_DoOneAudioUpdate(aiBufP, chunkLen, cmdP,
                                        gAudioContext.audioBufferParameters.updatesPerFrame
                                            as libc::c_int - i);
        aiBufLen -= chunkLen;
        aiBufP = aiBufP.offset((chunkLen * 2 as libc::c_int) as isize);
        i -= 1
    }
    j = 0 as libc::c_int;
    while j < gAudioContext.numSynthesisReverbs as libc::c_int {
        if gAudioContext.synthesisReverbs[j as usize].framesToIgnore as
               libc::c_int != 0 as libc::c_int {
            gAudioContext.synthesisReverbs[j as usize].framesToIgnore =
                gAudioContext.synthesisReverbs[j as
                                                   usize].framesToIgnore.wrapping_sub(1)
        }
        gAudioContext.synthesisReverbs[j as usize].curFrame =
            (gAudioContext.synthesisReverbs[j as usize].curFrame as
                 libc::c_int ^ 1 as libc::c_int) as u8_0;
        j += 1
    }
    *cmdCnt = cmdP.wrapping_offset_from(cmdStart) as libc::c_int;
    return cmdP;
}
#[no_mangle]
pub unsafe extern "C" fn func_800DB2C0(mut updateIndexStart: s32,
                                       mut noteIndex: s32) {
    let mut temp_v1: *mut NoteSubEu = 0 as *mut NoteSubEu;
    let mut i: s32 = 0;
    i = updateIndexStart + 1 as libc::c_int;
    while i <
              gAudioContext.audioBufferParameters.updatesPerFrame as
                  libc::c_int {
        temp_v1 =
            &mut *gAudioContext.noteSubsEu.offset((gAudioContext.numNotes * i
                                                       + noteIndex) as isize)
                as *mut NoteSubEu;
        if !((*temp_v1).bitField0.needsInit() == 0) { break ; }
        (*temp_v1).bitField0.set_enabled(0 as libc::c_int as u8_0);
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioSynth_LoadRingBuffer1AtTemp(mut cmd: *mut Acmd,
                                                          mut reverb:
                                                              *mut SynthesisReverb,
                                                          mut bufIndex: s16)
 -> *mut Acmd {
    let mut bufItem: *mut ReverbRingBufferItem =
        &mut *(*(*reverb).items.as_mut_ptr().offset((*reverb).curFrame as
                                                        isize)).as_mut_ptr().offset(bufIndex
                                                                                        as
                                                                                        isize)
            as *mut ReverbRingBufferItem;
    cmd =
        AudioSynth_LoadRingBufferPart(cmd, 0x3e0 as libc::c_int as u16_0,
                                      (*bufItem).startPos as u16_0,
                                      (*bufItem).lengthA as s32, reverb);
    if (*bufItem).lengthB as libc::c_int != 0 as libc::c_int {
        // Ring buffer wrapped
        cmd =
            AudioSynth_LoadRingBufferPart(cmd,
                                          (0x3e0 as libc::c_int +
                                               (*bufItem).lengthA as
                                                   libc::c_int) as u16_0,
                                          0 as libc::c_int as u16_0,
                                          (*bufItem).lengthB as s32, reverb)
    }
    return cmd;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSynth_SaveRingBuffer1AtTemp(mut cmd: *mut Acmd,
                                                          mut reverb:
                                                              *mut SynthesisReverb,
                                                          mut bufIndex: s16)
 -> *mut Acmd {
    let mut bufItem: *mut ReverbRingBufferItem =
        &mut *(*(*reverb).items.as_mut_ptr().offset((*reverb).curFrame as
                                                        isize)).as_mut_ptr().offset(bufIndex
                                                                                        as
                                                                                        isize)
            as *mut ReverbRingBufferItem;
    cmd =
        AudioSynth_SaveRingBufferPart(cmd, 0x3e0 as libc::c_int as u16_0,
                                      (*bufItem).startPos as u16_0,
                                      (*bufItem).lengthA as s32, reverb);
    if (*bufItem).lengthB as libc::c_int != 0 as libc::c_int {
        // Ring buffer wrapped
        cmd =
            AudioSynth_SaveRingBufferPart(cmd,
                                          (0x3e0 as libc::c_int +
                                               (*bufItem).lengthA as
                                                   libc::c_int) as u16_0,
                                          0 as libc::c_int as u16_0,
                                          (*bufItem).lengthB as s32, reverb)
    }
    return cmd;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSynth_LeakReverb(mut cmd: *mut Acmd,
                                               mut reverb:
                                                   *mut SynthesisReverb)
 -> *mut Acmd {
    // Leak some audio from the left reverb channel into the right reverb channel and vice versa (pan)
    let fresh0 = cmd;
    cmd = cmd.offset(1);
    let mut _a: *mut Acmd = fresh0;
    (*_a).words.w0 =
        (10 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0xc80 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 24 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_a).words.w1 =
        (0x720 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 16 as libc::c_int |
            (0x1a0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh1 = cmd;
    cmd = cmd.offset(1);
    let mut _a_0: *mut Acmd = fresh1;
    (*_a_0).words.w0 =
        (12 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x1a as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((*reverb).leakRtl as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_a_0).words.w1 =
        (0xe20 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 16 as libc::c_int |
            (0xc80 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh2 = cmd;
    cmd = cmd.offset(1);
    let mut _a_1: *mut Acmd = fresh2;
    (*_a_1).words.w0 =
        (12 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x1a as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((*reverb).leakLtr as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_a_1).words.w1 =
        (0x720 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 16 as libc::c_int |
            (0xe20 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    return cmd;
}
#[no_mangle]
pub unsafe extern "C" fn func_800DB4E4(mut cmd: *mut Acmd, mut arg1: s32,
                                       mut reverb: *mut SynthesisReverb,
                                       mut arg3: s16) -> *mut Acmd {
    let mut item: *mut ReverbRingBufferItem =
        &mut *(*(*reverb).items.as_mut_ptr().offset((*reverb).curFrame as
                                                        isize)).as_mut_ptr().offset(arg3
                                                                                        as
                                                                                        isize)
            as *mut ReverbRingBufferItem;
    let mut offsetA: s16 = 0;
    let mut offsetB: s16 = 0;
    offsetA =
        (((*item).startPos & 7 as libc::c_int) * 2 as libc::c_int) as s16;
    offsetB =
        (offsetA as libc::c_int + (*item).lengthA as libc::c_int +
             0xf as libc::c_int & !(0xf as libc::c_int)) as s16;
    cmd =
        AudioSynth_LoadRingBufferPart(cmd, 0x3e0 as libc::c_int as u16_0,
                                      ((*item).startPos -
                                           offsetA as libc::c_int /
                                               2 as libc::c_int) as u16_0,
                                      0x1a0 as libc::c_int, reverb);
    if (*item).lengthB as libc::c_int != 0 as libc::c_int {
        // Ring buffer wrapped
        cmd =
            AudioSynth_LoadRingBufferPart(cmd,
                                          (0x3e0 as libc::c_int +
                                               offsetB as libc::c_int) as
                                              u16_0,
                                          0 as libc::c_int as u16_0,
                                          0x1a0 as libc::c_int -
                                              offsetB as libc::c_int, reverb)
    }
    let fresh3 = cmd;
    cmd = cmd.offset(1);
    let mut _a: *mut Acmd = fresh3;
    (*_a).words.w0 =
        (8 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((0x3e0 as libc::c_int + offsetA as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_a).words.w1 =
        (0xc80 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 16 as libc::c_int |
            ((arg1 * 2 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh4 = cmd;
    cmd = cmd.offset(1);
    let mut _a_0: *mut Acmd = fresh4;
    (*_a_0).words.w0 =
        (5 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((*reverb).resampleFlags as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((*reverb).unk_0E as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_a_0).words.w1 = (*reverb).unk_30 as u32_0;
    let fresh5 = cmd;
    cmd = cmd.offset(1);
    let mut _a_1: *mut Acmd = fresh5;
    (*_a_1).words.w0 =
        (8 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((0x3e0 as libc::c_int + 0x1a0 as libc::c_int +
                  offsetA as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_a_1).words.w1 =
        (0xe20 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 16 as libc::c_int |
            ((arg1 * 2 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh6 = cmd;
    cmd = cmd.offset(1);
    let mut _a_2: *mut Acmd = fresh6;
    (*_a_2).words.w0 =
        (5 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((*reverb).resampleFlags as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((*reverb).unk_0E as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_a_2).words.w1 = (*reverb).unk_34 as u32_0;
    return cmd;
}
#[no_mangle]
pub unsafe extern "C" fn func_800DB680(mut cmd: *mut Acmd,
                                       mut reverb: *mut SynthesisReverb,
                                       mut bufIndex: s16) -> *mut Acmd {
    let mut bufItem: *mut ReverbRingBufferItem =
        &mut *(*(*reverb).items.as_mut_ptr().offset((*reverb).curFrame as
                                                        isize)).as_mut_ptr().offset(bufIndex
                                                                                        as
                                                                                        isize)
            as *mut ReverbRingBufferItem;
    let fresh7 = cmd;
    cmd = cmd.offset(1);
    let mut _a: *mut Acmd = fresh7;
    (*_a).words.w0 =
        (8 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0xc80 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_a).words.w1 =
        (0x720 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 16 as libc::c_int |
            (((*bufItem).unk_18 as libc::c_int * 2 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh8 = cmd;
    cmd = cmd.offset(1);
    let mut _a_0: *mut Acmd = fresh8;
    (*_a_0).words.w0 =
        (5 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((*reverb).resampleFlags as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((*bufItem).unk_16 as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_a_0).words.w1 = (*reverb).unk_38 as u32_0;
    cmd =
        AudioSynth_SaveBufferOffset(cmd, 0x720 as libc::c_int as u16_0,
                                    (*bufItem).startPos as u16_0,
                                    (*bufItem).lengthA as s32,
                                    (*reverb).leftRingBuf);
    if (*bufItem).lengthB as libc::c_int != 0 as libc::c_int {
        // Ring buffer wrapped
        cmd =
            AudioSynth_SaveBufferOffset(cmd,
                                        (0x720 as libc::c_int +
                                             (*bufItem).lengthA as
                                                 libc::c_int) as u16_0,
                                        0 as libc::c_int as u16_0,
                                        (*bufItem).lengthB as s32,
                                        (*reverb).leftRingBuf)
    }
    let fresh9 = cmd;
    cmd = cmd.offset(1);
    let mut _a_1: *mut Acmd = fresh9;
    (*_a_1).words.w0 =
        (8 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0xe20 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_a_1).words.w1 =
        (0x720 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 16 as libc::c_int |
            (((*bufItem).unk_18 as libc::c_int * 2 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh10 = cmd;
    cmd = cmd.offset(1);
    let mut _a_2: *mut Acmd = fresh10;
    (*_a_2).words.w0 =
        (5 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((*reverb).resampleFlags as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((*bufItem).unk_16 as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_a_2).words.w1 = (*reverb).unk_3C as u32_0;
    cmd =
        AudioSynth_SaveBufferOffset(cmd, 0x720 as libc::c_int as u16_0,
                                    (*bufItem).startPos as u16_0,
                                    (*bufItem).lengthA as s32,
                                    (*reverb).rightRingBuf);
    if (*bufItem).lengthB as libc::c_int != 0 as libc::c_int {
        // Ring buffer wrapped
        cmd =
            AudioSynth_SaveBufferOffset(cmd,
                                        (0x720 as libc::c_int +
                                             (*bufItem).lengthA as
                                                 libc::c_int) as u16_0,
                                        0 as libc::c_int as u16_0,
                                        (*bufItem).lengthB as s32,
                                        (*reverb).rightRingBuf)
    }
    return cmd;
}
#[no_mangle]
pub unsafe extern "C" fn func_800DB828(mut cmd: *mut Acmd, mut arg1: s32,
                                       mut reverb: *mut SynthesisReverb,
                                       mut arg3: s16) -> *mut Acmd {
    let mut item: *mut ReverbRingBufferItem =
        &mut *(*(*reverb).items.as_mut_ptr().offset((*reverb).curFrame as
                                                        isize)).as_mut_ptr().offset(arg3
                                                                                        as
                                                                                        isize)
            as *mut ReverbRingBufferItem;
    let mut offsetA: s16 = 0;
    let mut offsetB: s16 = 0;
    (*item).unk_14 =
        ((((*item).unk_18 as libc::c_int) << 0xf as libc::c_int) / arg1) as
            u16_0;
    offsetA =
        (((*item).startPos & 7 as libc::c_int) * 2 as libc::c_int) as s16;
    (*item).unk_16 =
        ((arg1 << 0xf as libc::c_int) / (*item).unk_18 as libc::c_int) as
            u16_0;
    offsetB =
        (offsetA as libc::c_int + (*item).lengthA as libc::c_int +
             0xf as libc::c_int & !(0xf as libc::c_int)) as s16;
    cmd =
        AudioSynth_LoadRingBufferPart(cmd, 0x3e0 as libc::c_int as u16_0,
                                      ((*item).startPos -
                                           offsetA as libc::c_int /
                                               2 as libc::c_int) as u16_0,
                                      0x1a0 as libc::c_int, reverb);
    if (*item).lengthB as libc::c_int != 0 as libc::c_int {
        // Ring buffer wrapped
        cmd =
            AudioSynth_LoadRingBufferPart(cmd,
                                          (0x3e0 as libc::c_int +
                                               offsetB as libc::c_int) as
                                              u16_0,
                                          0 as libc::c_int as u16_0,
                                          0x1a0 as libc::c_int -
                                              offsetB as libc::c_int, reverb)
    }
    let fresh11 = cmd;
    cmd = cmd.offset(1);
    let mut _a: *mut Acmd = fresh11;
    (*_a).words.w0 =
        (8 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((0x3e0 as libc::c_int + offsetA as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_a).words.w1 =
        (0xc80 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 16 as libc::c_int |
            ((arg1 * 2 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh12 = cmd;
    cmd = cmd.offset(1);
    let mut _a_0: *mut Acmd = fresh12;
    (*_a_0).words.w0 =
        (5 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((*reverb).resampleFlags as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((*item).unk_14 as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_a_0).words.w1 = (*reverb).unk_30 as u32_0;
    let fresh13 = cmd;
    cmd = cmd.offset(1);
    let mut _a_1: *mut Acmd = fresh13;
    (*_a_1).words.w0 =
        (8 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((0x3e0 as libc::c_int + 0x1a0 as libc::c_int +
                  offsetA as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_a_1).words.w1 =
        (0xe20 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 16 as libc::c_int |
            ((arg1 * 2 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh14 = cmd;
    cmd = cmd.offset(1);
    let mut _a_2: *mut Acmd = fresh14;
    (*_a_2).words.w0 =
        (5 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((*reverb).resampleFlags as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((*item).unk_14 as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_a_2).words.w1 = (*reverb).unk_34 as u32_0;
    return cmd;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSynth_FilterReverb(mut cmd: *mut Acmd,
                                                 mut count: s32,
                                                 mut reverb:
                                                     *mut SynthesisReverb)
 -> *mut Acmd {
    // Apply a filter (convolution) to each reverb channel.
    if !(*reverb).filterLeft.is_null() {
        let fresh15 = cmd;
        cmd = cmd.offset(1);
        let mut _a: *mut Acmd = fresh15;
        (*_a).words.w0 =
            (7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (2 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                (count as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_a).words.w1 = (*reverb).filterLeft as u32_0;
        let fresh16 = cmd;
        cmd = cmd.offset(1);
        let mut _a_0: *mut Acmd = fresh16;
        (*_a_0).words.w0 =
            (7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((*reverb).resampleFlags as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                (0xc80 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_a_0).words.w1 = (*reverb).filterLeftState as u32_0
    }
    if !(*reverb).filterRight.is_null() {
        let fresh17 = cmd;
        cmd = cmd.offset(1);
        let mut _a_1: *mut Acmd = fresh17;
        (*_a_1).words.w0 =
            (7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (2 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                (count as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_a_1).words.w1 = (*reverb).filterRight as u32_0;
        let fresh18 = cmd;
        cmd = cmd.offset(1);
        let mut _a_2: *mut Acmd = fresh18;
        (*_a_2).words.w0 =
            (7 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((*reverb).resampleFlags as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                (0xe20 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_a_2).words.w1 = (*reverb).filterRightState as u32_0
    }
    return cmd;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSynth_MaybeMixRingBuffer1(mut cmd: *mut Acmd,
                                                        mut reverb:
                                                            *mut SynthesisReverb,
                                                        mut arg2: s32)
 -> *mut Acmd {
    let mut temp_a3: *mut SynthesisReverb = 0 as *mut SynthesisReverb;
    temp_a3 =
        &mut *gAudioContext.synthesisReverbs.as_mut_ptr().offset((*reverb).unk_05
                                                                     as isize)
            as *mut SynthesisReverb;
    if (*temp_a3).downsampleRate as libc::c_int == 1 as libc::c_int {
        cmd = AudioSynth_LoadRingBuffer1AtTemp(cmd, temp_a3, arg2 as s16);
        let fresh19 = cmd;
        cmd = cmd.offset(1);
        let mut _a: *mut Acmd = fresh19;
        (*_a).words.w0 =
            (12 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0x34 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                ((*reverb).unk_08 as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_a).words.w1 =
            (0xc80 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
                |
                (0x3e0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        cmd = AudioSynth_SaveRingBuffer1AtTemp(cmd, temp_a3, arg2 as s16)
    }
    return cmd;
}
#[no_mangle]
pub unsafe extern "C" fn func_800DBB94() { }
#[no_mangle]
pub unsafe extern "C" fn AudioSynth_ClearBuffer(mut cmd: *mut Acmd,
                                                mut arg1: s32,
                                                mut arg2: s32) {
    let mut _a: *mut Acmd = cmd;
    (*_a).words.w0 =
        (2 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (arg1 as u32_0 &
                 (((0x1 as libc::c_int) << 24 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_a).words.w1 = arg2 as u32_0;
}
#[no_mangle]
pub unsafe extern "C" fn func_800DBBBC() { }
#[no_mangle]
pub unsafe extern "C" fn func_800DBBC4() { }
#[no_mangle]
pub unsafe extern "C" fn func_800DBBCC() { }
#[no_mangle]
pub unsafe extern "C" fn AudioSynth_Mix(mut cmd: *mut Acmd, mut arg1: s32,
                                        mut arg2: s32, mut arg3: s32,
                                        mut arg4: s32) {
    let mut _a: *mut Acmd = cmd;
    (*_a).words.w0 =
        (12 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (arg1 as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (arg2 as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_a).words.w1 =
        (arg3 as u32_0 &
             (((0x1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 16 as libc::c_int |
            (arg4 as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_800DBC08() { }
#[no_mangle]
pub unsafe extern "C" fn func_800DBC10() { }
#[no_mangle]
pub unsafe extern "C" fn func_800DBC18() { }
#[no_mangle]
pub unsafe extern "C" fn AudioSynth_SetBuffer(mut cmd: *mut Acmd,
                                              mut flags: s32, mut dmemIn: s32,
                                              mut dmemOut: s32,
                                              mut count: s32) {
    let mut _a: *mut Acmd = cmd;
    (*_a).words.w0 =
        (8 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (flags as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (dmemIn as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_a).words.w1 =
        (dmemOut as u32_0 &
             (((0x1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 16 as libc::c_int |
            (count as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_800DBC54() { }
#[no_mangle]
pub unsafe extern "C" fn func_800DBC5C() { }
// possible fake match?
#[no_mangle]
pub unsafe extern "C" fn AudioSynth_DMemMove(mut cmd: *mut Acmd,
                                             mut dmemIn: s32,
                                             mut dmemOut: s32,
                                             mut count: s32) {
    (*cmd).words.w0 =
        (10 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (dmemIn as u32_0 &
                 (((0x1 as libc::c_int) << 24 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*cmd).words.w1 =
        (dmemOut as u32_0 &
             (((0x1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 16 as libc::c_int |
            (count as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_800DBC90() { }
#[no_mangle]
pub unsafe extern "C" fn func_800DBC98() { }
#[no_mangle]
pub unsafe extern "C" fn func_800DBCA0() { }
#[no_mangle]
pub unsafe extern "C" fn func_800DBCA8() { }
#[no_mangle]
pub unsafe extern "C" fn AudioSynth_InterL(mut cmd: *mut Acmd,
                                           mut dmemIn: s32, mut dmemOut: s32,
                                           mut count: s32) {
    (*cmd).words.w0 =
        (17 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (count as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*cmd).words.w1 =
        (dmemIn as u32_0 &
             (((0x1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 16 as libc::c_int |
            (dmemOut as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSynth_EnvSetup1(mut cmd: *mut Acmd,
                                              mut arg1: s32, mut arg2: s32,
                                              mut arg3: s32, mut arg4: s32) {
    let mut _a: *mut Acmd = cmd;
    (*_a).words.w0 =
        (18 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (arg1 as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (arg2 as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_a).words.w1 =
        (arg3 as u32_0 &
             (((0x1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 16 as libc::c_int |
            (arg4 as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_800DBD08() { }
#[no_mangle]
pub unsafe extern "C" fn AudioSynth_LoadBuffer(mut cmd: *mut Acmd,
                                               mut arg1: s32, mut arg2: s32,
                                               mut arg3: s32) {
    let mut _a: *mut Acmd = cmd;
    (*_a).words.w0 =
        (20 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((arg2 >> 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (arg1 as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_a).words.w1 = arg3 as u32_0;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSynth_SaveBuffer(mut cmd: *mut Acmd,
                                               mut arg1: s32, mut arg2: s32,
                                               mut arg3: s32) {
    let mut _a: *mut Acmd = cmd;
    (*_a).words.w0 =
        (21 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((arg2 >> 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (arg1 as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_a).words.w1 = arg3 as u32_0;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSynth_EnvSetup2(mut cmd: *mut Acmd,
                                              mut volLeft: s32,
                                              mut volRight: s32) {
    (*cmd).words.w0 =
        (22 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int;
    (*cmd).words.w1 =
        (volLeft as u32_0 &
             (((0x1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 16 as libc::c_int |
            (volRight as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_800DBD7C() { }
#[no_mangle]
pub unsafe extern "C" fn func_800DBD84() { }
#[no_mangle]
pub unsafe extern "C" fn func_800DBD8C() { }
#[no_mangle]
pub unsafe extern "C" fn AudioSynth_S8Dec(mut cmd: *mut Acmd, mut flags: s32,
                                          mut state: *mut s16) {
    let mut _a: *mut Acmd = cmd;
    (*_a).words.w0 =
        (23 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (flags as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int;
    (*_a).words.w1 = state as u32_0;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSynth_HiLoGain(mut cmd: *mut Acmd,
                                             mut gain: s32, mut dmemIn: s32,
                                             mut dmemOut: s32,
                                             mut count: s32) {
    (*cmd).words.w0 =
        (14 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (gain as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (count as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*cmd).words.w1 =
        (dmemIn as u32_0 &
             (((0x1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 16 as libc::c_int |
            (dmemOut as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSynth_UnkCmd19(mut cmd: *mut Acmd,
                                             mut arg1: s32, mut arg2: s32,
                                             mut arg3: s32, mut arg4: s32) {
    (*cmd).words.w0 =
        (25 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (arg4 as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (arg3 as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*cmd).words.w1 =
        (arg1 as u32_0 &
             (((0x1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 16 as libc::c_int |
            (arg2 as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_800DBE18() { }
#[no_mangle]
pub unsafe extern "C" fn func_800DBE20() { }
#[no_mangle]
pub unsafe extern "C" fn func_800DBE28() { }
#[no_mangle]
pub unsafe extern "C" fn func_800DBE30() { }
#[no_mangle]
pub unsafe extern "C" fn AudioSynth_UnkCmd3(mut cmd: *mut Acmd, mut arg1: s32,
                                            mut arg2: s32, mut arg3: s32) {
    (*cmd).words.w0 =
        (3 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (arg3 as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*cmd).words.w1 =
        (arg1 as u32_0 &
             (((0x1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 16 as libc::c_int |
            (arg2 as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_800DBE5C() { }
#[no_mangle]
pub unsafe extern "C" fn func_800DBE64() { }
#[no_mangle]
pub unsafe extern "C" fn func_800DBE6C() { }
#[no_mangle]
pub unsafe extern "C" fn AudioSynth_LoadFilter(mut cmd: *mut Acmd,
                                               mut flags: s32,
                                               mut countOrBuf: s32,
                                               mut addr: s32) {
    let mut _a: *mut Acmd = cmd;
    (*_a).words.w0 =
        (7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (flags as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (countOrBuf as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_a).words.w1 = addr as u32_0;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSynth_LoadFilterCount(mut cmd: *mut Acmd,
                                                    mut count: s32,
                                                    mut addr: s32) {
    let mut _a: *mut Acmd = cmd;
    (*_a).words.w0 =
        (7 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (count as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_a).words.w1 = addr as u32_0;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSynth_LoadRingBuffer1(mut cmd: *mut Acmd,
                                                    mut arg1: s32,
                                                    mut reverb:
                                                        *mut SynthesisReverb,
                                                    mut bufIndex: s16)
 -> *mut Acmd {
    let mut ringBufferItem: *mut ReverbRingBufferItem =
        &mut *(*(*reverb).items.as_mut_ptr().offset((*reverb).curFrame as
                                                        isize)).as_mut_ptr().offset(bufIndex
                                                                                        as
                                                                                        isize)
            as *mut ReverbRingBufferItem;
    cmd =
        AudioSynth_LoadRingBufferPart(cmd, 0xc80 as libc::c_int as u16_0,
                                      (*ringBufferItem).startPos as u16_0,
                                      (*ringBufferItem).lengthA as s32,
                                      reverb);
    if (*ringBufferItem).lengthB as libc::c_int != 0 as libc::c_int {
        // Ring buffer wrapped
        cmd =
            AudioSynth_LoadRingBufferPart(cmd,
                                          (0xc80 as libc::c_int +
                                               (*ringBufferItem).lengthA as
                                                   libc::c_int) as u16_0,
                                          0 as libc::c_int as u16_0,
                                          (*ringBufferItem).lengthB as s32,
                                          reverb)
    }
    return cmd;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSynth_LoadRingBuffer2(mut cmd: *mut Acmd,
                                                    mut arg1: s32,
                                                    mut reverb:
                                                        *mut SynthesisReverb,
                                                    mut bufIndex: s16)
 -> *mut Acmd {
    let mut bufItem: *mut ReverbRingBufferItem =
        &mut *(*(*reverb).items2.as_mut_ptr().offset((*reverb).curFrame as
                                                         isize)).as_mut_ptr().offset(bufIndex
                                                                                         as
                                                                                         isize)
            as *mut ReverbRingBufferItem;
    cmd =
        AudioSynth_LoadRingBufferPart(cmd, 0xc80 as libc::c_int as u16_0,
                                      (*bufItem).startPos as u16_0,
                                      (*bufItem).lengthA as s32, reverb);
    if (*bufItem).lengthB as libc::c_int != 0 as libc::c_int {
        // Ring buffer wrapped
        cmd =
            AudioSynth_LoadRingBufferPart(cmd,
                                          (0xc80 as libc::c_int +
                                               (*bufItem).lengthA as
                                                   libc::c_int) as u16_0,
                                          0 as libc::c_int as u16_0,
                                          (*bufItem).lengthB as s32, reverb)
    }
    return cmd;
}
// = DMEM_WET_LEFT_CH + DEFAULT_LEN_1CH
#[no_mangle]
pub unsafe extern "C" fn AudioSynth_LoadRingBufferPart(mut cmd: *mut Acmd,
                                                       mut dmem: u16_0,
                                                       mut startPos: u16_0,
                                                       mut length: s32,
                                                       mut reverb:
                                                           *mut SynthesisReverb)
 -> *mut Acmd {
    let fresh20 = cmd;
    cmd = cmd.offset(1);
    let mut _a: *mut Acmd = fresh20;
    (*_a).words.w0 =
        (20 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((length >> 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (dmem as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_a).words.w1 =
        &mut *(*reverb).leftRingBuf.offset(startPos as isize) as *mut s16 as
            u32_0;
    let fresh21 = cmd;
    cmd = cmd.offset(1);
    let mut _a_0: *mut Acmd = fresh21;
    (*_a_0).words.w0 =
        (20 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((length >> 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((dmem as libc::c_int + 0x1a0 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_a_0).words.w1 =
        &mut *(*reverb).rightRingBuf.offset(startPos as isize) as *mut s16 as
            u32_0;
    return cmd;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSynth_SaveRingBufferPart(mut cmd: *mut Acmd,
                                                       mut dmem: u16_0,
                                                       mut startPos: u16_0,
                                                       mut length: s32,
                                                       mut reverb:
                                                           *mut SynthesisReverb)
 -> *mut Acmd {
    let fresh22 = cmd;
    cmd = cmd.offset(1);
    let mut _a: *mut Acmd = fresh22;
    (*_a).words.w0 =
        (21 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((length >> 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (dmem as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_a).words.w1 =
        &mut *(*reverb).leftRingBuf.offset(startPos as isize) as *mut s16 as
            u32_0;
    let fresh23 = cmd;
    cmd = cmd.offset(1);
    let mut _a_0: *mut Acmd = fresh23;
    (*_a_0).words.w0 =
        (21 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((length >> 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            ((dmem as libc::c_int + 0x1a0 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_a_0).words.w1 =
        &mut *(*reverb).rightRingBuf.offset(startPos as isize) as *mut s16 as
            u32_0;
    return cmd;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSynth_SaveBufferOffset(mut cmd: *mut Acmd,
                                                     mut dmem: u16_0,
                                                     mut offset: u16_0,
                                                     mut length: s32,
                                                     mut buf: *mut s16)
 -> *mut Acmd {
    let fresh24 = cmd;
    cmd = cmd.offset(1);
    let mut _a: *mut Acmd = fresh24;
    (*_a).words.w0 =
        (21 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((length >> 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (dmem as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_a).words.w1 = &mut *buf.offset(offset as isize) as *mut s16 as u32_0;
    return cmd;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSynth_MaybeLoadRingBuffer2(mut cmd: *mut Acmd,
                                                         mut arg1: s32,
                                                         mut reverb:
                                                             *mut SynthesisReverb,
                                                         mut bufIndex: s16)
 -> *mut Acmd {
    if (*reverb).downsampleRate as libc::c_int == 1 as libc::c_int {
        cmd = AudioSynth_LoadRingBuffer2(cmd, arg1, reverb, bufIndex)
    }
    return cmd;
}
#[no_mangle]
pub unsafe extern "C" fn func_800DC164(mut cmd: *mut Acmd, mut arg1: s32,
                                       mut reverb: *mut SynthesisReverb,
                                       mut arg3: s16) -> *mut Acmd {
    // Sets DMEM_WET_{LEFT,RIGHT}_CH, clobbers DMEM_TEMP
    if (*reverb).downsampleRate as libc::c_int == 1 as libc::c_int {
        if (*reverb).unk_18 as libc::c_int != 0 as libc::c_int {
            cmd = func_800DB828(cmd, arg1, reverb, arg3)
        } else { cmd = AudioSynth_LoadRingBuffer1(cmd, arg1, reverb, arg3) }
    } else { cmd = func_800DB4E4(cmd, arg1, reverb, arg3) }
    return cmd;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSynth_SaveReverbSamples(mut cmd: *mut Acmd,
                                                      mut reverb:
                                                          *mut SynthesisReverb,
                                                      mut bufIndex: s16)
 -> *mut Acmd {
    let mut bufItem: *mut ReverbRingBufferItem =
        &mut *(*(*reverb).items.as_mut_ptr().offset((*reverb).curFrame as
                                                        isize)).as_mut_ptr().offset(bufIndex
                                                                                        as
                                                                                        isize)
            as *mut ReverbRingBufferItem;
    if (*reverb).downsampleRate as libc::c_int == 1 as libc::c_int {
        if (*reverb).unk_18 as libc::c_int != 0 as libc::c_int {
            cmd = func_800DB680(cmd, reverb, bufIndex)
        } else {
            // Put the oldest samples in the ring buffer into the wet channels
            cmd =
                AudioSynth_SaveRingBufferPart(cmd,
                                              0xc80 as libc::c_int as u16_0,
                                              (*bufItem).startPos as u16_0,
                                              (*bufItem).lengthA as s32,
                                              reverb);
            if (*bufItem).lengthB as libc::c_int != 0 as libc::c_int {
                // Ring buffer wrapped
                cmd =
                    AudioSynth_SaveRingBufferPart(cmd,
                                                  (0xc80 as libc::c_int +
                                                       (*bufItem).lengthA as
                                                           libc::c_int) as
                                                      u16_0,
                                                  0 as libc::c_int as u16_0,
                                                  (*bufItem).lengthB as s32,
                                                  reverb)
            }
        }
    } else {
        // Downsampling is done later by CPU when RSP is done, therefore we need to have
        // double buffering. Left and right buffers are adjacent in memory.
        let fresh25 = cmd;
        cmd = cmd.offset(1);
        AudioSynth_SaveBuffer(fresh25, 0xc80 as libc::c_int,
                              0x340 as libc::c_int,
                              (*reverb).items[(*reverb).curFrame as
                                                  usize][bufIndex as
                                                             usize].toDownsampleLeft
                                  as s32);
    }
    (*reverb).resampleFlags = 0 as libc::c_int as u8_0;
    return cmd;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSynth_SaveRingBuffer2(mut cmd: *mut Acmd,
                                                    mut reverb:
                                                        *mut SynthesisReverb,
                                                    mut bufIndex: s16)
 -> *mut Acmd {
    let mut bufItem: *mut ReverbRingBufferItem =
        &mut *(*(*reverb).items2.as_mut_ptr().offset((*reverb).curFrame as
                                                         isize)).as_mut_ptr().offset(bufIndex
                                                                                         as
                                                                                         isize)
            as *mut ReverbRingBufferItem;
    cmd =
        AudioSynth_SaveRingBufferPart(cmd, 0xc80 as libc::c_int as u16_0,
                                      (*bufItem).startPos as u16_0,
                                      (*bufItem).lengthA as s32, reverb);
    if (*bufItem).lengthB as libc::c_int != 0 as libc::c_int {
        // Ring buffer wrapped
        cmd =
            AudioSynth_SaveRingBufferPart(cmd,
                                          (0xc80 as libc::c_int +
                                               (*bufItem).lengthA as
                                                   libc::c_int) as u16_0,
                                          0 as libc::c_int as u16_0,
                                          (*bufItem).lengthB as s32, reverb)
    }
    return cmd;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSynth_DoOneAudioUpdate(mut aiBuf: *mut s16,
                                                     mut aiBufLen: s32,
                                                     mut cmd: *mut Acmd,
                                                     mut updateIndex: s32)
 -> *mut Acmd {
    let mut noteIndices: [u8_0; 92] = [0; 92];
    let mut count: s16 = 0;
    let mut reverbIndex: s16 = 0;
    let mut reverb: *mut SynthesisReverb = 0 as *mut SynthesisReverb;
    let mut useReverb: s32 = 0;
    let mut t: s32 = 0;
    let mut i: s32 = 0;
    let mut noteSubEu: *mut NoteSubEu = 0 as *mut NoteSubEu;
    let mut noteSubEu2: *mut NoteSubEu = 0 as *mut NoteSubEu;
    let mut unk14: s32 = 0;
    t = gAudioContext.numNotes * updateIndex;
    count = 0 as libc::c_int as s16;
    if gAudioContext.numSynthesisReverbs as libc::c_int == 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < gAudioContext.numNotes {
            if (*gAudioContext.noteSubsEu.offset((t + i) as
                                                     isize)).bitField0.enabled()
                   != 0 {
                let fresh26 = count;
                count = count + 1;
                noteIndices[fresh26 as usize] = i as u8_0
            }
            i += 1
        }
    } else {
        reverbIndex = 0 as libc::c_int as s16;
        while (reverbIndex as libc::c_int) <
                  gAudioContext.numSynthesisReverbs as libc::c_int {
            i = 0 as libc::c_int;
            while i < gAudioContext.numNotes {
                noteSubEu =
                    &mut *gAudioContext.noteSubsEu.offset((t + i) as isize) as
                        *mut NoteSubEu;
                if (*noteSubEu).bitField0.enabled() as libc::c_int != 0 &&
                       (*noteSubEu).bitField1.reverbIndex() as libc::c_int ==
                           reverbIndex as libc::c_int {
                    let fresh27 = count;
                    count = count + 1;
                    noteIndices[fresh27 as usize] = i as u8_0
                }
                i += 1
            }
            reverbIndex += 1
        }
        i = 0 as libc::c_int;
        while i < gAudioContext.numNotes {
            noteSubEu =
                &mut *gAudioContext.noteSubsEu.offset((t + i) as isize) as
                    *mut NoteSubEu;
            if (*noteSubEu).bitField0.enabled() as libc::c_int != 0 &&
                   (*noteSubEu).bitField1.reverbIndex() as libc::c_int >=
                       gAudioContext.numSynthesisReverbs as libc::c_int {
                let fresh28 = count;
                count = count + 1;
                noteIndices[fresh28 as usize] = i as u8_0
            }
            i += 1
        }
    }
    let fresh29 = cmd;
    cmd = cmd.offset(1);
    let mut _a: *mut Acmd = fresh29;
    (*_a).words.w0 =
        (2 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (0x940 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 24 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_a).words.w1 = 0x340 as libc::c_int as u32_0;
    i = 0 as libc::c_int;
    reverbIndex = 0 as libc::c_int as s16;
    while (reverbIndex as libc::c_int) <
              gAudioContext.numSynthesisReverbs as libc::c_int {
        reverb =
            &mut *gAudioContext.synthesisReverbs.as_mut_ptr().offset(reverbIndex
                                                                         as
                                                                         isize)
                as *mut SynthesisReverb;
        useReverb = (*reverb).useReverb as s32;
        if useReverb != 0 {
            cmd = func_800DC164(cmd, aiBufLen, reverb, updateIndex as s16);
            let fresh30 = cmd;
            cmd = cmd.offset(1);
            let mut _a_0: *mut Acmd = fresh30;
            (*_a_0).words.w0 =
                (12 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0x34 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    ((*reverb).unk_0A as u32_0 &
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_a_0).words.w1 =
                (0xc80 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                    (0x940 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            unk14 = (*reverb).unk_14 as s32;
            if unk14 != 0 {
                let fresh31 = cmd;
                cmd = cmd.offset(1);
                let mut _a_1: *mut Acmd = fresh31;
                (*_a_1).words.w0 =
                    (10 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (0xc80 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 24 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                (*_a_1).words.w1 =
                    (0x3e0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                        (0x340 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int
            }
            let fresh32 = cmd;
            cmd = cmd.offset(1);
            let mut _a_2: *mut Acmd = fresh32;
            (*_a_2).words.w0 =
                (12 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0x34 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (((*reverb).unk_0C as libc::c_int + 0x8000 as libc::c_int)
                         as u32_0 &
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_a_2).words.w1 =
                (0xc80 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                    (0xc80 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            if (*reverb).leakRtl as libc::c_int != 0 as libc::c_int ||
                   (*reverb).leakLtr as libc::c_int != 0 as libc::c_int {
                cmd = AudioSynth_LeakReverb(cmd, reverb)
            }
            if unk14 != 0 {
                cmd =
                    AudioSynth_SaveReverbSamples(cmd, reverb,
                                                 updateIndex as s16);
                if (*reverb).unk_05 as libc::c_int != -(1 as libc::c_int) {
                    cmd =
                        AudioSynth_MaybeMixRingBuffer1(cmd, reverb,
                                                       updateIndex)
                }
                cmd =
                    AudioSynth_MaybeLoadRingBuffer2(cmd, aiBufLen, reverb,
                                                    updateIndex as s16);
                let fresh33 = cmd;
                cmd = cmd.offset(1);
                let mut _a_3: *mut Acmd = fresh33;
                (*_a_3).words.w0 =
                    (12 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (0x34 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            16 as libc::c_int |
                        ((*reverb).unk_16 as u32_0 &
                             (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                (*_a_3).words.w1 =
                    (0x3e0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                        (0xc80 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int
            }
        }
        while i < count as libc::c_int {
            noteSubEu2 =
                &mut *gAudioContext.noteSubsEu.offset((*noteIndices.as_mut_ptr().offset(i
                                                                                            as
                                                                                            isize)
                                                           as libc::c_int + t)
                                                          as isize) as
                    *mut NoteSubEu;
            if !((*noteSubEu2).bitField1.reverbIndex() as libc::c_int ==
                     reverbIndex as libc::c_int) {
                break ;
            }
            cmd =
                AudioSynth_ProcessNote(noteIndices[i as usize] as s32,
                                       noteSubEu2,
                                       &mut (*gAudioContext.notes.offset(*noteIndices.as_mut_ptr().offset(i
                                                                                                              as
                                                                                                              isize)
                                                                             as
                                                                             isize)).synthesisState,
                                       aiBuf, aiBufLen, cmd, updateIndex);
            i += 1
        }
        if useReverb != 0 {
            if !(*reverb).filterLeft.is_null() ||
                   !(*reverb).filterRight.is_null() {
                cmd =
                    AudioSynth_FilterReverb(cmd, aiBufLen * 2 as libc::c_int,
                                            reverb)
            }
            if unk14 != 0 {
                cmd =
                    AudioSynth_SaveRingBuffer2(cmd, reverb,
                                               updateIndex as s16)
            } else {
                cmd =
                    AudioSynth_SaveReverbSamples(cmd, reverb,
                                                 updateIndex as s16);
                if (*reverb).unk_05 as libc::c_int != -(1 as libc::c_int) {
                    cmd =
                        AudioSynth_MaybeMixRingBuffer1(cmd, reverb,
                                                       updateIndex)
                }
            }
        }
        reverbIndex += 1
    }
    while i < count as libc::c_int {
        cmd =
            AudioSynth_ProcessNote(noteIndices[i as usize] as s32,
                                   &mut *gAudioContext.noteSubsEu.offset((t +
                                                                              *noteIndices.as_mut_ptr().offset(i
                                                                                                                   as
                                                                                                                   isize)
                                                                                  as
                                                                                  libc::c_int)
                                                                             as
                                                                             isize),
                                   &mut (*gAudioContext.notes.offset(*noteIndices.as_mut_ptr().offset(i
                                                                                                          as
                                                                                                          isize)
                                                                         as
                                                                         isize)).synthesisState,
                                   aiBuf, aiBufLen, cmd, updateIndex);
        i += 1
    }
    updateIndex = aiBufLen * 2 as libc::c_int;
    let fresh34 = cmd;
    cmd = cmd.offset(1);
    let mut _a_4: *mut Acmd = fresh34;
    (*_a_4).words.w0 =
        (13 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((updateIndex >> 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0x3c0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_a_4).words.w1 =
        (0x940 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 16 as libc::c_int |
            (0xae0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    let fresh35 = cmd;
    cmd = cmd.offset(1);
    let mut _a_5: *mut Acmd = fresh35;
    (*_a_5).words.w0 =
        (21 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            ((updateIndex * 2 as libc::c_int >> 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0x3c0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_a_5).words.w1 = aiBuf as u32_0;
    return cmd;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSynth_ProcessNote(mut noteIndex: s32,
                                                mut noteSubEu: *mut NoteSubEu,
                                                mut synthState:
                                                    *mut NoteSynthesisState,
                                                mut aiBuf: *mut s16,
                                                mut aiBufLen: s32,
                                                mut cmd: *mut Acmd,
                                                mut updateIndex: s32)
 -> *mut Acmd {
    let mut current_block: u64;
    let mut pad1: [s32; 3] = [0; 3];
    let mut audioFontSample: *mut SoundFontSample = 0 as *mut SoundFontSample;
    let mut loopInfo: *mut AdpcmLoop = 0 as *mut AdpcmLoop;
    let mut nSamplesUntilLoopEnd: s32 = 0;
    let mut nSamplesInThisIteration: s32 = 0;
    let mut noteFinished: s32 = 0;
    let mut restart: s32 = 0;
    let mut flags: s32 = 0;
    let mut resamplingRateFixedPoint: u16_0 = 0;
    let mut nSamplesInFirstFrame: s32 = 0;
    let mut nTrailingSamplesToIgnore: s32 = 0;
    let mut phi_a1_2: s32 = 0;
    let mut frameIndex: s32 = 0;
    let mut skipBytes: s32 = 0;
    let mut temp_v1_6: s32 = 0;
    let mut buf: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut nSamplesToDecode: s32 = 0;
    let mut sampleAddr: u32_0 = 0;
    let mut samplesLenFixedPoint: u32_0 = 0;
    let mut samplesLenAdjusted: s32 = 0;
    let mut nSamplesProcessed: s32 = 0;
    let mut loopEndPos: s32 = 0;
    let mut nSamplesToProcess: s32 = 0;
    let mut phi_s4: s32 = 0;
    let mut nFirstFrameSamplesToIgnore: s32 = 0;
    let mut pad2: [s32; 7] = [0; 7];
    let mut frameSize: s32 = 0;
    let mut nFramesToDecode: s32 = 0;
    let mut skipInitialSamples: s32 = 0;
    let mut sampleDataStart: s32 = 0;
    let mut sampleData: *mut u8_0 = 0 as *mut u8_0;
    let mut nParts: s32 = 0;
    let mut curPart: s32 = 0;
    let mut sampleDataStartPad: s32 = 0;
    let mut side: s32 = 0;
    let mut resampledTempLen: s32 = 0;
    let mut noteSamplesDmemAddrBeforeResampling: u16_0 = 0;
    let mut sampleDataOffset: s32 = 0;
    let mut thing: s32 = 0;
    let mut s5: s32 = 0;
    let mut note: *mut Note = 0 as *mut Note;
    let mut nSamplesToLoad: u32_0 = 0;
    let mut unk7: u16_0 = 0;
    let mut unkE: u16_0 = 0;
    let mut filter: *mut s16 = 0 as *mut s16;
    let mut bookOffset: s32 = 0;
    let mut finished: s32 = 0;
    let mut aligned: s32 = 0;
    let mut addr: s16 = 0;
    let mut unused: u16_0 = 0;
    bookOffset = (*noteSubEu).bitField1.bookOffset() as s32;
    finished = (*noteSubEu).bitField0.finished() as s32;
    note = &mut *gAudioContext.notes.offset(noteIndex as isize) as *mut Note;
    flags = 0 as libc::c_int;
    if (*noteSubEu).bitField0.needsInit() as libc::c_int == 1 as libc::c_int {
        flags = 0x1 as libc::c_int;
        (*synthState).restart = 0 as libc::c_int as u8_0;
        (*synthState).samplePosInt = (*note).unk_BC as s32;
        (*synthState).samplePosFrac = 0 as libc::c_int as u16_0;
        (*synthState).curVolLeft = 0 as libc::c_int as s16;
        (*synthState).curVolRight = 0 as libc::c_int as s16;
        (*synthState).prevHeadsetPanRight = 0 as libc::c_int as u8_0;
        (*synthState).prevHeadsetPanLeft = 0 as libc::c_int as u8_0;
        (*synthState).reverbVol = (*noteSubEu).reverbVol;
        (*synthState).numParts = 0 as libc::c_int as u8_0;
        (*synthState).unk_1A = 1 as libc::c_int as u8_0;
        (*note).noteSubEu.bitField0.set_finished(0 as libc::c_int as u8_0);
        finished = 0 as libc::c_int
    }
    resamplingRateFixedPoint = (*noteSubEu).resamplingRateFixedPoint;
    nParts =
        (*noteSubEu).bitField1.hasTwoParts() as libc::c_int +
            1 as libc::c_int;
    samplesLenFixedPoint =
        (resamplingRateFixedPoint as libc::c_int * aiBufLen * 2 as libc::c_int
             + (*synthState).samplePosFrac as libc::c_int) as u32_0;
    nSamplesToLoad = samplesLenFixedPoint >> 16 as libc::c_int;
    (*synthState).samplePosFrac =
        (samplesLenFixedPoint & 0xffff as libc::c_int as libc::c_uint) as
            u16_0;
    // Partially-optimized out no-op ifs required for matching. SM64 decomp
    // makes it clear that this is how it should look.
    if !((*synthState).numParts as libc::c_int == 1 as libc::c_int &&
             nParts == 2 as libc::c_int) {
        ((*synthState).numParts as libc::c_int == 2 as libc::c_int) &&
            nParts == 1 as libc::c_int;
    }
    (*synthState).numParts = nParts as u8_0;
    if (*noteSubEu).bitField1.isSyntheticWave() != 0 {
        cmd =
            AudioSynth_LoadWaveSamples(cmd, noteSubEu, synthState,
                                       nSamplesToLoad as s32);
        noteSamplesDmemAddrBeforeResampling =
            (0x580 as libc::c_int +
                 (*synthState).samplePosInt * 2 as libc::c_int) as u16_0;
        (*synthState).samplePosInt =
            ((*synthState).samplePosInt as
                 libc::c_uint).wrapping_add(nSamplesToLoad) as s32 as s32
    } else {
        audioFontSample = (*(*noteSubEu).sound.soundFontSound).sample;
        loopInfo = (*audioFontSample).loop_0;
        loopEndPos = (*loopInfo).end as s32;
        sampleAddr = (*audioFontSample).sampleAddr as u32_0;
        resampledTempLen = 0 as libc::c_int;
        curPart = 0 as libc::c_int;
        while curPart < nParts {
            nSamplesProcessed = 0 as libc::c_int;
            s5 = 0 as libc::c_int;
            if nParts == 1 as libc::c_int {
                samplesLenAdjusted = nSamplesToLoad as s32
            } else if nSamplesToLoad & 1 as libc::c_int as libc::c_uint != 0 {
                samplesLenAdjusted =
                    (nSamplesToLoad &
                         !(1 as libc::c_int) as
                             libc::c_uint).wrapping_add((curPart *
                                                             2 as libc::c_int)
                                                            as libc::c_uint)
                        as s32
            } else { samplesLenAdjusted = nSamplesToLoad as s32 }
            if (*audioFontSample).codec() as libc::c_int ==
                   CODEC_ADPCM as libc::c_int ||
                   (*audioFontSample).codec() as libc::c_int ==
                       CODEC_SMALL_ADPCM as libc::c_int {
                if gAudioContext.curLoadedBook !=
                       (*(*audioFontSample).book).book.as_mut_ptr() {
                    let mut nEntries: u32_0 = 0;
                    match bookOffset {
                        1 => {
                            gAudioContext.curLoadedBook =
                                &mut *D_8012FBA8.as_mut_ptr().offset(1 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)
                                    as *mut s16
                        }
                        2 | 3 | _ => {
                            gAudioContext.curLoadedBook =
                                (*(*audioFontSample).book).book.as_mut_ptr()
                        }
                    }
                    nEntries =
                        (16 as libc::c_int * (*(*audioFontSample).book).order
                             * (*(*audioFontSample).book).npredictors) as
                            u32_0;
                    let fresh36 = cmd;
                    cmd = cmd.offset(1);
                    let mut _a: *mut Acmd = fresh36;
                    (*_a).words.w0 =
                        (11 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            24 as libc::c_int |
                            (nEntries &
                                 (((0x1 as libc::c_int) << 24 as libc::c_int)
                                      - 1 as libc::c_int) as libc::c_uint) <<
                                0 as libc::c_int;
                    (*_a).words.w1 = gAudioContext.curLoadedBook as u32_0
                }
            }
            while nSamplesProcessed != samplesLenAdjusted {
                noteFinished = 0 as libc::c_int;
                restart = 0 as libc::c_int;
                phi_s4 = 0 as libc::c_int;
                nFirstFrameSamplesToIgnore =
                    (*synthState).samplePosInt & 0xf as libc::c_int;
                nSamplesUntilLoopEnd =
                    loopEndPos - (*synthState).samplePosInt;
                nSamplesToProcess = samplesLenAdjusted - nSamplesProcessed;
                if nFirstFrameSamplesToIgnore == 0 as libc::c_int &&
                       (*synthState).restart == 0 {
                    nFirstFrameSamplesToIgnore = 16 as libc::c_int
                }
                nSamplesInFirstFrame =
                    16 as libc::c_int - nFirstFrameSamplesToIgnore;
                if nSamplesToProcess < nSamplesUntilLoopEnd {
                    nFramesToDecode =
                        (nSamplesToProcess - nSamplesInFirstFrame +
                             15 as libc::c_int) / 16 as libc::c_int;
                    nSamplesToDecode = nFramesToDecode * 16 as libc::c_int;
                    nTrailingSamplesToIgnore =
                        nSamplesInFirstFrame + nSamplesToDecode -
                            nSamplesToProcess
                } else {
                    nSamplesToDecode =
                        nSamplesUntilLoopEnd - nSamplesInFirstFrame;
                    nTrailingSamplesToIgnore = 0 as libc::c_int;
                    if nSamplesToDecode <= 0 as libc::c_int {
                        nSamplesToDecode = 0 as libc::c_int;
                        nSamplesInFirstFrame = nSamplesUntilLoopEnd
                    }
                    nFramesToDecode =
                        (nSamplesToDecode + 15 as libc::c_int) /
                            16 as libc::c_int;
                    if (*loopInfo).count != 0 as libc::c_int as libc::c_uint {
                        // Loop around and restart
                        restart = 1 as libc::c_int
                    } else { noteFinished = 1 as libc::c_int }
                }
                match (*audioFontSample).codec() as libc::c_int {
                    0 => {
                        frameSize = 9 as libc::c_int;
                        skipInitialSamples = 16 as libc::c_int;
                        sampleDataStart = 0 as libc::c_int;
                        current_block = 16974974966130203269;
                    }
                    3 => {
                        frameSize = 5 as libc::c_int;
                        skipInitialSamples = 16 as libc::c_int;
                        sampleDataStart = 0 as libc::c_int;
                        current_block = 16974974966130203269;
                    }
                    1 => {
                        frameSize = 16 as libc::c_int;
                        skipInitialSamples = 16 as libc::c_int;
                        sampleDataStart = 0 as libc::c_int;
                        current_block = 16974974966130203269;
                    }
                    2 => {
                        let fresh37 = cmd;
                        cmd = cmd.offset(1);
                        AudioSynth_ClearBuffer(fresh37, 0x580 as libc::c_int,
                                               samplesLenAdjusted *
                                                   2 as libc::c_int +
                                                   0x20 as libc::c_int);
                        flags = 0 as libc::c_int;
                        skipBytes = 0 as libc::c_int;
                        nSamplesProcessed = samplesLenAdjusted;
                        s5 = samplesLenAdjusted;
                        current_block = 3473272802436943065;
                    }
                    5 => {
                        let fresh38 = cmd;
                        cmd = cmd.offset(1);
                        AudioSynth_ClearBuffer(fresh38, 0x580 as libc::c_int,
                                               samplesLenAdjusted *
                                                   2 as libc::c_int +
                                                   0x20 as libc::c_int);
                        flags = 0 as libc::c_int;
                        skipBytes = 0 as libc::c_int;
                        nSamplesProcessed = samplesLenAdjusted;
                        s5 = samplesLenAdjusted;
                        current_block = 3473272802436943065;
                    }
                    4 | _ => { current_block = 16974974966130203269; }
                }
                match current_block {
                    16974974966130203269 => {
                        if nFramesToDecode != 0 as libc::c_int {
                            frameIndex =
                                ((*synthState).samplePosInt +
                                     skipInitialSamples -
                                     nFirstFrameSamplesToIgnore) /
                                    16 as libc::c_int;
                            sampleDataOffset = frameIndex * frameSize;
                            if (*audioFontSample).medium() as libc::c_int ==
                                   MEDIUM_RAM as libc::c_int {
                                sampleData =
                                    ((sampleDataStart + sampleDataOffset) as
                                         libc::c_uint).wrapping_add(sampleAddr)
                                        as *mut u8_0
                            } else if (*audioFontSample).medium() as
                                          libc::c_int ==
                                          MEDIUM_UNK as libc::c_int {
                                return cmd
                            } else {
                                sampleData =
                                    AudioLoad_DmaSampleData(((sampleDataStart
                                                                  +
                                                                  sampleDataOffset)
                                                                 as
                                                                 libc::c_uint).wrapping_add(sampleAddr),
                                                            (nFramesToDecode *
                                                                 frameSize +
                                                                 0x10 as
                                                                     libc::c_int
                                                                 +
                                                                 0xf as
                                                                     libc::c_int
                                                                 &
                                                                 !(0xf as
                                                                       libc::c_int))
                                                                as u32_0,
                                                            flags,
                                                            &mut (*synthState).sampleDmaIndex,
                                                            (*audioFontSample).medium()
                                                                as s32) as
                                        *mut u8_0
                            }
                            if sampleData.is_null() { return cmd }
                            sampleDataStartPad =
                                (sampleData as u32_0 &
                                     0xf as libc::c_int as libc::c_uint) as
                                    s32;
                            aligned =
                                nFramesToDecode * frameSize +
                                    16 as libc::c_int + 0xf as libc::c_int &
                                    !(0xf as libc::c_int);
                            addr = (0x940 as libc::c_int - aligned) as s16;
                            let fresh39 = cmd;
                            cmd = cmd.offset(1);
                            let mut _a_0: *mut Acmd = fresh39;
                            (*_a_0).words.w0 =
                                (20 as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int |
                                    ((aligned >> 4 as libc::c_int) as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        16 as libc::c_int |
                                    (addr as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               16 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        0 as libc::c_int;
                            (*_a_0).words.w1 =
                                sampleData.offset(-(sampleDataStartPad as
                                                        isize)) as u32_0
                        } else {
                            nSamplesToDecode = 0 as libc::c_int;
                            sampleDataStartPad = 0 as libc::c_int
                        }
                        if (*synthState).restart != 0 {
                            let fresh40 = cmd;
                            cmd = cmd.offset(1);
                            let mut _a_1: *mut Acmd = fresh40;
                            (*_a_1).words.w0 =
                                (15 as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int;
                            (*_a_1).words.w1 =
                                (*(*audioFontSample).loop_0).state.as_mut_ptr()
                                    as u32_0;
                            flags = 0x2 as libc::c_int;
                            (*synthState).restart = 0 as libc::c_int as u8_0
                        }
                        nSamplesInThisIteration =
                            nSamplesToDecode + nSamplesInFirstFrame -
                                nTrailingSamplesToIgnore;
                        if nSamplesProcessed == 0 as libc::c_int {
                            skipBytes =
                                nFirstFrameSamplesToIgnore * 2 as libc::c_int
                        } else {
                            phi_s4 =
                                s5 + 16 as libc::c_int + 0xf as libc::c_int &
                                    !(0xf as libc::c_int)
                        }
                        match (*audioFontSample).codec() as libc::c_int {
                            0 => {
                                aligned =
                                    nFramesToDecode * frameSize +
                                        0x10 as libc::c_int +
                                        0xf as libc::c_int &
                                        !(0xf as libc::c_int);
                                addr =
                                    (0x940 as libc::c_int - aligned) as s16;
                                let fresh41 = cmd;
                                cmd = cmd.offset(1);
                                let mut _a_2: *mut Acmd = fresh41;
                                (*_a_2).words.w0 =
                                    (8 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            16 as libc::c_int |
                                        ((addr as libc::c_int +
                                              sampleDataStartPad) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   16 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int;
                                (*_a_2).words.w1 =
                                    ((0x580 as libc::c_int + phi_s4) as u32_0
                                         &
                                         (((0x1 as libc::c_int) <<
                                               16 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        16 as libc::c_int |
                                        ((nSamplesToDecode * 2 as libc::c_int)
                                             as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   16 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int;
                                let fresh42 = cmd;
                                cmd = cmd.offset(1);
                                let mut _a_3: *mut Acmd = fresh42;
                                (*_a_3).words.w0 =
                                    (1 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (flags as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            16 as libc::c_int;
                                (*_a_3).words.w1 =
                                    (*(*synthState).synthesisBuffers).adpcmdecState.as_mut_ptr()
                                        as u32_0
                            }
                            3 => {
                                aligned =
                                    nFramesToDecode * frameSize +
                                        0x10 as libc::c_int +
                                        0xf as libc::c_int &
                                        !(0xf as libc::c_int);
                                addr =
                                    (0x940 as libc::c_int - aligned) as s16;
                                let fresh43 = cmd;
                                cmd = cmd.offset(1);
                                let mut _a_4: *mut Acmd = fresh43;
                                (*_a_4).words.w0 =
                                    (8 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        (0 as libc::c_int as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            16 as libc::c_int |
                                        ((addr as libc::c_int +
                                              sampleDataStartPad) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   16 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int;
                                (*_a_4).words.w1 =
                                    ((0x580 as libc::c_int + phi_s4) as u32_0
                                         &
                                         (((0x1 as libc::c_int) <<
                                               16 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        16 as libc::c_int |
                                        ((nSamplesToDecode * 2 as libc::c_int)
                                             as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   16 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            0 as libc::c_int;
                                let fresh44 = cmd;
                                cmd = cmd.offset(1);
                                let mut _a_5: *mut Acmd = fresh44;
                                (*_a_5).words.w0 =
                                    (1 as libc::c_int as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               8 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        24 as libc::c_int |
                                        ((flags | 4 as libc::c_int) as u32_0 &
                                             (((0x1 as libc::c_int) <<
                                                   8 as libc::c_int) -
                                                  1 as libc::c_int) as
                                                 libc::c_uint) <<
                                            16 as libc::c_int;
                                (*_a_5).words.w1 =
                                    (*(*synthState).synthesisBuffers).adpcmdecState.as_mut_ptr()
                                        as u32_0
                            }
                            1 => {
                                aligned =
                                    nFramesToDecode * frameSize +
                                        0x10 as libc::c_int +
                                        0xf as libc::c_int &
                                        !(0xf as libc::c_int);
                                addr =
                                    (0x940 as libc::c_int - aligned) as s16;
                                let fresh45 = cmd;
                                cmd = cmd.offset(1);
                                AudioSynth_SetBuffer(fresh45,
                                                     0 as libc::c_int,
                                                     addr as libc::c_int +
                                                         sampleDataStartPad,
                                                     0x580 as libc::c_int +
                                                         phi_s4,
                                                     nSamplesToDecode *
                                                         2 as libc::c_int);
                                let fresh46 = cmd;
                                cmd = cmd.offset(1);
                                AudioSynth_S8Dec(fresh46, flags,
                                                 (*(*synthState).synthesisBuffers).adpcmdecState.as_mut_ptr());
                            }
                            _ => { }
                        }
                        if nSamplesProcessed != 0 as libc::c_int {
                            let fresh47 = cmd;
                            cmd = cmd.offset(1);
                            let mut _a_6: *mut Acmd = fresh47;
                            (*_a_6).words.w0 =
                                (10 as libc::c_int as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           8 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 24 as libc::c_int |
                                    ((0x580 as libc::c_int + phi_s4 +
                                          nFirstFrameSamplesToIgnore *
                                              2 as libc::c_int) as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               24 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) <<
                                        0 as libc::c_int;
                            (*_a_6).words.w1 =
                                ((0x580 as libc::c_int + s5) as u32_0 &
                                     (((0x1 as libc::c_int) <<
                                           16 as libc::c_int) -
                                          1 as libc::c_int) as libc::c_uint)
                                    << 16 as libc::c_int |
                                    ((nSamplesInThisIteration *
                                          2 as libc::c_int) as u32_0 &
                                         (((0x1 as libc::c_int) <<
                                               16 as libc::c_int) -
                                              1 as libc::c_int) as
                                             libc::c_uint) << 0 as libc::c_int
                        }
                        nSamplesProcessed += nSamplesInThisIteration;
                        match flags {
                            1 => {
                                skipBytes = 0x20 as libc::c_int;
                                s5 =
                                    (nSamplesToDecode + 0x10 as libc::c_int) *
                                        2 as libc::c_int
                            }
                            2 => {
                                s5 =
                                    nSamplesInThisIteration * 2 as libc::c_int
                                        + s5
                            }
                            _ => {
                                if s5 != 0 as libc::c_int {
                                    s5 =
                                        nSamplesInThisIteration *
                                            2 as libc::c_int + s5
                                } else {
                                    s5 =
                                        (nFirstFrameSamplesToIgnore +
                                             nSamplesInThisIteration) *
                                            2 as libc::c_int
                                }
                            }
                        }
                        flags = 0 as libc::c_int
                    }
                    _ => { }
                }
                if noteFinished != 0 {
                    let fresh48 = cmd;
                    cmd = cmd.offset(1);
                    AudioSynth_ClearBuffer(fresh48, 0x580 as libc::c_int + s5,
                                           (samplesLenAdjusted -
                                                nSamplesProcessed) *
                                               2 as libc::c_int);
                    finished = 1 as libc::c_int;
                    (*note).noteSubEu.bitField0.set_finished(1 as libc::c_int
                                                                 as u8_0);
                    func_800DB2C0(updateIndex, noteIndex);
                    break ;
                } else if restart != 0 {
                    (*synthState).restart = 1 as libc::c_int as u8_0;
                    (*synthState).samplePosInt = (*loopInfo).start as s32
                } else { (*synthState).samplePosInt += nSamplesToProcess }
            }
            match nParts {
                1 => {
                    noteSamplesDmemAddrBeforeResampling =
                        (0x580 as libc::c_int + skipBytes) as u16_0
                }
                2 => {
                    match curPart {
                        0 => {
                            let fresh49 = cmd;
                            cmd = cmd.offset(1);
                            AudioSynth_InterL(fresh49,
                                              0x580 as libc::c_int +
                                                  skipBytes,
                                              0x3c0 as libc::c_int +
                                                  0x20 as libc::c_int,
                                              samplesLenAdjusted /
                                                  2 as libc::c_int +
                                                  7 as libc::c_int &
                                                  !(7 as libc::c_int));
                            resampledTempLen = samplesLenAdjusted;
                            noteSamplesDmemAddrBeforeResampling =
                                (0x3c0 as libc::c_int + 0x20 as libc::c_int)
                                    as u16_0;
                            if finished != 0 {
                                let fresh50 = cmd;
                                cmd = cmd.offset(1);
                                AudioSynth_ClearBuffer(fresh50,
                                                       noteSamplesDmemAddrBeforeResampling
                                                           as libc::c_int +
                                                           resampledTempLen,
                                                       samplesLenAdjusted +
                                                           0x10 as
                                                               libc::c_int);
                            }
                        }
                        1 => {
                            let fresh51 = cmd;
                            cmd = cmd.offset(1);
                            AudioSynth_InterL(fresh51,
                                              0x580 as libc::c_int +
                                                  skipBytes,
                                              0x3c0 as libc::c_int +
                                                  0x20 as libc::c_int +
                                                  resampledTempLen,
                                              samplesLenAdjusted /
                                                  2 as libc::c_int +
                                                  7 as libc::c_int &
                                                  !(7 as libc::c_int));
                        }
                        _ => { }
                    }
                }
                _ => { }
            }
            if finished != 0 { break ; }
            curPart += 1
        }
    }
    flags = 0 as libc::c_int;
    if (*noteSubEu).bitField0.needsInit() as libc::c_int == 1 as libc::c_int {
        (*noteSubEu).bitField0.set_needsInit(0 as libc::c_int as u8_0);
        flags = 0x1 as libc::c_int
    }
    cmd =
        AudioSynth_FinalResample(cmd, synthState, aiBufLen * 2 as libc::c_int,
                                 resamplingRateFixedPoint,
                                 noteSamplesDmemAddrBeforeResampling, flags);
    if bookOffset == 3 as libc::c_int {
        let fresh52 = cmd;
        cmd = cmd.offset(1);
        AudioSynth_UnkCmd19(fresh52, 0x3c0 as libc::c_int,
                            0x3c0 as libc::c_int, aiBufLen * 2 as libc::c_int,
                            0 as libc::c_int);
    }
    if bookOffset == 2 as libc::c_int {
        let fresh53 = cmd;
        cmd = cmd.offset(1);
        AudioSynth_UnkCmd3(fresh53, 0x3c0 as libc::c_int,
                           0x3c0 as libc::c_int, aiBufLen * 2 as libc::c_int);
    }
    phi_a1_2 = (*noteSubEu).unk_2 as s32;
    if phi_a1_2 != 0 as libc::c_int {
        if phi_a1_2 < 0x10 as libc::c_int { phi_a1_2 = 0x10 as libc::c_int }
        let fresh54 = cmd;
        cmd = cmd.offset(1);
        AudioSynth_HiLoGain(fresh54, phi_a1_2, 0x3c0 as libc::c_int,
                            0 as libc::c_int,
                            aiBufLen * 2 as libc::c_int +
                                0x20 as libc::c_int);
    }
    filter = (*noteSubEu).filter;
    if !filter.is_null() {
        let fresh55 = cmd;
        cmd = cmd.offset(1);
        AudioSynth_LoadFilterCount(fresh55, aiBufLen * 2 as libc::c_int,
                                   filter as s32);
        let fresh56 = cmd;
        cmd = cmd.offset(1);
        AudioSynth_LoadFilter(fresh56, flags, 0x3c0 as libc::c_int,
                              (*(*synthState).synthesisBuffers).mixEnvelopeState.as_mut_ptr()
                                  as s32);
    }
    unk7 = (*noteSubEu).unk_07 as u16_0;
    unkE = (*noteSubEu).unk_0E;
    buf =
        &mut *(*(*synthState).synthesisBuffers).panSamplesBuffer.as_mut_ptr().offset(0x18
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         isize)
            as *mut s16 as *mut libc::c_void;
    if unk7 as libc::c_int != 0 as libc::c_int &&
           (*noteSubEu).unk_0E as libc::c_int != 0 as libc::c_int {
        let fresh57 = cmd;
        cmd = cmd.offset(1);
        AudioSynth_DMemMove(fresh57, 0x3c0 as libc::c_int,
                            0x760 as libc::c_int,
                            aiBufLen * 2 as libc::c_int);
        thing = 0x760 as libc::c_int - unk7 as libc::c_int;
        if (*synthState).unk_1A as libc::c_int != 0 as libc::c_int {
            let fresh58 = cmd;
            cmd = cmd.offset(1);
            AudioSynth_ClearBuffer(fresh58, thing, unk7 as s32);
            (*synthState).unk_1A = 0 as libc::c_int as u8_0
        } else {
            let fresh59 = cmd;
            cmd = cmd.offset(1);
            AudioSynth_LoadBuffer(fresh59, thing, unk7 as s32, buf as s32);
        }
        let fresh60 = cmd;
        cmd = cmd.offset(1);
        AudioSynth_SaveBuffer(fresh60,
                              0x3c0 as libc::c_int +
                                  aiBufLen * 2 as libc::c_int -
                                  unk7 as libc::c_int, unk7 as s32,
                              buf as s32);
        let fresh61 = cmd;
        cmd = cmd.offset(1);
        AudioSynth_Mix(fresh61,
                       aiBufLen * 2 as libc::c_int >> 4 as libc::c_int,
                       unkE as s32, 0x760 as libc::c_int, thing);
        let fresh62 = cmd;
        cmd = cmd.offset(1);
        AudioSynth_DMemMove(fresh62, thing, 0x3c0 as libc::c_int,
                            aiBufLen * 2 as libc::c_int);
    } else { (*synthState).unk_1A = 1 as libc::c_int as u8_0 }
    if (*noteSubEu).headsetPanRight as libc::c_int != 0 as libc::c_int ||
           (*synthState).prevHeadsetPanRight as libc::c_int !=
               0 as libc::c_int {
        side = 1 as libc::c_int
    } else if (*noteSubEu).headsetPanLeft as libc::c_int != 0 as libc::c_int
                  ||
                  (*synthState).prevHeadsetPanLeft as libc::c_int !=
                      0 as libc::c_int {
        side = 2 as libc::c_int
    } else { side = 0 as libc::c_int }
    cmd =
        AudioSynth_ProcessEnvelope(cmd, noteSubEu, synthState, aiBufLen,
                                   0x3c0 as libc::c_int as u16_0, side,
                                   flags);
    if (*noteSubEu).bitField1.usesHeadsetPanEffects2() != 0 {
        if flags & 0x1 as libc::c_int == 0 { flags = 0 as libc::c_int }
        cmd =
            AudioSynth_NoteApplyHeadsetPanEffects(cmd, noteSubEu, synthState,
                                                  aiBufLen * 2 as libc::c_int,
                                                  flags, side)
    }
    return cmd;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSynth_FinalResample(mut cmd: *mut Acmd,
                                                  mut synthState:
                                                      *mut NoteSynthesisState,
                                                  mut count: s32,
                                                  mut pitch: u16_0,
                                                  mut inpDmem: u16_0,
                                                  mut resampleFlags: s32)
 -> *mut Acmd {
    if pitch as libc::c_int == 0 as libc::c_int {
        let fresh63 = cmd;
        cmd = cmd.offset(1);
        AudioSynth_ClearBuffer(fresh63, 0x3c0 as libc::c_int, count);
    } else {
        let fresh64 = cmd;
        cmd = cmd.offset(1);
        let mut _a: *mut Acmd = fresh64;
        (*_a).words.w0 =
            (8 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                (inpDmem as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_a).words.w1 =
            (0x3c0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
                |
                (count as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh65 = cmd;
        cmd = cmd.offset(1);
        let mut _a_0: *mut Acmd = fresh65;
        (*_a_0).words.w0 =
            (5 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (resampleFlags as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                (pitch as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_a_0).words.w1 =
            (*(*synthState).synthesisBuffers).finalResampleState.as_mut_ptr()
                as u32_0
    }
    return cmd;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSynth_ProcessEnvelope(mut cmd: *mut Acmd,
                                                    mut noteSubEu:
                                                        *mut NoteSubEu,
                                                    mut synthState:
                                                        *mut NoteSynthesisState,
                                                    mut aiBufLen: s32,
                                                    mut inBuf: u16_0,
                                                    mut headsetPanSettings:
                                                        s32, mut flags: s32)
 -> *mut Acmd {
    let mut phi_a1: u32_0 = 0;
    let mut curVolLeft: u16_0 = 0;
    let mut targetVolLeft: u16_0 = 0;
    let mut phi_t1: s32 = 0;
    let mut reverbVol: s16 = 0;
    let mut curVolRight: u16_0 = 0;
    let mut rampLeft: s16 = 0;
    let mut rampRight: s16 = 0;
    let mut rampReverb: s16 = 0;
    let mut sourceReverbVol: s16 = 0;
    let mut targetVolRight: u16_0 = 0;
    let mut pad: s32 = 0;
    curVolLeft = (*synthState).curVolLeft as u16_0;
    targetVolLeft = (*noteSubEu).targetVolLeft;
    targetVolLeft =
        ((targetVolLeft as libc::c_int) << 4 as libc::c_int) as u16_0;
    reverbVol = (*noteSubEu).reverbVol as s16;
    curVolRight = (*synthState).curVolRight as u16_0;
    targetVolRight = (*noteSubEu).targetVolRight;
    targetVolRight =
        ((targetVolRight as libc::c_int) << 4 as libc::c_int) as u16_0;
    if targetVolLeft as libc::c_int != curVolLeft as libc::c_int {
        rampLeft =
            ((targetVolLeft as libc::c_int - curVolLeft as libc::c_int) /
                 (aiBufLen >> 3 as libc::c_int)) as s16
    } else { rampLeft = 0 as libc::c_int as s16 }
    if targetVolRight as libc::c_int != curVolRight as libc::c_int {
        rampRight =
            ((targetVolRight as libc::c_int - curVolRight as libc::c_int) /
                 (aiBufLen >> 3 as libc::c_int)) as s16
    } else { rampRight = 0 as libc::c_int as s16 }
    sourceReverbVol = (*synthState).reverbVol as s16;
    phi_t1 = sourceReverbVol as libc::c_int & 0x7f as libc::c_int;
    if sourceReverbVol as libc::c_int != reverbVol as libc::c_int {
        rampReverb =
            (((reverbVol as libc::c_int & 0x7f as libc::c_int) - phi_t1 <<
                  9 as libc::c_int) / (aiBufLen >> 3 as libc::c_int)) as s16;
        (*synthState).reverbVol = reverbVol as u8_0
    } else { rampReverb = 0 as libc::c_int as s16 }
    (*synthState).curVolLeft =
        (curVolLeft as libc::c_int +
             rampLeft as libc::c_int * (aiBufLen >> 3 as libc::c_int)) as s16;
    (*synthState).curVolRight =
        (curVolRight as libc::c_int +
             rampRight as libc::c_int * (aiBufLen >> 3 as libc::c_int)) as
            s16;
    if (*noteSubEu).bitField1.usesHeadsetPanEffects2() != 0 {
        let fresh66 = cmd;
        cmd = cmd.offset(1);
        AudioSynth_ClearBuffer(fresh66, 0x5c0 as libc::c_int,
                               0x1a0 as libc::c_int);
        let fresh67 = cmd;
        cmd = cmd.offset(1);
        AudioSynth_EnvSetup1(fresh67, phi_t1 * 2 as libc::c_int,
                             rampReverb as s32, rampLeft as s32,
                             rampRight as s32);
        let fresh68 = cmd;
        cmd = cmd.offset(1);
        AudioSynth_EnvSetup2(fresh68, curVolLeft as s32, curVolRight as s32);
        match headsetPanSettings {
            1 => { phi_a1 = D_801304A4 }
            2 => { phi_a1 = D_801304A8 }
            _ => { phi_a1 = D_801304AC }
        }
    } else {
        let fresh69 = cmd;
        cmd = cmd.offset(1);
        let mut _a: *mut Acmd = fresh69;
        (*_a).words.w0 =
            (18 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((phi_t1 * 2 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                (rampReverb as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_a).words.w1 =
            (rampLeft as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
                |
                (rampRight as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh70 = cmd;
        cmd = cmd.offset(1);
        let mut _a_0: *mut Acmd = fresh70;
        (*_a_0).words.w0 =
            (22 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int;
        (*_a_0).words.w1 =
            (curVolLeft as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
                |
                (curVolRight as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        phi_a1 = D_801304AC
    }
    let fresh71 = cmd;
    cmd = cmd.offset(1);
    let mut _a_1: *mut Acmd = fresh71;
    (*_a_1).words.w0 =
        D_801304A0 |
            ((inBuf as libc::c_int >> 4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (aiBufLen as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 8 as libc::c_int |
            (((sourceReverbVol as libc::c_int & 0x80 as libc::c_int) >>
                  7 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 1 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 4 as libc::c_int |
            ((*noteSubEu).bitField0.stereoHeadsetEffects() as u32_0 &
                 (((0x1 as libc::c_int) << 1 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 3 as libc::c_int |
            ((*noteSubEu).bitField0.usesHeadsetPanEffects() as u32_0 &
                 (((0x1 as libc::c_int) << 1 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 2 as libc::c_int |
            ((*noteSubEu).bitField0.stereoStrongRight() as u32_0 &
                 (((0x1 as libc::c_int) << 1 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 1 as libc::c_int |
            ((*noteSubEu).bitField0.stereoStrongLeft() as u32_0 &
                 (((0x1 as libc::c_int) << 1 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_a_1).words.w1 = phi_a1;
    return cmd;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSynth_LoadWaveSamples(mut cmd: *mut Acmd,
                                                    mut noteSubEu:
                                                        *mut NoteSubEu,
                                                    mut synthState:
                                                        *mut NoteSynthesisState,
                                                    mut nSamplesToLoad: s32)
 -> *mut Acmd {
    let mut temp_v0: s32 = 0;
    let mut unk6: s32 = (*noteSubEu).unk_06 as s32;
    let mut samplePosInt: s32 = (*synthState).samplePosInt;
    let mut repeats: s32 = 0;
    if (*noteSubEu).bitField1.bookOffset() as libc::c_int != 0 as libc::c_int
       {
        let fresh72 = cmd;
        cmd = cmd.offset(1);
        AudioSynth_LoadBuffer(fresh72, 0x580 as libc::c_int,
                              nSamplesToLoad * 2 as libc::c_int +
                                  0xf as libc::c_int & !(0xf as libc::c_int),
                              gWaveSamples[8 as libc::c_int as usize] as s32);
        gWaveSamples[8 as libc::c_int as usize] =
            gWaveSamples[8 as libc::c_int as
                             usize].offset((nSamplesToLoad * 2 as libc::c_int)
                                               as isize);
        return cmd
    } else {
        let fresh73 = cmd;
        cmd = cmd.offset(1);
        let mut _a: *mut Acmd = fresh73;
        (*_a).words.w0 =
            (20 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                ((0x80 as libc::c_int >> 4 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                (0x580 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_a).words.w1 = (*noteSubEu).sound.samples as u32_0;
        if unk6 != 0 as libc::c_int {
            samplePosInt =
                samplePosInt *
                    D_801304C0[(unk6 >> 2 as libc::c_int) as usize] as
                        libc::c_int /
                    D_801304C0[(unk6 & 3 as libc::c_int) as usize] as
                        libc::c_int
        }
        samplePosInt &= 0x3f as libc::c_int;
        temp_v0 = 0x40 as libc::c_int - samplePosInt;
        if temp_v0 < nSamplesToLoad {
            repeats =
                (nSamplesToLoad - temp_v0 + 0x3f as libc::c_int) /
                    0x40 as libc::c_int;
            if repeats != 0 as libc::c_int {
                let fresh74 = cmd;
                cmd = cmd.offset(1);
                let mut _a_0: *mut Acmd = fresh74;
                (*_a_0).words.w0 =
                    (9 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        24 as libc::c_int |
                        (repeats as u32_0 &
                             (((0x1 as libc::c_int) << 8 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            16 as libc::c_int |
                        (0x580 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int;
                (*_a_0).words.w1 =
                    ((0x580 as libc::c_int + 0x80 as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                        (0x80 as libc::c_int as u32_0 &
                             (((0x1 as libc::c_int) << 16 as libc::c_int) -
                                  1 as libc::c_int) as libc::c_uint) <<
                            0 as libc::c_int
            }
        }
        (*synthState).samplePosInt = samplePosInt
    }
    return cmd;
}
#[no_mangle]
pub unsafe extern "C" fn AudioSynth_NoteApplyHeadsetPanEffects(mut cmd:
                                                                   *mut Acmd,
                                                               mut noteSubEu:
                                                                   *mut NoteSubEu,
                                                               mut synthState:
                                                                   *mut NoteSynthesisState,
                                                               mut bufLen:
                                                                   s32,
                                                               mut flags: s32,
                                                               mut side: s32)
 -> *mut Acmd {
    let mut dest: u16_0 = 0;
    let mut pitch: u16_0 = 0;
    let mut prevPanShift: u8_0 = 0;
    let mut panShift: u8_0 = 0;
    match side {
        1 => {
            dest = 0x940 as libc::c_int as u16_0;
            panShift = (*noteSubEu).headsetPanRight;
            prevPanShift = (*synthState).prevHeadsetPanRight;
            (*synthState).prevHeadsetPanLeft = 0 as libc::c_int as u8_0;
            (*synthState).prevHeadsetPanRight = panShift
        }
        2 => {
            dest = 0xae0 as libc::c_int as u16_0;
            panShift = (*noteSubEu).headsetPanLeft;
            prevPanShift = (*synthState).prevHeadsetPanLeft;
            (*synthState).prevHeadsetPanLeft = panShift;
            (*synthState).prevHeadsetPanRight = 0 as libc::c_int as u8_0
        }
        _ => { return cmd }
    }
    if flags != 0x1 as libc::c_int {
        // Slightly adjust the sample rate in order to fit a change in pan shift
        if panShift as libc::c_int != prevPanShift as libc::c_int {
            pitch =
                (((bufLen << 0xf as libc::c_int) / 2 as libc::c_int -
                      1 as libc::c_int) /
                     ((bufLen + panShift as libc::c_int -
                           prevPanShift as libc::c_int - 2 as libc::c_int) /
                          2 as libc::c_int)) as u16_0;
            let fresh75 = cmd;
            cmd = cmd.offset(1);
            let mut _a: *mut Acmd = fresh75;
            (*_a).words.w0 =
                (8 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (0x5c0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_a).words.w1 =
                (0x3c0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                    ((bufLen + panShift as libc::c_int -
                          prevPanShift as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            let fresh76 = cmd;
            cmd = cmd.offset(1);
            let mut _a_0: *mut Acmd = fresh76;
            (*_a_0).words.w0 =
                (6 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (pitch as u32_0 &
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_a_0).words.w1 =
                (0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int
        } else {
            let fresh77 = cmd;
            cmd = cmd.offset(1);
            let mut _a_1: *mut Acmd = fresh77;
            (*_a_1).words.w0 =
                (10 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0x5c0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 24 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_a_1).words.w1 =
                (0x3c0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                    (bufLen as u32_0 &
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int
        }
        if prevPanShift as libc::c_int != 0 as libc::c_int {
            let fresh78 = cmd;
            cmd = cmd.offset(1);
            let mut _a_2: *mut Acmd = fresh78;
            (*_a_2).words.w0 =
                (20 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (((prevPanShift as libc::c_int + 0xf as libc::c_int &
                           !(0xf as libc::c_int)) >> 4 as libc::c_int) as
                         u32_0 &
                         (((0x1 as libc::c_int) << 8 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        16 as libc::c_int |
                    (0x5c0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_a_2).words.w1 =
                &mut *(*(*synthState).synthesisBuffers).panResampleState.as_mut_ptr().offset(0x8
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 isize)
                    as *mut s16 as u32_0;
            let fresh79 = cmd;
            cmd = cmd.offset(1);
            let mut _a_3: *mut Acmd = fresh79;
            (*_a_3).words.w0 =
                (10 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0x3c0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 24 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_a_3).words.w1 =
                ((0x5c0 as libc::c_int + prevPanShift as libc::c_int) as u32_0
                     &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                    ((bufLen + panShift as libc::c_int -
                          prevPanShift as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int
        } else {
            let fresh80 = cmd;
            cmd = cmd.offset(1);
            let mut _a_4: *mut Acmd = fresh80;
            (*_a_4).words.w0 =
                (10 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    24 as libc::c_int |
                    (0x3c0 as libc::c_int as u32_0 &
                         (((0x1 as libc::c_int) << 24 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int;
            (*_a_4).words.w1 =
                (0x5c0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                    ((bufLen + panShift as libc::c_int) as u32_0 &
                         (((0x1 as libc::c_int) << 16 as libc::c_int) -
                              1 as libc::c_int) as libc::c_uint) <<
                        0 as libc::c_int
        }
    } else {
        // Just shift right
        let fresh81 = cmd;
        cmd = cmd.offset(1);
        let mut _a_5: *mut Acmd = fresh81;
        (*_a_5).words.w0 =
            (10 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0x5c0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 24 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_a_5).words.w1 =
            (0x3c0 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
                |
                (bufLen as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        let fresh82 = cmd;
        cmd = cmd.offset(1);
        let mut _a_6: *mut Acmd = fresh82;
        (*_a_6).words.w0 =
            (2 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0x5c0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 24 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_a_6).words.w1 = panShift as u32_0;
        let fresh83 = cmd;
        cmd = cmd.offset(1);
        let mut _a_7: *mut Acmd = fresh83;
        (*_a_7).words.w0 =
            (10 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (0x3c0 as libc::c_int as u32_0 &
                     (((0x1 as libc::c_int) << 24 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_a_7).words.w1 =
            ((0x5c0 as libc::c_int + panShift as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
                |
                (bufLen as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int
    }
    if panShift != 0 {
        // Save excessive samples for next iteration
        let fresh84 = cmd;
        cmd = cmd.offset(1);
        let mut _a_8: *mut Acmd = fresh84;
        (*_a_8).words.w0 =
            (21 as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 24 as libc::c_int
                |
                (((panShift as libc::c_int + 0xf as libc::c_int &
                       !(0xf as libc::c_int)) >> 4 as libc::c_int) as u32_0 &
                     (((0x1 as libc::c_int) << 8 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    16 as libc::c_int |
                ((0x5c0 as libc::c_int + bufLen) as u32_0 &
                     (((0x1 as libc::c_int) << 16 as libc::c_int) -
                          1 as libc::c_int) as libc::c_uint) <<
                    0 as libc::c_int;
        (*_a_8).words.w1 =
            &mut *(*(*synthState).synthesisBuffers).panResampleState.as_mut_ptr().offset(0x8
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             isize)
                as *mut s16 as u32_0
    }
    let fresh85 = cmd;
    cmd = cmd.offset(1);
    let mut _a_9: *mut Acmd = fresh85;
    (*_a_9).words.w0 =
        (4 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 24 as libc::c_int |
            (((bufLen + 0x3f as libc::c_int & !(0x3f as libc::c_int)) >>
                  4 as libc::c_int) as u32_0 &
                 (((0x1 as libc::c_int) << 8 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 16 as libc::c_int
            |
            (0x7fff as libc::c_int as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    (*_a_9).words.w1 =
        (0x5c0 as libc::c_int as u32_0 &
             (((0x1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int)
                 as libc::c_uint) << 16 as libc::c_int |
            (dest as u32_0 &
                 (((0x1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as libc::c_uint) << 0 as libc::c_int;
    return cmd;
}
