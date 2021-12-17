#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn osSetIntMask(_: OSIntMask) -> OSIntMask;
    #[no_mangle]
    fn AudioLoad_InitSampleDmaBuffers(arg0: s32);
    #[no_mangle]
    fn AudioLoad_IsFontLoadComplete(fontId: s32) -> s32;
    #[no_mangle]
    fn AudioLoad_SetFontLoadStatus(fontId: s32, status: s32);
    #[no_mangle]
    fn AudioLoad_InitSlowLoads();
    #[no_mangle]
    fn AudioLoad_InitAsyncLoads();
    #[no_mangle]
    fn AudioLoad_LoadPermanentSamples();
    #[no_mangle]
    fn AudioLoad_InitScriptLoads();
    #[no_mangle]
    fn Audio_WritebackDCache(mem: *mut libc::c_void, size: s32);
    #[no_mangle]
    fn Audio_NoteDisable(note: *mut Note);
    #[no_mangle]
    fn Audio_GetInstrumentInner(fontId: s32, instId: s32) -> *mut Instrument;
    #[no_mangle]
    fn Audio_GetDrum(fontId: s32, drumId: s32) -> *mut Drum;
    #[no_mangle]
    fn Audio_GetSfx(fontId: s32, sfxId: s32) -> *mut SoundFontSound;
    #[no_mangle]
    fn Audio_InitNoteFreeList();
    #[no_mangle]
    fn Audio_AudioListRemove(item: *mut AudioListItem);
    #[no_mangle]
    fn Audio_NoteInitAll();
    #[no_mangle]
    fn AudioSeq_SequencePlayerDisableAsFinished(seqPlayer:
                                                    *mut SequencePlayer);
    #[no_mangle]
    fn AudioSeq_SequencePlayerDisable(seqPlayer: *mut SequencePlayer);
    #[no_mangle]
    fn AudioSeq_AudioListPushBack(list: *mut AudioListItem,
                                  item: *mut AudioListItem);
    #[no_mangle]
    fn AudioSeq_ResetSequencePlayer(seqPlayer: *mut SequencePlayer);
    #[no_mangle]
    fn AudioSeq_InitSequencePlayerChannels(playerIdx: s32);
    #[no_mangle]
    fn AudioSeq_InitSequencePlayers();
    #[no_mangle]
    static mut gAudioContext: AudioContext;
    #[no_mangle]
    static mut sHighPassFilterData: [s16; 120];
    #[no_mangle]
    static mut sLowPassFilterData: [s16; 128];
    #[no_mangle]
    fn osWritebackDCacheAll();
    #[no_mangle]
    static D_8014A6C0: [s16; 0];
    #[no_mangle]
    fn osAiSetFrequency(frequency: u32_0) -> s32;
    #[no_mangle]
    static mut gAudioSpecs: [AudioSpec; 18];
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
pub type OSIntMask = u32_0;
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
pub type C2RustUnnamed_1 = libc::c_uint;
pub const MEDIUM_DISK_DRIVE: C2RustUnnamed_1 = 3;
pub const MEDIUM_CART: C2RustUnnamed_1 = 2;
pub const MEDIUM_UNK: C2RustUnnamed_1 = 1;
pub const MEDIUM_RAM: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const CODEC_S16: C2RustUnnamed_2 = 5;
pub const CODEC_REVERB: C2RustUnnamed_2 = 4;
pub const CODEC_SMALL_ADPCM: C2RustUnnamed_2 = 3;
pub const CODEC_S16_INMEMORY: C2RustUnnamed_2 = 2;
pub const CODEC_S8: C2RustUnnamed_2 = 1;
pub const CODEC_ADPCM: C2RustUnnamed_2 = 0;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const SAMPLE_TABLE: C2RustUnnamed_3 = 2;
pub const FONT_TABLE: C2RustUnnamed_3 = 1;
pub const SEQUENCE_TABLE: C2RustUnnamed_3 = 0;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const CACHE_PERMANENT: C2RustUnnamed_4 = 3;
pub const CACHE_EITHER: C2RustUnnamed_4 = 2;
pub const CACHE_PERSISTENT: C2RustUnnamed_4 = 1;
pub const CACHE_TEMPORARY: C2RustUnnamed_4 = 0;
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
    pub bitField0: C2RustUnnamed_7,
    pub bitField1: C2RustUnnamed_6,
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
    pub sound: C2RustUnnamed_5,
    pub filter: *mut s16,
    pub pad_18: [libc::c_char; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
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
pub struct C2RustUnnamed_6 {
    #[bitfield(name = "reverbIndex", ty = "u8_0", bits = "0..=2")]
    #[bitfield(name = "bookOffset", ty = "u8_0", bits = "3..=4")]
    #[bitfield(name = "isSyntheticWave", ty = "u8_0", bits = "5..=5")]
    #[bitfield(name = "hasTwoParts", ty = "u8_0", bits = "6..=6")]
    #[bitfield(name = "usesHeadsetPanEffects2", ty = "u8_0", bits = "7..=7")]
    pub reverbIndex_bookOffset_isSyntheticWave_hasTwoParts_usesHeadsetPanEffects2: [u8; 1],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
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
    pub changes: C2RustUnnamed_9,
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
    pub u: C2RustUnnamed_8,
    pub pool: *mut NotePool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
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
pub union C2RustUnnamed_9 {
    pub s: C2RustUnnamed_10,
    pub asByte: u8_0,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
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
    pub action: C2RustUnnamed_11,
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
pub union C2RustUnnamed_11 {
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
    pub c2rust_unnamed: C2RustUnnamed_13,
    pub c2rust_unnamed_0: C2RustUnnamed_12,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
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
pub union C2RustUnnamed_13 {
    pub opArgs: u32_0,
    pub c2rust_unnamed: C2RustUnnamed_14,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
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
pub struct StorageChange {
    pub oldAddr: u32_0,
    pub newAddr: u32_0,
    pub size: u32_0,
    pub newMedium: u8_0,
}
#[no_mangle]
pub unsafe extern "C" fn func_800DDE20(mut arg0: f32_0) -> f32_0 {
    return 256.0f32 *
               gAudioContext.audioBufferParameters.unkUpdatesPerFrameScaled /
               arg0;
}
#[no_mangle]
pub unsafe extern "C" fn func_800DDE3C() {
    let mut i: s32 = 0;
    *gAudioContext.unk_3520.offset(255 as libc::c_int as isize) =
        func_800DDE20(0.25f32);
    *gAudioContext.unk_3520.offset(254 as libc::c_int as isize) =
        func_800DDE20(0.33f32);
    *gAudioContext.unk_3520.offset(253 as libc::c_int as isize) =
        func_800DDE20(0.5f32);
    *gAudioContext.unk_3520.offset(252 as libc::c_int as isize) =
        func_800DDE20(0.66f32);
    *gAudioContext.unk_3520.offset(251 as libc::c_int as isize) =
        func_800DDE20(0.75f32);
    i = 128 as libc::c_int;
    while i < 251 as libc::c_int {
        *gAudioContext.unk_3520.offset(i as isize) =
            func_800DDE20((251 as libc::c_int - i) as f32_0);
        i += 1
    }
    i = 16 as libc::c_int;
    while i < 128 as libc::c_int {
        *gAudioContext.unk_3520.offset(i as isize) =
            func_800DDE20((4 as libc::c_int * (143 as libc::c_int - i)) as
                              f32_0);
        i += 1
    }
    i = 1 as libc::c_int;
    while i < 16 as libc::c_int {
        *gAudioContext.unk_3520.offset(i as isize) =
            func_800DDE20((60 as libc::c_int * (23 as libc::c_int - i)) as
                              f32_0);
        i += 1
    }
    *gAudioContext.unk_3520.offset(0 as libc::c_int as isize) = 0.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_ResetLoadStatus() {
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i < 0x30 as libc::c_int {
        if gAudioContext.fontLoadStatus[i as usize] as libc::c_int !=
               5 as libc::c_int {
            gAudioContext.fontLoadStatus[i as usize] =
                0 as libc::c_int as u8_0
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 0x30 as libc::c_int {
        if gAudioContext.sampleFontLoadStatus[i as usize] as libc::c_int !=
               5 as libc::c_int {
            gAudioContext.sampleFontLoadStatus[i as usize] =
                0 as libc::c_int as u8_0
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 0x80 as libc::c_int {
        if gAudioContext.seqLoadStatus[i as usize] as libc::c_int !=
               5 as libc::c_int {
            gAudioContext.seqLoadStatus[i as usize] = 0 as libc::c_int as u8_0
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_DiscardFont(mut fontId: s32) {
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i < gAudioContext.numNotes {
        let mut note: *mut Note =
            &mut *gAudioContext.notes.offset(i as isize) as *mut Note;
        if (*note).playbackState.fontId as libc::c_int == fontId {
            if (*note).playbackState.unk_04 as libc::c_int == 0 as libc::c_int
                   &&
                   (*note).playbackState.priority as libc::c_int !=
                       0 as libc::c_int {
                (*(*note).playbackState.parentLayer).set_enabled(0 as
                                                                     libc::c_int
                                                                     as u8_0);
                (*(*note).playbackState.parentLayer).set_finished(1 as
                                                                      libc::c_int
                                                                      as u8_0)
            }
            Audio_NoteDisable(note);
            Audio_AudioListRemove(&mut (*note).listItem);
            AudioSeq_AudioListPushBack(&mut gAudioContext.noteFreeLists.disabled,
                                       &mut (*note).listItem);
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_ReleaseNotesForFont(mut fontId: s32) {
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i < gAudioContext.numNotes {
        let mut note: *mut Note =
            &mut *gAudioContext.notes.offset(i as isize) as *mut Note;
        let mut state: *mut NotePlaybackState = &mut (*note).playbackState;
        if (*state).fontId as libc::c_int == fontId {
            if (*state).priority as libc::c_int != 0 as libc::c_int &&
                   (*state).adsr.action.s.state() as libc::c_int ==
                       ADSR_STATE_DECAY as libc::c_int {
                (*state).priority = 1 as libc::c_int as u8_0;
                (*state).adsr.fadeOutVel =
                    gAudioContext.audioBufferParameters.updatesPerFrameInv;
                (*state).adsr.action.s.set_release(1 as libc::c_int as u8_0)
            }
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_DiscardSequence(mut seqId: s32) {
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i <
              gAudioContext.audioBufferParameters.numSequencePlayers as
                  libc::c_int {
        if gAudioContext.seqPlayers[i as usize].enabled() as libc::c_int != 0
               &&
               gAudioContext.seqPlayers[i as usize].seqId as libc::c_int ==
                   seqId {
            AudioSeq_SequencePlayerDisable(&mut *gAudioContext.seqPlayers.as_mut_ptr().offset(i
                                                                                                  as
                                                                                                  isize));
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_WritebackDCache(mut mem: *mut libc::c_void,
                                                   mut size: u32_0) {
    Audio_WritebackDCache(mem, size as s32);
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_AllocZeroedAttemptExternal(mut pool:
                                                                  *mut AudioAllocPool,
                                                              mut size: u32_0)
 -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    if !gAudioContext.externalPool.start.is_null() {
        ret = AudioHeap_AllocZeroed(&mut gAudioContext.externalPool, size)
    }
    if ret.is_null() { ret = AudioHeap_AllocZeroed(pool, size) }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_AllocAttemptExternal(mut pool:
                                                            *mut AudioAllocPool,
                                                        mut size: u32_0)
 -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    if !gAudioContext.externalPool.start.is_null() {
        ret = AudioHeap_Alloc(&mut gAudioContext.externalPool, size)
    }
    if ret.is_null() { ret = AudioHeap_Alloc(pool, size) }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_AllocDmaMemory(mut pool:
                                                      *mut AudioAllocPool,
                                                  mut size: u32_0)
 -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    ret = AudioHeap_Alloc(pool, size);
    if !ret.is_null() { AudioHeap_WritebackDCache(ret, size); }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_AllocDmaMemoryZeroed(mut pool:
                                                            *mut AudioAllocPool,
                                                        mut size: u32_0)
 -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    ret = AudioHeap_AllocZeroed(pool, size);
    if !ret.is_null() { AudioHeap_WritebackDCache(ret, size); }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_AllocZeroed(mut pool: *mut AudioAllocPool,
                                               mut size: u32_0)
 -> *mut libc::c_void {
    let mut ret: *mut u8_0 = AudioHeap_Alloc(pool, size) as *mut u8_0;
    let mut ptr: *mut u8_0 = 0 as *mut u8_0;
    if !ret.is_null() {
        ptr = ret;
        while ptr < (*pool).cur {
            *ptr = 0 as libc::c_int as u8_0;
            ptr = ptr.offset(1)
        }
    }
    return ret as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_Alloc(mut pool: *mut AudioAllocPool,
                                         mut size: u32_0)
 -> *mut libc::c_void {
    let mut aligned: u32_0 =
        size.wrapping_add(0xf as libc::c_int as libc::c_uint) &
            !(0xf as libc::c_int) as libc::c_uint;
    let mut ret: *mut u8_0 = (*pool).cur;
    if (*pool).start.offset((*pool).size as isize) >=
           (*pool).cur.offset(aligned as isize) {
        (*pool).cur = (*pool).cur.offset(aligned as isize)
    } else { return 0 as *mut libc::c_void }
    (*pool).count += 1;
    return ret as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_AllocPoolInit(mut pool:
                                                     *mut AudioAllocPool,
                                                 mut mem: *mut libc::c_void,
                                                 mut size: u32_0) {
    (*pool).start =
        ((mem as u32_0).wrapping_add(0xf as libc::c_int as libc::c_uint) &
             !(0xf as libc::c_int) as libc::c_uint) as *mut u8_0;
    (*pool).cur = (*pool).start;
    (*pool).size =
        size.wrapping_sub(mem as u32_0 & 0xf as libc::c_int as libc::c_uint)
            as s32;
    (*pool).count = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_PersistentCacheClear(mut persistent:
                                                            *mut AudioPersistentCache) {
    (*persistent).pool.count = 0 as libc::c_int;
    (*persistent).numEntries = 0 as libc::c_int as u32_0;
    (*persistent).pool.cur = (*persistent).pool.start;
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_TemporaryCacheClear(mut temporary:
                                                           *mut AudioTemporaryCache) {
    (*temporary).pool.count = 0 as libc::c_int;
    (*temporary).pool.cur = (*temporary).pool.start;
    (*temporary).nextSide = 0 as libc::c_int as u32_0;
    (*temporary).entries[0 as libc::c_int as usize].ptr =
        (*temporary).pool.start;
    (*temporary).entries[1 as libc::c_int as usize].ptr =
        (*temporary).pool.start.offset((*temporary).pool.size as isize);
    (*temporary).entries[0 as libc::c_int as usize].id =
        -(1 as libc::c_int) as s16;
    (*temporary).entries[1 as libc::c_int as usize].id =
        -(1 as libc::c_int) as s16;
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_ResetPool(mut pool: *mut AudioAllocPool) {
    (*pool).count = 0 as libc::c_int;
    (*pool).cur = (*pool).start;
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_PopCache(mut tableType: s32) {
    let mut loadedPool: *mut AudioCache = 0 as *mut AudioCache;
    let mut persistentPool: *mut AudioAllocPool = 0 as *mut AudioAllocPool;
    let mut persistent: *mut AudioPersistentCache =
        0 as *mut AudioPersistentCache;
    let mut entryPtr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut table: *mut u8_0 = 0 as *mut u8_0;
    match tableType {
        0 => {
            loadedPool = &mut gAudioContext.seqCache;
            table = gAudioContext.seqLoadStatus.as_mut_ptr()
        }
        1 => {
            loadedPool = &mut gAudioContext.fontCache;
            table = gAudioContext.fontLoadStatus.as_mut_ptr()
        }
        2 => {
            loadedPool = &mut gAudioContext.sampleBankCache;
            table = gAudioContext.sampleFontLoadStatus.as_mut_ptr()
        }
        _ => { }
    }
    persistent = &mut (*loadedPool).persistent;
    persistentPool = &mut (*persistent).pool;
    if (*persistent).numEntries == 0 as libc::c_int as libc::c_uint { return }
    entryPtr =
        (*persistent).entries[(*persistent).numEntries.wrapping_sub(1 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint)
                                  as usize].ptr as *mut libc::c_void;
    (*persistentPool).cur = entryPtr as *mut u8_0;
    (*persistentPool).count -= 1;
    if tableType == SAMPLE_TABLE as libc::c_int {
        AudioHeap_DiscardSampleBank((*persistent).entries[(*persistent).numEntries.wrapping_sub(1
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_uint)
                                                              as usize].id as
                                        s32);
    }
    if tableType == FONT_TABLE as libc::c_int {
        AudioHeap_DiscardFont((*persistent).entries[(*persistent).numEntries.wrapping_sub(1
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_uint)
                                                        as usize].id as s32);
    }
    *table.offset((*persistent).entries[(*persistent).numEntries.wrapping_sub(1
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_uint)
                                            as usize].id as isize) =
        0 as libc::c_int as u8_0;
    (*persistent).numEntries = (*persistent).numEntries.wrapping_sub(1);
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_InitMainPools(mut initPoolSize: s32) {
    AudioHeap_AllocPoolInit(&mut gAudioContext.audioInitPool,
                            gAudioContext.audioHeap as *mut libc::c_void,
                            initPoolSize as u32_0);
    AudioHeap_AllocPoolInit(&mut gAudioContext.audioSessionPool,
                            gAudioContext.audioHeap.offset(initPoolSize as
                                                               isize) as
                                *mut libc::c_void,
                            gAudioContext.audioHeapSize.wrapping_sub(initPoolSize
                                                                         as
                                                                         libc::c_uint));
    gAudioContext.externalPool.start = 0 as *mut u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_SessionPoolsInit(mut split:
                                                        *mut AudioPoolSplit4) {
    gAudioContext.audioSessionPool.cur = gAudioContext.audioSessionPool.start;
    AudioHeap_AllocPoolInit(&mut gAudioContext.notesAndBuffersPool,
                            AudioHeap_Alloc(&mut gAudioContext.audioSessionPool,
                                            (*split).wantSeq),
                            (*split).wantSeq);
    AudioHeap_AllocPoolInit(&mut gAudioContext.cachePool,
                            AudioHeap_Alloc(&mut gAudioContext.audioSessionPool,
                                            (*split).wantCustom),
                            (*split).wantCustom);
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_CachePoolInit(mut split:
                                                     *mut AudioPoolSplit2) {
    gAudioContext.cachePool.cur = gAudioContext.cachePool.start;
    AudioHeap_AllocPoolInit(&mut gAudioContext.persistentCommonPool,
                            AudioHeap_Alloc(&mut gAudioContext.cachePool,
                                            (*split).wantPersistent),
                            (*split).wantPersistent);
    AudioHeap_AllocPoolInit(&mut gAudioContext.temporaryCommonPool,
                            AudioHeap_Alloc(&mut gAudioContext.cachePool,
                                            (*split).wantTemporary),
                            (*split).wantTemporary);
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_PersistentCachesInit(mut split:
                                                            *mut AudioPoolSplit3) {
    gAudioContext.persistentCommonPool.cur =
        gAudioContext.persistentCommonPool.start;
    AudioHeap_AllocPoolInit(&mut gAudioContext.seqCache.persistent.pool,
                            AudioHeap_Alloc(&mut gAudioContext.persistentCommonPool,
                                            (*split).wantSeq),
                            (*split).wantSeq);
    AudioHeap_AllocPoolInit(&mut gAudioContext.fontCache.persistent.pool,
                            AudioHeap_Alloc(&mut gAudioContext.persistentCommonPool,
                                            (*split).wantFont),
                            (*split).wantFont);
    AudioHeap_AllocPoolInit(&mut gAudioContext.sampleBankCache.persistent.pool,
                            AudioHeap_Alloc(&mut gAudioContext.persistentCommonPool,
                                            (*split).wantSample),
                            (*split).wantSample);
    AudioHeap_PersistentCacheClear(&mut gAudioContext.seqCache.persistent);
    AudioHeap_PersistentCacheClear(&mut gAudioContext.fontCache.persistent);
    AudioHeap_PersistentCacheClear(&mut gAudioContext.sampleBankCache.persistent);
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_TemporaryCachesInit(mut split:
                                                           *mut AudioPoolSplit3) {
    gAudioContext.temporaryCommonPool.cur =
        gAudioContext.temporaryCommonPool.start;
    AudioHeap_AllocPoolInit(&mut gAudioContext.seqCache.temporary.pool,
                            AudioHeap_Alloc(&mut gAudioContext.temporaryCommonPool,
                                            (*split).wantSeq),
                            (*split).wantSeq);
    AudioHeap_AllocPoolInit(&mut gAudioContext.fontCache.temporary.pool,
                            AudioHeap_Alloc(&mut gAudioContext.temporaryCommonPool,
                                            (*split).wantFont),
                            (*split).wantFont);
    AudioHeap_AllocPoolInit(&mut gAudioContext.sampleBankCache.temporary.pool,
                            AudioHeap_Alloc(&mut gAudioContext.temporaryCommonPool,
                                            (*split).wantSample),
                            (*split).wantSample);
    AudioHeap_TemporaryCacheClear(&mut gAudioContext.seqCache.temporary);
    AudioHeap_TemporaryCacheClear(&mut gAudioContext.fontCache.temporary);
    AudioHeap_TemporaryCacheClear(&mut gAudioContext.sampleBankCache.temporary);
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_AllocCached(mut tableType: s32,
                                               mut size: s32, mut cache: s32,
                                               mut id: s32)
 -> *mut libc::c_void {
    let mut current_block: u64;
    let mut loadedPool: *mut AudioCache = 0 as *mut AudioCache;
    let mut tp: *mut AudioTemporaryCache = 0 as *mut AudioTemporaryCache;
    let mut pool: *mut AudioAllocPool = 0 as *mut AudioAllocPool;
    let mut mem: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut firstVal: u8_0 = 0;
    let mut secondVal: u8_0 = 0;
    let mut i: s32 = 0;
    let mut table: *mut u8_0 = 0 as *mut u8_0;
    let mut side: s32 = 0;
    match tableType {
        0 => {
            loadedPool = &mut gAudioContext.seqCache;
            table = gAudioContext.seqLoadStatus.as_mut_ptr()
        }
        1 => {
            loadedPool = &mut gAudioContext.fontCache;
            table = gAudioContext.fontLoadStatus.as_mut_ptr()
        }
        2 => {
            loadedPool = &mut gAudioContext.sampleBankCache;
            table = gAudioContext.sampleFontLoadStatus.as_mut_ptr()
        }
        _ => { }
    }
    if cache == CACHE_TEMPORARY as libc::c_int {
        tp = &mut (*loadedPool).temporary;
        pool = &mut (*tp).pool;
        if (*pool).size < size { return 0 as *mut libc::c_void }
        firstVal =
            if (*tp).entries[0 as libc::c_int as usize].id as libc::c_int ==
                   -(1 as libc::c_int) {
                0 as libc::c_int
            } else {
                *table.offset((*tp).entries[0 as libc::c_int as usize].id as
                                  isize) as libc::c_int
            } as u8_0;
        secondVal =
            if (*tp).entries[1 as libc::c_int as usize].id as libc::c_int ==
                   -(1 as libc::c_int) {
                0 as libc::c_int
            } else {
                *table.offset((*tp).entries[1 as libc::c_int as usize].id as
                                  isize) as libc::c_int
            } as u8_0;
        if tableType == FONT_TABLE as libc::c_int {
            if firstVal as libc::c_int == 4 as libc::c_int {
                i = 0 as libc::c_int;
                while i < gAudioContext.numNotes {
                    if (*gAudioContext.notes.offset(i as
                                                        isize)).playbackState.fontId
                           as libc::c_int ==
                           (*tp).entries[0 as libc::c_int as usize].id as
                               libc::c_int &&
                           (*gAudioContext.notes.offset(i as
                                                            isize)).noteSubEu.bitField0.enabled()
                               as libc::c_int != 0 as libc::c_int {
                        break ;
                    }
                    i += 1
                }
                if i == gAudioContext.numNotes {
                    AudioLoad_SetFontLoadStatus((*tp).entries[0 as libc::c_int
                                                                  as usize].id
                                                    as s32, 3 as libc::c_int);
                    firstVal = 3 as libc::c_int as u8_0
                }
            }
            if secondVal as libc::c_int == 4 as libc::c_int {
                i = 0 as libc::c_int;
                while i < gAudioContext.numNotes {
                    if (*gAudioContext.notes.offset(i as
                                                        isize)).playbackState.fontId
                           as libc::c_int ==
                           (*tp).entries[1 as libc::c_int as usize].id as
                               libc::c_int &&
                           (*gAudioContext.notes.offset(i as
                                                            isize)).noteSubEu.bitField0.enabled()
                               as libc::c_int != 0 as libc::c_int {
                        break ;
                    }
                    i += 1
                }
                if i == gAudioContext.numNotes {
                    AudioLoad_SetFontLoadStatus((*tp).entries[1 as libc::c_int
                                                                  as usize].id
                                                    as s32, 3 as libc::c_int);
                    secondVal = 3 as libc::c_int as u8_0
                }
            }
        }
        if firstVal as libc::c_int == 0 as libc::c_int {
            (*tp).nextSide = 0 as libc::c_int as u32_0
        } else if secondVal as libc::c_int == 0 as libc::c_int {
            (*tp).nextSide = 1 as libc::c_int as u32_0
        } else if !(firstVal as libc::c_int == 3 as libc::c_int &&
                        secondVal as libc::c_int == 3 as libc::c_int) {
            if firstVal as libc::c_int == 3 as libc::c_int {
                (*tp).nextSide = 0 as libc::c_int as u32_0
            } else if secondVal as libc::c_int == 3 as libc::c_int {
                (*tp).nextSide = 1 as libc::c_int as u32_0
            } else {
                // Check if there is a side which isn't in active use, if so, evict that one.
                if tableType == SEQUENCE_TABLE as libc::c_int {
                    if firstVal as libc::c_int == 2 as libc::c_int {
                        i = 0 as libc::c_int;
                        while i <
                                  gAudioContext.audioBufferParameters.numSequencePlayers
                                      as libc::c_int {
                            if gAudioContext.seqPlayers[i as usize].enabled()
                                   as libc::c_int != 0 as libc::c_int &&
                                   gAudioContext.seqPlayers[i as usize].seqId
                                       as libc::c_int ==
                                       (*tp).entries[0 as libc::c_int as
                                                         usize].id as
                                           libc::c_int {
                                break ;
                            }
                            i += 1
                        }
                        if i ==
                               gAudioContext.audioBufferParameters.numSequencePlayers
                                   as libc::c_int {
                            (*tp).nextSide = 0 as libc::c_int as u32_0;
                            current_block = 18316757414259624557;
                        } else { current_block = 5181772461570869434; }
                    } else { current_block = 5181772461570869434; }
                    match current_block {
                        18316757414259624557 => { }
                        _ => {
                            if secondVal as libc::c_int == 2 as libc::c_int {
                                i = 0 as libc::c_int;
                                while i <
                                          gAudioContext.audioBufferParameters.numSequencePlayers
                                              as libc::c_int {
                                    if gAudioContext.seqPlayers[i as
                                                                    usize].enabled()
                                           as libc::c_int != 0 as libc::c_int
                                           &&
                                           gAudioContext.seqPlayers[i as
                                                                        usize].seqId
                                               as libc::c_int ==
                                               (*tp).entries[1 as libc::c_int
                                                                 as usize].id
                                                   as libc::c_int {
                                        break ;
                                    }
                                    i += 1
                                }
                                if i ==
                                       gAudioContext.audioBufferParameters.numSequencePlayers
                                           as libc::c_int {
                                    (*tp).nextSide =
                                        1 as libc::c_int as u32_0;
                                    current_block = 18316757414259624557;
                                } else {
                                    current_block = 3634396408142324656;
                                }
                            } else { current_block = 3634396408142324656; }
                        }
                    }
                } else if tableType == FONT_TABLE as libc::c_int {
                    if firstVal as libc::c_int == 2 as libc::c_int {
                        i = 0 as libc::c_int;
                        while i < gAudioContext.numNotes {
                            if (*gAudioContext.notes.offset(i as
                                                                isize)).playbackState.fontId
                                   as libc::c_int ==
                                   (*tp).entries[0 as libc::c_int as usize].id
                                       as libc::c_int &&
                                   (*gAudioContext.notes.offset(i as
                                                                    isize)).noteSubEu.bitField0.enabled()
                                       as libc::c_int != 0 as libc::c_int {
                                break ;
                            }
                            i += 1
                        }
                        if i == gAudioContext.numNotes {
                            (*tp).nextSide = 0 as libc::c_int as u32_0;
                            current_block = 18316757414259624557;
                        } else { current_block = 17711149709958600598; }
                    } else { current_block = 17711149709958600598; }
                    match current_block {
                        18316757414259624557 => { }
                        _ => {
                            if secondVal as libc::c_int == 2 as libc::c_int {
                                i = 0 as libc::c_int;
                                while i < gAudioContext.numNotes {
                                    if (*gAudioContext.notes.offset(i as
                                                                        isize)).playbackState.fontId
                                           as libc::c_int ==
                                           (*tp).entries[1 as libc::c_int as
                                                             usize].id as
                                               libc::c_int &&
                                           (*gAudioContext.notes.offset(i as
                                                                            isize)).noteSubEu.bitField0.enabled()
                                               as libc::c_int !=
                                               0 as libc::c_int {
                                        break ;
                                    }
                                    i += 1
                                }
                                if i == gAudioContext.numNotes {
                                    (*tp).nextSide =
                                        1 as libc::c_int as u32_0;
                                    current_block = 18316757414259624557;
                                } else {
                                    current_block = 3634396408142324656;
                                }
                            } else { current_block = 3634396408142324656; }
                        }
                    }
                } else { current_block = 3634396408142324656; }
                match current_block {
                    18316757414259624557 => { }
                    _ =>
                    // No such luck. Evict the side that wasn't chosen last time, except
            // if it is being loaded into.
                    {
                        if (*tp).nextSide == 0 as libc::c_int as libc::c_uint
                           {
                            if firstVal as libc::c_int == 1 as libc::c_int {
                                if secondVal as libc::c_int ==
                                       1 as libc::c_int {
                                    current_block = 3268579060917229304;
                                } else {
                                    (*tp).nextSide =
                                        1 as libc::c_int as u32_0;
                                    current_block = 18316757414259624557;
                                }
                            } else { current_block = 18316757414259624557; }
                        } else if secondVal as libc::c_int == 1 as libc::c_int
                         {
                            if firstVal as libc::c_int == 1 as libc::c_int {
                                current_block = 3268579060917229304;
                            } else {
                                (*tp).nextSide = 0 as libc::c_int as u32_0;
                                current_block = 18316757414259624557;
                            }
                        } else { current_block = 18316757414259624557; }
                        match current_block {
                            18316757414259624557 => { }
                            _ => {
                                // Both sides are being loaded into.
                                return 0 as *mut libc::c_void
                            }
                        }
                    }
                }
            }
        }
        // Use the opposite side from last time.
        side = (*tp).nextSide as s32;
        if (*tp).entries[side as usize].id as libc::c_int !=
               -(1 as libc::c_int) {
            if tableType == SAMPLE_TABLE as libc::c_int {
                AudioHeap_DiscardSampleBank((*tp).entries[side as usize].id as
                                                s32);
            }
            *table.offset((*tp).entries[side as usize].id as isize) =
                0 as libc::c_int as u8_0;
            if tableType == FONT_TABLE as libc::c_int {
                AudioHeap_DiscardFont((*tp).entries[side as usize].id as s32);
            }
        }
        match side {
            0 => {
                (*tp).entries[0 as libc::c_int as usize].ptr = (*pool).start;
                (*tp).entries[0 as libc::c_int as usize].id = id as s16;
                (*tp).entries[0 as libc::c_int as usize].size = size as u32_0;
                (*pool).cur = (*pool).start.offset(size as isize);
                if (*tp).entries[1 as libc::c_int as usize].id as libc::c_int
                       != -(1 as libc::c_int) &&
                       (*tp).entries[1 as libc::c_int as usize].ptr <
                           (*pool).cur {
                    if tableType == SAMPLE_TABLE as libc::c_int {
                        AudioHeap_DiscardSampleBank((*tp).entries[1 as
                                                                      libc::c_int
                                                                      as
                                                                      usize].id
                                                        as s32);
                    }
                    *table.offset((*tp).entries[1 as libc::c_int as usize].id
                                      as isize) = 0 as libc::c_int as u8_0;
                    match tableType {
                        0 => {
                            AudioHeap_DiscardSequence((*tp).entries[1 as
                                                                        libc::c_int
                                                                        as
                                                                        usize].id
                                                          as s32);
                        }
                        1 => {
                            AudioHeap_DiscardFont((*tp).entries[1 as
                                                                    libc::c_int
                                                                    as
                                                                    usize].id
                                                      as s32);
                        }
                        _ => { }
                    }
                    (*tp).entries[1 as libc::c_int as usize].id =
                        -(1 as libc::c_int) as s16;
                    (*tp).entries[1 as libc::c_int as usize].ptr =
                        (*pool).start.offset((*pool).size as isize)
                }
                ret =
                    (*tp).entries[0 as libc::c_int as usize].ptr as
                        *mut libc::c_void
            }
            1 => {
                (*tp).entries[1 as libc::c_int as usize].ptr =
                    ((*pool).start.offset((*pool).size as
                                              isize).offset(-(size as isize))
                         as u32_0 & !(0xf as libc::c_int) as libc::c_uint) as
                        *mut u8_0;
                (*tp).entries[1 as libc::c_int as usize].id = id as s16;
                (*tp).entries[1 as libc::c_int as usize].size = size as u32_0;
                if (*tp).entries[0 as libc::c_int as usize].id as libc::c_int
                       != -(1 as libc::c_int) &&
                       (*tp).entries[1 as libc::c_int as usize].ptr <
                           (*pool).cur {
                    if tableType == SAMPLE_TABLE as libc::c_int {
                        AudioHeap_DiscardSampleBank((*tp).entries[0 as
                                                                      libc::c_int
                                                                      as
                                                                      usize].id
                                                        as s32);
                    }
                    *table.offset((*tp).entries[0 as libc::c_int as usize].id
                                      as isize) = 0 as libc::c_int as u8_0;
                    match tableType {
                        0 => {
                            AudioHeap_DiscardSequence((*tp).entries[0 as
                                                                        libc::c_int
                                                                        as
                                                                        usize].id
                                                          as s32);
                        }
                        1 => {
                            AudioHeap_DiscardFont((*tp).entries[0 as
                                                                    libc::c_int
                                                                    as
                                                                    usize].id
                                                      as s32);
                        }
                        _ => { }
                    }
                    (*tp).entries[0 as libc::c_int as usize].id =
                        -(1 as libc::c_int) as s16;
                    (*pool).cur = (*pool).start
                }
                ret =
                    (*tp).entries[1 as libc::c_int as usize].ptr as
                        *mut libc::c_void
            }
            _ => { return 0 as *mut libc::c_void }
        }
        (*tp).nextSide ^= 1 as libc::c_int as libc::c_uint;
        return ret
    } else {
        mem =
            AudioHeap_Alloc(&mut (*loadedPool).persistent.pool,
                            size as u32_0);
        (*loadedPool).persistent.entries[(*loadedPool).persistent.numEntries
                                             as usize].ptr = mem as *mut u8_0;
        if mem.is_null() {
            match cache {
                2 => {
                    return AudioHeap_AllocCached(tableType, size,
                                                 CACHE_TEMPORARY as
                                                     libc::c_int, id)
                }
                0 | 1 => { return 0 as *mut libc::c_void }
                _ => { }
            }
        }
        (*loadedPool).persistent.entries[(*loadedPool).persistent.numEntries
                                             as usize].id = id as s16;
        (*loadedPool).persistent.entries[(*loadedPool).persistent.numEntries
                                             as usize].size = size as u32_0;
        let fresh0 = (*loadedPool).persistent.numEntries;
        (*loadedPool).persistent.numEntries =
            (*loadedPool).persistent.numEntries.wrapping_add(1);
        return (*loadedPool).persistent.entries[fresh0 as usize].ptr as
                   *mut libc::c_void
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_SearchCaches(mut tableType: s32,
                                                mut cache: s32, mut id: s32)
 -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    // Always search the permanent cache in addition to the regular ones.
    ret = AudioHeap_SearchPermanentCache(tableType, id);
    if !ret.is_null() { return ret }
    if cache == CACHE_PERMANENT as libc::c_int {
        return 0 as *mut libc::c_void
    }
    return AudioHeap_SearchRegularCaches(tableType, cache, id);
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_SearchRegularCaches(mut tableType: s32,
                                                       mut cache: s32,
                                                       mut id: s32)
 -> *mut libc::c_void {
    let mut i: u32_0 = 0;
    let mut loadedPool: *mut AudioCache = 0 as *mut AudioCache;
    let mut temporary: *mut AudioTemporaryCache =
        0 as *mut AudioTemporaryCache;
    let mut persistent: *mut AudioPersistentCache =
        0 as *mut AudioPersistentCache;
    match tableType {
        0 => { loadedPool = &mut gAudioContext.seqCache }
        1 => { loadedPool = &mut gAudioContext.fontCache }
        2 => { loadedPool = &mut gAudioContext.sampleBankCache }
        _ => { }
    }
    temporary = &mut (*loadedPool).temporary;
    if cache == CACHE_TEMPORARY as libc::c_int {
        if (*temporary).entries[0 as libc::c_int as usize].id as libc::c_int
               == id {
            (*temporary).nextSide = 1 as libc::c_int as u32_0;
            return (*temporary).entries[0 as libc::c_int as usize].ptr as
                       *mut libc::c_void
        } else if (*temporary).entries[1 as libc::c_int as usize].id as
                      libc::c_int == id {
            (*temporary).nextSide = 0 as libc::c_int as u32_0;
            return (*temporary).entries[1 as libc::c_int as usize].ptr as
                       *mut libc::c_void
        } else { return 0 as *mut libc::c_void }
    }
    persistent = &mut (*loadedPool).persistent;
    i = 0 as libc::c_int as u32_0;
    while i < (*persistent).numEntries {
        if (*persistent).entries[i as usize].id as libc::c_int == id {
            return (*persistent).entries[i as usize].ptr as *mut libc::c_void
        }
        i = i.wrapping_add(1)
    }
    if cache == CACHE_EITHER as libc::c_int {
        return AudioHeap_SearchCaches(tableType,
                                      CACHE_TEMPORARY as libc::c_int, id)
    }
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn func_800DF1D8(mut p: f32_0, mut q: f32_0,
                                       mut out: *mut u16_0) {
    // With the bug below fixed, this mysterious unused function computes two recurrences
    // out[0..7] = a_i, out[8..15] = b_i, where
    // a_{-2} = b_{-1} = 262159 = 2^18 + 15
    // a_{-1} = b_{-2} = 0
    // a_i = q * a_{i-1} + p * a_{i-2}
    // b_i = q * b_{i-1} + p * b_{i-2}
    // These grow exponentially if p < -1 or p + |q| > 1.
    let mut i: s32 = 0;
    let mut tmp: [f32_0; 16] = [0.; 16];
    tmp[0 as libc::c_int as usize] = q * 262159.0f32;
    tmp[8 as libc::c_int as usize] = p * 262159.0f32;
    tmp[1 as libc::c_int as usize] = q * p * 262159.0f32;
    tmp[9 as libc::c_int as usize] = (p * p + q) * 262159.0f32;
    i = 2 as libc::c_int;
    while i < 8 as libc::c_int {
        // ! @bug value should be stored to tmp[i] and tmp[8 + i], otherwise we read
        // ! garbage in later loop iterations.
        *out.offset(i as isize) =
            (q * tmp[(i - 2 as libc::c_int) as usize] +
                 p * tmp[(i - 1 as libc::c_int) as usize]) as u16_0;
        *out.offset((8 as libc::c_int + i) as isize) =
            (q * tmp[(6 as libc::c_int + i) as usize] +
                 p * tmp[(7 as libc::c_int + i) as usize]) as u16_0;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        *out.offset(i as isize) = tmp[i as usize] as u16_0;
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_ClearFilter(mut filter: *mut s16) {
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        *filter.offset(i as isize) = 0 as libc::c_int as s16;
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_LoadLowPassFilter(mut filter: *mut s16,
                                                     mut cutoff: s32) {
    let mut i: s32 = 0;
    let mut ptr: *mut s16 =
        &mut *sLowPassFilterData.as_mut_ptr().offset((8 as libc::c_int *
                                                          cutoff) as isize) as
            *mut s16;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        *filter.offset(i as isize) = *ptr.offset(i as isize);
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_LoadHighPassFilter(mut filter: *mut s16,
                                                      mut cutoff: s32) {
    let mut i: s32 = 0;
    let mut ptr: *mut s16 =
        &mut *sHighPassFilterData.as_mut_ptr().offset((8 as libc::c_int *
                                                           (cutoff -
                                                                1 as
                                                                    libc::c_int))
                                                          as isize) as
            *mut s16;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        *filter.offset(i as isize) = *ptr.offset(i as isize);
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_LoadFilter(mut filter: *mut s16,
                                              mut lowPassCutoff: s32,
                                              mut highPassCutoff: s32) {
    let mut i: s32 = 0;
    if lowPassCutoff == 0 as libc::c_int && highPassCutoff == 0 as libc::c_int
       {
        // Identity filter
        AudioHeap_LoadLowPassFilter(filter, 0 as libc::c_int);
    } else if highPassCutoff == 0 as libc::c_int {
        AudioHeap_LoadLowPassFilter(filter, lowPassCutoff);
    } else if lowPassCutoff == 0 as libc::c_int {
        AudioHeap_LoadHighPassFilter(filter, highPassCutoff);
    } else {
        let mut ptr1: *mut s16 =
            &mut *sLowPassFilterData.as_mut_ptr().offset((8 as libc::c_int *
                                                              lowPassCutoff)
                                                             as isize) as
                *mut s16;
        let mut ptr2: *mut s16 =
            &mut *sHighPassFilterData.as_mut_ptr().offset((8 as libc::c_int *
                                                               (highPassCutoff
                                                                    -
                                                                    1 as
                                                                        libc::c_int))
                                                              as isize) as
                *mut s16;
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            *filter.offset(i as isize) =
                ((*ptr1.offset(i as isize) as libc::c_int +
                      *ptr2.offset(i as isize) as libc::c_int) /
                     2 as libc::c_int) as s16;
            i += 1
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_UpdateReverb(mut reverb:
                                                    *mut SynthesisReverb) {
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_UpdateReverbs() {
    let mut count: s32 = 0;
    let mut i: s32 = 0;
    let mut j: s32 = 0;
    if gAudioContext.audioBufferParameters.specUnk4 as libc::c_int ==
           2 as libc::c_int {
        count = 2 as libc::c_int
    } else { count = 1 as libc::c_int }
    i = 0 as libc::c_int;
    while i < gAudioContext.numSynthesisReverbs as libc::c_int {
        j = 0 as libc::c_int;
        while j < count {
            AudioHeap_UpdateReverb(&mut *gAudioContext.synthesisReverbs.as_mut_ptr().offset(i
                                                                                                as
                                                                                                isize));
            j += 1
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_ClearAiBuffers() {
    let mut ind: s32 = 0;
    let mut i: s32 = 0;
    ind = gAudioContext.curAIBufIdx;
    gAudioContext.aiBufLengths[ind as usize] =
        gAudioContext.audioBufferParameters.minAiBufferLength;
    i = 0 as libc::c_int;
    while i < 0x580 as libc::c_int {
        *gAudioContext.aiBuffers[ind as usize].offset(i as isize) =
            0 as libc::c_int as s16;
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_ResetStep() -> s32 {
    let mut i: s32 = 0;
    let mut j: s32 = 0;
    let mut sp24: s32 = 0;
    if gAudioContext.audioBufferParameters.specUnk4 as libc::c_int ==
           2 as libc::c_int {
        sp24 = 2 as libc::c_int
    } else { sp24 = 1 as libc::c_int }
    match gAudioContext.resetStatus as libc::c_int {
        5 => {
            i = 0 as libc::c_int;
            while i <
                      gAudioContext.audioBufferParameters.numSequencePlayers
                          as libc::c_int {
                AudioSeq_SequencePlayerDisableAsFinished(&mut *gAudioContext.seqPlayers.as_mut_ptr().offset(i
                                                                                                                as
                                                                                                                isize));
                i += 1
            }
            gAudioContext.audioResetFadeOutFramesLeft =
                2 as libc::c_int / sp24;
            ::std::ptr::write_volatile(&mut gAudioContext.resetStatus as
                                           *mut u8_0,
                                       ::std::ptr::read_volatile::<u8_0>(&gAudioContext.resetStatus
                                                                             as
                                                                             *const u8_0).wrapping_sub(1))
        }
        4 => {
            if gAudioContext.audioResetFadeOutFramesLeft != 0 as libc::c_int {
                gAudioContext.audioResetFadeOutFramesLeft -= 1;
                AudioHeap_UpdateReverbs();
            } else {
                i = 0 as libc::c_int;
                while i < gAudioContext.numNotes {
                    if (*gAudioContext.notes.offset(i as
                                                        isize)).noteSubEu.bitField0.enabled()
                           as libc::c_int != 0 &&
                           (*gAudioContext.notes.offset(i as
                                                            isize)).playbackState.adsr.action.s.state()
                               as libc::c_int !=
                               ADSR_STATE_DISABLED as libc::c_int {
                        (*gAudioContext.notes.offset(i as
                                                         isize)).playbackState.adsr.fadeOutVel
                            =
                            gAudioContext.audioBufferParameters.updatesPerFrameInv;
                        let ref mut fresh1 =
                            (*gAudioContext.notes.offset(i as
                                                             isize)).playbackState.adsr.action.s;
                        (*fresh1).set_release(1 as libc::c_int as u8_0)
                    }
                    i += 1
                }
                gAudioContext.audioResetFadeOutFramesLeft =
                    8 as libc::c_int / sp24;
                ::std::ptr::write_volatile(&mut gAudioContext.resetStatus as
                                               *mut u8_0,
                                           ::std::ptr::read_volatile::<u8_0>(&gAudioContext.resetStatus
                                                                                 as
                                                                                 *const u8_0).wrapping_sub(1))
            }
        }
        3 => {
            if gAudioContext.audioResetFadeOutFramesLeft != 0 as libc::c_int {
                gAudioContext.audioResetFadeOutFramesLeft -= 1;
                AudioHeap_UpdateReverbs();
            } else {
                gAudioContext.audioResetFadeOutFramesLeft =
                    2 as libc::c_int / sp24;
                ::std::ptr::write_volatile(&mut gAudioContext.resetStatus as
                                               *mut u8_0,
                                           ::std::ptr::read_volatile::<u8_0>(&gAudioContext.resetStatus
                                                                                 as
                                                                                 *const u8_0).wrapping_sub(1))
            }
        }
        2 => {
            AudioHeap_ClearAiBuffers();
            if gAudioContext.audioResetFadeOutFramesLeft != 0 as libc::c_int {
                gAudioContext.audioResetFadeOutFramesLeft -= 1
            } else {
                ::std::ptr::write_volatile(&mut gAudioContext.resetStatus as
                                               *mut u8_0,
                                           ::std::ptr::read_volatile::<u8_0>(&gAudioContext.resetStatus
                                                                                 as
                                                                                 *const u8_0).wrapping_sub(1));
                AudioHeap_DiscardSampleCaches();
                AudioHeap_DiscardSampleBanks();
            }
        }
        1 => {
            AudioHeap_Init();
            ::std::ptr::write_volatile(&mut gAudioContext.resetStatus as
                                           *mut u8_0,
                                       0 as libc::c_int as u8_0);
            i = 0 as libc::c_int;
            while i < 3 as libc::c_int {
                gAudioContext.aiBufLengths[i as usize] =
                    gAudioContext.audioBufferParameters.maxAiBufferLength;
                j = 0 as libc::c_int;
                while j < 0x580 as libc::c_int {
                    *gAudioContext.aiBuffers[i as usize].offset(j as isize) =
                        0 as libc::c_int as s16;
                    j += 1
                }
                i += 1
            }
        }
        _ => { }
    }
    if (gAudioContext.resetStatus as libc::c_int) < 3 as libc::c_int {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_Init() {
    let mut pad1: [s32; 4] = [0; 4];
    let mut mem: *mut s16 = 0 as *mut s16;
    let mut persistentMem: s32 = 0;
    let mut temporaryMem: s32 = 0;
    let mut totalMem: s32 = 0;
    let mut wantMisc: s32 = 0;
    let mut intMask: OSIntMask = 0;
    let mut i: s32 = 0;
    let mut j: s32 = 0;
    let mut pad2: s32 = 0;
    let mut spec: *mut AudioSpec = 0 as *mut AudioSpec;
    spec =
        &mut *gAudioSpecs.as_mut_ptr().offset(gAudioContext.audioResetSpecIdToLoad
                                                  as isize) as *mut AudioSpec;
    gAudioContext.sampleDmaCount = 0 as libc::c_int as u32_0;
    gAudioContext.audioBufferParameters.frequency =
        (*spec).frequency as u16_0;
    gAudioContext.audioBufferParameters.aiFrequency =
        osAiSetFrequency(gAudioContext.audioBufferParameters.frequency as
                             u32_0) as u16_0;
    gAudioContext.audioBufferParameters.samplesPerFrameTarget =
        (gAudioContext.audioBufferParameters.frequency as libc::c_int /
             gAudioContext.refreshRate + 0xf as libc::c_int &
             0xfff0 as libc::c_int) as s16;
    gAudioContext.audioBufferParameters.minAiBufferLength =
        (gAudioContext.audioBufferParameters.samplesPerFrameTarget as
             libc::c_int - 0x10 as libc::c_int) as s16;
    gAudioContext.audioBufferParameters.maxAiBufferLength =
        (gAudioContext.audioBufferParameters.samplesPerFrameTarget as
             libc::c_int + 0x10 as libc::c_int) as s16;
    gAudioContext.audioBufferParameters.updatesPerFrame =
        ((gAudioContext.audioBufferParameters.samplesPerFrameTarget as
              libc::c_int + 0x10 as libc::c_int) / 0xd0 as libc::c_int +
             1 as libc::c_int) as s16;
    gAudioContext.audioBufferParameters.samplesPerUpdate =
        (gAudioContext.audioBufferParameters.samplesPerFrameTarget as
             libc::c_int /
             gAudioContext.audioBufferParameters.updatesPerFrame as
                 libc::c_int & !(7 as libc::c_int)) as s16;
    gAudioContext.audioBufferParameters.samplesPerUpdateMax =
        (gAudioContext.audioBufferParameters.samplesPerUpdate as libc::c_int +
             8 as libc::c_int) as s16;
    gAudioContext.audioBufferParameters.samplesPerUpdateMin =
        (gAudioContext.audioBufferParameters.samplesPerUpdate as libc::c_int -
             8 as libc::c_int) as s16;
    gAudioContext.audioBufferParameters.resampleRate =
        32000.0f32 /
            gAudioContext.audioBufferParameters.frequency as s32 as
                libc::c_float;
    gAudioContext.audioBufferParameters.unkUpdatesPerFrameScaled =
        1.0f32 / 256.0f32 /
            gAudioContext.audioBufferParameters.updatesPerFrame as libc::c_int
                as libc::c_float;
    gAudioContext.audioBufferParameters.unk_24 =
        gAudioContext.audioBufferParameters.updatesPerFrame as libc::c_int as
            libc::c_float * 0.25f32;
    gAudioContext.audioBufferParameters.updatesPerFrameInv =
        1.0f32 /
            gAudioContext.audioBufferParameters.updatesPerFrame as libc::c_int
                as libc::c_float;
    gAudioContext.sampleDmaBufSize1 = (*spec).sampleDmaBufSize1 as s32;
    gAudioContext.sampleDmaBufSize2 = (*spec).sampleDmaBufSize2 as s32;
    gAudioContext.numNotes = (*spec).numNotes as s32;
    gAudioContext.audioBufferParameters.numSequencePlayers =
        (*spec).numSequencePlayers as s16;
    if gAudioContext.audioBufferParameters.numSequencePlayers as libc::c_int >
           4 as libc::c_int {
        gAudioContext.audioBufferParameters.numSequencePlayers =
            4 as libc::c_int as s16
    }
    gAudioContext.unk_2 = (*spec).unk_14;
    gAudioContext.tempoInternalToExternal =
        (gAudioContext.audioBufferParameters.updatesPerFrame as libc::c_int as
             libc::c_float * 2880000.0f32 /
             *D_8014A6C0.as_ptr().offset(1 as libc::c_int as isize) as
                 libc::c_int as libc::c_float / gAudioContext.unk_2960) as
            u32_0 as s16;
    gAudioContext.unk_2870 = gAudioContext.refreshRate as f32_0;
    gAudioContext.unk_2870 *=
        gAudioContext.audioBufferParameters.updatesPerFrame as libc::c_int as
            libc::c_float;
    gAudioContext.unk_2870 /=
        gAudioContext.audioBufferParameters.aiFrequency as libc::c_int as
            libc::c_float;
    gAudioContext.unk_2870 /=
        gAudioContext.tempoInternalToExternal as libc::c_int as libc::c_float;
    gAudioContext.audioBufferParameters.specUnk4 = (*spec).unk_04 as s16;
    gAudioContext.audioBufferParameters.samplesPerFrameTarget =
        (gAudioContext.audioBufferParameters.samplesPerFrameTarget as
             libc::c_int *
             gAudioContext.audioBufferParameters.specUnk4 as libc::c_int) as
            s16;
    gAudioContext.audioBufferParameters.maxAiBufferLength =
        (gAudioContext.audioBufferParameters.maxAiBufferLength as libc::c_int
             * gAudioContext.audioBufferParameters.specUnk4 as libc::c_int) as
            s16;
    gAudioContext.audioBufferParameters.minAiBufferLength =
        (gAudioContext.audioBufferParameters.minAiBufferLength as libc::c_int
             * gAudioContext.audioBufferParameters.specUnk4 as libc::c_int) as
            s16;
    gAudioContext.audioBufferParameters.updatesPerFrame =
        (gAudioContext.audioBufferParameters.updatesPerFrame as libc::c_int *
             gAudioContext.audioBufferParameters.specUnk4 as libc::c_int) as
            s16;
    if gAudioContext.audioBufferParameters.specUnk4 as libc::c_int >=
           2 as libc::c_int {
        gAudioContext.audioBufferParameters.maxAiBufferLength =
            (gAudioContext.audioBufferParameters.maxAiBufferLength as
                 libc::c_int - 0x10 as libc::c_int) as s16
    }
    gAudioContext.maxAudioCmds =
        gAudioContext.numNotes * 0x10 as libc::c_int *
            gAudioContext.audioBufferParameters.updatesPerFrame as libc::c_int
            + (*spec).numReverbs as libc::c_int * 0x18 as libc::c_int +
            0x140 as libc::c_int;
    persistentMem =
        (*spec).persistentSeqMem.wrapping_add((*spec).persistentFontMem).wrapping_add((*spec).persistentSampleMem).wrapping_add(0x10
                                                                                                                                    as
                                                                                                                                    libc::c_int
                                                                                                                                    as
                                                                                                                                    libc::c_uint)
            as s32;
    temporaryMem =
        (*spec).temporarySeqMem.wrapping_add((*spec).temporaryFontMem).wrapping_add((*spec).temporarySampleMem).wrapping_add(0x10
                                                                                                                                 as
                                                                                                                                 libc::c_int
                                                                                                                                 as
                                                                                                                                 libc::c_uint)
            as s32;
    totalMem = persistentMem + temporaryMem;
    wantMisc =
        gAudioContext.audioSessionPool.size - totalMem - 0x100 as libc::c_int;
    if !gAudioContext.externalPool.start.is_null() {
        gAudioContext.externalPool.cur = gAudioContext.externalPool.start
    }
    gAudioContext.sessionPoolSplit.wantSeq = wantMisc as u32_0;
    gAudioContext.sessionPoolSplit.wantCustom = totalMem as u32_0;
    AudioHeap_SessionPoolsInit(&mut gAudioContext.sessionPoolSplit);
    gAudioContext.cachePoolSplit.wantPersistent = persistentMem as u32_0;
    gAudioContext.cachePoolSplit.wantTemporary = temporaryMem as u32_0;
    AudioHeap_CachePoolInit(&mut gAudioContext.cachePoolSplit);
    gAudioContext.persistentCommonPoolSplit.wantSeq =
        (*spec).persistentSeqMem;
    gAudioContext.persistentCommonPoolSplit.wantFont =
        (*spec).persistentFontMem;
    gAudioContext.persistentCommonPoolSplit.wantSample =
        (*spec).persistentSampleMem;
    AudioHeap_PersistentCachesInit(&mut gAudioContext.persistentCommonPoolSplit);
    gAudioContext.temporaryCommonPoolSplit.wantSeq = (*spec).temporarySeqMem;
    gAudioContext.temporaryCommonPoolSplit.wantFont =
        (*spec).temporaryFontMem;
    gAudioContext.temporaryCommonPoolSplit.wantSample =
        (*spec).temporarySampleMem;
    AudioHeap_TemporaryCachesInit(&mut gAudioContext.temporaryCommonPoolSplit);
    AudioHeap_ResetLoadStatus();
    gAudioContext.notes =
        AudioHeap_AllocZeroed(&mut gAudioContext.notesAndBuffersPool,
                              (gAudioContext.numNotes as
                                   libc::c_uint).wrapping_mul(::std::mem::size_of::<Note>()
                                                                  as
                                                                  libc::c_ulong))
            as *mut Note;
    Audio_NoteInitAll();
    Audio_InitNoteFreeList();
    gAudioContext.noteSubsEu =
        AudioHeap_AllocZeroed(&mut gAudioContext.notesAndBuffersPool,
                              ((gAudioContext.audioBufferParameters.updatesPerFrame
                                    as libc::c_int * gAudioContext.numNotes)
                                   as
                                   libc::c_uint).wrapping_mul(::std::mem::size_of::<NoteSubEu>()
                                                                  as
                                                                  libc::c_ulong))
            as *mut NoteSubEu;
    i = 0 as libc::c_int;
    while i != 2 as libc::c_int {
        gAudioContext.abiCmdBufs[i as usize] =
            AudioHeap_AllocDmaMemoryZeroed(&mut gAudioContext.notesAndBuffersPool,
                                           (gAudioContext.maxAudioCmds as
                                                libc::c_uint).wrapping_mul(::std::mem::size_of::<u64_0>()
                                                                               as
                                                                               libc::c_ulong))
                as *mut Acmd;
        i += 1
    }
    gAudioContext.unk_3520 =
        AudioHeap_Alloc(&mut gAudioContext.notesAndBuffersPool,
                        (0x100 as libc::c_int as
                             libc::c_uint).wrapping_mul(::std::mem::size_of::<f32_0>()
                                                            as libc::c_ulong))
            as *mut f32_0;
    func_800DDE3C();
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        gAudioContext.synthesisReverbs[i as usize].useReverb =
            0 as libc::c_int as u8_0;
        i += 1
    }
    gAudioContext.numSynthesisReverbs = (*spec).numReverbs as s8;
    i = 0 as libc::c_int;
    while i < gAudioContext.numSynthesisReverbs as libc::c_int {
        let mut settings: *mut ReverbSettings =
            &mut *(*spec).reverbSettings.offset(i as isize) as
                *mut ReverbSettings;
        let mut reverb: *mut SynthesisReverb =
            &mut *gAudioContext.synthesisReverbs.as_mut_ptr().offset(i as
                                                                         isize)
                as *mut SynthesisReverb;
        (*reverb).downsampleRate = (*settings).downsampleRate;
        (*reverb).windowSize =
            ((*settings).windowSize as libc::c_int * 64 as libc::c_int) as
                u16_0;
        (*reverb).windowSize =
            ((*reverb).windowSize as libc::c_int /
                 (*reverb).downsampleRate as libc::c_int) as u16_0;
        (*reverb).unk_0C = (*settings).unk_4;
        (*reverb).unk_0A = (*settings).unk_A as s16;
        (*reverb).unk_14 =
            ((*settings).unk_6 as libc::c_int * 64 as libc::c_int) as u16_0;
        (*reverb).unk_16 = (*settings).unk_8 as s16;
        (*reverb).unk_18 = 0 as libc::c_int as u8_0;
        (*reverb).leakRtl = (*settings).leakRtl as s16;
        (*reverb).leakLtr = (*settings).leakLtr as s16;
        (*reverb).unk_05 = (*settings).unk_10;
        (*reverb).unk_08 = (*settings).unk_12 as s16;
        (*reverb).useReverb = 8 as libc::c_int as u8_0;
        (*reverb).leftRingBuf =
            AudioHeap_AllocZeroedAttemptExternal(&mut gAudioContext.notesAndBuffersPool,
                                                 ((*reverb).windowSize as
                                                      libc::c_uint).wrapping_mul(::std::mem::size_of::<s16>()
                                                                                     as
                                                                                     libc::c_ulong))
                as *mut s16;
        (*reverb).rightRingBuf =
            AudioHeap_AllocZeroedAttemptExternal(&mut gAudioContext.notesAndBuffersPool,
                                                 ((*reverb).windowSize as
                                                      libc::c_uint).wrapping_mul(::std::mem::size_of::<s16>()
                                                                                     as
                                                                                     libc::c_ulong))
                as *mut s16;
        (*reverb).nextRingBufPos = 0 as libc::c_int;
        (*reverb).unk_20 = 0 as libc::c_int;
        (*reverb).curFrame = 0 as libc::c_int as u8_0;
        (*reverb).bufSizePerChan = (*reverb).windowSize as s32;
        (*reverb).framesToIgnore = 2 as libc::c_int as u8_0;
        (*reverb).resampleFlags = 1 as libc::c_int as u8_0;
        (*reverb).sound.sample = &mut (*reverb).sample;
        (*reverb).sample.loop_0 = &mut (*reverb).loop_0;
        (*reverb).sound.tuning = 1.0f32;
        (*reverb).sample.set_codec(CODEC_REVERB as libc::c_int as u32_0);
        (*reverb).sample.set_medium(MEDIUM_RAM as libc::c_int as u32_0);
        (*reverb).sample.set_size(((*reverb).windowSize as libc::c_int *
                                       2 as libc::c_int) as u32_0);
        (*reverb).sample.sampleAddr = (*reverb).leftRingBuf as *mut u8_0;
        (*reverb).loop_0.start = 0 as libc::c_int as u32_0;
        (*reverb).loop_0.count = 1 as libc::c_int as u32_0;
        (*reverb).loop_0.end = (*reverb).windowSize as u32_0;
        if (*reverb).downsampleRate as libc::c_int != 1 as libc::c_int {
            (*reverb).unk_0E =
                (0x8000 as libc::c_int /
                     (*reverb).downsampleRate as libc::c_int) as u16_0;
            (*reverb).unk_30 =
                AudioHeap_AllocZeroed(&mut gAudioContext.notesAndBuffersPool,
                                      0x20 as libc::c_int as u32_0);
            (*reverb).unk_34 =
                AudioHeap_AllocZeroed(&mut gAudioContext.notesAndBuffersPool,
                                      0x20 as libc::c_int as u32_0);
            (*reverb).unk_38 =
                AudioHeap_AllocZeroed(&mut gAudioContext.notesAndBuffersPool,
                                      0x20 as libc::c_int as u32_0);
            (*reverb).unk_3C =
                AudioHeap_AllocZeroed(&mut gAudioContext.notesAndBuffersPool,
                                      0x20 as libc::c_int as u32_0);
            j = 0 as libc::c_int;
            while j <
                      gAudioContext.audioBufferParameters.updatesPerFrame as
                          libc::c_int {
                mem =
                    AudioHeap_AllocZeroedAttemptExternal(&mut gAudioContext.notesAndBuffersPool,
                                                         0x340 as libc::c_int
                                                             as u32_0) as
                        *mut s16;
                (*reverb).items[0 as libc::c_int as
                                    usize][j as usize].toDownsampleLeft = mem;
                (*reverb).items[0 as libc::c_int as
                                    usize][j as usize].toDownsampleRight =
                    mem.offset((0x1a0 as libc::c_int as
                                    libc::c_uint).wrapping_div(::std::mem::size_of::<s16>()
                                                                   as
                                                                   libc::c_ulong)
                                   as isize);
                mem =
                    AudioHeap_AllocZeroedAttemptExternal(&mut gAudioContext.notesAndBuffersPool,
                                                         0x340 as libc::c_int
                                                             as u32_0) as
                        *mut s16;
                (*reverb).items[1 as libc::c_int as
                                    usize][j as usize].toDownsampleLeft = mem;
                (*reverb).items[1 as libc::c_int as
                                    usize][j as usize].toDownsampleRight =
                    mem.offset((0x1a0 as libc::c_int as
                                    libc::c_uint).wrapping_div(::std::mem::size_of::<s16>()
                                                                   as
                                                                   libc::c_ulong)
                                   as isize);
                j += 1
            }
        }
        if (*settings).lowPassFilterCutoffLeft as libc::c_int !=
               0 as libc::c_int {
            (*reverb).filterLeftState =
                AudioHeap_AllocDmaMemoryZeroed(&mut gAudioContext.notesAndBuffersPool,
                                               0x40 as libc::c_int as u32_0)
                    as *mut s16;
            (*reverb).filterLeft =
                AudioHeap_AllocDmaMemory(&mut gAudioContext.notesAndBuffersPool,
                                         (8 as libc::c_int as
                                              libc::c_uint).wrapping_mul(::std::mem::size_of::<s16>()
                                                                             as
                                                                             libc::c_ulong))
                    as *mut s16;
            AudioHeap_LoadLowPassFilter((*reverb).filterLeft,
                                        (*settings).lowPassFilterCutoffLeft as
                                            s32);
        } else { (*reverb).filterLeft = 0 as *mut s16 }
        if (*settings).lowPassFilterCutoffRight as libc::c_int !=
               0 as libc::c_int {
            (*reverb).filterRightState =
                AudioHeap_AllocDmaMemoryZeroed(&mut gAudioContext.notesAndBuffersPool,
                                               0x40 as libc::c_int as u32_0)
                    as *mut s16;
            (*reverb).filterRight =
                AudioHeap_AllocDmaMemory(&mut gAudioContext.notesAndBuffersPool,
                                         (8 as libc::c_int as
                                              libc::c_uint).wrapping_mul(::std::mem::size_of::<s16>()
                                                                             as
                                                                             libc::c_ulong))
                    as *mut s16;
            AudioHeap_LoadLowPassFilter((*reverb).filterRight,
                                        (*settings).lowPassFilterCutoffRight
                                            as s32);
        } else { (*reverb).filterRight = 0 as *mut s16 }
        i += 1
    }
    AudioSeq_InitSequencePlayers();
    j = 0 as libc::c_int;
    while j <
              gAudioContext.audioBufferParameters.numSequencePlayers as
                  libc::c_int {
        AudioSeq_InitSequencePlayerChannels(j);
        AudioSeq_ResetSequencePlayer(&mut *gAudioContext.seqPlayers.as_mut_ptr().offset(j
                                                                                            as
                                                                                            isize));
        j += 1
    }
    AudioHeap_InitSampleCaches((*spec).persistentSampleCacheMem as u32_0,
                               (*spec).temporarySampleCacheMem as u32_0);
    AudioLoad_InitSampleDmaBuffers(gAudioContext.numNotes);
    gAudioContext.preloadSampleStackTop = 0 as libc::c_int;
    AudioLoad_InitSlowLoads();
    AudioLoad_InitScriptLoads();
    AudioLoad_InitAsyncLoads();
    gAudioContext.unk_4 = 0x1000 as libc::c_int as u16_0;
    AudioLoad_LoadPermanentSamples();
    intMask = osSetIntMask(1 as libc::c_int as OSIntMask);
    osWritebackDCacheAll();
    osSetIntMask(intMask);
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_SearchPermanentCache(mut tableType: s32,
                                                        mut id: s32)
 -> *mut libc::c_void {
    let mut i: s32 = 0;
    i = 0 as libc::c_int;
    while i < gAudioContext.permanentPool.count {
        if gAudioContext.permanentCache[i as usize].tableType as libc::c_int
               == tableType &&
               gAudioContext.permanentCache[i as usize].id as libc::c_int ==
                   id {
            return gAudioContext.permanentCache[i as usize].ptr as
                       *mut libc::c_void
        }
        i += 1
    }
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_AllocPermanent(mut tableType: s32,
                                                  mut id: s32,
                                                  mut size: u32_0)
 -> *mut libc::c_void {
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut index: s32 = 0;
    index = gAudioContext.permanentPool.count;
    ret = AudioHeap_Alloc(&mut gAudioContext.permanentPool, size);
    gAudioContext.permanentCache[index as usize].ptr = ret as *mut u8_0;
    if ret.is_null() { return 0 as *mut libc::c_void }
    gAudioContext.permanentCache[index as usize].tableType = tableType as s16;
    gAudioContext.permanentCache[index as usize].id = id as s16;
    gAudioContext.permanentCache[index as usize].size = size;
    panic!("Reached end of non-void function without returning");
    // ! @bug UB: missing return. "ret" is in v0 at this point, but doing an
    // explicit return uses an additional register.
    // return ret;
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_AllocSampleCache(mut size: u32_0,
                                                    mut fontId: s32,
                                                    mut sampleAddr:
                                                        *mut libc::c_void,
                                                    mut medium: s8,
                                                    mut cache: s32)
 -> *mut libc::c_void {
    let mut entry: *mut SampleCacheEntry = 0 as *mut SampleCacheEntry;
    if cache == CACHE_TEMPORARY as libc::c_int {
        entry = AudioHeap_AllocTemporarySampleCacheEntry(size)
    } else { entry = AudioHeap_AllocPersistentSampleCacheEntry(size) }
    if !entry.is_null() {
        // ! @bug Should use sampleBankId, not fontId
        (*entry).sampleBankId = fontId as s8;
        (*entry).sampleAddr = sampleAddr;
        (*entry).origMedium = medium;
        return (*entry).allocatedAddr as *mut libc::c_void
    }
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_InitSampleCaches(mut persistentSize: u32_0,
                                                    mut temporarySize:
                                                        u32_0) {
    let mut mem: *mut libc::c_void = 0 as *mut libc::c_void;
    mem =
        AudioHeap_AllocAttemptExternal(&mut gAudioContext.notesAndBuffersPool,
                                       persistentSize);
    if mem.is_null() {
        gAudioContext.persistentSampleCache.pool.size = 0 as libc::c_int
    } else {
        AudioHeap_AllocPoolInit(&mut gAudioContext.persistentSampleCache.pool,
                                mem, persistentSize);
    }
    mem =
        AudioHeap_AllocAttemptExternal(&mut gAudioContext.notesAndBuffersPool,
                                       temporarySize);
    if mem.is_null() {
        gAudioContext.temporarySampleCache.pool.size = 0 as libc::c_int
    } else {
        AudioHeap_AllocPoolInit(&mut gAudioContext.temporarySampleCache.pool,
                                mem, temporarySize);
    }
    gAudioContext.persistentSampleCache.size = 0 as libc::c_int;
    gAudioContext.temporarySampleCache.size = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_AllocTemporarySampleCacheEntry(mut size:
                                                                      u32_0)
 -> *mut SampleCacheEntry {
    let mut allocAfter: *mut u8_0 = 0 as *mut u8_0;
    let mut allocBefore: *mut u8_0 = 0 as *mut u8_0;
    let mut mem: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut index: s32 = 0;
    let mut i: s32 = 0;
    let mut ret: *mut SampleCacheEntry = 0 as *mut SampleCacheEntry;
    let mut preload: *mut AudioPreloadReq = 0 as *mut AudioPreloadReq;
    let mut pool: *mut AudioSampleCache = 0 as *mut AudioSampleCache;
    let mut start: *mut u8_0 = 0 as *mut u8_0;
    let mut end: *mut u8_0 = 0 as *mut u8_0;
    pool = &mut gAudioContext.temporarySampleCache;
    allocBefore = (*pool).pool.cur;
    mem = AudioHeap_Alloc(&mut (*pool).pool, size);
    if mem.is_null() {
        // Reset the pool and try again. We still keep pointers to within the
        // pool, so we have to be careful to discard existing overlapping
        // allocations further down.
        let mut old: *mut u8_0 = (*pool).pool.cur;
        (*pool).pool.cur = (*pool).pool.start;
        mem = AudioHeap_Alloc(&mut (*pool).pool, size);
        if mem.is_null() {
            (*pool).pool.cur = old;
            return 0 as *mut SampleCacheEntry
        }
        allocBefore = (*pool).pool.start
    }
    allocAfter = (*pool).pool.cur;
    index = -(1 as libc::c_int);
    i = 0 as libc::c_int;
    while i < gAudioContext.preloadSampleStackTop {
        preload =
            &mut *gAudioContext.preloadSampleStack.as_mut_ptr().offset(i as
                                                                           isize)
                as *mut AudioPreloadReq;
        if (*preload).isFree == 0 as libc::c_int {
            start = (*preload).ramAddr;
            end =
                (*preload).ramAddr.offset((*(*preload).sample).size() as
                                              libc::c_int as
                                              isize).offset(-(1 as libc::c_int
                                                                  as isize));
            if !(end < allocBefore && start < allocBefore) {
                if !(end >= allocAfter && start >= allocAfter) {
                    // Overlap, skip this preload.
                    (*preload).isFree = 1 as libc::c_int
                }
            }
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < (*pool).size {
        if !((*pool).entries[i as usize].inUse as libc::c_int ==
                 0 as libc::c_int) {
            start = (*pool).entries[i as usize].allocatedAddr;
            end =
                start.offset((*pool).entries[i as usize].size as
                                 isize).offset(-(1 as libc::c_int as isize));
            if !(end < allocBefore && start < allocBefore) {
                if !(end >= allocAfter && start >= allocAfter) {
                    // Overlap, discard existing entry.
                    AudioHeap_DiscardSampleCacheEntry(&mut *(*pool).entries.as_mut_ptr().offset(i
                                                                                                    as
                                                                                                    isize)); // = change.oldAddr
                    if index == -(1 as libc::c_int) { index = i }
                }
            }
        }
        i += 1
    }
    if index == -(1 as libc::c_int) {
        let fresh2 = (*pool).size;
        (*pool).size = (*pool).size + 1;
        index = fresh2
    }
    ret =
        &mut *(*pool).entries.as_mut_ptr().offset(index as isize) as
            *mut SampleCacheEntry;
    (*ret).inUse = 1 as libc::c_int as s8;
    (*ret).allocatedAddr = mem as *mut u8_0;
    (*ret).size = size;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_UnapplySampleCacheForFont(mut entry:
                                                                 *mut SampleCacheEntry,
                                                             mut fontId:
                                                                 s32) {
    let mut drum: *mut Drum = 0 as *mut Drum;
    let mut inst: *mut Instrument = 0 as *mut Instrument;
    let mut sfx: *mut SoundFontSound = 0 as *mut SoundFontSound;
    let mut instId: s32 = 0;
    let mut drumId: s32 = 0;
    let mut sfxId: s32 = 0;
    instId = 0 as libc::c_int;
    while instId <
              (*gAudioContext.soundFonts.offset(fontId as
                                                    isize)).numInstruments as
                  libc::c_int {
        inst = Audio_GetInstrumentInner(fontId, instId);
        if !inst.is_null() {
            if (*inst).normalRangeLo as libc::c_int != 0 as libc::c_int {
                AudioHeap_UnapplySampleCache(entry,
                                             (*inst).lowNotesSound.sample);
            }
            if (*inst).normalRangeHi as libc::c_int != 0x7f as libc::c_int {
                AudioHeap_UnapplySampleCache(entry,
                                             (*inst).highNotesSound.sample);
            }
            AudioHeap_UnapplySampleCache(entry,
                                         (*inst).normalNotesSound.sample);
        }
        instId += 1
    }
    drumId = 0 as libc::c_int;
    while drumId <
              (*gAudioContext.soundFonts.offset(fontId as isize)).numDrums as
                  libc::c_int {
        drum = Audio_GetDrum(fontId, drumId);
        if !drum.is_null() {
            AudioHeap_UnapplySampleCache(entry, (*drum).sound.sample);
        }
        drumId += 1
    }
    sfxId = 0 as libc::c_int;
    while sfxId <
              (*gAudioContext.soundFonts.offset(fontId as isize)).numSfx as
                  libc::c_int {
        sfx = Audio_GetSfx(fontId, sfxId);
        if !sfx.is_null() {
            AudioHeap_UnapplySampleCache(entry, (*sfx).sample);
        }
        sfxId += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_DiscardSampleCacheEntry(mut entry:
                                                               *mut SampleCacheEntry) {
    let mut numFonts: s32 = 0;
    let mut sampleBankId1: s32 = 0;
    let mut sampleBankId2: s32 = 0;
    let mut fontId: s32 = 0;
    numFonts = (*gAudioContext.soundFontTable).numEntries as s32;
    fontId = 0 as libc::c_int;
    while fontId < numFonts {
        sampleBankId1 =
            (*gAudioContext.soundFonts.offset(fontId as isize)).sampleBankId1
                as s32;
        sampleBankId2 =
            (*gAudioContext.soundFonts.offset(fontId as isize)).sampleBankId2
                as s32;
        if sampleBankId1 != 0xff as libc::c_int &&
               (*entry).sampleBankId as libc::c_int == sampleBankId1 ||
               sampleBankId2 != 0xff as libc::c_int &&
                   (*entry).sampleBankId as libc::c_int == sampleBankId2 ||
               (*entry).sampleBankId as libc::c_int == 0 as libc::c_int {
            if !AudioHeap_SearchCaches(FONT_TABLE as libc::c_int,
                                       CACHE_EITHER as libc::c_int,
                                       fontId).is_null() {
                if AudioLoad_IsFontLoadComplete(fontId) != 0 as libc::c_int {
                    AudioHeap_UnapplySampleCacheForFont(entry, fontId);
                }
            }
        }
        fontId += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_UnapplySampleCache(mut entry:
                                                          *mut SampleCacheEntry,
                                                      mut sample:
                                                          *mut SoundFontSample) {
    if !sample.is_null() {
        if (*sample).sampleAddr == (*entry).allocatedAddr {
            (*sample).sampleAddr = (*entry).sampleAddr as *mut u8_0;
            (*sample).set_medium((*entry).origMedium as u32_0)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_AllocPersistentSampleCacheEntry(mut size:
                                                                       u32_0)
 -> *mut SampleCacheEntry {
    let mut pool: *mut AudioSampleCache = 0 as *mut AudioSampleCache;
    let mut entry: *mut SampleCacheEntry = 0 as *mut SampleCacheEntry;
    let mut mem: *mut libc::c_void = 0 as *mut libc::c_void;
    pool = &mut gAudioContext.persistentSampleCache;
    mem = AudioHeap_Alloc(&mut (*pool).pool, size);
    if mem.is_null() { return 0 as *mut SampleCacheEntry }
    entry =
        &mut *(*pool).entries.as_mut_ptr().offset((*pool).size as isize) as
            *mut SampleCacheEntry;
    (*entry).inUse = 1 as libc::c_int as s8;
    (*entry).allocatedAddr = mem as *mut u8_0;
    (*entry).size = size;
    (*pool).size += 1;
    return entry;
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_DiscardSampleCacheForFont(mut entry:
                                                                 *mut SampleCacheEntry,
                                                             mut sampleBankId1:
                                                                 s32,
                                                             mut sampleBankId2:
                                                                 s32,
                                                             mut fontId:
                                                                 s32) {
    if (*entry).sampleBankId as libc::c_int == sampleBankId1 ||
           (*entry).sampleBankId as libc::c_int == sampleBankId2 ||
           (*entry).sampleBankId as libc::c_int == 0 as libc::c_int {
        AudioHeap_UnapplySampleCacheForFont(entry, fontId);
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_DiscardSampleCaches() {
    let mut numFonts: s32 = 0;
    let mut sampleBankId1: s32 = 0;
    let mut sampleBankId2: s32 = 0;
    let mut fontId: s32 = 0;
    let mut j: s32 = 0;
    numFonts = (*gAudioContext.soundFontTable).numEntries as s32;
    fontId = 0 as libc::c_int;
    while fontId < numFonts {
        sampleBankId1 =
            (*gAudioContext.soundFonts.offset(fontId as isize)).sampleBankId1
                as s32;
        sampleBankId2 =
            (*gAudioContext.soundFonts.offset(fontId as isize)).sampleBankId2
                as s32;
        if !(sampleBankId1 == 0xff as libc::c_int &&
                 sampleBankId2 == 0xff as libc::c_int) {
            if !(AudioHeap_SearchCaches(FONT_TABLE as libc::c_int,
                                        CACHE_PERMANENT as libc::c_int,
                                        fontId).is_null() ||
                     AudioLoad_IsFontLoadComplete(fontId) == 0) {
                j = 0 as libc::c_int;
                while j < gAudioContext.persistentSampleCache.size {
                    AudioHeap_DiscardSampleCacheForFont(&mut *gAudioContext.persistentSampleCache.entries.as_mut_ptr().offset(j
                                                                                                                                  as
                                                                                                                                  isize),
                                                        sampleBankId1,
                                                        sampleBankId2,
                                                        fontId);
                    j += 1
                }
                j = 0 as libc::c_int;
                while j < gAudioContext.temporarySampleCache.size {
                    AudioHeap_DiscardSampleCacheForFont(&mut *gAudioContext.temporarySampleCache.entries.as_mut_ptr().offset(j
                                                                                                                                 as
                                                                                                                                 isize),
                                                        sampleBankId1,
                                                        sampleBankId2,
                                                        fontId);
                    j += 1
                }
            }
        }
        fontId += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_ChangeStorage(mut change:
                                                     *mut StorageChange,
                                                 mut sample:
                                                     *mut SoundFontSample) {
    if !sample.is_null() {
        let mut start: u32_0 = (*change).oldAddr;
        let mut end: u32_0 = (*change).oldAddr.wrapping_add((*change).size);
        if start <= (*sample).sampleAddr as u32_0 &&
               ((*sample).sampleAddr as u32_0) < end {
            (*sample).sampleAddr =
                (*sample).sampleAddr.offset(-(start as
                                                  isize)).offset((*change).newAddr
                                                                     as
                                                                     isize);
            (*sample).set_medium((*change).newMedium as u32_0)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_DiscardSampleBank(mut sampleBankId: s32) {
    AudioHeap_ApplySampleBankCacheInternal(0 as libc::c_int, sampleBankId);
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_ApplySampleBankCache(mut sampleBankId:
                                                            s32) {
    AudioHeap_ApplySampleBankCacheInternal(1 as libc::c_int, sampleBankId);
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_ApplySampleBankCacheInternal(mut apply:
                                                                    s32,
                                                                mut sampleBankId:
                                                                    s32) {
    let mut sampleBankTable: *mut AudioTable = 0 as *mut AudioTable;
    let mut entry: *mut AudioTableEntry = 0 as *mut AudioTableEntry;
    let mut numFonts: s32 = 0;
    let mut instId: s32 = 0;
    let mut drumId: s32 = 0;
    let mut sfxId: s32 = 0;
    let mut change: StorageChange =
        StorageChange{oldAddr: 0, newAddr: 0, size: 0, newMedium: 0,};
    let mut sampleBankId1: s32 = 0;
    let mut sampleBankId2: s32 = 0;
    let mut fontId: s32 = 0;
    let mut drum: *mut Drum = 0 as *mut Drum;
    let mut inst: *mut Instrument = 0 as *mut Instrument;
    let mut sfx: *mut SoundFontSound = 0 as *mut SoundFontSound;
    let mut fakematch: *mut u32_0 = 0 as *mut u32_0;
    let mut pad: [s32; 4] = [0; 4];
    sampleBankTable = gAudioContext.sampleBankTable;
    numFonts = (*gAudioContext.soundFontTable).numEntries as s32;
    change.oldAddr =
        AudioHeap_SearchCaches(SAMPLE_TABLE as libc::c_int,
                               CACHE_EITHER as libc::c_int, sampleBankId) as
            u32_0;
    if change.oldAddr == 0 as libc::c_int as libc::c_uint { return }
    entry =
        &mut *(*sampleBankTable).entries.as_mut_ptr().offset(sampleBankId as
                                                                 isize) as
            *mut AudioTableEntry;
    change.size = (*entry).size;
    change.newMedium = (*entry).medium as u8_0;
    if change.newMedium as libc::c_int == MEDIUM_CART as libc::c_int ||
           change.newMedium as libc::c_int == MEDIUM_DISK_DRIVE as libc::c_int
       {
        change.newAddr = (*entry).romAddr
    } else { change.newAddr = 0 as libc::c_int as u32_0 }
    fakematch = &mut change.oldAddr;
    if apply != 0 as libc::c_int && apply == 1 as libc::c_int {
        let mut temp: u32_0 = change.newAddr;
        change.newAddr = *fakematch;
        change.oldAddr = temp;
        change.newMedium = MEDIUM_RAM as libc::c_int as u8_0
    }
    let mut current_block_51: u64;
    fontId = 0 as libc::c_int;
    while fontId < numFonts {
        sampleBankId1 =
            (*gAudioContext.soundFonts.offset(fontId as isize)).sampleBankId1
                as s32;
        sampleBankId2 =
            (*gAudioContext.soundFonts.offset(fontId as isize)).sampleBankId2
                as s32;
        if sampleBankId1 != 0xff as libc::c_int ||
               sampleBankId2 != 0xff as libc::c_int {
            if !(AudioLoad_IsFontLoadComplete(fontId) == 0 ||
                     AudioHeap_SearchCaches(FONT_TABLE as libc::c_int,
                                            CACHE_EITHER as libc::c_int,
                                            fontId).is_null()) {
                if sampleBankId1 == sampleBankId {
                    current_block_51 = 6417057564578538666;
                } else if sampleBankId2 == sampleBankId {
                    current_block_51 = 6417057564578538666;
                } else { current_block_51 = 14763689060501151050; }
                match current_block_51 {
                    14763689060501151050 => { }
                    _ => {
                        instId = 0 as libc::c_int;
                        while instId <
                                  (*gAudioContext.soundFonts.offset(fontId as
                                                                        isize)).numInstruments
                                      as libc::c_int {
                            inst = Audio_GetInstrumentInner(fontId, instId);
                            if !inst.is_null() {
                                if (*inst).normalRangeLo as libc::c_int !=
                                       0 as libc::c_int {
                                    AudioHeap_ChangeStorage(&mut change,
                                                            (*inst).lowNotesSound.sample);
                                }
                                if (*inst).normalRangeHi as libc::c_int !=
                                       0x7f as libc::c_int {
                                    AudioHeap_ChangeStorage(&mut change,
                                                            (*inst).highNotesSound.sample);
                                }
                                AudioHeap_ChangeStorage(&mut change,
                                                        (*inst).normalNotesSound.sample);
                            }
                            instId += 1
                        }
                        drumId = 0 as libc::c_int;
                        while drumId <
                                  (*gAudioContext.soundFonts.offset(fontId as
                                                                        isize)).numDrums
                                      as libc::c_int {
                            drum = Audio_GetDrum(fontId, drumId);
                            if !drum.is_null() {
                                AudioHeap_ChangeStorage(&mut change,
                                                        (*drum).sound.sample);
                            }
                            drumId += 1
                        }
                        sfxId = 0 as libc::c_int;
                        while sfxId <
                                  (*gAudioContext.soundFonts.offset(fontId as
                                                                        isize)).numSfx
                                      as libc::c_int {
                            sfx = Audio_GetSfx(fontId, sfxId);
                            if !sfx.is_null() {
                                AudioHeap_ChangeStorage(&mut change,
                                                        (*sfx).sample);
                            }
                            sfxId += 1
                        }
                    }
                }
            }
        }
        fontId += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn AudioHeap_DiscardSampleBanks() {
    let mut pool: *mut AudioCache = 0 as *mut AudioCache;
    let mut persistent: *mut AudioPersistentCache =
        0 as *mut AudioPersistentCache;
    let mut temporary: *mut AudioTemporaryCache =
        0 as *mut AudioTemporaryCache;
    let mut i: u32_0 = 0;
    pool = &mut gAudioContext.sampleBankCache;
    temporary = &mut (*pool).temporary;
    if (*temporary).entries[0 as libc::c_int as usize].id as libc::c_int !=
           -(1 as libc::c_int) {
        AudioHeap_DiscardSampleBank((*temporary).entries[0 as libc::c_int as
                                                             usize].id as
                                        s32);
    }
    if (*temporary).entries[1 as libc::c_int as usize].id as libc::c_int !=
           -(1 as libc::c_int) {
        AudioHeap_DiscardSampleBank((*temporary).entries[1 as libc::c_int as
                                                             usize].id as
                                        s32);
    }
    persistent = &mut (*pool).persistent;
    i = 0 as libc::c_int as u32_0;
    while i < (*persistent).numEntries {
        AudioHeap_DiscardSampleBank((*persistent).entries[i as usize].id as
                                        s32);
        i = i.wrapping_add(1)
    };
}
