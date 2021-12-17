#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    static mut gStereoPanVolume: [f32_0; 128];
    #[no_mangle]
    fn AudioHeap_AllocDmaMemory(pool: *mut AudioAllocPool, size: u32_0)
     -> *mut libc::c_void;
    #[no_mangle]
    fn AudioLoad_IsFontLoadComplete(fontId: s32) -> s32;
    #[no_mangle]
    static mut gHeadsetPanVolume: [f32_0; 128];
    #[no_mangle]
    static mut gHeadsetPanQuantization: [u16_0; 64];
    #[no_mangle]
    static mut gDefaultNoteSub: NoteSubEu;
    #[no_mangle]
    fn Audio_AdsrInit(adsr: *mut AdsrState, envelope: *mut AdsrEnvelope,
                      volOut: *mut s16);
    #[no_mangle]
    fn Audio_NoteVibratoUpdate(note: *mut Note);
    #[no_mangle]
    fn Audio_AdsrUpdate(adsr: *mut AdsrState) -> f32_0;
    #[no_mangle]
    fn AudioSeq_AudioListPushBack(list: *mut AudioListItem,
                                  item: *mut AudioListItem);
    #[no_mangle]
    fn Audio_NotePortamentoInit(note: *mut Note);
    #[no_mangle]
    fn Audio_NoteVibratoInit(note: *mut Note);
    #[no_mangle]
    static mut gWaveSamples: [*mut s16; 9];
    #[no_mangle]
    fn AudioSeq_SequenceChannelDisable(channel: *mut SequenceChannel);
    #[no_mangle]
    fn AudioSeq_AudioListPopBack(list: *mut AudioListItem)
     -> *mut libc::c_void;
    #[no_mangle]
    static mut gZeroNoteSub: NoteSubEu;
    #[no_mangle]
    static mut gDefaultPanVolume: [f32_0; 128];
    #[no_mangle]
    static mut gAudioContext: AudioContext;
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
pub const ADSR_STATE_SUSTAIN: C2RustUnnamed_0 = 8;
pub const ADSR_STATE_RELEASE: C2RustUnnamed_0 = 7;
pub const ADSR_STATE_DECAY: C2RustUnnamed_0 = 6;
pub const ADSR_STATE_HANG: C2RustUnnamed_0 = 5;
pub const ADSR_STATE_FADE: C2RustUnnamed_0 = 4;
pub const ADSR_STATE_LOOP: C2RustUnnamed_0 = 3;
pub const ADSR_STATE_START_LOOP: C2RustUnnamed_0 = 2;
pub const ADSR_STATE_INITIAL: C2RustUnnamed_0 = 1;
pub const ADSR_STATE_DISABLED: C2RustUnnamed_0 = 0;
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
pub struct NoteSubAttributes {
    pub reverbVol: u8_0,
    pub unk_1: u8_0,
    pub pan: u8_0,
    pub stereo: Stereo,
    pub frequency: f32_0,
    pub velocity: f32_0,
    pub unk_0C: [libc::c_char; 4],
    pub filter: *mut s16,
    pub unk_14: u8_0,
    pub unk_16: u16_0,
}
#[no_mangle]
pub unsafe extern "C" fn Audio_InitNoteSub(mut note: *mut Note,
                                           mut sub: *mut NoteSubEu,
                                           mut attrs:
                                               *mut NoteSubAttributes) {
    let mut volRight: f32_0 = 0.; // approx 1/sqrt(2)
    let mut volLeft: f32_0 = 0.;
    let mut smallPanIndex: s32 = 0;
    let mut pad: u64_0 = 0;
    let mut strongLeft: u8_0 = 0;
    let mut strongRight: u8_0 = 0;
    let mut vel: f32_0 = 0.;
    let mut pan: u8_0 = 0;
    let mut reverbVol: u8_0 = 0;
    let mut sp24: StereoData =
        StereoData{unused_bit2_strongRight_strongLeft_stereoHeadsetEffects_usesHeadsetPanEffects:
                       [0; 1],};
    let mut stereoHeadsetEffects: s32 =
        (*note).playbackState.stereoHeadsetEffects as s32;
    vel = (*attrs).velocity;
    pan = (*attrs).pan;
    reverbVol = (*attrs).reverbVol;
    sp24 = (*attrs).stereo.s;
    (*sub).bitField0 = (*note).noteSubEu.bitField0;
    (*sub).bitField1 = (*note).noteSubEu.bitField1;
    (*sub).sound.samples = (*note).noteSubEu.sound.samples;
    (*sub).unk_06 = (*note).noteSubEu.unk_06;
    Audio_NoteSetResamplingRate(sub, (*attrs).frequency);
    pan = (pan as libc::c_int & 0x7f as libc::c_int) as u8_0;
    (*sub).bitField0.set_stereoStrongRight(0 as libc::c_int as u8_0);
    (*sub).bitField0.set_stereoStrongLeft(0 as libc::c_int as u8_0);
    (*sub).bitField0.set_stereoHeadsetEffects(sp24.stereoHeadsetEffects());
    (*sub).bitField0.set_usesHeadsetPanEffects(sp24.usesHeadsetPanEffects());
    if stereoHeadsetEffects != 0 &&
           gAudioContext.soundMode as libc::c_int == 1 as libc::c_int {
        smallPanIndex = pan as libc::c_int >> 1 as libc::c_int;
        if smallPanIndex > 0x3f as libc::c_int {
            smallPanIndex = 0x3f as libc::c_int
        }
        (*sub).headsetPanLeft =
            gHeadsetPanQuantization[smallPanIndex as usize] as u8_0;
        (*sub).headsetPanRight =
            gHeadsetPanQuantization[(0x3f as libc::c_int - smallPanIndex) as
                                        usize] as u8_0;
        (*sub).bitField1.set_usesHeadsetPanEffects2(1 as libc::c_int as u8_0);
        volLeft = gHeadsetPanVolume[pan as usize];
        volRight =
            gHeadsetPanVolume[(0x7f as libc::c_int - pan as libc::c_int) as
                                  usize]
    } else if stereoHeadsetEffects != 0 &&
                  gAudioContext.soundMode as libc::c_int == 0 as libc::c_int {
        strongRight = 0 as libc::c_int as u8_0;
        strongLeft = strongRight;
        (*sub).headsetPanRight = 0 as libc::c_int as u8_0;
        (*sub).headsetPanLeft = 0 as libc::c_int as u8_0;
        (*sub).bitField1.set_usesHeadsetPanEffects2(0 as libc::c_int as u8_0);
        volLeft = gStereoPanVolume[pan as usize];
        volRight =
            gStereoPanVolume[(0x7f as libc::c_int - pan as libc::c_int) as
                                 usize];
        if (pan as libc::c_int) < 0x20 as libc::c_int {
            strongLeft = 1 as libc::c_int as u8_0
        } else if pan as libc::c_int > 0x60 as libc::c_int {
            strongRight = 1 as libc::c_int as u8_0
        }
        (*sub).bitField0.set_stereoStrongRight(strongRight);
        (*sub).bitField0.set_stereoStrongLeft(strongLeft);
        match sp24.bit2() as libc::c_int {
            1 => {
                (*sub).bitField0.set_stereoStrongRight(sp24.strongRight());
                (*sub).bitField0.set_stereoStrongLeft(sp24.strongLeft())
            }
            2 => {
                (*sub).bitField0.set_stereoStrongRight((sp24.strongRight() as
                                                            libc::c_int |
                                                            strongRight as
                                                                libc::c_int)
                                                           as u8_0);
                (*sub).bitField0.set_stereoStrongLeft((sp24.strongLeft() as
                                                           libc::c_int |
                                                           strongLeft as
                                                               libc::c_int) as
                                                          u8_0)
            }
            3 => {
                (*sub).bitField0.set_stereoStrongRight((sp24.strongRight() as
                                                            libc::c_int ^
                                                            strongRight as
                                                                libc::c_int)
                                                           as u8_0);
                (*sub).bitField0.set_stereoStrongLeft((sp24.strongLeft() as
                                                           libc::c_int ^
                                                           strongLeft as
                                                               libc::c_int) as
                                                          u8_0)
            }
            0 | _ => { }
        }
    } else if gAudioContext.soundMode as libc::c_int == 3 as libc::c_int {
        (*sub).bitField0.set_stereoHeadsetEffects(0 as libc::c_int as u8_0);
        (*sub).bitField0.set_usesHeadsetPanEffects(0 as libc::c_int as u8_0);
        volLeft = 0.707f32;
        volRight = 0.707f32
    } else {
        (*sub).bitField0.set_stereoStrongRight(sp24.strongRight());
        (*sub).bitField0.set_stereoStrongLeft(sp24.strongLeft());
        volLeft = gDefaultPanVolume[pan as usize];
        volRight =
            gDefaultPanVolume[(0x7f as libc::c_int - pan as libc::c_int) as
                                  usize]
    }
    vel = if 0.0f32 > vel { 0.0f32 } else { vel };
    vel = if 1.0f32 < vel { 1.0f32 } else { vel };
    (*sub).targetVolLeft =
        (vel * volLeft * (0x1000 as libc::c_int as libc::c_float - 0.001f32))
            as s32 as u16_0;
    (*sub).targetVolRight =
        (vel * volRight * (0x1000 as libc::c_int as libc::c_float - 0.001f32))
            as s32 as u16_0;
    (*sub).unk_2 = (*attrs).unk_1;
    (*sub).filter = (*attrs).filter;
    (*sub).unk_07 = (*attrs).unk_14;
    (*sub).unk_0E = (*attrs).unk_16;
    (*sub).reverbVol = reverbVol;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_NoteSetResamplingRate(mut noteSubEu:
                                                         *mut NoteSubEu,
                                                     mut resamplingRateInput:
                                                         f32_0) {
    let mut resamplingRate: f32_0 = 0.0f32;
    if resamplingRateInput < 2.0f32 {
        (*noteSubEu).bitField1.set_hasTwoParts(0 as libc::c_int as u8_0);
        if 1.99998f32 < resamplingRateInput {
            resamplingRate = 1.99998f32
        } else { resamplingRate = resamplingRateInput }
    } else {
        (*noteSubEu).bitField1.set_hasTwoParts(1 as libc::c_int as u8_0);
        if 3.99996f32 < resamplingRateInput {
            resamplingRate = 1.99998f32
        } else { resamplingRate = resamplingRateInput * 0.5f32 }
    }
    (*noteSubEu).resamplingRateFixedPoint =
        (resamplingRate * 32768.0f32) as s32 as u16_0;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_NoteInit(mut note: *mut Note) {
    if (*(*note).playbackState.parentLayer).adsr.releaseRate as libc::c_int ==
           0 as libc::c_int {
        Audio_AdsrInit(&mut (*note).playbackState.adsr,
                       (*(*(*note).playbackState.parentLayer).channel).adsr.envelope,
                       &mut (*note).playbackState.adsrVolScaleUnused);
    } else {
        Audio_AdsrInit(&mut (*note).playbackState.adsr,
                       (*(*note).playbackState.parentLayer).adsr.envelope,
                       &mut (*note).playbackState.adsrVolScaleUnused);
    }
    (*note).playbackState.unk_04 = 0 as libc::c_int as u8_0;
    (*note).playbackState.adsr.action.s.set_state(ADSR_STATE_INITIAL as
                                                      libc::c_int as u8_0);
    (*note).noteSubEu = gDefaultNoteSub;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_NoteDisable(mut note: *mut Note) {
    if (*note).noteSubEu.bitField0.needsInit() as libc::c_int ==
           1 as libc::c_int {
        (*note).noteSubEu.bitField0.set_needsInit(0 as libc::c_int as u8_0)
    }
    (*note).playbackState.priority = 0 as libc::c_int as u8_0;
    (*note).noteSubEu.bitField0.set_enabled(0 as libc::c_int as u8_0);
    (*note).playbackState.unk_04 = 0 as libc::c_int as u8_0;
    (*note).noteSubEu.bitField0.set_finished(0 as libc::c_int as u8_0);
    (*note).playbackState.parentLayer =
        -(1 as libc::c_int) as *mut SequenceLayer;
    (*note).playbackState.prevParentLayer =
        -(1 as libc::c_int) as *mut SequenceLayer;
    (*note).playbackState.adsr.action.s.set_state(ADSR_STATE_DISABLED as
                                                      libc::c_int as u8_0);
    (*note).playbackState.adsr.current = 0 as libc::c_int as f32_0;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_ProcessNotes() {
    let mut current_block: u64;
    let mut pad: [s32; 2] = [0; 2];
    let mut attrs: *mut NoteAttributes = 0 as *mut NoteAttributes;
    let mut noteSubEu2: *mut NoteSubEu = 0 as *mut NoteSubEu;
    let mut noteSubEu: *mut NoteSubEu = 0 as *mut NoteSubEu;
    let mut note: *mut Note = 0 as *mut Note;
    let mut playbackState: *mut NotePlaybackState =
        0 as *mut NotePlaybackState;
    let mut subAttrs: NoteSubAttributes =
        NoteSubAttributes{reverbVol: 0,
                          unk_1: 0,
                          pan: 0,
                          stereo:
                              Stereo{s:
                                         StereoData{unused_bit2_strongRight_strongLeft_stereoHeadsetEffects_usesHeadsetPanEffects:
                                                        [0; 1],},},
                          frequency: 0.,
                          velocity: 0.,
                          unk_0C: [0; 4],
                          filter: 0 as *mut s16,
                          unk_14: 0,
                          unk_16: 0,};
    let mut bookOffset: u8_0 = 0;
    let mut scale: f32_0 = 0.;
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i < gAudioContext.numNotes {
        note = &mut *gAudioContext.notes.offset(i as isize) as *mut Note;
        noteSubEu2 =
            &mut *gAudioContext.noteSubsEu.offset((gAudioContext.noteSubEuOffset
                                                       + i) as isize) as
                *mut NoteSubEu;
        playbackState = &mut (*note).playbackState;
        if (*playbackState).parentLayer !=
               -(1 as libc::c_int) as *mut SequenceLayer {
            if ((*playbackState).parentLayer as u32_0) <
                   0x7fffffff as libc::c_int as libc::c_uint {
                current_block = 4906268039856690917;
            } else if note != (*(*playbackState).parentLayer).note &&
                          (*playbackState).unk_04 as libc::c_int ==
                              0 as libc::c_int {
                (*playbackState).adsr.action.s.set_release(1 as libc::c_int as
                                                               u8_0);
                (*playbackState).adsr.fadeOutVel =
                    gAudioContext.audioBufferParameters.updatesPerFrameInv;
                (*playbackState).priority = 1 as libc::c_int as u8_0;
                (*playbackState).unk_04 = 2 as libc::c_int as u8_0;
                current_block = 12000628879093114636;
            } else {
                if (*(*playbackState).parentLayer).enabled() == 0 &&
                       (*playbackState).unk_04 as libc::c_int ==
                           0 as libc::c_int &&
                       (*playbackState).priority as libc::c_int >=
                           1 as libc::c_int {
                    // do nothing
                    current_block = 11057878835866523405;
                } else if (*(*(*playbackState).parentLayer).channel).seqPlayer.is_null()
                 {
                    AudioSeq_SequenceChannelDisable((*(*playbackState).parentLayer).channel);
                    (*playbackState).priority = 1 as libc::c_int as u8_0;
                    (*playbackState).unk_04 = 1 as libc::c_int as u8_0;
                    current_block = 4906268039856690917;
                } else if (*(*(*(*playbackState).parentLayer).channel).seqPlayer).muted()
                              as libc::c_int != 0 &&
                              (*(*(*playbackState).parentLayer).channel).muteBehavior
                                  as libc::c_int & 0x40 as libc::c_int != 0 {
                    current_block = 11057878835866523405;
                } else { current_block = 12000628879093114636; }
                match current_block {
                    4906268039856690917 => { }
                    12000628879093114636 => { }
                    _ =>
                    // do nothing
                    {
                        Audio_SeqLayerNoteRelease((*playbackState).parentLayer);
                        Audio_AudioListRemove(&mut (*note).listItem);
                        Audio_AudioListPushFront(&mut (*(*note).listItem.pool).decaying,
                                                 &mut (*note).listItem);
                        (*playbackState).priority = 1 as libc::c_int as u8_0;
                        (*playbackState).unk_04 = 2 as libc::c_int as u8_0;
                        current_block = 12000628879093114636;
                    }
                }
            }
        } else if (*playbackState).unk_04 as libc::c_int == 0 as libc::c_int
                      &&
                      (*playbackState).priority as libc::c_int >=
                          1 as libc::c_int {
            current_block = 4906268039856690917;
        } else { current_block = 12000628879093114636; }
        match current_block {
            12000628879093114636 => {
                if (*playbackState).priority as libc::c_int !=
                       0 as libc::c_int {
                    noteSubEu = &mut (*note).noteSubEu;
                    if (*playbackState).unk_04 as libc::c_int >=
                           1 as libc::c_int ||
                           (*noteSubEu).bitField0.finished() as libc::c_int !=
                               0 {
                        if (*playbackState).adsr.action.s.state() as
                               libc::c_int ==
                               ADSR_STATE_DISABLED as libc::c_int ||
                               (*noteSubEu).bitField0.finished() as
                                   libc::c_int != 0 {
                            if (*playbackState).wantedParentLayer !=
                                   -(1 as libc::c_int) as *mut SequenceLayer {
                                Audio_NoteDisable(note);
                                if !(*(*playbackState).wantedParentLayer).channel.is_null()
                                   {
                                    Audio_NoteInitForLayer(note,
                                                           (*playbackState).wantedParentLayer);
                                    Audio_NoteVibratoInit(note);
                                    Audio_NotePortamentoInit(note);
                                    Audio_AudioListRemove(&mut (*note).listItem);
                                    AudioSeq_AudioListPushBack(&mut (*(*note).listItem.pool).active,
                                                               &mut (*note).listItem);
                                    (*playbackState).wantedParentLayer =
                                        -(1 as libc::c_int) as
                                            *mut SequenceLayer;
                                    current_block = 8869332144787829186;
                                    // don't skip
                                } else {
                                    Audio_NoteDisable(note);
                                    Audio_AudioListRemove(&mut (*note).listItem);
                                    AudioSeq_AudioListPushBack(&mut (*(*note).listItem.pool).disabled,
                                                               &mut (*note).listItem);
                                    (*playbackState).wantedParentLayer =
                                        -(1 as libc::c_int) as
                                            *mut SequenceLayer;
                                    current_block = 4906268039856690917;
                                }
                            } else {
                                if (*playbackState).parentLayer !=
                                       -(1 as libc::c_int) as
                                           *mut SequenceLayer {
                                    (*(*playbackState).parentLayer).set_bit1(1
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 u8_0)
                                }
                                Audio_NoteDisable(note);
                                Audio_AudioListRemove(&mut (*note).listItem);
                                AudioSeq_AudioListPushBack(&mut (*(*note).listItem.pool).disabled,
                                                           &mut (*note).listItem);
                                current_block = 4906268039856690917;
                            }
                        } else { current_block = 8869332144787829186; }
                    } else if (*playbackState).adsr.action.s.state() as
                                  libc::c_int ==
                                  ADSR_STATE_DISABLED as libc::c_int {
                        if (*playbackState).parentLayer !=
                               -(1 as libc::c_int) as *mut SequenceLayer {
                            (*(*playbackState).parentLayer).set_bit1(1 as
                                                                         libc::c_int
                                                                         as
                                                                         u8_0)
                        }
                        Audio_NoteDisable(note);
                        Audio_AudioListRemove(&mut (*note).listItem);
                        AudioSeq_AudioListPushBack(&mut (*(*note).listItem.pool).disabled,
                                                   &mut (*note).listItem);
                        current_block = 4906268039856690917;
                    } else { current_block = 8869332144787829186; }
                    match current_block {
                        4906268039856690917 => { }
                        _ => {
                            scale =
                                Audio_AdsrUpdate(&mut (*playbackState).adsr);
                            Audio_NoteVibratoUpdate(note);
                            attrs = &mut (*playbackState).attributes;
                            if (*playbackState).unk_04 as libc::c_int ==
                                   1 as libc::c_int ||
                                   (*playbackState).unk_04 as libc::c_int ==
                                       2 as libc::c_int {
                                subAttrs.frequency = (*attrs).freqScale;
                                subAttrs.velocity = (*attrs).velocity;
                                subAttrs.pan = (*attrs).pan;
                                subAttrs.reverbVol = (*attrs).reverb;
                                subAttrs.stereo = (*attrs).stereo;
                                subAttrs.unk_1 = (*attrs).unk_1;
                                subAttrs.filter = (*attrs).filter;
                                subAttrs.unk_14 = (*attrs).unk_4;
                                subAttrs.unk_16 = (*attrs).unk_6;
                                bookOffset =
                                    (*noteSubEu).bitField1.bookOffset()
                            } else {
                                let mut layer: *mut SequenceLayer =
                                    (*playbackState).parentLayer;
                                let mut channel: *mut SequenceChannel =
                                    (*layer).channel;
                                subAttrs.frequency = (*layer).noteFreqScale;
                                subAttrs.velocity = (*layer).noteVelocity;
                                subAttrs.pan = (*layer).notePan;
                                if (*layer).stereo.asByte as libc::c_int ==
                                       0 as libc::c_int {
                                    subAttrs.stereo = (*channel).stereo
                                } else { subAttrs.stereo = (*layer).stereo }
                                subAttrs.reverbVol = (*channel).reverb;
                                subAttrs.unk_1 = (*channel).unk_0C;
                                subAttrs.filter = (*channel).filter;
                                subAttrs.unk_14 = (*channel).unk_0F;
                                subAttrs.unk_16 = (*channel).unk_20;
                                bookOffset =
                                    ((*channel).bookOffset as libc::c_int &
                                         0x7 as libc::c_int) as u8_0;
                                if (*(*channel).seqPlayer).muted() as
                                       libc::c_int != 0 &&
                                       (*channel).muteBehavior as libc::c_int
                                           & 8 as libc::c_int != 0 {
                                    subAttrs.frequency = 0.0f32;
                                    subAttrs.velocity = 0.0f32
                                }
                            }
                            subAttrs.frequency *=
                                (*playbackState).vibratoFreqScale *
                                    (*playbackState).portamentoFreqScale;
                            subAttrs.frequency *=
                                gAudioContext.audioBufferParameters.resampleRate;
                            subAttrs.velocity *= scale;
                            Audio_InitNoteSub(note, noteSubEu2,
                                              &mut subAttrs);
                            (*noteSubEu).bitField1.set_bookOffset(bookOffset)
                        }
                    }
                }
            }
            _ => { }
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn Audio_InstrumentGetSound(mut instrument:
                                                      *mut Instrument,
                                                  mut semitone: s32)
 -> *mut SoundFontSound {
    let mut sound: *mut SoundFontSound = 0 as *mut SoundFontSound;
    if semitone < (*instrument).normalRangeLo as libc::c_int {
        sound = &mut (*instrument).lowNotesSound
    } else if semitone <= (*instrument).normalRangeHi as libc::c_int {
        sound = &mut (*instrument).normalNotesSound
    } else { sound = &mut (*instrument).highNotesSound }
    return sound;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_GetInstrumentInner(mut fontId: s32,
                                                  mut instId: s32)
 -> *mut Instrument {
    let mut inst: *mut Instrument = 0 as *mut Instrument;
    if fontId == 0xff as libc::c_int { return 0 as *mut Instrument }
    if AudioLoad_IsFontLoadComplete(fontId) == 0 {
        gAudioContext.audioErrorFlags = fontId + 0x10000000 as libc::c_int;
        return 0 as *mut Instrument
    }
    if instId >=
           (*gAudioContext.soundFonts.offset(fontId as isize)).numInstruments
               as libc::c_int {
        gAudioContext.audioErrorFlags =
            (fontId << 8 as libc::c_int) + instId + 0x3000000 as libc::c_int;
        return 0 as *mut Instrument
    }
    inst =
        *(*gAudioContext.soundFonts.offset(fontId as
                                               isize)).instruments.offset(instId
                                                                              as
                                                                              isize);
    if inst.is_null() {
        gAudioContext.audioErrorFlags =
            (fontId << 8 as libc::c_int) + instId + 0x1000000 as libc::c_int;
        return inst
    }
    return inst;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_GetDrum(mut fontId: s32, mut drumId: s32)
 -> *mut Drum {
    let mut drum: *mut Drum = 0 as *mut Drum;
    if fontId == 0xff as libc::c_int { return 0 as *mut Drum }
    if AudioLoad_IsFontLoadComplete(fontId) == 0 {
        gAudioContext.audioErrorFlags = fontId + 0x10000000 as libc::c_int;
        return 0 as *mut Drum
    }
    if drumId >=
           (*gAudioContext.soundFonts.offset(fontId as isize)).numDrums as
               libc::c_int {
        gAudioContext.audioErrorFlags =
            (fontId << 8 as libc::c_int) + drumId + 0x4000000 as libc::c_int;
        return 0 as *mut Drum
    }
    if ((*gAudioContext.soundFonts.offset(fontId as isize)).drums as u32_0) <
           0x80000000 as libc::c_uint {
        return 0 as *mut Drum
    }
    drum =
        *(*gAudioContext.soundFonts.offset(fontId as
                                               isize)).drums.offset(drumId as
                                                                        isize);
    if drum.is_null() {
        gAudioContext.audioErrorFlags =
            (fontId << 8 as libc::c_int) + drumId + 0x5000000 as libc::c_int
    }
    return drum;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_GetSfx(mut fontId: s32, mut sfxId: s32)
 -> *mut SoundFontSound {
    let mut sfx: *mut SoundFontSound = 0 as *mut SoundFontSound;
    if fontId == 0xff as libc::c_int { return 0 as *mut SoundFontSound }
    if AudioLoad_IsFontLoadComplete(fontId) == 0 {
        gAudioContext.audioErrorFlags = fontId + 0x10000000 as libc::c_int;
        return 0 as *mut SoundFontSound
    }
    if sfxId >=
           (*gAudioContext.soundFonts.offset(fontId as isize)).numSfx as
               libc::c_int {
        gAudioContext.audioErrorFlags =
            (fontId << 8 as libc::c_int) + sfxId + 0x4000000 as libc::c_int;
        return 0 as *mut SoundFontSound
    }
    if ((*gAudioContext.soundFonts.offset(fontId as isize)).soundEffects as
            u32_0) < 0x80000000 as libc::c_uint {
        return 0 as *mut SoundFontSound
    }
    sfx =
        &mut *(*gAudioContext.soundFonts.offset(fontId as
                                                    isize)).soundEffects.offset(sfxId
                                                                                    as
                                                                                    isize)
            as *mut SoundFontSound;
    if sfx.is_null() {
        gAudioContext.audioErrorFlags =
            (fontId << 8 as libc::c_int) + sfxId + 0x5000000 as libc::c_int
    }
    if (*sfx).sample.is_null() { return 0 as *mut SoundFontSound }
    return sfx;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_SetFontInstrument(mut instrumentType: s32,
                                                 mut fontId: s32,
                                                 mut index: s32,
                                                 mut value: *mut libc::c_void)
 -> s32 {
    if fontId == 0xff as libc::c_int { return -(1 as libc::c_int) }
    if AudioLoad_IsFontLoadComplete(fontId) == 0 {
        return -(2 as libc::c_int)
    }
    match instrumentType {
        0 => {
            if index >=
                   (*gAudioContext.soundFonts.offset(fontId as
                                                         isize)).numDrums as
                       libc::c_int {
                return -(3 as libc::c_int)
            }
            let ref mut fresh0 =
                *(*gAudioContext.soundFonts.offset(fontId as
                                                       isize)).drums.offset(index
                                                                                as
                                                                                isize);
            *fresh0 = value as *mut Drum
        }
        1 => {
            if index >=
                   (*gAudioContext.soundFonts.offset(fontId as isize)).numSfx
                       as libc::c_int {
                return -(3 as libc::c_int)
            }
            *(*gAudioContext.soundFonts.offset(fontId as
                                                   isize)).soundEffects.offset(index
                                                                                   as
                                                                                   isize)
                = *(value as *mut SoundFontSound)
        }
        _ => {
            if index >=
                   (*gAudioContext.soundFonts.offset(fontId as
                                                         isize)).numInstruments
                       as libc::c_int {
                return -(3 as libc::c_int)
            }
            let ref mut fresh1 =
                *(*gAudioContext.soundFonts.offset(fontId as
                                                       isize)).instruments.offset(index
                                                                                      as
                                                                                      isize);
            *fresh1 = value as *mut Instrument
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_SeqLayerDecayRelease(mut layer:
                                                        *mut SequenceLayer,
                                                    mut target: s32) {
    let mut note: *mut Note = 0 as *mut Note;
    let mut attrs: *mut NoteAttributes = 0 as *mut NoteAttributes;
    let mut chan: *mut SequenceChannel = 0 as *mut SequenceChannel;
    let mut i: s32 = 0;
    if layer == -(1 as libc::c_int) as *mut SequenceLayer { return }
    (*layer).set_bit3(0 as libc::c_int as u8_0);
    if (*layer).note.is_null() { return }
    note = (*layer).note;
    attrs = &mut (*note).playbackState.attributes;
    if (*note).playbackState.wantedParentLayer == layer {
        (*note).playbackState.wantedParentLayer =
            -(1 as libc::c_int) as *mut SequenceLayer
    }
    if (*note).playbackState.parentLayer != layer {
        if (*note).playbackState.parentLayer ==
               -(1 as libc::c_int) as *mut SequenceLayer &&
               (*note).playbackState.wantedParentLayer ==
                   -(1 as libc::c_int) as *mut SequenceLayer &&
               (*note).playbackState.prevParentLayer == layer &&
               target != ADSR_STATE_DECAY as libc::c_int {
            (*note).playbackState.adsr.fadeOutVel =
                gAudioContext.audioBufferParameters.updatesPerFrameInv;
            (*note).playbackState.adsr.action.s.set_release(1 as libc::c_int
                                                                as u8_0)
        }
        return
    }
    if (*note).playbackState.adsr.action.s.state() as libc::c_int !=
           ADSR_STATE_DECAY as libc::c_int {
        (*attrs).freqScale = (*layer).noteFreqScale;
        (*attrs).velocity = (*layer).noteVelocity;
        (*attrs).pan = (*layer).notePan;
        if !(*layer).channel.is_null() {
            chan = (*layer).channel;
            (*attrs).reverb = (*chan).reverb;
            (*attrs).unk_1 = (*chan).unk_0C;
            (*attrs).filter = (*chan).filter;
            if !(*attrs).filter.is_null() {
                i = 0 as libc::c_int;
                while i < 8 as libc::c_int {
                    (*attrs).filterBuf[i as usize] =
                        *(*attrs).filter.offset(i as isize);
                    i += 1
                }
                (*attrs).filter = (*attrs).filterBuf.as_mut_ptr()
            }
            (*attrs).unk_6 = (*chan).unk_20;
            (*attrs).unk_4 = (*chan).unk_0F;
            if (*(*chan).seqPlayer).muted() as libc::c_int != 0 &&
                   (*chan).muteBehavior as libc::c_int & 8 as libc::c_int != 0
               {
                (*note).noteSubEu.bitField0.set_finished(1 as libc::c_int as
                                                             u8_0)
            }
            if (*layer).stereo.asByte as libc::c_int == 0 as libc::c_int {
                (*attrs).stereo = (*chan).stereo
            } else { (*attrs).stereo = (*layer).stereo }
            (*note).playbackState.priority = (*chan).someOtherPriority
        } else {
            (*attrs).stereo = (*layer).stereo;
            (*note).playbackState.priority = 1 as libc::c_int as u8_0
        }
        (*note).playbackState.prevParentLayer =
            (*note).playbackState.parentLayer;
        (*note).playbackState.parentLayer =
            -(1 as libc::c_int) as *mut SequenceLayer;
        if target == ADSR_STATE_RELEASE as libc::c_int {
            (*note).playbackState.adsr.fadeOutVel =
                gAudioContext.audioBufferParameters.updatesPerFrameInv;
            (*note).playbackState.adsr.action.s.set_release(1 as libc::c_int
                                                                as u8_0);
            (*note).playbackState.unk_04 = 2 as libc::c_int as u8_0
        } else {
            (*note).playbackState.unk_04 = 1 as libc::c_int as u8_0;
            (*note).playbackState.adsr.action.s.set_decay(1 as libc::c_int as
                                                              u8_0);
            if (*layer).adsr.releaseRate as libc::c_int == 0 as libc::c_int {
                (*note).playbackState.adsr.fadeOutVel =
                    *gAudioContext.unk_3520.offset((*(*layer).channel).adsr.releaseRate
                                                       as isize)
            } else {
                (*note).playbackState.adsr.fadeOutVel =
                    *gAudioContext.unk_3520.offset((*layer).adsr.releaseRate
                                                       as isize)
            }
            (*note).playbackState.adsr.sustain =
                (*(*layer).channel).adsr.sustain as s32 as f32_0 *
                    (*note).playbackState.adsr.current / 256.0f32
        }
    }
    if target == ADSR_STATE_DECAY as libc::c_int {
        Audio_AudioListRemove(&mut (*note).listItem);
        Audio_AudioListPushFront(&mut (*(*note).listItem.pool).decaying,
                                 &mut (*note).listItem);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Audio_SeqLayerNoteDecay(mut layer:
                                                     *mut SequenceLayer) {
    Audio_SeqLayerDecayRelease(layer, ADSR_STATE_DECAY as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Audio_SeqLayerNoteRelease(mut layer:
                                                       *mut SequenceLayer) {
    Audio_SeqLayerDecayRelease(layer, ADSR_STATE_RELEASE as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Audio_BuildSyntheticWave(mut note: *mut Note,
                                                  mut layer:
                                                      *mut SequenceLayer,
                                                  mut waveId: s32) -> s32 {
    let mut freqScale: f32_0 = 0.;
    let mut ratio: f32_0 = 0.;
    let mut sampleCountIndex: u8_0 = 0;
    if waveId < 128 as libc::c_int { waveId = 128 as libc::c_int }
    freqScale = (*layer).freqScale;
    if (*layer).portamento.mode as libc::c_int != 0 as libc::c_int &&
           0.0f32 < (*layer).portamento.extent {
        freqScale *= (*layer).portamento.extent + 1.0f32
    }
    if freqScale < 0.99999f32 {
        sampleCountIndex = 0 as libc::c_int as u8_0;
        ratio = 1.0465f32
    } else if freqScale < 1.99999f32 {
        sampleCountIndex = 1 as libc::c_int as u8_0;
        ratio = 0.52325f32
    } else if freqScale < 3.99999f32 {
        sampleCountIndex = 2 as libc::c_int as u8_0;
        ratio = 0.26263f32
    } else { sampleCountIndex = 3 as libc::c_int as u8_0; ratio = 0.13081f32 }
    (*layer).freqScale *= ratio;
    (*note).playbackState.waveId = waveId as u8_0;
    (*note).playbackState.sampleCountIndex = sampleCountIndex;
    (*note).noteSubEu.sound.samples =
        &mut *(*gWaveSamples.as_mut_ptr().offset((waveId - 128 as libc::c_int)
                                                     as
                                                     isize)).offset((sampleCountIndex
                                                                         as
                                                                         libc::c_int
                                                                         *
                                                                         64 as
                                                                             libc::c_int)
                                                                        as
                                                                        isize)
            as *mut s16;
    return sampleCountIndex as s32;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_InitSyntheticWave(mut note: *mut Note,
                                                 mut layer:
                                                     *mut SequenceLayer) {
    let mut sampleCountIndex: s32 = 0;
    let mut waveSampleCountIndex: s32 = 0;
    let mut waveId: s32 = (*layer).instOrWave as s32;
    if waveId == 0xff as libc::c_int {
        waveId = (*(*layer).channel).instOrWave as s32
    }
    sampleCountIndex = (*note).playbackState.sampleCountIndex as s32;
    waveSampleCountIndex = Audio_BuildSyntheticWave(note, layer, waveId);
    if waveSampleCountIndex != sampleCountIndex {
        (*note).noteSubEu.unk_06 =
            (waveSampleCountIndex * 4 as libc::c_int + sampleCountIndex) as
                u8_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn Audio_InitNoteList(mut list: *mut AudioListItem) {
    (*list).prev = list;
    (*list).next = list;
    (*list).u.count = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_InitNoteLists(mut pool: *mut NotePool) {
    Audio_InitNoteList(&mut (*pool).disabled);
    Audio_InitNoteList(&mut (*pool).decaying);
    Audio_InitNoteList(&mut (*pool).releasing);
    Audio_InitNoteList(&mut (*pool).active);
    (*pool).disabled.pool = pool;
    (*pool).decaying.pool = pool;
    (*pool).releasing.pool = pool;
    (*pool).active.pool = pool;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_InitNoteFreeList() {
    let mut i: s32 = 0;
    Audio_InitNoteLists(&mut gAudioContext.noteFreeLists);
    i = 0 as libc::c_int;
    while i < gAudioContext.numNotes {
        let ref mut fresh2 =
            (*gAudioContext.notes.offset(i as isize)).listItem.u.value;
        *fresh2 =
            &mut *gAudioContext.notes.offset(i as isize) as *mut Note as
                *mut libc::c_void;
        let ref mut fresh3 =
            (*gAudioContext.notes.offset(i as isize)).listItem.prev;
        *fresh3 = 0 as *mut AudioListItem;
        AudioSeq_AudioListPushBack(&mut gAudioContext.noteFreeLists.disabled,
                                   &mut (*gAudioContext.notes.offset(i as
                                                                         isize)).listItem);
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn Audio_NotePoolClear(mut pool: *mut NotePool) {
    let mut i: s32 = 0;
    let mut source: *mut AudioListItem = 0 as *mut AudioListItem;
    let mut cur: *mut AudioListItem = 0 as *mut AudioListItem;
    let mut dest: *mut AudioListItem = 0 as *mut AudioListItem;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        match i {
            0 => {
                source = &mut (*pool).disabled;
                dest = &mut gAudioContext.noteFreeLists.disabled
            }
            1 => {
                source = &mut (*pool).decaying;
                dest = &mut gAudioContext.noteFreeLists.decaying
            }
            2 => {
                source = &mut (*pool).releasing;
                dest = &mut gAudioContext.noteFreeLists.releasing
            }
            3 => {
                source = &mut (*pool).active;
                dest = &mut gAudioContext.noteFreeLists.active
            }
            _ => { }
        }
        loop  {
            cur = (*source).next;
            if cur == source || cur.is_null() { break ; }
            Audio_AudioListRemove(cur);
            AudioSeq_AudioListPushBack(dest, cur);
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn Audio_NotePoolFill(mut pool: *mut NotePool,
                                            mut count: s32) {
    let mut i: s32 = 0;
    let mut j: s32 = 0;
    let mut note: *mut Note = 0 as *mut Note;
    let mut source: *mut AudioListItem = 0 as *mut AudioListItem;
    let mut dest: *mut AudioListItem = 0 as *mut AudioListItem;
    Audio_NotePoolClear(pool);
    i = 0 as libc::c_int;
    j = 0 as libc::c_int;
    while j < count {
        if i == 4 as libc::c_int { return }
        match i {
            0 => {
                source = &mut gAudioContext.noteFreeLists.disabled;
                dest = &mut (*pool).disabled
            }
            1 => {
                source = &mut gAudioContext.noteFreeLists.decaying;
                dest = &mut (*pool).decaying
            }
            2 => {
                source = &mut gAudioContext.noteFreeLists.releasing;
                dest = &mut (*pool).releasing
            }
            3 => {
                source = &mut gAudioContext.noteFreeLists.active;
                dest = &mut (*pool).active
            }
            _ => { }
        }
        while j < count {
            note = AudioSeq_AudioListPopBack(source) as *mut Note;
            if note.is_null() { break ; }
            AudioSeq_AudioListPushBack(dest, &mut (*note).listItem);
            j += 1
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn Audio_AudioListPushFront(mut list:
                                                      *mut AudioListItem,
                                                  mut item:
                                                      *mut AudioListItem) {
    // add 'item' to the front of the list given by 'list', if it's not in any list
    if (*item).prev.is_null() {
        (*item).prev = list;
        (*item).next = (*list).next;
        (*(*list).next).prev = item;
        (*list).next = item;
        (*list).u.count += 1;
        (*item).pool = (*list).pool
    };
}
#[no_mangle]
pub unsafe extern "C" fn Audio_AudioListRemove(mut item: *mut AudioListItem) {
    // remove 'item' from the list it's in, if any
    if !(*item).prev.is_null() {
        (*(*item).prev).next = (*item).next;
        (*(*item).next).prev = (*item).prev;
        (*item).prev = 0 as *mut AudioListItem
    };
}
#[no_mangle]
pub unsafe extern "C" fn Audio_FindNodeWithPrioLessThan(mut list:
                                                            *mut AudioListItem,
                                                        mut limit: s32)
 -> *mut Note {
    let mut cur: *mut AudioListItem = (*list).next;
    let mut best: *mut AudioListItem = 0 as *mut AudioListItem;
    if cur == list { return 0 as *mut Note }
    best = cur;
    while cur != list {
        if (*((*best).u.value as *mut Note)).playbackState.priority as
               libc::c_int >=
               (*((*cur).u.value as *mut Note)).playbackState.priority as
                   libc::c_int {
            best = cur
        }
        cur = (*cur).next
    }
    if best.is_null() { return 0 as *mut Note }
    if limit <=
           (*((*best).u.value as *mut Note)).playbackState.priority as
               libc::c_int {
        return 0 as *mut Note
    }
    return (*best).u.value as *mut Note;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_NoteInitForLayer(mut note: *mut Note,
                                                mut layer:
                                                    *mut SequenceLayer) {
    let mut pad: [s32; 3] = [0; 3];
    let mut instId: s16 = 0;
    let mut playback: *mut NotePlaybackState = &mut (*note).playbackState;
    let mut sub: *mut NoteSubEu = &mut (*note).noteSubEu;
    (*note).playbackState.prevParentLayer =
        -(1 as libc::c_int) as *mut SequenceLayer;
    (*note).playbackState.parentLayer = layer;
    (*playback).priority = (*(*layer).channel).notePriority;
    (*layer).set_notePropertiesNeedInit(1 as libc::c_int as u8_0);
    (*layer).set_bit3(1 as libc::c_int as u8_0);
    (*layer).note = note;
    (*(*layer).channel).noteUnused = note;
    (*(*layer).channel).layerUnused = layer;
    (*layer).noteVelocity = 0.0f32;
    Audio_NoteInit(note);
    instId = (*layer).instOrWave as s16;
    if instId as libc::c_int == 0xff as libc::c_int {
        instId = (*(*layer).channel).instOrWave
    }
    (*sub).sound.soundFontSound = (*layer).sound;
    if instId as libc::c_int >= 0x80 as libc::c_int &&
           (instId as libc::c_int) < 0xc0 as libc::c_int {
        (*sub).bitField1.set_isSyntheticWave(1 as libc::c_int as u8_0)
    } else { (*sub).bitField1.set_isSyntheticWave(0 as libc::c_int as u8_0) }
    if (*sub).bitField1.isSyntheticWave() != 0 {
        Audio_BuildSyntheticWave(note, layer, instId as s32);
    }
    (*playback).fontId = (*(*layer).channel).fontId;
    (*playback).stereoHeadsetEffects =
        (*(*layer).channel).stereoHeadsetEffects();
    (*sub).bitField1.set_reverbIndex(((*(*layer).channel).reverbIndex as
                                          libc::c_int & 3 as libc::c_int) as
                                         u8_0);
}
#[no_mangle]
pub unsafe extern "C" fn func_800E82C0(mut note: *mut Note,
                                       mut layer: *mut SequenceLayer) {
    // similar to Audio_NoteReleaseAndTakeOwnership, hard to say what the difference is
    Audio_SeqLayerNoteRelease((*note).playbackState.parentLayer);
    (*note).playbackState.wantedParentLayer = layer;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_NoteReleaseAndTakeOwnership(mut note:
                                                               *mut Note,
                                                           mut layer:
                                                               *mut SequenceLayer) {
    (*note).playbackState.wantedParentLayer = layer;
    (*note).playbackState.priority = (*(*layer).channel).notePriority;
    (*note).playbackState.adsr.fadeOutVel =
        gAudioContext.audioBufferParameters.updatesPerFrameInv;
    (*note).playbackState.adsr.action.s.set_release(1 as libc::c_int as u8_0);
}
#[no_mangle]
pub unsafe extern "C" fn Audio_AllocNoteFromDisabled(mut pool: *mut NotePool,
                                                     mut layer:
                                                         *mut SequenceLayer)
 -> *mut Note {
    let mut note: *mut Note =
        AudioSeq_AudioListPopBack(&mut (*pool).disabled) as *mut Note;
    if !note.is_null() {
        Audio_NoteInitForLayer(note, layer);
        Audio_AudioListPushFront(&mut (*pool).active, &mut (*note).listItem);
    }
    return note;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_AllocNoteFromDecaying(mut pool: *mut NotePool,
                                                     mut layer:
                                                         *mut SequenceLayer)
 -> *mut Note {
    let mut note: *mut Note =
        AudioSeq_AudioListPopBack(&mut (*pool).decaying) as *mut Note;
    if !note.is_null() {
        Audio_NoteReleaseAndTakeOwnership(note, layer);
        AudioSeq_AudioListPushBack(&mut (*pool).releasing,
                                   &mut (*note).listItem);
    }
    return note;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_AllocNoteFromActive(mut pool: *mut NotePool,
                                                   mut layer:
                                                       *mut SequenceLayer)
 -> *mut Note {
    let mut rNote: *mut Note = 0 as *mut Note;
    let mut aNote: *mut Note = 0 as *mut Note;
    let mut rPriority: s32 = 0;
    let mut aPriority: s32 = 0;
    aPriority = 0x10 as libc::c_int;
    rPriority = aPriority;
    rNote =
        Audio_FindNodeWithPrioLessThan(&mut (*pool).releasing,
                                       (*(*layer).channel).notePriority as
                                           s32);
    if !rNote.is_null() { rPriority = (*rNote).playbackState.priority as s32 }
    aNote =
        Audio_FindNodeWithPrioLessThan(&mut (*pool).active,
                                       (*(*layer).channel).notePriority as
                                           s32);
    if !aNote.is_null() { aPriority = (*aNote).playbackState.priority as s32 }
    if rNote.is_null() && aNote.is_null() { return 0 as *mut Note }
    if aPriority < rPriority {
        Audio_AudioListRemove(&mut (*aNote).listItem);
        func_800E82C0(aNote, layer);
        AudioSeq_AudioListPushBack(&mut (*pool).releasing,
                                   &mut (*aNote).listItem);
        (*aNote).playbackState.priority = (*(*layer).channel).notePriority;
        return aNote
    }
    (*rNote).playbackState.wantedParentLayer = layer;
    (*rNote).playbackState.priority = (*(*layer).channel).notePriority;
    return rNote;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_AllocNote(mut layer: *mut SequenceLayer)
 -> *mut Note {
    let mut ret: *mut Note = 0 as *mut Note;
    let mut policy: u32_0 = (*(*layer).channel).noteAllocPolicy as u32_0;
    if policy & 1 as libc::c_int as libc::c_uint != 0 {
        ret = (*layer).note;
        if !ret.is_null() && (*ret).playbackState.prevParentLayer == layer &&
               (*ret).playbackState.wantedParentLayer ==
                   -(1 as libc::c_int) as *mut SequenceLayer {
            Audio_NoteReleaseAndTakeOwnership(ret, layer);
            Audio_AudioListRemove(&mut (*ret).listItem);
            AudioSeq_AudioListPushBack(&mut (*(*ret).listItem.pool).releasing,
                                       &mut (*ret).listItem);
            return ret
        }
    }
    if policy & 2 as libc::c_int as libc::c_uint != 0 {
        ret =
            Audio_AllocNoteFromDisabled(&mut (*(*layer).channel).notePool,
                                        layer);
        if !(ret.is_null() &&
                 {
                     ret =
                         Audio_AllocNoteFromDecaying(&mut (*(*layer).channel).notePool,
                                                     layer);
                     ret.is_null()
                 } &&
                 {
                     ret =
                         Audio_AllocNoteFromActive(&mut (*(*layer).channel).notePool,
                                                   layer);
                     ret.is_null()
                 }) {
            return ret
        }
    } else if policy & 4 as libc::c_int as libc::c_uint != 0 {
        ret =
            Audio_AllocNoteFromDisabled(&mut (*(*layer).channel).notePool,
                                        layer);
        if !(ret.is_null() &&
                 {
                     ret =
                         Audio_AllocNoteFromDisabled(&mut (*(*(*layer).channel).seqPlayer).notePool,
                                                     layer);
                     ret.is_null()
                 } &&
                 {
                     ret =
                         Audio_AllocNoteFromDecaying(&mut (*(*layer).channel).notePool,
                                                     layer);
                     ret.is_null()
                 } &&
                 {
                     ret =
                         Audio_AllocNoteFromDecaying(&mut (*(*(*layer).channel).seqPlayer).notePool,
                                                     layer);
                     ret.is_null()
                 } &&
                 {
                     ret =
                         Audio_AllocNoteFromActive(&mut (*(*layer).channel).notePool,
                                                   layer);
                     ret.is_null()
                 } &&
                 {
                     ret =
                         Audio_AllocNoteFromActive(&mut (*(*(*layer).channel).seqPlayer).notePool,
                                                   layer);
                     ret.is_null()
                 }) {
            return ret
        }
    } else if policy & 8 as libc::c_int as libc::c_uint != 0 {
        ret =
            Audio_AllocNoteFromDisabled(&mut gAudioContext.noteFreeLists,
                                        layer);
        if !(ret.is_null() &&
                 {
                     ret =
                         Audio_AllocNoteFromDecaying(&mut gAudioContext.noteFreeLists,
                                                     layer);
                     ret.is_null()
                 } &&
                 {
                     ret =
                         Audio_AllocNoteFromActive(&mut gAudioContext.noteFreeLists,
                                                   layer);
                     ret.is_null()
                 }) {
            return ret
        }
    } else {
        ret =
            Audio_AllocNoteFromDisabled(&mut (*(*layer).channel).notePool,
                                        layer);
        if !(ret.is_null() &&
                 {
                     ret =
                         Audio_AllocNoteFromDisabled(&mut (*(*(*layer).channel).seqPlayer).notePool,
                                                     layer);
                     ret.is_null()
                 } &&
                 {
                     ret =
                         Audio_AllocNoteFromDisabled(&mut gAudioContext.noteFreeLists,
                                                     layer);
                     ret.is_null()
                 } &&
                 {
                     ret =
                         Audio_AllocNoteFromDecaying(&mut (*(*layer).channel).notePool,
                                                     layer);
                     ret.is_null()
                 } &&
                 {
                     ret =
                         Audio_AllocNoteFromDecaying(&mut (*(*(*layer).channel).seqPlayer).notePool,
                                                     layer);
                     ret.is_null()
                 } &&
                 {
                     ret =
                         Audio_AllocNoteFromDecaying(&mut gAudioContext.noteFreeLists,
                                                     layer);
                     ret.is_null()
                 } &&
                 {
                     ret =
                         Audio_AllocNoteFromActive(&mut (*(*layer).channel).notePool,
                                                   layer);
                     ret.is_null()
                 } &&
                 {
                     ret =
                         Audio_AllocNoteFromActive(&mut (*(*(*layer).channel).seqPlayer).notePool,
                                                   layer);
                     ret.is_null()
                 } &&
                 {
                     ret =
                         Audio_AllocNoteFromActive(&mut gAudioContext.noteFreeLists,
                                                   layer);
                     ret.is_null()
                 }) {
            return ret
        }
    }
    (*layer).set_bit3(1 as libc::c_int as u8_0);
    return 0 as *mut Note;
}
#[no_mangle]
pub unsafe extern "C" fn Audio_NoteInitAll() {
    let mut note: *mut Note = 0 as *mut Note;
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i < gAudioContext.numNotes {
        note = &mut *gAudioContext.notes.offset(i as isize) as *mut Note;
        (*note).noteSubEu = gZeroNoteSub;
        (*note).playbackState.priority = 0 as libc::c_int as u8_0;
        (*note).playbackState.unk_04 = 0 as libc::c_int as u8_0;
        (*note).playbackState.parentLayer =
            -(1 as libc::c_int) as *mut SequenceLayer;
        (*note).playbackState.wantedParentLayer =
            -(1 as libc::c_int) as *mut SequenceLayer;
        (*note).playbackState.prevParentLayer =
            -(1 as libc::c_int) as *mut SequenceLayer;
        (*note).playbackState.waveId = 0 as libc::c_int as u8_0;
        (*note).playbackState.attributes.velocity = 0.0f32;
        (*note).playbackState.adsrVolScaleUnused = 0 as libc::c_int as s16;
        (*note).playbackState.adsr.action.asByte = 0 as libc::c_int as u8_0;
        (*note).vibratoState.active = 0 as libc::c_int as u8_0;
        (*note).portamento.cur = 0 as libc::c_int as u16_0;
        (*note).portamento.speed = 0 as libc::c_int as u16_0;
        (*note).playbackState.stereoHeadsetEffects = 0 as libc::c_int as u8_0;
        (*note).unk_BC = 0 as libc::c_int as u32_0;
        (*note).synthesisState.synthesisBuffers =
            AudioHeap_AllocDmaMemory(&mut gAudioContext.notesAndBuffersPool,
                                     0x1e0 as libc::c_int as u32_0) as
                *mut NoteSynthesisBuffers;
        i += 1
    };
}
